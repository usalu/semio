# Wires Document Contract Ownership

Wires' document has four fields: `wiresFixture`, `content`, `camera`, and `meta`. Its artifact and snapshot schemas now declare these exact fields in JSON Schema, TypeScript, GraphQL, and Protobuf. The stale `boardFixture` projection is removed. The diff declares its actual five native slots, including the optional whole-artifact replacement; it no longer invents another document shape.

`content` references the shared OS Store `ArtifactChild` contract and IO identity schema. Dynamic values reference the shared Value module's new schema representations under `🧰️framework/🔨️modules/🌱️value/🧬️schema`. The JSON wire projection supports null, booleans, finite numbers, strings, arrays, and string-keyed objects. The TypeScript parser rejects cycles and non-JSON values. The Protobuf representation preserves distinct native integer and floating-number variants. These definitions belong to framework Value rather than individual plugins.

## Validation

The new independent Ajv oracle first reproduced the stale document schema: the actual committed native snapshot was rejected because `boardFixture` was required and `content`, `camera`, and `meta` were forbidden. After correction, the oracle passed against the committed artifact, snapshot, and diff fixtures and the owned TypeScript parsers. It also rejects editor-era `boardFixture` input.

The first native run executed 18 committed-diff laws: 17 passed and one revealed a stale resize-node assertion expecting nine diff slots after the contract had been reduced to five. After correcting that assertion and its docstring, the repeat native run passed all 18 laws with zero failures through Nx in 15 minutes 21 seconds including shared build-lock time.

## Mutation Payload Ownership

The aggregate JSON mutation schema incorrectly referenced bare payload schemas, so it rejected actual committed wire records containing the `mutation` discriminator. An added oracle reproduced this rejection. The aggregate now declares its ten tagged variants and references individual property definitions in their direct mutation payload schemas. Payload TypeScript, GraphQL, and Protobuf definitions live beside those leaf schemas; the aggregate imports/unites them instead of owning duplicate payload records or stale whole-document `wiresFixture`/`boardFixture` fields. Dynamic node/edge/relationship values reference the shared Value owner.

The corrected oracle passed all ten committed wire inputs and rejected an extra locale field on each. Independent TypeScript AST extraction confirmed each leaf's exact payload fields. GraphQL and Protobuf were authored but have not been compiled; no corresponding compiler is installed in the current environment.

The permanent command is root `📜️script.ts verify wires-document-contract [oracle]`, exposed through Nx and matching launch configurations. GraphQL and Protobuf files have not been compiled in this validation.
