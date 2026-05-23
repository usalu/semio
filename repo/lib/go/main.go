// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// Shared Go library for repo CLI and server: event kinds, payloads, and emit helper.
// #endregion 🧲Header

package repo

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"
)

// #region 📋EventKind

// 📡EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.
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

// #endregion 📋EventKind

// #region 🏗️Event

// ✉️Event is the canonical envelope for a changing interaction sent from CLI to server.
type Event struct {
	Kind    EventKind       `json:"kind"`
	Source  string          `json:"source"`
	Payload json.RawMessage `json:"payload"`
}

// #endregion 🏗️Event

// #region 🌨️Payloads

// 📦TicketPayload holds common ticket identifiers.
type TicketPayload struct {
	ID    string `json:"id"`
	Year  int    `json:"year,omitempty"`
	Month int    `json:"month,omitempty"`
	Day   int    `json:"day,omitempty"`
	Slug  string `json:"slug,omitempty"`
}

// 🎫TicketOpenPayload payload for ticket.open.
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

// 📪TicketClosePayload payload for ticket.close.
type TicketClosePayload struct {
	TicketPayload
	Summary string   `json:"summary"`
	Files   []string `json:"files"`
	Author  string   `json:"author,omitempty"`
}

// 🔓TicketReopenPayload payload for ticket.reopen.
type TicketReopenPayload struct {
	TicketPayload
	Prompt string `json:"prompt"`
	LLM    string `json:"llm,omitempty"`
	Client string `json:"client"`
	Author string `json:"author,omitempty"`
}

// ♻️TicketChangePayload payload for ticket.change.
type TicketChangePayload struct {
	TicketPayload
	Title  *string `json:"title,omitempty"`
	Prompt *string `json:"prompt,omitempty"`
	Goal   *string `json:"goal,omitempty"`
	Parent *string `json:"parent,omitempty"`
	Author string  `json:"author,omitempty"`
}

// ⛳GoalPayload holds common goal identifiers.
type GoalPayload struct {
	ID string `json:"id"`
}

// 🎯GoalOpenPayload payload for goal.open.
type GoalOpenPayload struct {
	GoalPayload
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Author      string `json:"author,omitempty"`
}

// 🏁GoalClosePayload payload for goal.close.
type GoalClosePayload struct {
	GoalPayload
	Summary string `json:"summary"`
	Author  string `json:"author,omitempty"`
}

