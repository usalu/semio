# Effective Cargo Profile To Direct Rlib Projection

## Current Source Contract

This implements the supported ordinary native target-rlib mapper/identity closure. It does not rename production Cargo profiles or change global configuration, compiler quotas, Original/lease issuance or unsupported host/binary/wasm adapters. No actual compiler, Native image, cache publication or SDK setup was run by this task.

The Core `native-build/🕸️units/📐️profile` owner defines closed full observed Cargo profile DATA and its copied/frozen reader. Core graph units now carry `compilerProfile` alongside existing exact profile/planning/compilation digests. Existing cfg-relevant `compilerArguments` projection remains available for Source planning, including unsupported producer kinds; full direct-rlib projection occurs separately in library descriptor materialization. Unknown/missing profile fields fail the schema rather than selecting defaults.

Library RustUnit now requires `profileData`; no null/old-format branch exists. The reader validates the complete profile and exact profile-derived argument prefix, and only the fixed enumerated direct rlib flags remain allowed. Caller incremental paths/output flags remain refused. Planning copies/freezes the nested profile before hashing, so an external post-plan incremental toggle cannot change an accepted plan under its earlier digest. Complete profile DATA enters the canonical unit semantic digest and thus recursive dependency digest, actual Nx environment cache binding and artifact receipt identity. Thin/fat remain distinct DATA/digests even when their rlib codegen switches coincide. Old receipts fail against changed profile plans.

## Observed Mapping

| Effective profile | Direct rlib effect |
| --- | --- |
| opt level, panic, codegen units, debuginfo, assertions, overflow | Explicit bounded enum/scalar flags retain actual effective values; defaults may be made explicit. |
| `lto=off` | `-Clto=off` and `-Cembed-bitcode=no`. |
| `lto=false` | `-Cembed-bitcode=no`, **no LTO switch**; preserves Cargo local-LTO behavior. |
| `lto=thin` / `fat` | `-Clinker-plugin-lto` in the qualified ordinary target-rlib context. No literal thin/fat switch is invented. |
| split debuginfo | Exact bounded option when debuginfo is active. |
| rpath | Explicit `-Crpath` only for true. |
| resolved strip | None emits no strip; observed symbols/debuginfo emit exact bounded flags. |
| incremental | Boolean DATA only in descriptor/cache identity. Actual compiler appends its path beneath the checked funded workspace temporary directory after reservation recheck/path validation. No descriptor path is accepted. |

Temporary incremental output is correctness support, not a warm incremental reuse/performance claim; ordinary owned scratch retirement still removes it. The fixed16GiB enclosing filesystem and existing writer/capacity protocol are unchanged.

Host, coalesced host/target, non-build/non-rlib, wasm, non-default codegen backend, absent effective defaults, unobserved true-LTO and unsupported/deferred strip representations remain unavailable. Materialization returns a typed `effective-profile` adapter refusal and propagates it to consumers. The actual generic Cargo fallback stays uncached where the existing selected-graph adapter policy requires it. Root’s separate host oracle is evidence for retaining the refusal, not authorization to expand this mapper.

## Constructor And Caller Closure

The dependency-free `captureRustLibraryDescriptorV1` free profile-argv option was removed. It now opens the actual managed compiler/toolchain store, calls actual `captureCargoCompilationGraphV1`, selects the exact dependency-free target profile, projects it, captures complete cfg and Source/configuration observations while custody is retained, and returns full profile DATA. The actual sole external caller in unit tests was updated; there is no argv-to-profile inference, nullable profile or historical runtime API.

Authored neutral unit/graph fixtures now explicitly state complete profiles and owned argument prefixes. Their supported neutral LTO intent is explicit off. The older planning-only cfg semantics remain. Existing source selection/codegen boundaries and package/compiler environment evidence are retained.

The Native owner concurrently removed its temporary return-handle API. With that owner’s exact contract, the Core unit tests, library unit tests and ticket `registry-ready/native/📜️script.ts` were migrated to `withNativeImageStorageV1(options, async(image,control)=>...)`. Effects use the actual preaccepted shortened callback control, allocation/unit retirement is retained and temporary image.close is gone. Root directories are only raw-removed after known scoped completion; unknown failed partial parents remain protected. This migration is Source/strict only, not renewed physical fixture qualification.

## TDD And Independent Evidence

- Missing projector RED80145 was a real missing-module prerequisite, followed by pure projection115/1665.
- Full-profile unit identity RED66650:115pass/1fail before required DTO/schema integration.
- Nested-profile snapshot RED9223:117pass/1fail exposed post-plan shared-object mutation; copy/freeze fixes it.
- Actual default-local observations from Root15108 preceded false-case RED7078. False was refused before the mapped behavior was added; it was never coerced to off.
- Own neutral corpus contains12 actual ordinary root/dependency rlib observations from Root’s complete release matrix, plus6 local-LTO observations from the later actual prototype matrix. Ajv validates full profiles. Independent SQLite default semantics compare every relevant observed codegen effect, including implicit assertion/overflow defaults; metadata/extra filenames are excluded as separate identities, and actual owned incremental paths are normalized only to a portable observation marker.
- A real issued control (no fabricated Original/store/registrar) drives the actual Core graph→library materializer→unit path with deterministic readonly observation ports. It tests profile DATA transfer, exact unit schema and typed unsupported propagation. This is portable DATA materialization, not a private compiler identity/Native runtime proof.
- Profile-field mutations change the real semantic/cache digest and reject previous receipts. Incomplete prefix and caller incremental path refuse. Actual TypeScript AST checks constructor actual graph/store calls, compiler-owned incremental append after recheck and actual mapper call sites. Embedded graph/unit profile schemas are independently compared to the canonical profile schema to prevent schema drift.

