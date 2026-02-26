// #region 🔖Header
// [🧰semiorepo📚go💻events](semiorepo://p/i/semio-repo/b/l/go/f/events.go)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// GPL-3.0
// Shared event kinds and payloads for CLI→server event-based communication.
// #endregion 🔖Header

package repo

import "encoding/json"

// #region 🔖EventKind
// [🧰semiorepo📚go💻events🔖eventkind](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/EventKind)

// EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.
// [🧰semiorepo📚go💻events🔖eventkind✂️eventkind](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/EventKind/d/i/EventKind)
type EventKind string

const (
	EventTicketOpen        EventKind = "ticket.open"
	EventTicketClose       EventKind = "ticket.close"
	EventTicketReopen      EventKind = "ticket.reopen"
	EventTicketChange      EventKind = "ticket.change"
	EventGoalOpen          EventKind = "goal.open"
	EventGoalClose         EventKind = "goal.close"
	EventGoalReopen        EventKind = "goal.reopen"
	EventGoalChange        EventKind = "goal.change"
	EventContributorAdd    EventKind = "contributor.add"
	EventContributorRemove EventKind = "contributor.remove"
	EventCheckpoint        EventKind = "checkpoint"
	EventTodoCreate        EventKind = "todo.create"
	EventTodoChange        EventKind = "todo.change"
	EventTodoDelete        EventKind = "todo.delete"
	EventDraftCreate       EventKind = "draft.create"
	EventDraftDelete       EventKind = "draft.delete"
	EventFileCreate        EventKind = "file.create"
	EventFileMove          EventKind = "file.move"
	EventFileDelete        EventKind = "file.delete"
	EventFolderCreate      EventKind = "folder.create"
	EventFolderMove        EventKind = "folder.move"
	EventFolderDelete      EventKind = "folder.delete"
	EventSectionCreate     EventKind = "section.create"
	EventSectionMove       EventKind = "section.move"
	EventSectionDelete     EventKind = "section.delete"
	EventIntegrate         EventKind = "integrate"
	EventExtract           EventKind = "extract"
)

// #endregion 🔖EventKind

// #region 🔖Event
// [🧰semiorepo📚go💻events🔖event](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Event)

// Event is the canonical envelope for a changing interaction sent from CLI to server.
// [🧰semiorepo📚go💻events🔖event✂️event](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Event/d/i/Event)
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion 🔖Event

// #region 🔖Payloads
// [🧰semiorepo📚go💻events🔖payloads](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads)

// TicketPayload holds common ticket identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️ticketpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TicketPayload)
type TicketPayload struct {
	ID    string `json:"id"`
	Year  int    `json:"year,omitempty"`
	Month int    `json:"month,omitempty"`
	Day   int    `json:"day,omitempty"`
	Slug  string `json:"slug,omitempty"`
}

// TicketOpenPayload payload for ticket.open.
// [🧰semiorepo📚go💻events🔖payloads✂️ticketopenpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TicketOpenPayload)
type TicketOpenPayload struct {
	TicketPayload
	Title  string `json:"title"`
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
	Goal   string `json:"goal"`
	Parent string `json:"parent,omitempty"`
}

// TicketClosePayload payload for ticket.close.
// [🧰semiorepo📚go💻events🔖payloads✂️ticketclosepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TicketClosePayload)
type TicketClosePayload struct {
	TicketPayload
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Author  string   `json:"author,omitempty"`
}

// TicketReopenPayload payload for ticket.reopen.
// [🧰semiorepo📚go💻events🔖payloads✂️ticketreopenpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TicketReopenPayload)
type TicketReopenPayload struct {
	TicketPayload
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
}

// TicketChangePayload payload for ticket.change.
// [🧰semiorepo📚go💻events🔖payloads✂️ticketchangepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TicketChangePayload)
type TicketChangePayload struct {
	TicketPayload
	Title  *string `json:"title,omitempty"`
	Prompt *string `json:"prompt,omitempty"`
	Goal   *string `json:"goal,omitempty"`
	Parent *string `json:"parent,omitempty"`
	Author string  `json:"author,omitempty"`
}

// GoalPayload holds common goal identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️goalpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/GoalPayload)
type GoalPayload struct {
	ID string `json:"id"`
}

// GoalOpenPayload payload for goal.open.
// [🧰semiorepo📚go💻events🔖payloads✂️goalopenpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/GoalOpenPayload)
type GoalOpenPayload struct {
	GoalPayload
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Author      string `json:"author,omitempty"`
}

// GoalClosePayload payload for goal.close.
// [🧰semiorepo📚go💻events🔖payloads✂️goalclosepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/GoalClosePayload)
type GoalClosePayload struct {
	GoalPayload
	Summary string `json:"summary"`
	Author  string `json:"author,omitempty"`
}

