# Interactive-Job Migration Recipe — What Must Be True Before `BatchOnlyPendingRewrite` Can Honestly Become `Migrated`

Scope: puzzle3d (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`), cross-checked against `✒️writer` (100% migrated) and against a prior parallel investigation for `💠️lowpoly`
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️research/📝️interactive-job-migration-recipe.md` — different plugin, same framework, consistent conclusions).

**Bottom line, stated up front:** the label is *not* the only difference — there is a real, actively-enforced structural contract behind `Migrated`. But puzzle3d is in an unusual state: the *tool-level* job implementations for ~50 of its 60 remaining actions **already exist as dead code** in the same file. What's missing is one piece of *app-level* infrastructure — an `ArtifactStoreOneItemPreparationFactory` for the Snapshot/Artifact store — that the existing dead code has no way to commit through yet.

---

## 1. Every validator/gate that reads `InteractiveJobClassification`

Found five independent gates, at two very different depths. `grep -n "InteractiveJobClassification" 🧰️framework/**/*.rs` for the full call-site list.

### 1a. `validate_interactive_job_classification` — `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:950-968`
Rejects only `Unclassified`. `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, `Deleted` and `Migrated` all pass. Its own doc comment (line 947-949) says exactly this: "Deleted and batch-only declarations are classified data, while UI dispatch separately rejects dispositions that are not `Migrated`." This function is a completeness check, not a UI-safety check.

### 1b. `validate_ui_dispatch_classification` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11915-11921`
```rust
fn validate_ui_dispatch_classification(owner: &str, id: &str, classification: ...) -> Result<(), Fault> {
    if classification == InteractiveJobClassification::Migrated { Ok(()) }
    else { Err(Fault::new(..., "interactive-job.not-ui-safe", ...)) }
}
```
Called from `dispatch_action` (line 21872) and `dispatch_command` (line 21919), *before* command construction. **In isolation this is a pure boolean label check** — equality against one enum variant, nothing about the handler's shape. This is the gate ESTABLISHED FACTS already named.

### 1c. `ActionBus::register` / `register_once` — `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:535-538, 566-569`
A *different* subsystem ("Typed routing from renderer actions to resumable interactive jobs", module doc at line 2-8). Rejects registering a `ToolJobFactory` whose `classification() != Migrated` with `ToolRegistrationError::NonInteractiveClassification`. A `ToolJobFactory` is a real trait (line 328-366): `keys()`, `payload_schema_id()`, `execution_contract()`, `create_job(...) -> Self::Job where Self::Job: InteractiveJob`, plus wire/checkpoint decode hooks. This is infrastructure-shaped, not label-shaped — but nothing *forces* an app to register anything here at all.

### 1d. `ArtifactToolFactoryRegistry::register` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12730-12751`
The plugin-layer wrapper apps actually call from `register_tool_job_factories`. Beyond re-checking `classification == Migrated` (line 12750), it enforces:
- the factory's `TOOL_IDS` is an **exact bijection** with its `keys()` (line 12742-12745),
- a **non-empty publication-lane contract for every tool id**, `HostOnly` must be the sole lane if present (line 12746-12749),
- schema match against the owning `ArtifactApp::DOCUMENT_SCHEMA` (line 12738-12740).

### 1e. `AppActionRegistry::validate_tool_job_rows` (via `tool_job_registration`) — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12059-12137`
**This is the real enforcement point, and it runs at app-registry build time, not at dispatch time.** It computes `migrated = self.migrated_tool_ids()` (every action/command whose manifest classification is `Migrated`, line 12045-12051) and `expected = generated_ids ∩ migrated`, then requires that `A::bounded_first_step_tool_proofs()` contains **exactly one proof row per expected id**, each either:
- **"generic"** — `factory == BOUNDED_FIRST_STEP_FACTORY` with no factory type, unregistered in the bus — or
- **"exact"** — matching a live `ArtifactOwnedToolJobFactory` registration in the `ActionBus` (factory type id/name, owner witness, controller, contract, and `bus.admit_exact_wire(...)` all cross-checked).

If `seen != expected`, the app fails to build with `interactive-job.catalog-incomplete` — "a migrated generated command lacks its exact owner-local bounded reducer proof" (line 12124). If a proof row doesn't match reality, it fails with `interactive-job.catalog-authority` (line 12089-12110), which even names the specific mismatched field in its message. **Flipping a manifest classification to `Migrated` without also adding a matching `bounded_first_step_tool_proofs!` row makes the app fail to construct at all** — this is not optional paperwork, it is checked at startup.

### 1f. Runtime dispatch branch — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22493-22551`
Once classification+proof pass, the *actual command handling* branches on `admission.proof`:
- `QualifiedToolProof::Bounded(_)` → wraps the command in a generic `TypedCommandFullOperationJob<A>` (a real `Prepare → Reducer → Output` cursor state machine, line 16359+).
- `QualifiedToolProof::AppOwned(_)` → calls `A::build_tool_job(...)`, i.e. the app's own factory.

**Both branches are `InteractiveJob`s.** Even the "generic"/single-step case is not the old direct-call path — it's a one-shot job that still carries `phase`/cursor fields. This matches Phase 8's own language (§3 below): "small tools complete in their first step but use the same path."