// 🔄GoalReopenPayload payload for goal.reopen.
type GoalReopenPayload struct {
	GoalPayload
	Prompt string `json:"prompt"`
	Client string `json:"client"`
	LLM    string `json:"llm,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📐GoalChangePayload payload for goal.change.
type GoalChangePayload struct {
	GoalPayload
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	Author      string  `json:"author,omitempty"`
}

// 👥ContributorPayload holds contributor identifiers.
type ContributorPayload struct {
	Github string `json:"github"`
	Author string `json:"author,omitempty"`
}

// 💾CheckpointPayload payload for checkpoint (GitHub push).
type CheckpointPayload struct {
	Author       string   `json:"author"`
	Github       string   `json:"github"`
	Sha          string   `json:"sha"`
	Message      string   `json:"message"`
	Files        []string `json:"files"`
	Technologies []string `json:"technologies,omitempty"`
	Bundles      []string `json:"bundles,omitempty"`
	Folders      []string `json:"folders,omitempty"`
	FilesChanged []string `json:"files_changed,omitempty"`
	Sections     []string `json:"sections,omitempty"`
	Definitions  []string `json:"definitions,omitempty"`
}

// ✅TodoPayload holds todo identifiers.
type TodoPayload struct {
	ID       string `json:"id"`
	ParentID string `json:"parent_id,omitempty"`
	Name     string `json:"name,omitempty"`
	Author   string `json:"author,omitempty"`
}

// 🆕TodoCreatePayload payload for todo.create.
type TodoCreatePayload struct {
	TodoPayload
}

// ✏️TodoChangePayload payload for todo.change.
type TodoChangePayload struct {
	TodoPayload
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// 🗑️TodoDeletePayload payload for todo.delete.
type TodoDeletePayload struct {
	TodoPayload
}

// 💼WorkItem represents a single item a contributor is working on (technology, bundle, folder, file, section, definition, ticket, goal, todo).
type WorkItem struct {
	Kind string `json:"kind"`
	ID   string `json:"id"`
}

// 🤝ContributorWork holds all work items for one contributor.
type ContributorWork struct {
	Github       string   `json:"github"`
	Tickets      []string `json:"tickets"`
	Goals        []string `json:"goals"`
	Todos        []string `json:"todos"`
	Technologies []string `json:"technologies"`
	Bundles      []string `json:"bundles"`
	Folders      []string `json:"folders"`
	Files        []string `json:"files"`
	Sections     []string `json:"sections"`
	Definitions  []string `json:"definitions"`
}

// 📝DraftPayload holds draft identifiers.
type DraftPayload struct {
	Slug   string `json:"slug"`
	Title  string `json:"title,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📄FilePayload holds file operation identifiers.
type FilePayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📁FolderPayload holds folder operation identifiers.
type FolderPayload struct {
	Path   string `json:"path"`
	From   string `json:"from,omitempty"`
	Author string `json:"author,omitempty"`
}

// 📑SectionPayload holds section operation identifiers.
type SectionPayload struct {
	File    string `json:"file"`
	Name    string `json:"name"`
	OldName string `json:"old_name,omitempty"`
	Parent  string `json:"parent,omitempty"`
	Author  string `json:"author,omitempty"`
}

// 🧬IntegratePayload holds integrate operation identifiers.
type IntegratePayload struct {
	Source        string `json:"source"`
	TargetFile    string `json:"target_file"`
	TargetSection string `json:"target_section"`
	Author        string `json:"author,omitempty"`
}

// 🧲ExtractPayload holds extract operation identifiers.
type ExtractPayload struct {
	SourceFile    string `json:"source_file"`
	SourceSection string `json:"source_section"`
	TargetFile    string `json:"target_file"`
	Author        string `json:"author,omitempty"`
}

// #endregion 🌨️Payloads

// #region 🧳Emit

// 📤Emit posts an event to the repo server. No-op when SEMIO_SERVER_ADDR is unset.
func Emit(kind EventKind, source string, payload interface{}) {
	addr := strings.TrimSpace(os.Getenv("SEMIO_SERVER_ADDR"))
	if addr == "" {
		return
	}
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return
	}
	ev := Event{Kind: kind, Source: source, Payload: payloadBytes}
	body, err := json.Marshal(ev)
	if err != nil {
		return
	}
	url := addr
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		url = "http://" + addr
	}
	url = strings.TrimSuffix(url, "/") + "/api/v1/events"
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if token := strings.TrimSpace(os.Getenv("SEMIO_SERVER_TOKEN")); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(req)
}

// #endregion 🧳Emit

// #region 📹Parsing

// Shared source code parsing primitives for section/definition extraction used by CLI and server.

// 📑ParsedSection represents a parsed source code section from region markers or markdown headings.
type ParsedSection struct {
	Name      string
	Path      string
	StartLine int
	EndLine   int
}

// 📖ParsedDefinition represents a parsed source code definition from regex patterns.
type ParsedDefinition struct {
	Name      string
	StartLine int
	EndLine   int
}

