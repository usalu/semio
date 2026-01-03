// repo/main.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/bmatcuk/doublestar/v4"
	"github.com/spf13/cobra"
	"gopkg.in/yaml.v3"
)

// #region Types

type ScopeKind string

const (
	ScopeRepo       ScopeKind = "repo"
	ScopeProject    ScopeKind = "project"
	ScopeFolder     ScopeKind = "folder"
	ScopeFile       ScopeKind = "file"
	ScopeSection    ScopeKind = "section"
	ScopeDefinition ScopeKind = "definition"
)

type Scope struct {
	Raw            string    `json:"raw"`
	Kind           ScopeKind `json:"kind"`
	ProjectName    string    `json:"projectName,omitempty"`
	FilePath       string    `json:"filePath,omitempty"`
	SectionPath    []string  `json:"sectionPath,omitempty"`
	DefinitionName string    `json:"definitionName,omitempty"`
}

type ViolationPriority string

const (
	PriorityHigh   ViolationPriority = "high"
	PriorityMedium ViolationPriority = "medium"
	PriorityLow    ViolationPriority = "low"
)

type TextEdit struct {
	Start   int    `json:"start"`
	End     int    `json:"end"`
	NewText string `json:"newText"`
}

type Fix struct {
	Description string               `json:"description"`
	Edits       map[string][]TextEdit `json:"edits"`
}

type Violation struct {
	ID      string        `json:"id"`
	Summary string        `json:"summary"`
	Kind    ViolationKind `json:"kind"`
	Scope   string        `json:"scope"`
	Line    int           `json:"line,omitempty"`
	Column  int           `json:"column,omitempty"`
	Excerpt string        `json:"excerpt,omitempty"`
	Autofix *Fix          `json:"autofix,omitempty"`
}

type Project struct {
	Name        string   `json:"name"`
	Root        string   `json:"root"`
	SourceRoot  string   `json:"sourceRoot,omitempty"`
	ProjectType string   `json:"projectType,omitempty"`
	Tags        []string `json:"tags,omitempty"`
}

type SectionInfo struct {
	Name       string        `json:"name"`
	StartLine  int           `json:"startLine"`
	EndLine    int           `json:"endLine"`
	StartIndex int           `json:"startIndex"`
	EndIndex   int           `json:"endIndex"`
	Children   []SectionInfo `json:"children"`
}

type DefinitionKind string

const (
	DefFunction  DefinitionKind = "function"
	DefClass     DefinitionKind = "class"
	DefVariable  DefinitionKind = "variable"
	DefInterface DefinitionKind = "interface"
	DefType      DefinitionKind = "type"
	DefEnum      DefinitionKind = "enum"
	DefMethod    DefinitionKind = "method"
	DefProperty  DefinitionKind = "property"
)

type DefinitionInfo struct {
	Name       string         `json:"name"`
	Kind       DefinitionKind `json:"kind"`
	StartLine  int            `json:"startLine"`
	EndLine    int            `json:"endLine"`
	StartIndex int            `json:"startIndex"`
	EndIndex   int            `json:"endIndex"`
}

type TicketStatus string

const (
	TicketOpen   TicketStatus = "open"
	TicketClosed TicketStatus = "closed"
)

type TicketIterationFiles struct {
	Updated []FileLineStats `json:"updated,omitempty"`
	Created []FileLineStats `json:"created,omitempty"`
	Removed []FileLineStats `json:"removed,omitempty"`
}

type FileLineStats struct {
	Path  string     `json:"path"`
	Lines *LineStats `json:"lines,omitempty"`
}

type LineStats struct {
	Added   int `json:"added"`
	Removed int `json:"removed"`
}

type TicketDate struct {
	Started string `json:"started,omitempty"`
	Ended   string `json:"ended,omitempty"`
}

type TicketIteration struct {
	Prompt string               `json:"prompt"`
	Model  string               `json:"model,omitempty"`
	Date   TicketDate           `json:"date"`
	Author string               `json:"author,omitempty"`
	Commit string               `json:"commit,omitempty"`
	Files  *TicketIterationFiles `json:"files,omitempty"`
	Lines  *LineStats           `json:"lines,omitempty"`
}

type TicketFiles struct {
	Updated []FileLineStats `json:"updated,omitempty"`
	Created []string        `json:"created,omitempty"`
	Removed []string        `json:"removed,omitempty"`
}

type TicketFrontmatter struct {
	Slug       string             `yaml:"slug" json:"slug"`
	Prompt     string             `yaml:"prompt" json:"prompt"`
	Summary    string             `yaml:"summary,omitempty" json:"summary,omitempty"`
	Status     TicketStatus       `yaml:"status" json:"status"`
	Author     string             `yaml:"author,omitempty" json:"author,omitempty"`
	Date       TicketDateCreated  `yaml:"date" json:"date"`
	Commit     string             `yaml:"commit,omitempty" json:"commit,omitempty"`
	Model      string             `yaml:"model,omitempty" json:"model,omitempty"`
	Iterations []TicketIteration  `yaml:"iterations,omitempty" json:"iterations,omitempty"`
	Files      *TicketFiles       `yaml:"files,omitempty" json:"files,omitempty"`
	Lines      *LineStats         `yaml:"lines,omitempty" json:"lines,omitempty"`
}

type TicketDateCreated struct {
	Created  string `yaml:"created" json:"created"`
	Finished string `yaml:"finished,omitempty" json:"finished,omitempty"`
}

type Ticket struct {
	Year        int               `json:"year"`
	Month       int               `json:"month"`
	Day         int               `json:"day"`
	Slug        string            `json:"slug"`
	Frontmatter TicketFrontmatter `json:"frontmatter"`
	Content     string            `json:"content"`
	FilePath    string            `json:"filePath"`
}

type ViolationKind string

const (
	ViolationHeaderMissingRegion       ViolationKind = "header:missing-region"
	ViolationHeaderMissingFilename     ViolationKind = "header:missing-filename"
	ViolationHeaderMissingContributors ViolationKind = "header:missing-contributors"
	ViolationHeaderMissingLicense      ViolationKind = "header:missing-license"
	ViolationHeaderWrongLicense        ViolationKind = "header:wrong-license"
	ViolationSectionEmpty              ViolationKind = "section:empty"
	ViolationSectionMissingStartName   ViolationKind = "section:missing-start-name"
	ViolationSectionMissingEndName     ViolationKind = "section:missing-end-name"
	ViolationSectionNameMismatch       ViolationKind = "section:name-mismatch"
	ViolationCommentInline             ViolationKind = "comment:inline"
	ViolationCommentBlock              ViolationKind = "comment:block"
	ViolationCommentJSDoc              ViolationKind = "comment:jsdoc"
	ViolationDevDocsMissingFile        ViolationKind = "dev-docs:missing-file"
	ViolationDevDocsMissingFolder      ViolationKind = "dev-docs:missing-folder"
	ViolationDevDocsWrongFilePath      ViolationKind = "dev-docs:wrong-file-path"
	ViolationDevDocsWrongFolderPath    ViolationKind = "dev-docs:wrong-folder-path"
	ViolationDevDocsWrongFileName      ViolationKind = "dev-docs:wrong-file-name"
	ViolationDevDocsWrongFolderName    ViolationKind = "dev-docs:wrong-folder-name"
	ViolationDevDocsWrongFileOrder     ViolationKind = "dev-docs:wrong-file-order"
	ViolationDevDocsWrongFolderOrder   ViolationKind = "dev-docs:wrong-folder-order"
	ViolationDevDocsMissingComponent   ViolationKind = "dev-docs:missing-component"
	ViolationDevDocsWrongComponentName ViolationKind = "dev-docs:wrong-component-name"
	ViolationDevDocsWrongComponentOrder ViolationKind = "dev-docs:wrong-component-order"
)

type ViolationKindMeta struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
	Autofixable bool              `json:"autofixable"`
}

