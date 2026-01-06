// #region Header

// go/repo/repo.go

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

// #endregion Header
package repo

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/bmatcuk/doublestar/v4"
	"github.com/usalu/semio/go/repo/graph"
	"gopkg.in/yaml.v3"
)

// #region Types

type ScopeKind string

const (
	ScopeRepo       ScopeKind = "repo"
	ScopeProject    ScopeKind = "bundle"
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

type Fix struct {
	Description string                     `json:"description"`
	Edits       map[string][]graph.TextEdit `json:"edits"`
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

type Bundle struct {
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

type DefinitionInfo struct {
	Name       string              `json:"name"`
	Kind       graph.DefinitionKind `json:"kind"`
	StartLine  int                 `json:"startLine"`
	EndLine    int                 `json:"endLine"`
	StartIndex int                 `json:"startIndex"`
	EndIndex   int                 `json:"endIndex"`
}

type TicketSectionMetrics struct {
	Definitions []string           `yaml:"definitions,omitempty" json:"definitions,omitempty"`
	Lines       *graph.LineMetrics `yaml:"lines" json:"lines"`
}

type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

type TicketBundles map[string]TicketBundleMetrics

type LanguagePlugin interface {
	Name() string
	Extensions() []string
	MatchesExtension(ext string) bool
	SupportsSections() bool
	SupportsDefinitions() bool
	SupportsComments() bool
	SupportsHeaders() bool
	UsesIndentScoping() bool
	CommentPrefix() string
	ParseSections(content string) []SectionInfo
	ParseDefinitions(content string, lines []string) []DefinitionRange
	FormatSectionStart(name string) string
	FormatSectionEnd(name string) string
	FormatSectionBoth(name string) string
	FormatHeader(filePath, year, author, license string) string
	PolicySectionStartMatch(line string) (matched bool, name string)
	PolicySectionEndMatch(line string) (matched bool, name string)
	ExtraOrphanDefinitions(lines []string) []DefinitionRange
	ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation
}

type DefinitionRange struct {
	Name    string
	Start   int
	End     int
	Excerpt string
}

type BaseLanguage struct {
	name               string
	extensions         []string
	sectionStart       *regexp.Regexp
	sectionEnd         *regexp.Regexp
	definitionRegexp   *regexp.Regexp
	commentPrefix      string
	sectionStartFmt    string
	sectionEndFmt      string
	sectionBothFmt     string
	headerFmt          string
	usesIndentScoping  bool
	policySectionStart *regexp.Regexp
	policySectionEnd   *regexp.Regexp
}

func (l *BaseLanguage) Name() string                    { return l.name }
func (l *BaseLanguage) Extensions() []string            { return l.extensions }
func (l *BaseLanguage) CommentPrefix() string           { return l.commentPrefix }
func (l *BaseLanguage) UsesIndentScoping() bool         { return l.usesIndentScoping }
func (l *BaseLanguage) SupportsSections() bool          { return l.sectionStart != nil }
func (l *BaseLanguage) SupportsDefinitions() bool       { return l.definitionRegexp != nil }
func (l *BaseLanguage) SupportsComments() bool          { return l.commentPrefix != "" }
func (l *BaseLanguage) SupportsHeaders() bool           { return l.headerFmt != "" }

func (l *BaseLanguage) MatchesExtension(ext string) bool {
	ext = strings.ToLower(ext)
	for _, langExt := range l.extensions {
		if ext == langExt {
			return true
		}
	}
	return false
}

func (l *BaseLanguage) FormatSectionStart(name string) string {
	if l.sectionStartFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionStartFmt, name)
}

func (l *BaseLanguage) FormatSectionEnd(name string) string {
	if l.sectionEndFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionEndFmt, name)
}

func (l *BaseLanguage) FormatSectionBoth(name string) string {
	if l.sectionBothFmt == "" {
		return ""
	}
	if l.sectionEndFmt == "" {
		return fmt.Sprintf(l.sectionBothFmt, name)
	}
	return fmt.Sprintf(l.sectionBothFmt, name, name)
}

func (l *BaseLanguage) FormatHeader(filePath, year, author, license string) string {
	if l.headerFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.headerFmt, filePath, year, author, license)
}

func (l *BaseLanguage) PolicySectionStartMatch(line string) (bool, string) {
	if l.policySectionStart == nil {
		return false, ""
	}
	match := l.policySectionStart.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		name = strings.TrimSpace(match[1])
	}
	return true, name
}

func (l *BaseLanguage) PolicySectionEndMatch(line string) (bool, string) {
	if l.policySectionEnd == nil {
		return false, ""
	}
	match := l.policySectionEnd.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		name = strings.TrimSpace(match[1])
	}
	return true, name
}

func (l *BaseLanguage) ParseSections(content string) []SectionInfo {
	if l.sectionStart == nil {
		return nil
	}
	lines := strings.Split(content, "\n")
	var stack []*SectionInfo
	var roots []SectionInfo
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := l.sectionStart.FindStringSubmatch(line); match != nil {
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
		} else if l.sectionEnd != nil && l.sectionEnd.MatchString(line) {
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

func (l *BaseLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name string
		line int
	}
	var defStarts []defStart
	for i, line := range lines {
		matches := l.definitionRegexp.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if len(match) > 1 && match[1] != "" {
				defStarts = append(defStarts, defStart{name: match[1], line: i + 1})
			}
		}
	}
	var defRanges []DefinitionRange
	for i := 0; i < len(defStarts); i++ {
		start := defStarts[i].line
		end := start
		if l.usesIndentScoping {
			startIndent := len(lines[start-1]) - len(strings.TrimLeft(lines[start-1], " \t"))
			for lineIndex := start; lineIndex < len(lines); lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				currentIndent := len(line) - len(strings.TrimLeft(line, " \t"))
				if currentIndent <= startIndent {
					end = lineIndex
					break
				}
				end = lineIndex + 1
			}
		} else {
			braceDepth := 0
			sawOpen := false
			for lineIndex := start - 1; lineIndex < len(lines); lineIndex++ {
				line := lines[lineIndex]
				for _, ch := range line {
					if ch == '{' {
						braceDepth++
						sawOpen = true
					} else if ch == '}' {
						if braceDepth > 0 {
							braceDepth--
						}
						if sawOpen && braceDepth == 0 {
							end = lineIndex + 1
							lineIndex = len(lines)
							break
						}
					}
				}
				if sawOpen && braceDepth == 0 && end > start {
					break
				}
			}
			if !sawOpen {
				if i+1 < len(defStarts) {
					end = defStarts[i+1].line - 1
				}
			}
		}
		if end < start {
			end = start
		}
		defRanges = append(defRanges, DefinitionRange{
			Name:    defStarts[i].name,
			Start:   start,
			End:     end,
			Excerpt: defStarts[i].name,
		})
	}
	return defRanges
}

func (l *BaseLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	return nil
}

func (l *BaseLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation {
	return nil
}

type TypeScriptLanguage struct {
	BaseLanguage
}

func NewTypeScriptLanguage() *TypeScriptLanguage {
	return &TypeScriptLanguage{
		BaseLanguage: BaseLanguage{
			name:               "typescript",
			extensions:         []string{".ts", ".tsx", ".js", ".jsx"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:export\s+)?(?:const|let|var|function|class|interface|type|enum)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			sectionStartFmt:    "// #region %s",
			sectionEndFmt:      "// #endregion %s",
			sectionBothFmt:     "\n// #region %s\n\n// #endregion %s\n",
			headerFmt:          "// #region Header\n\n// %s\n\n// %s %s\n\n%s\n\n// #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *TypeScriptLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation {
	var violations []Violation
	// Find header section to exclude comments within it
	sections := l.ParseSections(content)
	var headerSection *SectionInfo
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	charIndex := 0
	scanState := CommentScanState{}
	for i, line := range lines {
		lineNum := i + 1
		// Skip comments inside header section
		if headerSection != nil && lineNum >= headerSection.StartLine && lineNum <= headerSection.EndLine {
			charIndex += len(line) + 1
			continue
		}
		lineStart := charIndex
		j := 0
		for j < len(line) {
			if scanState.InBlockComment {
				if j+1 < len(line) && line[j] == '*' && line[j+1] == '/' {
					if scanState.BlockCommentIsJsDoc {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("JSDoc comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentJSDoc,
							file, scanState.BlockCommentStartLine, "", &Fix{
								Description: "Remove JSDoc comment",
								Edits: map[string][]graph.TextEdit{
									file: {{Start: scanState.BlockCommentStartIndex, End: lineStart + j + 2, NewText: ""}},
								},
							}))
					} else {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentBlock,
							file, scanState.BlockCommentStartLine, "", &Fix{
								Description: "Remove block comment",
								Edits: map[string][]graph.TextEdit{
									file: {{Start: scanState.BlockCommentStartIndex, End: lineStart + j + 2, NewText: ""}},
								},
							}))
					}
					scanState.InBlockComment = false
					j += 2
					continue
				}
				j++
				continue
			}
			if scanState.Escaped {
				scanState.Escaped = false
				j++
				continue
			}
			if line[j] == '\\' && (scanState.InSingleQuote || scanState.InDoubleQuote || scanState.InTemplateRaw()) {
				scanState.Escaped = true
				j++
				continue
			}
			if scanState.InSingleQuote {
				if line[j] == '\'' {
					scanState.InSingleQuote = false
				}
				j++
				continue
			}
			if scanState.InDoubleQuote {
				if line[j] == '"' {
					scanState.InDoubleQuote = false
				}
				j++
				continue
			}
			if scanState.InTemplateRaw() {
				if line[j] == '`' {
					scanState.Templates = scanState.Templates[:len(scanState.Templates)-1]
					j++
					continue
				}
				if j+1 < len(line) && line[j] == '$' && line[j+1] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth = 1
					j += 2
					continue
				}
				j++
				continue
			}
			if len(scanState.Templates) > 0 && scanState.Templates[len(scanState.Templates)-1].ExprDepth > 0 {
				if line[j] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth++
					j++
					continue
				}
				if line[j] == '}' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth--
					j++
					continue
				}
			}
			if line[j] == '\'' {
				scanState.InSingleQuote = true
				j++
				continue
			}
			if line[j] == '"' {
				scanState.InDoubleQuote = true
				j++
				continue
			}
			if line[j] == '`' {
				scanState.Templates = append(scanState.Templates, CommentTemplateState{ExprDepth: 0})
				j++
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '*' {
				isJsDoc := j+2 < len(line) && line[j+2] == '*'
				scanState.InBlockComment = true
				scanState.BlockCommentStartLine = lineNum
				scanState.BlockCommentStartIndex = lineStart + j
				scanState.BlockCommentIsJsDoc = isJsDoc
				j += 2
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '/' {
				// Skip URL schemes like http://, https://, ftp://, etc.
				if j > 0 && line[j-1] == ':' {
					j += 2
					continue
				}
				// Skip escaped slashes in regex like /pattern\//
				if j > 0 && line[j-1] == '\\' {
					j += 2
					continue
				}
				trimmed := strings.TrimSpace(line)
				if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") {
					break
				}
				if strings.HasPrefix(trimmed, "// eslint-") || strings.HasPrefix(trimmed, "// @ts-") || strings.HasPrefix(trimmed, "// noinspection") || strings.HasPrefix(trimmed, "// TODO") || strings.HasPrefix(trimmed, "// semio-ignore-") {
					break
				}
				debugMarker := strings.Contains(line, "[DEBUG]")
				if !debugMarker {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
						ViolationCodeCommentInline,
						file, lineNum, strings.TrimSpace(line[j:]), &Fix{
							Description: "Remove inline comment",
							Edits: map[string][]graph.TextEdit{
								file: {{Start: lineStart + j, End: lineStart + len(line), NewText: ""}},
							},
						}))
				}
				break
			}
			j++
		}
		charIndex += len(line) + 1
	}
	return violations
}

type GoLanguage struct {
	BaseLanguage
}

func NewGoLanguage() *GoLanguage {
	return &GoLanguage{
		BaseLanguage: BaseLanguage{
			name:               "go",
			extensions:         []string{".go"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			sectionStartFmt:    "// #region %s",
			sectionEndFmt:      "// #endregion %s",
			sectionBothFmt:     "\n// #region %s\n\n// #endregion %s\n",
			headerFmt:          "// #region Header\n\n// %s\n\n// %s %s\n\n%s\n\n// #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *GoLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "package ") {
			name := fmt.Sprintf("package-%d", i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: trimmed})
			break
		}
	}
	for i := 0; i < len(lines); i++ {
		trimmed := strings.TrimSpace(lines[i])
		if strings.HasPrefix(trimmed, "import ") {
			start := i + 1
			end := start
			if strings.HasPrefix(trimmed, "import (") {
				for j := i + 1; j < len(lines); j++ {
					if strings.TrimSpace(lines[j]) == ")" {
						end = j + 1
						i = j
						break
					}
				}
			}
			name := fmt.Sprintf("import-%d", start)
			defs = append(defs, DefinitionRange{Name: name, Start: start, End: end, Excerpt: strings.TrimSpace(lines[start-1])})
		}
	}
	return defs
}

type PythonLanguage struct {
	BaseLanguage
}

