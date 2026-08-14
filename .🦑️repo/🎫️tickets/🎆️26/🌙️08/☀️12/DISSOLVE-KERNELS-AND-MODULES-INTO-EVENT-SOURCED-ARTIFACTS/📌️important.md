# Important — read first, every agent

Ticket: `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` ("DKM"), issue #2550, goal `🎯aioptimizedrepo`.
Approved plan: `/Users/ueli/.claude/plans/dissolve-and-unify-all-splendid-fountain.md`, copied here as `📓️design-full-plan.md`.

## The thesis (one paragraph)

The artifact layer is properly CQRS event-sourced at the top (`ArtifactStore` → `🌿️vcs` → `📡️spr` → backbone), but **every computational kernel below it is still imperative**. The brep kernel mutates topology arenas through `register_solid/face/wire` and `&mut self` Euler/boolean ops behind a host-owned `BrepEngineHost`; `◻2d` runs a **second, parallel non-artifact store** (`DrawingStore`/`DrawingEngine` over `EngineCache`); two separate mesh implementations expose `set_vertex_position`-style APIs; `🗺️surface` and `🖥️platform` hold setter-based UI state; `🌊️flow/🌿️vcs` (40 hits) and `🪐️space` (70 hits) still use the banned `CollectionMutation`; `🛢️db`'s `LiveQuery` hand-rolls `self.snapshot = new_snapshot`. And **70 `set-snapshot` mutation dirs exist repo-wide** — whole-document replace is CRUD wearing artifact clothing. Everything in this ticket exists to make those shapes impossible, not merely absent.

## The doctrine (binding; every design decision resolves against this table)

| Tier | What | The ONLY legal home |
|---|---|---|
| (a) Authoritative state | anything persisted or shared between users | an `ArtifactStore` envelope; snapshot fields in a subset `🧬️schema/📸️snapshot` |
| (b) Semantic edits | every change to (a) | a `🧬️mutations/<slug>/{🦠️mutation,🔺️diff,↩️inverse}` triad, verb from SMO's `APPROVED_VERBS`; diff built from `(payload, base)`; inverse reconstructed from `base` |
| (c) Derived computation | anything computable from a snapshot | a `💡️inference` facet — `InferredField<P>` with a real `DepHash` dependency chain |
| (d) Ephemeral working reps | halfedge adjacency, BVH, brep arenas, tessellation buffers | a **local variable** inside a `🔺️diff` constructor or an `InferredField::{plan,dep_input,compute}` body. Never a durable struct field, never `thread_local!`, never crossing a dispatch boundary |
| (e) Pure compute | algorithms (booleans, WFC, bezier math, statistics) | an engine crate whose public API is consumed **only** from (b)/(c)/(d), an analyzer, or the draft lane. A standalone solver *run* is itself an artifact: problem spec = snapshot authored by mutations, solution = inference |

**Edit vs. derivation — the rule that settles every geometry question.** A user gesture that changes authoritative geometry (boolean, extrude, Euler op, fillet, path boolean) is a **mutation**: a `group_id`-batched set of entity-level `create-*`/`delete-*`/`change-*`/`replace-*`, whose diff is computed by calling pure engine fns on an ephemeral rep built from `base`. A view, query, or measurement (tessellation, normals, AABB, mass properties, validation, flattened scene, intersection preview) is an **inference**. Bulk or procedural generation (STEP import, scripted thousands of creates) is neither — it goes through `ArtifactStore::reset`, the sanctioned non-undoable rebase, never a mutation storm.

**A whole-document replace is not a mutation at all.** Deleting `set-snapshot` facets is not a stylistic cleanup; it is the point. There is no replacement mutation — `ArtifactStore::reset` is the sanctioned path, and that is a locked decision.

**Dirty-flag counters are not state.** Fields like `Platform::generation`/`chrome_generation` exist to tell observers "something changed". Event sourcing provides that for free from the edit log; they are deleted, not migrated.

## Verb rulings from SMO (BINDING — obtained 2026-08-12 before authoring, do not re-litigate)

The governing test, in SMO's words: **`change` sets one scalar field; `replace` is a whole-value swap of a large structured sub-payload. The discriminator is whether the editor ever manipulates the value's interior piecewise.** An enum-with-payload used as an opaque selector (you swap the whole thing or nothing) is still ONE field → `change`. A value whose interior the editor edits (curve control points, gradient stops) → `replace`.