var violationKindMetas = map[ViolationKind]ViolationKindMeta{
	ViolationHeaderMissingRegion: {
		Kind:        ViolationHeaderMissingRegion,
		Priority:    PriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: false,
	},
	ViolationHeaderMissingFilename: {
		Kind:        ViolationHeaderMissingFilename,
		Priority:    PriorityLow,
		Reason:      "Filename must be documented in header",
		Solution:    "Add filename comment in header region",
		Autofixable: false,
	},
	ViolationHeaderMissingContributors: {
		Kind:        ViolationHeaderMissingContributors,
		Priority:    PriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	ViolationHeaderMissingLicense: {
		Kind:        ViolationHeaderMissingLicense,
		Priority:    PriorityLow,
		Reason:      "SPDX license identifier is required",
		Solution:    "Add SPDX license header comment",
		Autofixable: false,
	},
	ViolationHeaderWrongLicense: {
		Kind:        ViolationHeaderWrongLicense,
		Priority:    PriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Update license to AGPL-3.0-or-later",
		Autofixable: false,
	},
	ViolationSectionEmpty: {
		Kind:        ViolationSectionEmpty,
		Priority:    PriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	ViolationSectionMissingStartName: {
		Kind:        ViolationSectionMissingStartName,
		Priority:    PriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	ViolationSectionMissingEndName: {
		Kind:        ViolationSectionMissingEndName,
		Priority:    PriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	ViolationSectionNameMismatch: {
		Kind:        ViolationSectionNameMismatch,
		Priority:    PriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	ViolationCommentInline: {
		Kind:        ViolationCommentInline,
		Priority:    PriorityLow,
		Reason:      "Inline comments are forbidden - documentation belongs in README.md and AGENTS.md",
		Solution:    "Remove inline comment and document in README.md or AGENTS.md",
		Autofixable: false,
	},
	ViolationCommentBlock: {
		Kind:        ViolationCommentBlock,
		Priority:    PriorityLow,
		Reason:      "Block comments are forbidden - documentation belongs in README.md and AGENTS.md",
		Solution:    "Remove block comment and document in README.md or AGENTS.md",
		Autofixable: false,
	},
	ViolationCommentJSDoc: {
		Kind:        ViolationCommentJSDoc,
		Priority:    PriorityLow,
		Reason:      "JSDoc comments are forbidden - documentation belongs in README.md and AGENTS.md",
		Solution:    "Remove JSDoc comment and document in README.md or AGENTS.md",
		Autofixable: false,
	},
	ViolationDevDocsMissingFile: {
		Kind:        ViolationDevDocsMissingFile,
		Priority:    PriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsMissingFolder: {
		Kind:        ViolationDevDocsMissingFolder,
		Priority:    PriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsWrongFilePath: {
		Kind:        ViolationDevDocsWrongFilePath,
		Priority:    PriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderPath: {
		Kind:        ViolationDevDocsWrongFolderPath,
		Priority:    PriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileName: {
		Kind:        ViolationDevDocsWrongFileName,
		Priority:    PriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄 PATH)",
		Solution:    "Rename section to ## 📄 PATH",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderName: {
		Kind:        ViolationDevDocsWrongFolderName,
		Priority:    PriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁 PATH/)",
		Solution:    "Rename section to ## 📁 PATH/",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileOrder: {
		Kind:        ViolationDevDocsWrongFileOrder,
		Priority:    PriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderOrder: {
		Kind:        ViolationDevDocsWrongFolderOrder,
		Priority:    PriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsMissingComponent: {
		Kind:        ViolationDevDocsMissingComponent,
		Priority:    PriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentName: {
		Kind:        ViolationDevDocsWrongComponentName,
		Priority:    PriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentOrder: {
		Kind:        ViolationDevDocsWrongComponentOrder,
		Priority:    PriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
}

func GetViolationKindMeta(kind ViolationKind) ViolationKindMeta {
	if meta, ok := violationKindMetas[kind]; ok {
		return meta
	}
	return ViolationKindMeta{
		Kind:        kind,
		Priority:    PriorityLow,
		Reason:      "Unknown violation",
		Solution:    "Fix the violation",
		Autofixable: false,
	}
}

type PolicyMeta struct {
	ID             string            `json:"id"`
	Name           string            `json:"name"`
	Description    string            `json:"description"`
	Scopes         []string          `json:"scopes"`
	Priority       ViolationPriority `json:"priority"`
	ViolationKinds []string          `json:"violationKinds"`
}

type AnalyzeReport struct {
	Timestamp  string      `json:"timestamp"`
	Status     string      `json:"status"`
	Scope      string      `json:"scope"`
	Summary    Summary     `json:"summary"`
	Violations []Violation `json:"violations"`
}

type Summary struct {
	Total      int            `json:"total"`
	ByPriority map[string]int `json:"byPriority"`
	ByKind     map[string]int `json:"byKind"`
}

type FileCache struct {
	FilePath   string      `json:"filePath"`
	Hash       string      `json:"hash"`
	Timestamp  string      `json:"timestamp"`
	Violations []Violation `json:"violations"`
}

type OutputType string

const (
	OutputInfo    OutputType = "info"
	OutputSuccess OutputType = "success"
	OutputError   OutputType = "error"
	OutputWarn    OutputType = "warn"
	OutputPlain   OutputType = "plain"
)

type OutputLine struct {
	Type OutputType `json:"type"`
	Text string     `json:"text"`
}

type CommandOutput struct {
	Lines    []OutputLine `json:"lines"`
	ExitCode int          `json:"exitCode"`
}

type ToolResult struct {
	Output   CommandOutput `json:"output"`
	Data     interface{}   `json:"data,omitempty"`
	Error    string        `json:"error,omitempty"`
}

type Contributor struct {
	Github        string                   `json:"github"`
	Name          string                   `json:"name,omitempty"`
	Emails        []string                 `json:"emails,omitempty"`
	Links         map[string]string        `json:"links,omitempty"`
	Contributions ContributorContributions `json:"contributions,omitempty"`
}

type ContributorContributions struct {
	Projects    []string `json:"projects,omitempty"`
	Folders     []string `json:"folders,omitempty"`
	Files       []string `json:"files,omitempty"`
	Regions     []string `json:"regions,omitempty"`
	Definitions []string `json:"definitions,omitempty"`
}

// #endregion

// #region Utils

var rootDir string

func init() {
	wd, err := os.Getwd()
	if err != nil {
		rootDir = "."
	} else {
		rootDir = findRepoRoot(wd)
	}
}

func findRepoRoot(start string) string {
	dir := start
	for {
		if _, err := os.Stat(filepath.Join(dir, "package.json")); err == nil {
			if _, err := os.Stat(filepath.Join(dir, "nx.json")); err == nil {
				return dir
			}
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return start
		}
		dir = parent
	}
}

var (
	cachedGitignorePatterns []string
	gitignoreLoaded         bool
	gitignoreMutex          sync.Mutex
)

func getGitignorePatterns() []string {
	gitignoreMutex.Lock()
	defer gitignoreMutex.Unlock()
	if gitignoreLoaded {
		return cachedGitignorePatterns
	}
	gitignorePath := filepath.Join(rootDir, ".gitignore")
	content, err := os.ReadFile(gitignorePath)
	if err != nil {
		gitignoreLoaded = true
		return nil
	}
	for _, line := range strings.Split(string(content), "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			if !strings.Contains(line, "/") {
				line = "**/" + line
			}
			cachedGitignorePatterns = append(cachedGitignorePatterns, line)
		}
	}
	gitignoreLoaded = true
	return cachedGitignorePatterns
}

func isGitIgnored(filePath string) bool {
	relPath, err := filepath.Rel(rootDir, filePath)
	if err != nil {
		return false
	}
	relPath = NormalizePath(relPath)
	for _, pattern := range getGitignorePatterns() {
		if matched, _ := doublestar.Match(pattern, relPath); matched {
			return true
		}
	}
	return false
}

func policyAppliesToScope(policyID string, scope Scope) bool {
	switch policyID {
	case "header":
		return scope.Kind == ScopeFile && isSourceFile(scope.FilePath)
	case "section":
		return scope.Kind == ScopeFile && isSourceFile(scope.FilePath)
	case "comment":
		return scope.Kind == ScopeFile && isSourceFile(scope.FilePath)
	case "dev-docs":
		return scope.Kind == ScopeRepo || scope.Kind == ScopeFolder || scope.Kind == ScopeFile
	default:
		return true
	}
}

func isSourceFile(filePath string) bool {
	ext := filepath.Ext(filePath)
	return ext == ".ts" || ext == ".tsx" || ext == ".js" || ext == ".jsx" ||
		ext == ".py" || ext == ".go" || ext == ".cs"
}

func GetRootDir() string {
	return rootDir
}

func SetRootDir(dir string) {
	rootDir = dir
}

func NormalizePath(p string) string {
	return strings.ReplaceAll(p, "\\", "/")
}

func EnsureDir(dirPath string) error {
	return os.MkdirAll(dirPath, 0755)
}

func GetRelativePath(filePath string) string {
	rel, err := filepath.Rel(rootDir, filePath)
	if err != nil {
		return filePath
	}
	return NormalizePath(rel)
}

func ReadTextFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func WriteTextFile(filePath string, content string) error {
	if err := EnsureDir(filepath.Dir(filePath)); err != nil {
		return err
	}
	return os.WriteFile(filePath, []byte(content), 0644)
}

func WriteJSONFile(filePath string, data interface{}) error {
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(filePath, string(jsonBytes)+"\n")
}

func ReadJSONFile(filePath string, v interface{}) error {
	data, err := ReadTextFile(filePath)
	if err != nil {
		return err
	}
	return json.Unmarshal([]byte(data), v)
}

func FileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func IsDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}

func LoadGitignore(cwd string) ([]string, error) {
	gitignorePath := filepath.Join(cwd, ".gitignore")
	if !FileExists(gitignorePath) {
		return nil, nil
	}
	content, err := ReadTextFile(gitignorePath)
	if err != nil {
		return nil, err
	}
	var patterns []string
	for _, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			patterns = append(patterns, line)
		}
	}
	return patterns, nil
}

func SimpleGlob(pattern string, cwd string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	if cwd == "" {
		cwd = rootDir
	}
	var gitignorePatterns []string
	if respectGitignore {
		var err error
		gitignorePatterns, err = LoadGitignore(cwd)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	var files []string
	absPattern := filepath.Join(cwd, pattern)
	matches, err := doublestar.FilepathGlob(absPattern)
	if err != nil {
		return nil, err
	}
	for _, match := range matches {
		rel, err := filepath.Rel(cwd, match)
		if err != nil {
			continue
		}
		relNorm := NormalizePath(rel)
		ignored := false
		for _, ig := range allIgnore {
			if matched, _ := doublestar.Match(ig, relNorm); matched {
				ignored = true
				break
			}
		}
		if !ignored {
			files = append(files, relNorm)
		}
	}
	return files, nil
}

func ISOTimestamp() string {
	return time.Now().UTC().Format(time.RFC3339)
}

func FormatDate(t time.Time) (year, month, day int) {
	return t.Year(), int(t.Month()), t.Day()
}

func PadNumber(n, width int) string {
	return fmt.Sprintf("%0*d", width, n)
}

func Slugify(text string) string {
	re := regexp.MustCompile(`[^A-Z0-9]+`)
	slug := re.ReplaceAllString(strings.ToUpper(text), "-")
	return strings.Trim(slug, "-")
}

func ExecCommand(command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	if cwd == "" {
		cwd = rootDir
	}
	cmd := exec.Command(command, args...)
	cmd.Dir = cwd
	var stdoutBuf, stderrBuf strings.Builder
	cmd.Stdout = &stdoutBuf
	cmd.Stderr = &stderrBuf
	err := cmd.Run()
	exitCode = 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = 1
		}
	}
	return stdoutBuf.String(), stderrBuf.String(), exitCode
}

func GetGitAuthor() string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	name = strings.TrimSpace(name)
	email = strings.TrimSpace(email)
	if email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	return name
}

func GetGitCommit() string {
	commit, _, _ := ExecCommand("git", []string{"rev-parse", "HEAD"}, "")
	return strings.TrimSpace(commit)
}

func GetGitIgnoredSet(paths []string) map[string]bool {
	if len(paths) == 0 {
		return make(map[string]bool)
	}
	args := append([]string{"check-ignore"}, paths...)
	stdout, _, _ := ExecCommand("git", args, "")
	ignored := make(map[string]bool)
	for _, line := range strings.Split(stdout, "\n") {
		line = strings.TrimSpace(line)
		if line != "" {
			ignored[NormalizePath(line)] = true
		}
	}
	return ignored
}

func GetLanguageFromPath(filePath string) string {
	ext := strings.ToLower(filepath.Ext(filePath))
	switch ext {
	case ".ts", ".tsx", ".js", ".jsx":
		return "typescript"
	case ".go":
		return "go"
	case ".py":
		return "python"
	case ".cs":
		return "csharp"
	case ".md", ".mdx":
		return "markdown"
	default:
		return ""
	}
}

func NewOutput() *CommandOutput {
	return &CommandOutput{Lines: []OutputLine{}, ExitCode: 0}
}

func (o *CommandOutput) Info(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputInfo, Text: text})
}

func (o *CommandOutput) Success(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputSuccess, Text: text})
}

func (o *CommandOutput) Error(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputError, Text: text})
	o.ExitCode = 1
}

func (o *CommandOutput) Warn(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputWarn, Text: text})
}

func (o *CommandOutput) Plain(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: text})
}

func (o *CommandOutput) Print() {
	for _, line := range o.Lines {
		fmt.Println(line.Text)
	}
}

