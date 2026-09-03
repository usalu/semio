# Stdio Catalog-Root Completion Audit

Read-only source audit, 2026-09-03. No build or test was run. `target/`, dev cache, and a prior fingerprint are deliberately not treated as deployment or completion evidence.

## Decision and first blockers

`stdio` is the zero-dependency root of the generated catalog. Its generated row has no descriptor-derived identity or hashes, and its owner root has neither `🔣️.json` nor `🛂️.descriptor.semio`. This is the first deterministic blocker: a strict catalog cannot establish the root package's immutable component/descriptor identity, so none of its 33 direct dependents (or the 59-node closure) is safe to call complete.

The present Rust `describe` producer is not sufficient to cure that absence: when called with only the component it sets `core_wasm_sha256` equal to the raw component hash because it has no core input. The strict verifier hashes an actual extracted core module and compares it to that field. Thus a pair emitted by the current `stdio:describe` alone would be structurally well-formed but cannot be strict-catalog evidence once a real core is staged.

There is a third, separate authority blocker after the corrected pair exists: the hub's `linked_native_codec_bindings()` returns an empty vector. `stdio` contains many concrete `ArtifactCodec::of` call sites, but they are not an explicit, descriptor-bound static receipt; its declarative capability ledger explicitly reports zero registered codecs. Do not manufacture bindings from app names, file extensions, or a dynamically populated global codec registry.

| Order | Severity | Deterministic condition | Why it blocks |
| --- | --- | --- | --- |
| 1 | Critical | Owner root `✏️s/🔌️plugins/🗄️stdio` lacks both descriptor forms; generated row has no `hashes`. | No immutable `(pluginId, packageId, component bytes, core bytes, descriptor bytes)` association exists for the root. |
| 2 | Critical | Rust `describe_component` writes `core_wasm_sha256 = wasm_sha256` when passed only the component. | Strict catalog completion compares this claim to a separately extracted core module, so this is an identity false-positive rather than a valid core receipt. |
| 3 | Critical | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:199-201` supplies no `NativeCodecBinding`. | A selected package with a native codec cannot satisfy trusted-catalog loading; a package without one remains unusable for authoritative pack/spr operations. |
| 4 | High | Schema ledger says `6` codecs declared / `0` executable-registered; Rust source has concrete codecs outside that authority declaration. | The proposed static factory cannot honestly infer which source implementations are approved document codecs. |
| 5 | High | No current command constructs the required fresh-root `raw/core/descriptor` triplet for `catalog-complete`. | The strict verifier correctly rejects ambient `target/`/dev-cache residue; it cannot prove a release candidate yet. |
| 6 | Medium | Normal registry `check` still warns for descriptor absence. | It is intentionally transitional, so it is not a release gate. Keep `catalog-complete` fail-closed until all roots are migrated, then retire the warning exception. |
| 7 | Medium | `serde` and `serde_json` are explicitly interim dependencies in `Cargo.toml`. | This is a WASI/source-risk to resolve only if the isolated build reports it; it is not evidence of a current compiler error. |

## Live census and source ownership

| Subject | Current source-backed result | Evidence |
| --- | --- | --- |
| Cargo/component identity | Cargo package `semio-s-plugin-stdio`, `cdylib` + `rlib`, component metadata `semio:stdio`, role `plugin`. `plugin-root` is default so only this component emits the installer; dependent crates must use it without default features. | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:1-29,44` |
| Registry row | `pluginId=stdio`, package name `semio-s-plugin-stdio`, wasm output `semio_s_plugin_stdio.wasm`, no dependencies and no hashes/capabilities/contributions. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json:1312-1319` |
| Closure | 33 direct generated dependents: `animate`, `architect`, `block`, `cad`, `dag`, `demonstrator`, `draw`, `energy`, `fem`, `flow`, `flow-extension-brep`, `forms`, `gis`, `imperative`, `layout`, `lowpoly`, `mathematical`, `norm`, `note`, `playbook`, `procedural`, `process`, `puzzle`, `raster`, `reasoning-mindmap`, `remodel`, `s`, `sequence`, `shooting`, `sourcing`, `trinity`, `vcs`, `writer`. The broader generated graph has 59 rows. | Generated registry dependency census; source ordering is derived in `…/📇️registry/📜️script.ts:2346-2360`. |
| Root export | The Rust package includes the root source and emits `plugin_exports!(plugin, plugin::StdioApps)` under the default root feature. | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:123-140` |
| Schema artifacts | 36 checked-in artifact-definition JSON files and exactly 36 `include_str!` entries. `validate_catalog` requires 36, unique identifiers/MIME/extensions/runtime capabilities and closed dependencies. All 36 referenced paths exist at audit time. | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:210-262,533-585` |
| Root assembly | All 36 source roots have a factory; 26 call `runtime_assembly`, while 10 intentionally remain `definition_only_assembly`. A definition-only root must not be upgraded to runtime merely to fill a catalog row. | `…/📇️registry/🦀️.rs:681-734`; type boundary at `589-620` |
| Source applications | `StdioApps` contains 176 enum variants, exactly 88 editors and 88 viewers. Those are source app declarations, not descriptor or server-open-plan authority while the descriptor pair is absent. | `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:9-188` |
| Capabilities | The JSON source corpus declares 6 codecs, 3 mutations and 67 inferences; only the 3 mutations and 67 inferences set `executable_registration`. The built-in test encodes the same honest ledger and says implemented/verified are zero. | `…/📇️registry/🦀️.rs:184-205,805-841` |
| Existing Rust codecs | There are concrete `ArtifactCodec::of`/`document_codec_bare` and legacy registration paths across stdio, including e.g. glTF, CSV, DXF and text. They are useful factory candidates but are not a generated trust receipt. | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs:31-43,56`; `…/📊️csv/🦀️.rs:45-53,65`; `…/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs:61,188` |