func NewPythonLanguage() *PythonLanguage {
	return &PythonLanguage{
		BaseLanguage: BaseLanguage{
			name:               "python",
			extensions:         []string{".py"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:def|class|async\s+def)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  true,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

type CSharpLanguage struct {
	BaseLanguage
}

func NewCSharpLanguage() *CSharpLanguage {
	return &CSharpLanguage{
		BaseLanguage: BaseLanguage{
			name:               "csharp",
			extensions:         []string{".cs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:public|private|protected|internal|static|partial|abstract|sealed|virtual|override|async)*\s*(?:class|struct|interface|enum|delegate|record|void|string|int|bool|[A-Z][A-Za-z0-9_<>]*)\s+([A-Z][A-Za-z0-9_]*)\s*[<({]`),
			commentPrefix:      "//",
			sectionStartFmt:    "#region %s",
			sectionEndFmt:      "#endregion %s",
			sectionBothFmt:     "\n#region %s\n\n#endregion %s\n",
			headerFmt:          "#region Header\n\n// %s\n\n// %s %s\n\n%s\n\n#endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

type JSONLanguage struct {
	BaseLanguage
}

func NewJSONLanguage() *JSONLanguage {
	return &JSONLanguage{
		BaseLanguage: BaseLanguage{
			name:              "json",
			extensions:        []string{".json"},
			commentPrefix:     "",
			usesIndentScoping: false,
		},
	}
}

func (l *JSONLanguage) SupportsSections() bool    { return true }
func (l *JSONLanguage) SupportsDefinitions() bool { return false }
func (l *JSONLanguage) SupportsComments() bool    { return false }
func (l *JSONLanguage) SupportsHeaders() bool     { return false }

func (l *JSONLanguage) ParseSections(content string) []SectionInfo {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

type MarkdownLanguage struct {
	BaseLanguage
}

func NewMarkdownLanguage() *MarkdownLanguage {
	return &MarkdownLanguage{
		BaseLanguage: BaseLanguage{
			name:              "markdown",
			extensions:        []string{".md", ".mdx"},
			sectionStart:      regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`),
			commentPrefix:     "",
			sectionStartFmt:   "## %s",
			sectionEndFmt:     "",
			sectionBothFmt:    "\n## %s\n\n",
			headerFmt:         "",
			usesIndentScoping: false,
		},
	}
}

func (l *MarkdownLanguage) SupportsSections() bool    { return true }
func (l *MarkdownLanguage) SupportsDefinitions() bool { return false }
func (l *MarkdownLanguage) SupportsComments() bool    { return false }
func (l *MarkdownLanguage) SupportsHeaders() bool     { return false }

func (l *MarkdownLanguage) ParseSections(content string) []SectionInfo {
	return ParseMarkdownSectionsInternal(content)
}

var languageRegistry = []LanguagePlugin{
	NewTypeScriptLanguage(),
	NewGoLanguage(),
	NewPythonLanguage(),
	NewCSharpLanguage(),
	NewJSONLanguage(),
	NewMarkdownLanguage(),
}

func GetLanguage(filePath string) LanguagePlugin {
	ext := strings.ToLower(filepath.Ext(filePath))
	for _, lang := range languageRegistry {
		if lang.MatchesExtension(ext) {
			return lang
		}
	}
	return nil
}

func GetLanguageByName(name string) LanguagePlugin {
	for _, lang := range languageRegistry {
		if lang.Name() == name {
			return lang
		}
	}
	return nil
}

type TicketIterationFiles struct {
	Updated []FileLineMetrics `yaml:"updated,omitempty" json:"updated,omitempty"`
	Created []FileLineMetrics `yaml:"created,omitempty" json:"created,omitempty"`
	Removed []FileLineMetrics `yaml:"removed,omitempty" json:"removed,omitempty"`
}

type FileLineMetrics struct {
	Path  string     `yaml:"path" json:"path"`
	Lines *graph.LineMetrics `yaml:"lines,omitempty" json:"lines,omitempty"`
}

type TicketDate struct {
	Started string `yaml:"started,omitempty" json:"started,omitempty"`
	Ended   string `yaml:"ended,omitempty" json:"ended,omitempty"`
}

type TicketIteration struct {
	Prompt   string                `yaml:"prompt" json:"prompt"`
	Model    string                `yaml:"model,omitempty" json:"model,omitempty"`
	Date     TicketDate            `yaml:"date" json:"date"`
	Author   string                `yaml:"author,omitempty" json:"author,omitempty"`
	Commit   string                `yaml:"commit,omitempty" json:"commit,omitempty"`
	Ignore   bool                  `yaml:"ignore,omitempty" json:"ignore,omitempty"`
	Declared *TicketIterationFiles `yaml:"declared,omitempty" json:"declared,omitempty"`
	Bundles  TicketBundles         `yaml:"bundles,omitempty" json:"bundles,omitempty"`
}

type TicketFiles struct {
	Updated []FileLineMetrics `yaml:"updated,omitempty" json:"updated,omitempty"`
	Created []FileLineMetrics `yaml:"created,omitempty" json:"created,omitempty"`
	Removed []FileLineMetrics `yaml:"removed,omitempty" json:"removed,omitempty"`
}

type TicketFrontmatter struct {
	Slug       string             `yaml:"slug" json:"slug"`
	Prompt     string             `yaml:"prompt" json:"prompt"`
	Summary    string             `yaml:"summary,omitempty" json:"summary,omitempty"`
	Status     graph.TicketStatus `yaml:"status" json:"status"`
	Author     string             `yaml:"author,omitempty" json:"author,omitempty"`
	Date       TicketDateCreated  `yaml:"date" json:"date"`
	Commit     string             `yaml:"commit,omitempty" json:"commit,omitempty"`
	Model      string             `yaml:"model,omitempty" json:"model,omitempty"`
	Ignore     bool               `yaml:"ignore,omitempty" json:"ignore,omitempty"`
	Iterations []TicketIteration  `yaml:"iterations,omitempty" json:"iterations,omitempty"`
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
	FolderPath  string            `json:"folderPath"`
	FilePath    string            `json:"filePath"`
}

type ViolationKind string

const (
	ViolationCodeHeaderMissingRegion       ViolationKind = "code:header:missing-region"
	ViolationCodeHeaderMissingFilename     ViolationKind = "code:header:missing-filename"
	ViolationCodeHeaderMissingContributors ViolationKind = "code:header:missing-contributors"
	ViolationCodeHeaderMissingLicense      ViolationKind = "code:header:missing-license"
	ViolationCodeHeaderWrongLicense        ViolationKind = "code:header:wrong-license"
	ViolationCodeSectionEmpty              ViolationKind = "code:section:empty"
	ViolationCodeSectionOrphanDefinition   ViolationKind = "code:section:orphan-definition"
	ViolationCodeSectionMissingStartName   ViolationKind = "code:section:missing-start-name"
	ViolationCodeSectionMissingEndName     ViolationKind = "code:section:missing-end-name"
	ViolationCodeSectionNameMismatch       ViolationKind = "code:section:name-mismatch"
	ViolationCodeCommentInline             ViolationKind = "code:comment:inline"
	ViolationCodeCommentBlock              ViolationKind = "code:comment:block"
	ViolationCodeCommentJSDoc              ViolationKind = "code:comment:jsdoc"
	ViolationDevDocsMissingFile            ViolationKind = "dev-docs:missing-file"
	ViolationDevDocsMissingFolder          ViolationKind = "dev-docs:missing-folder"
	ViolationDevDocsWrongFilePath          ViolationKind = "dev-docs:wrong-file-path"
	ViolationDevDocsWrongFolderPath        ViolationKind = "dev-docs:wrong-folder-path"
	ViolationDevDocsWrongFileName          ViolationKind = "dev-docs:wrong-file-name"
	ViolationDevDocsWrongFolderName        ViolationKind = "dev-docs:wrong-folder-name"
	ViolationDevDocsWrongFileOrder         ViolationKind = "dev-docs:wrong-file-order"
	ViolationDevDocsWrongFolderOrder       ViolationKind = "dev-docs:wrong-folder-order"
	ViolationDevDocsMissingComponent       ViolationKind = "dev-docs:missing-component"
	ViolationDevDocsWrongComponentName     ViolationKind = "dev-docs:wrong-component-name"
	ViolationDevDocsWrongComponentOrder    ViolationKind = "dev-docs:wrong-component-order"
	ViolationSketchpadImportThirdParty     ViolationKind = "sketchpad:import:third-party-outside-elements"
	ViolationSketchpadStateMultipleMachines ViolationKind = "sketchpad:state:multiple-machines"
	ViolationSketchpadStateCreateActor     ViolationKind = "sketchpad:state:create-actor-usage"
	ViolationSketchpadStateYjsAppState     ViolationKind = "sketchpad:state:yjs-app-state"
	ViolationSketchpadStateForbiddenStore  ViolationKind = "sketchpad:state:forbidden-store"
	ViolationSketchpadHooksNonTriadic      ViolationKind = "sketchpad:hooks:non-triadic"
)

type ViolationKindInfo struct {
	Kind        ViolationKind            `json:"kind"`
	Priority    graph.ViolationPriority  `json:"priority"`
	Reason      string                   `json:"reason"`
	Solution    string                   `json:"solution"`
	Autofixable bool                     `json:"autofixable"`
}

var violationKindInfoTable = map[ViolationKind]ViolationKindInfo{
	ViolationCodeHeaderMissingRegion: {
		Kind:        ViolationCodeHeaderMissingRegion,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingFilename: {
		Kind:        ViolationCodeHeaderMissingFilename,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Filename must be documented in header",
		Solution:    "Add filename comment in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingContributors: {
		Kind:        ViolationCodeHeaderMissingContributors,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingLicense: {
		Kind:        ViolationCodeHeaderMissingLicense,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "SPDX license identifier is required",
		Solution:    "Add SPDX license header comment",
		Autofixable: false,
	},
	ViolationCodeHeaderWrongLicense: {
		Kind:        ViolationCodeHeaderWrongLicense,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Update license to AGPL-3.0-or-later",
		Autofixable: false,
	},
	ViolationCodeSectionEmpty: {
		Kind:        ViolationCodeSectionEmpty,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	ViolationCodeSectionOrphanDefinition: {
		Kind:        ViolationCodeSectionOrphanDefinition,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "All code must be inside named sections",
		Solution:    "Move code into an existing section or add a new section",
		Autofixable: false,
	},
	ViolationCodeSectionMissingStartName: {
		Kind:        ViolationCodeSectionMissingStartName,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	ViolationCodeSectionMissingEndName: {
		Kind:        ViolationCodeSectionMissingEndName,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	ViolationCodeSectionNameMismatch: {
		Kind:        ViolationCodeSectionNameMismatch,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	ViolationCodeCommentInline: {
		Kind:        ViolationCodeCommentInline,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Inline comments are forbidden",
		Solution:    "Remove inline comment",
		Autofixable: true,
	},
	ViolationCodeCommentBlock: {
		Kind:        ViolationCodeCommentBlock,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Block comments are forbidden",
		Solution:    "Remove block comment",
		Autofixable: true,
	},
	ViolationCodeCommentJSDoc: {
		Kind:        ViolationCodeCommentJSDoc,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "JSDoc comments are forbidden",
		Solution:    "Remove JSDoc comment",
		Autofixable: true,
	},
	ViolationDevDocsMissingFile: {
		Kind:        ViolationDevDocsMissingFile,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsMissingFolder: {
		Kind:        ViolationDevDocsMissingFolder,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsWrongFilePath: {
		Kind:        ViolationDevDocsWrongFilePath,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderPath: {
		Kind:        ViolationDevDocsWrongFolderPath,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileName: {
		Kind:        ViolationDevDocsWrongFileName,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄 PATH)",
		Solution:    "Rename section to ## 📄 PATH",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderName: {
		Kind:        ViolationDevDocsWrongFolderName,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁 PATH/)",
		Solution:    "Rename section to ## 📁 PATH/",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileOrder: {
		Kind:        ViolationDevDocsWrongFileOrder,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderOrder: {
		Kind:        ViolationDevDocsWrongFolderOrder,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsMissingComponent: {
		Kind:        ViolationDevDocsMissingComponent,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentName: {
		Kind:        ViolationDevDocsWrongComponentName,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentOrder: {
		Kind:        ViolationDevDocsWrongComponentOrder,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
	ViolationSketchpadImportThirdParty: {
		Kind:        ViolationSketchpadImportThirdParty,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "Third party imports must only be in elements.tsx",
		Solution:    "Move third party import to elements.tsx and re-export from there",
		Autofixable: false,
	},
	ViolationSketchpadStateMultipleMachines: {
		Kind:        ViolationSketchpadStateMultipleMachines,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "Only one state machine is allowed (createMachine can only be used once)",
		Solution:    "Consolidate state management into a single state machine",
		Autofixable: false,
	},
	ViolationSketchpadStateCreateActor: {
		Kind:        ViolationSketchpadStateCreateActor,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "createActor is forbidden in sketchpad",
		Solution:    "Remove createActor usage and use the single state machine instead",
		Autofixable: false,
	},
	ViolationSketchpadStateYjsAppState: {
		Kind:        ViolationSketchpadStateYjsAppState,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "Yjs should only be used for kit data synchronization, not app state",
		Solution:    "Move app state to the state machine and use Yjs only for kit data sync",
		Autofixable: false,
	},
	ViolationSketchpadStateForbiddenStore: {
		Kind:        ViolationSketchpadStateForbiddenStore,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "Stores outside of State Management sections are forbidden",
		Solution:    "Move store to a State Management section or remove it",
		Autofixable: false,
	},
	ViolationSketchpadHooksNonTriadic: {
		Kind:        ViolationSketchpadHooksNonTriadic,
		Priority:    graph.ViolationPriorityHigh,
		Reason:      "UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()",
		Solution:    "Refactor to use triadic hook pattern with useSELECTOR",
		Autofixable: false,
	},
}

func (k ViolationKind) Info() ViolationKindInfo {
	if info, ok := violationKindInfoTable[k]; ok {
		return info
	}
	return ViolationKindInfo{
		Kind:        k,
		Priority:    graph.ViolationPriorityLow,
		Reason:      "Unknown violation",
		Solution:    "Fix the violation",
		Autofixable: false,
	}
}

type Policy struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Scopes      []string          `json:"scopes"`
	Priority    graph.ViolationPriority `json:"priority"`
	Kinds       []ViolationKind   `json:"kinds"`
	Run         PolicyFunc        `json:"-"`
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

type ContributorTicket struct {
	Year     int                `json:"year"`
	Month    int                `json:"month"`
	Day      int                `json:"day"`
	Slug     string             `json:"slug"`
	Status   graph.TicketStatus `json:"status"`
	FilePath string             `json:"filePath,omitempty"`
}

type ContributorCommit struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

type ContributorContributions struct {
	Bundles    []string `json:"bundles,omitempty"`
	Folders     []string `json:"folders,omitempty"`
	Files       []string `json:"files,omitempty"`
	Regions     []string `json:"regions,omitempty"`
	Definitions []string `json:"definitions,omitempty"`
	Tickets     []ContributorTicket `json:"tickets,omitempty"`
	Commits     []ContributorCommit `json:"commits,omitempty"`
	Lines       *graph.LineMetrics           `json:"lines,omitempty"`
}

// #region Codebase Types

type CountMetrics struct {
	Added   int `json:"added,omitempty"`
	Updated int `json:"updated,omitempty"`
	Removed int `json:"removed,omitempty"`
}

type BundleMetricsInfo struct {
	Folders    int `json:"folders"`
	Files      int `json:"files"`
	Sections   int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FolderMetricsInfo struct {
	Files      int `json:"files"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FileMetricsInfo struct {
	Sections   int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines      int `json:"lines"`
}

type SectionMetricsInfo struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type DefinitionMetricsInfo struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type RangePosition struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

type FileRange struct {
	Start RangePosition `json:"start"`
	End   RangePosition `json:"end"`
}

type ViolationFile struct {
	ID    string    `json:"id"`
	Path  string    `json:"path"`
	URI   string    `json:"uri"`
	Range *FileRange `json:"range,omitempty"`
}

type ViolationFolder struct {
	ID   string `json:"id"`
	Path string `json:"path"`
	URI  string `json:"uri"`
}

type CodebaseViolation struct {
	ID          string            `json:"id"`
	Folders     []ViolationFolder `json:"folders,omitempty"`
	Files       []ViolationFile   `json:"files,omitempty"`
	Kind        ViolationKind     `json:"kind"`
	Priority    graph.ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
}

type CodebaseBundle struct {
	ID           string             `json:"id"`
	Folder       string             `json:"folder"`
	URI          string             `json:"uri"`
	Contributors []string           `json:"contributors,omitempty"`
	Tickets      []string           `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInfo `json:"metrics,omitempty"`
}

type CodebaseFolder struct {
	ID      string             `json:"id"`
	Path    string             `json:"path"`
	URI     string             `json:"uri"`
	Metrics *FolderMetricsInfo `json:"metrics,omitempty"`
}

type FileViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    graph.ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebaseFile struct {
	ID         string             `json:"id"`
	Path       string             `json:"path"`
	URI        string             `json:"uri"`
	Metrics    *FileMetricsInfo   `json:"metrics,omitempty"`
	Violations []FileViolationRef `json:"violations,omitempty"`
}

type CodebaseSection struct {
	ID      string              `json:"id"`
	Path    string              `json:"path"`
	URI     string              `json:"uri"`
	Metrics *SectionMetricsInfo `json:"metrics,omitempty"`
}

type CodebaseDefinition struct {
	ID      string                 `json:"id"`
	Path    string                 `json:"path"`
	URI     string                 `json:"uri"`
	Metrics *DefinitionMetricsInfo `json:"metrics,omitempty"`
}

type ContributorIcons struct {
	Avatar         string `json:"avatar,omitempty"`
	AvatarRound    string `json:"avatar-round-90x90,omitempty"`
	Github         string `json:"github,omitempty"`
}

type ContributorBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type ContributorFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type ContributorFileContrib struct {
	ID      string       `json:"id"`
	Metrics *graph.LineMetrics `json:"metrics,omitempty"`
}

type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *graph.LineMetrics `json:"metrics,omitempty"`
}

type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *graph.LineMetrics `json:"metrics,omitempty"`
}

type ContributorContributionsInfo struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

type ContributorMetricsInfo struct {
	Commits     int `json:"commits"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Lines       int `json:"lines"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
}

type CodebaseContributor struct {
	ID            string                        `json:"id"`
	URI           string                        `json:"uri"`
	Path          string                        `json:"path"`
	Name          string                        `json:"name,omitempty"`
	Icons         *ContributorIcons             `json:"icons,omitempty"`
	Emails        []string                      `json:"emails,omitempty"`
	Links         map[string]string             `json:"links,omitempty"`
	Contributions *ContributorContributionsInfo `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInfo       `json:"metrics,omitempty"`
}

type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

type TicketBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFileContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketSectionContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *graph.LineMetrics `json:"metrics,omitempty"`
}

type CodebaseTicket struct {
	ID          string                    `json:"id"`
	Path        string                    `json:"path"`
	URI         string                    `json:"uri"`
	Date        *TicketDateInfo           `json:"date,omitempty"`
	Commit      string                    `json:"commit,omitempty"`
	Year        string                    `json:"year"`
	Month       string                    `json:"month"`
	Day         string                    `json:"day"`
	Slug        string                    `json:"slug"`
	Prompt      string                    `json:"prompt,omitempty"`
	Model       string                    `json:"model,omitempty"`
	Author      string                    `json:"author,omitempty"`
	Status      graph.TicketStatus        `json:"status"`
	Bundles     []TicketBundleContrib     `json:"bundles,omitempty"`
	Folders     []TicketFolderContrib     `json:"folders,omitempty"`
	Files       []TicketFileContrib       `json:"files,omitempty"`
	Sections    []TicketSectionContrib    `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib `json:"definitions,omitempty"`
}

type PolicyViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    graph.ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebasePolicy struct {
	ID         string               `json:"id"`
	Name       string               `json:"name"`
	Scopes     []string             `json:"scopes,omitempty"`
	Violations []PolicyViolationRef `json:"violations,omitempty"`
}

type TreeNodeKind string

const (
	TreeNodeRepo       TreeNodeKind = "repo"
	TreeNodeBundle     TreeNodeKind = "bundle"
	TreeNodeFolder     TreeNodeKind = "folder"
	TreeNodeFile       TreeNodeKind = "file"
	TreeNodeSection    TreeNodeKind = "section"
	TreeNodeDefinition TreeNodeKind = "definition"
)

type TreeNode struct {
	Kind     TreeNodeKind         `json:"kind"`
	Children map[string]*TreeNode `json:"children,omitempty"`
}

type Codebase struct {
	Bundles      []CodebaseBundle      `json:"bundles"`
	Folders      []CodebaseFolder      `json:"folders"`
	Files        []CodebaseFile        `json:"files"`
	Sections     []CodebaseSection     `json:"sections"`
	Definitions  []CodebaseDefinition  `json:"definitions"`
	Contributors []CodebaseContributor `json:"contributors"`
	Tickets      []CodebaseTicket      `json:"tickets"`
	Policies     []CodebasePolicy      `json:"policies"`
	Violations   []CodebaseViolation   `json:"violations"`
	Tree         map[string]*TreeNode  `json:"tree"`
}

// #endregion Codebase Types

// #endregion Types

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

func GetRootDir() string {
	return rootDir
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
	case "code":
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

func ParseCodeSections(content string, languageName string) []SectionInfo {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

func ParseMarkdownSectionsInternal(content string) []SectionInfo {
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

type JsonSectionLocation struct {
	Path       string
	KeyStart   int
	KeyEnd     int
	ValueStart int
	ValueEnd   int
	Section    *SectionInfo
}

type jsonContext struct {
	kind      byte
	section   *SectionInfo
	path      string
	expectKey bool
	location  *JsonSectionLocation
}

func ParseJSONSectionsDetailed(content string) ([]SectionInfo, map[string]*JsonSectionLocation, error) {
	var sections []SectionInfo
	locations := make(map[string]*JsonSectionLocation)
	var stack []jsonContext
	line := 1
	inString := false
	escape := false
	stringStart := 0
	stringBuf := strings.Builder{}
	pendingKey := ""
	pendingKeyStart := 0
	pendingKeyEnd := 0
	pendingKeyLine := 0
	var awaitingValue *JsonSectionLocation
	for i := 0; i < len(content); i++ {
		ch := content[i]
		if ch == '\n' {
			line++
		}
		if inString {
			if escape {
				escape = false
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '\\' {
				escape = true
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '"' {
				inString = false
				value := stringBuf.String()
				stringBuf.Reset()
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' && stack[len(stack)-1].expectKey && awaitingValue == nil {
					pendingKey = value
					pendingKeyStart = stringStart
					pendingKeyEnd = i
					pendingKeyLine = line
					stack[len(stack)-1].expectKey = false
				} else if awaitingValue != nil && awaitingValue.ValueStart == stringStart {
					awaitingValue.ValueEnd = i
					awaitingValue.Section.EndLine = line
					awaitingValue.Section.EndIndex = i + 1
					awaitingValue = nil
				}
				continue
			}
			stringBuf.WriteByte(ch)
			continue
		}
		if ch == '"' {
			if awaitingValue != nil {
				awaitingValue.ValueStart = i
			}
			inString = true
			stringStart = i
			continue
		}
		if len(stack) > 0 && stack[len(stack)-1].kind == '{' && pendingKey != "" && ch == ':' {
			parent := stack[len(stack)-1].section
			path := pendingKey
			if stack[len(stack)-1].path != "" {
				path = stack[len(stack)-1].path + "/" + pendingKey
			}
			section := SectionInfo{
				Name:       pendingKey,
				StartLine:  pendingKeyLine,
				EndLine:    -1,
				StartIndex: pendingKeyStart,
				EndIndex:   -1,
				Children:   []SectionInfo{},
			}
			var sectionRef *SectionInfo
			if parent != nil {
				parent.Children = append(parent.Children, section)
				sectionRef = &parent.Children[len(parent.Children)-1]
			} else {
				sections = append(sections, section)
				sectionRef = &sections[len(sections)-1]
			}
			location := &JsonSectionLocation{
				Path:     path,
				KeyStart: pendingKeyStart,
				KeyEnd:   pendingKeyEnd,
				Section:  sectionRef,
			}
			locations[path] = location
			awaitingValue = location
			pendingKey = ""
			continue
		}
		if awaitingValue != nil {
			if ch == '{' || ch == '[' {
				awaitingValue.ValueStart = i
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   awaitingValue.Section,
					path:      awaitingValue.Path,
					expectKey: ch == '{',
					location:  awaitingValue,
				})
				awaitingValue = nil
				continue
			}
			if ch == '-' || (ch >= '0' && ch <= '9') || ch == 't' || ch == 'f' || ch == 'n' {
				awaitingValue.ValueStart = i
				end := i
				for end < len(content) {
					c := content[end]
					if c == '\n' {
						line++
					}
					if c == ',' || c == '}' || c == ']' || c == ' ' || c == '\t' || c == '\r' || c == '\n' {
						break
					}
					end++
				}
				awaitingValue.ValueEnd = end - 1
				awaitingValue.Section.EndLine = line
				awaitingValue.Section.EndIndex = end
				awaitingValue = nil
				i = end - 1
				continue
			}
		}
		if ch == '{' || ch == '[' {
			if awaitingValue == nil {
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   nil,
					path:      "",
					expectKey: ch == '{',
					location:  nil,
				})
			}
			continue
		}
		if ch == '}' || ch == ']' {
			if len(stack) > 0 {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if top.location != nil {
					top.location.ValueEnd = i
					top.location.Section.EndLine = line
					top.location.Section.EndIndex = i + 1
				}
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
					stack[len(stack)-1].expectKey = true
				}
			}
			continue
		}
		if ch == ',' {
			if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
				stack[len(stack)-1].expectKey = true
			}
			continue
		}
	}
	for _, location := range locations {
		if location.Section.EndIndex == -1 {
			location.Section.EndLine = line
			location.Section.EndIndex = len(content)
		}
	}
	return sections, locations, nil
}

func ParseJSONSections(content string) []SectionInfo {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

func ParseSections(content string, filePath string) []SectionInfo {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	return language.ParseSections(content)
}

func NormalizeSectionPath(sectionPath string) []string {
	cleaned := strings.ReplaceAll(sectionPath, "#", "/")
	raw := strings.FieldsFunc(cleaned, func(r rune) bool { return r == '/' })
	var parts []string
	for _, part := range raw {
		if part != "" {
			parts = append(parts, part)
		}
	}
	return parts
}

func jsonLineStart(content string, index int) int {
	if index <= 0 {
		return 0
	}
	pos := strings.LastIndex(content[:index], "\n")
	if pos == -1 {
		return 0
	}
	return pos + 1
}

func jsonLineIndent(content string, index int) string {
	start := jsonLineStart(content, index)
	end := start
	for end < len(content) && (content[end] == ' ' || content[end] == '\t') {
		end++
	}
	return content[start:end]
}

func jsonIsWhitespace(ch byte) bool {
	return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

func jsonFindMatching(content string, start int, open byte, close byte) (int, bool) {
	if start < 0 || start >= len(content) || content[start] != open {
		return -1, false
	}
	inString := false
	escape := false
	depth := 0
	for i := start; i < len(content); i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			inString = true
			continue
		}
		if ch == open {
			depth++
			continue
		}
		if ch == close {
			depth--
			if depth == 0 {
				return i, true
			}
		}
	}
	return -1, false
}

func jsonFindRootObjectRange(content string) (int, int, bool) {
	for i := 0; i < len(content); i++ {
		if jsonIsWhitespace(content[i]) {
			continue
		}
		if content[i] != '{' {
			return -1, -1, false
		}
		end, ok := jsonFindMatching(content, i, '{', '}')
		return i, end, ok
	}
	return -1, -1, false
}

func jsonFindObjectRange(content string, locations map[string]*JsonSectionLocation, path string) (int, int, bool) {
	if path == "" {
		return jsonFindRootObjectRange(content)
	}
	location, ok := locations[path]
	if !ok || location.ValueStart < 0 || location.ValueStart >= len(content) {
		return -1, -1, false
	}
	if content[location.ValueStart] != '{' {
		return -1, -1, false
	}
	return location.ValueStart, location.ValueEnd, true
}

func jsonObjectHasEntries(content string, start, end int) bool {
	for i := start + 1; i < end; i++ {
		if !jsonIsWhitespace(content[i]) {
			return true
		}
	}
	return false
}

func jsonFindFirstKeyIndent(content string, start, end int) string {
	depth := 0
	inString := false
	escape := false
	expectKey := true
	for i := start + 1; i < end; i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			if depth == 0 && expectKey {
				return jsonLineIndent(content, i)
			}
			inString = true
			continue
		}
		if ch == '{' || ch == '[' {
			depth++
			expectKey = ch == '{'
			continue
		}
		if ch == '}' || ch == ']' {
			if depth > 0 {
				depth--
			}
			if depth == 0 {
				expectKey = true
			}
			continue
		}
		if ch == ',' && depth == 0 {
			expectKey = true
		}
	}
	return ""
}

func jsonInsertEntry(content string, objectStart, objectEnd int, entry string) (string, bool) {
	if objectStart < 0 || objectEnd <= objectStart {
		return content, false
	}
	hasEntries := jsonObjectHasEntries(content, objectStart, objectEnd)
	parentIndent := jsonLineIndent(content, objectEnd)
	childIndent := jsonFindFirstKeyIndent(content, objectStart, objectEnd)
	if childIndent == "" {
		childIndent = parentIndent + "  "
	}
	insert := ""
	if hasEntries {
		insert = ",\n" + childIndent + entry + "\n" + parentIndent
	} else {
		insert = "\n" + childIndent + entry + "\n" + parentIndent
	}
	return content[:objectEnd] + insert + content[objectEnd:], true
}

func jsonReplaceKey(content string, keyStart, keyEnd int, newName string) string {
	quoted := strconv.Quote(newName)
	return content[:keyStart] + quoted + content[keyEnd+1:]
}

func jsonExtractEntry(content string, keyStart int, valueEnd int) (string, int, int) {
	start := jsonLineStart(content, keyStart)
	end := valueEnd + 1
	for end < len(content) && jsonIsWhitespace(content[end]) {
		end++
	}
	if end < len(content) && content[end] == ',' {
		end++
	} else {
		left := start
		for left > 0 && jsonIsWhitespace(content[left-1]) {
			left--
		}
		if left > 0 && content[left-1] == ',' {
			start = left - 1
		}
	}
	entry := strings.TrimSpace(content[start:end])
	entry = strings.TrimSuffix(entry, ",")
	return entry, start, end
}

func jsonRenameEntryKey(entry string, newName string) string {
	inString := false
	escape := false
	stringStart := -1
	for i := 0; i < len(entry); i++ {
		ch := entry[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' && stringStart >= 0 {
				quoted := strconv.Quote(newName)
				return entry[:stringStart] + quoted + entry[i+1:]
			}
			continue
		}
		if ch == '"' {
			inString = true
			stringStart = i
		}
	}
	return entry
}

func jsonReindentEntry(entry string, indent string) string {
	lines := strings.Split(entry, "\n")
	minIndent := -1
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		leading := 0
		for leading < len(line) && (line[leading] == ' ' || line[leading] == '\t') {
			leading++
		}
		if minIndent == -1 || leading < minIndent {
			minIndent = leading
		}
	}
	if minIndent < 0 {
		return indent + strings.TrimSpace(entry)
	}
	for i, line := range lines {
		if strings.TrimSpace(line) == "" {
			lines[i] = ""
			continue
		}
		if minIndent > 0 && len(line) >= minIndent {
			line = line[minIndent:]
		}
		lines[i] = indent + line
	}
	return strings.Join(lines, "\n")
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

var policies = []Policy{
	{
		ID:          "code",
		Name:        "Code",
		Description: "Validates source file headers, sections, and comments",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go}"},
		Priority:    graph.ViolationPriorityLow,
		Kinds: []ViolationKind{
			ViolationCodeHeaderMissingRegion,
			ViolationCodeHeaderMissingFilename,
			ViolationCodeHeaderMissingContributors,
			ViolationCodeHeaderMissingLicense,
			ViolationCodeHeaderWrongLicense,
			ViolationCodeSectionEmpty,
			ViolationCodeSectionOrphanDefinition,
			ViolationCodeSectionMissingStartName,
			ViolationCodeSectionMissingEndName,
			ViolationCodeSectionNameMismatch,
			ViolationCodeCommentInline,
			ViolationCodeCommentBlock,
			ViolationCodeCommentJSDoc,
		},
		Run: codePolicy,
	},
	{
		ID:          "dev-docs",
		Name:        "DevDocs",
		Description: "Validates README.md and AGENTS.md documentation structure",
		Scopes:      []string{"README.md", "AGENTS.md"},
		Priority:    graph.ViolationPriorityLow,
		Kinds: []ViolationKind{
			ViolationDevDocsMissingFile,
			ViolationDevDocsMissingFolder,
			ViolationDevDocsWrongFilePath,
			ViolationDevDocsWrongFolderPath,
			ViolationDevDocsWrongFileName,
			ViolationDevDocsWrongFolderName,
			ViolationDevDocsWrongFileOrder,
			ViolationDevDocsWrongFolderOrder,
			ViolationDevDocsMissingComponent,
			ViolationDevDocsWrongComponentName,
			ViolationDevDocsWrongComponentOrder,
		},
		Run: devDocsPolicy,
	},
	{
		ID:          "sketchpad",
		Name:        "Sketchpad",
		Description: "Validates sketchpad imports, state management, and hook patterns",
		Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
		Priority:    graph.ViolationPriorityHigh,
		Kinds: []ViolationKind{
			ViolationSketchpadImportThirdParty,
			ViolationSketchpadStateMultipleMachines,
			ViolationSketchpadStateCreateActor,
			ViolationSketchpadStateYjsAppState,
			ViolationSketchpadStateForbiddenStore,
			ViolationSketchpadHooksNonTriadic,
		},
		Run: sketchpadPolicy,
	},
}

func FindPolicy(id string) (Policy, bool) {
	for _, p := range policies {
		if p.ID == id {
			return p, true
		}
	}
	return Policy{}, false
}

func GetPolicies() []Policy {
	return policies
}

type PolicyContext struct {
	Scope    Scope
	RootDir  string
	Bundles []Bundle
	fileCache     map[string]string
	sectionCache  map[string][]SectionInfo
	ignoreCache   map[string]map[int][]string // file -> line -> ignore patterns
}

func NewPolicyContext(scope Scope, bundles []Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Bundles:     bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]SectionInfo),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

func (ctx *PolicyContext) Files() ([]string, error) {
	return ScopeToFiles(ctx.Scope, ctx.Bundles)
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

// ParseIgnoreDirectives parses // semio-ignore-* comments from file content
// Returns a map of line number -> list of ignore patterns
func ParseIgnoreDirectives(content string) map[int][]string {
	result := make(map[int][]string)
	lines := strings.Split(content, "\n")
	ignorePrefix := "// semio-ignore-"
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, ignorePrefix) {
			pattern := strings.TrimPrefix(trimmed, ignorePrefix)
			// Handle multiple patterns on same line separated by comma
			patterns := strings.Split(pattern, ",")
			for _, p := range patterns {
				p = strings.TrimSpace(p)
				if p != "" {
					result[i+1] = append(result[i+1], p)
				}
			}
		}
	}
	return result
}

// IgnoreDirectives returns parsed ignore directives for a file (cached)
func (ctx *PolicyContext) IgnoreDirectives(filePath string) map[int][]string {
	if ignores, ok := ctx.ignoreCache[filePath]; ok {
		return ignores
	}
	content := ctx.ReadText(filePath)
	ignores := ParseIgnoreDirectives(content)
	ctx.ignoreCache[filePath] = ignores
	return ignores
}

// IsIgnored checks if a violation should be ignored based on semio-ignore directives
// The ignore directive on line N affects violations on lines N+1 through the end of
// the next definition (approximated as next 100 lines or until next ignore directive)
func (ctx *PolicyContext) IsIgnored(filePath string, violationLine int, kind ViolationKind) bool {
	ignores := ctx.IgnoreDirectives(filePath)
	kindStr := string(kind)
	// Check if any ignore directive applies to this line
	for ignoreLine, patterns := range ignores {
		// Ignore directive applies to lines after it (up to ~100 lines or next directive)
		if violationLine > ignoreLine && violationLine <= ignoreLine+100 {
			for _, pattern := range patterns {
				// Pattern matches if the kind starts with the pattern
				if strings.HasPrefix(kindStr, pattern) {
					return true
				}
			}
		}
	}
	return false
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

// extractFileFromScope extracts the file path from a scope string
// Scope formats: "file.ts", "file.ts#Section", "file.ts::definition"
func extractFileFromScope(scope string) string {
	// Remove section suffix (after #)
	if idx := strings.Index(scope, "#"); idx != -1 {
		scope = scope[:idx]
	}
	// Remove definition suffix (after ::)
	if idx := strings.Index(scope, "::"); idx != -1 {
		scope = scope[:idx]
	}
	return scope
}

// FilterIgnored removes violations that are ignored via semio-ignore directives
func (ctx *PolicyContext) FilterIgnored(violations []Violation) []Violation {
	var result []Violation
	for _, v := range violations {
		file := extractFileFromScope(v.Scope)
		if !ctx.IsIgnored(file, v.Line, v.Kind) {
			result = append(result, v)
		}
	}
	return result
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

func CheckPolicies(scope Scope, bundles []Bundle, policyIDs []string) ([]Violation, error) {
	ctx := NewPolicyContext(scope, bundles)
	var violations []Violation
	var policiesToRun []Policy
	if len(policyIDs) > 0 {
		for _, p := range policies {
			for _, id := range policyIDs {
				if p.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range policies {
			if matchesScope(p.Scopes, scope) {
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
		language := GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
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
			headerContent := generateFileHeader(file, language)
			if headerContent != "" {
				autofix := &Fix{
					Description: "Add header section",
					Edits: map[string][]graph.TextEdit{
						file: {{Start: 0, End: 0, NewText: headerContent + "\n"}},
					},
				}
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
file, 0, "", autofix))
			} else {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
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
				ViolationCodeHeaderMissingFilename,
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
				ViolationCodeHeaderMissingContributors,
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
				ViolationCodeHeaderMissingLicense,
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
					ViolationCodeHeaderWrongLicense,
fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
			}
		}
	}
	return ctx.FilterIgnored(violations)
}

func sectionPolicy(ctx *PolicyContext) []Violation {
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
		language := GetLanguage(file)
		if language == nil || !language.SupportsSections() {
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
			if matched, name := language.PolicySectionStartMatch(line); matched {
				if name == "" {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						ViolationCodeSectionMissingStartName,
file, lineNum, strings.TrimSpace(line), nil))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if matched, endName := language.PolicySectionEndMatch(line); matched {
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								ViolationCodeSectionMissingEndName,
					file, lineNum, strings.TrimSpace(line), nil))
						} else if endName != open.name {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								ViolationCodeSectionNameMismatch,
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
					ViolationCodeSectionEmpty,
fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, "", nil))
			}
			for _, child := range s.Children {
				checkSection(child)
			}
		}
		for _, s := range sections {
			checkSection(s)
		}
		covered := make([]bool, len(lines))
		var markCovered func(s SectionInfo)
		markCovered = func(s SectionInfo) {
			start := s.StartLine
			if start < 1 {
				start = 1
			}
			end := s.EndLine
			if end < start {
				end = start
			}
			if end > len(lines) {
				end = len(lines)
			}
			for lineIndex := start; lineIndex <= end; lineIndex++ {
				covered[lineIndex-1] = true
			}
			for _, child := range s.Children {
				markCovered(child)
			}
		}
		for _, s := range sections {
			markCovered(s)
		}
		type lineRange struct {
			start int
			end   int
		}
		type defRange struct {
			name  string
			start int
			end   int
		}
		type orphanRangeInfo struct {
			start          int
			end            int
			firstLine      string
			isCommentBlock bool
		}
		orphanLines := make([]bool, len(lines))
		for i, line := range lines {
			if covered[i] {
				continue
			}
			line = strings.TrimSuffix(line, "\r")
			if strings.TrimSpace(line) == "" {
				continue
			}
			if startMatched, _ := language.PolicySectionStartMatch(line); startMatched {
				continue
			}
			if endMatched, _ := language.PolicySectionEndMatch(line); endMatched {
				continue
			}
			orphanLines[i] = true
		}
		var orphanRanges []lineRange
		inOrphan := false
		startLine := 0
		for i := 0; i < len(orphanLines); i++ {
			if orphanLines[i] {
				if !inOrphan {
					inOrphan = true
					startLine = i + 1
				}
			} else if inOrphan {
				orphanRanges = append(orphanRanges, lineRange{start: startLine, end: i})
				inOrphan = false
			}
		}
		if inOrphan {
			orphanRanges = append(orphanRanges, lineRange{start: startLine, end: len(lines)})
		}
		commentPrefix := language.CommentPrefix()
		var defRanges []defRange
		defExcerpts := make(map[string]string)
		if language.SupportsDefinitions() {
			parsedDefs := language.ParseDefinitions(content, lines)
			for _, def := range parsedDefs {
				defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
				defExcerpts[def.Name] = def.Excerpt
			}
		}
		extraDefs := language.ExtraOrphanDefinitions(lines)
		for _, def := range extraDefs {
			defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
			defExcerpts[def.Name] = def.Excerpt
		}
		var orphanInfos []orphanRangeInfo
		for _, orphanRange := range orphanRanges {
			firstLine := ""
			isCommentBlock := true
			for lineIndex := orphanRange.start; lineIndex <= orphanRange.end; lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex-1], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				if firstLine == "" {
					firstLine = strings.TrimSpace(line)
				}
				if !strings.HasPrefix(strings.TrimSpace(line), commentPrefix) {
					isCommentBlock = false
				}
			}
			orphanInfos = append(orphanInfos, orphanRangeInfo{
				start:          orphanRange.start,
				end:            orphanRange.end,
				firstLine:      firstLine,
				isCommentBlock: isCommentBlock,
			})
			if isCommentBlock {
				name := fmt.Sprintf("comment-block-%d", orphanRange.start)
				defRanges = append(defRanges, defRange{name: name, start: orphanRange.start, end: orphanRange.end})
				defExcerpts[name] = firstLine
			}
		}
		reportedDefs := make(map[string]bool)
		for _, orphanRange := range orphanInfos {
			matched := false
			for _, defRange := range defRanges {
				if orphanRange.start <= defRange.end && orphanRange.end >= defRange.start {
					if !reportedDefs[defRange.name] {
						reportedDefs[defRange.name] = true
						excerpt := defRange.name
						if value, ok := defExcerpts[defRange.name]; ok && value != "" {
							excerpt = value
						}
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Orphan definition outside sections at %s:%d", file, defRange.start),
							ViolationCodeSectionOrphanDefinition,
							fmt.Sprintf("%s::%s", file, defRange.name),
							defRange.start,
							excerpt,
							nil))
					}
					matched = true
				}
			}
			if matched {
				continue
			}
			name := fmt.Sprintf("orphan-block-%d", orphanRange.start)
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Orphan definition outside sections at %s:%d", file, orphanRange.start),
				ViolationCodeSectionOrphanDefinition,
				fmt.Sprintf("%s::%s", file, name),
				orphanRange.start,
				orphanRange.firstLine,
				nil))
		}
	}
	return ctx.FilterIgnored(violations)
}

type CommentTemplateState struct {
	ExprDepth int
}

type CommentScanState struct {
	InBlockComment        bool
	BlockCommentStartLine int
	BlockCommentStartIndex int
	BlockCommentIsJsDoc   bool
	InSingleQuote         bool
	InDoubleQuote         bool
	Templates             []CommentTemplateState
	Escaped               bool
}

func (state *CommentScanState) InTemplateRaw() bool {
	if len(state.Templates) == 0 {
		return false
	}
	return state.Templates[len(state.Templates)-1].ExprDepth == 0
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
		language := GetLanguage(file)
		if language == nil || !language.SupportsComments() {
			continue
		}
		lines := strings.Split(content, "\n")
		langViolations := language.ScanComments(ctx, file, content, lines)
		violations = append(violations, langViolations...)
	}
	return ctx.FilterIgnored(violations)
}

func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

func codePolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	violations = append(violations, headerPolicy(ctx)...)
	violations = append(violations, sectionPolicy(ctx)...)
	violations = append(violations, commentPolicy(ctx)...)
	return violations
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
	return ctx.FilterIgnored(violations)
}

func sketchpadPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	elementsFile := ""
	for _, file := range files {
		if strings.HasSuffix(file, "elements.tsx") {
			elementsFile = file
			break
		}
	}
	thirdPartyPackages := []string{
		"react", "xstate", "yjs", "@radix-ui", "@dnd-kit", "zustand", "immer",
		"framer-motion", "lucide-react", "clsx", "tailwind", "three", "@react-three",
	}
	createMachineCount := 0
	for _, file := range files {
		if !strings.HasSuffix(file, ".ts") && !strings.HasSuffix(file, ".tsx") {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lines := strings.Split(content, "\n")
		isElementsFile := file == elementsFile
		sections := ctx.Sections(file)
		isStateManagementSection := func(lineNum int) bool {
			for _, section := range sections {
				if strings.Contains(strings.ToLower(section.Name), "state management") ||
					strings.Contains(strings.ToLower(section.Name), "state-management") {
					if lineNum >= section.StartLine && lineNum <= section.EndLine {
						return true
					}
				}
			}
			return false
		}
		for lineNum, line := range lines {
			lineNumber := lineNum + 1
			if !isElementsFile && strings.Contains(line, "import ") {
				for _, pkg := range thirdPartyPackages {
					importPattern := fmt.Sprintf(`from\s+['"]%s`, regexp.QuoteMeta(pkg))
					if matched, _ := regexp.MatchString(importPattern, line); matched {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Third party import '%s' must only be in elements.tsx", pkg),
							ViolationSketchpadImportThirdParty,
							file, lineNumber, strings.TrimSpace(line), nil))
						break
					}
				}
			}
			if strings.Contains(line, "createMachine(") || strings.Contains(line, "createMachine<") {
				createMachineCount++
				if createMachineCount > 1 {
					violations = append(violations, ctx.CreateViolation(
						"createMachine can only be used once in sketchpad",
						ViolationSketchpadStateMultipleMachines,
						file, lineNumber, strings.TrimSpace(line), nil))
				}
			}
			if strings.Contains(line, "createActor(") || strings.Contains(line, "createActor<") {
				violations = append(violations, ctx.CreateViolation(
					"createActor is forbidden in sketchpad",
					ViolationSketchpadStateCreateActor,
					file, lineNumber, strings.TrimSpace(line), nil))
			}
			yjsAppStatePatterns := []string{"Y.Doc(", "new Doc(", "Y.Map(", "Y.Array(", "Y.Text("}
			for _, pattern := range yjsAppStatePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if !strings.Contains(strings.ToLower(file), "kit") &&
						!strings.Contains(strings.ToLower(file), "sync") {
						violations = append(violations, ctx.CreateViolation(
							"Yjs should only be used for kit data synchronization, not app state",
							ViolationSketchpadStateYjsAppState,
							file, lineNumber, strings.TrimSpace(line), nil))
					}
				}
			}
			storePatterns := []string{"create(", "createStore(", "useStore("}
			for _, pattern := range storePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if strings.Contains(line, "zustand") || strings.Contains(line, "store") {
						violations = append(violations, ctx.CreateViolation(
							"Stores outside of State Management sections are forbidden",
							ViolationSketchpadStateForbiddenStore,
							file, lineNumber, strings.TrimSpace(line), nil))
					}
				}
			}
		}
	}
	return ctx.FilterIgnored(violations)
}

// #endregion Policies

// #region Codebase

type CodebaseContext struct {
	RootDir    string
	RootURI    string
	Bundles    []Bundle
	Files      []string
	Violations []Violation
	Tickets    []Ticket
	Policies   []Policy
}

func NewCodebaseContext() *CodebaseContext {
	rootURI := "file://" + NormalizePath(rootDir)
	return &CodebaseContext{
		RootDir: rootDir,
		RootURI: rootURI,
	}
}

func (ctx *CodebaseContext) LoadBundles() {
	ctx.Bundles = GetProjects()
}

func (ctx *CodebaseContext) LoadFiles() error {
	files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, ctx.Bundles)
	if err != nil {
		return err
	}
	ctx.Files = files
	return nil
}

