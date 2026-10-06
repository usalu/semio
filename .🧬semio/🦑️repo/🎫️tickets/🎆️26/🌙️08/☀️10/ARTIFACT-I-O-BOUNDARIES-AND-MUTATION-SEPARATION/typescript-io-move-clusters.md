# Definite TypeScript Physical Codec Move Clusters

For each path below, prefix is `✏️s/🔌️plugins/{plugin}/🗿️artifacts/{artifact}/🏅️standards/{version}/🪆️subsets/{subset}`. All moves are within that prefix. Source ownership only was reviewed; no tests run.

| Cluster | Schema source | Target beneath 🚪️io | Consumers and validation |
|---|---|---|---|
| Forms (📋️forms/📋️forms/🔖️1/✳️any) | 🌱️value/🔣️json/🟦️.ts (all functions) | 📝️text/📸️snapshot/🔣️json, split Diff and ChangeBlockField into 🔺️diff and 🧬️mutations | Artifact root 🟦️.ts:7 exports this directly; Forms persistence/document/change-block-field tests; response exporter |
| Forms response export | 📨️response/📤️export/🟦️.ts | 📝️text/📸️snapshot/📨️response/{json,csv} | Streaming export and exportResponses; preserve adjacent 🧪️tests/🟦️.ts and fixtures |
| Forms dictionary | 🧾️dictionary/🪪️native-json/🟦️.ts | 📝️text/📸️snapshot/🧾️dictionary/🪪️native-json | formDictionaryNativeJson / formDictionaryFromNativeJson |
| Architect (🏛️architect/🏛️program/🔖️1/✳️any) | 🔣️json/🟦️.ts | 📝️text/📸️snapshot/🔣️json | programArtifactToJson/fromJson; old IO import/export serializers reexport these schema functions |
| Layout (📏️layout/📏️layout/🔖️1/✳️any) | 🪪️native-json/🟦️.ts | 📝️text/📸️snapshot/🪪️native-json | native word/value JSON projection helpers |
| Semio drawing (🗄️stdio/🧿️semio/🔖️v1/🖊️drawing) | 📸️snapshot/🪪️native-json/🟦️.ts (17 lines, 6072 chars) | 📝️text/📸️snapshot/🪪️native-json | semioDrawingSnapshotFromNativeJson / semioDrawingSnapshotNativeJson |
| Semio graph (🗄️stdio/🧿️semio/🔖️v1/🕸️graph) | 📸️snapshot/🟦️.ts:65–76 | 📝️text/📸️snapshot/🔣️json | graphWord, graphJson, parseSemioGraphJsonValue, semioGraphJsonValue; preserve independent semantic validation |
| PDF (🗄️stdio/📖️pdf/7️⃣1.7/🧱️base) | 📸️snapshot/🪪️native-json/**/🟦️.ts | 📝️text/📸️snapshot/🪪️native-json | 9 modules: root, document, metadata, navigation, resource, content, font, form, annotation; numeric transport conversion is physical |
| Remodeling (📸️remodel/📸️remodeling/🔖️1/✳️any) | 📸️snapshot/🟦️.ts:781–906 | 📝️text/📸️snapshot/🔣️json | Transport branch of decodeValue/decodeRecord, decodeRemodelingSnapshot, writeValueJson/writeRecordJson, encodeRemodelingSnapshot, text parsing; preserve semantic decodeRecord(...,false), specs; split diff/mutation consumers |
| Block 3D (🧱️block/🧊️3d/🔖️1/✳️any) | 📸️snapshot/🟦️.ts:39–41 and jsonGeometry helper | 📝️text/📸️snapshot/🔣️json | block3dSnapshotToJsonText / block3dSnapshotFromJsonText |
| TXT (🗄️stdio/🔤️txt/🔖️utf-8/✳️any) | 🔨️modules/🧬️mutation-support/🟦️.ts:34–75 | 💾️binary/🧬️mutations/🔣️protobuf | TxtProtobufReader, txtProtobufKey, txtProtobufString; do not automatically move Unicode/u32 semantic validators or GraphQL coercers |

No direct non-test schema→IO imports were found in targeted TS/Python/C# scan. IO→schema is the correct dependency direction, but architect wrappers currently point at physical code still in schema. No non-test Python/C# json.loads/dumps/JsonSerializer/BinaryReader/BinaryWriter hits were found in artifact schemas. Kit snapshot JSON.stringify is duplicate-ID diagnostics, not a codec. Exclude ordinary cloning, diagnostics and semantic value parsers.