---

## 2. Structural comparison: the 4 `Migrated` actions vs. `BatchOnlyPendingRewrite` ones

`PUZZLE3D_RETAINED_TOOL_IDS` (line 2480) = `["openAddObjectDialog", "worldPointerDown", "setLocale", "setTerminology"]` — **exactly** the 4 `Migrated` ids. This list is simultaneously: the `ToolFactoryKey` set for `Puzzle3dRetainedCommandJobFactory` (line 6096), `ArtifactOwnedToolJobFactory::TOOL_IDS` (line 6148), the gate in `build_tool_job` (line 6426), and the `tools: [...]` list inside the `bounded_first_step_tool_proofs!` macro invocation (line 6417). All four sync points are driven off one const, so puzzle3d's current classification *is* structurally honest for these 4.

**The structural difference is real, and it is at the *store publication* layer, not (only) at the job-shape layer:**

- `RetainedPuzzleCommandJob` (`crate::retained_command`) is a genuine `InteractiveJob`: staged (`Puzzle3dPrecomputeCommandStage::{Complete, Closing}` etc.), wire-decodable with checkpoint validation (`create_job_from_wire_pages_with_payload`, line 6124-6141), and its commit only proceeds after `validate_wire_checkpoint`.
- Crucially, `worldPointerDown`/`transformBegin`/`transformEnd` route to `NoopPuzzleCommandWork` (line 6480) and `openAddObjectDialog`/`setLocale`/`setTerminology` (plus ~20 others already coded) route to `Puzzle3dScalarConfigWork` (line 6479) — i.e. **the currently-Migrated actions are cheap, scalar, config-only, or no-op actions.** None of them writes to the Snapshot/Artifact document.
- The framework's commit path for *any* retained job — generic or exact — optionally uses an `artifact_one_item_factory: Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>` and a `config_one_item_factory` (`🦀️.rs:18549-18551`, referenced from `TypedCommandFullOperationJob`'s construction). Puzzle3d's `ArtifactApp` impl supplies `build_config_store_one_item_preparation_factory` (→ `Puzzle3dConfigStorePreparationFactory`, line 6397) but **never overrides `build_artifact_store_one_item_preparation_factory`** — confirmed by `grep`, zero matches in the file. So there is currently no bounded, cursorized, "advance/checkpoint/close_step" path (trait at `🏪️store/🦀️.rs:13042-13037`) for committing a `Puzzle3dMutation` into the Snapshot store at all; only the ordinary/legacy publication path exists for that lane.

This is exactly what the plugin's own **authoritative fixture** says, verbatim (`✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json`, cross-checked against source by a TS oracle at `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:25-46`):

| group | status | lanes | blocker | route count |
|---|---|---|---|---|
| 0 | Migrated | HostOnly | — | 2 (`openAddObjectDialog`, `worldPointerDown`) |
| 1 | Migrated | Config | — | 2 (`setLocale`, `setTerminology`) |
| 2 | BatchOnlyPendingRewrite | Artifact | "artifact-lane completion has no app-owned retained preparation and root-retirement factory" | 1 (`addTargetVolume`) |
| 3 | BatchOnlyPendingRewrite | Artifact+Config | "artifact/config completion has neither app-owned retained preparation factory nor bounded root retirement" | 18, **includes `setActiveExample`** |
| 4 | BatchOnlyPendingRewrite | Config | "config-lane completion has no app-owned retained preparation and root-retirement factory" | 35 |
| 5 | BatchOnlyPendingRewrite | HostOnly | "the current empty/effect-only completion does not reproduce the route's semantic runtime transition" | 6 (`fillBuildTick`, `registerBrushMesh`, `suggestionsTick`, `engagementRepeatLast`, `transformBegin`, `transformEnd`) |

Group 5's blocker is a different, second real gap: these actions currently emit an *empty or ui-scope-only* `Emit` from the ordinary reducer (confirmed at `🦀️.rs:6066-6069`, the `Puzzle3dPrecomputeCommand` step match: `"registerBrushMesh" => emit.ui_scope = UiDirtyScope::None`) — the retained-job "completion" object has nothing that reproduces what these actions actually do at runtime (they mutate app-local precompute/session state, not the document), so wrapping them in a job today would silently no-op the real behavior.

**Answer to the "what structurally must be true" question:** an action must (a) be wrapped in a real `InteractiveJob` (generic or exact — this part already exists as dead code for ~50/60 remaining actions, see §5), **and** (b) whatever store lane its `Emit` writes to (Artifact and/or Config) must have a bounded, cursorized `ArtifactStoreOneItemPreparationFactory` wired up on the `ArtifactApp` so the commit itself is resumable/bounded rather than a whole-document replace. For puzzle3d, (b) is the actual blocker for 54/60 remaining actions (groups 2-4); group 5's 6 actions need a distinct fix (their reducer needs to actually represent their runtime effect, not emit empty).

---

