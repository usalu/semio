# Status

## W0 — Recon: DONE

- Ticket opened, goal `🎯aioptimizedrepo`.
- `📌️important.md` (hard rules + hot-file ownership) and `📓️design-full-plan.md` (the approved plan) written.
- `📓️smo-clearance.md` snapshot computed: 18 plugins SMO-clear, 15 not (incl. stdio itself — expected).
- Baseline `CARGO_TARGET_DIR=🎯️target cargo check -p semio-framework-os-kernel`: clean, 49 pre-existing warnings, 0 errors (`scratch-baseline-kernel-check.txt`).
- SMO's own `📓️status.md` at recon time: wave 0 done, wave 1 exemplars (cad/draw/gis×2/fem-3d) done, wave 2 mass fan-out in flight (25 reports so far). No mention of wave 4 (ratchet) starting — `🔌️plugin/🦀️component.rs` is clear for W1 to claim.

## W1 — Kernel primitives + CompositionCoordinator: IN PROGRESS (restructured)

**Claim**: this ticket owns `🔌️plugin/🦀️component.rs`, `🚪️io`, `🧬️schema`, `🛂️manifest`, `📡️spr`, `🏪️store`, `🌿️vcs`, `🎠️kernel`, `🗣️dsl`. Re-verified at restart: SMO is still at wave-2 (25 reports, no wave-3/4 dirs, status.md unchanged) — no ratchet contention on `🔌️plugin/🦀️component.rs`.

**First attempt (single mega-agent) was killed** when the driving session exited, landing ZERO edits (`git status -- 🧰️framework` empty) and no report. Restructured into crate-scoped bounded rounds so each increment is independently verifiable and survivable:

| Round | Agent | Crate | Scope |
|---|---|---|---|
| 1 | A1 | `semio-framework` | `🚪️io` `🔖️ArtifactRef` region (ArtifactRef, ArtifactKindId, canonical-grammar validator, uri codec) + `🎠️kernel` InvocationId/EditRef/`UndoGroup.member_edits` |
| 1 | A2 | `semio-framework-schema` | `🔖️ArtifactCompositionSpec` (ChildSlotSpec/LinkSlotSpec/ArtifactCompositionFields, GraphQL preamble) + `#[derive(ArtifactSchema)]` slot emission |
| 2 | B1 | `semio-framework-os-kernel` | `📡️spr` `MutationMeta.group_id` + `🌿️vcs` `Checkpoint.composition_pins` & new VcsError variants |
| 3 | B2 | `semio-framework-os-kernel` | `🏪️store` `🔖️Composition` (ArtifactChild/ArtifactLink/OwnerRef/LinkPin/ArtifactRefs/LinkResolver/ChildStoreFactory, envelope.owner) + `🔖️CompositionCoordinator` (dispatch_group, SpaceMember wire methods, CompositionGraph) + handcrafted DSL/Pack encoding of the new value types |
| 4 | C1 | `semio-framework-plugin` | `Emit.child_emits`, VcsArtifactApp children map, dispatch_emit group routing, group undo/redo, ArtifactChildren + derive_artifact_facets! children arm, WIT `resolve-artifact-link` |
| 5 | D1 | `semio-framework-os-kernel` | testkit composition laws + kernel unit tests |

