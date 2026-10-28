// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️runtime-registry reads the schema-declared runtime registry: pinned runtimes with a SHA-256 per operating system and architecture, the implementation entries and the EN/DE texts.

// #endregion 🧲️Header

package launcher

import (
	"bytes"
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"sort"
	"strings"
)

// #region 🗂️Registry

// 📝️Text holds one text per language.
type Text map[string]string

// 📦️Distribution is one pinned archive of a runtime for one operating system, architecture and C library.
type Distribution struct {
	Platform string `json:"platform"`
	Arch     string `json:"arch"`
	Libc     string `json:"libc"`
	Archive  string `json:"archive"`
	Bytes    int64  `json:"bytes"`
	Sha256   string `json:"sha256"`
}

// 🧰️Runtime is one swappable adapter: how to get it and how it starts a script.
type Runtime struct {
	Version    string `json:"version"`
	Executable struct {
		Unix    string `json:"unix"`
		Windows string `json:"windows"`
	} `json:"executable"`
	Probe struct {
		Arguments []string `json:"arguments"`
	} `json:"probe"`
	Download struct {
		Base                  string `json:"base"`
		Format                string `json:"format"`
		Member                string `json:"member"`
		MaximumExtractedBytes int64  `json:"maximumExtractedBytes"`
	} `json:"download"`
	Launch struct {
		Before      []string            `json:"before"`
		Options     map[string][]string `json:"options"`
		Environment map[string]string   `json:"environment"`
	} `json:"launch"`
	Distributions []Distribution `json:"distributions"`
}

// 🚪️Entry is one repository implementation script with the arguments that precede the caller's.
type Entry struct {
	Runtime string   `json:"runtime"`
	Script  string   `json:"script"`
	Prefix  []string `json:"prefix"`
}

// 🧭️Registry is the decoded runtime registry.
type Registry struct {
	Schema    string   `json:"$schema"`
	Contract  int      `json:"contract"`
	Languages []string `json:"languages"`
	Language  struct {
		Protocol string `json:"protocol"`
	} `json:"language"`
	Restart struct {
		Policy string `json:"policy"`
	} `json:"restart"`
	Routing struct {
		Catalogue string `json:"catalogue"`
	} `json:"routing"`
	Selected       string             `json:"selected"`
	Runtimes       map[string]Runtime `json:"runtimes"`
	Implementation struct {
		Version struct {
			File string `json:"file"`
			Key  string `json:"key"`
		} `json:"version"`
		Entries map[string]Entry `json:"entries"`
	} `json:"implementation"`
	Cache struct {
		Environment        string   `json:"environment"`
		MirrorEnvironment  string   `json:"mirrorEnvironment"`
		RuntimeEnvironment string   `json:"runtimeEnvironment"`
		ToolsPartition     []string `json:"toolsPartition"`
		User               []string `json:"user"`
		MaximumPathLength  int      `json:"maximumPathLength"`
	} `json:"cache"`
	Messages map[string]Text `json:"messages"`
}

// ❌️RegistryError names why the registry cannot be used: a Kind (unreadable, malformed, contract, incomplete, selected, runtime, distribution, entry, message) and the Subject it concerns.
type RegistryError struct {
	Path    string
	Kind    string
	Subject string
	Found   int
}

// 🔤️Error joins the path and the reason.
func (e RegistryError) Error() string { return e.Path + ": " + e.Kind + " " + e.Subject }

// 🔢️RequiredMessages are the text identifiers the launcher and the runtime guard of the build implementation print; the registry must carry every one in every language.
var RequiredMessages = []string{"runtimeUnknown", "runtimeUnsupported", "entryUnknown", "entryMissing", "provisioning", "downloading", "waitingForPeer", "ready", "downloadFailed", "integrityFailed", "extractFailed", "probeFailed", "pathTooLong", "cacheUnavailable", "cancelled", "handOffFailed", "domainNeedsInstall", "helpNative", "helpDashboard", "helpOsMcp", "versionRuntimeReady", "versionRuntimeAbsent", "versionImplementation", "versionImplementationUnknown", "languageRefusal", "languageUnknownFlag", "languageUnknownVariable", "mirrorRefused", "redirectRefused", "extractTooLarge", "unexpected", "guardTest", "guardMissing", "guardLoop"}

