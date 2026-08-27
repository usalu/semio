# Sequence Direct-Leaf Cutover

Date: 2026-08-27. Lane: TERRA-ROOT-SEQUENCE-DIRECT-01. Baseline: d03b1fdb6da7c4ea97043e5618d8f4098a43dff7.

## Scope and Result

Exact production root: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`.

Eight direct owners now have completed descriptors, direct payload and wire schemas, TypeScript, GraphQL, and protobuf counterparts. The root is a transparent aggregate and direct detector registry. Schema has no text/binary root; its existing IO-owned codec surface was preserved. Every direct leaf retains its MutationKind/SEMANTICS and optional diff/inverse facets. No compatibility module or nested payload owner remains.

| Direct leaf | Variant | Snapshot detection | Inverse |
|---|---|---|---|
| 🌱create-step | CreateStep | detect | explicit-mutation |
| 🗑️delete-step | DeleteStep | detect | plan |
| 📍move-step | MoveStep | detect | explicit-mutation |
| 🔧edit-step-params | EditStepParams | detect | explicit-mutation |
| 🗂️change-step-collapsed | ChangeStepCollapsed | detect | explicit-mutation |
| 🔗connect-steps | ConnectSteps | detect | explicit-mutation |
| ✂️disconnect-steps | DisconnectSteps | detect | explicit-mutation |
| 🧬duplicate-step | DuplicateStep | apply-only | explicit-mutation |

## Test-First and Ownership Changes

- The pre-cutover policy baseline was 10 findings: eight direct-owner, root-purity, and hidden-generation. The ordered language-neutral detector fixture was added before leaf detection implementation.
- Seven direct leaves own detection contributions; duplicate-step is explicitly apply-only because a snapshot cannot distinguish duplication from creation. Generic operations only index scenes, collect contributions, and order them by leaf-supplied (phase, source index, within-item priority). Before-index construction preserves first-match behavior for repeated IDs.
- The committed detector fixture proves delete-before-edit ordering; per-step move/params/collapse ordering; insertion; deletion-cascade suppression; disconnection; retarget disconnect-before-connect; and no-change as an empty plan. The original planner's retarget behavior is preserved, including its existing semantics around deleted old endpoints.
- Sixteen single-mutation inverse/outcome/absorb law tests moved from the former root into their direct owners. Cross-kind laws, store bridges, case bridges, and catalog correspondence remain in sibling schema operations. KINDS is assembled from leaf SEMANTICS rather than duplicated string identities.
- Catalog scenario directory names now resolve to the actual committed fixture directories. All eight catalog vectors retain mutation/before/after/outcome and Rust test files.

## Exact Write Set

At every direct directory in the roster above, the current direct file set is:

```text
🦀️component.rs
🔣️component.json
🔣️payload.schema.json
🔣️wire.schema.json
🟦️component.ts
🔗️component.graphql
🛰️component.proto
```

This is 56 direct files. Rust and TypeScript were moved from each old `🦠️mutation/` payload directory; those 16 old file paths and eight now-empty directories were removed. Each leaf's `🔺️diff/🦀️component.rs` and `↩️inverse/🦀️component.rs` was updated to the direct type/builder route (16 facet files).

Aggregate/shared/consumer files:

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️component.graphql`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️component.proto`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔣️component.json`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs` (direct mounts plus schema operations)
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️component.rs` (one direct create_step path only)

No root script, shared API, taxonomy, STDIO, other plugin, AGENTS, or Git state mutation was made. Real compose paths were never accessed.

## Executed Verification

| Gate | Result |
|---|---|
| Existence-checked policyMutationStructuralBreaches(process.cwd(), [exactRoot]) | 0 across all 17 classes; coordinator independently confirmed 0 |
| Ajv vs dependency-free internal validator | 48/48 agreements: 16 valid +32 invalid descriptors/payloads |
| Aggregate wire schema | 18 accepted: eight leaf vectors +10 detector emissions; eight malformed extra-field rejections |
| Catalog/direct/variant/vector roster | 8/8/8/8; all five required fixture files resolve per vector |
| Independent Lodash differenceBy/keyBy/isEqual planner | Two language-neutral fixtures, exact 10-mutation output/order |
| Isolated production Rust detector runtime | rustc +nightly compiled and executable exited 0 for the same two fixtures and 10 mutations |
| Nightly rustc AST | 26/26 changed Rust sources parsed; eight wrapped variants agree with internal inspector; eight MutationKind/SEMANTICS facts agree |
| Root AST purity | Zero payloads, match arms, or includes |
| Bun TypeScript syntax | Nine direct/aggregate sources parsed |
| rustfmt --check | 26/26 changed Rust owner/facet/root/operations sources clean |
| git diff --check -- Sequence plugin | Clean |
| Scoped debug/stale-route/nested-owner scan | Zero in mutation root, operations, and changed routes |
| bun nx show project @semio-tech/sequence-plugin --json | Exit 0; registered test/test-quick/test-long/test-exhaustive use the same Rust crate |

The isolated Rust probe copies the production detector function bodies unchanged and the shared assembler with only async removed. Schema-shaped DTO stand-ins are used to avoid linking STDIO; this proves detector logic/order, not the whole crate's macros, trait execution, stores, or codecs. Its temporary [DEBUG] output was captured and the logging statement removed afterward. A pre-existing debug line remains outside this lane in Sequence's TypeScript package script; it was not introduced or edited here.

Full registered `bun nx run @semio-tech/sequence-plugin:test-quick` was NOT launched, per coordinator serialization while the shared STDIO test codegen gate was active. No whole-plugin runtime pass is claimed. GraphQL/protobuf parser tools (`graphql`, `protobufjs`, `protoc`) are unavailable; schema identity/surface checks ran, but those toolchains were not installed or claimed as executed.

Evidence: `🧪️sequence-direct-policy.log`, `🧪️sequence-direct-ajv.log`, `🧪️sequence-direct-nightly-ast.log`, `🧪️sequence-direct-runtime.log`, `🧪️sequence-detection/🦀️component.rs`, and `📓️sequence-direct-verification-commands.md`.