Current Source prototype15650 and release34508 are both collected exit0:118tests/1,813assertions (6.19/6.33s). Expanded strict71210/96020 retained a concurrent Nx-storage `buildMode` type error. After its owner supplied the exact authority projection correction, final expanded strict31813/34326 BOTH collected exit0, including both actual unit-test files, constructor/materializer/compiler and ticket registry fixture. Earlier temporary API/coedit strict failures remain recorded rather than rewritten.

Existing editor-registered `sdk-derivative/📜️script.ts vite-environment|vite-environment-types <mode>` now includes the focused profile laws and expanded actual production/test roots; no new executable command or script exists. All task physical scope remains held.

## Pending Runtime Acceptance

Actual direct compiler semantics are not qualified merely by vector equality: a new frozen SDK/Source capture and authorized tiny direct rlib effects, supported normal/dependency contexts, actual funded incremental path/refusal, Source/cfg/receipt mutation, strict artifact inventory, ordinary Nx cache phases and runtime/performance proof remain required. The revised io-base64 constructor route, direct fixed compiler/renamed-dependency unit tests, selected producer cold/change and consumed-byte ABA laws need current authorized physical requalification. Existing failed Originals/SIGKILL/low-headroom/Source refusals remain failures. Root’s actual Cargo oracle outputs are independent observations, not new mapper runtime successes.

The79-member artifact group runtime is separately accepted narrowly in `build-cache-acceptance-current.md`: positive current-macOS read/FD/lifetime only, no publisher/adversarial/128/platform inference.

## Current Source Census

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🟦️.ts`: 4141B; SHA256 `27b5e4599aa959985eb64720f36b717caf33b8df8ae9514afa3b63041df7ba8c`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧬️schema/🔣️.json`: 2786B; SHA256 `17c4c8dd119781796525746574d502b25705db083c9f967b292e40075f8b19d2`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/🔣️.json`: 13114B; SHA256 `c9efd3a3e77e830f14ff9587796585dc4e1f77010a13b73641359dae62549fee`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/🟦️.ts`: 10293B; SHA256 `f55f04aad8b72bcac61e269729c6aeab471e1988e1ffd3f7557f9afad029a62d`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts`: 10450B; SHA256 `4d9227b959203d536577553f8d89a4aec13949e479dd6bb9cb4c3f9788ebee78`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧬️schema/🔣️.json`: 18061B; SHA256 `8612a30d25ec6009d701753c3985ab263c2db497713cf3760c81a0b5f11c3ec7`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts`: 9610B; SHA256 `e3a5c3e7567c04609c159895b73dfc0b2f4a995de00793e81ed9439328dfc693`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json`: 9106B; SHA256 `bec9a06bccdfb051c31ec412dc442ebdd97d597c0c29e62e172e36748f4b5463`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts`: 8428B; SHA256 `99860121a999917c07f309523343ffe5166473d038d2b6f6e68b66697d89e900`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🟦️.ts`: 7069B; SHA256 `1058e6f98c5d302466acfaf9b409844bf3daa054dc8411dbeaa54b13bdf7bcf0`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts`: 8045B; SHA256 `b5eb6a555e72a98aefc981a2240dafee2ab0b1239c89eb944569e8f1a11d1624`.

## Final Gate And Observation Hashes

All own handles are terminal; no new physical compile/image/publication was launched. Exact final logs and independent actual Cargo observation sources:

- `🗑️generated/rlib-profile-local-handoff-vite-environment-prototype.log`: 13892B, SHA256 `532824021e3d35c1948b4dc52e1804a10fd004044cc6db2bdd937a8dd8aab73a`.
- `🗑️generated/rlib-profile-local-handoff-vite-environment-release.log`: 13890B, SHA256 `65f87aec10563637a450234dae95d8305533006f0b894b186d187f530978e9fa`.
- `🗑️generated/rlib-profile-final-strict-prototype.log`: 0B, SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- `🗑️generated/rlib-profile-final-strict-release.log`: 0B, SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- `🗑️generated/rlib-profile-local-red.log`: 24114B, SHA256 `776a7f8015c362d751d659767162faafb20dbbcb15ac9d08631cf8b6c992ea05`.
- `🗑️generated/rlib-profile-snapshot-red.log`: 18869B, SHA256 `6da8b212685de9b531a70dd03cf2688735f09db30b7ca5f982294a0094343869`.
- `🗑️generated/root-profile-dependency-release.log`: 66556B, SHA256 `de3261a0aa2cfbb830e4b8a0c5d142c23e5274c531cbff27e69a103402d7c038`.
- `🗑️generated/root-profile-local-cargo-prototype.log`: 34054B, SHA256 `b6c60097cb74771e22b77aa305ad6a78e9e8ac05c3fd541d89d266c22d682204`.
- `🗑️generated/root-profile-local-cargo-release.log`: 34096B, SHA256 `29293deedba89dce89882ab42a09a62aab16914c86612fc2afef25ba226fcfa1`.

Source remains frozen for review. Root/Storage must coordinate any new frozen SDK capture; current physical mapper/constructor/incremental gates remain unexecuted.

The subsequent exact suffix audit/repair supersedes the broad suffix allowlist from this checkpoint. See `rlib-profile-suffix-current.md`: genuine opt3→opt0 override RED, closed scheduler-only suffix, shared constructor/materializer argument projection, typed compiler-argument refusals, BOTH119/1973 and expanded strict0. Physical acceptance remains pending.
