// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📥️runtime-provision finds a pinned runtime in the tools partition or the user cache and otherwise fetches it: download, size and SHA-256 check, extraction of the executable, version probe, atomic publication.

// #endregion 🧲️Header

package launcher

import (
	"archive/zip"
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"time"
)

// #region 📥️Provision

// 🏷️Failure is a localizable failure: a registry text identifier with its values.
type Failure struct {
	ID     string
	Values map[string]string
}

// 🔤️Error names the text identifier and its values.
func (f Failure) Error() string {
	keys := sortedKeys(f.Values)
	parts := make([]string, 0, len(keys)+1)
	parts = append(parts, f.ID)
	for _, key := range keys {
		parts = append(parts, key+"="+f.Values[key])
	}
	return strings.Join(parts, " ")
}

// 📍️Install is a usable runtime: where its executable is and which store served it.
type Install struct{ Executable, Directory, Origin string }

// 📨️ProvisionRequest holds everything provisioning needs; nothing is read from the process beside what is passed.
type ProvisionRequest struct {
	Root     string
	Name     string
	Registry *Registry
	Runtime  Runtime
	Platform string
	Arch     string
	Musl     bool
	Env      Environment
	Client   *http.Client
	Progress func(id string, values map[string]string)
	Now      func() time.Time
}

// 🧾️toolchainReceipt is the receipt next to an installed executable; the TypeScript tools store writes the same two leading fields.
type toolchainReceipt struct {
	Archive    string `json:"archive"`
	Executable string `json:"executable"`
	Bytes      int64  `json:"bytes,omitempty"`
	Runtime    string `json:"runtime,omitempty"`
	Version    string `json:"version,omitempty"`
}

const receiptName = ".toolchain.json"

// 🏪️StoreDirectories answers where an archive's install lives in the tools partition and in the user cache.
func (q ProvisionRequest) StoreDirectories(row Distribution) (tools string, user string, err error) {
	cache := q.Registry.Cache
	tools = filepath.Join(append(append([]string{q.Root}, cache.ToolsPartition...), q.Name, row.Sha256)...)
	base := q.Env(cache.Environment)
	if base == "" {
		directory, failure := os.UserCacheDir()
		if failure != nil {
			return "", "", Failure{ID: "cacheUnavailable", Values: map[string]string{"reason": failure.Error()}}
		}
		base = filepath.Join(append([]string{directory}, cache.User...)...)
	}
	return tools, filepath.Join(base, q.Name, row.Sha256), nil
}

func (q ProvisionRequest) row() (Distribution, error) {
	row, ok := q.Runtime.Distribution(q.Platform, q.Arch, q.Musl)
	if !ok {
		libc := ""
		if q.Musl {
			libc = "/musl"
		}
		return row, Failure{ID: "runtimeUnsupported", Values: map[string]string{"runtime": q.Name, "version": q.Runtime.Version, "platform": q.Platform, "arch": q.Arch, "libc": libc}}
	}
	return row, nil
}

func verifyInstalled(directory string, name string, row Distribution) bool {
	executable := filepath.Join(directory, name)
	info, err := os.Lstat(executable)
	if err != nil || !info.Mode().IsRegular() || (runtime.GOOS != "windows" && info.Mode().Perm()&0o100 == 0) {
		return false
	}
	raw, err := os.ReadFile(filepath.Join(directory, receiptName))
	var receipt toolchainReceipt
	if err != nil || json.Unmarshal(raw, &receipt) != nil || receipt.Archive != row.Sha256 {
		return false
	}
	return receipt.Bytes == 0 || receipt.Bytes == info.Size()
}

// 🔎️Locate answers an installed runtime without side effects: the tools partition first, then the user cache.
func (q ProvisionRequest) Locate() (Install, bool, error) {
	row, err := q.row()
	if err != nil {
		return Install{}, false, err
	}
	tools, user, err := q.StoreDirectories(row)
	if err != nil {
		return Install{}, false, err
	}
	name := q.Runtime.ExecutableName()
	for _, candidate := range []struct{ directory, origin string }{{tools, "tools-partition"}, {user, "user-cache"}} {
		if verifyInstalled(candidate.directory, name, row) {
			return Install{Executable: filepath.Join(candidate.directory, name), Directory: candidate.directory, Origin: candidate.origin}, true, nil
		}
	}
	return Install{}, false, nil
}

