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
package main

import (
	"bufio"
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"math/rand"
	"net/http"
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
	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
	"github.com/spf13/cobra"
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
	DefinitionKindPort DefinitionKind = "interface"
	DefinitionKindType      DefinitionKind = "type"
	DefinitionKindEnum      DefinitionKind = "enum"
	DefinitionKindMethod    DefinitionKind = "method"
	DefinitionKindProperty  DefinitionKind = "property"
)

func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindFunction, DefinitionKindClass, DefinitionKindVariable,
		DefinitionKindPort, DefinitionKindType, DefinitionKindEnum,
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
	"opus-4-5",
	"claude-opus-4",
	"sonnet-4-5",
	"claude-sonnet-4",
	"haiku-4-5",
	"gemini-3-pro",
	"gemini-3-flash",
	"gpt-5-2",
	"gpt-5-2-codex",
	"gpt-5-mini",
}

var AllowedUIs = []string{
	"copilot-chat",
	"antigravity",
	"cursor",
	"claude-code",
	"codex",
	"droid",
}

func NormalizeLLMSlug(llm string) string {
	return strings.ToLower(Slugify(llm))
}

func NormalizeUISlug(ui string) string {
	return strings.ToLower(Slugify(ui))
}

func ResolveAllowedLLM(llm string) (string, error) {
	llmSlug := NormalizeLLMSlug(llm)
	bestMatch := ""
	for _, allowed := range AllowedLLMs {
		if strings.HasPrefix(llmSlug, allowed) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("llm '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMs, ", "))
	}
	return bestMatch, nil
}

func ResolveAllowedUI(ui string) (string, error) {
	uiSlug := NormalizeUISlug(ui)
	bestMatch := ""
	for _, allowed := range AllowedUIs {
		if strings.HasPrefix(uiSlug, allowed) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("ui '%s' is not allowed. Please use one of: %s", uiSlug, strings.Join(AllowedUIs, ", "))
	}
	return bestMatch, nil
}

type Range struct {
	Start int `json:"start"`
	End   int `json:"end"`
}

type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
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

type AnalyzeMetrics struct {
	Total       int            `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int            `json:"autofixable"`
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
func (b *Bundle) GetID() string { return normalizeBundleID(b.Name) }

func normalizeBundleLabel(name string) string {
	if name == "" {
		return ""
	}
	if strings.HasPrefix(name, "@semio/") || strings.HasPrefix(name, "@semio-repo/") || name == "@semio-repo" {
		return name
	}
	if name == "vscode" {
		return "@semio-repo/vscode"
	}
	if name == "repo" {
		return "@semio-repo"
	}
	return "@semio/" + name
}

func normalizeBundleID(name string) string {
	return normalizeBundleLabel(name)
}

func bundlePathPrefix(name string) string {
	if name == "" {
		return ""
	}
	if name == "@semio-repo" {
		return ""
	}
	if strings.HasPrefix(name, "@semio/") {
		return strings.TrimPrefix(name, "@semio/") + "/"
	}
	return name + "/"
}

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

func (s *Section) IsNode() {}
func (s *Section) GetID() string {
	if s.FilePath != "" && s.Path != "" {
		return s.FilePath + "#" + s.Path
	}
	return "section:" + s.Name
}

type Definition struct {
	Name        string         `json:"name"`
	Kind        DefinitionKind `json:"kind"`
	FilePath    string         `json:"filePath,omitempty"`
	SectionPath string         `json:"sectionPath,omitempty"`
	StartLine   int            `json:"startLine"`
	EndLine     int            `json:"endLine"`
	StartIndex  int            `json:"startIndex"`
	EndIndex    int            `json:"endIndex"`
}

func (d *Definition) IsNode() {}
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
	Github        string                          `yaml:"github" json:"github"`
	Name          string                          `yaml:"name,omitempty" json:"name,omitempty"`
	Emails        []string                        `yaml:"emails,omitempty" json:"emails,omitempty"`
	Links         map[string]string               `yaml:"links,omitempty" json:"links,omitempty"`
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
	Year        int         `json:"year"`
	Month       int         `json:"month"`
	Day         int         `json:"day"`
	Slug        string      `json:"slug"`
	Data        *TicketData `json:"data,omitempty"`
	FolderPath  string      `json:"folderPath"`
	JsonPath    string      `json:"jsonPath,omitempty"`
	PlanPath    string      `json:"planPath,omitempty"`
	TicketPath  string      `json:"ticketPath,omitempty"`
}

func (t *Ticket) IsNode() {}
func (t *Ticket) GetID() string {
	return fmt.Sprintf("@semio/tickets/%d/%02d/%02d/%s", t.Year, t.Month, t.Day, t.Slug)
}

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

func (t *Ticket) GetLatestPrompt() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[len(t.Data.Iterations)-1].Prompt
	}
	return ""
}

func (t *Ticket) GetLLM() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[len(t.Data.Iterations)-1].LLM
	}
	return ""
}

func (t *Ticket) GetUI() string {
	if t.Data != nil && len(t.Data.Iterations) > 0 {
		return t.Data.Iterations[len(t.Data.Iterations)-1].UI
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

func (t *Ticket) GetFiles() *TicketDiffs {
	result := newTicketDiffs()
	if t.Data == nil {
		return result
	}
	for _, iteration := range t.Data.Iterations {
		if iteration.Diff == nil {
			continue
		}
		result.Bundles.Added = append(result.Bundles.Added, iteration.Diff.Bundles.Added...)
		result.Bundles.Modified = append(result.Bundles.Modified, iteration.Diff.Bundles.Modified...)
		result.Bundles.Deleted = append(result.Bundles.Deleted, iteration.Diff.Bundles.Deleted...)
		result.Bundles.Renamed = append(result.Bundles.Renamed, iteration.Diff.Bundles.Renamed...)
		result.Folders.Added = append(result.Folders.Added, iteration.Diff.Folders.Added...)
		result.Folders.Modified = append(result.Folders.Modified, iteration.Diff.Folders.Modified...)
		result.Folders.Deleted = append(result.Folders.Deleted, iteration.Diff.Folders.Deleted...)
		result.Folders.Renamed = append(result.Folders.Renamed, iteration.Diff.Folders.Renamed...)
		result.Files.Added = append(result.Files.Added, iteration.Diff.Files.Added...)
		result.Files.Modified = append(result.Files.Modified, iteration.Diff.Files.Modified...)
		result.Files.Deleted = append(result.Files.Deleted, iteration.Diff.Files.Deleted...)
		result.Files.Renamed = append(result.Files.Renamed, iteration.Diff.Files.Renamed...)
		result.Sections.Added = append(result.Sections.Added, iteration.Diff.Sections.Added...)
		result.Sections.Modified = append(result.Sections.Modified, iteration.Diff.Sections.Modified...)
		result.Sections.Deleted = append(result.Sections.Deleted, iteration.Diff.Sections.Deleted...)
		result.Sections.Renamed = append(result.Sections.Renamed, iteration.Diff.Sections.Renamed...)
		result.Definitions.Added = append(result.Definitions.Added, iteration.Diff.Definitions.Added...)
		result.Definitions.Modified = append(result.Definitions.Modified, iteration.Diff.Definitions.Modified...)
		result.Definitions.Deleted = append(result.Definitions.Deleted, iteration.Diff.Definitions.Deleted...)
		result.Definitions.Renamed = append(result.Definitions.Renamed, iteration.Diff.Definitions.Renamed...)
	}
	return result
}

type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

type TicketFileContrib struct {
	FileID   string                 `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}

type Policy struct {
	ID             string               `json:"id"`
	Name           string               `json:"name"`
	Description    *string              `json:"description,omitempty"`
	Scopes         []string             `json:"scopes"`
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

func (v *ViolationKindMeta) IsNode() {}
func (v *ViolationKindMeta) GetID() string {
	return "@semio/policies/" + v.PolicyID + "/violations/" + string(v.Kind)
}

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

	SectionID string       `json:"sectionId"`
	Metrics   *LineMetrics `json:"metrics"`
}

type ContributionDefinition struct {
	DefinitionID string       `json:"definitionId"`
	Metrics      *LineMetrics `json:"metrics"`
}

type SemanticChangeType string

const (
	SemanticChangeAdded    SemanticChangeType = "added"
	SemanticChangeDeleted  SemanticChangeType = "deleted"
	SemanticChangeModified SemanticChangeType = "modified"
	SemanticChangeRenamed  SemanticChangeType = "renamed"
)

type SemanticChange struct {
	Kind     string
	Status   SemanticChangeType
	Path     string
	FromPath string
	ToPath   string
	Lines    LineMetrics
}

func newTicketDiffSet() TicketDiffSet {
	return TicketDiffSet{
		Deleted:  []TicketFile{},
		Renamed:  []TicketFileRenamed{},
		Modified: []TicketFile{},
		Added:    []TicketFile{},
	}
}

func newTicketDiffs() *TicketDiffs {
	return &TicketDiffs{
		Bundles:     newTicketDiffSet(),
		Folders:     newTicketDiffSet(),
		Files:       newTicketDiffSet(),
		Sections:    newTicketDiffSet(),
		Definitions: newTicketDiffSet(),
	}
}

func addTicketDiffEntry(set *TicketDiffSet, change SemanticChange) {
	lines := &LineMetrics{Added: change.Lines.Added, Removed: change.Lines.Removed}
	switch change.Status {
	case SemanticChangeAdded:
		set.Added = append(set.Added, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeDeleted:
		set.Deleted = append(set.Deleted, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeRenamed:
		set.Renamed = append(set.Renamed, TicketFileRenamed{From: change.FromPath, To: change.ToPath, Lines: lines})
	case SemanticChangeModified:
		set.Modified = append(set.Modified, TicketFile{Path: change.Path, Lines: lines})
	}
}

func mergeLineMetrics(target *LineMetrics, add LineMetrics) {
	if target == nil {
		return
	}
	target.Added += add.Added
	target.Removed += add.Removed
}

func computeLineMetricsForDiff(diff *DiffLines, baseCommit, filePath string) LineMetrics {
	if diff == nil {
		return LineMetrics{}
	}
	if len(diff.Added) > 0 && len(diff.Removed) == 0 {
		return LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), filePath)), Removed: 0}
	}
	if len(diff.Removed) > 0 && len(diff.Added) == 0 {
		return LineMetrics{Added: 0, Removed: CountLinesAtCommit(baseCommit, filePath)}
	}
	return LineMetrics{Added: len(diff.Added), Removed: len(diff.Removed)}
}

func buildCodebasePathSet(codebase *Codebase) map[string]struct{} {
	result := make(map[string]struct{})
	if codebase == nil {
		return result
	}
	for _, bundle := range codebase.Bundles {
		result[bundle.ID] = struct{}{}
	}
	for _, folder := range codebase.Folders {
		result[folder.Path] = struct{}{}
	}
	for _, file := range codebase.Files {
		result[file.Path] = struct{}{}
	}
	for _, section := range codebase.Sections {
		result[section.Path] = struct{}{}
	}
	for _, def := range codebase.Definitions {
		result[def.Path] = struct{}{}
	}
	return result
}

func buildFolderLineTotals(files []string, baseCommit string) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	for _, file := range files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		currentTotals[folder] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[folder] += CountLinesAtCommit(baseCommit, file)
	}
	return currentTotals, baseTotals
}

func buildBundleLineTotals(files []string, baseCommit string, bundles []Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	for _, file := range files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ResolveBundleForPath(file, bundles)
		if bundleName == "" {
			bundleName = "@semio-repo"
		}
		currentTotals[bundleName] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[bundleName] += CountLinesAtCommit(baseCommit, file)
	}
	return currentTotals, baseTotals
}

func extractFilePrefix(path string) string {
	if path == "" {
		return ""
	}
	if idx := strings.Index(path, "#"); idx != -1 {
		return path[:idx]
	}
	if idx := strings.Index(path, "§"); idx != -1 {
		return path[:idx]
	}
	return path
}

func reconcileRenamePairs(diffSet *TicketDiffSet, matchKey func(path string) string) {
	if diffSet == nil {
		return
	}
	usedAdded := make(map[int]struct{})
	var renamed []TicketFileRenamed
	var remainingDeleted []TicketFile
	for _, del := range diffSet.Deleted {
		matchIndex := -1
		key := matchKey(del.Path)
		for i, add := range diffSet.Added {
			if _, ok := usedAdded[i]; ok {
				continue
			}
			if key != "" && matchKey(add.Path) != key {
				continue
			}
			removedLines := 0
			if del.Lines != nil {
				removedLines = del.Lines.Removed
			}
			addedLines := 0
			if add.Lines != nil {
				addedLines = add.Lines.Added
			}
			if removedLines > 0 && addedLines > 0 && removedLines != addedLines {
				continue
			}
			matchIndex = i
			break
		}
		if matchIndex == -1 {
			remainingDeleted = append(remainingDeleted, del)
			continue
		}
		add := diffSet.Added[matchIndex]
		usedAdded[matchIndex] = struct{}{}
		lines := &LineMetrics{}
		if add.Lines != nil {
			lines.Added = add.Lines.Added
		}
		if del.Lines != nil {
			lines.Removed = del.Lines.Removed
		}
		renamed = append(renamed, TicketFileRenamed{From: del.Path, To: add.Path, Lines: lines})
	}
	var remainingAdded []TicketFile
	for i, add := range diffSet.Added {
		if _, ok := usedAdded[i]; ok {
			continue
		}
		remainingAdded = append(remainingAdded, add)
	}
	diffSet.Added = remainingAdded
	diffSet.Deleted = remainingDeleted
	diffSet.Renamed = append(diffSet.Renamed, renamed...)
}

func buildSectionDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines) TicketDiffSet {
	result := newTicketDiffSet()
	currentSectionMap := make(map[string]CodebaseSection)
	baseSectionMap := make(map[string]CodebaseSection)
	for _, section := range currentCodebase.Sections {
		currentSectionMap[section.Path] = section
	}
	for _, section := range baseCodebase.Sections {
		baseSectionMap[section.Path] = section
	}

	for path := range currentSectionMap {
		if _, ok := baseSectionMap[path]; !ok {
			lines := 0
			if currentSectionMap[path].Metrics != nil {
				lines = currentSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseSectionMap {
		if _, ok := currentSectionMap[path]; !ok {
			lines := 0
			if baseSectionMap[path].Metrics != nil {
				lines = baseSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCommit(baseCommit, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
		if lang == nil || !lang.SupportsSections() {
			continue
		}
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		addedMap := computeSectionLineMap(currentSections, diff.Added, "")
		removedMap := computeSectionLineMap(baseSections, diff.Removed, "")
		for sectionPath, addedLines := range addedMap {
			removedLines := removedMap[sectionPath]
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: filePath + "#" + sectionPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for sectionPath, removedLines := range removedMap {
			if _, ok := addedMap[sectionPath]; ok {
				continue
			}
			if len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: filePath + "#" + sectionPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

func buildDefinitionDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines) TicketDiffSet {
	result := newTicketDiffSet()
	currentDefMap := make(map[string]CodebaseDefinition)
	baseDefMap := make(map[string]CodebaseDefinition)
	for _, def := range currentCodebase.Definitions {
		currentDefMap[def.Path] = def
	}
	for _, def := range baseCodebase.Definitions {
		baseDefMap[def.Path] = def
	}

	for path := range currentDefMap {
		if _, ok := baseDefMap[path]; !ok {
			lines := 0
			if currentDefMap[path].Metrics != nil {
				lines = currentDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseDefMap {
		if _, ok := currentDefMap[path]; !ok {
			lines := 0
			if baseDefMap[path].Metrics != nil {
				lines = baseDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCommit(baseCommit, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		currentLines := strings.Split(content, "\n")
		baseLines := strings.Split(baseContent, "\n")
		currentDefs := lang.ParseDefinitions(content, currentLines)
		baseDefs := lang.ParseDefinitions(baseContent, baseLines)
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		for _, def := range currentDefs {
			addedLines := computeLinesInRange(diff.Added, def.Start, def.End)
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(currentSections, def.Start, def.End, "")
			defPath := filePath + "§" + def.Name
			if sectionPath != "" {
				defPath = filePath + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for _, def := range baseDefs {
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(baseSections, def.Start, def.End, "")
			defPath := filePath + "§" + def.Name
			if sectionPath != "" {
				defPath = filePath + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

func BuildSemanticDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines, diffStatuses []GitDiffStatus, bundles []Bundle) *TicketDiffs {
	result := newTicketDiffs()
	currentFiles := []string{}
	if currentCodebase != nil {
		for _, file := range currentCodebase.Files {
			currentFiles = append(currentFiles, file.Path)
		}
	}
	baseFiles := []string{}
	if baseCodebase != nil {
		for _, file := range baseCodebase.Files {
			baseFiles = append(baseFiles, file.Path)
		}
	}
	currentFolderLines, _ := buildFolderLineTotals(currentFiles, baseCommit)
	_, baseFolderLines := buildFolderLineTotals(baseFiles, baseCommit)
	currentBundleLines, _ := buildBundleLineTotals(currentFiles, baseCommit, bundles)
	_, baseBundleLines := buildBundleLineTotals(baseFiles, baseCommit, bundles)

	currentBundleMap := make(map[string]CodebaseBundle)
	baseBundleMap := make(map[string]CodebaseBundle)
	for _, bundle := range currentCodebase.Bundles {
		currentBundleMap[bundle.ID] = bundle
	}
	for _, bundle := range baseCodebase.Bundles {
		baseBundleMap[bundle.ID] = bundle
	}
	for id, bundle := range currentBundleMap {
		if _, ok := baseBundleMap[id]; !ok {
			lines := LineMetrics{Added: currentBundleLines[id], Removed: 0}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeAdded, Path: normalizeBundleLabel(id), Lines: lines})
		} else if bundle.Metrics != nil {
			baseBundle := baseBundleMap[id]
			if baseBundle.Metrics != nil {
				added := currentBundleLines[id] - baseBundleLines[id]
				removed := 0
				if added < 0 {
					removed = -added
					added = 0
				}
				if added > 0 || removed > 0 {
					addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeModified, Path: normalizeBundleLabel(id), Lines: LineMetrics{Added: added, Removed: removed}})
				}
			}
		}
	}
	for id := range baseBundleMap {
		if _, ok := currentBundleMap[id]; !ok {
			lines := LineMetrics{Added: 0, Removed: baseBundleLines[id]}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeDeleted, Path: normalizeBundleLabel(id), Lines: lines})
		}
	}

	currentFolderMap := make(map[string]CodebaseFolder)
	baseFolderMap := make(map[string]CodebaseFolder)
	for _, folder := range currentCodebase.Folders {
		currentFolderMap[folder.Path] = folder
	}
	for _, folder := range baseCodebase.Folders {
		baseFolderMap[folder.Path] = folder
	}
	for path := range currentFolderMap {
		if _, ok := baseFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: currentFolderLines[path]}})
		}
	}
	for path := range baseFolderMap {
		if _, ok := currentFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: baseFolderLines[path]}})
		}
	}

	for _, status := range diffStatuses {
		if status.Status == "renamed" {
			fromFolder := NormalizePath(filepath.Dir(status.From))
			toFolder := NormalizePath(filepath.Dir(status.To))
			if fromFolder != toFolder && fromFolder != "." && toFolder != "." {
				addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeRenamed, FromPath: fromFolder, ToPath: toFolder, Lines: LineMetrics{Added: currentFolderLines[toFolder], Removed: baseFolderLines[fromFolder]}})
			}
			addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: SemanticChangeRenamed, FromPath: status.From, ToPath: status.To, Lines: LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), status.To)), Removed: CountLinesAtCommit(baseCommit, status.From)}})
		}
	}

	for filePath, diff := range diffLines {
		metrics := computeLineMetricsForDiff(diff, baseCommit, filePath)
		status := SemanticChangeModified
		if len(diff.Added) > 0 && len(diff.Removed) == 0 {
			status = SemanticChangeAdded
		} else if len(diff.Removed) > 0 && len(diff.Added) == 0 {
			status = SemanticChangeDeleted
		}
		addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: status, Path: filePath, Lines: metrics})
	}

	reconcileRenamePairs(&result.Bundles, func(path string) string {
		return path
	})

	result.Sections = buildSectionDiffs(baseCodebase, currentCodebase, baseCommit, diffLines)
	result.Definitions = buildDefinitionDiffs(baseCodebase, currentCodebase, baseCommit, diffLines)

	return result
}

