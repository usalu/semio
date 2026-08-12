# Requeue backlog — open items before this ticket can close

Accumulated from lane reports. Nothing here is lost work; each item names its source and what
must happen. Ordered by blocking-ness.

## A-STAGED: the flow bridge deletion surface (blocked on DKM, then mechanical)

The last real (non-prose) `CollectionMutation` code in this ticket's entire plugin scope. Deletion
is **gated on DKM's semantic framework enum landing** — not on their variant list being final.

Reason, measured rather than assumed: the bridge is not a thin wrapper that can be unwrapped early.
`FlowMutation`'s **entire wire codec is the bridge**:

| site | role |
|---|---|
| `…/🧬️mutations/🦀️component.rs:116` | `OpBinary::encode_op` → `to_framework_mutation` |
| `…/🧬️mutations/🦀️component.rs:120` | `OpBinary::decode_op` → `from_framework_mutation` |
| `…/🧬️mutations/🦀️component.rs:130` | `OpText::parse_op` → `from_framework_mutation` |
| `…/🧬️mutations/🦀️component.rs:138` | `OpText::print_op` → `to_framework_mutation` |
| `…/⚙️engine/🦀️component.rs:144` | `filter_map(from_framework_mutation)` |
| `…/⚙️engine/🦀️component.rs:155` | `filter_map(from_framework_mutation)` |

Deleting the conversions today would leave flow with **no wire codec at all**, against a framework
enum still in its generic shape. **Consequence of the no-bridge ruling that must be stated:** when
the conversion goes, flow's `OpText`/`OpBinary` must be *rewritten* — derived from the new semantic
framework enum, or handcrafted per variant as space does — not merely unwrapped.

(Both `decode_op` and `parse_op` already carry the correct error for `set-fixture`: *"whole-document
replace is banned; route through `ArtifactStore::reset`"* — evidence the locked no-import decision
is already enforced at the wire boundary.)

## 🔴 THE LAW HARNESS RAN AND FOUND 6 REAL FAILURES IN MIGRATED FACETS

First time this ticket's triad law tests have actually executed. `cargo test -p semio-s-plugin-puzzle
--lib` → **443 passed, 6 failed**. These are genuine defects in this ticket's own migration work,
not churn, and they are exactly the class the harness exists to catch:

| failing test | facet | class |
|---|---|---|
| `delete_node_severs_and_reconnects_edges` | puzzle2d | delete-cascade inverse |
| `connect_disconnect_vortices_inverse_law_and_cascade` | puzzle3d | connect/disconnect inverse + cascade |
| `connect_disconnect_grips_inverse_law_and_cascade` | puzzle5d | connect/disconnect inverse + cascade |
| `puzzle2d_delta_ops_are_granular_and_round_trip` | puzzle2d | delta granularity / round-trip |
| `puzzle3d_delta_ops_round_trip_and_stay_granular` | puzzle3d | delta granularity / round-trip |
| `puzzle5d_delta_ops_round_trip_and_stay_granular` | puzzle5d | delta granularity / round-trip |

Two failure families, both predicted by the taxonomy as the hard parts:
1. **Cascade inverses.** `📓️taxonomy.md`'s addressing convention requires `delete`/`disconnect` to
   capture the full removed payload *plus the severed cascade*, re-`connect`ed after `create` in
   reverse dependency order. All three puzzle facets fail this, which suggests the cascade is
   either not captured or not replayed in the right order.
2. **Delta granularity.** The diff must be sparse and built directly from `(payload, base)`; a test
   named "stay granular" failing implies a diff is asserting more than the mutation touched — the
   same over-broad-diff defect that makes concurrent merges impossible (see the mergeability
   argument recorded under the space rulings).

**Not yet fixed**: re-running to get assertion detail is currently blocked because
`semio-s-plugin-stdio` went red again (UCAS's in-flight `✳️table` subset — 4 unresolved
`include_str!` targets), and puzzle depends on it. The 443/6 result above was obtained while stdio
was still cached.

**This is the single most important open item in the ticket.** Every other facet's law tests are
still unrun, so the true failure count across 54 facets is unknown and 6 is a floor, not a total.

### Prerequisite discovered and fixed: 28 facets could not compile their law tests at all

`X::kinds()` is a `protocol::SemanticMutation` trait method, so calling it requires the trait in
scope. 28 facets called it in their `#[cfg(test)]` modules **without importing the trait**, so those
test binaries never built — which is why the law harness had never run anywhere. Added
`use protocol::SemanticMutation;` to all 28 (trinity×2, raster, process, norm×7, block×3, dag,
sequence, space, procedural3d, imperative, sourcing, note, puzzle×3, fem×2, playbook, energy).

