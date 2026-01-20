# Semio Repo Dev Server Plan (Stateful Go Server + Stateless Repo CLI)

This document is a **detailed implementation plan** for a small, stateful Go dev server that collaborates with a **stateless** repo binary (CLI).  
The server is the **source of truth** for synchronization, ticket state, repo semantic indexing, diff ingestion, warnings, and outbound webhooks (GitHub/Discord/etc.).

Constraints / requirements:

- **Server is stateful** (persistent DB + in-memory caches).
- **Repo binary is stateless** and only runs on explicit commands (no daemon, no long-lived watchers).
- Collaboration happens via:
  - **diffs** (git/uncommitted patch, staged patch, or file snapshots)
  - **events** (ticket open/close/reopen, file changed, precommit check, etc.)
- Server code lives in **exactly one file**: `go/server/main.go`.
- Keep it simple: handful of devs, single tenant.
- Provide an extensible structure: internal event bus + handlers + DB tables.

---

## 1) Core concept

### 1.1 Source of truth boundary

- The **server owns**:
  - Tickets (open/closed, prompt, summary)
  - File/section/definition index (semantic model)
  - Ownership / “claims” mapping (ticket → scopes)
  - Warnings and violations (policy engine)
  - Outbound notifications (Discord, etc.)
  - GitHub webhook enrichment (e.g., close-with-comment)

- The **repo binary (CLI) owns**:
  - Collecting diffs / working tree state
  - Executing local git commands
  - Sending explicit events to server
  - Displaying warnings/errors returned by server
  - Optionally applying autofixes returned by server (patch output)

The repo CLI does **not** persist server-related state locally.

---

## 2) Server runtime layout (single file)

Although the server is one file, structure it internally using sections:

- **Configuration**
- **DB schema + migrations**
- **Models (types)**
- **Event bus**
- **Diff ingestion + parsers**
- **Semantic index builder (sections/definitions)**
- **Ownership engine**
- **Warning/violation engine**
- **HTTP API (commands + queries)**
- **Webhook receivers**
- **Outbound webhook senders**
- **Main startup + graceful shutdown**

Use comment regions to keep it navigable:

```go
// #region Config
// ...
// #endregion Config
```

---

## 3) Data model (SQLite)

Use SQLite for persistence. Keep schema small and append-only where possible.

### 3.1 Tables

#### `repos`
- `id TEXT PRIMARY KEY`
- `name TEXT`
- `path TEXT` (server’s canonical repo root path)
- `created_at DATETIME`

#### `tickets`
- `id TEXT PRIMARY KEY`  
  Suggested: `@semio/tickets/YYYY/MM/DD/slug` to align with your existing IDs.
- `status TEXT` (`open|closed`)
- `title TEXT`
- `prompt TEXT` (last prompt)
- `summary TEXT` (close summary)
- `llm TEXT`
- `ui TEXT`
- `author TEXT`
- `github_issue TEXT` (optional)
- `created_at DATETIME`
- `closed_at DATETIME NULL`

#### `scopes`
A scope is a “thing in the repo” that can be claimed/owned.
- `id TEXT PRIMARY KEY`  
  Examples:
  - `file:go/repo/repo.go`
  - `section:go/repo/repo.go#Languages.Go`
  - `def:go/repo/repo.go#Languages.Go§NewGoLanguage`
- `kind TEXT` (`file|section|definition`)
- `file_path TEXT`
- `section_path TEXT NULL`
- `definition_name TEXT NULL`
- `start_line INT NULL`
- `end_line INT NULL`
- `updated_at DATETIME`

#### `ticket_claims`
Maps open tickets to claimed scopes.
- `ticket_id TEXT`
- `scope_id TEXT`
- `claim_type TEXT` (`touched|declared|inferred`)
- `first_seen_at DATETIME`
- `last_seen_at DATETIME`
- PRIMARY KEY (`ticket_id`, `scope_id`)

#### `violations`
- `id TEXT PRIMARY KEY`
- `kind TEXT`
- `priority TEXT` (`high|medium|low`)
- `scope_id TEXT`
- `file_path TEXT`
- `line INT NULL`
- `column INT NULL`
- `summary TEXT`
- `excerpt TEXT`
- `autofixable BOOL`
- `detected_at DATETIME`
- `ticket_id TEXT NULL` (if attributable)
- `resolved_at DATETIME NULL`

