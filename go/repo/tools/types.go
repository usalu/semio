// repo/tools/types.go

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

package tools

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
	ID          string            `json:"id"`
	Summary     string            `json:"summary"`
	Kind        string            `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
	Reason      string            `json:"reason"`
	Scope       string            `json:"scope"`
	Line        int               `json:"line,omitempty"`
	Column      int               `json:"column,omitempty"`
	Excerpt     string            `json:"excerpt,omitempty"`
	Autofix     *Fix              `json:"autofix,omitempty"`
}

type NxProject struct {
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

type PolicyMeta struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Scopes      []string          `json:"scopes"`
	Priority    ViolationPriority `json:"priority"`
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