// #region GraphQL Input Types

type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

type TicketOpenInput struct {
	Title    string `json:"title"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	UI       string `json:"ui"`
	NoIssue  bool   `json:"noIssue,omitempty"`
	PlanPath string `json:"planPath,omitempty"`
}

type TicketCloseInput struct {
	Year    int      `json:"year"`
	Month   int      `json:"month"`
	Day     int      `json:"day"`
	Slug    string   `json:"slug"`
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Title   *string  `json:"title,omitempty"`
}

type TicketReopenInput struct {
	Year   int     `json:"year"`
	Month  int     `json:"month"`
	Day    int     `json:"day"`
	Slug   string  `json:"slug"`
	Prompt string  `json:"prompt"`
	LLM    string  `json:"llm"`
	Title  *string `json:"title,omitempty"`
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

type Violation struct {
	ID      string        `json:"id"`
	Summary string        `json:"summary"`
	Kind    ViolationKind `json:"kind"`
	Scope   string        `json:"scope"`
	Line    int           `json:"line,omitempty"`
	Column  int           `json:"column,omitempty"`
	Excerpt string        `json:"excerpt,omitempty"`
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
	ExtractImports(content string) ([]string, string)
	FormatImports(imports []string) string
	ExtractPackage(content string) (string, string)
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

func (l *BaseLanguage) Name() string              { return l.name }
func (l *BaseLanguage) Extensions() []string      { return l.extensions }
func (l *BaseLanguage) CommentPrefix() string     { return l.commentPrefix }
func (l *BaseLanguage) UsesIndentScoping() bool   { return l.usesIndentScoping }
func (l *BaseLanguage) SupportsSections() bool    { return l.sectionStart != nil }
func (l *BaseLanguage) SupportsDefinitions() bool { return l.definitionRegexp != nil }
func (l *BaseLanguage) SupportsComments() bool    { return l.commentPrefix != "" }
func (l *BaseLanguage) SupportsHeaders() bool     { return l.headerFmt != "" }

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

func (l *BaseLanguage) ExtractImports(content string) ([]string, string) {
	return []string{}, content
}

func (l *BaseLanguage) FormatImports(imports []string) string {
	return ""
}

func (l *BaseLanguage) ExtractPackage(content string) (string, string) {
	return "", content
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
	inlineCommentActive := false
	for i, line := range lines {
		lineNum := i + 1
		// Skip comments inside header section
		if headerSection != nil && lineNum >= headerSection.StartLine && lineNum <= headerSection.EndLine {
			charIndex += len(line) + 1
			continue
		}
		lineStart := charIndex
		j := 0
		foundInline := false
		for j < len(line) {
			if scanState.InBlockComment {
				if j+1 < len(line) && line[j] == '*' && line[j+1] == '/' {
					if scanState.BlockCommentIsJsDoc {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("JSDoc comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentJSDoc,
							file, scanState.BlockCommentStartLine, ""))
					} else {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentBlock,
							file, scanState.BlockCommentStartLine, ""))
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
					foundInline = true
					if !inlineCommentActive {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
							ViolationCodeCommentInline,
							file, lineNum, strings.TrimSpace(line[j:])))
						inlineCommentActive = true
					}
				}
				break
			}
			j++
		}
		if !foundInline {
			if strings.TrimSpace(line) != "" {
				inlineCommentActive = false
			}
		}
		charIndex += len(line) + 1
	}
	return violations
}

// #endregion TypeScript

func (l *TypeScriptLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") {
			currentImport := line
			if !strings.Contains(line, ";") {
				for j := i + 1; j < len(lines); j++ {
					currentImport += "\n" + lines[j]
					if strings.Contains(lines[j], ";") {
						i = j
						break
					}
				}
			}
			imports = append(imports, currentImport)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *TypeScriptLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	return strings.Join(uniqueImports, "\n")
}

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

func (l *GoLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	inImportBlock := false
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		if inImportBlock {
			if trimmed == ")" {
				inImportBlock = false
			} else if trimmed != "" {
				// Inside import block, keep it raw or trim?
				// existing logic was just trimmed, but we might want to keep quotes logic?
				// But FormatImports adds quotes if we provide strings.
				// Wait, ExtractImports implementation I wrote earlier just appended lines.
				// Go imports inside () don't have "import" prefix.
				// e.g. "fmt"
				imports = append(imports, strings.Trim(trimmed, ",")) // handle comma if present (Go doesn't use commas usually in imports)
			}
			continue
		}

		if strings.HasPrefix(trimmed, "import (") {
			inImportBlock = true
			continue
		}

		if strings.HasPrefix(trimmed, "import ") {
			// Single line import: import "fmt" or import alias "pkg"
			imports = append(imports, strings.TrimPrefix(trimmed, "import "))
			continue
		}

		bodyLines = append(bodyLines, line)
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *GoLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	importBlock := "import (\n"
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	for _, imp := range uniqueImports {
		importBlock += "\t" + imp + "\n"
	}
	importBlock += ")"
	return importBlock
}

func (l *GoLanguage) ExtractPackage(content string) (string, string) {
	lines := strings.Split(content, "\n")
	pkg := ""
	var bodyLines []string
	foundPkg := false
	for _, line := range lines {
		if !foundPkg && strings.HasPrefix(strings.TrimSpace(line), "package ") {
			pkg = line
			foundPkg = true
			continue
		}
		bodyLines = append(bodyLines, line)
	}
	return pkg, strings.Join(bodyLines, "\n")
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

func (l *PythonLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") || strings.HasPrefix(trimmed, "from ") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *PythonLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
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

func (l *CSharpLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "using ") && strings.HasSuffix(trimmed, ";") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *CSharpLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
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
			definitionRegexp:   regexp.MustCompile(`^(?:def|class|module)\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`),
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

// #region Shell

type ShellLanguage struct {
	BaseLanguage
}

func NewShellLanguage() *ShellLanguage {
	return &ShellLanguage{
		BaseLanguage: BaseLanguage{
			name:               "shell",
			extensions:         []string{".sh"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\))?\s*\{`),
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

// #endregion Shell

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
			definitionRegexp:   regexp.MustCompile(`(?i)^(?:CREATE\s+(?:OR\s+REPLACE\s+)?(?:TABLE|VIEW|PROCEDURE|FUNCTION|TRIGGER|INDEX|TYPE|SCHEMA|DATABASE|SEQUENCE|MATERIALIZED\s+VIEW))\s+(?:IF\s+NOT\s+EXISTS\s+)?([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)`),
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
			definitionRegexp:   regexp.MustCompile(`^(?:type|interface|enum|input|union|scalar|query|mutation|subscription|fragment|extend\s+type|extend\s+interface|extend\s+enum|extend\s+union|extend\s+input)\s+([A-Za-z_][A-Za-z0-9_]*)`),
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
	NewShellLanguage(),
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
	UI     string       `json:"ui,omitempty"`
	Author string       `json:"author"`
	Date   time.Time    `json:"date"`
	Commit string       `json:"commit"`
	Files  []TicketFile `json:"files,omitempty"`
	Diff   *TicketDiffs `json:"diff,omitempty"`
}

type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