`📐️cad` was skipped — its test module has neither anchor pattern and needs manual placement.

## ⚠️ RELEASE CLAIMS FROM THE UNTRUSTWORTHY WINDOW MUST BE RE-ESTABLISHED

`🖨️raster` was recorded RELEASED in `📓️plugin-release-status.md` with "cargo test 66 passed / 0
failed". On re-run after the disk cleared it **did not compile**: `📸️snapshot/📝️text/🦀️component.rs`
used `include_str!("../../../../📚️examples/…")` — four levels up, where the asset sits three up at
`🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`. Fixed (one `../` removed);
`cargo test -p semio-s-plugin-raster --lib` → **66 passed / 0 failed** confirmed.

The recorded result was *true when recorded* and had gone stale. **A release ledger built from past
builds is a derived artifact, not a live predicate** — the same trap this session spent the day
warning peers about, this time inside its own file. Every plugin released during the disk-full or
`semio-framework-plugin`-red window is being re-verified rather than assumed.

Note for whoever audits further: the `../` depth for this include is **inconsistent repo-wide** —
16 files use three levels from that position, 5 use four. Worth a sweep; it is exactly the
compiler-only failure class below.

### The ceiling on structural verification (APA's formulation, adopted)

> **Structural checks tell you the shape is right; only the compiler tells you the shape is true.**

raster is the proof: triad dirs 1:1 with variants, unique emoji, every `#[path]` mount resolving,
banned-token sweep clean — all true, and the plugin was broken, because an `include_str!` target is
resolved by the compiler and nothing else. Structural checks caught real breakage today (dangling
mounts, name-set drift) and remain worth running; they simply cannot close this ticket alone.

### The sccache false-clean trap (cost this session one run, and nearly cost APA a whole sweep)

`.cargo/config.toml:2` sets `rustc-wrapper = "sccache"` repo-wide, and sccache currently fails with
`Operation not permitted`. When it does, **cargo exits 0** and emits only:
```
error: process didn't exit successfully: `sccache … rustc -vV` (exit status: 2)
sccache: error: Operation not permitted (os error 1)
```
No crate is compiled and none is named, so any error-count that greps for file paths returns zero
and reads as a clean run. **Always run verification with `RUSTC_WRAPPER=""`**, and confirm a
`Finished`/`Checking` line actually appeared before trusting a green result.

## Method rules adopted this session (all learned from real errors, mostly this session's own)

1. **Check mtime before declaring anything unowned.** "Nobody owns this" is far stronger than
   "I can't tell who owns it", and usually only the second is true. (Cost: this session wrongly
   attributed a live mid-propagation rename to a closed ticket and told a peer to patch it.)
2. **A derived artifact is not a live predicate.** Report counts, file existence, directory
   contents and agent completion notices all mislead. (Cost: a peer read this ticket's report count
   as "session dormant"; this ticket read directory contents as "facet migrated" and completion
   notices as "work landed" — both wrong.)
3. **A cargo result is evidence only if the run reached the target.** `cargo check` never builds
   `#[cfg(test)]` code — four repo-wide breaks hid there today. And `--all-targets` proves nothing
   about targets a failing earlier crate prevented it from reaching.
4. **Compare name sets, not cardinalities.** `procedural3d` passed a 14-dirs-vs-14-variants audit
   while 8 of its dirs were misnamed.
5. **A bare identifier grep is a search, not a census.** (Cost: "9 files reference
   `CollectionMutation`" was two unrelated types sharing a name; "70 hits in space" was mostly
   already-conformant vocabulary.)
