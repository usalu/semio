# Rust Binary Unit Adapter Design

Read-only Source design audit, 2026-10-15. No Cargo/compiler execution, probes, publication or implementation edits.

## Narrow Supported Extension

Start with an actual workspace `binary` unit performing `build`, whose entire predecessor graph is already supported library units. Preserve refusal for build-script/run, proc-macro, foreign Source, test harness and other unmodeled operations. This gives a genuine binary producer without pretending its generated inputs or host plugins are modeled.

Current seam: `framework/process/artifacts/native-build/🕸️units/🟦️.ts:23` recognizes binary kind but marks only library/lib-or-rlib build output supported. Repository `caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts:17-21` materializes every admitted unit as library with one libNAME.rlib output. Compiler port `units/🏗️compiler/🦀️rustc/🟦️.ts:20-25` likewise requires one exact rlib and hardcodes crate-type=rlib. These are the three production adaptations, not a reason to bypass Cargo's observed unit evidence.

## Schema and Identity First

Retain `RustUnitV1`'s existing binary target discriminator, mode/profile, target triple/host role, compiler/sysroot/linker digests, cfg snapshot, features, captured Source inputs and recursive dependency digests. Add a closed output artifact declaration sufficient to distinguish library, executable and target module outputs and expected executable-mode policy. Refactor the string-output consumers consistently; do not add an alternate legacy binary receipt format.

Separate Cargo target name from rustc crate identifier: Cargo binary names may contain hyphens while rustc identifiers use underscores. Derive this relationship through a closed named compiler-target observation, rather than weakening the current identifier guard. Obtain exact output filenames from the actual admitted compiler's `--print file-names` observation for the same target/crate type/name/triple and effective flags; bind the observed result to the descriptor/compiler identity. Do not append .exe based on the host OS or assume every binary target produces a native executable (.wasm is a material example).

If exact Cargo-authored filename needs an explicit -o mapping, keep it an owned output declaration in the first-party compiler port, not forwarded caller argv. Reject multiple/unmodeled output products until each sidecar is declared and charged. Native Windows PDB/import/debug sidecars and platform linker artifacts require real inventory evidence, not a guessed single file.

`units/🟦️.ts:31-36` already propagates unit and predecessor semantic identities into its plan, mode-specific slot and Nx digest input/dependencies. Include the new compiler-target/output observation in this identity. Existing per-unit output EX, dependency SH and artifact publication charge can remain the ownership model. The promotion expectation already binds exact unit/digest/mode/output slot; its artifact verifier must additionally validate declared output role/executable-mode policy where needed.

## Compiler and Receipt Path

Generalize the existing production compiler port into a unit-kind dispatch, with one common Source/cfg/compiler/dependency verification and ownership path. Library uses its current rlib contract; binary uses actual observed bin output declaration and explicit owned target path. Do not add a facade that invokes untracked Cargo to create the binary.

Keep all predecessor readers held through real linking and consumed-byte dep-info verification (`rustc/🟦️.ts:27`). Only library predecessor artifacts may be supplied to --extern until proc-macro/build-script behavior is explicitly modeled. Preserve the existing restrictive argument admission; binary-specific linker behavior must be captured/validated against the actual retained linker/toolchain rather than allowing arbitrary flags/output paths.

Capture actual generated binary bytes, complete sidecar inventory, hashes, sizes and required mode in the common first-party artifact receipt. Verify from real compiler output before publication, and from actual restored files before cache promotion/consumption. A synthetic or relabeled receipt does not prove executable contents or runnable permissions. Change cache restore tests to ensure executable mode survives, rather than repairing it with an uncharged post-restore chmod.

## Concrete Validation Plan

Extend the current unit schema/corpus and descriptor materialization tests with observed library→binary edges, hyphenated Cargo names, target-specific file names, mode/cfg/features and explicit unsupported predecessor cases. Ajv independently validates closed output contracts; SQLite can compute expected dependency/order/count cases; independently observed compiler file-name output must validate the production naming interface.

Add a small actual native binary fixture with one independently owned library dependency. Compile through the new production port, execute its actual returned binary under the existing owned process runner, publish/restore through the supported Nx hook, and execute the restored artifact. Change the binary Source, then its library Source, and independently demonstrate changed semantic digest/output receipt and changed runtime result. Prove unchanged inputs permit actual artifact reuse. Use the existing prototype/release command registrations and register any new executable test command in launch.json; all permanent commands remain existing/respective 📜️script.ts.

Test mismatched target filename, foreign/missing sidecar, altered binary bytes, lost executable permission, copied receipt and promotion after another revision replaces the stable slot. Existing output custody must hold through binary consumers just as library linking does. Cancel during compilation/linking and assert accepted child termination/drain precedes workspace/output ownership release.

## Explicit Blockers

Binary support does not make common build-script/proc-macro/registry chains supported. Effective platform linker flags, native sidecars and exact executable permissions need production evidence. The current paid CPU inheritance frontier also remains separate: binary adaptation must not adopt an environment-only grant. No implementation readiness, runtime result or universal target coverage is claimed by this design.
