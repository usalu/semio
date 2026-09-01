# Interaction Engine Failures — Root Cause Report

Scope: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📄️artifact/🟦️component.ts` (exclusive slice). No functional edits landed in this file — investigation only (temporary `[DEBUG]` logs added and removed; final `git diff` on the file is empty of my changes). Two other in-file lines changed during the session belong to a concurrent dev, not me.

## Before / after (real runs)

- Before (start of session): `Test Files 5 failed | 4 passed (9)` — `Tests 63 failed | 258 passed (321)`. 58 of those 63 failures are in my file (145 tests total there).
- After (final run, same command): `Test Files 3 failed | 6 passed (9)` — `Tests 60 failed | 261 passed (321)`. My file: still **58 failed / 145 total, 0 fixed**. The renderer/stately/geometry/brepjs delta (63→60) is a sibling agent's concurrent work, not mine.

Command used: `cd .../📦️packages/🟦️typescript && CAD_JS_RENDERER_PLAY_PORT=6041 bun ./📜️script.ts test long` (the `bun nx run …` wrapper currently fails before tests even start, unrelated taxonomy-schema error: `fileKindResolutionRules must own ".cmd.semio" exactly once for "semio-command"` — a separate, pre-existing repo-wide issue, not touched here).

## Root cause 1 (dominant — explains 57 of the 58 failures: all of groups A, B, C, E, G, and 4 of 5 in F)

**`EffectSpec` discriminant field mismatch: JSON assets use `"mutation"`, the TS engine checks `"operation"`.**

Verified with instrumentation (temporarily added to my file, then removed): for `entity.createAnchor` on the `empty` fixture, sending the single `selection.changed` step correctly evaluates the `selectionHasPoint` guard as true and the state machine reports `afterState: "committed"` — but `Object.keys(this.sm.getContext())` is `[]`. The transition's three `assign` effects (`hostKind`, `hostId`, `hitPoint`) never wrote anything, so the immediately-following `runCommit` fails its `hasHostAndHitPoint` guard, and `restoreUndoSnapshotAfterFailedFinalCommit()` (in my file, working as designed) rolls the session back to `selectHost` — which is exactly the `expected 'selectHost' to be 'committed'` symptom, and the shape of nearly every other failure (`expected 'first_corner' to be 'first_corner_height'`, `TypeError: null is not an object (evaluating 'res.ok')` from a `lastResponse` that never got set, undo/redo context fields staying `undefined`, `interaction.call` resuming into the wrong nested target, etc.).

Confirmed by inspection + grep, not just this one test:
- Every interaction JSON's transition effects use `"mutation": "assign" | "clear" | "append" | …` (49/49 files checked contain `"mutation": "assign"`; 0 contain `"operation": "assign"`).
- The Rust schema is authoritative and agrees with the JSON assets: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs:234` — `#[serde(tag = "mutation", rename_all = "camelCase")] pub enum Effect { Assign {..}, Clear {..}, Append {..}, Emit {..}, Raise {..}, …, Action {..}, InteractionCall {..} }`.
- The TS side is wrong and lives entirely in **forbidden files** (not my slice):
  - `EffectSpec` type: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts:558-578` — every variant tags on `operation`, should tag on `mutation`.
  - Consumers checking `.operation` on an `EffectSpec`/`TransitionSpec.effects[]` item that all need to switch to `.mutation`: `📐️geometry/🟦️component.ts:911-917` (`staticInitialContext`), `:3104` and `:3129` (interaction-call/action id collectors); `🎬️actions/🟦️component.ts:1139-1177` (`applyEffectAsync` — assign/clear/append/kernel.query/interaction.call/action branches) and `:1210` (`applyTransition`'s `interaction.call` childCall check).
  - Note: `ActionSpec.steps[]` (a different type, used inside action JSON `"steps"`, e.g. `{"operation":"kernel.call", ...}`) legitimately uses `operation` — that schema is not modeled in Rust and is untouched by this bug. Only the interaction `TransitionSpec.effects[]`/`EffectSpec` schema is mismatched.

This is squarely the "engine" side per the ticket's own split, not a JSON-asset defect (the assets already match the authoritative Rust schema), so I did not edit any JSON to route around it. **Handoff**: rename the discriminant from `operation` to `mutation` (and its string literals) across the `EffectSpec` type and all `.operation`-on-effect call sites in `📐️geometry/🟦️component.ts` and `🎬️actions/🟦️component.ts` listed above. I expect this single change to flip the large majority of the 58 to green; re-run `core interaction e2e fixtures`, `core interaction box`, `core interaction length entry`, `core interactions`, `core interaction session undo redo`, `core undo routing`, and the 4 registry tests in `core action and interaction registries` after the rename.

## Root cause 2 (isolated — 1 of the 58: `every typology ships construct kit or legacy create interactions`)

All 11 `building.building.*` typologies under `…/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/*/🔣️typology.json` (Wall, Ceiling, Roof, Slab, Column, Stair, Door, Window, Foundation, Railing, Beam) ship with **no `actions` and no `interactions` arrays at all** (verified: `grep -q "\"actions\""` misses on all 11), unlike every other typology (e.g. `spatial.shape.primitive.box`'s `typology.json` correctly lists 14 actions + 1 interaction). The `aec.building.concrete` extension does define some wall/column construction actions (`constructVerticalConcreteWall.json`, `constructWallFromHorizontalPathAndProfile(s).json`, mushroom-column actions), but none are wired back onto any typology's `actions`/`interactions` arrays, and that extension ships no interaction JSON at all.

This matches `AGENTS.md`'s `ModelSpace` example, where Wall/Roof/BasePlate objects are produced only by a `spatial.shape_to_aec.building.energy`-style **transformation**, never by a direct user construct interaction — so the gap may be intentional, not a bug. Deciding whether these 11 typologies should be excluded from this check (transformation-derived, like the existing `.kernel.` exclusion) or should really ship a full construct kit (new interaction JSON + typology `actions`/`interactions` wiring per typology) is a modeling decision I did not make unilaterally: I cannot touch the test (forbidden — no weakening/skipping failing tests), and inventing 11 new construct interactions without a specified UX/args shape risks being simply wrong. Left failing; flagged as a distinct follow-up, separate from root cause 1.

## Groups vs. root cause

- A (27, e2e fixtures), B (7, box), C (7, length entry), E (5, transform.move/copy), G (2, undo/redo routing): **root cause 1**, verified representative case in each group.
- D (4, measure distance/area): **root cause 1** — the `TypeError: null is not an object (evaluating 'res.ok')` is `lastResponse` staying `null` because commit never runs.
- F (5, registries): 4 of 5 are **root cause 1** (`curve.interpolateCurve`/`primitive.box` commit-binding, `interaction.call` nested-session resume target); 1 (`every typology ships construct kit…`) is **root cause 2**.

## What I did not do

- Did not edit `📐️geometry/🟦️component.ts`, `🎬️actions/🟦️component.ts`, `📔️registry/🟦️component.ts`, `📺️renderer/🟦️component.tsx`, `🎰️stately/🟦️component.ts`, `🧱️brepjs/🟦️component.ts`, or any Rust — all forbidden/sibling territory.
- Did not touch the failing test assertions.
- Did not edit any model-definition JSON asset — in every case investigated, the JSON was already correct against the Rust schema; changing it would have been changing correct data to work around a broken engine.