## 3. Phase-8 plan's own acceptance criteria

Ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📋️master.md`, "Phase 8" section (line 99-101):

> ### Phase 8 — Every remaining tool (fleet: ~8 Sonnet packets sliced by plugin: cad, draw, layout, block, process, sourcing, vcs, animate + framework commands)
> Tool registry: tools supply an `InteractiveJob` factory instead of event callbacks (`Tool input → Operation spec → Job factory → Progress/preview → Commit`), wired through dispatch/action-bus. Small tools complete in their first step but use the same path. Imports/exports, serialization, compression, selection expansion, snapping, boolean geometry, routing, animation baking, search, diffing, package ops — same contract. Classify every command from the Phase 0 inventory: `migrated | batch-only pending rewrite | forbidden from UI | deleted`; release build rejects unclassified.
> **Gate:** inventory 100% classified; zero unmigrated interactive callbacks reachable from UI.

This confirms exactly what §1-2 found in code: "Migrated" is defined as "supplies an `InteractiveJob` factory ... wired through dispatch/action-bus," with the explicit carve-out that a single-step tool still has to go through the same path (that's the "generic"/`TypedCommandFullOperationJob` route, §1e-1f). The gate is binary reachability from UI, not a quality bar beyond that — but "wired through" is doing real work: a classification with no proof row fails at app-build time (§1e), so the plan's own acceptance criterion and the framework's actual enforcement agree.

No separate PHASE-8-named ticket folder exists (the phase lives entirely inside `📋️master.md`/`📌️status.md` of the master ticket); puzzle3d's own remaining-work tracking is the `🧪️publication-authority` fixture in §2, which is the more precise, plugin-specific version of the same acceptance criterion.

---

## 4. Reference app with a high `Migrated` ratio

Counted `.action_interactive_job(..., Migrated)` vs `BatchOnlyPendingRewrite)` across editor files:

| plugin | Migrated | BatchOnly | ratio |
|---|---|---|---|
| ✒️writer | 18 | 0 | 100% |
| 🌍️gis | 15 | 0 | 100% |
| 🔱️trinity | 8 | 1 | 89% |
| 📐️cad | 24 | 16 | 60% |
| 🌊️flow | 21 | 16 | 57% |
| 🧩️puzzle3d | 4 | 60 | 6% |

Used `✒️writer` (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`) as the reference, since it is fully migrated and structurally simpler than puzzle3d (one factory covers everything, not per-action bespoke work).

Concretely, what writer has that puzzle3d's remaining 54 actions don't:
- **One `WriterCommandJobFactory` covers all 18 tool ids** (`WRITER_COMMAND_TOOL_IDS`, factory at line 732), not per-action bespoke `Work` types — `build_tool_job` (line 1214-1223) just re-wraps `*request.command` + `writer_text_owner(&request.snapshot)` into one payload and calls the *same* `command.dispatch(doc, cfg)` reducer used by `handle` (line 1223-1233) for the ordinary path. So for writer, "Migrated" did **not** require rewriting business logic — only wiring it through the job factory.
- **`build_artifact_store_one_item_preparation_factory` is implemented** (`WriterArtifactStorePreparationFactory`, referenced at line 700+/line ~1059 trait default override) — this is exactly the piece missing in puzzle3d. Writer's document (rope/text buffer) apparently has a natural one-item bounded preparation shape; puzzle3d's Snapshot (attractions/objects/target_volumes/references/compatibility — several parallel collections) does not yet have one written for it.
- Writer's `bounded_first_step_tool_proofs!` (line 1134-1152) lists all 18 tool ids under **one shared `factory_type: WriterCommandJobFactory`** with **one shared contract** (`resumable(4_096, 4_096, 1, 64, 2_000, 1, 1)`) — no per-action tuning was needed. Puzzle3d instead needs bespoke per-action `Work` types (§2) because several of its actions (`setActiveExample`, `createAttraction`, fill/brush operations) are genuinely unbounded in item count and need real multi-stage chunking to stay under the 8ms step budget — writer's single-document-replace-per-command shape doesn't have that problem.

So the "high-migrated" reference app shows migration *can* be nearly free when an action is naturally single-step and the store already has a one-item preparation factory; puzzle3d's low ratio is explained by (a) a missing app-level Artifact-store preparation factory (one-time, blocks 19 actions across groups 2-3) and (b) several actions that are genuinely non-trivial multi-stage jobs by nature (which is why bespoke `Work` structs were written for them in the first place, even if currently unreachable).

---

## 5. Concrete migration recipe for `setActiveExample`, and how mechanical the rest is

### Current, surprising state of the code
`build_tool_job` (line 6416-6494) already contains:
```rust
if !PUZZLE3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
...
"setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
```
`Puzzle3dSetActiveExampleWork` (line 4582-~4750) is a **fully implemented, staged, budget-checked** `PuzzleCommandWork`: an `extent()` method that sums every collection's size against `PUZZLE_COMMAND_WORK_ITEMS` and refuses (returns `None`) if the example is too large for one bounded operation (line 4636-4650), and a `step()` state machine walking `DeleteAttractions → DeleteObjects → DeleteVolumes → DeleteReferences → DeleteCompatibility → CreateObjects → CreateAttractions → CreateVolumes → CreateReferences → CreateCompatibility → Publish → Complete`, emitting one bounded mutation + one localized (en/de) progress message per step (line 4630 onward). This is **never called** anywhere — `grep` for `build_tool_job(` inside the file returns nothing; the only tests referencing `Puzzle3dSetActiveExampleWork` (lines 7637, 7657) are "hostile static law" tests that `include_str!` the source and assert the *text* `"setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"` is present — they prove the arm exists in source, not that it ever executes.