var (
	shaPattern          = regexp.MustCompile(`^[a-f0-9]{64}$`)
	versionPattern      = regexp.MustCompile(`^[0-9]+\.[0-9]+\.[0-9]+$`)
	basePattern         = regexp.MustCompile(`^https://[a-z0-9.-]+/[A-Za-z0-9._/-]+$`)
	placeholderPattern  = regexp.MustCompile(`\{[A-Za-z]+\}`)
	unknownFieldPattern = regexp.MustCompile(`unknown field "([^"]*)"`)
)

// 📖️LoadRegistry reads and validates the registry of the repository at root.
func LoadRegistry(root string) (*Registry, error) {
	path := filepath.Join(root, RegistryPath)
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, RegistryError{Path: path, Kind: "unreadable"}
	}
	return DecodeRegistry(path, raw)
}

// 🔬️DecodeRegistry reads the contract of a registry document before anything else, so a registry of another contract is refused as such (with the command that rebuilds the launcher) and never as a malformed field of a shape this launcher does not know, then decodes it strictly and validates its constraints.
func DecodeRegistry(path string, raw []byte) (*Registry, error) {
	var header struct {
		Contract *int `json:"contract"`
	}
	if json.Unmarshal(raw, &header) == nil && header.Contract != nil && *header.Contract != Contract {
		return nil, RegistryError{Path: path, Kind: "contract", Found: *header.Contract}
	}
	decoder := json.NewDecoder(bytes.NewReader(raw))
	decoder.DisallowUnknownFields()
	registry := &Registry{}
	if err := decoder.Decode(registry); err != nil {
		return nil, RegistryError{Path: path, Kind: "malformed", Subject: unknownField(err)}
	}
	if registry.Contract != Contract {
		return nil, RegistryError{Path: path, Kind: "contract", Found: registry.Contract}
	}
	if kind, subject := registry.problem(); kind != "" {
		return nil, RegistryError{Path: path, Kind: kind, Subject: subject}
	}
	return registry, nil
}

func unknownField(err error) string {
	if found := unknownFieldPattern.FindStringSubmatch(err.Error()); found != nil {
		return found[1]
	}
	return "-"
}

func (r *Registry) problem() (kind, subject string) {
	if len(r.Languages) < 2 || r.Language.Protocol == "" || r.Restart.Policy == "" || r.Routing.Catalogue == "" || r.Cache.MaximumPathLength != 256 || len(r.Cache.ToolsPartition) == 0 || len(r.Cache.User) == 0 {
		return "incomplete", "-"
	}
	if _, ok := r.Runtimes[r.Selected]; !ok {
		return "selected", r.Selected
	}
	for _, name := range sortedKeys(r.Runtimes) {
		runtimeRow := r.Runtimes[name]
		if !versionPattern.MatchString(runtimeRow.Version) || len(runtimeRow.Probe.Arguments) == 0 || runtimeRow.Download.Format != "zip" || !basePattern.MatchString(runtimeRow.Download.Base) || runtimeRow.Download.MaximumExtractedBytes < 1<<20 || len(runtimeRow.Distributions) == 0 {
			return "runtime", name
		}
		for _, row := range runtimeRow.Distributions {
			if !shaPattern.MatchString(row.Sha256) || row.Bytes < 1 || !strings.HasSuffix(row.Archive, ".zip") || (row.Libc != "any" && row.Libc != "gnu" && row.Libc != "musl") {
				return "distribution", name + "/" + row.Archive
			}
		}
	}
	if len(r.Implementation.Entries) == 0 || r.Implementation.Version.File == "" || r.Implementation.Version.Key == "" {
		return "incomplete", "-"
	}
	for _, name := range sortedKeys(r.Implementation.Entries) {
		entry := r.Implementation.Entries[name]
		if _, ok := r.Runtimes[r.entryRuntime(entry)]; !ok || entry.Script == "" {
			return "entry", name
		}
	}
	return r.messageProblem()
}

