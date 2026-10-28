// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📥️provision-contract proves provisioning against a synthetic runtime archive on a loopback server: the integrity fixture, a single download for concurrent starts, cancellation, store order and the cache budget.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"errors"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

func provisionRequest(t *testing.T, archive []byte, fake *synthetic, cache string, sink *[]string) ProvisionRequest {
	t.Helper()
	registry, rt, _ := syntheticRegistry(t, archive, "bun-linux-x64")
	root := t.TempDir()
	progress := func(id string, values map[string]string) {
		if sink != nil {
			*sink = append(*sink, id)
		}
	}
	return ProvisionRequest{Root: root, Name: "bun", Registry: registry, Runtime: rt, Platform: "linux", Arch: "x64", Env: environment(map[string]string{"SEMIO_RUNTIME_CACHE": cache, "SEMIO_RUNTIME_MIRROR": fake.server.URL}), Progress: progress}
}

func mutated(archive []byte, mutation string) []byte {
	switch mutation {
	case "flip":
		copied := append([]byte{}, archive...)
		copied[len(copied)/2] ^= 0xff
		return copied
	case "truncate":
		return archive[:len(archive)-5]
	case "extend":
		return append(append([]byte{}, archive...), 0, 0, 0)
	}
	return archive
}

// 📥️Each mutation of the archive is refused for its own reason and installs nothing.
func TestProvisioningIntegrityFixture(t *testing.T) {
	skipWithoutShell(t)
	var fixture struct {
		Provisioning []struct{ Name, Mutation, Outcome, What string }
	}
	readFixture(t, launcherFixture, &fixture)
	for _, row := range fixture.Provisioning {
		member, content := "bun-linux-x64/bun", syntheticRuntime(syntheticVersion)
		if row.Mutation == "member" {
			member = "bun-linux-x64/other"
		}
		if row.Mutation == "version" {
			content = syntheticRuntime("0.0.1")
		}
		honest := zipOf(t, member, content)
		served, status := mutated(honest, row.Mutation), http.StatusOK
		if row.Mutation == "missing" {
			status = http.StatusNotFound
		}
		fake := serveArchive(t, served, status, nil)
		cache := t.TempDir()
		request := provisionRequest(t, honest, fake, cache, nil)
		install, err := request.Ensure(context.Background())
		var failure Failure
		if row.Outcome == "installed" {
			if err != nil || !strings.HasPrefix(install.Executable, cache) {
				t.Fatalf("%s: %v %+v", row.Name, err, install)
			}
			continue
		}
		if !errors.As(err, &failure) || failure.ID != row.Outcome || (row.What != "" && failure.Values["what"] != row.What) {
			t.Fatalf("%s: %v", row.Name, err)
		}
		entries, _ := os.ReadDir(filepath.Join(cache, "bun"))
		for _, entry := range entries {
			if !strings.HasPrefix(entry.Name(), ".lock-") {
				t.Fatalf("%s: left %s behind", row.Name, entry.Name())
			}
		}
	}
}

// 📥️A second start reuses the install without the network, and concurrent starts download once.
func TestProvisioningDownloadsOnce(t *testing.T) {
	skipWithoutShell(t)
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake := serveArchive(t, archive, http.StatusOK, nil)
	cache := t.TempDir()
	var group sync.WaitGroup
	results := make([]Install, 4)
	for index := range results {
		group.Add(1)
		go func() {
			defer group.Done()
			request := provisionRequest(t, archive, fake, cache, nil)
			request.Root = t.TempDir()
			var err error
			if results[index], err = request.Ensure(context.Background()); err != nil {
				t.Error(err)
			}
		}()
	}
	group.Wait()
	if got := fake.requests.Load(); got != 1 {
		t.Fatalf("expected one download, got %d", got)
	}
	for _, result := range results {
		if result.Executable != results[0].Executable || result.Executable == "" {
			t.Fatalf("starts disagree: %+v", results)
		}
	}
	fake.server.Close()
	again := provisionRequest(t, archive, fake, cache, nil)
	if install, present, err := again.Locate(); !present || err != nil || install.Origin != "user-cache" {
		t.Fatalf("reuse without network: %+v %v %v", install, present, err)
	}
}

