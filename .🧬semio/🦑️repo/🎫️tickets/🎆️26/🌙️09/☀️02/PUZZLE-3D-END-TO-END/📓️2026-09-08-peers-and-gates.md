# Peers And Gates — 2026-09-08 23:46 CEST snapshot

Read-only survey for the coordinating session working `PUZZLE-3D-END-TO-END`. No source edited, no
builds run. All times below are relative to snapshot time (23:46 CEST); re-check mtimes before
trusting "N min ago" later in the session.

## TL;DR — hot files/subtrees to avoid right now

| Subtree / file | Owner ticket | Last touch | Risk to puzzle-3d work |
|---|---|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/👁️viewer/🦀️.rs` (staged, not yet committed) | `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` | ~14 min ago | Widens `ArtifactViewer` trait signature (`Option<&ViewModel>` param added, `render`'s `view_state` un-underscored). If puzzle-3d touches `Puzzle3dViewer`/`Puzzle5dViewer` trait impls, expect the same signature under your feet. |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (staged) | same | ~13 min ago | Small (2-line) SetLocale/SetTerminology test cleanup already landed. |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{🧊️3d,◻️2d}/…/✏️editor/🦀️.rs` | same | already committed (HEAD, no working diff) | Editor-cleanup script already stripped all `SetLocale`/`SetTerminology` arms from puzzle 2d/3d editors (locale/terminology moved OS-wide). Do **not** reintroduce artifact-local `SetLocale`/`SetTerminology` mutations here — the OS-wide replacement is expected. `🖐️5d` editor was **not** in that cleanup pass. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**`, `🔨️modules/📡️spr/**`, `🔨️modules/📺️renderer/**`, `🧵️backbone-worker.ts`, `🔨️modules/🌊️flow/📦️packages/rust/Cargo.toml` | `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` (window-context/`ViewModel`/mutation-ownership work) | 0–8 min ago — **live edits in progress right now** | Core plugin/reactor/renderer plumbing puzzle-3d's viewer/editor/window code depends on (`ArtifactViewer`, `ActionMeta`, `plugin_mount_surface`/`plugin_render_surface`, ViewModel projection) is being rewritten mid-session. |
| Puzzle's `🧬️schema/**` and mutation `.schema.json` leaves (plugins wave, "W6"/"W7") | `SCOPE-OWNED-SCHEMA-CONTRACTS` | last plugin-wave log entry 22:35 (≈71 min ago), ticket still open/running | Repo-wide schema-contract relocation; puzzle is explicitly in scope (see `🗑️generated/puzzle-wasm-check.txt` in that ticket) via the "plugins" work packages (W6/W6c/W6d/W6e, W7/W7c/W7d "mutation aggregates + leaf relocation, non-stdio"). Mutation leaf `$id`s/locations can move under you. |
| `INTERACTIVITY_AUDIT_PUZZLE_*` files listed below (root `📜️script.ts`) | n/a (repo-wide gate, not a peer ticket) | script.ts itself edited 23:14 (32 min ago) by someone | Extremely literal string/substring gate — see §4. |
| `.vscode/🧩️launch.seed.jsonc` / `.vscode/launch.json` | working tree, uncommitted (`M` in git status) | — | Hand-edit only the seed; regenerate, never hand-edit `launch.json` (see §4). |

Not currently hot for puzzle-3d specifically:
- `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` — active (many `.md` touched in last 3h) but scoped to stdio/Norm/framework artifact **packages** (Cargo/Nx/Bun composability), not puzzle. Status: **not finished** — "Validation In Progress… remaining native and aggregate gates must finish before ticket closure" (`📓️results.md`). Low direct collision risk, but if it changes root `📜️script.ts` Nx-input normalization or launch generation while you're running gates, expect transient noise.
- `COMPLETE-SEMIO-END-TO-END`, `ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS`, `END-TO-END-TESTING-REFACTOR` — open tickets with recent note activity but their touched files (per `git status`) are not under `🧩️puzzle` or the `💻️os` subtrees this snapshot checked; treat as background noise, re-check if you see unexpected diffs outside puzzle.
- No ticket folder exists for `🌙️09/☀️07` (the date the task asked to check) — only `☀️01/02/03/05/06/08` exist under `🌙️09`.

## 1. Live tickets (open, touched in last 3h or otherwise still active)

All are `status: open`.

| Ticket | Title | Description (trimmed) |
|---|---|---|
| `2026/09/08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES` | Composable Stdio Artifact Packages | Turn every stdio artifact into an independently compilable composable package (Cargo/Nx/Bun/schema-first). Fleet: GPT 6 Astra coordination, GPT 5.6 Sol execution, GPT 5.6 Terra read-only audits. |
| `2026/09/08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` | Correct Command Config and Mutation Ownership Levels | Audit/refactor commands, configs, mutations to proper domain/OS ownership. "Locale is global OS-wide, not Jack-specific." Fleet: GPT 6 Astra coordination, GPT 5.6 Sol execution, GPT 5.6 Terra read-only audits. |
| `2026/09/08/SCOPE-OWNED-SCHEMA-CONTRACTS` | Scope-Owned Schema Contracts | Repo-wide: every application contract gets exactly one eligible scope owner under `🧬️schema/`, all native formats, named exports; fixtures become examples only. Fleet: Fable 5.1 coordination, Opus 5 execution, Sonnet 5 read-only audits. |
| `2026/09/02/PUZZLE-3D-END-TO-END` | Puzzle 3d End To End | **This ticket** — `bun dev:puzzle:3d` boot, every window a working example, fill/editor tools working at runtime. |
| `2026/09/02/COMPLETE-SEMIO-END-TO-END` | Complete Semio End to End | Whole-repo completion umbrella (OS frontend, hub backend, AI, collaboration, admin, tests). |
| `2026/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` | Zero Warnings Zero Errors Across All Rust Compilation Targets | Drive every native + wasm32 compilation target to 0 warnings/0 errors. |
| `2026/08/23/END-TO-END-TESTING-REFACTOR` | End To End Testing Refactor | New repo test domain (taxonomy, Gherkin, oracle registry, reporting, cleanup…) as the language-neutral test-ownership unit. Very high note-churn in last 3h (100+ files), but not touching puzzle/os by `git status`. |
| `2026/09/03/PROCEDURAL-3D-END-TO-END` | Procedural 3d End To End | Sibling "end to end" ticket, generation3d app — same goal (`r2602/runningsketchpad`), no file overlap seen this snapshot. |
| `2026/09/06/PUZZLE-2D-END-TO-END` | Puzzle 2d End To End | Sibling puzzle ticket (2d artifact, 40 editor actions, 26 mutations) — **shares the `🧩️puzzle` plugin tree**; no uncommitted 2d-specific files this snapshot, but its ticket folder is active. Coordinate before touching anything shared between 2d/3d/5d (e.g. `AGENTS.md` terminology, shared `world3d`/renderer code). |

## 2. Uncommitted working tree

- `git status --short | wc -l` → **202** modified paths.
- Top-level histogram: `✏️s` 110, `🧰️framework` 65, `.🧬semio` 25, `.vscode` 2.
- Puzzle-specific uncommitted files (both staged, from `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS`):
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` — `ArtifactViewer::intent`-ish handler gains `_view_state: Option<&semio_framework_plugin::ViewModel>`; `render`'s `view_state` param is used now (no longer `_`-prefixed placeholder).
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — 2-line deletion, locale/terminology test cleanup.
- `💻️os` files edited in the last 0–10 minutes (i.e., someone is actively mid-edit right now): `🔨️modules/🔌️plugin/🦀️.rs` and its `🧪️tests/*` (dependency-contribution, contributed-mutation-wire, publication-fixtures-*, app-declarations-fixture, plugin-runtime-plugin-builder-contract, composition, mutation-fixtures-*, declaration-channels), `🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/**`, `🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/**`, `🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `🧪️tests/🔬️standalone/🦀️.rs`, `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, `🧵️backbone-worker.ts` (3 min ago), `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json` and `📺️renderer/🧬️schema/🔣️.json` (4 min ago), `🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (6 min ago). This is one continuous, currently-running edit session on plugin/reactor/renderer/spr/mutation-law infrastructure — matches the `window-context-ownership.md` narrative (ActionMeta/ViewModel projection, surface presence, ownership levels).

## 3. Auto-commit cadence

`git log --date=iso -15`:

```
de617a7c17 2026-09-08 23:25:57 +0200
025ec86a42 2026-09-08 20:58:18 +0200
9869c6e99b 2026-09-08 17:55:05 +0200
6152f9ca6a 2026-09-08 16:26:40 +0200
0a0bb74380 2026-09-08 14:30:02 +0200
860e015bf6 2026-09-08 04:50:36 +0200
aa72759d41 2026-09-07 16:52:51 +0200
49e59877ca 2026-09-07 06:49:48 +0200
0fab4f0c30 2026-09-06 20:48:37 +0200
2d2b39eb7f 2026-09-06 12:08:43 +0200
```

Cadence is irregular, roughly every 2.5–13 hours (not a fixed timer) — commit messages themselves embed
a frozen fake date template (`🎆️26🌙️06☀️04🚩️NNN`), so always read `--date=iso`, never the message
text, to reason about when something actually landed. The 202 currently-uncommitted files (§2) postdate
the last commit (23:25:57) and are still accumulating live.

## 4. Gates at ticket close

Root `📜️script.ts` (2,117,807 bytes, 30,680 lines, itself edited 23:14 today — 32 min before this
snapshot, by a peer session) is the single CLI (`bun ./📜️script.ts <command> <subcommand…>`), registering:
`os, semio, examples, setup, start, dev, generate, new, schema, lint, verify, format, test, bench, stdio,
build, cpp, publish, purge, clean, micro-commit, commit`.

Ordered gate list to run at close, most puzzle-relevant first:

1. **`bun ./📜️script.ts verify dependencies literal-external`** — red-until-zero: fails the process if
   any literal third-party runtime dependency is found outside the allowed oracle/toolchain exceptions
   (`target=0, current=<n>`). This is `INTERACTIVITY_ALL_APP_REQUIRED_GATES`'s `⚖️gate📦️dependencies0️⃣`.
2. **`bun ./📜️script.ts verify interactivity`** (`⚖️gate⚡️interactivity`) and its narrower forms:
   - `verify interactivity tool-jobs` (`⚖️gate⚡️interactivity🎯️tool-jobs`)
   - `verify interactivity apps` (`⚖️gate⚡️interactivity🧭️apps`) — includes the launch.json/registry
     coverage check (`INTERACTIVITY_ALL_APP_LAUNCH_FILE`/`…LAUNCH_SEED_FILE`/`…PLAYGROUND_FILE`).
   - `verify interactivity apps --actions` (`⚖️gate⚡️interactivity🧭️apps🎛️actions`)
   - The **puzzle-3d fill interactivity audit** specifically (see below) runs as part of the general
     `verify interactivity` sweep — it is string-literal source-scanning, not a separate CLI verb.
3. **`bun ./📜️script.ts verify dependencies`** (unrestricted report/summary form; `⚖️gate📦️dependencies`).
4. **`bun nx run @semio-tech/puzzle-js:publication-authority-audit`** — puzzle-specific gate
   (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` → `PublicationAuthorityAuditScript`,
   registered via `.register("publication-authority-audit", PublicationAuthorityAuditScript)`; other
   plugins alias it to their generic `TestScript`, puzzle has its own implementation). Also present in
   `.vscode/launch.json` as a launcher.
5. **`bun nx run repo:policy-check`**, **`bun nx run repo:graph-check`**, **`bun nx run repo:artifact-check`**
   — all three route through `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
   (`policy-check`→`PolicyScript`, `artifact-check`→`PolicyScript` again, `graph-check`→`GraphScript`);
   also present in `.vscode/launch.json` (`⚡️nx policy-check` / `graph-check` / `artifact-check`).
6. **Taxonomy/artifact-name registry**: `bun ./📜️script.ts verify taxonomy` (root `VerifyScript`
   `segments[0] === "taxonomy"`) validates `taxonomy.json` (owned at
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`) against the live tree —
   `TaxonomyCliOperation = "inventory" | "plan" | "apply" | "verify"`.
7. **Launch-seed freshness/regeneration**: `.vscode/🧩️launch.seed.jsonc` is hand-edited (per
   `🖥️launch.ts`'s own docstring: "Never hand-edit `.vscode/launch.json` directly: edit the seed…
   then regenerate"). Regenerate with **`bun nx run @semio-tech/plugin-registry:generate`**
   (`cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts generate`);
   freshness is checked by **`bun nx run @semio-tech/plugin-registry:check`**. Note `.vscode/🧩️launch.seed.jsonc`
   itself is currently in `git status` as modified (uncommitted) — check it regenerates clean before close.
8. `bun ./📜️script.ts verify rust-warnings`, `verify semantic-vocabulary`, `verify package-purity`,
   `verify layering`, `verify abstraction-ownership` exist as further general-purpose `verify` subcommands
   (not puzzle-specific but repo-wide gates that would still block a close if broken by your edits).

No `publication-authority-audit`, `policy-check`, `graph-check`, `artifact-check` subcommands exist on
the **root** `verify`/`schema` router; they are separate Nx-project-scoped commands (puzzle's own
`📜️script.ts`, and the repo-caching module's `📜️script.ts`), reached only via `bun nx run <project>:<target>`
as listed above — use the `nx run` form, not `bun ./📜️script.ts <name>`.

### 4a. The puzzle interactivity string-literal audit (root `📜️script.ts`)

Constants at **lines 8361–8373**, functions at **lines 8798 (`interactivityPuzzleFillEnvelopeFailures`),
8873 (`interactivityPuzzleFillP4eFailures`), 9063 (`interactivityPuzzleFillPreviewJsonFailures`, exported)**:

```
INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE   = …/🧊️3d/…/✏️editor/⏳️precompute/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE      = …/🧊️3d/…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE   = …/🧊️3d/…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_ACTION_FILE     = …/🧊️3d/…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE     = …/🧊️3d/…/🧬️schema/🦀️component.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE  = …/🧊️3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE   = 🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx
INTERACTIVITY_AUDIT_PUZZLE5D_FILL_PRECOMPUTE_FILE = …/🖐️5d/…/✏️editor/🧠️precompute/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE5D_FILL_WINDOW_FILE     = …/🖐️5d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE3D_TERMINOLOGY_FILE     = …/🧊️3d/…/✏️editor/🗣️terminology/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE5D_TERMINOLOGY_FILE     = …/🖐️5d/…/✏️editor/🗣️terminology/🦀️.rs
INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE = …/🧊️3d/…/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json
INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_TEST_FILE   = 🧰️framework/…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts
```

The checkers are **exact substring/regex matches against production source** (comments/tests stripped
via `interactivityProductionSource`), not semantic checks. They enforce, among ~30 invariants:

- Exact fixed-capacity constant names/values (`FILL_ENVELOPE_PAGE_BYTES = 16*1024`,
  `FILL_ENVELOPE_MAX_PAGES = 256`, `FILL_ENVELOPE_MAX_ITEMS = 65_536`, `FILL_PREVIEW_JSON_MAX_BYTES = 4*1024`,
  `FIXED_OWNER_SLOTS = 32`, …) — renaming or resizing any of these fails the gate.
- No `serde_json::`/whole-state `checkpoint_bytes()`/`FillWorkerState` in the UI/worker route (bans
  whole-state (de)serialization on the interactive path; only bounded-fuel cursor-based JSON page
  encoding — `FillPreviewJsonPhase::{Census,Reserve,Encode,Validate,Ready,Rejected,Closing,Terminal}` —
  is allowed).
- Exact struct/enum names (`FixedOwnerMap`, `FixedOwnerVec`, `FillEnvelopeTokenCursor`,
  `FillBuilderOwnerCensusCursor`, `FillBuilderRetirementCursor`, `CollisionIndexMutation`, …) and exact
  method call sites (`registry.begin_measurement(...)`, `authority.fill.as_ref()?.try_lock().ok()?`,
  `self.spatial_index.step_query(query, owner)`, …).
- Named fixture/test function existence (dozens of `fn fill_worker_*`/`fn spatial_*`/
  `fn retained_owner_census_*` literal names must exist verbatim in the precompute/fill/geometry test
  modules) — deleting or renaming a fixture test fails the gate even if a replacement with equivalent
  coverage is added under a different name.
- **Terminology strings, verbatim, both languages**: both `INTERACTIVITY_AUDIT_PUZZLE3D_TERMINOLOGY_FILE`
  and the 5d counterpart must contain exactly
  `fill_progress: native_en "Fill progress", native_de "Füllfortschritt", reuse_en "Fill progress", reuse_de "Füllfortschritt"`
  and must **not** silently default an unsupported locale/terminology (`{ Locale::De } else { Locale::En }`
  and `.unwrap_or(Terminology::Native)` are explicitly banned patterns — must fail closed with `None` instead).
- Renderer (`World3dHost/🟦️.tsx`) must keep exact type/JSX/aria strings: `type WorldFillDiagnosticRecord`,
  `function FillDiagnosticOverlay`, `aria-label={label}`, `<span>{diagnostic.statusLabel}</span>`,
  `data-fill-registry-generation={diagnostic.registryGeneration}`, `data-fill-truncated={diagnostic.truncated}`,
  and the parser (`parseWorldBrushPreview`) must keep its exact allow-listed key sets
  (`WORLD_FILL_ROOT_KEYS`, `WORLD_FILL_DIAGNOSTIC_KEYS`, `WORLD_FILL_GHOST_KEYS`) and bound
  (`WORLD_FILL_COLOR_MAX_BYTES = 128`).
- A JSON fixture (`INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE`) must parse with
  `schema === "semio.puzzle3d.fill-preview-json.v1"`, `limits.maximumBytes === 4096`,
  `limits.maximumColorBytes === 128`, `limits.maximumStatusLabelBytes === 256` (checked further past
  line 9175, not fully re-transcribed here — read `interactivityPuzzleFillPreviewJsonFailures` in full
  before editing this fixture).

**Practical implication**: any refactor of the puzzle-3d/5d fill precompute/geometry/schema/window/
renderer/terminology files listed above must grep this audit's constant/string literals first
(`grep -n "INTERACTIVITY_AUDIT_PUZZLE" 📜️script.ts` then read lines 8798–9250ish) and preserve every
matched identifier, constant value, and translated string exactly, even when the change is a pure
rename/refactor with identical behavior — the gate does not understand semantics, only literal source
text.

## 5. Docstring/emoji conventions (puzzle 3d examples)

Every docstring starts with one unique, semantically-fitting emoji; no comments inside definitions
(only doc comments above them). Confirmed live in this plugin:

**Rust** (`✏️editor/⏳️precompute/🦀️.rs`, line 16):
```rust
/// ⏳️ Default cap on how many objects one fill session may plan — was `⚙️engine`'s own
/// `FILL_COUNT_MAX`; distinct from (and not to be confused with) the UI-facing
/// `crate::editor::puzzle3d::PUZZLE3D_FILL_COUNT_MAX` slider clamp.
```

**Rust** (`✏️editor/🦀️.rs`, line 68 — window-option doc comments):
```rust
/// 🌀️ Window option: emit every object's vortices into the 3D scene.
/// 🧭️ Window option: arrow tip points away from the vortex point along `direction`.
```

**TypeScript** (`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`, lines 3–5, 141, 146):
```ts
/** @emoji 🌐️ `World3dHost` — the 3D world viewport scene host: mesh/instance parsing, point-cloud and
 * vortex-marker layers, catalogue-drop and selection-preview stores, instance chrome, the transform
 * gumball, and the full R3F `World3dHost` component mounted inside a Mode window. */

