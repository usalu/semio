---
goal: R26-03
---

# Ticket

## Summary

Comprehensive audit completed

## Plan

- [x] Search all references in repo/cli/main.go
- [x] Search all references in repo/cli/main_test.go
- [x] Search all references in repo/vscode/extension.ts
- [x] Search all references in repo/vscode/extension.test.ts
- [x] Search all references in repo/go/repo/main.go (does not exist)
- [x] Compile results with line numbers and context

## Changes

No code changes. Research-only ticket.

## Log

- 2026-02-23: Completed full codebase audit across all specified files.

## Results

### FILE 1: repo/cli/main.go

#### `TicketPath` (field on Ticket struct)

**Line 8388** — in `Ticket` struct definition:

```go
// L8384: Interactions  []Interaction         `json:"-" yaml:"-"`
// L8385: Sessions      []TicketSession       `json:"sessions,omitempty" yaml:"sessions,omitempty"`
// L8386: FolderPath    string                `json:"-" yaml:"-"`
// L8387: JsonPath      string                `json:"-" yaml:"-"`
// L8388: TicketPath    string                `json:"-" yaml:"-"`
// L8389: ImportantPath string                `json:"-" yaml:"-"`
```

#### `TicketProgressInput` (type definition)

**Lines 9304-9312** — in `GraphQL Input Types` section:

```go
// L9304: // TicketProgressInput holds the data fields for a ticket progress input record.
// L9306: type TicketProgressInput struct {
// L9307:   Year    int    `json:"year"`
// L9308:   Month   int    `json:"month"`
// L9309:   Day     int    `json:"day"`
// L9310:   Slug    string `json:"slug"`
// L9311:   Summary string `json:"summary,omitempty"`
// L9312: }
```

#### `ticket_progress` (policy check string)

**Line 17630** — in `repoPolicy` function:

```go
// L17628: canonicalCommands := []string{
// L17629:   "tree",
// L17630:   "ticket_open", "ticket_close", "ticket_reopen", "ticket_progress",
// L17631:   "goal_open", "goal_close", "goal_reopen",
```

#### `ToolTicketProgress` (policy tracking token check)

**Line 17654** — in `repoPolicy` function:

```go
// L17652: trackingTokens := []string{
// L17653:   "ToolTicketOpen",
// L17654:   "ToolTicketClose",
// L17655:   "ToolTicketProgress",
// L17656: }
```

NOTE: `ToolTicketProgress` is referenced but NEVER DEFINED as a function anywhere in the codebase. The policy checks for its existence in `go/repo/main.go`.

#### `ticket.TicketPath` and `ticket.md` (in BuildCodebaseTickets)

**Lines 18446-18448** — in `BuildCodebaseTickets` function:

```go
// L18446: ticketPath := ticket.TicketPath
// L18447: if ticketPath == "" {
// L18448:   ticketPath = fmt.Sprintf(".repo/🎫️/%02d/%02d/%02d/%s/ticket.md", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
// L18449: }
```

**Lines 18481-18482** — same function, building CodebaseTicket:

```go
// L18480: result = append(result, CodebaseTicket{
// L18481:   ID:   ticketID,
// L18482:   Path: ticketPath,
// L18483:   URI:  ctx.FileURI(ticketPath),
```

#### `GetTicketPath` (function definition)

**Lines 18981-18985** — in `Types/Tickets` section:

```go
// L18984: func GetTicketPath(year, month, day int, slug string) string {
// L18985:   return filepath.Join(GetTicketsDir(), PadNumber(year, 2), PadNumber(month, 2), PadNumber(day, 2), slug)
// L18986: }
```

#### `GetTicketFilePath` (function definition)

**Lines 18988-18992** — in `Types/Tickets` section:

```go
// L18991: func GetTicketFilePath(year, month, day int, slug string) string {
// L18992:   return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.md")
// L18993: }
```

#### `GetTicketPath` usages in other path functions

**Line 18999** — in `GetImportantFilePath`:

```go
// L18999: return filepath.Join(GetTicketPath(year, month, day, slug), "important.md")
```

**Line 19006** — in `GetTicketJsonPath`:

```go
// L19006: return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.json")
```

#### `GetTicketPath` and `GetTicketFilePath` in RenameTicket

