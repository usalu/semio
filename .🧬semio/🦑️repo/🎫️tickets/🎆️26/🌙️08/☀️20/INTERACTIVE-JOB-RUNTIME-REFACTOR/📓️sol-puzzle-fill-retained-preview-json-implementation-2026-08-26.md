# Puzzle Fill Retained Preview JSON Implementation

## Scope

This packet replaces Puzzle3d fill's whole-envelope worker serialization and the Puzzle3d/Puzzle5d renderer-side aggregate projection. Puzzle2d is unchanged. The production renderer now advances a retained fixed-cap JSON cursor and continues consuming the exact last valid page while a locale or fill generation replacement is pending.

## Production path

- Puzzle3d FillBuilder::publish_preview retains the canonical typed preview and publishes an empty worker payload.
- FillPreviewJsonCursor performs a byte census before allocation, reserves the exact bounded output, encodes one scalar field, collection item, or escaped source byte per fuel grant, validates identity and exact length, then atomically replaces the ready page.
- Puzzle3dPrecomputeSession::fill_preview_json_page advances at most 256 one-fuel grants under a two-millisecond deadline and returns the retained last valid page.
- Puzzle3d's main 3D window and Puzzle5d's actual 3D window resolve color plus the active terminology/locale's required fill_progress label and pass the retained page to world3d_scene_extended.
- Puzzle5d's precompute adapter delegates directly to the Puzzle3d retained cursor; it does not recreate an aggregate.
- World3dHost requires a non-empty statusLabel, fails closed for a missing or malformed label, and uses the same value visibly and in the status region's ARIA label. The fill overlay contains no hardcoded English fallback. Brush preview behavior remains separate.

## Bounds and lifecycle

- Output cap: 4096 bytes.
- Color cap: 128 bytes.
- Required status-label cap: 256 bytes.
- Candidate page: exactly eight nullable items.
- Identity fence: operation, base revision, registry generation, fill generation, and preview sequence; locale text is an additional cursor request identity component.
- Expensive phases: retire superseded owner, census, reserve, encode, validate, ready/rejected, incremental close, terminal.
- Zero fuel and reached deadline make no cursor progress. Cancellation retains the prior ready page. Close releases at most one retained owner per call and terminal close is idempotent.
- Already-empty preview and commit outputs remain empty.

## Schema and laws

- 🔭️preview-json.schema.json is the language-neutral JSON schema fixture and requires statusLabel.
- 🔭️preview-json-law.json is the byte law fixture with English and German labels and exact expected JSON.
- A test-only serde oracle independently serializes the same owned projection and is required to match both locale fixtures byte-for-byte.
- Rust hostile laws cover exact output cap and cap plus one, missing/oversized locale labels, malformed/omitted input, non-finite numbers, zero fuel, deadline, stale generation, cancellation at census/reserve/encode/validate and after ready, exact old-page pointer/bytes during locale-invalidated encoding, interrupted close, exact terminal handback, and idempotent terminal behavior.
- Renderer laws cover English/German parser acceptance, visible/ARIA parity, exact ASCII and multibyte UTF-8 label caps plus one, and missing, empty, non-string, or malformed label suppression.

## Files

### Added

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️fixtures/🔭️preview-json.schema.json
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️fixtures/🔭️preview-json-law.json

### Updated

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts

The root 📜️script.ts was not edited; the coordinator owns the focused static-gate integration.

## Verification

Run:

- rustfmt --edition 2021 on the nine touched Rust sources: exit 0. Unrelated formatting in the two large editor roots was manually restored so their diffs contain only the label argument.
- rustfmt --edition 2021 --check on the seven touched leaf Rust sources: exit 0.
- jq empty on both JSON fixtures: exit 0.
- Scoped git diff --check: exit 0.
- Scoped rg census and negative-token checks: exit 0; the only non-test/non-fixture fillBuildPreview producer is the retained Puzzle3d cursor, and the old serde_json::to_vec(&self.preview), serde_json::to_value(build), and session.fill_progress().preview paths are absent.

Not run:

- Cargo build or tests.
- Nx, Bun/Vitest, Wasm, browser, or runtime UI checks.

These were intentionally not run because concurrent Rust packets overlap this workspace. Compilation and runtime behavior are therefore not claimed. No static blocker was observed in the scoped inspection.

## Static gate handoff

