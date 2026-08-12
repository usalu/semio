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

Names drift — re-run `ListAgents` before every handshake; never cache an address across waves. **Never infer a live session's state from static files it wrote** — that oracle has been proven unreliable in both directions in this tree. Ask the session.

**Standing agreements (coordinator maintains; agents obey):**
- **We build ON UCAS's primitives** (`ArtifactRef`, Composition regions, `MutationMeta.group_id`, `UndoGroup.member_edits`) — never a parallel mechanism.
- **SMO owns the verb vocabulary.** Every verb above was submitted for their review before authoring. Four mechanical gates apply to every facet we write: triad dirs ↔ dispatch variants 1:1 in both directions; unique emoji per sibling dir within a facet; real leaves (a genuine `impl MutationKind<`, a real `pub fn diff` built from `(payload, base)`, a real `pub fn inverse` from `base` returning `Vec::new()` when the target is absent); a non-stub `🟦️component.ts` beside every triad `🦀️component.rs`. **If a facet cannot be authored conformingly, leave the enum EMPTY with no triad dirs and flag it** — never invent vocabulary.
- **SMO handed us `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` explicitly** — it is a kernel bridge that makes their plugin-side elimination impossible. Their `✏️s/🔌️plugins/🌊️flow/**` dispatch has `from_framework_mutation`/`to_framework_mutation` converting into that kernel enum. **Send SMO the target enum shape BEFORE authoring** so they can update or delete the bridge. Same for `🪐️space`.
- **APA owns escape-hatch deletion** (`register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`) and the declarative registration shape. We don't touch them.
- **IIF owns the `💡️inference` fan-out** for ~31 stdio subsets. They have explicitly excluded `✳️brep`/`✳️drawing`/`✳️mesh` and deferred them to DKM.
- **`📜️script.ts` order: APA → UCAS-W6 → SMO → IIF → DKM (position 5).** Announce on all channels before and after. Report-mode first, always; a rule that gates before the tree is clean blocks four other sessions for a violation they did not create.
- **Never "fix" another session's file.** On a red compile outside your boundary: retry the scoped check 3× at 60s intervals; if it persists, grep the cargo output to prove zero errors originate in your own paths, record it under `## Concurrent-churn observations`, report `blocked-churn`, and stop.
- **SMO's plugin release status is a live file, not a cached fact**: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. Read it, don't remember it.

## Report shape (every agent, every wave)

Append to your assigned report file in this ticket folder. Sections, in order: **what changed** (file:line + grep anchors) · **files touched** (created/updated/removed) · **verification commands run, with real output pasted** · `## sharedFileRequests` (file, region, reason, patch file path) · `## Concurrent-churn observations` · honest pass/fail. A wave is not done until its report exists.