func ListDirEntries(dir string, dirsOnly bool) ([]string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasPrefix(e.Name(), ".") {
			continue
		}
		if dirsOnly && !e.IsDir() {
			continue
		}
		if !dirsOnly && e.IsDir() {
			continue
		}
		names = append(names, e.Name())
	}
	return names, nil
}

func WalkDir(dir string, fn func(path string, isDir bool) error) error {
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if strings.HasPrefix(info.Name(), ".") {
			if info.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		return fn(path, info.IsDir())
	})
}

func ParseScope(raw string) Scope {
	if raw == "" || raw == "@semio" {
		return Scope{Raw: "@semio", Kind: ScopeRepo}
	}
	if strings.Contains(raw, "§") {
		parts := strings.SplitN(raw, "§", 2)
		return Scope{Raw: raw, Kind: ScopeDefinition, FilePath: parts[0], DefinitionName: parts[1]}
	}
	if strings.Contains(raw, "#") {
		parts := strings.Split(raw, "#")
		return Scope{Raw: raw, Kind: ScopeSection, FilePath: parts[0], SectionPath: parts[1:]}
	}
	if strings.HasPrefix(raw, "@semio/") {
		return Scope{Raw: raw, Kind: ScopeProject, ProjectName: raw}
	}
	if strings.HasSuffix(raw, "/") {
		return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
	}
	ext := strings.ToLower(filepath.Ext(raw))
	codeExtensions := map[string]bool{".ts": true, ".tsx": true, ".js": true, ".jsx": true, ".py": true, ".cs": true, ".go": true, ".json": true, ".md": true, ".yaml": true, ".yml": true, ".sql": true, ".graphql": true}
	if codeExtensions[ext] {
		return Scope{Raw: raw, Kind: ScopeFile, FilePath: raw}
	}
	return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
}

func ReadLines(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

// #endregion Utils

// #region Sections

type sectionPatterns struct {
	Start *regexp.Regexp
	End   *regexp.Regexp
}

var patterns = map[string]sectionPatterns{
	"typescript": {
		Start: regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
	},
	"python": {
		Start: regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
	},
	"csharp": {
		Start: regexp.MustCompile(`(?i)^\s*#region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(.+?))?\s*$`),
	},
}

func ParseCodeSections(content string, language string) []SectionInfo {
	lines := strings.Split(content, "\n")
	var stack []*SectionInfo
	var roots []SectionInfo
	pat, ok := patterns[language]
	if !ok {
		return roots
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := pat.Start.FindStringSubmatch(line); match != nil {
			name := strings.TrimSpace(match[1])
			section := &SectionInfo{
				Name:       name,
				StartLine:  lineNum,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []SectionInfo{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.Children = append(parent.Children, *section)
				section = &parent.Children[len(parent.Children)-1]
			}
			stack = append(stack, section)
		} else if pat.End.MatchString(line) {
			if len(stack) > 0 {
				section := stack[len(stack)-1]
				section.EndLine = lineNum
				section.EndIndex = charIndex + len(line)
				stack = stack[:len(stack)-1]
				if len(stack) == 0 {
					roots = append(roots, *section)
				}
			}
		}
		charIndex += len(line) + 1
	}
	return roots
}

func ParseMarkdownSections(content string) []SectionInfo {
	lines := strings.Split(content, "\n")
	var sections []SectionInfo
	type stackItem struct {
		level   int
		section *SectionInfo
	}
	var stack []stackItem
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	frontmatterLines := 0
	if strings.HasPrefix(content, "---") {
		endIndex := strings.Index(content[3:], "---")
		if endIndex != -1 {
			frontmatterContent := content[:endIndex+6]
			frontmatterLines = strings.Count(frontmatterContent, "\n")
		}
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(match[2])
			for len(stack) > 0 && stack[len(stack)-1].level >= level {
				popped := stack[len(stack)-1]
				popped.section.EndLine = frontmatterLines + i
				popped.section.EndIndex = lineStart - 1
				stack = stack[:len(stack)-1]
			}
			section := &SectionInfo{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []SectionInfo{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.section.Children = append(parent.section.Children, *section)
				section = &parent.section.Children[len(parent.section.Children)-1]
			} else {
				sections = append(sections, *section)
				section = &sections[len(sections)-1]
			}
			stack = append(stack, stackItem{level: level, section: section})
		}
		charIndex += len(line) + 1
	}
	for len(stack) > 0 {
		popped := stack[len(stack)-1]
		popped.section.EndLine = frontmatterLines + len(lines)
		popped.section.EndIndex = len(content)
		stack = stack[:len(stack)-1]
	}
	return sections
}

func ParseSections(content string, filePath string) []SectionInfo {
	lang := GetLanguageFromPath(filePath)
	if lang == "markdown" {
		return ParseMarkdownSections(content)
	}
	if lang != "" {
		return ParseCodeSections(content, lang)
	}
	return nil
}

func FindSection(sections []SectionInfo, name string) *SectionInfo {
	for i := range sections {
		if sections[i].Name == name {
			return &sections[i]
		}
		if found := FindSection(sections[i].Children, name); found != nil {
			return found
		}
	}
	return nil
}

// #endregion Sections

// #region Policies

type PolicyFunc func(ctx *PolicyContext) []Violation

type RegisteredPolicy struct {
	Meta PolicyMeta
	Run  PolicyFunc
}

var policyMetas = []PolicyMeta{
	{
		ID:          "header",
		Name:        "Header",
		Description: "Validates source file header section with filename, contributors, and license",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go}"},
		Priority:    PriorityLow,
		ViolationKinds: []string{
			string(ViolationHeaderMissingRegion),
			string(ViolationHeaderMissingFilename),
			string(ViolationHeaderMissingContributors),
			string(ViolationHeaderMissingLicense),
			string(ViolationHeaderWrongLicense),
		},
	},
	{
		ID:          "section",
		Name:        "Section",
		Description: "Validates section blocks for proper naming and content",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go}"},
		Priority:    PriorityLow,
		ViolationKinds: []string{
			string(ViolationSectionEmpty),
			string(ViolationSectionMissingStartName),
			string(ViolationSectionMissingEndName),
			string(ViolationSectionNameMismatch),
		},
	},
	{
		ID:          "comment",
		Name:        "Comment",
		Description: "Detects forbidden comments (inline, block, JSDoc) - documentation belongs in README.md and AGENTS.md",
		Scopes:      []string{"**/*.{ts,tsx}"},
		Priority:    PriorityLow,
		ViolationKinds: []string{
			string(ViolationCommentInline),
			string(ViolationCommentBlock),
			string(ViolationCommentJSDoc),
		},
	},
	{
		ID:          "dev-docs",
		Name:        "DevDocs",
		Description: "Validates README.md and AGENTS.md documentation structure",
		Scopes:      []string{"README.md", "AGENTS.md"},
		Priority:    PriorityLow,
		ViolationKinds: []string{
			string(ViolationDevDocsMissingFile),
			string(ViolationDevDocsMissingFolder),
			string(ViolationDevDocsWrongFilePath),
			string(ViolationDevDocsWrongFolderPath),
			string(ViolationDevDocsWrongFileName),
			string(ViolationDevDocsWrongFolderName),
			string(ViolationDevDocsWrongFileOrder),
			string(ViolationDevDocsWrongFolderOrder),
			string(ViolationDevDocsMissingComponent),
			string(ViolationDevDocsWrongComponentName),
			string(ViolationDevDocsWrongComponentOrder),
		},
	},
}

var policyFuncs = map[string]PolicyFunc{
	"header":   headerPolicy,
	"section":  sectionPolicy,
	"comment":  commentPolicy,
	"dev-docs": devDocsPolicy,
}

func getRegisteredPolicies() []RegisteredPolicy {
	var policies []RegisteredPolicy
	for _, meta := range policyMetas {
		if fn, ok := policyFuncs[meta.ID]; ok {
			policies = append(policies, RegisteredPolicy{Meta: meta, Run: fn})
		}
	}
	return policies
}

func GetRegisteredPolicies() []PolicyMeta {
	return policyMetas
}

type PolicyContext struct {
	Scope    Scope
	RootDir  string
	Projects []Project
	fileCache     map[string]string
	sectionCache  map[string][]SectionInfo
}

func NewPolicyContext(scope Scope, projects []Project) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Projects:     projects,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]SectionInfo),
	}
}

func (ctx *PolicyContext) Files() ([]string, error) {
	return ScopeToFiles(ctx.Scope, ctx.Projects)
}

func (ctx *PolicyContext) ReadText(filePath string) string {
	absPath := filepath.Join(rootDir, filePath)
	if content, ok := ctx.fileCache[absPath]; ok {
		return content
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		ctx.fileCache[absPath] = ""
		return ""
	}
	ctx.fileCache[absPath] = content
	return content
}

func (ctx *PolicyContext) Sections(filePath string) []SectionInfo {
	if sections, ok := ctx.sectionCache[filePath]; ok {
		return sections
	}
	content := ctx.ReadText(filePath)
	sections := ParseSections(content, filePath)
	ctx.sectionCache[filePath] = sections
	return sections
}

