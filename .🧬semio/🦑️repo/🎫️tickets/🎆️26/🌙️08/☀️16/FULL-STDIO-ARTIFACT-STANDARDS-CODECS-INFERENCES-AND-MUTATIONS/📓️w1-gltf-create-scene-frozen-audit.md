# W1 glTF `create-scene.v1` Frozen Audit

## Verdict

**NOT ACCEPTED / BLOCKED.** The local leaf now exports the frozen Rust `GltfMutationLeafDescriptor` and has a useful shared JSON vector, but it is not mount-ready in the current tree and does not satisfy strict stale-state/exact-inverse or full OCP parity requirements.

Scope was read-only. No production or test file was edited and Cargo was not run.

All paths are repository-relative. `create-scene/...` below expands to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/...`.

## Blocking findings

### 1. Integrated Rust mount cannot compile the leaf (P0)

All four Rust leaf entry points import the old module name:

- `create-scene/🦠️mutation/🦀️component.rs:3`
- `create-scene/🔺️diff/🦀️component.rs:4`
- `create-scene/↩️inverse/🦀️component.rs:4`
- `create-scene/🦀️component.rs:10` (the descriptor error adapter)

They require `schema::mutations::top_level_collections_private`. The current integrated glue declares the same source file as `pub mod top_level_private` at `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2265-2266`; it has no `top_level_collections_private` alias. The ticket harness masks this by declaring a private alias at `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/src/lib.rs:64-111`, so its historical local Cargo result does not prove integrated mountability. Mounting `create_scene::DESCRIPTOR` therefore stops at unresolved Rust imports before registry/runtime parity can be demonstrated.

### 2. The leaf is absent from the descriptor and wire assemblies (P0)

The common descriptor itself is shaped correctly at `create-scene/🦀️component.rs:8`, but the schema mutation root imports/registers only the two material descriptors at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:3-4,46`. The JSON assembly likewise has only those two members at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json:4-14`. `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2257-2296` mounts the material modules and has no `create_scene` module. Rust dispatch obtains its registry only from this incomplete root (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🦀️component.rs:180-186`), so the command is unknown/unreachable.

TypeScript has no equivalent leaf registry: the current dispatcher is still the closed legacy union, including `NoMutation`, `SetSnapshot`, `Set*`, and generic insert/remove variants at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🟦️component.ts:1-30,52-80`. Thus the TS implementation cannot be mounted through the frozen OCP boundary even though its phase helpers exist.

### 3. Direct diff and inverse are not strict against unrelated stale state (P1)

The forward diff stores only the scene count, default-scene value, and one insertion anchor (`create-scene/🔺️diff/🦀️component.rs:19-29`, mirrored in `🔺️diff/🟦️component.ts:8`; checks at Rust `:51-59` and TS `:23-25`). The inverse stores only the post-count, inserted scene, one post-insertion anchor, and default-scene values (`create-scene/↩️inverse/🦀️component.rs:19-30`, mirrored in `↩️inverse/🟦️component.ts:8`; checks at Rust `:71-89` and TS `:29-36`). A different scene outside the immediate anchor can therefore change without rejection.

Read-only Bun probes confirmed this behavior: with three scenes and insertion at position 1, changing a distant post-state scene to `{nodes:[99]}` still produced `diffAccepted: true`; applying the inverse to that changed post-state produced `inverseAccepted: true` and returned the changed scene instead of the exact base. This violates the frozen requirement that direct planning reject stale bases and inverse reconstruction restore/reject exactly. The sole vector exercises only position 0 and the immediate anchor (`create-scene/🧪️contract/🔣️component.json:5-44`; assertions at `🟦️component.ts:72-95` and `🦀️component.rs:113-158`), so it does not detect this case.

### 4. Reference repair and collection mutation still cross a generic seam (P1)

The three phases delegate semantic mutation to the all-family helper rather than owning the scene repair algebra: mutation calls `repair`/`Change::Insert` (`🦠️mutation/🦀️component.rs:3,31-35`), diff calls `repair`/`Change::Insert` (`🔺️diff/🦀️component.rs:4,69-74`), and inverse calls generic `scenes_op` (`↩️inverse/🦀️component.rs:4,93-98`). TypeScript similarly calls shared generic `insert`/`remove` (`🦠️mutation/🟦️component.ts:3,25-31`, `🔺️diff/🟦️component.ts:3,30-35`, `↩️inverse/🟦️component.ts:3,39-46`).

That helper is a generic family switch over all top-level collections and retains a whole-collection `family_diff` path (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️top-level-collections-private/🦀️component.rs:8-17`, TS `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️top-level-collections-private/🟦️component.ts:4-45`). This leaves command-owned reference repair and direct sparse planning behind a generic/aggregate seam, contrary to the OCP boundary even though `create-scene` itself does not call `family_diff`.

