// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪟️runtime-outside-partition proves that on Windows the launcher runs the runtime from the user cache, never from the tools partition: a running image pins its directory, and the partition is attached and detached by renaming it. Order, adoption of an installed copy, concurrent adoption, a refused adoption, and (on a Windows host) the pin itself.

// #endregion 🧲️Header

package launcher

import (
	"bytes"
	"context"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"testing"
	"time"
)

func windowsRequest(t *testing.T, cache string) (ProvisionRequest, Distribution, *synthetic) {
	t.Helper()
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake := serveArchive(t, archive, http.StatusOK, nil)
	request := provisionRequest(t, archive, fake, cache, nil)
	request.Platform = "win32"
	request.Runtime.Distributions = []Distribution{{Platform: "win32", Arch: "x64", Libc: "any", Archive: request.Runtime.Distributions[0].Archive, Bytes: request.Runtime.Distributions[0].Bytes, Sha256: request.Runtime.Distributions[0].Sha256}}
	request.Arch = "x64"
	row, _ := request.row()
	return request, row, fake
}

func install(t *testing.T, request ProvisionRequest, directory string, row Distribution, content string) {
	t.Helper()
	name := request.Runtime.ExecutableName()
	if err := os.MkdirAll(directory, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(directory, name), []byte(content), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(directory, receiptName), []byte(`{"archive":"`+row.Sha256+`"}`), 0o644); err != nil {
		t.Fatal(err)
	}
}

// 🪟️Windows looks in the user cache first; every other platform keeps the tools partition first.
func TestWindowsPrefersTheUserCache(t *testing.T) {
	request, row, _ := windowsRequest(t, t.TempDir())
	tools, user, _ := request.StoreDirectories(row)
	install(t, request, tools, row, "tools")
	install(t, request, user, row, "user")
	found, present, err := request.Locate()
	if err != nil || !present || found.Origin != "user-cache" {
		t.Fatalf("%+v %v %v", found, present, err)
	}
	request.Platform = "linux"
	request.Runtime.Distributions[0].Platform = "linux"
	if found, present, _ = request.Locate(); !present || found.Origin != "tools-partition" {
		t.Fatalf("other platforms keep the tools partition first: %+v", found)
	}
}

// 🪟️A runtime that only the tools partition holds is copied into the user cache once, without a download, and the copy is what runs.
func TestWindowsAdoptsTheToolsPartitionRuntime(t *testing.T) {
	request, row, fake := windowsRequest(t, t.TempDir())
	tools, user, _ := request.StoreDirectories(row)
	install(t, request, tools, row, "payload "+strings.Repeat("x", 1<<16))
	name := request.Runtime.ExecutableName()
	first, err := request.Ensure(context.Background())
	if err != nil || first.Origin != "user-cache" || first.Directory != user || first.Executable != filepath.Join(user, name) || fake.requests.Load() != 0 {
		t.Fatalf("%+v %v requests=%d", first, err, fake.requests.Load())
	}
	want, _ := os.ReadFile(filepath.Join(tools, name))
	got, _ := os.ReadFile(first.Executable)
	if !bytes.Equal(want, got) || !verifyInstalled(user, name, row) || !verifyInstalled(tools, name, row) {
		t.Fatal("the copy must be byte-equal and both installs valid")
	}
	if info, err := os.Stat(first.Executable); err != nil || (runtime.GOOS != "windows" && info.Mode().Perm()&0o100 == 0) {
		t.Fatalf("copy must stay executable: %v %v", info, err)
	}
	before, _ := os.Stat(first.Executable)
	time.Sleep(20 * time.Millisecond)
	second, err := request.Ensure(context.Background())
	after, _ := os.Stat(first.Executable)
	if err != nil || second.Origin != "user-cache" || !before.ModTime().Equal(after.ModTime()) {
		t.Fatalf("a second start must reuse the copy: %+v %v", second, err)
	}
}

