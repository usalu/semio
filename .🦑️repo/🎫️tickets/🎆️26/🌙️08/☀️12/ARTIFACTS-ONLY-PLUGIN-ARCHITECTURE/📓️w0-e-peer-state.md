# W0-E — Peer-state census (UCAS + SMO), read-only

Computed 2026-08-12 ~15:15–15:26 CEST. Both peer tickets are **live, writing right now** — this
snapshot decays in minutes, not hours. Re-run before any APA wave dispatch.

## GO / WAIT recommendation, up front

**WAIT on every plugin-restructure wave. WAIT on any `🔌️plugin/🦀️component.rs` /
`🏪️store/🦀️component.rs` touch.** Rationale, in order of severity:

1. UCAS's own framework primitives (`ArtifactChild`/`ArtifactLink`/`CompositionCoordinator`/
   `OwnerRef` in `🏪️store`) are **not landed yet** (§3e) — B2 is "dispatched", no report. APA's
   plugin-root work depends on composition semantics UCAS hasn't shipped.
2. `semio-s-plugin-stdio` is red right now from UCAS's in-flight subset renames (SMO's own
   `📓️quality-audit.md:61-64`: *"cargo gates cannot pass for any lane right now"*) — **every**
   plugin crate depends on stdio, so no plugin, including SMO's "released" ones, is
   compile-verifiable at this instant.
3. SMO's own `📓️status.md` **contradicts itself** on the two plugins it claims are free
   (`🪐️space`, `🔋️energy`, §5) — flagged, do not treat as clear without a fresh SMO ping.
4. Two of the four crates SMO's notes describe as fixed still have a **currently-dangling**
   `#[path]` glue mount (§4) — not the one SMO's notes describe (that one IS fixed), a different,
   still-open one that lands squarely in APA's own future territory (app `🦀️component.rs` files).
5. `🗄️stdio` is UCAS's hard exclusive claim (§1) — never touch without their "roster frozen"
   signal, which has not been given.

Net: of 33 plugins, **zero** clear a literal, skeptical NOW test today (§5 table). The two closest
(`space`, `energy`) need one clarifying ping to SMO before APA can trust them.

---

## 1. Peer tickets — wave status summary

### UCAS (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`)

Reports read: `📌️important.md`, `📓️status.md`, `📓️smo-clearance.md`, all 3 files in
`📓️wave1-reports/` (a1-framework-core, a2-schema-composition, b1-spr-vcs). `📓️wave2-reports/`,
`📓️wave3-reports/`, `📓️wave4-reports/` are all **empty directories** — nothing has landed in any
of those waves.

- **W0 recon: DONE.**
- **W1 (kernel primitives + CompositionCoordinator): IN PROGRESS**, restructured into 5 bounded
  rounds after a first mega-agent attempt was killed and landed zero edits.
  - Round 1 A1 (framework-core: `ArtifactRef`/`ArtifactKindId`/`EditRef`/`UndoGroup.member_edits`):
    **DONE**, report exists, `cargo check -p semio-framework` clean, 125 lib tests passed (per
    their report — not independently re-run by me).
  - Round 1 A2 (schema: `ArtifactCompositionSpec`/slot emission): **DONE**, report exists, clean
    check + 9/9 lib tests (per their report).
  - Round 2 B1 (spr `MutationMeta.group_id` + vcs `Checkpoint.composition_pins`): **DONE**, report
    exists; 802/804 lib tests passed, 2 failures attributed to concurrent SMO churn (their claim,
    not independently re-run).
  - Round 3 B2 (store `🔖️Composition` + `CompositionCoordinator`): **dispatched, NOT done** — no
    `b2-*-report.md` file exists anywhere in `📓️wave1-reports/`. Independently verified absent
    from source (§3e).
  - Round 4 C1 (plugin `Emit.child_emits`, WIT link resolution, group undo/redo): **not
    dispatched yet** per status.md (listed as a future round only).
  - Round 5 D1 (testkit + kernel tests): **not dispatched yet**.
- **W2 (stdio)**: **CLAIMED**, prep phase running now (mechanical subset renames
  `✳️workflow`→`✳️flow`, `✳️object`→`✳️value`). Confirmed on disk: `🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`
  currently contains `✳️flow` and `✳️value`, no `✳️workflow` or `✳️object` remnants — the rename has
  physically landed. New subsets (`text`, `table`, spatial `object`, `graph`, `kit`) **not yet
  created** — not present in that directory listing.
