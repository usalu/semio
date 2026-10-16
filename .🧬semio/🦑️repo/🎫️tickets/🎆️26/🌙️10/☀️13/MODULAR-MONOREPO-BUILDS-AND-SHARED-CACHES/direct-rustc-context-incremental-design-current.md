# Direct Rust Compiler Context and Incremental State

## Decision Boundary

This is a read-only Source/evidence plan. No production mapper, metadata argument, descriptor, cache key, namespace, reservation, lease, profile or pruner is changed. No Cargo/rustc/Native/SDK/image operation or test ran in this scope. The existing content-key behavior is conservative; Source-dependent `-Cmetadata` and temporary incremental state alone do not establish a correctness defect. They do establish that direct edited-Source builds cannot reuse the incremental tree from an earlier direct operation, because that tree is removed. Performance benefit and a safe replacement metadata policy still require actual pinned observations.

## Actual Call Graph and Current Keys

1. The dependency-free constructor captures actual Cargo metadata, actual unit graph/profile, compiler identity and cfg. It requires one dependency-free, unfeatured library without build scripts, retains observed package compiler environment and full profile, and includes its physical Source/configuration inventory. Its caller supplies the descriptor id.
2. The production selected path captures actual Cargo metadata/unit graph and Source boundaries, then materializes only supported native workspace target rlibs. Stable Cargo unit ids include package/version/provenance, target/entry/edition, platform, operation, full profile, features and roles. Raw graph dependency indices are replaced by stable ids. Unsupported host/dual-role/wasm/binary/proc-macro/build-script/foreign producers remain refused by the direct adapter. Effective flags beyond the exact owned profile/cfg/scheduler vocabulary also refuse.
3. `planRustUnitsV1` canonicalizes the entire normalized unit (including Source/configuration bytes, complete profile, environment, compiler and exact argv), plus recursive dependency semantic digests and explicit artifact digest when already bound. That SHA256 is the content/semantic plan key. It remains the full invalidation key and must not become a context-only key.
4. The current output slot is a separate 20-hex hash of id/target/profile/features/cfg under `units/<mode>/<slot>`. Default actual Cargo id already includes profile DATA; it is not the content digest. Source changes can reuse a stable slot while changing its receipt and charge allocation. Selected descriptors are immutable per-unit transitive slices; their full content path is in the actual Nx command.
5. The registered unit compiler validates bound digest/mode, rechecks actual current Source boundary, opens the actual Rust image, retains the exact unit-output EX, validates dependency receipts, and binds actual artifact receipt SHA values before compiling. `bindRustUnitDependencyArtifactsV1` returns a plan with bound dependencyDigests and preserves the prior plan.digest; exact artifact SHA is checked again under dependency readers before dispatch and recorded in the final receipt. This distinction must remain explicit in a future context DTO.
6. A cold build opens the actual CompilerStore/toolchain and global compiler writer EX with initial physical headroom admission. It creates a fresh `buildDirectory/units/<random>` output scratch and sibling `.tmp`. The funded path guard requires those exact single-component paths, absence before dispatch, same actual reservation/root/mode/owner and current compiler lifetime.
7. The rustc adapter uses `-Cmetadata=` plus `plan.digest.slice(0,20)`, exact full profile/cfg/features/extern arguments and `-Zthreads=1`. If incremental is true, it appends `-Cincremental=<actual temporary>/incremental` only after reservation/root checks and directory creation. TMPDIR/TMP/TEMP are that same funded temporary. This path is not descriptor-supplied.
8. It validates exact compiler/sysroot/linker identity and current cfg, holds each dependency reader through actual rustc and checksum dep-info evidence, and verifies every dependency receipt/content SHA immediately before dispatch. CPU capacity is retained through the actual child and consumed-byte proof.
9. The compiler owner verifies Source before and after dispatch, captures exactly declared rlib output bytes, and returns an explicitly retirable result. The script reserves/measures staged publication, writes a new receipt/charge, replaces the stable output under custody, retires the previous allocation and always retires the exact fresh output and temporary scratch. On failure, the compiler owner attempts exact workspace retirement; no successful receipt is returned. Therefore the current incremental tree is scratch correctness machinery, not a warm persistent cache.

## Existing Physical and Retirement Protocol

