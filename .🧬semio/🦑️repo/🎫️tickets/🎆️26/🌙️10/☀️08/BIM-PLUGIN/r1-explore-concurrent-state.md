# 📓️ R1 Explore Concurrent State For The BIM Plugin

Read-only exploration, 2026-10-08, checkout `C:\git\semio`. No repo file was modified. Repo MCP and semio MCP both
reported CONNECTION_CLOSED in this session, so `repo://goals` was not read; everything below comes from files and `git`
(read-only `status`, `ls-files`, `rg`). Heuristic counts come from grep, not from the policy gate, unless stated.

## 1. Git status (91 entries, grouped)

| Area | Modified entries | Notes |
|---|---|---|
| `🧰️framework/` | 48 | 20 under `🛍️products/` (not enumerated), 14 under `🔨️modules/` (TUI `🖱️ui/⌨️tui/*` 11 files, `📦️packages` Cargo.toml, `🧱️elements`, `🎯️targets`) |
| `.🧬semio/` | 24 | ticket bookkeeping and untracked ticket files (DASHBOARD r2 audits and scripts are untracked `??`) |
| `✏️s/` | 7 | `🔌️plugins/` 5 (`🧩️puzzle`, `🔋️energy`, `🗿️artifacts/🔋️model` and its subsets, `🌀️procedural`), `🧑‍💻dev/` 2 (`💬️agent-reply`, `🤖️live-agent-loop`) |
| `🎓️teaching/` | 6 | architecture quiz/pets READMEs and stack |
| `🌎️hub/` | 3 | README, `🚀️local-bootstrap`, `🤝️integration-harness` |
| root | 4 | `📜️script.ts`, `README.md`, `Cargo.lock`, `.devcontainer/README.md` |

Top-level plugin areas touched right now: framework (mutation/replication, TUI, products/os), `🧩️puzzle`, `🔋️energy`,
`🌀️procedural`, `🎓️teaching`, `🌎️hub`. The framework and teaching changes are the concurrent infrastructure work.

## 2. DIFF-ONLY-MUTATIONS ticket (`26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`)

Goal `🎯runningframework`, status open. Ticket bookkeeping manual (repo MCP `ticket_open` returned a malformed result).

### 2.1 Laws (design.md)

- L1 declarative concrete diff: `m.diff(base)` builds a sparse typed diff from payload and reads of `base`. No
  `D::between(base, mutated)`, no generic JSON patch, no whole-snapshot diff.
- L2 concrete inverse: `m.inverse(base)` is built in the leaf, never derived from the forward diff.
- L3 inverse diffs sum to the negative: `Σ.apply(after) == base` and `Σ == m.diff(base).diff().inverse(base)`
  (`MutationDiff::absorb` must coalesce same-key entries).
- L4 central apply only: no leaf calls `MutationDiff::apply`, takes `&mut P`, or clones base to write into it.
- L5 impossible by design: `MutationDiff::apply` takes an `ApplyCapability` that only `apply_diff` mints.

Violation codes: V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE, V2-EMPTY-INVERSE,
V3-LEAF-APPLY, V3-HAND-MUTATION, V4-LAW-UNTESTED, V5-ABSORB.
Rulings: `fold_plan_diff`/`fold_plan_inverse` stay for composites; generic seams (`diff_from_model`,
`graph_edit_diff`, `*_selection_inverse`, `Restore`/`SetSnapshot` inverses, `apply_in_place`,
`MutationOutcome::apply_to(&mut P)`) are deleted, not wrapped. Build rule for executors: every cargo call wrapped in
`🚦️gate.sh`, foreground only.

### 2.2 Framework API: design vs this checkout