Replace the old transport predicate requiring object.insert("fillBuildPreview".into(), serde_json::to_value(build).ok()?) with predicates requiring session.fill_preview_json_page, FillPreviewJsonCursor, the census/encode/validate phases, try_reserve_exact, the fixed caps, StepOutcome::PreviewReady(Vec::new()), Puzzle5d adapter/window consumption, required statusLabel, non-empty parser validation, and visible/ARIA diagnostic.statusLabel. Add negative predicates for the three removed whole-aggregate paths listed above.

## Fresh adversarial audit remediation

The first fresh Terra audit was RED on three exact admission issues; all three source paths now fail closed:

- World3dHost requires candidatePage length to equal eight. Renderer laws include both seven and nine entries.
- candidateGhost is admitted only as null or a non-array object with exactly six schema fields: string targetVortexFullId/objectKindId/meshUrl, nonnegative safe-integer sourceVortexIndex, finite origin[3], and finite orientation[4]. Laws reject arrays, arbitrary objects, missing and extra fields, wrong types, unsafe/negative indices, short tuples, NaN, and infinity.
- Puzzle3d and Puzzle5d label resolution returns Option and recognizes only the explicit en/en-US/de/de-DE tags plus generated native/reuse terminology values. Unsupported locale or terminology returns None. Active render paths return an assembly error; engagement, measure, and context-menu paths suppress output. No resolver maps an unknown value to English or native. Both terminology modules contain four-cell acceptance laws and unsupported-axis rejection laws.

After remediation, rustfmt/write and rustfmt/check on both terminology sources, jq validation of both fixtures, scoped git diff --check, strict-parser token checks, and negative fallback-token checks all exited 0. Cargo, Nx/Vitest, Bun, Wasm, browser, and runtime UI checks remain intentionally unrun under the overlap restriction.

### Second audit remediation

The second fresh Terra audit found one remaining additional-properties gap. World3dHost now uses module-retained allowed-key sets and a non-allocating own-key census during parse:

- A fill diagnostic root rejects every key outside the nine schema root properties.
- fillBuildPreview must contain exactly the 24 required diagnostic keys and no additional key.
- candidateGhost uses the same census for its exact six-key contract.
- Brush-only preview records without fillBuildPreview keep their separate parser behavior.

Renderer laws explicitly accept a schema-valid one-key null-ghost root with the exact diagnostic shape and reject one extra root key and one extra diagnostic key.

After the second remediation, a jq schema cardinality assertion for 9/24/6 keys, scoped additional-properties token checks, negative old-parser token checks, and scoped git diff --check all exited 0. No broad build or runtime command was run.

### Third audit remediation

The third fresh Terra audit found that allowed optional root properties were not validated. World3dHost now keeps `fillBuildPreview` as the root's only required property while validating every present known root property against the schema: strings for target/object/mesh/color, a 128-byte UTF-8 color cap, a nonnegative safe source index, finite origin[3] and orientation[4], and opacity exactly 0.35. Unknown root properties still fail closed.

When candidateGhost is non-null, its six authoritative values must exactly equal the corresponding root values, including sourceVortexIndex and both tuples. Renderer laws accept the one-key null-ghost root and a full nine-key ghost root; reject wrong type, range, tuple length, nonfinite value, byte-cap overflow, and opacity-constant violations for every optional root shape; and reject divergence of each of the six ghost/root authority values. The root census remains an allowed-key census, not an exact-nine requirement.

After the third remediation, the targeted schema assertion, source/law token checks, and scoped diff check all exited 0. Cargo, Nx/Vitest, Bun, Wasm, browser, cache, and runtime UI commands remained intentionally unrun under the overlap restriction.

### Fourth audit remediation

The fourth fresh Terra audit found two schema/producer boundary mismatches. The schema now gives both root and candidateGhost sourceVortexIndex a maximum of 9007199254740991. The retained Rust encoder has the same named bound, preflights the shared source authority before fuel consumption or cursor-owner mutation, and independently guards the root and candidateGhost emission positions. A maximum-plus-one attempt cannot allocate, publish, change the cursor checkpoint, replace the ready owner, or invalidate its exact bytes/identity; on narrow usize platforms the law proves every representable value remains below the wire maximum.