### The recipe
1. **Blocking prerequisite (app-wide, not per-action): implement `Puzzle3dArtifactStorePreparationFactory`.** Add an `impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation>` (mirror `Puzzle3dConfigStorePreparationFactory` at line 6332, and `WriterArtifactStorePreparationFactory` for the general shape) and override `Puzzle3dPlayApp::build_artifact_store_one_item_preparation_factory` (currently absent — the trait default is `None`, see `🦀️.rs:11059-11062`) to return it. This is the fixture's blocker for `setActiveExample`'s whole group (group 3, 18 routes) and is shared work — do it once, unblocks all 18+1 Artifact-lane routes.
2. Add `"setActiveExample"` to `PUZZLE3D_RETAINED_TOOL_IDS` (line 2480). This single edit simultaneously registers the `ToolFactoryKey`, satisfies the `TOOL_IDS` bijection check (§1d), and opens the `build_tool_job` gate for this id.
3. Add `ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] }` to `PUBLICATION_CONTRACTS` (line 6151-6155) — lanes must match the fixture's `["Artifact","Config"]` for this route.
4. Add `"setActiveExample"` to the `tools: [...]` list inside `bounded_first_step_tool_proofs!` (line 6417-6424) — it will use the same shared `Puzzle3dRetainedCommandJobFactory`/`resumable(8_192, 512, 1, 262_144, 7_500, 1, 1)` contract already declared there, unless its payload footprint needs a larger `max_raw_wire_bytes`/`max_output_bytes` (check against the largest example fixture — `NAKAGIN_EXAMPLE_FIXTURE`/`CONCRETE_FOREST_EXAMPLE_FIXTURE`, both `LazyLock`-forced at `initial_snapshot`, line 6503-6504).
5. Flip the manifest declaration at line 7038: `.action_interactive_job("setActiveExample", BatchOnlyPendingRewrite)` → `Migrated`.
6. Update the two generated cross-checks that must now agree with source:
   - `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json` — move `"setActiveExample"` out of group 3's `routes` into a new (or existing) `Migrated` group with `lanes: ["Artifact","Config"]`.
   - The TS oracle (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`) re-derives `retainedIds`/`manifestPairs`/`publicationContracts` straight from source via regex (lines 25-46), so it should self-verify once 2-5 land — but run it (`bun ./📜️script.ts test`) to confirm, since it's the actual enforced oracle, not just documentation.
7. **Write a real functional test.** None currently exists that calls `build_tool_job`/dispatches `"setActiveExample"` through the job path and observes multiple `step()` calls with progress — only the string-match "hostile static law" tests. Add one exercising a large example (forcing multiple steps) and asserting bounded mutation counts per step, mirroring the existing `set_fill_count_and_finish` pattern (line 7355+) used for the fill-count feature.
8. The "hostile static law" tests (line 7635-7671) do **not** need to change — they already enforce the shape this recipe reaches.

None of steps 2-6 touch `Puzzle3dSetActiveExampleWork`'s logic — it is already correct and already staged. Step 1 is the only piece of new business logic required, and it's app-wide infrastructure, not specific to `setActiveExample`.

### How mechanical are the remaining ~59?

Cross-referencing the 60 `BatchOnlyPendingRewrite` ids against `build_tool_job`'s match arms:

- **50 of 60** already have a **named, dedicated `Work` implementation** wired into `build_tool_job`'s match (just gated by `PUZZLE3D_RETAINED_TOOL_IDS`) — e.g. `translateSelection`/`rotateSelection`/`scaleSelection` → `Puzzle3dScaleWork`; `createAttraction` → `Puzzle3dCreateAttractionWork`; `patchInspector` → `Puzzle3dPatchInspectorWork`; `addBrushObject`/`addObjectKind` → dedicated Work; `engagementAbort`/`RepeatLast`/`Submit` → dedicated Work; `setObjectKindWeight`/`setVortexKindWeight` → `Puzzle3dKindWeightWork`; `cycleBrushCandidate(Back)`/`fillBuildTick`/`registerBrushMesh`/`setFillCount`/`suggestionsTick` → `Puzzle3dPrecomputeCommandWork`; ~20 scalar/toggle actions (`setCamera`, `setProjection*`, `setSun*`, `setLod*`, `setGrid*`, etc.) → `Puzzle3dScalarConfigWork`.
- **10 of 60** (`addTargetVolume`, `deleteAttraction`, `deleteSelection`, `deleteTargetVolume`, `duplicateSelection`, `openVortexSuggestions`, `selectSameKindSelection`, `setFixtureJson`, `setSelectionFlag`, `setTargetVolumeFlag`) fall through to the generic `_ => BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent)` arm (line 6482) — still a legitimate bounded wrapper around the shared reducer/extent functions, just not bespoke.

So this is **not one uniform shape** — there are effectively three:
1. **~44 shared "same-implementation" actions** (all the `Puzzle3dScalarConfigWork`/`Puzzle3dPrecomputeCommandWork`/`Puzzle3dKindWeightWork`-style groups): moving them is steps 2-6 above, batched — e.g. moving all ~20 `Puzzle3dScalarConfigWork` ids can be one wave since they share one Work type and (per the fixture) mostly the Config lane. **Once the Artifact/Config preparation factories exist, this group is close to 3-line-per-action mechanical**, modulo the shared fixture/oracle updates done once per wave.
2. **~6-8 bespoke-Work actions** (`createAttraction`, `setActiveExample`, `Puzzle3dScaleWork` group, `Puzzle3dKindWeightWork`, `Puzzle3dPatchInspectorWork`, brush/engagement Work types) — same mechanical steps, but each depends on the same one-time Artifact-store-factory prerequisite (step 1), and each Work's `extent()`/budget numbers should be spot-checked against real fixture sizes before flipping.
3. **10 generic-fallback actions** — same mechanical steps, no bespoke Work to write, but should be individually reviewed since the shared `puzzle3d_retained_reduce`/`puzzle3d_retained_extent` functions were evidently *not* written with these ids in mind yet (worth confirming each one's mutation shape is actually representable there before flipping).
4. **Group 5 (6 actions: `fillBuildTick`, `registerBrushMesh`, `suggestionsTick`, `engagementRepeatLast`, `transformBegin`, `transformEnd`)** are the outlier — their blocker is not the store-authority prerequisite but that their current `Emit` is empty/effect-only and doesn't represent their real runtime behavior (§2). These need actual reducer work, not just wiring, before they can be honestly migrated.

**Overall: for 54/60 actions the label-vs-reality gap is real but almost entirely closed by one shared piece of infrastructure (the Artifact-lane store preparation factory) plus mechanical per-action wiring; the job-shaped logic itself was already written and is just unreachable. For 6/60 (group 5) real reducer logic is still missing.**

---

## Files referenced (all read, no edits made)

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:829-968` — enum + `validate_interactive_job_classification`
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:1-8, 328-366, 520-580` — `ActionBus`, `ToolJobFactory` trait, registration gate
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10960-11072, 11915-11921, 12059-12137, 12441-12760, 16359+, 18549-18551, 21860-21930, 22460-22551` — `ArtifactApp` trait defaults, `validate_ui_dispatch_classification`, `validate_tool_job_rows`, `ArtifactToolFactoryRegistry`, `TypedCommandFullOperationJob`, `dispatch_action`/`dispatch_command`, runtime job-build branch
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13008-13037` — `ArtifactStoreOneItemPreparationFactory`/`ArtifactStoreOneItemPreparation`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2480, 4582-4750, 6060-6155, 6332-6494, 6980-7060, 7355-7420, 7600-7671` — puzzle3d editor: retained ids, `Puzzle3dSetActiveExampleWork`, factories, `build_tool_job`, manifest classification block, hostile-static-law tests
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:700-745, 1125-1235` — reference app (100% Migrated)
- `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json` and `🔣️.schema.json` — authoritative per-route blocker fixture and its laws
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:1-50, 150-180` — TS oracle cross-checking fixture against source
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📋️master.md:99-101` — Phase 8 plan text
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️research/📝️interactive-job-migration-recipe.md` — parallel prior investigation (lowpoly plugin), cross-referenced for consistency