func (ctx *CodebaseContext) LoadViolations() error {
	for _, file := range ctx.Files {
		violations, err := AnalyzeFile(file, ctx.Bundles)
		if err != nil {
			continue
		}
		ctx.Violations = append(ctx.Violations, violations...)
	}
	return nil
}

func (ctx *CodebaseContext) LoadTickets() error {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return err
	}
	ctx.Tickets = tickets
	return nil
}

func (ctx *CodebaseContext) LoadPolicies() {
	ctx.Policies = GetPolicies()
}

func (ctx *CodebaseContext) GetBundleForFile(filePath string) string {
	normalizedPath := NormalizePath(filePath)
	var matchedBundle string
	var matchedLen int
	for _, bundle := range ctx.Bundles {
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedLen = len(root)
			}
		}
	}
	return matchedBundle
}

func (ctx *CodebaseContext) FileURI(path string) string {
	return ctx.RootURI + "/" + NormalizePath(path)
}

func (ctx *CodebaseContext) FolderURI(path string) string {
	return "folder://" + NormalizePath(filepath.Join(rootDir, path))
}

func BuildCodebaseBundles(ctx *CodebaseContext) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})
	contributorSets := make(map[string]map[string]struct{})
	ticketSets := make(map[string]map[string]struct{})
	violationCounts := make(map[string]int)

	for _, bundle := range ctx.Bundles {
		folderSets[bundle.Name] = make(map[string]struct{})
		contributorSets[bundle.Name] = make(map[string]struct{})
		ticketSets[bundle.Name] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[bundleName] += strings.Count(content, "\n") + 1
			sections := ParseSections(content, file)
			sectionCounts[bundleName] += countSections(sections)
			lang := GetLanguage(file)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				definitionCounts[bundleName] += len(defs)
			}
			headerSection := FindSection(sections, "Header")
			if headerSection != nil {
				headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
				for _, line := range strings.Split(headerContent, "\n") {
					if name, email, ok := ParseContributorIdentity(line); ok {
						_ = name
						contributorSets[bundleName][email] = struct{}{}
					}
				}
			}
		}
	}

	for _, v := range ctx.Violations {
		bundleName := ctx.GetBundleForFile(v.Scope)
		if bundleName != "" {
			violationCounts[bundleName]++
		}
	}

	for _, ticket := range ctx.Tickets {
		for _, iteration := range ticket.Frontmatter.Iterations {
			for bundleName := range iteration.Bundles {
				if _, ok := ticketSets[bundleName]; ok {
					ticketID := fmt.Sprintf("%04d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
					ticketSets[bundleName][ticketID] = struct{}{}
				}
			}
		}
	}

	for _, bundle := range ctx.Bundles {
		var contributors []string
		for c := range contributorSets[bundle.Name] {
			contributors = append(contributors, c)
		}
		sort.Strings(contributors)

		var tickets []string
		for t := range ticketSets[bundle.Name] {
			tickets = append(tickets, t)
		}
		sort.Strings(tickets)

		result = append(result, CodebaseBundle{
			ID:           bundle.Name,
			Folder:       bundle.Root,
			URI:          ctx.FileURI(bundle.Root),
			Contributors: contributors,
			Tickets:      tickets,
			Metrics: &BundleMetricsInfo{
				Folders:     len(folderSets[bundle.Name]),
				Files:       fileCounts[bundle.Name],
				Sections:    sectionCounts[bundle.Name],
				Definitions: definitionCounts[bundle.Name],
				Lines:       lineCounts[bundle.Name],
				Violations:  violationCounts[bundle.Name],
			},
		})
	}
	return result
}

