# TypeScript Artifact IO Audit

Read-only source audit, 2026-10-06. No runtime tests executed. The current checkout was inspected with rg and direct source reads; simultaneous changes may supersede paths.

## Confirmed physical codecs still owned by schema

Move each codec to the same artifact/subset `🚪️io/📝️text/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}` (JSON/CSV/GraphQL), or `🚪️io/💾️binary/🧬️mutations` (Protobuf), with native JSON nested under the appropriate representation member. Retain semantic types/owned-value validation in schema; redirect consumers directly to IO, avoiding schema reexports.

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/🎯️navigation/🟦️.ts

47 lines; 7495 bytes. Exported entry points: pdfFitFromNativeJson, pdfDestinationFromNativeJson, pdfFileFromNativeJson, pdfActionKindFromNativeJson, pdfActionFromNativeJson, pdfOutlineFromNativeJson, pdfOpenActionFromNativeJson, pdfNamedDestinationFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/📇️metadata/🟦️.ts

22 lines; 5062 bytes. Exported entry points: pdfPageModeFromNativeJson, pdfPageLayoutFromNativeJson, pdfInfoFromNativeJson, pdfEmbeddedFromNativeJson, pdfIntentFromNativeJson, pdfEncryptionFromNativeJson, pdfPreferencesFromNativeJson, pdfMarkFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/🟦️.ts

127 lines; 11382 bytes. Exported entry points: pdfCosFromNativeJson, pdfDictionaryFromNativeJson, pdfFunctionFromNativeJson, pdfColorFromNativeJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪪️native-json/🟦️.ts.

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/📄️document/🟦️.ts

27 lines; 6172 bytes. Exported entry points: pdfPageFromNativeJson, pdfNamedColorFromNativeJson, pdfNamedPropertiesFromNativeJson, pdfIndirectFromNativeJson, pdfLabelFromNativeJson, pdfSnapshotFromNativeJson.

Consumers/import-path matches: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📄️document/🟦️.ts.

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/🖼️resource/🟦️.ts

43 lines; 9565 bytes. Exported entry points: pdfCcittFromNativeJson, pdfImageCodecFromNativeJson, pdfImageMaskFromNativeJson, pdfImageFromNativeJson, pdfGroupFromNativeJson, pdfFormFromNativeJson, pdfSoftMaskFromNativeJson, pdfStateFromNativeJson, pdfShadingKindFromNativeJson, pdfShadingFromNativeJson, pdfPatternKindFromNativeJson, pdfPatternFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/🖋️content/🟦️.ts

58 lines; 6564 bytes. Exported entry points: pdfMatrixFromNativeJson, pdfTextFromNativeJson, pdfTextItemFromNativeJson, pdfPropertyFromNativeJson, pdfInlineFromNativeJson, pdfOperationFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/🔤️font/🟦️.ts

47 lines; 8043 bytes. Exported entry points: pdfDescriptorFromNativeJson, pdfProgramFromNativeJson, pdfEncodingFromNativeJson, pdfUnicodeMappingFromNativeJson, pdfUnicodeFromNativeJson, pdfCidMappingFromNativeJson, pdfCMapFromNativeJson, pdfGidFromNativeJson, pdfWidthRunFromNativeJson, pdfVerticalRunFromNativeJson, pdfCidFontFromNativeJson, pdfCharProcFromNativeJson, pdfFontKindFromNativeJson, pdfFontFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/📝️form/🟦️.ts

16 lines; 4320 bytes. Exported entry points: pdfFieldKindFromNativeJson, pdfFieldFromNativeJson, pdfAcroFromNativeJson, pdfOptionalGroupFromNativeJson, pdfOptionalFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪪️native-json/📌️annotation/🟦️.ts

57 lines; 8619 bytes. Exported entry points: pdfDateFromNativeJson, pdfAppearanceStateFromNativeJson, pdfAppearanceEntryFromNativeJson, pdfAppearanceFromNativeJson, pdfBorderFromNativeJson, pdfMarkupFromNativeJson, pdfAnnotationKindFromNativeJson, pdfAnnotationFromNativeJson.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🪪️native-json/🟦️.ts

17 lines; 6119 bytes. Exported entry points: semioDrawingSnapshotFromNativeJson, semioDrawingSnapshotNativeJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪪️native-json/🟦️.ts.

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🟦️.ts

37 lines; 2429 bytes. Exported entry points: RESPONSE_COLUMNS, responseRows, exportResponses.

Consumers/import-path matches: Use symbol lookup; relative member imports are shared across artifacts..

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌱️value/🔣️json/🟦️.ts

