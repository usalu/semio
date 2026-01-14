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
	"database/sql"
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
	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/language/ast"
	"github.com/graphql-go/graphql/language/parser"
	_ "modernc.org/sqlite"
)

// #region GraphQL Types

type Node interface {
	IsNode()
	GetID() string
}

type DefinitionKind string

const (
	DefinitionKindFunction  DefinitionKind = "function"
	DefinitionKindClass     DefinitionKind = "class"
	DefinitionKindVariable  DefinitionKind = "variable"
	DefinitionKindInterface DefinitionKind = "interface"
	DefinitionKindType      DefinitionKind = "type"
	DefinitionKindEnum      DefinitionKind = "enum"
	DefinitionKindMethod    DefinitionKind = "method"
	DefinitionKindProperty  DefinitionKind = "property"
)

func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindFunction, DefinitionKindClass, DefinitionKindVariable,
		DefinitionKindInterface, DefinitionKindType, DefinitionKindEnum,
		DefinitionKindMethod, DefinitionKindProperty:
		return true
	}
	return false
}

func (e DefinitionKind) String() string {
	return string(e)
}

type TicketStatus string

const (
	TicketStatusOpen   TicketStatus = "open"
	TicketStatusClosed TicketStatus = "closed"
)

func (e TicketStatus) IsValid() bool {
	switch e {
	case TicketStatusOpen, TicketStatusClosed:
		return true
	}
	return false
}

func (e TicketStatus) String() string {
	return string(e)
}

type ViolationPriority string

const (
	ViolationPriorityHigh   ViolationPriority = "high"
	ViolationPriorityMedium ViolationPriority = "medium"
	ViolationPriorityLow    ViolationPriority = "low"
)

func (e ViolationPriority) IsValid() bool {
	switch e {
	case ViolationPriorityHigh, ViolationPriorityMedium, ViolationPriorityLow:
		return true
	}
	return false
}

func (e ViolationPriority) String() string {
	return string(e)
}

var AllowedLLMs = []string{
	"claude-opus-4-5",
	"claude-opus-4",
	"claude-sonnet-4-5",
	"claude-sonnet-4",
	"claude-haiku-4-5",
	"gemini-3-pro",
	"gemini-3-flash",
	"gpt-5-2",
	"gpt-5-mini",
}

type Range struct {
	Start int `json:"start"`
	End   int `json:"end"`
}

type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

type TextEdit struct {
	Start   int    `json:"start"`
	End     int    `json:"end"`
	NewText string `json:"newText"`
}

type DiffLines struct {
	Added   []int
	Removed []int
}

type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}



type ContributorIcons struct {
	Avatar      *string `json:"avatar,omitempty"`
	AvatarRound *string `json:"avatarRound,omitempty"`
	Github      *string `json:"github,omitempty"`
}

type ContributorLink struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

type TicketDate struct {
	Created  time.Time  `json:"created"`
	Finished *time.Time `json:"finished,omitempty"`
}

