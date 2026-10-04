# PowerPoint Canonical Schema Closure

The aggregate Stdio TypeScript gate passed its build/compiler checks but failed when the OOXML test registered the current PPTX diff schema without its referenced snapshot schema. The diff now replaces only canonical schema/OPC/XML-part fields and refers to the snapshot’s OpcPackage and PptxXmlPart definitions. The old test also tried to validate a removed PptxShapeDiff projection definition.

The test now registers the snapshot before the diff, validates a canonical xmlParts placeholder-kind replacement, and compares the complete TypeScript guard output with that neutral fixture. The placeholder fixture is handcrafted slide XML with its type attribute set to body, preserving qualified element structure. Two stale presentation shadow fields were removed from the snapshot/replacement boundary fixture; its canonical XML declarations, prolog, doctype, root and epilog remain intact. Production schemas/codecs were not changed in this repair.

The observed red receipt is `natural-file-schema-typescript-7.log` from the ownership lane. The fresh aggregate rerun is `pptx-canonical-schema-aggregate-1.log`; its verdict is pending.

## Files

- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧫️fixtures/🏷️placeholder-kind/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🧭️xml-document-boundaries/🔣️.json`

The fresh aggregate rerun completed successfully in21.6s with cache disabled: `@semio-tech/stdio-js:test`, artifacts=36, all36dependency tasks successful. This confirms the shared OOXML schema/guard fixtures at this checkpoint, including DOCX retained public schema corrections from the ownership lane. It does not replace native file-byte or browser acceptance.
