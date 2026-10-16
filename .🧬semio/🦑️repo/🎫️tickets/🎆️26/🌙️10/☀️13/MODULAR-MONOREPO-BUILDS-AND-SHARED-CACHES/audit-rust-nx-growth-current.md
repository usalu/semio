# Rust Nx Growth Audit

2026-10-16. Source-only independent review. No test, compiler, SDK, image, Native/storage/lease, network or build effect ran. CPU two-intent files were not edited. Exact Source below is a moving epoch; prior test reports remain prior evidence, not new runtime results.

## P2 — Direct Unit Source Verification Can Allocate Beyond Its Declared Budget

Unit receipt verifyRustUnitInputsV1 lines13–15 calls readFileSync on the complete current input before comparing bytes.length with the descriptor's expected extent. A changed Source/configuration file can therefore allocate an arbitrarily larger Buffer before semantic rejection. Unit Script line31 also reads each entire Source with only a per-file global ceiling and checks aggregate file/byte/active-line limits after full closure discovery. It does not reserve remaining aggregate bytes/files as the corrected ordinary Cargo Source admission owner does (admission lines21–23).

Required correction: use the existing first-party bounded descriptor reader with exact expected extent and remaining aggregate/file admission, retain real cancellation/control and close ordering, and reject growth before allocation. Preserve distinct authored per-unit caps. Meaningful neutral normal-file vectors should cover replaced oversized input, aggregate exhaustion before later reads, changed extent and cancellation. This is an admission-resource defect, not proof of stale artifact acceptance: current digest mismatch still refuses after the expensive read.

## Scalability Frontier — Incremental Slots Are Count-Bounded but Lack Pressure Selection

Compiler-state model lines9–11 chooses an existing clean context for reuse, otherwise an unused slot or least-sequence slot. Schema maximumSlots is8. Volume lines59–60 resets on dirty/missing/digest mismatch or selected context replacement; line63 commits measured state. No pressure-based selection appears in this actual slot decision: eight large clean trees can fill the fixed compiler filesystem while the requested clean context is always reused. Fixed backing limits total growth but do not establish sustained useful iteration or recovery before compiler ENOSPC.

Add a schema-owned physical headroom/retained-state admission decision based on fresh actual census under the already-required EX/full-FD/recensus custody. Retire only authenticated inactive slots; retain the requested reusable context when feasible, and reset its own state explicitly when unavoidable. Keep immutable output artifact retirement distinct. This is a missing growth qualification/design, not permission to delete existing trees or infer fresh capacity. New state creation still requires actual unchanged4GiB floor plus separate4GiB charge; recorded prior free space is insufficient and no fresh observation was made.

## Current Per-Crate Contracts

Unit planner lines30–35 hashes the canonical complete normalized unit plus recursive predecessor semantic/artifact identities and compiler context. Unit facts include exact input content digests, package/version/provenance, compiler/sysroot/linker, arguments, environment, cfg/features, target/host role, mode/profile/evidence and declared outputs. A source-only edit can retain compiler context while changing full plan digest; context is intentionally distinct from immutable cache content identity.

Nx projection lines37–40 creates one rust-unit project per physical variant. Inputs include each exact declared file, its unique digest environment and nontransitive predecessor receipt outputs; dependsOn names exact predecessor compile tasks. Only immutable plan.outputDirectory is declared as output. Compiler cwd/incremental tree and sibling .charge.json are excluded from output archive. Task metadata carries exact id/digest/mode/output into SDK native hash results. This avoids one giant source-crate output/cohort contract in this projection.

Graph materializer lines24–25 requires actual admitted source/cfg/compiler evidence and retains observed profile/compilation/flags/argument/features/dependency evidence. Unsupported adapters are refused rather than silently compiled through the direct mapper. Corrected ordinary admission now populates every workspace unit, including equivalent or differently flagged variants, and applies min(global,authored) file/byte/active-line limits before compiler dispatch. The direct descriptor has a separate authored active-line check. No new dedup skip was found.

Receipt lines18–33 checks exact artifact inventory/content and complete unit/dependency receipt identities. Script lines33–40 holds output EX, binds current dependency receipts, verifies prior immutable artifacts before reuse, publishes replacement with rollback staging and retires prior physical charge only after prior bytes cease being held. Rust promotion lines19–26 validates the expected unit/digest/mode, bounded receipt and full consumed artifact descriptor bytes/inventory. Patched SDK promotion must still retain its actual output writer/reader custody; static full-content verification does not prove the installed SDK has executed it.

## Runtime Proof Boundary and Remaining Work

The recorded Cargo unit-admission law and direct context/receiver laws demonstrate Source/data/coordination behavior. The recorded tiny actual Cargo context sequence (cold2, unchanged0, root-edit1, dependency-edit2, context2) characterizes Cargo rather than production direct-rustc + ordinary Nx cache restoration. Current direct persistent producer volume, edited-Source warm incremental reuse, exact restore of pruned outputs with valid physical charges, corruption/receipt/dependency/context invalidation through the real installed SDK, pressure eviction and cross-platform FD/cancellation behavior remain unqualified.

The ordinary Nx fixture Source already contains cold/package/source/workspace change and cache-status/hash/charge assertions, but reading those assertions is not running them. Required next semantic runtime cohort should establish cold, unchanged, source-only, dependency-only, configuration/context-only, unrelated crate and full restore from pruned output. Confirm per-unit full artifact hashes and physical charge accounting, not elapsed timing alone. Source cap refusal must occur before compiler and must name the offending unit. CPU consumed-profile migration is a shared identity boundary and needs aggregate strict/runtime freeze; no assumption about its latest in-flight bytes is made here.

No new full-content invalidation defect was established in the scoped planner/receipt/promotion Source. The two actionable growth gaps above and existing mandatory Graph closure retention frontier prevent claiming bounded predictable end-to-end warm iteration.

## Exact Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts` — SHA-256 `5ee8c6295e080ba9eb679c815c8583e3058699ed06bae6be16d2065806b95a3a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🟦️.ts` — SHA-256 `128c0276dd1653d4ed8a08cba20a5464ae8c9adbd85da1cb94800abd35413239`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts` — SHA-256 `b8ac972af3ee0de348b01b059e2bb106c11c7a2bd027ac9688446eb157d3f2bb`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts` — SHA-256 `009c0de109f01ae49666dc24dfae4608860d5f9fa2bf2e14ffad79436c578beb`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🟦️.ts` — SHA-256 `d4209f145b7ecb8200499faa1752ced1a2487e85d9541c9c6f8276dc67d7b455`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/🟦️.ts` — SHA-256 `4e2792421f26dfe045edcd27e315be1258e128c44cfc36b57069be602bd87315`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts` — SHA-256 `602a09dd887623a75f3952241ea1e6e9c0808ba8aec6aba169a54fd048ea65b2`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json` — SHA-256 `4f135ab8578eba434689953280a5c1dca4d54247b49b4d3d4f6a85a90a3fdd9c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🦀️promotion/🟦️.ts` — SHA-256 `a7ebbcad3fc73e52586f4acabb18ec15faef2ec78178280abe5dca2d222dbe39`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` — SHA-256 `20a3793e6b5ba079646f0fdf94490bd3a5e056fa5ea0e98dc90ae3649eaa32a2`