type TicketSectionMetrics struct {
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

type TicketFileMetricsEntry struct {
	Path     string                          `json:"path"`
	Lines    *LineMetrics                    `json:"lines,omitempty"`
	Sections map[string]TicketSectionMetrics `json:"sections,omitempty"`
}





type Autofix struct {
	Description string       `json:"description"`
	Edits       []FileEdit `json:"edits"`
}

type FileEdit struct {
	Path  string     `json:"path"`
	Edits []TextEdit `json:"edits"`
}

type AnalyzeMetrics struct {
	Total       int             `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int             `json:"autofixable"`
}

type PriorityCount struct {
	High   int `json:"high"`
	Medium int `json:"medium"`
	Low    int `json:"low"`
}

type Repo struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Path string `json:"path"`
}

func (r *Repo) IsNode()       {}
func (r *Repo) GetID() string { return "@semio" }

type Bundle struct {
	Name        string   `json:"name"`
	Root        string   `json:"root"`
	SourceRoot  string   `json:"sourceRoot,omitempty"`
	ProjectType string   `json:"projectType,omitempty"`
	Tags        []string `json:"tags,omitempty"`
}

func (b *Bundle) IsNode()       {}
func (b *Bundle) GetID() string { return "@semio/" + b.Name }

type Folder struct {
	ID       string  `json:"id"`
	Path     string  `json:"path"`
	URI      string  `json:"uri"`
	Name     string  `json:"name"`
	ParentID *string `json:"parentId,omitempty"`
	BundleID *string `json:"bundleId,omitempty"`
}

func (f *Folder) IsNode()       {}
func (f *Folder) GetID() string { return f.ID }

type File struct {
	ID        string  `json:"id"`
	Path      string  `json:"path"`
	URI       string  `json:"uri"`
	Name      string  `json:"name"`
	Extension string  `json:"extension"`
	FolderID  *string `json:"folderId,omitempty"`
	BundleID  *string `json:"bundleId,omitempty"`
}

func (f *File) IsNode()       {}
func (f *File) GetID() string { return f.ID }

type Section struct {
	Name       string    `json:"name"`
	Path       string    `json:"path,omitempty"`
	FilePath   string    `json:"filePath,omitempty"`
	StartLine  int       `json:"startLine"`
	EndLine    int       `json:"endLine"`
	StartIndex int       `json:"startIndex"`
	EndIndex   int       `json:"endIndex"`
	Children   []Section `json:"children,omitempty"`
}

func (s *Section) IsNode()       {}
func (s *Section) GetID() string {
	if s.FilePath != "" && s.Path != "" {
		return s.FilePath + "#" + s.Path
	}
	return "section:" + s.Name
}

type Definition struct {
	Name       string         `json:"name"`
	Kind       DefinitionKind `json:"kind"`
	FilePath   string         `json:"filePath,omitempty"`
	SectionPath string        `json:"sectionPath,omitempty"`
	StartLine  int            `json:"startLine"`
	EndLine    int            `json:"endLine"`
	StartIndex int            `json:"startIndex"`
	EndIndex   int            `json:"endIndex"`
}

func (d *Definition) IsNode()       {}
func (d *Definition) GetID() string {
	if d.FilePath != "" {
		if d.SectionPath != "" {
			return d.FilePath + "#" + d.SectionPath + "§" + d.Name
		}
		return d.FilePath + "§" + d.Name
	}
	return "definition:" + d.Name
}

type Contributor struct {
	Github        string                      `yaml:"github" json:"github"`
	Name          string                      `yaml:"name,omitempty" json:"name,omitempty"`
	Emails        []string                    `yaml:"emails,omitempty" json:"emails,omitempty"`
	Links         map[string]string           `yaml:"links,omitempty" json:"links,omitempty"`
	Contributions ContributorContributionsStorage `yaml:"contributions,omitempty" json:"contributions,omitempty"`
}

func (c *Contributor) IsNode()       {}
func (c *Contributor) GetID() string { return "@semio/contributors/" + c.Github }

type Commit struct {
	ID       string    `json:"id"`
	SHA      string    `json:"sha"`
	Title    string    `json:"title"`
	AuthorID *string   `json:"authorId,omitempty"`
	Date     time.Time `json:"date"`
}

func (c *Commit) IsNode()       {}
func (c *Commit) GetID() string { return "@semio/commits/" + c.SHA }

type Ticket struct {
	Year         int               `json:"year"`
	Month        int               `json:"month"`
	Day         int         `json:"day"`
	Slug        string      `json:"slug"`
	Data        *TicketData `json:"data,omitempty"`
	FolderPath  string      `json:"folderPath"`
	JsonPath    string      `json:"jsonPath,omitempty"`
	PlanPath    string      `json:"planPath,omitempty"`
	LogPath     string      `json:"logPath,omitempty"`
	SummaryPath string      `json:"summaryPath,omitempty"`
}

func (t *Ticket) IsNode()       {}
func (t *Ticket) GetID() string { return fmt.Sprintf("@semio/tickets/%d/%02d/%02d/%s", t.Year, t.Month, t.Day, t.Slug) }

func (t *Ticket) GetTitle() string {
	if t.Data != nil {
		return t.Data.Title
	}
	return t.Slug
}

func (t *Ticket) GetPrompt() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[0].Prompt
	}
	return ""
}

func (t *Ticket) GetLLM() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[len(t.Data.Iterations)-1].LLM
	}
	return ""
}

func (t *Ticket) GetStatus() TicketStatus {
	if t.Data != nil {
		return t.Data.Status
	}
	return ""
}

func (t *Ticket) GetAuthor() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[0].Author
	}
	return ""
}

func (t *Ticket) GetCommit() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[0].Commit
	}
	return ""
}

func (t *Ticket) GetSummary() string {
	if t.Data != nil {
		return t.Data.Summary
	}
	return ""
}

func (t *Ticket) GetDateCreated() time.Time {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[0].Date
	}
	return time.Time{}
}

func (t *Ticket) GetDateFinished() *time.Time {
	if t.Data != nil {
		return t.Data.Dates.Closed
	}
	return nil
}

type TicketFilesResult struct {
	Updated []TicketFile `json:"updated,omitempty"`
	Created []TicketFile `json:"created,omitempty"`
	Removed []TicketFile `json:"removed,omitempty"`
}

func (t *Ticket) GetFiles() *TicketFilesResult {
	filesMap := make(map[string]TicketFile)
	if t.Data != nil {
		for _, iter := range t.Data.Iterations {
			for _, f := range iter.Files {
				filesMap[f.Path] = f
			}
		}
	}
	res := &TicketFilesResult{}
	for _, f := range filesMap {
		switch f.Status {
		case "created":
			res.Created = append(res.Created, f)
		case "removed":
			res.Removed = append(res.Removed, f)
		default:
			res.Updated = append(res.Updated, f)
		}
	}
	return res
}

type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

type TicketFileContrib struct {
	FileID   string                    `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}

type Policy struct {
	ID             string              `json:"id"`
	Name           string              `json:"name"`
	Description    *string             `json:"description,omitempty"`
	Scopes         []string            `json:"scopes"`
	ViolationKinds []*ViolationKindMeta `json:"violationKinds"`
}

func (p *Policy) IsNode()       {}
func (p *Policy) GetID() string { return "@semio/policies/" + p.Name }

type ViolationKindMeta struct {
	Kind        ViolationKind     `json:"kind"`
	PolicyID    string            `json:"policyId"`
	Priority    ViolationPriority `json:"priority"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
	Autofixable bool              `json:"autofixable"`
}

func (v *ViolationKindMeta) IsNode()       {}
func (v *ViolationKindMeta) GetID() string { return "@semio/policies/" + v.PolicyID + "/violations/" + string(v.Kind) }

type AnalyzeResult struct {
	Violations []*Violation    `json:"violations"`
	Metrics    *AnalyzeMetrics `json:"metrics"`
}

type FixResult struct {
	Fixed      int          `json:"fixed"`
	Remaining  int          `json:"remaining"`
	Violations []*Violation `json:"violations"`
}

type ContributorContributions struct {
	Bundles     []ContributionBundle     `json:"bundles"`
	Folders     []ContributionFolder     `json:"folders"`
	Files       []ContributionFile       `json:"files"`
	Sections    []ContributionSection    `json:"sections"`
	Definitions []ContributionDefinition `json:"definitions"`
}

type ContributionBundle struct {
	BundleID string        `json:"bundleId"`
	Metrics  *CountMetrics `json:"metrics"`
}

type ContributionFolder struct {
	FolderID string        `json:"folderId"`
	Metrics  *CountMetrics `json:"metrics"`
}

type ContributionFile struct {
	FileID  string       `json:"fileId"`
	Metrics *LineMetrics `json:"metrics"`
}

type ContributionSection struct {
	SectionID string       `json:"sectionId"`
	Metrics   *LineMetrics `json:"metrics"`
}

type ContributionDefinition struct {
	DefinitionID string       `json:"definitionId"`
	Metrics      *LineMetrics `json:"metrics"`
}

// #region GraphQL Input Types

type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

type TicketOpenInput struct {
	Title  string `json:"title"`
	Prompt string `json:"prompt"`
	LLM    string `json:"llm"`
}

type TicketCloseInput struct {
	Year    int      `json:"year"`
	Month   int      `json:"month"`
	Day     int      `json:"day"`
	Slug    string   `json:"slug"`
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
}

type TicketReopenInput struct {
	Year   int    `json:"year"`
	Month  int    `json:"month"`
	Day    int    `json:"day"`
	Slug   string `json:"slug"`
	Prompt string `json:"prompt"`
	LLM    string `json:"llm"`
}

type ContributorAddInput struct {
	Github string   `json:"github"`
	Name   *string  `json:"name,omitempty"`
	Emails []string `json:"emails,omitempty"`
}

// #endregion GraphQL Input Types

// #endregion GraphQL Types

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

func (v *Violation) IsNode()       {}
func (v *Violation) GetID() string { return v.ID }

func (v *Violation) Priority() ViolationPriority {
	return v.Kind.Info().Priority
}

func (v *Violation) Autofixable() bool {
	return v.Kind.Info().Autofixable
}


type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

type TicketBundles map[string]TicketBundleMetrics

// #region Languages

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
	ParseSections(content string) []Section
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

func (l *BaseLanguage) ParseSections(content string) []Section {
	if l.sectionStart == nil {
		return nil
	}
	lines := strings.Split(content, "\n")
	var stack []*Section
	var roots []Section
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := l.sectionStart.FindStringSubmatch(line); match != nil {
			name := strings.TrimSpace(match[1])
			section := &Section{
				Name:       name,
				StartLine:  lineNum,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []Section{},
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


// #region TypeScript

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
	var headerSection *Section
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
								Edits: map[string][]TextEdit{
									file: {{Start: scanState.BlockCommentStartIndex, End: lineStart + j + 2, NewText: ""}},
								},
							}))
					} else {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentBlock,
							file, scanState.BlockCommentStartLine, "", &Fix{
								Description: "Remove block comment",
								Edits: map[string][]TextEdit{
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
							Edits: map[string][]TextEdit{
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

// #endregion TypeScript

// #region Go

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
			definitionRegexp:   regexp.MustCompile(`^(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)`),
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

// #endregion Python

// #region C#

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

// #endregion C#

// #region JSON

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

func (l *JSONLanguage) ParseSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

// #endregion JSON

// #region Markdown

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

func (l *MarkdownLanguage) ParseSections(content string) []Section {
	return ParseMarkdownSectionsInternal(content)
}

// #endregion Markdown

// #region Rust

type RustLanguage struct {
	BaseLanguage
}

func NewRustLanguage() *RustLanguage {
	return &RustLanguage{
		BaseLanguage: BaseLanguage{
			name:               "rust",
			extensions:         []string{".rs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:pub\s+)?(?:fn|struct|enum|trait|impl|type|const|static|mod)\s+([A-Za-z_][A-Za-z0-9_]*)`),
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

func (l *RustLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	modRegexp := regexp.MustCompile(`^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;`)
	for i, line := range lines {
		if match := modRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("mod-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}

// #endregion Rust

// #region Ruby

type RubyLanguage struct {
	BaseLanguage
}

func NewRubyLanguage() *RubyLanguage {
	return &RubyLanguage{
		BaseLanguage: BaseLanguage{
			name:               "ruby",
			extensions:         []string{".rb", ".rake", ".gemspec"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:def|class|module)\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *RubyLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name  string
		line  int
		depth int
	}
	var defStack []defStart
	var defRanges []DefinitionRange
	endRegexp := regexp.MustCompile(`^\s*end\s*$`)
	blockStartRegexp := regexp.MustCompile(`(?:^|\s)(?:if|unless|case|while|until|for|begin|do)\b`)
	depth := 0
	for i, line := range lines {
		lineNum := i + 1
		trimmed := strings.TrimSpace(line)
		if matches := l.definitionRegexp.FindAllStringSubmatch(line, -1); matches != nil {
			for _, match := range matches {
				if len(match) > 1 && match[1] != "" {
					defStack = append(defStack, defStart{name: match[1], line: lineNum, depth: depth})
					depth++
				}
			}
		} else if blockStartRegexp.MatchString(line) && !strings.Contains(line, " do ") {
			depth++
		}
		if endRegexp.MatchString(trimmed) {
			if depth > 0 {
				depth--
			}
			for len(defStack) > 0 && defStack[len(defStack)-1].depth == depth {
				def := defStack[len(defStack)-1]
				defStack = defStack[:len(defStack)-1]
				defRanges = append(defRanges, DefinitionRange{
					Name:    def.name,
					Start:   def.line,
					End:     lineNum,
					Excerpt: def.name,
				})
			}
		}
	}
	for _, def := range defStack {
		defRanges = append(defRanges, DefinitionRange{
			Name:    def.name,
			Start:   def.line,
			End:     len(lines),
			Excerpt: def.name,
		})
	}
	sort.Slice(defRanges, func(i, j int) bool {
		return defRanges[i].Start < defRanges[j].Start
	})
	return defRanges
}

func (l *RubyLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	moduleRegexp := regexp.MustCompile(`^\s*module\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`)
	for i, line := range lines {
		if match := moduleRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("module-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}

// #endregion Ruby

// #region TOML

type TomlLanguage struct {
	BaseLanguage
}

func NewTomlLanguage() *TomlLanguage {
	return &TomlLanguage{
		BaseLanguage: BaseLanguage{
			name:              "toml",
			extensions:        []string{".toml"},
			sectionStart:      regexp.MustCompile(`^\s*\[{1,2}([^\]]+)\]{1,2}\s*$`),
			commentPrefix:     "#",
			usesIndentScoping: false,
		},
	}
}

func (l *TomlLanguage) SupportsSections() bool    { return true }
func (l *TomlLanguage) SupportsDefinitions() bool { return false }
func (l *TomlLanguage) SupportsComments() bool    { return true }
func (l *TomlLanguage) SupportsHeaders() bool     { return false }

func (l *TomlLanguage) ParseSections(content string) []Section {
	lines := strings.Split(content, "\n")
	var sections []Section
	var currentSection *Section
	for i, line := range lines {
		lineNum := i + 1
		if match := l.sectionStart.FindStringSubmatch(line); match != nil {
			if currentSection != nil {
				currentSection.EndLine = lineNum - 1
				sections = append(sections, *currentSection)
			}
			currentSection = &Section{
				Name:      match[1],
				StartLine: lineNum,
				EndLine:   len(lines),
			}
		}
	}
	if currentSection != nil {
		sections = append(sections, *currentSection)
	}
	return sections
}

// #endregion TOML

// #region YAML

type YamlLanguage struct {
	BaseLanguage
}

func NewYamlLanguage() *YamlLanguage {
	return &YamlLanguage{
		BaseLanguage: BaseLanguage{
			name:              "yaml",
			extensions:        []string{".yaml", ".yml"},
			commentPrefix:     "#",
			usesIndentScoping: true,
		},
	}
}

func (l *YamlLanguage) SupportsSections() bool    { return false }
func (l *YamlLanguage) SupportsDefinitions() bool { return false }
func (l *YamlLanguage) SupportsComments() bool    { return true }
func (l *YamlLanguage) SupportsHeaders() bool     { return false }

// #endregion YAML

// #region SQL

type SqlLanguage struct {
	BaseLanguage
}

func NewSqlLanguage() *SqlLanguage {
	return &SqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "sql",
			extensions:         []string{".sql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*--\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?i)(?:CREATE\s+(?:OR\s+REPLACE\s+)?(?:TABLE|VIEW|PROCEDURE|FUNCTION|TRIGGER|INDEX|TYPE|SCHEMA|DATABASE|SEQUENCE|MATERIALIZED\s+VIEW))\s+(?:IF\s+NOT\s+EXISTS\s+)?([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "--",
			sectionStartFmt:    "-- #region %s",
			sectionEndFmt:      "-- #endregion %s",
			sectionBothFmt:     "\n-- #region %s\n\n-- #endregion %s\n",
			headerFmt:          "-- #region Header\n\n-- %s\n\n-- %s %s\n\n%s\n\n-- #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*--\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion SQL

// #region GraphQL

type GraphqlLanguage struct {
	BaseLanguage
}

func NewGraphqlLanguage() *GraphqlLanguage {
	return &GraphqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "graphql",
			extensions:         []string{".graphql", ".gql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:type|interface|enum|input|union|scalar|query|mutation|subscription|fragment|extend\s+type|extend\s+interface|extend\s+enum|extend\s+union|extend\s+input)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# #region %s",
			sectionEndFmt:      "# #endregion %s",
			sectionBothFmt:     "\n# #region %s\n\n# #endregion %s\n",
			headerFmt:          "# #region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion GraphQL

var languageRegistry = []LanguagePlugin{
	NewTypeScriptLanguage(),
	NewGoLanguage(),
	NewPythonLanguage(),
	NewCSharpLanguage(),
	NewJSONLanguage(),
	NewMarkdownLanguage(),
	NewRustLanguage(),
	NewRubyLanguage(),
	NewTomlLanguage(),
	NewYamlLanguage(),
	NewSqlLanguage(),
	NewGraphqlLanguage(),
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

// #endregion Languages

type TicketIteration struct {
	Prompt string       `json:"prompt"`
	LLM    string       `json:"llm"`
	Author string       `json:"author"`
	Date   time.Time    `json:"date"`
	Commit string       `json:"commit"`
	Files  []TicketFile `json:"files,omitempty"`
}

type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

type TicketFile struct {
	Path     string          `json:"path"`
	Status   string          `json:"status"`
	Lines    *LineMetrics    `json:"lines,omitempty"`
	Sections []TicketSection `json:"sections,omitempty"`
}

type TicketData struct {
	Title      string            `json:"title"`
	Iterations []TicketIteration `json:"iterations"`
	Status     TicketStatus      `json:"status"`
	Dates      TicketDates       `json:"dates"`
	Summary    string            `json:"summary,omitempty"`
	Files      []TicketFile      `json:"files,omitempty"`
}

type TicketDates struct {
	Closed *time.Time `json:"closed,omitempty"`
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

var violationKindInfoTable = map[ViolationKind]ViolationKindMeta{
	ViolationCodeHeaderMissingRegion: {
		Kind:        ViolationCodeHeaderMissingRegion,
		Priority:    ViolationPriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingFilename: {
		Kind:        ViolationCodeHeaderMissingFilename,
		Priority:    ViolationPriorityLow,
		Reason:      "Filename must be documented in header",
		Solution:    "Add filename comment in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingContributors: {
		Kind:        ViolationCodeHeaderMissingContributors,
		Priority:    ViolationPriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingLicense: {
		Kind:        ViolationCodeHeaderMissingLicense,
		Priority:    ViolationPriorityLow,
		Reason:      "SPDX license identifier is required",
		Solution:    "Add SPDX license header comment",
		Autofixable: false,
	},
	ViolationCodeHeaderWrongLicense: {
		Kind:        ViolationCodeHeaderWrongLicense,
		Priority:    ViolationPriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Update license to AGPL-3.0-or-later",
		Autofixable: false,
	},
	ViolationCodeSectionEmpty: {
		Kind:        ViolationCodeSectionEmpty,
		Priority:    ViolationPriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	ViolationCodeSectionOrphanDefinition: {
		Kind:        ViolationCodeSectionOrphanDefinition,
		Priority:    ViolationPriorityLow,
		Reason:      "All code must be inside named sections",
		Solution:    "Move code into an existing section or add a new section",
		Autofixable: false,
	},
	ViolationCodeSectionMissingStartName: {
		Kind:        ViolationCodeSectionMissingStartName,
		Priority:    ViolationPriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	ViolationCodeSectionMissingEndName: {
		Kind:        ViolationCodeSectionMissingEndName,
		Priority:    ViolationPriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	ViolationCodeSectionNameMismatch: {
		Kind:        ViolationCodeSectionNameMismatch,
		Priority:    ViolationPriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	ViolationCodeCommentInline: {
		Kind:        ViolationCodeCommentInline,
		Priority:    ViolationPriorityLow,
		Reason:      "Inline comments are forbidden",
		Solution:    "Remove inline comment",
		Autofixable: true,
	},
	ViolationCodeCommentBlock: {
		Kind:        ViolationCodeCommentBlock,
		Priority:    ViolationPriorityLow,
		Reason:      "Block comments are forbidden",
		Solution:    "Remove block comment",
		Autofixable: true,
	},
	ViolationCodeCommentJSDoc: {
		Kind:        ViolationCodeCommentJSDoc,
		Priority:    ViolationPriorityLow,
		Reason:      "JSDoc comments are forbidden",
		Solution:    "Remove JSDoc comment",
		Autofixable: true,
	},
	ViolationDevDocsMissingFile: {
		Kind:        ViolationDevDocsMissingFile,
		Priority:    ViolationPriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsMissingFolder: {
		Kind:        ViolationDevDocsMissingFolder,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsWrongFilePath: {
		Kind:        ViolationDevDocsWrongFilePath,
		Priority:    ViolationPriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderPath: {
		Kind:        ViolationDevDocsWrongFolderPath,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileName: {
		Kind:        ViolationDevDocsWrongFileName,
		Priority:    ViolationPriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄 PATH)",
		Solution:    "Rename section to ## 📄 PATH",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderName: {
		Kind:        ViolationDevDocsWrongFolderName,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁 PATH/)",
		Solution:    "Rename section to ## 📁 PATH/",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileOrder: {
		Kind:        ViolationDevDocsWrongFileOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderOrder: {
		Kind:        ViolationDevDocsWrongFolderOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsMissingComponent: {
		Kind:        ViolationDevDocsMissingComponent,
		Priority:    ViolationPriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentName: {
		Kind:        ViolationDevDocsWrongComponentName,
		Priority:    ViolationPriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentOrder: {
		Kind:        ViolationDevDocsWrongComponentOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
	ViolationSketchpadImportThirdParty: {
		Kind:        ViolationSketchpadImportThirdParty,
		Priority:    ViolationPriorityHigh,
		Reason:      "Third party imports must only be in elements.tsx",
		Solution:    "Move third party import to elements.tsx and re-export from there",
		Autofixable: false,
	},
	ViolationSketchpadStateMultipleMachines: {
		Kind:        ViolationSketchpadStateMultipleMachines,
		Priority:    ViolationPriorityHigh,
		Reason:      "Only one state machine is allowed (createMachine can only be used once)",
		Solution:    "Consolidate state management into a single state machine",
		Autofixable: false,
	},
	ViolationSketchpadStateCreateActor: {
		Kind:        ViolationSketchpadStateCreateActor,
		Priority:    ViolationPriorityHigh,
		Reason:      "createActor is forbidden in sketchpad",
		Solution:    "Remove createActor usage and use the single state machine instead",
		Autofixable: false,
	},
	ViolationSketchpadStateYjsAppState: {
		Kind:        ViolationSketchpadStateYjsAppState,
		Priority:    ViolationPriorityHigh,
		Reason:      "Yjs should only be used for kit data synchronization, not app state",
		Solution:    "Move app state to the state machine and use Yjs only for kit data sync",
		Autofixable: false,
	},
	ViolationSketchpadStateForbiddenStore: {
		Kind:        ViolationSketchpadStateForbiddenStore,
		Priority:    ViolationPriorityHigh,
		Reason:      "Stores outside of State Management sections are forbidden",
		Solution:    "Move store to a State Management section or remove it",
		Autofixable: false,
	},
	ViolationSketchpadHooksNonTriadic: {
		Kind:        ViolationSketchpadHooksNonTriadic,
		Priority:    ViolationPriorityHigh,
		Reason:      "UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()",
		Solution:    "Refactor to use triadic hook pattern with useSELECTOR",
		Autofixable: false,
	},
}

func (k ViolationKind) Info() ViolationKindMeta {
	if info, ok := violationKindInfoTable[k]; ok {
		return info
	}
	return ViolationKindMeta{
		Kind:        k,
		Priority:    ViolationPriorityLow,
		Reason:      "Unknown violation",
		Solution:    "Fix the violation",
		Autofixable: false,
	}
}

type PolicyDef struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Scopes      []string          `json:"scopes"`
	Priority    ViolationPriority `json:"priority"`
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


type ContributorTicket struct {
	Year     int                `json:"year"`
	Month    int                `json:"month"`
	Day      int                `json:"day"`
	Slug     string             `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string             `json:"filePath,omitempty"`
}

type ContributorCommit struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

type ContributorContributionsStorage struct {
	Bundles    []string `json:"bundles,omitempty"`
	Folders     []string `json:"folders,omitempty"`
	Files       []string `json:"files,omitempty"`
	Regions     []string `json:"regions,omitempty"`
	Definitions []string `json:"definitions,omitempty"`
	Tickets     []ContributorTicket `json:"tickets,omitempty"`
	Commits     []ContributorCommit `json:"commits,omitempty"`
	Lines       *LineMetrics           `json:"lines,omitempty"`
}

// #region Codebase Types

type BundleMetricsInternal struct {
	Folders    int `json:"folders"`
	Files      int `json:"files"`
	Sections   int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FolderMetricsInternal struct {
	Files      int `json:"files"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FileMetricsInternal struct {
	Sections   int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines      int `json:"lines"`
}

type SectionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type DefinitionMetricsInternal struct {
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
	Priority    ViolationPriority `json:"priority"`
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
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseFolder struct {
	ID      string             `json:"id"`
	Path    string             `json:"path"`
	URI     string             `json:"uri"`
	Metrics *FolderMetricsInternal `json:"metrics,omitempty"`
}

type FileViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebaseFile struct {
	ID         string             `json:"id"`
	Path       string             `json:"path"`
	URI        string             `json:"uri"`
	Metrics    *FileMetricsInternal   `json:"metrics,omitempty"`
	Violations []FileViolationRef `json:"violations,omitempty"`
}

type CodebaseSection struct {
	ID      string              `json:"id"`
	Path    string              `json:"path"`
	URI     string              `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseDefinition struct {
	ID      string                 `json:"id"`
	Path    string                 `json:"path"`
	URI     string                 `json:"uri"`
	Metrics *DefinitionMetricsInternal `json:"metrics,omitempty"`
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
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorContributionsInternal struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

type ContributorMetricsInternal struct {
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
	Contributions *ContributorContributionsInternal `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInternal       `json:"metrics,omitempty"`
}

type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

type TicketBundleContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFolderContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFileContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketSectionContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
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
	Status      TicketStatus        `json:"status"`
	Bundles     []TicketBundleContribInfo     `json:"bundles,omitempty"`
	Folders     []TicketFolderContribInfo     `json:"folders,omitempty"`
	Files       []TicketFileContribInfo       `json:"files,omitempty"`
	Sections    []TicketSectionContribInfo    `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib `json:"definitions,omitempty"`
}

type PolicyViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
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

func GetGitAuthorGithub() string {
	contributorsDir := filepath.Join(GetRootDir(), "contributors")
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	name = strings.TrimSpace(name)
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	email = strings.TrimSpace(email)

	fallback := name
	if email != "" {
		fallback = fmt.Sprintf("%s <%s>", name, email)
	}

	entries, err := os.ReadDir(contributorsDir)
	if err != nil {
		return fallback
	}

	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		github := entry.Name()
		configPath := filepath.Join(contributorsDir, github, "config.json")
		if !FileExists(configPath) {
			continue
		}
		raw, err := ReadTextFile(configPath)
		if err != nil {
			continue
		}
		var config struct {
			Name   string   `json:"name"`
			Emails []string `json:"emails"`
		}
		if err := json.Unmarshal([]byte(raw), &config); err != nil {
			continue
		}
		if email != "" {
			for _, e := range config.Emails {
				if strings.EqualFold(e, email) {
					return github
				}
			}
		}
		if strings.EqualFold(config.Name, name) {
			return github
		}
	}
	return fallback
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

func ParseCodeSections(content string, languageName string) []Section {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

func ParseMarkdownSectionsInternal(content string) []Section {
	lines := strings.Split(content, "\n")
	var sections []Section
	type stackItem struct {
		level   int
		section *Section
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
			section := &Section{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []Section{},
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
	Section    *Section
}

type jsonContext struct {
	kind      byte
	section   *Section
	path      string
	expectKey bool
	location  *JsonSectionLocation
}

func ParseJSONSectionsDetailed(content string) ([]Section, map[string]*JsonSectionLocation, error) {
	var sections []Section
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
			section := Section{
				Name:       pendingKey,
				StartLine:  pendingKeyLine,
				EndLine:    -1,
				StartIndex: pendingKeyStart,
				EndIndex:   -1,
				Children:   []Section{},
			}
			var sectionRef *Section
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

func ParseJSONSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

func ParseSections(content string, filePath string) []Section {
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

func FindSection(sections []Section, name string) *Section {
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

var policies = []PolicyDef{
	{
		ID:          "code",
		Name:        "Code",
		Description: "Validates source file headers, sections, and comments",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go}"},
		Priority:    ViolationPriorityLow,
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
		Priority:    ViolationPriorityLow,
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
		Priority:    ViolationPriorityHigh,
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

func FindPolicy(id string) (PolicyDef, bool) {
	for _, p := range policies {
		if p.ID == id {
			return p, true
		}
	}
	return PolicyDef{}, false
}

func GetPolicies() []PolicyDef {
	return policies
}

type PolicyContext struct {
	Scope    Scope
	RootDir  string
	Bundles []Bundle
	fileCache     map[string]string
	sectionCache  map[string][]Section
	ignoreCache   map[string]map[int][]string // file -> line -> ignore patterns
}

func NewPolicyContext(scope Scope, bundles []Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Bundles:     bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]Section),
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

func (ctx *PolicyContext) Sections(filePath string) []Section {
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
		ID:      buildViolationID(scope, line, 0),
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
	var policiesToRun []PolicyDef
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
		var headerSection *Section
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
					Edits: map[string][]TextEdit{
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
		var checkSection func(s Section)
		checkSection = func(s Section) {
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
		var markCovered func(s Section)
		markCovered = func(s Section) {
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
	Policies   []PolicyDef
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
		ticketID := fmt.Sprintf("%04d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		if ticket.Data != nil && ticket.Data.Files != nil {
			for _, entry := range ticket.Data.Files {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
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
			Metrics: &BundleMetricsInternal{
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

func countSections(sections []Section) int {
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
			Metrics: &FolderMetricsInternal{
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

		var metrics *FileMetricsInternal
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
			metrics = &FileMetricsInternal{
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

func addSections(ctx *CodebaseContext, result *[]CodebaseSection, file, bundleName, content string, sections []Section, parentPath string) {
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
			Metrics: &SectionMetricsInternal{
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
				Metrics: &DefinitionMetricsInternal{
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
				avatar := ctx.FileURI(GetRelativePath(avatarPath))
				icons.Avatar = &avatar
			}
			if FileExists(avatarRoundPath) {
				avatarRound := ctx.FileURI(GetRelativePath(avatarRoundPath))
				icons.AvatarRound = &avatarRound
			}
			if githubLink, ok := c.Links["github"]; ok {
				github := githubLink + ".png"
				icons.Github = &github
			}
		}

		var contributions *ContributorContributionsInternal
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &ContributorContributionsInternal{}
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
			Metrics: &ContributorMetricsInternal{
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
		if ticket.Data != nil {
			ticketPath = fmt.Sprintf("tickets/%04d/%02d/%02d/%s/ticket.json", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		}

		bundleFiles := make(map[string]int)
		if ticket.Data != nil && ticket.Data.Files != nil {
			for _, entry := range ticket.Data.Files {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					bundleFiles[bundleName]++
				}
			}
		}
		var bundleContribs []TicketBundleContribInfo
		for bundleName, fileCount := range bundleFiles {
			bundleContribs = append(bundleContribs, TicketBundleContribInfo{
				ID: bundleName,
				Metrics: &CountMetrics{
					Added: fileCount,
				},
			})
		}

		model := ticket.GetLLM()

		var finishedStr string
		if f := ticket.GetDateFinished(); f != nil {
			finishedStr = f.Format(time.RFC3339)
		}

		result = append(result, CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &TicketDateInfo{
				Created:  ticket.GetDateCreated().Format(time.RFC3339),
				Finished: finishedStr,
			},
			Commit:   ticket.GetCommit(),
			Year:     fmt.Sprintf("%04d", ticket.Year),
			Month:    fmt.Sprintf("%02d", ticket.Month),
			Day:      fmt.Sprintf("%02d", ticket.Day),
			Slug:     ticket.Slug,
			Prompt:   ticket.GetPrompt(),
			Model:    model,
			Author:   ticket.GetAuthor(),
			Status:   ticket.GetStatus(),
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

func GetTicketJsonPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.json")
}

func GetTicketPlanPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "plan.md")
}

func GetTicketLogPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "log.md")
}

func GetTicketSummaryPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "summary.md")
}

func CreateTicket(title, prompt, llm, planPath string) (*Ticket, error) {
	now := time.Now()
	year, month, day := FormatDate(now)
	slug := Slugify(title)
	
	// Normalize LLM string
	llmSlug := strings.ToLower(llm)
	llmSlug = strings.ReplaceAll(llmSlug, " ", "-")
	llmSlug = strings.ReplaceAll(llmSlug, ".", "-")

	isAllowed := false
	for _, allowed := range AllowedLLMs {
		if allowed == llmSlug {
			isAllowed = true
			break
		}
	}

	if !isAllowed {
		return nil, fmt.Errorf("model '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMs, ", "))
	}

	ticketDir := GetTicketPath(year, month, day, slug)
	if err := EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	planFilePath := GetTicketPlanPath(year, month, day, slug)
	logFilePath := GetTicketLogPath(year, month, day, slug)
	summaryFilePath := GetTicketSummaryPath(year, month, day, slug)
	gitAuthor := GetGitAuthorGithub()
	gitCommit := GetGitCommit()
	ticketData := &TicketData{
		Title:  title,
		Status: TicketStatusOpen,
		Iterations: []TicketIteration{{
			Prompt: prompt,
			LLM:    llmSlug,
			Author: gitAuthor,
			Date:   now,
			Commit: gitCommit,
		}},
		Dates: TicketDates{},
	}
	ticket := &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        slug,
		Data:        ticketData,
		FolderPath:  ticketDir,
		JsonPath:    jsonPath,
		PlanPath:    planFilePath,
		LogPath:     logFilePath,
		SummaryPath: summaryFilePath,
	}
	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	if planPath != "" && FileExists(planPath) {
		planContent, err := ReadTextFile(planPath)
		if err != nil {
			return nil, fmt.Errorf("failed to read plan file: %w", err)
		}
		if err := WriteTextFile(planFilePath, planContent); err != nil {
			return nil, fmt.Errorf("failed to write plan file: %w", err)
		}
	} else {
		if err := WriteTextFile(planFilePath, ""); err != nil {
			return nil, fmt.Errorf("failed to write plan file: %w", err)
		}
	}
	if err := WriteTextFile(logFilePath, ""); err != nil {
		return nil, fmt.Errorf("failed to write log file: %w", err)
	}
	if err := WriteTextFile(summaryFilePath, ""); err != nil {
		return nil, fmt.Errorf("failed to write summary file: %w", err)
	}
	return ticket, nil
}

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	planPath := GetTicketPlanPath(year, month, day, slug)
	logPath := GetTicketLogPath(year, month, day, slug)
	summaryPath := GetTicketSummaryPath(year, month, day, slug)
	if !FileExists(jsonPath) {
		return nil, fmt.Errorf("ticket not found: %s", jsonPath)
	}
	raw, err := ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var data TicketData
	if err := json.Unmarshal([]byte(raw), &data); err != nil {
		return nil, err
	}
	return &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        slug,
		Data:        &data,
		FolderPath:  folderPath,
		JsonPath:    jsonPath,
		PlanPath:    planPath,
		LogPath:     logPath,
		SummaryPath: summaryPath,
	}, nil
}

func SaveTicket(ticket *Ticket) error {
	if ticket.Data == nil {
		return fmt.Errorf("ticket data is nil")
	}
	jsonBytes, err := json.MarshalIndent(ticket.Data, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(ticket.JsonPath, string(jsonBytes))
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
						ticketJsonPath := GetTicketJsonPath(yearInt, monthInt, dayInt, slug)
						ticketFilePath := GetTicketFilePath(yearInt, monthInt, dayInt, slug)
						if FileExists(ticketJsonPath) || FileExists(ticketFilePath) {
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

func ComputeTicketFiles(ticket *Ticket, files []string) ([]TicketFile, error) {
	if ticket.Data == nil {
		return nil, fmt.Errorf("ticket data is nil")
	}
	if len(ticket.Data.Iterations) == 0 {
		return nil, fmt.Errorf("no iterations found for ticket")
	}
	baseCommit := ticket.Data.Iterations[0].Commit
	if baseCommit == "" {
		return nil, fmt.Errorf("no base commit found for ticket")
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}

	diffLines, err := GetGitDiffLines(baseCommit, "", files)
	if err != nil {
		return nil, err
	}

	var result []TicketFile
	for _, filePath := range files {
		fileDiff := diffLines[filePath]
		sections := []TicketSection{}

		if fileDiff != nil && (len(fileDiff.Added) > 0 || len(fileDiff.Removed) > 0) {
			content, readErr := ReadTextFile(filepath.Join(GetRootDir(), filePath))
			if readErr == nil {
				lines := strings.Split(content, "\n")
				lang := GetLanguage(filePath)
				if lang != nil {
					fileSections := lang.ParseSections(content)
					fileDefs := lang.ParseDefinitions(content, lines)
					// Filter definitions to only include top-level ones
					// This logic usually depends on the parser implementation,
					// but here we can enforce it by checking if def is inside another def?
					// Or just trust ParseDefinitions if we fix it later.
					// For child exclusion, we update computeAffectedSections.
					removedLineMap := map[string][]int{}
					if len(fileDiff.Removed) > 0 {
						stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", baseCommit, filePath)}, "")
						if exitCode == 0 && stderr == "" {
							removedLineMap = computeSectionLineMap(lang.ParseSections(stdout), fileDiff.Removed, "")
						}
					}
					affectedSections := computeAffectedSections(filePath, fileSections, fileDefs, computeSectionLineMap(fileSections, fileDiff.Added, ""), removedLineMap, "")
					for _, sectionMetrics := range affectedSections {
						sections = append(sections, sectionMetrics)
					}
				}
			}
		}
		
		// Always include the file, even if no metrics found, if it was passed in files list?
		// User said: "Files should not have updated, added, removed. Just array of files."
		result = append(result, TicketFile{
			Path:     filePath,
			Sections: sections,
		})
	}
	return result, nil
}

func computeSectionLineMap(sections []Section, diffLines []int, parentPath string) map[string][]int {
	result := map[string][]int{}
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		linesInSection := computeLinesInRange(diffLines, section.StartLine, section.EndLine)
		childLines := []int{}
		for _, child := range section.Children {
			childLines = append(childLines, computeLinesInRange(diffLines, child.StartLine, child.EndLine)...)
		}
		exclusiveLines := setDifference(linesInSection, childLines)
		if len(exclusiveLines) > 0 {
			result[sectionPath] = append(result[sectionPath], exclusiveLines...)
		}
		if len(section.Children) > 0 {
			for key, value := range computeSectionLineMap(section.Children, diffLines, sectionPath) {
				result[key] = append(result[key], value...)
			}
		}
	}
	return result
}

func computeAffectedSections(filePath string, sections []Section, defs []DefinitionRange, addedLineMap map[string][]int, removedLineMap map[string][]int, parentPath string) []TicketSection {
	var result []TicketSection
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		exclusiveAddedLines := addedLineMap[sectionPath]
		exclusiveRemovedLines := removedLineMap[sectionPath]

		if len(exclusiveAddedLines) > 0 || len(exclusiveRemovedLines) > 0 {
			var affectedDefs []string
			for _, def := range defs {
				// Check if def is in this section
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					// Check if def is NOT in any child section
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						// Only use added lines to determine affected definitions
						// Removed lines reference OLD file positions which don't map to NEW file definitions
						defAddedLines := computeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, TicketSection{
				Name:        sectionPath,
				Range:       &Range{Start: section.StartIndex, End: section.EndIndex},
				Definitions: uniqueStrings(affectedDefs),
				Lines:       &LineMetrics{Added: len(exclusiveAddedLines), Removed: len(exclusiveRemovedLines)},
			})
		}

		if len(section.Children) > 0 {
			childResults := computeAffectedSections(filePath, section.Children, defs, addedLineMap, removedLineMap, sectionPath)
			result = append(result, childResults...)
		}
	}
	return result
}

func setDifference(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var diff []int
	for _, x := range a {
		if !m[x] {
			diff = append(diff, x)
		}
	}
	return diff
}

func setIntersection(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var intersection []int
	for _, x := range a {
		if m[x] {
			intersection = append(intersection, x)
		}
	}
	return intersection
}

func uniqueStrings(strs []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	for _, s := range strs {
		if !seen[s] {
			seen[s] = true
			result = append(result, s)
		}
	}
	return result
}

func GetGitDiffLines(baseCommit, headCommit string, paths []string) (map[string]*DiffLines, error) {
	if baseCommit == "" {
		return nil, fmt.Errorf("base commit is required")
	}
	args := BuildGitDiffArgs("-U0", baseCommit, headCommit, paths)
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff failed: %s", strings.TrimSpace(stderr))
	}
	result := make(map[string]*DiffLines)
	var currentFile string
	// Capture both old (-start,count) and new (+start,count) line ranges
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				// Parse removed lines (old file)
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

				// Parse added lines (new file)
				newStart, _ := strconv.Atoi(match[3])
				newCount := 1
				if match[4] != "" {
					newCount, _ = strconv.Atoi(match[4])
				}
				for i := 0; i < newCount; i++ {
					result[currentFile].Added = append(result[currentFile].Added, newStart+i)
				}
			}
		}
	}
	return result, nil
}

func computeLinesInRange(changedLines []int, startLine, endLine int) []int {
	var result []int
	for _, line := range changedLines {
		if line >= startLine && line <= endLine {
			result = append(result, line)
		}
	}
	return result
}

func findSectionForLine(sections []Section, line int) string {
	for _, section := range sections {
		if line >= section.StartLine && line <= section.EndLine {
			if len(section.Children) > 0 {
				childSection := findSectionForLine(section.Children, line)
				if childSection != "" {
					return section.Name + "/" + childSection
				}
			}
			return section.Name
		}
	}
	return ""
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

// buildFolderID constructs a globally unique folder ID
func buildFolderID(path string, bundleID *string) string {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	if bundleID != nil && *bundleID != "" {
		// Extract bundle name from bundle ID (@semio/BUNDLE → BUNDLE)
		bundleName := strings.TrimPrefix(*bundleID, "@semio/")
		return "@semio/" + bundleName + "/" + normalizedPath
	}
	return "@semio/repo/" + normalizedPath
}

// buildFileID constructs a globally unique file ID
func buildFileID(path string, bundleID *string) string {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	if bundleID != nil && *bundleID != "" {
		// Extract bundle name from bundle ID (@semio/BUNDLE → BUNDLE)
		bundleName := strings.TrimPrefix(*bundleID, "@semio/")
		return "@semio/" + bundleName + "/" + normalizedPath
	}
	return "@semio/repo/" + normalizedPath
}

// buildSectionID constructs a globally unique section ID
func buildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 {
		return fileID
	}
	return fileID + "#" + strings.Join(sectionPath, "#")
}

// buildDefinitionID constructs a globally unique definition ID
func buildDefinitionID(fileID string, sectionPath []string, name string) string {
	if len(sectionPath) > 0 {
		return fileID + "#" + strings.Join(sectionPath, "#") + "§" + name
	}
	return fileID + "§" + name
}

// buildViolationID constructs a globally unique violation ID
func buildViolationID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("@semio/violations/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("@semio/violations/%s#%d", scope, line)
	}
	return fmt.Sprintf("@semio/violations/%s", scope)
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

func GetGitDiffSectionLineMetrics(baseCommit, endCommit, filePath string) map[string]LineMetrics {
	// Conserved for backward compatibility if needed, but primarily logic moved to ComputeTicketFiles
	// Just return nil or implement basic logic if used elsewhere.
	// It seems it was only used in BuildTicketBundles which is being removed/replaced.
	return nil
}

func FlattenSections(sections []Section) []Section {
	var result []Section
	var flatten func(secs []Section)
	flatten = func(secs []Section) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

func FinishTicket(ticket *Ticket, summary string, files []string) error {
	if ticket.Data == nil {
		return fmt.Errorf("ticket data is nil")
	}
	if summary == "" {
		return fmt.Errorf("summary is required to finish a ticket")
	}
	if len(files) == 0 {
		return fmt.Errorf("at least one file is required to finish a ticket")
	}
	tickFilesResult, err := ComputeTicketFiles(ticket, files)
	if err != nil {
		return err
	}
	ticket.Data.Summary = summary
	ticket.Data.Files = tickFilesResult
	ticket.Data.Status = TicketStatusClosed
	now := time.Now()
	ticket.Data.Dates.Closed = &now
	return SaveTicket(ticket)
}

func ReopenTicket(ticket *Ticket, prompt, llm string) error {
	if ticket.Data == nil {
		return fmt.Errorf("ticket data is nil")
	}
	if ticket.Data.Status == TicketStatusOpen {
		return fmt.Errorf("ticket is already open")
	}
	gitAuthor := GetGitAuthorGithub()
	gitCommit := GetGitCommit()
	llmSlug := strings.ToLower(strings.ReplaceAll(llm, " ", "-"))
	
	iteration := TicketIteration{
		Prompt: prompt,
		LLM:    llmSlug,
		Author: gitAuthor,
		Date:   time.Now(),
		Commit: gitCommit,
	}
	
	ticket.Data.Iterations = append(ticket.Data.Iterations, iteration)
	ticket.Data.Status = TicketStatusOpen
	ticket.Data.Dates.Closed = nil
	return SaveTicket(ticket)
}

func CanCloseTicket(ticket *Ticket) (bool, []string) {
	var reasons []string
	if ticket.Data == nil {
		reasons = append(reasons, "Ticket data is nil")
		return false, reasons
	}
	planContent, _ := ReadTextFile(ticket.PlanPath)
	if planContent == "" || strings.TrimSpace(planContent) == "# Plan" {
		reasons = append(reasons, "Plan section is empty")
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
		Contributions: ContributorContributionsStorage{},
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
	Lines       LineMetrics
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

func addSectionsToContributor(state *ContributorContributionState, filePath string, section Section) {
	regionKey := filePath + "#" + section.Name
	state.Regions[regionKey] = struct{}{}
	for _, child := range section.Children {
		addSectionsToContributor(state, filePath, child)
	}
}

func findSectionForDefinition(sections []Section, defStart, defEnd int, parentPath string) string {
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
		contributors[i].Contributions = ContributorContributionsStorage{}
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
		if ticket.Data == nil || len(ticket.Data.Iterations) == 0 {
			continue
		}
		// Use first iteration author/commit for attribution? Or iterate all iterations?
		// For now using first iteration as 'creator'.
		firstIter := ticket.Data.Iterations[0]
		
		ticketKey := fmt.Sprintf("%04d-%02d-%02d-%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		ticketContributors := map[string]struct{}{}
		
		if name, email, ok := ParseContributorIdentity(firstIter.Author); ok {
			if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" {
				ticketContributors[github] = struct{}{}
				if ticket.Data.Files != nil && stateByGithub[github] != nil {
					for _, file := range ticket.Data.Files {
						for _, section := range file.Sections {
							if section.Lines != nil {
								stateByGithub[github].Lines.Added += section.Lines.Added
								stateByGithub[github].Lines.Removed += section.Lines.Removed
							}
						}
					}
				}
			}
		}
		if firstIter.Commit != "" {
			if name, email, ok := ParseContributorIdentity(firstIter.Author); ok {
				if github := ResolveContributorGithub(name, email, emailToGithub, nameToGithub); github != "" && stateByGithub[github] != nil {
					commitTitle := commitTitleCache[firstIter.Commit]
					if commitTitle == "" {
						commitTitle = GetGitCommitTitle(firstIter.Commit)
						if commitTitle == "" {
							commitTitle = firstIter.Commit
						}
						commitTitleCache[firstIter.Commit] = commitTitle
					}
					stateByGithub[github].Commits[firstIter.Commit] = ContributorCommit{Title: commitTitle, Sha: firstIter.Commit}
				}
			}
		}
		for github := range ticketContributors {
			if stateByGithub[github] == nil {
				continue
			}
			stateByGithub[github].Tickets[ticketKey] = ContributorTicket{
				Year:     ticket.Year,
				Month:    ticket.Month,
				Day:      ticket.Day,
				Slug:     ticket.Slug,
				Status:   ticket.Data.Status,
				FilePath: ticket.JsonPath,
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


func NewRepoContext(dir string) *repoContext {
	return &repoContext{rootDir: dir}
}

func (c *repoContext) GetRootDir() string {
	return c.rootDir
}

func (c *repoContext) GetBundles() []*Bundle {
	bundles := GetProjects()
	result := make([]*Bundle, len(bundles))
	for i := range bundles {
		result[i] = &bundles[i]
	}
	return result
}

func (c *repoContext) GetFolders() []*Folder {
	bundles := GetProjects()
	files, _ := ScopeToFiles(Scope{Kind: ScopeRepo}, bundles)
	folderSet := make(map[string]bool)
	for _, f := range files {
		dir := filepath.Dir(f)
		for dir != "." && dir != "" {
			folderSet[dir] = true
			dir = filepath.Dir(dir)
		}
	}
	result := make([]*Folder, 0, len(folderSet))
	for path := range folderSet {
		name := filepath.Base(path)
		parent := filepath.Dir(path)
		var parentID *string
		if parent != "." && parent != "" {
			pid := "folder:" + parent
			parentID = &pid
		}
		uri := "file://" + NormalizePath(filepath.Join(c.rootDir, path))
		result = append(result, &Folder{
			ID:       "folder:" + path,
			Path:     path,
			URI:      uri,
			Name:     name,
			ParentID: parentID,
		})
	}
	return result
}

func (c *repoContext) GetFiles() []*File {
	bundles := GetProjects()
	files, _ := ScopeToFiles(Scope{Kind: ScopeRepo}, bundles)
	result := make([]*File, 0, len(files))
	for _, path := range files {
		name := filepath.Base(path)
		ext := filepath.Ext(name)
		dir := filepath.Dir(path)
		var folderID *string
		if dir != "." && dir != "" {
			fid := "folder:" + dir
			folderID = &fid
		}
		uri := "file://" + NormalizePath(filepath.Join(c.rootDir, path))
		result = append(result, &File{
			ID:        "file:" + path,
			Path:      path,
			URI:       uri,
			Name:      name,
			Extension: ext,
			FolderID:  folderID,
		})
	}
	return result
}

func (c *repoContext) GetSections() []*Section {
	bundles := GetProjects()
	files, _ := ScopeToFiles(Scope{Kind: ScopeRepo}, bundles)
	var result []*Section
	for _, path := range files {
		absPath := filepath.Join(c.rootDir, path)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, path)
		c.collectSections(&result, sections, path, nil)
	}
	return result
}

func (c *repoContext) collectSections(result *[]*Section, sections []Section, filePath string, parentID *string) {
	for i := range sections {
		s := &sections[i]
		id := fmt.Sprintf("section:%s#%s", filePath, s.Name)
		if parentID != nil {
			// section ID derived from parent path
			// But for listing we just need an ID
			id = fmt.Sprintf("section:%s#%s", filePath, s.Name) // simplified for now
		}
		s.FilePath = filePath
		*result = append(*result, s)
		if len(s.Children) > 0 {
			var pidStr string = id // Simplified
			c.collectSections(result, s.Children, filePath, &pidStr)
		}
	}
}

func (c *repoContext) GetDefinitions() []*Definition {
	bundles := GetProjects()
	files, _ := ScopeToFiles(Scope{Kind: ScopeRepo}, bundles)
	var result []*Definition
	for _, path := range files {
		absPath := filepath.Join(c.rootDir, path)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		language := GetLanguage(path)
		if language == nil || !language.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := language.ParseDefinitions(content, lines)
		
		bundleName := ResolveBundleForPath(path, bundles)
		var fileID string
		if bundleName != "" {
			relativePath := strings.TrimPrefix(path, strings.TrimPrefix(bundles[0].Root, "")+"/")
			fileID = "@semio/" + bundleName + "/" + relativePath
		} else {
			fileID = "@semio/repo/" + path
		}
		
		for _, d := range defs {
			result = append(result, &Definition{
				Name:      d.Name,
				FilePath:  fileID,
				StartLine: d.Start,
				EndLine:   d.End,
			})
		}
	}
	return result
}

func (c *repoContext) GetContributors() ([]*Contributor, error) {
	contributors, err := ListContributors()
	if err != nil {
		return nil, err
	}
	result := make([]*Contributor, len(contributors))
	for i := range contributors {
		result[i] = &contributors[i]
	}
	return result, nil
}

func (c *repoContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*Ticket
	for i := range tickets {
		t := &tickets[i]
		if status != nil && t.GetStatus() != *status {
			continue
		}
		result = append(result, t)
	}
	return result, nil
}

func (c *repoContext) GetPolicies() []*Policy {
	policies := GetPolicies()
	result := make([]*Policy, len(policies))
	for i, p := range policies {
		var scopes []string
		scopes = append(scopes, p.Scopes...)
		var descPtr *string
		if p.Description != "" {
			descPtr = &p.Description
		}
		var violationKinds []*ViolationKindMeta
		for _, kind := range p.Kinds {
			info := kind.Info()
			priority := ViolationPriorityMedium
			switch info.Priority {
			case ViolationPriorityHigh:
				priority = ViolationPriorityHigh
			case ViolationPriorityLow:
				priority = ViolationPriorityLow
			}
				violationKinds = append(violationKinds, &ViolationKindMeta{
				Kind:        kind,
				PolicyID:    p.ID,
				Priority:    priority,
				Autofixable: info.Autofixable,
				Reason:      info.Reason,
				Solution:    info.Solution,
			})
		}
		result[i] = &Policy{
			ID:             "@semio/policies/" + p.ID,
			Name:           p.ID,
			Description:    descPtr,
			Scopes:         scopes,
			ViolationKinds: violationKinds,
		}
	}
	return result
}

func (c *repoContext) GetViolationKinds() []*ViolationKindMeta {
	var result []*ViolationKindMeta
	for _, p := range GetPolicies() {
		for _, kind := range p.Kinds {
			info := kind.Info()
			priority := ViolationPriorityMedium
			switch info.Priority {
			case ViolationPriorityHigh:
				priority = ViolationPriorityHigh
			case ViolationPriorityLow:
				priority = ViolationPriorityLow
			}
			result = append(result, &ViolationKindMeta{
				Kind:        kind,
				PolicyID:    p.ID,
				Priority:    priority,
				Autofixable: info.Autofixable,
				Reason:      info.Reason,
				Solution:    info.Solution,
			})
		}
	}
	return result
}

func (c *repoContext) Analyze(scope *string) (*AnalyzeResult, error) {
	scopeRaw := "@semio"
	if scope != nil {
		scopeRaw = *scope
	}
	toolResult := ToolAnalyze(scopeRaw, nil)
	report, ok := toolResult.Data.(AnalyzeReport)
	if !ok {
		return &AnalyzeResult{
			Violations: []*Violation{},
			Metrics:    &AnalyzeMetrics{Total: 0, ByPriority: &PriorityCount{}, Autofixable: 0},
		}, nil
	}
	kindInfoMap := make(map[ViolationKind]ViolationKindMeta)
	for _, p := range GetPolicies() {
		for _, kind := range p.Kinds {
			kindInfoMap[kind] = kind.Info()
		}
	}
	violations := make([]*Violation, len(report.Violations))
	for i, v := range report.Violations {
		var excerptPtr *string
		if v.Summary != "" {
			excerptPtr = &v.Summary
		}
		excerpt := ""
		if excerptPtr != nil {
			excerpt = *excerptPtr
		}
		violations[i] = &Violation{
			ID:      v.ID,
			Summary: v.Summary,
			Kind:    v.Kind,
			Scope:   v.Scope,
			Line:    v.Line,
			Column:  v.Column,
			Excerpt: excerpt,
			Autofix: v.Autofix,
		}
	}
	return &AnalyzeResult{
		Violations: violations,
		Metrics: &AnalyzeMetrics{
			Total: report.Summary.Total,
			ByPriority: &PriorityCount{
				High:   report.Summary.ByPriority["high"],
				Medium: report.Summary.ByPriority["medium"],
				Low:    report.Summary.ByPriority["low"],
			},
			Autofixable: 0,
		},
	}, nil
}

func (c *repoContext) Fix(scope *string) (*FixResult, error) {
	scopeRaw := "@semio"
	if scope != nil {
		scopeRaw = *scope
	}
	ToolFix(scopeRaw)
	return &FixResult{Fixed: 0, Remaining: 0, Violations: []*Violation{}}, nil
}

func (c *repoContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	return CreateTicket(input.Title, input.Prompt, input.LLM, "")
}

func (c *repoContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if err := FinishTicket(ticket, input.Summary, input.Files); err != nil {
		return nil, err
	}
	return ticket, nil
}

func (c *repoContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if err := ReopenTicket(ticket, input.Prompt, input.LLM); err != nil {
		return nil, err
	}
	return ticket, nil
}

func (c *repoContext) FolderCreate(path string) (*Folder, error) {
	result := ToolFolderCreate(path)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	return &Folder{
		ID:   buildFolderID(normalizedPath, bundleID),
		Path: normalizedPath,
		URI:  fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name: name,
		BundleID: bundleID,
	}, nil
}

func (c *repoContext) FolderMove(src, dst string) (*Folder, error) {
	result := ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(dst, "\\", "/")
	name := filepath.Base(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	return &Folder{
		ID:   buildFolderID(normalizedPath, bundleID),
		Path: normalizedPath,
		URI:  fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name: name,
		BundleID: bundleID,
	}, nil
}

func (c *repoContext) FolderDelete(path string) error {
	result := ToolFolderDelete(path)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) FileCreate(path string) (*File, error) {
	result := ToolFileCreate(path)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	var folderID *string
	if folderPath != "." {
		id := buildFolderID(folderPath, bundleID)
		folderID = &id
	}
	
	return &File{
		ID:        buildFileID(normalizedPath, bundleID),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
		BundleID:  bundleID,
	}, nil
}

func (c *repoContext) FileMove(src, dst string) (*File, error) {
	result := ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	normalizedPath := strings.ReplaceAll(dst, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	var folderID *string
	if folderPath != "." {
		id := buildFolderID(folderPath, bundleID)
		folderID = &id
	}
	
	return &File{
		ID:        buildFileID(normalizedPath, bundleID),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", c.rootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
		BundleID:  bundleID,
	}, nil
}

func (c *repoContext) FileDelete(path string) error {
	result := ToolFileDelete(path)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	return &Section{
		Name: name,
	}, nil
}

func (c *repoContext) SectionMove(file, oldName, newName string) (*Section, error) {
	result := ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	return &Section{
		Name: newName,
	}, nil
}

func (c *repoContext) SectionDelete(file, name string) error {
	result := ToolSectionDelete(file, name)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

func (c *repoContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	result := ToolContributorAdd(input.Github)
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	contrib, ok := result.Data.(*Contributor)
	if !ok {
		return nil, fmt.Errorf("unexpected result type")
	}
	return contrib, nil
}

func (c *repoContext) ContributorRemove(github string) error {
	result := ToolContributorRemove(github)
	if result.Error != "" {
		return fmt.Errorf("%s", result.Error)
	}
	return nil
}

type GraphQLExecutor interface {
	ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error)
}

var graphQLExecutorFactory func(rootDir string, ctx *repoContext) (GraphQLExecutor, error)

func SetGraphQLExecutorFactory(factory func(rootDir string, ctx *repoContext) (GraphQLExecutor, error)) {
	graphQLExecutorFactory = factory
}

func ExecuteGraphQL(query string, variables map[string]interface{}) (string, error) {
	if graphQLExecutorFactory == nil {
		return "", fmt.Errorf("GraphQL executor factory not set")
	}
	ctx := NewRepoContext(rootDir)
	executor, err := graphQLExecutorFactory(rootDir, ctx)
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

func ToolTicketOpen(title, prompt, llm, planPath string) ToolResult {
	output := NewOutput()
	if prompt == "" {
		prompt = title
	}
	ticket, err := CreateTicket(title, prompt, llm, planPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🎫 Created ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Folder: %s", ticket.FolderPath))
	if ticket.JsonPath != "" {
		output.Info(fmt.Sprintf("   JSON:   %s", ticket.JsonPath))
	}
	if ticket.PlanPath != "" {
		output.Info(fmt.Sprintf("   Plan:   %s", ticket.PlanPath))
	}
	if ticket.LogPath != "" {
		output.Info(fmt.Sprintf("   Log:    %s", ticket.LogPath))
	}
	if ticket.SummaryPath != "" {
		output.Info(fmt.Sprintf("   Summary: %s", ticket.SummaryPath))
	}
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
		if t.GetStatus() == TicketStatusClosed {
			status = "✅"
		}
		output.Plain(fmt.Sprintf("   %s %d/%s/%s/%s", status, t.Year, PadNumber(t.Month, 2), PadNumber(t.Day, 2), t.Slug))
		output.Plain(fmt.Sprintf("      %s", t.GetTitle()))
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
	output.Plain(fmt.Sprintf("   Status: %s", ticket.GetStatus()))
	output.Plain(fmt.Sprintf("   Created: %s", ticket.GetDateCreated()))
	output.Plain(fmt.Sprintf("   Prompt: %s", ticket.GetPrompt()))
	if ticket.GetLLM() != "" {
		output.Plain(fmt.Sprintf("   LLM: %s", ticket.GetLLM()))
	}
	planContent, _ := ReadTextFile(ticket.PlanPath)
	if planContent != "" {
		output.Plain(fmt.Sprintf("\n%s", planContent))
	}
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketClose(year, month, day int, slug, summary string, files []string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := FinishTicket(ticket, summary, files); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	filesResult := ticket.GetFiles()
	filesCount := 0
	sectionsCount := 0
	defsCount := 0
	if filesResult != nil {
		filesCount = len(filesResult.Updated) + len(filesResult.Removed)
		for _, entry := range filesResult.Updated {
			sectionsCount += len(entry.Sections)
			for _, section := range entry.Sections {
				defsCount += len(section.Definitions)
			}
		}
	}
	output.Success(fmt.Sprintf("\n✅ Ticket finished: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Summary: %s", summary))
	output.Info(fmt.Sprintf("   Files: %d", filesCount))
	output.Info(fmt.Sprintf("   Sections affected: %d", sectionsCount))
	output.Info(fmt.Sprintf("   Definitions affected: %d", defsCount))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketReopen(year, month, day int, slug, prompt, llm string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ReopenTicket(ticket, prompt, llm); err != nil {
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
	var printSection func(s Section, indent string)
	printSection = func(s Section, indent string) {
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
	var printSection func(s Section, prefix string)
	printSection = func(s Section, prefix string) {
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
	return ToolResult{Output: *output, Data: []Definition{}}
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

// #region SQLite Export

type ExportResult struct {
	Path       string `json:"path"`
	Bundles    int    `json:"bundles"`
	Folders    int    `json:"folders"`
	Files      int    `json:"files"`
	Sections   int    `json:"sections"`
	Definitions int   `json:"definitions"`
	Contributors int  `json:"contributors"`
	Tickets    int    `json:"tickets"`
	Policies   int    `json:"policies"`
	ViolationKinds int `json:"violationKinds"`
	Violations int    `json:"violations"`
}

func ExportToSQLite(outputPath string, ctx RepoContext) (*ExportResult, error) {
	if outputPath == "" {
		outputPath = filepath.Join(ctx.GetRootDir(), "repo.db")
	}
	if err := os.Remove(outputPath); err != nil && !os.IsNotExist(err) {
		return nil, fmt.Errorf("failed to remove existing database: %w", err)
	}
	db, err := sql.Open("sqlite", outputPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}
	defer db.Close()
	schemaPath := filepath.Join(ctx.GetRootDir(), "sql", "sqlite", "repo", "schema.sql")
	schemaBytes, err := os.ReadFile(schemaPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read schema file: %w", err)
	}
	if _, err := db.Exec(string(schemaBytes)); err != nil {
		return nil, fmt.Errorf("failed to execute schema: %w", err)
	}
	tx, err := db.Begin()
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()
	result := &ExportResult{Path: outputPath}
	if err := exportRepo(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export repo: %w", err)
	}
	if result.Bundles, err = exportBundles(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export bundles: %w", err)
	}
	if result.Folders, err = exportFolders(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export folders: %w", err)
	}
	if result.Files, result.Sections, result.Definitions, err = exportFiles(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export files: %w", err)
	}
	if result.Contributors, err = exportContributors(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export contributors: %w", err)
	}
	if result.Tickets, err = exportTickets(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export tickets: %w", err)
	}
	if result.Policies, result.ViolationKinds, err = exportPolicies(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export policies: %w", err)
	}
	analyzeResult, err := ctx.Analyze(nil)
	if err != nil {
		return nil, fmt.Errorf("failed to analyze: %w", err)
	}
	if result.Violations, err = exportViolations(tx, analyzeResult.Violations); err != nil {
		return nil, fmt.Errorf("failed to export violations: %w", err)
	}
	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}
	return result, nil
}

func exportRepo(tx *sql.Tx, ctx RepoContext) error {
	_, err := tx.Exec(`INSERT INTO repo (id, name, path, exported_at) VALUES (?, ?, ?, ?)`,
		"repo:semio",
		"semio",
		ctx.GetRootDir(),
		time.Now().UTC().Format(time.RFC3339))
	return err
}

func exportBundles(tx *sql.Tx, ctx RepoContext) (int, error) {
	bundles := ctx.GetBundles()
	stmt, err := tx.Prepare(`INSERT INTO bundle (id, name, root, source_root, project_type, uri) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	tagStmt, err := tx.Prepare(`INSERT INTO bundle_tag (bundle_id, tag) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer tagStmt.Close()
	for _, b := range bundles {
		id := "bundle:" + b.Name
		uri := "file://" + NormalizePath(filepath.Join(ctx.GetRootDir(), b.Root))
		var sourceRoot, projectType interface{}
		if b.SourceRoot != "" {
			sourceRoot = b.SourceRoot
		}
		if b.ProjectType != "" {
			projectType = b.ProjectType
		}
		if _, err := stmt.Exec(id, b.Name, b.Root, sourceRoot, projectType, uri); err != nil {
			return 0, err
		}
		for _, tag := range b.Tags {
			if _, err := tagStmt.Exec(id, tag); err != nil {
				return 0, err
			}
		}
	}
	return len(bundles), nil
}

func exportFolders(tx *sql.Tx, ctx RepoContext) (int, error) {
	folders := ctx.GetFolders()
	stmt, err := tx.Prepare(`INSERT INTO folder (id, path, uri, name, parent_id, bundle_id) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	bundles := ctx.GetBundles()
	bundleMap := make(map[string]string)
	for _, b := range bundles {
		bundleMap[b.Root] = "bundle:" + b.Name
	}
	for _, f := range folders {
		var bundleID interface{}
		for root, bid := range bundleMap {
			if strings.HasPrefix(f.Path, root) {
				bundleID = bid
				break
			}
		}
		if _, err := stmt.Exec(f.ID, f.Path, f.URI, f.Name, f.ParentID, bundleID); err != nil {
			return 0, err
		}
	}
	return len(folders), nil
}

func exportFiles(tx *sql.Tx, ctx RepoContext) (int, int, int, error) {
	files := ctx.GetFiles()
	fileStmt, err := tx.Prepare(`INSERT INTO file (id, path, uri, name, extension, folder_id, bundle_id, lines) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer fileStmt.Close()
	sectionStmt, err := tx.Prepare(`INSERT INTO section (id, name, path, file_id, parent_id, start_line, end_line, start_column, end_column) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer sectionStmt.Close()
	defStmt, err := tx.Prepare(`INSERT INTO definition (id, name, kind, file_id, section_id, start_line, end_line, start_column, end_column) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer defStmt.Close()
	bundles := ctx.GetBundles()
	bundleMap := make(map[string]string)
	for _, b := range bundles {
		bundleMap[b.Root] = "bundle:" + b.Name
	}
	totalSections := 0
	totalDefs := 0
	for _, f := range files {
		var bundleID interface{}
		for root, bid := range bundleMap {
			if strings.HasPrefix(f.Path, root) {
				bundleID = bid
				break
			}
		}
		absPath := filepath.Join(ctx.GetRootDir(), f.Path)
		lines := 0
		if content, err := ReadTextFile(absPath); err == nil {
			lines = strings.Count(content, "\n") + 1
		}
		if _, err := fileStmt.Exec(f.ID, f.Path, f.URI, f.Name, f.Extension, f.FolderID, bundleID, lines); err != nil {
			return 0, 0, 0, err
		}
		if content, err := ReadTextFile(absPath); err == nil {
			sections := ParseSections(content, f.Path)
			sectionCount, err := exportSectionsRecursive(sectionStmt, sections, f.ID, f.Path, nil)
			if err != nil {
				return 0, 0, 0, err
			}
			totalSections += sectionCount
		}
	}
	return len(files), totalSections, totalDefs, nil
}

func exportSectionsRecursive(sectionStmt *sql.Stmt, sections []Section, fileID, filePath string, parentID *string) (int, error) {
	count := 0
	for _, s := range sections {
		sectionID := fmt.Sprintf("section:%s#%s", filePath, s.Name)
		sectionPath := s.Name
		if parentID != nil {
			sectionPath = strings.TrimPrefix(*parentID, "section:"+filePath+"#") + "/" + s.Name
			sectionID = fmt.Sprintf("section:%s#%s", filePath, sectionPath)
		}
		if _, err := sectionStmt.Exec(sectionID, s.Name, sectionPath, fileID, parentID, s.StartLine, s.EndLine, 0, 0); err != nil {
			return 0, err
		}
		count++
		if len(s.Children) > 0 {
			childCount, err := exportSectionsRecursive(sectionStmt, s.Children, fileID, filePath, &sectionID)
			if err != nil {
				return 0, err
			}
			count += childCount
		}
	}
	return count, nil
}

func exportContributors(tx *sql.Tx, ctx RepoContext) (int, error) {
	contributors, err := ctx.GetContributors()
	if err != nil {
		return 0, err
	}
	stmt, err := tx.Prepare(`INSERT INTO contributor (id, github, name, avatar_url, avatar_round_url, github_icon_url) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	emailStmt, err := tx.Prepare(`INSERT INTO contributor_email (contributor_id, email) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer emailStmt.Close()
	linkStmt, err := tx.Prepare(`INSERT INTO contributor_link (contributor_id, name, url) VALUES (?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer linkStmt.Close()
	for _, c := range contributors {
		id := "contributor:" + c.Github
		var name interface{}
		if c.Name != "" {
			name = c.Name
		}
		if _, err := stmt.Exec(id, c.Github, name, nil, nil, nil); err != nil {
			return 0, err
		}
		for _, email := range c.Emails {
			if _, err := emailStmt.Exec(id, email); err != nil {
				return 0, err
			}
		}
		for linkName, url := range c.Links {
			if _, err := linkStmt.Exec(id, linkName, url); err != nil {
				return 0, err
			}
		}
	}
	return len(contributors), nil
}

func exportTickets(tx *sql.Tx, ctx RepoContext) (int, error) {
	tickets, err := ctx.GetTickets(nil, nil, nil, nil)
	if err != nil {
		return 0, err
	}
	ticketStmt, err := tx.Prepare(`INSERT INTO ticket (id, year, month, day, slug, title, path, uri, prompt, summary, status, author_id, model, llm, commit_sha, created_at, finished_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer ticketStmt.Close()
	ticketFileStmt, err := tx.Prepare(`INSERT INTO ticket_file (ticket_id, file_path) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer ticketFileStmt.Close()
	for _, t := range tickets {
		ticketID := t.GetID()
		uri := "file://" + NormalizePath(t.FolderPath)
		status := string(t.GetStatus())
		if status == "" {
			status = "open"
		}
		var authorID, model, llm, summary, commit, finishedAt interface{}
		if author := t.GetAuthor(); author != "" {
			authorID = "contributor:" + author
		}
		if val := t.GetLLM(); val != "" {
			llm = val
		}
		if val := t.GetCommit(); val != "" {
			commit = val
		}
		if s := t.GetSummary(); s != "" {
			summary = s
		}
		createdAtTime := t.GetDateCreated()
		var createdAt string
		if createdAtTime.IsZero() {
			createdAt = time.Now().UTC().Format(time.RFC3339)
		} else {
			createdAt = createdAtTime.Format(time.RFC3339)
		}
		if f := t.GetDateFinished(); f != nil {
			finishedAt = f.Format(time.RFC3339)
		}
		if _, err := ticketStmt.Exec(ticketID, t.Year, t.Month, t.Day, t.Slug, t.GetTitle(), t.FolderPath, uri, t.GetPrompt(), summary, status, authorID, model, llm, commit, createdAt, finishedAt); err != nil {
			return 0, err
		}
		for _, entry := range t.GetFiles().Updated {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range t.GetFiles().Created {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range t.GetFiles().Removed {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
	}
	return len(tickets), nil
}

func exportPolicies(tx *sql.Tx, ctx RepoContext) (int, int, error) {
	policies := ctx.GetPolicies()
	policyStmt, err := tx.Prepare(`INSERT INTO policy (id, name, description) VALUES (?, ?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer policyStmt.Close()
	scopeStmt, err := tx.Prepare(`INSERT INTO policy_scope (policy_id, scope) VALUES (?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer scopeStmt.Close()
	kindStmt, err := tx.Prepare(`INSERT INTO violation_kind (id, policy_id, priority, autofixable, reason, solution) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer kindStmt.Close()
	totalKinds := 0
	for _, p := range policies {
		policyID := "policy:" + p.ID
		var desc interface{}
		if p.Description != nil {
			desc = *p.Description
		}
		if _, err := policyStmt.Exec(policyID, p.Name, desc); err != nil {
			return 0, 0, err
		}
		for _, scope := range p.Scopes {
			if _, err := scopeStmt.Exec(policyID, scope); err != nil {
				return 0, 0, err
			}
		}
		for _, vk := range p.ViolationKinds {
			kindID := "violationKind:" + string(vk.Kind)
			autofixable := 0
			if vk.Autofixable {
				autofixable = 1
			}
			if _, err := kindStmt.Exec(kindID, policyID, string(vk.Priority), autofixable, vk.Reason, vk.Solution); err != nil {
				return 0, 0, err
			}
			totalKinds++
		}
	}
	return len(policies), totalKinds, nil
}

func exportViolations(tx *sql.Tx, violations []*Violation) (int, error) {
	stmt, err := tx.Prepare(`INSERT INTO violation (id, kind_id, scope, file_id, folder_id, line, column_num, excerpt, summary, autofix_description, autofix_edits) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	for _, v := range violations {
		kindID := "violationKind:" + string(v.Kind)
		var fileID, folderID, line, column, excerpt, autofixDesc, autofixEdits interface{}
		if v.Line > 0 {
			line = v.Line
		}
		if v.Column > 0 {
			column = v.Column
		}
		if v.Excerpt != "" {
			excerpt = v.Excerpt
		}
		filePath := extractFileFromScope(v.Scope)
		if filePath != "" {
			fileID = "file:" + filePath
			dir := filepath.Dir(filePath)
			if dir != "." && dir != "" {
				folderID = "folder:" + dir
			}
		}
		if v.Autofix != nil {
			autofixDesc = v.Autofix.Description
			if editsJSON, err := json.Marshal(v.Autofix.Edits); err == nil {
				autofixEdits = string(editsJSON)
			}
		}
		if _, err := stmt.Exec(v.ID, kindID, v.Scope, fileID, folderID, line, column, excerpt, v.Summary, autofixDesc, autofixEdits); err != nil {
			return 0, err
		}
	}
	return len(violations), nil
}

func ToolExport(outputPath string) ToolResult {
	output := NewOutput()
	output.Info("\n📦 Exporting repo to SQLite...")
	ctx := NewRepoContext(rootDir)
	result, err := ExportToSQLite(outputPath, ctx)
	if err != nil {
		output.Error(fmt.Sprintf("Export failed: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("Exported to: %s", result.Path))
	output.Plain(fmt.Sprintf("  Bundles: %d", result.Bundles))
	output.Plain(fmt.Sprintf("  Folders: %d", result.Folders))
	output.Plain(fmt.Sprintf("  Files: %d", result.Files))
	output.Plain(fmt.Sprintf("  Sections: %d", result.Sections))
	output.Plain(fmt.Sprintf("  Definitions: %d", result.Definitions))
	output.Plain(fmt.Sprintf("  Contributors: %d", result.Contributors))
	output.Plain(fmt.Sprintf("  Tickets: %d", result.Tickets))
	output.Plain(fmt.Sprintf("  Policies: %d", result.Policies))
	output.Plain(fmt.Sprintf("  Violation Kinds: %d", result.ViolationKinds))
	output.Plain(fmt.Sprintf("  Violations: %d", result.Violations))
	return ToolResult{Output: *output, Data: result}
}

// #endregion SQLite Export

// #endregion Commands

// #region GraphQL Context Interface

type RepoContext interface {
	GetRootDir() string
	GetBundles() []*Bundle
	GetFolders() []*Folder
	GetFiles() []*File
	GetSections() []*Section
	GetDefinitions() []*Definition
	GetContributors() ([]*Contributor, error)
	GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error)
	GetPolicies() []*Policy
	GetViolationKinds() []*ViolationKindMeta
	Analyze(scope *string) (*AnalyzeResult, error)
	Fix(scope *string) (*FixResult, error)
	TicketOpen(input TicketOpenInput) (*Ticket, error)
	TicketClose(input TicketCloseInput) (*Ticket, error)
	TicketReopen(input TicketReopenInput) (*Ticket, error)
	FolderCreate(path string) (*Folder, error)
	FolderMove(src, dst string) (*Folder, error)
	FolderDelete(path string) error
	FileCreate(path string) (*File, error)
	FileMove(src, dst string) (*File, error)
	FileDelete(path string) error
	SectionCreate(file, name string, parent *string) (*Section, error)
	SectionMove(file, oldName, newName string) (*Section, error)
	SectionDelete(file, name string) error
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
}

// #endregion GraphQL Context Interface

// #region GraphQL Resolver

type Resolver struct {
	RootDir string
	Ctx     RepoContext
}

func NewResolver(rootDir string) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: NewRepoContext(rootDir)}
}

func NewResolverWithContext(rootDir string, ctx RepoContext) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: ctx}
}

func (r *Resolver) context() RepoContext {
	return r.Ctx
}

// #endregion GraphQL Resolver

// #region Default Context

type defaultContext struct {
	rootDir string
}

func NewDefaultContext(rootDir string) RepoContext {
	return &defaultContext{rootDir: rootDir}
}

func (c *defaultContext) GetRootDir() string { return c.rootDir }

func (c *defaultContext) GetBundles() []*Bundle { return []*Bundle{} }

func (c *defaultContext) GetFolders() []*Folder { return []*Folder{} }

func (c *defaultContext) GetFiles() []*File { return []*File{} }

func (c *defaultContext) GetDefinitions() []*Definition { return []*Definition{} }

func (c *defaultContext) GetSections() []*Section { return []*Section{} }

func (c *defaultContext) GetContributors() ([]*Contributor, error) { return []*Contributor{}, nil }

func (c *defaultContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return []*Ticket{}, nil
}

func (c *defaultContext) GetPolicies() []*Policy { return []*Policy{} }

func (c *defaultContext) GetViolationKinds() []*ViolationKindMeta { return []*ViolationKindMeta{} }

func (c *defaultContext) Analyze(scope *string) (*AnalyzeResult, error) {
	return &AnalyzeResult{Violations: []*Violation{}, Metrics: &AnalyzeMetrics{}}, nil
}

func (c *defaultContext) Fix(scope *string) (*FixResult, error) {
	return &FixResult{Violations: []*Violation{}}, nil
}

func (c *defaultContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderMove(src, dst string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderDelete(path string) error { return nil }

func (c *defaultContext) FileCreate(path string) (*File, error) { return nil, nil }

func (c *defaultContext) FileMove(src, dst string) (*File, error) { return nil, nil }

func (c *defaultContext) FileDelete(path string) error { return nil }

func (c *defaultContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionDelete(file, name string) error { return nil }

func (c *defaultContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *defaultContext) ContributorRemove(github string) error { return nil }

var _ RepoContext = (*defaultContext)(nil)

// #endregion Default Context

// #region GraphQL Executor

func parseFileListInput(f map[string]interface{}) *FileListInput {
	files := &FileListInput{}
	if updated, ok := f["updated"].([]interface{}); ok {
		for _, u := range updated {
			if s, ok := u.(string); ok {
				files.Updated = append(files.Updated, s)
			}
		}
	}
	if created, ok := f["created"].([]interface{}); ok {
		for _, c := range created {
			if s, ok := c.(string); ok {
				files.Created = append(files.Created, s)
			}
		}
	}
	if removed, ok := f["removed"].([]interface{}); ok {
		for _, r := range removed {
			if s, ok := r.(string); ok {
				files.Removed = append(files.Removed, s)
			}
		}
	}
	return files
}

type Executor struct {
	resolver *Resolver
	schema   graphql.Schema
}

func NewExecutor(rootDir string) (*Executor, error) {
	resolver := NewResolver(rootDir)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func NewExecutorWithContext(rootDir string, ctx RepoContext) (*Executor, error) {
	resolver := NewResolverWithContext(rootDir, ctx)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func (e *Executor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	result := graphql.Do(graphql.Params{
		Context:        ctx,
		Schema:         e.schema,
		RequestString:  query,
		VariableValues: variables,
	})
	if len(result.Errors) > 0 {
		return nil, fmt.Errorf("graphql errors: %v", result.Errors)
	}
	return result.Data, nil
}

func (e *Executor) ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error) {
	data, err := e.Execute(ctx, query, variables)
	if err != nil {
		return "", err
	}
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", err
	}
	return string(jsonBytes), nil
}

func (e *Executor) ValidateQuery(query string) error {
	_, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	return err
}

func (e *Executor) GetOperationType(query string) (string, error) {
	doc, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	if err != nil {
		return "", err
	}
	for _, def := range doc.Definitions {
		if opDef, ok := def.(*ast.OperationDefinition); ok {
			return string(opDef.Operation), nil
		}
	}
	return "query", nil
}

// #endregion GraphQL Executor

// #region Schema Builder

func buildSchema(resolver *Resolver) (graphql.Schema, error) {
	/* positionType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Position",
		Fields: graphql.Fields{
			"line":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"character": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	}) */

	rangeType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Range",
		Fields: graphql.Fields{
			"start": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"end":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	countMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "CountMetrics",
		Fields: graphql.Fields{
			"added":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"updated": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"removed": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	priorityCountType := graphql.NewObject(graphql.ObjectConfig{
		Name: "PriorityCount",
		Fields: graphql.Fields{
			"high":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"medium": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"low":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	analyzeMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeMetrics",
		Fields: graphql.Fields{
			"total":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"byPriority":  &graphql.Field{Type: priorityCountType},
			"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	definitionKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "DefinitionKind",
		Values: graphql.EnumValueConfigMap{
			"FUNCTION":  &graphql.EnumValueConfig{Value: DefinitionKindFunction},
			"CLASS":     &graphql.EnumValueConfig{Value: DefinitionKindClass},
			"VARIABLE":  &graphql.EnumValueConfig{Value: DefinitionKindVariable},
			"INTERFACE": &graphql.EnumValueConfig{Value: DefinitionKindInterface},
			"TYPE":      &graphql.EnumValueConfig{Value: DefinitionKindType},
			"ENUM":      &graphql.EnumValueConfig{Value: DefinitionKindEnum},
			"METHOD":    &graphql.EnumValueConfig{Value: DefinitionKindMethod},
			"PROPERTY":  &graphql.EnumValueConfig{Value: DefinitionKindProperty},
		},
	})

	ticketStatusEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "TicketStatus",
		Values: graphql.EnumValueConfigMap{
			"OPEN":   &graphql.EnumValueConfig{Value: TicketStatusOpen},
			"CLOSED": &graphql.EnumValueConfig{Value: TicketStatusClosed},
		},
	})

	violationPriorityEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "ViolationPriority",
		Values: graphql.EnumValueConfigMap{
			"HIGH":   &graphql.EnumValueConfig{Value: ViolationPriorityHigh},
			"MEDIUM": &graphql.EnumValueConfig{Value: ViolationPriorityMedium},
			"LOW":    &graphql.EnumValueConfig{Value: ViolationPriorityLow},
		},
	})

	var bundleType *graphql.Object
	var folderType *graphql.Object
	var fileType *graphql.Object
	var sectionType *graphql.Object
	var definitionType *graphql.Object
	var violationType *graphql.Object
	var violationKindType *graphql.Object
	var policyType *graphql.Object
	var ticketType *graphql.Object
	var contributorType *graphql.Object
	var repoType *graphql.Object

	bundleType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Bundle",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bundle := p.Source.(*Bundle)
						return bundle.GetID(), nil
					},
				},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"root":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"sourceRoot":  &graphql.Field{Type: graphql.String},
				"projectType": &graphql.Field{Type: graphql.String},
				"tags":        &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bundle := p.Source.(*Bundle)
						return "file://" + filepath.ToSlash(filepath.Join(rootDir, bundle.Root)), nil
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Folder{}, nil
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*File{}, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
			}
		}),
	})

	folderType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Folder",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":       &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":     &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":     &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"parent":   &graphql.Field{Type: folderType},
				"children": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Folder{}, nil
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*File{}, nil
					},
				},
				"bundle": &graphql.Field{Type: bundleType},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
			}
		}),
	})

	fileType = graphql.NewObject(graphql.ObjectConfig{
		Name: "File",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":        &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"extension": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"folder":    &graphql.Field{Type: folderType},
				"bundle":    &graphql.Field{Type: bundleType},
				"sections": &graphql.Field{
					Type: graphql.NewList(sectionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						file := p.Source.(*File)
						absPath := filepath.Join(rootDir, file.Path)
						if !FileExists(absPath) {
							return []*Section{}, nil
						}
						content, err := ReadTextFile(absPath)
						if err != nil {
							return nil, err
						}
						sections := ParseSections(content, file.Path)
						result := make([]*Section, len(sections))
						stack := make([]*Section, 0, len(sections))
						for i := range sections {
							sections[i].FilePath = file.Path
							sections[i].Path = sections[i].Name
							result[i] = &sections[i]
							if len(sections[i].Children) > 0 {
								stack = append(stack, result[i])
							}
						}
						for len(stack) > 0 {
							section := stack[len(stack)-1]
							stack = stack[:len(stack)-1]
							for i := range section.Children {
								child := &section.Children[i]
								child.FilePath = file.Path
								if section.Path == "" {
									child.Path = child.Name
								} else {
									child.Path = section.Path + "/" + child.Name
								}
								if len(child.Children) > 0 {
									stack = append(stack, child)
								}
							}
						}
						return result, nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewList(definitionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Definition{}, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewList(violationType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"content": &graphql.Field{Type: graphql.String},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Contributor{}, nil
					},
				},
			}
		}),
	})

	sectionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Section",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						return section.GetID(), nil
					},
				},
				"name":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file": &graphql.Field{
					Type: fileType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						if section.FilePath == "" {
							return nil, nil
						}
						normalizedPath := strings.ReplaceAll(section.FilePath, "\\", "/")
						name := filepath.Base(normalizedPath)
						ext := filepath.Ext(name)
						folderPath := filepath.Dir(normalizedPath)
						var folderID *string
						if folderPath != "." {
							id := fmt.Sprintf("folder:%s", folderPath)
							folderID = &id
						}
						return &File{
							ID:        fmt.Sprintf("file:%s", normalizedPath),
							Path:      normalizedPath,
							URI:       fmt.Sprintf("file://%s/%s", rootDir, normalizedPath),
							Name:      name,
							Extension: ext,
							FolderID:  folderID,
						}, nil
					},
				},
				"parent": &graphql.Field{Type: sectionType},
				"children": &graphql.Field{
					Type: graphql.NewList(sectionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						if len(section.Children) == 0 {
							return []*Section{}, nil
						}
						children := make([]*Section, len(section.Children))
						for i := range section.Children {
							children[i] = &section.Children[i]
						}
						return children, nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewList(definitionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Definition{}, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewList(violationType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"range": &graphql.Field{
					Type: rangeType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						return &Range{Start: section.StartIndex, End: section.EndIndex}, nil
					},
				},
			}
		}),
	})

	definitionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Definition",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						definition := p.Source.(*Definition)
						return definition.GetID(), nil
					},
				},
				"name":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"kind":    &graphql.Field{Type: graphql.NewNonNull(definitionKindEnum)},
				"file":    &graphql.Field{Type: graphql.NewNonNull(fileType)},
				"section": &graphql.Field{Type: sectionType},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"range": &graphql.Field{
					Type: graphql.NewNonNull(rangeType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						definition := p.Source.(*Definition)
						return &Range{Start: definition.StartIndex, End: definition.EndIndex}, nil
					},
				},
			}
		}),
	})

	textEditType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TextEdit",
		Fields: graphql.Fields{
			"start":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"end":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"newText": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	fileEditType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FileEdit",
		Fields: graphql.Fields{
			"path":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"edits": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(textEditType)))},
		},
	})

	autofixType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Autofix",
		Fields: graphql.Fields{
			"description": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"edits":       &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileEditType)))},
		},
	})

	violationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Violation",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"kind": &graphql.Field{
					Type: graphql.NewNonNull(violationKindType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						info := violation.Kind.Info()
						return &info, nil
					},
				},
				"scope":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file":    &graphql.Field{Type: fileType},
				"folder":  &graphql.Field{Type: folderType},
				"line":    &graphql.Field{Type: graphql.Int},
				"column":  &graphql.Field{Type: graphql.Int},
				"excerpt": &graphql.Field{Type: graphql.String},
				"autofix": &graphql.Field{Type: autofixType},
			}
		}),
	})

	violationKindType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ViolationKind",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						kind := p.Source.(*ViolationKindMeta)
						return kind.GetID(), nil
					},
				},
				"policy":      &graphql.Field{Type: graphql.NewNonNull(policyType)},
				"priority":    &graphql.Field{Type: graphql.NewNonNull(violationPriorityEnum)},
				"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"reason":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"solution":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
				},
			}
		}),
	})

	policyType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Policy",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":             &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":           &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"description":    &graphql.Field{Type: graphql.String},
				"scopes":         &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"violationKinds": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType)))},
			}
		}),
	})

	/*
	ticketIterationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketIteration",
		Fields: graphql.Fields{
			"prompt": &graphql.Field{Type: graphql.String},
			"llm":    &graphql.Field{Type: graphql.String},
			"author": &graphql.Field{Type: graphql.String},
			"date":   &graphql.Field{Type: graphql.DateTime},
			"commit": &graphql.Field{Type: graphql.String},
			// Files could be added here if needed, keeping it simple for now or matching struct
		},
	})
	*/

	ticketDateType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketDate",
		Fields: graphql.Fields{
			"created":  &graphql.Field{Type: graphql.NewNonNull(graphql.DateTime)},
			"finished": &graphql.Field{Type: graphql.DateTime},
		},
	})

	ticketType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Ticket",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.GetID(), nil
					},
				},
				"year":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"month":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"day":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"slug":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.JsonPath, nil
					},
				},
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						absPath := filepath.Join(rootDir, ticket.JsonPath)
						return "file://" + strings.ReplaceAll(absPath, "\\", "/"), nil
					},
				},
				"prompt": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.GetPrompt(), nil
					},
				},
				"summary": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return nil, nil
					},
				},
				"status": &graphql.Field{
					Type: graphql.NewNonNull(ticketStatusEnum),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						status := ticket.GetStatus()
						if status == "" || status == "open" {
							return TicketStatusOpen, nil
						}
						if status == "finished" || status == "closed" {
							return TicketStatusClosed, nil
						}
						return TicketStatusOpen, nil
					},
				},
				"author": &graphql.Field{
					Type: contributorType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						author := ticket.GetAuthor()
						if author == "" {
							return nil, nil
						}
						contributors, err := ListContributors()
						if err != nil {
							return &Contributor{Github: author, Name: author}, nil
						}
						for i := range contributors {
							if contributors[i].Github == author || contributors[i].Name == author {
								return &contributors[i], nil
							}
							for _, email := range contributors[i].Emails {
								if email == author || strings.Contains(author, email) {
									return &contributors[i], nil
								}
							}
						}
						return &Contributor{Github: author, Name: author}, nil
					},
				},
				"model": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						llm := ticket.GetLLM()
						if llm == "" {
							return nil, nil
						}
						return llm, nil
					},
				},
				"commit": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						commit := ticket.GetCommit()
						if commit == "" {
							return nil, nil
						}
						return commit, nil
					},
				},
				"date": &graphql.Field{
					Type: graphql.NewNonNull(ticketDateType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						created := ticket.GetDateCreated()
						if created.IsZero() {
							created = time.Date(ticket.Year, time.Month(ticket.Month), ticket.Day, 0, 0, 0, 0, time.UTC)
						}
						
						finished := ticket.GetDateFinished()
						return map[string]interface{}{
							"created":  created,
							"finished": finished,
						}, nil
					},
				},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
			}
		}),
	})

	contributorIconsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorIcons",
		Fields: graphql.Fields{
			"avatar":      &graphql.Field{Type: graphql.String},
			"avatarRound": &graphql.Field{Type: graphql.String},
			"github":      &graphql.Field{Type: graphql.String},
		},
	})

	contributorLinkType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorLink",
		Fields: graphql.Fields{
			"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"url":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	/*
	contributorCommitType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorCommit",
		Fields: graphql.Fields{
			"title": &graphql.Field{Type: graphql.String},
			"sha":   &graphql.Field{Type: graphql.String},
		},
	})
	*/

	contributorType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Contributor",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						contributor := p.Source.(*Contributor)
						return contributor.GetID(), nil
					},
				},
				"github":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":    &graphql.Field{Type: graphql.String},
				"emails":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"links": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorLinkType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						contributor := p.Source.(*Contributor)
						links := []ContributorLink{}
						for name, url := range contributor.Links {
							links = append(links, ContributorLink{Name: name, URL: url})
						}
						return links, nil
					},
				},
				"icons":   &graphql.Field{Type: contributorIconsType},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType)))},
			}
		}),
	})

	repoResolverInstance := &repoResolver{resolver}

	repoType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Repo",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":   &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"bundles": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Bundles(p.Context, repo)
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Folders(p.Context, repo)
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Files(p.Context, repo)
					},
				},
				"sections": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Sections(p.Context, repo)
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Definitions(p.Context, repo)
					},
				},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Contributors(p.Context, repo)
					},
				},
				"tickets": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
					Args: graphql.FieldConfigArgument{
						"year":   &graphql.ArgumentConfig{Type: graphql.Int},
						"month":  &graphql.ArgumentConfig{Type: graphql.Int},
						"day":    &graphql.ArgumentConfig{Type: graphql.Int},
						"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						var year, month, day *int
						var status *TicketStatus
						if v, ok := p.Args["year"].(int); ok {
							year = &v
						}
						if v, ok := p.Args["month"].(int); ok {
							month = &v
						}
						if v, ok := p.Args["day"].(int); ok {
							day = &v
						}
						if v, ok := p.Args["status"].(TicketStatus); ok {
							status = &v
						}
						return repoResolverInstance.Tickets(p.Context, repo, year, month, day, status)
					},
				},
				"policies": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.Policies(p.Context, repo)
					},
				},
				"violationKinds": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolverInstance.ViolationKinds(p.Context, repo)
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						var scope *string
						if v, ok := p.Args["scope"].(string); ok {
							scope = &v
						}
						return repoResolverInstance.Violations(p.Context, repo, scope)
					},
				},
			}
		}),
	})

	analyzeResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeResult",
		Fields: graphql.Fields{
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
			"metrics":    &graphql.Field{Type: graphql.NewNonNull(analyzeMetricsType)},
		},
	})

	fixResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FixResult",
		Fields: graphql.Fields{
			"fixed":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"remaining":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
		},
	})

	queryResolverInstance := &queryResolver{resolver}

	queryType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"node": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewUnion(graphql.UnionConfig{
					Name:  "Node",
					Types: []*graphql.Object{repoType, bundleType, folderType, fileType, sectionType, definitionType, contributorType, ticketType, policyType, violationKindType, violationType},
					ResolveType: func(p graphql.ResolveTypeParams) *graphql.Object {
						switch p.Value.(type) {
						case *Repo:
							return repoType
						case *Bundle:
							return bundleType
						case *Folder:
							return folderType
						case *File:
							return fileType
						case *Section:
							return sectionType
						case *Definition:
							return definitionType
						case *Contributor:
							return contributorType
						case *Ticket:
							return ticketType
						case *Policy:
							return policyType
						case *ViolationKindMeta:
							return violationKindType
						case *Violation:
							return violationType
						default:
							return nil
						}
					},
				})),
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.ID)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Node(p.Context, id)
				},
			},
			"repo": &graphql.Field{
				Type: graphql.NewNonNull(repoType),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Repo(p.Context)
				},
			},
			"bundles": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Bundles(p.Context)
				},
			},
			"folders": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Folders(p.Context)
				},
			},
			"files": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Files(p.Context)
				},
			},
			"sections": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Sections(p.Context)
				},
			},
			"definitions": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Definitions(p.Context)
				},
			},
			"contributors": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Contributors(p.Context)
				},
			},
			"tickets": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
				Args: graphql.FieldConfigArgument{
					"year":   &graphql.ArgumentConfig{Type: graphql.Int},
					"month":  &graphql.ArgumentConfig{Type: graphql.Int},
					"day":    &graphql.ArgumentConfig{Type: graphql.Int},
					"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var year, month, day *int
					var status *TicketStatus
					if y, ok := p.Args["year"].(int); ok {
						year = &y
					}
					if m, ok := p.Args["month"].(int); ok {
						month = &m
					}
					if d, ok := p.Args["day"].(int); ok {
						day = &d
					}
					if s, ok := p.Args["status"].(string); ok {
						st := TicketStatus(s)
						status = &st
					}
					return queryResolverInstance.Tickets(p.Context, year, month, day, status)
				},
			},
			"policies": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Policies(p.Context)
				},
			},
			"violationKinds": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.ViolationKinds(p.Context)
				},
			},
			"violations": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return queryResolverInstance.Violations(p.Context, scope)
				},
			},
			"bundle": &graphql.Field{
				Type: bundleType,
				Args: graphql.FieldConfigArgument{
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					name := p.Args["name"].(string)
					return queryResolverInstance.Bundle(p.Context, name)
				},
			},
			"folder": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolverInstance.Folder(p.Context, path)
				},
			},
			"file": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolverInstance.File(p.Context, path)
				},
			},
			"section": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"path":        &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"sectionPath": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					sectionPathRaw := p.Args["sectionPath"].([]interface{})
					sectionPath := make([]string, len(sectionPathRaw))
					for i, v := range sectionPathRaw {
						sectionPath[i] = v.(string)
					}
					return queryResolverInstance.Section(p.Context, path, sectionPath)
				},
			},
			"definition": &graphql.Field{
				Type: definitionType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					name := p.Args["name"].(string)
					return queryResolverInstance.Definition(p.Context, path, name)
				},
			},
			"contributor": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Contributor(p.Context, id)
				},
			},
			"ticket": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"year":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"month": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"day":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"slug":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					year := p.Args["year"].(int)
					month := p.Args["month"].(int)
					day := p.Args["day"].(int)
					slug := p.Args["slug"].(string)
					return queryResolverInstance.Ticket(p.Context, year, month, day, slug)
				},
			},
			"policy": &graphql.Field{
				Type: policyType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Policy(p.Context, id)
				},
			},
			"violationKind": &graphql.Field{
				Type: violationKindType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.ViolationKind(p.Context, id)
				},
			},
			"analyze": &graphql.Field{
				Type: graphql.NewNonNull(analyzeResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return queryResolverInstance.Analyze(p.Context, scope)
				},
			},
		},
	})

	mutationResolverInstance := &mutationResolver{resolver}

	ticketOpenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketOpenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"title":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	ticketCloseInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketCloseInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"summary": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"files":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
		},
	})

	ticketReopenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketReopenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorAddInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "ContributorAddInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"github": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"name":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"emails": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
		},
	})

	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"fix": &graphql.Field{
				Type: graphql.NewNonNull(fixResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return mutationResolverInstance.Fix(p.Context, scope)
				},
			},
			"ticketOpen": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketOpenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketOpenInput{
						Title:  inputMap["title"].(string),
						Prompt: inputMap["prompt"].(string),
						LLM:    inputMap["llm"].(string),
					}
					return mutationResolverInstance.TicketOpen(p.Context, input)
				},
			},
			"ticketClose": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketCloseInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					var files []string
					if filesRaw, ok := inputMap["files"].([]interface{}); ok {
						for _, f := range filesRaw {
							if s, ok := f.(string); ok {
								files = append(files, s)
							}
						}
					}
					input := TicketCloseInput{
						Year:    inputMap["year"].(int),
						Month:   inputMap["month"].(int),
						Day:     inputMap["day"].(int),
						Slug:    inputMap["slug"].(string),
						Summary: inputMap["summary"].(string),
						Files:   files,
					}
					return mutationResolverInstance.TicketClose(p.Context, input)
				},
			},
			"ticketReopen": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketReopenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketReopenInput{
						Year:   inputMap["year"].(int),
						Month:  inputMap["month"].(int),
						Day:    inputMap["day"].(int),
						Slug:   inputMap["slug"].(string),
						Prompt: inputMap["prompt"].(string),
						LLM:    inputMap["llm"].(string),
					}
					return mutationResolverInstance.TicketReopen(p.Context, input)
				},
			},
			"contributorAdd": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(contributorAddInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := ContributorAddInput{
						Github: inputMap["github"].(string),
					}
					if n, ok := inputMap["name"].(string); ok {
						input.Name = &n
					}
					if emails, ok := inputMap["emails"].([]interface{}); ok {
						for _, e := range emails {
							if s, ok := e.(string); ok {
								input.Emails = append(input.Emails, s)
							}
						}
					}
					return mutationResolverInstance.ContributorAdd(p.Context, input)
				},
			},
			"contributorRemove": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"github": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					github := p.Args["github"].(string)
					return mutationResolverInstance.ContributorRemove(p.Context, github)
				},
			},
			"folderCreate": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FolderCreate(p.Context, path)
				},
			},
			"folderMove": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolverInstance.FolderMove(p.Context, src, dst)
				},
			},
			"folderDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FolderDelete(p.Context, path)
				},
			},
			"fileCreate": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FileCreate(p.Context, path)
				},
			},
			"fileMove": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolverInstance.FileMove(p.Context, src, dst)
				},
			},
			"fileDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FileDelete(p.Context, path)
				},
			},
			"sectionCreate": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"parent": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					var parent *string
					if par, ok := p.Args["parent"].(string); ok {
						parent = &par
					}
					return mutationResolverInstance.SectionCreate(p.Context, file, name, parent)
				},
			},
			"sectionMove": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"oldName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"newName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					oldName := p.Args["oldName"].(string)
					newName := p.Args["newName"].(string)
					return mutationResolverInstance.SectionMove(p.Context, file, oldName, newName)
				},
			},
			"sectionDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"file": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					return mutationResolverInstance.SectionDelete(p.Context, file, name)
				},
			},
		},
	})

	_ = rangeType
	_ = countMetricsType

	return graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})
}