func (ctx *PolicyContext) CreateViolation(summary string, kind ViolationKind, scope string, line int, excerpt string, autofix *Fix) Violation {
	return Violation{
		ID:      fmt.Sprintf("%s-%d-%s", kind, time.Now().UnixNano(), randomString(6)),
		Summary: summary,
		Kind:    kind,
		Scope:   scope,
		Line:    line,
		Excerpt: excerpt,
		Autofix: autofix,
	}
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

func CheckPolicies(scope Scope, projects []Project, policyIDs []string) ([]Violation, error) {
	ctx := NewPolicyContext(scope, projects)
	var violations []Violation
	var policiesToRun []RegisteredPolicy
	allPolicies := getRegisteredPolicies()
	if len(policyIDs) > 0 {
		for _, p := range allPolicies {
			for _, id := range policyIDs {
				if p.Meta.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range allPolicies {
			if matchesScope(p.Meta.Scopes, scope) {
				policiesToRun = append(policiesToRun, p)
			}
		}
	}
	for _, policy := range policiesToRun {
		policyViolations := policy.Run(ctx)
		violations = append(violations, policyViolations...)
	}
	return violations, nil
}

func matchesScope(policyScopes []string, targetScope Scope) bool {
	for _, pattern := range policyScopes {
		if pattern == "*" || pattern == "**/*" {
			return true
		}
		if strings.HasPrefix(pattern, "@semio") {
			if targetScope.Kind == ScopeRepo || (targetScope.Kind == ScopeProject && strings.HasPrefix(targetScope.ProjectName, pattern)) {
				return true
			}
		}
		if targetScope.Kind == ScopeRepo && strings.HasPrefix(pattern, "**/*.") {
			return true
		}
		if targetScope.FilePath != "" {
			normalizedTarget := NormalizePath(targetScope.FilePath)
			normalizedPattern := NormalizePath(pattern)
			if matched, _ := doublestar.Match(normalizedPattern, normalizedTarget); matched {
				return true
			}
		}
	}
	return false
}

func headerPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	agplMarkers := []string{"GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang == "" || lang == "markdown" {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *SectionInfo
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			headerContent := generateFileHeader(file, lang)
			if headerContent != "" {
				autofix := &Fix{
					Description: "Add header section",
					Edits: map[string][]TextEdit{
						file: {{Start: 0, End: 0, NewText: headerContent + "\n"}},
					},
				}
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationHeaderMissingRegion,
file, 0, "", autofix))
			} else {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationHeaderMissingRegion,
file, 0, "", nil))
			}
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		headerLines := strings.Split(headerContent, "\n")
		filename := filepath.Base(file)
		hasFilename := false
		for _, line := range headerLines {
			if strings.Contains(line, filename) {
				hasFilename = true
				break
			}
		}
		if !hasFilename {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing filename in header of %s", file),
				ViolationHeaderMissingFilename,
			fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		}
		contributorPattern := regexp.MustCompile(`\d{4}\s+[\w\s]+<[\w.@-]+>`)
		hasContributors := false
		for _, line := range headerLines {
			if contributorPattern.MatchString(line) {
				hasContributors = true
				break
			}
		}
		if !hasContributors {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing contributors in header of %s", file),
				ViolationHeaderMissingContributors,
fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		}
		hasLicense := false
		for _, marker := range agplMarkers {
			if strings.Contains(headerContent, marker) {
				hasLicense = true
				break
			}
		}
		if !hasLicense {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing license in header of %s", file),
				ViolationHeaderMissingLicense,
fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		} else {
			wrongLicenses := []string{"MIT", "Apache", "BSD"}
			hasWrongLicense := false
			for _, wrong := range wrongLicenses {
				if strings.Contains(headerContent, wrong) {
					hasWrongLicense = true
					break
				}
			}
			if strings.Contains(headerContent, "GPL") && !strings.Contains(headerContent, "AGPL") {
				hasWrongLicense = true
			}
			if hasWrongLicense {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Wrong license in header of %s", file),
					ViolationHeaderWrongLicense,
fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
			}
		}
	}
	return violations
}

func sectionPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	sectionPatterns := map[string]struct{ start, end *regexp.Regexp }{
		"typescript": {
			start: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
		"go": {
			start: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
		"python": {
			start: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
		"csharp": {
			start: regexp.MustCompile(`(?i)^\s*#region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang == "" || lang == "markdown" {
			continue
		}
		pat, ok := sectionPatterns[lang]
		if !ok {
			continue
		}
		lines := strings.Split(content, "\n")
		type stackItem struct {
			name string
			line int
		}
		var stack []stackItem
		for i, line := range lines {
			lineNum := i + 1
			line = strings.TrimSuffix(line, "\r")
			if match := pat.start.FindStringSubmatch(line); match != nil {
				name := ""
				if len(match) > 1 {
					name = strings.TrimSpace(match[1])
				}
				if name == "" {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						ViolationSectionMissingStartName,
file, lineNum, strings.TrimSpace(line), nil))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if match := pat.end.FindStringSubmatch(line); match != nil {
				endName := ""
				if len(match) > 1 {
					endName = strings.TrimSpace(match[1])
				}
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								ViolationSectionMissingEndName,
					file, lineNum, strings.TrimSpace(line), nil))
						} else if endName != open.name {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								ViolationSectionNameMismatch,
					file, lineNum, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName), nil))
						}
					}
				}
			}
		}
		sections := ctx.Sections(file)
		var checkSection func(s SectionInfo)
		checkSection = func(s SectionInfo) {
			sectionContent := content[s.StartIndex:s.EndIndex]
			sectionLines := strings.Split(sectionContent, "\n")
			nonEmpty := 0
			for _, line := range sectionLines[1 : len(sectionLines)-1] {
				trimmed := strings.TrimSpace(line)
				if trimmed != "" && !strings.HasPrefix(trimmed, "//") && !strings.HasPrefix(trimmed, "#") {
					nonEmpty++
				}
			}
			if nonEmpty == 0 && len(s.Children) == 0 {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Empty section \"%s\" in %s", s.Name, file),
					ViolationSectionEmpty,
fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, "", nil))
			}
			for _, child := range s.Children {
				checkSection(child)
			}
		}
		for _, s := range sections {
			checkSection(s)
		}
	}
	return violations
}

func commentPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang != "typescript" {
			continue
		}
		lines := strings.Split(content, "\n")
		charIndex := 0
		inBlockComment := false
		blockCommentStartLine := 0
		blockCommentStartIndex := 0
		inJsDoc := false
		jsDocStartLine := 0
		jsDocStartIndex := 0
		for i, line := range lines {
			lineNum := i + 1
			lineStart := charIndex
			lineEnd := lineStart + len(line) + 1
			trimmed := strings.TrimSpace(line)
			if strings.HasPrefix(trimmed, "/**") && !strings.HasSuffix(trimmed, "*/") {
				inJsDoc = true
				jsDocStartLine = lineNum
				jsDocStartIndex = lineStart
				charIndex = lineEnd
				continue
			}
			if inJsDoc {
				if strings.HasSuffix(trimmed, "*/") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("JSDoc comment in %s:%d", file, jsDocStartLine),
						ViolationCommentJSDoc,
file, jsDocStartLine, "", nil))
					inJsDoc = false
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "/*") && !strings.HasPrefix(trimmed, "/**") && !strings.HasSuffix(trimmed, "*/") {
				inBlockComment = true
				blockCommentStartLine = lineNum
				blockCommentStartIndex = lineStart
				charIndex = lineEnd
				continue
			}
			if inBlockComment {
				if strings.HasSuffix(trimmed, "*/") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Block comment in %s:%d", file, blockCommentStartLine),
						ViolationCommentBlock,
file, blockCommentStartLine, "", nil))
					inBlockComment = false
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "/*") && strings.HasSuffix(trimmed, "*/") {
				if strings.HasPrefix(trimmed, "/**") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("JSDoc comment in %s:%d", file, lineNum),
						ViolationCommentJSDoc,
file, lineNum, truncate(trimmed, 80), nil))
				} else {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Block comment in %s:%d", file, lineNum),
						ViolationCommentBlock,
file, lineNum, truncate(trimmed, 80), nil))
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") {
				charIndex = lineEnd
				continue
			}
			if strings.Contains(trimmed, "[DEBUG]") {
				charIndex = lineEnd
				continue
			}
			isHeaderLine := strings.Contains(trimmed, "Copyright") ||
				strings.Contains(trimmed, "License") ||
				strings.Contains(trimmed, "SPDX") ||
				strings.Contains(trimmed, "GNU") ||
				strings.Contains(trimmed, "AGPL")
			if isHeaderLine {
				charIndex = lineEnd
				continue
			}
			charIndex = lineEnd
			_ = blockCommentStartIndex
			_ = jsDocStartIndex
		}
	}
	return violations
}

func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

func devDocsPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	agentsContent := ctx.ReadText("AGENTS.md")
	if agentsContent == "" {
		return violations
	}
	codebaseStart := strings.Index(agentsContent, "\n# Codebase")
	if codebaseStart == -1 {
		return violations
	}
	codebaseContent := agentsContent[codebaseStart:]
	nextH1 := strings.Index(codebaseContent[1:], "\n# ")
	if nextH1 != -1 {
		codebaseContent = codebaseContent[:nextH1+1]
	}
	fileSectionRegex := regexp.MustCompile(`(?m)^## 📄\s*(.+?)\s*$`)
	folderSectionRegex := regexp.MustCompile(`(?m)^## 📁\s*(.+?)\s*$`)
	fileMatches := fileSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	folderMatches := folderSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	var fileSections []struct {
		path string
		line int
		pos  int
	}
	var folderSections []struct {
		path string
		line int
		pos  int
	}
	for _, match := range fileMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		fileSections = append(fileSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for _, match := range folderMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		folderSections = append(folderSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for i := 0; i < len(fileSections)-1; i++ {
		if fileSections[i].path > fileSections[i+1].path {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("File section '%s' should come after '%s' (alphabetical order)", fileSections[i].path, fileSections[i+1].path),
				ViolationDevDocsWrongFileOrder,
				"AGENTS.md", fileSections[i+1].line, "", nil))
		}
	}
	for i := 0; i < len(folderSections)-1; i++ {
		if folderSections[i].path > folderSections[i+1].path {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Folder section '%s' should come after '%s' (alphabetical order)", folderSections[i].path, folderSections[i+1].path),
				ViolationDevDocsWrongFolderOrder,
				"AGENTS.md", folderSections[i+1].line, "", nil))
		}
	}
	return violations
}

// #endregion Policies

// #region Tickets

func GetTicketsDir() string {
	return filepath.Join(rootDir, "tickets")
}

func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), strconv.Itoa(year), PadNumber(month, 2), PadNumber(day, 2), slug+".md")
}

func CreateTicket(slug, prompt, model string, files []string) (*Ticket, error) {
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required to create a ticket")
	}
	now := time.Now()
	year, month, day := FormatDate(now)
	normalizedSlug := Slugify(slug)
	filePath := GetTicketPath(year, month, day, normalizedSlug)
	gitAuthor := GetGitAuthor()
	gitCommit := GetGitCommit()
	var iterationFiles *TicketIterationFiles
	if len(files) > 0 {
		iterationFiles = &TicketIterationFiles{}
		for _, f := range files {
			iterationFiles.Updated = append(iterationFiles.Updated, FileLineStats{Path: f})
		}
	}
	firstIteration := TicketIteration{
		Prompt: prompt,
		Model:  model,
		Date:   TicketDate{Started: ISOTimestamp()},
		Author: gitAuthor,
		Files:  iterationFiles,
	}
	frontmatter := TicketFrontmatter{
		Slug:       normalizedSlug,
		Prompt:     prompt,
		Status:     TicketOpen,
		Author:     gitAuthor,
		Date:       TicketDateCreated{Created: ISOTimestamp()},
		Commit:     gitCommit,
		Model:      model,
		Iterations: []TicketIteration{firstIteration},
	}
	content := `# Previously

# Plan

# Changes`
	ticket := &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        normalizedSlug,
		Frontmatter: frontmatter,
		Content:     content,
		FilePath:    filePath,
	}
	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	return ticket, nil
}

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	filePath := GetTicketPath(year, month, day, slug)
	if !FileExists(filePath) {
		return nil, fmt.Errorf("ticket not found: %s", filePath)
	}
	raw, err := ReadTextFile(filePath)
	if err != nil {
		return nil, err
	}
	frontmatter, content, err := parseFrontmatter(raw)
	if err != nil {
		return nil, err
	}
	return &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        slug,
		Frontmatter: frontmatter,
		Content:     content,
		FilePath:    filePath,
	}, nil
}

