# Norm Artifact Package Extraction

## Scope

This packet extracts the 15 norm artifacts into independently addressable Rust crates while keeping implementation sources in the domain taxonomy. Framework workflow/run is explicitly outside this packet and remains owned by the coordinator.

## Architecture

The package-neutral lower boundary is `semio-s-artifact-norm-contract` at `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust`. Its `lib.path` mounts `📇️registry/🧬️contract/🦀️.rs`, which owns:

- common norm document/compliance APIs and `impl_norm_artifact_record!`;
- config schema and mutations;
- common editor/viewer app-surface APIs and `norm_owned_tool_job_factory!`;
- artifact-definition assembly;
- a language-neutral package declaration parser and semantic validator.

The contract has framework/pack dependencies only. It has no norm artifact, norm plugin, or stdio dependency.

Each artifact owns its existing taxonomy source root, its editor/viewer implementation modules, its artifact-local `🧬️schema/📜️artifact-definition.json`, and a Cargo declaration under its local `📦️packages/🦀️rust`. Package names are `semio-s-artifact-norm-<id>`. Artifact definitions validate their local package schema before assembly.

The norm plugin package is now a composition facade: its package root mounts `norm/🦀️.rs`, while the plugin source imports the 15 leaf crates directly to assemble the closed `NormApps` enum, definitions, editors, viewers, activations, and capability requests. It does not expose a public `artifacts::<id>` compatibility namespace.

## Dependency graph

- `din4108`, `din16798`, and `vdi3805` depend only on the shared contract and framework support.
- `din18599` depends on `din16798`, `din4108`, and stdio `semio`.
- `en1990` depends on stdio `semio`.
- `en1991` depends on `en1990`.
- `en1992` has an optional direct dependency on the `semio-s-artifact-fem-2d` leaf.
- `en1993` depends on `en1992` and has an optional direct dependency on the `semio-s-artifact-fem-2d` leaf.
- `en1994` depends on `en1993`.
- `en1995` through `en1999` form the observed standard chain and also use `en1990` where their sources require it.
- `iso16757` depends on `en1999`.

The plugin `cross-fem` feature propagates into the two FEM-owning leaves.

The two cross-domain helpers now import the 2D artifact's public `elements2d` and `model` APIs directly. Their optional feature graph contains no dependency on the owning FEM plugin and constructs the leaf's closed `Elements::BeamEb2` variant explicitly.

## Schema test

The contract has positive and negative language-neutral JSON fixtures. Its Rust test parses the valid fixture with the first-party pack codec and independently with `serde_json`, compares canonical identity fields, and verifies that the first-party semantic validator rejects a syntactically valid invalid fixture.

All 15 artifact schemas were parsed independently with Bun and verified against the identity rules:

- `id = s.norm.<artifact>`;
- `rust_package = semio-s-artifact-norm-<artifact>`;
- `nx_project = @semio-tech/norm-<artifact>-rs`.

## External consumer audit

No Rust source outside the norm plugin imports the removed `semio_s_plugin_norm::{artifacts,editor,viewer,document,config,app_surface}` APIs. The framework DSL fixture-sweep package still declares a norm plugin dependency for integration coverage; it does not use the removed umbrella source API.

## Validation

Passing:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-s-artifact-norm-contract --lib
```

Result: 18 passed, 0 failed. This covers the package schema oracle and the mounted shared compliance, config, and app-surface behavior.

```text
cargo metadata --no-deps --format-version 1
```

This resolves all 16 norm package manifests and the intended leaf-to-leaf dependency graph.

```text
bun -e <artifact-schema identity audit> <each of 15 artifact schemas>
```

This validates all 15 language-neutral package declarations.

```text
cargo tree -p semio-s-artifact-norm-contract --prefix none
```

This confirms the lower contract has no norm plugin, norm leaf, or stdio back-edge.

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-s-artifact-norm-din16798 --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-s-artifact-norm-vdi3805 --lib --message-format short
```

Both independently compiled leaves pass. DIN 16798 exercises a standalone package boundary; VDI 3805 exercises the macro-heavy source tree that requires conservative Nx inputs.

`cargo fmt --package ... -- --check` parsed the contract and representative small/composed/macro-heavy leaves, then reported pre-existing repository formatting differences. It was used as a parse audit rather than recorded as a passing format gate.

