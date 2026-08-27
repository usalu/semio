# Retained Publication and Startup Authority Packet

## Startup Defect and Exact Fix

The live constructor checked every proof against the generic factory sentinel before registering real app factories. Custom factories were statically catalogued but trapped at activation. Registration now precedes proof validation. Each custom proof carries compiler-derived TypeId and type_name through a typed macro witness; activation joins that witness to the exact live app owner, controller/tool key, document schema, registered payload schema, execution contract, and ActionBus registration. A custom proof never constructs the generic fallback. Missing, wrong, copied, and unregistered witnesses remain denied.

The remaining generic rows belong to Forms and Procedural3d, whose owner hooks do not register custom factories. Existing local sentinel-named Block5d/Puzzle3d/Puzzle5d types were renamed to concrete domain factory names. VCS, Sequence, and Process3d catalogs were split according to their real per-tool registrations. Writer and Puzzle3d/Puzzle5d stale bounded contracts were corrected to the registered resumable contracts. Flow's equivalent split was owned by the Flow executor.

## Verification Status

The coordinator reported the canonical Nx source selftest clean with 33 production proof owners, 254 custom rows, 31 generic rows, and 645 checks. Subsequent additions contribute 28 independent Ajv exact-authority/runtime-source mutation checks and three associated-constant hostile cases; those additions need the next canonical rerun. The coordinator then executed the exact native canonical-edit sealer tests: seven passed, and retained-member tests: three passed. These member tests cover direct ordered publication and prepublication reservation/abort, not atomic mounted groups. Native plugin and CAD startup gates remain queued/running; no Wasm startup pass is claimed.

The full-corpus source census remains red; it is not implied by the focused source scan. Wires and Imperative had genuinely incorrect proof document-schema literals, now corrected against their artifact constants. Jack's literal was correct: its artifact constant delegates to an inherent Snapshot::SCHEMA. The resolver now follows that exact imported module and same-source inherent implementation; wrong snapshot owner, missing implementation, and wrong associated field remain rejected.

The broader canonical filter exposed an old test's missing exact retirement owners, then a production cold-load seed bug. The harness now installs its owners and closes both stores. Cold causal seeding formerly applied `?` to a duplicate edit/operation identity, dropping its partially populated DAG. It now preflights exact unique capacity/identity bytes before creating a DAG and transfers every duplicate's returned String owner into the Store's bounded displaced retirement queue. Reset reserves all displaced-owner capacity before creating its new populated DAG. Strict causal duplicate and Drop guards remain unchanged. The coordinator's canonical rerun passed18/18, including the repaired load/reset regression and new runtime-seed test (843filtered,1.88seconds; `🧪coordinator-store-canonical-native-r4-2026-08-27.txt`).

The new Rust startup fixture tests missing/wrong TypeId and type name, a different factory with the same concrete owner, wrong controller/document/tool/payload schema/contract, absent registration, different live bus, generic sentinel over an app registration, duplicate rows, and the preserved unregistered generic path. Hostile substitutions and the copied-owner assertion execute inside cfg(test) helpers in the authority-owning module; production proof fields/registration-map/validator visibility remain private. The real CAD smoke constructs EditorApp<CadPlayApp> from create_cad_app, checks all 25 exact factory rows including generated setActiveUtility, then closes with one item/4096 bytes. The CAD route fixture and strict schema carry these activation assertions.

The root source scan resolves local, imported, qualified, and explicitly aliased compiler type references against taxonomy module declarations. It validates the real factory implementation, owner, TOOL_IDS and registration site, including Note's delegated registration. The cross-file strict fixture carries missing/wrong witness and same-owner/different-factory adversaries. The scan also marks Child publication unavailable, matching the runtime gate.

## Canonical Test Routes

