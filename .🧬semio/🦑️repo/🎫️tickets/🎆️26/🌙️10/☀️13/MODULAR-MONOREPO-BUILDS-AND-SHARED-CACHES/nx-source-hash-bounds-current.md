# Bounded Exact Nx Source Hash Reads

## Inspected Actual Seam

The raw MJS digest currently reads complete Source leaves synchronously, allows followed symlink leaves, accepts a caller-owned digest cache, and reads closure/lock JSON and TOML without finite byte admission. The real SDK Graph adapter does authenticate namespace before DATA capture and runs its body inside withReceivedNxGraphTaskV1, but the algorithm only sees before/after signal checks. No expensive read yields to the carried control. Selected Cargo lock lookup also performs unbounded synchronous manifest/lock reads while climbing workspace ancestors.

Actual raw consumers found: digest/script.ts, lock/script.ts, the genuine Graph inputs adapter, the existing build-mode test owner, and the existing Graph DATA test's empty-plan/absent-receipt law. Frozen runtime reexports already target the real Graph adapter; all four production patch consumers await it. No change to those lifetime boundaries or Source snapshot ordering is proposed.

## Chosen Source Integration

1. Keep the exact path/file/missing hash formula and selected Cargo lock closure projection. Required raw functions become asynchronous and take an actual issued ResourceLeaseControl. Remove public reusable Map parameters. A private batch owns all maps and counters; new calls/receipt rechecks always create a new batch. Cross-batch digest reuse is impossible by API. Existing package-body/dependency/absence invalidation stays intact.
2. Add one domain Source-read owner under build/inputs/digest, not a new Native/store/lease owner. It authenticates the actual control before caller DATA and enrolls the complete batch through genuine withRequestedIssuedResourceControlV1(control,0), with every accepted read registered by withIssuedResourceOperationV1. Every async read, ancestry probe, chunk and final close uses that exact carried control. The real Graph adapter passes its actual received phase, awaits the algorithm and validates output DATA. Standalone existing script entrypoints receive their real process Original and author a finite phase; no structural control or custom callback can mint authority.
3. Use the frozen canonical PREOPEN descriptor helper for actual descriptor admission/close, unchanged. The general Core file observer is read-only input to this design: it cannot supply the required async per-chunk yield, explicit EOF replay and complete workspace ancestry behavior, so it will not be copied or modified. The domain reader supplies those Source-specific predicates and batch counters directly. One descriptor is open at a time, plus one 64 KiB chunk (and one admitted finite metadata buffer only for JSON/TOML); large Source is hashed in chunks without file-sized allocation.
4. Each read first captures canonical root/path and all existing physical ancestors (no symlinks). Before allocating, lstat requires a regular no-follow single-link leaf and exact safe extent within both role and remaining batch capacity. Open uses O_RDONLY|O_NOFOLLOW|O_NONBLOCK through canonical descriptor admission, then compares dev/inode/mode/nlink/size/mtimeNs/ctimeNs. Every chunk checks the real phase, rechecks physical identity/ancestry and awaits its continuation. EOF beyond the admitted extent, full postread descriptor/path/ancestry and close must complete before the result/cache entry transfers. Partial buffers are wiped; the helper preserves primary Error/undefined on close. No same-user atomicity claim is made.
5. Preserve missing ENOENT/ENOTDIR semantics for absent inputs and normal-file ancestor blockers, with bounded existing-ancestor/absence rechecks. Present symlinks, directories and other nonregular objects refuse. Missing Cargo.lock retains the exact old missing-marker digest. A selected manifest/workspace/lock stays confined to the explicit repository/fixture root with a closed ancestor limit; no arbitrary ancestor outside it is read.
6. JSON closure/lock inventories are descriptor-read within metadata limits before JSON.parse. Their closed schema, file/name/count/UTF8 byte limits and metadata aggregate must pass before Source reads. TOML is bounded before parser allocation and remains behind the existing first-party selected-lock function; no external parser type escapes. Parser records live only in the current private batch. All actual callers/tests and the precise adjacent MJS declaration migrate together; no sync/optional compatibility branch remains.

