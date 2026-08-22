# P8z Independent Global Payload-Authority Audit

## Verdict

**REJECT — P0.** The repair removed the specifically named process-global
registries, and several data-model migrations are coherent at source level. It
does not, however, provide the promised operation-owned resumability, exact
pre-deserialization protection, or collision/ABA-safe CAD freshness. Those are
required properties of this repair, not optional runtime polish.

No production or ticket metadata was changed by this audit.

## Audit Basis

Read in full before inspecting the current worktree:

- `AGENTS.md`.
- `📓️p8t-independent-remaining-tools-global-audit.md`.
- `📓️p8w-global-payload-authority-repair.md`.

Read-only source inspection then covered CAD, Block3d, Process3d, Sourcing,
Note, Layout, Puzzle3d and Puzzle5d, including their changed Rust/schema/text,
binary, GraphQL, JSON, Proto and TypeScript surfaces where present.

## P0 Findings

| ID | Finding | Current source evidence | Required repair |
| --- | --- | --- | --- |
| P0-01 | CAD’s replacement for the thread-local preview sequence is a 64-bit content hash, not a generation. It cannot be collision-safe or ABA-safe and is not monotonic. | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:902-906` returns `DefaultHasher::finish()` over `engagement_session_json`. The payload changes `A → B → A` produce the same value; distinct values also share a finite 64-bit output space. The changed test still asserts a monotonic ordering at `:2141-2149`, which arbitrary hashes do not possess. | Persist an exact, increment-only engagement preview generation in the authoritative config/operation checkpoint. Increment on each relevant transition; freshness must compare an operation identity plus generation, never a content hash. Add equality, ABA, collision-injection, restart and two-app tests. |
| P0-02 | Puzzle3d’s isolated fill worker has no authoritative scene/session/operation state to resume. Every actual worker invocation creates an empty session and therefore faults as stale on its first tick. | `…/🧩️puzzle/🗿️artifacts/🧊️3d/…/⏳️precompute/🦀️component.rs:899-910` decodes only `{ job, operation, generation }`, creates `Puzzle3dPrecomputeSession::new()`, optionally calls `restore_fill_job`, and immediately calls `drive_fill_job`. `restore_fill_job` requires both a populated `fill_job` and `engine.fill` at `:844-850`; a new session has neither. Likewise cold restore requires an already configured scene plus fill at `:768-783`. The `restored` boolean is discarded at `:903-905`. | The worker input/checkpoint must carry enough snapshot-owned operation state to reconstruct the fill session before stepping (or obtain it through a documented operation-scoped authority). Make restoration fail closed and observable. Add an actual cold-worker/checkpoint-restart/two-document test; ensure the operation and generation used by the worker are persisted, not app-local only. |
| P0-03 | Sourcing’s asserted pre-deserialization cardinality and leaf caps run after an unrestricted generic JSON deserialization/allocation. | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/…/🎮️commands/📄️set-artifact-json/🦀️component.rs:35-44` only checks raw bytes before `serde_json::from_str::<Value>` at `:38`; `validate_json_value` runs at `:39` after the complete JSON tree exists. Thus a depth/cardinality/leaf workload is not rejected pre-deserialization as P8w claims. | Replace it with a bounded streaming/token envelope validator that caps depth, entries, keys and string bytes before materializing a generic `Value` or `CurateSnapshot`; prove exact max/+1 cases. |

## Additional Blocking-Risk Evidence

### Unbounded contribution decoding and permanent payload retention

Process and Sourcing removed their mutex registries, but their replacement
re-parses contribution payloads synchronously for each caller and converts
document/configuration-derived labels and identifiers into non-reclaimable
`'static` allocations.

- Process: `…/🏭️process/…/✏️editor/🦀️component.rs:632-633` is
  `Box::leak`; its `ContributedMachineCatalog` exposes three `&'static str`
  fields at `:641-645`; each contribution parse executes an unbounded
  `serde_json::from_str` at `:688-700`.
