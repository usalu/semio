// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧭️workspace-root resolves the repository root from a start directory: the nearest ancestor whose package.json declares workspaces next to a 📋️project.json.

// #endregion 🧲️Header

package launcher

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// #region 🧭️Root

// 🔢️MaximumRootDepth bounds the upward search, as the TypeScript and shell resolvers do.
const MaximumRootDepth = 32

// 🧱️RootMarkers are the file names that pair up at a repository root.
var RootMarkers = [2]string{"📋️project.json", "package.json"}

// 🚫️RootNotFoundError reports that no ancestor carries the paired markers.
type RootNotFoundError struct{ Start string }

// 🔤️Error names the start directory.
func (e RootNotFoundError) Error() string { return "workspace root not found from " + e.Start }

// ⚠️RootInvalidError reports a candidate root whose package.json is unreadable or malformed (Kind).
type RootInvalidError struct{ File, Kind string }

// 🔤️Error names the file and the reason.
func (e RootInvalidError) Error() string { return e.File + ": " + e.Kind }

// 📍️StartDirectory picks where the search begins: SEMIO_WORKSPACE_ROOT, then REPO_ROOT, then the working directory.
func StartDirectory(env Environment, getwd func() (string, error)) (string, error) {
	for _, name := range []string{"SEMIO_WORKSPACE_ROOT", "REPO_ROOT"} {
		if value := strings.TrimSpace(env(name)); value != "" {
			return filepath.Abs(value)
		}
	}
	return getwd()
}

func isPlainFile(path string) bool {
	info, err := os.Lstat(path)
	return err == nil && info.Mode().IsRegular()
}

func declaresWorkspaces(file string) (bool, error) {
	raw, err := os.ReadFile(file)
	if err != nil {
		return false, RootInvalidError{File: file, Kind: "unreadable"}
	}
	var document struct {
		Workspaces json.RawMessage `json:"workspaces"`
	}
	if err := json.Unmarshal(raw, &document); err != nil {
		return false, RootInvalidError{File: file, Kind: "malformed"}
	}
	text := strings.TrimSpace(string(document.Workspaces))
	return strings.HasPrefix(text, "[") || strings.HasPrefix(text, "{"), nil
}

// 🔍️ResolveRoot walks upwards from start and answers the first directory with paired markers and a workspaces declaration.
func ResolveRoot(start string) (string, error) {
	directory, err := filepath.Abs(start)
	if err != nil {
		return "", err
	}
	for depth := 0; depth < MaximumRootDepth; depth++ {
		if isPlainFile(filepath.Join(directory, RootMarkers[0])) && isPlainFile(filepath.Join(directory, RootMarkers[1])) {
			declared, err := declaresWorkspaces(filepath.Join(directory, RootMarkers[1]))
			if err != nil {
				return "", err
			}
			if declared {
				return directory, nil
			}
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			break
		}
		directory = parent
	}
	return "", RootNotFoundError{Start: start}
}

// 🧱️IsRootNotFound reports whether err is a missing root.
func IsRootNotFound(err error) bool {
	var missing RootNotFoundError
	return errors.As(err, &missing)
}

// #endregion 🧭️Root