---

## Wave 1 — implementation

Scope delivered: `setActiveExample` migrated end to end for `Puzzle3dPlayApp`, plus the one shared piece of infrastructure the recipe identified as the real blocker (`Puzzle3dArtifactStorePreparationFactory`). No other action's classification was touched.

### What changed (file:line, current source)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  - `puzzle3d_config_store_mutation_bytes` (~line 6198): added an admitted `Puzzle3dConfigMutation::Snapshot { config }` arm (bounded via the existing `puzzle3d_config_store_bounded_bytes` helper). `setActiveExample`'s `Publish` stage emits `Puzzle3dConfigMutation::Snapshot { config: Puzzle3dRuntime::default() }` as its one Config mutation; the Config-lane preparation factory's admission match previously only recognized `SetLocale`/`SetTerminology` and would have rejected this at runtime with "Puzzle3d Config preparation rejected its exact mutation envelope" — this was a real, second gap not called out explicitly in the original recipe text (found by reading `Puzzle3dConfigStorePreparation::advance()`'s phase-0 check against the Publish stage's actual `Emit`).
  - New `Puzzle3dArtifactStorePreparationFactory` / `Puzzle3dArtifactStorePreparation` (~line 6376-6552, right after `Puzzle3dConfigStorePreparationFactory`'s impl block, before `impl ArtifactEditor for Puzzle3dPlayApp`): `impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation>` and its `ArtifactStoreOneItemPreparation` companion. Mirrors `Puzzle5dStorePreparationFactory`/`Puzzle5dStorePreparation` (`🗿️artifacts/🖐️5d/.../✏️editor/🦀️.rs:7995-8155`, the one other puzzle-family app with an Artifact-lane one-item factory) almost verbatim: admits an arbitrary `Puzzle3dMutation` generically via `protocol::Mutation`/`protocol::MutationDiff` (`mutation.inverse(base)`, `mutation.diff(base).into_parts().0.apply(base)` — the local `.into_parts().0` idiom matches `Puzzle3dConfigStorePreparation`'s existing style rather than Puzzle5d's `.diff()` accessor style, since Puzzle3d already established that local convention), fixed `work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` footprint (not a narrow per-variant allowlist, because `setActiveExample`'s work loop emits ~10 different mutation kinds — delete/create object, attraction, target volume, reference, compatibility, domain, catalogs).
  - `Puzzle3dPlayApp::build_artifact_store_one_item_preparation_factory` override added next to the existing `build_config_store_one_item_preparation_factory` (~line 6584-6586): `Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))`. This was the trait-default-`None` gap the recipe identified as blocking all 19 Artifact-lane routes (groups 2-3 of the fixture).
  - `PUZZLE3D_RETAINED_TOOL_IDS` (~line 2480): added `"setActiveExample"` (now 5 entries).
  - `Puzzle3dRetainedCommandJobFactory::PUBLICATION_CONTRACTS` (~line 6216): added `ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[Artifact, Config] }`.
  - `bounded_first_step_tool_proofs!`'s `tools: [...]` list (~line 6608): added `"setActiveExample"`, sharing the existing `resumable(8_192, 512, 1, 262_144, 7_500, 1, 1)` contract with the other 4 ids (Nakagin's wire payload is a tiny `{"exampleId":"nakagin"}` JSON string, well under the 8_192-byte raw-wire cap; the mutation *count* bound is enforced separately by `Puzzle3dSetActiveExampleWork::extent()` against `PUZZLE_COMMAND_WORK_ITEMS=4096`, not by this contract).
  - Manifest classification (~line 7188): `.action_interactive_job("setActiveExample", ...)` flipped `BatchOnlyPendingRewrite` → `Migrated`.
  - Two new `#[test]` functions added directly after `set_active_example_hostile_static_law_rejects_whole_document_reset` (~line 7912-7971):
    - `set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` — constructs `Puzzle3dSetActiveExampleWork` directly (no app/framework machinery), drives `.step()` in a loop for the Nakagin fixture, and asserts (a) more than one `Progress` step occurs before `Complete`, (b) the completed `Emit.artifact_mutations` carries more than one mutation, (c) exactly one config mutation. This is a direct, unmocked exercise of the real production `PuzzleCommandWork` state machine — not a string-match test.
    - `set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document` — `app_with_registry()` + `bind_instance_id(1)` (bare `testkit::app()` faults closed with `interactive-job.catalog-authority` for this plugin, per the task brief — confirmed by static trace of `AppActionRegistry::validate_tool_job_rows`: an empty registry's `migrated_tool_ids()` is always `∅`, which can never satisfy `expected.contains(row.tool_id)` for any of the `bounded_first_step_tool_proofs!` rows), dispatches `setActiveExample` through `dispatch_typed` (the real `InteractiveJobClassification::Migrated` path — `admit_command_wire` → `qualified_tool_proof` → `Puzzle3dRetainedCommandJobFactory` → `RetainedPuzzleCommandJob` → `ArtifactToolCompletion` → the shared publication loop's `self.store.begin_apply_one(..., self.artifact_one_item_factory.as_deref())`, i.e. exactly the new factory above), then drains the resulting typed operation via repeated `app.maintenance_step(...)` turns (the same call a real host issues every actor tick) until the document's first object id changes, proving the full wire end to end.