| Area | Ruling |
|---|---|
| brep create/delete | ✅ `create-{vertex,edge,loop,face,shell,solid}` / `delete-{…}`, id-keyed, delete captures payload + severed cascade |
| brep `move-vertex{vertex_id,new_point}` | ✅ approved (`move` = absolute spatial reposition) |
| brep curve/surface | ⚠️ **CORRECTED**: `replace-curve{edge_id,new_curve}` / `replace-surface{face_id,new_surface}` — NOT `change-*`. A NURBS curve has control points the editor edits individually |
| brep booleans/Euler/sweep/offset/fillet | ✅ NOT new verbs — `group_id`-batched sets of the primitives above, diffs from pure engine fns against `base` |
| brep tessellate/measure/validate | ✅ inferences, never mutations |
| drawing fill | ⚠️ **`replace-fill`** — verified: `FillStyle` is an enum with `LinearGradient{…, stops: Vec<GradientStop>}` / `RadialGradient{…}`; gradient stops are edited piecewise → structured |
| drawing stroke | ⚠️ **decompose** — verified: `StrokeStyle` is a 5-field struct (`color`,`width`,`cap`,`join`,`dash`) whose fields are independently set by the editor. NOT one cohesive facet, so neither `update` nor a single `replace-stroke`: author `change-stroke-color`, `change-stroke-width`, `change-stroke-cap`, `change-stroke-join`, `replace-stroke-dash` (the dash array is structured) |
| drawing structure | ✅ `create/delete-{layer,node}`, `move`, `drag-nodes` (separate PLURAL mutation, never a `Vec` arg on the singular verb), `rotate`, `scale`, `reorder-nodes{from,to}`, `group`/`ungroup`, `flatten`/`unflatten`, `replace-path{node_id,new_segments}` |
| mesh `set-primitive-geometry`→`replace-primitive-geometry` | ✅ reasoning approved, ❌ **but it is SMO's to do, not ours** — `✳️mesh` is a stdio subset in their lane. Do not touch |
| flow `update-widget`/`update-synapse` | ❌ **REJECTED**. The existing `Patch` arm is an option-bag of `Option<field>`s, which the taxonomy forbids as a mutation payload outright (option-bags may survive only as diff-INTERNAL types). Decompose per the widget's real fields: `rename-widget`, `change-widget-<field>` per scalar, `move-widget`, `resize-widget`, `replace-widget-<payload>` for structured blobs. Same for synapse |
| platform `set-panel-visibility{panel,visible}` | ✅ approved — a real address (`panel`) plus one field is exactly the narrow case `set` survives for |
| platform `set-active-app` | ⚠️ **CORRECTED**: `change-active-app{new_id}` — there is no address; `id` is the new *value* and the target is the snapshot root, i.e. a document-level scalar |

**Standing lesson SMO has now issued to three sessions in one day:** `update` is not a generic "modify". It survives only where ≥2 fields are genuinely inseparable and never meaningfully set one at a time. **Measure the fields that actually move before choosing it.** A `Patch` with all-optional fields is proof they are set one at a time.

### ⚠️ Before deriving a verb, read BOTH rule documents

The general rule and the specific rule live in **different files**, and the specific one wins:

- `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` — the **verb table**: the general axis (scalar vs structured, addressed vs root, authored body vs setting).
- `../SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the **shape rules**: id-keyed collection, ordered collection, edge collection, hierarchy. Rule 5 is the hierarchy rule: `move-to-<container>{id, new_parent}`.

This is not hypothetical. DKM derived `change-folder-parent` for tree re-parenting by applying the verb table correctly — one scalar link field, identity and contents preserved, therefore `change` — and was still wrong, because derivation rule 5 already covered that shape. **The measurement was right; the rulebook was incomplete.** That is a different failure from reasoning-from-memory and it does not respond to the same fix: measuring harder would not have helped. The mitigation is procedural — **check whether a specific rule already covers this shape before deriving from the general one.**

Related discipline, same root: **a bare identifier grep is a *search*, not a *census*.** "Referenced 70 times" and "70 things to fix" are different claims and must never be quoted interchangeably. A grep for `CollectionMutation` returned 9 framework files and was reported as one surface; it was two unrelated types sharing a name. The same trap recurred twice more the same day: `🧊️3d`/`◻2d`/`🌿️vcs`/`🪐️space` name several different things each, and `Vec3` is **three** unrelated types (math `f32`, brep `f64`, engine `[f64;3]` alias) whose references were being quoted as one 1117-strong blast radius.

### ⚠️ The evidentiary-bar rule (adopted from APA, who reached it the hard way)

> **When a conclusion would license an action you would otherwise consider off-limits, that is precisely when the evidentiary bar goes up, not down.**

APA twice concluded that a blocking file was unowned — first as "orphaned debt from a closed ticket", then as "an abandoned session" — and each conclusion would have licensed editing another session's live file. Both were disproved by a single `stat` in one step, and the evidence was available before the claim was made each time. The specific error the second time: **treating absence of a reply as evidence.** It is only absence of evidence. *A peer that is heads-down looks exactly like a peer that is gone, and the two are distinguished by the file, not by the channel.*

Corollary, binding here: **DKM has no procedure for adopting another session's file on the strength of silence, and will not acquire one.** If a file you need is owned and its owner is unresponsive, the answer is to work on something else and say so — never to define a threshold past which taking it becomes acceptable.

Why the *absence* of a procedure beats a carefully calibrated one: a threshold is a thing you can be argued down. Any criteria for "when it becomes acceptable to take someone's file" will eventually be met by a session that is merely busy, because **the evidence available over a channel cannot distinguish busy from gone.** Removing the procedure removes the failure mode.

**Not all wrong beliefs are equal — sort yours by what they would license.** Five sessions made comparable measurement errors in one day, but they divide sharply:
- **Inert**: a stale error count, a sequencing problem that didn't exist. Wrong beliefs that cost some time and nothing else.
- **Action-licensing**: "this file is orphaned debt", "that session is gone". Each would have authorised writing to another session's live file.

The second kind is what the evidentiary-bar rule exists for. Before acting on a conclusion, ask which kind it is; if it would unlock something you'd otherwise refuse, treat that as a reason to measure again rather than as permission.

## Hard rules (non-negotiable)

1. **No git-modifying commands, ever** — no `commit`, `stash`, `checkout`, `reset`, `rebase`, no worktrees. `isolation: "worktree"` is forbidden on every Agent/Workflow call. The tree is shared with four other live agent sessions plus a human.
2. **The repo AUTO-COMMITS** (commits look like `🐙️ueli🎆️26🌙️MM☀️DD🚩️<n>`, incrementing). `git status` is therefore **not** a churn detector: work that landed minutes ago shows clean, and a vanished edit was committed, not lost. Detect churn with `git log --oneline -5 -- <path>`, `stat -f '%Sm' <path>`, `git log --oneline -3`. Never run a git command to "recover" anything.
3. **All temp files, logs, scratch and reports go inside this ticket folder.** Scratch logs are `.txt` or `.md`, **never `.log`** (`*.log` is repo-gitignored and silently drops out of `ticket_close`'s file list).
4. **Never call `ticket_close` or `ticket_reopen`.** Only the coordinator closes this ticket, with an explicit path.
5. **Never run bare `cargo check`/`build`/`test`.** Always `-p <crate>`, always `CARGO_TARGET_DIR=<this-ticket>/🎯️target`. The lock serializes concurrent checks — "Blocking waiting for file lock on build directory" is normal; wait, do not kill it. **A red workspace is not a red crate.** No workspace-wide check before W5.
6. **`bun` + `nx` only.** Permanent scripts live in `script.ts` at the relevant directory; `project.json`/`package.json` only call it. Temporary one-shot scripts go in this ticket folder.
7. **Greenfield.** No compatibility layers, no legacy support, no deprecations, no migration scripts, no adapters. Delete and handcraft the replacement. Regenerate fixtures; never upcast them.
8. **Regions** (`//#region Name` / `//#endregion Name`) for all structure. Extend existing files; no new files outside the taxonomy shapes. No new test files — extend existing ones. No example files.
9. `[DEBUG] ` prefix on every temporary log line, removed before the wave is reported done.
10. **Docstrings start with a unique fitting emoji.** No comments inside definitions.
11. **Never claim a test passed without running it.** Paste the real command and its real output into your report. A Haiku scout re-runs your claimed commands independently — a discrepancy is treated as a failed wave, not a rounding error.

## Repo gotchas that have already cost other sessions real time

12. **Derive crates keep two byte-identical copies**: `<module>/✨️derive/🦀️component.rs` **and** `<module>/✨️derive/📦️packages/🦀️rust/📦️glue.rs`. Cargo compiles the **glue** copy — editing only `component.rs` silently does nothing. Mirror by hand, then `diff -q`. (`mcp__repo__file_integrate` has corrupted this mirroring before.)
13. **`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` is dual-mounted** — into `semio-framework` as `io` and into `semio-framework-os-kernel` as `os_io`. One edit, two crates. (UCAS-claimed; read-only for us.)
14. **Additive struct fields still break struct literals** (serde `default` affects (de)serialization, not Rust construction). After adding a field, `grep -rln "TypeName {"` across the whole workspace and fix or file every literal.
15. **Adding an enum variant is expensive** where the enum is matched exhaustively. Measure with `grep -rln "EnumName::"` first.
16. **The semantic-vocabulary policy greps raw file content including comments.** Prose merely *naming* the banned identifiers trips it. Never write them in docstrings or comments anywhere under `✏️s/`. (This ticket folder is exempt — it is not under `✏️s/`.)
17. **`semio-framework-os-kernel-db` is pre-broken** (~53 errors: a stale `#[path]` in its `📦️glue.rs` pointing at `📄️document` after a rename to `📄️artifact`, plus cascading unresolved imports). Tracked as `task_9a4155cc` by the inference-family session. It predates DKM — if your lane fixes it, report it in its own clearly-labelled section, never blended into DKM's diff.

## ⚠️ Full paths, never bare emoji — this ticket's most likely self-inflicted wound

These glyphs name **different things** in different places. Every report, message and hot-file row MUST use the full path:

