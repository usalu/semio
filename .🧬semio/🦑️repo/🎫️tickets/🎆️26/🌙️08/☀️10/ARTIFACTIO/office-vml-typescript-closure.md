# Office VML TypeScript Closure

Three strict Office InsertVmlPart payload contracts now carry owned `document: XmlDocument`. Canonical TypeScript leaf parsers validate exact payload keys and call the owned XML guard. These strict owners previously had no TypeScript mutation codecs or aggregates, only subset metadata. Rust native codecs and neutral fixture closure are assigned to the XLSX agent.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🟦️.ts`

## Final Contract Paths

PPTX uses the authored `🖼️insert-vml-part` leaf; DOCX and XLSX use `✒️insert-vml-part`. All strict leaves now require the owned document.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🖼️insert-vml-part/🧬️schema/🔣️.json`: ['path', 'document']
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json`: ['path', 'document']
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/✒️insert-vml-part/🧬️schema/🔣️.json`: ['path', 'document']

Three independent Ajv acceptance/source-markup rejection tests execute in the registered DOCX physical suite and compile all three canonical TypeScript leaf parsers. Final refreshed DOCX/PPTX/XLSX managed test is active, session32555, log `🗑️generated/office-vml-typescript-repaired-test.log`. The preceding refresh failed for a wrong PPTX leaf emoji in test imports and native XML decimal fixtures using the owned guard; those paths/binders are repaired.

## Actual Final Verification

Fresh registered `bun nx run-many -t test -p @semio-tech/stdio-docx,@semio-tech/stdio-pptx,@semio-tech/stdio-xlsx --excludeTaskDependencies --output-style=static` passed all three targets in 24.8 seconds. DOCX: 11 typed SetPart/VML tests and 10 SQLite tests; PPTX/XLSX suites also passed. Log `🗑️generated/office-vml-typescript-repaired-test.log`. No final VML TypeScript work remains unverified.