## Consumed Finite Policy

These are authored supported-work bounds, not measurements or a promise that every maximum completes in 120 seconds. The policy is copied into private primitive snapshots before any work:

- 64 KiB chunks; one live descriptor; 64 physical ancestors; absolute/relative path at most 256 UTF-16 units and 1024 UTF8 bytes.
- 262144 unique Source paths per batch, 65536 files per closure, 16384 inventories (aligned with existing receipt count), 512 MiB per Source file and 8 GiB aggregate Source bytes. Large Source affects I/O admission, not retained file-sized memory.
- 128 MiB per closure JSON (enough for the complete bounded name roster including UTF8/JSON escaping), 32 MiB per TOML/lock metadata file, and 256 MiB aggregate metadata bytes. Parsed/current metadata plus digest/claim maps remain finite; these are read-memory/I/O bounds, not a Native allocation or quota reservation.
- Existing task/instruction/environment/receipt DATA bounds remain unchanged. Raw exported calls capture the same closed DATA policy before their first await; no caller can mutate policy or caches.

The byte/file choices intentionally support the whole bounded roster and large first-party Source rather than following the observed 1.1 MiB closure. Unsupported inputs refuse before whole-file allocation. No global physical quota, cache volume, compiler floor or Original budget changes.

## TDD and Runtime Plan

Schema/corpus first, genuine failing Source/ordinary-file laws before implementation. Actual received Original/control laws will exercise the consumed owner: byte/file/aggregate refusal before buffer/read, regular/symlink/hardlink/ancestor/EOF/identity cases, same batch dedup but changed next batch, missing-to-present invalidation, cancellation and thrown undefined. Independent Ajv/SQLite boundary decisions, Node crypto/file bytes and TOML selected-closure outputs will agree with our results. No fake Graph namespace, Native/provider or structural control is accepted. Failed authority cases stay in their own child Original where needed.

Use the existing Source route with the new permanent test leaf and actual caller strict roots, retaining both previous DATA snapshot/authentication Source laws. Only existing script.ts executables are extended; every top-level Bun validation is born with both compile caches disabled. Strict remains serialized and uses unchanged 120-second Originals. The managed Graph store-to-hash completion and coupled physical retirement remain unqualified; ordinary filesystem fixtures do not assert those physical properties.

## Actual Source Integration and Ownership

The required raw signatures now include genuine carried control and return Promises; every discovered caller is migrated and awaited. The existing Graph adapter still authenticates `requireNxGraphNativeDirectoryV1` BEFORE caller DTO capture. Its existing `withReceivedNxGraphTaskV1` then supplies the actual retained graph-task phase to the async algorithm. That actual owner delegates to the privately authenticated graph task owner; this patch adds no registrar, provider, Native receipt or alternate graph scope. Existing frozen runtime static reexports and the four awaited patch consumers remain unchanged.

Source paths remain confined to the explicit root. Metadata paths use only two fixed namespaces captured from the existing `repositoryBuildGraphDataDirectoryV1` owner before the first await. That preserves genuine privately managed dataset remapping instead of reconstructing a free `.cache` path. The key must be 64 hex digits and descriptor-captured bytes must hash to that key. A namespace path is DATA, never Native authority.

The opaque read token is admitted only by its actual factory. Token copy/retirement/foreign-root checks refuse; a caller Map cannot seed stale content. Reads are serialized inside the batch and registered under actual canonical operations before I/O. Batch births close at callback return/failure; all accepted reads drain before the requested scope settles. First recorded read/body failure remains primary, with a bounded three-entry ordered failure ledger preserving thrown undefined. Private facts expose counters only. A successful duplicate within one batch consumes no extra I/O; every new call/receipt check uses new current bytes and missing records.

