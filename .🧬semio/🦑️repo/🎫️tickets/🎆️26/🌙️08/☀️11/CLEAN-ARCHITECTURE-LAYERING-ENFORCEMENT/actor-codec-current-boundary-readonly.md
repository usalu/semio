# Actor Codec Current Boundary Readonly Audit

Static source inspection only. No runtime probes, type checks, tests, or registered-route execution were performed. Historical receipts do not qualify the current sources.

## Actual Boundary

Paths below are relative to the workspace. General owner is `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️component-codec/🟦️.ts`.

- Lines 22–31 close the declared success union and export signatures. Lines 56–58 immediately wrap whatever the JavaScript component returns. The closed declaration is authored, but malformed successes are not runtime refused. A wrongly shaped genesis, mirror, or hash can therefore become a declared `ShardCodecAnswer`.
- Line 61 recognizes fault bytes with realm-local `instanceof Uint8Array`. A real foreign-realm byte array fails that predicate; this is a predicate consequence, not evidence that the production worker actually creates foreign-realm faults.
- Product generator `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts:951` embeds the defining function through `toString()`. Lines 1036 and 1122 supply actual imported component codec exports to it. This is the earliest meaningful unknown-to-first-party-result boundary. Any validator it uses must remain self-contained or be explicitly embedded alongside it.
- Generator lines 843–852 enforce activation and actor serialization, then relay the API result. General `📮️shard-client/🟦️.ts:1517` resolves `message.value` without codec validation; lines 1635–1648 erase generic promise resolution, and 2259–2263 declare the received result using `send<ShardCodecAnswer>`. Thus the transport receiving side also trusts a runtime result without proving the union.
- Product renderer `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1952–1956` unwraps the declared answer; it does not validate it. Lines 2074–2090 validate only the two ProgramBridge operations. `codecBytes` uses realm-local `instanceof`; `codecMirror` reconstructs a tuple at line 2090, losing the original mirror-array identity. Genesis has no equivalent Product validation.

## Minimal Defining Contract

Keep the existing first-party request, pair, success, answer, and export interface ownership. Validate operation-specific unknown output in the General defining codec arm before returning `{ ok }`: actual unsigned-byte view for hash; object with actual pack and spr byte views for genesis; exact two-string array for mirror. Preserve the returned object, both byte views, tuple, and original guest fault by identity. Do not encode, coerce, spread, slice, or reconstruct values to make them satisfy the contract.

Use an intrinsic typed-array brand predicate with `ArrayBuffer.isView` plus a genuine Uint8Array brand check; do not admit arbitrary one-byte views, plain numeric arrays, or spoofable string tags alone. Keep the generated arm self-contained. A first-party operation-aware answer admission function can be reused when `ShardClient.codec` receives transport `unknown`; that is distinct from guest export invocation. It should enforce exactly one answer arm, the requested operation's success shape, and fault-byte shape while returning the same admitted owner. Product needs no independent competing codec authority after consuming the validated General contract.

Operation-specific correlation is stronger than the current uncorrelated union: a pair is valid somewhere in `ShardCodecSuccess` but invalid for a hash request. Decide exact additional-key policy in schema before implementing object checks. Current projected DocumentPair and answer schema are closed; TS structural types alone do not express that policy. The hash schema currently permits any byte length; the renderer docstring's 32-byte assertion is not presently a schema law and must not be invented by a validator without changing the defining contract.

## Test Qualification

General codec test `🧬️component-codec/🧪️tests/🟦️.ts:19–64` statically has seven ordinary component cases, owner identity assertions, and Ajv/Buffer/SQLite comparisons. Its `invalidAnswers` are fed only to the JSON projection schema at line 23, never through `actorCodecAnswer`; they do not qualify malformed guest-result refusal. Same-realm constructed faults do not qualify cross-realm acceptance. Pair inputs are compared deeply at line 57 rather than by identity. SQLite compares stored projections; it does not independently establish byte/pair/mirror admission or owner preservation.

Portable fixture laws should cover wrong-operation success, malformed pair byte members, mirror lengths/types, wrong fault payloads, original host-trap rethrow, missing exports/unknown operation, and no coercion. JavaScript-only laws additionally cover realm branding, original output/pair/member identity, and generated-function parity. Run those same portable projections against a genuinely independent validator, keeping schema projection and runtime owner laws distinguished.

Public-facet suite `🧰️framework/🔨️modules/🎭️actor/🧪️tests/📣️public-facets/🟦️.ts` tests seven facets in one test. No obvious strict-Ajv schema blocker was found in its closed enum schema by reading; no claim of successful compile or assertions is made. Lines 25–35 use the TypeScript parser to roster export names and type-only markers; they do not resolve exported type names or semantically type-check them. Lines 38–45 dynamically import and resolve actual workspace package exports, so package linking/import-graph failures can stop before later facet assertions. Its exact schema/fixture roster must change whenever the public codec facet adds or removes an admission symbol. The codec test's `Equal` type at lines 11–12 is meaningful only under an actual TypeScript check; Bun execution alone erases it.

The public codec facet currently explicitly reexports the defining owner and schema. It is a first-party interface seam, but publishing it alone does not qualify boundary enforcement, generated browser behavior, worker receiving, or TypeScript semantics.
