// #region 🔖Header
// [💻semio-repo/go/events.go](semiorepo://file/semio-repo/go/events.go)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// GPL-3.0
// Shared event kinds and payloads for CLI→server event-based communication.
// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖License

// #endregion 🔖Header

package repo

import "encoding/json"

// #region 🔖EventKind
// [🔖semio-repo/go/events.go#EventKind](semiorepo://section/semio-repo/go/events.go/EVENT-KIND)

// EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.
// [🪨semio-repo/go/events.go#EventKind§EventKind](semiorepo://definition/semio-repo/go/events.go/EVENT-KIND/EVENT-KIND)
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
// [🔖semio-repo/go/events.go#Event](semiorepo://section/semio-repo/go/events.go/EVENT)

// Event is the canonical envelope for a changing interaction sent from CLI to server.
// [🛠️semio-repo/go/events.go#Event§Event](semiorepo://definition/semio-repo/go/events.go/EVENT/EVENT)
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion 🔖Event

// #region 🔖Payloads
// [🔖semio-repo/go/events.go#Payloads](semiorepo://section/semio-repo/go/events.go/PAYLOADS)

// TicketPayload holds common ticket identifiers.
// [🛠️semio-repo/go/events.go#Payloads§TicketPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TICKET-PAYLOAD)
type TicketPayload struct {
	ID    string `json:"id"`
	Year  int    `json:"year,omitempty"`
	Month int    `json:"month,omitempty"`
	Day   int    `json:"day,omitempty"`
	Slug  string `json:"slug,omitempty"`
}

// TicketOpenPayload payload for ticket.open.
// [🛠️semio-repo/go/events.go#Payloads§TicketOpenPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TICKET-OPEN-PAYLOAD)
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
// [🛠️semio-repo/go/events.go#Payloads§TicketClosePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TICKET-CLOSE-PAYLOAD)
type TicketClosePayload struct {
	TicketPayload
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Author  string   `json:"author,omitempty"`
}

// TicketReopenPayload payload for ticket.reopen.
// [🛠️semio-repo/go/events.go#Payloads§TicketReopenPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TICKET-REOPEN-PAYLOAD)
type TicketReopenPayload struct {
	TicketPayload
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
}

// TicketChangePayload payload for ticket.change.
// [🛠️semio-repo/go/events.go#Payloads§TicketChangePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TICKET-CHANGE-PAYLOAD)
type TicketChangePayload struct {
	TicketPayload
	Title  *string `json:"title,omitempty"`
	Prompt *string `json:"prompt,omitempty"`
	Goal   *string `json:"goal,omitempty"`
	Parent *string `json:"parent,omitempty"`
	Author string  `json:"author,omitempty"`
}

// GoalPayload holds common goal identifiers.
// [🛠️semio-repo/go/events.go#Payloads§GoalPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/GOAL-PAYLOAD)
type GoalPayload struct {
	ID string `json:"id"`
}

// GoalOpenPayload payload for goal.open.
// [🛠️semio-repo/go/events.go#Payloads§GoalOpenPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/GOAL-OPEN-PAYLOAD)
type GoalOpenPayload struct {
	GoalPayload
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Author      string `json:"author,omitempty"`
}

// GoalClosePayload payload for goal.close.
// [🛠️semio-repo/go/events.go#Payloads§GoalClosePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/GOAL-CLOSE-PAYLOAD)
type GoalClosePayload struct {
	GoalPayload
	Summary string `json:"summary"`
	Author  string `json:"author,omitempty"`
}

// GoalReopenPayload payload for goal.reopen.
// [🛠️semio-repo/go/events.go#Payloads§GoalReopenPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/GOAL-REOPEN-PAYLOAD)
type GoalReopenPayload struct {
	GoalPayload
	Prompt string `json:"prompt"`
	Client string `json:"client"`
	LLM    string `json:"llm,omitempty"`
	Author string `json:"author,omitempty"`
}