func (q ProvisionRequest) say(id string, values map[string]string) {
	if q.Progress != nil {
		q.Progress(id, values)
	}
}

func (q ProvisionRequest) source(row Distribution) (base string, link string, err error) {
	base = q.Runtime.Download.Base
	if mirror := strings.TrimRight(q.Env(q.Registry.Cache.MirrorEnvironment), "/"); mirror != "" {
		parsed, failure := url.Parse(mirror)
		if failure != nil || (parsed.Scheme != "https" && !(parsed.Scheme == "http" && isLoopback(parsed.Hostname()))) {
			return "", "", Failure{ID: "mirrorRefused", Values: map[string]string{"url": mirror, "source": base}}
		}
		base = mirror
	}
	return base, base + "/" + row.Archive, nil
}

func isLoopback(host string) bool {
	address := net.ParseIP(host)
	return host == "localhost" || (address != nil && address.IsLoopback())
}

// 🏗️Ensure answers the installed runtime and fetches it when absent; ctx cancels the download and the wait for a peer.
func (q ProvisionRequest) Ensure(ctx context.Context) (Install, error) {
	if found, ok, err := q.Locate(); ok || err != nil {
		return found, err
	}
	row, _ := q.row()
	_, user, err := q.StoreDirectories(row)
	if err != nil {
		return Install{}, err
	}
	limit := q.Registry.Cache.MaximumPathLength
	longest := filepath.Join(filepath.Dir(user), ".prepare-0123456789abcdef", strings.ReplaceAll(q.Runtime.Download.Member, "{stem}", strings.TrimSuffix(row.Archive, ".zip")))
	if len(filepath.Join(user, q.Runtime.ExecutableName())) > limit || len(longest) > limit {
		return Install{}, Failure{ID: "pathTooLong", Values: map[string]string{"path": longest, "limit": strconv.Itoa(limit)}}
	}
	if err := os.MkdirAll(filepath.Dir(user), 0o755); err != nil {
		return Install{}, Failure{ID: "cacheUnavailable", Values: map[string]string{"reason": err.Error()}}
	}
	release, err := q.lock(ctx, filepath.Join(filepath.Dir(user), ".lock-"+row.Sha256[:16]))
	if err != nil {
		return Install{}, err
	}
	defer release()
	if found, ok, err := q.Locate(); ok || err != nil {
		return found, err
	}
	if err := q.fetch(ctx, row, user); err != nil {
		if errors.Is(err, context.Canceled) || ctx.Err() != nil {
			return Install{}, Failure{ID: "cancelled"}
		}
		return Install{}, err
	}
	found, ok, err := q.Locate()
	if !ok && err == nil {
		err = Failure{ID: "integrityFailed", Values: map[string]string{"what": "receipt", "archive": row.Archive, "actual": "missing", "expected": row.Sha256}}
	}
	return found, err
}

func (q ProvisionRequest) lock(ctx context.Context, path string) (func(), error) {
	announced := false
	for {
		file, err := os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
		if err == nil {
			_, _ = file.WriteString(strconv.Itoa(os.Getpid()))
			_ = file.Close()
			return func() { _ = os.Remove(path) }, nil
		}
		if !errors.Is(err, os.ErrExist) {
			return nil, Failure{ID: "cacheUnavailable", Values: map[string]string{"reason": err.Error()}}
		}
		if info, statFailure := os.Stat(path); statFailure == nil && q.now().Sub(info.ModTime()) > 20*time.Minute {
			_ = os.Remove(path)
			continue
		}
		if !announced {
			announced = true
			q.say("waitingForPeer", map[string]string{"runtime": q.Name, "version": q.Runtime.Version})
		}
		if found, ok, _ := q.Locate(); ok {
			_ = found
			return func() {}, nil
		}
		select {
		case <-ctx.Done():
			return nil, Failure{ID: "cancelled"}
		case <-time.After(500 * time.Millisecond):
		}
	}
}

func (q ProvisionRequest) now() time.Time {
	if q.Now != nil {
		return q.Now()
	}
	return time.Now()
}