The stdio owner subsequently repaired that boundary. A fresh `cargo tree -p semio-s-artifact-norm-en1990 --prefix none` now shows only `semio-s-artifact-stdio-semio` and `semio-s-artifact-stdio-contract`, with zero unrelated stdio artifact leaves. The final full norm wildcard and plugin test commands are queued after the shared OS-host build and listed in `📓️framework-space-artifact-extraction.md`.

Both optional FEM dependency trees resolve directly to `semio-s-artifact-fem-2d`, and static source/manifests contain no `semio-s-plugin-fem`, `fem::core`, or `dep:fem` reference:

```text
cargo tree -p semio-s-artifact-norm-en1992 --features cross-fem --prefix none
cargo tree -p semio-s-artifact-norm-en1993 --features cross-fem --prefix none
```

The first complete leaf compile also found that the extracted ISO 16757 root re-exported `Iso16757Snapshot` through both its canonical document-schema route and its generated standards route. The redundant standards-route re-export was removed; the canonical root API remains unchanged.

The corrected complete leaf gate passes with both FEM integrations enabled:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check \
  -p 'semio-s-artifact-norm-din*' \
  -p 'semio-s-artifact-norm-en*' \
  -p semio-s-artifact-norm-iso16757 \
  -p semio-s-artifact-norm-vdi3805 \
  --features semio-s-artifact-norm-en1992/cross-fem,semio-s-artifact-norm-en1993/cross-fem \
  --lib --keep-going --message-format short
```

Result: all 15 norm leaves pass. EN 1992 and EN 1993 compile against `semio-s-artifact-fem-2d`; ISO 16757 compiles with its single canonical snapshot export. Existing lint warnings remain outside the extraction boundary.

The final thin-plugin runtime gate was started once with incremental output disabled:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-s-plugin-norm --lib --message-format short
```

The original attached session was `67307` with Cargo PID `22032`. After the execution turn finalized, the session became unavailable (`Unknown process id`) and the PID disappeared without a durable raw log. No pass/fail result is claimed from that run. The explicit config and surface integration targets registered afterward are the authoritative replacement gates.

## Integration handoff

Nx declarations are owned by the Nx execution packet. Native source inputs must explicitly include the lower contract taxonomy sources because they are outside the contract package directory. VDI 3805 must conservatively include its complete artifact owner glob because Rust source discovery cannot see every macro-generated module.

## Related finding

Before the stdio owner fix, `semio-s-artifact-stdio-semio` unconditionally depended on all other 35 stdio leaves, causing `en1990` and its downstream norm chain to pull the full stdio catalog. This was reported to the coordinator and reassigned to the stdio owner because it directly defeats the selective-compilation goal.

## Coordinator Integration Target Audit

The final read-only audit found two missing explicit Cargo integration targets still referenced by persistent script routes. The coordinator mounted the existing config mutation test under the shared Norm contract package and routed that command directly to the contract. The full thirty-app surface-render integration test remains a parent plugin composition test and is now explicitly mounted from its taxonomy source, with its existing third-party oracle dependencies declared only for tests. Both target paths resolve on disk. Native execution is pending. The remaining seven-line parent package facade was removed; Cargo now mounts the taxonomy root directly, where its unchanged plugin export macro resides.

The two independent source/oracle commands completed with exit 0: config checked 5 cases, 5 hostile payloads, 4 undeclared forms, 13 text vectors and 25 binary vectors; surface checked 15 variants, 30 apps, 120 bodies and 5 hostile vectors with AJV. These are source/oracle checks; the new native integration targets still require execution.

Additional coordinator-owned files:
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`

A focused scan of literal Rust include paths in the shared Norm config and app-surface taxonomy found 0 unresolved references.

Before native execution, the mounted integration surfaces were audited against the current graph. The config test imports only `semio-s-artifact-norm-contract` plus the framework kernel traits, and its semantic kind, text opcode, and binary tag match the owned change-selected-check-index descriptor. The surface test imports the public framework testkit and the fifteen artifact crates directly; every editor/viewer type path and plugin app/artifact identity resolves from the current package roots. No Rust source or manifest below Norm refers to the removed parent artifact/config/app-surface API or to `semio-s-plugin-fem`. Both integration sources pass `rustfmt --check`; the surface test import layout was normalized during this audit.

The three authoritative native commands run sequentially with `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`, a unique daemon-free Nx workspace-data directory per gate, and the one ticket Cargo target: contract `config-mutation-test`, plugin `test -- --lib`, then plugin `surface-render-test`.

The first gate completed successfully:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-config-2 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo bun x nx run @semio-tech/norm-plugin:config-mutation-test --output-style=stream
```