| Glyph | Meanings that coexist |
|---|---|
| `🧊️3d` | framework module `🧰️framework/🔨️modules/🧊️3d/` **and** plugin subsets `✏️s/🔌️plugins/{puzzle,fem,process,procedural}/🗿️artifacts/🧊️3d/` |
| `◻2d` | framework module `🧰️framework/🔨️modules/◻2d/` **and** plugin subsets under `✏️s/🔌️plugins/*/🗿️artifacts/◻2d/` |
| `🌿️vcs` | **UCAS-claimed** `💻️os/🔨️modules/🌿️vcs/🦀️component.rs` **and OUR target** `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` |
| `🪐️space` | **our target** `💻️os/🔨️modules/🪐️space/🦀️component.rs` **and SMO's** plugin `✏️s/🔌️plugins/🪐️space/**` |
| `🛂️manifest` | UCAS's `🧰️framework/🔨️modules/🛂️manifest/` **and** APA's per-plugin `✏️s/🔌️plugins/*/🛂️manifest/` |
| `⚙️engine` | **our** `💻️os/🔨️modules/⚙️engine/` (EngineCache) **and** dozens of per-artifact `⚙️engine/` dirs in plugins |

## Hot-file ownership (binding)

| File / subtree | Owner | Everyone else |
|---|---|---|
| `🧰️framework/🔨️modules/🧊️3d/**` | the assigned W3a lane agent for that file cluster | read-only; file a `sharedFileRequests` entry |
| `🧰️framework/🔨️modules/◻2d/**` | W3a drawing agent | read-only |
| `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` | W3a mesh agent | read-only |
| `🧰️framework/🔨️modules/🗺️surface/<subdir>/**` | that subdir's W3b agent | nobody else |
| `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` | W2 exemplar agent, then frozen | read-only |
| `💻️os/🔨️modules/⚙️engine/🦀️component.rs` | W1 mechanism agent, then frozen | read-only |
| `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` | W3c flow agent | read-only |
| `💻️os/🔨️modules/🪐️space/🦀️component.rs` | W3c space agent | read-only |
| `💻️os/🔨️modules/🛢️db/**` | W3c db agent | read-only |
| `🧰️framework/🔨️modules/🧮️math/**` | DKM (**claimed late**, announced to IIF + APA, no objection) | read-only |
| repo-root `📜️script.ts`, `🔣️taxonomy.json` | W6 ratchet agent ONLY (queue position 5) | read-only, even for allowlist entries — request it |
| `✏️s/🔌️plugins/🗄️stdio/**` | **UCAS, not us** — pending handoff | do not enter without the coordinator's explicit go |
| `✏️s/🔌️plugins/🌊️flow/**`, `✏️s/🔌️plugins/🪐️space/**` | **SMO** | do not enter; file a `sharedFileRequests` entry |

## Cross-session protocol — FIVE sessions share this tree

| Session | Ticket | Address (re-resolve every wave) |
|---|---|---|
| DKM (us) | `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` #2550 | this session |
| UCAS | `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` #2548 | `semio-b2 [8f1c0b]` |
| APA | `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` #2549 | `semio-52 [1db563]` |
| SMO | `SEMANTIC-MUTATIONS-OVERHAUL` #2545 | `semio-9f [edf593]` |
| IIF | `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` | `uds:/tmp/cc-socks/64627.sock` |

Names drift — re-run `ListAgents` before every handshake; never cache an address across waves.

### ⚠️ A document written by a session is a derived artifact of that session, not a live predicate about it

That oracle has been proven unreliable in both directions in this tree. Ask the session.

The worked example that makes this bite, rather than sound obvious: UCAS's `📓️status.md` is **their own authoritative record, minutes old, written by the owner**, and it says *"W1 (kernel): CODE COMPLETE."* It is still **not a release** — because another line of the same document says *"Signal APA when C1 unfreezes the file"*, deferring to a signal that had not been sent. **"Complete" and "released to you" are different claims, and only the owner can make the second.**

Without that distinction the rule collapses into "don't trust documents", which is both wrong and unusable. The point is not that documents lie; it is that a document answers the question its author was asking, which is rarely the question you are asking of it.

**Count your gates, and say the count out loud.** DKM holds SMO's verb approval and IIF's deferral for the three stdio subsets, and has entered zero files there, because UCAS's handoff is a third gate. Two of three open is not open. When you are tempted to proceed, state which gates are open and which is not — the tempting reading is almost always the one that quietly drops a gate from the list.

