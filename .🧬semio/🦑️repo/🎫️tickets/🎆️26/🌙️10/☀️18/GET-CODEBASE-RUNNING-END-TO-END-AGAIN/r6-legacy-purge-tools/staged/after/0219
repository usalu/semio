// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📼️ticket_data carries the interaction, agent, diff and management data of tickets.

// #endregion 🧲️Header

package model

import (
	time "time"
)

// #region 📼️Ticket Data

// 🟪️TicketDate holds the data fields for a ticket date record.
type TicketDate struct {
	Created  time.Time  `json:"created"`
	Finished *time.Time `json:"finished,omitempty"`
}

// 📄️InteractionFile holds a file reference with path, id and uri.
type InteractionFile struct {
	Path string `json:"path" yaml:"path"`
	ID   string `json:"id,omitempty" yaml:"id,omitempty"`
	URI  string `json:"uri,omitempty" yaml:"uri,omitempty"`
}

// 🟫️Interaction holds the data fields for a interaction record.
type Interaction struct {
	Kind       string            `json:"kind" yaml:"kind"`
	Date       string            `json:"date" yaml:"date"`
	Author     string            `json:"author" yaml:"author"`
	System     string            `json:"system" yaml:"system"`
	Client     string            `json:"client" yaml:"client"`
	Checkpoint string            `json:"checkpoint" yaml:"checkpoint"`
	Prompt     string            `json:"prompt,omitempty" yaml:"prompt,omitempty"`
	Summary    string            `json:"summary,omitempty" yaml:"summary,omitempty"`
	LLM        string            `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort     string            `json:"effort,omitempty" yaml:"effort,omitempty"`
	Files      []InteractionFile `json:"files,omitempty" yaml:"files,omitempty"`
}

// 💾️CheckpointDiffRename holds a from/to pair for renamed entities.
type CheckpointDiffRename struct {
	From string `json:"from" yaml:"from"`
	To   string `json:"to" yaml:"to"`
}

// 🆕️CheckpointDiffStats holds deleted/renamed/modified/created lists for a diff category.
type CheckpointDiffStats struct {
	Deleted  []string               `json:"deleted,omitempty" yaml:"deleted,omitempty"`
	Renamed  []CheckpointDiffRename `json:"renamed,omitempty" yaml:"renamed,omitempty"`
	Modified []string               `json:"modified,omitempty" yaml:"modified,omitempty"`
	Created  []string               `json:"created,omitempty" yaml:"created,omitempty"`
}

// 📁️CheckpointDiff holds diff stats for technologies, bundles, folders, files, sections, and definitions.
type CheckpointDiff struct {
	Technologies CheckpointDiffStats `json:"technologies,omitempty" yaml:"technologies,omitempty"`
	Bundles      CheckpointDiffStats `json:"bundles,omitempty" yaml:"bundles,omitempty"`
	Folders      CheckpointDiffStats `json:"folders,omitempty" yaml:"folders,omitempty"`
	Files        CheckpointDiffStats `json:"files,omitempty" yaml:"files,omitempty"`
	Sections     CheckpointDiffStats `json:"sections,omitempty" yaml:"sections,omitempty"`
	Definitions  CheckpointDiffStats `json:"definitions,omitempty" yaml:"definitions,omitempty"`
}

// 🎫️TicketAgentPlanStep holds a single step in an agent plan with optional timestamps.
type TicketAgentPlanStep struct {
	ID          string `json:"id" yaml:"id"`
	Name        string `json:"name" yaml:"name"`
	Description string `json:"description,omitempty" yaml:"description,omitempty"`
	Status      string `json:"status,omitempty" yaml:"status,omitempty"`
	Ideated     string `json:"ideated,omitempty" yaml:"ideated,omitempty"`
	Started     string `json:"started,omitempty" yaml:"started,omitempty"`
	Completed   string `json:"completed,omitempty" yaml:"completed,omitempty"`
	Abandoned   string `json:"abandoned,omitempty" yaml:"abandoned,omitempty"`
}

// 💠️TicketAgentPlan holds the plan steps for an agent.
type TicketAgentPlan struct {
	Steps []TicketAgentPlanStep `json:"steps,omitempty" yaml:"steps,omitempty"`
}

// 🔳️TicketAgent holds an agent record with contributor and plan.
type TicketAgent struct {
	Session     string           `json:"session" yaml:"session"`
	Contributor string           `json:"contributor,omitempty" yaml:"contributor,omitempty"`
	System      string           `json:"system,omitempty" yaml:"system,omitempty"`
	Client      string           `json:"client,omitempty" yaml:"client,omitempty"`
	LLM         string           `json:"llm,omitempty" yaml:"llm,omitempty"`
	Effort      string           `json:"effort,omitempty" yaml:"effort,omitempty"`
	Transcript  string           `json:"transcript,omitempty" yaml:"transcript,omitempty"`
	Plan        *TicketAgentPlan `json:"plan,omitempty" yaml:"plan,omitempty"`
}


// 🎁️InteractionResource holds a flat interaction enriched with its source context.
type InteractionResource struct {
	Interaction
	SourceKind string `json:"sourceKind"`
	SourceID   string `json:"sourceId"`
	GoalID     string `json:"goalId,omitempty"`
	TicketID   string `json:"ticketId,omitempty"`
}

// ▫️TicketSection holds the data fields for a ticket section record.
type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// ◾TicketFile holds the data fields for a ticket file record.
type TicketFile struct {
	Path  string       `json:"path"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🐙️TicketManagementData holds the data fields for a ticket github data record.
type TicketManagementData struct {
	Issue string `json:"issue,omitempty"`
}

// ◽TicketFileRenamed holds the data fields for a ticket file renamed record.
type TicketFileRenamed struct {
	From  string       `json:"from"`
	To    string       `json:"to"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

// 🗃️TicketDiffSet holds the data fields for a ticket diff set record.
type TicketDiffSet struct {
	Deleted  []TicketFile        `json:"deleted"`
	Renamed  []TicketFileRenamed `json:"renamed"`
	Modified []TicketFile        `json:"modified"`
	Added    []TicketFile        `json:"added"`
}

// ◻TicketDiffs holds the data fields for a ticket diffs record.
type TicketDiffs struct {
	Bundles     TicketDiffSet `json:"bundles"`
	Folders     TicketDiffSet `json:"folders"`
	Files       TicketDiffSet `json:"files"`
	Sections    TicketDiffSet `json:"sections"`
	Definitions TicketDiffSet `json:"definitions"`
}

// ◼TicketData holds the data fields for a ticket data record.
type TicketData struct {
	Title      string                `json:"title"`
	Status     TicketStatus          `json:"status"`
	Summary    string                `json:"summary,omitempty"`
	Management *TicketManagementData `json:"github,omitempty"`
	Goal       string                `json:"goal,omitempty"`
	Parent     string                `json:"parent,omitempty"`
}

// 🎫️TicketDateInfo holds the data fields for a ticket date info record.
type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

// 🔳️TicketBundleContribInfo holds the data fields for a ticket bundle contrib info record.
type TicketBundleContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🔲️TicketFolderContribInfo holds the data fields for a ticket folder contrib info record.
type TicketFolderContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▪️TicketFileContribInfo holds the data fields for a ticket file contrib info record.
type TicketFileContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▫️TicketSectionContribInfo holds the data fields for a ticket section contrib info record.
type TicketSectionContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ◾TicketDefinitionContrib holds the data fields for a ticket definition contrib record.
type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🗃️CodebaseTicket holds the data fields for a codebase ticket record.
type CodebaseTicket struct {
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	URI         string                     `json:"uri"`
	Date        *TicketDateInfo            `json:"date,omitempty"`
	Checkpoint  string                     `json:"checkpoint,omitempty"`
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

// #endregion 📼️Ticket Data