### 5. JSON Schema phase identities collide and numeric facets are not fully parity-safe (P1)

All three JSON Schema documents use the same `$id` (`s.stdio.gltf.mutation.create-scene.v1`) at:

- `create-scene/🦠️mutation/🔣️component.json:3`
- `create-scene/🔺️diff/🔣️component.json:3`
- `create-scene/↩️inverse/🔣️component.json:3`

The phase documents therefore collide when assembled. Existing phase schemas use distinct `.mutation`, `.diff`, and `.inverse` schema IDs while preserving the same canonical command ID in `x-semio`; `create-scene` does not.

The physical facets also widen/narrow the index/version domain differently: Rust uses `usize` for positions and `u32` for typed phase versions (`🦠️mutation/🦀️component.rs:12-14`, `🔺️diff/🦀️component.rs:19-29`, `↩️inverse/🦀️component.rs:19-30`), TypeScript uses unrestricted `number`, JSON Schema uses unbounded JSON `integer`, GraphQL uses signed 32-bit `Int` (`🦠️mutation/🔗️component.graphql:1`, phase GraphQL files), and Proto uses unsigned 32-bit `uint32` (`🦠️mutation/🛰️component.proto:3`, phase Proto files). This is not exact Rust/TS/JSON/GraphQL/Proto parity at the schema boundary.

## Passing local evidence

- `bun .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/verify_w1_a_gltf_create_scene.mjs` passed its narrow static/common-descriptor, de-duplicated GraphQL/Proto declaration, and TypeScript canonical-vector checks.
- The canonical vector is imported by TS (`🧪️contract/🟦️component.ts:3`) and deserialized by Rust (`🧪️contract/🦀️component.rs:84`), and both exercise descriptor planning/application, malformed payload, range/default-reference, forged-path, replay/anchor stale, inverse stale, serialization, and canonical ID laws.
- The descriptor wrapper recomputes paths in both application phases (`create-scene/🦀️component.rs:41-49`) and the typed phases reject forged paths (`🔺️diff/🦀️component.rs:60-62`, `↩️inverse/🦀️component.rs:75-76`, with TS mirrors).

These are leaf-local signals only; they do not clear the integrated mount, OCP, strict stale/exact-inverse, or schema parity blockers above.

## Required exit gates

1. Make the helper module name used by the leaf identical to the integrated glue export, then mount `create_scene` in the Rust root/glue and replace the legacy TS dispatch path with the open descriptor registry.
2. Make stale preconditions and inverse state exhaustive enough to reject unrelated post-state edits (or carry and validate the authoritative pre/post scene sequence), and add append/no-default/distant-stale vectors in the one shared JSON fixture.
3. Remove the generic/whole-collection semantic seam from the leaf path; keep any reusable mechanics behind a non-semantic private kernel with command-owned validation and path algebra.
4. Give each phase JSON Schema a unique phase `$id` while preserving the one canonical command ID, and choose matching index/version domains across Rust, TS, JSON Schema, GraphQL, and Proto.
5. Re-run the integrated Rust + TS + schema/GraphQL/Proto + generic text/binary registry gates after mounting. The ticket-local harness and Bun verifier are insufficient acceptance evidence.