The schema registry binds executable identities only for glTF's registered inference services and mutations; it is not a blanket proof for codec factories (`…/📇️registry/🦀️.rs:310-338,614-620`). The API deliberately distinguishes `Runtime(ArtifactDeclaration)` from schema-only `Definition`, and its test rejects runtime capability rows on a definition-only artifact (`…:589-620,807-815`). That is the correct safety boundary for static linkage generation.

## Descriptor, component, core, and hash path

The owned `describe` target establishes the raw component descriptor, but is **not** the strict owner-root producer today:

1. `@semio-tech/stdio-plugin:describe` calls `describePluginComponent(repoRoot, "semio-s-plugin-stdio", ownerRoot, true)` (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:33-42`).
2. It runs `cargo rustc -p … --lib --crate-type cdylib --target wasm32-wasip2`, finds the debug component at `target/wasm32-wasip2/debug/semio_s_plugin_stdio.wasm`, then invokes the Rust descriptor emitter (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts:50-75`).
3. The emitter writes the canonical packed descriptor and readable JSON directly to the owner root, not a generated directory (`…:56-66`). It correctly hashes raw bytes and canonical descriptor bytes, but its documented one-file fallback writes `core_wasm_sha256 = wasm_sha256` (`…/📇️describe/📦️packages/🦀️rust/🦀️.rs:300-329`). That fallback must never be committed as a strict `stdio` completion pair.

The normal dev materializer demonstrates the correct two-file calculation: it consumes a cargo component, uses JCO to extract core WASM, probes the component's `describe` export through Node JSPI, and patches SHA-256 over the exact raw/core bytes before writing the owner pair (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:941-1010`). Its browser output is nevertheless cache/output state, not a durable bundle (`…:1001-1008`). Reuse that algorithm in an isolated producer; do not promote its old output. Ship-only `wasm-opt` touches extracted core modules (`…/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:500-537`); hash the exact core staged to the fresh root, never a later cache selection.

`catalog-complete` already enforces the desired contract: bounded artifact size 64 MiB, canonical JSON/pack agreement, exactly three descriptor hashes, canonical pack bytes, a self-hash, explicit fresh build root, streamed SHA-256 with cancellation, and raw/core/descriptor equality checks (`…/📇️registry/📜️script.ts:2008-2015,2268-2284,2380-2424,2430-2467`). It deliberately refuses ambient output. The missing piece is a producer for its one-row fresh-root layout:

```text
<fresh-root>/stdio/raw/semio_s_plugin_stdio.wasm
<fresh-root>/stdio/core/semio_s_plugin_stdio.wasm
<fresh-root>/stdio/descriptor/🛂️.descriptor.semio
```

The root pair does not exist now, so the generated row's absent hashes are correct rather than stale generated data to hand-edit. Existing target fingerprints and `target/wasm32-wasip2/wasm-release/deps/semio_s_plugin_stdio.d` only show historical dependency bookkeeping (the latter is timestamped 2026-09-02T23:53:59+0200); no current `output` diagnostic was found. In particular, all current `📄️txt` schema include paths exist. A past missing-path report is therefore stale until a clean isolated compile reproduces it.

## Bounded implementation packet

### A. Make `stdio` a verified source identity first

1. In the existing stdio Rust package and descriptor-emitter tests, add a neutral fixture for one root component and assert: descriptor JSON and packed form decode to the same value; `manifest.pluginId=stdio`; package ID is the identifier returned by the component; raw/core SHA-256 are lower-case 64-hex and non-zero; **the core digest is calculated from supplied extracted core bytes, not assigned from raw bytes**; descriptor self-hash is canonical; every artifact identity comes from `artifact_assemblies`; duplicate artifact/schema/package identities fail.
2. Extend the existing descriptor-emitter API and its `📜️script.ts` caller rather than adding a new script file: accept an explicit core-WASM input (or a prevalidated extracted-core byte stream) and reject catalog publication without it. Compute the three hashes once, then serialize the final canonical pair atomically. The existing one-component `describe` mode may remain a non-release diagnostic, but cannot write a strict owner pair.
3. Add one `catalog-root` operation to the existing stdio, registry, or dev `📜️script.ts`. It must construct into a caller-supplied absolute empty directory, use a dedicated `CARGO_TARGET_DIR`, build just stdio once, extract the core from that exact raw component, invoke the new two-file describe path, and atomically rename the completed `stdio` directory. Do not copy from `target/` or `plugin-modules` after the fact.
4. Apply a 1,200,000 ms hard build deadline already defined by `buildBudgetMs()` (`…/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1229-1235`), one cargo compilation at a time, a maximum materialize concurrency of four only if processing several independent components (`…/🧑️‍💻️dev/…/📜️script.ts:1063-1079`), 64 MiB per staged artifact, 64 KiB hash chunks, cancellation checked between chunks, and atomic cleanup of the temporary root. Treat deadline/cancel/oversize/component-export failure as no publication.
5. On success, commit only the emitted owner-root `✏️s/🔌️plugins/🗄️stdio/🔣️.json` and `🛂️.descriptor.semio`; generate the registry afterward. Never commit `target/`, JCO output, a dev cache, or the fresh verification root.

### B. Resolve native-codec authority without pretending every format is executable

1. Add a code-generated Rust `stdio` native-factory receipt sourced from a single explicit list of real `ArtifactCodec::of` factory functions. Each row must contain `pluginId`, emitted `packageId`, artifact kind, exact document schema, non-zero `pack_schema_hash`, extension, read/write/mutation/inference capability IDs, and the static factory function. It must be generated from/checked against the descriptor plus Rust declaration, not inferred from `StdioApps`, MIME, extension, or `register_document_codec` side effects.
2. Require an exact bijection: each descriptor-declared native codec has one receipt and one binding; no receipt exists for a descriptor-absent or `Definition`-only capability; duplicates and mismatched schema/hash/package fail before global registration. Existing trusted loading independently checks the four-part binding key and rejects duplicates (`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:517-529`) and rejects undeclared bindings (`…:316-323`).
3. Replace the hub's empty `linked_native_codec_bindings()` only with bindings made from that generated receipt. `NativeCodecBinding` is deliberately a tuple of plugin, package, artifact kind and `ArtifactCodec` (`…/trusted-catalog/🦀️.rs:109-122`); loader verification requires the descriptor/trust-record identity match (`…:532-560`). Keep glTF inference/mutation service identity separate from document codec identity.