// #endregion Schema Builder

// #region Query Resolvers

func (r *Resolver) Query() QueryResolver {
	return &queryResolver{r}
}

type queryResolver struct{ *Resolver }

func (r *queryResolver) Node(ctx context.Context, id string) (Node, error) {
	parts := strings.SplitN(id, ":", 2)
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid node id format: %s", id)
	}
	kind, nodeID := parts[0], parts[1]
	switch kind {
	case "repo":
		return r.Repo(ctx)
	case "bundle":
		return r.Bundle(ctx, nodeID)
	case "folder":
		return r.Folder(ctx, nodeID)
	case "file":
		return r.File(ctx, nodeID)
	case "contributor":
		return r.Contributor(ctx, nodeID)
	case "policy":
		return r.Policy(ctx, nodeID)
	case "violationKind":
		return r.ViolationKind(ctx, nodeID)
	default:
		return nil, fmt.Errorf("unknown node kind: %s", kind)
	}
}

func (r *queryResolver) Repo(ctx context.Context) (*Repo, error) {
	return &Repo{
		ID:   "repo:semio",
		Name: "semio",
		Path: r.RootDir,
	}, nil
}

func (r *queryResolver) Bundles(ctx context.Context) ([]*Bundle, error) {
	if r.Ctx != nil {
		return r.Ctx.GetBundles(), nil
	}
	return []*Bundle{}, nil
}