// 🪟️Peers that adopt at the same moment end with one valid copy, no staging leftovers and no refusal.
func TestWindowsAdoptionIsRaceFree(t *testing.T) {
	request, row, fake := windowsRequest(t, t.TempDir())
	tools, user, _ := request.StoreDirectories(row)
	install(t, request, tools, row, strings.Repeat("y", 1<<20))
	var group sync.WaitGroup
	failures := make(chan string, 16)
	for i := 0; i < 16; i++ {
		group.Add(1)
		go func() {
			defer group.Done()
			if found, err := request.Ensure(context.Background()); err != nil || found.Origin != "user-cache" {
				failures <- "adopt: " + found.Origin + " " + errString(err)
			}
		}()
	}
	group.Wait()
	close(failures)
	for failure := range failures {
		t.Fatal(failure)
	}
	entries, _ := os.ReadDir(filepath.Dir(user))
	for _, entry := range entries {
		if strings.HasPrefix(entry.Name(), ".prepare-") {
			t.Fatalf("staging leftover %s", entry.Name())
		}
	}
	if !verifyInstalled(user, request.Runtime.ExecutableName(), row) || fake.requests.Load() != 0 {
		t.Fatalf("one valid copy and no download expected, requests=%d", fake.requests.Load())
	}
}

func errString(err error) string {
	if err == nil {
		return "<nil>"
	}
	return err.Error()
}

// 🪟️A refused adoption (the cache location cannot be created) never fails the start: the tools-partition copy still runs.
func TestWindowsAdoptionFailureKeepsTheToolsCopy(t *testing.T) {
	blocker := filepath.Join(t.TempDir(), "file")
	if err := os.WriteFile(blocker, []byte("not a directory"), 0o644); err != nil {
		t.Fatal(err)
	}
	request, row, _ := windowsRequest(t, filepath.Join(blocker, "cache"))
	tools, _, _ := request.StoreDirectories(row)
	install(t, request, tools, row, "payload")
	found, err := request.Ensure(context.Background())
	if err != nil || found.Origin != "tools-partition" {
		t.Fatalf("%+v %v", found, err)
	}
}

func TestPinHelperSleeps(t *testing.T) {
	if os.Getenv("SEMIO_TEST_PIN_HELPER") == "" {
		t.Skip("helper of TestWindowsRunningImagePinsItsDirectory")
	}
	time.Sleep(30 * time.Second)
}

// 🪟️On a Windows host a running image pins the directory it lives in, so the tools partition cannot be detached while a command runs from it; the adopted copy in the user cache leaves the partition free to move (skipped elsewhere).
func TestWindowsRunningImagePinsItsDirectory(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("the pin is a Windows behaviour")
	}
	self, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	image, err := os.ReadFile(self)
	if err != nil {
		t.Fatal(err)
	}
	request, row, _ := windowsRequest(t, t.TempDir())
	tools, _, _ := request.StoreDirectories(row)
	install(t, request, tools, row, string(image))
	start := func(executable string) *exec.Cmd {
		command := exec.Command(executable, "-test.run=^TestPinHelperSleeps$")
		command.Env = append(os.Environ(), "SEMIO_TEST_PIN_HELPER=1")
		if err := command.Start(); err != nil {
			t.Fatal(err)
		}
		time.Sleep(500 * time.Millisecond)
		return command
	}
	pinned := start(filepath.Join(tools, request.Runtime.ExecutableName()))
	moved := tools + "-moved"
	if err := os.Rename(tools, moved); err == nil {
		_ = os.Rename(moved, tools)
		_ = pinned.Process.Kill()
		t.Fatal("renaming a directory that holds a running image must fail on Windows")
	}
	_ = pinned.Process.Kill()
	_, _ = pinned.Process.Wait()
	adopted, err := request.Ensure(context.Background())
	if err != nil || adopted.Origin != "user-cache" {
		t.Fatalf("%+v %v", adopted, err)
	}
	running := start(adopted.Executable)
	defer func() { _ = running.Process.Kill(); _, _ = running.Process.Wait() }()
	if err := os.Rename(tools, moved); err != nil {
		t.Fatalf("the tools partition must be free to move while the adopted copy runs: %v", err)
	}
}