### C. Close catalog evidence, then unlock the dependent wave

1. Run the strict source/fresh-artifact verifier for stdio before any dependent is called complete. It must report the expected raw/core/descriptor SHA-256 receipt and no unverified node.
2. Regenerate `plugins.json` from the committed pair; use strict `catalog-complete` as the release condition. The ordinary `check` warning at `…/📇️registry/📜️script.ts:1944-1963,2543-2554` must not be accepted as a green.
3. Build direct dependents only in their graph order after stdio's pair and factory receipt are accepted. First wave: `animate`, `architect`, `block`, `cad`, `dag`, `demonstrator`, `draw`, `energy`, `fem`, `flow`, `forms`, `gis`, `imperative`, `layout`, `lowpoly`, `mathematical`, `norm`, `note`, `playbook`, `procedural`, `process`, `puzzle`, `raster`, `reasoning-mindmap`, `remodel`, `s`, `sequence`, `shooting`, `sourcing`, `trinity`, `vcs`, `writer`; place `flow-extension-brep` only after `flow`. Recompute the generated topological order rather than preserving this human list if other edges change.
4. Only then compose the hub trusted catalog/startup and P2-C reader. A descriptor pair demonstrates an immutable component; it does not by itself solve the separate 64 MiB authority pair versus 496 KiB DB blob ceiling or infer a usable native codec.

## Required test and oracle packet

| Test | Independent oracle / assertion |
| --- | --- |
| Descriptor emission | Decode canonical packed bytes using the independent Pack implementation; compare JSON structural value and recompute SHA-256 with Node/WebCrypto or Rust `sha2`, not the emitter helper. |
| Component/core | Use a second WASM component parser/tool in test-only tooling to inspect the emitted component and extracted core; assert the required actor exports and that raw/core hashes match the owner pair and fresh-root copies. |
| Source registry | Fixture with 36 artifacts, then one missing/duplicate include identity, one duplicate MIME, and one definition-only runtime capability. Assert `validate_catalog`/assembly fails and does not emit a receipt. |
| Static factory | For one real stdio codec and one definition-only artifact, load a neutral trusted bundle. Independent oracle verifies real pack/spr compile-print-apply round-trip for the former; the latter must have no binding. Tamper package ID, component SHA, schema hash, descriptor self-hash, or a duplicate binding and require no global codec registration. |
| Fresh-root/cancel | Mutate an input during streamed hashing, supply an oversize artifact, cancellation file, missing core, and a stale raw component. All fail closed and leave no final fresh-root node. |
| Closure | Independent topological-sort oracle starts at stdio, proves all direct dependents follow it, and proves no cache file can satisfy `catalog-complete`. |

Focused commands for the implementation owner (not executed by this audit):

```sh
# Current raw-only diagnostic; it is not strict-catalog evidence until packet A supplies core bytes.
CARGO_TARGET_DIR=/absolute/disposable/stdio-target bun nx run @semio-tech/stdio-plugin:describe

# Focused source and descriptor/factory tests once added.
bun nx run @semio-tech/stdio-plugin:test -- quick
bun nx run @semio-tech/plugin-registry:test

# Re-render only after the owner pair is committed.
bun nx run @semio-tech/plugin-registry:generate
bun nx run @semio-tech/plugin-registry:check

# After packet A adds catalog-root to an existing script, it must create this root with raw+actual core.
bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root /absolute/fresh-catalog-root
```

## Exit criteria

`stdio` is complete only when a clean isolated build has produced and committed the owner descriptor pair with the actual extracted-core hash; the generated row exactly carries its three hashes; a non-cache fresh root verifies raw/core/descriptor bytes; the static receipt has an exact, tested relation to every descriptor-declared native document codec; trusted hub startup consumes that receipt; and the topologically dependent catalog wave has fresh, independently checked evidence. The present tree meets none of those completion assertions for `stdio`; no build result is claimed here.