type TicketFile struct {
	Path  string       `json:"path"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

type TicketGithubData struct {
	Issue string `json:"issue,omitempty"`
}

type TicketFileRenamed struct {
	From  string       `json:"from"`
	To    string       `json:"to"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

type TicketDiffSet struct {
	Deleted  []TicketFile        `json:"deleted"`
	Renamed  []TicketFileRenamed `json:"renamed"`
	Modified []TicketFile        `json:"modified"`
	Added    []TicketFile        `json:"added"`
}

type TicketDiffs struct {
	Bundles     TicketDiffSet `json:"bundles"`
	Folders     TicketDiffSet `json:"folders"`
	Files       TicketDiffSet `json:"files"`
	Sections    TicketDiffSet `json:"sections"`
	Definitions TicketDiffSet `json:"definitions"`
}

type TicketData struct {
	Title      string            `json:"title"`
	Iterations []TicketIteration `json:"iterations"`
	Status     TicketStatus      `json:"status"`
	Dates      TicketDates       `json:"dates"`
	Summary    string            `json:"summary,omitempty"`
	GitHub     *TicketGithubData `json:"github,omitempty"`
}

type TicketDates struct {
	Closed *time.Time `json:"closed,omitempty"`
}

type ViolationKind string

const (
	ViolationCodeHeaderMissingRegion        ViolationKind = "code:header:missing-region"
	ViolationCodeHeaderMissingFilename      ViolationKind = "code:header:missing-filename"
	ViolationCodeHeaderMissingContributors  ViolationKind = "code:header:missing-contributors"
	ViolationCodeHeaderMissingLicense       ViolationKind = "code:header:missing-license"
	ViolationCodeHeaderWrongLicense         ViolationKind = "code:header:wrong-license"
	ViolationCodeSectionEmpty               ViolationKind = "code:section:empty"
	ViolationCodeSectionOrphanDefinition    ViolationKind = "code:section:orphan-definition"
	ViolationCodeSectionMissingStartName    ViolationKind = "code:section:missing-start-name"
	ViolationCodeSectionMissingEndName      ViolationKind = "code:section:missing-end-name"
	ViolationCodeSectionNameMismatch        ViolationKind = "code:section:name-mismatch"
	ViolationCodeCommentInline              ViolationKind = "code:comment:inline"
	ViolationCodeCommentBlock               ViolationKind = "code:comment:block"
	ViolationCodeCommentJSDoc               ViolationKind = "code:comment:jsdoc"
	ViolationDevDocsMissingFile             ViolationKind = "dev-docs:missing-file"
	ViolationDevDocsMissingFolder           ViolationKind = "dev-docs:missing-folder"
	ViolationDevDocsWrongFilePath           ViolationKind = "dev-docs:wrong-file-path"
	ViolationDevDocsWrongFolderPath         ViolationKind = "dev-docs:wrong-folder-path"
	ViolationDevDocsWrongFileName           ViolationKind = "dev-docs:wrong-file-name"
	ViolationDevDocsWrongFolderName         ViolationKind = "dev-docs:wrong-folder-name"
	ViolationDevDocsWrongFileOrder          ViolationKind = "dev-docs:wrong-file-order"
	ViolationDevDocsWrongFolderOrder        ViolationKind = "dev-docs:wrong-folder-order"
	ViolationDevDocsMissingComponent        ViolationKind = "dev-docs:missing-component"
	ViolationDevDocsWrongComponentName      ViolationKind = "dev-docs:wrong-component-name"
	ViolationDevDocsWrongComponentOrder     ViolationKind = "dev-docs:wrong-component-order"
	ViolationSketchpadImportThirdParty      ViolationKind = "sketchpad:import:third-party-outside-elements"
	ViolationSketchpadStateMultipleMachines ViolationKind = "sketchpad:state:multiple-machines"
	ViolationSketchpadStateCreateActor      ViolationKind = "sketchpad:state:create-actor-usage"
	ViolationSketchpadStateYjsAppState      ViolationKind = "sketchpad:state:yjs-app-state"
	ViolationSketchpadStateForbiddenStore   ViolationKind = "sketchpad:state:forbidden-store"
	ViolationSketchpadHooksNonTriadic       ViolationKind = "sketchpad:hooks:non-triadic"
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
	Output CommandOutput `json:"output"`
	Data   interface{}   `json:"data,omitempty"`
	Error  string        `json:"error,omitempty"`
}

type ContributorTicket struct {
	Year     int          `json:"year"`
	Month    int          `json:"month"`
	Day      int          `json:"day"`
	Slug     string       `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string       `json:"filePath,omitempty"`
}

type ContributorCommit struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

type ContributorContributionsStorage struct {
	Bundles     []string            `json:"bundles,omitempty"`
	Folders     []string            `json:"folders,omitempty"`
	Files       []string            `json:"files,omitempty"`
	Regions     []string            `json:"regions,omitempty"`
	Definitions []string            `json:"definitions,omitempty"`
	Tickets     []ContributorTicket `json:"tickets,omitempty"`
	Commits     []ContributorCommit `json:"commits,omitempty"`
	Lines       *LineMetrics        `json:"lines,omitempty"`
}

// #region Codebase Types

type BundleMetricsInternal struct {
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type FolderMetricsInternal struct {
	Files      int `json:"files"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FileMetricsInternal struct {
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
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
	ID    string     `json:"id"`
	Path  string     `json:"path"`
	URI   string     `json:"uri"`
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
	ID           string                 `json:"id"`
	Folder       string                 `json:"folder"`
	URI          string                 `json:"uri"`
	Contributors []string               `json:"contributors,omitempty"`
	Tickets      []string               `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseFolder struct {
	ID      string                 `json:"id"`
	Path    string                 `json:"path"`
	URI     string                 `json:"uri"`
	Metrics *FolderMetricsInternal `json:"metrics,omitempty"`
}

type FileViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebaseFile struct {
	ID         string               `json:"id"`
	Path       string               `json:"path"`
	URI        string               `json:"uri"`
	Metrics    *FileMetricsInternal `json:"metrics,omitempty"`
	Violations []FileViolationRef   `json:"violations,omitempty"`
}

type CodebaseSection struct {
	ID      string                  `json:"id"`
	Path    string                  `json:"path"`
	URI     string                  `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseDefinition struct {
	ID      string                     `json:"id"`
	Path    string                     `json:"path"`
	URI     string                     `json:"uri"`
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
	ID            string                            `json:"id"`
	URI           string                            `json:"uri"`
	Path          string                            `json:"path"`
	Name          string                            `json:"name,omitempty"`
	Icons         *ContributorIcons                 `json:"icons,omitempty"`
	Emails        []string                          `json:"emails,omitempty"`
	Links         map[string]string                 `json:"links,omitempty"`
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
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	URI         string                     `json:"uri"`
	Date        *TicketDateInfo            `json:"date,omitempty"`
	Commit      string                     `json:"commit,omitempty"`
	Year        string                     `json:"year"`
	Month       string                     `json:"month"`
	Day         string                     `json:"day"`
	Slug        string                     `json:"slug"`
	Prompt      string                     `json:"prompt,omitempty"`
	LLM         string                     `json:"llm,omitempty"`
	Author      string                     `json:"author,omitempty"`
	Status      TicketStatus               `json:"status"`
	Bundles     []TicketBundleContribInfo  `json:"bundles,omitempty"`
	Folders     []TicketFolderContribInfo  `json:"folders,omitempty"`
	Files       []TicketFileContribInfo    `json:"files,omitempty"`
	Sections    []TicketSectionContribInfo `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib  `json:"definitions,omitempty"`
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

var (
	rootDir  string
	executor *Executor
)

func init() {
	wd, err := os.Getwd()
	if err != nil {
		rootDir = "."
	} else {
		rootDir = findRepoRoot(wd)
	}
	SetRootDir(rootDir)
	var e error
	executor, e = NewExecutorWithContext(rootDir, NewRepoContext(rootDir))
	if e != nil {
		fmt.Fprintf(os.Stderr, "Warning: Failed to initialize GraphQL executor: %v\n", e)
	}
}

func GetRootDir() string {
	return rootDir
}

func SetRootDir(dir string) {
	rootDir = dir
}

func GetRepoMetaDir() string {
	return filepath.Join(rootDir, ".semio-repo")
}

func GetRepoMetaPath(path string) string {
	return filepath.Join(GetRepoMetaDir(), path)
}

func findRepoRoot(startDir string) string {
	dir, err := filepath.Abs(startDir)
	if err != nil {
		return startDir
	}
	// First, walk up looking for .git (prioritize git root over go.mod)
	searchDir := dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, ".git")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			break
		}
		searchDir = parent
	}
	// If no .git found, fall back to looking for go.mod
	searchDir = dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, "go.mod")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			return startDir
		}
		searchDir = parent
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
	contributorsDir := filepath.Join(GetRepoMetaDir(), "contributors")
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

func (o *CommandOutput) Json(data interface{}) {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err == nil {
		o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: string(bytes)})
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
	Scope        Scope
	RootDir      string
	Bundles      []Bundle
	fileCache    map[string]string
	sectionCache map[string][]Section
	ignoreCache  map[string]map[int][]string // file -> line -> ignore patterns
	filesOverride []string
}

func NewPolicyContext(scope Scope, bundles []Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Bundles:      bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]Section),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

func NewPolicyContextWithFiles(scope Scope, bundles []Bundle, files []string) *PolicyContext {
	ctx := NewPolicyContext(scope, bundles)
	ctx.filesOverride = files
	return ctx
}

func (ctx *PolicyContext) Files() ([]string, error) {
	if ctx.filesOverride != nil {
		return ctx.filesOverride, nil
	}
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

func (ctx *PolicyContext) CreateViolation(summary string, kind ViolationKind, scope string, line int, excerpt string) Violation {
	return Violation{
		ID:      buildViolationID(scope, line, 0),
		Summary: summary,
		Kind:    kind,
		Scope:   scope,
		Line:    line,
		Excerpt: excerpt,
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
	return CheckPoliciesWithContext(ctx, policyIDs)
}

func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Violation, error) {
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
			if matchesScope(p.Scopes, ctx.Scope) {
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
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
					file, 0, ""))
			} else {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
					file, 0, ""))
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
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
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
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
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
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
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
					fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
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
						file, lineNum, strings.TrimSpace(line)))
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
								file, lineNum, strings.TrimSpace(line)))
						} else if endName != open.name {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								ViolationCodeSectionNameMismatch,
								file, lineNum, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName)))
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
					fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, ""))
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
							excerpt))
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
				orphanRange.firstLine))
		}
	}
	return ctx.FilterIgnored(violations)
}

type CommentTemplateState struct {
	ExprDepth int
}

type CommentScanState struct {
	InBlockComment         bool
	BlockCommentStartLine  int
	BlockCommentStartIndex int
	BlockCommentIsJsDoc    bool
	InSingleQuote          bool
	InDoubleQuote          bool
	Templates              []CommentTemplateState
	Escaped                bool
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
				"AGENTS.md", fileSections[i+1].line, ""))
		}
	}
	for i := 0; i < len(folderSections)-1; i++ {
		if folderSections[i].path > folderSections[i+1].path {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Folder section '%s' should come after '%s' (alphabetical order)", folderSections[i].path, folderSections[i+1].path),
				ViolationDevDocsWrongFolderOrder,
				"AGENTS.md", folderSections[i+1].line, ""))
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
							file, lineNumber, strings.TrimSpace(line)))
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
						file, lineNumber, strings.TrimSpace(line)))
				}
			}
			if strings.Contains(line, "createActor(") || strings.Contains(line, "createActor<") {
				violations = append(violations, ctx.CreateViolation(
					"createActor is forbidden in sketchpad",
					ViolationSketchpadStateCreateActor,
					file, lineNumber, strings.TrimSpace(line)))
			}
			yjsAppStatePatterns := []string{"Y.Doc(", "new Doc(", "Y.Map(", "Y.Array(", "Y.Text("}
			for _, pattern := range yjsAppStatePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if !strings.Contains(strings.ToLower(file), "kit") &&
						!strings.Contains(strings.ToLower(file), "sync") {
						violations = append(violations, ctx.CreateViolation(
							"Yjs should only be used for kit data synchronization, not app state",
							ViolationSketchpadStateYjsAppState,
							file, lineNumber, strings.TrimSpace(line)))
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
							file, lineNumber, strings.TrimSpace(line)))
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
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
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
		fileDiffs := ticket.GetFiles().Files
		if fileDiffs.Added != nil || fileDiffs.Modified != nil || fileDiffs.Deleted != nil || fileDiffs.Renamed != nil {
			for _, entry := range fileDiffs.Modified {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Added {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Deleted {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Renamed {
				bundleName := ctx.GetBundleForFile(entry.To)
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
			URI:           ctx.FileURI(".semio-repo/contributors/" + c.Github),
			Path:          ".semio-repo/contributors/" + c.Github + "/contributor.json",
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
		ticketPath := ticket.TicketPath
		if ticketPath == "" {
			ticketPath = fmt.Sprintf(".semio-repo/tickets/%04d/%02d/%02d/%s/ticket.md", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		}

		bundleFiles := make(map[string]int)
		var paths []string
		files := ticket.GetFiles().Files
		if files.Added != nil {
			for _, f := range files.Added {
				paths = append(paths, f.Path)
			}
		}
		if files.Modified != nil {
			for _, f := range files.Modified {
				paths = append(paths, f.Path)
			}
		}
		if files.Deleted != nil {
			for _, f := range files.Deleted {
				paths = append(paths, f.Path)
			}
		}
		if files.Renamed != nil {
			for _, f := range files.Renamed {
				paths = append(paths, f.To)
			}
		}
		for _, path := range paths {
			bundleName := ctx.GetBundleForFile(path)
			if bundleName != "" {
				bundleFiles[bundleName]++
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

		llm := ticket.GetLLM()

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
			Commit:  ticket.GetCommit(),
			Year:    fmt.Sprintf("%04d", ticket.Year),
			Month:   fmt.Sprintf("%02d", ticket.Month),
			Day:     fmt.Sprintf("%02d", ticket.Day),
			Slug:    ticket.Slug,
			Prompt:  ticket.GetPrompt(),
			LLM:     llm,
			Author:  ticket.GetAuthor(),
			Status:  ticket.GetStatus(),
			Bundles: bundleContribs,
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

func BuildCodebaseSnapshot(files []string, bundles []Bundle, commit string) (*Codebase, error) {
	ctx := &CodebaseContext{RootDir: rootDir, RootURI: "file://" + NormalizePath(rootDir)}
	ctx.Bundles = bundles
	ctx.Files = files
	ctx.Policies = GetPolicies()
	codebase := &Codebase{}
	codebase.Bundles = BuildCodebaseBundlesForFiles(ctx, commit)
	codebase.Folders = BuildCodebaseFoldersForFiles(ctx, commit)
	codebase.Files = BuildCodebaseFilesForFiles(ctx, commit)
	codebase.Sections = BuildCodebaseSectionsForFiles(ctx, commit)
	codebase.Definitions = BuildCodebaseDefinitionsForFiles(ctx, commit)
	codebase.Tree = BuildCodebaseTree(ctx, codebase.Bundles, codebase.Files, codebase.Sections, codebase.Definitions)
	return codebase, nil
}

func BuildCodebaseBundlesForFiles(ctx *CodebaseContext, commit string) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})

	for _, bundle := range ctx.Bundles {
		folderSets[bundle.Name] = make(map[string]struct{})
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
		content, err := ReadTextFileAtCommit(commit, file)
		if err != nil {
			continue
		}
		lineCounts[bundleName] += CountLines(content)
		sections := ParseSections(content, file)
		sectionCounts[bundleName] += countSections(sections)
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			definitionCounts[bundleName] += len(defs)
		}
	}

	for _, bundle := range ctx.Bundles {
		result = append(result, CodebaseBundle{
			ID:     bundle.Name,
			Folder: bundle.Root,
			URI:    ctx.FileURI(bundle.Root),
			Metrics: &BundleMetricsInternal{
				Folders:     len(folderSets[bundle.Name]),
				Files:       fileCounts[bundle.Name],
				Sections:    sectionCounts[bundle.Name],
				Definitions: definitionCounts[bundle.Name],
				Lines:       lineCounts[bundle.Name],
			},
		})
	}
	return result
}

func BuildCodebaseFoldersForFiles(ctx *CodebaseContext, commit string) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		content, err := ReadTextFileAtCommit(commit, file)
		if err == nil {
			lineCounts[folder] += CountLines(content)
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		bundleName := ctx.GetBundleForFile(folder)
		id := folder
		if bundleName != "" {
			id = bundleName + "/" + folder
		}
		parent := filepath.Dir(folder)
		var parentID *string
		if parent != "." && parent != "" {
			parentValue := parent
			parentID = &parentValue
		}
		result = append(result, CodebaseFolder{
			ID:       id,
			Path:     folder,
			URI:      ctx.FolderURI(folder),
			Name:     filepath.Base(folder),
			ParentID: parentID,
			Metrics: &FolderMetricsInternal{
				Files: fileCounts[folder],
				Lines: lineCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseFilesForFiles(ctx *CodebaseContext, commit string) []CodebaseFile {
	var result []CodebaseFile
	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		id := file
		if bundleName != "" {
			id = bundleName + "/" + filepath.Base(file)
		}
		var metrics *FileMetricsInternal
		content, err := ReadTextFileAtCommit(commit, file)
		if err == nil {
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
		result = append(result, CodebaseFile{
			ID:      id,
			Path:    file,
			URI:     ctx.FileURI(file),
			Metrics: metrics,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseSectionsForFiles(ctx *CodebaseContext, commit string) []CodebaseSection {
	var result []CodebaseSection
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCommit(commit, file)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)
		bundleName := ctx.GetBundleForFile(file)
		addSectionsForContent(ctx, &result, file, bundleName, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func addSectionsForContent(ctx *CodebaseContext, result *[]CodebaseSection, file, bundleName, content string, sections []Section, parentPath string) {
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
			},
		})
		addSectionsForContent(ctx, result, file, bundleName, content, section.Children, sectionPath)
	}
}

func BuildCodebaseDefinitionsForFiles(ctx *CodebaseContext, commit string) []CodebaseDefinition {
	var result []CodebaseDefinition
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCommit(commit, file)
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
					Lines: def.End - def.Start + 1,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
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
	return filepath.Join(GetRepoMetaDir(), "tickets")
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

func normalizeTicketKeyword(value string) string {
	return strings.ToUpper(strings.TrimSpace(value))
}

func hasTicketKeyword(text, keyword string) bool {
	return strings.Contains(strings.ToUpper(text), keyword)
}

func LatestTicket() (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, fmt.Errorf("no tickets found")
	}
	sort.Slice(tickets, func(i, j int) bool {
		if tickets[i].Year != tickets[j].Year {
			return tickets[i].Year > tickets[j].Year
		}
		if tickets[i].Month != tickets[j].Month {
			return tickets[i].Month > tickets[j].Month
		}
		if tickets[i].Day != tickets[j].Day {
			return tickets[i].Day > tickets[j].Day
		}
		return tickets[i].GetDateCreated().After(tickets[j].GetDateCreated())
	})
	return &tickets[0], nil
}

func shouldContinueTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "CONTINUE")
}

func shouldSkipTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "NOTICKET")
}

func OpenTicket(title, prompt, llm, ui, planPath string, noIssue bool) (*Ticket, error) {
	if prompt == "" {
		prompt = title
	}
	if shouldSkipTicket(prompt) {
		return nil, nil
	}
	if shouldContinueTicket(prompt) {
		latest, err := LatestTicket()
		if err != nil {
			return nil, err
		}
		if latest.GetStatus() == TicketStatusClosed {
			return latest, ReopenTicket(latest, prompt, llm)
		}
		return latest, nil
	}
	return CreateTicket(title, prompt, llm, ui, planPath, noIssue)
}

func UpdateTicketTitle(ticket *Ticket, title string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if ticket.Data == nil {
		return fmt.Errorf("ticket data is nil")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("ticket title is required")
	}
	if title == strings.ToLower(title) {
		return fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT a slug or all lowercase")
	}
	if title == strings.ToUpper(title) {
		return fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT only in caps")
	}
	if strings.Contains(title, "-") || strings.Contains(title, "_") {
		return fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT contain dashes or underscores")
	}
	slug := Slugify(title)
	newFolderPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, slug)
	if slug != ticket.Slug {
		if FileExists(newFolderPath) {
			return fmt.Errorf("ticket folder already exists: %s", newFolderPath)
		}
		if err := EnsureDir(filepath.Dir(newFolderPath)); err != nil {
			return err
		}
		if err := os.Rename(ticket.FolderPath, newFolderPath); err != nil {
			return err
		}
	}
	ticket.Data.Title = title
	ticket.Slug = slug
	ticket.FolderPath = newFolderPath
	ticket.JsonPath = GetTicketJsonPath(ticket.Year, ticket.Month, ticket.Day, slug)
	ticket.PlanPath = GetTicketPlanPath(ticket.Year, ticket.Month, ticket.Day, slug)
	ticket.TicketPath = GetTicketFilePath(ticket.Year, ticket.Month, ticket.Day, slug)
	return nil
}

func CreateTicket(title, prompt, llm, ui, planPath string, noIssue bool) (*Ticket, error) {
	title = strings.TrimSpace(title)
	if title == strings.ToLower(title) {
		return nil, fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT a slug or all lowercase")
	}
	if title == strings.ToUpper(title) {
		return nil, fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT only in caps")
	}
	if strings.Contains(title, "-") || strings.Contains(title, "_") {
		return nil, fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT contain dashes or underscores")
	}

	now := time.Now()
	year, month, day := FormatDate(now)
	slug := Slugify(title)

	llmSlug, err := ResolveAllowedLLM(llm)
	if err != nil {
		return nil, err
	}
	uiSlug, err := ResolveAllowedUI(ui)
	if err != nil {
		return nil, err
	}

	ticketDir := GetTicketPath(year, month, day, slug)
	if err := EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	planFilePath := GetTicketPlanPath(year, month, day, slug)
	ticketFilePath := GetTicketFilePath(year, month, day, slug)
	gitAuthor := GetGitAuthorGithub()
	gitCommit := GetGitCommit()

	var planContent string
	if planPath != "" && FileExists(planPath) {
		planContent, err = ReadTextFile(planPath)
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
	if err := WriteTextFile(ticketFilePath, buildTicketMarkdown(planContent)); err != nil {
		return nil, fmt.Errorf("failed to write ticket file: %w", err)
	}

	ticketData := &TicketData{
		Title:  title,
		Status: TicketStatusOpen,
		Iterations: []TicketIteration{{
			Prompt: prompt,
			LLM:    llmSlug,
			UI:     uiSlug,
			Author: gitAuthor,
			Date:   now,
			Commit: gitCommit,
		}},
		Dates: TicketDates{},
	}

	skipIssue := noIssue || strings.Contains(prompt, "NOISSUE")
	if !skipIssue {
		issueBody := prompt
		if planContent != "" {
			issueBody = planContent
		}
		issueBody = formatPromptHeading(issueBody)
		issueURL, err := ghCreateIssue(title, issueBody)
		if err == nil && issueURL != "" {
			ticketData.GitHub = &TicketGithubData{Issue: issueURL}
		} else if err != nil {
			fmt.Printf("Warning: Failed to create GitHub issue: %v\n", err)
		}
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
		TicketPath:  ticketFilePath,
	}

	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	return ticket, nil
}

func ghCreateIssue(title, body string) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", body, "--label", "ticket"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := strings.TrimSpace(stdout)
	if issueURL != "" {
		ghAddIssueToProject(issueURL)
	}
	return issueURL, nil
}

func buildProjectLinkArgs(issueURL string) []string {
	return []string{"project", "item-add", "2", "--owner", "usalu", "--url", issueURL}
}

func ghAddIssueToProject(issueURL string) {
	if issueURL == "" {
		return
	}
	ExecCommand("gh", buildProjectLinkArgs(issueURL), "")
}

func CountLines(content string) int {
	if content == "" {
		return 0
	}
	return strings.Count(content, "\n") + 1
}

func CountLinesInFile(path string) int {
	content, err := ReadTextFile(path)
	if err != nil {
		return 0
	}
	return CountLines(content)
}

func CountLinesAtCommit(commit, filePath string) int {
	stdout, _, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", commit, filePath)}, "")
	if exitCode != 0 {
		return 0
	}
	return CountLines(stdout)
}

func ReadTextFileAtCommit(commit, filePath string) (string, error) {
	if commit == "" {
		return ReadTextFile(filepath.Join(rootDir, filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", commit, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

func ListFilesAtCommit(commit string) ([]string, error) {
	if commit == "" {
		files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, GetProjects())
		if err != nil {
			return nil, err
		}
		return files, nil
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"ls-tree", "-r", "--name-only", commit}, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git ls-tree failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		files = append(files, strings.TrimSpace(line))
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

func formatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖 Prompt"
	}
	return "# 🤖 Prompt\n\n" + body
}

func formatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍 Summary"
	}
	return "# 🔍 Summary\n\n" + body
}

func buildTicketMarkdown(planContent string) string {
	var builder strings.Builder
	builder.WriteString("# Ticket\n\n## Todos\n\n")
	if planContent != "" {
		builder.WriteString(strings.TrimSpace(planContent))
		builder.WriteString("\n\n")
	}
	builder.WriteString("## Changes\n\n")
	builder.WriteString("## Log\n\n")
	builder.WriteString("## Summary\n")
	return builder.String()
}

func updateTicketSummaryFile(ticketPath, summary string) error {
	if ticketPath == "" {
		return nil
	}
	content, err := ReadTextFile(ticketPath)
	if err != nil {
		return err
	}
	marker := "## Summary"
	if !strings.Contains(content, marker) {
		content = strings.TrimRight(content, "\n") + "\n\n" + marker + "\n\n" + summary + "\n"
		return WriteTextFile(ticketPath, content)
	}
	parts := strings.Split(content, marker)
	prefix := strings.TrimRight(parts[0], "\n")
	updated := prefix + "\n\n" + marker + "\n\n" + summary + "\n"
	return WriteTextFile(ticketPath, updated)
}

func FilterTicketWorkspaceFiles(ticket *Ticket, files []string) []string {
	if ticket == nil {
		return files
	}
	if len(files) == 0 {
		return files
	}
	if ticket.FolderPath == "" {
		return files
	}
	relative := NormalizePath(ticket.FolderPath)
	if filepath.IsAbs(ticket.FolderPath) {
		relative = GetRelativePath(ticket.FolderPath)
	}
	relative = strings.TrimPrefix(relative, "./")
	if relative == "" {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		if normalized == relative || strings.HasPrefix(normalized, relative+"/") {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

func ghAddComment(issueURL, comment string) error {
	args := []string{"issue", "comment", issueURL, "--body", comment}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue comment failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghAddLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--add-label", label)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghCloseIssue(issueURL string) error {
	args := []string{"issue", "close", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue close failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghReopenIssue(issueURL string) error {
	args := []string{"issue", "reopen", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue reopen failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghUpdateIssueTitle(issueURL, title string) error {
	args := []string{"issue", "edit", issueURL, "--title", title}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
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

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	planPath := GetTicketPlanPath(year, month, day, slug)
	ticketPath := GetTicketFilePath(year, month, day, slug)
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
		TicketPath:  ticketPath,
	}, nil
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
						if FileExists(ticketJsonPath) {
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

func LoadBundles() []Bundle {
	// Read bundles from project.json files in the repository
	var bundles []Bundle
	projectsDir := rootDir

	// Look for project.json files that define bundles
	entries, err := os.ReadDir(projectsDir)
	if err != nil {
		return bundles
	}

	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		projectJsonPath := filepath.Join(projectsDir, entry.Name(), "project.json")
		if FileExists(projectJsonPath) {
			content, err := ReadTextFile(projectJsonPath)
			if err != nil {
				continue
			}
			var project struct {
				Name        string   `json:"name"`
				Root        string   `json:"root"`
				SourceRoot  string   `json:"sourceRoot"`
				ProjectType string   `json:"projectType"`
				Tags        []string `json:"tags"`
			}
			if err := json.Unmarshal([]byte(content), &project); err != nil {
				continue
			}
			if project.Name == "" {
				project.Name = entry.Name()
			}
			if project.Root == "" {
				project.Root = entry.Name()
			}
			bundles = append(bundles, Bundle{
				Name:        project.Name,
				Root:        project.Root,
				SourceRoot:  project.SourceRoot,
				ProjectType: project.ProjectType,
				Tags:        project.Tags,
			})
		}
	}

	// Also check subdirectories (js/, go/, etc.)
	for _, subDir := range []string{"js", "go", "dotnet", "python"} {
		subPath := filepath.Join(projectsDir, subDir)
		if !FileExists(subPath) {
			continue
		}
		subEntries, err := os.ReadDir(subPath)
		if err != nil {
			continue
		}
		for _, entry := range subEntries {
			if !entry.IsDir() {
				continue
			}
			projectJsonPath := filepath.Join(subPath, entry.Name(), "project.json")
			if FileExists(projectJsonPath) {
				content, err := ReadTextFile(projectJsonPath)
				if err != nil {
					continue
				}
				var project struct {
					Name        string   `json:"name"`
					Root        string   `json:"root"`
					SourceRoot  string   `json:"sourceRoot"`
					ProjectType string   `json:"projectType"`
					Tags        []string `json:"tags"`
				}
				if err := json.Unmarshal([]byte(content), &project); err != nil {
					continue
				}
				if project.Name == "" {
					project.Name = entry.Name()
				}
				if project.Root == "" {
					project.Root = filepath.Join(subDir, entry.Name())
				}
				bundles = append(bundles, Bundle{
					Name:        project.Name,
					Root:        project.Root,
					SourceRoot:  project.SourceRoot,
					ProjectType: project.ProjectType,
					Tags:        project.Tags,
				})
			}
		}
	}

	return bundles
}

func GetProjects() []Bundle {
	rootPackagePath := filepath.Join(rootDir, "package.json")
	workspaceBundles := make(map[string]Bundle)
	if FileExists(rootPackagePath) {
		raw, err := ReadTextFile(rootPackagePath)
		if err == nil {
			var rootPackage struct {
				Workspaces []string `json:"workspaces"`
			}
			if err := json.Unmarshal([]byte(raw), &rootPackage); err == nil {
				for _, workspace := range rootPackage.Workspaces {
					workspacePath := filepath.Join(rootDir, workspace)
					packagePath := filepath.Join(workspacePath, "package.json")
					if FileExists(packagePath) {
						content, err := ReadTextFile(packagePath)
						if err != nil {
							continue
						}
						var pkg struct {
							Name       string `json:"name"`
							SourceRoot string `json:"sourceRoot"`
						}
						if err := json.Unmarshal([]byte(content), &pkg); err != nil {
							continue
						}
						bundleName := strings.TrimPrefix(pkg.Name, "@semio/")
						if workspace == "js/vscode" {
							bundleName = "vscode"
						}
						if workspace == "go/repo" {
							bundleName = "repo"
						}
						workspaceBundles[workspace] = Bundle{
							Name:       bundleName,
							Root:       workspace,
							SourceRoot: pkg.SourceRoot,
						}
						continue
					}
					if strings.HasPrefix(workspace, "py/") {
						workspaceBundles[workspace] = Bundle{
							Name: "py",
							Root: workspace,
						}
					}
				}
			}
		}
	}
	if len(workspaceBundles) == 0 {
		return []Bundle{
			{Name: "js", Root: "js"},
			{Name: "js", Root: "javascript"},
			{Name: "py", Root: "py"},
			{Name: "net", Root: "net"},
			{Name: "net", Root: "dotnet"},
			{Name: "play", Root: "js/play"},
			{Name: "play", Root: "javascript/play"},
			{Name: "vscode", Root: "js/vscode"},
			{Name: "vscode", Root: "javascript/vscode"},
			{Name: "docs", Root: "js/docs"},
			{Name: "docs", Root: "javascript/docs"},
			{Name: "assets", Root: "assets"},
			{Name: "grasshopper", Root: "net/Semio.Grasshopper"},
			{Name: "grasshopper", Root: "dotnet/Semio.Grasshopper"},
			{Name: "yak", Root: "yak"},
			{Name: "repo", Root: "go/repo"},
			{Name: "go", Root: "go/semio"},
		}
	}
	var bundles []Bundle
	for _, bundle := range workspaceBundles {
		bundles = append(bundles, bundle)
	}
	return bundles
}

func ResolveBundleForPath(filePath string, bundles []Bundle) string {
	var bestMatch string
	var maxLen int
	for _, b := range bundles {
		if b.Name == "" {
			continue
		}
		if strings.HasPrefix(filePath, b.Root+"/") || filePath == b.Root {
			if len(b.Root) > maxLen {
				maxLen = len(b.Root)
				bestMatch = normalizeBundleLabel(b.Name)
			}
		}
	}
	return bestMatch
}

func formatLineMetrics(metrics *LineMetrics) string {
	if metrics == nil {
		return ""
	}
	var parts []string
	if metrics.Removed > 0 {
		parts = append(parts, fmt.Sprintf("-%d", metrics.Removed))
	}
	if metrics.Added > 0 {
		parts = append(parts, fmt.Sprintf("+%d", metrics.Added))
	}
	if len(parts) == 0 {
		return ""
	}
	return " " + strings.Join(parts, " ")
}

func formatPathWithBundle(path string, bundles []Bundle) string {
	bundleName := ResolveBundleForPath(path, bundles)
	if bundleName == "" {
		return path
	}
	bundleLabel := normalizeBundleLabel(bundleName)
	root := ""
	for _, bundle := range bundles {
		if normalizeBundleLabel(bundle.Name) == bundleLabel || bundle.Name == bundleName {
			root = bundle.Root
			break
		}
	}
	if root != "" {
		relative := strings.TrimPrefix(path, root+"/")
		if relative != path {
			if relative == "" {
				return bundleLabel
			}
			return bundleLabel + "/" + relative
		}
	}
	return bundleLabel + "/" + path
}

func formatSemanticPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	return formatPathWithBundle(filePath, bundles) + remainder
}

func formatDeletedPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	base := formatPathWithBundle(filePath, bundles)
	if remainder == "" {
		return "<del>" + base + "</del>"
	}
	return base + "<del>" + remainder + "</del>"
}

func commonPrefixLength(a, b string) int {
	limit := len(a)
	if len(b) < limit {
		limit = len(b)
	}
	idx := 0
	for idx < limit {
		if a[idx] != b[idx] {
			break
		}
		idx++
	}
	return idx
}

func commonSuffixLength(a, b string, prefix int) int {
	max := len(a) - prefix
	if len(b)-prefix < max {
		max = len(b) - prefix
	}
	idx := 0
	for idx < max {
		if a[len(a)-1-idx] != b[len(b)-1-idx] {
			break
		}
		idx++
	}
	return idx
}

func formatRenameDelta(from, to string) string {
	if from == to {
		return from
	}
	prefix := commonPrefixLength(from, to)
	suffix := commonSuffixLength(from, to, prefix)
	fromMiddle := from[prefix : len(from)-suffix]
	toMiddle := to[prefix : len(to)-suffix]
	return from[:prefix] + "<del>" + fromMiddle + "</del>" + toMiddle + from[len(from)-suffix:]
}

func formatRenamePath(from, to string, bundles []Bundle) string {
	fromFormatted := formatSemanticPath(from, bundles)
	toFormatted := formatSemanticPath(to, bundles)
	return formatRenameDelta(fromFormatted, toFormatted)
}

func appendDiffLines(lines *[]string, diffSet TicketDiffSet, iconAdded, iconChanged, iconRemoved, iconRenamed string, bundles []Bundle, formatter func(string) string, renameFormatter func(string, string) string) {
	if len(diffSet.Added) > 0 {
		sort.Slice(diffSet.Added, func(i, j int) bool { return diffSet.Added[i].Path < diffSet.Added[j].Path })
		for _, entry := range diffSet.Added {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconAdded, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Modified) > 0 {
		sort.Slice(diffSet.Modified, func(i, j int) bool { return diffSet.Modified[i].Path < diffSet.Modified[j].Path })
		for _, entry := range diffSet.Modified {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconChanged, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Renamed) > 0 {
		sort.Slice(diffSet.Renamed, func(i, j int) bool { return diffSet.Renamed[i].To < diffSet.Renamed[j].To })
		for _, entry := range diffSet.Renamed {
			path := renameFormatter(entry.From, entry.To)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRenamed, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Deleted) > 0 {
		sort.Slice(diffSet.Deleted, func(i, j int) bool { return diffSet.Deleted[i].Path < diffSet.Deleted[j].Path })
		for _, entry := range diffSet.Deleted {
			path := formatDeletedPath(entry.Path, bundles)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRemoved, path, formatLineMetrics(entry.Lines)))
		}
	}
}

func generateMetricsComment(diffs *TicketDiffs, bundles []Bundle) string {
	if diffs == nil {
		return ""
	}
	var lines []string
	appendDiffLines(&lines, diffs.Bundles, "📦", "📦", "📦", "📦", bundles, func(path string) string {
		return path
	}, func(from, to string) string {
		return formatRenameDelta(from, to)
	})
	appendDiffLines(&lines, diffs.Folders, "📂", "📁", "📁", "📁", bundles, func(path string) string {
		return formatPathWithBundle(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Files, "📄", "📝", "📄", "📄", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Sections, "📑", "🔖", "🔖", "🔖", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Definitions, "🏷️", "🏷️", "🏷️", "🏷️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	if len(lines) == 0 {
		return ""
	}
	return strings.Join(lines, "\n")
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

	if ticket.Data.GitHub != nil && ticket.Data.GitHub.Issue != "" {
		issueURL := ticket.Data.GitHub.Issue

		// 1. Add comment with summary and metrics

		// 2. Add labels
		bundles := GetProjects()
		labels := make(map[string]struct{})
		if tickFilesResult != nil {
			addLabel := func(path string) {
				if path == "" {
					return
				}
				labels[path] = struct{}{}
			}
			bundleDiffs := tickFilesResult.Bundles
			for _, entry := range bundleDiffs.Added {
				addLabel(entry.Path)
			}
			for _, entry := range bundleDiffs.Modified {
				addLabel(entry.Path)
			}
			for _, entry := range bundleDiffs.Deleted {
				addLabel(entry.Path)
			}
			for _, entry := range bundleDiffs.Renamed {
				addLabel(entry.From)
				addLabel(entry.To)
			}
		}
		var labelList []string
		for l := range labels {
			labelList = append(labelList, l)
		}
		if len(labelList) > 0 {
			if err := ghAddLabels(issueURL, labelList); err != nil {
				fmt.Printf("Warning: Failed to add labels to GitHub issue: %v\n", err)
			}
		}

		// 3. Add metrics comment and close
		comment := formatSummaryHeading(summary)
		metricsComment := generateMetricsComment(tickFilesResult, bundles)
		if metricsComment != "" {
			comment += "\n\n# ✍️ Changes\n\n" + metricsComment
		}

		if err := ghAddComment(issueURL, comment); err != nil {
			fmt.Printf("Warning: Failed to add summary and metrics comment to GitHub issue: %v\n", err)
		}

		// Close issue
		if err := ghCloseIssue(issueURL); err != nil {
			fmt.Printf("Warning: Failed to close GitHub issue: %v\n", err)
		}
	}

	ticket.Data.Summary = summary
	if err := updateTicketSummaryFile(ticket.TicketPath, summary); err != nil {
		return err
	}
	if len(ticket.Data.Iterations) > 0 {
		lastIndex := len(ticket.Data.Iterations) - 1
		ticket.Data.Iterations[lastIndex].Diff = tickFilesResult
	}
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
	llmSlug, err := ResolveAllowedLLM(llm)
	if err != nil {
		return err
	}
	uiSlug, err := ResolveAllowedUI(ticket.GetUI())
	if err != nil {
		return err
	}

	iteration := TicketIteration{
		Prompt: prompt,
		LLM:    llmSlug,
		UI:     uiSlug,
		Author: gitAuthor,
		Date:   time.Now(),
		Commit: gitCommit,
	}

	ticket.Data.Iterations = append(ticket.Data.Iterations, iteration)
	ticket.Data.Status = TicketStatusOpen
	ticket.Data.Dates.Closed = nil

	if ticket.Data.GitHub != nil && ticket.Data.GitHub.Issue != "" {
		issueURL := ticket.Data.GitHub.Issue
		if err := ghReopenIssue(issueURL); err != nil {
			fmt.Printf("Warning: Failed to reopen GitHub issue: %v\n", err)
		}
		ghAddIssueToProject(issueURL)
		comment := formatPromptHeading(prompt)
		if err := ghAddComment(issueURL, comment); err != nil {
			fmt.Printf("Warning: Failed to add prompt comment to GitHub issue: %v\n", err)
		}
	}

	return SaveTicket(ticket)
}

func ToolTicketOpen(title, prompt, llm, ui, planPath string, noIssue bool) ToolResult {
	output := NewOutput()
	resolvedPrompt := prompt
	if resolvedPrompt == "" {
		resolvedPrompt = title
	}
	ticket, err := OpenTicket(title, prompt, llm, ui, planPath, noIssue)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if ticket == nil {
		output.Info("\n🚫 Ticket creation skipped (NOTICKET)")
		return ToolResult{Output: *output}
	}
	if shouldContinueTicket(resolvedPrompt) {
		output.Success(fmt.Sprintf("\n↩️ Continued ticket: %s", ticket.Slug))
		return ToolResult{Output: *output, Data: ticket}
	}
	output.Success(fmt.Sprintf("\n🎫 Created ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Folder: %s", ticket.FolderPath))
	if ticket.JsonPath != "" {
		output.Info(fmt.Sprintf("   JSON:   %s", ticket.JsonPath))
	}
	if ticket.PlanPath != "" {
		output.Info(fmt.Sprintf("   Plan:   %s", ticket.PlanPath))
	}
	if ticket.TicketPath != "" {
		output.Info(fmt.Sprintf("   Ticket: %s", ticket.TicketPath))
	}
	if ticket.Data != nil && ticket.Data.GitHub != nil && ticket.Data.GitHub.Issue != "" {
		output.Info(fmt.Sprintf("   Issue:  %s", ticket.Data.GitHub.Issue))
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
	ticketContent, _ := ReadTextFile(ticket.TicketPath)
	if ticketContent != "" {
		output.Plain(fmt.Sprintf("\n%s", ticketContent))
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
		filesCount = len(filesResult.Files.Modified) + len(filesResult.Files.Added) + len(filesResult.Files.Renamed) + len(filesResult.Files.Deleted)
		sectionsCount = len(filesResult.Sections.Modified) + len(filesResult.Sections.Added) + len(filesResult.Sections.Renamed) + len(filesResult.Sections.Deleted)
		defsCount = len(filesResult.Definitions.Modified) + len(filesResult.Definitions.Added) + len(filesResult.Definitions.Renamed) + len(filesResult.Definitions.Deleted)
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

func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) ToolResult {
	output := NewOutput()

	// 1. Read source content
	absSourcePath := filepath.Join(rootDir, sourcePath)
	if !FileExists(absSourcePath) {
		output.Error(fmt.Sprintf("Error: Source file not found: %s", sourcePath))
		return ToolResult{Output: *output, Error: "source file not found"}
	}
	sourceContent, err := ReadTextFile(absSourcePath)
	if err != nil {
		output.Error(fmt.Sprintf("Error reading source file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 2. Read target file
	absTargetFilePath := filepath.Join(rootDir, targetFilePath)
	if !FileExists(absTargetFilePath) {
		output.Error(fmt.Sprintf("Error: Target file not found: %s", targetFilePath))
		return ToolResult{Output: *output, Error: "target file not found"}
	}
	targetContent, err := ReadTextFile(absTargetFilePath)
	if err != nil {
		output.Error(fmt.Sprintf("Error reading target file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 3. Get target language
	targetLanguage := GetLanguage(targetFilePath)
	if targetLanguage == nil || !targetLanguage.SupportsSections() {
		output.Error("Error: Target file type does not support sections")
		return ToolResult{Output: *output, Error: "unsupported target file type"}
	}

	output.Info(fmt.Sprintf("Splitting headers..."))

	// 4. Split Headers
	sourceHeader, sourceBody := SplitHeader(sourceContent, targetLanguage)
	targetHeader, targetBody := SplitHeader(targetContent, targetLanguage)

	output.Info(fmt.Sprintf("Source Header len: %d, Source Body len: %d", len(sourceHeader), len(sourceBody)))

	// 5. Extract Imports
	_, sourceBodyNoPkg := targetLanguage.ExtractPackage(sourceBody)
	targetPkg, targetBodyNoPkg := targetLanguage.ExtractPackage(targetBody)

	sourceImports, sourceCode := targetLanguage.ExtractImports(sourceBodyNoPkg)
	targetImports, targetCode := targetLanguage.ExtractImports(targetBodyNoPkg)

	// 6. Merge Headers
	mergedHeader := MergeHeaders(targetHeader, sourceHeader, targetLanguage)

	// 7. Merge Imports
	mergedImports := UniqueStrings(append(targetImports, sourceImports...))

	// 8. Format the new section with source content
	startMarker := targetLanguage.FormatSectionStart(targetSectionName)
	endMarker := targetLanguage.FormatSectionEnd(targetSectionName)

	// Ensure content ends with newline if it doesn't
	if !strings.HasSuffix(sourceCode, "\n") && sourceCode != "" {
		sourceCode += "\n"
	}

	sectionContent := "\n" + startMarker + "\n" + sourceCode + endMarker + "\n"

	// 9. Handle insertion
	var updatedBody string
	if targetParentSectionName != "" {
		// Find parent section
		sections := targetLanguage.ParseSections(targetCode)
		parentSection := FindSection(sections, targetParentSectionName)
		if parentSection == nil {
			output.Error(fmt.Sprintf("Error: Parent section not found: %s", targetParentSectionName))
			return ToolResult{Output: *output, Error: "parent section not found"}
		}

		// Insert at the end of parent section (before parent's end marker)
		if parentSection.EndLine == -1 {
			output.Error(fmt.Sprintf("Error: Parent section %s is not properly closed", targetParentSectionName))
			return ToolResult{Output: *output, Error: "parent section not closed"}
		}

		// Insert before the end marker line of the parent section
		lines := strings.Split(targetCode, "\n")
		newLines := make([]string, 0, len(lines)+strings.Count(sectionContent, "\n"))
		newLines = append(newLines, lines[:parentSection.EndLine-1]...)
		newLines = append(newLines, strings.Split(strings.Trim(sectionContent, "\n"), "\n")...)
		newLines = append(newLines, lines[parentSection.EndLine-1:]...)
		updatedBody = strings.Join(newLines, "\n")
	} else {
		// Append to the end of the target file
		updatedBody = targetCode
		if !strings.HasSuffix(updatedBody, "\n") && updatedBody != "" {
			updatedBody += "\n"
		}
		updatedBody += sectionContent
	}

	// 10. Reassemble
	finalContent := mergedHeader
	if finalContent != "" {
		if !strings.HasSuffix(finalContent, "\n") {
			finalContent += "\n"
		}
		finalContent += "\n"
	}

	if targetPkg != "" {
		finalContent += targetPkg + "\n\n"
	}

	formattedImports := targetLanguage.FormatImports(mergedImports)
	if formattedImports != "" {
		finalContent += formattedImports + "\n\n"
	}

	finalContent += updatedBody

	// 12. Write target file
	if err := WriteTextFile(absTargetFilePath, finalContent); err != nil {
		output.Error(fmt.Sprintf("Error writing target file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	output.Success(fmt.Sprintf("\n🧩 Integrated %s into %s section of %s", sourcePath, targetSectionName, targetFilePath))
	return ToolResult{Output: *output}
}

func SplitHeader(content string, lang LanguagePlugin) (string, string) {
	sections := lang.ParseSections(content)
	for _, s := range sections {
		if strings.EqualFold(s.Name, "Header") {
			header := content[:s.EndIndex]
			body := content[s.EndIndex:]
			return header, body
		}
	}
	return "", content
}

func MergeHeaders(targetHeader, sourceHeader string, lang LanguagePlugin) string {
	if targetHeader == "" {
		return sourceHeader
	}
	if sourceHeader == "" {
		return targetHeader
	}
	targetLines := strings.Split(targetHeader, "\n")
	sourceLines := strings.Split(sourceHeader, "\n")
	seen := make(map[string]bool)
	for _, line := range targetLines {
		seen[strings.TrimSpace(line)] = true
	}
	var insertIdx = -1
	for i, line := range targetLines {
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			insertIdx = i
		}
	}
	if insertIdx == -1 {
		return targetHeader
	}

	var newLines []string
	for _, line := range sourceLines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		if matched, _ := lang.PolicySectionStartMatch(line); matched {
			continue
		}
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			continue
		}
		if !seen[trimmed] {
			newLines = append(newLines, line)
		}
	}

	if len(newLines) == 0 {
		return targetHeader
	}

	res := make([]string, 0, len(targetLines)+len(newLines)+1)
	res = append(res, targetLines[:insertIdx]...)
	res = append(res, newLines...)
	res = append(res, targetLines[insertIdx:]...)
	return strings.Join(res, "\n")
}

func UniqueStrings(input []string) []string {
	keys := make(map[string]bool)
	list := []string{}
	for _, entry := range input {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
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
	Path           string `json:"path"`
	Bundles        int    `json:"bundles"`
	Folders        int    `json:"folders"`
	Files          int    `json:"files"`
	Sections       int    `json:"sections"`
	Definitions    int    `json:"definitions"`
	Contributors   int    `json:"contributors"`
	Tickets        int    `json:"tickets"`
	Policies       int    `json:"policies"`
	ViolationKinds int    `json:"violationKinds"`
	Violations     int    `json:"violations"`
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
	ticketStmt, err := tx.Prepare(`INSERT INTO ticket (id, year, month, day, slug, title, path, uri, prompt, summary, status, author_id, llm, ui, commit_sha, created_at, finished_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`)
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
		var authorID, llm, ui, summary, commit, finishedAt interface{}
		if author := t.GetAuthor(); author != "" {
			authorID = "contributor:" + author
		}
		if val := t.GetLLM(); val != "" {
			llm = val
		}
		if val := t.GetUI(); val != "" {
			ui = val
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
		if _, err := ticketStmt.Exec(ticketID, t.Year, t.Month, t.Day, t.Slug, t.GetTitle(), t.FolderPath, uri, t.GetPrompt(), summary, status, authorID, llm, ui, commit, createdAt, finishedAt); err != nil {
			return 0, err
		}
		fileDiffs := t.GetFiles().Files
		for _, entry := range fileDiffs.Modified {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Added {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Deleted {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Renamed {
			if _, err := ticketFileStmt.Exec(ticketID, entry.To); err != nil {
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
	stmt, err := tx.Prepare(`INSERT INTO violation (id, kind_id, scope, file_id, folder_id, line, column_num, excerpt, summary) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	for _, v := range violations {
		kindID := "violationKind:" + string(v.Kind)
		var fileID, folderID, line, column, excerpt interface{}
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
		if _, err := stmt.Exec(v.ID, kindID, v.Scope, fileID, folderID, line, column, excerpt, v.Summary); err != nil {
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

// #region GraphQL Context Port

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
	Integrate(source, targetSection, targetFile, targetParent *string) (*File, error)
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
}

// #endregion GraphQL Context Port

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

// repoContext is the full implementation of RepoContext
type repoContext struct {
	rootDir string
	bundles []Bundle
}

func NewRepoContext(rootDir string) RepoContext {
	ctx := &repoContext{rootDir: rootDir}
	ctx.bundles = LoadBundles()
	return ctx
}

func (c *repoContext) GetRootDir() string { return c.rootDir }

func (c *repoContext) GetBundles() []*Bundle {
	result := make([]*Bundle, len(c.bundles))
	for i := range c.bundles {
		result[i] = &c.bundles[i]
	}
	return result
}

func (c *repoContext) GetFolders() []*Folder { return []*Folder{} }

func (c *repoContext) GetFiles() []*File { return []*File{} }

func (c *repoContext) GetDefinitions() []*Definition { return []*Definition{} }

func (c *repoContext) GetSections() []*Section { return []*Section{} }

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
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, &tickets[i])
		}
	}
	return result, nil
}

func (c *repoContext) GetPolicies() []*Policy {
	policies := GetRegisteredPolicies()
	result := make([]*Policy, len(policies))
	for i := range policies {
		var descPtr *string
		if policies[i].Description != "" {
			d := policies[i].Description
			descPtr = &d
		}
		result[i] = &Policy{
			ID:          policies[i].ID,
			Name:        policies[i].Name,
			Description: descPtr,
			Scopes:      policies[i].Scopes,
		}
	}
	return result
}

func (c *repoContext) GetViolationKinds() []*ViolationKindMeta {
	var result []*ViolationKindMeta
	for _, meta := range violationKindInfoTable {
		m := meta
		result = append(result, &m)
	}
	return result
}

func (c *repoContext) Analyze(scope *string) (*AnalyzeResult, error) {
	scopeStr := "@semio"
	if scope != nil {
		scopeStr = *scope
	}
	violations, err := CheckPolicies(ParseScope(scopeStr), c.bundles, nil)
	if err != nil {
		return nil, err
	}
	result := make([]*Violation, len(violations))
	for i := range violations {
		result[i] = &violations[i]
	}
	return &AnalyzeResult{Violations: result, Metrics: &AnalyzeMetrics{Total: len(violations)}}, nil
}

func (c *repoContext) Fix(scope *string) (*FixResult, error) {
	return &FixResult{Violations: []*Violation{}}, nil
}

func (c *repoContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	return OpenTicket(input.Title, input.Prompt, input.LLM, input.UI, input.PlanPath, input.NoIssue)
}

func (c *repoContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	// Handle title update if provided
	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}
		// Update GitHub issue title if exists
		if ticket.Data.GitHub != nil && ticket.Data.GitHub.Issue != "" {
			if err := ghUpdateIssueTitle(ticket.Data.GitHub.Issue, *input.Title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
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
	// Handle title update if provided
	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}
		// Update GitHub issue title if exists
		if ticket.Data.GitHub != nil && ticket.Data.GitHub.Issue != "" {
			if err := ghUpdateIssueTitle(ticket.Data.GitHub.Issue, *input.Title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}
	if err := ReopenTicket(ticket, input.Prompt, input.LLM); err != nil {
		return nil, err
	}
	return ticket, nil
}

func (c *repoContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

func (c *repoContext) FolderMove(src, dst string) (*Folder, error) { return nil, nil }

func (c *repoContext) FolderDelete(path string) error { return nil }

func (c *repoContext) FileCreate(path string) (*File, error) { return nil, nil }

func (c *repoContext) FileMove(src, dst string) (*File, error) { return nil, nil }

func (c *repoContext) FileDelete(path string) error { return nil }

func (c *repoContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}

func (c *repoContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}

func (c *repoContext) SectionDelete(file, name string) error { return nil }

func (c *repoContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	s := ""
	if source != nil {
		s = *source
	}
	ts := ""
	if targetSection != nil {
		ts = *targetSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}
	tp := ""
	if targetParent != nil {
		tp = *targetParent
	}
	result := ToolIntegrate(s, ts, tf, tp)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &File{ID: "file:" + tf, Path: tf, Name: filepath.Base(tf)}, nil
}

func (c *repoContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *repoContext) ContributorRemove(github string) error { return nil }

var _ RepoContext = (*repoContext)(nil)

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

func (c *defaultContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	return nil, nil
}

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
			"INTERFACE": &graphql.EnumValueConfig{Value: DefinitionKindPort},
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

	ticketUIEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "TicketUI",
		Values: graphql.EnumValueConfigMap{
			"COPILOT_CHAT": &graphql.EnumValueConfig{Value: "copilot-chat"},
			"ANTIGRAVITY":  &graphql.EnumValueConfig{Value: "antigravity"},
			"CURSOR":       &graphql.EnumValueConfig{Value: "cursor"},
			"CLAUDE_CODE":  &graphql.EnumValueConfig{Value: "claude-code"},
			"CODEX":        &graphql.EnumValueConfig{Value: "codex"},
			"DROID":        &graphql.EnumValueConfig{Value: "droid"},
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
				"id":     &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"parent": &graphql.Field{Type: folderType},
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
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
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
						return &Range{Start: section.StartLine, End: section.EndLine}, nil
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
						return &Range{Start: definition.StartLine, End: definition.EndLine}, nil
					},
				},
			}
		}),
	})

	violationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Violation",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"kindId": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return string(violation.Kind), nil
					},
				},
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
				"summary": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Summary, nil
					},
				},
				"priority": &graphql.Field{
					Type: graphql.NewNonNull(violationPriorityEnum),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Priority(), nil
					},
				},
				"autofixable": &graphql.Field{
					Type: graphql.NewNonNull(graphql.Boolean),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Autofixable(), nil
					},
				},
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
				"year":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"month": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"day":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"slug":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						if ticket.TicketPath != "" {
							return ticket.TicketPath, nil
						}
						return ticket.JsonPath, nil
					},
				},
				"llm": &graphql.Field{
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
				"ui": &graphql.Field{
					Type: ticketUIEnum,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						ui := ticket.GetUI()
						if ui == "" {
							return nil, nil
						}
						return ui, nil
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
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						path := ticket.JsonPath
						if ticket.TicketPath != "" {
							path = ticket.TicketPath
						}
						absPath := filepath.Join(rootDir, path)
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
				"github": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":   &graphql.Field{Type: graphql.String},
				"emails": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
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
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetBundles(), nil
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetFolders(), nil
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetFiles(), nil
					},
				},
				"sections": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetSections(), nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetDefinitions(), nil
					},
				},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetContributors()
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
						_ = p.Source.(*Repo)
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
						return repoResolverInstance.Ctx.GetTickets(year, month, day, status)
					},
				},
				"policies": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetPolicies(), nil
					},
				},
				"violationKinds": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetViolationKinds(), nil
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
					if s, ok := p.Args["status"].(TicketStatus); ok {
						status = &s
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
			"title":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"ui":       &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(ticketUIEnum)},
			"noIssue":  &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"planPath": &graphql.InputObjectFieldConfig{Type: graphql.String},
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
			"title":   &graphql.InputObjectFieldConfig{Type: graphql.String},
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
			"title":  &graphql.InputObjectFieldConfig{Type: graphql.String},
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
						UI:     inputMap["ui"].(string),
					}
					if inputMap["noIssue"] != nil {
						input.NoIssue = inputMap["noIssue"].(bool)
					}
					if inputMap["planPath"] != nil {
						input.PlanPath = inputMap["planPath"].(string)
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
					if t, ok := inputMap["title"].(string); ok {
						input.Title = &t
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
					if t, ok := inputMap["title"].(string); ok {
						input.Title = &t
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
			"integrate": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"source":        &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetSection": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetFile":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetParent":  &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					source := p.Args["source"].(string)
					targetSection := p.Args["targetSection"].(string)
					targetFile := p.Args["targetFile"].(string)
					var targetParent *string
					if par, ok := p.Args["targetParent"].(string); ok {
						targetParent = &par
					}
					return mutationResolverInstance.Integrate(p.Context, &source, &targetSection, &targetFile, targetParent)
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
		id := normalizeBundleID(bundleName)
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
		id := normalizeBundleID(bundleName)
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

func (r *mutationResolver) Integrate(ctx context.Context, source, targetSection, targetFile, targetParent *string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.Integrate(source, targetSection, targetFile, targetParent)
	}
	return nil, fmt.Errorf("not implemented")
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
	Integrate(ctx context.Context, source, targetSection, targetFile, targetParent *string) (*File, error)
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

// #region Mcp

func runMcpServer(cmd *cobra.Command, args []string) error {
	s := server.NewMCPServer(
		"semio-repo",
		"1.0.0",
		server.WithToolCapabilities(true),
	)
	s.AddTool(
		mcp.NewTool("analyze",
			mcp.WithDescription("Analyze codebase for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to analyze (e.g., @semio, @semio/js, path/to/file.ts)"), mcp.DefaultString("@semio")),
		),
		analyze,
	)
	s.AddTool(
		mcp.NewTool("fix",
			mcp.WithDescription("Apply autofixes for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to fix"), mcp.DefaultString("@semio")),
		),
		fix,
	)
	s.AddTool(
		mcp.NewTool("policy_list",
			mcp.WithDescription("List all registered policies"),
		),
		policyList,
	)
	s.AddTool(
		mcp.NewTool("policy_check",
			mcp.WithDescription("Check a specific policy"),
			mcp.WithString("id", mcp.Required(), mcp.Description("Policy ID to check")),
			mcp.WithString("scope", mcp.Description("Scope to analyze"), mcp.DefaultString("@semio")),
		),
		policyCheck,
	)
	s.AddTool(
		mcp.NewTool("ticket_open",
			mcp.WithDescription("Open a new development ticket"),
			mcp.WithString("title", mcp.Required(), mcp.Description("Ticket title (will be uppercased and kebab-cased for folder name)")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Ticket prompt/description")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM used for this ticket")),
			mcp.WithString("ui", mcp.Required(), mcp.Description("UI used for this ticket")),
			mcp.WithBoolean("noIssue", mcp.Description("Skip GitHub issue creation")),
			mcp.WithString("planPath", mcp.Description("Optional plan file path to seed ticket plan.md")),
		),
		ticketOpen,
	)
	s.AddTool(
		mcp.NewTool("ticket_list",
			mcp.WithDescription("List development tickets"),
			mcp.WithNumber("year", mcp.Description("Filter by year")),
			mcp.WithNumber("month", mcp.Description("Filter by month")),
			mcp.WithNumber("day", mcp.Description("Filter by day")),
		),
		ticketList,
	)
	s.AddTool(
		mcp.NewTool("ticket_read",
			mcp.WithDescription("Read a specific ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketRead,
	)
	s.AddTool(
		mcp.NewTool("ticket_close",
			mcp.WithDescription("Close a ticket with summary and affected file changes"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("summary", mcp.Required(), mcp.Description("Summary of the ticket work")),
			mcp.WithArray("files", mcp.Description("Files to include (at least one required)")),
			mcp.WithString("title", mcp.Description("New title for the ticket (also updates GitHub issue)")),
		),
		ticketClose,
	)
	s.AddTool(
		mcp.NewTool("ticket_reopen",
			mcp.WithDescription("Reopen a closed ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("New prompt/description for the ticket")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM used for this ticket")),
			mcp.WithString("title", mcp.Description("New title for the ticket (also updates GitHub issue)")),
		),
		ticketReopen,
	)
	s.AddTool(
		mcp.NewTool("contributor_add",
			mcp.WithDescription("Add a contributor by GitHub username"),
			mcp.WithString("github", mcp.Required(), mcp.Description("GitHub username")),
		),
		contributorAdd,
	)
	s.AddTool(
		mcp.NewTool("contributor_list",
			mcp.WithDescription("List all contributors"),
		),
		contributorList,
	)
	s.AddTool(
		mcp.NewTool("contributor_remove",
			mcp.WithDescription("Remove a contributor"),
			mcp.WithString("github", mcp.Required(), mcp.Description("GitHub username")),
		),
		contributorRemove,
	)
	s.AddTool(
		mcp.NewTool("project_list",
			mcp.WithDescription("List Nx bundles in the monorepo"),
		),
		projectList,
	)
	s.AddTool(
		mcp.NewTool("project_tree",
			mcp.WithDescription("Show bundle dependency tree"),
		),
		projectTree,
	)
	s.AddTool(
		mcp.NewTool("folder_create",
			mcp.WithDescription("Create a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to create")),
		),
		folderCreate,
	)
	s.AddTool(
		mcp.NewTool("folder_move",
			mcp.WithDescription("Move a folder"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source folder path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target folder path")),
		),
		folderMove,
	)
	s.AddTool(
		mcp.NewTool("folder_delete",
			mcp.WithDescription("Delete a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to delete")),
		),
		folderDelete,
	)
	s.AddTool(
		mcp.NewTool("folder_list",
			mcp.WithDescription("List folders in a path"),
			mcp.WithString("path", mcp.Description("Path to list folders from"), mcp.DefaultString(".")),
		),
		folderList,
	)
	s.AddTool(
		mcp.NewTool("folder_tree",
			mcp.WithDescription("Show folder tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		folderTree,
	)
	s.AddTool(
		mcp.NewTool("file_create",
			mcp.WithDescription("Create a file with appropriate header"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to create")),
		),
		fileCreate,
	)
	s.AddTool(
		mcp.NewTool("file_move",
			mcp.WithDescription("Move a file"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source file path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target file path")),
		),
		fileMove,
	)
	s.AddTool(
		mcp.NewTool("file_delete",
			mcp.WithDescription("Delete a file"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to delete")),
		),
		fileDelete,
	)
	s.AddTool(
		mcp.NewTool("file_list",
			mcp.WithDescription("List files in scope"),
			mcp.WithString("scope", mcp.Description("Scope to list files from"), mcp.DefaultString("@semio")),
		),
		fileList,
	)
	s.AddTool(
		mcp.NewTool("file_tree",
			mcp.WithDescription("Show file tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		fileTree,
	)
	s.AddTool(
		mcp.NewTool("section_create",
			mcp.WithDescription("Create a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionCreate,
	)
	s.AddTool(
		mcp.NewTool("section_move",
			mcp.WithDescription("Rename a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("old_name", mcp.Required(), mcp.Description("Old section name")),
			mcp.WithString("new_name", mcp.Required(), mcp.Description("New section name")),
		),
		sectionMove,
	)
	s.AddTool(
		mcp.NewTool("section_delete",
			mcp.WithDescription("Delete a section from a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionDelete,
	)
	s.AddTool(
		mcp.NewTool("section_list",
			mcp.WithDescription("List sections in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionList,
	)
	s.AddTool(
		mcp.NewTool("section_tree",
			mcp.WithDescription("Show section tree in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionTree,
	)
	s.AddTool(
		mcp.NewTool("integrate",
			mcp.WithDescription("Integrate source code into a target file section"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source file path")),
			mcp.WithString("target_section", mcp.Required(), mcp.Description("Target section name")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Target file path")),
			mcp.WithString("target_parent_section", mcp.Description("Optional target parent section name")),
		),
		sectionIntegrate,
	)
	s.AddTool(
		mcp.NewTool("definition_list",
			mcp.WithDescription("List definitions in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		definitionList,
	)
	s.AddTool(
		mcp.NewTool("graphql",
			mcp.WithDescription("Execute a GraphQL query against the repo schema"),
			mcp.WithString("query", mcp.Required(), mcp.Description("GraphQL query string")),
			mcp.WithString("variables", mcp.Description("JSON object with query variables")),
		),
		graphqlQuery,
	)
	if err := server.ServeStdio(s); err != nil {
		return err
	}
	return nil
}

func textResult(text string) *mcp.CallToolResult {
	return mcp.NewToolResultText(text)
}

// #region Args
func getArgs(request mcp.CallToolRequest) map[string]interface{} {
	if args, ok := request.Params.Arguments.(map[string]interface{}); ok {
		return args
	}
	return make(map[string]interface{})
}

func getStringArg(args map[string]interface{}, key string) (string, bool, error) {
	value, ok := args[key]
	if !ok {
		return "", false, nil
	}
	str, ok := value.(string)
	if !ok || str == "" {
		return "", true, fmt.Errorf("invalid %s", key)
	}
	return str, true, nil
}

func requireStringArg(args map[string]interface{}, key string) (string, error) {
	value, ok, err := getStringArg(args, key)
	if err != nil {
		return "", err
	}
	if !ok {
		return "", fmt.Errorf("missing %s", key)
	}
	return value, nil
}

func getIntArg(args map[string]interface{}, key string) (int, bool, error) {
	value, ok := args[key]
	if !ok {
		return 0, false, nil
	}
	number, ok := value.(float64)
	if !ok || number != math.Trunc(number) {
		return 0, true, fmt.Errorf("invalid %s", key)
	}
	return int(number), true, nil
}

func requireIntArg(args map[string]interface{}, key string) (int, error) {
	value, ok, err := getIntArg(args, key)
	if err != nil {
		return 0, err
	}
	if !ok {
		return 0, fmt.Errorf("missing %s", key)
	}
	return value, nil
}

func getStringSliceArg(args map[string]interface{}, key string) ([]string, bool, error) {
	value, ok := args[key]
	if !ok {
		return nil, false, nil
	}
	list, ok := value.([]interface{})
	if !ok || len(list) == 0 {
		return nil, true, fmt.Errorf("invalid %s", key)
	}
	result := make([]string, 0, len(list))
	for _, item := range list {
		str, ok := item.(string)
		if !ok || str == "" {
			return nil, true, fmt.Errorf("invalid %s", key)
		}
		result = append(result, str)
	}
	return result, true, nil
}

func getBoolArg(args map[string]interface{}, key string) (bool, bool, error) {
	value, ok := args[key]
	if !ok {
		return false, false, nil
	}
	boolVal, ok := value.(bool)
	if !ok {
		return false, true, fmt.Errorf("invalid %s", key)
	}
	return boolVal, true, nil
}

// #endregion Args

// #region Paths
func requireFilePath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid file path: %s", path)
	}
	if info.IsDir() {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

func requireFolderPath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	if !info.IsDir() {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

func requireFileTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if info.IsDir() {
			return fmt.Errorf("invalid file path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

func requireFolderTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if !info.IsDir() {
			return fmt.Errorf("invalid folder path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// #endregion Paths

// #region GraphQL

func gql(query string, variables map[string]interface{}) (string, error) {
	return executor.ExecuteJSON(context.Background(), query, variables)
}

// #endregion GraphQL

// #region Handlers
func analyze(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	query := `query Analyze($scope: String) {
		analyze(scope: $scope) {
			violations {
				id
				kindId
				kind {
					id
					priority
					autofixable
					reason
					solution
				}
				scope
				excerpt
			}
			metrics {
				total
				byPriority {
					high
					medium
					low
				}
				autofixable
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fix(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	query := `mutation Fix($scope: String) {
		fix(scope: $scope) {
			fixed
			remaining
		}
	}`
	result, err := gql(query, map[string]interface{}{"scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func policyList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Policies {
		repo {
			policies {
				id
				name
				description
				scopes
				violationKinds {
					id
					priority
					autofixable
					reason
					solution
				}
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func policyCheck(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	query := `query PolicyCheck($id: String!, $scope: String) {
		policy(id: $id) {
			id
			name
			description
			scopes
			violationKinds {
				id
				priority
				autofixable
				reason
				solution
			}
		}
		analyze(scope: $scope) {
			violations {
				id
				kindId
				scope
				excerpt
			}
			metrics {
				total
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"id": id, "scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketOpen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	title, err := requireStringArg(args, "title")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	llm, err := requireStringArg(args, "llm")
	if err != nil {
		return nil, err
	}
	ui, err := requireStringArg(args, "ui")
	if err != nil {
		return nil, err
	}
	input := map[string]interface{}{
		"title":  title,
		"prompt": prompt,
		"llm":    llm,
		"ui":     ui,
	}
	if planPath, ok, _ := getStringArg(args, "planPath"); ok && planPath != "" {
		input["planPath"] = planPath
	}
	if noIssue, ok, _ := getBoolArg(args, "noIssue"); ok {
		input["noIssue"] = noIssue
	}
	query := `mutation TicketOpen($input: TicketOpenInput!) {
		ticketOpen(input: $input) {
			id
			slug
			prompt
			status
		}
	}`
	result, err := gql(query, map[string]interface{}{"input": input})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, yearOk, err := getIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, monthOk, err := getIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, dayOk, err := getIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	if monthOk && !yearOk {
		return nil, fmt.Errorf("missing year")
	}
	if dayOk && !monthOk {
		return nil, fmt.Errorf("missing month")
	}
	variables := make(map[string]interface{})
	if yearOk {
		variables["year"] = year
	}
	if monthOk {
		variables["month"] = month
	}
	if dayOk {
		variables["day"] = day
	}
	query := `query Tickets($year: Int, $month: Int, $day: Int) {
		repo {
			tickets(year: $year, month: $month, day: $day) {
				id
				year
				month
				day
				slug
				prompt
				summary
				status
				llm
			}
		}
	}`
	result, err := gql(query, variables)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketRead(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	query := `query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) {
		ticket(year: $year, month: $month, day: $day, slug: $slug) {
			id
			year
			month
			day
			slug
			prompt
			summary
			status
			llm
			commit
			date {
				created
				finished
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"year": year, "month": month, "day": day, "slug": slug,
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketClose(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	summary, err := requireStringArg(args, "summary")
	if err != nil {
		return nil, err
	}
	files, _, err := getStringSliceArg(args, "files")
	if err != nil {
		return nil, err
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}
	for _, file := range files {
		if err := requireFilePath(file); err != nil {
			return nil, err
		}
	}
	input := map[string]interface{}{
		"year":    year,
		"month":   month,
		"day":     day,
		"slug":    slug,
		"summary": summary,
		"files":   files,
	}
	// Add optional title if provided
	if title, ok, _ := getStringArg(args, "title"); ok && title != "" {
		input["title"] = title
	}
	query := `mutation TicketClose($input: TicketCloseInput!) {
		ticketClose(input: $input) {
			id
			slug
			status
		}
	}`
	result, err := gql(query, map[string]interface{}{"input": input})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketReopen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	llm, err := requireStringArg(args, "llm")
	if err != nil {
		return nil, err
	}
	input := map[string]interface{}{
		"year":   year,
		"month":  month,
		"day":    day,
		"slug":   slug,
		"prompt": prompt,
		"llm":    llm,
	}
	// Add optional title if provided
	if title, ok, _ := getStringArg(args, "title"); ok && title != "" {
		input["title"] = title
	}
	query := `mutation TicketReopen($input: TicketReopenInput!) {
		ticketReopen(input: $input) {
			id
			slug
			status
		}
	}`
	result, err := gql(query, map[string]interface{}{"input": input})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func contributorAdd(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	query := `mutation ContributorAdd($input: ContributorAddInput!) {
		contributorAdd(input: $input) {
			id
			github
			name
			emails
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"github": github},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func contributorList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Contributors {
		repo {
			contributors {
				id
				github
				name
				emails
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func contributorRemove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	query := `mutation ContributorRemove($input: ContributorRemoveInput!) {
		contributorRemove(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"github": github},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func projectList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Bundles {
		repo {
			bundles {
				id
				name
				root
				projectType
				tags
				uri
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func projectTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Bundles {
		repo {
			bundles {
				id
				name
				root
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func folderCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFolderTargetPath(path); err != nil {
		return nil, err
	}
	query := `mutation FolderCreate($input: FolderCreateInput!) {
		folderCreate(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func folderMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	if err := requireFolderPath(source); err != nil {
		return nil, err
	}
	if err := requireFolderTargetPath(target); err != nil {
		return nil, err
	}
	query := `mutation FolderMove($input: FolderMoveInput!) {
		folderMove(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"source": source, "target": target},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func folderDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	query := `mutation FolderDelete($input: FolderDeleteInput!) {
		folderDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func folderList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func folderTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fileCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFileTargetPath(path); err != nil {
		return nil, err
	}
	query := `mutation FileCreate($input: FileCreateInput!) {
		fileCreate(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fileMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(source); err != nil {
		return nil, err
	}
	if err := requireFileTargetPath(target); err != nil {
		return nil, err
	}
	query := `mutation FileMove($input: FileMoveInput!) {
		fileMove(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"source": source, "target": target},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fileDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(path); err != nil {
		return nil, err
	}
	query := `mutation FileDelete($input: FileDeleteInput!) {
		fileDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fileList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	query := `query Bundle($name: String!) {
		bundle(name: $name) {
			id
			name
			root
		}
	}`
	result, err := gql(query, map[string]interface{}{"name": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func fileTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `mutation SectionCreate($input: SectionCreateInput!) {
		sectionCreate(input: $input) {
			id
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "name": section},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	oldName, err := requireStringArg(args, "old_name")
	if err != nil {
		return nil, err
	}
	newName, err := requireStringArg(args, "new_name")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `mutation SectionMove($input: SectionMoveInput!) {
		sectionMove(input: $input) {
			id
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "oldName": oldName, "newName": newName},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `mutation SectionDelete($input: SectionDeleteInput!) {
		sectionDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "name": section},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			sections {
				id
				name
				range { start end }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			sections {
				id
				name
				range { start end }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionIntegrate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetSection, err := requireStringArg(args, "target_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	targetParentSection, _, err := getStringArg(args, "target_parent_section")
	if err != nil {
		return nil, err
	}

	if err := requireFilePath(source); err != nil {
		return nil, err
	}
	if err := requireFilePath(targetFile); err != nil {
		return nil, err
	}

	query := `mutation Integrate($input: IntegrateInput!) {
		integrate(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{
			"sourcePath":              source,
			"targetSectionName":       targetSection,
			"targetFilePath":          targetFile,
			"targetParentSectionName": targetParentSection,
		},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func definitionList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			definitions {
				id
				name
				kind
				range { start end }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func graphqlQuery(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	query, err := requireStringArg(args, "query")
	if err != nil {
		return nil, err
	}
	variablesStr, _, err := getStringArg(args, "variables")
	if err != nil {
		return nil, err
	}
	var variables map[string]interface{}
	if variablesStr != "" {
		if err := json.Unmarshal([]byte(variablesStr), &variables); err != nil {
			return nil, fmt.Errorf("invalid variables JSON: %w", err)
		}
	}
	result, gqlErr := gql(query, variables)
	if gqlErr != nil {
		return textResult(fmt.Sprintf(`{"error": %q}`, gqlErr.Error())), nil
	}
	return textResult(result), nil
}

// #endregion Handlers

// #endregion Mcp

// #region Cli

// #region Init
// #endregion Init

// #region GraphQL Helpers

func printGQL(query string, variables map[string]interface{}) error {
	result, err := gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// #endregion GraphQL Helpers

// #region Commands

func main() {
	if err := Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func Execute() error {
	return rootCmd.Execute()
}

var rootCmd = &cobra.Command{
	Use:   "repo",
	Short: "Monorepo CLI for Semio",
	Long:  `repo - Monorepo CLI for Semio. All commands output JSON via GraphQL.`,
}

var mcpCmd = &cobra.Command{
	Use:   "mcp",
	Short: "Run MCP server",
	RunE:  runMcpServer,
}

func init() {
	rootCmd.AddCommand(mcpCmd)
	rootCmd.AddCommand(graphqlCmd)
	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(fixCmd)
	rootCmd.AddCommand(policyCmd)
	rootCmd.AddCommand(ticketCmd)
	rootCmd.AddCommand(contributorCmd)
	rootCmd.AddCommand(bundleCmd)
	rootCmd.AddCommand(folderCmd)
	rootCmd.AddCommand(fileCmd)
	rootCmd.AddCommand(sectionCmd)
}

// #region GraphQL Command

var graphqlCmd = &cobra.Command{
	Use:   "graphql <query>",
	Short: "Execute a GraphQL query",
	Long:  `Execute a GraphQL query against the repo schema and return JSON result.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		query := args[0]
		variablesJSON, _ := cmd.Flags().GetString("variables")
		var variables map[string]interface{}
		if variablesJSON != "" {
			if err := json.Unmarshal([]byte(variablesJSON), &variables); err != nil {
				return fmt.Errorf("invalid variables JSON: %w", err)
			}
		}

		// Support JSON encoded query/variables (urql style)
		var payload struct {
			Query     string                 `json:"query"`
			Variables map[string]interface{} `json:"variables"`
		}
		if err := json.Unmarshal([]byte(query), &payload); err == nil && payload.Query != "" {
			query = payload.Query
			if variables == nil {
				variables = make(map[string]interface{})
			}
			for k, v := range payload.Variables {
				if _, exists := variables[k]; !exists {
					variables[k] = v
				}
			}
		}

		return printGQL(query, variables)
	},
}

func init() {
	graphqlCmd.Flags().StringP("variables", "v", "", "JSON object with query variables")
}

// #endregion GraphQL Command

// #region Serve Command

var devCmd = &cobra.Command{
	Use:   "dev",
	Short: "Start GraphQL server with GraphiQL interface",
	Long:  `Start an HTTP server exposing the GraphQL API with introspection and GraphiQL interface.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		port, _ := cmd.Flags().GetInt("port")
		addr := fmt.Sprintf(":%d", port)

		http.HandleFunc("/graphql", graphqlHandler)
		http.HandleFunc("/", graphiqlHandler)

		fmt.Printf("GraphQL server running at http://localhost%s/graphql\n", addr)
		fmt.Printf("GraphiQL interface at http://localhost%s/\n", addr)
		return http.ListenAndServe(addr, nil)
	},
}

func graphqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

	if r.Method == http.MethodOptions {
		w.WriteHeader(http.StatusOK)
		return
	}

	var request struct {
		Query         string                 `json:"query"`
		Variables     map[string]interface{} `json:"variables"`
		OperationName string                 `json:"operationName"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, `{"errors":[{"message":"Invalid JSON"}]}`, http.StatusBadRequest)
		return
	}

	result, err := executor.Execute(r.Context(), request.Query, request.Variables)
	response := map[string]interface{}{}
	if err != nil {
		response["errors"] = []map[string]string{{"message": err.Error()}}
	} else {
		response["data"] = result
	}

	json.NewEncoder(w).Encode(response)
}

func graphiqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte(graphiqlHTML))
}

const graphiqlHTML = `<!DOCTYPE html>
<html>
<head>
  <title>GraphiQL - Semio Repo</title>
  <style>
    body { height: 100%; margin: 0; width: 100%; overflow: hidden; }
    #graphiql { height: 100vh; }
  </style>
  <script src="https://cdn.jsdelivr.net/npm/react@18.2.0/umd/react.production.min.js" crossorigin></script>
  <script src="https://cdn.jsdelivr.net/npm/react-dom@18.2.0/umd/react-dom.production.min.js" crossorigin></script>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphiql@3.0.10/graphiql.min.css" />
</head>
<body>
  <div id="graphiql">Loading...</div>
  <script src="https://cdn.jsdelivr.net/npm/graphiql@3.0.10/graphiql.min.js" crossorigin></script>
  <script>
    const root = ReactDOM.createRoot(document.getElementById('graphiql'));
    root.render(
      React.createElement(GraphiQL, {
        fetcher: async (graphQLParams) => {
          const response = await fetch('/graphql', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(graphQLParams),
          });
          return response.json();
        },
      }),
    );
  </script>
</body>
</html>`

func init() {
	devCmd.Flags().IntP("port", "p", 8080, "Port to listen on")
	rootCmd.AddCommand(devCmd)
}

// #endregion Serve Command

// #region Analyze Command

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		if scope == nil {
			ctx := NewCodebaseContext()
			ctx.LoadBundles()
			if err := ctx.LoadFiles(); err != nil {
				return err
			}
			if err := ctx.LoadViolations(); err != nil {
				return err
			}
			if err := ctx.LoadTickets(); err != nil {
				return err
			}
			ctx.LoadPolicies()
			codebase := BuildCodebase(ctx)
			reportPath := filepath.Join(GetRepoMetaDir(), "reports", "codebase.json")
			if err := WriteJSONFile(reportPath, codebase); err != nil {
				return err
			}
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
			query Analyze($scope: String) {
				analyze(scope: $scope) {
					violations {
						id
						summary
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
					}
					metrics { total autofixable byPriority { high medium low } }
				}
			}
		`, variables)
	},
}

// #endregion Analyze Command

// #region Fix Command

var fixCmd = &cobra.Command{
	Use:   "fix [scope]",
	Short: "Apply autofixes for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
			mutation Fix($scope: String) {
				fix(scope: $scope) {
					fixed
					remaining
					violations {
						id
						summary
						scope
						excerpt
						line
					}
				}
			}
		`, variables)
	},
}

// #endregion Fix Command

// #region Export Command

var exportCmd = &cobra.Command{
	Use:   "export [output]",
	Short: "Export repo data to SQLite database",
	Long:  `Export all repo data (bundles, folders, files, sections, contributors, tickets, policies, violations) to a SQLite database file.`,
	Args:  cobra.MaximumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		outputPath := ""
		if len(args) > 0 {
			outputPath = args[0]
		}
		ctx := NewRepoContext(findRepoRoot("."))
		result, err := ExportToSQLite(outputPath, ctx)
		if err != nil {
			return err
		}
		jsonBytes, err := json.MarshalIndent(result, "", "  ")
		if err != nil {
			return err
		}
		fmt.Println(string(jsonBytes))
		return nil
	},
}

func init() {
	rootCmd.AddCommand(exportCmd)
}

// #endregion Export Command

// #region Policy Commands

var policyCmd = &cobra.Command{
	Use:   "policy",
	Short: "Policy management commands",
}

var policyListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all registered policies",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
			query Policies {
				repo {
					policies {
						id
						name
						description
						scopes
						violationKinds { id priority autofixable reason solution }
					}
				}
			}
		`, nil)
	},
}

func init() {
	policyCmd.AddCommand(policyListCmd)
}

// #endregion Policy Commands

// #region Ticket Commands

var ticketCmd = &cobra.Command{
	Use:   "ticket",
	Short: "Ticket management commands",
}

var ticketOpenCmd = &cobra.Command{
	Use:   "open [title] [prompt] [llm] [ui]",
	Short: "Open a new ticket",
	Long: `Open a new ticket. Supports two syntaxes:

Positional: repo ticket open <title> <prompt> <llm> <ui>
Explicit:   repo ticket open --title <title> --prompt <prompt> --llm <llm> --ui <ui>

LLM values: opus-4-5, claude-sonnet-4, haiku-4-5, gemini-3-pro, gpt-5-2, gpt-5-2-codex
UI values:  claude-code, cursor, copilot-chat, antigravity, codex, droid`,
	Args: cobra.MaximumNArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		noIssue, _ := cmd.Flags().GetBool("no-issue")

		// Get title from positional arg or flag
		title, _ := cmd.Flags().GetString("title")
		if title == "" && len(args) > 0 {
			title = args[0]
		}
		if title == "" {
			return fmt.Errorf("title is required (positional arg or --title flag)")
		}

		// Get prompt from positional arg or flag
		prompt, _ := cmd.Flags().GetString("prompt")
		if prompt == "" && len(args) > 1 {
			prompt = args[1]
		}
		if prompt == "" {
			prompt = title
		}
		if prompt == "" {
			return fmt.Errorf("prompt is required (positional arg or --prompt flag)")
		}

		// Get LLM from positional arg or flag
		llm, _ := cmd.Flags().GetString("llm")
		if llm == "" && len(args) > 2 {
			llm = args[2]
		}
		if llm == "" {
			return fmt.Errorf("llm is required (positional arg or --llm flag): opus-4-5, claude-sonnet-4, haiku-4-5, gemini-3-pro, gpt-5-2, gpt-5-2-codex")
		}

		// Get UI from positional arg or flag
		ui, _ := cmd.Flags().GetString("ui")
		if ui == "" && len(args) > 3 {
			ui = args[3]
		}
		if ui == "" {
			return fmt.Errorf("ui is required (positional arg or --ui flag): claude-code, cursor, copilot-chat, antigravity, codex, droid")
		}

		// Map lowercase UI values to GraphQL enum names
		uiMap := map[string]string{
			"claude-code":  "CLAUDE_CODE",
			"cursor":       "CURSOR",
			"copilot-chat": "COPILOT_CHAT",
			"antigravity":  "ANTIGRAVITY",
			"codex":        "CODEX",
			"droid":        "DROID",
		}
		uiEnum, ok := uiMap[ui]
		if !ok {
			return fmt.Errorf("invalid ui value: %s (valid: claude-code, cursor, copilot-chat, antigravity, codex, droid)", ui)
		}

		input := map[string]interface{}{
			"title":   title,
			"prompt":  prompt,
			"llm":     llm,
			"ui":      uiEnum,
			"noIssue": noIssue,
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketOpen($input: TicketOpenInput!) {
				ticketOpen(input: $input) {
					id
					slug
					status
					path
					uri
				}
			}
		`, variables)
	},
}

func init() {
	ticketOpenCmd.Flags().Bool("no-issue", false, "Do not create a GitHub issue")
	ticketOpenCmd.Flags().String("title", "", "Ticket title")
	ticketOpenCmd.Flags().String("prompt", "", "Ticket prompt/description")
	ticketOpenCmd.Flags().String("llm", "", "LLM (opus-4-5, claude-sonnet-4, haiku-4-5, gemini-3-pro, gpt-5-2, gpt-5-2-codex)")
	ticketOpenCmd.Flags().String("ui", "", "UI (claude-code, cursor, copilot-chat, antigravity, codex, droid)")
	ticketCmd.AddCommand(ticketOpenCmd)
}

var ticketListCmd = &cobra.Command{
	Use:   "list [year] [month] [day]",
	Short: "List tickets",
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{}
		if len(args) > 0 {
			y, _ := strconv.Atoi(args[0])
			variables["year"] = y
		}
		if len(args) > 1 {
			m, _ := strconv.Atoi(args[1])
			variables["month"] = m
		}
		if len(args) > 2 {
			d, _ := strconv.Atoi(args[2])
			variables["day"] = d
		}
		return printGQL(`
			query Tickets($year: Int, $month: Int, $day: Int) {
				repo {
					tickets(year: $year, month: $month, day: $day) {
						id
						year
						month
						day
						slug
						status
						prompt
						summary
						path
						uri
						date { created finished }
					}
				}
			}
		`, variables)
	},
}

var ticketCloseCmd = &cobra.Command{
	Use:   "close <YYYY/MM/DD/SLUG> <summary> <files...>",
	Short: "Close a ticket with summary and files",
	Args:  cobra.MinimumNArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		// Parse YYYY/MM/DD/SLUG path format
		path := args[0]
		summary := args[1]
		files := args[2:]
		year, month, day, slug, err := parseTicketPath(path)
		if err != nil {
			return err
		}
		input := map[string]interface{}{
			"year":    year,
			"month":   month,
			"day":     day,
			"slug":    slug,
			"summary": summary,
			"files":   files,
		}
		// Add optional title if provided
		if title, _ := cmd.Flags().GetString("title"); title != "" {
			input["title"] = title
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketClose($input: TicketCloseInput!) {
				ticketClose(input: $input) {
					id
					slug
					status
					date { created finished }
				}
			}
		`, variables)
	},
}

func init() {
	ticketCloseCmd.Flags().String("title", "", "New title for the ticket (also updates GitHub issue)")
}

var ticketReopenCmd = &cobra.Command{
	Use:   "reopen <YYYY/MM/DD/SLUG> <prompt> <llm>",
	Short: "Reopen a ticket",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		// Parse YYYY/MM/DD/SLUG path format
		path := args[0]
		year, month, day, slug, err := parseTicketPath(path)
		if err != nil {
			return err
		}
		input := map[string]interface{}{
			"year":   year,
			"month":  month,
			"day":    day,
			"slug":   slug,
			"prompt": args[1],
			"llm":    args[2],
		}
		// Add optional title if provided
		if title, _ := cmd.Flags().GetString("title"); title != "" {
			input["title"] = title
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketReopen($input: TicketReopenInput!) {
				ticketReopen(input: $input) {
					id
					slug
					status
				}
			}
		`, variables)
	},
}

func init() {
	ticketReopenCmd.Flags().String("title", "", "New title for the ticket (also updates GitHub issue)")
}

// parseTicketPath parses a ticket path in YYYY/MM/DD/SLUG format
func parseTicketPath(path string) (year, month, day int, slug string, err error) {
	parts := filepath.SplitList(path)
	if len(parts) == 1 {
		// Try splitting by forward slash
		parts = nil
		for _, p := range filepath.SplitList(path) {
			parts = append(parts, p)
		}
		// Manual split by /
		parts = splitPath(path)
	}
	if len(parts) != 4 {
		return 0, 0, 0, "", fmt.Errorf("invalid ticket path format, expected YYYY/MM/DD/SLUG, got: %s", path)
	}
	year, err = strconv.Atoi(parts[0])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid year in path: %s", parts[0])
	}
	month, err = strconv.Atoi(parts[1])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid month in path: %s", parts[1])
	}
	day, err = strconv.Atoi(parts[2])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid day in path: %s", parts[2])
	}
	slug = parts[3]
	return
}

// splitPath splits a path by / or \
func splitPath(path string) []string {
	var parts []string
	current := ""
	for _, c := range path {
		if c == '/' || c == '\\' {
			if current != "" {
				parts = append(parts, current)
				current = ""
			}
		} else {
			current += string(c)
		}
	}
	if current != "" {
		parts = append(parts, current)
	}
	return parts
}

func init() {
	ticketCmd.AddCommand(ticketOpenCmd)
	ticketCmd.AddCommand(ticketListCmd)
	ticketCmd.AddCommand(ticketCloseCmd)
	ticketCmd.AddCommand(ticketReopenCmd)
}

// #endregion Ticket Commands

// #region Contributor Commands

var contributorCmd = &cobra.Command{
	Use:   "contributor",
	Short: "Contributor management commands",
}

var contributorListCmd = &cobra.Command{
	Use:   "list",
	Short: "List contributors",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
			query Contributors {
				repo {
					contributors {
						id
						github
						name
						emails
						icons { avatar avatarRound github }
						links { name url }
						metrics { commits tickets bundles folders files sections definitions lines }
					}
				}
			}
		`, nil)
	},
}

var contributorAddCmd = &cobra.Command{
	Use:   "add <github>",
	Short: "Add a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		name, _ := cmd.Flags().GetString("name")
		emails, _ := cmd.Flags().GetStringSlice("email")
		input := map[string]interface{}{
			"github": args[0],
		}
		if name != "" {
			input["name"] = name
		}
		if len(emails) > 0 {
			input["emails"] = emails
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation ContributorAdd($input: ContributorAddInput!) {
				contributorAdd(input: $input) {
					id
					github
					name
					emails
				}
			}
		`, variables)
	},
}

var contributorRemoveCmd = &cobra.Command{
	Use:   "remove <github>",
	Short: "Remove a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"github": args[0],
		}
		return printGQL(`
			mutation ContributorRemove($github: String!) {
				contributorRemove(github: $github)
			}
		`, variables)
	},
}

func init() {
	contributorAddCmd.Flags().String("name", "", "Contributor name")
	contributorAddCmd.Flags().StringSlice("email", nil, "Contributor emails")
	contributorCmd.AddCommand(contributorListCmd)
	contributorCmd.AddCommand(contributorAddCmd)
	contributorCmd.AddCommand(contributorRemoveCmd)
}

// #endregion Contributor Commands

// #region Bundle Commands

var bundleCmd = &cobra.Command{
	Use:   "bundle",
	Short: "Bundle management commands",
}

var bundleListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Nx bundles",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
			query Bundles {
				repo {
					bundles {
						id
						name
						root
						sourceRoot
						projectType
						tags
						uri
					}
				}
			}
		`, nil)
	},
}

func init() {
	bundleCmd.AddCommand(bundleListCmd)
}

// #endregion Bundle Commands

// #region Folder Commands

var folderCmd = &cobra.Command{
	Use:   "folder",
	Short: "Folder management commands",
}

var folderCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FolderCreate($path: String!) {
				folderCreate(path: $path) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var folderMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a folder",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"src": args[0],
			"dst": args[1],
		}
		return printGQL(`
			mutation FolderMove($src: String!, $dst: String!) {
				folderMove(src: $src, dst: $dst) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var folderDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FolderDelete($path: String!) {
				folderDelete(path: $path)
			}
		`, variables)
	},
}

func init() {
	folderCmd.AddCommand(folderCreateCmd)
	folderCmd.AddCommand(folderMoveCmd)
	folderCmd.AddCommand(folderDeleteCmd)
}

// #endregion Folder Commands

// #region File Commands

var fileCmd = &cobra.Command{
	Use:   "file",
	Short: "File management commands",
}

var fileCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FileCreate($path: String!) {
				fileCreate(path: $path) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var fileMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"src": args[0],
			"dst": args[1],
		}
		return printGQL(`
			mutation FileMove($src: String!, $dst: String!) {
				fileMove(src: $src, dst: $dst) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var fileDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FileDelete($path: String!) {
				fileDelete(path: $path)
			}
		`, variables)
	},
}

func init() {
	fileCmd.AddCommand(fileCreateCmd)
	fileCmd.AddCommand(fileMoveCmd)
	fileCmd.AddCommand(fileDeleteCmd)
}

// #endregion File Commands

// #region Section Commands

var sectionCmd = &cobra.Command{
	Use:   "section",
	Short: "Section management commands",
}

var sectionCreateCmd = &cobra.Command{
	Use:   "create <file> <name>",
	Short: "Create a section in a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		parent, _ := cmd.Flags().GetString("parent")
		variables := map[string]interface{}{
			"file": args[0],
			"name": args[1],
		}
		if parent != "" {
			variables["parent"] = parent
		}
		return printGQL(`
			mutation SectionCreate($file: String!, $name: String!, $parent: String) {
				sectionCreate(file: $file, name: $name, parent: $parent) {
					id
					name
					range { start end }
				}
			}
		`, variables)
	},
}

var sectionMoveCmd = &cobra.Command{
	Use:   "move <file> <old-name> <new-name>",
	Short: "Move/rename a section",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"file":    args[0],
			"oldName": args[1],
			"newName": args[2],
		}
		return printGQL(`
			mutation SectionMove($file: String!, $oldName: String!, $newName: String!) {
				sectionMove(file: $file, oldName: $oldName, newName: $newName) {
					id
					name
					range { start end }
				}
			}
		`, variables)
	},
}

var sectionDeleteCmd = &cobra.Command{
	Use:   "delete <file> <name>",
	Short: "Delete a section from a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"file": args[0],
			"name": args[1],
		}
		return printGQL(`
			mutation SectionDelete($file: String!, $name: String!) {
				sectionDelete(file: $file, name: $name)
			}
		`, variables)
	},
}

var integrateCmd = &cobra.Command{
	Use:   "integrate <source> <target-section-name> <target-file> [<target-parent-section-name>]",
	Short: "Integrate a source file into a target file's section",
	Args:  cobra.RangeArgs(3, 4),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"source":        args[0],
			"targetSection": args[1],
			"targetFile":    args[2],
		}
		if len(args) == 4 {
			variables["targetParent"] = args[3]
		}
		return printGQL(`
			mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) {
				integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) {
					id
					path
				}
			}
		`, variables)
	},
}

var sectionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List sections in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			query SectionList($path: String!) {
				file(path: $path) {
					sections {
						id
						name
						range { start end }
						children {
							id
							name
							range { start end }
							children {
								id
								name
								range { start end }
								children {
									id
									name
									range { start end }
								}
							}
						}
					}
				}
			}
		`, variables)
	},
}

func init() {
	sectionCreateCmd.Flags().String("parent", "", "Parent section name")
	sectionCmd.AddCommand(sectionListCmd)
	sectionCmd.AddCommand(sectionCreateCmd)
	sectionCmd.AddCommand(sectionMoveCmd)
	sectionCmd.AddCommand(sectionDeleteCmd)
	sectionCmd.AddCommand(integrateCmd)
}

// #endregion Section Commands

// #region Definition Commands

var definitionCmd = &cobra.Command{
	Use:   "definition",
	Short: "Definition management commands",
}

var definitionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List definitions in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			query DefinitionList($path: String!) {
				file(path: $path) {
					definitions {
						id
						name
						kind
						range { start end }
					}
				}
			}
		`, variables)
	},
}

func init() {
	definitionCmd.AddCommand(definitionListCmd)
	rootCmd.AddCommand(definitionCmd)
}

// #endregion Definition Commands

// #endregion Commands

// #region Missing Utilities

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
			files = []string{scope.FilePath}
		}
	}
	if err != nil {
		return nil, err
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

func filterConsideredFiles(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		if normalized == ".semio-repo" || strings.HasPrefix(normalized, ".semio-repo/") {
			continue
		}
		if normalized == "assets/repo" || strings.HasPrefix(normalized, "assets/repo/") {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

func ComputeTicketFiles(ticket *Ticket, files []string) (*TicketDiffs, error) {
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
	files = FilterTicketWorkspaceFiles(ticket, files)
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}

	diffStatuses, err := GetGitDiffStatus(baseCommit, "", files)
	if err != nil {
		return nil, err
	}
	diffLines, err := GetGitDiffLines(baseCommit, "", files)
	if err != nil {
		return nil, err
	}

	currentFiles := make(map[string]struct{})
	baseFiles := make(map[string]struct{})
	for _, file := range files {
		currentFiles[file] = struct{}{}
		baseFiles[file] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.To != "" {
			currentFiles[status.To] = struct{}{}
		}
		if status.From != "" {
			baseFiles[status.From] = struct{}{}
		}
	}
	var currentFileList []string
	for file := range currentFiles {
		currentFileList = append(currentFileList, file)
	}
	var baseFileList []string
	for file := range baseFiles {
		baseFileList = append(baseFileList, file)
	}

	bundles := GetProjects()
	baseCodebase, err := BuildCodebaseSnapshot(baseFileList, bundles, baseCommit)
	if err != nil {
		return nil, err
	}
	currentCodebase, err := BuildCodebaseSnapshot(currentFileList, bundles, "")
	if err != nil {
		return nil, err
	}

	result := BuildSemanticDiffs(baseCodebase, currentCodebase, baseCommit, diffLines, diffStatuses, bundles)
	return result, nil
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
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Fields(line)
			if len(parts) >= 4 {
				newPath := strings.TrimPrefix(parts[3], "b/")
				currentFile = newPath
				if result[currentFile] == nil {
					result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
				}
			}
		} else if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

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

func buildViolationID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("@semio/violations/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("@semio/violations/%s#%d", scope, line)
	}
	return fmt.Sprintf("@semio/violations/%s", scope)
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

func buildFolderID(path string, bundleID *string) string {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	if bundleID != nil && *bundleID != "" {
		bundleName := strings.TrimPrefix(*bundleID, "@semio/")
		return "@semio/" + bundleName + "/" + normalizedPath
	}
	return "@semio/repo/" + normalizedPath
}

func buildFileID(path string, bundleID *string) string {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	if bundleID != nil && *bundleID != "" {
		bundleName := strings.TrimPrefix(*bundleID, "@semio/")
		return "@semio/" + bundleName + "/" + normalizedPath
	}
	return "@semio/repo/" + normalizedPath
}

func buildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 {
		return fileID
	}
	return fileID + "#" + strings.Join(sectionPath, "#")
}

func buildDefinitionID(fileID string, sectionPath []string, name string) string {
	if len(sectionPath) > 0 {
		return fileID + "#" + strings.Join(sectionPath, "#") + "§" + name
	}
	return fileID + "§" + name
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
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						defAddedLines := computeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, TicketSection{
				Name:        sectionPath,
				Range:       &Range{Start: section.StartLine, End: section.EndLine},
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
			return []string{"diff", flag, "-M", baseCommit}
		}
		return append([]string{"diff", flag, "-M", baseCommit, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "-M", baseCommit, headCommit}
	}
	return append([]string{"diff", flag, "-M", baseCommit, headCommit, "--"}, paths...)
}

type GitDiffStatus struct {
	Status string
	From   string
	To     string
}

func GetGitDiffStatus(baseCommit, headCommit string, paths []string) ([]GitDiffStatus, error) {
	if baseCommit == "" {
		return nil, fmt.Errorf("base commit is required")
	}
	args := []string{"diff", "--name-status", "-M", baseCommit}
	if headCommit != "" {
		args = append(args, headCommit)
	}
	if len(paths) > 0 {
		args = append(args, "--")
		args = append(args, paths...)
	}
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff status failed: %s", strings.TrimSpace(stderr))
	}
	var results []GitDiffStatus
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		status := strings.TrimSpace(parts[0])
		if strings.HasPrefix(status, "R") && len(parts) >= 3 {
			results = append(results, GitDiffStatus{Status: "renamed", From: parts[1], To: parts[2]})
			continue
		}
		file := parts[1]
		switch status {
		case "A":
			results = append(results, GitDiffStatus{Status: "added", To: file})
		case "D":
			results = append(results, GitDiffStatus{Status: "deleted", From: file})
		case "M":
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		default:
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		}
	}
	return results, nil
}

// #endregion Missing Utilities

func AnalyzeFile(filePath string, bundles []Bundle) ([]Violation, error) {
	scope := Scope{
		Kind:     ScopeFile,
		FilePath: filePath,
	}
	files := filterConsideredFiles([]string{filePath})
	files = filterGitIgnored(files)
	ctx := NewPolicyContextWithFiles(scope, bundles, files)
	return CheckPoliciesWithContext(ctx, nil)
}

func ParseContributorIdentity(line string) (name, email string, ok bool) {
	re := regexp.MustCompile(`\d{4}\s+(.+?)\s*<([^>]+)>`)
	m := re.FindStringSubmatch(line)
	if m == nil {
		return "", "", false
	}
	return strings.TrimSpace(m[1]), strings.TrimSpace(m[2]), true
}

func findSectionForDefinition(sections []Section, startLine, endLine int, prefix string) string {
	for _, s := range sections {
		if startLine >= s.StartLine && endLine <= s.EndLine {
			p := s.Name
			if prefix != "" {
				p = prefix + "#" + s.Name
			}
			if len(s.Children) > 0 {
				if cp := findSectionForDefinition(s.Children, startLine, endLine, p); cp != "" {
					return cp
				}
			}
			return p
		}
	}
	return prefix
}

func ListContributors() ([]Contributor, error) {
	var result []Contributor
	dir := filepath.Join(GetRepoMetaDir(), "contributors")
	if !FileExists(dir) {
		return result, nil
	}
	entries, _ := os.ReadDir(dir)
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		p := filepath.Join(dir, e.Name(), "contributor.json")
		if !FileExists(p) {
			continue
		}
		content, _ := ReadTextFile(p)
		var c Contributor
		json.Unmarshal([]byte(content), &c)
		if c.Github == "" {
			c.Github = e.Name()
		}
		result = append(result, c)
	}
	return result, nil
}

func GetContributorAvatarPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github, "avatar.png")
}

func GetContributorAvatarRoundPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github, "avatar-round.png")
}

func GetContributorPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github)
}

func CreateContributor(github string) (*Contributor, error) {
	dir := GetContributorPath(github)
	if FileExists(dir) {
		return nil, fmt.Errorf("contributor already exists: %s", github)
	}
	EnsureDir(dir)
	c := Contributor{Github: github}
	data, _ := json.MarshalIndent(c, "", "  ")
	WriteTextFile(filepath.Join(dir, "contributor.json"), string(data))
	return &c, nil
}

func RemoveContributor(github string) error {
	dir := filepath.Join(GetRepoMetaDir(), "contributors", github)
	if !FileExists(dir) {
		return fmt.Errorf("contributor not found: %s", github)
	}
	return os.RemoveAll(dir)
}

func GetRegisteredPolicies() []PolicyDef {
	return GetPolicies()
}

func filterGitIgnored(files []string) []string {
	if len(files) == 0 {
		return files
	}
	relPaths := make([]string, len(files))
	for i, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		relPaths[i] = normalized
	}
	ignored := GetGitIgnoredSet(relPaths)
	filtered := make([]string, 0, len(files))
	for i, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		if normalized != "" && ignored[relPaths[i]] {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

var repoResolverInstance *Resolver

func init() {
	repoResolverInstance = NewResolver(rootDir)
}

// #region Resolver Methods

func (r *Resolver) Bundles(ctx context.Context, repo *Repo) ([]*Bundle, error) {
	return r.Ctx.GetBundles(), nil
}

func (r *Resolver) Folders(ctx context.Context, repo *Repo) ([]*Folder, error) {
	return r.Ctx.GetFolders(), nil
}

func (r *Resolver) Files(ctx context.Context, repo *Repo) ([]*File, error) {
	return r.Ctx.GetFiles(), nil
}

func (r *Resolver) Sections(ctx context.Context, repo *Repo) ([]*Section, error) {
	return r.Ctx.GetSections(), nil
}

func (r *Resolver) Definitions(ctx context.Context, repo *Repo) ([]*Definition, error) {
	return r.Ctx.GetDefinitions(), nil
}

func (r *Resolver) Contributors(ctx context.Context, repo *Repo) ([]*Contributor, error) {
	return r.Ctx.GetContributors()
}

func (r *Resolver) Tickets(ctx context.Context, repo *Repo, year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return r.Ctx.GetTickets(year, month, day, status)
}

func (r *Resolver) Policies(ctx context.Context, repo *Repo) ([]*Policy, error) {
	return r.Ctx.GetPolicies(), nil
}

func (r *Resolver) ViolationKinds(ctx context.Context, repo *Repo) ([]*ViolationKindMeta, error) {
	return r.Ctx.GetViolationKinds(), nil
}

func (r *Resolver) Violations(ctx context.Context, repo *Repo, scope *string) ([]*Violation, error) {
	analysis, err := r.Ctx.Analyze(scope)
	if err != nil {
		return nil, err
	}
	return analysis.Violations, nil
}

// #endregion Resolver Methods

// #region Missing Tool Functions

func ToolAnalyze(scopeRaw string, policyIDs []string) ToolResult {
	scope := ParseScope(scopeRaw)
	bundles := GetProjects()
	violations, err := CheckPolicies(scope, bundles, policyIDs)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}

	byPriority := make(map[string]int)
	for range violations {
		// Assuming v.Kind can map to priority, but for now just counting
		// Use empty map or implement priority logic if needed
	}

	report := AnalyzeReport{
		Timestamp:  time.Now().Format(time.RFC3339),
		Status:     "success",
		Scope:      scopeRaw,
		Violations: violations,
		Summary: Summary{
			Total:      len(violations),
			ByPriority: byPriority,
		},
	}
	output := NewOutput()
	bytes, _ := json.MarshalIndent(report, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: report}
}

func ToolFix(scopeRaw string) ToolResult {
	ctx := NewRepoContext(rootDir)
	res, err := ctx.Fix(&scopeRaw)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}
	output := NewOutput()
	bytes, _ := json.MarshalIndent(res, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: res}
}

func ToolPolicyList() ToolResult {
	policies := GetRegisteredPolicies()
	output := NewOutput()
	bytes, _ := json.MarshalIndent(policies, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: policies}
}

func ToolPolicyCheck(policyID, scopeRaw string) ToolResult {
	return ToolAnalyze(scopeRaw, []string{policyID})
}

func ToolPolicyViolationList(policyID string) ToolResult {
	// This appears to list violations for a specific policy across the whole repo
	return ToolAnalyze("@semio", []string{policyID})
}

// #endregion Missing Tool Functions
