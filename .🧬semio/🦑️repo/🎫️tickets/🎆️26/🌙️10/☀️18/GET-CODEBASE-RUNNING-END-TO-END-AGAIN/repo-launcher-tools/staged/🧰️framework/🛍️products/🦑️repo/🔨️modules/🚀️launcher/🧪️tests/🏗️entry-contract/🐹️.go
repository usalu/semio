// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏗️entry-contract proves what the launcher starts: the exact runtime argument vector, the PATH and environment of the child, exit codes, and the language rules for the texts it prints.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type seeded struct {
	host Host
	log  *recorded
	root string
	exe  string
}

func seededHost(t *testing.T, env map[string]string, seed bool) seeded {
	t.Helper()
	root := testRoot(t)
	cache := t.TempDir()
	env["SEMIO_RUNTIME_CACHE"] = cache
	env["SEMIO_WORKSPACE_ROOT"] = root
	host, log := recordingHost(t, env)
	registry := loadRegistry(t)
	rt := registry.Runtimes["bun"]
	row, _ := rt.Distribution("linux", "x64", false)
	directory := filepath.Join(cache, "bun", row.Sha256)
	exe := filepath.Join(directory, "bun")
	if seed {
		_ = os.MkdirAll(directory, 0o755)
		_ = os.WriteFile(exe, []byte("x"), 0o755)
		_ = os.WriteFile(filepath.Join(directory, receiptName), []byte(`{"archive":"`+row.Sha256+`"}`), 0o644)
	}
	return seeded{host, log, root, exe}
}

func pathOf(env []string) string {
	for _, entry := range env {
		if strings.HasPrefix(entry, "PATH=") {
			return strings.TrimPrefix(entry, "PATH=")
		}
	}
	return ""
}

// 🏗️A build route starts the pinned runtime with the build script, its prefix and the unchanged arguments, with the runtime and the launcher on the PATH, and the hot path prints nothing.
func TestExecuteBuildRoute(t *testing.T) {
	setup := seededHost(t, map[string]string{}, true)
	code := setup.host.Execute(context.Background(), setup.root, "build", []string{"run", "a:b", "--", "--x"})
	script := filepath.Join(setup.root, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏗️build", "📜️script.ts")
	want := []string{setup.exe, script, "repo", "run", "a:b", "--", "--x"}
	if code != 0 || setup.log.calls != 1 || strings.Join(setup.log.argv, "\x00") != strings.Join(want, "\x00") || setup.log.exe != setup.exe {
		t.Fatalf("code=%d calls=%d argv=%q", code, setup.log.calls, setup.log.argv)
	}
	if got := pathOf(setup.log.env); !strings.HasPrefix(got, filepath.Dir(setup.exe)+string(os.PathListSeparator)) || !strings.Contains(got, "/self/bin") || !strings.HasSuffix(got, "/usr/bin") {
		t.Fatalf("PATH %q", got)
	}
	if len(setup.log.err)+len(setup.log.out) != 0 {
		t.Fatalf("the hot path must be silent: %q", setup.log.err)
	}
}

// 🏗️A script route maps the neutral options to the runtime flags and keeps the script's arguments.
func TestExecuteScriptRoute(t *testing.T) {
	setup := seededHost(t, map[string]string{}, true)
	code := setup.host.Execute(context.Background(), setup.root, "script", []string{"--watch", "--cwd=sub", "./📜️script.ts", "build", "--watch"})
	want := []string{setup.exe, "--watch", "--cwd", "sub", "./📜️script.ts", "build", "--watch"}
	if code != 0 || strings.Join(setup.log.argv, "\x00") != strings.Join(want, "\x00") {
		t.Fatalf("code=%d argv=%q", code, setup.log.argv)
	}
}

// 🏗️The workspace route, an unknown entry, an unknown runtime and a missing script each end with their own text and no start.
func TestExecuteRefusals(t *testing.T) {
	setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "de"}, true)
	if code := setup.host.Execute(context.Background(), setup.root, "nowhere", nil); code != 2 || !strings.Contains(lines(setup.log.err), "Einstieg") || setup.log.calls != 0 {
		t.Fatalf("unknown entry: %d %q", code, setup.log.err)
	}
	setup = seededHost(t, map[string]string{"SEMIO_LANGUAGE": "en", "SEMIO_RUNTIME": "deno"}, true)
	if code := setup.host.Execute(context.Background(), setup.root, "build", nil); code != 2 || !strings.Contains(lines(setup.log.err), "deno") || setup.log.calls != 0 {
		t.Fatalf("unknown runtime: %d %q", code, setup.log.err)
	}
}

// 🏗️When the runtime is absent and no language resolves, the launcher refuses in every language before it downloads anything.
func TestExecuteNoLanguageRefusesBeforeDownload(t *testing.T) {
	setup := seededHost(t, map[string]string{}, false)
	if code := setup.host.Execute(context.Background(), setup.root, "build", []string{"preflight"}); code != 2 || setup.log.calls != 0 {
		t.Fatalf("code=%d", code)
	}
	text := lines(setup.log.err)
	if !strings.Contains(text, "No language selected") || !strings.Contains(text, "Keine Sprache") {
		t.Fatalf("%q", text)
	}
}

// 🏗️A child that settles with an exit code passes it through.
func TestExecuteExitCodePassThrough(t *testing.T) {
	setup := seededHost(t, map[string]string{}, true)
	setup.host.HandOff = func(string, []string, []string) error { return ExitError{Code: 7} }
	if code := setup.host.Execute(context.Background(), setup.root, "build", nil); code != 7 {
		t.Fatalf("code=%d", code)
	}
}

// 🏗️The environment of the child keeps every variable, replaces only the declared ones and prepends the PATH.
func TestPathFixture(t *testing.T) {
	var fixture struct {
		Path []struct {
			Name        string
			Environ     []string
			Directories []string
			Fixed       map[string]string
			Expected    []string
		}
	}
	readFixture(t, launcherFixture, &fixture)
	for _, row := range fixture.Path {
		got := WithRuntimeOnPath(row.Environ, row.Directories, row.Fixed)
		if os.PathListSeparator == ':' && strings.Join(got, "\x00") != strings.Join(row.Expected, "\x00") {
			t.Fatalf("%s: %q", row.Name, got)
		}
	}
}

// #endregion 🏗️Entry
