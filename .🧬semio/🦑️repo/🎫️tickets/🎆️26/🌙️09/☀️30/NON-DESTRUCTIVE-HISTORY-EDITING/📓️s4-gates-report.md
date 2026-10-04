# 📓️ S4-GATES — repo gates, CLOSURE-4, fault notices, CODES-TAX residue (session 4)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, WP S4-GATES (Opus executor, session 4). Inherits S3-GATES
(`📓️s3-gates-report.md`), S3-CLOSURE item CLOSURE-4 (`📓️s3-closure-report.md`), S3-NOTICES (`📓️s3-notices-report.md`) and
S3-CODES-TAX residue (`📓️s3-codes-tax-report.md`). Scratch: `🗑️generated/s4-gates/`. Ticket input scripts: `🧪️s4-gates-*.py`.

## Session 4 — 2026-10-04

### 0. Resume (02:13)

- Read rules 1–38, AGENTS.md, design §11/§20.5/§20.12, plan roster, `📓️s4-resume.md` §0/§0.1/§2.7/§3.9/§4/§6.1/§7 and the last
  sections of the four inherited reports.
- Coordinator decisions in force: docstring-emoji uniqueness PER FILE for every file this ticket touched (no repo-wide sweep); the 17
  unapproved peer dependency additions are listed for their owners (never removed by us); the framework notice table covers the 35
  framework-namespace guest codes.

### Progress

(milestones appended below)

#### M1 (02:15–03:40) — repair-first, framework notice table, scoped notices gate, CLOSURE-4 source

**Repair-first (rule 34).** No half-finished edit of the four inherited WPs found: the inherited gate files (root `📜️script.ts`
closure/outcome-law regions, `🧪️test/🧬️schema/📋️orchestration/🟦️.ts` `📢️FaultNotices`, `🧪️test/📜️script.ts`) are untouched since
10-03 11:10–11:51 (before the S3 cut); later mtimes on `⚠️diagnostic`/`🛂️manifest`/`🎠️kernel` are peers' (value/DSL wave). Runs:
`bun ./📜️script.ts test outcome-law-gate` **25/0**; `verify history-closure --json` self-test **31/31**, census `coalesce-key 10` (was 14:
S4-BUMP landing) · `bracket-verb 11` (stale descriptors) · rest 0. Left by S3-NOTICES: one `[DEBUG]` println in
`🎠️kernel/🧪️tests/🧪️fault-notices/🦀️.rs` (removed in M2, see §debug-tags).

**NOTICES — framework table for framework-namespace guest codes (coordinator decision).** New framework table beside
`HISTORY_NOTICE_LABELS`, told with the fault's own severity (history-lane rows stay warnings in wgpu/React):
- Rust `🎠️kernel/🦀️.rs` region `🔖️FaultNotices`: `FRAMEWORK_FAULT_NOTICE_LABELS` (11 rows) + `framework_fault_notice(code)`; `fault_notice`
  resolves history table → framework table → app table (wgpu `classify_dispatch_fault_notice` and React `appFaultNoticeV1` both go through it).
- TS twin `🎠️kernel/🟦️.ts` `FRAMEWORK_FAULT_NOTICE_LABELS`, `frameworkFaultNotice`, `faultNotice` updated.
- Schema-first: `🎠️kernel/🧬️schema/🔣️framework-notices/🔣️.json` + fixture `🎠️kernel/🧫️fixtures/🧫️framework-notices/🔣️.json` (Rust and TS carry
  exactly its rows). Rows: `app.command.{unsupported, invalid, invalid-args, invalid-payload, targets-required, kind-unavailable,
  target-in-use, tool-mismatch}`, `mutation.{target-missing, target-mismatch, too-large}` (en/de). `app.command.tool-mismatch` is NEW: the
  13 plugins' identical `<app>.retained.tool-mismatch` refusal can switch to it (routed to owners).
- Tests: Rust `🎠️kernel/🧪️tests/🧪️framework-notices/🦀️.rs` (mirror + both locales + same placeholders + guest-fault resolution); TS twin
  `🧪️tests/🧪️framework-notices/🟦️.ts` (Ajv strict + hostile rows + resolution), registered in `🎠️kernel/🧪️tests/🎚️config/🟦️.ts`; manifest
  corpus `🛂️manifest/🧫️fixtures/🧫️fault-notices/🔣️.json` +2 resolutions (framework code de; too-large as a cause), i18next oracle resources
  extended with the framework rows (`🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts`).
- Gate: `FAULT_NOTICE_FRAMEWORK_TABLES` (both kernel fixtures) label codes.

**NOTICES — scoped gate mode.** `bun ./📜️script.ts schema fault-notices --scope history-editing` (`FaultNoticeScope`, `faultNoticeInScope`):
keeps every framework-namespace code, every code with a tool-flow segment (`tool|gesture|gumball|drag|scrub|press|stroke|transaction|
history|replay|ink`), every anonymous fault in a tool-flow source (`🛠️`, `tool`, `gesture`, `gumball`, `scrub`, `press`, `transaction` path
segment) and every declared-table/descriptor verdict. Gate fixture `🧫️fault-notices-gate` gains a `🛠️tools` source and `scoped.findings`;
test asserts both scopes.

**CLOSURE-4 (source).**
- `🌿️vcs/🦀️.rs`: `VcsError::TooLarge { rows, capacity }` → fault code `mutation.too-large` (framework notice, no params); law
  `🌿️vcs/🧪️tests/🧪️fault-params/🦀️.rs::an_oversized_edit_refuses_under_the_framework_too_large_code`.
- `🏪️store/🦀️.rs`: `ArtifactStoreOneItemFootprint::for_gesture` (merged `for_leaf` rows vs `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS`,
  refused as `TooLarge`); the batch fold and commit contracts now refuse an over-declared row count as `TooLarge` (bounded ceilings
  1025/768/258/4096) instead of the English `ValidationFailed`.
- `🔌️plugin/🦀️.rs`: artifact publication pre-admits the gesture with `for_gesture` (localized refusal before the batch admission's
  English string); `store_publication_fault` keeps `TooLarge`'s code.
- Law `📡️spr/🎮️command/🦀️.rs::mutation_inverse_rows_declaration_failures` (re-exported from `📡️spr/🦀️.rs`), emitted into every derived
  payload law (`🗣️dsl/✨️derive/🦀️.rs`): each editable op answers exactly its schema-declared rows; each `perTarget` leaf grown along its
  first target field is admitted at the ceiling and refused one target past it as `mutation.too-large`.
- Hand aggregates vs schemas: closure gate rule `footprint-default` (root `📜️script.ts`, `HistoryClosureRule.item`): an `impl … Mutation<…>`
  that forwards `INPUT_SCHEMAS` must forward `fn inverse_rows`. Self-test +3 cases (34).

Runs: `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` **5/0** (54 expects); kernel vitest (owner script, explicit
policy) `🧪️framework-notices 🧪️history-notices` **2 files / 5 tests pass**; `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` **2/0**;
`verify history-closure --self-test` **34 cases pass**; `cargo check -p semio-framework-os-kernel --lib --tests` lib + lib-test GREEN 03:18
(one E0507 of mine fixed within 10 min; remaining red = peer integration test `sqlite_snapshot_native_admission`, `native_decoding`
unresolved). Scoped gate on the repo: **54** findings (faultNoticeFramework 35 → **0**); remaining = plugin-owned (table below, M2).

#### Routing — `schema fault-notices --scope history-editing` (03:50, 54 findings, all plugin-owned)

Paths relative to `✏️s/🔌️plugins/<plugin>/🗿️artifacts/` (playbook: `✏️s/🔌️plugins/`). `*.tool-mismatch` (13 copies of one refusal) → the framework code `app.command.tool-mismatch` (labelled in the framework table); other codes → the app's `fault_notices()` table (en/de); anonymous `Fault::from(text)` → a named code + notice.