func parseFrontmatter(raw string) (TicketFrontmatter, string, error) {
	var frontmatter TicketFrontmatter
	if !strings.HasPrefix(raw, "---") {
		return frontmatter, raw, nil
	}
	endIndex := strings.Index(raw[3:], "---")
	if endIndex == -1 {
		return frontmatter, raw, nil
	}
	yamlContent := raw[3 : endIndex+3]
	content := strings.TrimPrefix(raw[endIndex+6:], "\n")
	if err := yaml.Unmarshal([]byte(yamlContent), &frontmatter); err != nil {
		return frontmatter, content, err
	}
	return frontmatter, content, nil
}

func SaveTicket(ticket *Ticket) error {
	yamlBytes, err := yaml.Marshal(ticket.Frontmatter)
	if err != nil {
		return err
	}
	content := fmt.Sprintf("---\n%s---\n%s", string(yamlBytes), ticket.Content)
	return WriteTextFile(ticket.FilePath, content)
}

func ListTickets(year, month, day *int) ([]Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []Ticket
	var years []string
	if year != nil {
		years = []string{strconv.Itoa(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	for _, y := range years {
		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{PadNumber(*month, 2)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{PadNumber(*day, 2)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				entries, err := os.ReadDir(dayPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if !e.IsDir() && strings.HasSuffix(e.Name(), ".md") {
						slug := strings.TrimSuffix(e.Name(), ".md")
						yearInt, _ := strconv.Atoi(y)
						monthInt, _ := strconv.Atoi(m)
						dayInt, _ := strconv.Atoi(d)
						ticket, err := ReadTicket(yearInt, monthInt, dayInt, slug)
						if err == nil {
							tickets = append(tickets, *ticket)
						}
					}
				}
			}
		}
	}
	return tickets, nil
}

func StartIteration(ticket *Ticket, prompt, model string, files []string) error {
	if len(files) == 0 {
		return fmt.Errorf("at least one file is required to start an iteration")
	}
	gitAuthor := GetGitAuthor()
	var iterationFiles *TicketIterationFiles
	if len(files) > 0 {
		iterationFiles = &TicketIterationFiles{}
		for _, f := range files {
			iterationFiles.Updated = append(iterationFiles.Updated, FileLineStats{Path: f})
		}
	}
	iteration := TicketIteration{
		Prompt: prompt,
		Model:  model,
		Date:   TicketDate{Started: ISOTimestamp()},
		Author: gitAuthor,
		Files:  iterationFiles,
	}
	ticket.Frontmatter.Iterations = append(ticket.Frontmatter.Iterations, iteration)
	return SaveTicket(ticket)
}

func EndIteration(ticket *Ticket) error {
	if len(ticket.Frontmatter.Iterations) == 0 {
		return fmt.Errorf("no active iteration to end")
	}
	lastIdx := len(ticket.Frontmatter.Iterations) - 1
	last := &ticket.Frontmatter.Iterations[lastIdx]
	if last.Date.Ended != "" {
		return fmt.Errorf("last iteration already ended")
	}
	last.Date.Ended = ISOTimestamp()
	last.Commit = GetGitCommit()
	return SaveTicket(ticket)
}

func FinishTicket(ticket *Ticket) error {
	if len(ticket.Frontmatter.Iterations) > 0 {
		last := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
		if last.Date.Ended == "" {
			return fmt.Errorf("cannot finish ticket with unfinished iteration")
		}
	}
	ticket.Frontmatter.Status = TicketClosed
	ticket.Frontmatter.Date.Finished = ISOTimestamp()
	return SaveTicket(ticket)
}

func ReopenTicket(ticket *Ticket) error {
	if ticket.Frontmatter.Status == TicketOpen {
		return fmt.Errorf("ticket is already open")
	}
	if len(ticket.Frontmatter.Iterations) > 0 {
		last := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
		if last.Date.Ended == "" {
			return fmt.Errorf("cannot reopen ticket with unfinished iteration")
		}
	}
	ticket.Frontmatter.Status = TicketOpen
	ticket.Frontmatter.Date.Finished = ""
	return SaveTicket(ticket)
}

func CanCloseTicket(ticket *Ticket) (bool, []string) {
	var reasons []string
	violationsRe := regexp.MustCompile(`(?s)## Violations.*?(?:\n## |$)`)
	if match := violationsRe.FindString(ticket.Content); match != "" {
		if !strings.Contains(match, "(No violations)") && strings.Contains(match, "- [") {
			reasons = append(reasons, "Violations section is not empty")
		}
	}
	planRe := regexp.MustCompile(`(?s)# Plan.*?(?:\n# |$)`)
	if match := planRe.FindString(ticket.Content); match == "" || strings.TrimSpace(match) == "# Plan" {
		reasons = append(reasons, "Plan section is empty")
	}
	changesRe := regexp.MustCompile(`(?s)# Changes.*?(?:\n# |$)`)
	if match := changesRe.FindString(ticket.Content); match == "" || strings.TrimSpace(match) == "# Changes" {
		reasons = append(reasons, "Changes section is empty")
	}
	return len(reasons) == 0, reasons
}

// #endregion Tickets

// #region Contributors

func GetContributorsDir() string {
	return filepath.Join(rootDir, "contributors")
}

func GetContributorPath(github string) string {
	return filepath.Join(GetContributorsDir(), github)
}

func GetContributorJsonPath(github string) string {
	return filepath.Join(GetContributorPath(github), "contributor.json")
}

func GetContributorAvatarPath(github string) string {
	return filepath.Join(GetContributorPath(github), "avatar.png")
}

func GetContributorAvatarRoundPath(github string) string {
	return filepath.Join(GetContributorPath(github), "avatar-round-90x90.png")
}

func ContributorExists(github string) bool {
	return FileExists(GetContributorJsonPath(github))
}

func CreateContributor(github string) (*Contributor, error) {
	if ContributorExists(github) {
		return ReadContributor(github)
	}
	contributorDir := GetContributorPath(github)
	if err := EnsureDir(contributorDir); err != nil {
		return nil, err
	}
	if err := DownloadGitHubAvatar(github, contributorDir); err != nil {
		return nil, err
	}
	contributor := &Contributor{
		Github:        github,
		Links:         map[string]string{"github": fmt.Sprintf("https://github.com/%s", github)},
		Contributions: ContributorContributions{},
	}
	if err := SaveContributor(contributor); err != nil {
		return nil, err
	}
	return contributor, nil
}

func ReadContributor(github string) (*Contributor, error) {
	jsonPath := GetContributorJsonPath(github)
	if !FileExists(jsonPath) {
		return nil, fmt.Errorf("contributor not found: %s", github)
	}
	raw, err := ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var contributor Contributor
	if err := json.Unmarshal([]byte(raw), &contributor); err != nil {
		return nil, err
	}
	return &contributor, nil
}

func SaveContributor(contributor *Contributor) error {
	jsonPath := GetContributorJsonPath(contributor.Github)
	jsonBytes, err := json.MarshalIndent(contributor, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(jsonPath, string(jsonBytes))
}

func ListContributors() ([]Contributor, error) {
	dir := GetContributorsDir()
	if !FileExists(dir) {
		return nil, nil
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var contributors []Contributor
	for _, e := range entries {
		if e.IsDir() {
			contributor, err := ReadContributor(e.Name())
			if err == nil {
				contributors = append(contributors, *contributor)
			}
		}
	}
	return contributors, nil
}

func RemoveContributor(github string) error {
	path := GetContributorPath(github)
	if !FileExists(path) {
		return fmt.Errorf("contributor not found: %s", github)
	}
	return os.RemoveAll(path)
}

func DownloadGitHubAvatar(github, targetDir string) error {
	avatarUrl := fmt.Sprintf("https://github.com/%s.png", github)
	avatarPath := filepath.Join(targetDir, "avatar.png")
	stdout, _, exitCode := ExecCommand("curl", []string{"-s", "-L", "-o", avatarPath, avatarUrl}, "")
	if exitCode != 0 {
		return fmt.Errorf("failed to download avatar: %s", stdout)
	}
	return nil
}

func AddContributorProject(github string, project string) error {
	contributor, err := ReadContributor(github)
	if err != nil {
		contributor, err = CreateContributor(github)
		if err != nil {
			return err
		}
	}
	for _, p := range contributor.Contributions.Projects {
		if p == project {
			return nil
		}
	}
	contributor.Contributions.Projects = append(contributor.Contributions.Projects, project)
	return SaveContributor(contributor)
}

// #endregion Contributors

// #region Nx

var (
	cachedProjectNames   []string
	cachedProjectDetails = make(map[string]Project)
	nxMutex              sync.Mutex
)

func GetProjectNames() []string {
	nxMutex.Lock()
	defer nxMutex.Unlock()
	if cachedProjectNames != nil {
		return cachedProjectNames
	}
	stdout, _, exitCode := ExecCommand("npx", []string{"nx", "show", "projects", "--json"}, "")
	if exitCode != 0 {
		cachedProjectNames = []string{}
		return cachedProjectNames
	}
	var names []string
	if err := json.Unmarshal([]byte(stdout), &names); err != nil {
		cachedProjectNames = []string{}
		return cachedProjectNames
	}
	cachedProjectNames = names
	return cachedProjectNames
}

func GetProjectDetails(name string) Project {
	nxMutex.Lock()
	defer nxMutex.Unlock()
	if proj, ok := cachedProjectDetails[name]; ok {
		return proj
	}
	stdout, _, exitCode := ExecCommand("npx", []string{"nx", "show", "project", name, "--json"}, "")
	if exitCode != 0 {
		proj := Project{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(stdout), &config); err != nil {
		proj := Project{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	proj := Project{Name: name}
	if root, ok := config["root"].(string); ok {
		proj.Root = root
	}
	if sourceRoot, ok := config["sourceRoot"].(string); ok {
		proj.SourceRoot = sourceRoot
	}
	if projectType, ok := config["projectType"].(string); ok {
		proj.ProjectType = projectType
	}
	if tags, ok := config["tags"].([]interface{}); ok {
		for _, t := range tags {
			if tag, ok := t.(string); ok {
				proj.Tags = append(proj.Tags, tag)
			}
		}
	}
	cachedProjectDetails[name] = proj
	return proj
}

func GetProjects() []Project {
	names := GetProjectNames()
	projects := make([]Project, len(names))
	for i, name := range names {
		projects[i] = GetProjectDetails(name)
	}
	return projects
}

func RunNxTarget(target string, projects []string, extraArgs []string) (success bool, output string) {
	args := []string{"nx"}
	if len(projects) == 1 {
		args = append(args, "run", projects[0]+":"+target)
	} else if len(projects) > 1 {
		args = append(args, "run-many", "-t", target, "-p", strings.Join(projects, ","))
	} else {
		args = append(args, "run-many", "-t", target)
	}
	args = append(args, extraArgs...)
	stdout, stderr, exitCode := ExecCommand("npx", args, "")
	return exitCode == 0, stdout + stderr
}

func filterGitIgnored(files []string) []string {
	if len(files) == 0 {
		return files
	}
	ignored := GetGitIgnoredSet(files)
	var filtered []string
	for _, f := range files {
		if !ignored[f] {
			filtered = append(filtered, f)
		}
	}
	return filtered
}

func ScopeToFiles(scope Scope, projects []Project) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case ScopeRepo:
		files, err = SimpleGlob("**/*.{ts,tsx,py,cs,go}", rootDir, ignorePatterns, true)
	case ScopeProject:
		for _, proj := range projects {
			if proj.Name == scope.ProjectName {
				files, err = SimpleGlob(proj.Root+"/**/*.{ts,tsx,py,cs,go}", rootDir, ignorePatterns, true)
				break
			}
		}
	case ScopeFolder:
		if scope.FilePath != "" {
			files, err = SimpleGlob(scope.FilePath+"**/*.{ts,tsx,py,cs,go}", rootDir, ignorePatterns, true)
		}
	case ScopeFile, ScopeSection, ScopeDefinition:
		if scope.FilePath != "" {
			return []string{scope.FilePath}, nil
		}
	}
	if err != nil {
		return nil, err
	}
	if len(files) <= 10 {
		return files, nil
	}
	return filterGitIgnored(files), nil
}

// #endregion Nx

// #region Commands

func Execute() error {
	return rootCmd.Execute()
}

var rootCmd = &cobra.Command{
	Use:   "repo",
	Short: "Monorepo CLI for Semio",
	Long:  `repo - Monorepo CLI for Semio. All commands output JSON for programmatic use.`,
}

func init() {
	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(fixCmd)
	rootCmd.AddCommand(policyCmd)
	rootCmd.AddCommand(ticketCmd)
	rootCmd.AddCommand(contributorCmd)
	rootCmd.AddCommand(projectCmd)
	rootCmd.AddCommand(folderCmd)
	rootCmd.AddCommand(fileCmd)
	rootCmd.AddCommand(sectionCmd)
	rootCmd.AddCommand(definitionCmd)
	rootCmd.AddCommand(updateMetabolismCmd)
}

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope...]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := ToolAnalyze(scope, args)
		return outputResult(result)
	},
}

var fixCmd = &cobra.Command{
	Use:   "fix [scope]",
	Short: "Apply autofixes for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := ToolFix(scope)
		return outputResult(result)
	},
}

var policyCmd = &cobra.Command{
	Use:   "policy",
	Short: "Policy management commands",
}

var policyListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all registered policies",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolPolicyList()
		return outputResult(result)
	},
}