- **W3–W7 (exemplars, mass fan-out, serializer, ratchet, verify+close): not started.**

### SMO (`SEMANTIC-MUTATIONS-OVERHAUL`)

Reports read: `📓️status.md` (⚠️ `📌️important.md` is **empty** — 0 bytes, contains nothing),
`📓️quality-audit.md`, all 5 files in `📓️wave1-reports/`, all 25 files in `📓️wave2-reports/`
(filenames only + targeted reads), all 4 files in `📓️waveR-reports/` (r1 read in full, r2a/r2b/r2c
by filename + status.md summary). `📓️waveC-reports/` and `📓️waveM-reports/` are **both empty
directories** — Wave C and Wave M are running per status.md's tables but have **landed zero
written reports**; all evidence of their progress is status.md's own prose plus scratch `.txt`
cargo-check logs at the ticket top level (not per-lane reports).

- **Wave 0 (mechanism+policy): DONE.**
- **Wave 1 (exemplars — cad, draw, fem-3d, gismap, gisterrain): DONE**, 5 reports, matches
  filenames on disk exactly.
- **Wave 2 (mass fan-out): PARTIAL** — status.md claims "25 facet reports, 32/107 facets
  migrated"; **25 report files independently counted on disk, matches exactly.** Status.md's own
  caveat: agents were denied `📦️glue.rs` edits, leaving 4 crates compile-broken (writer, vcs, flow,
  sequence) — this is what Wave R1 (below) addresses.
