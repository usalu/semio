# Sol Public Directory Projection Boundary

Date: 2026-09-04  
Scope: collaboration repair blueprint, Lane 1 only

## Boundary

The directory wire now uses a breaking discriminated `public | member | author` projection in its paired Rust, TypeScript, and JSON schemas. Public space metadata is constructed directly from the folded space and durable document descriptors. It contains discoverability metadata and document identity only; owner-user identity, caller role, active connections, members, invitations, account data, presence, actors, HLC, event cursor, bootstrap frontier, current frontier, and checkpoint state are structurally absent. `documentId` remains discovery metadata and grants no open authority; MCP rejects a public-only detail and D1 remains the sole document-open authority.

The hub uses one `DirectorySpaceAccessDecisionV1` for list, detail, REST event, generic directory-message, and socket-message paths. Private nonmembers remain `404`. Raw `DirectoryEvent`, connection, presence, and rebootstrap frames are member-only even for a public space. Public event discovery is denied rather than implemented by redacting the raw durable event.

Public descriptor lookup failure is propagated as a typed HTTP failure. It is never converted into an authoritative empty public catalog.

Public visibility is absent from document authority: anonymous/public-nonmember requests cannot read document currentness or use blob `GET`, `HEAD`, or `PUT`. Global directory heartbeats are withheld because their unscoped `headSeq` cannot be attributed to an authorized space; member sockets prove liveness with authorized scoped events instead.

The native and TypeScript directory clients decode the discriminator and fail closed on unknown fields or a discriminator/role/visibility mismatch. The authenticated MCP remote binding accepts only member/author detail and treats a public-only detail as missing membership.

## Neutral contract

`🌎️hub/📇️directory/🧪️fixtures/🧬️public-space-detail-v1/` contains four positive anonymous/public-nonmember/member/author traces plus twenty-one hostile public mutations covering the blueprint keys and literal connection, presence, cursor, bootstrap, checkpoint, and storage-locator fields. Its standalone strict JSON Schema denies the raw identity-bearing `DirectoryEvent` vector. The permanent Bun oracle uses AJV 2020 independently of the Rust and TypeScript decoders.

## Permanent gate

`os-hub:space-public-boundary-check` is owned by the existing hub `📜️script.ts`. It runs the independent hostile fixture oracle, exact-one discovery and exact execution of three real REST/WebSocket laws, the targeted TypeScript narrowing law, and an all-feature hub check. The launch seed uses the canonical repository `bun ./📜️script.ts nx ...` router.

## Current-source evidence

- Session `13878`: `cargo check -p semio-framework-os-kernel --all-features`, exit `0` after the final schema/client cutover.
- Direct strict main-schema AJV compile: `directory-schema-ajv: green`.
- Direct hostile fixture AJV oracle: four positives and twenty-one hostile mutations/forbidden keys, green.
- Session `32272`: final-source all-feature hub production check, exit `0`.
- Corrected registered TypeScript target: one selected `DirectoryClient space public boundary` law passed, 236 skipped, Nx exit `0`.
- Session `93304`: canonical root-router plugin-registry generation exited `0` and regenerated `.vscode/launch.json` from the launch seed.
- Session `38633`: registered plugin-registry generated-source/launch freshness exited `0`.
- The graph owner subsequently regenerated the permission-prefixed `flow_dag` manifest and reported its registered generation, freshness, and 184/184 quick laws green; the next isolated hub retry advanced past that dependency.

## Superseded diagnostics and qualifications

- The first TypeScript run stopped before discovery on a doubled test-config path. The existing OS `📜️script.ts` now resolves the physical single test directory; the rerun passed.
- Hub retries `67265`, `76602`, `9361`, `76350`, and `98422` stopped before owned hub code on successively moving UI-contract, UI/WGPU, plugin, and graph generated-source paths. Those diagnostics were superseded by their owners' current-source repairs.
- Session `8790` advanced past graph and exited `101` before owned hub code on a transient DB preview include; the current source already contains the owner's repaired path.
- Session `21193` advanced past graph and DB and exited `101` before owned hub code because `semio-s-plugin-stdio` still generated an include for the former DXF R12 header taxonomy. The active D0 stdio owner is repairing that generated edge; this is not a public-boundary assertion result.
- Sessions `42434`, `9703`, and `99559` advanced through successive active stdio taxonomy repairs without reaching an owned diagnostic. Their include failures are superseded on current source.
- This report claims no serialized redemption, liveness heartbeat, presence lease, descriptor-bound relay, or compatibility form from blueprint lanes 2–5.