Crate map established at restart: `semio-framework` = 🚪️io, 🛂️manifest (mounts 🎠️kernel via #[path]:2824), 🔺️mesh, …; `semio-framework-schema` = 🧬️schema+derive; `semio-framework-os-kernel` = 🌿️vcs, 🎒️pack, 🏪️store, 📡️spr, 🗣️dsl, ⚙️engine, 💡️inference, 🧩️extension, 🧬️semio; `semio-framework-plugin` = 🔌️plugin.

Reports land in `📓️wave1-reports/`.

### Design deviation D1 — no dedicated `Shape::Child`/`Shape::LinkRef` DSL variants

The approved plan carried these from the earlier INLINE-child design, where a child slot embedded a nested snapshot and needed a block-with-header text form. The user's child-as-own-envelope decision reduced `ArtifactChild` to a flat two-string handle (`child_id` + `target: ArtifactRef`), so an ordinary record encoding is sufficient and loses nothing — composition semantics are carried by the schema-level slot tables (`ArtifactCompositionFields`, agent A2), not by the wire shape.

Measured cost of keeping them: `Shape::` is exhaustively matched across ~20 files in this crate (🎒️pack ×4, 📡️spr/📜️history, 🏪️store, 🪐️space, ♾️infinite ×3, 🔁️workflow, 🌊️flow/🌿️vcs, 🗣️dsl ×8). Adding enum variants would break every one of those matches for zero capability gain. Dropped; the DSL/Pack encoding of the new value types is handcrafted inside `🏪️store` by agent B2 instead (the pattern `BackboneMessage` already uses there).

### Orchestrator collateral fix (outside any agent's boundary)

A1's additive `UndoGroup.member_edits` broke two struct literals in `🔌️plugin/🦀️component.rs` (E0063 at :5415, :5476 — serde defaults do not help Rust struct construction). Fixed in place by the orchestrator (`member_edits: Vec::new()` on both) because Round 4's owner had not started and leaving `semio-framework-plugin` red would have poisoned every downstream agent's verification. `cargo check -p semio-framework-plugin` green afterwards.

### Progress

- **A1 (framework-core): DONE** — `🚪️io:85-185` ArtifactRef/ArtifactKindId + validator + uri codec (5 tests); `🎠️kernel` EditRef:451, `UndoGroup.member_edits`:465, TS mirror :242-253; `InvocationId` already existed at :46 and was reused. `cargo check -p semio-framework` clean; `cargo test -p semio-framework --lib` 125 passed.
- **A2 (schema): DONE** — `🧬️schema:93-140` `🔖️ArtifactCompositionSpec` (ChildSlotSpec/LinkSlotSpec/ArtifactCompositionFields with `&[]` defaults, GRAPHQL_COMPOSITION_PREAMBLE) + tests :695-732; `#[derive(ArtifactSchema)]` now emits `ArtifactCompositionFields` by syntactic field classification; TS mirror updated. `cargo check -p semio-framework-schema` clean; `--lib` 9/9 passed. Two findings promoted into `📌️important.md`: the derive-crate glue duplication rule, and `#[link(...)]`→`#[link_slot(...)]` (`link` is a built-in Rust attribute — hard error as a field attribute).
- **B1 (spr+vcs): DONE** — `MutationMeta.group_id: Option<String>` (`📡️spr/🎮️command:389-425`) threaded through `.spr` persistence via `HistoryOpMeta.group_id` (`📡️spr/📜️history:83-97`, presence-bit-4 in `write_op_meta`/`read_op_meta` :602/:651) and the store bridge fns (:1169-1194), round-trip proven narrowly and through the shared `sample_log()` fixture. `CompositionPin` + `Checkpoint.composition_pins` (`🌿️vcs:109-129`), `content_addressed_checkpoint_id` pin-extended (byte-identical for empty pins, proven by reimplementing the old formula in a test) with deterministic sort. `VcsError::{CompositionCycle, OwnershipViolation}`. Check clean, 49 warnings = baseline; `--lib` 802 passed / 2 failed, both `os_dsl::fixture_sweep` over fem/norm/dag plugin fixtures = live SMO churn (retried 3×, grep-proven unrelated).
- **B2 (store composition + coordinator): DONE** — `🔖️Composition` (`🏪️store:166-571`): `ArtifactChild<S>` with hand-written `Clone`/`Debug`/`PartialEq` + `#[serde(bound="")]` to dodge the `PhantomData<S>` bound trap, `OwnerRef`, `ArtifactLink`/`LinkPin`, `ArtifactRefs`, `LinkResolver`/`LinkState`, `ChildStoreFactory` + registry, handcrafted `DslField` records (no new `Shape` variants, per D1). `🔖️CompositionCoordinator` (`:4494-5003`): `CompositionGraph` (ownership forest + link DAG), two-phase `dispatch_group` with reverse-order compensation, `undo_group`/`redo_group`. `CompositionPin.child_ref` corrected to the real `crate::os_io::ArtifactRef`. 15 new tests pass; check clean at 49 warnings; `--lib` 817 passed / 2 failed (the same pre-existing fem/norm/dag fixture-sweep pair).
  - **Deviations accepted**: `SpaceMember` gained 8 object-safe methods, not 3 — the extra 5 (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`, …) are needed to stamp group ids and owners through a type-erased interface. `GroupReceipt` gained `created_children` because a genesis-created member has no caller-held reference and would otherwise be silently dropped, making `ChildGenesis` pointless. `CompositionCoordinator` is STATEFUL (owns the graph) because `SpaceMember` is fully type-erased, so a cross-document ownership graph cannot be derived from a single call's arguments. `GroupMeta.actor`/`coalesce_key` accepted but not wired (no object-safe seam yet) — filed as a `sharedFileRequests` item.
  - **Correctly deferred**: making `ArtifactEnvelope.dialect` required — measured blast radius 106 files / 168 call sites. Left `Option`, filed rather than half-done.

### W1 COMPLETE (kernel). Now running:
- **C1 (plugin composition runtime)** — `Emit.child_emits`/`ChildEmit`, `VcsArtifactApp` child-store map, `dispatch_emit` group routing, group undo/redo, `ArtifactChildren` + `derive_artifact_facets!` children arm, WIT `resolve-artifact-link`. Crate `semio-framework-plugin`.
- **W2a (`✳️text` subset exemplar)** — crate `semio-s-plugin-stdio`. Deliberately ONE subset end-to-end as the template for the remaining four (`table`, `graph`, spatial `object`, `kit`), mirroring the exemplar→fan-out pattern that worked for SMO and is planned for our W3/W4. Its report carries a `## Template for the remaining subsets` section. `🚪️io` hub-routing leaves are explicitly out of scope for that round.

Different crates, so the two run in parallel safely.

### Correction to B1 — `ArtifactRef` IS reachable from os-kernel

B1 concluded `ArtifactRef` could not be imported into `semio-framework-os-kernel` without inverting the dependency graph, and fell back to a raw `String` for `CompositionPin.child_ref`. **That premise was wrong.** `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` is **dual-mounted**: `semio-framework` mounts it as `io`, and os-kernel mounts the *same source file* as `os_io` (`💻️os/📦️packages/🦀️rust/📦️glue.rs:237-238`, `#[path = "../../../../🔨️modules/🚪️io/🦀️component.rs"]`). Store already consumes `crate::os_io::ArtifactDialect` at :88/:105/:662. B2 is fixing `CompositionPin.child_ref` to the typed `crate::os_io::ArtifactRef` and using the real type throughout, so the ticket keeps ONE identity type rather than degrading to stringly-typed refs — the specific outcome this ticket exists to prevent.

`InvocationId` is a genuine exception: `🎠️kernel` is mounted only by `🛂️manifest` inside `semio-framework`, so it really is unreachable here and `group_id: Option<String>` stays.

### Environment discovery — the repo AUTO-COMMITS

A background process periodically commits the whole tree (`🐙️ueli…🚩️<n>`; flag 492 landed mid-wave and swept B1's framework edits). Consequence: `git status` reports CLEAN for work that landed minutes ago, so it is **not** a churn detector, and a vanished edit is committed, not lost. `📌️important.md` rule 6 now directs all agents to `git log --oneline -- <path>` + mtime instead. This also explains why an early progress check showed zero framework modifications despite confirmed edits.
- **W2-prep (stdio subset renames): running** — see stdio claim below.

## W2 — stdio: CLAIMED (prep started early)

**Claim**: this ticket owns `✏️s/🔌️plugins/🗄️stdio/**` from now. Justification for starting before full SMO clearance: stdio is 0/37 covered by SMO and would otherwise never clear; SMO has been dormant since our recon (wave-2 report count static at 25, no wave-3/4 dirs, `git status -- 🗄️stdio` empty). If the SMO session resumes and needs stdio, it must coordinate via this file.

### Cross-session agreement with SEMANTIC-MUTATIONS-OVERHAUL (SMO), 2026-08-12

The SMO session is **live, not dormant** (my earlier inference from a static report count was wrong — that count was a *previous* session's stopped wave). It currently has ~9 migration lanes running. Agreement reached over the session channel:

1. **stdio ordering: we go first, SMO follows.** Ours changes which subsets *exist*; theirs rewrites the mutation *vocabulary inside* each. SMO is holding all ~53 stdio mutation facets — its stdio lane is written but unlaunched, and nothing of theirs is in `🗄️stdio/**`. I must signal **"roster frozen"** (not merely "compiles") before they start.
2. **New subsets must be born conforming.** SMO asked that the 5 new subsets not be scaffolded with banned vocabulary. We take the strict path, not their empty-enum fallback: every new facet follows `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` + `📓️fanout-brief.md`. Three hard rules — no `SetSnapshot` (whole-doc replace goes through `ArtifactStore::reset`, outside history; a locked user decision), no `NoMutation` (return `Vec::new()` from `MutationKind::inverse`), no public `CollectionMutation`. If a subset cannot be done conformingly, leave its enum EMPTY and flag it — never invent vocabulary.
3. **The `✳️workflow`/`✳️object` mutation facets carry across unchanged** — we invest nothing in their existing vocabulary; that debt is SMO's to clear.
4. **`✳️any` becomes an 18-way union**; SMO has the final variant list and migrates that facet last. Trap flagged to them: the name `object` survives the roster change but its *meaning* does not (old value-tree → `✳️value`; `✳️object` is now spatial).

### Future overlap: `🏪️store/🦀️component.rs` (agreed protocol)

SMO's **final ratchet** also lands in this file — it adds `SemanticMutation` trait bounds to `ArtifactStore` and starts populating `MutationMeta.semantic_kind`/`label` (both currently written `None` at every construction site, including the six we touched for `group_id`). Our work there is the `🔖️Composition` + `🔖️CompositionCoordinator` regions and the handcrafted DSL/Pack encoding of the new value types — different regions, so no collision expected. Their ratchet is gated behind every facet migrating, so it is realistically the last thing they do. Agreed protocol: **they ping before entering the file and will wait rather than race if we are mid-edit.** Reciprocate.

### Third session: ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE (APA, #2549) — scope split

A third session (`semio-52`) owns APA: make artifacts the ONLY IO/state/registration mechanism in plugins (a plugin = apps + artifacts). It deletes per-plugin `🛂️manifest`/`🎟️capabilities`/`🔧️setup` facet dirs, replaces imperative `.setup(fn)` with declarative `ArtifactDeclaration`, seals the global `register_*` family behind a `Registrar` token, deletes the `semio_framework_os::register_mesh_*/solid_*/dwg_*/app_io` escape hatches, and capability-gates WIT imports/HostEffects. Agreed split:

1. **Registration CEDED to APA.** Our `declare_artifact!`/`plugin!` macros are deleted from this plan — APA's declarative shape subsumes them, with a fuller design behind it. We keep only the composition *runtime* in `🔌️plugin/🦀️component.rs`. Signal APA when C1 unfreezes the file.
2. **W2 scope reduced**: stdio subset roster only, NO registration migration — avoids half-converting 37 artifacts to a shape APA would replace.
3. **W6 scope reduced**: `MeshExporter`/`MeshImporter` deletion is APA's.
4. **APA takes repo-root `📜️script.ts` + `🔣️taxonomy.json` BEFORE our W6**, sequentially — our W6 sits behind W2/W3/W4/W5 and blocking their seal that long is unreasonable. They add their five policy regions (report-mode only, non-gating), notify, then W6 adds ours without touching theirs.
   - ~~Hazard: flipping `pluginChildDirs` before per-plugin cleanup would fail W2 verification.~~ **RETRACTED — the hazard does not exist, and the ordering is REVERSED.** APA read the enforcement code and pushed back; independently verified all three of their findings:
     1. `🔌️plugin/🦀️component.rs:2226-2235` holds a **runtime `assert!`** (not a lint) that reads `pluginChildDirs` dynamically and asserts `<plugin>/<child>/🦀️component.rs` is a file, per plugin. Deleting `🔧️setup/` from even one plugin while the list names it panics the gate — so the flip is a **precondition** for incremental facet cleanup, not a consequence of it.
     2. `policyPluginRootShapeBreaches` (`📜️script.ts:4646-4692`) and the registry codegen (`📇️registry/📜️script.ts:1232-1236`) are **presence-only** — they flag *missing* leaves and have no absence/forbidden check. Flipping the list is purely permissive; stdio's leftover facet dirs go unflagged. Our objection was raised from plan text without reading the enforcement.
     3. The flip target in APA's own plan was wrong and would have broken all 33 plugins: `🗿️artifacts/🦀️component.rs` exists in **0 of 33** plugins (all 33 have `🎛️apps/🦀️component.rs`), and `🗿️artifacts` is governed by the separate `artifactsDirName` key (`🔣️taxonomy.json:173`). Correct target is **`["🎛️apps"]`**.
   - Consumer APA missed, reported back: `🔍️discovery/🟦️component.ts:174` (type) and `:582-586` (`validateTaxonomy` **requires a non-empty array**). `["🎛️apps"]` satisfies it, but emptying the list later — tempting under APA's "plugin = apps + artifacts" thesis — is a schema change, not a value change, and `🧪️index.test.ts:1290-1293` asserts the rejection.
5. **W4 ordering: UCAS before APA, per plugin**, using our `📓️wave4-reports/<plugin>-report.md` as their clearance oracle (mirroring how we gate on SMO). Rationale: our pass deletes plugin-local types and repoints to stdio; theirs moves files between dirs — moving first would invalidate our agents' paths mid-flight. Offered them **demonstrator, vcs, space, note** immediately (our change there is reference-only/trivial).

**Boundary corrections issued to APA:**
- The `register_mesh_exporter`/`register_app_io` family also lives in `💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (OUR file), not just their two — they file `sharedFileRequests`, we remove during C1.
- `🛂️manifest` is two different things sharing one emoji: `🧰️framework/🔨️modules/🛂️manifest/` (ours) vs `✏️s/🔌️plugins/*/🛂️manifest/` (theirs). A `**/🛂️manifest/**` glob would cross the line.
- Their `💻️os/🖥️host/🦀️component.rs` is NOT our `💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (IoRouter, needed by C1's WIT work).

**Three sessions are now in the plugin tree** (SMO mutations, UCAS content, APA structure). Each publishes per-plugin reports; each gates on the others'.

### Design corrections C1/C2 — verb set, after peer review

**C1 `adopt` → `inline`.**

SMO's `📓️taxonomy.md` revealed that the closed `APPROVED_VERBS` table already defines `extract`/`inline` as inverse partners meaning "hoist a fragment into a reusable entity / dissolve back" — exactly our child↔standalone promotion semantics. Our planned `adopt` verb is NOT approved. Dropped in favour of `inline`.

**C2 `update-link-pin` → `change-link-pin`** (caught by SMO's review of our verb set). `update` is reserved for an inseparable ≥2-field facet rewritten atomically. Re-pinning sets the single `pin` field while `target`/`role` stay put, and a link with a stale pin is meaningful (that is the point of `Head` vs a frozen pin) — so it is `change`, record `ChangedLinkPin`. Root cause worth remembering: `update` was reached for as a generic "modify", the exact habit the mutations overhaul exists to kill; `pin`'s value being an enum-with-payload made it *look* multi-field, but an enum-with-payload is still **one field taking one cohesive value**. That distinction recurs for `LinkPin`'s neighbours.

Also ruled and recorded in `📌️important.md`: **`bind`/`unbind`, not `connect`/`disconnect`** — a link fills a *named slot* as a handle, not an edge row in an edge collection ("a parameterization gets bind/unbind instead").

Net: the complete composition verb set is **entirely within the existing approved core** — `create`/`delete`, `extract`/`inline`, `bind`/`unbind`, `change` — so this ticket needs no verb-spine change in `📡️spr`. Plan file and `📓️design-full-plan.md` updated; stale `Shape::Child`/`Shape::LinkRef` references in the design doc also corrected to match deviation D1.

### W2-prep (subset renames): DONE

`✳️object`→`✳️value` (76 files) and `✳️workflow`→`✳️flow` (72 files), exhaustive: dirs (plain `mv`), Rust type/field/fn/const families, all 5 schema-twin languages plus grammar/protocol/spicy/ksy leaves, the `✳️any` union arms, `glue.rs` `#[path]` tree, engine registrations, `🪆️subsets/🔣️component.json`, and ~52 cross-subset doc-comment citations elsewhere in stdio. Fixtures **regenerated** from real `print_dsl`/`encode_pack` output (greenfield rule — not hand-migrated) for `🕸️graph`, `🌊️pipeline`, `🌐️envelope`.

Two judgement calls worth keeping:
- **Collision avoided**: the subset's top-level snapshot-diff struct became `SemioValueTreeDiff`, NOT `SemioValueDiff` — the latter already exists as the recursive leaf-value diff. Renaming blindly would have collided two distinct concepts.
- **Bulk-substitution bug caught**: `JsonValue::Object` (an unrelated external enum from the `json` subset, cited inside the value subset's JSON bridge) was mis-rewritten to `JsonValue::Value`; found via compiler errors and reverted. Exactly the "read each hit in context, do not blind-sed" hazard the brief warned about.

**Verification**: `cargo check -p semio-s-plugin-stdio --tests` 0 errors. `cargo nextest --profile long -p semio-s-plugin-stdio`: **2021 passed, 5 failed, 3 skipped** (2026 total), reproduced identically twice. The 5 failures are `inference_default_law`/outline tests in the csv/html/json/md/pdf `inferences` facets — the agent reports them git-clean and pre-existing (i.e. untouched by this or any concurrent session).

✅ **Independently verified — 2021 / 5 / 3 IS the stdio baseline.** Orchestrator re-ran `cargo nextest --profile long -p semio-s-plugin-stdio --no-fail-fast` and reproduced the numbers exactly. The five are:
```
csv|html|json|pdf  …/💡️inferences/…::tests::inference_default_law
md                 …/💡️inferences/outline/…::collects_headings_and_counts_words_and_blocks
```
**Decisive evidence they are pre-existing**: `git log` on those `💡️inferences` facet dirs shows last commit `a46ac1f883` (flag **491**) — the HEAD at this session's start. Every commit from this ticket is 492+. They therefore cannot have been touched by the renames. Corroborated by scope: they are `💡️inferences` facets of the csv/html/json/md/pdf *format* artifacts, untouched by the `🧿️semio` subset work; SMO attributes them to ticket `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`. **Any run showing more than these five is a new regression.**

(Transient blocker cleared: `semio-framework-os-kernel` briefly went red with 29× `E0753 expected outer doc comment` from a stray `//!` at `🏪️store:179` written by our own in-flight B2 agent. SMO pinpointed it to the exact line; B2 corrected it itself before intervention. Note for future one-character "helpful" fixes in a live region: the correct repair was `//`, and guessing `///` would have silently attached a doc comment to whatever item landed next — compiling cleanly while being wrong.)

Prep agent runs the two mechanical subset renames only (`✳️workflow`→`✳️flow`, `✳️object`→`✳️value`), which are independent of the kernel primitives and unblock the rest of W2. New subsets (`text`, `table`, the spatial `object`, `graph`, `kit`) and child/link slots come in the main W2 agent after Round 3 lands.

## Remaining

W2 stdio roster (gated on W1 + stdio SMO-clearance or explicit claim), W3 exemplars, W4 mass fan-out (~29 plugins, width 7), W5 serializer, W6 policy ratchet, W7 verify+close. Per the "no pause between waves" precedent SMO itself set, each wave launches automatically as the prior one's report lands — this will span many turns/notifications.