The persistent `repo:compiler:<mode>` charge is one shared compiler-workspace reservation. It is not a physical per-crate quota or a proof that incremental growth fits in its logical charge. The enclosing Rust filesystem is the hard16GiB envelope. The actual global writer EX serializes cold writes, retains actual image/toolchain lifetime and rechecks the unchanged initial4GiB floor; pressure recovery runs under that custody. Compiler CPU scheduling is separate.

The canonical layout is compact `b/p|b/r` and `t/p|t/r`, with captures/tmp under `cargo/<mode>` and registry home shared. Existing inactive retirement recognizes Cargo build units and incremental crate/session trees beneath exact compact roots, under global writer, actual CargoStore EX, physical profile locks, full descriptor inventory and recensus. `scanCargoIncrementalUnits` retains the newest finalized session plus working sessions and exposes superseded finalized sessions separately; a `-working` row prevents whole-crate retirement regardless of age. This is Source-recognized Cargo layout, not permission to drop a direct compiler tree into that layout and assume ownership. Direct temporary trees currently disappear rather than join this protocol.

There is no independently persisted per-direct-unit incremental charge, manifest, byte/entry ceiling or explicit retirement unit today. A proposal that only changes `-Cmetadata` or retains `<random>.tmp` would leak unaccounted state and is rejected. Unknown trees, lock inodes and historical material remain protected.

## Retained Pinned Observations

Actual existing local-LTO Cargo matrices contain metadata and extra-filename even though the earlier summary emphasized optimization flags. Both root-profile-local-cargo-prototype.log and root-profile-local-cargo-release.log record these exact pairs:

| Declared case/context/crate | metadata | extra-filename | incremental |
| --- | --- | --- | --- |
| prototype-local/single/root | bfadebb81a9c5209 | -fb37ad33229594e6 | owned prototype path |
| prototype-local/dependency/leaf | 27e885f81b6e7274 | -294e05317b8968ff | owned prototype path |
| prototype-local/dependency/root | 28ec790314ddfe06 | -859d912d5ee6c565 | owned prototype path |
| release-local/single/root | c0f74f873082da93 | -15b6c18566834479 | absent |
| release-local/dependency/leaf | 80cf90c058717ed7 | -848a0d424432b90e | absent |
| release-local/dependency/root | 41ee9b97bcc643ae | -03d0cba9e271ba92 | absent |

Outer receiver mode, concrete scratch parent and incremental root differ between those two runs; the six values match. This is an observation of these fixtures only. Single/dependency root values differ, and profile cases differ. No same-crate body edit, dependency edit, same-body semantic environment edit, incremental query reuse or direct-rlib warm speed was characterized. No mapping to Cargo's private metadata hash algorithm is inferred.

The authored repository census did not find Cargo's internal `UnitHash`/metadata/fingerprint Rust Source. The Source pin is nightly-2026-07-20/minimal with rust-src and llvm-tools-preview; rust-src is standard-library Source, not demonstrated Cargo internal Source. Existing retained actual compiler observations are the presently available pinned oracle. No unauthenticated tool cache or current web Cargo implementation was adopted as the pinned algorithm, and no tool image was opened to hunt for it.

## Narrow Proposed API and Supported Context

Propose a pure `RustRlibCompilerContextV1` DATA owner next to the direct compiler, after observation. It would have two explicit APIs:

- `readRustRlibCompilerContextV1(value: unknown): RustRlibCompilerContextV1`: closed schema validation/copy. This issues no compiler/storage/control authority.
- `projectRustRlibCompilerContextV1(context): {version:1,digest:string,metadata:string}`: canonical deterministic context projection. The existing plan.digest remains untouched as the complete content and cache/receipt identity. Neither context nor metadata is an output authorization.

The initial context must be constructed from actual captured Cargo facts before materialization, with explicit package version/provenance, exact target/edition/role/native triple, operation, full observed profile, features/custom cfg/full cfg, exact projected compiler flags, complete private compiler/sysroot/linker identities and complete effective semantic package/custom environment. Existing RustUnit does not expose package version independently of its actual default id; do not parse a truncated id or guess it. A version/provenance field must be carried from actual owner facts if observations require it. Defining compiler-driver code may conservatively join the context so adapter changes never silently reuse historical internal state.