- **Wave R (repair): IN PROGRESS.**
  - R1 (glue repair for writer/vcs/flow/sequence + flow's 8 diff-leaf rewrites): report exists,
    marked "running" in the status table but the report itself is a completed writeup — **the
    triad-dir glue mounts it targeted are confirmed fixed on disk** (§4). The report also
    surfaces two framework-level blockers it explicitly did NOT fix (both since resolved — see
    §4) and one it explicitly chose NOT to fix (the app-panel path — still broken, see §4).
  - R2a (gis 12 leaves + shooting 1 leaf): **DONE** per status table, 170/170 gis lib tests
    (their claim).
  - R2b (animate 6 + layout 4 + process 2 leaves): status table says "running"; report file
    exists (18KB) — likely complete or near-complete, table not yet updated. UNVERIFIED which.
  - R2c (architect 4 leaves): **DONE** per status table — leaves were already correct, only
    doc-comment token rewording needed.
  - R3 (`📜️script.ts` policy trueing): **HELD**, deliberately, until fan-out settles.
- **Wave C (cleanup+funnel): IN PROGRESS, zero reports landed.** Table in status.md: architect
  lane running (105→149-cleared-of-254 per a one-token coordinator fix, 105 remaining are this
  ticket's own funnel debt); shooting/demonstrator/lowpoly lane running; "remaining"
  (animate/process/reasoning/layout/gis/mathematical/writer/vcs/flow app+config debt) **not yet
  launched**.
- **Wave M (mass fan-out): IN PROGRESS, zero reports landed.** Table: space+trinity running,
  note running, singles (energy-model/sourcing-curate/raster/dag) running, block running, puzzle
  running, norm (all 15 facets) running, stdio **not yet launched** (deliberately, waiting on
  UCAS), odd remainder (remodel/imperative/playbook) **not yet launched**.
- **Playbook design decision**: resolved to move ~452 framework lines + the mutation vocabulary
  into the plugin itself; work is scoped but, per the empty `waveM-reports/` dir, **not executed
  yet**.

### What each says about the other

- UCAS's `📓️status.md` narrates a live negotiated protocol with SMO covering: stdio ordering
  (UCAS first), the composition verb set (SMO reviewed and corrected `adopt`→`inline`,
  `update-link-pin`→`change-link-pin`), and a "ping before entering `🏪️store/🦀️component.rs`"
  agreement for SMO's eventual ratchet.
- SMO's `📓️status.md` has a dedicated section "Cross-ticket coordination with
  `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`" (this ticket) that **already assumes APA is executing**,
  states APA is "not disjoint" from SMO's Wave C app-side rewrites, and lists per-plugin
  release/hold state (reproduced and cross-checked in §5). It also flags an **open, unresolved
  design question routed to APA**: whether `DraftMutation` stays `NoDraftMutation` (54 apps today)
  or becomes a real per-app type under APA's thread_local→Draft-lane work, because SMO's final
  ratchet wants `DraftMutation: SemanticMutation` — SMO is waiting on APA's answer.

---

## 2. Liveness evidence

Current time at check: **Wed Aug 12 15:26:20 CEST 2026** (also captured mid-run at 15:17:15 and
15:18:50 — both below).

| Ticket | Newest file (excl. `🎯️target/`) | mtime | Confidence |
|---|---|---|---|
| UCAS | `scratch-w2prep-6.txt` | Aug 12 **15:15:49** | High — stdio-prep scratch file 10 min old at last check, `🎯️target/` build artifacts from `semio-s-plugin-stdio` compiling at 15:15:42-43 in the same window |
| SMO | `waveM-trinity-cargo-check-1.txt` | Aug 12 **15:18:50** | High — this is the single newest file across *either* ticket folder; `📓️status.md` itself was rewritten at 15:16:35, `📓️quality-audit.md` at 15:15:23 |

Repo-wide auto-commit corroborates: `git log --oneline -3` now shows
`1caac91709 🐙️ueli🎆️26🌙️06☀️04🚩️492` at the tip — flag **492**, one higher than the **491** this
session's own `git status` snapshot started at. The auto-committer has advanced during this
census, independent evidence both peers are still producing committed work.

**Confidence call: both sessions are actively writing right now, not dormant.** Do not schedule
any APA wave assuming either has paused.

---

## 3. UCAS landed-state verification (source-checked, not status.md-trusted)

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| a | `ArtifactRef` + `ArtifactKindId` in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` | **YES** | `struct ArtifactKindId(String)` at `:95`; `struct ArtifactRef {` at `:162` |
| b | `EditRef` + `UndoGroup.member_edits` in `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` | **YES** | `struct EditRef {` at `:451`; `pub member_edits: Vec<EditRef>,` at `:465` |
| c | `ChildSlotSpec`/`LinkSlotSpec`/`ArtifactCompositionFields` in `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` | **YES** | `struct ChildSlotSpec {` `:102`; `struct LinkSlotSpec {` `:111`; `trait ArtifactCompositionFields {` `:122`, plus unit tests at `:720-725` exercising both |
| d | `MutationMeta.group_id` in `📡️spr/🎮️command` | **YES** | `struct MutationMeta {` `:389`; `pub group_id: Option<String>,` at `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:424`. Threaded through `.spr` persistence in the sibling `📜️history/🦀️component.rs` (`group_id` field `:96`, presence-bit-4 encode `:645-646`, decode `:671-672`) |
| e | `ArtifactChild`/`ArtifactLink`/`CompositionCoordinator`/`OwnerRef` in `🏪️store` | **NO** — none exist | Grepped `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/` (component.rs + worker + sync subdirs) for all 4 names: **zero struct/enum definitions**. Only 2 hits total, both forward-looking *comments* in `🦀️component.rs:741` and `:2541` ("`CompositionCoordinator` that … is a later wave" / "… is a later wave") — i.e. the code itself documents that this is not built yet. Matches status.md's own "B2: dispatched" (no completion claim) |
| f | Any `declare_artifact!` or `plugin!` macro | **NO** — neither exists anywhere in the repo (excluding `🎯️target/`) | `grep -rn "macro_rules! declare_artifact\|macro_rules! plugin!\|declare_artifact!\s*("` repo-wide: zero hits. The only `plugin`-named macro anywhere is the pre-existing, unrelated `macro_rules! plugin_exports {` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:7144` (also duplicated in an old ticket's `.pre-patch.rs` scratch file, irrelevant) |

**Implication for APA**: the composition primitives APA's target architecture will eventually
want to compose against (child/link/owner records, a coordinator) are roughly **60% landed**
(io/kernel/schema primitives yes; store-level composition no) and there is **no** declarative
plugin macro of any kind yet — APA W1 cannot assume either exists.

---

## 4. SMO red-crate check — writer / vcs / flow / sequence glue mounts

Method: read every `#[path = "..."]` string in each plugin's `📦️glue.rs`, resolve it relative to
the glue file's own directory, `test -e` each target. Not compiled.

**Headline: the mutation-triad dangling mounts SMO's notes describe are FIXED. All 4 crates are
still RED right now, but from a different, single, identical dangling mount per crate that R1
explicitly chose not to touch.**

| Crate | Mutation-triad `#[path]` mounts (the ones SMO's R1 report describes) | Other dangling mount found |
|---|---|---|
| `✒️writer` | **green** — all 4 triad dirs (`🏷️rename-writer`, `🔗change-uri`, `🌐change-language`, `✏️edit-text`) × 3 leaves = 12 paths, all resolve | **RED** — `📦️glue.rs:406` `#[path = "../../🎛️apps/✒️writer/📌️panels/📄️document/🦀️component.rs"]`, `pub mod document;` at `:407` |
| `🌿️vcs` | **green** — all 6 triad dirs × 3 leaves = 18 paths, all resolve | **RED** — `📦️glue.rs:403`, same `📌️panels/📄️document` pattern |
| `🌊️flow` | **green** — all 9 triad dirs × 3 leaves = 27 paths, all resolve | **RED** — `📦️glue.rs:460`, same pattern |
| `🎬️sequence` | **green** — all 8 triad dirs × 3 leaves = 24 paths, all resolve | **RED** — `📦️glue.rs:406`, same pattern |

Root cause of the still-open one (confirmed on disk, not just quoted from their report): every one
of these 4 plugins' app dir has `🎛️apps/<plugin>/📌️panels/📄️artifact/🦀️component.rs` **present**
and `📌️panels/📄️document/` **absent** — e.g.
`✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/📌️panels/📄️artifact/🦀️component.rs` exists,
`…/📌️panels/📄️document/` does not, confirmed for all 4 via directory listing. `git log --oneline
-3` on the `📄️artifact` dir shows it was created at auto-commit flag **480**, which predates R1's
own glue-fix commit (flag **492**, same as this session's `git log -3` tip) — R1's report says so
explicitly (`📓️waveR-reports/r1-glue-and-flow-report.md:82-88`): *"pre-existing, out-of-scope
blocker… renamed to `📄️artifact` by a different concurrent session"* and states plainly it
deliberately left this alone because it is app-layer, not `🧬️mutations`-facet territory.

**This is squarely APA's territory** (app `🦀️component.rs` files are explicitly on APA's list per
SMO's own coordination note, §1). Grep-anchor for whoever picks this up:
`grep -n "📌️panels/📄️document" "✏️s/🔌️plugins/<plugin>/📦️packages/🦀️rust/📦️glue.rs"` — 1 hit per
crate, all 4 identical shape.

Two framework-level blockers R1's report flagged as unresolved-by-them at write time were
independently re-checked and are **now fixed** (someone — almost certainly UCAS A1/A2 — landed
them after R1 wrote its report):

- `UndoGroup { … }` missing `member_edits` field, `🔌️plugin/🦀️component.rs:5415` and `:5476` —
  **now present** at both sites (`member_edits: Vec::new()` in both struct literals). Matches
  UCAS status.md's own "orchestrator collateral fix" claim.
- `#[proc_macro_derive(...)]` "must reside in crate root" error at
  `🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs:309` — file is now only 262 lines total (was
  ≥309 when R1 hit the error), derive attribute now sits unmounted at the file's own top level
  (`:189`, no wrapping `#[path]`/`mod`). Consistent with UCAS's A2 report claiming this crate
  compiles clean.

**Per-crate verdict for the literal question asked ("does the glue file reference a directory that
does not exist"): RED / RED / RED / RED**, all 4, all from the same single
`📌️panels/📄️document` mount, not from the mutation triads.

---

## 5. Per-plugin clearance table (all 33)

Two different "clear" concepts are in play and **must not be conflated**: UCAS's
`📓️smo-clearance.md` snapshot (§ below) measures whether a plugin's `🧬️mutations/**` is safe for
*UCAS* to add composition facets to — a narrower, staler, and different question than "may APA
restructure this whole plugin root." The table below answers APA's question, sourced primarily
from SMO's own `📓️status.md` "Cross-ticket coordination with APA" section (§1), cross-checked
against actual wave-report file presence.

⚠️ **UCAS's `📓️smo-clearance.md` is explicitly self-described as stale** ("computed at ticket-open
… re-check before every W4 dispatch") and already contradicts SMO's live status.md in both
directions (e.g. marks `space`/`energy` NOT-clear at UCAS's ticket-open time while SMO's status.md
now claims them released; marks `cad`/`draw`/`forms`/`fem`/`procedural` CLEAR while SMO's current
status.md doesn't mention them in either its released or held lists at all). Treat it as
historical color only, not as ground truth.

| Plugin | SMO wave report? | UCAS claim? | APA clearance | Why |
|---|---|---|---|---|
| `✒️writer` | Y — wave2 + waveR-R1 | No | **LATER** | Compile-red (§4); SMO holds, "Wave C app-debt not yet launched" |
| `🌿️vcs` | Y — wave2 + waveR-R1 | No | **LATER** | same |
| `🌊️flow` | Y — wave2 + waveR-R1 | No | **LATER** | same |
| `🎬️sequence` | Y — wave2 + waveR-R1 | No | **LATER** | same |
| `🏛️architect` | Y — wave2 + waveR-R2c | No | **LATER** | SMO Wave C lane "running", 105 app-debt errors, 44 already cleared |
| `🎥️shooting` | Y — wave2 + waveR-R2a (DONE) | No | **LATER** | SMO Wave C shooting/demonstrator/lowpoly lane "running" |
| `🎪️demonstrator` | Y — wave2 | No | **LATER** | same lane |
| `💠️lowpoly` | Y — wave2 | No | **LATER** | same lane |
| `🎞️animate` | Y — wave2 + waveR-R2b | No | **LATER** | Wave C "remaining" app+config debt not yet launched |
| `🏭️process` | Y — wave2 + waveR-R2b | No | **LATER** | same |
| `💡️reasoning` | Y — wave2 | No | **LATER** | Wave C "remaining", not yet launched |
| `📏️layout` | Y — wave2 + waveR-R2b | No | **LATER** | same |
| `🌍️gis` | Y — wave1 (×2) + waveR-R2a (DONE, 170/170) | No | **LATER** | Wave C "remaining" app+config debt explicitly listed, not yet launched |
| `➗️mathematical` | Y — wave2 | No | **LATER** | Wave C "remaining"; quality-audit shows 1 active apply-and-capture leaf |
| `📐️cad` | Y — wave1 (DONE) | No | **NEVER-WITHOUT-HANDOFF** | Absent from SMO's current released/held lists entirely — a gap, not a clearance; ping SMO before touching |
| `🖍️draw` | Y — wave1 (DONE) | No | **NEVER-WITHOUT-HANDOFF** | same gap |
| `🌀️procedural` | Y — wave2 (×2) | No | **NEVER-WITHOUT-HANDOFF** | same gap |
| `📋️forms` | Y — wave2 | No | **NEVER-WITHOUT-HANDOFF** | same gap |
| `🏗️fem` | Partial — wave1 3d only, 2d missing (per stale UCAS snapshot, UNVERIFIED against current disk) | No | **NEVER-WITHOUT-HANDOFF** | partial coverage + absent from current released/held lists |
| `🗒️note` | N — `📓️waveM-reports/` empty | No | **LATER** | Wave M "note" lane "running"; fresh scratch cargo-check files (`scratch-note-cargo-check-2/3.txt`, ≤15 min old) prove active work right now |
| `🧱️block` | N | No | **LATER** | Wave M "block" lane running |
| `🧩️puzzle` | N | No | **LATER** | Wave M "puzzle" lane running |
| `📕️norm` | Partial — 9/15 facets in wave2 (per stale snapshot, UNVERIFIED for current 15) | No | **LATER** | Wave M "norm: all 15 facets… running" |
| `🔱️trinity` | N | No | **LATER** | Wave M "space+trinity" lane; newest file in EITHER ticket (`waveM-trinity-cargo-check-1.txt`, 15:18:50) proves this is being worked on at this literal instant |
| `🕸️dag` | N | No | **LATER** | Wave M "singles" lane running |
| `🖨️raster` | N | No | **LATER** | same lane |
| `🗄️stdio` | Partial — 61 leaves explicitly deferred by cross-ticket agreement | **YES, exclusive** (UCAS `📌️important.md` hot-file table + `📓️status.md` "W2 — stdio: CLAIMED") | **NEVER-WITHOUT-HANDOFF** | UCAS's hard exclusive claim; mid-restructure right now (§1); SMO itself is waiting on UCAS's "roster frozen" signal before resuming — APA must wait for the same signal, not go around it |
| `📸️remodel` | N | No | **LATER** | Wave M "odd remainder", not yet launched (but claimed, do not preempt) |
| `📜️imperative` | N | No | **LATER** | same; quality-audit shows 1 active apply-and-capture leaf |
| `📖️playbook` | N | No | **NEVER-WITHOUT-HANDOFF** | SMO has a fully scoped, unexecuted plan to delete ~452 framework lines and move the vocabulary into this exact plugin's root — an APA restructure landing mid-flight here would collide catastrophically |
| `🔋️energy` | N report file, but quality-audit spot-check (`📓️quality-audit.md:47-54`) calls its `replace-model` leaf finished | No | **LATER, not NOW despite SMO's claim** | **Contradiction inside SMO's own status.md**: the coordination section says "Released to APA now: `🪐️space`, `🔋️energy`… no further SMO work queued", but the Wave M lane table in the *same file* lists `"energy-model, sourcing-curate, raster, dag: running"` — i.e. still in flight. Do not trust either half alone; ping SMO for an explicit, unambiguous confirmation first |
| `🪐️space` | N report file, quality-audit spot-check (`:38-45`) calls its `🏠️home` facet finished | No | **LATER, not NOW despite SMO's claim** | Same contradiction: "released" claim vs. Wave M table's `"space + trinity: running"`, reinforced by fresh `waveM-space-home-*` scratch files (cargo-check runs as late as 14:46, ~40 min before status.md's "released" line was (re)written at 15:16 — could be stale-but-true or could mean the lane isn't actually closed). Ping SMO before claiming |
| `🪵️sourcing` | N | No | **LATER** | Wave M "singles" lane running |

**Zero plugins qualify as clean NOW.** Every plugin is either (a) actively claimed and being
written to within the last ~15 minutes, (b) explicitly held by SMO for not-yet-launched Wave C/M
work, (c) UCAS's exclusive stdio claim, or (d) an unaccounted-for gap in SMO's own coordination notes
that needs a direct ping before anyone treats it as safe. `space` and `energy` are the two to
re-check first — one Slack-equivalent ping to SMO asking them to reconcile their own contradiction
would very likely convert both to NOW.

---

## 6. Repo conventions an APA executor agent must obey (beyond this ticket's own rules block)

Numbered so an agent can tick them off literally. Sourced from `/Users/ueli/Documents/semio/CLAUDE.md`
(root, already given verbatim to every agent) plus UCAS's `📌️important.md` (SMO's is empty) —
only conventions NOT already stated in this ticket's assignment rules are listed.

1. **Derive crates keep two byte-identical copies.** `<module>/✨️derive/🦀️component.rs` and
   `<module>/✨️derive/📦️packages/🦀️rust/📦️glue.rs` must be identical — Cargo compiles the *glue*
   copy only, so editing `component.rs` alone silently does nothing. Verified true for
   `🧬️schema/✨️derive` and `🗣️dsl/✨️derive`. After any edit: mirror by hand, then `diff -q` the pair
   before reporting done.
2. **`mcp__repo__file_integrate` mis-mirrors** — it wraps the whole file in a nested
   `mod helpers {}`, duplicating everything. Do the derive-crate mirror above by hand, not with
   that tool.
3. **`#[link(...)]` is unusable as a custom field attribute** — `link` is a built-in Rust attribute
   (extern-block FFI); applying it to a struct field is a hard compile error (E0659/E0539/E0459),
   not a lint. If APA ever needs a link-style attribute of its own, name it something else
   entirely (UCAS landed on `#[link_slot(...)]` for the same reason).
4. **Additive struct fields still break struct literals** — serde's `#[serde(default)]` affects
   only (de)serialization, never Rust struct-literal construction. After adding any field to a
   struct that is constructed via literal syntax anywhere, `grep -rn "TypeName {"` across the
   **whole workspace**, not just your own crate, and fix every literal (or file it as a shared-file
   request if outside your boundary).
5. **Adding an enum variant is expensive wherever the enum is matched exhaustively.** Before
   deciding to add one, run `grep -rln "EnumName::"` and read the hit count — UCAS measured
   `Shape::` at ~20 files and dropped a planned variant addition entirely rather than touch all of
   them (their design deviation D1).
6. **The repo auto-commits the whole tree periodically** (commits `🐙️ueli🎆️26🌙️06☀️04🚩️<n>`,
   monotonically incrementing — currently at **492**). `git status --porcelain` reports **clean**
   for work that landed and got swept into a commit minutes ago — it is NOT a churn detector and
   an empty status does NOT mean "nobody touched this file." If your own edits vanish from `git
   status`, they were committed, not lost — never run a git-modifying command to "recover" them
   (forbidden anyway). Use `git log --oneline -5 -- <path>` and `stat -f '%Sm' <path>` (macOS) to
   detect real churn instead, and never assume a clean tree means a file is safe to blind-overwrite
   — read it first.
7. **`CARGO_TARGET_DIR` is per-ticket and shared across concurrent checks within that ticket** —
   the lockfile serializes them, so `"Blocking waiting for file lock on build directory"` during a
   scoped `cargo check` is normal and should be waited out, not killed.
8. **The semantic-mutation vocabulary is a closed, comment-scanned list.** `SetSnapshot`,
   `NoMutation`, and public `CollectionMutation` are banned repo-wide, and the policy rule that
   enforces this **greps raw file content including comments and docstrings** — merely naming one
   of these in prose (even to say "don't use X") trips it outside files exempted from the scan.
   APA doesn't author `🧬️mutations/**` facets, but any docstring or region comment APA writes
   anywhere in a plugin must still avoid these three literal tokens.
9. **The approved composition-verb vocabulary is closed**: `create`/`delete`, `extract`/`inline`,
   `bind`/`unbind`, `change`. Notably **not** `connect`/`disconnect` (a link fills a *named slot*,
   not an edge-collection row) and **not** `update` (reserved for an inseparable ≥2-field facet
   rewritten atomically — a single-field re-pin is `change`, even when that field's value is an
   enum-with-payload that superficially looks multi-field).
10. **Hot-file ownership is binding and enforced by convention, not tooling.** UCAS's
    `📌️important.md` names specific owner-agents for `🔌️plugin/🦀️component.rs`,
    `🚪️io`/`🧬️schema`/`🛂️manifest`/`🎠️kernel`/`📡️spr`/`🏪️store`/`🌿️vcs`/`🗣️dsl`, and stdio's
    `📦️glue.rs`/`📦️index.ts`/`📇️catalog.json` — everyone else is read-only on those files and must
    file a "sharedFileRequests" entry rather than edit directly. APA will need to negotiate its own
    entries into (or explicit exceptions from) this table before touching any of them, especially
    `🔌️plugin/🦀️component.rs` and app `🦀️component.rs` files.
11. **`MutationMeta` (`📡️spr/🎮️command/🦀️component.rs:389`) is a shared struct with two
    concurrent stakeholders** — SMO's Wave-0 `semantic_kind`/`label` fields (both still
    unconditionally constructed `None` everywhere) and UCAS's `group_id` (§3d). Both sides'
    protocol is "notify the other before touching this struct again" — if APA needs a field here,
    ping both peers first, not just one.
12. **Report shape convention** (used by all three tickets): what changed, files touched with
    file:line anchors, verification commands + their literal output, a `## sharedFileRequests`
    section (file, region, reason, patch-file path) for anything outside your own boundary, a
    `## Concurrent-churn observations` section, and an honest pass/fail — SMO's R1 report is the
    cleanest example of the "I could not verify X, here is exactly why" pattern to imitate rather
    than a false "compiles/passes" claim.
13. **`📜️script.ts` (repo-root) has a three-way write-order agreement already in place**: SMO's
    status.md records the proposed order as **APA → UCAS-W6 → SMO**, one writer at a time,
    announce in-channel immediately before and after each session. APA should expect to go
    *first* here, not last, and should announce accordingly rather than assuming it can queue
    behind the others.
14. **Docstrings need a leading emoji** (already in this ticket's own rules block, restated here
    only because both peer reports demonstrate it in practice) — e.g. UCAS's own EditRef doc-comment
    starts `/// 🔖️...`; match that convention exactly when annotating any new APA-side type.

Files not fully ingested but referenced by both peers for anyone doing deeper mutation-facet work
later (out of APA's own scope, since APA does not touch `🧬️mutations/**`, but worth knowing they
exist): SMO's `📓️remaining-work-map.md`, `📓️fanout-brief.md`, `📓️taxonomy.md`,
`📓️stdio-lane-brief.md`; UCAS's `📓️design-full-plan.md`, `📓️design-composition.md`,
`📓️design-stdio-subsets.md`, `📓️orchestration.md`.

---

*Report file: `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w0-e-peer-state.md`*
