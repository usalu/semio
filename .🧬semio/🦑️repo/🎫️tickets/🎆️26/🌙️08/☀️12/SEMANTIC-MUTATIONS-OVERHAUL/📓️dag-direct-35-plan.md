# Dag Direct 35 — Frozen Direct-Leaf Adoption Plan

## Scope and source inventory

The only production mutation authority is [`DagMutation`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs) in the Infinite directed-DAG component.  It has fourteen concrete operations, an inline `Mutation<DagSnapshot>` implementation without the now-required `DESCRIPTORS` or `descriptor`, and a fourteen-variant `DagMutationDsl` conversion twin used for text and binary operations.  Infinite Rust glue reexports the aggregate and makes no concrete mutation.

Outside OS, the independent plugin-Dag artifact and command/codec roots also name `DagMutation`.  They are a separate taxonomy owner and must be audited at cutover for imports of the Infinite type; their own direct artifact vocabulary is not rewritten by this packet.  The explicit review set is plugin Rust glue plus its DAG artifact's mutation, text/binary codec, editor command, viewer, inference, and test roots.  Generated/cache/ticket and every compose path are excluded.

The future aggregate root is exactly:

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧬️schema/🧬️mutations/🦀️.rs`

It will contain only a transparent fourteen-newtype `DagMutation` aggregate deriving `dsl::Mutations` and `dsl::DslOps`.  Each direct leaf is `🦀️.rs` (not `component.rs`) and owns its payload, `MutationLeaf` metadata, `MutationKind`, `DslRecord` field grammar, `🔣️.json` descriptor, payload schema and leaf-local apply/diff/inverse contribution.  The aggregate owns strict `🔣️.json` envelope schema.  The existing snapshot DSL mirror remains snapshot infrastructure; the aggregate-wide `DagMutationDsl` and its conversion helpers are retired.  No compose path is read or written.

## Frozen direct roster and descriptor contract

Every descriptor uses `schemaVersion: 1`, `diffParticipation: "apply-only"` (current detection remains centralized / unavailable to direct leaves), `outcomeClasses: ["applied"]`, `composition: "atomic"`, and `requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"]`.  `textOpcode` is the semantic kind.  `payloadSchema` is the frozen literal `🧬️schema/🔣️.json`, resolved relative to each leaf.  Tags are unique within this aggregate.  The glyph/path pair is checked against the repository taxonomy and each verb against `APPROVED_VERBS`.

| Leaf owner | Variant / kind | Emoji | Tag | Invertibility | Payload contract |
| --- | --- | --- | ---: | --- | --- |
| `➕️create-node` | `CreateNode` / `create-node` | ➕️ | 0 | explicit-mutation | `node`, `index: u64` |
| `🗑️delete-node` | `DeleteNode` / `delete-node` | 🗑️ | 1 | plan | `id` |
| `✏️rename-node` | `RenameNode` / `rename-node` | ✏️ | 2 | explicit-mutation | `id`, `newId` |
| `🔤️change-node-name` | `ChangeNodeName` / `change-node-name` | 🔤️ | 3 | explicit-mutation | `id`, `newName` |
| `↔️move-node` | `MoveNode` / `move-node` | ↔️ | 4 | explicit-mutation | `id`, `x`, `y` |
| `📐️resize-node` | `ResizeNode` / `resize-node` | 📐️ | 5 | explicit-mutation | `id`, `width`, `height` |
| `🖼️change-node-icon` | `ChangeNodeIcon` / `change-node-icon` | 🖼️ | 6 | explicit-mutation | `id`, `newIcon` |
| `🔡️change-node-abbreviation` | `ChangeNodeAbbreviation` / `change-node-abbreviation` | 🔡 | 7 | explicit-mutation | `id`, `newAbbreviation` |
| `🧮️change-node-operator-kind` | `ChangeNodeOperatorKind` / `change-node-operator-kind` | 🧮️ | 8 | explicit-mutation | `id`, required nullable `newOperatorKind` |
| `🔁️replace-node-kind` | `ReplaceNodeKind` / `replace-node-kind` | 🔁️ | 9 | explicit-mutation | `id`, full tagged `newKind` |
| `🗃️replace-node-properties` | `ReplaceNodeProperties` / `replace-node-properties` | 🗃️ | 10 | explicit-mutation | `id`, strict `newProperties` |
| `🔀️reorder-nodes` | `ReorderNodes` / `reorder-nodes` | 🔀 | 11 | explicit-mutation | `order: string[]`; runtime requires a complete unique permutation |
| `🔗️connect-nodes` | `ConnectNodes` / `connect-nodes` | 🔗 | 12 | explicit-mutation | `id`, `source`, `target`, `routeStyle`, `properties`, `index: u64` |
| `✂️disconnect-nodes` | `DisconnectNodes` / `disconnect-nodes` | ✂️ | 13 | explicit-mutation | `id` |

