// #region 🔖Header
// [🧰semiorepo📚go💻eventsgo](semiorepo://file/semio-repo/go/events.go)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// GPL-3.0
// Shared event kinds and payloads for CLI→server event-based communication.
// #endregion 🔖Header

package repo

import "encoding/json"

// #region 🔖EventKind
// [🧰semiorepo📚go💻eventsgo🔖eventkind](semiorepo://section/semio-repo/go/events.go/event-kind)

// EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.
// [🧰semiorepo📚go💻eventsgo🔖eventkind✂️eventkind](semiorepo://definition/semio-repo/go/events.go/eventkind/eventkind)
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
	EventCommit            EventKind = "commit"
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
// [🧰semiorepo📚go💻eventsgo🔖event](semiorepo://section/semio-repo/go/events.go/event)

// Event is the canonical envelope for a changing interaction sent from CLI to server.
// [🧰semiorepo📚go💻eventsgo🔖event✂️event](semiorepo://definition/semio-repo/go/events.go/event/event)
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion 🔖Event

// #region 🔖Payloads
// [🧰semiorepo📚go💻eventsgo🔖payloads](semiorepo://section/semio-repo/go/events.go/payloads)

// TicketPayload holds common ticket identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️ticketpayload](semiorepo://definition/semio-repo/go/events.go/payloads/ticketpayload)
type TicketPayload struct {
	ID    string `json:"id"`
	Year  int    `json:"year,omitempty"`
	Month int    `json:"month,omitempty"`
	Day   int    `json:"day,omitempty"`
	Slug  string `json:"slug,omitempty"`
}

// TicketOpenPayload payload for ticket.open.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️ticketopenpayload](semiorepo://definition/semio-repo/go/events.go/payloads/ticketopenpayload)
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
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️ticketclosepayload](semiorepo://definition/semio-repo/go/events.go/payloads/ticketclosepayload)
type TicketClosePayload struct {
	TicketPayload
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Author  string   `json:"author,omitempty"`
}

// TicketReopenPayload payload for ticket.reopen.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️ticketreopenpayload](semiorepo://definition/semio-repo/go/events.go/payloads/ticketreopenpayload)
type TicketReopenPayload struct {
	TicketPayload
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
}

// TicketChangePayload payload for ticket.change.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️ticketchangepayload](semiorepo://definition/semio-repo/go/events.go/payloads/ticketchangepayload)
type TicketChangePayload struct {
	TicketPayload
	Title  *string `json:"title,omitempty"`
	Prompt *string `json:"prompt,omitempty"`
	Goal   *string `json:"goal,omitempty"`
	Parent *string `json:"parent,omitempty"`
	Author string  `json:"author,omitempty"`
}

// GoalPayload holds common goal identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️goalpayload](semiorepo://definition/semio-repo/go/events.go/payloads/goalpayload)
type GoalPayload struct {
	ID string `json:"id"`
}

// GoalOpenPayload payload for goal.open.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️goalopenpayload](semiorepo://definition/semio-repo/go/events.go/payloads/goalopenpayload)
type GoalOpenPayload struct {
	GoalPayload
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Author      string `json:"author,omitempty"`
}

// GoalClosePayload payload for goal.close.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️goalclosepayload](semiorepo://definition/semio-repo/go/events.go/payloads/goalclosepayload)
type GoalClosePayload struct {
	GoalPayload
	Summary string `json:"summary"`
	Author  string `json:"author,omitempty"`
}

// GoalReopenPayload payload for goal.reopen.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️goalreopenpayload](semiorepo://definition/semio-repo/go/events.go/payloads/goalreopenpayload)
type GoalReopenPayload struct {
	GoalPayload
	Prompt string `json:"prompt"`
	Client string `json:"client"`
	LLM    string `json:"llm,omitempty"`
	Author string `json:"author,omitempty"`
}

// GoalChangePayload payload for goal.change.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️goalchangepayload](semiorepo://definition/semio-repo/go/events.go/payloads/goalchangepayload)
type GoalChangePayload struct {
	GoalPayload
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	Author      string  `json:"author,omitempty"`
}

// ContributorPayload holds contributor identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️contributorpayload](semiorepo://definition/semio-repo/go/events.go/payloads/contributorpayload)
type ContributorPayload struct {
	Github string `json:"github"`
	Author string `json:"author,omitempty"`
}

// CommitPayload payload for commit (GitHub push).
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️commitpayload](semiorepo://definition/semio-repo/go/events.go/payloads/commitpayload)
type CommitPayload struct {
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
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️todopayload](semiorepo://definition/semio-repo/go/events.go/payloads/todopayload)
type TodoPayload struct {
	ID       string `json:"id"`
	ParentID string `json:"parent_id,omitempty"`
	Name     string `json:"name,omitempty"`
	Author   string `json:"author,omitempty"`
}

// TodoCreatePayload payload for todo.create.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️todocreatepayload](semiorepo://definition/semio-repo/go/events.go/payloads/todocreatepayload)
type TodoCreatePayload struct {
	TodoPayload
}

// TodoChangePayload payload for todo.change.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️todochangepayload](semiorepo://definition/semio-repo/go/events.go/payloads/todochangepayload)
type TodoChangePayload struct {
	TodoPayload
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// TodoDeletePayload payload for todo.delete.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️tododeletepayload](semiorepo://definition/semio-repo/go/events.go/payloads/tododeletepayload)
type TodoDeletePayload struct {
	TodoPayload
}

// WorkItem represents a single item a contributor is working on (project, bundle, folder, file, section, definition, ticket, goal, todo).
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️workitem](semiorepo://definition/semio-repo/go/events.go/payloads/workitem)
type WorkItem struct {
	Kind string `json:"kind"`
	ID   string `json:"id"`
}

// ContributorWork holds all work items for one contributor.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️contributorwork](semiorepo://definition/semio-repo/go/events.go/payloads/contributorwork)
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
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️draftpayload](semiorepo://definition/semio-repo/go/events.go/payloads/draftpayload)
type DraftPayload struct {
	Slug   string `json:"slug"`
	Title  string `json:"title,omitempty"`
	Author string `json:"author,omitempty"`
}

// FilePayload holds file operation identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️filepayload](semiorepo://definition/semio-repo/go/events.go/payloads/filepayload)
type FilePayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// FolderPayload holds folder operation identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️folderpayload](semiorepo://definition/semio-repo/go/events.go/payloads/folderpayload)
type FolderPayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// SectionPayload holds section operation identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️sectionpayload](semiorepo://definition/semio-repo/go/events.go/payloads/sectionpayload)
type SectionPayload struct {
	File    string `json:"file"`
	Name    string `json:"name"`
	OldName string `json:"old_name,omitempty"`
	Parent  string `json:"parent,omitempty"`
	Author  string `json:"author,omitempty"`
}

// IntegratePayload holds integrate operation identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️integratepayload](semiorepo://definition/semio-repo/go/events.go/payloads/integratepayload)
type IntegratePayload struct {
	Source        string `json:"source"`
	TargetFile    string `json:"target_file"`
	TargetSection string `json:"target_section"`
	Author        string `json:"author,omitempty"`
}

// ExtractPayload holds extract operation identifiers.
// [🧰semiorepo📚go💻eventsgo🔖payloads✂️extractpayload](semiorepo://definition/semio-repo/go/events.go/payloads/extractpayload)
type ExtractPayload struct {
	SourceFile    string `json:"source_file"`
	SourceSection string `json:"source_section"`
	TargetFile    string `json:"target_file"`
	Author        string `json:"author,omitempty"`
}

// #endregion 🔖Payloads