func (r *queryResolver) Folders(ctx context.Context) ([]*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFolders(), nil
	}
	return []*Folder{}, nil
}

func (r *queryResolver) Files(ctx context.Context) ([]*File, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFiles(), nil
	}
	return []*File{}, nil
}

func (r *queryResolver) Sections(ctx context.Context) ([]*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.GetSections(), nil
	}
	return []*Section{}, nil
}

func (r *queryResolver) Definitions(ctx context.Context) ([]*Definition, error) {
	if r.Ctx != nil {
		return r.Ctx.GetDefinitions(), nil
	}
	return []*Definition{}, nil
}

func (r *queryResolver) Contributors(ctx context.Context) ([]*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.GetContributors()
	}
	return []*Contributor{}, nil
}

func (r *queryResolver) Tickets(ctx context.Context, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.GetTickets(year, month, day, status)
	}
	return []*Ticket{}, nil
}

func (r *queryResolver) Policies(ctx context.Context) ([]*Policy, error) {
	if r.Ctx != nil {
		return r.Ctx.GetPolicies(), nil
	}
	return []*Policy{}, nil
}

func (r *queryResolver) ViolationKinds(ctx context.Context) ([]*ViolationKindMeta, error) {
	if r.Ctx != nil {
		return r.Ctx.GetViolationKinds(), nil
	}
	return []*ViolationKindMeta{}, nil
}