var policyCheckCmd = &cobra.Command{
	Use:   "check <id> [scope]",
	Short: "Check a specific policy",
	Args:  cobra.RangeArgs(1, 2),
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 1 {
			scope = args[1]
		}
		result := ToolPolicyCheck(args[0], scope)
		return outputResult(result)
	},
}

var policyViolationCmd = &cobra.Command{
	Use:   "violation",
	Short: "Policy violation commands",
}

var policyViolationListCmd = &cobra.Command{
	Use:   "list <policyId>",
	Short: "List violation kinds for a policy",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolPolicyViolationList(args[0])
		return outputResult(result)
	},
}

func init() {
	policyCmd.AddCommand(policyListCmd)
	policyCmd.AddCommand(policyCheckCmd)
	policyCmd.AddCommand(policyViolationCmd)
	policyViolationCmd.AddCommand(policyViolationListCmd)
}

var ticketCmd = &cobra.Command{
	Use:   "ticket",
	Short: "Ticket management commands",
}

var ticketCreateCmd = &cobra.Command{
	Use:   "create <slug>",
	Short: "Create a new ticket",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		files, _ := cmd.Flags().GetStringSlice("file")
		result := ToolTicketCreate(args[0], prompt, model, files)
		return outputResult(result)
	},
}

var ticketListCmd = &cobra.Command{
	Use:   "list [year] [month] [day]",
	Short: "List tickets",
	RunE: func(cmd *cobra.Command, args []string) error {
		var year, month, day *int
		if len(args) > 0 {
			y, _ := strconv.Atoi(args[0])
			year = &y
		}
		if len(args) > 1 {
			m, _ := strconv.Atoi(args[1])
			month = &m
		}
		if len(args) > 2 {
			d, _ := strconv.Atoi(args[2])
			day = &d
		}
		result := ToolTicketList(year, month, day)
		return outputResult(result)
	},
}

var ticketReadCmd = &cobra.Command{
	Use:   "read <year> <month> <day> <slug>",
	Short: "Read a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketRead(year, month, day, args[3])
		return outputResult(result)
	},
}

var ticketIterateCmd = &cobra.Command{
	Use:   "iterate",
	Short: "Ticket iteration commands",
}

var ticketIterateStartCmd = &cobra.Command{
	Use:   "start <year> <month> <day> <slug>",
	Short: "Start a ticket iteration",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		files, _ := cmd.Flags().GetStringSlice("file")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketIterateStart(year, month, day, args[3], prompt, model, files)
		return outputResult(result)
	},
}

var ticketIterateEndCmd = &cobra.Command{
	Use:   "end <year> <month> <day> <slug>",
	Short: "End a ticket iteration",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketIterateEnd(year, month, day, args[3])
		return outputResult(result)
	},
}

var ticketFinishCmd = &cobra.Command{
	Use:   "finish <year> <month> <day> <slug>",
	Short: "Finish a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketFinish(year, month, day, args[3])
		return outputResult(result)
	},
}

var ticketReopenCmd = &cobra.Command{
	Use:   "reopen <year> <month> <day> <slug>",
	Short: "Reopen a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketReopen(year, month, day, args[3])
		return outputResult(result)
	},
}

func init() {
	ticketCreateCmd.Flags().String("prompt", "", "Ticket prompt")
	ticketCreateCmd.Flags().String("model", "", "Model used")
	ticketCreateCmd.Flags().StringSlice("file", nil, "Files to include (can be specified multiple times)")
	ticketIterateStartCmd.Flags().String("prompt", "", "Iteration prompt")
	ticketIterateStartCmd.Flags().String("model", "", "Model used")
	ticketIterateStartCmd.Flags().StringSlice("file", nil, "Files to include (can be specified multiple times)")
	ticketIterateCmd.AddCommand(ticketIterateStartCmd)
	ticketIterateCmd.AddCommand(ticketIterateEndCmd)
	ticketCmd.AddCommand(ticketCreateCmd)
	ticketCmd.AddCommand(ticketListCmd)
	ticketCmd.AddCommand(ticketReadCmd)
	ticketCmd.AddCommand(ticketIterateCmd)
	ticketCmd.AddCommand(ticketFinishCmd)
	ticketCmd.AddCommand(ticketReopenCmd)
}

var contributorCmd = &cobra.Command{
	Use:   "contributor",
	Short: "Contributor management commands",
}

var contributorAddCmd = &cobra.Command{
	Use:   "add <github>",
	Short: "Add a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolContributorAdd(args[0])
		return outputResult(result)
	},
}

var contributorListCmd = &cobra.Command{
	Use:   "list",
	Short: "List contributors",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolContributorList()
		return outputResult(result)
	},
}

var contributorRemoveCmd = &cobra.Command{
	Use:   "remove <github>",
	Short: "Remove a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolContributorRemove(args[0])
		return outputResult(result)
	},
}

func init() {
	contributorCmd.AddCommand(contributorAddCmd)
	contributorCmd.AddCommand(contributorListCmd)
	contributorCmd.AddCommand(contributorRemoveCmd)
}

var projectCmd = &cobra.Command{
	Use:   "project",
	Short: "Project management commands",
}

var projectListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Nx projects",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolProjectList()
		return outputResult(result)
	},
}

var projectTreeCmd = &cobra.Command{
	Use:   "tree",
	Short: "Show project dependency tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolProjectTree()
		return outputResult(result)
	},
}

func init() {
	projectCmd.AddCommand(projectListCmd)
	projectCmd.AddCommand(projectTreeCmd)
}

var folderCmd = &cobra.Command{
	Use:   "folder",
	Short: "Folder management commands",
}

var folderCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFolderCreate(args[0])
		return outputResult(result)
	},
}

var folderMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a folder",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFolderMove(args[0], args[1])
		return outputResult(result)
	},
}

var folderDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFolderDelete(args[0])
		return outputResult(result)
	},
}

var folderListCmd = &cobra.Command{
	Use:   "list [path]",
	Short: "List folders",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := ToolFolderList(path)
		return outputResult(result)
	},
}

var folderTreeCmd = &cobra.Command{
	Use:   "tree [path]",
	Short: "Show folder tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := ToolFolderTree(path)
		return outputResult(result)
	},
}

func init() {
	folderCmd.AddCommand(folderCreateCmd)
	folderCmd.AddCommand(folderMoveCmd)
	folderCmd.AddCommand(folderDeleteCmd)
	folderCmd.AddCommand(folderListCmd)
	folderCmd.AddCommand(folderTreeCmd)
}

var fileCmd = &cobra.Command{
	Use:   "file",
	Short: "File management commands",
}

var fileCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFileCreate(args[0])
		return outputResult(result)
	},
}

var fileMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFileMove(args[0], args[1])
		return outputResult(result)
	},
}

var fileDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFileDelete(args[0])
		return outputResult(result)
	},
}

var fileListCmd = &cobra.Command{
	Use:   "list [scope]",
	Short: "List files in scope",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := ToolFileList(scope)
		return outputResult(result)
	},
}

var fileTreeCmd = &cobra.Command{
	Use:   "tree [path]",
	Short: "Show file tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := ToolFileTree(path)
		return outputResult(result)
	},
}

func init() {
	fileCmd.AddCommand(fileCreateCmd)
	fileCmd.AddCommand(fileMoveCmd)
	fileCmd.AddCommand(fileDeleteCmd)
	fileCmd.AddCommand(fileListCmd)
	fileCmd.AddCommand(fileTreeCmd)
}

var sectionCmd = &cobra.Command{
	Use:   "section",
	Short: "Section management commands",
}

var sectionCreateCmd = &cobra.Command{
	Use:   "create <file> <section-path>",
	Short: "Create a section in a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionCreate(args[0], args[1])
		return outputResult(result)
	},
}

var sectionMoveCmd = &cobra.Command{
	Use:   "move <file> <old-section> <new-section>",
	Short: "Move/rename a section",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionMove(args[0], args[1], args[2])
		return outputResult(result)
	},
}