Own Source paths/content, output artifact bytes and dependency content changes still always enter the content plan/receipt. Whether they enter the separate crate metadata context is an **observation question**, not an approved omission. Direct dependency contexts/extern names/public/noprelude facts must be explicit if supported; never replace validated dependency receipt digests with context digests. Actual incremental path and TMP are physical owner results, excluded from a portable DATA context only because they are not caller-controlled semantic inputs and rustc receives their real owned values. Synthetic SEMIO_CARGO_UNIT_EVIDENCE currently derives selected compile facts; separating it from compiler-visible environment needs exact Source/env closure audit, not a name-based blanket exclusion.

Supported proposal remains only current ordinary native workspace target-role rlib builds. Host/dual-role, wasm, binary/proc-macro/build-script, foreign-source, deferred/unqualified profile defaults and unobserved compiler flags stay typed refusals. No external metadata/hash algorithm promise or extra user mode.

## Finite Observation Plan Before Production Mapping

Extend the existing guarded profile fixture owner/registered route (no new executable script or mode). Stage a closed language-neutral experiment corpus first. Each preauthored Original retains unchanged120s total/110s body/10s cleanup and the actual CompilerStore/global writer/CPU1/offline/4GiB floor. No execution is authorized by this report. Parent must schedule the read/write window. Fixture-owned files remain within the current tiny2MiB complete inventory bound and are removed under the retained writer before its scope closes.

A minimal first operation uses the current prototype-local incremental setting and one dependency graph (two ordinary rlibs), not a rerun of all unrelated profile cases. Five phases: cold; unchanged; root body edit; leaf body edit; one explicit profile/codegen-context edit. Retain the same exact fixture path/build root across phases under one already-accepted scope. Capture every actual Cargo JSON compiler-artifact record, fresh flag, complete actual rustc argv/metadata/extra-filename/incremental payload, graph profile/unit edges, exact bounded session roster and rlib SHA/size, with wall/child stage metrics. An unchanged phase may produce no rustc command; this is an actual Cargo hit, not missing evidence to fill with invented argv. Root/leaf edited phases must execute their genuine affected closure.

Independent TOML parses authored effective settings; SQLite derives affected closure. Python shlex independently decodes every retained complete verbose command and agrees with the closed first-party argument parser, including Unicode/spaces. Do not normalize away metadata or extra-filename differences. The complete bounded output is inspected before fixing expected relationships in the neutral corpus. For direct-rustc comparison, a separately approved second tiny operation can replay only exact observed supported flags under the same physical writer policy; it may not expose arbitrary flags or reuse state without a published ownership design. Query reuse requires actual pinned incremental statistics or another independently justified observation; session path persistence alone is not a speed or reuse proof. No unstable query-statistics flag is added to production solely for this oracle.

Required distinctions: own Source edit versus package/profile/feature/cfg/environment context edit; dependency body edit versus changed extern edge/context; outer Original identity/temp placement versus semantic argv. If five phases or complete post-measurement cannot fit the existing budget/bound, preserve genuine failure and decompose independent experiments rather than increase limits or sample away checks.

## Bounded Warm-State Proposal, Not Yet Approved Implementation

Phase A can change context projection only after exact observations and pure Ajv/SQLite/canonicalization vectors, while keeping current temporary incremental deletion. This isolates crate identity correctness from persistence and makes no warm reuse claim. Tests must show context/content distinctions plus unchanged full Source/receipt/cache invalidation.

Phase B needs a distinct truthful owner lifecycle under existing enforcing Rust storage before retaining any state. It must authenticate exact current compiler context, reservation and retained output/global-writer custody, precheck physical headroom, validate a closed own manifest and every retained path, and retain custody through rustc, Source/dep-info verification, receipt publication and failed-output cleanup. Reuse cannot follow a path/receipt alone. A context-local incremental tree must have bounded files/entries/bytes plus explicit finalized-session retirement, retained prior-state failure behavior and exact recensus; those data limits do not create a filesystem quota or hard pre-write growth guarantee. Rust16 remains the actual hard envelope, and ENOSPC must fail without publishing an artifact.

