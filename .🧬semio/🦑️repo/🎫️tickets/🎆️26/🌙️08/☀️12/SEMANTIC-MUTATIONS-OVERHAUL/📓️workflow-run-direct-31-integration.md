# Workflow Run Direct 31 Integration

## Cutover

`RunMutation` is mounted from `workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs` and reexports `StartRun`, `StartRunNode`, `FinishRunNode`, `AppendRunLog`, and `SealRun` through the workflow component. The aggregate derives both `dsl::Mutations` and `dsl::DslOps`.

The workflow component no longer defines an inline `RunMutation`, a manual `Mutation<RunArtifact>` implementation, `RunMutationDsl`, or conversion helpers. Its public text and binary codecs operate on the aggregate's generated `DslVariants` directly.

`RunDiff::apply` and `apply_run_operation` now construct and match the five wrapped canonical variants. Existing sealed-run rejection remains at `apply_run_operation_checked`.

## Consumers

The workflow tests, runner component/runtime tests, and runner binary use only the five canonical wrapped variants. The OS host has no `RunMutation` constructor or match site.

## Validation

The scoped absence scan found no old Run variants, private DSL twin, conversion helper, or manual aggregate mutation implementation in the workflow or runner source. `git diff --check` passed for the cutover files. The retained neutral Ajv oracle command passed five valid and five invalid payload vectors:

```text
[DEBUG] Workflow run neutral schemas passed valid=5 invalid=5
```

No Cargo command was run; the shared compiler gate is owned by the root lane.

## Workflow Direct Cutover

The Workflow aggregate is mounted from `workflow/🧬️schema/🧬️mutations/🦀️.rs`. Its eighteen direct owners contain the leaf payload, source-validated descriptor, strict JSON payload schema, forward diff, inverse, semantic identity, and DSL field grammar. The aggregate derives `dsl::Mutations` and `dsl::DslOps`; the workflow component reexports every payload and uses the generated `DslVariants` directly for text and binary operations.

The historical inline enum, manual `Mutation<WorkflowSnapshot>` implementation, `WorkflowMutationDsl`, and conversion helpers are absent. `RenameNode`, `ChangeParameter`, `UpdateNodePorts`, and `AddInput` replace the former non-canonical names without aliases. Existing `WorkflowDiff` remains the application infrastructure, but maps only to wrapped canonical leaf variants.

The Workflow component and the OS host's public editing calls, codec tests, and flow-fixture operation construction all use the wrapped direct leaves. A scoped scan found no old names or direct twin. `git diff --check` passed.

The retained scoped Bun/Nx oracle compiles each schema with Ajv 2020 and accepted 18 representative Workflow payloads while rejecting 18 corresponding unknown-field payloads. This schema oracle is separate from the pending root-owned Rust compiler/runtime gate.

## Source Checkpoint

- Workflow aggregate + 18 leaves: `workflow/🧬️schema/🧬️mutations/🦀️.rs` and its direct leaf `🦀️.rs` files.
- Run aggregate + 5 leaves: `workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs` and its direct leaf `🦀️.rs` files.
- Mounted consumers: `workflow/🦀️component.rs`, `os/🖥️host/🦀️component.rs`, `os/🔨️modules/🏃️run/🦀️component.rs`, and `os/🔨️modules/🏃️run/📦️bin.rs`.

```text
a1a97e7e43ac8f117db4379b133488803d2f9e01aaf8fad260f359d7e095b8081  workflow/🦀️component.rs
53e1044cceec42907c6e741230279aa439b806d6bb232c4ef592580c3cd90211  os/🖥️host/🦀️component.rs
f5b9a7a5040bd1785b6894756f624bbc9d9c70c8452bbe78a6b57c007faf5fd0  os/🔨️modules/🏃️run/🦀️component.rs
9777aa71af1509725a3daece9acc6906093fc9c2a3068e789103af11a69bec22  os/🔨️modules/🏃️run/📦️bin.rs
7c0ff96c191927a190584d4bb758d97536306b154cd8b424233c6aa41a202ea1  workflow mutations aggregate
77b59d87f1ebc293022002c07c7c1afe5cdd1d1ac4584cbc7cc13d79cb92667e  run mutations aggregate
```

The complete leaf hash inventory was captured immediately before the root-owned framework build; all 18 Workflow and five Run leaf sources were included.

## Compiler-Repair Checkpoint

`RunSink::record` is now async and every real runner, binary, and converted runtime-test caller awaits it. The helper remains `async fn fresh_sink() -> RunSink`; the four sealed-run regression paths await `record` before deriving their prior records.

The retained scoped Bun/Nx oracle now validates all 23 actual leaf descriptors against the authoritative Draft 07 `library/🔣️mutation-descriptor.schema.json`, then additionally verifies the exact canonical owner, leaf-owned payload schema locator, and text-kind identity. It passed:

```text
[DEBUG] Run direct neutral fixture accepted descriptors=5 codecs=5
[DEBUG] Run payload parity accepted leafPositive=6 leafNegative=19
[DEBUG] Current flattened aggregate is missing and closed-leaf refs accepted=0/5
[DEBUG] Proposed shared-payload aggregate accepted=5 rejected=22
[DEBUG] Authoritative descriptor contract accepted leaves=23
```