func (q ProvisionRequest) fetch(ctx context.Context, row Distribution, final string) error {
	base, link, err := q.source(row)
	if err != nil {
		return err
	}
	q.say("provisioning", map[string]string{"runtime": q.Name, "version": q.Runtime.Version, "platform": q.Platform, "arch": q.Arch, "megabytes": strconv.FormatInt(row.Bytes>>20, 10), "source": base})
	token := make([]byte, 8)
	_, _ = rand.Read(token)
	staging := filepath.Join(filepath.Dir(final), ".prepare-"+hex.EncodeToString(token))
	if err := os.Mkdir(staging, 0o755); err != nil {
		return Failure{ID: "cacheUnavailable", Values: map[string]string{"reason": err.Error()}}
	}
	defer os.RemoveAll(staging)
	archive := filepath.Join(staging, row.Archive)
	if err := q.download(ctx, row, link, base, archive); err != nil {
		return err
	}
	executable := filepath.Join(staging, q.Runtime.ExecutableName())
	digest, size, err := q.extract(row, archive, executable)
	if err != nil {
		return err
	}
	if err := os.Remove(archive); err != nil {
		return err
	}
	if err := q.probe(ctx, executable); err != nil {
		return err
	}
	receipt, _ := json.Marshal(toolchainReceipt{Archive: row.Sha256, Executable: digest, Bytes: size, Runtime: q.Name, Version: q.Runtime.Version})
	if err := os.WriteFile(filepath.Join(staging, receiptName), receipt, 0o644); err != nil {
		return err
	}
	if !verifyInstalled(final, q.Runtime.ExecutableName(), row) {
		_ = os.RemoveAll(final)
	}
	if err := renameRetrying(os.Rename, renameTransient, time.Sleep, staging, final); err != nil {
		if verifyInstalled(final, q.Runtime.ExecutableName(), row) {
			return nil
		}
		return Failure{ID: "cacheUnavailable", Values: map[string]string{"reason": err.Error()}}
	}
	return nil
}

// 🔁️renameRetrying publishes by rename and repeats a refusal the host calls transient (Windows: a scanner or indexer that holds the fresh executable for a moment) after 10, 20, 40 … 1280 ms, 2.55 s in all, like the Go command's own robust rename; any other error, and the last transient one, is returned. See https://cs.opensource.google/go/go/+/refs/tags/go1.26.0:src/cmd/go/internal/robustio/robustio_windows.go
func renameRetrying(rename func(string, string) error, transient func(error) bool, sleep func(time.Duration), from, to string) error {
	for wait := 10 * time.Millisecond; ; wait *= 2 {
		err := rename(from, to)
		if err == nil || !transient(err) || wait > 1280*time.Millisecond {
			return err
		}
		sleep(wait)
	}
}

func (q ProvisionRequest) download(ctx context.Context, row Distribution, link, base, destination string) error {
	client := q.Client
	if client == nil {
		client = &http.Client{
			Transport: &http.Transport{Proxy: http.ProxyFromEnvironment, ResponseHeaderTimeout: 60 * time.Second, TLSHandshakeTimeout: 30 * time.Second},
			CheckRedirect: func(next *http.Request, via []*http.Request) error {
				if len(via) >= 10 || (next.URL.Scheme != "https" && !isLoopback(next.URL.Hostname())) {
					return Failure{ID: "redirectRefused", Values: map[string]string{"url": next.URL.String()}}
				}
				return nil
			},
		}
	}
	deadline, stop := context.WithTimeout(ctx, 30*time.Minute)
	defer stop()
	var failure error
	for attempt := 1; attempt <= 3; attempt++ {
		var retry bool
		retry, failure = q.transfer(deadline, client, row, link, destination)
		if failure == nil || !retry || deadline.Err() != nil {
			break
		}
		select {
		case <-deadline.Done():
		case <-time.After(time.Duration(attempt) * time.Second):
		}
	}
	if failure == nil {
		return nil
	}
	var integrity Failure
	if errors.As(failure, &integrity) || errors.Is(failure, context.Canceled) || ctx.Err() != nil {
		return failure
	}
	return Failure{ID: "downloadFailed", Values: map[string]string{"url": link, "reason": failure.Error(), "source": base, "directory": filepath.Dir(destination)}}
}