// GoalChangePayload payload for goal.change.
// [🛠️semio-repo/go/events.go#Payloads§GoalChangePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/GOAL-CHANGE-PAYLOAD)
type GoalChangePayload struct {
	GoalPayload
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	Author      string  `json:"author,omitempty"`
}

// ContributorPayload holds contributor identifiers.
// [🛠️semio-repo/go/events.go#Payloads§ContributorPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/CONTRIBUTOR-PAYLOAD)
type ContributorPayload struct {
	Github string `json:"github"`
	Author string `json:"author,omitempty"`
}

// CommitPayload payload for commit (GitHub push).
// [🛠️semio-repo/go/events.go#Payloads§CommitPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/COMMIT-PAYLOAD)
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
// [🛠️semio-repo/go/events.go#Payloads§TodoPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TODO-PAYLOAD)
type TodoPayload struct {
	ID       string `json:"id"`
	ParentID string `json:"parent_id,omitempty"`
	Name     string `json:"name,omitempty"`
	Author   string `json:"author,omitempty"`
}

// TodoCreatePayload payload for todo.create.
// [🛠️semio-repo/go/events.go#Payloads§TodoCreatePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TODO-CREATE-PAYLOAD)
type TodoCreatePayload struct {
	TodoPayload
}

// TodoChangePayload payload for todo.change.
// [🛠️semio-repo/go/events.go#Payloads§TodoChangePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TODO-CHANGE-PAYLOAD)
type TodoChangePayload struct {
	TodoPayload
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// TodoDeletePayload payload for todo.delete.
// [🛠️semio-repo/go/events.go#Payloads§TodoDeletePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/TODO-DELETE-PAYLOAD)
type TodoDeletePayload struct {
	TodoPayload
}

// WorkItem represents a single item a contributor is working on (project, bundle, folder, file, section, definition, ticket, goal, todo).
// [🛠️semio-repo/go/events.go#Payloads§WorkItem](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/WORK-ITEM)
type WorkItem struct {
	Kind string `json:"kind"`
	ID   string `json:"id"`
}

// ContributorWork holds all work items for one contributor.
// [🛠️semio-repo/go/events.go#Payloads§ContributorWork](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/CONTRIBUTOR-WORK)
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
// [🛠️semio-repo/go/events.go#Payloads§DraftPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/DRAFT-PAYLOAD)
type DraftPayload struct {
	Slug   string `json:"slug"`
	Title  string `json:"title,omitempty"`
	Author string `json:"author,omitempty"`
}

// FilePayload holds file operation identifiers.
// [🛠️semio-repo/go/events.go#Payloads§FilePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/FILE-PAYLOAD)
type FilePayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// FolderPayload holds folder operation identifiers.
// [🛠️semio-repo/go/events.go#Payloads§FolderPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/FOLDER-PAYLOAD)
type FolderPayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// SectionPayload holds section operation identifiers.
// [🛠️semio-repo/go/events.go#Payloads§SectionPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/SECTION-PAYLOAD)
type SectionPayload struct {
	File    string `json:"file"`
	Name    string `json:"name"`
	OldName string `json:"old_name,omitempty"`
	Parent  string `json:"parent,omitempty"`
	Author  string `json:"author,omitempty"`
}

// IntegratePayload holds integrate operation identifiers.
// [🛠️semio-repo/go/events.go#Payloads§IntegratePayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/INTEGRATE-PAYLOAD)
type IntegratePayload struct {
	Source        string `json:"source"`
	TargetFile    string `json:"target_file"`
	TargetSection string `json:"target_section"`
	Author        string `json:"author,omitempty"`
}

// ExtractPayload holds extract operation identifiers.
// [🛠️semio-repo/go/events.go#Payloads§ExtractPayload](semiorepo://definition/semio-repo/go/events.go/PAYLOADS/EXTRACT-PAYLOAD)
type ExtractPayload struct {
	SourceFile    string `json:"source_file"`
	SourceSection string `json:"source_section"`
	TargetFile    string `json:"target_file"`
	Author        string `json:"author,omitempty"`
}

// #endregion 🔖Payloads
