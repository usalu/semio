# OS Config Direct 30 Proposal

## Scope

This packet adopts the existing three typed config aggregates as canonical direct-owner
aggregates. It does not change Flow's admission-error arm, Store, protocol APIs, or taxonomy.
The canonical aggregate source is
`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`.
Its direct owners are exactly the five immediate children below; no historical
`🦀️component.rs`/`🔣️component.json` source or descriptor remains mounted as a fallback.

| Aggregate | Direct owner | Variant | Semantic identity | Invertibility | Outcomes |
| --- | --- | --- | --- | --- | --- |
| OpeningConfigMutation | `📌️set-default-app` | SetDefaultApp | set/default-app/set-default-app/Set | explicit-mutation | applied, warning |
| OpeningConfigMutation | `🧹clear-default-app` | ClearDefaultApp | clear/default-app/clear-default-app/Cleared | explicit-mutation | applied, warning |
| MergePolicyConfigMutation | `🛡️change-merge-policy` | ChangeMergePolicy | change/merge-policy/change-merge-policy/Changed | self | applied, warning |
| IdentityConfigMutation | `🪪️sign-in` | SignIn | set/identity/sign-in/Set | explicit-mutation | applied |
| IdentityConfigMutation | `🚪️sign-out` | SignOut | clear/identity/sign-out/Cleared | explicit-mutation | applied |

Every descriptor has the exact fourteen fields, `payloadSchema: "🧬️schema/🔣️.json"`,
`textOpcode: null`, `binaryTag: null`, `diffParticipation: "apply-only"`, and
`composition: "atomic"`. Its required surfaces are `rust`, `typescript`, `json-schema`, and
`text`: JSON text encoding/decoding exists today, but no leaf-owned binary opcode/tag or GraphQL
or protobuf surface exists. The `ChangeMergePolicy` semantic record must change from `Change` to
the approved past-tense `Changed`.

## Canonical Files

For each listed leaf, create/retain exactly:

- `🦀️.rs`: one `dsl::MutationLeaf` payload and its `MutationKind` behavior.
- `🔣️.json`: its descriptor.
- `🧬️schema/🔣️.json`: strict JSON Schema for the payload only.
- `🟦️.ts`: the TypeScript payload and behavior implementation.

The aggregate owns only the three enum wrappers, `#[derive(dsl::Mutations)]`, aggregate codec
mechanics, and structural roster tests. Move the `#[path]` mounts in plugin-host Rust glue to
these primary filenames; do not leave an alias module or duplicate implementation.

## Payload Contracts

All JSON Schema objects use `additionalProperties: false` and require every stated property.

| Leaf | Payload |
| --- | --- |
| SetDefaultApp | `{ dialect: { artifactKind, standard, subset }, role: "viewer" | "editor", app: { pluginId, appId } }`; all five scalar identifiers are strings. |
| ClearDefaultApp | `{ dialect: { artifactKind, standard, subset }, role: "viewer" | "editor" }`; all three coordinate strings are required. |
| ChangeMergePolicy | `{ policy: "LaissezFaire" | "Normal" | "Vigilant" }`. |
| SignIn | `{ userId, email, displayName, hubBaseUrl, sessionToken, issuedAtMs }`; first five are strings and `issuedAtMs` is a non-negative integer. |
| SignOut | `{}` only. |

`SignIn.issuedAtMs` is an acceptance mismatch to resolve within this packet: Rust stores `u64`,
while TypeScript's JSON number cannot exactly represent values above `9007199254740991`. The
proposed canonical JSON domain is the non-negative safe-integer range `0..9007199254740991`,
enforced by owned Rust deserialization/entry validation and TypeScript validation. This preserves
the current Rust storage representation without admitting nonportable JSON payloads.

All Rust payloads and aggregate envelopes must use `deny_unknown_fields`; the existing empty
`SignOut` otherwise admits unknown payload fields while the schema rejects them. Aggregate
envelopes retain the existing `mutation` discriminator and flattened canonical payload shape.

## Immediate Consumer Writes

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs` updates its six canonical path mounts.
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️component.rs` keeps only opening-state and aggregate codec mechanics, referring to the new aggregate module path.
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts` and the five leaf `🟦️.ts` files update direct imports.
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` update TypeScript aggregate imports.
- Existing leaf-owned Rust and plugin-host fixture tests update their source/descriptor include paths and gain descriptor, strict-envelope, and schema cases. The generated WGPU target is not edited.

## Neutral Matrix

`🧪️os-config-direct-30/🧫️fixtures/🔣️canonical-payloads.json` fixes one valid payload per leaf,
the `SignOut` empty object, and invalid strictness/domain vectors. An Ajv ticket oracle should
compile each leaf schema and validate all vectors; a Rust runtime gate remains root-owned and is
not implied by this schema-only proposal.
