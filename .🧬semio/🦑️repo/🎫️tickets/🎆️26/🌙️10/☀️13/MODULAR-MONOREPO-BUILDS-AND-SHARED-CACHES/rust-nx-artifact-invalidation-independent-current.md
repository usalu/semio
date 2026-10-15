# Rust Nx Artifact Invalidation Independent Audit

Source/config-only review, 2026-10-15. No tests, runtime, images, or production edits.

## Concrete Requirement Coverage Gap

Per-crate Nx artifacts are not universal for authored Rust builds. `library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts:17-18` refuses foreign-source, binary, proc-macro, build-script, and build-script-run adapters and propagates those refusals to their dependents. `library/🏗️build/🕸️graph/🦀️rust/🟦️.ts:44` switches the selected target to bounded Cargo fallback when any unsupported adapter exists; `:30` explicitly sets that target's `cache:false` and `outputs:[]`. This routes the whole selected target through Cargo rather than publishing even its otherwise supported per-crate Nx outputs. An authored build selecting a library with a build.rs or proc-macro/registry dependency therefore cannot satisfy universal artifact-based Nx caching through this route. Without declared fallback it refuses instead of caching.

This is a concrete capability gap, not a stale cache-hit defect. A bounded declared fallback specimen is encoded in `graph/🦀️rust/📥️intent/🧪️tests/🔣️.json:61-76`. Reproduction from Source is to select such a declared target whose actual unit graph includes one listed unsupported adapter, then inspect the resulting target's uncached empty-output fallback. No runtime reproduction was executed here.

## Supported Library Invalidation

`units/🟦️.ts:31` hashes normalized source/configuration inputs, features, cfg and cfg snapshot, compiler/sysroot/linker identity, arguments, environment, outputs, mode/profile, and recursively resolved dependency semantic digests. A changed captured Source digest therefore changes its unit semantic digest and all reverse dependents. Feature/configuration/toolchain changes that change effective unit evidence also change that digest.

`units/🟦️.ts:36` declares literal unit input files and its unit-specific digest environment as Nx inputs, declares direct predecessor receipt files, adds explicit predecessor compile dependencies, and declares the complete unit output directory. Output slots include mode plus target/profile/features/cfg identity (`:31`), isolating prototype and release. Dependency semantic hashing provides transitive invalidation even though Nx's receipt input is explicitly nontransitive.

`descriptor/🎯️selection/🟦️.ts:33-34` reads unit/workspace manifests, workspace lockfile, toolchain/config files along cwd ancestry, and sealed compiler implementation inputs into graph-level observations. Only defining sealed implementation files are returned there as direct unit configuration inputs; raw Cargo manifests/config are not all independently present in each unit Nx input list. Effective Cargo observations are encoded in `descriptor/🕸️graph/🟦️.ts:21`, including compiler arguments, environment, features, profile digest and unit evidence. No concrete semantically meaningful manifest/config change that escapes this evidence was established in this review.

## Limits

No stale-hit defect established for the inspected supported library path. Shared artifact restore concurrency is not qualified by this review: unit output slots are stable across Source revisions, so current restore/publish locking must be assessed at the actual Nx output custody implementation, not inferred from the per-unit producer declaration. No concurrency test or restore runtime was executed. This report does not claim coverage of every generated target or adapter.
