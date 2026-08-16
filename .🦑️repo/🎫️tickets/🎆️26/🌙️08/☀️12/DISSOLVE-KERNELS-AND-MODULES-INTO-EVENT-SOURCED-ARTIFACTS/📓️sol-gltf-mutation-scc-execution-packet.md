# GlTF Mutation SCC Execution Packet

## Current Scoped Result

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

The command completed in report mode with 27 components, 64 errors, and 0 warnings.

The five former standards/subset findings are absent. The 64 errors are exactly:

| Count | Problem | Owner |
| ---: | --- | --- |
| 1 | `manifest-child-missing` | `🧊️gltf` artifact root |
| 56 | 28 paired `manifest-child-missing` and `member-component-leaf-missing` findings | command roots under `🧬️schema/🧬️mutations` |
| 3 | `manifest-child-missing` | retired schema `💾️binary`, `📝️text`, and `🧭️planning` roots |
| 4 | `collection-authored-behavior` | mutation-root Rust, TypeScript, GraphQL, and Protocol contracts |

Thus the requested mutation structural group is 59 errors (`56 + 3`). Its four aggregate-root behavior errors are inseparable from the same source SCC and must move in the same lease.

## Boundary and Current Graph

Let `G` be:

```text
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any
```

The source SCC is `G/🧬️schema/🧬️mutations` plus its direct consumers. Every one of the 28 command payloads imports both the closed `GltfMutation` union and shared planning/validation code; the aggregate union dispatches every command. Splitting commands into concurrent source leases would therefore require stale aliases or a cyclic intermediate API and is forbidden.

The 28 terminal mutations are:

```text
🚫️no-mutation             📄set-snapshot          🏷️set-asset
➕️insert-scene             ➖️remove-scene          ✏️set-scene
➕️insert-node              ➖️remove-node           ✏️set-node
🔄️transform-node           🌳️reparent-node         🔗️bind-node-mesh
➕️insert-mesh              ➖️remove-mesh           ✏️set-mesh
➕️insert-accessor          ➖️remove-accessor       ✏️set-accessor
➕️insert-material          ➖️remove-material       ✏️set-material
🔗️bind-primitive-material  ➕️insert-buffer         ➖️remove-buffer
✏️set-buffer                ➕️insert-animation      ➖️remove-animation
✏️set-animation
```

Each presently owns exactly these six source leaves, all of which move together into its immediate canonical leaves:

```text
<mutation>/🦠️mutation/🦀️component.rs
<mutation>/🦠️mutation/🟦️component.ts
<mutation>/🔺️diff/🦀️component.rs
<mutation>/🔺️diff/🟦️component.ts
<mutation>/↩️inverse/🦀️component.rs
<mutation>/↩️inverse/🟦️component.ts
```

The merged `<mutation>/{🦀️component.rs,🟦️component.ts}` keeps its event, diff, inverse, and tests as private regions. It is one event-sourced command/state transition, so no module is introduced for its facets.

## Semantic Destination

Create one qualified subset-owned module:

```text
G/🔨️modules/🧭️mutation-dispatch
semantic id: s.stdio.gltf.module.mutation-dispatch
```

It owns the closed `GltfMutation` union, command dispatch, planning, reference validation, rejection/application/derivation contracts, and the aggregate Rust/TypeScript/GraphQL/JSON-Schema/Protocol facets. Its direct production consumers are the 28 independent terminal mutations above, so it exceeds the two-consumer minimum and its LCA is exactly `G`.

Move into it:

```text
G/🧬️schema/🧬️mutations/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}
G/🧬️schema/🧬️mutations/🧭️planning/🦀️component.rs
```

The old mutation root becomes only `🔣️component.json`, a canonical `x-semio` list manifest declaring the 28 command children. It has no authored language leaf and no forwarding export.

`G/🔨️modules/🔣️component.json` gains the `mutation-dispatch` member manifest. Its module declaration names all 28 terminal mutation IDs, not the intermediary collection or test consumers.

## Mutation Transport I/O Prerequisite

The following are executable `OpText`/`OpBinary` transport boundaries, not mutations:

```text
G/🧬️schema/🧬️mutations/📝️text
G/🧬️schema/🧬️mutations/💾️binary
```