Result: Nx passed and nextest reported 1 passed, 0 failed.

The first correctly formed plugin library gate, session `51981`, compiled all Norm sources and the parent plugin binary. Its post-build nextest metadata step then failed because a concurrently added glTF test manifest referred to the nonexistent workspace package `semio-framework-value`. The stdio artifact owner replaced those imports with the framework protocol traits, removed that manifest edge, and verified locked offline metadata for the 36-package catalog. This was an external graph failure; it did not report a Norm diagnostic.

The warm retry used the same acceptance shape with a new isolated Nx graph:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-lib-3 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo bun x nx run @semio-tech/norm-plugin:test --output-style=stream -- --lib
```

Session `49067` exited 1 after 6m03s. The retry reached the current shared `semio-framework-plugin` source and failed on the concurrent `ConfigView::window` integration plus missing thread-safety bounds in the new erased window config ownership path. It again reported no Norm source diagnostic. The framework owner repaired that shared compilation boundary.

The authoritative warm plugin retry then completed successfully:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-lib-4 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo bun x nx run @semio-tech/norm-plugin:test --output-style=stream -- --lib
```

Result: session `58231` exited 0 after 18m20s. Nx passed, and nextest reported 16 passed, 0 failed across the parent plugin library binary. This compiled the current framework plugin, all fifteen directly composed Norm packages, and the selective semio codec package.

The first surface-render integration gate was session `83800`:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-surface-1 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo bun x nx run @semio-tech/norm-plugin:surface-render-test --output-style=stream
```

It exited 1 after 14m04s with both native cases failing. The failures identified two actual integration defects: fixture lane ids use the canonical lower-case vocabulary while the test projected Rust `Debug` variant names, and the default DIN 4108 document JSON exceeded the UI component's 512-byte text capacity. The panic on that over-capacity insertion also exposed the intentionally strict disposer during unwind.

The shared lower app-surface now publishes `publication_lane_id`, the single canonical mapping used by both integration tests. Document JSON is rendered as a bounded column of exact UTF-8 chunks, each no larger than `UI_TEXT_MAX_BYTES`, rather than being truncated or rejected. A focused contract regression renders a document containing 512 multibyte `ä` characters, retires the component tree, parses every text child, and proves their reassembly equals the complete source JSON. The independent source/AJV oracle was rerun after the change and passed its 15 variants, 30 apps, 120 bodies, and 5 hostile vectors.

The first focused native regression was session `93063`:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-contract-chunk-1 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo bun x nx run @semio-tech/norm-artifact-contract-rs:test --output-style=stream -- long_unicode_document_text_is_admitted_in_exact_utf8_chunks
```

It exited 1 after 37m11s with the one selected test failing `duplicate-key`: every chunked text node carried the UI builder's default identity. The chunk builder now assigns the stable unique id `norm-text-chunk-{index}` before each node is built. The source/AJV oracle completed again after this repair with the same 15/30/120/5 counts.

The warm focused retry completed successfully:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-data/norm-contract-chunk-2 \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo \
bun x nx run @semio-tech/norm-artifact-contract-rs:test --output-style=stream -- \
  long_unicode_document_text_is_admitted_in_exact_utf8_chunks