**Standing agreements (coordinator maintains; agents obey):**
- **We build ON UCAS's primitives** (`ArtifactRef`, Composition regions, `MutationMeta.group_id`, `UndoGroup.member_edits`) — never a parallel mechanism.
- **SMO owns the verb vocabulary.** Every verb above was submitted for their review before authoring. Four mechanical gates apply to every facet we write: triad dirs ↔ dispatch variants 1:1 in both directions; unique emoji per sibling dir within a facet; real leaves (a genuine `impl MutationKind<`, a real `pub fn diff` built from `(payload, base)`, a real `pub fn inverse` from `base` returning `Vec::new()` when the target is absent); a non-stub `🟦️component.ts` beside every triad `🦀️component.rs`. **If a facet cannot be authored conformingly, leave the enum EMPTY with no triad dirs and flag it** — never invent vocabulary.

  ### ⚠️ The four gates are NECESSARY, NOT SUFFICIENT — a structural audit is not a correctness audit

  The gates verify that a `pub fn diff` **exists**. They cannot verify that it **behaves**. A facet can satisfy all four and still fail its own round-trip laws.

  This is not hypothetical. UCAS's `✳️text` facet was audited against all four gates by the vocabulary owner, passed with zero rework, was relayed as "done and audited clean" — and then produced **6 real failures** when the long suite was actually run (`insert_remove_run_round_trips`, `add_remove_mark_round_trips`, `reorder_runs_round_trips`, plus three composition laws). Two further subsets, `✳️table` and `✳️graph`, had already been authored from it as a template before anyone ran the tests.

  **Binding consequence for DKM: no facet this ticket authors is "done" on a gate pass.** Every triad requires its law tests actually executed — inverse round-trip, diff-consistency, determinism — and the real output pasted into the wave report. This matters most for the brep and drawing lanes, which will author ~30 triads between them; signing those off structurally would reproduce exactly the above at ten times the scale.

  Corollary worth remembering separately: **a fragile test harness is a defect announcing itself.** UCAS's `✳️text` harness needed a special case with a comment explaining why the inverse had to be diffed against current state rather than `base`. That comment was the bug, written down, months before it was recognised. If a law test needs an exception to pass, the exception is the finding.
- **SMO handed us `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` explicitly** — it is a kernel bridge that makes their plugin-side elimination impossible. Their `✏️s/🔌️plugins/🌊️flow/**` dispatch has `from_framework_mutation`/`to_framework_mutation` converting into that kernel enum. **Send SMO the target enum shape BEFORE authoring** so they can update or delete the bridge. Same for `🪐️space`.
- **APA owns escape-hatch deletion** (`register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`) and the declarative registration shape. We don't touch them.
- **IIF owns the `💡️inference` fan-out** for ~31 stdio subsets. They have explicitly excluded `✳️brep`/`✳️drawing`/`✳️mesh` and deferred them to DKM.
- **`📜️script.ts` order: APA → UCAS-W6 → SMO → IIF → DKM (position 5).** Announce on all channels before and after. Report-mode first, always; a rule that gates before the tree is clean blocks four other sessions for a violation they did not create.
- **Never "fix" another session's file.** On a red compile outside your boundary: retry the scoped check 3× at 60s intervals; if it persists, grep the cargo output to prove zero errors originate in your own paths, record it under `## Concurrent-churn observations`, report `blocked-churn`, and stop.
- **SMO's plugin release status is a live file, not a cached fact**: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. Read it, don't remember it.

## ⚠️ For W6's policy authoring: measure REACH, not MUTABILITY

Discovered by APA when DKM's brep design surfaced `📐️cad`'s `static HOST: OnceLock<BrepEngineHost>`. Their `PolicyRulePluginPurity` **does not catch it**, deliberately: the rule exempts bare `OnceLock`/`OnceCell`/`LazyLock` as write-once-by-type, because every artifact's `io_registry` uses `static ENTRIES: OnceLock<Vec<ComposerEntry>>` and flagging those would drown the signal.

The rule is measuring the wrong property:

| Site | Mutability | What it actually is |
|---|---|---|
| `OnceLock<Vec<ComposerEntry>>` | write-once | a plugin caching **its own immutable data** — fine |
| `OnceLock<BrepEngineHost>` | write-once | a plugin holding a **handle to host-owned engine state** for the process lifetime — the exact anti-pattern this ticket exists to remove |

Identical by mutability, entirely different violations. In APA's phrasing: **it isn't ambient *mutability*, it's ambient *reach*.** A `OnceLock` makes the handle unforgeable after init and does nothing about the fact that a plugin has one at all.

**Binding consequence for DKM's W6 rules.** `policyEngineCacheScopeBreaches`, `policyEngineRepEscapeBreaches` and `policyEngineConsumptionOutsideFacetBreaches` must be written to detect **what a construct reaches**, not whether it mutates. A rule keyed on `&mut`/`static mut` will pass every handle-shaped violation. And widening a mutability rule to cover reach produces false positives against sanctioned tables — the two need **separate** checks, not one broadened one.

## ⚠️ A FACET MUST LAND ATOMICALLY — a partial facet is a workspace-wide build failure

Learned expensively: DKM's brep and drawing authoring took `semio-s-plugin-stdio` red, and **every plugin in the repo depends on stdio**, so four other sessions could not gate anything for the duration.

Three references point at each other, and a gap in any one is a **hard build error, not an incomplete feature**:
- the **triad leaves** (`🦠️mutation`/`🔺️diff`/`↩️inverse`) reference **enum variants**;
- the **dispatch enum** references the **triad modules**;
- **`📦️glue.rs`'s `#[path]` mount** references the **triad directories**.