func (r *Registry) messageProblem() (kind, subject string) {
	for _, id := range RequiredMessages {
		text, ok := r.Messages[id]
		if !ok {
			return "message", id
		}
		reference := placeholderSet(text[r.Languages[0]])
		for _, language := range r.Languages {
			if text[language] == "" || placeholderSet(text[language]) != reference {
				return "message", id
			}
		}
	}
	return "", ""
}

func sortedKeys[V any](table map[string]V) []string {
	keys := make([]string, 0, len(table))
	for key := range table {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	return keys
}

func placeholderSet(text string) string {
	found := placeholderPattern.FindAllString(text, -1)
	sort.Strings(found)
	return strings.Join(found, ",")
}

func (r *Registry) entryRuntime(entry Entry) string {
	if entry.Runtime != "" {
		return entry.Runtime
	}
	return r.Selected
}

// ❓️UnknownError names a runtime or entry that the registry does not declare.
type UnknownError struct {
	Name     string
	Declared []string
}

// 🔤️Error names the unknown identifier.
func (e UnknownError) Error() string {
	return e.Name + " is not declared (declared: " + strings.Join(e.Declared, ", ") + ")"
}

// 🎛️Pick answers the runtime that serves an entry: the SEMIO_RUNTIME choice, else the entry's own, else the selected one.
func (r *Registry) Pick(entry Entry, env Environment) (string, Runtime, error) {
	name := env(r.Cache.RuntimeEnvironment)
	if name == "" {
		name = r.entryRuntime(entry)
	}
	picked, ok := r.Runtimes[name]
	if !ok {
		return name, Runtime{}, UnknownError{Name: name, Declared: sortedKeys(r.Runtimes)}
	}
	return name, picked, nil
}

// 🎯️Distribution answers the pinned archive for a host.
func (rt *Runtime) Distribution(platform, arch string, musl bool) (Distribution, bool) {
	for _, row := range rt.Distributions {
		libc := "gnu"
		if musl {
			libc = "musl"
		}
		if row.Platform == platform && row.Arch == arch && (row.Libc == "any" || row.Libc == libc) {
			return row, true
		}
	}
	return Distribution{}, false
}

// 🖥️HostTarget names the host in registry terms: operating system, architecture and whether its C library is musl.
func HostTarget() (platform, arch string, musl bool) {
	platform = map[string]string{"windows": "win32"}[runtime.GOOS]
	if platform == "" {
		platform = runtime.GOOS
	}
	arch = map[string]string{"amd64": "x64"}[runtime.GOARCH]
	if arch == "" {
		arch = runtime.GOARCH
	}
	if runtime.GOOS == "linux" {
		found, _ := filepath.Glob("/lib/ld-musl-*.so.1")
		musl = len(found) > 0
	}
	return platform, arch, musl
}

// 📛️ExecutableName is the file name of the runtime executable on the host.
func (rt *Runtime) ExecutableName() string {
	if runtime.GOOS == "windows" {
		return rt.Executable.Windows
	}
	return rt.Executable.Unix
}

// #endregion 🗂️Registry

// 📥️InstallCommand is the exact command that rebuilds and reinstalls the launcher of the checkout whose registry is at registryPath.
func InstallCommand(registryPath string) string {
	directory := filepath.Join(filepath.Dir(filepath.Dir(registryPath)), "📥️install")
	if runtime.GOOS == "windows" {
		return `powershell -NoProfile -ExecutionPolicy Bypass -File "` + filepath.Join(directory, "🔵️.ps1") + `"`
	}
	return `bash "` + filepath.Join(directory, "🐚️.sh") + `"`
}