func (q ProvisionRequest) transfer(ctx context.Context, client *http.Client, row Distribution, link, destination string) (bool, error) {
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, link, nil)
	if err != nil {
		return false, err
	}
	response, err := client.Do(request)
	if err != nil {
		return true, err
	}
	defer response.Body.Close()
	if response.StatusCode != http.StatusOK {
		return response.StatusCode >= 500, fmt.Errorf("HTTP %d", response.StatusCode)
	}
	file, err := os.OpenFile(destination, os.O_CREATE|os.O_TRUNC|os.O_WRONLY, 0o600)
	if err != nil {
		return false, err
	}
	defer file.Close()
	hash := sha256.New()
	var done int64
	nextPercent, last := int64(10), q.now()
	buffer := make([]byte, 256<<10)
	for {
		count, readFailure := response.Body.Read(buffer)
		if count > 0 {
			done += int64(count)
			if done > row.Bytes {
				return false, Failure{ID: "integrityFailed", Values: map[string]string{"what": "size", "archive": row.Archive, "actual": strconv.FormatInt(row.Bytes, 10) + "+", "expected": strconv.FormatInt(row.Bytes, 10)}}
			}
			hash.Write(buffer[:count])
			if _, writeFailure := file.Write(buffer[:count]); writeFailure != nil {
				return false, writeFailure
			}
			if percent := done * 100 / row.Bytes; percent >= nextPercent || q.now().Sub(last) >= 10*time.Second {
				q.say("downloading", map[string]string{"runtime": q.Name, "version": q.Runtime.Version, "percent": strconv.FormatInt(percent, 10), "done": strconv.FormatInt(done>>20, 10), "total": strconv.FormatInt(row.Bytes>>20, 10)})
				nextPercent, last = percent/10*10+10, q.now()
			}
		}
		if readFailure == io.EOF {
			break
		}
		if readFailure != nil {
			return true, readFailure
		}
	}
	if done != row.Bytes {
		return false, Failure{ID: "integrityFailed", Values: map[string]string{"what": "size", "archive": row.Archive, "actual": strconv.FormatInt(done, 10), "expected": strconv.FormatInt(row.Bytes, 10)}}
	}
	if actual := hex.EncodeToString(hash.Sum(nil)); actual != row.Sha256 {
		return false, Failure{ID: "integrityFailed", Values: map[string]string{"what": "SHA-256", "archive": row.Archive, "actual": actual, "expected": row.Sha256}}
	}
	return false, nil
}

func (q ProvisionRequest) extract(row Distribution, archive, executable string) (string, int64, error) {
	member := strings.NewReplacer("{stem}", strings.TrimSuffix(row.Archive, ".zip"), "{executable}", q.Runtime.ExecutableName()).Replace(q.Runtime.Download.Member)
	reader, err := zip.OpenReader(archive)
	if err != nil {
		return "", 0, Failure{ID: "extractFailed", Values: map[string]string{"member": member, "archive": row.Archive}}
	}
	defer reader.Close()
	for _, entry := range reader.File {
		if entry.Name != member {
			continue
		}
		limit := q.Runtime.Download.MaximumExtractedBytes
		if entry.UncompressedSize64 > uint64(limit) {
			return "", 0, Failure{ID: "extractTooLarge", Values: map[string]string{"member": member, "archive": row.Archive, "limit": strconv.FormatInt(limit, 10)}}
		}
		source, err := entry.Open()
		if err != nil {
			return "", 0, Failure{ID: "extractFailed", Values: map[string]string{"member": member, "archive": row.Archive}}
		}
		defer source.Close()
		target, err := os.OpenFile(executable, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o755)
		if err != nil {
			return "", 0, err
		}
		hash := sha256.New()
		size, copyFailure := io.Copy(io.MultiWriter(target, hash), io.LimitReader(source, limit+1))
		if closeFailure := target.Close(); copyFailure == nil {
			copyFailure = closeFailure
		}
		if copyFailure != nil || size != int64(entry.UncompressedSize64) {
			return "", 0, Failure{ID: "extractFailed", Values: map[string]string{"member": member, "archive": row.Archive}}
		}
		return hex.EncodeToString(hash.Sum(nil)), size, os.Chmod(executable, 0o755)
	}
	return "", 0, Failure{ID: "extractFailed", Values: map[string]string{"member": member, "archive": row.Archive}}
}

func (q ProvisionRequest) probe(ctx context.Context, executable string) error {
	deadline, stop := context.WithTimeout(ctx, 30*time.Second)
	defer stop()
	output, err := exec.CommandContext(deadline, executable, q.Runtime.Probe.Arguments...).Output()
	actual := strings.TrimSpace(string(output))
	if err != nil {
		actual = err.Error()
	}
	if err != nil || actual != q.Runtime.Version {
		return Failure{ID: "probeFailed", Values: map[string]string{"runtime": q.Name, "actual": actual, "expected": q.Runtime.Version}}
	}
	return nil
}

// #endregion 📥️Provision