Do not invent a new quota, namespace or lease now. Candidate placement can only be decided after reviewing whether the existing compact Cargo incremental owner can truthfully include these explicitly authored direct units with its same global-writer/store/profile/descriptor protocol, or whether a dedicated bounded owner is required. Existing global writer already serializes admitted direct cold writes; do not introduce an opposite lock order. Cleanup must happen only after actual child/descriptor settlement and must never wait for the operation's own output/dependency/image reader. Pure artifact hits/readers remain separate. If existing authenticated retirement cannot reclaim a proposed persistent unit, persistence remains refused.

Validation gates before any performance claim: context/body-edit observations; neutral context schema and exact mutation vectors; retained/foreign/stale/escaped actual custody refusals; bounded overgrowth and ENOSPC failure/no receipt; actual concurrent reader/compiler/retirer behavior; Source/dependency edit correctness and actual query reuse; repeated warm edited-source timings with unchanged caps. No current checkpoint qualifies these future gates.

## Source/Evidence Hashes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts`: `e3a5c3e7567c04609c159895b73dfc0b2f4a995de00793e81ed9439328dfc693`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🟦️.ts`: `95d82afff0fa59294e3f2c54bbefcaa11362f51bd201161deb040f8ef68e257d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts`: `910e222d741896bcfcdb27d9acdbf205fee0d9e0c7bb409b9c5752ee58fc71db`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🟦️.ts`: `ff68fb6fd5b1b2134448a58951342c6151880bd2e5903ededf6a21c3c8ba8531`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts`: `b5eb6a555e72a98aefc981a2240dafee2ab0b1239c89eb944569e8f1a11d1624`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🟦️.ts`: `128c0276dd1653d4ed8a08cba20a5464ae8c9adbd85da1cb94800abd35413239`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts`: `1838723d52b747677e8eb0c925a1af8595b8aa2b0787f99232a85516d62c63f1`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`: `4b7699c44d0809177226df4acd155ac7167d4eee61824c423d40b281e4aecde7`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts`: `ac196011ea280a9fba0e113a7759251d828b9bfa82efc406352bf96be8cadabe`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts`: `14f23063cdf0cf9c1aef8c4c3c44fa3d1e181de8669748babb5da5fa6529b565`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts`: `6aaef1d88708a526b63cc311a07539f3f0eb1072abfe462e3c26e94c02a5f613`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/root-profile-local-cargo-prototype.log`: `b6c60097cb74771e22b77aa305ad6a78e9e8ac05c3fd541d89d266c22d682204`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/root-profile-local-cargo-release.log`: `29293deedba89dce89882ab42a09a62aab16914c86612fc2afef25ba226fcfa1`.

## Staged Observation DATA Design

The existing fixture author already supplies exact root `VALUE=7` and leaf `value()=11`, no_std and two explicit rlibs. The proposed five-phase corpus can therefore use exact checked text replacements rather than generic Cargo manifest mutation:

| Phase | Physical edit | Observation requirement, not guessed metadata result |
| --- | --- | --- |
| cold | exact existing prototype-local/dependency fixture | complete commands for both crates; actual metadata/extra-filename/incremental paths and current outputs |
| unchanged | no edit | retain actual Cargo fresh JSON; if no rustc invocation occurs, no fabricated repeated command |
| root-body | exact root VALUE7→8 | complete root command and current artifact bytes; observe whether metadata/path changes |
| leaf-body | exact leaf return11→12 | complete leaf and actual affected consumer commands/outputs; observe metadata independently for each |
| context | codegen-units256→128 in BOTH manifest and private `.cargo/config.toml` | complete effective graph and actual flags; no parent-config override ambiguity |

All text replacements require exactly one matching original spelling. Store exact per-phase Source hashes and actual observed environment/argv, not a profile digest standing in for these facts. The independent SQLite closure is authored base→consumer, and Python command parser must account for every `-C` token. Preserve case order because this is a controlled evolution of one real persisted Cargo state, and keep the physical tree until all five observations/final inventory settle. Existing retained prototype-local/dependency complete fixture was62,266 logical bytes (single28,129), well under2MiB; that does not guarantee future phase growth, so every phase still measures the complete current tree and refuses the existing cap. No extra session is deleted merely to manufacture a warm result.