// GoalReopenPayload payload for goal.reopen.
// [🧰semiorepo📚go💻events🔖payloads✂️goalreopenpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/GoalReopenPayload)
type GoalReopenPayload struct {
	GoalPayload
	Prompt string `json:"prompt"`
	Client string `json:"client"`
	LLM    string `json:"llm,omitempty"`
	Author string `json:"author,omitempty"`
}

// GoalChangePayload payload for goal.change.
// [🧰semiorepo📚go💻events🔖payloads✂️goalchangepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/GoalChangePayload)
type GoalChangePayload struct {
	GoalPayload
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	Author      string  `json:"author,omitempty"`
}

// ContributorPayload holds contributor identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️contributorpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/ContributorPayload)
type ContributorPayload struct {
	Github string `json:"github"`
	Author string `json:"author,omitempty"`
}

// CheckpointPayload payload for checkpoint (GitHub push).
// [🧰semiorepo📚go💻events🔖payloads✂️checkpointpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/CheckpointPayload)
type CheckpointPayload struct {
	Author       string   `json:"author"`
	Github       string   `json:"github"`
	Sha          string   `json:"sha"`
	Message      string   `json:"message"`
	Files        []string `json:"files"`
	Projects     []string `json:"projects,omitempty"`
	Bundles      []string `json:"bundles,omitempty"`
	Folders      []string `json:"folders,omitempty"`
	FilesChanged []string `json:"files_changed,omitempty"`
	Sections     []string `json:"sections,omitempty"`
	Definitions  []string `json:"definitions,omitempty"`
}

// TodoPayload holds todo identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️todopayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TodoPayload)
type TodoPayload struct {
	ID       string `json:"id"`
	ParentID string `json:"parent_id,omitempty"`
	Name     string `json:"name,omitempty"`
	Author   string `json:"author,omitempty"`
}

// TodoCreatePayload payload for todo.create.
// [🧰semiorepo📚go💻events🔖payloads✂️todocreatepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TodoCreatePayload)
type TodoCreatePayload struct {
	TodoPayload
}

// TodoChangePayload payload for todo.change.
// [🧰semiorepo📚go💻events🔖payloads✂️todochangepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TodoChangePayload)
type TodoChangePayload struct {
	TodoPayload
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// TodoDeletePayload payload for todo.delete.
// [🧰semiorepo📚go💻events🔖payloads✂️tododeletepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/TodoDeletePayload)
type TodoDeletePayload struct {
	TodoPayload
}

// WorkItem represents a single item a contributor is working on (project, bundle, folder, file, section, definition, ticket, goal, todo).
// [🧰semiorepo📚go💻events🔖payloads✂️workitem](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/WorkItem)
type WorkItem struct {
	Kind string `json:"kind"`
	ID   string `json:"id"`
}

// ContributorWork holds all work items for one contributor.
// [🧰semiorepo📚go💻events🔖payloads✂️contributorwork](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/ContributorWork)
type ContributorWork struct {
	Github      string   `json:"github"`
	Tickets     []string `json:"tickets"`
	Goals       []string `json:"goals"`
	Todos       []string `json:"todos"`
	Projects    []string `json:"projects"`
	Bundles     []string `json:"bundles"`
	Folders     []string `json:"folders"`
	Files       []string `json:"files"`
	Sections    []string `json:"sections"`
	Definitions []string `json:"definitions"`
}

// DraftPayload holds draft identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️draftpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/DraftPayload)
type DraftPayload struct {
	Slug   string `json:"slug"`
	Title  string `json:"title,omitempty"`
	Author string `json:"author,omitempty"`
}

// FilePayload holds file operation identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️filepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/FilePayload)
type FilePayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// FolderPayload holds folder operation identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️folderpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/FolderPayload)
type FolderPayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// SectionPayload holds section operation identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️sectionpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/SectionPayload)
type SectionPayload struct {
	File    string `json:"file"`
	Name    string `json:"name"`
	OldName string `json:"old_name,omitempty"`
	Parent  string `json:"parent,omitempty"`
	Author  string `json:"author,omitempty"`
}

// IntegratePayload holds integrate operation identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️integratepayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/IntegratePayload)
type IntegratePayload struct {
	Source        string `json:"source"`
	TargetFile    string `json:"target_file"`
	TargetSection string `json:"target_section"`
	Author        string `json:"author,omitempty"`
}

// ExtractPayload holds extract operation identifiers.
// [🧰semiorepo📚go💻events🔖payloads✂️extractpayload](semiorepo://p/i/semio-repo/b/l/go/f/events.go/s/Payloads/d/i/ExtractPayload)
type ExtractPayload struct {
	SourceFile    string `json:"source_file"`
	SourceSection string `json:"source_section"`
	TargetFile    string `json:"target_file"`
	Author        string `json:"author,omitempty"`
}

// #endregion 🔖Payloads