- `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json`: moved `"setActiveExample"` out of `Puzzle3dPlayApp`'s `BatchOnlyPendingRewrite`/`["Artifact","Config"]` group into a new `Migrated`/`["Artifact","Config"]` group.

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`:
  - Fixed a **pre-existing, already-committed** bug (present before this ticket, unrelated to this change): `PublicationAuthorityAuditScript.run` read the fixture/schema from a flat `🔣️publication-authority.json`/`🔣️publication-authority.schema.json` path that has not existed since the fixture was moved into `🧪️publication-authority/🔣️.json` + `🔣️.schema.json` (commit `a8d1caf41f`/`21fbcd3538`, confirmed via `git log` — both the fixture's current location and the stale script reference were introduced in the same commits, i.e. the reference was simply never updated). Fixed both paths to the real location. Without this the audit command cannot run *at all*, for any owner, regardless of this ticket's changes.
  - `ownerOracle`'s `exactFactory` check: was `owner.owner === "Puzzle5dPlayApp" || (!production.includes("build_artifact_store_one_item_preparation_factory") && ...)` — hard-coded that only Puzzle5d may have an Artifact-lane one-item factory. Extended the allow-list to `Puzzle3dPlayApp` too (restructured the boolean so the artifact-factory presence/absence check and the draft/presence/transient absence checks are independent clauses).
  - Puzzle3d-specific structural checks: added five more `production.includes(...)` assertions requiring `Puzzle3dArtifactStorePreparationFactory`'s struct, both trait impls, the `build_artifact_store_one_item_preparation_factory` fn, and its `Some(std::sync::Arc::new(...))` wiring — mirrors the existing Config-preparation checks immediately above them.
  - Hostile-mutation battery for `Puzzle3dPlayApp`: added a `missingArtifactPreparation` mutation (removes the new factory wiring, asserts the oracle rejects it) alongside the existing `missingPreparation` (Config). Also changed `staleAuthority`'s `.replace(...)` to `.replaceAll(...)` — the stale-authority guard line is now duplicated verbatim across the Config *and* Artifact preparations, and a single-occurrence replace would silently leave the un-mutated Artifact copy's guard intact, making the plain `.includes(...)` check downstream still see the pattern and falsely pass the hostile mutation. Verified this distinction actually matters (see below).

### Verification actually performed

**Rust build/test: written but not run to a passing result.** `cargo check -p semio-s-plugin-puzzle` (both plain and `--tests`, both via the live shared `target/` and via an isolated `CARGO_TARGET_DIR`) currently fails to compile *`semio-framework` itself*, at `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1818/1822` — `the trait bound Effect: serde::Deserialize<'de> is not satisfied`. Confirmed via `git status`/`git diff --stat` that this file is **currently modified and uncommitted by a different, live concurrent session** (also independently confirmed: an earlier attempt hit a *different* error in the same file, `CapabilityGrant: ToValue`/`FromValue` not satisfied, which had resolved by the next attempt — the file is being actively edited mid-flight). This is upstream of and unrelated to every file this ticket touched; I did not attempt to fix it (out of scope, and it is someone else's in-progress work). I retried the isolated-target build twice over roughly 40 minutes; both times it failed in the same unrelated file. I did not fabricate a pass — the two new `#[test]` functions above are real but their pass/fail status is **not confirmed by rustc**.

