# Peer Assembly Current-Behavior Review

Observed 2026-10-20T23:38:47.080Z. Static current-Source review only. No Source edit, function evaluation, test, native command, seal refresh or process operation occurred. Root reports the earlier native75119 cohort revoked and its owner handling cancellation/terminal drain; this work did not touch that handle. Source edits remain on hold pending Root's actual-terminal confirmation.

## Evidence Boundary

The current project assembly is exactly 37,273 bytes / SHA-256 b7802a9f54b5e52b34a4ea17bac72a2db69d3f40f5c0bfceb310fd57cfee0dc0, matching Root's reported peer edit. Root's prior sealed observation is 37,226 bytes / SHA prefix76c1ba. I have no retained full text of that old version in the recovered read-only kernel or the earlier Cargo review reports. Those reports describe consumer behavior but are not a complete old Source image. Therefore this is an honest current-behavior review; the 47-byte size difference does not establish the exact textual delta, author intent or exclusive affected lines. I did not reconstruct an alleged prior version or overwrite the peer's work.

## Actual Cargo Consumer Still Fits

The exact existing test `live inference bootstrap publishes the same closed project helper exports at cold and revised loads` remains at 16,348 bytes / SHA-256 3bb8fee29bc04cf9305a1ec8214d6a6645f6383d5fa61802603997c0c08de703. It selects the actual current assembly dependency declaration, actual wire and actual projectWithDefaults AST; it does not embed an old copy of that consumer.

- Current wire lines202–210 still obtains loaded from registry.bootstrap and binds the same collaborator domains. Its dependency declaration at line199 still contains loaded and the functions used by this selected path.
- Current projectWithDefaults lines291–295 checks whether a script exists, derives nativeRoot from metadata or root, conditionally admits truthy metadata.nativeRoot, then invokes loaded.cargoManifestInputV1(nativeRoot) unconditionally before target construction. The exact original missing-facade failure boundary therefore remains present.
- The six fixture cases pass no metadata.nativeRoot, report no present files and build zero targets with controlled empty collaborators. Their valid workspace/nested/Unicode roots still reach the callable helper. Removing cargoManifestInputV1 at cold or revised publication still must fail at the unconditional call before the projectInputs completion sentinel.
- Current target construction (line297), generator-owner iteration (308), admitted empty target set (309), fallback collaborator (311) and final projectInputs call/return (394–399) remain compatible with the existing controlled registry. All normalized-target loops are empty in these cases; the returned targets remains empty and namedInputs remains the exact sentinel object. SQLite still independently derives expected acceptance from the actual own callable data descriptor and expected manifest coordinate.
- The actual cold initializer and Object.assign revision publication are still read and selected from the true bootstrap. The existing helper-identity and descriptor oracle contract remains relevant; no fixture adjustment, additional collaborator stub or fallback was found necessary by this read.

The six-row facade test deliberately does not claim to validate malformed owner coordinates, target-command normalization, generated dependencies, caching/native routing, or nonempty target loops. The separate existing eleven-vector test remains the actual inferProjectInputs plus strict Source DATA malformed-coordinate coverage. A peer edit in an unentered target branch is outside the zero-target facade test's acceptance scope even if that test later passes.

## Substantive Current Target-Branch Boundaries

Current nonempty-target normalization preserves explicit Source/orchestration gating before native routing (lines328–331), requires native-project ownership for explicit native execution (329), and requires a Bun command for the native owner-command route (335). It projects the manifest through the reviewed loaded helper and uses that same manifest for declared native command ownership/driver invocation (340–341). The process-owner wrapper excludes an already native owner-command (349), preventing that path from being wrapped twice. Generated dependencies are joined from actual targetScriptSources (377). Native lock input handling, cached precision derivation and generated output coupling remain separate subsequent branches.

The current inherited-target construction at line324 supplies default inputs for an authored cache:false target lacking inputs, while line326 substitutes project/workspace coordinates in a string cwd before target policy evaluation. These are concrete current behaviors, not attributed diff claims. Reviewing their broader correctness would require their own target-policy fixtures and actual selected nonempty consumer coverage; the Cargo zero-target regression supplies no evidence for those branches. No new concrete incompatibility in the selected Cargo regression was found.

Disposition: preserve the exact peer assembly bytes. Existing Cargo actual-consumer regression remains statically well formed for its declared six-case purpose. No executed pass or full peer-diff clearance is claimed. Root must use this changed Source hash in any fresh seal after its own attribution review; the revoked native cohort cannot establish this new version's acceptance.

## Exact Current Inputs

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs — 37273 bytes; SHA-256 b7802a9f54b5e52b34a4ea17bac72a2db69d3f40f5c0bfceb310fd57cfee0dc0
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🟦️.ts — 16348 bytes; SHA-256 3bb8fee29bc04cf9305a1ec8214d6a6645f6383d5fa61802603997c0c08de703
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🧫️fixtures/🔣️.json — 11419 bytes; SHA-256 e4371bdd9de647c5b7c9fd0df232fb7a4e1686c1959652e66889b65c5987d279
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs — 8369 bytes; SHA-256 d9aebb8d8e6e0025f1d44e2af7ecd2108fbf1e8051a04fa88fc3cb0203ea49a3