```

Result: session `86471` exited 0 after 107m02s including the shared target queue. Nx passed; Nextest ran the one selected law with 1 passed, 0 failed, and 19 skipped. Durable log: `🗑️generated/norm-contract-chunk-test-2.txt`.

The earlier surface retry lost its observable controller during the execution-slot handoff and was replaced by a fresh attached route. That attached session reached Cargo after 66m42s, then exited 1 before compilation could finish because the shared disk filled while Cargo wrote the Norm integration-test fingerprint. It emitted no Rust diagnostic and ran no surface test. Durable environmental-failure log: `🗑️generated/norm-surface-render-test-3.txt`.

The final warmed surface-render integration retry compiled and executed both cases, then exited with 0 passed and 2 failed. At the default test-thread stack the public-surface case overflowed its stack, while the retained-cohort case first reached the strict artifact-store `Drop` witness and then aborted during cursor disposal on unwind. Durable log: `🗑️generated/norm-surface-render-test-4.txt`. This result is a failure, not an active or accepted gate.

The retained test binary was then executed directly with `RUST_MIN_STACK=67108864` to remove only the diagnostic obstruction. It reached the actual first public-surface error in four seconds: DIN 4108's report rendered sibling rows with duplicate default keys, and the projection rejected the tree with `duplicate-key`. The subsequent strict disposer panic was secondary unwind evidence. Durable first-cause log: `🗑️generated/norm-surface-retained-binary-stack-witness.txt`.

`render_report` now assigns every check row the stable identity `norm-report-check-{index}`. The surface test drives every created app through the shared `close_registered_fixture_app` completion helper and defers any rendered-surface panic until after the app reaches its exact terminal-empty witness. The router supplies the measured 64 MiB test-thread stack for this target, leaving the production runtime unchanged. File-scoped rustfmt, TypeScript transpilation, and diff-check pass.

The current ordinary Nx acceptance retry uses a fresh isolated graph and the shared ticket Cargo target:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-norm-surface-final \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo \
bun x nx run @semio-tech/norm-plugin:surface-render-test
```

Session `90355` exited 1 after 4m37s before Norm compiled. The current shared `semio-framework-os-kernel` snapshot produced seven `E0277` diagnostics because a concurrent GIS change awaited `u8` values. The GIS owner repaired that shared prerequisite after this receipt. No Norm result is inferred from this external compile failure. Durable log: `🗑️generated/norm-surface-render-test-final.txt`.

The duplicate-row repair now has a direct regression in the app-surface unit suite. It constructs two value-identical checks, renders and exactly retires the report tree, parses the projection through `serde_json`, and asserts the stable sibling keys `norm-report-check-0` and `norm-report-check-1`. This closes the one-row coverage gap that allowed the original default-key collision.

A fresh surface retry and the focused two-row unit law remain required against the repaired shared kernel.

The current document-contract oracle was also queued through its ordinary Nx target after the canonical schema-import repairs. Session `51731` exited 1 during project-graph construction, before the oracle ran: root `📜️script.ts` imports the currently absent replication map test module `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧪️tests/🟦️.ts`, and the graph plugin separately reported the absent source project `npm:@asamuzakjp/css-color`. This is no Norm result. Durable log: `🗑️generated/norm-document-contract-current.txt`. A retry remains required after those external graph prerequisites settle.

After the root graph imports were repaired, session `78611` proved graph construction succeeded but used the retired project name `abstraction-ownership-validation`; Nx correctly reported that the project does not exist, so no oracle result is inferred. Durable invocation-error log: `🗑️generated/norm-document-contract-final.txt`.

The corrected ordinary route completed successfully:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_NO_CLOUD=true \
NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-workspace-norm-document-contract-final-2 \
bun x nx run workspace:norm-document-contract --output-style=stream --skip-nx-cache
```

Result: session `79144` exited 0. The independent oracle matched EN 1990's 20 native snapshots and 6 committed diffs, DIN 18599's 26 native snapshots and 12 committed diffs, and both domains' independent owner/child rejection vectors. Nx completed the target in 7.3 seconds. Durable log: `🗑️generated/norm-document-contract-final-2.txt`.

## Taxonomy router recovery

The current Norm source inventory exposed 23 moved fixture modules whose `#[path]` registrations still skipped the committed `🧪️tests` directory. The exact repair updates six aggregate fixture routers: EN 1991 (2 registrations), EN 1994 (3), EN 1995 (2), EN 1997 (3), EN 1998 (9), and EN 1999 (4). Every registration now resolves to its existing case source; no fixture body or expectation changed.

A post-repair read-only census parsed all literal `#[path]`, `include_str!`, and `include_bytes!` references below the Norm source root. It resolved 7,312 references with zero missing targets. The durable receipt is `🗑️generated/norm-rust-literal-path-census-after-repair.txt`; the pre-repair failure inventory is retained in `🗑️generated/norm-rust-path-census-before-repair.txt`. Exact-file rustfmt and scoped diff checks passed for the six routers.

The independent surface inventory also passes on the current tree: 15 variants, 30 apps, 120 declared bodies, AJV validation, and five hostile vectors. Receipt: `🗑️generated/norm-surface-render-source-final-2.txt`. Native surface execution and the focused two-row report identity law remain pending the serialized Cargo queue; no native pass is inferred from these source checks.