#### `warnings`
Warnings are “soft” conflicts or drift messages.
- `id TEXT PRIMARY KEY`
- `kind TEXT` (e.g., `conflict:definition`, `drift:section-moved`)
- `severity TEXT` (`info|warn|error`)
- `message TEXT`
- `ticket_id TEXT NULL`
- `scope_id TEXT NULL`
- `created_at DATETIME`
- `acknowledged_at DATETIME NULL`
- `ack_by TEXT NULL`

#### `events`
Optional but very useful for debugging/replay.
- `id TEXT PRIMARY KEY`
- `type TEXT`
- `source TEXT` (`repo-cli|github|server`)
- `payload_json TEXT`
- `created_at DATETIME`

> For a small team, an `events` table provides excellent observability and makes everything debuggable.

---

## 4) Event system (internal)

### 4.1 Why an internal event bus

Even in a small server, it prevents “giant handler functions” and makes it easy to add:
- new notifications
- new checks
- new policy rules
- new integrations

### 4.2 Event types

Minimum set:

- `TicketOpened`
- `TicketClosed`
- `TicketReopened`
- `DiffIngested`
- `IndexUpdated`
- `ClaimsUpdated`
- `ViolationsComputed`
- `WarningsComputed`
- `GitHubIssueEventReceived`
- `DiscordNotified`

Events come from:
- Repo CLI (explicit commands)
- GitHub webhooks
- Server internal jobs (reindex, recompute)

### 4.3 Processing model

For simplicity:
- single buffered channel
- single goroutine that dispatches to handlers synchronously
- each handler may enqueue further events

This keeps ordering deterministic and avoids complicated concurrency issues.

---

## 5) Diff ingestion strategy (stateless CLI)

### 5.1 Inputs from repo CLI

Repo CLI sends one of:

1) **Unified patch** for working tree:
- `git diff` output

2) Unified patch for staged changes:
- `git diff --staged`

3) File snapshots for specific files:
- `{path, content}` pairs (useful if diff generation is hard)

Recommended primary path: **unified diff**.

### 5.2 Server parses diff → file hunks → line maps

From diff, compute:
- updated files list
- added/removed line ranges
- approximate “touched” sections/definitions (via line overlap with index)

Then:
- update claims for the active ticket
- update warnings and violations

---

## 6) Semantic index: sections and definitions

### 6.1 Parsing approach

For each supported language extension:
- Parse **sections** (region markers, markdown headings, JSON paths, etc.)
- Parse **definitions** (regex-based or language-specific heuristics)

Store results in:
- `scopes` table (file/section/definition)
- include `start_line`, `end_line`

### 6.2 Index update triggers

- On `DiffIngested`:
  - only reindex files that changed
- On explicit CLI command:
  - `POST /repo/reindex` to rebuild everything

### 6.3 In-memory caches

Cache latest index for fast overlap checks:
- `map[filePath][]SectionScope`
- `map[filePath][]DefinitionScope`

Refresh caches after DB writes.

---

## 7) Ticket model & collaboration

### 7.1 Active ticket selection

Because the repo CLI is stateless, it must include **ticket id** in each command/event.

Two patterns:

- CLI always passes `--ticket <id>`
- or CLI passes `--ticket auto` and includes “current branch” or “ticket file”, but server still receives an explicit `ticket_id` resolved by CLI.

Recommended: **always pass explicit ticket id**.

### 7.2 Claiming rules

When diff touches a scope:
- Create or update `ticket_claims(ticket_id, scope_id)` with `claim_type=touched`.

Optional improvements:
- “Declared claims”: a ticket plan file can list intended scopes; CLI can send them when opening the ticket.

### 7.3 Conflict detection

A conflict is when:
- same **definition scope** is claimed by multiple open tickets
- same **section scope** is edited by multiple open tickets (optional)
- same file is heavily modified by multiple tickets (optional threshold)

The server generates warnings:
- `conflict:definition`
- includes:
  - definition id
  - tickets involved
  - last activity timestamps

---

## 8) Warning & violation engine

### 8.1 Goals

- Provide early, actionable feedback:
  - “You are modifying a definition claimed by ticket X”
  - “Section moved; ticket Y references old path”
  - “Inline comment violates policy”
- Keep noise low.

### 8.2 Severity levels

- `info`: FYI
- `warn`: likely trouble soon
- `error`: block commit / close

### 8.3 “Precommit check” endpoint

Repo CLI calls this right before commit:
- server evaluates:
  - unresolved `error` severity warnings
  - high priority policy violations
- response indicates:
  - OK / block
  - message list
  - optional autofix patches

---

## 9) HTTP API design

Use JSON over HTTP. Keep API small.

### 9.1 Auth

For a handful of devs:
- either no auth on LAN / localhost
- or a shared bearer token