func (r *queryResolver) Violations(ctx context.Context, scope *string) ([]*Violation, error) {
	if r.Ctx != nil {
		result, err := r.Ctx.Analyze(scope)
		if err != nil {
			return nil, err
		}
		return result.Violations, nil
	}
	return []*Violation{}, nil
}

func (r *queryResolver) Bundle(ctx context.Context, name string) (*Bundle, error) {
	if r.Ctx != nil {
		bundles := r.Ctx.GetBundles()
		for _, b := range bundles {
			if b.Name == name {
				return b, nil
			}
		}
	}
	return &Bundle{
		Name: name,
		Tags: []string{},
	}, nil
}

func (r *queryResolver) Folder(ctx context.Context, path string) (*Folder, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	return &Folder{
		ID:       buildFolderID(normalizedPath, bundleID),
		Path:     normalizedPath,
		URI:      fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name:     name,
		BundleID: bundleID,
	}, nil
}

func (r *queryResolver) File(ctx context.Context, path string) (*File, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := "@semio/" + bundleName
		bundleID = &id
	}
	
	var folderID *string
	if folderPath != "." {
		id := buildFolderID(folderPath, bundleID)
		folderID = &id
	}
	
	return &File{
		ID:        buildFileID(normalizedPath, bundleID),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
		BundleID:  bundleID,
	}, nil
}