File: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` (1867 lines).

| API (design) | In this checkout | Status |
|---|---|---|
| `ApplyCapability { _sealed: () }` | line 97, private field, documented as minted only by `apply_diff` | landed |
| `apply_diff<P, D>(diff, base) -> MutationApplyResult<P>` | line 104, mints the capability and calls `diff.apply` | landed |
| `MutationDiff::apply(&self, base, capability)` | line 120 | landed |
| `MutationDiff: Clone + Default + PartialEq + ToValue + FromValue + DiffAlgebra<P>` | line 117 | landed |
| `MutationDiff::absorb(&mut self, other)` | line 135, normative contract doc (structural, total, base-free, sequential coalesce) | landed (soundness per type not verified) |
| `DiffAlgebra<P>`: `inverse`, `between`, `is_empty` | line 171 | landed; `between` still exists (design: sync/import only, gate-forbidden in leaves) |
| `MutationOutcome::apply_to` deleted | no `fn apply_to` in the mutation file; `impl MutationOutcome` has no apply_to | deleted in framework core |
| Composite forwards the capability | not checked per composite | unverified |
| `assert_mutation_inverse_sum_law` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs:641`, re-exported as `os_spr::protocol_laws` (package lib line 189) | landed, but `pub async fn`, while the design fixed contract is a sync fn. Executors call it with `.await`; tests must match |
| `assert_mutation_inverse_law` (pre-existing) | line 572 (same file), plus `_cold` variant line 605 | existing |
| `assert_diff_algebra_between_law` | same file, after the sum law | added |
| `store::apply_operation`, `store::apply_outcome` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` lines 25219, 25232 | added per fw-spine report (replaces `apply_to` semantics) |
| `os_spr::{ApplyCapability, apply_diff}` facade | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` line 143 onward | per fw-spine report |

Counts (heuristic, whole repo): `ApplyCapability` or `apply_diff(` in 644 Rust files; 239 `impl MutationDiff<`; 361
`DiffAlgebra<` impl or mention lines (156 `impl … DiffAlgebra<` lines); 214 `retire_cold` uses.

### 2.3 Executors, coordination, gate

- `📓️coordination.md`: wave 1 audits (Haiku) done, 14 audit reports. Baseline: 2,887 hand `MutationKind` impls in 96
  artifacts, about 1,700 with a V1–V3 code, 0 leaves with an L3 test, 57 whole-state `MutationDiff` impls, 91 hand
  `impl Mutation<` (the later count of `impl.*Mutation<` in plugins is 94).
- Wave 2 (Sonnet) labels: fw-spine, fw-gate, fw-os-leaves, energy, architect, norm-a/b/c, stdio-semio/pdf/gltf/mid/small,
  puzzle-trinity, block-wfc, draw-cad-gis-raster, shooting-remodel-note, fem-procedural-layout, small-plugins.
- Only two executor reports exist in the ticket folder: `📓️exec-fw-spine.md` (SPINE-KERNEL-GREEN 02:07, kernel lib
  compiles, law helper landed) and `📓️exec-fw-gate.md`. The other 12 `📓️exec-*.md` files named in coordination do
  not exist yet, so the plugin work is in flight or was not written down.
- `fw-gate`: rules R8–R16 added to `verify mutation-outcome-law` in root `📜️script.ts`
  (`//#region 🔧️PolicyRuleMutationOutcomeMergePolicy`), fixture `🧫️fixtures/🧫️diff-only-law-gate/🔣️.json`, tests
  `🧪️tests/🧪️diff-only-law-gate/🟦️.ts` (25 pass in an isolated cwd). The gate run at that time exited 1 with 4,152
  breaches (4,140 diff-only). Largest: R15 untested leaves 1,905, R12 base clone 841, R11 derived inverse 475, R14 restore
  inverse 309. Open issue: `bun test` with cwd = repo root segfaults in this environment, so `test outcome-law-gate` was
  not verified through the script route. The gate was not re-run for this report (read-only).

### 2.4 Which plugins are converted (heuristic, not gate-verified)

Files under `✏️s/🔌️plugins/` mentioning `assert_mutation_inverse_sum_law` (a leaf-level L3 test):
`🏛️architect` 263, `🗄️stdio` 84, `🎥️shooting` 32, `🖍️draw` 25, `📜️imperative` 4, `🕸️dag` 3, `💡️reasoning` 2,
`🎪️demonstrator` 2, `📕️norm` 1. The other 25 plugin directories have none.

