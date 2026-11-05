# Staffage Media Freshness

## Audited Chain

The clip runtime fingerprints scene sources and recursively follows literal repository import_module calls. This reaches shared visuals and the staffage bridge; their source bytes and the bridge-owned cast manifest participate in video provenance. The runtime also adds casts_in content identities for every literal cast or declared dynamic family. These identities include authored cast state/view/scale, asset JSON, selected environment presets, exporter modules/schema sources and bundled print profiles. A stale completed video fails reusable_clip; nonreusable renders disable Manim partial-animation reuse. Render publication compares provenance again after rendering.

## Fixed Gap

The content identity functions were memoized for the entire Python process. A catalog, environment, print-profile, schema or exporter edit during a multi-clip rendering session could therefore leave the cast map unchanged at its post-render publication checkpoint. casts_in now clears the three underlying content-input memoizations once at each render checkpoint, then reuses computed identities across that checkpoint's casts. The normal pre/post provenance comparison now observes these edits and refuses stale publication.

## Verification

Six language-neutral JSON vectors cover edits to an exporter module, newly added exporter module, schema, catalog asset, print profile and environment preset. All six failed in a warm process before the implementation fix and passed afterward. The test also verifies the live cooling heating-comparison scene's literal-import dependency closure reaches its scene, shared visuals and staffage bridge. Both tests pass without importing Manim: the test compiles the actual identity functions from their live AST. The third-party jsonschema validator independently accepts the same shared cast entry as the bridge validator. Redundant asset_revision memoization was removed because its input catalog bytes are already memoized; the existing direct-content invalidation test now passes when its input cache is refreshed. No full video regeneration or narration/TTS changes were made.

Added course_staffage and media_staffage to the explicit authoring test command so these gates run with normal verification.

## Guard Data Reconciliation

Removed obsolete rule_b exceptions for now-thermal or removed desk, chair, table, blackboard and crate casts. The existing nonthermal landscape tree exception remains. Added only the exact conduction _brick_block function to rule_a's diagram exceptions: its four masonry courses form two adjoining wall-material control volumes recoloured by temperature trackers before the microscopic conduction lens. This is a section-material study, not a freestanding physical brick object.

## Files

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🧪️tests/🔣️.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️media_staffage.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts

The full guard now memoizes clip path inventories (clip_sources and energy_clip_sources) during one test run. Source text is reread for every scan so concurrent content edits stay visible. The tests do not mutate the source tree.

## Final Full Guard Verification

The complete live staffage-only guard passed all 39 tests in 427.489 seconds (zero failures, zero skips), using the provisioned Manim virtual environment through repo tools exec and unittest module invocation. This includes actual course-source primitive-object/composition checks, precise diagram allowances, thermal/nonthermal policy, projection/mirror/scale/cast checks and scene thermal spans. The authoring test command now registers both course-coverage and media-freshness gates. No further verification sessions are running for this work.
