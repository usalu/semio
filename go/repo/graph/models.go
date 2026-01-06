// #region Header

// go/repo/graph/models.go

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

package graph

import (
	"time"
)

// #region Interfaces

type Node interface {
	IsNode()
	GetID() string
}

// #endregion Interfaces

// #region Enums

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

// #endregion Enums

// #region Value Types

type Position struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

type Range struct {
	Start Position `json:"start"`
	End   Position `json:"end"`
}

type RepoMetrics struct {
	Bundles      int `json:"bundles"`
	Folders      int `json:"folders"`
	Files        int `json:"files"`
	Sections     int `json:"sections"`
	Definitions  int `json:"definitions"`
	Lines        int `json:"lines"`
	Contributors int `json:"contributors"`
	Tickets      int `json:"tickets"`
	Violations   int `json:"violations"`
}

type BundleMetrics struct {
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type FolderMetrics struct {
	Files      int `json:"files"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FileMetrics struct {
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
}

type SectionMetrics struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type DefinitionMetrics struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}

type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

type ContributorMetrics struct {
	Commits     int `json:"commits"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
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

type IterationDate struct {
	Started time.Time  `json:"started"`
	Ended   *time.Time `json:"ended,omitempty"`
}

type IterationFiles struct {
	Updated []string `json:"updated"`
	Created []string `json:"created"`
	Removed []string `json:"removed"`
}

type TicketMetrics struct {
	Iterations int          `json:"iterations"`
	Bundles    int          `json:"bundles"`
	Files      int          `json:"files"`
	Lines      *LineMetrics `json:"lines"`
}

type Autofix struct {
	Description string     `json:"description"`
	Edits       []FileEdit `json:"edits"`
}

type FileEdit struct {
	Path  string     `json:"path"`
	Edits []TextEdit `json:"edits"`
}

type TextEdit struct {
	Start   int    `json:"start"`
	End     int    `json:"end"`
	NewText string `json:"newText"`
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

// #endregion Value Types

// #region Entity Types

type Repo struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Path string `json:"path"`
}

func (r *Repo) IsNode()        {}
func (r *Repo) GetID() string  { return r.ID }

type Bundle struct {
	ID          string   `json:"id"`
	Name        string   `json:"name"`
	Root        string   `json:"root"`
	SourceRoot  *string  `json:"sourceRoot,omitempty"`
	ProjectType *string  `json:"projectType,omitempty"`
	Tags        []string `json:"tags"`
	URI         string   `json:"uri"`
}

func (b *Bundle) IsNode()       {}
func (b *Bundle) GetID() string { return b.ID }

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
	ID       string  `json:"id"`
	Name     string  `json:"name"`
	Path     string  `json:"path"`
	FileID   string  `json:"fileId"`
	ParentID *string `json:"parentId,omitempty"`
	Range    *Range  `json:"range"`
}

func (s *Section) IsNode()       {}
func (s *Section) GetID() string { return s.ID }

type Definition struct {
	ID        string         `json:"id"`
	Name      string         `json:"name"`
	Kind      DefinitionKind `json:"kind"`
	FileID    string         `json:"fileId"`
	SectionID *string        `json:"sectionId,omitempty"`
	Range     *Range         `json:"range"`
}

func (d *Definition) IsNode()       {}
func (d *Definition) GetID() string { return d.ID }

type Contributor struct {
	ID      string              `json:"id"`
	Github  string              `json:"github"`
	Name    *string             `json:"name,omitempty"`
	Emails  []string            `json:"emails"`
	Links   []ContributorLink   `json:"links"`
	Icons   *ContributorIcons   `json:"icons,omitempty"`
	Bundles []*Bundle           `json:"bundles"`
	Files   []*File             `json:"files"`
	Tickets []*Ticket           `json:"tickets"`
	Metrics *ContributorMetrics `json:"metrics"`
}

func (c *Contributor) IsNode()       {}
func (c *Contributor) GetID() string { return c.ID }

type Commit struct {
	ID       string    `json:"id"`
	SHA      string    `json:"sha"`
	Title    string    `json:"title"`
	AuthorID *string   `json:"authorId,omitempty"`
	Date     time.Time `json:"date"`
}

func (c *Commit) IsNode()       {}
func (c *Commit) GetID() string { return c.ID }

type Ticket struct {
	ID       string         `json:"id"`
	Year     int            `json:"year"`
	Month    int            `json:"month"`
	Day      int            `json:"day"`
	Slug     string         `json:"slug"`
	Path     string         `json:"path"`
	URI      string         `json:"uri"`
	Prompt   string         `json:"prompt"`
	Summary  *string        `json:"summary,omitempty"`
	Status   TicketStatus   `json:"status"`
	AuthorID *string        `json:"authorId,omitempty"`
	Model    *string        `json:"model,omitempty"`
	Commit   *string        `json:"commit,omitempty"`
	Date     *TicketDate    `json:"date"`
	Bundles  []*Bundle      `json:"bundles"`
	Files    []*File        `json:"files"`
	Metrics  *TicketMetrics `json:"metrics"`
}

func (t *Ticket) IsNode()       {}
func (t *Ticket) GetID() string { return t.ID }

type TicketIteration struct {
	Prompt   string          `json:"prompt"`
	Model    *string         `json:"model,omitempty"`
	AuthorID *string         `json:"authorId,omitempty"`
	Commit   *string         `json:"commit,omitempty"`
	Date     *IterationDate  `json:"date"`
	Files    *IterationFiles `json:"files,omitempty"`
}

type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

type TicketFileContrib struct {
	FileID   string                `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}

type Policy struct {
	ID             string           `json:"id"`
	Name           string           `json:"name"`
	Description    *string          `json:"description,omitempty"`
	Scopes         []string         `json:"scopes"`
	ViolationKinds []*ViolationKind `json:"violationKinds"`
}

func (p *Policy) IsNode()       {}
func (p *Policy) GetID() string { return p.ID }

type ViolationKind struct {
	ID          string            `json:"id"`
	PolicyID    string            `json:"policyId"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
}

func (v *ViolationKind) IsNode()       {}
func (v *ViolationKind) GetID() string { return v.ID }

type Violation struct {
	ID       string         `json:"id"`
	KindID   string         `json:"kindId"`
	Kind     *ViolationKind `json:"kind,omitempty"`
	Scope    string         `json:"scope"`
	FileID   *string        `json:"fileId,omitempty"`
	FolderID *string        `json:"folderId,omitempty"`
	Line     *int           `json:"line,omitempty"`
	Column   *int           `json:"column,omitempty"`
	Excerpt  *string        `json:"excerpt,omitempty"`
	Autofix  *Autofix       `json:"autofix,omitempty"`
}

func (v *Violation) IsNode()       {}
func (v *Violation) GetID() string { return v.ID }

// #endregion Entity Types

// #region Result Types

type AnalyzeResult struct {
	Violations []*Violation    `json:"violations"`
	Metrics    *AnalyzeMetrics `json:"metrics"`
}

type FixResult struct {
	Fixed      int          `json:"fixed"`
	Remaining  int          `json:"remaining"`
	Violations []*Violation `json:"violations"`
}

// #endregion Result Types

// #region Contribution Types

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

// #endregion Contribution Types