func (r *queryResolver) Section(ctx context.Context, path string, sectionPath []string) (*Section, error) {
	sectionName := strings.Join(sectionPath, "#")
	return &Section{
		Name: sectionName,
	}, nil
}

func (r *queryResolver) Definition(ctx context.Context, path string, name string) (*Definition, error) {
	return &Definition{
		Name: name,
		Kind: DefinitionKindFunction,
	}, nil
}

func (r *queryResolver) Contributor(ctx context.Context, id string) (*Contributor, error) {
	if r.Ctx != nil {
		contributors, err := r.Ctx.GetContributors()
		if err == nil {
			for _, c := range contributors {
				if c.Github == id {
					return c, nil
				}
			}
		}
	}
	return &Contributor{
		Github: id,
		Emails: []string{},
		Links:  map[string]string{},
	}, nil
}

func (r *queryResolver) Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error) {
	if r.Ctx != nil {
		y, m, d := year, month, day
		tickets, err := r.Ctx.GetTickets(&y, &m, &d, nil)
		if err == nil {
			for _, t := range tickets {
				if t.Slug == slug {
					return t, nil
				}
			}
		}
	}
	return &Ticket{
		Year:  year,
		Month: month,
		Day:   day,
		Slug:  slug,
	}, nil
}

func (r *queryResolver) Policy(ctx context.Context, id string) (*Policy, error) {
	if r.Ctx != nil {
		policies := r.Ctx.GetPolicies()
		for _, p := range policies {
			if p.Name == id {
				return p, nil
			}
		}
	}
	return &Policy{
		ID:     "@semio/policies/" + id,
		Name:   id,
		Scopes: []string{},
	}, nil
}

