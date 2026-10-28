// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔖️version-contract proves --version and --help in both languages, with and without a provisioned runtime and a repository.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🔖️Version names launcher, runtime and implementation in the chosen language and never fetches the runtime.
func TestVersionReport(t *testing.T) {
	for _, language := range []string{"en", "de"} {
		for _, seed := range []bool{false, true} {
			setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": language}, seed)
			if code := setup.host.Version(nil); code != 0 || setup.log.calls != 0 {
				t.Fatalf("code=%d", code)
			}
			text := lines(setup.log.out)
			launcherLine := map[string]string{"en": "repo launcher " + Version, "de": "repo Launcher " + Version}[language]
			bun := loadRegistry(t).Runtimes["bun"].Version
			ready := map[string]string{"en": "ready, " + setup.exe, "de": "bereit, " + setup.exe}[language]
			absent := map[string]string{"en": "not provisioned yet", "de": "noch nicht bereitgestellt"}[language]
			implementation := map[string]string{"en": "implementation ", "de": "Implementierung "}[language]
			if !strings.Contains(text, launcherLine) || !strings.Contains(text, "bun "+bun) || !strings.Contains(text, implementation) {
				t.Fatalf("%s: %q", language, text)
			}
			if seed && !strings.Contains(text, ready) || !seed && !strings.Contains(text, absent) {
				t.Fatalf("%s seed=%v: %q", language, seed, text)
			}
		}
	}
}

// 🔖️Without a language the version report refuses in every language; without a repository it still names the launcher.
func TestVersionWithoutLanguageOrRepository(t *testing.T) {
	setup := seededHost(t, map[string]string{}, false)
	if code := setup.host.Version(nil); code != 2 || !strings.Contains(lines(setup.log.err), "Keine Sprache") {
		t.Fatalf("code=%d %q", code, setup.log.err)
	}
	empty := t.TempDir()
	host, log := recordingHost(t, map[string]string{"SEMIO_WORKSPACE_ROOT": empty})
	if code := host.Version(nil); code != 0 || !strings.Contains(lines(log.out), "repo Launcher "+Version) || !strings.Contains(lines(log.out), "no repository here") {
		t.Fatalf("code=%d %q", code, log.out)
	}
	if code := host.Main(nil, []string{"run"}); code != 2 || !strings.Contains(lines(log.err), "Kein Repository gefunden") {
		t.Fatalf("code=%d %q", code, log.err)
	}
	_ = filepath.Join
	_ = os.Getenv
}

// 🔖️Help is the help of the one build catalogue: the launcher starts the build front door for it instead of printing a list of its own; a domain verb points at the full command.
func TestHelpAndDomainVerb(t *testing.T) {
	script := []string{"🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏗️build", "🚪️door", "📜️script.ts"}
	for _, arguments := range [][]string{nil, {"--help"}, {"-h"}, {"--help", "--lang=de"}} {
		setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "en"}, true)
		want := append([]string{setup.exe, filepath.Join(append([]string{setup.root}, script...)...), "repo"}, arguments...)
		if code := setup.host.Main(context.Background(), arguments); code != 0 || strings.Join(setup.log.argv, "\x00") != strings.Join(want, "\x00") || len(setup.log.out) != 0 {
			t.Fatalf("%q: code=%d argv=%q out=%q", arguments, code, setup.log.argv, setup.log.out)
		}
	}
	setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "en"}, true)
	if code := setup.host.Main(context.Background(), []string{"ticket", "list"}); code != 2 || !strings.Contains(lines(setup.log.err), "repo run repo-mcp:install") {
		t.Fatalf("%q", setup.log.err)
	}
}

// 🔖️Say answers a registry text in the selected language and nothing when no language is selected.
func TestSayAnswersTheSelectedLanguage(t *testing.T) {
	setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "de"}, true)
	if text, ok := setup.host.Say(setup.root, nil, "helpNative", nil); !ok || !strings.HasPrefix(text, "Native Befehle") {
		t.Fatalf("%v %q", ok, text)
	}
	if text, ok := setup.host.Say(setup.root, []string{"--lang=en"}, "helpNative", nil); !ok || !strings.HasPrefix(text, "Native commands") {
		t.Fatalf("%v %q", ok, text)
	}
}

// #endregion 🔖️Version