**What I verified instead, directly:** ran the actual regex-based `ownerOracle`/`fixtureOracle`/hostile-mutation logic from `📜️script.ts` (copy-pasted verbatim into a throwaway scratchpad script, `/private/tmp/.../scratchpad/diag_puzzle3d.ts` and `diag_puzzle3d_hostile.ts`, executed with `bun`) directly against the real, current `✏️editor/🦀️.rs` source and the real fixture JSON — this is the actual textual/structural cross-check the TS oracle performs, run for real, not simulated by reasoning:
- `exactArray(pairs.keys(), appRoutes)`: **true** (65/65 routes, manifest pairs match the fixture 1:1)
- every route's classification matches the fixture: **true**, no mismatches
- `PUZZLE3D_RETAINED_TOOL_IDS` == fixture's migrated set: **true** (`openAddObjectDialog, worldPointerDown, setLocale, setTerminology, setActiveExample`)
- `bounded_first_step_tool_proofs!`'s `tools:` list == migrated set: **true**
- `PUBLICATION_CONTRACTS` lanes exactly match the fixture per route, including `setActiveExample → [Artifact, Config]`: **true**
- `exactFactory` (owner/controller/schema/tool identity, registration call, no draft/presence/transient factories, Artifact factory now permitted for Puzzle3d): **true**
- all 22 Puzzle3d-specific structural `.includes(...)` checks (Config prep factory + new Artifact prep factory: struct, both trait impls, builder fn, wiring): **all OK**
- hostile mutations all correctly **rejected** (oracle flips to `false`): missing Config preparation, missing Artifact preparation, widened terminology allowlist, stale generation-authority check (after fixing to `replaceAll`), missing `Progress` checkpoint step, hostile activation of `addTargetVolume` (the next BatchOnly route) to `Migrated`, missing publication contract.