90 lines; 10658 bytes. Exported entry points: parseFormsJsonValue, formsValueJson, formsValueJsonProjection, parseFormsJsonCondition, formsConditionJson, parseFormsJsonQuestion, formsQuestionJson, parseFormsJsonDefinition, formsDefinitionJson, parseFormsJsonResponse, formsResponseJson, parseFormsJsonArtifact, parseFormsJsonDiff, formsArtifactJson, parseFormsJsonChangeBlockField, formsBlockFieldJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/📍️placement/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧩️extensions/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧪️tests/🔬️patches/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🧪️tests/🔬️editing/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🧪️tests/🔬️events/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧪️change-block-field/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/💾️persistence/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests/🟦️.ts.

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🪪️native-json/🟦️.ts

36 lines; 3701 bytes. Exported entry points: formDictionaryNativeJson, formDictionaryFromNativeJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪪️native-json/🟦️.ts.

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔨️modules/🧬️mutation-support/🟦️.ts

75 lines; 4663 bytes. Exported entry points: TxtMutationDecodeError, txtMaximumU32, failTxtMutationDecode, txtOwn, txtWireRecord, txtExact, coerceTxtMutationUInt32Variable, coerceTxtMutationUInt32Literal, txtUnicode, TxtProtobufReader, txtProtobufKey, txtProtobufString.

Consumers/import-path matches: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️set-trailing-newline/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line/🟦️.ts.

### ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️json/🟦️.ts

478 lines; 31798 bytes. Exported entry points: programArtifactFromJson, programArtifactToJson, programDiffFromJson, programDiffToJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/📍️placement/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧩️extensions/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧪️tests/🔬️patches/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🧪️tests/🔬️editing/🟦️.ts; ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts; ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts; ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts; ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔣️json/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🟦️.ts; ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🧪️tests/🟦️.ts.

### ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪪️native-json/🟦️.ts

34 lines; 8018 bytes. Exported entry points: layoutArtifactFromNativeJson, layoutArtifactNativeJson, layoutDiffFromNativeJson, layoutDiffNativeJson.

Consumers/import-path matches: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts; ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts; ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪪️native-json/🟦️.ts; ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts; ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts.

## Inline extractions

- Remodeling snapshot `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`: physical transport branches in `decodeValue`/`decodeRecord` (781–820), `decodeRemodelingSnapshot` (821), `writeValueJson`/`writeRecordJson` (847–891), `encodeRemodelingSnapshot` (893), `remodelingSnapshotToJsonText` (896), JSON text parser from 899. Preserve semantic decodeRecord(..., false) and specs under schema. Also inspect diff and mutations importing those writers.
- Block 3D `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`: `jsonGeometry`, `block3dSnapshotToJsonText` (39), `block3dSnapshotFromJsonText` (41): Binary64-to-JSON conversion and JSON text parsing belong in text snapshot IO.
- Semio graph `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts`: `graphWord`, `graphJson` (65–72), `parseSemioGraphJsonValue` (74), `semioGraphJsonValue` (76) convert bigint words to 16-digit hexadecimal wire records. Semantic graphRecord/graphProperties validation may remain.

## Exclusions and dependency checks

- JSON diagnostic messages, structuredClone or JSON cloning, fixture loaders, byte-length validation, and owned-value parsers alone are not physical codec violations. No non-test Python/C# artifact schema json.loads/dumps/JsonSerializer/BinaryReader/BinaryWriter occurrences were found in the targeted scan.
- Semio kit snapshot only used JSON.stringify for a duplicate-ID diagnostic in the scanned file: do not extract that.
- No direct artifact schema imports of `🚪️io` were found in the targeted TypeScript/Python/C# non-test scan. However physical implementations remain in schema and IO wrappers import them: architect program legacy export/import JSON wrappers explicitly reexport `programArtifactToJson`/`programArtifactFromJson` from schema/json.
- Forms artifact root `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🟦️.ts:7` explicitly exports JSON codec functions from schema/value/json; redirect to IO ownership.

## Validation to preserve

- Existing Forms response export `📨️response/📤️export/🧪️tests/🟦️.ts` tests JSON chunks and CSV output; preserve them under IO tests. Existing forms schema persistence/document/change-block-field tests consume JSON conversion and should point to IO.
- Preserve each relevant artifact schema document-contract/fixture tests as semantic contract coverage; codec round-trip/third-party comparison coverage belongs beside text/binary IO. No test-pass claim is made by this audit.

## TXT Boundary Qualification

Extract only `TxtProtobufReader`, `txtProtobufKey`, and `txtProtobufString` to binary mutation IO. Unicode/u32 checks and GraphQL scalar coercers may be semantic admission and must not be moved solely for their transport naming. See `typescript-io-move-clusters.md` for exact source/target clusters.
