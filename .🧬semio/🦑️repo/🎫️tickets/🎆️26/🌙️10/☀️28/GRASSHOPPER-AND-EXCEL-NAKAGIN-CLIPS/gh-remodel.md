# Grasshopper Remodel

Evidence chronology: test counts, interim podium datums and retained one-off scripts below describe earlier checkpoints. Current source acceptance is tracked in remodel-summary.md, final-architecture-gate.md and final visual/native ledgers. The final pitch is 2.80 m and podium roof 7.72 m. Historical 59-test and 7.5 m statements are superseded; no removed migration input should be restored.

Current cleanup note: the historical authoring-input retention statements below are superseded. The one-off rewrite scripts gh-remodel.py, gh-canvas.py, gh-fixtures.py, gh-datum.py and gh-partial.py were removed under the repository’s no-migration requirement. They must not be restored. Reusable verification scripts (validate-scenes.py, check-scenes.py and audit-circulation.py), audits and reports remain.

The authored model now follows the shared compact floor schedule: Tower A has 76 capsules, Tower B 64. Its current aggregate tree contains 75 facade branches: 65 pairs and ten singletons; the proposed A–D program counts are 36/35/33/36. These are explicit language-agnostic contracts in the case JSON.

The tower definition constructs authored canonical body frames from actual origins and axes. Series and Graft Tree create logical floor-elevation branches; Move applies their 2.80 m pitch and the three source phase offsets above the 7.72 m podium roof datum. Cull Index removes absent authored attachment identities. Orient maps each reference casing body frame directly to its authored world body frame; side-entry door-port frames remain distinct metadata. The radial pair rotations and inferred spacing/clearance pipeline have been removed.

Both the tower output and aggregate tree carry actual shared exterior assemblies for the six geometry families. Engine Brep.parts() exposes owned semantic world-space polyhedra with role, material, key, vertices and faces. Source meshes, window wall choices and handed door geometry survive placement. Structural boxes and the podium extrusion remain explicit simplified teaching constructions; architectural scene drawings use shared detailed structural geometry.

Validation: 59 Python tests pass. Rhino independently verifies the handed source-to-target transform; NumPy independently verifies mesh volumes. The tests also compare all 140 canonical planes, actual floor counts, authored family mesh placement, aggregate ragged branches, localized casting behavior, and canvas wiring. JavaScript validates the schema, rejects 27 malformed models, and reproduces tree/casting cases independently.

Files changed:

- 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py
- 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🔣️.json
- 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json
- 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json
- 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py

Ticket scripts gh-remodel.py, gh-canvas.py and gh-fixtures.py are preserved as inputs. Generated JSON staging outputs can be removed with the ticket generated directory after the parent finishes verification.

## Final Architectural Datum Integration

The final model raises all capsule placements to the authored podium roof datum of 7.5 m. The z0 control now binds directly to podium.levels.2, and Series starts from that value. The obsolete multiplication of podium storey count by capsule pitch has been removed.

Core height is constructed with actual Grasshopper arithmetic: n × sh + z0 + crown_offset + service_overrun. The crown offset comes from the highest authored capsule crown relative to the nominal floor series; the service overrun is bound to the shared 2.8 m assumption. The resulting core heights are 42.1333 m for Tower A and 36.6333 m for Tower B. The capitals start at these derived heights. Core, level and podium views show the updated native graph; a separate height view exposes every arithmetic stage.

The independent portable fixture now includes the podium datum and both core heights. A raw JSON floor walk verifies the highest capsule crown, the roof datum, core height and capital base. Validation after the final datum integration: all 60 Python tests pass; the JavaScript schema, canvas, tree and casting oracle checks pass. The fixed fixtures were exercised against the former graph first, reproducing the five incorrect-height/placement failures before the implementation was changed.

The additional ticket input script gh-datum.py is preserved. Its generated staging JSON lives under the ticket generated directory for final cleanup.

## Final Native Verification Pass

The final portable suite passes 61 tests. An additional test first reproduced the partial-height animation bug, then verified the corrected floor-dependent crown offset for one, two and all floors. The height graph now selects the authored per-floor offset with Subtraction and List Item. This preserves the lower first-floor crown rather than extending it to the later three-phase maximum. All JavaScript schema and independent tree/casting checks pass.

The first native Anatomy/Capsule verification ran both English and German plus the toolkit Probe. Both Probe cases passed. Both Capsule cases exposed an absent first-floor facade selector before the first animation step; both Anatomy cases exposed a fabricated first-floor east-facade key after the plugin chapter. These exact source failures were sent to the scene owner for correction and will be rerun in fresh processes.

The non-lifecycle presentation contract run initially found one readability failure in the core view: six vertically stacked controls reduced widget scale below 0.75. The core view was rearranged into three rows without changing its arithmetic or wires. A fresh contract run is in progress against the corrected layout.

The final fresh non-lifecycle presentation run passes all 11 tests in 132.821 seconds, including construction of every named engine view, frame containment, widget scale, wire kinds, messages, bilingual narration, source dispatch, imports and provenance. The core-height calculation is split into a nine-object height view and a five-object crown-selection view so each remains readable in the native canvas frame. The corrected Capsule English lifecycle has completed all 24 chapters successfully; the remaining language/scene cases continue in the fresh verification process.

