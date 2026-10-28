// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧰️harness holds the shared helpers of the launcher cases: the repository root, fixtures, a recording host and synthetic runtime archives.

// #endregion 🧲️Header

package launcher

import (
	"archive/zip"
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"sync/atomic"
	"testing"
)

// #region 🧰️Harness

func testRoot(t *testing.T) string {
	t.Helper()
	start, err := StartDirectory(os.Getenv, os.Getwd)
	if err != nil {
		t.Fatal(err)
	}
	root, err := ResolveRoot(start)
	if err != nil {
		t.Fatal(err)
	}
	return root
}

// 🕳️emptyWorkspace answers an empty directory that no workspace root resolves from: the first scratch base (the issued temporary directory, the per-user cache, the shared temporary directories of Unix) holding a fresh directory without a repository above it, so a case about a missing repository holds when a harness issues its scratch below a checkout.
func emptyWorkspace(t *testing.T) string {
	t.Helper()
	bases := []string{os.TempDir()}
	if cache, err := os.UserCacheDir(); err == nil {
		bases = append(bases, cache)
	}
	if runtime.GOOS != "windows" {
		bases = append(bases, "/tmp", "/var/tmp")
	}
	for _, base := range bases {
		directory, err := os.MkdirTemp(base, "launcher-empty-")
		if err != nil {
			continue
		}
		t.Cleanup(func() { os.RemoveAll(directory) })
		if _, err := ResolveRoot(directory); IsRootNotFound(err) {
			return directory
		}
	}
	t.Fatal("no scratch base lies outside every repository")
	return ""
}

func readFixture(t *testing.T, relative string, into any) {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(testRoot(t), filepath.FromSlash(relative)))
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(raw, into); err != nil {
		t.Fatal(err)
	}
}

const launcherFixture = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧫️fixtures/🔣️.json"

func loadRegistry(t *testing.T) *Registry {
	t.Helper()
	registry, err := LoadRegistry(testRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	return registry
}

type recorded struct {
	out, err []string
	calls    int
	exe      string
	argv     []string
	env      []string
}

func environment(values map[string]string) Environment {
	return func(name string) string { return values[name] }
}

func recordingHost(t *testing.T, values map[string]string) (Host, *recorded) {
	t.Helper()
	log := &recorded{}
	return Host{
		Env:     environment(values),
		Environ: func() []string { return []string{"PATH=/usr/bin", "HOME=/h"} },
		Stdout:  func(text string) { log.out = append(log.out, text) },
		Stderr:  func(text string) { log.err = append(log.err, text) },
		Getwd:   os.Getwd,
		System:  func() []string { return nil },
		Target:  func() (string, string, bool) { return "linux", "x64", false },
		HandOff: func(executable string, argv []string, env []string) error {
			log.calls++
			log.exe, log.argv, log.env = executable, argv, env
			return nil
		},
		Self: func() string { return "/self/bin/repo" },
	}, log
}

const syntheticVersion = "9.9.9"

func syntheticRuntime(version string) string {
	return "#!/bin/sh\necho " + version + "\n"
}

func zipOf(t *testing.T, member, content string) []byte {
	t.Helper()
	var buffer bytes.Buffer
	writer := zip.NewWriter(&buffer)
	header := &zip.FileHeader{Name: member, Method: zip.Deflate}
	header.SetMode(0o755)
	entry, err := writer.CreateHeader(header)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := entry.Write([]byte(content)); err != nil {
		t.Fatal(err)
	}
	if err := writer.Close(); err != nil {
		t.Fatal(err)
	}
	return buffer.Bytes()
}

type synthetic struct {
	server   *httptest.Server
	requests atomic.Int32
	request  ProvisionRequest
}

func syntheticRegistry(t *testing.T, archive []byte, name string) (*Registry, Runtime, Distribution) {
	t.Helper()
	sum := sha256.Sum256(archive)
	registry := loadRegistry(t)
	rt := registry.Runtimes["bun"]
	rt.Version = syntheticVersion
	rt.Download.Member = "{stem}/{executable}"
	row := Distribution{Platform: "linux", Arch: "x64", Libc: "gnu", Archive: name + ".zip", Bytes: int64(len(archive)), Sha256: hex.EncodeToString(sum[:])}
	rt.Distributions = []Distribution{row}
	rt.Executable.Unix, rt.Executable.Windows = "bun", "bun.exe"
	return registry, rt, row
}

func serveArchive(t *testing.T, body []byte, status int, handler func(http.ResponseWriter, *http.Request)) *synthetic {
	t.Helper()
	fake := &synthetic{}
	fake.server = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fake.requests.Add(1)
		if handler != nil {
			handler(w, r)
			return
		}
		if status != http.StatusOK {
			http.Error(w, "no", status)
			return
		}
		_, _ = w.Write(body)
	}))
	t.Cleanup(fake.server.Close)
	return fake
}

func skipWithoutShell(t *testing.T) {
	t.Helper()
	if runtime.GOOS == "windows" {
		t.Skip("the synthetic runtime is a shell script")
	}
}

func lines(text []string) string { return strings.Join(text, "\n") }

// #endregion 🧰️Harness