Running the real `bun ./📜️script.ts publication-authority-audit` command itself currently still fails, but on **`Puzzle2dPlayApp`**, checked first in `fixture.owners`, before it ever reaches Puzzle3d — traced this with the same throwaway-script technique: Puzzle2d's actual source now has `addNode`/`forceLayout`/`setActiveExample`/32 other ids already flipped to `Migrated` with a live `bounded_first_step_tool_proofs!` block, while the fixture JSON still records them as `BatchOnlyPendingRewrite` with only 3 routes total for that owner. This is **another concurrent session's in-progress puzzle2d migration**, not yet fixture-synced — confirmed unrelated to Puzzle3d/this change (Puzzle2d's branch returns early in `ownerOracle`, before any of the code this ticket touches is even reached). Did not touch Puzzle2d's fixture or source.

**Coordinator's two specific questions, answered:**
1. *Two factories per lane ("retained preparation" + "root-retirement")* — traced the actual gate (`VcsArtifactApp::with_registry_on_bus`'s `unsupported_publication_contracts` construction, `🔌️plugin/🦀️.rs:19143-19148`): for the `Artifact`/`Config` lanes this checks *only* `artifact_one_item_factory.is_none()` / `config_one_item_factory.is_none()` — no second factory. The "root-retirement" pairing in the fixture's blocker text applies to the *`Presence`/`Transient`* lanes only (`presence_local_root_retirement_factory`/`transient_local_root_retirement_factory`, a distinct `SnapshotRetirementFactory<P>` mechanism `A::build_presence_local_root_retirement_factory()`/`build_transient_local_root_retirement_factory()`), which `setActiveExample` never declares. For Artifact/Config, the equivalent "root" concern is `MemberStoreOwners.snapshot_retirement`/`initial_snapshot_retirement`, installed via `ArtifactStore::install_member_store_owners_exact` — and Puzzle3d's `build_document_store_owners()`/`build_config_store_owners()` (already present, `✏️editor/🦀️.rs:~6555-6561`, both return `Some(semio_framework_plugin::bounded_document_store_owners::<...>())`/`bounded_config_store_owners`) already supply this for both lanes. So the fixture's "and root-retirement factory" phrasing is boilerplate copied across all five lane-blocker strings, not a literal second missing piece for Artifact/Config specifically; the one-item preparation factory was genuinely the whole gap for `setActiveExample`.
2. *Link to the "runtime live cleanup faulted for instance 1" / `take_returned_snapshot_read_retirement` bug* — **no**, these are different mechanisms. `ArtifactStoreOneItemPreparationFactory<P, Mutation>` (what I added) governs how one *mutation* is prepared/committed; `SnapshotRetirementFactory<P>`/`snapshot_retirement_factory` (what `take_returned_snapshot_read_retirement` needs) governs how a `SnapshotRead<P>` *read handle* is retired back to the registry — installed via the *same* `build_document_store_owners`/`build_config_store_owners` mentioned above, which Puzzle3d already has for both the Document and Config stores. My new factory does not touch `snapshot_retirement_factory` at all. Separately, another session already has a test-in-progress in this exact file for that bug (`local_interaction_query_return_does_not_fault_the_next_maintenance_step`, ~line 7580) attributing it to the *local interaction query* snapshot-read lease, not to any tool-job publication path — consistent with this being a genuinely separate issue, not one this ticket's infrastructure fixes as a side effect.

### What did not work / residual risk

- No confirmed `rustc` pass (framework-wide breakage from unrelated concurrent work, see above). If that resolves, re-run: `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-puzzle`, then `cargo test -p semio-s-plugin-puzzle --lib set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document`, then the existing hostile-static-law tests (`*_hostile_static_law_*`, ~line 7635-8200 in the current file), then `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit` (will still fail on Puzzle2d until that other session syncs its fixture — not blocking for this ticket's own review, but worth flagging to whoever owns that work).
- The two new Rust tests' exact API usage (`Puzzle3dPlayApp::initial_snapshot()`, `semio_framework_plugin::app::InteractionHoverState::default()`, `Puzzle3dCommand::from_action(...) -> Option<Self>`, `PluginApp::maintenance_step`) was confirmed against other working call sites in this repo (`✏️s/🔌️plugins/🌿️vcs/.../✏️editor/🦀️.rs:1383`, `🏭️process/.../✏️editor/🦀️.rs:2566`, and this same puzzle3d file's own `local_interaction_query_return_does_not_fault_the_next_maintenance_step` test) via a research subagent — not by compiling, so a signature mismatch (if any drifted since) would only surface on an actual build.
- `set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document`'s completion-detection loop (poll `maintenance_step` until `first_object_id` changes, capped at 5000 ticks) is a best-effort design based on tracing `VcsArtifactApp::maintenance_step`'s round-robin tool-operation stage (`🔌️plugin/🦀️.rs:~23596-23643`) and `start_typed_command_operation`'s single internal `drive_worker_step` call; not empirically confirmed how many ticks Nakagin actually needs.

### Wave 1 addendum — final build attempt result

A third isolated-`CARGO_TARGET_DIR` `cargo check -p semio-s-plugin-puzzle --tests` completed (~80+ errors, `head`-capped). The `semio-framework` blocker from earlier resolved, but the puzzle crate itself now fails with a **different, much larger** set of errors: `error[E0053]`/`E0277`/`E0308` — `dsl::DslValue` vs `serde_json::Value` type mismatches on `command_from_action`, `ToValue`/`FromValue`/`Serialize`/`Deserialize` unsatisfied for `serde_json::Value` and dozens of plugin-local types — spread identically across **puzzle2d, puzzle3d, and puzzle5d alike** (e.g. `Puzzle2dCommand`, `Puzzle3dCommand`, `Puzzle5dCommand`, their respective `Config`/`ConfigMutation`, `MeshData`, `Puzzle3dDiff`, etc. all fail the same way). This is the `serde`→`dsl`/`ToValue`/`FromValue` migration referenced by ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` (visible in the ticket tree at session start), evidently mid-flight and not yet landed for the puzzle plugin.

Confirmed via `git diff -U0`'s hunk headers that **none** of the ~13 puzzle3d-editor-file errors fall inside any hunk this ticket's change touches (my hunks: `@2480`, `@6155`, `@6203`, `@6377-6558`, `@6587`, `@6607`, `@7228`, `@7581`, `@7913`; the errors are at lines 185, 224, 1130, 1705, 1707, 1886, 1889, 6715, 7333, 7530, 7632, 9012 — all pre-existing code untouched by this ticket, e.g. line 6715 is `command_from_action`, between my `@6607` and `@7228` hunks). This is a second, independent, unrelated pre-existing/concurrent breakage — not caused by this ticket's changes, and out of scope to fix here (it spans all three puzzle dimensions and looks like a separate, larger migration in progress).

**Net result: still no clean `rustc` pass for the crate, for reasons demonstrably unrelated to this ticket's diff.** The TS-oracle-based structural verification in the section above remains the strongest evidence available that this ticket's own code is correct; a real `cargo test` pass is still pending on the puzzle plugin's `DslValue`/`ToValue`/`FromValue` migration landing (someone else's concurrent, larger effort).
