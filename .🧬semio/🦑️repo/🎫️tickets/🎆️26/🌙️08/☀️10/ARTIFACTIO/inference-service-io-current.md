# Inference Service Semantic Admission and Wire IO

The TypeScript inference service now admits owned semantic values and schema records without a JSON codec. Physical declaration strings, JSON request/response bodies, UTF8 byte bounds, and route encoding belong to explicit service IO. Store and GIS receiving paths select that IO directly. No string-or-object compatibility admission or semantic-root codec alias remains.

## Contract and ownership

`service/schema/value` defines readonly `ServiceValueV1`, `ServiceSchemaRecordV1`, and limits. `boundedServicePayloadV1` recursively owns and freezes values under 4096 nodes, depth32, and semantic capacity16384. Schema capacity is32768. Node/string/key/number capacity is semantic accounting; it does not encode bytes. Signed zero survives. Nonfinite numbers, unsafe integers, foreign prototypes, symbols, accessors, nonenumerable fields, sparse or extended arrays, and cycles are refused; shared acyclic children are independently copied. Literal `__proto__` keys remain ordinary owned fields.

`AdmittedDocumentServiceDeclarationV1` and its operations own schema records. Pure `admitDocumentServiceDeclarationV1` rejects wire strings and validates schemas through `compileDocumentSchemaV1`. The directory document-http compiler body moved from its deleted root TypeScript file to `document-http/schema`; its public input is a boolean or owned schema record. Existing closed keyword and reference/depth policies remain. Registry admission, dispatch, and status perform no JSON conversion. Contribution identity uses guarded owner/service text separated by a control character rather than a serialized array.

`service/io` owns `DocumentServiceWireDeclarationV1`, whose native input/output schema fields are strings. `admitDocumentServiceWireDeclarationV1` decodes those strings, validates wire UTF8 bounds and metadata bounds, then invokes pure admission. `encodeDocumentServiceWireDeclarationV1` produces the existing native protocol from an admitted declaration. `documentServiceRequestV1` owns JSON/route/body encoding. `admitDocumentServiceResponseV1` owns JSON response admission before pure schema validation. Pretty-printed valid schema JSON remains admitted. Typed values are not accepted as raw schema strings, and strings are not accepted as semantic schema records.

The semantic GIS declaration helper supplies schema records. Its separate `client/io` producer supplies serialized native topic declarations. Store Worker admits raw lease topics through service IO, uses the admitted model internally, and selects explicit request/response IO. Two serialized WASM topic test producers use the wire helper; semantic request fixtures retain the typed helper.

## Schema and independent laws

The service JSON schema adds admitted declaration/operation definitions with schema-record fields. The neutral IO fixture names semantic and wire owners, node/depth/capacity policy, signed-zero and frozen-copy expectations, and a Unicode body accepted semantically but refused by the physical byte budget.

Existing four neutral operation vectors, twelve lifecycle events, six independent Ajv schema oracle cases, and eight hostile cases remain exercised. New laws compare owned values with system `structuredClone`, disable both JSON codecs while exercising actual declaration admission/registry dispatch/status, check semantic and wire budget distinctions, independently validate all six GIS raw schemas with Ajv, test both wrong admission directions, and check current TS syntax and actual receiving fixture ownership. Temporary runtime evidence uses `[DEBUG]`.

The actual GIS receiving config previously selected a generic worker with no GIS laws: Receiving1 executed zero selected laws and is not counted as a pass. Selecting the actual GIS variant exposed thirteen preexisting relocated fixture URLs; all three literal URL roots were corrected and a source guard now resolves every fixture/schema URL. The registered receiving target selects only the 23 GIS inference-port laws.

## Exact validation receipts

| Run | Handle | Result |
| --- | --- | --- |
| SourceRED |87908| Terminal1; intended ownership failure found semantic-root JSON.parse, 1 pass/1 fail. |
| Source2 |69910| Terminal0; strict source and 2 laws/6 assertions. |
| Source3 |50982| Terminal0; strict source and 7 laws/54 assertions. |
| Source4 current |30695| Terminal0; strict source and 8 laws/72 assertions, including final wire metadata UTF8 checks and receiver fixture guard. |
| Receiving1 |69635| Terminal0 but 20 skipped and zero executed; not runtime GREEN. |
| Receiving2 |67608| Terminal1; 10 selected pass/13 missing-fixture-path failures. |
| Receiving3 |90943| Terminal0; 23 selected laws pass; 126 unrelated laws filtered and one unrelated file skipped. |
| Receiving4 current |16420| Terminal0; 23 selected laws pass after final wire metadata changes; 126 unrelated laws filtered and one unrelated file skipped. |

Source target is `@semio-tech/framework-os:installed-service-source`; receiving target is `@semio-tech/gis-gismap-js:inference-service-receiving-check`. Both run through Bun/Nx and permanent package `script.ts` dispatch, project/package entries, and live/seed launch rows. Current generated logs are `space-history-io/inference-service-source-{red,2,3,4}.log` and `inference-service-receiving-{1,2,3,4}.log` inside the ticket generated directory.

## Exact source manifest

Paths are repository-relative. This slice edits only the indicated implementations and receiving registrations; unrelated shared Worker JSON operations remain outside this slice.

- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🚪️io/🟦️.ts` (new actual physical bodies)
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🧬️schema/🧺️value/🟦️.ts` (new pure value model/admission)
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🧫️fixtures/🧱️io-boundary.json` (new neutral fixture)
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🧪️tests/🧱️io-boundary/🟦️.ts` (new laws)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🟦️.ts` (deleted codec-bearing root)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🧬️schema/🟦️.ts` (actual pure compiler body)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client/🚪️io/🟦️.ts` (new wire producer)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}`
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`

## Limits

The TypeScript source and selected actual Store/GIS runtime are verified as stated above. The receiving test transport is the existing authenticated fake transport; this is not a live Hub or native/WASM compiler proof. Native raw DocumentHttpPort strings remain at their explicit existing native IO boundary. No native retry was launched while the shared OS receiving floor is held. Whole artifact IO goal, PDF native stream witnesses, and retained multi-item retirement remain open in their scoped reports.

## Relocated GIS Compiler Receiving Replay

Three `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/{🌉️component-cold-map-patch,💡️inference-control,📇️native-codecs}/🟦️.ts` now directly import the actual test-only native JSON/Ajv compiler at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧰️schema/🟦️.ts`. Exact URL resolution establishes five upward path segments; a six-up assumption was refused by the new actual relative-import law and corrected. No production Schema compiler alias was reintroduced.

Source6 `9143` terminal0:strict service suite +9runtime laws/88assertions including all three helper roots/every direct relative link. Source7 `38947` extends the permanent same strict command to all three receiver sources directly and currently fails on exactly3 RegistryTS55/56/57 unknown reduce-accumulator types, no missing GIS import. Root owns that shared Registry receiving diagnostic. Files additionally edited: these3 GIS imports, service IO boundary law, and only InstalledServiceSourceScript's explicit strict input list in OS TypeScript `📜️script.ts`.