**Lines 19132-19148** — in `RenameTicket` function:

```go
// L19132: newFolderPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, slug)
// ...
// L19148: ticket.TicketPath = GetTicketFilePath(ticket.Year, ticket.Month, ticket.Day, slug)
```

#### `GetTicketPath`, `GetTicketFilePath`, `buildTicketMarkdown`, `TicketPath` in CreateTicket

**Lines 19196-19245** — in `CreateTicket` function:

```go
// L19196: ticketDir := GetTicketPath(year, month, day, slug)
// L19202: ticketFilePath := GetTicketFilePath(year, month, day, slug)
// L19225: if err := WriteTextFile(ticketFilePath, buildTicketMarkdown(goal, parent)); err != nil {
// L19245: TicketPath:    ticketFilePath,
```

#### `buildTicketMarkdown` (function definition)

**Lines 19451-19468** — standalone function:

```go
// L19451: func buildTicketMarkdown(goal, parent string) string {
// L19452:   var builder strings.Builder
// ...builds markdown with frontmatter, # Ticket, ## Summary, ## Changes, ## Log, ## Todos, ## Plan...
// L19468: }
```

#### `updateTicketSummaryFile` (function definition)

**Lines 19471-19484** — standalone function:

```go
// L19471: func updateTicketSummaryFile(ticketPath, summary string) error {
// L19472:   if ticketPath == "" {
// L19473:     return nil
// L19474:   }
// L19475:   content, err := ReadTextFile(ticketPath)
// ...
// L19482:     return WriteTextFile(ticketPath, content)
// L19484:   return WriteTextFile(ticketPath, replaceSectionContent(content, marker, summary))
// L19485: }
```

#### `GetTicketPath`, `GetTicketFilePath`, `TicketPath` in ReadTicket

**Lines 19921-19942** — in `ReadTicket` function:

```go
// L19921: folderPath := GetTicketPath(year, month, day, slug)
// L19923: ticketPath := GetTicketFilePath(year, month, day, slug)
// ...
// L19942: ticket.TicketPath = ticketPath
```

#### `ProgressTicket` (function definition)

**Lines 21512-21547** — in `Types/Tickets` section:

```go
// L21515: func ProgressTicket(ticket *Ticket, summary string) (string, error) {
// L21516:   if summary == "" {
// L21517:     return "No summary provided", nil
// L21518:   }
// L21520:   entry := fmt.Sprintf("\n- %s: %s", time.Now().Format("2006-01-02 15:04"), summary)
// L21522:   content, err := ReadTextFile(ticket.TicketPath)
// ...
// L21541:   if err := WriteTextFile(ticket.TicketPath, newContent); err != nil {
```

#### `updateTicketSummaryFile` + `ticket.TicketPath` in FinishTicket

**Line 21645** — in `FinishTicket` function:

```go
// L21644: ticket.Summary = summary
// L21645: if err := updateTicketSummaryFile(ticket.TicketPath, summary); err != nil {
// L21646:   return err
// L21647: }
```

#### `ticket.TicketPath` in ToolTicketRead (ReadTextFile)

**Line 21909** — in `ToolTicketRead` function:

```go
// L21909: ticketContent, _ := ReadTextFile(ticket.TicketPath)
```

#### `TicketProgress` in RepoContext interface

**Line 23744** — in `RepoContext` interface:

```go
// L23744: TicketProgress(input TicketProgressInput) (string, error)
```

#### `GetTicketPath` in TicketDelete

**Line 24597** — in `repoContext.TicketDelete`:

```go
// L24597: path = GetTicketPath(input.Year, input.Month, input.Day, input.Slug)
```

#### `TicketProgress` on repoContext (method)

**Lines 26345-26351** — in `repoContext`:

```go
// L26345: func (c *repoContext) TicketProgress(input TicketProgressInput) (string, error) {
// L26346:   ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
// L26350:   return ProgressTicket(ticket, input.Summary)
// L26351: }
```

#### `TicketProgress` on defaultContext (no-op)

**Lines 27054-27056** — in `defaultContext`:

```go
// L27054: func (c *defaultContext) TicketProgress(input TicketProgressInput) (string, error) {
// L27055:   return "", nil
// L27056: }
```