// 📥️The tools partition wins over the user cache and nothing is downloaded.
func TestProvisioningPrefersToolsPartition(t *testing.T) {
	skipWithoutShell(t)
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake := serveArchive(t, archive, http.StatusOK, nil)
	request := provisionRequest(t, archive, fake, t.TempDir(), nil)
	row, _ := request.row()
	tools, _, _ := request.StoreDirectories(row)
	if err := os.MkdirAll(tools, 0o755); err != nil {
		t.Fatal(err)
	}
	_ = os.WriteFile(filepath.Join(tools, "bun"), []byte(syntheticRuntime(syntheticVersion)), 0o755)
	_ = os.WriteFile(filepath.Join(tools, receiptName), []byte(`{"archive":"`+row.Sha256+`","executable":"ignored"}`), 0o644)
	install, err := request.Ensure(context.Background())
	if err != nil || install.Origin != "tools-partition" || fake.requests.Load() != 0 {
		t.Fatalf("%+v %v requests=%d", install, err, fake.requests.Load())
	}
	_ = os.WriteFile(filepath.Join(tools, receiptName), []byte(`{"archive":"0000"}`), 0o644)
	if _, present, _ := request.Locate(); present {
		t.Fatal("a receipt of another archive must not be trusted")
	}
}

// 📥️A cancelled download stops promptly, installs nothing and leaves no staging directory.
func TestProvisioningCancellation(t *testing.T) {
	skipWithoutShell(t)
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	started := make(chan struct{}, 1)
	fake := serveArchive(t, nil, 0, func(w http.ResponseWriter, r *http.Request) {
		_, _ = w.Write(archive[:10])
		w.(http.Flusher).Flush()
		started <- struct{}{}
		<-r.Context().Done()
	})
	cache := t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	go func() { <-started; cancel() }()
	begin := time.Now()
	_, err := provisionRequest(t, archive, fake, cache, nil).Ensure(ctx)
	var failure Failure
	if !errors.As(err, &failure) || failure.ID != "cancelled" || time.Since(begin) > 5*time.Second {
		t.Fatalf("%v after %s", err, time.Since(begin))
	}
	entries, _ := os.ReadDir(filepath.Join(cache, "bun"))
	for _, entry := range entries {
		if strings.HasPrefix(entry.Name(), ".prepare-") || len(entry.Name()) == 64 {
			t.Fatalf("left %s behind", entry.Name())
		}
	}
}

// 📥️Mirrors must be https or loopback, and the cache path stays within the path budget.
func TestProvisioningRefusals(t *testing.T) {
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake := serveArchive(t, archive, http.StatusOK, nil)
	request := provisionRequest(t, archive, fake, t.TempDir(), nil)
	request.Env = environment(map[string]string{"SEMIO_RUNTIME_CACHE": t.TempDir(), "SEMIO_RUNTIME_MIRROR": "http://example.com/mirror"})
	var failure Failure
	if _, err := request.Ensure(context.Background()); !errors.As(err, &failure) || failure.ID != "mirrorRefused" {
		t.Fatalf("plain http mirror: %v", err)
	}
	request.Env = environment(map[string]string{"SEMIO_RUNTIME_CACHE": filepath.Join(t.TempDir(), strings.Repeat("d", 260))})
	if _, err := request.Ensure(context.Background()); !errors.As(err, &failure) || failure.ID != "pathTooLong" {
		t.Fatalf("long cache path: %v", err)
	}
	request.Platform = "freebsd"
	if _, _, err := request.Locate(); !errors.As(err, &failure) || failure.ID != "runtimeUnsupported" {
		t.Fatalf("unsupported host: %v", err)
	}
}

// #endregion 📥️Provision
