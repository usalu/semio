# Imperative Decorative-Async Residual Repair

## Scope

Owned:

- `✏️s/🔨️modules/📜️imperative/**`
- `✏️s/🔌️plugins/📜️imperative/**`, excluding the coordinator-owned root typed-enum/glue/manifest edits

The repair keeps `ImperativeApps` intact and does not edit Cargo manifests.

## Compiler Ratchet

Command:

```sh
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-plugin-app-imperative cargo check -p semio-s-plugin-imperative --tests --message-format=json
```

| Boundary | Errors | Attribution |
| --- | ---: | --- |
| Baseline | 594 | 232 E0308, 104 E0277, 91 E0609, 84 E0599, 61 E0053, 6 E0600, 5 E0425, 5 E0369, 4 E0271, 1 E0432, 1 E0422 |
| Iteration 2 | 533 | Exact 61-site E0053 trait signature correction |
| Iteration 4 | 382 | Registry/extension manifest seam and core artifact/editor host de-async |
| Iteration 5 | 221 | Mutation, schema, snapshot codec, topology, inference, and diff de-async |
| Iteration 6 | 131 | Command, panel, mode, window, terminology, and manifest helper de-async |
| Iteration 7 | 73 | IO serializers/deserializers and semantic `BuiltNode` render migration |
| Iteration 8 | 59 tests / 23 lib | Remaining diagnostics are framework-owned pure seams and framework testkit futures |
| Iterations 9–12 | 23 → 14 → 1 → 0 lib | Shared label/tree/emit/install seams and synchronous plugin composition root |
| Iterations 13–18 | 35 → 10 → 2 → 0 tests | Genuine-async testkit call-shape restoration, live executor fixtures, and shared descriptor normalization |

Structured evidence is retained in `📝️r21-imperative-*.json`, `*.stderr.txt`, and `*-errors.tsv`.

## Repairs

- Removed decorative async from the imperative module registry, extension SDK, host engine, artifact working-scene helpers, snapshot/text/binary codecs, mutation constructors, topology/inference/diff helpers, editor commands, manifests, panels, modes, window helpers, and all IO leaf serializers/deserializers.
- Replaced async builder/analyzer calls in the derived IO export seam with direct synchronous artifact DSL/pack parsing.
- Made the wasm session surface synchronous because every exported method performs bounded in-memory work.
- Migrated editor and viewer render boundaries from legacy `UiNode` to semantic `BuiltNode` leaves and `ComponentTree` roots through `built_to_component_tree`.
- Preserved typed `ImperativeApps` glue and the synchronous `TopicContribution::{new,decode}` cut.
- Removed stale awaits and ready wrappers from pure imperative code. Retained awaits are contract-required trait boundaries or real store/backbone/dispatch/render suspension in tests and testkit.
- Replaced the executor tests' accidentally empty contributed registry with a deterministic local operator registry so sequencing, branching, and repeat behavior execute instead of halting on the first missing operator.
- Restored imperative testkit wrappers around app construction, typed dispatch, snapshots, rendering, and backbones to genuine async. These operations transitively touch the artifact store and must be polled; making only their outer wrappers synchronous had hidden live futures.

## External Seams

The iteration-8 lib boundary was compiler-exact and consisted of framework-owned pure futures:

- `Emit::{mutations,config}`
- `app_commands!` generated `command_id` and `dispatch`
- `resolve_labels_for_locale`
- `PanelTreeBuilder` and semantic tree-item helpers
- manifest `CommandDefinition::with_args` and `InteractionRef::new`
- framework-internal adapter residue

The test-only delta additionally exposed framework testkit/runtime methods such as app construction, typed dispatch, rendering, snapshots, and the Store constructor. Those are genuine async and are now awaited. Pure `every_command` remains synchronous.

The semantic tree helper now accepts `(ActionId, Option<UiValue>)`; imperative catalogue entries preserve their `addStep { kind }` arguments.

## Gates

### Green

- `cargo check -p semio-s-plugin-imperative --lib --message-format=json`: exit 0, zero diagnostics. Evidence: `📝️r21-imperative-lib-iteration12.json`.
- `cargo check -p semio-s-plugin-imperative --tests --message-format=json`: exit 0, zero diagnostics. Evidence: `📝️r21-imperative-tests-iteration18.json`.
- `CARGO_TARGET_DIR=…/🧪️target-plugin-app-imperative bun nx run @semio-tech/s-imperative:test`: exit 0; 7/7 tests passed. Evidence: `📝️r21-imperative-module-nx-test2.txt`.
- `cargo test -p semio-s-plugin-imperative --no-run --message-format=json`: exit 0, zero diagnostics. Evidence: `📝️r21-imperative-native-no-run.json`.

### Constrained

- The first `bun nx run @semio-tech/imperative-plugin:test` attempt was killed by the repository's 15-second command budget while cold-compiling `semio-s-plugin-stdio`; no test executed. Evidence: `📝️r21-imperative-plugin-nx-test2.txt`.
- A warmed retry still rebuilt the full stdio dependency under nextest-specific flags. It was stopped after the compiler remained clean because the coordinator directed the packet to proceed to P4c rather than wait on the unrelated stdio wall. Evidence: `📝️r21-imperative-plugin-nx-test3.txt`.
- Release, wasm, clippy, and fmt were not claimed: the same full-stdio dependency wall and concurrent disk pressure made those product-wide profiles disproportionate after the focused native compiler/test evidence was green.

### Storage event

The packet-local Cargo cache filled the disk during iteration 16. Only `🧪️target-plugin-app-imperative` was cleared with `cargo clean --target-dir …` (10.4 GiB); all source and diagnostic logs were retained. Subsequent gates used `CARGO_INCREMENTAL=0`.
