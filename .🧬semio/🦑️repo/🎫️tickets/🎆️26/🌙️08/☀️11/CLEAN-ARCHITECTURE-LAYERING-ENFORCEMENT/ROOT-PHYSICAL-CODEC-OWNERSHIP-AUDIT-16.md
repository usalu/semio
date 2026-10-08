# Physical Codec Ownership Audit 16

Read-only static inspection; no tests run, no production edits. Paths below are repo-relative.

Original suite: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🟦️.ts`. Original fixture: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/physical-codecs/🧫️fixtures/🔣️.json`.

## Current execution

General `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:103` runs this suite from `test artifact-io-ownership`, alongside architectural ownership and Generic UTF-8 tests. Each subprocess has 120000 ms budget. `.vscode/launch.json:6124` and `:29148` both call `bun nx run @semio-tech/repo-lib:test-artifact-io-ownership --skip-nx-cache`. The suite is a General-to-Specific source dependency even though its tests are IO-oriented.

## Precise slices to move

Preserve original test names, every assertion, Ajv independent const/schema checks, Buffer UTF-8/IEEE754 oracles, rejection cases, and fixture values. Place each destination under its existing IO owner as `🧪️tests/physical-codecs/🟦️.ts`, with local `🧫️fixtures/🔣️.json`; move only the listed fixture slices. Duplicate the tiny independent Buffer word oracle where needed; do not import a sibling plugin or leave an import to General fixture.

| Original case | Fixture slice | Destination owner directory | Preserved obligations |
|---|---|---|---|
| Forms JSON and response exports follow the neutral transport corpus | `forms` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json` | exact JSON, Ajv const, existing response export oracle |
| Graph JSON preserves nonfinite words and ordered literal fields | `graph` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json` | negative-zero/NaN payload exact bits, complete Ajv const roundtrip |
| PDF native JSON converts only at IO and canonical guards require owned words | `pdf + word` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json` | COS/function owned admission; reject both native representations; exponential discriminant and n bits; independent native PdfFunction schema |
| TXT physical protobuf field decoding preserves Unicode and rejects truncated frames | `txt` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✏️set-line` | Ajv decoded const, Buffer Unicode oracle, truncated frame rejection |
| Remodeling JSON projects canonical defaults and decodes each exact scalar role | `word` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json` | default full roundtrip, syncOffsetMs exact bits, canonical native JSON rejection, Ajv serialized word const |
| Block 3D physical JSON resolves words before canonical snapshot admission | `block` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json` | radius word bits, owned reread equality, Ajv complete serialized const |
| GIS semantic requests acquire JSON byte admission only at transport IO | `gisInference` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🌐️hub` | exact JSON and 176-byte UTF-8 length, Ajv independent JSON const, oversized-request rejection |

## Existing delegated tests already have owner routes

| Function | Existing owned caller | Wrapper deadline |
|---|---|---|
| `testProgramDocumentContract` | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/📜️script.ts:53`, `verify program-document-contract`; GUI `.vscode/launch.json:30555`, project `@semio-tech/architect-program-rs:verify-program-document-contract` | 60000 ms |
| `testLayoutDocumentContractOracle` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📜️script.ts:25`, `verify layout-document-contract`; GUI `.vscode/launch.json:30445`, project `@semio-tech/layout-layout-rs:verify-layout-document-contract` | Bun default, no explicit override |
| `testResumableQueryOracle` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/📜️script.ts:19`, `verify jack-query-ownership [oracle]` | 60000 ms |
| `testFormsResponseExport` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts:26`; registered by owner TypeScript script publicOptions; GUI `.vscode/launch.json:25757` calls `@semio-tech/forms-js:test` | Bun default, no explicit override |

Delegated functions are not executed only from General. Remove their General wrapper dependency only after preserving the wrapper registrations/deadlines in owner-executed suites (especially Program and Jack 60000 ms). Existing direct script calls do not themselves preserve Bun per-test deadlines. Keep all underlying corpus and oracle files at their current owners.

## Registration and retained Generic laws

Forms, PDF, Stdio Semio and Block have artifact TypeScript package script suite registries; add the exact destination to those existing registries. TXT TypeScript script currently calls `runArtifactTypeScriptPackageMain` with no explicit suites; supply its new owned suite. GIS has a custom TypeScript router; add an owner command for this precise transport case, rather than importing it into General. Remodeling currently has only a Rust artifact script; extend that existing `📜️script.ts` to execute the owned Bun suite, preserving 120000 ms outer budget and add its owner Nx/GUI route. Do not create a new script filename.

The seven inline cases have no Generic artifact-free law: each requires Specific parsers, schema, service identity, or snapshots. The shared `word` fixture is independent scalar data, but its existing assertions are attached to Specific PDF/Remodeling semantics; partition locally, not into a global test aggregator. General should retain the independent UTF-8 suite and ownership-direction suite already called by its script. General may retain pure architecture source-inspection laws, but must not import the moved artifact suites, registry of concrete plugin paths, or concrete plugin project names. Workspace orchestration can schedule owner commands without creating General-to-Specific source imports.

## Exact existing imports (delegated corpus owners included)

- `gisMapInferenceRequestToJson`: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🌐️hub/🟦️.ts`
- `sealGisMapInferenceJobRequestV1`: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts`
- `testLayoutDocumentContractOracle`: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts`
- `testProgramDocumentContract`: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-contract/🟦️.ts`
- `testResumableQueryOracle`: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts`
- `parseFormsJsonValue`: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `formsValueJson`: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `testFormsResponseExport`: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🟦️.ts`
- `parseSemioGraphJsonValue`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `semioGraphJsonValue`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `pdfCosFromNativeJson`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts`
- `pdfFunctionFromNativeJson`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts`
- `parsePdfObject`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts`
- `parsePdfFunction`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts`
- `schema as pdfNativeSchema`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts`
- `decodeSetLineProtobuf`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✏️set-line/🟦️.ts`
- `decodeRemodelingSnapshot`: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `remodelingSnapshotToJsonText`: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `defaultRemodelingSnapshot`: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `parseRemodelingSnapshot`: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `block3dSnapshotFromJsonText`: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
- `block3dSnapshotToJsonText`: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`
