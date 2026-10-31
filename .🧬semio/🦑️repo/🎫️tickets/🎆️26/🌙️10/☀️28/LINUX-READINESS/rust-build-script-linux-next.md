# Rust Build Script Linux Gate: Current Next Step

Read-only bounded Source/log audit, 2026-10-29. No edits, tests, compiler/native, daemon, lease or process commands were run.

## Outcome

**Earlier gate93180's blanket build-script Source refusal has already been fixed in current Source.** No new schema/effect contract implementation is needed for this WGPU script. The next action is the coordinating owner's existing canonical workspace gate, not another build-script implementation or duplicate runtime launch. This audit cannot establish that the full graph currently passes.

## Original Failure and Producer

Ticket generated log `🗑️generated/workspace-contract-fixture-retry.log:1023` records `refused build.graph.rust-sources-unresolved`, reason build-script, root:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`.

This is Source graph inference failure, not a compiler/linker failure. Existing ticket reports `rust-build-script-gate.md` and `authored-build-script-inventory.md` already document both original diagnosis and completed fix. The latter found one authored enabled build script; no new whole-workspace scan was repeated here.

Producer Cargo.toml:8 declares build.rs. Adapter build.rs:7 literally includes ../../🏗️builder/🦀️.rs. Builder 🏗️builder/🦀️.rs:4 reads CARGO_CFG_TARGET_OS and only on Windows reads CARGO_CFG_TARGET_ENV, choosing /STACK:8388608 for MSVC or -Wl,--stack,8388608 otherwise, then prints cargo:rustc-link-arg-bin=semio-wgpu-native=... at line6. Linux emits no linker directive. Preserve these bytes and Windows behavior.

## Exact Current Contract and Consumer

Base below is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/`.

- `🧬️schema/🔣️.json:81`: BuildScriptPolicy limits64files,1MiB/file,16MiB total,256path units and target environment names.
- `🧬️schema/🔣️.json:93`: closed BuildScriptAssertion requires version1, kind link-only, exact script path, unique declared target environment subset, effects exactly [cargo:rustc-link-arg-bin], outputs exactly[], and finite SHA256 file rows.
- `🟨️.mjs:90`: assertedCargoBuildScriptInputsV1 validates the assertion, rejects ordinary **and target-specific** build-dependencies, exact Cargo-selected script identity, opaque/generated/escaped/duplicate paths, nonregular/symlink leaves, oversized inputs and stale hashes. It requires equality with discovered literal source closure.
- `🟨️.mjs:120`: actual cargoSourceSetOfV1 now calls this admission before returning unresolved(build-script). Successful assertion proceeds to normal rustSourceFiles derivation; no fallback glob and no blanket script acceptance was introduced.
- WGPU Cargo.toml:360 now supplies the authored package.metadata.semio.build-script contract, exact environment and effect class, no outputs and the adapter/helper SHA256 rows at368/372.

Independent read-only Python SHA256 comparison during this audit confirms **both current files match their authored hashes**:
adapter596f5b41bde0f0527d707cebd2e4ca31578aae654e8013e2002cc6d6ea6fc060;
builderba342d6001b13c69cd0ef660b9812a98e1cf8912aaa5e3af89c97f3825aa221c.

This is a reviewed-byte assertion, not automatic semantic proof that arbitrary Rust is side-effect-free. The implementation docstring explicitly retains that distinction.

## Existing Tests and Evidence Boundary

`🧪️tests/📋️build-script/🟦️.ts:14` tests the actual assertion helper extracted from Source against language-neutral 🔣️.json, closed schema, Ajv, independent Bun SHA256 and SQLite closure equality. At line30 the real cargoSourceSetOfV1/rustSourceFiles/helper are jointly exercised with finite filesystem ports. Unknown scripts, stale assertions, dynamic/unlisted includes and build=false remain distinguished.

Existing rust-build-script-gate.md reports16semantic cases plus6wiring cases,81assertions; it also records real rustc oracle79034 success with emit=dep-info matching the two-file closure. These are prior reported executions, not rerun here and not a whole-workspace green result.

The smallest clean schema-first exact contract requested already exists in these files. Keep its unknown-script refusal and closed effect/input boundary. If a new canonical gate still reports this root/reason, inspect the selected Source snapshot/observed hashes and actual new terminal log before making changes; the historical93180 failure alone is stale evidence of a current defect.