The root color schema retains maxLength 128 and adds the owned annotation x-semio-maxUtf8Bytes 128, matching the encoder and renderer byte census. The language-neutral law fixture declares the safe-index maximum/max+1 and ASCII 128/129 plus multibyte 64/65 color cases. Rust laws consume those declarations, assert the schema annotations and maxima, require maximum output to match the test-only serde oracle byte-for-byte, require the owned oracle admission fence to reject maximum-plus-one and byte overflow, and preserve the prior retained page on hostile index preflight.

After the fourth remediation, jq fixture/schema assertions, rustfmt on the scoped Rust source, source/law token checks, and scoped diff checks were run. Cargo, Nx/Vitest, Bun, Wasm, browser, cache, and runtime UI commands remained intentionally unrun under the overlap restriction.

### Fifth audit remediation

The fifth fresh Terra audit found that the diagnostic numeric fields and status label were not yet schema-coherent with the renderer. All fourteen diagnostic integer properties now declare maximum 9007199254740991 while retaining their truthful minimum of zero or one. The language-neutral fixture enumerates all fourteen fields, their minima, and the shared maximum. A native diagnostic admission preflight covers every u64 and usize field before cancellation, deadline, fuel, cursor, owner, allocation, or publication work; identity admission uses the same bounds. Root/ghost pose and last-sample finite checks plus color and status byte checks are also part of that mutation-free preflight.

Portable Rust laws map the fourteen fixture field names to all seven u64 and seven usize producer members. Every representable maximum must encode identically to the independently fenced serde oracle. Every representable maximum-plus-one must preserve fuel, checkpoint, phase, exact ready pointer/text/identity, color/status owner pointers and values, and empty output/retirement slots. Narrow usize platforms instead prove their maximum is wire-safe and exercise it through the same oracle parity path. Renderer laws admit and reject maximum/plus-one for all fourteen names.

The statusLabel schema retains minLength 1 and maxLength 256 and adds x-semio-maxUtf8Bytes 256. The shared law now covers ASCII 256/257 and multibyte 128/129 repetitions, with declared byte counts 256/257 and 256/258. Accepted labels require byte-identical retained/serde output; rejected labels preserve the last ready owners before fuel use. The root schema also declares x-semio-maxEncodedUtf8Bytes 4096. The Rust oracle asserts that full-wire bound, the existing retained-cursor law covers exact 4096/+1 admission before reserve, and the renderer now applies the bound only to fill diagnostic pages while leaving brush-only preview behavior separate; its direct laws cover exact 4096/+1 pages.

After the fifth remediation, targeted jq fixture/schema assertions, rustfmt write/check, source/law token checks, and scoped diff checks were run. Cargo, Nx/Vitest, Bun, Wasm, browser, cache, and runtime UI commands remained intentionally unrun under the overlap restriction.

### Sixth audit remediation

The sixth Terra audit found that the declared 4096-byte aggregate wire cap was enforced only by the fuel-consuming incremental census. FillPreviewJsonAdmission::read now runs the same FillPreviewJsonPass unit stream used by census and encode, stops at the first byte above 4096, and returns before cancellation, deadline, fuel, cursor, owner, allocation, or publication work. Numeric and floating-point units were moved from allocating format!/to_string construction to a fixed 128-byte std::fmt::Write owner, so this shared admission pass is allocation-free. The incremental census remains as a defensive consistency check.

The language-neutral full-wire law now names stage as its native source, names serde_json as the exact-4096 oracle, records all retained state that 4097 must preserve, and enumerates all eleven native string sources. Rust laws prove exact 4096 output is byte-identical to serde, aggregate 4097 preserves fuel/checkpoint/phase/ready identity and bytes/color and status owners/transient owners, and raw 4097-byte values in every stage, optional, nested ghost, candidate-page, and rejection source fail in admission. Existing color and status-label laws complete the source census. A renderer regression also proves a greater-than-4096 ordinary brush page without fillBuildPreview remains admitted.

Focused static verification on 2026-08-26:

- Both targeted jq -e fixture assertions exited 0.
- rustfmt --edition 2021 and rustfmt --edition 2021 --check on the fill Rust source exited 0.
- A retained-preview region census found no format! or to_string allocation in admission/census/encode unit construction; only the bounded output reserve and final UTF-8 owner conversion remain.
- Cargo, Nx/Vitest, Bun, Wasm, browser, cache, and runtime UI commands were not run under the concurrent-packet restriction, so compilation and runtime behavior are not claimed by this packet.
