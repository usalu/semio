---
name: Architect Technology
overview: "Create the `architect` technology: headless `architect_spine` covering all 65 architectural-programming feature areas end-to-end, plus a full s/OS `DocumentApp` at `architect/plugin` with triangular undirected adjacency-matrix editing as the signature surface."
todos:
  - id: goal-ticket
    content: Open goal architect; ticket_open with plan_id; feature checklist in ticket folder
    status: completed
  - id: scaffold
    content: Scaffold architect/plugin + architect/plugin, Cargo/nx/launch/registry/AGENTS.md
    status: completed
  - id: kernel-document
    content: Implement kernel types + Program root with all 65-area registers and serde
    status: completed
  - id: adjacency-operations
    content: Undirected adjacency model, conflicts, ProgramOperation VCS, validate/analyze/report/search/trace/exchange
    status: completed
  - id: tests-plugin
    content: "Co-located tests: round-trip, operations/undo, adjacency, analysis, exchange"
    status: completed
  - id: plugin-app
    content: "DocumentApp: adjacency triangle list, graph, registers, reports, panels, full actions"
    status: completed
  - id: verify-close
    content: Run architect_spine tests + plugin registry/build; ticket_close
    status: completed
isProject: false
---

# Architect Technology — Program + Plugin

## Decisions (locked)

- **Paths:** [`architect/plugin`](architect/plugin) (headless) + [`architect/plugin`](architect/plugin) (standard program path; not `architect/constrain`).
- **Depth:** Option 2A — full domain for all 65 feature areas + complete program (CRUD, analysis, exchange, reports, adjacency UI). No stubs, no empty registers.
- **Goal:** Approving this plan authorizes opening goal `architect` (title “Architect”, due `2026-12-31`). Ticket binds to `🎯️architect`.
- **No mixing:** Do not depend on `coda`, `compose`, `puzzle`, or `mit-bestand`. Adjacency concepts may be reimplemented cleanly; prior art in [`coda/client/lib/programming`](coda/client/lib/programming/go/main.go) is reference only.
- **Undirected adjacency:** Store only canonical pairs `(a, b)` with `a < b`; UI edits the lower triangle; graph view is undirected. Depend on [`mathematical_graph`](mathematical/graph/rs) for topology helpers (`normalize_undirected`, `Adjacency`).

## Architecture

```mermaid
flowchart TB
  plugin["architect_spine\nDocumentApp WASM"]
  program["architect_spine\nProgram + Operations + Analysis"]
  vcs["vcs"]
  graph["mathematical_graph"]
  sdk["semio-framework-plugin"]

  program --> program
  program --> sdk
  program --> vcs
  program --> vcs
  program --> graph
```

**Core information chain (document spine):**

`Project → Stakeholders → Users → Activities → Functions → Elements → Requirements → Relationships/Adjacencies → Constraints → Criteria → Decisions → Validation`

All other feature areas hang off this spine as typed registers, cross-links, and analysis/report APIs.

## Crate layout

```
architect/
  AGENTS.md                          # created once with technology (agents must not edit later)
  plugin/
    rs/Cargo.toml                    # architect_spine
    rs/lib.rs                        # module router
    rs/src/*.rs                      # domain modules (regions)
    script.ts + project.json         # @semio-tech/architect-spine → test
  plugin/
    rs/Cargo.toml                    # architect-spine, semio:architect, playground ports
    rs/lib.rs                        # DocumentApp + semio_plugin!
```

**Cargo names:** `architect_spine`, `architect-spine`  
**Nx:** `@semio-tech/architect-spine`  
**Schema:** `architect.program`  
**Playground:** variant `architect`, ports react `6090` / wgpu `6190` (unused).

## Headless domain — `architect_spine`

### Shared kernel ([`architect/spine/rs/src/kernel.rs`](architect/spine/rs/src/kernel.rs))

- `EntityId(String)` with serial helpers (`stakeholder-1`, `element-1`, …)
- `Priority` (mandatory / essential / preferred / optional / deferred / prohibited)
- `LifecycleStatus` (draft → proposed → under-review → validated → approved → rejected → deferred → superseded → archived + open/closed/at-risk/blocked/in-progress/complete)
- `Ownership` (owner, authority, consultants, participants)
- `TraceLink` (from/to entity + link kind for full traceability)
- `QuantitySpec` (min/max/target/current/forecast/peak/average + unit)
- `TextField`, `TaggedNote`, `TimestampMeta` (created/updated/author)
- `PluginError` / `ProgramDiagnostic`