#### `ticket.TicketPath` in GraphQL schema (path resolver)

**Lines 28298-28299** — in GraphQL ticket type definition:

```go
// L28297: Resolve: func(p graphql.ResolveParams) (interface{}, error) {
// L28298:   ticket := p.Source.(*Ticket)
// L28299:   if ticket.TicketPath != "" {
// L28300:     return ticket.TicketPath, nil
// L28301:   }
// L28302:   return ticket.JsonPath, nil
```

#### `ticket.TicketPath` in GraphQL schema (uri resolver)

**Lines 28342-28343** — in GraphQL ticket type definition:

```go
// L28341: ticket := p.Source.(*Ticket)
// L28342: path := ticket.JsonPath
// L28343: if ticket.TicketPath != "" {
// L28344:   path = ticket.TicketPath
// L28345: }
```

#### `TicketProgress` on mutationResolver (GraphQL)

**Lines 30827-30832** — in `mutationResolver`:

```go
// L30827: func (r *mutationResolver) TicketProgress(ctx context.Context, input TicketProgressInput) (string, error) {
// L30828:   if r.Ctx != nil {
// L30829:     return r.Ctx.TicketProgress(input)
// L30830:   }
// L30831:   return "", fmt.Errorf("not implemented")
// L30832: }
```

---

### FILE 2: repo/cli/main_test.go

#### `GetTicketPath` in TestGraphQL ticketOpen cleanup

**Line 532** — in `TestGraphQL` function (ticketOpen tests):

```go
// L530: to := resp.TicketOpen
// L531: ...
// L532: path := GetTicketPath(to.Year, to.Month, to.Day, to.Slug)
// L533: os.RemoveAll(path)
```

#### `ticket.md` in TestFilterTicketWorkspaceFiles

**Line 568** — in `TestFilterTicketWorkspaceFiles`:

```go
// L567: files := []string{
// L568:   ".repo/🎫️/26/01/20/SAMPLE/plan.md",
// L569:   "./.repo/🎫️/26/01/20/SAMPLE/ticket.md",
// L570:   filepath.Join(rootDir, ".repo", "🎫️", "26", "01", "20", "SAMPLE", "extra.txt"),
```

#### `GetTicketPath` in TestTicketReopenRejectsAlreadyOpenTicket cleanup

**Line 7331** — in `TestTicketReopenRejectsAlreadyOpenTicket`:

```go
// L7330: y, m, d, slug := parseTicketOpenResult(t, openOut)
// L7331: defer os.RemoveAll(GetTicketPath(y, m, d, slug))
// L7332: ticketPath := fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug)
```

#### `GetTicketPath` in TestTicketChange cleanup and usage

**Lines 8987, 9004** — in `TestTicketChange`:

```go
// L8987: defer os.RemoveAll(GetTicketPath(y, m, d, slug))
// ...
// L9004: ticketDir := GetTicketPath(y, m, d, slug)
// L9005: jsonContent, err := os.ReadFile(filepath.Join(ticketDir, "ticket.json"))
```

#### `GetTicketPath` in TestToolTicketLifecycle cleanup

**Lines 11140-11141** — in `TestToolTicketLifecycle`:

```go
// L11140: ticketPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
// L11141: os.RemoveAll(ticketPath)
```

---

### FILE 3: repo/vscode/extension.ts

#### `ticket.md` in ticket navigation (line 814)

**Lines 813-814** — in `activate` function, `ticket` case handler:

```typescript
// L813: const day = String(node.Day).padStart(2, "0");
// L814: ticketPath = path.join(wsRoot, ".repo", "🎫️", year, month, day, slug, "ticket.md");
```

#### `ticket.md` in ticket navigation fallback (line 818)

**Lines 817-818** — in `activate` function, `ticket` case handler fallback:

```typescript
// L817: if (parsed.path.match(/^\d+\/\d+\/\d+\/.+/)) {
// L818:   ticketPath = path.join(wsRoot, ".repo", "🎫️", parsed.path, "ticket.md");
```

#### `filePath` in TicketData interface (line 162)

**Line 162** — in `TicketData` interface:

```typescript
// L161: slug: string;
// L162: filePath: string;
// L163: frontmatter: Record<string, unknown>;
```

