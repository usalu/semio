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
	EventTicketOpenStarting        EventKind = "ticket.open.starting"
	EventTicketOpenEnded           EventKind = "ticket.open.ended"
	EventTicketCloseStarting       EventKind = "ticket.close.starting"
	EventTicketCloseEnded          EventKind = "ticket.close.ended"
	EventTicketReopenStarting      EventKind = "ticket.reopen.starting"
	EventTicketReopenEnded         EventKind = "ticket.reopen.ended"
	EventTicketChangeStarting      EventKind = "ticket.change.starting"
	EventTicketChangeEnded         EventKind = "ticket.change.ended"
	EventTicketReadStarting        EventKind = "ticket.read.starting"
	EventTicketReadEnded           EventKind = "ticket.read.ended"
	EventGoalOpenStarting          EventKind = "goal.open.starting"
	EventGoalOpenEnded             EventKind = "goal.open.ended"
	EventGoalCloseStarting         EventKind = "goal.close.starting"
	EventGoalCloseEnded            EventKind = "goal.close.ended"
	EventGoalReopenStarting        EventKind = "goal.reopen.starting"
	EventGoalReopenEnded           EventKind = "goal.reopen.ended"
	EventGoalChangeStarting        EventKind = "goal.change.starting"
	EventGoalChangeEnded           EventKind = "goal.change.ended"
	EventContributorAddStarting    EventKind = "contributor.add.starting"
	EventContributorAddEnded       EventKind = "contributor.add.ended"
	EventContributorRemoveStarting EventKind = "contributor.remove.starting"
	EventContributorRemoveEnded    EventKind = "contributor.remove.ended"
	EventCheckpointStarting        EventKind = "checkpoint.starting"
	EventCheckpointEnded           EventKind = "checkpoint.ended"
	EventTodoCreateStarting        EventKind = "todo.create.starting"
	EventTodoCreateEnded           EventKind = "todo.create.ended"
	EventTodoChangeStarting        EventKind = "todo.change.starting"
	EventTodoChangeEnded           EventKind = "todo.change.ended"
	EventTodoDeleteStarting        EventKind = "todo.delete.starting"
	EventTodoDeleteEnded           EventKind = "todo.delete.ended"
	EventDraftCreateStarting       EventKind = "draft.create.starting"
	EventDraftCreateEnded          EventKind = "draft.create.ended"
	EventDraftDeleteStarting       EventKind = "draft.delete.starting"
	EventDraftDeleteEnded          EventKind = "draft.delete.ended"
	EventFileCreateStarting        EventKind = "file.create.starting"
	EventFileCreateEnded           EventKind = "file.create.ended"
	EventFileMoveStarting          EventKind = "file.move.starting"
	EventFileMoveEnded             EventKind = "file.move.ended"
	EventFileDeleteStarting        EventKind = "file.delete.starting"
	EventFileDeleteEnded           EventKind = "file.delete.ended"
	EventFolderCreateStarting      EventKind = "folder.create.starting"
	EventFolderCreateEnded         EventKind = "folder.create.ended"
	EventFolderMoveStarting        EventKind = "folder.move.starting"
	EventFolderMoveEnded           EventKind = "folder.move.ended"
	EventFolderDeleteStarting      EventKind = "folder.delete.starting"
	EventFolderDeleteEnded         EventKind = "folder.delete.ended"
	EventSectionCreateStarting     EventKind = "section.create.starting"
	EventSectionCreateEnded        EventKind = "section.create.ended"
	EventSectionMoveStarting       EventKind = "section.move.starting"
	EventSectionMoveEnded          EventKind = "section.move.ended"
	EventSectionDeleteStarting     EventKind = "section.delete.starting"
	EventSectionDeleteEnded        EventKind = "section.delete.ended"
	EventIntegrateStarting         EventKind = "integrate.starting"
	EventIntegrateEnded            EventKind = "integrate.ended"
	EventExtractStarting           EventKind = "extract.starting"
	EventExtractEnded              EventKind = "extract.ended"
	EventExportStarting            EventKind = "export.starting"
	EventExportEnded               EventKind = "export.ended"
	EventAnalyzeStarting           EventKind = "analyze.starting"
	EventAnalyzeEnded              EventKind = "analyze.ended"
	EventFixStarting               EventKind = "fix.starting"
	EventFixEnded                  EventKind = "fix.ended"
	EventTreeStarting              EventKind = "tree.starting"
	EventTreeEnded                 EventKind = "tree.ended"
	EventGraphqlStarting           EventKind = "graphql.starting"
	EventGraphqlEnded              EventKind = "graphql.ended"
	EventMoveStarting              EventKind = "move.starting"
	EventMoveEnded                 EventKind = "move.ended"
	EventPolicyCheckStarting       EventKind = "policy.check.starting"
	EventPolicyCheckEnded          EventKind = "policy.check.ended"
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