A future neutral context corpus should distinguish four relations explicitly: same canonical DATA in another key order; own Source-only edit; effective context edit; and dependency content-only versus dependency-context edit. Only canonicalization and unchanged full content-key invalidation can be expected before the actual observation; metadata equality/inequality and which fields may be projected out remain pending observations. Closed schema vectors must reject missing compiler/package/target/profile facts, unsupported role/target and arbitrary owned path fields. No production DTO or fixture code has been authored in this read-only phase.

## Observer Source Implementation (Physical Run Held)

Root approved fixture Source/neutral work only. The exact closed5-phase corpus and schema now live under repository profile tests `⚙️arguments/📐️context`. `createCargoContextFixtureV1` delegates to the existing characterized prototype-local/dependency author; `applyCargoContextPhaseV1` preserves a frozen exact5-file roster, checks one occurrence of every replacement and enforces the existing16KiB authored Source bound. Nothing in this owner supplies expected metadata hashes.

A separate test-only settlement helper awaits every exact cleanup and retains the primary failure even when it is undefined. A successful body plus failed cleanup genuinely rejects. It owns no control/store/Original and is invoked only inside the real store/global-writer callback. Neutral laws exercise Error/undefined plus competing cleanup failure and final cleanup completion, with independent SQLite expected precedence.

The physical observer is authored but unexecuted. It uses actual `profile-oracle` registration, CompilerStore, global writer, CPU1 lease and inherited Original control/environment. One real fixture/build root persists through cold, unchanged, root-body, leaf-body and context phases; no sessions are swept to fit. It validates every exact file edit against the actual previous physical bytes, captures actual Cargo unit graph/profile/edges, complete Cargo JSON freshness and full rustc command tokens. Independent Python shlex must agree with the first-party complete `-C` decoder; unique metadata, extra-filename and owned incremental values are recorded without an equality expectation. SQLite verifies affected closure. Rlib bytes are read through the existing no-follow bounded physical reader; the complete perphase rooted no-symlink inventory refuses above2MiB,4096files or8192entries. These observational test limits are not new physical quotas. The enclosing Rust16/floor/writer remains authoritative.

The observer emits bounded phase records and final matrix, charging both structured copies before logging; the outer context-only child capture explicitly returns the full already-declared1MiB/65536line transport instead of the generic64KiB diagnostic tail. Original120/110/10 and every compiler/storage budget remain unchanged. Child failures, cancellation, output-cap refusal and cleanup failure remain fatal; no operation is retried or reborn.

`readCargoProfileOracleSelectionV1` projects exactly the three known selector fields, validates a closed at-most-one selection, and returns all/local/roles/context DATA. Invalid values or conflicting flags refuse before `createScriptProcessEnvelope`. The new physical law registers only for context selection, so existing all/local/roles experiments do not acquire an extra five-phase operation. Existing script command names remain unchanged. Both launch catalogs now have `build-mode-context prototype|release` entries calling the same cargo route with SEMIO_PROFILE_CONTEXT_ONLY=1 and both startup-cache disable fields. No new script/command or production profile/metadata/context/incremental persistence API was added.

### Neutral and Strict Evidence

- Genuine missing-owner RED: compiler-context-neutral-red1.log (exit1, no compiler/image).
- Initial fixture/selectors successor: compiler-context-neutral-prototype1.log (10pass/1fail): independent Ajv test accidentally retained the corpus root required keys while referencing Selector; corrected test reference, not an admission relaxation. Initial strict67766 also caught two test-type expressions, corrected.
- Initial current neutral64316:11/569GREEN. First expanded strict34515 failed only the then-active missing Native ResourceLeaseControl import.
- Genuine missing settlement-helper RED: compiler-context-drain-red1.log (exit1).
- Current helper successor32363:12/578GREEN; full expanded strict23089 exited0 at that Source checkpoint.
- Actual caller/AST successor60206:13/596GREEN; later broad strict85477 caught only the coeditor Native removal of prepareNativePlatformStorageV1 at its old import, preserved and reported to owner.
- Final neutral prototype16030:14/618GREEN1.61s; release67073:14/618GREEN1.84s. Ajv, independent TOML, SQLite, Bun/NodeSHA, TypeScript AST and both editor catalogs are exercised.
- Final expanded strict BOTH awaits the Native owner's coherent removed-prepare caller handback; the earlier green must not be presented as a final Native-transition pass.