The node schema is a strict full `DagNodeSpec`: `id`, `name`, `abbreviation`, `icon`, `x`, `y`, `width`, `height`, required-nullable `operatorKind`, strict `properties`, and a strict eleven-case `kind` union (`Computation`, `Slider`, `Select`, `Screen`, `Note`, `Image`, `Preview`, `Action`, `Export`, `Cluster`, `AppInstance`).  `PropertyBag`, ports, media and preview payloads are schema definitions rather than permissive objects.  All schemas are closed (`additionalProperties: false`); required-nullable fields must reject omission and accept literal null where the domain permits it.

Direct Rust payload coordinates are `u64`.  The shared boundary asserts `usize::BITS <= u64::BITS`; apply converts `u64` through `usize::try_from` and returns a structured invalid-index outcome before vector range checks.  Captured native positions are therefore lossless `u64` values on supported native and Wasm targets, without defaulting or truncation.

## Current red and required inverse repair

Store recovery replays `inverse(pre)` **in reverse order**.  The current delete inverse declares `[CreateNode, ConnectNodes…]`; store consequently applies connects before their endpoint node and fails.  It also loses the exact edge order because the existing diff always uses `next.edges.push` and neither connect nor disconnect carries an edge index.

The direct `DeleteNode` inverse must declare in reverse replay order: incident `ConnectNodes` operations from highest captured edge index to lowest, followed by `CreateNode`.  Store reversal therefore creates the node first and restores incident edges in original order.  `DisconnectNodes` likewise captures the removed edge index and returns indexed `ConnectNodes`; `ConnectNodes` applies by bounded insert, not append.  The direct leaf test must demonstrate: node placement, incident-edge restoration, edge order and the actual Store reversed-plan convention.  It must also show a disconnected edge is restored to its original index.  This is a source-law repair, not metadata decoration.

`CreateNode`, `DeleteNode`, and all index-bearing inverse paths require lower/upper index rejection; `ReorderNodes` must verify its supplied IDs are the exact current node permutation; endpoint existence and duplicate IDs retain their existing runtime errors.

## Required neutral matrix and real red oracle

Before production release, the ticket-local controller validates all fourteen proposed descriptor objects against the authoritative descriptor schema and checks: fourteen exact current enum/twin variants, repository-taxonomy glyph/path admission, approved verbs, unique tags, absent canonical aggregate root, strict neutral valid/invalid vector shape, and the source hypothesis of Store-reversed deletion order with absent edge index.  It is not a Rust runtime proof; root's compiler-client and runtime gates remain the actual baseline evidence.

The later language-neutral fixture matrix covers every leaf with: valid JSON payload, unknown-field rejection, required-field omission, text print/parse round trip, binary encode/decode round trip, Rust payload construction, descriptor/schema resolution, and apply/inverse outcome.  It adds operation-specific vectors for nullable `newOperatorKind`, all eleven node kinds, `u32::MAX` and overflow indices, out-of-range index, duplicate IDs, missing endpoints, reorder duplicate/missing/incomplete permutations, delete with no edges / one edge / multiple interleaved edges, and disconnect at first/middle/last edge.  The third-party schema validator is validation evidence only; it remains separate from owned semantic decoding and apply/inverse tests.

## Approved future write set

After root approval only:

- Add the fourteen leaf directories, each with `🦀️.rs`, `🔣️.json` descriptor, `🧬️schema/🔣️.json` payload schema, and leaf-owned tests beneath the canonical Dag mutation root; add aggregate `🦀️.rs` and strict aggregate `🔣️.json` there.
- Update the Dag component to mount/reexport the aggregate and payloads, migrate `DagDiff`, application, inverse, `OpText` and `OpBinary` to wrapped leaf values, then remove inline `DagMutation`, its manual `Mutation` implementation, `DagMutationDsl`, and both conversion helpers.
- Update the Infinite Rust glue reexport list only if public payload construction requires the direct leaves.  It has no concrete current constructor to rewrite.
- Update existing Dag component tests and add ticket-only neutral/oracle evidence.  No ordinary runtime command, host interaction, or snapshot DSL path is rewritten except mutation constructor sites forced by the direct cutover.

Explicit exclusions: every `compose/**` path, Store source, Flow source (including `FlowConfigMutation::SetContributions`), Workflow/Run, TXT, shared trait/derive/registry APIs, Cargo artifacts and root-owned runtime gates.  Current direct-leaf detection is not claimed: `DagMutation::diff` remains the present operation-specific switch, while the shared typed inverse/detection foundation is root-owned and must provide the eventual leaf contribution seam.