Leaf files under `🔺️diff` or `↩️inverse` still containing `.apply(` (V3-LEAF-APPLY candidates; gate R9 also flags
domain `.apply(` names): `🗄️stdio` 65, `🀄️wfc` 5, `🌍️gis` 4, `🔋️energy` 3, `📕️norm` 3, `✒️writer` 3, `🖨️raster` 2,
`📐️cad` 2, `🌀️procedural` 2, and 1 each in sourcing, note, trinity, remodel, layout, forms, process, animate, vcs,
flow, mathematical.

Remaining hand `impl … Mutation<` lines: `🧩️puzzle` 13, `🗄️stdio` 8, `🌀️procedural` 8, `🀄️wfc` 8, `🧱️block` 6,
`🏛️architect` 5, then 1–4 each elsewhere.

Plugin with no `🔌️plugins` entry yet: `bim` (no `✏️s/🔌️plugins/🏢️bim` or similar directory exists).

Conclusion: no plugin is fully converted in a verifiable sense. `🏛️architect` is furthest along by test count, but it still
has 5 hand `Mutation<` impls and no `exec-architect.md`. The `architect` goal matches the BIM ticket goal.

### 2.5 Implication for a new bim plugin

It should be born compliant: leaf `🔺️diff`/`↩️inverse` with no `.apply(`/`apply_to`/`between(`, a concrete inverse per
leaf, derived `Mutations` (not hand `impl Mutation<`), its own `🧪️tests` calling `assert_mutation_inverse_sum_law`
(R15 counts only the leaf's own tests, not the shared `🧬️mutations/🧪️tests`), and no `&mut` snapshot in leaves.

## 3. Dashboard launch cockpit and command registration

### 3.1 r2-plan and fleet-plan (ticket `26/09/23/DASHBOARD-LAUNCH-COCKPIT`, open, goal `🎯runningframework🎯runningproducts🎯runningrepo`)

- `r2-plan.md` (untracked, in progress): round 2 slices T-A/T-B/T-C/T-D (TUI), A-1 registry+CLI, A-2 daemon, A-3 view,
  L-1 launch removal, L-2 docs, C-1a/C-1b agent and editor canon, V-1 battle tests. Cross-slice rules: `TaskLabel` and
  `Ready` move to `🌀️daemon/✉️ipc`; `registry::Launch` is the only start input; root `package.json` keeps only `nx`,
  `setup`, `dashboard`, `dashboard:install`; `.vscode/settings.json` and `extensions.json` are canonical; agents and editors
  start servers only via `semio run <id> [--param k=v] --detach --wait-ready`. Note 7: `AGENTS.md` lines 50–51 contradict
  the goal; agents may not edit it (owner action).
- `fleet-plan.md` §1 target: `semio` (native Rust) is the single control plane. No file named `launch.json` or
  `launch.seed.jsonc` exists; the generator and all dependents are removed or re-expressed against the registry.
- `launch-dependents.md` (audit, 2026-10-07): the two `.vscode` launch files are a second output of the plugin-registry
  generator (`@semio-tech/plugin-registry:generate`, on the root `prepare` and dev-serve path). 54 registry and gate tests
  assert that a launch row exists. 209 files outside tickets depend on them.

### 3.2 Current checkout (what is still true)

- Tracked and still present: `.vscode/launch.json` (about 4,671 `"name"` entries), `.vscode/🧩️launch.seed.jsonc`,
  `.claude/launch.json` (79 names). So launch.json is NOT yet removed; L-1 has not landed.
- Root `package.json` scripts are already bootstrap-only: `setup`, `dashboard`, `dashboard:install`.
- The dashboard crate is at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/`. Registry:
  `🎮️registry/🦀️.rs`, schema `🧬️schema/🎮️registry/🔣️.json`, daemon schema `🧬️schema/🌀️daemon/🔣️.json`.
  The registry already decodes `targets.<name>.metadata.semio.dashboard` (line 959) and project-level
  `metadata.semio.dashboard` (line 972). Its crate README still mentions launch.json.
- 45 tracked `📋️project.json` files carry `metadata.semio.workspaceCommand` (verb + name per target). Their role in
  the registry was not verified.
- Plugin project pattern (example `🏛️architect/.../🦀️rust/📋️project.json`): targets are `nx:run-commands` calling
  `bun ./📜️script.ts <command>` with `metadata.semio.workspaceCommand: [verb, name]`.

### 3.3 Rule for registering a new plugin's commands

- Today (AGENTS.md, still in force on disk): every executable command is registered in `.vscode/launch.json`, by the
  existing order, grouping and naming. In practice that means a row in `🧩️launch.seed.jsonc` plus generation. Devs use
  launch.json, not the CLI.
- Target (binding in r2-plan and fleet-plan, not implemented): the plugin's owner `📋️project.json` gets
  `metadata.semio.dashboard` (target-level `verb`, `ready`, `requires`, `parameters`; project-level `tools`, `compounds`,
  `groups`). Nx targets and workspace scripts are discovered automatically. Command ids: `<project>:<target>[:configuration]`.
  Launch with `semio run <id> --detach --wait-ready`. No launch row, no seed row.
- Conflict to decide by the owner: AGENTS.md versus the cockpit goal. Memory note `dashboard-sole-control-plane.md`
  records the owner decisions: dashboard is the only control plane, root package.json is bootstrap only, agents start
  servers via `semio run`, `.vscode/settings.json` is canonical, MCP client files derive from root
  `metadata.semio.dashboard.tools`. It also flags the Windows ConPTY `Send` trap (capture the `Shared` wrapper via `raw()`).

## 4. Tickets 2026-09-21 to 2026-10-08 (matching titles)

Source: `🎫️ticket.json` in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️21..30/` and `🌙️10/☀️01..08/`. There is no 09/20
folder. Some folders have no ticket.json (GIS-2D-LOD-MODE-AUTOMATIC-SUFFIX, GIS-2D-VECTOR-TILE-LABEL-ANCHOR,
MOBILE-SHORT-TITLES, RESOLVE-MERGE-CONFLICTS).

Title matches (plugin, artifact, mutation, bim, 3d):

| Ticket | Status | One line |
|---|---|---|
| 26/09/22 BOUNDED-STORE-DISPOSERS-ON-REACHABLE-PLUGIN-APPS | open | Declare bounded store disposers only on ArtifactEditor/Viewer surfaces a live runtime mounts. Goal `🎯runningframework...`. |
| 26/09/26 NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS | open | Make every norm artifact a non-stubbed compliance assessment (complies, does not comply, how it could). |
| 26/09/26 COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE | open | All stdio artifact editors complete, every common format and detail editable. Goal `🎯runningstdio`. |
| 26/09/26 PUBLISH-PLUGINS-ON-THE-MODULES-ASSET-PAGE | open | Publish all plugin and extension modules on a fourth CDN page (modules host), each page under 1 GB. |
| 26/09/28 FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR | open | Replace the ad hoc PDF editor: text, images, vectors, pages, properties editable via real mutations. |
| 26/09/29 FIRST-CLASS-ARTIFACT-EXAMPLES | open | Every artifact registers examples; editors get a working examples dropdown. Goal `🎯updatedexamples`. |
| 26/09/30 UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O | open | Every artifact snapshot exportable and importable as SQLite over I/O, schema-first. Goal `🎯runningframework`. |
| 26/09/23 PUZZLE-5D-BACKGROUND-CLICK-CLEARS-3D-SELECTION | open | Clicking background must clear the 3D selection in puzzle 5d, as puzzle 3d does. |
| 26/10/08 BIM-PLUGIN | open | New `bim` plugin: model > site > building > storey > walls, columns, slabs, roofs; windows and doors in walls; parametric; mutations use declarative diffs and concrete inverses; no separate bim state module. Goal `🎯architect`. Repo MCP down (CONNECTION_CLOSED). |
| 26/10/08 DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES | open | Mutation laws L1–L5 across all artifacts and plugins (see section 2). Goal `🎯runningframework`. |

Adjacent (title does not match, but body covers plugin geometry, 2D/3D or procedural):

| Ticket | Status | One line |
|---|---|---|
| 26/09/22 WFC-END-TO-END | open | Wave function collapse end to end on bitmap, grid2d, wfc2d, grid3d, wfc3d; interactive fill. |
| 26/09/23 FLOW-AND-PROCEDURAL-FEATURE-COMPLETE | open | Flow synapses and widgets; procedural 2D catalogue with drag-and-drop live preview. |
| 26/09/23 LAYOUT-FEATURE-COMPLETE | open | Layout: frame hover, native placement and linked 2D artifacts, blueprint transform gumball. |
| 26/09/23 PUZZLE-5D-PART-DRAG-PREVIEW-IN-BOTH-WINDOWS | open | Part drag preview must show in 2D as well as 3D. |
| 26/09/23 PUZZLE-5D-CONTEXT-MENU-SUGGESTIONS-SELECTION-SYNC-AND-TRANSFORM-GUMBALL | closed | 5d vortex suggest menu, selection sync, gumball in 5d. |
| 26/09/23 PUZZLE-5D-2D-WINDOW-LEFT | closed | Default layout: 2D board left, 3D world right. |
| 26/09/29 PUZZLE-2D-HANDLE-SIZE-AND-EDGE-ANCHOR, PUZZLE-2D-NODE-ICON-FOLLOWS-DRAG | closed | 2D puzzle handle radius and icon drag fixes. |
| 26/09/23 DASHBOARD-LAUNCH-COCKPIT | open | Dashboard as the only control plane (see section 3). |

Not in range but relevant: `26/10/02 QUIZ-*` and `26/10/05 LAYERED-OVERVIEW-*` are unrelated.

## 5. Prior "bim" concepts in the repo

`rg -il "\bbim\b"` found 112 files outside `.🧬semio` and 522 lines overall under tickets. Relevant prior concepts:

1. `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/` (flow extension `bim`, Rust, Oct 4 to 8). Schemas: material, space, wall,
   slab, column, window, story, building; operators `bim.assemble.building`, `bim.assemble.story`, `bim.element.*`,
   `bim.measure.floorArea`, `bim.measure.grossVolume`. It runs on the neural engine (`Dictionary`, `Operator`). This is the
   closest existing "bim module", and it derives measures. The new BIM ticket says no separate bim module for holding or
   deriving state, so the owner must decide whether it is retired or folded in. Its `📦️packages/🦀️rust` has a
   `📋️project.json`.
2. `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim/` (stdio-bim, Oct 2): a typed runtime assembly of IFC 2x3 / IFC 4 /
   COBie / SAV editor and viewer apps (`StdioBimApps`). It is a composition for hub, not a domain plugin.
3. `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts`: BIM STEP import profile with typologies
   `building.building.slab|beam|column|wall`, model id `aec.building`. The cad examples hold a model definition
   `aec.building.structure` with a `from_building` transformation.
4. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/` (IFC artifact with composition, oracles, packages) and
   `💬️bcf/` (BIM Collaboration Format, standard 2.1, mutate-bcf feature).
5. `📕️norm/🗿️artifacts/📇️iso16757/` (ISO 16757, data structures for building product catalogues) and its
   `💡️inferences/⚖️checks` directory. The norm plugin has the only existing inference precedent with checks; the BIM ticket
   also wants "derived information from inferences".
6. Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` has an executor named "W2-W-bim" (`📓️w2-w-bim-report.md`). It is
   the IFC/STEP wire-witness conversion (ifc 2x3/4, step ap214), not a bim plugin. The name is a collision only.
7. Assets and text: `♻️mit-bestand` (an IFC/STEP concrete asset, a web page "zukunft-bau-entwerfen-mit-bestand", report
   appendices). `🧰️framework/.../📚️library/🔣️taxonomy.json`, `🖼️assets/📃️list/🔖️tags.json` and `mimes.csv` mention bim.

Nothing is named `bim` as a `✏️s/🔌️plugins/` plugin yet. The BIM ticket's goal `🎯architect` and the architect
plugin's program artifacts (`🏛️program`) are the nearest home.

## 6. Not verified

- The gate was not re-run; section 2.4 counts are grep heuristics.
- `bun test` in the repo root segfaults (per fw-gate report), so no test was run.
- Repo MCP and semio MCP were unreachable, so the goal tree was not read.
- The composite-capability forwarding and per-type `absorb` soundness were not checked.
- Scratch files written in the Windows temp directory (outside the repo) during the listing were deleted.