func countSections(sections []SectionInfo) int {
	count := len(sections)
	for _, s := range sections {
		count += countSections(s.Children)
	}
	return count
}

func BuildCodebaseFolders(ctx *CodebaseContext) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	violationCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[folder] += strings.Count(content, "\n") + 1
		}
	}

	for _, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				violationCounts[folder]++
			}
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		bundleName := ctx.GetBundleForFile(folder)
		id := folder
		if bundleName != "" {
			id = bundleName + "/" + folder
		}
		result = append(result, CodebaseFolder{
			ID:   id,
			Path: folder,
			URI:  ctx.FileURI(folder),
			Metrics: &FolderMetricsInfo{
				Files:      fileCounts[folder],
				Lines:      lineCounts[folder],
				Violations: violationCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func extractFilePath(scope string) string {
	scope = strings.Split(scope, "#")[0]
	scope = strings.Split(scope, "§")[0]
	return scope
}

func BuildCodebaseFiles(ctx *CodebaseContext) []CodebaseFile {
	var result []CodebaseFile
	violationsByFile := make(map[string][]Violation)

	for _, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			violationsByFile[filePath] = append(violationsByFile[filePath], v)
		}
	}

	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		id := file
		if bundleName != "" {
			id = bundleName + "/" + filepath.Base(file)
		}

		var metrics *FileMetricsInfo
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			sections := ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &FileMetricsInfo{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}

		var violations []FileViolationRef
		for _, v := range violationsByFile[file] {
			info := v.Kind.Info()
			violations = append(violations, FileViolationRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}

		result = append(result, CodebaseFile{
			ID:         id,
			Path:       file,
			URI:        ctx.FileURI(file),
			Metrics:    metrics,
			Violations: violations,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseSections(ctx *CodebaseContext) []CodebaseSection {
	var result []CodebaseSection

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)
		bundleName := ctx.GetBundleForFile(file)
		addSections(ctx, &result, file, bundleName, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func addSections(ctx *CodebaseContext, result *[]CodebaseSection, file, bundleName, content string, sections []SectionInfo, parentPath string) {
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		id := file + "#" + sectionPath
		if bundleName != "" {
			id = bundleName + "/" + filepath.Base(file) + "#" + sectionPath
		}
		sectionContent := ""
		if section.StartIndex < len(content) && section.EndIndex <= len(content) {
			sectionContent = content[section.StartIndex:section.EndIndex]
		}
		defCount := 0
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &SectionMetricsInfo{
				Definitions: defCount,
				Lines:       section.EndLine - section.StartLine + 1,
				Violations:  0,
			},
		})
		addSections(ctx, result, file, bundleName, content, section.Children, sectionPath)
	}
}

func BuildCodebaseDefinitions(ctx *CodebaseContext) []CodebaseDefinition {
	var result []CodebaseDefinition

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := ParseSections(content, file)
		bundleName := ctx.GetBundleForFile(file)

		for _, def := range defs {
			sectionPath := findSectionForDefinition(sections, def.Start, def.End, "")
			defPath := file
			if sectionPath != "" {
				defPath = file + "#" + sectionPath + "§" + def.Name
			} else {
				defPath = file + "§" + def.Name
			}
			id := defPath
			if bundleName != "" {
				id = bundleName + "/" + filepath.Base(file) + "§" + def.Name
			}
			result = append(result, CodebaseDefinition{
				ID:   id,
				Path: defPath,
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &DefinitionMetricsInfo{
					Definitions: 0,
					Lines:       def.End - def.Start + 1,
					Violations:  0,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseContributors(ctx *CodebaseContext) []CodebaseContributor {
	contributors, err := ListContributors()
	if err != nil {
		return nil
	}

	var result []CodebaseContributor
	for _, c := range contributors {
		avatarPath := GetContributorAvatarPath(c.Github)
		avatarRoundPath := GetContributorAvatarRoundPath(c.Github)

		var icons *ContributorIcons
		if FileExists(avatarPath) || FileExists(avatarRoundPath) {
			icons = &ContributorIcons{}
			if FileExists(avatarPath) {
				icons.Avatar = ctx.FileURI(GetRelativePath(avatarPath))
			}
			if FileExists(avatarRoundPath) {
				icons.AvatarRound = ctx.FileURI(GetRelativePath(avatarRoundPath))
			}
			if githubLink, ok := c.Links["github"]; ok {
				icons.Github = githubLink + ".png"
			}
		}

		var contributions *ContributorContributionsInfo
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &ContributorContributionsInfo{}
			for _, b := range c.Contributions.Bundles {
				contributions.Bundles = append(contributions.Bundles, ContributorBundleContrib{ID: b})
			}
			for _, f := range c.Contributions.Folders {
				contributions.Folders = append(contributions.Folders, ContributorFolderContrib{ID: f})
			}
			for _, f := range c.Contributions.Files {
				contributions.Files = append(contributions.Files, ContributorFileContrib{ID: f})
			}
			for _, r := range c.Contributions.Regions {
				contributions.Sections = append(contributions.Sections, ContributorSectionContrib{ID: r})
			}
			for _, d := range c.Contributions.Definitions {
				contributions.Definitions = append(contributions.Definitions, ContributorDefinitionContrib{ID: d})
			}
		}

		linesTotal := 0
		if c.Contributions.Lines != nil {
			linesTotal = c.Contributions.Lines.Added + c.Contributions.Lines.Removed
		}

		result = append(result, CodebaseContributor{
			ID:            c.Github,
			URI:           ctx.FileURI("contributors/" + c.Github),
			Path:          "contributors/" + c.Github + "/contributor.json",
			Name:          c.Name,
			Icons:         icons,
			Emails:        c.Emails,
			Links:         c.Links,
			Contributions: contributions,
			Metrics: &ContributorMetricsInfo{
				Commits:     len(c.Contributions.Commits),
				Tickets:     len(c.Contributions.Tickets),
				Bundles:     len(c.Contributions.Bundles),
				Folders:     len(c.Contributions.Folders),
				Files:       len(c.Contributions.Files),
				Lines:       linesTotal,
				Sections:    len(c.Contributions.Regions),
				Definitions: len(c.Contributions.Definitions),
			},
		})
	}
	return result
}

func BuildCodebaseTickets(ctx *CodebaseContext) []CodebaseTicket {
	var result []CodebaseTicket

	for _, ticket := range ctx.Tickets {
		ticketID := fmt.Sprintf("%04d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		ticketPath := fmt.Sprintf("tickets/%04d/%02d/%02d/%s/ticket.md", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)

		var bundleContribs []TicketBundleContrib
		for _, iteration := range ticket.Frontmatter.Iterations {
			for bundleName, TicketBundleMetrics := range iteration.Bundles {
				lines := AggregateBundleLines(TicketBundles{bundleName: TicketBundleMetrics})
				bundleContribs = append(bundleContribs, TicketBundleContrib{
					ID: bundleName,
					Metrics: &CountMetrics{
						Added: lines.Added,
					},
				})
			}
		}

		model := ticket.Frontmatter.Model
		for _, iteration := range ticket.Frontmatter.Iterations {
			if iteration.Model != "" {
				model = iteration.Model
			}
		}

		result = append(result, CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &TicketDateInfo{
				Created:  ticket.Frontmatter.Date.Created,
				Finished: ticket.Frontmatter.Date.Finished,
			},
			Commit:   ticket.Frontmatter.Commit,
			Year:     fmt.Sprintf("%04d", ticket.Year),
			Month:    fmt.Sprintf("%02d", ticket.Month),
			Day:      fmt.Sprintf("%02d", ticket.Day),
			Slug:     ticket.Slug,
			Prompt:   ticket.Frontmatter.Prompt,
			Model:    model,
			Author:   ticket.Frontmatter.Author,
			Status:   ticket.Frontmatter.Status,
			Bundles:  bundleContribs,
		})
	}
	return result
}

func BuildCodebasePolicies(ctx *CodebaseContext) []CodebasePolicy {
	var result []CodebasePolicy
	violationsByPolicy := make(map[string][]Violation)

	for _, v := range ctx.Violations {
		parts := strings.Split(string(v.Kind), ":")
		if len(parts) > 0 {
			policyID := parts[0]
			violationsByPolicy[policyID] = append(violationsByPolicy[policyID], v)
		}
	}

	for _, policy := range ctx.Policies {
		var violations []PolicyViolationRef
		for _, v := range violationsByPolicy[policy.ID] {
			info := v.Kind.Info()
			violations = append(violations, PolicyViolationRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}
		result = append(result, CodebasePolicy{
			ID:         policy.ID,
			Name:       policy.Name,
			Scopes:     policy.Scopes,
			Violations: violations,
		})
	}
	return result
}

func BuildCodebaseViolations(ctx *CodebaseContext) []CodebaseViolation {
	var result []CodebaseViolation

	for i, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		bundleName := ctx.GetBundleForFile(filePath)
		info := v.Kind.Info()

		violationID := fmt.Sprintf("%s#|%s|%s#%d", v.Kind, bundleName, filePath, i)

		var folders []ViolationFolder
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				folderID := folder
				if bundleName != "" {
					folderID = bundleName + "/" + folder
				}
				folders = append(folders, ViolationFolder{
					ID:   folderID,
					Path: folder,
					URI:  ctx.FolderURI(folder),
				})
			}
		}

		var files []ViolationFile
		if filePath != "" {
			fileID := filePath
			if bundleName != "" {
				fileID = bundleName + "/" + filepath.Base(filePath)
			}
			files = append(files, ViolationFile{
				ID:   fileID,
				Path: filePath,
				URI:  ctx.FileURI(filePath),
				Range: &FileRange{
					Start: RangePosition{Line: v.Line, Column: v.Column},
					End:   RangePosition{Line: v.Line, Column: v.Column},
				},
			})
		}

		result = append(result, CodebaseViolation{
			ID:          violationID,
			Folders:     folders,
			Files:       files,
			Kind:        v.Kind,
			Priority:    info.Priority,
			Autofixable: info.Autofixable,
			Reason:      info.Reason,
			Solution:    info.Solution,
		})
	}
	return result
}

func BuildCodebaseTree(ctx *CodebaseContext, bundles []CodebaseBundle, files []CodebaseFile, sections []CodebaseSection, definitions []CodebaseDefinition) map[string]*TreeNode {
	tree := make(map[string]*TreeNode)
	tree["@semio"] = &TreeNode{Kind: TreeNodeRepo, Children: make(map[string]*TreeNode)}
	root := tree["@semio"]

	for _, bundle := range bundles {
		root.Children[bundle.ID] = &TreeNode{Kind: TreeNodeBundle, Children: make(map[string]*TreeNode)}
	}

	folderNodes := make(map[string]*TreeNode)
	for _, file := range files {
		bundleName := ctx.GetBundleForFile(file.Path)
		var parent *TreeNode
		if bundleName != "" {
			parent = root.Children[bundleName]
		} else {
			parent = root
		}
		folder := NormalizePath(filepath.Dir(file.Path))
		if folder != "." {
			parts := strings.Split(folder, "/")
			for i, part := range parts {
				folderPath := strings.Join(parts[:i+1], "/")
				if _, ok := folderNodes[folderPath]; !ok {
					folderNode := &TreeNode{Kind: TreeNodeFolder, Children: make(map[string]*TreeNode)}
					if i == 0 {
						parent.Children[part] = folderNode
					} else {
						parentPath := strings.Join(parts[:i], "/")
						folderNodes[parentPath].Children[part] = folderNode
					}
					folderNodes[folderPath] = folderNode
				}
			}
			fileNode := &TreeNode{Kind: TreeNodeFile, Children: make(map[string]*TreeNode)}
			folderNodes[folder].Children[file.ID] = fileNode
		} else {
			fileNode := &TreeNode{Kind: TreeNodeFile, Children: make(map[string]*TreeNode)}
			parent.Children[file.ID] = fileNode
		}
	}

	return tree
}

func BuildCodebase(ctx *CodebaseContext) *Codebase {
	bundles := BuildCodebaseBundles(ctx)
	folders := BuildCodebaseFolders(ctx)
	files := BuildCodebaseFiles(ctx)
	sections := BuildCodebaseSections(ctx)
	definitions := BuildCodebaseDefinitions(ctx)
	contributors := BuildCodebaseContributors(ctx)
	tickets := BuildCodebaseTickets(ctx)
	policies := BuildCodebasePolicies(ctx)
	violations := BuildCodebaseViolations(ctx)
	tree := BuildCodebaseTree(ctx, bundles, files, sections, definitions)

	return &Codebase{
		Bundles:      bundles,
		Folders:      folders,
		Files:        files,
		Sections:     sections,
		Definitions:  definitions,
		Contributors: contributors,
		Tickets:      tickets,
		Policies:     policies,
		Violations:   violations,
		Tree:         tree,
	}
}

func ToolCodebase() ToolResult {
	output := NewOutput()
	ctx := NewCodebaseContext()

	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		output.Error(fmt.Sprintf("Error loading files: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ctx.LoadViolations(); err != nil {
		output.Error(fmt.Sprintf("Error loading violations: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ctx.LoadTickets(); err != nil {
		output.Error(fmt.Sprintf("Error loading tickets: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	ctx.LoadPolicies()

	codebase := BuildCodebase(ctx)

	output.Success(fmt.Sprintf("Codebase loaded: %d bundles, %d files, %d violations",
		len(codebase.Bundles), len(codebase.Files), len(codebase.Violations)))

	return ToolResult{Output: *output, Data: codebase}
}

// #endregion Codebase

// #region Tickets

func GetTicketsDir() string {
	return filepath.Join(rootDir, "tickets")
}

func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), strconv.Itoa(year), PadNumber(month, 2), PadNumber(day, 2), slug)
}

func GetTicketFilePath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.md")
}

func CreateTicket(slug, prompt, model string, files []string) (*Ticket, error) {
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required to create a ticket")
	}
	now := time.Now()
	year, month, day := FormatDate(now)
	normalizedSlug := Slugify(slug)
	ticketDir := GetTicketPath(year, month, day, normalizedSlug)
	if err := EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	filePath := GetTicketFilePath(year, month, day, normalizedSlug)
	gitAuthor := GetGitAuthor()
	gitCommit := GetGitCommit()
	var declaredFiles *TicketIterationFiles
	if len(files) > 0 {
		declaredFiles = &TicketIterationFiles{}
		for _, f := range files {
			declaredFiles.Updated = append(declaredFiles.Updated, FileLineMetrics{Path: f})
		}
	}
	firstIteration := TicketIteration{
		Prompt: prompt,
		Model:  model,
		Date:   TicketDate{Started: ISOTimestamp()},
		Author: gitAuthor,
		Declared: declaredFiles,
	}
	frontmatter := TicketFrontmatter{
		Slug:       normalizedSlug,
		Prompt:     prompt,
		Status:     graph.TicketStatusOpen,
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
		FolderPath:  ticketDir,
		FilePath:    filePath,
	}
	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	return ticket, nil
}

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	filePath := GetTicketFilePath(year, month, day, slug)
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
		FolderPath:  GetTicketPath(year, month, day, slug),
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
					if e.IsDir() {
						slug := e.Name()
						yearInt, _ := strconv.Atoi(y)
						monthInt, _ := strconv.Atoi(m)
						dayInt, _ := strconv.Atoi(d)
						ticketFilePath := GetTicketFilePath(yearInt, monthInt, dayInt, slug)
						if FileExists(ticketFilePath) {
							ticket, err := ReadTicket(yearInt, monthInt, dayInt, slug)
							if err == nil {
								tickets = append(tickets, *ticket)
							}
						}
					}
				}
			}
		}
	}
	return tickets, nil
}

func ProgressIteration(ticket *Ticket, prompt, model string) error {
	baseCommit := ticket.Frontmatter.Commit
	for i := len(ticket.Frontmatter.Iterations) - 1; i >= 0; i-- {
		if ticket.Frontmatter.Iterations[i].Commit != "" {
			baseCommit = ticket.Frontmatter.Iterations[i].Commit
			break
		}
	}
	if baseCommit == "" {
		return fmt.Errorf("no base commit found for iteration")
	}
	iterationFiles, _, err := GetGitDiffFileLineMetrics(baseCommit, "", nil)
	if err != nil {
		return err
	}
	if len(iterationFiles.Updated) == 0 && len(iterationFiles.Created) == 0 && len(iterationFiles.Removed) == 0 {
		return fmt.Errorf("no git changes found since last commit")
	}
	bundles := GetProjects()
	now := ISOTimestamp()
	iteration := TicketIteration{
		Prompt:  prompt,
		Model:   model,
		Date:    TicketDate{Started: now, Ended: now},
		Author:  GetGitAuthor(),
		Commit:  GetGitCommit(),
		Bundles: BuildTicketBundlesWithChangedDefs(&iterationFiles, bundles, baseCommit),
	}
	ticket.Frontmatter.Iterations = append(ticket.Frontmatter.Iterations, iteration)
	return SaveTicket(ticket)
}

func CollectTicketFilePaths(files *TicketIterationFiles) []string {
	if files == nil {
		return nil
	}
	pathsByName := map[string]bool{}
	paths := []string{}
	for _, file := range files.Updated {
		path := NormalizePath(file.Path)
		if path == "" {
			continue
		}
		if !pathsByName[path] {
			pathsByName[path] = true
			paths = append(paths, path)
		}
	}
	for _, file := range files.Created {
		path := NormalizePath(file.Path)
		if path == "" {
			continue
		}
		if !pathsByName[path] {
			pathsByName[path] = true
			paths = append(paths, path)
		}
	}
	for _, file := range files.Removed {
		path := NormalizePath(file.Path)
		if path == "" {
			continue
		}
		if !pathsByName[path] {
			pathsByName[path] = true
			paths = append(paths, path)
		}
	}
	return paths
}

func CollectBundleFilePaths(bundles TicketBundles) []string {
	if bundles == nil {
		return nil
	}
	pathsByName := map[string]bool{}
	paths := []string{}
	for _, bundleMetrics := range bundles {
		for path := range bundleMetrics.Files {
			normalizedPath := NormalizePath(path)
			if normalizedPath == "" {
				continue
			}
			if !pathsByName[normalizedPath] {
				pathsByName[normalizedPath] = true
				paths = append(paths, normalizedPath)
			}
		}
	}
	return paths
}

func BuildGitDiffArgs(flag, baseCommit, headCommit string, paths []string) []string {
	if headCommit == "" {
		if len(paths) == 0 {
			return []string{"diff", flag, "--no-renames", baseCommit}
		}
		return append([]string{"diff", flag, "--no-renames", baseCommit, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "--no-renames", baseCommit, headCommit}
	}
	return append([]string{"diff", flag, "--no-renames", baseCommit, headCommit, "--"}, paths...)
}

func GetGitDiffFileLineMetrics(baseCommit, headCommit string, paths []string) (TicketIterationFiles, graph.LineMetrics, error) {
	if baseCommit == "" {
		return TicketIterationFiles{}, graph.LineMetrics{}, fmt.Errorf("base commit is required")
	}
	stdout, stderr, exitCode := ExecCommand("git", BuildGitDiffArgs("--numstat", baseCommit, headCommit, paths), "")
	if exitCode != 0 {
		return TicketIterationFiles{}, graph.LineMetrics{}, fmt.Errorf("git diff numstat failed: %s", strings.TrimSpace(stderr))
	}
	lineMetricsByPath := map[string]graph.LineMetrics{}
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if strings.TrimSpace(line) == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 3 {
			continue
		}
		added := 0
		if parts[0] != "-" {
			if parsed, err := strconv.Atoi(parts[0]); err == nil {
				added = parsed
			}
		}
		removed := 0
		if parts[1] != "-" {
			if parsed, err := strconv.Atoi(parts[1]); err == nil {
				removed = parsed
			}
		}
		lineMetricsByPath[NormalizePath(parts[2])] = graph.LineMetrics{Added: added, Removed: removed}
	}
	stdout, stderr, exitCode = ExecCommand("git", BuildGitDiffArgs("--name-status", baseCommit, headCommit, paths), "")
	if exitCode != 0 {
		return TicketIterationFiles{}, graph.LineMetrics{}, fmt.Errorf("git diff name-status failed: %s", strings.TrimSpace(stderr))
	}
	statusByPath := map[string]string{}
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if strings.TrimSpace(line) == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		statusByPath[NormalizePath(parts[len(parts)-1])] = strings.TrimSpace(parts[0])
	}
	for path := range lineMetricsByPath {
		if _, ok := statusByPath[path]; !ok {
			statusByPath[path] = "M"
		}
	}
	if len(statusByPath) == 0 && len(lineMetricsByPath) == 0 {
		return TicketIterationFiles{}, graph.LineMetrics{}, nil
	}
	resultPaths := make([]string, 0, len(statusByPath))
	for path := range statusByPath {
		resultPaths = append(resultPaths, path)
	}
	sort.Strings(resultPaths)
	files := TicketIterationFiles{}
	total := graph.LineMetrics{}
	for i := 0; i < len(resultPaths); i++ {
		path := resultPaths[i]
		status := statusByPath[path]
		lineMetrics := lineMetricsByPath[path]
		total.Added += lineMetrics.Added
		total.Removed += lineMetrics.Removed
		if strings.HasPrefix(status, "A") {
			files.Created = append(files.Created, FileLineMetrics{Path: path, Lines: &lineMetrics})
			continue
		}
		if strings.HasPrefix(status, "D") {
			files.Removed = append(files.Removed, FileLineMetrics{Path: path, Lines: &lineMetrics})
			continue
		}
		files.Updated = append(files.Updated, FileLineMetrics{Path: path, Lines: &lineMetrics})
	}
	return files, total, nil
}