| Owner WP | Plugin | Class | File:line | Fix |
|---|---|---|---|---|
| S4-FLOWCAD | 🌊️flow | faultNoticeMissing | `🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2475` | `flow.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-GRAPHS | 🎬️sequence | faultNoticeMissing | `🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3192` | `sequence.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-GRAPHS | 🕸️dag | faultAnonymous | `🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:104` | Fault::from(text) → named code + notice |
| S4-GRAPHS | 🕸️dag | faultAnonymous | `🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:51` | Fault::from(text) → named code + notice |
| S4-GRAPHS | 🕸️dag | faultAnonymous | `🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:52` | Fault::from(text) → named code + notice |
| S4-GRAPHS | 🕸️dag | faultAnonymous | `🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:53` | Fault::from(text) → named code + notice |
| S4-PUZZLE | 🧩️puzzle | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:117` | Fault::from(text) → named code + notice |
| S4-PUZZLE | 🧩️puzzle | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:123` | Fault::from(text) → named code + notice |
| S4-STROKES | 🀄️wfc | faultNoticeMissing | `◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:941` | `wfc2d.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-STROKES | 🀄️wfc | faultNoticeMissing | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:544` | `wfc3d.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-STROKES | 📸️remodel | faultNoticeMissing | `📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:909` | `remodeling.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TEXT | 🔱️trinity | faultAnonymous | `🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:90` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🎥️shooting | faultNoticeMissing | `🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:706` | `shooting.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:159` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:163` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:50` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:71` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🖱️canvas-gesture/🦀️.rs:133` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🖱️canvas-gesture/🦀️.rs:63` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:161` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:167` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:168` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:170` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:52` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🏗️fem | faultAnonymous | `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:73` | Fault::from(text) → named code + notice |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs:52` | `drawing.gesture.retained-route` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1324` | `drawing.bounded.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1703` | `drawing.gesture.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:588` | `drawing.gesture.closing` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:617` | `drawing.gesture.saturated` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:621` | `drawing.gesture.owner` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:629` | `drawing.gesture.point-capacity` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:633` | `drawing.gesture.query-owner` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:643` | `drawing.gesture.query-capacity` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:666` | `drawing.gesture.query-output-capacity` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:753` | `drawing.gesture.command` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🖍️draw | faultNoticeMissing | `🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:282` | `drawing.viewer.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs:135` | `note.ink-tool.provisional` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs:159` | `note.ink-gesture.invalid` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs:179` | `note.ink-events.invalid` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs:324` | `note.ink-phase.invalid` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:172` | `note.retained.transaction` → declare in `fault_notices()` (en/de) |
| S4-TOOLS-A | 🗒️note | faultNoticeMissing | `🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:419` | `note.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-B | 🌀️procedural | faultAnonymous | `🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:397` | Fault::from(text) → named code + notice |
| S4-TOOLS-B | 🌀️procedural | faultAnonymous | `🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:482` | Fault::from(text) → named code + notice |
| S4-TOOLS-B | 🌀️procedural | faultAnonymous | `🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:457` | Fault::from(text) → named code + notice |
| S4-TOOLS-B | 🌀️procedural | faultAnonymous | `🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:549` | Fault::from(text) → named code + notice |
| S4-TOOLS-B | 🌍️gis | faultNoticeMissing | `🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:324` | `gis.map.viewer.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-B | 📖️playbook | faultNoticeMissing | `📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:984` | `playbook.module.procedural.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-B | 🔋️energy | faultNoticeMissing | `🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2185` | `energy.model.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-B | 🔋️energy | faultNoticeMissing | `🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:388` | `energy.model.viewer.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |
| S4-TOOLS-B + coordinator describe | 🌀️procedural | faultNoticeDescriptor | `🌎️hub/🧩️compositions/🌀️procedural/🔣️.json:—` | describe owed — unpublished ["generation3d.gumball.component-selection","generation3d.gumball.host-edit","gene |
| S4-WIRES-MATH | 💡️reasoning | faultAnonymous | `🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:113` | Fault::from(text) → named code + notice |
| coordinator (no S4 owner) | 🪐️space | faultNoticeMissing | `🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:327` | `s.space.index.retained.tool-mismatch` → use framework `app.command.tool-mismatch` |

54 findings

#### M2 (03:50–04:23, cut ~04:15, resumed 06:45) — debug-tags, docstrings, multi-scope taxonomy verifier

- **Resume check (06:45):** every M1 edit is on disk (`git diff HEAD --stat` over kernel/vcs/spr/orchestration/root script/taxonomy
  module; greps for `FRAMEWORK_FAULT_NOTICE_LABELS`, `for_gesture`, `TooLarge`, `mutation_inverse_rows_declaration_failures`,
  `footprint-default` all present). The multi-scope verifier landed at 04:23 (before the cut) and transpiled.
- **`[DEBUG]` (`verify debug-tags`, 03:55):** repo-wide 611 lines / 244 files (oracle agrees). Ticket window (`🧪️s4-gates-source-rules.ts`,
  1892 ticket files, lines added since `3eeee4f9119`): 42 tagged lines; the one left by S3-NOTICES (`🎠️kernel/🧪️tests/🧪️fault-notices/🦀️.rs`
  println) REMOVED. Remaining 41 are peers' (none written by this ticket's WPs): `🧊️3d/🥽️mesh/🧪️tests/🔬️jobs/🦀️.rs` ×24 (multi-UV mesh peer),
  `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` ×7 (mesh peer), `🌿️vcs/🧪️tests/🔬️unit/🦀️.rs:411,443` (paged-ledger ticket), `🏪️store/📜️space-history/…/🪶️sqlite/🦀️.rs:118,429`
  (sqlite peer), `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs:33,118` (hub restructure), `📡️replication/🎮️mutation/🧪️tests/🧪️outcome-code/🦀️.rs:120`
  (ValueError peer 10-03 01:14), `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3171` (download peer, 10-04), `🖍️draw/…/🎨️fill/🧪️tests/🔬️unit/🟦️.ts:139`
  (draw peer, mtime 10-04 01:27), `📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts:52,56,60` (asserts a generated script's tag), `📚️library/📦️packages/🟦️typescript/📜️script.ts:167`.
- **Docstrings (same sweep):** 54 docstrings without a leading emoji on window lines, 401 indented line comments, **2318 per-file emoji
  repeats** across 236 files (top: store 229, plugin 192, plugin time-travel 142, wgpu shell 103, manifest 86). Per coordinator decision
  the uniqueness is enforced per touched file; the repeats are concentrated in 1.5 MB–3 MB hot shared files (rule 17 forbids script rewrites;
  hand-editing 2318 docstrings under 20 writers is a wave of its own) → listed in `🗑️generated/s4-gates/source-rules.json`; fixed in the files
  this session created (all new files carry unique emojis).
- **§5 taxonomy — multi-scope verifier (coordinator request 03:52, freeze lifted 04:2x):** a single `verify taxonomy report --scope` pays a
  fixed ~35–70 s (repository index enumeration, source-admission byte sorts, the dead `TransactionRepositoryAuthority` over all index rows,
  incoming-reference scan). Landed in `📚️library/🧹️normalization/🟦️.ts` + `🚪️source-admission/📁️io/🟦️.ts`:
  `sourceAdmissionCapture` (index read once, rows keyed by normalized path) + `sourceAdmissionScopedOptions` (binary-searched scope range);
  `verifyTaxonomyScopes` (generator) — one capture, per-scope inventories, ONE incoming-reference scan (`incomingReferenceScan`, union of every
  scope's changing paths), ONE registry catalog input view, per-scope plan verdict (`taxonomyPlanVerdict`, shared with `verifyTaxonomy`);
  the unused `RetainedSourceAdmission.repositoryAuthority` (built per inventory, read nowhere) deleted. Root route
  `bun ./📜️script.ts verify taxonomy report --scopes-from <file> [--json]`. Law `🧹️normalization/🧪️tests/🧪️multi-scope-verification/🟦️.ts`
  (+ `🧫️fixtures/🧫️multi-scope-verification`, `🧬️schema/🔣️multi-scope-verification`; scopes: clean leaf, parent + nested child, a moving scope):
  each multi verdict equals `verifyTaxonomy` of that scope; wired as `bun ./📜️script.ts test multi-scope-verification` (library), nx
  `@semio-tech/repo-lib:test-multi-scope-verification`, package script, launch seed row (order 900.057741; `.vscode/launch.json` regeneration =
  coordinator). Scratch equivalence before landing: 4 scopes SAME (single vs multi).
- **First chunk (150 of 1582 roots, 04:12–04:28, pre-catalog-cache):** 959 s, 41 dirty; codes: `directory-kind-unresolved` 126,
  `fixed-source-disposition-unresolved` 4 (root-script body in `🎠️kernel/🫧️transient`, `🖱️ui/🌐️locale`, `🧪️test`, `🧬️schema/📄️source/🔗️closure/…`),
  `reference-preimage-unreadable` 2 (wgpu `⏪️time-travel`, `🧪️wgpu-time-travel`), `normalization-move-required` + `semantic-stem-unresolved`
  (`🌿️vcs/🧪️tests/🔬️paged-ledger/📜️oracle.py`, paged-ledger ticket), `mutation-payload-schema-authority-invalid` 1 (spr mutation-laws
  `🌐️add-counter-then-notify`), `generator-preview-invalid` 1 (`.vscode/launch.json`, plugin-registry preview status 1 — not ours).

#### M3 (06:45–08:55, cut; resumed 11:35) — law wiring, gates re-run, warnings, taxonomy results

- **Multi-scope verifier, final shape.** `verifyTaxonomyScopes` is a generator: every scope without a planned move is planned and yielded
  first (no incoming scan needed), then the one shared incoming-reference scan, then the moving scopes; the registry catalog input view
  is cached in the capture (`generatorPlanning`, it was a repo walk per scope). Root CLI streams per scope (`--json` = one JSON line per
  scope). Equivalence run (real module, 07:0x): **4/4 SAME** single vs multi — `🔬️paged-ledger` (moving; single 501 s), `💡️service-operation`,
  `🎠️kernel/🧪️tests` (nested parent), `🛣️path`. Law + wiring: `🧹️normalization/🧪️tests/🧪️multi-scope-verification/🟦️.ts` (Ajv fixture check +
  per-scope equality, order-independent), fixture/schema `🧫️fixtures/🧫️multi-scope-verification`, `🧬️schema/🔣️multi-scope-verification`; library
  `bun ./📜️script.ts test multi-scope-verification`, nx `@semio-tech/repo-lib:test-multi-scope-verification`, package script, launch SEED row.
- **§5 taxonomy over all 1582 ticket-created roots** (`🗑️generated/s4-gates/tax-scopes.txt`, from `🧪️s4-gates-new-dirs.py`): verdicts for
  1576 roots (150 in the first chunk, 1426 streamed); 411 clean, 1165 dirty; the 6 roots left unplanned were the moving ones of the killed
  stream (planned last) — `🌎️hub/🧩️compositions`, `🔁️workflow/…/📸️snapshot`, `🖥️host/🧫️fixtures/🧩️component`, `📚️library/📇️catalog`, jack
  `🐚️shell`, tiff `🧾️document/…/🧪️tests` — plus `🔬️paged-ledger` from the single run (4: `📜️oracle.py` must move to `🐍️.py`, stem unresolved,
  plugin-registry generator preview status 1). 3652 unique findings, **3518 inside ticket-created directories**: `directory-kind-unresolved`
  2006, `mutation-fixture-unpaired` 1015 (a `🧫️fixtures/🧬️mutations/<leaf>/<case>` bundle without its `🧬️schema/🧬️mutations/<leaf>/🧪️tests/<case>`
  implementation case), `mutation-fixture-invalid` 209, `mutation-payload-schema-authority-invalid` 174 + `projection-member-unresolved` 174
  (leaf registry identity; energy 132 each), `path-too-long` 40, `fixed-source-disposition-unresolved` 16 (root-script bodies in
  `✏️s/🧑‍💻dev/*` ×10, `🎠️kernel/🫧️transient`, `🖱️ui/🌐️locale`, `🧪️test`, `🧬️schema/📄️source`, spatial-kernel), `projection-catalog-coverage` 7,
  `reference-preimage-unreadable` 3 (files changed during the run). Owner table (routed to `main`):

| Owner | Area | Findings (code × count) |
|---|---|---|
| ? | 🌎️hub | directory-kind-unresolved 2, fixed-source-disposition-unresolved 1 |
| S4-AGNOSTIC | 🎪️demonstrator | directory-kind-unresolved 1 |
| S4-FLOWCAD | 📐️cad | directory-kind-unresolved 5, projection-member-unresolved 1, mutation-payload-schema-authority-invalid 1 |
| S4-FLOWCAD | 🌊️flow | directory-kind-unresolved 2, mutation-fixture-unpaired 1 |
| S4-GRAPHS | 🎬️sequence | directory-kind-unresolved 1 |
| S4-GRAPHS | 📜️imperative | directory-kind-unresolved 1 |
| S4-GRAPHS | 🪐️space | directory-kind-unresolved 1 |
| S4-INFRA | ✏️s/🧑‍💻dev | directory-kind-unresolved 103, fixed-source-disposition-unresolved 10 |
| S4-INFRA | ✏️s/🔨️modules | directory-kind-unresolved 9, fixed-source-disposition-unresolved 1 |
| S4-NORM | 📕️norm | directory-kind-unresolved 665, mutation-fixture-unpaired 608, mutation-fixture-invalid 36 |
| S4-PUZZLE | 🧩️puzzle | directory-kind-unresolved 80, mutation-fixture-unpaired 69, mutation-fixture-invalid 6 |
| S4-PUZZLE | 🧱️block | projection-member-unresolved 5, mutation-payload-schema-authority-invalid 2, directory-kind-unresolved 1 |
| S4-STDIO | 🗄️stdio | directory-kind-unresolved 132, mutation-fixture-unpaired 45, path-too-long 3, reference-preimage-unreadable 1, mutation-payload-schema-authority-invalid 1 |
| S4-STROKES | 📸️remodel | directory-kind-unresolved 138, mutation-fixture-unpaired 127, projection-member-unresolved 2, mutation-payload-schema-authority-invalid 2 |
| S4-STROKES | 🀄️wfc | directory-kind-unresolved 24, mutation-fixture-unpaired 16, mutation-fixture-invalid 6, projection-catalog-coverage 3 |
| S4-STROKES | 🏭️process | directory-kind-unresolved 14, mutation-fixture-unpaired 14, projection-member-unresolved 1 |
| S4-STROKES | 🖨️raster | directory-kind-unresolved 15, mutation-fixture-unpaired 6, projection-member-unresolved 1, mutation-payload-schema-authority-invalid 1 |
| S4-TEXT | 🔱️trinity | directory-kind-unresolved 85, path-too-long 25, mutation-fixture-unpaired 23, projection-member-unresolved 10, mutation-payload-schema-authority-invalid 10 |
| S4-TEXT | ✒️writer | directory-kind-unresolved 2 |
| S4-TEXT | 🌿️vcs | directory-kind-unresolved 1 |
| S4-TOOLS-A | 🏗️fem | directory-kind-unresolved 28, mutation-fixture-unpaired 15, mutation-fixture-invalid 13, path-too-long 5, projection-catalog-coverage 4 |
| S4-TOOLS-A | 💠️lowpoly | directory-kind-unresolved 29, mutation-fixture-unpaired 22, mutation-fixture-invalid 6, projection-member-unresolved 4, mutation-payload-schema-authority-invalid 4 |
| S4-TOOLS-A | 📏️layout | directory-kind-unresolved 26, mutation-fixture-unpaired 24, path-too-long 1 |
| S4-TOOLS-A | 🖍️draw | directory-kind-unresolved 38, mutation-fixture-unpaired 5, projection-member-unresolved 3, mutation-payload-schema-authority-invalid 2, mutation-fixture-invalid 1 |
| S4-TOOLS-A | 🎥️shooting | directory-kind-unresolved 9, projection-member-unresolved 8, mutation-payload-schema-authority-invalid 8 |
| S4-TOOLS-A | 🗒️note | directory-kind-unresolved 1 |
| S4-TOOLS-B | 🔋️energy | directory-kind-unresolved 309, projection-member-unresolved 132, mutation-payload-schema-authority-invalid 132, mutation-fixture-unpaired 31, mutation-fixture-invalid 12 |
| S4-TOOLS-B | 🌀️procedural | directory-kind-unresolved 11, projection-member-unresolved 6, mutation-payload-schema-authority-invalid 1 |
| S4-TOOLS-B | 🌍️gis | directory-kind-unresolved 12, path-too-long 4, mutation-fixture-unpaired 2 |
| S4-TOOLS-B | 📋️forms | directory-kind-unresolved 12, mutation-fixture-unpaired 3, mutation-fixture-invalid 3 |
| S4-TOOLS-B | 📖️playbook | directory-kind-unresolved 2 |
| S4-WIRES-MATH | 💡️reasoning | directory-kind-unresolved 5, mutation-fixture-unpaired 2, mutation-fixture-invalid 2 |
| S4-WIRES-MATH | ➗️mathematical | mutation-payload-schema-authority-invalid 3, directory-kind-unresolved 1 |
| coordinator | 🪵️sourcing | directory-kind-unresolved 1, projection-member-unresolved 1 |
| coordinator (W2-R architect) | 🏛️architect | directory-kind-unresolved 93 |
| framework | 📐️brep | directory-kind-unresolved 55 |
| framework | 🔨️modules/🏪️store | directory-kind-unresolved 24 |
| framework | 🔨️modules/🔌️plugin | directory-kind-unresolved 10, mutation-payload-schema-authority-invalid 6, path-too-long 2 |
| framework | 🧬️contract | directory-kind-unresolved 8 |
| framework | 🌐️locale | directory-kind-unresolved 6, fixed-source-disposition-unresolved 1 |
| framework | 📄️source | directory-kind-unresolved 5, fixed-source-disposition-unresolved 1 |
| framework | 🔨️modules/📺️renderer | directory-kind-unresolved 4, reference-preimage-unreadable 2 |
| framework | 📡️wire | directory-kind-unresolved 5 |
| framework | 🔨️modules/🪐️space | directory-kind-unresolved 4 |
| framework | 🎨️styling | directory-kind-unresolved 3 |
| framework | 🔨️modules/🌿️vcs | directory-kind-unresolved 1, normalization-move-required 1, semantic-stem-unresolved 1 |
| framework | 🔨️modules/📇️directory | directory-kind-unresolved 3 |
| framework | 🔨️modules/📚️library | directory-kind-unresolved 2 |
| framework | 🔨️modules/🧪️test | directory-kind-unresolved 2 |
| framework | 🛂️manifest | directory-kind-unresolved 2 |
| framework | 🔨️modules/🔁️workflow | directory-kind-unresolved 1 |
| framework | 🎛️controlled | directory-kind-unresolved 1 |
| framework | 🧪️tests | directory-kind-unresolved 1 |
| framework | 🫧️transient | fixed-source-disposition-unresolved 1 |
| framework | 🧬️retirement | directory-kind-unresolved 1 |
| framework | 🧫️fixtures | directory-kind-unresolved 1 |
| framework | 🧱️elements | directory-kind-unresolved 1 |
| framework | 🥽️mesh | directory-kind-unresolved 1 |
| framework | 📜️script.ts | fixed-source-disposition-unresolved 1 |
| framework | 🔌️adapter | directory-kind-unresolved 1 |
| framework | 🥒️gherkin | directory-kind-unresolved 1 |
| framework | ✅️validator | directory-kind-unresolved 1 |
| framework | 📶️state | directory-kind-unresolved 1 |
| framework | 🔨️modules/📡️spr | mutation-payload-schema-authority-invalid 1 |

total ours 3518 all 3652

- **Gates re-run.** `verify history-closure --json`: self-test 34/34; `coalesce-key` **0** (S4-BUMP landed), `bracket-verb` 11 (descriptors →
  describe wave), `footprint-default` **6** (puzzle ◻️2d `🧬️mutations/🦀️.rs:575,773`, 🖐️5d `:553,743`, 🧊️3d `:860,1066` → S4-PUZZLE), rest 0.
  `verify mutation-outcome-law`: **11 breaches**, all new plugin code (jack `mutation.child-refused` ×8 → S4-TEXT; stdio png/bmp
  `*.diff.invalid-bytes`, pptx `stdio.pptx.canonical-address` → S4-INFRA/S4-STDIO). `bun ./📜️script.ts test outcome-law-gate` 25/0 (M1).
  `verify docstrings emoji-first` repo-wide 4570 (at-emoji 482, no-emoji 4088; class pre-existing); ticket window 54 → the 17 en1991 diff
  headers fixed (all 80 en1991 `🔺️diff/🦀️.rs` headers `//! Diff for` → `//! 🔺️ Diff for`, doc-only) + 37 in the DSL peer's `🗣️dsl` crate.
  Per-file uniqueness of the docstrings this session wrote: 14 repeats found and replaced with emojis unused in their file (kernel Rust/TS,
  orchestration, root script, taxonomy module, source-admission io). `verify dependencies` (ratchet, 17:37 min): **20 NEW** — the 17 known peer
  additions plus 3 newer peers (`@noble/hashes` `🎒️pack/🌱️value`, `@webassemblyjs/leb128` `🎒️pack`, rust `hex` stdio bmp); listed for owners,
  not approved, not removed (coordinator decision). `literal-external=273`, oracle conflicts 3 (unchanged). `i18next` (the notices'
  oracle) is in the baseline as ui-react `production-runtime`; registering it as a test oracle would surface the same production-declaration
  conflict as `js:xstate` → left for the interface owner (same routing as xstate).
- **Rust warnings** (`cargo check -p semio-framework-replication -p semio-framework-tool-machine -p semio-framework-time-travel -p
  semio-framework-os-kernel -p semio-framework -p semio-framework-plugin --lib`, exit 0, 08:37): 1146 unique (1092 `unnecessary qualification`,
  the rest unused imports/dead fns, all in peer regions: DSL/value extraction, plugin L3/L7019/16114/19369/39289, tool-run, framework re-exports).
  In this session's regions: 1 (`for_leaf`/`for_gesture` `crate::os_spr::Mutation` qualification) — FIXED; `cargo check -p
  semio-framework-os-kernel --lib --tests` after it: lib + lib-test green (integration test `sqlite_snapshot_native_admission` red = peer
  `native_decoding`).
- **Rust tests run (before rule 43):** `cargo test -p semio-framework --lib -- framework_notices fault_notices history_notices` **8/8**;
  `cargo test -p semio-framework-os-kernel --lib -- fault_params semio_payload_law` **12/12** (2 vcs laws incl. `an_oversized_edit_refuses_under_the_framework_too_large_code`,
  10 derived payload laws now carrying `mutation_inverse_rows_declaration_failures`). OWED (rule 43): plugin crates' derived payload laws with
  `perTarget` leaves (puzzle 2d/3d/5d, layout, drawing, gen2d/gen3d, fem mesh, wfc…) — the cap/cap+1 `mutation.too-large` property runs there.
- **CODES-TAX residue:** owners assigned (wgpu nested-cargo catalog drift → S4-WGPU; generator dynamic-import discovery → S4-INFRA); the two
  stale fixtures are already deleted in the index with zero live readers (only the sha256-sealed census `🧼️remaining-package-purity-authority`
  names one, as frozen evidence).

#### M4 (11:35–12:40) — law re-run, F13, taxonomy class analysis (coordinator decision request), root scripts, new deps

- **Law** `🧪️multi-scope-verification`: first run 1/2 — the ONLY difference was the paged-ledger scope's `generator-preview-invalid` message
  (`status=-1` in the single run vs `status=1` in the multi run, different stderr digest): the plugin-registry preview subprocess is spawned
  afresh by each plan and times out under load. The law now compares a rejected preview without the subprocess's status/output digests
  (code, path, severity and message prefix stay exact); route budget = `TEST_LEVEL_BUDGET_MS.exhaustive` (the run needs ~21 min at load 70).
  Re-run in progress (result below when done).
- **F13 (`📓️audit-s4-core.md`):** `history-filter.unknown` (raised by the runtime for an undeclared agent/MCP history filter) added to the
  framework notice table — Rust + TS twin + fixture + schema pattern (`history-filter` namespace); kernel vitest 2 files / 6 tests, manifest
  oracle 5/0, `cargo check -p semio-framework --lib --tests` green (12:22). `timeTravel.name-invalid` is already localized through the
  time-travel label table (`TIME_TRAVEL_CODE_LABELS` → `RefusalNameInvalid`; React `ui.timeTravel.refusal.nameInvalid`; wgpu `TimeTravelLabel`),
  so it needs no framework row. `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` green (11:57) —
  the notices JS path (`js_program_fault`) compiles on wasm32 (S3-NOTICES' owed check).
- **F16 per-file docstring-emoji uniqueness:** this session's own docstrings fixed (M3); the audit's store/tool-run/channel/transient repeats
  are on peer-owned lines (S4-STORE, S4-RUNTIME, S4-BUMP) — hot shared files, hand edits by their owners (listed in `source-rules.json`).

##### Taxonomy classes — analysis and branch (coordinator request 12:0x)

Method: for each flagged path a layout key (last 4 segments, non-structural names → `*`); counted against the pre-ticket tree
(`3eeee4f9119`); then 46 PRE-TICKET directories of the same layouts (untouched by this ticket, 7 layouts, up to 12 plugins each) verified with
`--scopes-from` (`🗑️generated/s4-gates/tax-base-sample.*`), and owner-level (subset-root) scopes compared with narrow scopes for the same trees.

| Layout key (flagged here / in base) | Pre-ticket sample at the same narrow scope | Same subset at owner (subset-root) scope |
|---|---|---|
| `🧫️fixtures/🧬️mutations/*/*` (989 kind + 986 unpaired / 1825 in base) | 11 of 12 flagged `directory-kind-unresolved` + `mutation-fixture-unpaired` | wfc ◻️2d: unpaired 6 → 0, invalid 3 → 0; wfc 🧊️3d same; bitmap unpaired 4 → 0 |
| `🧬️mutations/*/🧪️tests/*` (148 / 1965), `…/⛔️refuses` (137 / 155), `…/✅️applies` (74 / 155) | flagged the same way (kind, catalog coverage) | resolved where the subset's vector catalog pairs them |
| `🧬️schema/📸️snapshot/🧪️tests/*` (70 / 10) | 7 of 7 flagged | (sqlite snapshot peer layout) |
| `✳️any/✏️editor/🎮️commands/*` (15 / 831) | 3 of 12 flagged | — |

Finding: the two big classes are mostly a **verifier scoping effect**, not a layout deviation and not a missing taxonomy rule. Mutation case
pairing (`normalizeMutationCasePairs` / `validateMutationCasePairs`, `📚️library/🧹️normalization/🟦️.ts` ≈3955–4095) only sees inventory
entries; a scope that holds the fixture bundle but not its `🧬️schema/🧬️mutations/<leaf>/🧪️tests/<case>` counterpart (or the subset's vector
catalog) can never pair it, so every case directory stays `directory-kind-unresolved` and every bundle `mutation-fixture-unpaired` — for
pre-ticket directories exactly as for ours. The unit of pairing is the SUBSET (`bySubset`).

Branch per class:
- `mutation-fixture-unpaired` (1015) and the case-directory share of `directory-kind-unresolved` (≈1300 of 2006: `🧫️fixtures/🧬️mutations/*/*`,
  `🧬️mutations/*/🧪️tests/*`): **neither repair wave nor taxonomy registration — verifier closure**: a scoped inventory must be closed under
  mutation case pairing (widen to the subset root, report only in-scope findings). In `verifyTaxonomyScopes` this is cheap (scopes of one
  subset share ONE subset inventory and plan) and it removes the same false positives from single `--scope` runs. Then re-run over the
  ticket roots; whatever stays is a real missing counterpart → per-plugin repair by owners.
- Remaining `directory-kind-unresolved` (≈700: snapshot `🧪️tests/*` sqlite dirs 70, `🎮️commands/*`, `📐️brep` 55, `🧑‍💻dev` 103, architect 93,
  tests/examples dirs): decided AFTER the closed re-run with the same HEAD comparison per layout (a layout flagged at scale in pre-ticket
  trees → taxonomy registration with law + fixture; a minority deviation → scripted repair wave after the live e2e).
- Energy registry identity (132 + 132) → S4-TOOLS-B; path-too-long 40 → S4-TEXT (§14) (relayed by the coordinator).

##### Root-script bodies (`fixed-source-disposition-unresolved`, 16)

Validator: `root-script` = command-router grammar (`📚️library/🔍️discovery/🟦️.ts` `ecmaCommandRouterModule`). None existed pre-ticket; 15 were
already rejected at HEAD (10-02). Bisected per file: 4 fail only on `throw Error(…)` (the grammar admits `throw new Error`) → FIXED:
`🌎️hub/🔐️auth/🔌️client/📦️packages/🦀️rust/📜️script.ts`, `🖱️ui/🌐️locale/📜️script.ts`, `🔨️modules/🧪️test/📜️script.ts`,
`🧬️schema/📄️source/🔗️closure/…/📜️script.ts`; `🎠️kernel/🫧️transient/📜️script.ts` failed on the mutating `args.shift()` → destructuring, FIXED.
All 5 transpile and pass the validator. The 11 `✏️s/🧑‍💻dev/*` and `🌐️spatial-kernel/…/🌊️session` scripts carry real logic in the router
(dynamic imports, multi-branch dispatch, `runCmd`/cargo argument assembly) → the logic moves into each composition's own module (a `🟦️.ts`
test/support module the router delegates to in one call); OPEN, next.

##### New dependencies (ratchet)

All three are dev/test-only and peers': `@noble/hashes` ^2.4.0 → `🎒️pack/🌱️value/📦️packages/🦀️rust/package.json` (new 10-04 01:34, value/pack
peer, `devDependencies`); `@webassemblyjs/leb128` 1.13.2 → `🎒️pack/📦️packages/🦀️rust/package.json` (new 10-04 05:07, pack peer,
`devDependencies`); rust `hex` 0.4.3 → stdio bmp `[dev-dependencies]` (10-04 00:25, stdio bmp oracle wave). No runtime dependency; none
ours → listed for their owners.

### M5 — PARKED 2026-10-04 (coordinator usage limit): fault-notice gate widened (F16/F9 partial), exact state

**Landed (bun only, rule 44), consistent and tested:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` region `📢️FaultNotices`:
- `FaultCodeSite` gains `text` (the literal of an anonymous `Fault::from`) and `within` (the innermost enclosing `fn`). A new `rustFnBodies` reads every `fn`, methods and nested ones included, and the `codeFn` reader now uses it.
- `rustFaultHelpers(source, consts)` reads free `fn`s at any depth (methods with `self` are skipped). It returns a `FaultHelper`: either the position of the parameter the code comes from, or a fixed code, for a `-> Fault` fn that builds exactly one fault from a literal or const. Calls written as `.name(` are never helper calls.
- The `🔌️plugin` SDK (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, owner `🔌️plugin`) is read like a plugin. `plugin.internal` (`FAULT_NOTICE_CATCH_ALL`) counts as an anonymous finding. `plugin_sdk_fault` is a fixed-code helper, so each call is one site, and the literal inside the helper's own body is not counted (F16).
- Scope `history-editing` covers `faultNoticeMissing`, `faultNoticeSyntax` and `faultAnonymous` when either of these holds:
  - the code or anonymous text has a tool-flow segment;
  - any site of it is a tool-flow site: a tool path segment, a tool, gesture or **load** `fn`, or a **plugin** source that publishes `ChildEmit` or `Emit::*drag*` (F16).
- The anonymous detail now quotes the text, for example `Fault::from("stdio-wav-audio-tool-mismatch")`, so raw `*-tool-mismatch` codes can be counted (F9). The usage docstring now shows the real route: `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema fault-notices`.
- New schema `🧪️test/🧬️schema/🔣️fault-notices-gate/🔣️.json`. The fixture `🧫️fault-notices-gate/🔣️.json` now carries `$schema`, plus:
  - a case "named consts and refusal helpers";
  - a planted `🧰️tool/✏️editor/🦀️.rs` with a const-keyed table, a helper and a chained `extra_fault_notices`.
- The test `🧪️tests/🧪️fault-notices-gate/🟦️.ts` validates the fixture with Ajv. Its tree-sitter oracle now reads consts and parameter helpers.
- **Run:** `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` → **3 pass / 0 fail (19 expects)**.

**Repo run** (`… schema fault-notices --scope history-editing --json`, `🗑️generated/s4-gates/fault-notices-scoped-8.json`): **526 findings**. It was 22 before the widening; 22 was the run before any F16 change, and it also had trinity anonymous fixed by a peer.

| Class | Count |
| --- | --- |
| Anonymous | 382 |
| Framework | 63 |
| Syntax | 46 |
| Missing | 15 |
| Invalid | 9 |
| Descriptor | 9 |
| Unresolved | 2 |

| Owner | Findings |
| --- | --- |
| SDK `🔌️plugin` | 173 (incl. 70 `plugin.internal` in scope) |
| flow | 82 |
| sequence | 81 |
| stdio | 73 |
| reasoning | 25 |
| wfc | 14 |
| puzzle | 11 |
| hub descriptors | 9 |
| others | 1–5 each |

- All four F16 examples are now caught: `wires-drag-transient-invalid` 1, `wires-drag-offset-non-finite` 1, `flow-retained-direct-route-mismatch` 3, `sequence-node-graph-route` 1.
- `tool-mismatch` appears in 96 findings. This is a finding count, not yet split into distinct raw codes against the audit's 21.

**Next steps (not started; parked):**
1. Split the 96 `tool-mismatch` findings into distinct raw codes per owner and route them (F9).
2. Check the SDK's 63 `faultNoticeFramework` plus 173 total for false positives. The `load` fn pattern may be too wide (for example the `document_load_archive` default refusals). Then decide whether the SDK's framework codes go into `FRAMEWORK_FAULT_NOTICE_LABELS` (framework owner) or get routed to S4-LOAD.
3. F9 "require `fault_notices()`" for raster, wfc, remodel, process, flow and sequence is enforced transitively: an anonymous fault must be named, and a named code must be labelled by a table. Confirm with the census once owners convert.
4. Planted gate-fixture cases still owed for:
   - the SDK `plugin.internal` helper;
   - a `ChildEmit` source;
   - a tool-flow `fn`;
   - text-pattern scope;
   - a fixed-code helper call in `build_tool_job`.
   These are designed in my notes but not yet in the fixture.
5. F3: a gate rule for `x-semio-inverse-rows: {bounded: N}` leaves with no refusal path at the cap (`delete-node` at `bounded: 1025`; the 55 patch leaves at `bounded: 128`). Not started.
6. F14: per-file docstring first-emoji uniqueness gate over touched files (`TM/🦀️.rs` ⏱ x5 and 🛠 x4, playbook root, wires root, `PLG/⏯️tool-run/🦀️.rs`, stdio `🩹️patch/🦀️.rs`). Not started.
7. Still pending from M4:
   - the root-script bodies (11) via owner modules;
   - verifier closure (awaiting the branch decision);
   - re-running the multi-scope law once the nested-cargo catalog seal is repaired;
   - OWED (rules 43/44): plugin crate derived-law cap/cap+1 runs, plugin lib check, cargo-direction layering.

### M6 — Fault-notice scope triage (resumed, bun only; kernel red after a peer's stash/pop, so no cargo)

**False positives removed.** The gate now applies five new scope rules. Each rule has a planted case in the gate fixture.
- **Trait default methods are out of the `history-editing` scope.** They stay in `all`. The SDK's default `PluginApp` archive-load refusals ("…unavailable for this app") are unreachable: the only production implementor, `VcsArtifactApp`, overrides all five (`PLG/🦀️.rs:35359-35456`). `FaultCodeSite.defaulted` is computed by `rustFnBodies` (the innermost enclosing block is a `trait` body). Planted: SDK `PluginApp` default, and plugin `DemoCapability::drag_tool` (tool-flow name and text, but still out of scope).
- **`#[cfg(…test…)]` items are not guest sources.** This covers `cfg(test)`, `cfg(any(test, …))` and an inner `#![cfg(test)]`, but not `cfg(not(test))`. They are dropped in both scopes. `rustTestOnlySpans` drops the SDK `artifact_app_laws` module (`law_load_running`, `law_load_outcome`) and inline test modules. It also drops impls inside them from declaration reading. Planted: SDK laws module, plugin `inline_tests`, and the snippet case "test-only items are not sites".
- **The load-function rule is narrowed to document load.** It now matches only `document|archive|envelope|retained`_load and load_`document|archive|envelope`. `blob_load`, `load_within` and `law_load_*` are no longer tool-flow functions. `time_travel` was added to the function rule, and `timeTravel`, `toolRun` and `toolTransaction` were added to the code rule (`time-travel` to the path rule). Planted: SDK `blob_load` (out), SDK `VcsArtifactApp::begin_document_archive_load` (in), and SDK `apply_time_travel_action` (in).
- **The SDK is the framework's own code.** Every code it raises gets the `faultNoticeFramework` verdict, whatever its syntax (`toolRun.*` and `transaction.*` were reported as `faultNoticeSyntax` before). SDK codes are in scope only under the flow rules, so `plugin.task.quota-exceeded` and similar runtime internals drop out. A plugin's framework-namespace code stays in scope whenever it is live. The SDK declares no notice table: its generic `E::fault_notices()` and `V::fault_notices()` forwarders gave 2 false `faultNoticeUnresolved` findings, and those are gone. Planted: the SDK `EditorApp` forwarder (no finding), and `plugin.task.quota-exceeded` (all-scope only).
- **Fixed-code refusal helpers.** A free `-> Fault` fn that builds exactly one fault from a literal or const (for example `plugin_sdk_fault` with `plugin.internal`, or `refuse_extent`) counts as one site per call, attributed to the caller's enclosing fn. A helper called inside `build_tool_job` therefore puts its code in scope. The catch-all literal inside the helper's own body is not counted. Planted: `refuse_extent()` inside `DemoEditor::build_tool_job` (in), and `route()` outside every flow (out).
- **Kept as true positives, with planted cases:**
  - a `ChildEmit` source (`🌳️children/🦀️.rs`, `demo-child-route`);
  - text match (`demo-command-tool-mismatch`);
  - a tool-flow fn (`demo-route-stale` in `build_tool_job`).
  The flow and sequence editors (77 and 78 anonymous faults) are real refusals of their retained steps; this follows F16's "any `Fault::from` in a file that publishes `ChildEmit`".

**Files changed (bun only):**
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` (`📢️FaultNotices`):
  - `FaultNoticeSubject`, and `faultNoticeInScope(scope, verdict, subject)`;
  - `rustTestOnlySpans`, and `rustFnBodies` with `defaulted`;
  - fixed-code helpers;
  - SDK framework verdict, and SDK declarations skipped.
- The gate fixture now has 2 more snippet cases (8 in all), plus a planted SDK file and planted demo-editor rules. `census` is now one row per owner, with `findings` and `scoped` counts. The schema was updated to match.
- The tree-sitter oracle now reads cfg-test items, fixed-code helpers, `within` and `defaulted`. The test compares the gate's `{code, via, within, defaulted}` against the oracle.

**Runs:**
- `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` → **3 pass / 0 fail (23 expects)**.
- `tsc -p T/🧪️s4-gates-tsconfig.json` (the orchestration module and the gate test) → **exit 0, 0 errors**. I checked that tsc reports errors by running it on a negative probe file.
- Repo `… schema fault-notices --scope history-editing --json` → **448** findings, down from 526 (`🗑️generated/s4-gates/fault-notices-scoped-9.json`).
- Census over all scopes: 135 labelled of 1126 guest codes, 2308 anonymous, 3308 findings.

**Residue per owner (448):** triage from `T/🧪️s4-gates-notice-triage.ts` → `🗑️generated/s4-gates/notice-triage-4.json`.

| Owner | Total | Anonymous | Missing | Syntax | Framework | Descriptor | Route |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 🌊️flow | 82 | 77 | 2 | 3 | | | S4-FLOWCAD |
| 🎬️sequence | 81 | 78 | 2 | 1 | | | coordinator (no owner WP) |
| 🗄️stdio | 73 | 69 | 1 | 3 | | | S4-STDIO |
| 🔌️plugin, flows | 54 | 21 | | | 33 | | **S4-GATES** (staged, see below) |
| 🔌️plugin, load | 52 | 47 | | | 5 | | **S4-LOAD** (F16 `plugin.document-load.*`) |
| 💡️reasoning (wires) | 17 | 16 | | | | 1 | S4-WIRES-MATH |
| 🀄️wfc | 14 | 11 | 3 | | | | S4-STROKES |
| 🧩️puzzle | 12 | 11 | | | | 1 | S4-PUZZLE |
| 🕸️dag | 6 | 2 | 2 | 1 | | 1 | S4-GRAPHS |
| 🌀️procedural | 5 | 4 | | | | 1 | S4-TOOLS-B |
| 🔱️trinity | 5 | 4 | | | | 1 | S4-TEXT |
| 🎞️animate | 4 | 4 | | | | | coordinator |
| 🏗️fem | 4 | 3 | | | | 1 | S4-TOOLS-A |
| 💠️lowpoly | 4 | 4 | | | | | S4-TOOLS-A |
| 🖨️raster | 4 | 3 | | 1 | | | S4-STROKES |
| 🏭️process | 3 | 3 | | | | | S4-STROKES |
| 📜️imperative | 3 | 2 | | 1 | | | coordinator |
| 🧱️block | 3 | 3 | | | | | coordinator |
| ➗️mathematical | 2 | 2 | | | | | S4-WIRES-MATH |
| 🌿️vcs | 2 | 2 | | | | | S4-TEXT |
| 🎪️demonstrator | 2 | 2 | | | | | coordinator |
| 📏️layout | 2 | 2 | | | | | S4-TOOLS-A |
| 📸️remodel | 2 | 1 | 1 | | | | S4-STROKES |
| 🖍️draw | 2 | | 1 | | | 1 | S4-TOOLS-A |
| ✒️writer | 1 | 1 | | | | | S4-TEXT |
| 📐️cad | 1 | 1 | | | | | S4-FLOWCAD |
| 🔋️energy | 1 | | 1 | | | | S4-TOOLS-B |
| 🎥️shooting, 🌍️gis, 🏛️architect, 📕️norm, 🪵️sourcing | 5 | 4 | 1 | | | | coordinator |
| 📖️playbook, 🗒️note | 2 | | | | | 2 | describe owed (coordinator) |

All 9 descriptors are describe-owed (procedural, fem, reasoning, playbook, trinity, dag, draw, note, puzzle). The `Missing` codes are mostly `*.retained.extent` in `build_tool_job` (wfc2d, wfc3d, shooting, remodeling, energy), plus `drawing.canvas.window-required`, `dag.node-graph-edit.*`, `flow.add-widget.child-delta-invalid`, `flow.retained.legacy-dispatch`, `sequence.retained.{config,example}-command` and `wfc3d.node-graph.row`. Each plugin's `fault_notices()` table needs these rows.

**F9: distinct raw `*-tool-mismatch` codes versus the audit's 21.** Source: `T/🧪️s4-gates-tool-mismatch.ts` → `🗑️generated/s4-gates/tool-mismatch-2.txt`. There are **45 distinct raw codes at 96 sites across 14 owners**. The audit counted 21 because it did not read stdio's 52 subset editors, block, puzzle, sourcing or the SDK.
- Converted since the audit, now 0 raw codes: fem 2d/3d, layout, lowpoly, wires, equation, playbook module.
- Remaining by owner:
  - stdio: 20 codes / 71 sites, of which `stdio-example-tool-mismatch` is 52 sites;
  - wfc 4, trinity 3 (jack), puzzle 4, block 3, process 2;
  - 1 each: writer, vcs, animate, demonstrator/playground, cad, norm, raster, sourcing.
- 20 owners already use `app.command.tool-mismatch`; wfc is partial.
- Fix: replace each with `FaultCode::new("app.command.tool-mismatch")`, the framework-labelled code.
- F9's "require `fault_notices()`" for raster, wfc, remodel, process, flow and sequence holds transitively: their unnamed and unlabelled refusals are findings above.

**Decision applied, rule-45 staging** (`🗑️generated/s4-gates/stage-r45/`, generated by `T/🧪️s4-gates-stage-notices.py`):
- **To S4-LOAD:** the SDK document-load refusals.
  - 47 anonymous `plugin_sdk_fault` sites: `PLG/🦀️.rs:26362-26580` (`advance_document_archive_load`, 31), `:26595-26605` (`drive_document_archive_load_retirements`, 2) and `:35359-35446` (the `VcsArtifactApp` begin/poll/cancel/acknowledge/document_load_archive overrides).
  - 5 coded load refusals: `artifact-envelope.stale-handle`, `artifact-envelope.load-stale-handle`, `artifact-store.replacement-stale-handle`, `window-config.owner`, `window-config.load-retirement`.
- **To my framework table, staged:**
  - `framework-notice-rows.json`, `kernel-rows.rs.txt` and `kernel-rows.ts.txt`: 49 en/de rows, appended after `window-transient.kind-unknown` in `FRAMEWORK_FAULT_NOTICE_LABELS`, in its TS twin and in `🧫️framework-notices/🔣️.json`. They cover 32 existing SDK codes (`timeTravel.*` 3, `toolRun.*` 11, `transaction.*` 6, `toolTransaction.shape`, `interactive-job.*` 11) and 17 new ones.
  - The schema pattern widened to `^(app|mutation|plugin|history-filter|window-transient|timeTravel|toolRun|toolTransaction|transaction|interactive-job)(\.…)+$`.
  - `sdk-naming.patch`: names the 21 anonymous SDK flow refusals with those 17 codes, keeping `FaultOrigin::Plugin`:
    - `⏪️time-travel` 4 → `timeTravel.{snapshot-retirement,member-store-kind,preview-mismatch}`;
    - `⏯️tool-run` 7 → `toolRun.{publication-handoff,snapshot-retirement,member-store-kind,member-base-lost,trace-lane,publication-retirement}`;
    - group history / transaction undo-redo-rollback 10 → `transaction.{group-history-dialect,group-history-root,group-history-tail,rollback-unknown,undo-foreign-tail,redo-foreign-tail,undo-failed,redo-failed}`.
  - **Verified on a scratch copy** of the SDK sources (5.7 MB). `patch -p1` applies all 3 files cleanly, and with the rows added to a copy of the framework fixture, `faultNoticeReport(scratch, SDK, "history-editing")` leaves **53**: 47 + 5 load (S4-LOAD) and `ledger-not-replayable`. All 54 SDK flow findings are cleared.
- **Rename owed:** `ledger-not-replayable` (`PLG/🦀️.rs:38373`, `replay_envelopes_fault`) is a single-segment code that fits no table grammar, and is also matched in `🌿️vcs/🦀️.rs` and `🌎️hub/🏗️bootstrap/🦀️.rs`. It needs a namespaced rename, for example `history.ledger-not-replayable`, by the vcs/hub owner (S4-TEXT for vcs).
- **To land the staging once framework saves reopen:**
  1. Apply the 4 kernel parts in one atomic write: Rust rows, TS rows, fixture rows and the schema pattern.
  2. Apply `sdk-naming.patch` (by Edit on its hunks).
  3. Run `cargo check -p semio-framework -p semio-framework-plugin --lib --tests`, the kernel framework-notices Rust and TS tests, the manifest fault-notices test and this gate test.

### M7 — F14: per-file docstring first-emoji uniqueness as a gate rule

**Rule and implementation.** The rule is AGENTS.md "start all docstrings with a unique … emoji", held per file (design §21.2).
- **Library**, in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`:
  - `docstringOpenersOfText` (🧷️) is the single opener enumeration, shared with the `docstrings` census.
    - A Rust run now ends where its doc kind changes, so a `//!` module doc followed by an item's `///` is two docstrings.
    - Before, they were one run, which also hid the item doc from `emoji-first`.
  - `docstringEmojiReuseOfText` and `DocstringEmojiReuse` report every marker grapheme that opens more than one docstring. The variation selector is ignored and ZWJ sequences count as one marker.
  - `runDocstringEmojiCensus(repoRoot, signal, onProgress, paths?)` runs over every tracked source, or over a list.
  - `docstringSources` is now shared with `runDocstringCensus`.
- **Gate:** root `📜️script.ts` `verify docstrings emoji-unique [--files-from <list>]` (`runDocstringEmojiGate`).
  - It prints the reusing files with their repeat counts, most first, and exits non-zero on any repeat.
  - Without a list it publishes the acceptance record `docstring-emoji-unique`. That record is registered in the goal plan (`🎯️acceptance/🎚️config/🔣️.json`, criterion 5.11, beside `docstring-emoji-first`).
  - Launch row `⚖️gate🧬️docstring-emoji-unique` (`bun nx run workspace:verify -- docstrings emoji-unique`, `4_gate` 411.355) is in `.vscode/🧩️launch.seed.jsonc`.
- **Law:** `🎯️acceptance/🧪️tests/🧮️source-census/🟦️.ts` "docstring emoji reuse", 4 cases in `🧫️fixtures/🧮️source-census/🔣️.json` → `docstringEmojiReuse`:
  - Rust runs and blocks with the variation selector ignored;
  - TS one-line and multi-line blocks, symbol glyphs, a `/**` inside a string, and line comments;
  - a ZWJ marker distinct from its first part;
  - unique emojis and the `@emoji` residue.

  Each case is checked against the hand-written expectation and against an independent **tree-sitter oracle** (Rust, TypeScript, TSX comment nodes, so string contents never count).

**Runs:**

| Command | Result |
| --- | --- |
| `bun test ./…/🎯️acceptance/🧪️tests/🧮️source-census/🟦️.ts` | **30 pass / 0 fail (46 expects)**; the 26 existing tests stay green |
| `bun test ./…/🎯️acceptance/🧪️tests/🎯️goal-gate/🟦️.ts` | **28 pass / 0 fail** |
| `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` | **3 pass / 0 fail** |
| `tsc` on both orchestration modules and both tests (`T/🧪️s4-gates-tsconfig.json`) | **exit 0** |
| `tsc` on root `📜️script.ts` (`🗑️generated/s4-gates/tsconfig-root-script.json`) | **exit 0**; `--listFiles` confirms the script and the acceptance module are in the program |

**Counts.**
- **Ticket-touched files** (`🧪️s3-gates-ticket-files.py` → 1914 Rust/TS files; `bun ./📜️script.ts verify docstrings emoji-unique --files-from 🗑️generated/s3-gates/ticket-files.txt` → `🗑️generated/s4-gates/emoji-unique-touched-2.txt`): **488 files reuse an emoji, 15203 repeats.** Top files:

  | File | Repeats |
  | --- | ---: |
  | `OS/🔌️plugin/🦀️.rs` | 1669 |
  | wgpu `🐚️Shell` | 1155 |
  | `OS/🏪️store/🦀️.rs` | 829 |
  | `FW/🛂️manifest/🦀️.rs` | 510 |
  | root `📜️script.ts` | 427 |
  | `📚️library/🔍️discovery` | 395 |
  | `REPO/🧪️test/🟦️.ts` | 337 |
  | `ShellHost` | 266 |
  | `ShellHelpers` | 254 |
  | `♾️infinite/🌍️world` | 250 |
  | `FW/🎠️kernel/🟦️.ts` | 210 |
  | `🌎️hub/🏗️bootstrap` | 201 |
  | `World3dHost` | 200 |
  | `🌉️mcp/🏠️workspace` | 192 |
  | `💻️os/🟦️.ts` | 189 |
  | `🔌️plugin/🖥️host` | 187 |
  | `🌊️flow/🖥️host` | 184 |
  | `⏪️time-travel` | 142 |
  | puzzle3d editor | 141 |

  By area (top-200 rows): renderer 3122, plugin SDK 2378, store 1158, manifest 616, library 613, infinite 611, ui 577.
- **Whole tree** (acceptance record): 41951 repeats in 7744 of 34398 sources. `docstring-emoji-unique` is FAIL, the same standing as `docstring-emoji-first`.
- **F14's named files** (`--files-from 🗑️generated/s4-gates/f14-files.txt`):

  | File | Repeats | Detail |
  | --- | ---: | --- |
  | `PLG/⏯️tool-run/🦀️.rs` | **69** | 🧹×8, 🎯×8, 🔁×7, … |
  | `TM/🦀️.rs` (`🛠️tool-machine`) | **67** | 🛠×4 `:1,976,996,1015`, 🧾×6, 💾×5, … |
  | flow editor | **21** | 🕹×10 `:139-184,2278,2309`; the audit called it clean, but it judged added lines only |
  | stdio `🩹️patch/🦀️.rs` | **20** | 📍×3 `:75,249,993`, ✂×3, 🧭×3, 🧬×3, … |
  | playbook root (`🔖️1/🦀️.rs`) | **0** | fixed by its owner |
  | wires root | **0** | fixed by its owner |
  | flow root | **0** | |

  Owners fix these, per the audit routing: tool-run and tool-machine → framework/S4-TOOLS-A, flow → S4-FLOWCAD, stdio → S4-STDIO.
- **Fixed in the files I touched this session**, so the docstrings I own are unique:
  - the `📢️FaultNotices` region of `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`: 15 openers re-emojied; the file went from 63 to 48 repeats, and the remainder is in other regions;
  - the docstring-census block of `🎯️acceptance/📋️orchestration/🟦️.ts`: 5 openers; 29 → 24;
  - the root gate (❄️).

  The test files I created (fault-notices gate, source census, kernel framework-notices, multi-scope law) have 0 repeats.