Before any file-sized buffer, role extent and aggregate counters are checked. Source is streamed with 64 KiB chunks; only metadata gets its already-admitted whole-body buffer. Actual descriptor/path/ancestor checks occur before every chunk, after EOF and before close. Metadata lexical validation and immutable object traversal also yield/check the same live control. JSON.parse and the existing behind-interface TOML parser are indivisible synchronous bounded calls, with control checks before/after; no hard OS syscall or parser preemption claim is made. TOML resource admission is deliberately conservative: raw bracket/container/work limits may refuse otherwise valid TOML containing many delimiters inside strings/comments. Parsed object depth/node limits additionally apply. No parser grammar or arbitrary metadata acceptance claim is made.

## Evidence and Honest Limits

The original `nx-source-read-red-prototype.log` is a genuine ordinary-file RED: the old consumed digest followed a symlink and returned success. `green1` is a retained independent Node oracle fixture bug (it updated the file marker before detecting a missing file), corrected to read first. `green2` is an earlier8-failure child checkpoint. `green3` is a retained new observer failure with read=false. The precise scheduling/expiry cause was not captured. Canonical yieldContinuation uses setImmediate; the earlier microtask-only description is retracted. The changed observer adds bounded 1ms event-loop pacing within the unchanged genuine 10s child Original, propagates observer failure and logs terminal DATA. No production or Native cause is inferred. `green4` and later show read=true for actual identity replacement and SIGINT.

Current Source final logs, strict results and exact hashes are listed below. The fourteen expected failures each receive their own genuine child Original, fail terminally, and are inspected as DATA by the independent healthy parent: symlink, hardlink, symlink ancestor, directory, escape, Source/JSON/TOML overextent, metadata depth, file-count roster, mid-read identity replacement, competing read error then body undefined, SIGINT after actual bytes, and body undefined while an accepted read drains. These do not catch/reuse a failed Original. Pure extent decisions consume the same actual admission predicate and agree with Ajv+SQLite. Actual ordinary Node crypto/file bytes and independent Bun TOML parse/selected expected projection agree with the fixture outputs.

All validation parents are born with BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 and NODE_DISABLE_COMPILE_CACHE=1. Existing authored 120s limits remain unchanged; strict collectors are sequential and include extendedDiagnostics. The command remains the existing registered `typescript-artifact/📜️script.ts test|types-owned prototype|release`, selected internally by SEMIO_NX_SOURCE_HASH_ONLY=1; no new human command/launch entry or executable filename was introduced. Existing SEMIO_SELECTED_SCOPE_SOURCE_ONLY selects the prior auth/DATA/cross-patch regressions.

This is bounded ordinary Source filesystem I/O under genuine control, not physical Native qualification. No Native/image/Rust/Cargo/compiler/SDK/network/service/publication/retirement operation ran. Managed Graph reader retention through full hash completion, mandatory coupled eviction, frozen SDK callback identity and physical cache funding remain independent unqualified frontiers. Same-batch dedup does not freeze an arbitrary same-user filesystem or provide a multi-file atomic snapshot; subsequent receipts must recheck current content. No cap, floor, byte charge, namespace or Original deadline was changed.

The complete expanded strict route keeps the actual historical build-mode test caller plus its contract test imports. Its failures are retained as actual caller diagnostics, never dropped or stubbed to create a green aggregate. Root can coordinate those concrete owners after this bounded Source freeze.


## Frozen Result Capture

- Final hash Source: prototype and release each3 tests/107 assertions,0 failures,8.63s/8.31s.
- Expanded actual caller strict: BOTH exit1, identical32 diagnostics in the two pre-existing build-mode test owners below; all new reader/digest/script/Graph adapter/DATA/frozen-runtime roots have0 diagnostics within that complete run.839 program files; prototype424165K/17.96s (program3.23s,check13.50s), release410003K/28.27s (program5.50s,check21.96s). These are actual extendedDiagnostics observations, not an attribution to contention.
- The earlier types1 own fixture narrowing diagnostics and types2 remaining one expect overload are preserved; corrected using genuine descriptor/data narrowing and an unknown deep-equality assertion input. No source root, strict flag, schema or assertion was removed.