func ResolveBundleForPath(filePath string, bundles []Bundle) string {
	normalizedPath := NormalizePath(filePath)
	var longestRoot string
	var bundleName string
	for _, bundle := range bundles {
		if bundle.Root == "" {
			continue
		}
		bundleRoot := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, bundleRoot+"/") || normalizedPath == bundleRoot {
			if len(bundleRoot) > len(longestRoot) {
				longestRoot = bundleRoot
				bundleName = bundle.Name
			}
		}
	}
	return bundleName
}

func GuessSectionName(filePath string) string {
	base := filepath.Base(filePath)
	ext := filepath.Ext(base)
	name := strings.TrimSuffix(base, ext)
	name = strings.ReplaceAll(name, "-", " ")
	name = strings.ReplaceAll(name, "_", " ")
	name = strings.ReplaceAll(name, ".", " ")
	words := strings.Fields(name)
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}

func BuildTicketBundles(files *TicketIterationFiles, bundles []Bundle, baseCommit string) TicketBundles {
	if files == nil {
		return nil
	}
	result := make(TicketBundles)
	processFile := func(f FileLineMetrics) {
		bundleName := ResolveBundleForPath(f.Path, bundles)
		if bundleName == "" {
			bundleName = "@semio"
		}
		if _, ok := result[bundleName]; !ok {
			result[bundleName] = TicketBundleMetrics{Files: make(map[string]TicketFileMetrics)}
		}
		bundle := result[bundleName]
		if bundle.Files == nil {
			bundle.Files = make(map[string]TicketFileMetrics)
		}
		TicketFileMetrics := TicketFileMetrics{Sections: make(map[string]TicketSectionMetrics)}
		absPath := filepath.Join(rootDir, f.Path)
		if FileExists(absPath) {
			sectionLines := GetGitDiffSectionLineMetrics(baseCommit, "", f.Path)
			for sectionName, lines := range sectionLines {
				defs := ExtractDefinitionsFromSection(f.Path, sectionName)
				TicketFileMetrics.Sections[sectionName] = TicketSectionMetrics{
					Definitions: defs,
					Lines:       &graph.LineMetrics{Added: lines.Added, Removed: lines.Removed},
				}
			}
		}
		if len(TicketFileMetrics.Sections) == 0 && f.Lines != nil {
			guessedSection := GuessSectionName(f.Path)
			TicketFileMetrics.Sections[guessedSection] = TicketSectionMetrics{
				Lines: f.Lines,
			}
		}
		bundle.Files[f.Path] = TicketFileMetrics
		result[bundleName] = bundle
	}
	for _, f := range files.Updated {
		processFile(f)
	}
	for _, f := range files.Created {
		processFile(f)
	}
	for _, f := range files.Removed {
		processFile(f)
	}
	return result
}