All own handles above are terminal. No Rust/tool image opened, no actual Cargo/rustc/Python command observer executed, and no native output/namespace/lease/policy/new runtime dependency changed. The only Python invocation is Source authored for a later explicitly scheduled physical observer. Windows verbose-shell spelling and actual query reuse remain unqualified.

### Exact Next Physical Dispatch Proposal (Not Executed)

After Root separately closes the Native Source epoch, authorizes a reader/writer window and confirms current physical headroom, use the existing editor `build-mode-context prototype` row, equivalent to:

```text
SEMIO_PROFILE_CONTEXT_ONLY=1 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 NODE_DISABLE_COMPILE_CACHE=1 bun <ticket>/build-mode/📜️script.ts cargo prototype
```

The actual command is still cargo under the existing project/Original envelope. It filters only `^pinned Cargo observes direct rlib context`; both other selector flags must be absent. No release/retry/metadata projection/persistence follows automatically. Collect true root/child terminal result plus the complete5-phase matrix before review. Any genuine Source drift, floor refusal, command/parser discrepancy, bound or deadline failure remains a failed checkpoint.

### Observer Source SHA256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🔣️.json`: `6893d09920c6d153fff5841f4cdc923ff6320fbee2fc2d06767710563a585867`; absolute UTF16 path 151.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🦀️.ts`: `238577842071f3369169b5d5f8da55695afbe3487a65f3c7eab4f182ff53a100`; absolute UTF16 path 149.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🟦️.ts`: `6e0aa9d20dc12b9b3b85217d4a8d9dd514905396ed692a8e99bafaf80ddc7f4a`; absolute UTF16 path 149.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts`: `d1a5a268a60191c2598b4a68ae04d0779eaaf374d1c264a70fc56385244bd09c`; absolute UTF16 path 158.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧬️schema/🔣️.json`: `60b746d8e47733ad91965b8954a9b41f647cb8f095941d8025c7c9550f4624c2`; absolute UTF16 path 161.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/🧭️selection/🟦️.ts`: `8bd0f9173e2629d269f88c7807501fc801c6812f98c1227cf8754b5fcdfd58e3`; absolute UTF16 path 151.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/🟦️.ts`: `a5fbe77ab19e8f6edc65033538a7d059d39be0050e3ec489c525c3dc3355be27`; absolute UTF16 path 126.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/🦀️.ts`: `0e29f97d3db46fc0d3788d9ab6815fb2d6e04e3c48f85007fe7045127b12b0cb`; absolute UTF16 path 126.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts`: `1602d5f122d85ce2bcc0fecdf3829fdd547c7172778f25a6574cbb4a55eb0fcf`; absolute UTF16 path 138.
- `generated/compiler-context-neutral-red1.log`: `50b94f53878620d024306846b490b5b64f4277f5c787eee85db2d5867064b8fc`.
- `generated/compiler-context-drain-red1.log`: `ae01cd562756ad844a7cf622f9e67dab09e41fe83acd69b69c8a5fd6dc99f8e1`.
- `generated/compiler-context-neutral-prototype5.log`: `dd78fd7dc880f7a2448d075b6be01fb2b7e8a193b2dcc8c91875aa4396bdcb69`.
- `generated/compiler-context-neutral-release2.log`: `6215afbbc4ed474296185b186d2545194f47df730ecff4dd5485dc549c1f31e3`.

## Final Source Handback

After Storage announced the removed-prepare caller closure coherent, final expanded strict prototype90501 and release86964 both collected exit0 (`compiler-context-types-prototype5.log`, `compiler-context-types-release5.log`). These supersede the temporary missing-import/export failures above without rewriting them. Final neutral remains BOTH14/618. Observer/selector/fixture Source is frozen, every own handle is terminal, and physical execution remains explicitly held for Root scheduling after the Native checkpoint. No actual context result or warm incremental reuse is claimed.

- `generated/compiler-context-types-prototype5.log` SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- `generated/compiler-context-types-release5.log` SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

## Streaming Inventory Successor

The staged observer's eager directory-list allocation was corrected before any physical execution. Final focused BOTH16/641 and expanded strict BOTH0 qualify the bounded opendir census; all actual handles are terminal. See [streaming inventory](context-streaming-inventory-current.md) for the genuine missing-enumerator RED, exact unchanged limits, independent tiny directory oracle and current Source hashes. Physical observer remains held.
