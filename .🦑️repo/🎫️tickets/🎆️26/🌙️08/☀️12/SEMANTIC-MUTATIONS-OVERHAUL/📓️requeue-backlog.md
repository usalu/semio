# Requeue backlog — open items before this ticket can close

Accumulated from lane reports. Nothing here is lost work; each item names its source and what
must happen. Ordered by blocking-ness.

## A. Coordinator rulings owed (mutation vocabulary is this ticket's to decide)

### A0. RULED — app COMMAND names are not mutation vocabulary and are out of scope

Raised independently by the remodel lane (`Set*`-named command structs in 3 `🎮️commands/` files,
"host wire surface with pinned ordinals, not mutations") and surfaced by the norm lane, whose apps
still contain `En1995Command::SetSnapshot(set_snapshot::ReplaceSnapshot { … })`.

**Ruling: the ban applies to mutation vocabulary, not to app command names.**

Rationale: the point of the taxonomy is that *mutation* intent survives into the diff, the
inverse, and the history — so undo labels and merges can recover what the user meant. An app
command is the host↔plugin wire surface; its name never enters history, and several carry pinned
ordinals that a rename would break. Renaming `setSnapshot` to something else buys no semantic
recovery and costs wire compatibility.

What *is* still banned in an app command is **constructing a banned mutation**. A command called
`setSnapshot` is fine when it routes to `ArtifactStore::reset` or decomposes into semantic
mutations; it is a violation only if it builds a whole-document mutation variant.

Verified for norm rather than taken on trust: `📘️en1995`'s dispatch enum contains only `Change*`
variants (`ChangeAnnex`, `ChangeMEdKnm`, …) and has no `set-snapshot` triad dir at all, so the
`SetSnapshot` *command* structurally cannot construct a whole-snapshot mutation — it must go
through the lane's `from_snapshot` decomposition. Same check must be applied to the other 14 norm
apps and to remodel's 3 command files before their hits are dismissed.

**Consequence for Wave R3**: the vocabulary policy rule currently greps raw tokens across
`🎮️commands/**`, which will produce false positives on legitimate command names. Refine it to
flag *constructions of banned mutation variants* rather than bare identifiers, or scope it to
`🧬️mutations/**` and let call-site correctness be enforced by the compiler (a banned variant that
no longer exists cannot be constructed).

### A1. remodel `replace-tracks` — the lane flagged this as worth challenging, and it was right

Source: `📓️waveM-reports/remodel-report.md`. `SetTracks` was kept as a single `replace-tracks`.
The lane's own evidence: tracks carry ids but have **zero per-track gestures**; the only writers
are a whole-run engine re-derivation and a `clearTracks` command.