func BuildTicketBundlesWithChangedDefs(files *TicketIterationFiles, bundles []Bundle, baseCommit string) TicketBundles {
	if files == nil {
		return nil
	}
	result := make(TicketBundles)
	processFile := func(f FileLineMetrics) {
		bundleName := ResolveBundleForPath(f.Path, bundles)
		if bundleName == "" {
			bundleName = "@semio"
		}
		if _, ok := result[bundleName]; !ok {
			result[bundleName] = TicketBundleMetrics{Files: make(map[string]TicketFileMetrics)}
		}
		bundle := result[bundleName]
		if bundle.Files == nil {
			bundle.Files = make(map[string]TicketFileMetrics)
		}
		TicketFileMetrics := TicketFileMetrics{Sections: make(map[string]TicketSectionMetrics)}
		absPath := filepath.Join(rootDir, f.Path)
		if FileExists(absPath) {
			changedLines := GetGitDiffChangedLines(baseCommit, "", f.Path)
			sectionLines := GetGitDiffSectionLineMetrics(baseCommit, "", f.Path)
			for sectionName, lines := range sectionLines {
				defs := ExtractChangedDefinitionsFromSection(f.Path, sectionName, changedLines)
				TicketFileMetrics.Sections[sectionName] = TicketSectionMetrics{
					Definitions: defs,
					Lines:       &graph.LineMetrics{Added: lines.Added, Removed: lines.Removed},
				}
			}
		}
		if len(TicketFileMetrics.Sections) == 0 && f.Lines != nil {
			guessedSection := GuessSectionName(f.Path)
			TicketFileMetrics.Sections[guessedSection] = TicketSectionMetrics{
				Lines: f.Lines,
			}
		}
		bundle.Files[f.Path] = TicketFileMetrics
		result[bundleName] = bundle
	}
	for _, f := range files.Updated {
		processFile(f)
	}
	for _, f := range files.Created {
		processFile(f)
	}
	for _, f := range files.Removed {
		processFile(f)
	}
	return result
}

func AggregateBundleLines(bundles TicketBundles) graph.LineMetrics {
	var total graph.LineMetrics
	for _, bundleMetrics := range bundles {
		for _, fileMetrics := range bundleMetrics.Files {
			for _, sectionMetrics := range fileMetrics.Sections {
				if sectionMetrics.Lines != nil {
					total.Added += sectionMetrics.Lines.Added
					total.Removed += sectionMetrics.Lines.Removed
				}
			}
		}
	}
	return total
}

func GetGitDiffSectionLineMetrics(baseCommit, endCommit, filePath string) map[string]graph.LineMetrics {
	absPath := filepath.Join(rootDir, filePath)
	content, err := ReadTextFile(absPath)
	if err != nil {
		return nil
	}
	lang := GetLanguage(filePath)
	if lang == nil {
		return nil
	}
	sections := lang.ParseSections(content)
	if len(sections) == 0 {
		return nil
	}
	args := []string{"diff", "--numstat", "-U0"}
	if baseCommit != "" {
		if endCommit != "" {
			args = append(args, baseCommit, endCommit)
		} else {
			args = append(args, baseCommit, "HEAD")
		}
	}
	args = append(args, "--", filePath)
	cmd := exec.Command("git", args...)
	cmd.Dir = rootDir
	out, err := cmd.Output()
	if err != nil {
		return nil
	}
	diffArgs := []string{"diff", "-U0"}
	if baseCommit != "" {
		if endCommit != "" {
			diffArgs = append(diffArgs, baseCommit, endCommit)
		} else {
			diffArgs = append(diffArgs, baseCommit, "HEAD")
		}
	}
	diffArgs = append(diffArgs, "--", filePath)
	diffCmd := exec.Command("git", diffArgs...)
	diffCmd.Dir = rootDir
	diffOut, err := diffCmd.Output()
	if err != nil {
		return nil
	}
	lineChanges := parseGitDiffHunks(string(diffOut))
	result := make(map[string]graph.LineMetrics)
	flatSections := flattenSections(sections)
	guessedSection := GuessSectionName(filePath)
	for _, change := range lineChanges {
		sectionName := guessedSection
		for _, sec := range flatSections {
			if change.lineNum >= sec.StartLine && (sec.EndLine == -1 || change.lineNum <= sec.EndLine) {
				sectionName = sec.Name
			}
		}
		metrics := result[sectionName]
		if change.isAdd {
			metrics.Added++
		} else {
			metrics.Removed++
		}
		result[sectionName] = metrics
	}
	_ = out
	return result
}

type lineChange struct {
	lineNum int
	isAdd   bool
}

func parseGitDiffHunks(diff string) []lineChange {
	var changes []lineChange
	hunkRe := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	lines := strings.Split(diff, "\n")
	var currentAddLine, currentRemoveLine int
	for _, line := range lines {
		if match := hunkRe.FindStringSubmatch(line); match != nil {
			currentRemoveLine, _ = strconv.Atoi(match[1])
			currentAddLine, _ = strconv.Atoi(match[3])
			continue
		}
		if strings.HasPrefix(line, "+") && !strings.HasPrefix(line, "+++") {
			changes = append(changes, lineChange{lineNum: currentAddLine, isAdd: true})
			currentAddLine++
		} else if strings.HasPrefix(line, "-") && !strings.HasPrefix(line, "---") {
			changes = append(changes, lineChange{lineNum: currentRemoveLine, isAdd: false})
			currentRemoveLine++
		} else if !strings.HasPrefix(line, "\\") && line != "" {
			currentAddLine++
			currentRemoveLine++
		}
	}
	return changes
}

func flattenSections(sections []SectionInfo) []SectionInfo {
	var result []SectionInfo
	var flatten func(secs []SectionInfo)
	flatten = func(secs []SectionInfo) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

func GetGitDiffChangedLines(baseCommit, endCommit, filePath string) map[int]bool {
	diffArgs := []string{"diff", "-U0"}
	if baseCommit != "" {
		if endCommit != "" {
			diffArgs = append(diffArgs, baseCommit, endCommit)
		} else {
			diffArgs = append(diffArgs, baseCommit, "HEAD")
		}
	}
	diffArgs = append(diffArgs, "--", filePath)
	diffCmd := exec.Command("git", diffArgs...)
	diffCmd.Dir = rootDir
	diffOut, err := diffCmd.Output()
	if err != nil {
		return nil
	}
	lineChanges := parseGitDiffHunks(string(diffOut))
	changedLines := make(map[int]bool)
	for _, change := range lineChanges {
		changedLines[change.lineNum] = true
	}
	return changedLines
}

func ExtractChangedDefinitionsFromSection(filePath, sectionName string, changedLines map[int]bool) []string {
	absPath := filepath.Join(rootDir, filePath)
	content, err := ReadTextFile(absPath)
	if err != nil {
		return nil
	}
	lang := GetLanguage(filePath)
	if lang == nil || !lang.SupportsDefinitions() {
		return nil
	}
	sections := lang.ParseSections(content)
	var section *SectionInfo
	flatSections := flattenSections(sections)
	for i := range flatSections {
		if flatSections[i].Name == sectionName {
			section = &flatSections[i]
			break
		}
	}
	if section == nil {
		return nil
	}
	lines := strings.Split(content, "\n")
	parsedDefs := lang.ParseDefinitions(content, lines)
	var defs []string
	seen := make(map[string]bool)
	endLine := section.EndLine
	if endLine == -1 {
		endLine = len(lines)
	}
	for _, def := range parsedDefs {
		if def.Start >= section.StartLine && def.Start <= endLine {
			for lineNum := def.Start; lineNum <= def.End && lineNum <= endLine; lineNum++ {
				if changedLines[lineNum] && !seen[def.Name] {
					seen[def.Name] = true
					defs = append(defs, def.Name)
					break
				}
			}
		}
	}
	return defs
}

func ExtractDefinitionsFromSection(filePath, sectionName string) []string {
	absPath := filepath.Join(rootDir, filePath)
	content, err := ReadTextFile(absPath)
	if err != nil {
		return nil
	}
	lang := GetLanguage(filePath)
	if lang == nil || !lang.SupportsDefinitions() {
		return nil
	}
	sections := lang.ParseSections(content)
	var section *SectionInfo
	flatSections := flattenSections(sections)
	for i := range flatSections {
		if flatSections[i].Name == sectionName {
			section = &flatSections[i]
			break
		}
	}
	if section == nil {
		return nil
	}
	lines := strings.Split(content, "\n")
	parsedDefs := lang.ParseDefinitions(content, lines)
	var defs []string
	seen := make(map[string]bool)
	endLine := section.EndLine
	if endLine == -1 {
		endLine = len(lines)
	}
	for _, def := range parsedDefs {
		if def.Start >= section.StartLine && def.Start <= endLine {
			if !seen[def.Name] {
				seen[def.Name] = true
				defs = append(defs, def.Name)
			}
		}
	}
	return defs
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
	baseCommit := ticket.Frontmatter.Commit
	for i := lastIdx - 1; i >= 0; i-- {
		if ticket.Frontmatter.Iterations[i].Commit != "" {
			baseCommit = ticket.Frontmatter.Iterations[i].Commit
			break
		}
	}
	declaredPaths := CollectTicketFilePaths(last.Declared)
	iterationFiles, _, err := GetGitDiffFileLineMetrics(baseCommit, "", declaredPaths)
	if err != nil {
		return err
	}
	if len(iterationFiles.Updated) == 0 && len(iterationFiles.Created) == 0 && len(iterationFiles.Removed) == 0 {
		return fmt.Errorf("iteration requires at least one file")
	}
	bundles := GetProjects()
	last.Date.Ended = ISOTimestamp()
	last.Commit = GetGitCommit()
	last.Bundles = BuildTicketBundles(&iterationFiles, bundles, baseCommit)
	return SaveTicket(ticket)
}

func FinishTicket(ticket *Ticket) error {
	if len(ticket.Frontmatter.Iterations) > 0 {
		last := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
		if last.Date.Ended == "" {
			return fmt.Errorf("cannot finish ticket with unfinished iteration")
		}
	}
	ticket.Frontmatter.Status = graph.TicketStatusClosed
	ticket.Frontmatter.Date.Finished = ISOTimestamp()
	return SaveTicket(ticket)
}

func ReopenTicket(ticket *Ticket) error {
	if ticket.Frontmatter.Status == graph.TicketStatusOpen {
		return fmt.Errorf("ticket is already open")
	}
	if len(ticket.Frontmatter.Iterations) > 0 {
		last := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
		if last.Date.Ended == "" {
			return fmt.Errorf("cannot reopen ticket with unfinished iteration")
		}
	}
	ticket.Frontmatter.Status = graph.TicketStatusOpen
	ticket.Frontmatter.Date.Finished = ""
	return SaveTicket(ticket)
}

func MigrateTicketBundles(bundles TicketBundles) TicketBundles {
	if bundles == nil {
		return nil
	}
	result := make(TicketBundles)
	for bundleName, bundleMetrics := range bundles {
		newBundle := TicketBundleMetrics{Files: make(map[string]TicketFileMetrics)}
		for filePath, fileMetrics := range bundleMetrics.Files {
			newFileMetrics := TicketFileMetrics{Sections: make(map[string]TicketSectionMetrics)}
			for sectionName, sectionMetrics := range fileMetrics.Sections {
				newSectionName := sectionName
				if sectionName == "_root" {
					newSectionName = GuessSectionName(filePath)
				}
				newFileMetrics.Sections[newSectionName] = sectionMetrics
			}
			if len(newFileMetrics.Sections) == 0 {
				newFileMetrics.Sections[GuessSectionName(filePath)] = TicketSectionMetrics{}
			}
			newBundle.Files[filePath] = newFileMetrics
		}
		result[bundleName] = newBundle
	}
	return result
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

type ContributorContributionState struct {
	Tickets     map[string]ContributorTicket
	Files       map[string]struct{}
	Folders     map[string]struct{}
	Bundles     map[string]struct{}
	Regions     map[string]struct{}
	Definitions map[string]struct{}
	Commits     map[string]ContributorCommit
	Lines       graph.LineMetrics
}

func ParseContributorIdentity(value string) (string, string, bool) {
	trimmed := strings.TrimSpace(value)
	if strings.HasPrefix(trimmed, "//") {
		trimmed = strings.TrimSpace(strings.TrimPrefix(trimmed, "//"))
	}
	if strings.HasPrefix(trimmed, "#") {
		trimmed = strings.TrimSpace(strings.TrimPrefix(trimmed, "#"))
	}
	if strings.HasPrefix(trimmed, "/*") {
		trimmed = strings.TrimSpace(strings.TrimPrefix(trimmed, "/*"))
	}
	if strings.HasPrefix(trimmed, "*") {
		trimmed = strings.TrimSpace(strings.TrimPrefix(trimmed, "*"))
	}
	if trimmed == "" {
		return "", "", false
	}
	match := regexp.MustCompile(`^\s*(?:\d{4}\s+)?(.+?)\s*<([^>]+)>\s*$`).FindStringSubmatch(trimmed)
	if match == nil {
		return "", "", false
	}
	name := strings.TrimSpace(match[1])
	email := strings.TrimSpace(match[2])
	if name == "" || email == "" {
		return "", "", false
	}
	return name, email, true
}

func ResolveContributorGithub(name, email string, emailToGithub map[string]string, nameToGithub map[string]string) string {
	if email != "" {
		if github, ok := emailToGithub[strings.ToLower(email)]; ok {
			return github
		}
	}
	if name != "" {
		if github, ok := nameToGithub[strings.ToLower(name)]; ok {
			return github
		}
	}
	return ""
}

func GetGitCommitTitle(sha string) string {
	if sha == "" {
		return ""
	}
	stdout, _, exitCode := ExecCommand("git", []string{"show", "-s", "--format=%s", sha}, "")
	if exitCode != 0 {
		return ""
	}
	return strings.TrimSpace(stdout)
}

func addSectionsToContributor(state *ContributorContributionState, filePath string, section SectionInfo) {
	regionKey := filePath + "#" + section.Name
	state.Regions[regionKey] = struct{}{}
	for _, child := range section.Children {
		addSectionsToContributor(state, filePath, child)
	}
}

func findSectionForDefinition(sections []SectionInfo, defStart, defEnd int, parentPath string) string {
	for _, section := range sections {
		if defStart >= section.StartLine && defEnd <= section.EndLine {
			sectionPath := section.Name
			if parentPath != "" {
				sectionPath = parentPath + "/" + section.Name
			}
			if len(section.Children) > 0 {
				if childPath := findSectionForDefinition(section.Children, defStart, defEnd, sectionPath); childPath != "" {
					return childPath
				}
			}
			return sectionPath
		}
	}
	return parentPath
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
	if len(contributors) == 0 {
		return contributors, nil
	}
	emailToGithub := map[string]string{}
	nameToGithub := map[string]string{}
	stateByGithub := map[string]*ContributorContributionState{}
	for i := range contributors {
		contributors[i].Contributions = ContributorContributions{}
		stateByGithub[contributors[i].Github] = &ContributorContributionState{
			Tickets:     map[string]ContributorTicket{},
			Files:       map[string]struct{}{},
			Folders:     map[string]struct{}{},
			Bundles:     map[string]struct{}{},
			Regions:     map[string]struct{}{},
			Definitions: map[string]struct{}{},
			Commits:     map[string]ContributorCommit{},
		}
		for _, email := range contributors[i].Emails {
			emailToGithub[strings.ToLower(email)] = contributors[i].Github
		}
		if contributors[i].Name != "" {
			nameToGithub[strings.ToLower(contributors[i].Name)] = contributors[i].Github
		}
	}
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	commitTitleCache := map[string]string{}
	for _, ticket := range tickets {
		ticketKey := fmt.Sprintf("%04d-%02d-%02d-%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		ticketContributors := map[string]struct{}{}
		if name, email, ok := ParseContributorIdentity(ticket.Frontmatter.Author); ok {
			if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" {
				ticketContributors[github] = struct{}{}
			}
		}
		for _, iteration := range ticket.Frontmatter.Iterations {
			if name, email, ok := ParseContributorIdentity(iteration.Author); ok {
				if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" {
					ticketContributors[github] = struct{}{}
					if iteration.Bundles != nil {
						iterationLines := AggregateBundleLines(iteration.Bundles)
						stateByGithub[github].Lines.Added += iterationLines.Added
						stateByGithub[github].Lines.Removed += iterationLines.Removed
					}
					if iteration.Commit != "" {
						commitTitle := commitTitleCache[iteration.Commit]
						if commitTitle == "" {
							commitTitle = GetGitCommitTitle(iteration.Commit)
							if commitTitle == "" {
								commitTitle = iteration.Commit
							}
							commitTitleCache[iteration.Commit] = commitTitle
						}
						stateByGithub[github].Commits[iteration.Commit] = ContributorCommit{Title: commitTitle, Sha: iteration.Commit}
					}
				}
			}
		}
		if ticket.Frontmatter.Commit != "" {
			if name, email, ok := ParseContributorIdentity(ticket.Frontmatter.Author); ok {
				if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" {
					commitTitle := commitTitleCache[ticket.Frontmatter.Commit]
					if commitTitle == "" {
						commitTitle = GetGitCommitTitle(ticket.Frontmatter.Commit)
						if commitTitle == "" {
							commitTitle = ticket.Frontmatter.Commit
						}
						commitTitleCache[ticket.Frontmatter.Commit] = commitTitle
					}
					stateByGithub[github].Commits[ticket.Frontmatter.Commit] = ContributorCommit{Title: commitTitle, Sha: ticket.Frontmatter.Commit}
				}
			}
		}
		for github := range ticketContributors {
			stateByGithub[github].Tickets[ticketKey] = ContributorTicket{
				Year:     ticket.Year,
				Month:    ticket.Month,
				Day:      ticket.Day,
				Slug:     ticket.Slug,
				Status:   ticket.Frontmatter.Status,
				FilePath: ticket.FilePath,
			}
		}
	}
	files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, nil)
	if err != nil {
		return nil, err
	}
	for _, filePath := range files {
		absPath := filepath.Join(rootDir, filePath)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, filePath)
		headerSection := FindSection(sections, "Header")
		if headerSection == nil {
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		lines := strings.Split(content, "\n")
		lang := GetLanguage(filePath)
		var defs []DefinitionRange
		if lang != nil && lang.SupportsDefinitions() {
			defs = lang.ParseDefinitions(content, lines)
		}
		for _, line := range strings.Split(headerContent, "\n") {
			if name, email, ok := ParseContributorIdentity(line); ok {
				if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" {
					stateByGithub[github].Files[filePath] = struct{}{}
					folder := NormalizePath(filepath.Dir(filePath))
					if folder != "." {
						stateByGithub[github].Folders[folder] = struct{}{}
					}
					for _, section := range sections {
						addSectionsToContributor(stateByGithub[github], filePath, section)
					}
					for _, def := range defs {
						sectionPath := findSectionForDefinition(sections, def.Start, def.End, "")
						var defKey string
						if sectionPath != "" {
							defKey = filePath + "#" + sectionPath + "§" + def.Name
						} else {
							defKey = filePath + "§" + def.Name
						}
						stateByGithub[github].Definitions[defKey] = struct{}{}
					}
				}
			}
		}
	}
	bundles := GetProjects()
	var projectRoots []struct {
		name string
		root string
	}
	for _, bundle := range bundles {
		if bundle.Root != "" {
			projectRoots = append(projectRoots, struct {
				name string
				root string
			}{name: bundle.Name, root: NormalizePath(bundle.Root)})
		}
	}
	sort.SliceStable(projectRoots, func(i, j int) bool {
		return len(projectRoots[i].root) > len(projectRoots[j].root)
	})
	for _, contributor := range contributors {
		state := stateByGithub[contributor.Github]
		for filePath := range state.Files {
			for _, bundle := range projectRoots {
				if strings.HasPrefix(filePath, bundle.root+"/") || filePath == bundle.root {
					state.Bundles[bundle.name] = struct{}{}
					break
				}
			}
		}
	}
	for i := range contributors {
		state := stateByGithub[contributors[i].Github]
		for ticketKey := range state.Tickets {
			contributors[i].Contributions.Tickets = append(contributors[i].Contributions.Tickets, state.Tickets[ticketKey])
		}
		sort.SliceStable(contributors[i].Contributions.Tickets, func(a, b int) bool {
			left := contributors[i].Contributions.Tickets[a]
			right := contributors[i].Contributions.Tickets[b]
			if left.Year != right.Year {
				return left.Year > right.Year
			}
			if left.Month != right.Month {
				return left.Month > right.Month
			}
			if left.Day != right.Day {
				return left.Day > right.Day
			}
			return left.Slug < right.Slug
		})
		for filePath := range state.Files {
			contributors[i].Contributions.Files = append(contributors[i].Contributions.Files, filePath)
		}
		sort.Strings(contributors[i].Contributions.Files)
		for folder := range state.Folders {
			contributors[i].Contributions.Folders = append(contributors[i].Contributions.Folders, folder)
		}
		sort.Strings(contributors[i].Contributions.Folders)
		for bundle := range state.Bundles {
			contributors[i].Contributions.Bundles = append(contributors[i].Contributions.Bundles, bundle)
		}
		sort.Strings(contributors[i].Contributions.Bundles)
		for region := range state.Regions {
			contributors[i].Contributions.Regions = append(contributors[i].Contributions.Regions, region)
		}
		sort.Strings(contributors[i].Contributions.Regions)
		for definition := range state.Definitions {
			contributors[i].Contributions.Definitions = append(contributors[i].Contributions.Definitions, definition)
		}
		sort.Strings(contributors[i].Contributions.Definitions)
		for _, commit := range state.Commits {
			contributors[i].Contributions.Commits = append(contributors[i].Contributions.Commits, commit)
		}
		sort.SliceStable(contributors[i].Contributions.Commits, func(a, b int) bool {
			left := contributors[i].Contributions.Commits[a]
			right := contributors[i].Contributions.Commits[b]
			if left.Title != right.Title {
				return left.Title < right.Title
			}
			return left.Sha < right.Sha
		})
		if state.Lines.Added != 0 || state.Lines.Removed != 0 {
			lines := state.Lines
			contributors[i].Contributions.Lines = &lines
		}
	}
	sort.SliceStable(contributors, func(i, j int) bool {
		leftTickets := len(contributors[i].Contributions.Tickets)
		rightTickets := len(contributors[j].Contributions.Tickets)
		if leftTickets != rightTickets {
			return leftTickets > rightTickets
		}
		return contributors[i].Github < contributors[j].Github
	})
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