- Sourcing: `…/🪵️sourcing/…/🧬️schema/🦀️component.rs:519-520` is the same
  `Box::leak`; the contributed module holds `&'static str` at `:527-532` and
  deserializes `typologyJson`/`kindsJson` without raw or decoded bounds at
  `:580-597`.

This is no longer a named `Mutex`/TLS registry, but it permits unbounded
process-lifetime retention of input-derived payload and unbounded synchronous
decode on render/action lookup. It prevents credit for the claimed 8-ms,
restart-safe, multi-document path. Use owned `String` at the domain boundary or
an owned interface that does not demand `&'static str`; bound and validate the
contribution envelope before deserializing.

### Stale and contradictory comments

The Note and Layout values themselves are now snapshot-owned, which is good,
but their comments still describe a scratch-cache authority that no longer
exists. Examples: Note `…/🗒️note/…/🦀️component.rs:342-351`; Layout
`…/📏️layout/…/🦀️component.rs:66-68`. Correcting them is not the P0, but
leaving them creates a false lifecycle contract for future callers.

## What The Source Does Establish

- The exact scan for the removed names (`PUZZLE3D_MESH_REGISTRY`,
  `CAD_PREVIEW_SEQ`, Process/Sourcing contribution mutex identifiers,
  `VORTEX_KIND_SCRATCH`, Note/Layout scratch identifiers) returned no matches
  in the seven assigned plugin trees.
- Block3d stores `vortex_kind_extra` in its snapshot (`…/block/…/📸️snapshot/🦀️component.rs:27-35`) rather than a thread-local catalog.
- Note’s `NoteTextChild` carries durable `paragraphs` alongside the composed
  handle (`…/note/…/🦀️component.rs:259-267`); `note_block_text` reads the
  record itself (`:342-346`).
- Layout’s `LayoutDrawingChild` carries the full drawing content (`…/layout/…/🦀️component.rs:42-47`) and the accessor reads the snapshot slot (`:66-70`).
- Process’s `stock_payload` and `step_payloads` appear consistently in Rust
  snapshot/diff structures, text and binary codecs, and the inspected Proto
  surfaces (e.g. `…/process3d/…/📸️snapshot/🦀️component.rs:247-272`,
  `…/📸️snapshot/🛰️component.proto:14-20`; diff equivalents retain matching
  shapes). The changed JSON descriptors parse.

These positives are insufficient to override P0-01 through P0-03.

## Commands And Results

All commands were read-only/static; no Cargo, build, runtime, Wasm, cache, git
mutation or ticket-metadata operation was run.

```text
rg -n -e <all removed registry identifiers> <seven plugin trees>
=> no matches (rg exit 1)

bun -e '<JSON.parse every changed descriptor>' -- <changed JSON files>
=> parsed 7 changed JSON descriptors

git diff --check -- <seven plugin trees>
=> exit 0

bun ./📜️script.ts verify interactivity
=> exit 0 (its declared 4 UI roots only)

bun ./📜️script.ts verify interactivity tool-jobs --format json
=> exit 1; 0 bounded rows, 875 remaining command dispositions and 12
   framework-reserved pending factories. This broad P8 gate remains red and
   does not validate this repair's runtime semantics.
```

## Required Gates After Repair

- Native and release compilation/type/borrow/Send checks for every changed
  plugin and the real job bridge.
- Actual isolated-worker execution: first fill slice, progress/checkpoint,
  cancellation, cold restart and two-document/two-app isolation.
- CAD preview sequence monotonicity, stale rejection, ABA and forced-hash
  collision tests using a persisted operation generation.
- Exact max/+1 raw, depth, cardinality and leaf tests for Sourcing imports and
  contribution payloads before any general JSON tree is materialized.
- Schema descriptor discovery and Wasm compilation/execution after the source
  gates are fixed.

Do not promote P8w’s cohort to PASS until these P0 paths are corrected and a
fresh independent audit plus the unrun gates pass.