// 🎭IsEmojiRune returns true if the rune is likely an emoji base character.
func IsEmojiRune(r rune) bool {
	if r >= 0x1F600 && r <= 0x1F64F {
		return true
	}
	if r >= 0x1F300 && r <= 0x1F5FF {
		return true
	}
	if r >= 0x1F680 && r <= 0x1F6FF {
		return true
	}
	if r >= 0x1F700 && r <= 0x1F77F {
		return true
	}
	if r >= 0x1F780 && r <= 0x1F7FF {
		return true
	}
	if r >= 0x1F800 && r <= 0x1F8FF {
		return true
	}
	if r >= 0x1F900 && r <= 0x1F9FF {
		return true
	}
	if r >= 0x1FA00 && r <= 0x1FA6F {
		return true
	}
	if r >= 0x1FA70 && r <= 0x1FAFF {
		return true
	}
	if r >= 0x2600 && r <= 0x26FF {
		return true
	}
	if r >= 0x2700 && r <= 0x27BF {
		return true
	}
	if r >= 0x2300 && r <= 0x23FF {
		return true
	}
	if r >= 0x2B50 && r <= 0x2B55 {
		return true
	}
	if r >= 0x200D && r <= 0x200D {
		return true
	}
	if r >= 0xFE00 && r <= 0xFE0F {
		return true
	}
	if r == 0x2139 || r == 0x2194 || r == 0x2195 {
		return true
	}
	if r >= 0x2196 && r <= 0x2199 {
		return true
	}
	if r >= 0x21A9 && r <= 0x21AA {
		return true
	}
	if r >= 0x231A && r <= 0x231B {
		return true
	}
	if r >= 0x25AA && r <= 0x25AB {
		return true
	}
	if r >= 0x25B6 && r <= 0x25C0 {
		return true
	}
	if r >= 0x25FB && r <= 0x25FE {
		return true
	}
	if r >= 0x2614 && r <= 0x2615 {
		return true
	}
	if r >= 0x2648 && r <= 0x2653 {
		return true
	}
	if r >= 0x267F && r <= 0x267F {
		return true
	}
	if r >= 0x2693 && r <= 0x2693 {
		return true
	}
	if r >= 0x26A1 && r <= 0x26A1 {
		return true
	}
	if r >= 0x26AA && r <= 0x26AB {
		return true
	}
	if r >= 0x26BD && r <= 0x26BE {
		return true
	}
	if r >= 0x26C4 && r <= 0x26C5 {
		return true
	}
	if r >= 0x26CE && r <= 0x26CF {
		return true
	}
	if r >= 0x26D4 && r <= 0x26D4 {
		return true
	}
	if r >= 0x26EA && r <= 0x26EA {
		return true
	}
	if r >= 0x26F2 && r <= 0x26F3 {
		return true
	}
	if r >= 0x26F5 && r <= 0x26F5 {
		return true
	}
	if r >= 0x26FA && r <= 0x26FA {
		return true
	}
	if r >= 0x26FD && r <= 0x26FD {
		return true
	}
	if r == 0x203C || r == 0x2049 {
		return true
	}
	if r == 0x20E3 {
		return true
	}
	if r == 0x00A9 || r == 0x00AE {
		return true
	}
	if r == 0x2122 {
		return true
	}
	return false
}

// 🧲ExtractEntityEmoji extracts the leading emoji and remaining text from a string.
func ExtractEntityEmoji(s string) (string, string) {
	if s == "" {
		return "", ""
	}
	runes := []rune(s)
	if len(runes) == 0 {
		return "", ""
	}
	i := 0
	r := runes[i]
	if !IsEmojiRune(r) {
		return "", s
	}
	i++
	for i < len(runes) {
		r = runes[i]
		if r == 0xFE0F || r == 0xFE0E {
			i++
		} else if r == 0x20E3 {
			i++
		} else if r == 0x200D {
			i++
			if i < len(runes) {
				i++
				for i < len(runes) && (runes[i] == 0xFE0F || runes[i] == 0xFE0E) {
					i++
				}
			}
		} else if r >= 0x1F3FB && r <= 0x1F3FF {
			i++
		} else {
			break
		}
	}
	emoji := string(runes[:i])
	remaining := string(runes[i:])
	return emoji, remaining
}