/** 🎨️ Compatible/suggested state (e.g. catalog-kind hover in puzzle) — resolves to the secondary
 * "highlighted" mesh style. */

/** 🪣️ 0-based position in a background-planned sequence (e.g. puzzle3d's fill plan) — see
 * `RevealCutoffStore`. Absent for ordinary (non-planned) instances. */
```

Note the TS convention uses `@emoji` inline in some places and a bare leading emoji in others — both
forms are live in this exact file; match whichever convention the nearest existing docstring in the
file you're editing uses.

## 6. `AGENTS.md` notes (read-only; do not edit per CLAUDE.md)

- `✏️s/AGENTS.md`: three lines, only technology name/emoji frontmatter (`technology: s`, `emoji: 🖥️`)
  plus one description sentence. No layout/naming rules.
- `✏️s/🔌️plugins/🧩️puzzle/AGENTS.md`: frontmatter (`technology: puzzle`, `emoji: 🧩`) plus the
  cross-dimension terminology table only — the 2d/3d/5d parallel vocabulary
  (`NodeKind`/`ObjectKind`/`PartKind`, `Node`/`Object`/`Part`, `HandleKind`/`VortexKind`/`GripKind`,
  `Handle`/`Vortex`/`Grip`, `EdgeKind`/`AttractionKind`/`FastenerKind`, `Edge`/`Attraction`/`Fastener`,
  `WireKind`/`CableKind`/`RopeKind`, `Wire`/`CableKind`/`RopeKind` — note the last row's 3d/5d columns
  look like a copy-paste bug, both say `CableKind`/`RopeKind` instead of `Cable`/`Rope`). Neither file
  states explicit editor/window/command code-layout rules; those rules live only in CLAUDE.md itself
  and in the interactivity audit's literal checks (§4a). Do not infer additional constraints beyond what
  is written.