func (r *queryResolver) ViolationKind(ctx context.Context, id string) (*ViolationKindMeta, error) {
	if r.Ctx != nil {
		kinds := r.Ctx.GetViolationKinds()
		for _, k := range kinds {
			if string(k.Kind) == id {
				return k, nil
			}
		}
	}
	return &ViolationKindMeta{
		Kind:        ViolationKind(id),
		Priority:    ViolationPriorityMedium,
		Autofixable: false,
		Reason:      "",
		Solution:    "",
	}, nil
}

func (r *queryResolver) Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Analyze(scope)
	}
	return &AnalyzeResult{
		Violations: []*Violation{},
		Metrics: &AnalyzeMetrics{
			Total:       0,
			ByPriority:  &PriorityCount{High: 0, Medium: 0, Low: 0},
			Autofixable: 0,
		},
	}, nil
}

// #endregion Query Resolvers

// #region Mutation Resolvers

func (r *Resolver) Mutation() MutationResolver {
	return &mutationResolver{r}
}

type mutationResolver struct{ *Resolver }

func (r *mutationResolver) Fix(ctx context.Context, scope *string) (*FixResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Fix(scope)
	}
	return &FixResult{
		Fixed:      0,
		Remaining:  0,
		Violations: []*Violation{},
	}, nil
}