func AddContributorProject(github string, bundle string) error {
	contributor, err := ReadContributor(github)
	if err != nil {
		contributor, err = CreateContributor(github)
		if err != nil {
			return err
		}
	}
	for _, p := range contributor.Contributions.Bundles {
		if p == bundle {
			return nil
		}
	}
	contributor.Contributions.Bundles = append(contributor.Contributions.Bundles, bundle)
	return SaveContributor(contributor)
}

// #endregion Contributors

// #region Nx

var (
	cachedProjectNames   []string
	cachedProjectDetails = make(map[string]Bundle)
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

func GetProjectDetails(name string) Bundle {
	nxMutex.Lock()
	defer nxMutex.Unlock()
	if proj, ok := cachedProjectDetails[name]; ok {
		return proj
	}
	stdout, _, exitCode := ExecCommand("npx", []string{"nx", "show", "project", name, "--json"}, "")
	if exitCode != 0 {
		proj := Bundle{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(stdout), &config); err != nil {
		proj := Bundle{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	proj := Bundle{Name: name}
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

func GetProjects() []Bundle {
	names := GetProjectNames()
	bundles := make([]Bundle, len(names))
	for i, name := range names {
		bundles[i] = GetProjectDetails(name)
	}
	return bundles
}

func RunNxTarget(target string, bundles []string, extraArgs []string) (success bool, output string) {
	args := []string{"nx"}
	if len(bundles) == 1 {
		args = append(args, "run", bundles[0]+":"+target)
	} else if len(bundles) > 1 {
		args = append(args, "run-many", "-t", target, "-p", strings.Join(bundles, ","))
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

func ScopeToFiles(scope Scope, bundles []Bundle) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case ScopeRepo:
		files, err = SimpleGlob("**/*.{ts,tsx,py,cs,go}", rootDir, ignorePatterns, true)
	case ScopeProject:
		for _, proj := range bundles {
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

type repoContext struct {
	rootDir string
}

var _ graph.RepoContext = (*repoContext)(nil)

func NewRepoContext(dir string) graph.RepoContext {
	return &repoContext{rootDir: dir}
}

func (c *repoContext) GetRootDir() string {
	return c.rootDir
}

func (c *repoContext) GetBundles() []*graph.Bundle {
	bundles := GetProjects()
	result := make([]*graph.Bundle, len(bundles))
	for i, b := range bundles {
		result[i] = &graph.Bundle{
			ID:   fmt.Sprintf("bundle:%s", b.Name),
			Name: b.Name,
			Root: b.Root,
			URI:  fmt.Sprintf("file://%s/%s", c.rootDir, b.Root),
			Tags: b.Tags,
		}
	}
	return result
}

func (c *repoContext) GetContributors() ([]*graph.Contributor, error) {
	contributors, err := ListContributors()
	if err != nil {
		return nil, err
	}
	result := make([]*graph.Contributor, len(contributors))
	for i, contrib := range contributors {
		var links []graph.ContributorLink
		for name, url := range contrib.Links {
			links = append(links, graph.ContributorLink{Name: name, URL: url})
		}
		var namePtr *string
		if contrib.Name != "" {
			namePtr = &contrib.Name
		}
		result[i] = &graph.Contributor{
			ID:      fmt.Sprintf("contributor:%s", contrib.Github),
			Github:  contrib.Github,
			Name:    namePtr,
			Emails:  contrib.Emails,
			Links:   links,
			Bundles: []*graph.Bundle{},
			Files:   []*graph.File{},
			Tickets: []*graph.Ticket{},
			Metrics: &graph.ContributorMetrics{
				Commits:     0,
				Tickets:     0,
				Bundles:     0,
				Folders:     0,
				Files:       0,
				Sections:    0,
				Definitions: 0,
				Lines:       0,
			},
		}
	}
	return result, nil
}

func (c *repoContext) GetTickets(year, month, day *int, status *graph.TicketStatus) ([]*graph.Ticket, error) {
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*graph.Ticket
	for _, t := range tickets {
		ticketStatus := graph.TicketStatusOpen
		if t.Frontmatter.Status == graph.TicketStatusClosed {
			ticketStatus = graph.TicketStatusClosed
		}
		if status != nil && ticketStatus != *status {
			continue
		}
		var summaryPtr *string
		if t.Frontmatter.Summary != "" {
			summaryPtr = &t.Frontmatter.Summary
		}
		createdTime, _ := time.Parse(time.RFC3339, t.Frontmatter.Date.Created)
		var finishedPtr *time.Time
		if t.Frontmatter.Date.Finished != "" {
			finishedTime, err := time.Parse(time.RFC3339, t.Frontmatter.Date.Finished)
			if err == nil {
				finishedPtr = &finishedTime
			}
		}
		result = append(result, &graph.Ticket{
			ID:      fmt.Sprintf("ticket:%d/%02d/%02d/%s", t.Year, t.Month, t.Day, t.Slug),
			Year:    t.Year,
			Month:   t.Month,
			Day:     t.Day,
			Slug:    t.Slug,
			Path:    fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", t.Year, t.Month, t.Day, t.Slug),
			URI:     fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", c.rootDir, t.Year, t.Month, t.Day, t.Slug),
			Prompt:  t.Frontmatter.Prompt,
			Summary: summaryPtr,
			Status:  ticketStatus,
			Date: &graph.TicketDate{
				Created:  createdTime,
				Finished: finishedPtr,
			},
			Bundles: []*graph.Bundle{},
			Files:   []*graph.File{},
			Metrics: &graph.TicketMetrics{
				Iterations: len(t.Frontmatter.Iterations),
				Bundles:    0,
				Files:      0,
				Lines:      nil,
			},
		})
	}
	return result, nil
}

func (c *repoContext) GetPolicies() []*graph.Policy {
	policies := GetPolicies()
	result := make([]*graph.Policy, len(policies))
	for i, p := range policies {
		var scopes []string
		scopes = append(scopes, p.Scopes...)
		var descPtr *string
		if p.Description != "" {
			descPtr = &p.Description
		}
		var violationKinds []*graph.ViolationKind
		for _, kind := range p.Kinds {
			info := kind.Info()
			priority := graph.ViolationPriorityMedium
			switch info.Priority {
			case graph.ViolationPriorityHigh:
				priority = graph.ViolationPriorityHigh
			case graph.ViolationPriorityLow:
				priority = graph.ViolationPriorityLow
			}
			violationKinds = append(violationKinds, &graph.ViolationKind{
				ID:          fmt.Sprintf("violationKind:%s", kind),
				PolicyID:    fmt.Sprintf("policy:%s", p.ID),
				Priority:    priority,
				Autofixable: info.Autofixable,
				Reason:      info.Reason,
				Solution:    info.Solution,
			})
		}
		result[i] = &graph.Policy{
			ID:             fmt.Sprintf("policy:%s", p.ID),
			Name:           p.ID,
			Description:    descPtr,
			Scopes:         scopes,
			ViolationKinds: violationKinds,
		}
	}
	return result
}

func (c *repoContext) GetViolationKinds() []*graph.ViolationKind {
	var result []*graph.ViolationKind
	for _, p := range GetPolicies() {
		for _, kind := range p.Kinds {
			info := kind.Info()
			priority := graph.ViolationPriorityMedium
			switch info.Priority {
			case graph.ViolationPriorityHigh:
				priority = graph.ViolationPriorityHigh
			case graph.ViolationPriorityLow:
				priority = graph.ViolationPriorityLow
			}
			result = append(result, &graph.ViolationKind{
				ID:          fmt.Sprintf("violationKind:%s", kind),
				PolicyID:    fmt.Sprintf("policy:%s", p.ID),
				Priority:    priority,
				Autofixable: info.Autofixable,
				Reason:      info.Reason,
				Solution:    info.Solution,
			})
		}
	}
	return result
}

func (c *repoContext) Analyze(scope *string) (*graph.AnalyzeResult, error) {
	scopeRaw := "@semio"
	if scope != nil {
		scopeRaw = *scope
	}
	toolResult := ToolAnalyze(scopeRaw, nil)
	report, ok := toolResult.Data.(AnalyzeReport)
	if !ok {
		return &graph.AnalyzeResult{
			Violations: []*graph.Violation{},
			Metrics:    &graph.AnalyzeMetrics{Total: 0, ByPriority: &graph.PriorityCount{}, Autofixable: 0},
		}, nil
	}
	kindInfoMap := make(map[ViolationKind]ViolationKindInfo)
	for _, p := range GetPolicies() {
		for _, kind := range p.Kinds {
			kindInfoMap[kind] = kind.Info()
		}
	}
	violations := make([]*graph.Violation, len(report.Violations))
	for i, v := range report.Violations {
		var excerptPtr *string
		if v.Summary != "" {
			excerptPtr = &v.Summary
		}
		info := kindInfoMap[v.Kind]
		priority := graph.ViolationPriorityMedium
		switch info.Priority {
		case graph.ViolationPriorityHigh:
			priority = graph.ViolationPriorityHigh
		case graph.ViolationPriorityLow:
			priority = graph.ViolationPriorityLow
		}
		kindID := fmt.Sprintf("violationKind:%s", v.Kind)
		parts := strings.SplitN(string(v.Kind), ":", 2)
		policyID := "unknown"
		if len(parts) > 0 {
			policyID = parts[0]
		}
		violations[i] = &graph.Violation{
			ID:      v.ID,
			KindID:  kindID,
			Kind: &graph.ViolationKind{
				ID:          kindID,
				PolicyID:    fmt.Sprintf("policy:%s", policyID),
				Priority:    priority,
				Autofixable: info.Autofixable,
				Reason:      info.Reason,
				Solution:    info.Solution,
			},
			Scope:   v.Scope,
			Excerpt: excerptPtr,
		}
	}
	return &graph.AnalyzeResult{
		Violations: violations,
		Metrics: &graph.AnalyzeMetrics{
			Total: report.Summary.Total,
			ByPriority: &graph.PriorityCount{
				High:   report.Summary.ByPriority["high"],
				Medium: report.Summary.ByPriority["medium"],
				Low:    report.Summary.ByPriority["low"],
			},
			Autofixable: 0,
		},
	}, nil
}

func (c *repoContext) Fix(scope *string) (*graph.FixResult, error) {
	scopeRaw := "@semio"
	if scope != nil {
		scopeRaw = *scope
	}
	ToolFix(scopeRaw)
	return &graph.FixResult{Fixed: 0, Remaining: 0, Violations: []*graph.Violation{}}, nil
}

func (c *repoContext) TicketCreate(input graph.TicketCreateInput) (*graph.Ticket, error) {
	var files []string
	if input.Files != nil {
		files = append(files, input.Files.Updated...)
		files = append(files, input.Files.Created...)
		files = append(files, input.Files.Removed...)
	}
	model := ""
	if input.Model != nil {
		model = *input.Model
	}
	ticket, err := CreateTicket(input.Slug, input.Prompt, model, files)
	if err != nil {
		return nil, err
	}
	return &graph.Ticket{
		ID:     fmt.Sprintf("ticket:%d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug),
		Year:   ticket.Year,
		Month:  ticket.Month,
		Day:    ticket.Day,
		Slug:   ticket.Slug,
		Path:   fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", ticket.Year, ticket.Month, ticket.Day, ticket.Slug),
		URI:    fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", c.rootDir, ticket.Year, ticket.Month, ticket.Day, ticket.Slug),
		Prompt: input.Prompt,
		Status: graph.TicketStatusOpen,
	}, nil
}

func (c *repoContext) TicketProgress(input graph.TicketProgressInput) (*graph.Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	model := ""
	if input.Model != nil {
		model = *input.Model
	}
	if err := ProgressIteration(ticket, input.Prompt, model); err != nil {
		return nil, err
	}
	status := graph.TicketStatusOpen
	if ticket.Frontmatter.Status == graph.TicketStatusClosed {
		status = graph.TicketStatusClosed
	}
	var finished *time.Time
	if ticket.Frontmatter.Date.Finished != "" {
		if parsed, err := time.Parse(time.RFC3339, ticket.Frontmatter.Date.Finished); err == nil {
			finished = &parsed
		}
	}
	created := time.Now()
	if ticket.Frontmatter.Date.Created != "" {
		if parsed, err := time.Parse(time.RFC3339, ticket.Frontmatter.Date.Created); err == nil {
			created = parsed
		}
	}
	return &graph.Ticket{
		ID:     fmt.Sprintf("ticket:%d/%02d/%02d/%s", input.Year, input.Month, input.Day, input.Slug),
		Year:   input.Year,
		Month:  input.Month,
		Day:    input.Day,
		Slug:   input.Slug,
		Path:   fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", input.Year, input.Month, input.Day, input.Slug),
		URI:    fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", c.rootDir, input.Year, input.Month, input.Day, input.Slug),
		Prompt: ticket.Frontmatter.Prompt,
		Status: status,
		Date:   &graph.TicketDate{Created: created, Finished: finished},
	}, nil
}

func (c *repoContext) TicketFinish(input graph.TicketFinishInput) (*graph.Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if err := FinishTicket(ticket); err != nil {
		return nil, err
	}
	created := time.Now()
	if ticket.Frontmatter.Date.Created != "" {
		if parsed, err := time.Parse(time.RFC3339, ticket.Frontmatter.Date.Created); err == nil {
			created = parsed
		}
	}
	now := time.Now()
	return &graph.Ticket{
		ID:     fmt.Sprintf("ticket:%d/%02d/%02d/%s", input.Year, input.Month, input.Day, input.Slug),
		Year:   input.Year,
		Month:  input.Month,
		Day:    input.Day,
		Slug:   input.Slug,
		Path:   fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", input.Year, input.Month, input.Day, input.Slug),
		URI:    fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", c.rootDir, input.Year, input.Month, input.Day, input.Slug),
		Prompt: ticket.Frontmatter.Prompt,
		Status: graph.TicketStatusClosed,
		Date:   &graph.TicketDate{Created: created, Finished: &now},
	}, nil
}

func (c *repoContext) TicketReopen(input graph.TicketReopenInput) (*graph.Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if err := ReopenTicket(ticket); err != nil {
		return nil, err
	}
	created := time.Now()
	if ticket.Frontmatter.Date.Created != "" {
		if parsed, err := time.Parse(time.RFC3339, ticket.Frontmatter.Date.Created); err == nil {
			created = parsed
		}
	}
	return &graph.Ticket{
		ID:     fmt.Sprintf("ticket:%d/%02d/%02d/%s", input.Year, input.Month, input.Day, input.Slug),
		Year:   input.Year,
		Month:  input.Month,
		Day:    input.Day,
		Slug:   input.Slug,
		Path:   fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", input.Year, input.Month, input.Day, input.Slug),
		URI:    fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", c.rootDir, input.Year, input.Month, input.Day, input.Slug),
		Prompt: ticket.Frontmatter.Prompt,
		Status: graph.TicketStatusOpen,
		Date:   &graph.TicketDate{Created: created},
	}, nil
}

func (c *repoContext) FolderCreate(path string) (*graph.Folder, error) {
	result := ToolFolderCreate(path)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	return &graph.Folder{
		ID:   fmt.Sprintf("folder:%s", normalizedPath),
		Path: normalizedPath,
		URI:  fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name: name,
	}, nil
}

func (c *repoContext) FolderMove(src, dst string) (*graph.Folder, error) {
	result := ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(dst, "\\", "/")
	name := filepath.Base(normalizedPath)
	return &graph.Folder{
		ID:   fmt.Sprintf("folder:%s", normalizedPath),
		Path: normalizedPath,
		URI:  fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name: name,
	}, nil
}

func (c *repoContext) FolderDelete(path string) error {
	result := ToolFolderDelete(path)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) FileCreate(path string) (*graph.File, error) {
	result := ToolFileCreate(path)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	var folderID *string
	if folderPath != "." {
		id := fmt.Sprintf("folder:%s", folderPath)
		folderID = &id
	}
	return &graph.File{
		ID:        fmt.Sprintf("file:%s", normalizedPath),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
	}, nil
}

func (c *repoContext) FileMove(src, dst string) (*graph.File, error) {
	result := ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(dst, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	var folderID *string
	if folderPath != "." {
		id := fmt.Sprintf("folder:%s", folderPath)
		folderID = &id
	}
	return &graph.File{
		ID:        fmt.Sprintf("file:%s", normalizedPath),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
	}, nil
}

func (c *repoContext) FileDelete(path string) error {
	result := ToolFileDelete(path)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) SectionCreate(file, name string, parent *string) (*graph.Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(file, "\\", "/")
	fileID := fmt.Sprintf("file:%s", normalizedPath)
	return &graph.Section{
		ID:     fmt.Sprintf("section:%s#%s", normalizedPath, name),
		Name:   name,
		Path:   fmt.Sprintf("%s#%s", normalizedPath, name),
		FileID: fileID,
	}, nil
}

func (c *repoContext) SectionMove(file, oldName, newName string) (*graph.Section, error) {
	result := ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(file, "\\", "/")
	fileID := fmt.Sprintf("file:%s", normalizedPath)
	return &graph.Section{
		ID:     fmt.Sprintf("section:%s#%s", normalizedPath, newName),
		Name:   newName,
		Path:   fmt.Sprintf("%s#%s", normalizedPath, newName),
		FileID: fileID,
	}, nil
}

func (c *repoContext) SectionDelete(file, name string) error {
	result := ToolSectionDelete(file, name)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) ContributorAdd(input graph.ContributorAddInput) (*graph.Contributor, error) {
	result := ToolContributorAdd(input.Github)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	contrib, ok := result.Data.(*Contributor)
	if !ok {
		return nil, fmt.Errorf("unexpected result type")
	}
	var links []graph.ContributorLink
	for name, url := range contrib.Links {
		links = append(links, graph.ContributorLink{Name: name, URL: url})
	}
	var namePtr *string
	if contrib.Name != "" {
		namePtr = &contrib.Name
	}
	return &graph.Contributor{
		ID:     fmt.Sprintf("contributor:%s", contrib.Github),
		Github: contrib.Github,
		Name:   namePtr,
		Emails: contrib.Emails,
		Links:  links,
	}, nil
}

func (c *repoContext) ContributorRemove(github string) error {
	result := ToolContributorRemove(github)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func ExecuteGraphQL(query string, variables map[string]interface{}) (string, error) {
	ctx := NewRepoContext(rootDir)
	executor, err := graph.NewExecutorWithContext(rootDir, ctx)
	if err != nil {
		return "", fmt.Errorf("failed to create executor: %w", err)
	}
	result, err := executor.ExecuteJSON(context.Background(), query, variables)
	if err != nil {
		return "", fmt.Errorf("graphql error: %w", err)
	}
	return result, nil
}

func OutputResult(result ToolResult) error {
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	return enc.Encode(result)
}

func FormatResult(result ToolResult) string {
	data, err := json.MarshalIndent(result, "", "  ")
	if err != nil {
		return fmt.Sprintf(`{"error": %q}`, err.Error())
	}
	return string(data)
}

func AnalyzeFile(filePath string, bundles []Bundle) ([]Violation, error) {
	absPath := filePath
	if !filepath.IsAbs(absPath) {
		absPath = filepath.Join(rootDir, filePath)
	}
	
	if isGitIgnored(absPath) {
		return []Violation{}, nil
	}
	
	scope := ParseScope(filePath)
	violations, err := CheckPolicies(scope, bundles, nil)
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
	var bundles []Bundle
	var projectsLoaded bool
	getProjectsLazy := func() []Bundle {
		if !projectsLoaded {
			bundles = GetProjects()
			projectsLoaded = true
		}
		return bundles
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
		info := v.Kind.Info()
		report.Summary.ByPriority[string(info.Priority)]++
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
	var allViolations []Violation
	// For file/section/definition scopes, skip bundle loading for speed
	if scope.Kind == ScopeFile || scope.Kind == ScopeSection || scope.Kind == ScopeDefinition {
		violations, err := AnalyzeFile(scope.FilePath, nil)
		if err != nil {
			output.Error(fmt.Sprintf("Error analyzing file: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		allViolations = append(allViolations, violations...)
	} else {
		bundles := GetProjects()
		files, _ := ScopeToFiles(scope, bundles)
		for _, file := range files {
			violations, err := AnalyzeFile(file, nil)
			if err != nil {
				continue
			}
			allViolations = append(allViolations, violations...)
		}
	}
	var fixable []Violation
	for _, v := range allViolations {
		info := v.Kind.Info()
		if info.Autofixable && v.Autofix != nil {
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
					if edit.Start < 0 || edit.End < 0 || edit.Start > len(content) || edit.End > len(content) || edit.Start > edit.End {
						continue
					}
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
	allPolicies := GetPolicies()
	output.Info("\n📜 Registered policies:\n")
	for _, p := range allPolicies {
		output.Plain(fmt.Sprintf("   %s", p.ID))
		output.Plain(fmt.Sprintf("      %s: %s", p.Name, p.Description))
		output.Plain(fmt.Sprintf("      Priority: %s", p.Priority))
		output.Plain("")
	}
	return ToolResult{Output: *output, Data: allPolicies}
}

func ToolPolicyCheck(policyID, scopeRaw string) ToolResult {
	output := NewOutput()
	if scopeRaw == "" {
		scopeRaw = "@semio"
	}
	scope := ParseScope(scopeRaw)
	bundles := GetProjects()
	violations, err := CheckPolicies(scope, bundles, []string{policyID})
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
	foundPolicy, ok := FindPolicy(policyID)
	if !ok {
		output.Error(fmt.Sprintf("Policy '%s' not found", policyID))
		return ToolResult{Output: *output, Error: fmt.Sprintf("Policy '%s' not found", policyID)}
	}
	output.Info(fmt.Sprintf("\n📋 Violation kinds for policy '%s':", policyID))
	for _, kind := range foundPolicy.Kinds {
		output.Plain(fmt.Sprintf("   - %s", kind))
	}
	return ToolResult{Output: *output, Data: foundPolicy.Kinds}
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
	output.Info(fmt.Sprintf("   Folder: %s", ticket.FolderPath))
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
		if t.Frontmatter.Status == graph.TicketStatusClosed {
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
	if err := ProgressIteration(ticket, prompt, model); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	lastIter := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
	fileCount := 0
	for _, b := range lastIter.Bundles {
		fileCount += len(b.Files)
	}
	output.Success(fmt.Sprintf("\n✅ Progress recorded on ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Files changed: %d", fileCount))
	output.Info(fmt.Sprintf("   Commit: %s", lastIter.Commit))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketProgress(year, month, day int, slug, prompt, model string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ProgressIteration(ticket, prompt, model); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	lastIter := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
	fileCount := 0
	for _, b := range lastIter.Bundles {
		fileCount += len(b.Files)
	}
	output.Success(fmt.Sprintf("\n✅ Progress recorded on ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Files changed: %d", fileCount))
	output.Info(fmt.Sprintf("   Commit: %s", lastIter.Commit))
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
		ticketCount := len(c.Contributions.Tickets)
		if ticketCount > 0 {
			output.Plain(fmt.Sprintf("      Tickets: %d", ticketCount))
		}
		projectCount := len(c.Contributions.Bundles)
		if projectCount > 0 {
			output.Plain(fmt.Sprintf("      Bundles: %d", projectCount))
		}
		fileCount := len(c.Contributions.Files)
		if fileCount > 0 {
			output.Plain(fmt.Sprintf("      Files: %d", fileCount))
		}
		commitCount := len(c.Contributions.Commits)
		if commitCount > 0 {
			output.Plain(fmt.Sprintf("      Commits: %d", commitCount))
		}
		if c.Contributions.Lines != nil {
			output.Plain(fmt.Sprintf("      Lines: +%d -%d", c.Contributions.Lines.Added, c.Contributions.Lines.Removed))
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
	bundles := GetProjects()
	output.Info(fmt.Sprintf("\n📦 Found %d bundles:\n", len(bundles)))
	for _, p := range bundles {
		output.Plain(fmt.Sprintf("   %s", p.Name))
		output.Plain(fmt.Sprintf("      Root: %s", p.Root))
		if len(p.Tags) > 0 {
			output.Plain(fmt.Sprintf("      Tags: %s", strings.Join(p.Tags, ", ")))
		}
	}
	return ToolResult{Output: *output, Data: bundles}
}

func ToolProjectTree() ToolResult {
	output := NewOutput()
	bundles := GetProjects()
	output.Info("\n📦 Bundle tree:\n")
	for _, p := range bundles {
		output.Plain(fmt.Sprintf("   └── %s (%s)", p.Name, p.Root))
	}
	return ToolResult{Output: *output, Data: bundles}
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
	language := GetLanguage(path)
	content := generateFileHeader(path, language)
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Created file: %s", path))
	return ToolResult{Output: *output}
}

func generateFileHeader(path string, language LanguagePlugin) string {
	if language == nil || !language.SupportsHeaders() {
		return ""
	}
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
	return language.FormatHeader(path, year, gitAuthor, formatLicenseLines(license, language.CommentPrefix()))
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
	bundles := GetProjects()
	files, err := ScopeToFiles(scope, bundles)
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
  language := GetLanguage(filePath)
  if language != nil && language.Name() == "json" {
   	pathParts := NormalizeSectionPath(sectionPath)
    if len(pathParts) == 0 {
      output.Error("Error: Section path required")
      return ToolResult{Output: *output, Error: "section path required"}
    }
    sectionName = pathParts[len(pathParts)-1]
    parentPath := strings.Join(pathParts[:len(pathParts)-1], "/")
    _, locations, err := ParseJSONSectionsDetailed(content)
    if err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    targetPath := strings.Join(pathParts, "/")
    if _, exists := locations[targetPath]; exists {
      output.Error(fmt.Sprintf("Error: Section already exists: %s", targetPath))
      return ToolResult{Output: *output, Error: "section exists"}
    }
    objectStart, objectEnd, ok := jsonFindObjectRange(content, locations, parentPath)
    if !ok {
      output.Error("Error: Parent section is not a JSON object")
      return ToolResult{Output: *output, Error: "parent not object"}
    }
    entry := fmt.Sprintf("%s: {}", strconv.Quote(sectionName))
    updated, inserted := jsonInsertEntry(content, objectStart, objectEnd, entry)
    if !inserted {
      output.Error("Error: Failed to insert section")
      return ToolResult{Output: *output, Error: "insert failed"}
    }
    if err := WriteTextFile(absPath, updated); err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    output.Success(fmt.Sprintf("\n🏷️ Created section \"%s\" in %s", sectionName, filePath))
    return ToolResult{Output: *output}
  }
  if language == nil || !language.SupportsSections() {
    output.Error("Error: Unsupported file type")
    return ToolResult{Output: *output, Error: "unsupported file type"}
  }
	newSection := language.FormatSectionBoth(sectionName)
	if newSection == "" {
		output.Error("Error: Cannot create section for this file type")
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
  language := GetLanguage(filePath)
  if language != nil && language.Name() == "json" {
    oldParts = NormalizeSectionPath(oldPath)
    newParts = NormalizeSectionPath(newPath)
    if len(oldParts) == 0 || len(newParts) == 0 {
      output.Error("Error: Section path required")
      return ToolResult{Output: *output, Error: "section path required"}
    }
    oldPathNormalized := strings.Join(oldParts, "/")
    newPathNormalized := strings.Join(newParts, "/")
    _, locations, err := ParseJSONSectionsDetailed(content)
    if err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    source, ok := locations[oldPathNormalized]
    if !ok {
      output.Error(fmt.Sprintf("Error: Section not found: %s", oldPathNormalized))
      return ToolResult{Output: *output, Error: "section not found"}
    }
    entry, start, end := jsonExtractEntry(content, source.KeyStart, source.ValueEnd)
    updated := content[:start] + content[end:]
    _, updatedLocations, err := ParseJSONSectionsDetailed(updated)
    if err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    newName = newParts[len(newParts)-1]
    entry = jsonRenameEntryKey(entry, newName)
    parentPath := strings.Join(newParts[:len(newParts)-1], "/")
    objectStart, objectEnd, ok := jsonFindObjectRange(updated, updatedLocations, parentPath)
    if !ok {
      output.Error("Error: Target section is not a JSON object")
      return ToolResult{Output: *output, Error: "target not object"}
    }
    entry = jsonReindentEntry(entry, "")
    finalContent, inserted := jsonInsertEntry(updated, objectStart, objectEnd, entry)
    if !inserted {
      output.Error("Error: Failed to move section")
      return ToolResult{Output: *output, Error: "move failed"}
    }
    if err := WriteTextFile(absPath, finalContent); err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    output.Success(fmt.Sprintf("\n🏷️ Renamed section \"%s\" to \"%s\" in %s", oldPathNormalized, newPathNormalized, filePath))
    return ToolResult{Output: *output}
  }
  if language != nil && language.SupportsSections() {
    oldStart := language.FormatSectionStart(oldName)
    newStart := language.FormatSectionStart(newName)
    if oldStart != "" && newStart != "" {
      content = strings.ReplaceAll(content, oldStart, newStart)
    }
    oldEnd := language.FormatSectionEnd(oldName)
    newEnd := language.FormatSectionEnd(newName)
    if oldEnd != "" && newEnd != "" {
      content = strings.ReplaceAll(content, oldEnd, newEnd)
    }
		if language.Name() == "markdown" {
			content = strings.ReplaceAll(content, "# "+oldName, "# "+newName)
		}
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
  language := GetLanguage(filePath)
  if language != nil && language.Name() == "json" {
    pathParts := NormalizeSectionPath(sectionPath)
    if len(pathParts) == 0 {
      output.Error("Error: Section path required")
      return ToolResult{Output: *output, Error: "section path required"}
    }
    _, locations, err := ParseJSONSectionsDetailed(content)
    if err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    location, ok := locations[strings.Join(pathParts, "/")]
    if !ok {
      output.Error(fmt.Sprintf("Error: Section not found: %s", strings.Join(pathParts, "/")))
      return ToolResult{Output: *output, Error: "section not found"}
    }
    _, start, end := jsonExtractEntry(content, location.KeyStart, location.ValueEnd)
    updated := content[:start] + content[end:]
    if err := WriteTextFile(absPath, updated); err != nil {
      output.Error(fmt.Sprintf("Error: %v", err))
      return ToolResult{Output: *output, Error: err.Error()}
    }
    output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", strings.Join(pathParts, "/"), filePath))
    return ToolResult{Output: *output}
  }
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

func ToolUpdateMetabolism() ToolResult {
	output := NewOutput()
	output.Info("\n🔄 Running update-metabolism via npx tsx...")
	stdout, stderr, exitCode := ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
	if exitCode != 0 {
		output.Error(fmt.Sprintf("Error: %s%s", stdout, stderr))
		return ToolResult{Output: *output, Error: "update-metabolism failed"}
	}
	output.Success(stdout)
	return ToolResult{Output: *output}
}

// #endregion Commands