var sectionDeleteCmd = &cobra.Command{
	Use:   "delete <file> <section-path>",
	Short: "Delete a section from a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionDelete(args[0], args[1])
		return outputResult(result)
	},
}

var sectionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List sections in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionList(args[0])
		return outputResult(result)
	},
}

var sectionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show section tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionTree(args[0])
		return outputResult(result)
	},
}

func init() {
	sectionCmd.AddCommand(sectionCreateCmd)
	sectionCmd.AddCommand(sectionMoveCmd)
	sectionCmd.AddCommand(sectionDeleteCmd)
	sectionCmd.AddCommand(sectionListCmd)
	sectionCmd.AddCommand(sectionTreeCmd)
}

var definitionCmd = &cobra.Command{
	Use:   "definition",
	Short: "Definition management commands",
}

var definitionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List definitions in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolDefinitionList(args[0])
		return outputResult(result)
	},
}

var definitionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show definition tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolDefinitionTree(args[0])
		return outputResult(result)
	},
}

func init() {
	definitionCmd.AddCommand(definitionListCmd)
	definitionCmd.AddCommand(definitionTreeCmd)
}

var updateMetabolismCmd = &cobra.Command{
	Use:   "update-metabolism",
	Short: "Update metabolism assets (exports kit to zip and copies to public folders)",
	RunE: func(cmd *cobra.Command, args []string) error {
		output := NewOutput()
		output.Info("\n🔄 Running update-metabolism via npx tsx...")
		stdout, stderr, exitCode := ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
		if exitCode != 0 {
			output.Error(fmt.Sprintf("Error: %s%s", stdout, stderr))
			return outputResult(ToolResult{Output: *output, Error: "update-metabolism failed"})
		}
		output.Success(stdout)
		return outputResult(ToolResult{Output: *output})
	},
}

func outputResult(result ToolResult) error {
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	return enc.Encode(result)
}

func AnalyzeFile(filePath string, projects []Project) ([]Violation, error) {
	absPath := filePath
	if !filepath.IsAbs(absPath) {
		absPath = filepath.Join(rootDir, filePath)
	}
	
	if isGitIgnored(absPath) {
		return []Violation{}, nil
	}
	
	scope := ParseScope(filePath)
	violations, err := CheckPolicies(scope, projects, nil)
	if err != nil {
		return nil, err
	}
	return violations, nil
}

func ToolAnalyze(scope string, scopes []string) ToolResult {
	output := NewOutput()
	var scopeRaws []string
	if scope != "" {
		scopeRaws = []string{scope}
	} else if len(scopes) > 0 {
		scopeRaws = scopes
	} else {
		scopeRaws = []string{"@semio"}
	}
	var allViolations []Violation
	var projects []Project
	var projectsLoaded bool
	getProjectsLazy := func() []Project {
		if !projectsLoaded {
			projects = GetProjects()
			projectsLoaded = true
		}
		return projects
	}
	for _, scopeRaw := range scopeRaws {
		s := ParseScope(scopeRaw)
		if s.Kind == ScopeFile || s.Kind == ScopeSection || s.Kind == ScopeDefinition {
			violations, err := AnalyzeFile(s.FilePath, nil)
			if err != nil {
				output.Error(fmt.Sprintf("Error analyzing file: %v", err))
				return ToolResult{Output: *output, Error: err.Error()}
			}
			allViolations = append(allViolations, violations...)
		} else {
			files, err := ScopeToFiles(s, getProjectsLazy())
			if err != nil {
				output.Error(fmt.Sprintf("Error getting files: %v", err))
				return ToolResult{Output: *output, Error: err.Error()}
			}
			for _, file := range files {
				violations, err := AnalyzeFile(file, nil)
				if err != nil {
					continue
				}
				allViolations = append(allViolations, violations...)
			}
		}
	}
	report := AnalyzeReport{
		Timestamp: ISOTimestamp(),
		Status:    "success",
		Scope:     strings.Join(scopeRaws, " "),
		Summary: Summary{
			Total:      len(allViolations),
			ByPriority: make(map[string]int),
			ByKind:     make(map[string]int),
		},
		Violations: allViolations,
	}
	if len(allViolations) > 0 {
		report.Status = "error"
	}
	for _, v := range allViolations {
		meta := GetViolationKindMeta(v.Kind)
		report.Summary.ByPriority[string(meta.Priority)]++
		report.Summary.ByKind[string(v.Kind)]++
	}

	if len(scopeRaws) == 1 && (scopeRaws[0] == "@semio" || scopeRaws[0] == "") {
		reportsDir := filepath.Join(rootDir, "reports")
		if err := EnsureDir(reportsDir); err == nil {
			outputPath := filepath.Join(reportsDir, "violations.json")
			WriteJSONFile(outputPath, report)
		}
	}

	output.Success(fmt.Sprintf("\n📊 Analysis complete: %d violations found", len(allViolations)))
	if report.Status == "error" {
		output.ExitCode = 1
	}
	return ToolResult{Output: *output, Data: report}
}

func ToolFix(scopeRaw string) ToolResult {
	output := NewOutput()
	if scopeRaw == "" {
		scopeRaw = "@semio"
	}
	scope := ParseScope(scopeRaw)
	projects := GetProjects()
	files, _ := ScopeToFiles(scope, projects)
	var allViolations []Violation
	for _, file := range files {
		violations, err := AnalyzeFile(file, projects)
		if err != nil {
			continue
		}
		allViolations = append(allViolations, violations...)
	}
	var fixable []Violation
	for _, v := range allViolations {
		meta := GetViolationKindMeta(v.Kind)
		if meta.Autofixable && v.Autofix != nil {
			fixable = append(fixable, v)
		}
	}
	fixedFiles := make(map[string]bool)
	fixed := 0
	for _, v := range fixable {
		if v.Autofix != nil {
			for filePath, edits := range v.Autofix.Edits {
				absPath := filepath.Join(rootDir, filePath)
				content, err := ReadTextFile(absPath)
				if err != nil {
					continue
				}
				for i := len(edits) - 1; i >= 0; i-- {
					edit := edits[i]
					content = content[:edit.Start] + edit.NewText + content[edit.End:]
				}
				WriteTextFile(absPath, content)
				fixedFiles[filePath] = true
			}
			fixed++
		}
	}
	output.Success(fmt.Sprintf("\n✅ Fixed %d violations", fixed))
	return ToolResult{Output: *output}
}

func ToolPolicyList() ToolResult {
	output := NewOutput()
	policies := GetRegisteredPolicies()
	output.Info("\n📜 Registered policies:\n")
	for _, p := range policies {
		output.Plain(fmt.Sprintf("   %s", p.ID))
		output.Plain(fmt.Sprintf("      %s: %s", p.Name, p.Description))
		output.Plain(fmt.Sprintf("      Priority: %s", p.Priority))
		output.Plain("")
	}
	return ToolResult{Output: *output, Data: policies}
}

func ToolPolicyCheck(policyID, scopeRaw string) ToolResult {
	output := NewOutput()
	if scopeRaw == "" {
		scopeRaw = "@semio"
	}
	scope := ParseScope(scopeRaw)
	projects := GetProjects()
	violations, err := CheckPolicies(scope, projects, []string{policyID})
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n📊 Policy \"%s\" found %d violations", policyID, len(violations)))
	for i, v := range violations {
		output.Plain(fmt.Sprintf("   %d. %s", i+1, v.Summary))
	}
	return ToolResult{Output: *output, Data: map[string]interface{}{"violations": violations}}
}

func ToolPolicyViolationList(policyID string) ToolResult {
	output := NewOutput()
	var foundPolicy *PolicyMeta
	for _, p := range policyMetas {
		if p.ID == policyID {
			foundPolicy = &p
			break
		}
	}
	if foundPolicy == nil {
		output.Error(fmt.Sprintf("Policy '%s' not found", policyID))
		return ToolResult{Output: *output, Error: fmt.Sprintf("Policy '%s' not found", policyID)}
	}
	output.Info(fmt.Sprintf("\n📋 Violation kinds for policy '%s':", policyID))
	for _, kind := range foundPolicy.ViolationKinds {
		output.Plain(fmt.Sprintf("   - %s", kind))
	}
	return ToolResult{Output: *output, Data: foundPolicy.ViolationKinds}
}