Implement:
- `Authorization: Bearer <token>`

### 9.2 Endpoints (commands/events)

#### Tickets
- `POST /tickets/open`
  - body: `{ticket_id, title, prompt, llm, ui, author, github_issue?}`
- `POST /tickets/close`
  - body: `{ticket_id, summary, files?}`
- `POST /tickets/reopen`
  - body: `{ticket_id, prompt, llm, title?}`

#### Diff ingestion
- `POST /diff/ingest`
  - body: `{ticket_id, repo_id, base_ref?, head_ref?, patch, context?}`
  - response: warnings + violations + updated claims summary

#### Precommit
- `POST /checks/precommit`
  - body: `{ticket_id, patch, staged: bool, branch?, commit_message?}`
  - response: `{ok, warnings, violations, suggested_fixes?}`

#### Indexing
- `POST /repo/reindex` (admin/dev only)
- `POST /repo/index-file` (reindex one file from snapshot)
  - body: `{file_path, content}`

### 9.3 Query endpoints (read-only)

- `GET /tickets?status=open`
- `GET /tickets/{id}`
- `GET /tickets/{id}/claims`
- `GET /warnings?ticket_id=...`
- `GET /violations?ticket_id=...`
- `GET /scopes?file=...`

> If you already prefer GraphQL for queries, you can expose `/graphql` for reads, but keep commands as explicit HTTP POSTs.

---

## 10) GitHub integration

### 10.1 Webhook receiver

- `POST /webhooks/github`
  - verify signature (recommended)
  - accept events:
    - `issues` (opened, closed, reopened)
    - `issue_comment` (created)
    - optionally `pull_request`

Server stores event payload in `events` table and emits internal events.

### 10.2 Close/reopen with comment correlation

When GitHub triggers “close with comment”:
- you will usually receive:
  - `issue_comment.created` (contains comment body)
  - `issues.closed` (no comment body)

Server correlation logic:
- maintain a short-lived in-memory map:
  - key: `repo + issue_number + actor`
  - value: comment body + timestamp
- when `issues.closed/reopened` arrives:
  - attach cached comment if within window (e.g. 90s)
  - else optionally fetch latest comment via GitHub API

Then emit internal:
- `TicketClosed` or `TicketReopened` enriched with comment/prompt

---

## 11) Discord integration

### 11.1 Outbound webhooks

Server sends messages to Discord webhook URL.

You can model outbound notifications as handlers subscribed to internal events:
- `TicketOpened` → post “Issue opened”
- `TicketClosed` → post “Closed + Summary”
- `TicketReopened` → post “Reopened + Prompt”
- `ConflictDetected` → post warning (optional)

### 11.2 Formatting conventions

Match your desired headings:
- `# 🤖 Prompt` on reopen
- `# 🧠 Summary` on close summaries

Your Discord payload should be an embed or a message with markdown.

---

## 12) Operational setup

### 12.1 Deployment model

For a small team:
- Run on a single dev box or small VM in the office/VPN
- Or run locally on each dev machine, but then state is per machine (not ideal)

Recommended:
- **One shared server** for the team.
- Repo CLI points to it via config env var:
  - `SEMIO_SERVER_URL=http://devserver:8787`
  - `SEMIO_SERVER_TOKEN=...`

### 12.2 Backups

SQLite file backups:
- nightly copy
- or git-ignore it but archive periodically

### 12.3 Observability

Minimum:
- structured logs (JSON)
- log request id + event id
- `/healthz` endpoint
- `/debug/state` (admin only) for quick introspection:
  - open tickets
  - active conflicts
  - last 100 events

---

## 13) Implementation plan (phased)

### Phase 0 — skeleton (1–2 days)
- `main.go` HTTP server
- config + token auth
- SQLite open + create tables
- minimal endpoints:
  - `/healthz`
  - `/tickets/open`
  - `/diff/ingest` (store patch in events table only)

### Phase 1 — diff → touched files (2–4 days)
- parse unified diff:
  - list files changed
  - compute added/removed line ranges
- store `DiffIngested` event
- return “files touched” response

### Phase 2 — semantic indexing (3–7 days)
- implement per-language parsing:
  - sections
  - definitions
- store scopes
- reindex on diff ingestion for touched files

### Phase 3 — claims + conflicts (3–7 days)
- on diff ingestion:
  - map hunks to scopes (line overlap)
  - update `ticket_claims`
- detect multi-ticket conflicts
- create warnings

### Phase 4 — policy violations (optional, 1–2 weeks)
- run policy scanners on touched files
- store violations and compute metrics
- integrate with precommit checks