**Binding rules:**
1. **Author one triad at a time — mutation + diff + inverse + enum variant + glue mount in the SAME change — and compile after each one.** Never batch several triads and reconcile at the end.
2. **Never delete a mounted directory without removing its mount in the same change.** A `#[path]` (or `Cargo.toml` `members`) entry pointing at a vanished path **aborts the build before compilation**, so it is not hidden behind `--tests` and it breaks every session on the machine, not just yours. This is the same failure that took the whole workspace down earlier via a relocated `🖍️draw/🔄️fsm`.
3. **If a verb cannot be authored conformingly, remove its directory, its enum variant and its mount together.** A missing verb is fine; a dangling reference is not.
4. **Generate mount paths by listing the directory — never hand-type them.** The emoji segments are unicode-normalization traps: a literal `↩️inverse` typed into a script silently failed to byte-match the on-disk name and produced *empty modules*, which presents as "the triad doesn't work" rather than "the path is wrong".

**The coordinator's error, recorded because the lesson is about propagation, not knowledge:** I had personally recorded the `🔄️fsm` lesson hours earlier — *a directory containing a `Cargo.toml` is inventory-only; a dangling workspace member breaks cargo machine-wide* — and then wrote two dispatch briefs that walked straight into the `#[path]` variant of it. **Knowing a rule and propagating it into the instructions that need it are different acts**, and I had done only the first.

**And the boundary lesson**, adopted by a peer as a general rule: the agents owned the triad *directories* but not the *mount that makes them real*, so every deletion was guaranteed to strand a reference in a file they could not touch. **A boundary that separates a definition from its registration is not a boundary, it's a race.** Whoever owns one must own both for the duration.

## 🔍 Sweep the pattern, don't wait for the compiler

A scoped `cargo check` only surfaces the crate you asked for, and a workspace build stops at the first failing crate — so latent instances of a known defect stay invisible for hours, surfacing one crate at a time.

Proven twice in one evening: a 30-second script walking every `#[path = "…"]` in the repo and `stat`-ing its target checked **8,328 mounts and found 20 dangling**, including two nobody had hit — one where a rename left `📄️document`→`📄️artifact` stranded, and one where **the emoji changed** (`➕create-widget` → `🌱create-widget`) so a text search for the slug still matched and only a filesystem check caught it. Separately, grepping for a *pattern* (`Self::infer` with its trait imported inside `mod tests`) found a second latent instance in 30 seconds that the compiler would have revealed hours later.

**When you fix a defect, grep for its shape across the whole tree before declaring it fixed.**

### …but: **grep to find, enumerate to count**

The other half, learned the same evening by two sessions making the *identical* error within an hour:
- One grepped a symbol, got **46 files**, and reported 46 broken. Exactly **1** imported it via the wrong path; the other 45 merely *used* it. "Mentions" is not "is broken".
- The other pattern-matched `#[path = "…"]`, got **20 dangling**, and broadcast that. **18 were prose inside `//!` doc comments** — one "dangling target" was literally the string `...`. Real count: **2**. They made this error *in the message praising the first session's retraction of the same error*.

**A pattern match locates candidates; it does not size a problem.** Check what each hit actually *does* before quoting a number — especially before sending it to another session, where an inflated number can trigger a wave of work against nothing.

## 🚪️ BINDING USER RULING (2026-08-13) — io flows through artifacts; effort is not an exemption

> **"All importers and exporters must flow over the io mechanism of artifacts. Everything must be migrated to artifacts, no matter the effort, unless it is domain-neutral framework functionality."**

Two consequences that override earlier reasoning in this ticket:

1. **"This would be a large job" is NOT a reason to stop.** Several waves have parked work as blocked when the real obstacle was size. Size is now explicitly not a blocker. A **proven structural impossibility** (a genuine Cargo cycle, demonstrated — not inferred) still is. Distinguish the two and report which one you hit, with evidence.
2. **The only exemption is DOMAIN-NEUTRAL framework functionality.** Test it by asking whether the code names a domain. `ComposerEntry`/`register_composer_entries`/`resolve`/`io_dispatch`/`IoKey`/`Dialect` in `🧰️framework/🔨️modules/🚪️io/` are generic dispatch over dialects and know nothing about geometry → **exempt, they stay, route everything through them.** `register_solid_exporter`/`register_mesh_importer`/`register_dwg_import_handler`/`SolidExporter`/`MeshExporter` name solids, meshes and DWG → **domain-specific, no exemption, they become artifact `🚪️io` facets.**

**Do not build a parallel io mechanism.** The artifact-side shape already exists and is in production: `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/<format>/🔖️<std>/✳️<subset>/` implementing `ArtifactDeserializer`/`ArtifactSerializer` (`type From`/`type Into`, `const FROM`/`INTO: Dialect`), registered via `register_composer_entries`, resolved by `io_dispatch`. Read a working pair and mirror it.

### ⚠️ The blocker that wasn't — check WHICH CRATE before declaring a cycle