#### `filePath?` in inline type (line 244)

**Line 244** — in a type definition:

```typescript
// L244: filePath?: string;
```

#### `extractFilePathFromScope` function (line 1017)

**Lines 1017-1056** — standalone function:

```typescript
// L1017: function extractFilePathFromScope(scope: string): string | undefined {
// L1053:   const filePath = parts[0];
// L1054:   if (filePath.endsWith(".ts") || ... || filePath.endsWith(".sh")) {
// L1055:     return filePath;
// L1056:   }
```

#### `resolveTicketPath` function (lines 1060-1067)

**Lines 1060-1067** — standalone function:

```typescript
// L1060: function resolveTicketPath(ticket: { year; month; day; slug; filePath? }): string | undefined {
// L1061:   if (ticket.filePath) return ticket.filePath;
// L1065:   const relPath = path.join(..., ticket.slug, "ticket.md");
// L1066:   return path.join(root, ".repo", "🎫️", relPath);
// L1067: }
```

#### `openFileAtLine` function (line 1069)

**Lines 1069-1072** — standalone function:

```typescript
// L1069: async function openFileAtLine(filePath: string, startLine: number, endLine?: number): Promise<void> {
// L1072:   const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
```

#### `filePath` in breach diagnostics (lines 1176-1178)

**Lines 1176-1178** — in breach diagnostics loop:

```typescript
// L1176: const filePath = extractFilePathFromScope(breach.scope);
// L1177: if (!filePath) continue;
// L1178: const absPath = path.join(root, filePath);
```

#### `filePath` in SectionTreeItem class (line 1577)

**Line 1577** — in `SectionTreeItem` constructor:

```typescript
// L1577: public filePath: string
```

#### `filePath` in SectionTreeProvider (lines 1640-1675)

**Lines 1640-1675** — in `SectionTreeProvider.getChildren`:

```typescript
// L1640: const filePath = path.relative(root, uri.fsPath);
// L1650:   ...`"${binaryPath}" section list --file "${filePath}" --json"`...
// L1673: private createSectionItems(sections: SectionInfo[], filePath: string): SectionTreeItem[] {
```

#### `filePath` in navigateToFile command (lines 1805-1814)

**Lines 1805-1814** — in command registration:

```typescript
// L1805: register("compose.navigateToFile", async (filePath: string) => {
// L1808:   const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
```

#### `filePath` in section navigate-to-line (lines 1820-1823)

**Lines 1820-1823** — in command handler:

```typescript
// L1820: const filePath = payload?.filePath;
// L1822: if (!filePath || typeof item?.range?.start !== "number") return;
// L1823: return openFileAtLine(filePath, item.range.start, item.range.end ?? undefined);
```

#### `resolveTicketPath` usage in ticketOpen command (lines 1862-1863)

**Lines 1862-1863** — in `compose.ticketOpen` command:

```typescript
// L1862: const t = { year, month, day, slug, filePath: undefined as string | undefined };
// L1863: const p = resolveTicketPath(t);
```

---

### FILE 4: repo/vscode/extension.test.ts

#### `filePath` and `ticket.md` in TicketData test (line 800)

**Lines 790-802** — in test suite:

```typescript
// L791: const ticket: TicketData = {
// L792:   year: 2024,
// L793:   month: 1,
// L794:   day: 1,
// L795:   slug: "test-ticket",
// L800:   filePath: "/path/to/ticket/ticket.md",
// L801:   interactions: []
// L802: };
```

---

### FILE 5: repo/go/repo/main.go

**File does not exist.** The directory `repo/go/` contains only: `README.md`, `emit.go`, `events.go`, `go.mod`. There are NO references to `ToolTicketProgress` in any Go file outside main.go.

---

### KEY FINDING: `ToolTicketProgress` is NEVER DEFINED

The policy at line 17654 checks for `ToolTicketProgress` in `go/repo/main.go`, but:

1. `go/repo/main.go` does not exist
2. `func ToolTicketProgress` is not defined anywhere in the codebase
3. The policy references it as a "tracking token" alongside `ToolTicketOpen` and `ToolTicketClose` which ARE defined

## Todos

- [x] Deliver comprehensive reference list
