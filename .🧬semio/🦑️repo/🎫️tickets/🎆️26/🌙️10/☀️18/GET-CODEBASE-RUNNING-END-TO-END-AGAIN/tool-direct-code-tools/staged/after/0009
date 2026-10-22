// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎨️formatter carries the formatter plans, availability probing and the fallback text formatter.

// #endregion 🧲️Header

package workspace

import (
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// #region 🎨️Formatter

var FormatterBinaryLookup = exec.LookPath

var FormatterCommandRun = runFormatterCommand

// 📋️formatterPlan holds the data fields for a formatterPlan record.
type formatterPlan struct {
	Binary      string
	Args        []string
	requirement []string
}

// 🔸️runFormatterCommand holds the data fields for a runFormatterCommand record.
func runFormatterCommand(binary string, args []string, workDir string) error {
	commandPath := binary
	if strings.Contains(binary, "/") || strings.Contains(binary, "\\") {
		commandPath = filepath.Join(workDir, binary)
	}
	cmd := ToolCommand(commandPath, args...)
	cmd.Dir = workDir
	return cmd.Run()
}

// 🛡️ IsIdeConfigurationPath identifies IDE-owned configuration from dashboard authority; the `.cursor` directory has no authority at all, not even for plans.
func IsIdeConfigurationPath(path string) bool {
	parts := strings.Split(strings.ToLower(NormalizePath(path)), "/")
	for _, part := range parts {
		switch part {
		case ".cursor", ".vscode", ".vscode-insiders", ".idea", ".fleet", ".zed", ".vs", ".run", ".nova", ".settings", ".metadata", ".windsurf", ".kiro", ".atom", ".brackets":
			return true
		}
	}
	if strings.Contains("/"+strings.Join(parts, "/")+"/", "/.github/hooks/") {
		return true
	}
	name := parts[len(parts)-1]
	for _, file := range []string{".project", ".classpath", ".vsconfig", ".cursorrules", ".windsurfrules", "cmakesettings.json"} {
		if name == file {
			return true
		}
	}
	for _, suffix := range []string{".sln", ".slnx", ".code-workspace", ".iml", ".ipr", ".iws", ".suo", ".user", ".userosscache", ".sln.docstates", ".sublime-project", ".sublime-workspace", ".sublime-build"} {
		if strings.HasSuffix(name, suffix) {
			return true
		}
	}
	if len(parts) > 1 && parts[len(parts)-2] == ".devcontainer" && name == "editor-overlay.json" {
		return true
	}
	return false
}

var ideInventoryBoundaryDirectories = []string{".git", "node_modules", ".venv"}

var ideInventoryExcludedRoots = []string{".cache", ".nx", ".pytest_cache", ".ruff_cache", "storybook-static", "test-results", ".🧬semio/🦑️repo/🎫️tickets", ".🧬semio/🦑️repo/⚡️cache", ".🧬semio/🦑️repo/🧹️tmp", ".repo/cache", "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/dist", "✏️s/🔌️plugins/🏭️process/dist", "✏️s/🔌️plugins/🪵️sourcing/dist"}

// 🧹️ IdeConfigurationPaths visits active physical owners without entering generated or historical files.
func IdeConfigurationPaths(root string, scope string) ([]string, error) {
	paths := []string{}
	base := filepath.Join(root, filepath.FromSlash(scope))
	err := filepath.WalkDir(base, func(path string, entry fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.Type()&os.ModeSymlink != 0 {
			return nil
		}
		relative, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		relative = filepath.ToSlash(relative)
		if entry.IsDir() {
			for _, name := range ideInventoryBoundaryDirectories {
				if entry.Name() == name {
					return filepath.SkipDir
				}
			}
			for _, path := range ideInventoryExcludedRoots {
				if relative == path {
					return filepath.SkipDir
				}
			}
		}
		if IsIdeConfigurationPath(relative) {
			paths = append(paths, relative)
			if entry.IsDir() {
				return filepath.SkipDir
			}
		}
		return nil
	})
	return paths, err
}

// 🏗️ DotnetFormatterProjects finds the nearest project owner within the repository.
func DotnetFormatterProjects(root string, relPath string) []string {
	projects := []string{}
	absolute, err := filepath.Abs(root)
	if err != nil {
		return projects
	}
	source := filepath.Join(absolute, filepath.FromSlash(relPath))
	if filepath.IsAbs(relPath) {
		source = filepath.Clean(relPath)
	}
	relative, err := filepath.Rel(absolute, source)
	if err != nil || relative == ".." || strings.HasPrefix(relative, ".."+string(filepath.Separator)) {
		return projects
	}
	for directory := filepath.Dir(source); ; directory = filepath.Dir(directory) {
		identity, err := os.Lstat(directory)
		if err != nil || !identity.IsDir() || identity.Mode()&os.ModeSymlink != 0 {
			return projects
		}
		entries, err := os.ReadDir(directory)
		if err != nil {
			return projects
		}
		for _, entry := range entries {
			if entry.Type().IsRegular() && strings.EqualFold(filepath.Ext(entry.Name()), ".csproj") {
				project, err := filepath.Rel(absolute, filepath.Join(directory, entry.Name()))
				if err == nil {
					projects = append(projects, filepath.ToSlash(project))
				}
			}
		}
		if len(projects) > 0 || directory == absolute {
			return projects
		}
	}
}

// 🗣️formatterPlansForLanguage holds the data fields for a formatterPlansForLanguage record.
func FormatterPlansForLanguage(languageName string, relPath string) []formatterPlan {
	prettierBinary := filepath.Join("node_modules", ".bin", "prettier")
	prettierPlan := formatterPlan{
		Binary:      prettierBinary,
		Args:        []string{"--write", "--ignore-unknown", relPath},
		requirement: []string{prettierBinary, ".prettierrc.json"},
	}
	switch languageName {
	case "typescript", "markdown", "json", "yaml", "graphql", "sql", "toml":
		return []formatterPlan{prettierPlan}
	case "go":
		return []formatterPlan{
			{Binary: "gofmt", Args: []string{"-w", relPath}, requirement: []string{"go.work"}},
		}
	case "python":
		return []formatterPlan{
			{Binary: "uv", Args: []string{"run", "--group", "dev", "ruff", "format", relPath}, requirement: []string{"pyproject.toml"}},
			{Binary: "uv", Args: []string{"run", "--group", "dev", "black", relPath}, requirement: []string{"pyproject.toml"}},
			prettierPlan,
		}
	case "csharp":
		plans := []formatterPlan{}
		for _, project := range DotnetFormatterProjects(GetRootDir(), relPath) {
			plans = append(plans, formatterPlan{Binary: "dotnet", Args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
		}
		return append(plans, prettierPlan)
	case "rust":
		return []formatterPlan{
			{Binary: "rustfmt", Args: []string{relPath}, requirement: []string{"Cargo.toml"}},
			prettierPlan,
		}
	case "ruby":
		return []formatterPlan{
			{Binary: "rubocop", Args: []string{"-A", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	case "shell":
		return []formatterPlan{
			{Binary: "shfmt", Args: []string{"-w", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	default:
		return []formatterPlan{prettierPlan}
	}
}

// 🔺️isFormatterPlanAvailable holds the data fields for a isFormatterPlanAvailable record.
func IsFormatterPlanAvailable(plan formatterPlan, workDir string) bool {
	if strings.Contains(plan.Binary, "/") || strings.Contains(plan.Binary, "\\") {
		if !FileExists(filepath.Join(workDir, plan.Binary)) {
			return false
		}
	} else {
		if _, err := FormatterBinaryLookup(plan.Binary); err != nil {
			return false
		}
	}
	for _, rel := range plan.requirement {
		if !FileExists(filepath.Join(workDir, rel)) {
			return false
		}
	}
	return true
}

// 🔻️fallbackFormatText holds the data fields for a fallbackFormatText record.
func FallbackFormatText(content string) string {
	lines := strings.Split(content, "\n")
	for i, line := range lines {
		lines[i] = strings.TrimRight(line, " \t")
	}
	formatted := strings.Join(lines, "\n")
	if formatted == "" {
		return formatted
	}
	if !strings.HasSuffix(formatted, "\n") {
		formatted += "\n"
	}
	return formatted
}

// #endregion 🎨️Formatter