// 💬ParseRegionMarker detects region start/end markers in a line, stripping common comment prefixes.
// 💬Supports any emoji prefix (e.g. #region 📋EventKind, #region 🔖Legacy).
func ParseRegionMarker(line string) (string, bool, bool) {
	trimmed := strings.TrimSpace(line)
	trimmed = strings.TrimPrefix(trimmed, "//")
	trimmed = strings.TrimPrefix(trimmed, "#")
	trimmed = strings.TrimPrefix(trimmed, "--")
	trimmed = strings.TrimPrefix(trimmed, "/*")
	trimmed = strings.TrimSuffix(trimmed, "*/")
	trimmed = strings.TrimSpace(trimmed)
	if strings.HasPrefix(trimmed, "#region ") {
		content := strings.TrimSpace(strings.TrimPrefix(trimmed, "#region "))
		emoji, name := ExtractEntityEmoji(content)
		if emoji != "" {
			return name, true, false
		}
	}
	if strings.HasPrefix(trimmed, "#endregion ") {
		content := strings.TrimSpace(strings.TrimPrefix(trimmed, "#endregion "))
		emoji, name := ExtractEntityEmoji(content)
		if emoji != "" {
			return name, true, true
		}
	}
	return "", false, false
}

// 🔬ParseMarkdownHeading parses a markdown heading line into level and title.
func ParseMarkdownHeading(line string) (int, string) {
	trimmed := strings.TrimSpace(line)
	if !strings.HasPrefix(trimmed, "#") {
		return 0, ""
	}
	level := 0
	for level < len(trimmed) && trimmed[level] == '#' {
		level++
	}
	if level == 0 || level > 6 {
		return 0, ""
	}
	name := strings.TrimSpace(trimmed[level:])
	if name == "" {
		return 0, ""
	}
	return level, name
}

// 🗣️DefinitionPatterns returns language-specific regex patterns for extracting definitions by file extension.
func DefinitionPatterns(ext string) []*regexp.Regexp {
	switch ext {
	case ".go":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*func\s+(?:\([^\)]*\)\s*)?([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*type\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*var\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*const\s+([A-Za-z0-9_]+)`),
		}
	case ".ts", ".tsx", ".js", ".jsx":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?class\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?interface\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?type\s+([A-Za-z0-9_]+)`),
		}
	case ".py":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*def\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*class\s+([A-Za-z0-9_]+)`),
		}
	case ".cs":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:class|struct|interface|enum|record)\s+([A-Za-z0-9_]+)`),
		}
	case ".rs":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?struct\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?enum\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:pub\s+)?trait\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*impl\s+([A-Za-z0-9_]+)`),
		}
	case ".rb":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*def\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*class\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*module\s+([A-Za-z0-9_]+)`),
		}
	case ".md", ".mdx":
		return []*regexp.Regexp{}
	default:
		return []*regexp.Regexp{}
	}
}

// 🌳ParseSectionsFromLines extracts sections from source lines using region markers and markdown headings.
func ParseSectionsFromLines(lines []string, ext string) []ParsedSection {
	var sections []ParsedSection
	type sectionFrame struct {
		Name      string
		StartLine int
		Level     int
		Path      string
	}
	var stack []sectionFrame
	for index, line := range lines {
		lineNumber := index + 1
		if name, ok, isEnd := ParseRegionMarker(line); ok {
			if isEnd {
				if len(stack) > 0 {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					sections = append(sections, ParsedSection{
						Name:      frame.Name,
						Path:      frame.Path,
						StartLine: frame.StartLine,
						EndLine:   lineNumber - 1,
					})
				}
			} else {
				path := name
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + name
				}
				stack = append(stack, sectionFrame{Name: name, StartLine: lineNumber, Level: 0, Path: path})
			}
			continue
		}
		if ext == ".md" || ext == ".mdx" {
			if level, title := ParseMarkdownHeading(line); level > 0 {
				for len(stack) > 0 && stack[len(stack)-1].Level >= level {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					sections = append(sections, ParsedSection{
						Name:      frame.Name,
						Path:      frame.Path,
						StartLine: frame.StartLine,
						EndLine:   lineNumber - 1,
					})
				}
				path := title
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + title
				}
				stack = append(stack, sectionFrame{Name: title, StartLine: lineNumber, Level: level, Path: path})
			}
		}
	}
	for _, frame := range stack {
		sections = append(sections, ParsedSection{
			Name:      frame.Name,
			Path:      frame.Path,
			StartLine: frame.StartLine,
			EndLine:   len(lines),
		})
	}
	return sections
}