Wave G5 stopped the brep dissolution on "`semio-framework-os-kernel` is framework-tier and can never depend on stdio, so framework-3d's kernel cannot be deleted." **Measured: false.** os-kernel defines none of those registries and does not depend on `semio-framework-3d`. They are compiled by **`semio-framework-os`** (`💻️os/🖥️host/📦️packages/🦀️rust`), which **already depends on both framework-3d AND stdio**. No cycle existed.

The trap: `💻️os` names several crates, and the genuinely-dead `💻️os/🦀️component.rs` sits beside the live `💻️os/🖥️host/🦀️component.rs`. **Before declaring a dependency cycle, name the crate from its `Cargo.toml` and read that manifest's actual deps.** A cycle claim licenses abandoning work — so it carries the same evidentiary bar as a deletion claim.

## 🧿️ Where a dissolved kernel's artifact goes — BINDING (settled with the stdio roster owner, 2026-08-13)

The `🧿️semio` v1 subset roster is **frozen at 18 + `✳️any`**. The bar for a 19th was never "nothing new ever" — it is *"genuinely shared content shape needed by ≥2 independent plugins"* (the bar `mesh`, `graph` and `kit` each cleared). Nothing in the math dissolution clears it today. **So: plugin-owned artifacts, no new stdio subsets.**

| Dissolved content | Home |
|---|---|
| CAS + polynomial | `➗️mathematical`'s own artifact — Equation/Function |
| WFC | plugin-owned Assembly (procedural-flavoured), **composing `✳️kit` children + `✳️value` rules** |
| statistics / entropy / probability / tabular / causal | **inferences over the existing `✳️table` + `✳️value`** — NOT a new artifact |
| sampling (an LLM token-sampler misfiled under math) | neural-flavoured plugin home |
| fuzzy, number theory | inference helpers under whichever artifact needs them |

**The cost of plugin-ownership is smaller than it looks, and this is the load-bearing fact:** `ArtifactChild` (owned composition) does require a stdio snapshot type, but **`ArtifactLink` (reference) does not** — any plugin can bind a link to ANY artifact in ANY plugin. So a plugin-owned artifact is still fully *referenceable* repo-wide. The only thing forgone is being an *owned child of two different parents at once*, which none of these want.

**Two design traps this ruling exists to prevent:**
1. **Do not mint private `Slot`/`Module`/`Rule` type towers.** `✳️kit` already generalizes "types/designs"; rules are `✳️value`-shaped data. Minting private equivalents re-creates the duplicated-vocabulary problem this whole ticket removes, one layer up.
2. **A directory name is not a content shape.** "statistics" reads like it wants a `✳️statistics` subset; measured, moments/fits/entropy/causal-queries are *derivations over tabular data* — inferences on `table`+`value`, not a new persisted shape. Sorting these correctly moved ~20k LOC from "design new artifacts" to "author inferences on existing ones."

If a case genuinely acquires a second independent plugin consumer, reopen it with the specific consumer named — not with a general argument.

### 🛑 A MOUNT IS NOT A STRING — to ask "is this file live?", RESOLVE the paths

"Is `X/🦀️component.rs` mounted?" cannot be answered by grepping for its name, because relative mounts routinely name a file whose distinguishing path segments **do not appear in the mount text at all**.

Worked example, which cost two agents a wrong answer *in opposite directions* on the same file. `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` is mounted — by `💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs:27` as:
```rust
#[path = "../../🦀️component.rs"]
```
The string contains no `🖥️host`. So `grep -rn '🖥️host/🦀️component.rs'` returns **only unrelated files** — `🧊️3d/📐️brep/⚙️engine/🖥️host/…` and `🌊️flow/🖥️host/…`, two *different* files that happen to share the segment name. One agent concluded "unmounted, dead"; another concluded "unmounted twin of the real one"; both were wrong, and one of them nearly blocked a deletion on it while the other nearly licensed editing a live file.

**The reliable method** — resolve every `#[path]` against its containing directory and compare realpaths:
```python
resolved = os.path.realpath(os.path.join(os.path.dirname(source_file), path_attr))
```
Run that over the tree and compare to your target. It also gives you the dangling-mount sweep for free.

Two corollaries that keep biting here: `🖥️host`, `⚙️engine`, `🧊️3d`, `◻2d`, `🌿️vcs`, `🪐️space` each name **several unrelated files**, so any bare-segment grep is ambiguous by construction. And "this file is unmounted/dead" is an **action-licensing** conclusion (it invites deletion) — so per the evidentiary-bar rule it needs the strongest available evidence, which is path resolution, never a substring match.

### 🛑 A CRATE IS NOT A DIRECTORY — never scope a dependency census by folder

This repo composes crates out of `#[path]` mounts that reach **across the tree**, so "does module X use Y" cannot be answered by grepping X's folder. The coordinator got this wrong and broke the build for two other waves:

`grep -rn "semio_framework_math\|\bmath::" "🧰️framework/🔨️modules/🖱️ui" --include="*.rs"` → **0 hits**, so the `semio-framework-math` dep in `🖱️ui`'s `Cargo.toml` was deleted as dead. But `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:222` mounts
`#[path = "../../../../../🧊️3d/🎬️scene/🦀️component.rs"] pub mod kernel_3d_scene;`
and **that** file's line 3 is `pub use semio_framework_math::algebra::{Mat4, Vec3};`. The ui crate was a real math consumer through a file living five directories away. Result: 6 errors in `semio-framework-ui`, which is upstream of nearly everything — two independent waves then measured it as their own baseline and **both attributed it to foreign churn**, because a plausible story ("`🧊️3d` is being dissolved by another wave, so this is theirs") fit perfectly.

**Two rules from this:**
1. To census a crate's real inputs, read its `📦️glue.rs` `#[path]` mounts first and include every mounted file, wherever it lives. Or ask cargo instead of the filesystem: remove the dep, `cargo check -p <crate> --all-features`, and let the compiler answer.
2. **A "pre-existing foreign error" that appears in more than one wave's baseline at the same moment deserves one minute of suspicion before it is written off** — especially when it sits in a crate that everything depends on. Shared novelty is evidence of a shared *recent cause*, and the most available cause is your own last change.

### 🛑 Commit-message dates are a STALE FIXED TEMPLATE — never attribute with them

Every auto-commit subject in this repo reads `🐙️ueli🎆️26🌙️06☀️04🚩️<n>` — the `🎆️🌙️☀️` date is a **frozen template string**, not the commit's date. Measured 2026-08-13:

```
git log -3 --date=iso --format='%h | %cd | %s'
515271bf60 | 2026-08-13 13:05:26 +0200 | 🐙️ueli🎆️26🌙️06☀️04🚩️503   ← says "June 4", really Aug 13
3550b3dc09 | 2026-08-13 10:05:39 +0200 | 🐙️ueli🎆️26🌙️06☀️04🚩️502
```

So `git log --oneline` is **actively misleading** for attribution: it makes today's churn look months old, which is precisely the direction that licenses the dangerous conclusion ("this file is stale/abandoned, therefore I may edit it"). The coordinator recorded one such attribution tonight ("June 2026 commits only") off this artefact; re-checked with real dates, the conclusion happened to survive on independent mtime evidence, but the stated reasoning was worthless. Flagged by UCAS, confirmed independently here.

**Use `git log --date=iso` (or `%cd` with `--date=iso`) and `stat -f '%Sm %N'`. Never quote a date read from a commit subject.** Note also that `stat` mtime is corrupted by your own `touch`-to-force-recheck — if you touched a file to defeat the cargo cache, its mtime is now yours and proves nothing about its author.

### 🛑 THREE mechanisms that manufacture FAKE compile errors — build directly before believing a failure

All three hit this tree in one evening. Each produces output a careful reader would reasonably file as "that crate is broken", and **none of them is a compile error**:

1. **Disk full.** `cargo` fails with `No space left on device`, which renders indistinguishably from real diagnostics. It swung one crate's count **94 → 16 → 116** across three runs. A count taken on a full disk is not a measurement — **`df -h` first**.
2. **Mid-transaction windows.** A wave that repoints an import before landing the mount (or deletes a directory before removing its `#[path]`) leaves minutes where a *correct* consumer names a *not-yet-existing* path. Two sessions recorded phantom failures this way tonight. Re-measure before reporting; the window may already have closed.
3. **Silent wall-clock kills.** The build harness enforces `SEMIO_BUILD_BUDGET_MS` via `runCmdStatus`, and under `FORCE_PLUGIN_BUILD=1` slow modules get killed with **non-zero status and NO error text**. Two crates reported as "build failed" compiled standalone in ~2.4s. **A timeout and a broken crate look identical in that harness.**

**The defence that works: when a failure has no error text, or the error names something you just changed, build the target directly before believing the report.** Discovered/confirmed by the state-architecture session; recorded here so it is not rediscovered one mechanism at a time.

### ⚠️ Two cargo artefacts that manufacture false confidence

1. **`cargo check` does not compile `#[cfg(test)]`.** Six separate instances in this repo in one day, including test code that **landed unverified in a closed ticket** because its own gate could not see it. Use `--tests`/`--all-targets`, and treat a green `check` as saying nothing about test code.
2. **A cached `cargo check` re-emits no diagnostics.** A second run over an unchanged crate prints nothing and exits 0 — which looks identical to "clean". The coordinator nearly reported stdio green off exactly this artefact. **If a result matters, `touch` a file in the crate to force a real recheck**, and treat any zero-diagnostic run you did not force as unverified.

## Report shape (every agent, every wave)

Append to your assigned report file in this ticket folder. Sections, in order: **what changed** (file:line + grep anchors) · **files touched** (created/updated/removed) · **verification commands run, with real output pasted** · `## sharedFileRequests` (file, region, reason, patch file path) · `## Concurrent-churn observations` · honest pass/fail. A wave is not done until its report exists.