Move them to specific I/O transport components:

```text
G/🚪️io/🧬️mutations/📝️text
G/🚪️io/🧬️mutations/💾️binary
```

Both are one frozen-tag transport protocol, whose encode/decode trait methods are inseparable in Rust. Their manifests declare `format` (`text` or `binary`) and `direction: transport`; no codec runtime code remains under `🧬️schema/🧬️mutations`.

Before the source move, Sol must make the schema-first taxonomy accept this collection:

- replace the singular inference-only I/O collection path rule with declared I/O semantic collections including `🚪️io/🧬️mutations`;
- extend the owned I/O direction enum with `transport` for inseparable bidirectional protocol traits;
- add `🚪️io/🧬️mutations` to `semanticCollections` as kind `io`;
- move the two `artifactSpecFilenames` entries from the old schema mutation paths to the new I/O paths; and
- add taxonomy fixture coverage for text and binary mutation transport manifests.

This is a central taxonomy/discovery lease only; it creates no exception or compatibility path.

## Direct Consumer and Generator Surface

| Path | Required atomic update |
| --- | --- |
| `G/🧬️schema/🦀️component.rs` lines 78–82 | Point `FacetLeaves.mutations` to `../🔨️modules/🧭️mutation-dispatch` aggregate facets. |
| `G/🧬️schema/🦀️component.rs` builder region | Import `GltfMutation` and call `apply_gltf_mutation` from `schema::modules::mutation_dispatch`. |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs` | Re-export `GltfMutation` from `schema::modules::mutation_dispatch`; point Ops/Spr language grammar and protocol rows to the new I/O transport components. |
| `G/🚪️io/🦀️component.rs` test regions 879–947 | Point grammar/protocol conformance tests and `demo_mutation_cases()` to their final module/I/O owners. |
| `G/🧬️schema/🔺️diff/🦀️component.rs` | Update documented source links when the owned transport/dispatch paths move; retain its value codecs. |
| `📜️script.ts` line 7389 | Replace the exact old glTF mutation aggregate leaf in the source/generator verification list with the dispatch module leaf. Sol-only hot-file change. |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` lines 2247–2521 | Final registrar update; Sol-only. |

The only exact external path referrers are the root script and the stdio Rust glue. The remaining direct runtime consumers above use symbol imports, not hard-coded emoji paths. There are no TypeScript package-barrel, lockfile, or Cargo manifest changes for this SCC.

## Central Registrar Sequence

Do not modify the central registrar until the source owner supplies a hash-stable request containing all new leaves and consumer updates.

1. In the current `pub mod mutations` block (lines 2247–2510), remove the root `mod component; pub use component::*`, `planning`, old `binary`/`text`, and all 84 nested facet mounts.
2. Retain `mutations` solely as mechanical registration of the 28 direct command `component.rs` leaves—one direct mount per command, no re-export.
3. In the `pub mod modules` block (lines 2512–2521), add exactly one `mutation_dispatch` mount for `G/🔨️modules/🧭️mutation-dispatch/🦀️component.rs`.
4. In the `pub mod io` block beginning at line 2523, add one mount per final text and binary mutation transport component. Their names and resolved paths must match the I/O manifest lease exactly.
5. Apply the root `📜️script.ts` generator-input update in the same central sublease. Re-run the source-path check and reject stale root/facet mounts before release.

## Execution Order

1. Sol completes the taxonomy/discovery I/O-transport prerequisite and its fixtures; no glTF source move yet.
2. One Terra lease owns all 28 commands, the aggregate dispatch module, mutation root/module manifests, the two I/O transport components, and the direct consumer files in the table. It uses no compatibility aliases and hands Sol the final mount/referrer request.
3. Sol performs the central glue and root-script registrar sublease above after rehashing the source/referrer set.
4. Regenerate through the established Nx surface, then run the stdio quick test, scoped taxonomy report and enforce, and representative mutation event-stream/text/binary round trips. Any budget termination is recorded as unresolved rather than passing.
5. Rerun the deterministic scoped census/report only after registrar completion; the expected remaining scoped finding is then the independent artifact-root manifest lease, not a mutation finding.
