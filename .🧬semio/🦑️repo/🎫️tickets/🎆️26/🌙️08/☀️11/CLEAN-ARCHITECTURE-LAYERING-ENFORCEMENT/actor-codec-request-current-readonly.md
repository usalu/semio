# Actor Codec Request Current Read-Only Audit

Inspected live source on 2026-10-14; source may change concurrently. No source edits, tests, native builds, or acceptance claims.

## Findings

- Defining helper admits exactly operation-specific own data fields: hash operation/artifactKind; genesis adds documentId; mirror adds pair. Pair requires own pack/spr only. Strings are nonempty without coercion. Own accessor and inherited missing-field objects refuse before property reads. Symbol extras refuse via Reflect.ownKeys.
- Unknown primitive string operation preserves actor-codec.operation:<operation>; missing/nonstring/accessor operation refuses actor-codec.request. RuntimeRequest projection preserves malformed fields and converts only pair hex members.
- Public codec facade explicitly exports request/answer helpers, runtime serializer, refusal map, five types and schema. Defining factory has zero imports, contains both admission helpers and guest answer function, and serializer emits the whole lexical scope with explicit member names.
- ShardClient.codec lines 2259–2264 admit before requireShard, captureActorActivation, nextRequestId and send. Original request is placed directly in transport message; response is admitted using selected operation. Guest function admits before checking codec or reading guest exports. Original request/pair/bytes are passed directly, without normalization.
- Product generator invokes serializer directly; actual worker relays msg.request. Actual renderer hash and print-mirror callers construct operation-specific objects without obsolete documentId/pair padding; generic renderer forwards request.
- Portable fixture has 24 request cases. Tests contain Ajv request projection comparison and independent Buffer byte comparison, SQLite mismatch accounting, foreign realm identity checks, and actual ShardClient prototype invocation. These tests were inspected, not run.

## Concrete Structural Limits

Reflect.ownKeys and Object.getOwnPropertyDescriptor necessarily trigger arbitrary Proxy traps. A Proxy may report own data descriptors while its get trap executes on value.artifactKind, value.documentId, value.pair, or pair.pack/spr. This prevents a universal no-user-code-side-effects guarantee for arbitrary JavaScript owners; ordinary own accessors are refused without invoking their getter. The function preserves original mutable owners, so a caller can mutate request.operation while ShardClient send awaits; response admission currently reads request.operation again. Neither observation is claimed as a completed defect fix or failing test. They delimit the inspected structural contract.

## Exact SHA-256 Snapshot

- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🟦️.ts`: `37929ccf5be43c8675a01dec334351220c1bc53af377d89e8f2db7c0d5607b56`
- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🧬️schema/🔣️.json`: `5e58aa102b88677ebc213e6269b2725e55f4fececd5fd36f6b7fe5fb3a870732`
- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🧪️tests/🧫️fixtures/🔣️.json`: `9d9568d536d447113d2f97cc08c72af665fc585334de2726f4628b2279c7151a`
- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🧪️tests/🟦️.ts`: `3a097a6a4a6b940bbbcfd665367f63ee2a551293b39de54adc982962492f6c00`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts`: `b7d03bf86210bbab13946243708f731be7c500db5c85eaa1ee199ed2822b73b9`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`: `0afe706c7b0c711973ae677143a70249b2ba5ca2865bb86673a069b6f5417165`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`: `b734a601c4cc36099491324ef045959e0ec713311f07c92be969b94ad21087fd`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`: `f8e89691832850978223373e2cac8da1fc6479d8af0af2b64146fbd645add681`