### Root document

```rust
Program {
  schema: "architect.program",
  meta: ProgramMeta,           // identity, purpose, terminology, classification, contexts
  project: ProjectDefinition,  // §1
  stakeholders: Vec<…>,        // §2
  users: Vec<…>,               // §3
  activities: Vec<…>,          // §4
  functions: Vec<…>,           // §5
  elements: Vec<…>,            // §6 program elements
  quantities: Vec<…>,          // §7
  relationships: Vec<…>,       // §8
  adjacencies: Vec<…>,         // §9 undirected edges
  processes: Vec<…>,           // §10
  flows: Vec<…>,               // §11
  access_rules: Vec<…>,        // §12
  operations: Vec<…>,          // §13
  equipment: Vec<…>,           // §14
  resources: Vec<…>,           // §15 furniture/resources
  storage: Vec<…>,             // §16
  environmental: Vec<…>,       // §17
  human_factors: Vec<…>,       // §18
  accessibility: Vec<…>,       // §19
  privacy: Vec<…>,             // §20
  safety: Vec<…>,              // §21
  security: Vec<…>,            // §22
  regulatory: Vec<…>,          // §23
  site_context: Vec<…>,        // §24
  organizational: Vec<…>,      // §25
  services: Vec<…>,            // §26
  infrastructure: Vec<…>,      // §27
  information: Vec<…>,         // §28
  communication: Vec<…>,       // §29
  wayfinding: Vec<…>,          // §30
  schedules: Vec<…>,           // §31
  flexibility: Vec<…>,         // §32
  growth: Vec<…>,              // §33
  sustainability: Vec<…>,      // §34
  resilience: Vec<…>,          // §35
  costs: Vec<…>,               // §36
  delivery: Vec<…>,            // §37 time/delivery
  risks: Vec<…>,               // §38
  conflicts: Vec<…>,           // §39
  requirements: Vec<…>,        // §40
  priorities: Vec<…>,          // §41
  scenarios: Vec<…>,           // §42
  options: Vec<…>,             // §43
  decisions: Vec<…>,           // §44
  validations: Vec<…>,         // §45
  performance: Vec<…>,         // §46
  quality: Vec<…>,             // §47
  documents: Vec<…>,           // §48 registers/docs
  changes: Vec<…>,             // §49 version/change
  collaboration: Vec<…>,       // §50
  workshops: Vec<…>,           // §57
  surveys: Vec<…>,             // §58
  issues: Vec<…>,              // §59
  audit_events: Vec<…>,        // §60
  templates: Vec<…>,           // §62
  knowledge: Vec<…>,           // §63
  benchmarks: Vec<…>,          // §64
  governance: Governance,      // §56
  traces: Vec<TraceLink>,      // §54
}
```

Each entity type carries **every field** from its feature section (typed enums for classifications; `Vec<String>` only for free-text lists like concerns/notes). Serde: `camelCase`, tagged `kind` where unions exist.

### Adjacency model (§9) — undirected graph

```rust
Adjacency {
  id, a: EntityId, b: EntityId,  // always a < b after normalize
  kind: Required | Preferred | Optional | Prohibited,
  connection: Direct | Indirect | Controlled | SharedAccess | None,
  separations: Vec<SeparationKind>, // acoustic, visual, security, …
  weight: f64,
  priority: Priority,
  rationale: Option<String>,
}
```

API: `normalize_pair`, `set_adjacency`, `adjacency_matrix` (dense lower-triangle view), `detect_adjacency_conflicts` (required vs prohibited, incompatible separations), `undirected_edges` for graph render.

### VCS operations

`ProgramOperation` enum covering upsert/remove/reorder for every register + bulk replace + adjacency set/clear + meta update. Implements `vcs::Operation<Program>` with diffs and backwards for undo.

### Analysis / reporting / search / exchange (real, not stubs)

| Module | Covers | Public API |
|--------|--------|------------|
| `validate` | §45 + cross-register integrity | `validate_plugin(&Program) -> Vec<Diagnostic>` |
| `analyze` | §51 | gap, conflict, dependency, capacity, demand, utilization, workflow, risk, cost, scenario, sensitivity, impact, trend |
| `report` | §52 + §65 | `ReportKind` → structured `ProgramReport` (executive summary through recommendation) |
| `search` | §53 | keyword/category/owner/status/priority/source/date filters + saved filters |
| `trace` | §54 | build/query `TraceLink` chains; full audit trail over `audit_events` |
| `exchange` | §61 | JSON import/export (native), CSV/TSV spreadsheet round-trip for registers, duplicate detection, merge |
| `template` | §62 | apply sector/project/requirement templates from `templates` register |
| `status_summary` | §55 | aggregate status counts across registers |