### Phase 5 — integrations (2–5 days)
- GitHub webhook receiver + correlation
- Discord notifier
- add admin endpoints

---

## 14) Exact responsibilities per endpoint (contract)

### `/diff/ingest` contract

Input:
- `ticket_id` (required)
- `patch` (required)
- optional context: `{branch, author, ui, llm}`

Server actions:
1. store event row
2. parse patch to changed files + hunks
3. reindex changed files (from filesystem or from snapshots if provided)
4. map hunk lines → section/definition scopes
5. update claims
6. recompute conflicts/warnings for affected scopes
7. return summary:
   - claimed scopes added/updated
   - conflicts
   - policy violations (if enabled)

Output:
```json
{
  "changed_files": ["..."],
  "claimed_scopes": ["def:...", "section:..."],
  "warnings": [...],
  "violations": [...],
  "blockers": [...]
}
```

### `/checks/precommit` contract

Input:
- `ticket_id`, `patch`, `staged`
- optional `commit_message`

Server actions:
- run same pipeline as ingest (or reuse last ingest if identical patch hash)
- decide blockers based on:
  - open conflicts at error severity
  - high priority violations
  - required documentation artifacts

Output:
```json
{
  "ok": false,
  "blockers": [...],
  "warnings": [...],
  "autofix_patch": "..."
}
```

---

## 15) Single-file Go implementation tips

To keep `main.go` maintainable:

- Use **small structs + pure functions**
- Use “region” comments to separate modules
- Prefer `database/sql` with prepared statements
- Avoid fancy router frameworks; `http.ServeMux` is enough
- Use minimal dependencies:
  - `modernc.org/sqlite` driver (already)
  - optionally `chi` router if desired, but not required

### Suggested internal section order

1. `package main`
2. imports
3. constants + config structs
4. models (Ticket, Scope, Warning, Violation)
5. DB init + migrations
6. event bus types + dispatcher
7. diff parsing helpers
8. indexing helpers
9. claim/conflict logic
10. policy logic (optional)
11. HTTP handlers
12. webhook handlers
13. outbound webhook sender
14. main()

---

## 16) Repo CLI command suite (stateless)

Examples the repo binary should offer:

- `repo ticket open --id ... --title ... --prompt ... --llm ...`
  - calls `/tickets/open`

- `repo diff ingest --ticket ... [--staged]`
  - collects `git diff` output
  - calls `/diff/ingest`

- `repo check precommit --ticket ... [--staged]`
  - collects patch
  - calls `/checks/precommit`
  - exits non-zero if `ok=false`

- `repo ticket close --ticket ... --summary ...`
  - calls `/tickets/close`

- `repo ticket reopen --ticket ... --prompt ...`
  - calls `/tickets/reopen`

---

## 17) Minimal security posture

- bind to `127.0.0.1` if single-machine
- bind to `0.0.0.0` behind VPN if shared
- require bearer token for all endpoints except `/healthz`
- verify GitHub signatures for `/webhooks/github`

---

## 18) Acceptance criteria (what “done” looks like)

1. Dev runs:
   - `repo ticket open ...`
   - `repo diff ingest --ticket ...`
2. Server:
   - persists ticket
   - parses diff
   - updates claims
   - detects conflicts
   - returns warnings
3. Dev runs:
   - `repo check precommit --ticket ...`
4. Server:
   - blocks commit when conflicts/violations exist
5. GitHub:
   - close/reopen events + comments arrive
6. Discord:
   - receives messages with **Prompt** on reopen and **Summary** on close

---

## 19) Follow-ups / optional enhancements

- Patch hashing & caching: avoid recomputation if same patch sent repeatedly.
- Per-branch state overlays (if you want ticket state per branch).
- “Ownership decay”: claims expire if not touched for N days.
- UI dashboard: simple HTML page listing open tickets and conflicts.
- WebSocket stream for real-time updates (not required with stateless CLI).

---

### Appendix A: Example JSON payloads

#### Diff ingest

```json
{
  "ticket_id": "@semio/tickets/2026/01/20/repo-github-headings",
  "repo_id": "@semio",
  "patch": "diff --git a/README.md b/README.md\n..."
}
```

#### Ticket close

```json
{
  "ticket_id": "@semio/tickets/2026/01/20/repo-github-headings",
  "summary": "Added GitHub integration headings for prompt + summary. Updated README and AGENTS.",
  "files": ["README.md", "AGENTS.md"]
}
```

---

If you want, I can also generate a **starter `go/server/main.go` skeleton** consistent with this plan (single file), including: config, SQLite schema creation, event bus, and stub handlers.