**Ruling — split it in two, and neither half is `replace-tracks`:**
1. **The `clearTracks` gesture is `clear-tracks`.** `clear` is in `APPROVED_VERBS` ("empty a
   collection/field wholesale"), and its inverse is defined: re-`create`/`add` every captured
   member from `base`. That is a real user gesture and gets real vocabulary.
2. **The whole-run engine re-derivation is not a mutation at all.** If tracks are recomputed from
   the run rather than authored, they are a *derived* value and belong in an `💡️inferences`
   facet — the same call APA and this session made for lowpoly's texture cache, and the same call
   DKM made for tessellation/measure/validate on brep. Authoring mutation vocabulary for a
   derived value means minting a diff and an inverse describing a cache fill.

A surviving `replace-tracks` would be a whole-collection setter, which the taxonomy forbids
outright. Requeue: confirm whether tracks are engine-derived; if yes, remove `replace-tracks` and
route re-derivation to an inference; keep `clear-tracks`.

### A2. remodel's two arguable `update` verbs
`update-camera-calibration` and `update-rig-extrinsic` are written up individually in the report.
Audit both against the `update` test: ≥2 fields, genuinely inseparable, never meaningfully set one
at a time. Three other sessions reached for `update` wrongly today; these two deserve the same
scrutiny before they stand.

## A3. Structural audit of all 54 non-stdio facets (coordinator-run, no cargo needed)

Checked the dispatch-coverage invariant directly — triad-dir set vs dispatch-variant set — across
every non-stdio facet. **Exactly 2 fail**, both leftovers from earlier waves whose agents could not
edit `📦️glue.rs`, so the enum was modernized while directories kept old names:

| facet | dirs | variants | nature |
|---|---|---|---|
| `🌀️procedural/🌀️procedural2d` | 8 | 14 | severe: every dir still `🎛set-*`/`➖remove-*` while variants are `CreateWidget`/`ConnectSynapse`/… ; 6 variants have no dir at all. Payloads are split between old dirs (`🎛set-widget/🦠️mutation` holds `CreateWidget`) and inline in the dispatch file (`🦀️component.rs:182` holds `CreateGeneration`) |
| `📋️forms/📋️forms` | 9 | 10 | **corrected on closer inspection — NOT "one missing dir"**. Same full drift as procedural2d: zero overlap between dir names and variant names |

**Correction to the forms entry** (first reading said "one variant missing its triad dir" — wrong,
and worth recording because the count alone was misleading):

- dirs, all OLD/playbook-shaped: `↔️move-block ↔️move-step ➕add-block ➕add-step ➖remove-block
  ➖remove-step 📖update-playbook 🩹update-block 🩹update-step`
- variants, all NEW/semantic: `CreateStep DeleteStep ReorderStep RenameStep ChangeStepDescription
  CreateBlock DeleteBlock MoveBlockToStep ReplaceBlock ChangeFormTitle`

Not a single name matches. The dirs are inherited from the playbook vocabulary (forms re-exports
playbook domain types), so this facet needs a complete 10-dir restructure, not a patch. A near-9
vs 10 count concealed a total mismatch — **when auditing this invariant, compare the name sets, not
the cardinalities.**

### A3-plan: procedural2d — complete dir↔payload mapping (analysis DONE, execution pending)

Measured, so the executing lane does not have to re-derive it. Every one of the 8 existing dirs
holds exactly one payload struct, correctly written but in a wrongly-named directory; the other 6
payloads are inlined in the dispatch file and need extracting into new dirs.

**Rename (8)** — `<current dir>` → `<target dir>` (payload struct it holds):

| current | target | payload |
|---|---|---|
| `🎛set-widget` | `🌱create-widget` | `CreateWidget` |
| `➖remove-widget` | `🗑️delete-widget` | `DeleteWidget` |
| `🎛set-layout` | `📍move-widget` | `MoveWidget` |
| `➖remove-layout` | `🧹clear-widget-layout` | `ClearWidgetLayout` |
| `🎛set-synapse` | `🔗connect-synapse` | `ConnectSynapse` |
| `➖remove-synapse` | `✂️disconnect-synapse` | `DisconnectSynapse` |
| `🎛set-camera` | *(pending verb ruling — see below)* | `UpdateCamera` |
| `🎛set-schema` | `🔤change-schema` | `ChangeSchema` |

**Extract from the dispatch file into new dirs (6)**: `ReplaceWidget` → `🔁replace-widget`;
`ReplaceSynapse` → `🔄replace-synapse`; `CreateGeneration` → `➕create-generation`;
`DeleteGeneration` → `➖delete-generation`; `RenameGeneration` → `🏷️rename-generation`;
`ChangeGenerationValue` → `🔢change-generation-value`.

Emoji above are pre-checked pairwise-distinct within the facet. Every rename must be mirrored in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`, and the dispatch file's
`use super::{…}` list (currently naming the 8 old module names) updated to match.

**Two verb rulings still owed before executing:**
- `UpdateCamera` — `update` is reserved for an inseparable ≥2-field facet never set one field at a
  time. Measure the camera type: if position/target/zoom are set independently this is
  `move-camera` + `change-camera-<field>` (the taxonomy has exact spatial verbs), and only a
  genuinely atomic camera facet justifies keeping `update`. Three sessions have now reached for
  `update` wrongly today.
- `ClearWidgetLayout` — `clear` is approved ("empty a collection/field wholesale") but its inverse
  must restore **every** captured member from `base`. Verify the existing inverse does that rather
  than restoring a single entry.

### Combined size of A3, and why it was NOT attempted by hand

procedural2d (14 variants) + forms (10 variants) = **24 triad dirs × 3 leaves ≈ 72 files**, plus
two `📦️glue.rs` rewires, plus extracting payload structs that currently live in old dirs and
inline in dispatch files. The assigned lane died on the session limit before starting, and the
coordinator deliberately did **not** begin it by hand: a half-restructured facet with dangling glue
mounts is strictly worse than a fully-documented one, and this ticket has already spent effort
today repairing exactly that failure mode in four other plugins.

Requeue as one lane, procedural2d first (it has the inline-payload complication), with the standing
rule that both facets must end at triad-dir set ≡ variant set with unique emoji and real glue
mounts.

Requeued (the assigned lane died on the session limit before starting). Also check
`🌀️procedural/🧊️procedural3d` — it passed the audit, but confirm rather than assume. Two
vocabulary items to rule on while there: `UpdateCamera` (likely `move-camera`/`change-camera-*` —
`update` needs inseparable fields) and `ClearWidgetLayout` (`clear` is approved, but verify the
inverse restores every captured member from `base`).

**Audit methodology note**: the first run reported 6 mismatches. Four were false positives — the
variant regex counted `Some(` in match arms. Corrected by filtering `Some|None|Ok|Err|Box|Vec|String`.
Recording because the same trap will catch anyone re-running this check.

## A4. FIXED by the coordinator: sequence's dead engine + stale test

`🎬️sequence/…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` held the last real (non-prose)
`CollectionMutation` code outside the flow kernel bridge:
- `steps_delta_from_collection_mutation`, `edges_delta_from_collection_mutation` and
  `diff_set_snapshot` — all three with **zero callers** in the plugin, the same dead-engine class
  removed from gis/flow/animate/process during Wave R. Whole `🔖️Helpers` region deleted, import
  narrowed to `use protocol::{MutationDiff, Patchable};`.
- Its test constructed `SequenceMutation::StepsAdd { index, item }` — a variant that **no longer
  exists** (sequence's dispatch is 8 semantic variants, 1:1 with its dirs). Rewritten to use the
  `create_step(step)` builder.

**This matters beyond the fix**: a stale test referencing a deleted variant means sequence's test
build could not have compiled, yet an earlier `cargo check --workspace --all-targets` reported zero
plugin errors. That run must have stopped at the `semio-framework-os-kernel` lib-test failure
before building plugin test targets. **Treat that "zero plugin errors" reading as unproven** and
re-run once the framework blockers clear.

## B. Verification gaps (block the ticket's exit criteria)

### B0. ⛔️ ALL CARGO EVIDENCE IN THIS SESSION'S LATER WINDOW IS UNRELIABLE — DISK FULL

`/System/Volumes/Data` is at **100%** (862Gi used of 926Gi, **2.8Gi free**). Root
`/Users/ueli/Documents/semio/target` is **428G** — pure regenerable build cache, and stale under
this repo's per-ticket `CARGO_TARGET_DIR` policy.

A full disk makes `rustc` fail while writing `rmeta`/link artifacts, and those failures surface as
*plausible but bogus* compile errors — missing crates, unresolved modules, missing manifests. So
several "blockers" circulating between sessions are probably artifacts, including **one of this
ticket's own findings**:

- **B1's "144 errors, `tempfile` not a dev-dependency" is RETRACTED.** `tempfile = "3.20.0"` is
  present at `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`, under
  `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]`, with a comment naming
  `🏪️store/🔄️sync`'s actor tests as the reason. DKM independently compiled and ran that crate's
  lib-test binary at ~17:35 (11 engine tests, 0 failed) — impossible if store/sync carried 144
  errors. The measurement was wrong, stale, or disk-induced.
- Similarly, DKM reported `✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust/Cargo.toml` as deleted,
  breaking workspace manifest load. **It exists on disk right now**, along with the whole
  `🔄️fsm/` subtree.

**Consequence for this ticket**: every gate, baseline and release claim resting on a build from
this window must be **re-run before it is trusted** — including `cargo check --workspace → 0
errors`, and the per-plugin test results (raster 66/0, gis 171/0, shooting 104/104) if they fall
inside it. Structural evidence (directory/variant audits, banned-token greps, file reads) is
unaffected and stands.

**Not actionable by this ticket**: deleting 428G of shared build cache is destructive, affects six
concurrent sessions mid-build, and costs everyone a cold rebuild. Escalated to the user for a
decision; no session should do it unilaterally.

### B1. Framework law tests cannot build — RETRACTED, see B0
`cargo check --workspace --all-targets` → `semio-framework-os-kernel` **lib test** fails with 144
errors, all in `🔨️modules/🏪️store/🔄️sync/🦀️component.rs`: `tempfile` is used but is not a
dev-dependency of the os-kernel crate, plus `DemoSnapshot`/`DemoMutation` fixtures failing
`ArtifactPack`/`OpText`/`OpBinary` bounds. Plus 1 error in `🧠️neural` (`Schema` has no field
`extension`).

Evidence it is not this ticket's: the `🔄️sync` module predates the ticket (commits 492/480/467),
this ticket never touches it, and this ticket is barred from editing `Cargo.toml`.

**Why it matters**: this blocks the framework's own `MiniMutation` fixture and the testkit law
helpers (`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`,
`assert_diff_algebra_*_law`) — the mechanism's correctness argument. Broadcast to peers; owner
must add the dev-dependency.

### B2. Per-plugin law tests not yet run
`cargo check --workspace --all-targets` shows **zero plugin errors**, so plugin test code compiles.
Actual `cargo test` runs are queued behind heavy machine contention (80+ cargo processes across
five concurrent sessions). Confirmed so far: `🖨️raster` 66/0, `🌍️gis` 171/0.

### B3. `assert_op_text_binary_equivalence` sweep never run
Multiple lanes skipped it. Needs a pass once B1/B2 clear.

### B4. `impl DiffAlgebra` missing on several artifact diffs
Explicitly noted for `RemodelDiff`; likely others. Required before the final ratchet tightens
`Mutation::Diff` to `MutationDiff<P> + DiffAlgebra<P>`.

## C. Known-incomplete lane work

| item | source | state |
|---|---|---|
| `🧱️block` 3d/5d marked `partial`, never compiled by their lane | block reports | workspace check now shows 0 plugin errors, so they compile; law tests unrun |
| `🕸️dag` test build | was blocked by the panels rename (now fixed by another session) | re-run |
| `🔱️trinity`/`♻️rewrite` — zero compile verification by its lane | trinity report | workspace check clean; law tests unrun |
| `📖️playbook` — no clean confirming run | playbook report | workspace check clean; law tests unrun |
| stale `📡️component.protocol.semio` / `📖️component.grammar.semio` | remodel, block, others | Wave B honesty sweep |
| `Set*`-named app **command** structs in 3 remodel `🎮️commands/` files | remodel report | these are host wire surface with pinned ordinals, NOT mutations — decide whether they are in scope at all |
| schema description files left generic in `graphql`/`json`/`proto` | block reports | Wave B honesty sweep |

## D. Deferred by cross-ticket agreement

- **stdio, 53 facets** — behind UCAS's roster restructure; unstarted, brief ready.
  Includes the approved `set-primitive-geometry` → `replace-primitive-geometry` rename in
  `✳️mesh` (approved for DKM, absorbed into this lane to keep third parties out of stdio).
- **`🧿️semio ✳️any`** — 18-way union dispatch, migrate last, after all sub-subsets.
- **Framework kernel bridges** — `🌊️flow/🌿️vcs` (40 `CollectionMutation` hits) taken by DKM
  (#2550); `🪐️space` module still unowned. Both are the hard floor preventing full
  `CollectionMutation` elimination from the plugin side.

## E. Policy + ratchet (Waves R3/B), not started

`📜️script.ts` write slot is queued APA → UCAS-W6 → SMO → inference-family. Contents unchanged
from the plan: repoint 4 wrong-depth rules, extend ts-mirror to flag MISSING mirrors, widen the
vocabulary scan beyond `🧬️mutations`/`🎮️commands`, prune stale allowlist entries, add
grammar-coverage + DiffAlgebra-scope rules. Then the staged ratchet.