### Tests (co-located in modules)

- Round-trip serde for `Program` with every register populated
- Adjacency normalize + conflict detection
- Operation apply/undo for representative operations per register
- `validate_plugin` catches broken refs and adjacency conflicts
- Analysis and report smoke on a fixture program
- Exchange import/export preserve ids and undirected edges

## Plugin — `architect/plugin`

Follow [`forms/plugin`](forms/plugin/rs) + [`flow/plugin`](flow/plugin/rs) patterns: `DocumentApp<Program, ProgramOperation>` + `semio_plugin!`.

### Manifest

- App id `architect`, document schema `architect.program`
- Standard panels: Document / Catalogue / Inspection
- Window kinds / body keys:
  - **`adjacency`** (primary) — triangular adjacency matrix as **list** with triangle chrome on the side
  - **`graph`** — undirected `NodeGraphScene` over program elements + adjacency edges
  - **`register`** — `BlockList` / table editor for the selected register kind
  - **`report`** — report body from `architect_spine::report`
- Modes: Edit / Review / Report
- Full operation + view_action surface for CRUD, search, filter, import/export, analysis run, validation run, status filters

### Signature UI: triangular adjacency list

Lower-triangle undirected editor (no duplicate upper cells):

```
        ElemA  ElemB  ElemC  ElemD
ElemB   [●️]
ElemC   [○️]    [●️]
ElemD   [×]    [ ]    [●️]
```

Rendered as a **list of pair rows** (`BlockList` or custom `UiNode::Tree` rows) ordered by `(row, col)` with `col < row`, plus a left-side **triangle glyph strip** (CSS/scene chrome) so the matrix identity is obvious. Each cell/row edits `Adjacency.kind` / weight / separations via inspector. Mutations emit `ProgramOperation::SetAdjacency` / `ClearAdjacency` with normalized endpoints.

Graph window: elements as nodes; edges from `adjacencies`; no direction arrows; layout via existing undirected board helpers if needed (`infinite_board_normal_undirected`) without leaking puzzle fixtures.

### Document / catalogue / inspection

- Document tree: all registers + counts + status badges
- Catalogue: templates + knowledge library insert
- Inspection: selected entity fields (all typed fields from the feature list), multi-select mixed values via existing inspector helpers

### Runtime (non-document)

Selection, active register, search query, saved filters, last report JSON, adjacency filter (kind), camera for graph — view actions only.

## Workspace / nx / launch wiring

1. Add `architect/spine/rs` and `architect/spine/rs` to [`Cargo.toml`](Cargo.toml) members.
2. [`architect/plugin/script.ts`](architect/plugin/script.ts) + [`project.json`](architect/plugin/project.json) — `runCargoTestBudgeted(["architect_spine"], …)`.
3. Plugin `Cargo.toml`: `[package.metadata.component] package = "semio:architect"`, playground `6090`/`6190`.
4. Regenerate plugin registry: `bun nx run @semio-tech/plugin-registry:generate`.
5. Register launch configs in [`.vscode/launch.json`](.vscode/launch.json) (existing order/grouping):
   - `🧪️test🏛️architect-spine`
   - `🛠️dev🏛️architect⚛️react`
   - `🛠️dev🏛️architect🧊️wgpu🌐️wasm`
   - `🛠️dev🏛️architect🧊️wgpu🖥️native`
6. Create [`architect/AGENTS.md`](architect/AGENTS.md) once with technology frontmatter (implementation agents must not edit afterward).

## Ticket workflow (on execute)

1. Open goal `architect` (authorized by plan approval).
2. `ticket_open` with goal `architect`, plan_id from this plan, emoji `🏛️`, title “Architect Program And Plugin”.
3. Implement plugin crate → program → registry → launch → tests.
4. Confirm: `cargo test -p architect_spine`, program builds via OS dev / component target, adjacency UI path exercised with `[DEBUG]` logs if needed.
5. `ticket_close` with summary + all touched files.

## Out of scope

- Geometry, plans, 3D, CAD, energy, norms
- Coda ACC validators / compose translators
- Multiple plugins or a separately named `constrain` crate