The current flattened-aggregate line is retained as an identified schema integration gap; it is not evidence of Rust or semantic runtime success. `git diff --check` passed for the scoped Workflow, Run, host, and ticket-oracle paths. No Cargo command was run.

```text
65d641bd834896dd39af248dbf29df44404a014966b3aa6368bf3f6a7294b4ba  workflow/mutations/↔️move-node/🦀️.rs
e4209c61870c770ccfda1699001876c727eeeceee4e75e380aab53bfbff58166  workflow/mutations/⛔️unbind-output/🦀️.rs
7ca19ad35bd338a47dd47d960a8163c8d64a36c0d8a079769207ee2c0c03e256  workflow/mutations/✂️disconnect-edge/🦀️.rs
fbe2685018792ec265b04ff9269e3b2a16ea877acc4f4931bf250185ef11d318  workflow/mutations/✏️rename-node/🦀️.rs
171f0f6cad84ce830a781f5d7d040711a776cbe1347fd77d2cee9873a39d8570  workflow/mutations/➕️add-node/🦀️.rs
65c33e2a9d259f8bf2f5b37e3470f5ea791cd80271ada021cef15119fbe32a05  workflow/mutations/📤bind-output/🦀️.rs
e07cbbc7db19cbffcf3f1151052f833a31d0e24febbe0b4d4085ff116b9de8c1  workflow/mutations/📥add-input/🦀️.rs
17005c9f5c06a0ca02dbe0e14ed8cc266653fe5dd77b0a0657ca4ecfd382609c  workflow/mutations/🔄update-node-ports/🦀️.rs
41abd8ad90a4c358209044296ea3240faf9fcb2a13370ffd6c88ab55fefc2eb9  workflow/mutations/🔌bind-input/🦀️.rs
aca06ef55b4b3f1af4310d745d18c3f40722ab09a201ca2c3b18beeefb69d1d9  workflow/mutations/🔒bind-parameter-field/🦀️.rs
02c45f3b96d432d6abb43d5d5b80259bc211ab47cd2d971fce6a879c0e913e5a  workflow/mutations/🔓unbind-parameter-field/🦀️.rs
29e329a2e93f24ed03ae163b3367102e2e294c66dff0a6a00c2a132a566b5c90  workflow/mutations/🔗connect-ports/🦀️.rs
cb305339aa1168a73f2d3b2bfa1f2671160c62239e042c26b9673b92aaee7f2b  workflow/mutations/🗑️remove-node/🦀️.rs
6bc564eab8b94bdc2337709de46bf3b3769dbfb7c5cc521acd2869711bf104ad  workflow/mutations/🚪unbind-input/🦀️.rs
fe7642a33bb8e900dfb49b624048a1b3dea0de64614ccffd2ecfe207904a8ad6  workflow/mutations/🚮remove-input/🦀️.rs
e90fc0e21fd022e08d16f32d63dc049bd40ed1f01cd986aa42b27ac2bafbd113  workflow/mutations/🦀️.rs
38625bec4c28019abb494c3c8683a54cf4f357565594427bddb11e674bf6eb09  workflow/mutations/🧩add-parameter/🦀️.rs
e8b7cbad15a3b0e95a7879b36ee006b36fadae701394ec6f2f43fa566a790db0  workflow/mutations/🧹remove-parameter/🦀️.rs
ed2b58ec3a5f050a65c92657c20653395f26f5e01edbf7128654566ff23bc0e1  workflow/mutations/🩹change-parameter/🦀️.rs
38e1367f5614eba42011899b1bee1063852c391329bef73285eb014bc815680d  run/mutations/▶️start-run-node/🦀️.rs
b33c8b801767c00dd5c1dd616c9bd2365127ef62086464928d69942c7b7d792e  run/mutations/✅️finish-run-node/🦀️.rs
c468284a2acc6582f59d85ef944251b83cf17f637a6c539702ce8d93768a8d03  run/mutations/🔏️seal-run/🦀️.rs
167602e9698cd9506edf7bf049809830f4f3e445c25c4b3ced65ef16e2b178a0  run/mutations/🚀️start-run/🦀️.rs
e5d0fa3f8f3ca06ad9507c1ffb5358b76c07d3c0c623a3b7b40e0fcb1af585b8  run/mutations/🦀️.rs
0e521d634210f7e5663e9f51621ada2b48029f1ea3c7c2c33dd122e2a0b099ae  run/mutations/🪵️append-run-log/🦀️.rs
1a97e7e43ac8f117db4379b133488803d2f9e01aaf8fad260f359d7e095b8081  workflow/🦀️component.rs
cb90da8698278d518539c55f62cfa9f7cb46704bd6422b08f2137a5fb4ef80eb  runner/🦀️component.rs
181296029c0942a9c7da6c0062304c45a6a2031fa65b8aca5c0c09ae5e0877ea  runner/📦️bin.rs
53e1044cceec42907c6e741230279aa439b806d6bb232c4ef592580c3cd90211  os-host/🦀️component.rs
e3caa73a077b85366139bc87ccef371cb7d74117fa5ec81c33165b092b435df8  ticket oracle/📜️script.ts
```