Kernel Check/Test now await runCargo and forward exact arguments. The plugin SDK now has its own awaited package router and Nx project. CAD Test awaits runCargoTestBudgeted after resolveTestLevel. The shared nextest helper now partitions supported runtime selection flags away from build arguments and forwards them to actual metadata execution; previously `-E` only reached binaries-only listing. Five command-vector fixtures, four malformed inputs, strict Ajv validation, and an independent Node argument parser cover this change. An optional SEMIO_TEST_ARTIFACT_DIR retains generated metadata beneath the caller's task directory without hardcoding a ticket; blank/unset preserves the previous default. The coordinator reported the expanded focused Nx gate passed two tests/25assertions,282filtered (8.79seconds). Its first invocation discovered package-script inference overriding the explicit project target and recursively calling Nx; package `nx.includedScripts:[]` now preserves the explicit script router, and that corrected actual graph was verified before the passing run.

Launch entries401–409 are now in the authoritative `.vscode/🧩️launch.seed.jsonc`, including CAD artifact-directory forwarding. Direct edits to generated launch.json were being overwritten by registry generation. Three canonical attempts found the project with targets:{} despite its existing 📋project declaration. The canonical command then passed with `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false`: it refreshed59plugin crates,60playgrounds,38framework packages and generated launch.json. Read-only jq projections and diff verified that all nine complete generated configuration objects exactly match the seed, including the CAD environment. Exact attempts and outcomes are retained in `retained-launch-generation-attempt.txt`; no Nx infrastructure source/configuration was changed.

Plugin native r4 compiled the exact-proof test, then its synthetic AppDefinition failed the production classification guard before executing the join cases. The fixture now explicitly classifies its deliberately monolithic/watchdog/malformed-view declarations BatchOnlyPendingRewrite. Only the selected registered factory row is promoted inside the exact authority test. This is not a production admission relaxation; plugin r5 and real CAD activation remain pending coordinator execution.

- `bun x nx run @semio-tech/framework-os-kernel:test --args=canonical_`
- `bun x nx run @semio-tech/framework-os-kernel:test --args=retained_member_`
- `bun x nx run @semio-tech/framework-plugin:test --args=retained_factory_proof_`
- `bun x nx run @semio-tech/cad-plugin:test --args='-- retained_factory_proofs_activate_'`
- `bun x nx run @semio-tech/repo-lib:test --args='-t "nextest execution filters"'`

## Exact Shared Files Changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`: member-owned wire/typed preparation, exact erased authority, real group reservation/abort, tests.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️member-publication.json`: ordered wire/stale/cancel/maximum-grant fixture.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`: exact startup proof join; Child failclosed and incremental retirement; runtime tests.
- Plugin siblings `🧪️tool-factory-proof.json` and `🧬️tool-factory-proof.schema.json`.
- Plugin retained-command `🧪️fixtures/🔣️owner-factory-resolution.json` and `🧬️schema/🔣️owner-factory-resolution.schema.json`.
- `📜️script.ts`: compiler witness parser/resolver, all-app scan, fixture/hostile checks; peer sealer sections preserved.
- Kernel `📦️packages/🦀️rust/📜️script.ts` and `📋️project.json`.
- Plugin `📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` (new).
- `.vscode/launch.json`.
- `.vscode/🧩️launch.seed.jsonc`: authoritative focused gate registrations.
- Puzzle `📦️packages/🟦️typescript/📜️script.ts`: renamed factory source oracle.
- CAD `🧪️retained-jobs/🔣️component.json` and `🔣️schema.json`: activation fixture/schema.
- CAD `📦️packages/🦀️rust/📜️script.ts`: awaited test-level/filter forwarding.
- Store `🧪️runtime-seed.json`: duplicate identity and bounded-retirement fixture.
- Repo library `📦️packages/🟦️typescript/📦️index.ts`, `🧪️index.test.ts`, and `🧪️tests/🦀️nextest/{🔣️schema.json,🔣️command-vectors.json}`: faithful nextest execution selection.
- Repo library `📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}` and `🧪️tests/🦀️nextest/🔣️artifact-location.json`: canonical routing and explicit task artifact retention.
- VCS `🌿️vcs/🦀️component.rs` and `🧪️group-history{,.schema}.json`: private shared decision/provisional fixed-slot history visibility.
- Store `🧪️group-cursor{,.schema}.json` and host `🖥️host/🦀️component.rs` two constructor sites: conditional immutable cursor owners.

## Exact App Owner Sources Changed

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

Other edits in these shared files belong to their respective executors and are not claimed as part of this packet.