### Complete Caller Diagnostic Roster

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/📥️contracts/🟦️.ts(33,407): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string | URL'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/📥️contracts/🟦️.ts(33,464): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/📥️contracts/🟦️.ts(33,1498): error TS18048: 'file' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(20,53): error TS7016: Could not find a declaration file for module '../../../🏗️build/📥️inputs/🔒️lock/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(26,49): error TS7016: Could not find a declaration file for module '../../../../🧪️test/🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(111,23): error TS18046: 'cacheInternals.cargoTargets' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(135,43): error TS2345: Argument of type '(Record<string, unknown> & { defaultConfiguration: "release" | "prototype"; configurations: Record<string, unknown>; metadata: Record<string, unknown>; }) | undefined' is not assignable to parameter of type 'Record<string, unknown>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(138,95): error TS2345: Argument of type '(Record<string, unknown> & { defaultConfiguration: "release" | "prototype"; configurations: Record<string, unknown>; metadata: Record<string, unknown>; }) | undefined' is not assignable to parameter of type 'Record<string, unknown>'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(146,25): error TS18046: 'cacheInternals.genericTargetCommandInputs' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(147,88): error TS18046: 'cacheInternals.genericTargetCommandInputs' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(148,16): error TS18046: 'commandSources' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(149,26): error TS18046: 'cacheInternals.genericTargetCommandInputs' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(152,17): error TS18046: 'cacheInternals.bunLockGraph' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(162,34): error TS7006: Parameter 'input' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(167,28): error TS18046: 'cacheInternals.genericTargetCommandInputs' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(168,1080): error TS18046: 'cacheInternals.nativeTargetCommandInputs' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(170,585): error TS2345: Argument of type 'string | undefined' is not assignable to parameter of type 'string'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(184,106): error TS7016: Could not find a declaration file for module '../../../🟨️.mjs'. '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs' implicitly has an 'any' type.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(185,145): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(185,199): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(187,103): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(187,242): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(188,56): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(189,229): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(190,53): error TS18046: 'goDependency' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(191,15): error TS18046: 'goOwner' is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,69): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(194,69): error TS2571: Object is of type 'unknown'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(202,89): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(202,100): error TS18048: 'line' is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(206,15): error TS2532: Object is possibly 'undefined'.
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts(206,15): error TS2571: Object is of type 'unknown'.
```

Root ownership coordination: actual build-mode test owner and its contracts child need real checked values/precise first-party JS inference, not ambient stubs or dropped roots. This patch only migrated the actual hash call hunks there.

### Exact Source Hashes

These16 explicit owned/caller/wrapper files are a Source snapshot, not a whole transitive SDK closure or physical publication epoch. Other lanes may later edit the shared wrapper/caller; those successor bytes must be qualified separately.

| Source | SHA256 |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🔣️schema.json` | `435301dabeca9e668557adb1e58a2cd54c993f5513062f8dc85d7ff6d377add2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs` | `56136d8319f473e7cdc809b6fd1aefa4d16c135a5c2abb09d946844e814ec182` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.d.mts` | `2cc28cde1af7f99a874347d4f3bb4cb0b5ee4cba3934c582dd49ac20b18c8af0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts` | `dc5249bc819fc69ffba3baece9546a9131d3f53d0c683ad33563ce62829b9049` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json` | `3bb515be6be4314e43fd8026c9f8ccdcf97a33b2153a62970d00475203d7f54e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs` | `f1ee05c507e58ecf7e08ab2f60cbbb44825938d49c42a4b65b08e7def3c8ed19` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.d.mts` | `c893f25ed37fe379024056f4cce1aab8b64a18c3fd27884a0e05dd5f1fc11b11` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🟦️.ts` | `7736f336f984fcc52413c90fe174eef4a1aacf96f9f0b2e9c0dbbf39d0494aa4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🔣️.json` | `f813d274807400fc007cf98991d108efce21445da82f1b4c636c97b052794bdc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs` | `68ca1400d1cd166cc748a663e1ee8b033c6fa2f74cac6d316ae93445880610a7` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.d.mts` | `b418c8a81d0e1ac890a289ff3c6b280dce04521337bb22d88fe8669b6e50e928` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📜️script.ts` | `7052cd8054ee8874e632646ca372696063ce95ae3a6461e2cf48707a3e1fe745` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/🟦️.ts` | `ba65695fb42df6e66e015132bdcf6c451c8e0da661dd063496abc05a4b92150e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧪️tests/🟦️.ts` | `a0c8a31dc78acfe90624a63f092244b648290d6b524ff16c4d47a550f105783b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts` | `05a5d3b31fca5a2764cf0a5915e338ceefe3add0ea4fcd49550a09ab9753c90e` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `2242971af82da86de37b4be795f1bc8952cea0236b9862ffc9572886f403323c` |

### Retained Generated Evidence Hashes

- `nx-source-read-red-prototype.log` — `1bf3e5bc13815322d6419eae45de1423f6afe678081571807557fbc2f9da1280`
- `nx-source-read-green1-prototype.log` — `3d4ebcb2236ee991cb8726ce59fcfaf74ba9326ca0097f8e109735ec6af6cdc6`
- `nx-source-read-green2-prototype.log` — `63f07b62e0ab74aed9c55975912f89fd87b365fd8c282d894e9a54a6c333db5f`
- `nx-source-read-green3-prototype.log` — `0aebb36779f4059bb8d6cbd778f3d01faf7703f26ca21669083bfbb1b782b155`
- `nx-source-read-green4-prototype.log` — `bdeb7ac11aa69d16cf322ddfa9914c59172f2d6dec0f160f7fa2dc15db42d9f8`
- `nx-source-read-green5-prototype.log` — `f505cdb4c9354310fd60b0d0da0cbe8a66197c1bc8b596801c8f23289a0e97ca`
- `nx-source-read-green5-release.log` — `897736422089acd0699dbd1108c2df6341002f49fcdd8cc80d6ba5307e9061c9`
- `nx-source-read-final-prototype.log` — `62ea516b10cfb52e2f6929f87d4d3ae6ff5a96c9a8348b92901f5d940094970f`
- `nx-source-read-final-release.log` — `6b1721de01495d81a03f2f785fe1cd1dfb09dc83145771f20f9278eb79c12842`
- `nx-source-types1-prototype.log` — `5db194da3a11f0fc6404dcc54332d9d1c2d42a5b0ccb7ff0b12a8b1477f4ef1a`
- `nx-source-types2-prototype.log` — `e3abc4dca4390a4b4e1657d0f86033daa356d795759c57f3bec617e8acc361c6`
- `nx-source-types2-release.log` — `bf2115d8f1e91b48de4522874f9906d9c719730fb68fed050587cc4822681a9c`
- `nx-source-types3-prototype.log` — `1c09beb3a43a4f6b3911fc2d501137fa7eb3df9cf54c2e075e3a704d6ffc13b6`
- `nx-source-types3-release.log` — `3e0233ad59b53ae5505ef9775ba0f740aa42f31839b22578a6d8efcb69b25d76`
- `nx-source-scope-regression-prototype.log` — `5facec7b5b30a38486bbb09dbc9b4b85eaff25c6caf82cf406c68140d6931dea`

## Final Scope Regression and Handoff

Existing auth/DATA/patch regression Source is BOTH23 tests/199 assertions,0 failures across4 isolated ordinary Bun files. This includes genuine absent receipt/missing task environment semantics, namespace-before-hostile DTO admission and Root’s four-awaited-consumer AST law. No managed Native positive namespace was fabricated. Final actual caller strict remains BOTH32 diagnostics exactly as enumerated above; zero new-owner diagnostics does not turn that complete caller check green.

All own children/Originals are collected terminal and all exact-own ordinary fixture directories are removed. No known own physical image, lease, store or retained descriptor was created in this task. Source is frozen for independent review; no broader retention/cache claim or follow-up physical operation is authorized. No additional unchanged strict or MCP retry was launched.

- `nx-source-scope-regression-release.log` — `b22e1d7560698df6733f7f805fed9286d71d8efc994c4c598eac24ef345ac129`