func (r *mutationResolver) TicketOpen(ctx context.Context, input TicketOpenInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketOpen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketClose(ctx context.Context, input TicketCloseInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketClose(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketReopen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.ContributorAdd(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorRemove(ctx context.Context, github string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.ContributorRemove(github)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderCreate(ctx context.Context, path string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderMove(ctx context.Context, src string, dst string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FolderDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileCreate(ctx context.Context, path string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileMove(ctx context.Context, src string, dst string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FileDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionCreate(file, name, parent)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionMove(file, oldName, newName)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionDelete(ctx context.Context, file string, name string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.SectionDelete(file, name)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

// #endregion Mutation Resolvers

// #region Entity Resolvers

type repoResolver struct{ *Resolver }

func (r *Resolver) Repo_() RepoResolver {
	return &repoResolver{r}
}

func (r *repoResolver) Bundles(ctx context.Context, obj *Repo) ([]*Bundle, error) {
	if r.Ctx != nil {
		return r.Ctx.GetBundles(), nil
	}
	return []*Bundle{}, nil
}

func (r *repoResolver) Folders(ctx context.Context, obj *Repo) ([]*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFolders(), nil
	}
	return []*Folder{}, nil
}

func (r *repoResolver) Files(ctx context.Context, obj *Repo) ([]*File, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFiles(), nil
	}
	return []*File{}, nil
}

func (r *repoResolver) Sections(ctx context.Context, obj *Repo) ([]*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.GetSections(), nil
	}
	return []*Section{}, nil
}

func (r *repoResolver) Definitions(ctx context.Context, obj *Repo) ([]*Definition, error) {
	if r.Ctx != nil {
		return r.Ctx.GetDefinitions(), nil
	}
	return []*Definition{}, nil
}

func (r *repoResolver) Contributors(ctx context.Context, obj *Repo) ([]*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.GetContributors()
	}
	return []*Contributor{}, nil
}

func (r *repoResolver) Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.GetTickets(year, month, day, status)
	}
	return []*Ticket{}, nil
}

func (r *repoResolver) Policies(ctx context.Context, obj *Repo) ([]*Policy, error) {
	if r.Ctx != nil {
		return r.Ctx.GetPolicies(), nil
	}
	return []*Policy{}, nil
}

func (r *repoResolver) ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKindMeta, error) {
	if r.Ctx != nil {
		return r.Ctx.GetViolationKinds(), nil
	}
	return []*ViolationKindMeta{}, nil
}

func (r *repoResolver) Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error) {
	if r.Ctx != nil {
		result, err := r.Ctx.Analyze(scope)
		if err != nil {
			return nil, err
		}
		return result.Violations, nil
	}
	return []*Violation{}, nil
}

// #endregion Entity Resolvers

// #region Resolver Interfaces

type QueryResolver interface {
	Node(ctx context.Context, id string) (Node, error)
	Repo(ctx context.Context) (*Repo, error)
	Bundles(ctx context.Context) ([]*Bundle, error)
	Folders(ctx context.Context) ([]*Folder, error)
	Files(ctx context.Context) ([]*File, error)
	Contributors(ctx context.Context) ([]*Contributor, error)
	Tickets(ctx context.Context, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error)
	Policies(ctx context.Context) ([]*Policy, error)
	ViolationKinds(ctx context.Context) ([]*ViolationKindMeta, error)
	Violations(ctx context.Context, scope *string) ([]*Violation, error)
	Bundle(ctx context.Context, name string) (*Bundle, error)
	Folder(ctx context.Context, path string) (*Folder, error)
	File(ctx context.Context, path string) (*File, error)
	Section(ctx context.Context, path string, sectionPath []string) (*Section, error)
	Definition(ctx context.Context, path string, name string) (*Definition, error)
	Contributor(ctx context.Context, id string) (*Contributor, error)
	Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error)
	Policy(ctx context.Context, id string) (*Policy, error)
	ViolationKind(ctx context.Context, id string) (*ViolationKindMeta, error)
	Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error)
}

type MutationResolver interface {
	Fix(ctx context.Context, scope *string) (*FixResult, error)
	TicketOpen(ctx context.Context, input TicketOpenInput) (*Ticket, error)
	TicketClose(ctx context.Context, input TicketCloseInput) (*Ticket, error)
	TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error)
	ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error)
	ContributorRemove(ctx context.Context, github string) (bool, error)
	FolderCreate(ctx context.Context, path string) (*Folder, error)
	FolderMove(ctx context.Context, src string, dst string) (*Folder, error)
	FolderDelete(ctx context.Context, path string) (bool, error)
	FileCreate(ctx context.Context, path string) (*File, error)
	FileMove(ctx context.Context, src string, dst string) (*File, error)
	FileDelete(ctx context.Context, path string) (bool, error)
	SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error)
	SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error)
	SectionDelete(ctx context.Context, file string, name string) (bool, error)
}

type RepoResolver interface {
	Bundles(ctx context.Context, obj *Repo) ([]*Bundle, error)
	Folders(ctx context.Context, obj *Repo) ([]*Folder, error)
	Files(ctx context.Context, obj *Repo) ([]*File, error)
	Contributors(ctx context.Context, obj *Repo) ([]*Contributor, error)
	Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error)
	Policies(ctx context.Context, obj *Repo) ([]*Policy, error)
	ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKindMeta, error)
	Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error)
}

// #endregion Resolver Interfaces