// 🧩ParseDefinitionsFromLines extracts definitions from source lines using the given regex patterns.
func ParseDefinitionsFromLines(lines []string, patterns []*regexp.Regexp) []ParsedDefinition {
	var defs []ParsedDefinition
	for index, line := range lines {
		lineNumber := index + 1
		for _, pattern := range patterns {
			matches := pattern.FindStringSubmatch(line)
			if len(matches) > 1 {
				defs = append(defs, ParsedDefinition{
					Name:      matches[len(matches)-1],
					StartLine: lineNumber,
					EndLine:   lineNumber,
				})
				break
			}
		}
	}
	return defs
}

// 🔭BuildScopeID generates a deterministic scope ID from kind, file path, section path, and definition name.
func BuildScopeID(kind string, filePath string, sectionPath string, definition string) string {
	if kind == "file" {
		return fmt.Sprintf("file:%s", filePath)
	}
	if kind == "section" {
		return fmt.Sprintf("section:%s#%s", filePath, sectionPath)
	}
	if sectionPath != "" {
		return fmt.Sprintf("def:%s#%s::%s", filePath, sectionPath, definition)
	}
	return fmt.Sprintf("def:%s#%s", filePath, definition)
}

// 📍ScopeEntry holds kind, id, file, section, and definition for a parsed scope.
type ScopeEntry struct {
	Kind        string
	ID          string
	FilePath    string
	SectionPath string
	Definition  string
	StartLine   int
	EndLine     int
}

// 🏗️BuildScopesForFile parses file content into scope entries for file, sections, and definitions.
func BuildScopesForFile(path string, content string) []ScopeEntry {
	lines := strings.Split(content, "\n")
	ext := strings.ToLower(filepath.Ext(path))

	var entries []ScopeEntry

	fileEntry := ScopeEntry{
		Kind:      "file",
		ID:        BuildScopeID("file", path, "", ""),
		FilePath:  path,
		StartLine: 1,
		EndLine:   len(lines),
	}
	entries = append(entries, fileEntry)

	sections := ParseSectionsFromLines(lines, ext)
	for _, s := range sections {
		entry := ScopeEntry{
			Kind:        "section",
			ID:          BuildScopeID("section", path, s.Path, ""),
			FilePath:    path,
			SectionPath: s.Path,
			StartLine:   s.StartLine,
			EndLine:     s.EndLine,
		}
		entries = append(entries, entry)
	}

	sectionByLine := map[int]string{}
	for _, s := range sections {
		for line := s.StartLine; line <= s.EndLine; line++ {
			sectionByLine[line] = s.Path
		}
	}

	patterns := DefinitionPatterns(ext)
	defs := ParseDefinitionsFromLines(lines, patterns)
	for _, d := range defs {
		sp := sectionByLine[d.StartLine]
		entry := ScopeEntry{
			Kind:        "definition",
			ID:          BuildScopeID("definition", path, sp, d.Name),
			FilePath:    path,
			SectionPath: sp,
			Definition:  d.Name,
			StartLine:   d.StartLine,
			EndLine:     d.EndLine,
		}
		entries = append(entries, entry)
	}

	return entries
}

// #endregion 📹Parsing