6. **Check whether a specific rule already covers the shape before deriving from the general one**
   — DKM's formulation. Distinct from the others: their measurement was correct and their general
   rule was applied correctly; the answer was still wrong because a more specific rule existed
   unread. Actionable version: the general axis (scalar vs structured, addressed vs root) lives in
   `📓️taxonomy.md`, the shape rules (id-keyed / ordered / edge / hierarchy collections) live in
   `📓️derivation-rules.md` — **derive a verb only after reading both.**

## A-RULINGS ISSUED to other tickets (this ticket owns the mutation vocabulary)

Recorded here because they are now binding on DKM (#2550) and APA (#2549), and because several
generalise. Full context in each ticket's design docs.

**Flow (DKM):** `connect`/`disconnect` for synapses (edge collection, rule 4) vs `create`/`delete`
for widgets ✔; `change-neuron-preview{new_preview}` **over** `toggle` — `toggle` is approved and
self-inverse, but it is *value-blind under concurrent merge*: two toggles converge to the original
state rather than to what either user intended, and this repo is event-sourced with real merge, so
convergence is not optional; `edit-note-text` ✔ (authored content body, not a scalar);
`SetFixture` and `SetLayout` die with no replacement (whole-doc replace → `ArtifactStore::reset`;
whole-list swap → N × `move-widget`); **the plugin-side bridge disappears entirely** rather than
being kept bridgeable.

**Flow's misplaced `camera` field:** `FlowFixture` persists `camera: CameraJson` while
`FlowMutation`'s doc three lines above says the camera is ephemeral view state. Both cannot be
true, and today the only way to change it is whole-document replace. Ruled: **do not delete it —
route it to APA's draft lane** (`🎛️apps/<app>/📝️draft/…`, ephemeral local-only, never enters a
checkpoint). Keeps the capability, puts it in the lane built for exactly this, and avoids minting
document vocabulary for view state. Needs UCAS (snapshot shape) + APA (draft lane).

**Space (DKM):** `SetName` → `RenameCollection` (identity field on the document root);
`Add*`/`Remove*` → `Create*`/`Delete*` (id-keyed entities).

**Space's `MoveFolder`/`MoveEntry` → `move-to-<container>` — the verb already existed.**
DKM correctly found that tree re-parenting is neither `move` (absolute spatial) nor `reorder`
(never spatial), and was about to either downgrade it to `change-folder-parent` or mint a new
`reparent` domain verb. Neither was needed: `📓️derivation-rules.md:23` rule 5 already specifies
`move-to-<container>{id,new_parent}` for hierarchy/parent-id fields. Their field test (one scalar
link, identity preserved → `change`) is correct in isolation; rule 5 is the more specific rule and
wins. **Settled by reading the derivation docs, not by recalling them** — third time in this
session that reading beat remembering.

**Space's whole-record `CollectionDiff`: handcraft the diff, do NOT extend the derive engine.**
DKM's own formulation is the best statement of why gate 3 exists that this program has produced:
*"replayability isn't the property the rule protects; mergeability is. A whole-record diff asserts
every field, so two users renaming a folder and moving it cannot merge."* The existing comment
defending the shape ("staying exactly as replayable") is true and answers a question the rule
wasn't asking. Handcrafting follows local precedent — that file already handcrafts its
`OpText`/`OpBinary` codecs for the same class of derive limitation. The derive engine's missing
nested-`Option` and "record + position" composite support should be its own ticket; extending
`dsl::DslDiff` mid-five-session-refactor has repo-wide blast radius to benefit one facet.

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

### A3-PROGRESS (coordinator, executed by hand without cargo)

**`🧊️procedural3d` — FIXED, complete.** It had the *same* drift as procedural2d but passed the
first numeric audit because its cardinality matched (14 dirs = 14 variants) while 8 dirs were
misnamed. This is the trap recorded above — compare name sets, not counts. Also carried a
pre-existing `➕` emoji collision (`➕create-generation` and `➕create-widget`).