The corrected Capsule scene completed every one of its 24 narrated native chapters in both English and German, including actual-slot selection, orientation, List Item and Wrap. The Anatomy rerun completed the original failing series/tree/address sections and then exposed a second remaining fixed-row assumption while building the data legend after legend_tool. The run was externally aborted during the next German case by the repository supervisor after a Windows process-memory query failed; it therefore supplied no final unittest traceback. The scene owner was sent the exact fixed-row legend source. The ticket wrapper now prints failing native subtest tracebacks immediately so external termination cannot hide the source failure. Anatomy will be rerun separately.

The isolated fresh Anatomy English native lifecycle now passes completely (178.972 seconds), including every narrated chapter, the corrected authored slot selections, directional morph hooks and variable-length data legend. A fresh German-only process is running next. The wrapper now records each completed scene/language outcome immediately to generated checks.jsonl, uses failfast mode, prints errors immediately with Windows-safe Unicode handling, and includes the language in each output folder.

## Final Verification Outcome

All assigned native coverage is complete. Beat2_Capsule completed all 24 chapters in English and German in the fresh combined run; Beat1_Anatomy completed all narrated chapters in separate fresh English (178.972 s) and German (200.179 s) runs. Both isolated Anatomy runs also passed the matching toolkit Probe, animation setup/interpolation/cleanup, finite geometry, visible Cairo pixels and exact narration-key assertions. Generated checks.jsonl records preserve the isolated per-language Probe and Anatomy success outcomes; earlier failed attempts remain as history.

The final engineering checks remain: 61 Python portable/independent-oracle tests passed; JavaScript accepted the authored schema, rejected 27 malformed models and reproduced the tree/casting contracts; all 11 non-lifecycle presentation/widget contracts passed. No source changes were made after the final scene fixes except the parent-owned formatting cleanup.

## Excel Native Observer Correction

The initial Excel Anatomy English run completed its animation execution in 284.179 seconds. The long interval after the formula chapter was not a hang: seventeen chapters used the scene's custom _clause dispatcher, which performs caption changes, sequential morphs and narration holds without calling step. The prior native observer recorded only six direct step calls and therefore rejected the completed scene.

The native test now observes both actual dispatch pathways and performs the same finite-geometry/Cairo checks after each. Delegation from a clause into a step with the same key is counted once, while genuinely repeated dispatches remain visible. A language-agnostic fixture and regression exercise direct, manual, delegated and repeated calls; the regression and all-scene static narration contract pass. Static AST checks now recognize both supported source methods. The ticket runner also reports clause progress. Scene animation flow was preserved.

A concurrent source write left an orphan ds) after Excel Anatomy's valid final function. Only that invalid tail was removed. All nine Grasshopper and Excel scene sources then compiled and imported successfully. New changed files for ticket closure: Excel presentation test Python and presentation cases JSON, plus the one-line orphan-tail correction in Excel Anatomy source. Ticket check-scenes.py is preserved as an input.

The corrected Excel Anatomy English native run passes all 23 narration clauses and complete animation/geometry/Cairo checks (337.631 seconds). Its durable success outcome is recorded under scenes-excel-Beat1_Anatomy-en. The next German-only process uses the same observer plus the aligned runtime-leaf harness.

The Excel harness now patches Manim Scene.play/wait at the runtime leaf rather than overriding a scene subclass's dispatcher. A real Manim subclass regression proves its custom play method can expand a sequence before the patched leaf receives it. Both this portable regression and the direct/manual/delegated narration-observer regression pass (2 tests, 17.659 seconds). Excel currently has no custom play dispatcher, so this preserves the just-completed English coverage while preventing the Grasshopper Capsule dispatcher issue from recurring in Excel.

Excel Anatomy German now also passes its fresh isolated native lifecycle (384.129 seconds). The durable per-subtest JSONL success record is verified. Together with English (337.631 seconds), both languages completed all 23 narration clauses, real animation interpolation/cleanup, finite geometry and visible Cairo pixels. The final full Excel non-lifecycle contract suite is running next; References English is owned by the parent worker.

Ticket inputs created or updated by this worker are gh-remodel.py, gh-canvas.py, gh-fixtures.py, gh-datum.py, gh-partial.py, check-scenes.py and validate-scenes.py; the authored report is gh-remodel.md. Generated staging JSON and native outputs remain under the ticket generated folder for final cleanup.

The final integrated Excel non-lifecycle suite passes all 17 tests (357.773 seconds), including both new observer/dispatcher regressions. Root has completed all 18 explicit native scene/language records. The remaining new architectural gate is circulation support and connector headroom; its independent portable fixtures and test module are under the shared Nakagin circulation domain, and detailed findings are in circulation.md.

## Expanded Final Circulation Proof

On the explicit 0.65 m elevated, roof-bearing tank-frame snapshot, all 12 independent circulation tests pass in 9.480 seconds. The graph starts only at the two exterior thresholds and reaches all 140 capsule landings, 26 lift approaches (including both basement stops), six bridge approaches and three bridge decks. All 552 actual stair treads are checked along continuous 2 mm centreline tubes at 2.1 m headroom; flat routes use continuous 0.6 m body-width strips and square corners both axes. Every physical part, services, equipment/support frames and conservative capsule envelope is a barrier. Actual face triangulation, triangle/prism SAT and ray containment replace AABB-only conclusions; NumPy clipping/solid-angle and NetworkX graph oracles agree. Detailed proof and semantic source fingerprints are in circulation.md.

This proves the defined geometric routes in an inferred reconstruction, not surveyed historical identity or regulatory egress certification. Full doorway-volume and structural-bearing checks are separate complementary gates. Subsequent capsule/podium fidelity work is a new active phase and must preserve these constraints.