## Final registered result

Session `99284` completed with exit `0` using the exact existing handle, not a restart:

`CARGO_TARGET_DIR='<ticket>/🗑️generated/space-public-boundary-sol-target' bun ./📜️script.ts nx run os-hub:space-public-boundary-check --skip-nx-cache`

The registered terminal confirms the independent AJV oracle (four positives, twenty-one hostile cases, twenty-one forbidden fields), exact-one discovery and execution of all three real REST/socket laws, the selected TypeScript discriminator/unknown-field law, and the final hub `--all-features` check. The earlier session `28170` passed the three Rust laws and TypeScript law but its trailing all-feature check returned `101` with the diagnostic omitted by output truncation. The same-target diagnostic check `20270` exited `0`, followed by the complete registered `99284` pass; the earlier trailing failure remains nonreproduced and is not credited as a pass.

The first route setup failure in session `1860` exposed corrupted native identity `target_os` string literals (`🍎️macos`, `🐧️linux`, `🪟️windows`). The four cfg expressions were restored to Rust's exact platform values without substituting test entropy. The registered kernel library `check --lib` subsequently exited `0`; this is host compile and cfg-validation evidence, not Windows/Linux runtime execution. The separate exact kernel entropy test attempt `75759` failed before its assertion because the infinite-canvas dependency derived an invalid emoji build-script crate name. That dependency repair is tracked separately and does not weaken the public authorization acceptance law.

## Identity dependency follow-up

The existing infinite-canvas Cargo build source was moved, without a duplicate wrapper, to `♾️infinite/📦️packages/🦀️rust/build.rs`; its manifest now uses the standard `build = "build.rs"` convention. The build logic still resolves all inputs relative to `CARGO_MANIFEST_DIR`, so this move does not change code-generation ownership or behavior.

Registered exact entropy-law rerun `55245` passed the former invalid build-script crate-name frontier, then exited `1` (`cargo` status `101`) before the identity test because `semio-framework-compiler` includes the absent `🌍️world/🔤️fonts/🔤️NotoColorEmoji-subset.ttf`; the physical file is `🔤️🟠️NotoColorEmoji-subset.ttf`. This is a separate compiler taxonomy blocker. No direct entropy test pass or cross-platform runtime pass is claimed.

The compiler font include was corrected only after filesystem and `file` inspection confirmed the exact renamed asset is a 152,308-byte TrueType font. A later coordinated taxonomy move restored the physical name `🔤️NotoColorEmoji-subset.ttf` and its source include together; current reread confirms both exist and agree. Current font SHA-256 is `ec69d141e4276776e952ecc10a5bd02f9282c96928de1787c25012fb2a18fb02`. No extra patch was applied to the already-consistent current pair. Registered entropy-law rerun `29204` passed the repaired compiler/infinite source, then ended with exit `1`: the existing runner killed `cargo test` after `1200000ms` (`spawnSync ETIMEDOUT`) before the exact identity assertion. Process snapshots showed CPU-active test-only stdio code generation; the runner's generic lock-contention suggestion is not an established cause. No supplemental runtime pass is claimed. The coordinator requested retention of the warm, ticket-local `space-public-boundary-sol-target` for subsequent work until final ticket cleanup; no sibling generated output was removed.

Targeted owned-file `git diff --check` is clean. Launch freshness session `14664` reported only `.vscode/launch.json` stale; registered generation `44658` and freshness `34875` both exited `0`. That regeneration exposed two concurrent Norm registrations made only in generated output. The canonical authority is `.vscode/🧩️launch.seed.jsonc`, read by the existing registry `🖥️launch.ts` module and emitted through `📜️script.ts generate`. All four Norm targets (mutation-leaf taxonomy, check, config-mutation source, config-mutation test) now have seed-owned entries at group `4_gate`, orders `407.9`, `407.91`, `407.92`, `407.93`; generation `86965` and final freshness `68645` both exited `0`. A direct Bun JSONC check confirms exactly four matching Norm entries and one public-boundary entry in both seed and output.

The shared root runner briefly rejected two generator-contract arrays during concurrent taxonomy movement. The requested reread found the repair already present: `external-step-assets.outputRoots` contains exactly twelve unique paths and `wgpu-frame-worker.inputPatterns` exactly 107 unique patterns, both ordered by the validator's actual JavaScript default `.sort()` comparator. No additional taxonomy mutation was made. Fresh canonical registered plugin-registry freshness session `35060` exits `0`, establishing that the root router accepts those current bytes and generated catalog/launch remain fresh.