Renames applied, with glue paths, glue module names, the dispatch `use` list, and **27 leaf files'
sibling references** all rewritten to match:

`🎛set-widget`→`🩹update-widget`, `🎛set-layout`→`📍move-widget`, `🎛set-schema`→`🔤change-schema`,
`🎛set-synapse`→`🔄update-synapse`, `🎛set-camera`→`📷update-camera`,
`➖remove-widget`→`❌delete-widget`, `➖remove-synapse`→`✂️disconnect-synapse`,
`➖remove-layout`→`🧹delete-widget-position`, `➕create-widget`→`🌱create-widget` (collision fix).

Verified statically (no compiler needed): **14 dirs = 14 variants**, zero duplicate emoji, zero
stale `set_*::`/`remove_*::` references, and **all 177 `#[path]` targets in the plugin's
`📦️glue.rs` resolve to files that exist**.

**`🌀️procedural2d` — 8 of 14 done.** The 7 misnamed dirs were renamed with the same full sweep
(glue paths + module names + dispatch refs; all 177 path targets still resolve).
Remaining: **6 variants whose leaves are still inline `pub mod` blocks in the dispatch file** —
`ReplaceWidget`, `ReplaceSynapse`, `CreateGeneration`, `DeleteGeneration`, `RenameGeneration`,
`ChangeGenerationValue`. Their own doc comments explain why: *"Overflow variant — no pre-wired
`📦️glue.rs` triad slot, so its leaves live inline"* — an earlier agent's workaround for being
denied glue access.

Each is a complete, correct `pub mod` with a real `MutationKind` impl, diff and inverse. **They
compile and behave correctly; the defect is purely the triad-directory convention.** Extraction is
mechanical (6 dirs × 3 leaves + 6 glue mounts, ~18 files) and was deliberately NOT started with the
disk at ~900Mi and a session limit pending — a half-extracted facet with dangling mounts would
break every session, which is a strictly worse outcome than a working convention violation.

**Static verification recipe for whoever finishes it** (works with cargo dead):
```
# every #[path] target in a plugin's glue must exist
grep -oE '#\[path = "[^"]+"\]' <glue> | sed 's/#\[path = "//;s/"\]//' \
  | grep '\.rs$' | while read p; do [ -f "<glue-dir>/$p" ] || echo "MISSING: $p"; done
# dir/variant name-set equality, and emoji uniqueness
ls <facet> | grep -vE 'component\.|💾️binary|📝️text'
```

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

- **B1's "144 errors, `tempfile` not a dev-dependency": RETRACTED, then the retraction itself
  proved WRONG. The original finding was correct.** Full history, because it is instructive:
  1. This session measured 144 errors in `🏪️store/🔄️sync`, `tempfile` unresolved. Reported.
  2. DKM and APA both measured *later*, found `tempfile = "3.20.0"` present in os-kernel's
     `Cargo.toml` and the lib-test binary linking fine. This session retracted the finding and
     attributed it to disk-full `rmeta` write failures producing bogus "cannot find crate" errors.
  3. IIF then git-triaged it properly
     (`INTRODUCE-INFERENCE-SCHEMA-FAMILY…/📓️p0-d2-oskernel-triage.md`): the `tempfile` gap was
     **real and pre-existing**, introduced by `8baa5706ec` (Aug 6, flag 450) — that test code was
     *never buildable*, it landed unverified. The companion `DemoSnapshot`/`DemoMutation` trait-bound
     failures came from `9391e1ed2b`/`b92a614cad` (Aug 7, flags 462–463), a derive-migration wave
     whose follow-up sweep missed those demo fixtures. Both predate every session working today.
  4. Both root causes were then fixed by other concurrent sessions (one committed, one still
     uncommitted), which is why the later measurements found a clean tree.

  **Net: the original read was right, the retraction over-corrected, and the current state is
  genuinely clean (0 errors).** The lesson is symmetric with the staleness rule this session has
  been applying all day — *a peer's contradicting measurement can be newer rather than better*.
  Before retracting, establish whether you and they measured the same tree at the same time. The
  disk-full explanation was plausible, which is exactly what made it easy to accept without proof.
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
