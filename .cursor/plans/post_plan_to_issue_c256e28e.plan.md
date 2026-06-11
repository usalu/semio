---
name: Post Plan To Issue
overview: When a ticket is opened or reopened with a bound plan (e.g. .cursor/plans for Cursor, and the equivalent for Claude/Codex/Copilot/Kiro), post the plan's markdown to the ticket's GitHub issue as a collapsible comment.
todos:
  - id: region
    content: Add 📝TicketPlanComment region in main.go with stripPlanFrontmatter, formatPlanComment, and postTicketPlanComment helpers.
    status: completed
  - id: bodyfile
    content: Update ghAddComment to use a temp --body-file for robustness with large plan bodies.
    status: completed
  - id: open
    content: Call postTicketPlanComment in CreateTicket after issue creation, before SaveTicket.
    status: completed
  - id: reopen
    content: Call postTicketPlanComment in ReopenTicket after the prompt comment, before SaveTicket.
    status: completed
  - id: validate
    content: Rebuild the Cursor MCP binary and confirm the plan comment is posted on open and reopen with [DEBUG] logs, then remove the logs.
    status: completed
isProject: false
---

# Post Bound Plan To GitHub Issue As A Comment

## Context

The repo MCP server is Go code in [repo/client/cli/main.go](repo/client/cli/main.go). A plan is already bound to a ticket through `plan_id`/`spec_id`:

- `ApplyTicketPlanFromIDs` (line 45267) resolves the id via `ResolvePlanSource` (line 45187) and stores `ticket.Plan.Source` (an absolute path to e.g. `.cursor/plans/<slug>_<id>.plan.md`, a `~/.claude/plan/<id>.md`, or a `.kiro/specs/<id>/` directory).
- It is called on open (`CreateTicket`, line 22158) and on reopen (`ReopenTicket`, line 24759).

GitHub interaction goes through `GetManagementProvider()` → `ghAddComment` (line 22475). Today the plan is never sent to GitHub; it is only archived into the ticket folder on close (`moveTicketPlanIntoFolder`). Issue comments are posted on open (issue body only), reopen (prompt), and close (summary).

Decisions confirmed: render the plan markdown inside a collapsible `<details>` (frontmatter stripped); post on open and reopen; leave windsurf/droid/antigravity out of scope (they have no plan resolution).

## Changes (all in [repo/client/cli/main.go](repo/client/cli/main.go))

### 1. New region `📝TicketPlanComment`

Add after the `📦MoveTicketPlan` region (after line 45349). Three helpers:

- `stripPlanFrontmatter(content string) string` — if content starts with `---\n`, drop through the next `\n---` line; return the remainder trimmed.
- `formatPlanComment(plan *TicketPlan, src string) (string, error)` — `os.Stat(src)`:
  - File: read it, strip frontmatter, wrap as a single section.
  - Directory (Kiro spec): glob `*.md`, sorted, one section per file.
  - Each section is a collapsible block so long plans stay tidy:

```text
# 📋 Plan

<details>
<summary>gumball_to_flow_9e9ce826.plan.md</summary>

<plan markdown body>

</details>
```

  Return empty string (no error) when there is no readable content.
- `postTicketPlanComment(ticket *Ticket, noManagement bool)` — guards (`noManagement`, `ticket.Plan == nil`, empty `Plan.Source`, missing `Management.Issue`), builds the body via `formatPlanComment`, and calls `GetManagementProvider().AddComment(issueURL, body)`. On error it prints a `Warning:` (same non-fatal pattern as the existing comment calls) so issue creation is never blocked.

### 2. Make `ghAddComment` robust for large bodies (line 22475)

Plans can exceed shell `ARG_MAX`. Switch from `--body` to writing the comment to an `os.CreateTemp` file and passing `--body-file <tmp>` (deferred remove). This keeps the `ManagementProvider.AddComment` interface unchanged and also benefits existing prompt/summary comments.

### 3. Call the helper on open

In `CreateTicket`, after the issue-creation block (after line 22166) and before `SaveTicket` (line 22168):

```go
postTicketPlanComment(ticket, noManagement)
```

At this point `ensureTicketGitHubIssue` has set `ticket.Management.Issue` and `ticket.Plan.Source` still points at the live plan file.

### 4. Call the helper on reopen

In `ReopenTicket`, after the existing prompt-comment block (after line 24815) and before `SaveTicket` (line 24817):

```go
postTicketPlanComment(ticket, noManagement)
```

## Why this works for "other IDEs" automatically

The helper reads `ticket.Plan.Source`, which is populated identically for every client that `ResolvePlanSource` supports (Cursor, Copilot, Claude, Codex, and Kiro spec directories). No per-client branching is needed in the comment path.

## Validation

- Rebuild the Cursor MCP binary (entry [repo/client/mcp/cursor/main.go](repo/client/mcp/cursor/main.go)) using the existing build task.
- Open a throwaway ticket via the rebuilt MCP with a real `plan_id`, with temporary `[DEBUG]` logs in `postTicketPlanComment` confirming the resolved source and the `gh issue comment` exit code, then verify the comment appears on the issue. Confirm reopen posts it too. Remove the `[DEBUG]` logs after confirming.
- This work must be done inside a ticket per repo rules (reopen the existing `GUMBALL-TO-FLOW`-adjacent ticket only if it covers this; otherwise open a new ticket associated with the most appropriate goal from `repo://goals`).