func ToolTicketCreate(slug, prompt, model string, files []string) ToolResult {
	output := NewOutput()
	if prompt == "" {
		prompt = slug
	}
	ticket, err := CreateTicket(slug, prompt, model, files)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🎫 Created ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Path: %s", ticket.FilePath))
	output.Info(fmt.Sprintf("   First iteration started with %d file(s)", len(files)))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketList(year, month, day *int) ToolResult {
	output := NewOutput()
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n🎫 Found %d tickets:\n", len(tickets)))
	for _, t := range tickets {
		status := "🟢"
		if t.Frontmatter.Status == TicketClosed {
			status = "✅"
		}
		output.Plain(fmt.Sprintf("   %s %d/%s/%s/%s", status, t.Year, PadNumber(t.Month, 2), PadNumber(t.Day, 2), t.Slug))
		if t.Frontmatter.Summary != "" {
			output.Plain(fmt.Sprintf("      %s", t.Frontmatter.Summary))
		}
	}
	return ToolResult{Output: *output, Data: tickets}
}

func ToolTicketRead(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n🎫 Ticket: %s", ticket.Slug))
	output.Plain(fmt.Sprintf("   Status: %s", ticket.Frontmatter.Status))
	output.Plain(fmt.Sprintf("   Created: %s", ticket.Frontmatter.Date.Created))
	output.Plain(fmt.Sprintf("   Prompt: %s", ticket.Frontmatter.Prompt))
	if ticket.Frontmatter.Model != "" {
		output.Plain(fmt.Sprintf("   Model: %s", ticket.Frontmatter.Model))
	}
	output.Plain(fmt.Sprintf("\n%s", ticket.Content))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketIterateStart(year, month, day int, slug, prompt, model string, files []string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := StartIteration(ticket, prompt, model, files); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🔄 Started iteration on ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Iteration started with %d file(s)", len(files)))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketIterateEnd(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := EndIteration(ticket); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n✅ Ended iteration on ticket: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketFinish(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := FinishTicket(ticket); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n✅ Ticket finished: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketReopen(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ReopenTicket(ticket); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🔓 Ticket reopened: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolContributorAdd(github string) ToolResult {
	output := NewOutput()
	contributor, err := CreateContributor(github)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n👤 Added contributor: %s", contributor.Github))
	output.Info(fmt.Sprintf("   Path: %s", GetContributorPath(github)))
	return ToolResult{Output: *output, Data: contributor}
}

func ToolContributorList() ToolResult {
	output := NewOutput()
	contributors, err := ListContributors()
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n👥 Found %d contributors:\n", len(contributors)))
	for _, c := range contributors {
		name := c.Name
		if name == "" {
			name = c.Github
		}
		output.Plain(fmt.Sprintf("   %s (@%s)", name, c.Github))
		projectCount := len(c.Contributions.Projects)
		if projectCount > 0 {
			output.Plain(fmt.Sprintf("      Projects: %d", projectCount))
		}
	}
	return ToolResult{Output: *output, Data: contributors}
}

func ToolContributorRemove(github string) ToolResult {
	output := NewOutput()
	if err := RemoveContributor(github); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Removed contributor: %s", github))
	return ToolResult{Output: *output}
}

func ToolProjectList() ToolResult {
	output := NewOutput()
	projects := GetProjects()
	output.Info(fmt.Sprintf("\n📦 Found %d projects:\n", len(projects)))
	for _, p := range projects {
		output.Plain(fmt.Sprintf("   %s", p.Name))
		output.Plain(fmt.Sprintf("      Root: %s", p.Root))
		if len(p.Tags) > 0 {
			output.Plain(fmt.Sprintf("      Tags: %s", strings.Join(p.Tags, ", ")))
		}
	}
	return ToolResult{Output: *output, Data: projects}
}

func ToolProjectTree() ToolResult {
	output := NewOutput()
	projects := GetProjects()
	output.Info("\n📦 Project tree:\n")
	for _, p := range projects {
		output.Plain(fmt.Sprintf("   └── %s (%s)", p.Name, p.Root))
	}
	return ToolResult{Output: *output, Data: projects}
}

func ToolFolderCreate(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder already exists: %s", path))
		return ToolResult{Output: *output, Error: "folder already exists"}
	}
	if err := EnsureDir(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📁 Created folder: %s", path))
	return ToolResult{Output: *output}
}

func ToolFolderMove(source, target string) ToolResult {
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		output.Error(fmt.Sprintf("Error: Source folder not found: %s", source))
		return ToolResult{Output: *output, Error: "source not found"}
	}
	if FileExists(absTarget) {
		output.Error(fmt.Sprintf("Error: Target folder already exists: %s", target))
		return ToolResult{Output: *output, Error: "target exists"}
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📁 Moved folder: %s → %s", source, target))
	return ToolResult{Output: *output}
}

func ToolFolderDelete(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	if err := os.RemoveAll(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted folder: %s", path))
	return ToolResult{Output: *output}
}

func ToolFolderList(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	folders, err := ListDirEntries(absPath, true)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	var relPaths []string
	for _, f := range folders {
		relPaths = append(relPaths, NormalizePath(filepath.Join(path, f)))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []string
	for _, f := range folders {
		relPath := NormalizePath(filepath.Join(path, f))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, f)
		}
	}
	output.Info(fmt.Sprintf("\n📁 Found %d folders in %s:\n", len(filtered), path))
	for _, f := range filtered {
		output.Plain(fmt.Sprintf("   %s/", f))
	}
	return ToolResult{Output: *output, Data: filtered}
}

func ToolFolderTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	output.Info(fmt.Sprintf("\n📁 Folder tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}

func printTree(output *CommandOutput, dir, prefix string) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return
	}
	var items []os.DirEntry
	for _, e := range entries {
		if !strings.HasPrefix(e.Name(), ".") {
			items = append(items, e)
		}
	}
	var relPaths []string
	for _, e := range items {
		relPaths = append(relPaths, GetRelativePath(filepath.Join(dir, e.Name())))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []os.DirEntry
	for _, e := range items {
		relPath := GetRelativePath(filepath.Join(dir, e.Name()))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, e)
		}
	}
	for i, e := range filtered {
		isLast := i == len(filtered)-1
		connector := "├── "
		if isLast {
			connector = "└── "
		}
		suffix := ""
		if e.IsDir() {
			suffix = "/"
		}
		output.Plain(fmt.Sprintf("%s%s%s%s", prefix, connector, e.Name(), suffix))
		if e.IsDir() {
			newPrefix := prefix + "│   "
			if isLast {
				newPrefix = prefix + "    "
			}
			printTree(output, filepath.Join(dir, e.Name()), newPrefix)
		}
	}
}

func ToolFileCreate(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File already exists: %s", path))
		return ToolResult{Output: *output, Error: "file already exists"}
	}
	lang := GetLanguageFromPath(path)
	content := generateFileHeader(path, lang)
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Created file: %s", path))
	return ToolResult{Output: *output}
}

func generateFileHeader(path, lang string) string {
	gitAuthor := GetGitAuthor()
	year := strconv.Itoa(time.Now().Year())
	license := `This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.`
	switch lang {
	case "typescript":
		return fmt.Sprintf(`// #region Header

// %s

// %s %s

%s

// #endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "//"))
	case "python":
		return fmt.Sprintf(`# region Header

# %s

# %s %s

%s

# endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "#"))
	case "csharp":
		return fmt.Sprintf(`#region Header

// %s

// %s %s

%s

#endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "//"))
	case "go":
		return fmt.Sprintf(`// region Header

// %s

// %s %s

%s

// endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "//"))
	default:
		return ""
	}
}

func formatLicenseLines(license, prefix string) string {
	lines := strings.Split(license, "\n")
	var formatted []string
	for _, line := range lines {
		if line == "" {
			formatted = append(formatted, prefix)
		} else {
			formatted = append(formatted, prefix+" "+line)
		}
	}
	return strings.Join(formatted, "\n")
}

func ToolFileMove(source, target string) ToolResult {
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		output.Error(fmt.Sprintf("Error: Source file not found: %s", source))
		return ToolResult{Output: *output, Error: "source not found"}
	}
	if FileExists(absTarget) {
		output.Error(fmt.Sprintf("Error: Target file already exists: %s", target))
		return ToolResult{Output: *output, Error: "target exists"}
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Moved file: %s → %s", source, target))
	return ToolResult{Output: *output}
}

func ToolFileDelete(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", path))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	if err := os.Remove(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted file: %s", path))
	return ToolResult{Output: *output}
}

func ToolFileList(scopeRaw string) ToolResult {
	output := NewOutput()
	scope := ParseScope(scopeRaw)
	projects := GetProjects()
	files, err := ScopeToFiles(scope, projects)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n📄 Found %d files in scope \"%s\":\n", len(files), scopeRaw))
	for i, f := range files {
		if i >= 50 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(files)-50))
			break
		}
		output.Plain(fmt.Sprintf("   %s", f))
	}
	return ToolResult{Output: *output, Data: files}
}

func ToolFileTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Path not found: %s", path))
		return ToolResult{Output: *output, Error: "path not found"}
	}
	output.Info(fmt.Sprintf("\n📄 File tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}

func ToolSectionCreate(filePath, sectionPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	lang := GetLanguageFromPath(filePath)
	var newSection string
	switch lang {
	case "markdown":
		newSection = fmt.Sprintf("\n## %s\n\n", sectionName)
	case "typescript":
		newSection = fmt.Sprintf("\n// #region %s\n\n// #endregion %s\n", sectionName, sectionName)
	case "python":
		newSection = fmt.Sprintf("\n# region %s\n\n# endregion %s\n", sectionName, sectionName)
	case "csharp":
		newSection = fmt.Sprintf("\n#region %s\n\n#endregion %s\n", sectionName, sectionName)
	default:
		output.Error("Error: Unsupported file type")
		return ToolResult{Output: *output, Error: "unsupported file type"}
	}
	if err := WriteTextFile(absPath, content+newSection); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🏷️ Created section \"%s\" in %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionMove(filePath, oldPath, newPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	oldParts := strings.Split(oldPath, "#")
	oldName := oldParts[len(oldParts)-1]
	newParts := strings.Split(newPath, "#")
	newName := newParts[len(newParts)-1]
	lang := GetLanguageFromPath(filePath)
	switch lang {
	case "markdown":
		content = strings.ReplaceAll(content, "# "+oldName, "# "+newName)
		content = strings.ReplaceAll(content, "## "+oldName, "## "+newName)
	case "typescript":
		content = strings.ReplaceAll(content, "// #region "+oldName, "// #region "+newName)
		content = strings.ReplaceAll(content, "// #endregion "+oldName, "// #endregion "+newName)
	case "python":
		content = strings.ReplaceAll(content, "# region "+oldName, "# region "+newName)
		content = strings.ReplaceAll(content, "# endregion "+oldName, "# endregion "+newName)
	case "csharp":
		content = strings.ReplaceAll(content, "#region "+oldName, "#region "+newName)
		content = strings.ReplaceAll(content, "#endregion "+oldName, "#endregion "+newName)
	}
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🏷️ Renamed section \"%s\" to \"%s\" in %s", oldName, newName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionDelete(filePath, sectionPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, filePath)
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	section := FindSection(sections, sectionName)
	if section == nil {
		output.Error(fmt.Sprintf("Error: Section not found: %s", sectionName))
		return ToolResult{Output: *output, Error: "section not found"}
	}
	lines := strings.Split(content, "\n")
	var newLines []string
	for i, line := range lines {
		lineNum := i + 1
		if lineNum < section.StartLine || lineNum > section.EndLine {
			newLines = append(newLines, line)
		}
	}
	if err := WriteTextFile(absPath, strings.Join(newLines, "\n")); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionList(filePath string) ToolResult {
	output := NewOutput()
	scope := ParseScope(filePath)
	if scope.Kind != ScopeFile && scope.Kind != ScopeSection {
		output.Error("Error: Scope must be a file or section")
		return ToolResult{Output: *output, Error: "invalid scope"}
	}
	absPath := filepath.Join(rootDir, scope.FilePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", scope.FilePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, scope.FilePath)
	output.Info(fmt.Sprintf("\n🏷️ Sections in %s:\n", scope.FilePath))
	var printSection func(s SectionInfo, indent string)
	printSection = func(s SectionInfo, indent string) {
		output.Plain(fmt.Sprintf("%s%s (lines %d-%d)", indent, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, indent+"  ")
		}
	}
	for _, s := range sections {
		printSection(s, "   ")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

func ToolSectionTree(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.Split(filePath, "#")[0])
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, filePath)
	output.Info(fmt.Sprintf("\n🏷️ Sections in %s:\n", filePath))
	var printSection func(s SectionInfo, prefix string)
	printSection = func(s SectionInfo, prefix string) {
		output.Plain(fmt.Sprintf("%s└── %s (lines %d-%d)", prefix, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, prefix+"    ")
		}
	}
	for _, s := range sections {
		printSection(s, "")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

func ToolDefinitionList(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	output.Info(fmt.Sprintf("\n📋 Definitions in %s:\n", filePath))
	output.Plain("   (definition parsing not implemented in Go - use TypeScript API)")
	return ToolResult{Output: *output, Data: []DefinitionInfo{}}
}

func ToolDefinitionTree(filePath string) ToolResult {
	return ToolDefinitionList(filePath)
}

// #endregion Commands

func main() {
	if err := Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

