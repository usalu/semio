# TypeScript Closure Verification

Actual registered Nx runs (Bun wrapper, isolated documented `NX_WORKSPACE_DATA_DIRECTORY` under generated ticket directory, `--excludeTaskDependencies`):

- `@semio-tech/stdio-gltf:test`: passed, 30.5 seconds; 490 language-neutral corpus cases compared to independent Ajv, 1 independent SQLite schema test, 12 canonical SQLite tests. Log: `🗑️generated/typescript-gltf-final-test.log`.
- `@semio-tech/note-note-js:test-document-contract`: passed; 66 native snapshots, 33 committed diffs, independent owner/child rejection vectors. Log: `🗑️generated/typescript-note-final-document-test.log`.
- `@semio-tech/note-note-js:test-snapshot-sqlite`: passed in preceding paired run. The accompanying document target initially failed because the snapshot oracle incorrectly used the artifact binder; the fresh document result above supersedes that failure. Log: `🗑️generated/typescript-note-current-test.log`.
- `run-many test` for DOCX/BCF/PPTX/XLSX/GIS terrain/Lowpoly: all six passed; each managed test compiles sources, builds package, probes consumers and executes suite. DOCX includes new typed SetPart admission/text/binary tests. Log: `🗑️generated/typescript-current-test.log`. glTF in that run failed missing Ajv registry registration and missing patch witness siblings; the fresh glTF run above supersedes those failures.
- Eight-project `check`: PPTX/XLSX/glTF/Note/GIS terrain/Lowpoly passed; BCF readonly fixture assignment and DOCX malformed schema import failed, subsequently repaired and verified through their successful managed tests. Log: `🗑️generated/typescript-current-check.log`.

Older shared workspace graph waits are not validation evidence. DOCX facade exports were added after the six-project result; a fresh check is required for that final facade surface. Rust JSON/GIS/Block check is running and has no completed evidence yet.

## Typed Contracts And Physical Bindings

DOCX SetPart canonical payload is `payload: { kind: "xml", document: XmlDocument } | { kind: "binary", bytes: readonly number[] }`. JSON schema, SDL, proto, authored mutation feature rows and native text grammar now match Rust. The TypeScript native text codec emits JSON payload hex; binary uses existing first-party packed wire values with tag 10 and length-prefixed UTF-8/path/content type/payload. Canonical parser rejects stale bytes fields, unknown payload variants, nonintegral/out-of-range octets and extra payload fields. Neutral corpus plus independent Ajv and Buffer decoding verify exact authored values.

Note uses typed block patches and canonical finite Binary64 admission. Native JSON snapshot fixture binding is located in IO, including SQLite and document contract consumers. BCF canonical readonly fixtures now build a new owner rather than mutating readonly topics. Semio Mesh/Object mutation unions and DOCX async subset syntax are repaired.

glTF physical native diff and mutation readers live under `🚪️io/📝️text`; canonical semantic readers retain owned word/tagged domain admission. The corpus imports the physical reader error class and parser twins, registers the independent registry SnapshotPatch schema, and includes before/after/diff siblings for the authored patch wire witness. Full physical reader manifest: `gltf-native-members-files.md`.

Strict DOCX/PPTX/XLSX VML JSON payload schemas and new canonical TypeScript leaves use `document: XmlDocument`. Previously these strict owners only had subset metadata, without TypeScript native VML codecs. Native Rust and neutral authored VML fixture closure are owned by the XLSX agent. Manifest: `office-vml-typescript-closure.md`.

## Final DOCX/XML Verification

Fresh `bun nx run-many -t test -p @semio-tech/stdio-docx,@semio-tech/stdio-xml --excludeTaskDependencies`: both managed targets passed in 34.9 seconds after facade exports, exact XML doctype canonical/native split, typed doctype neutral vector, and native XML omission/default serialization fixes. Log `🗑️generated/docx-xml-typed-final-test.log`. Canonical XML doctype requires bigint; native XML JSON binder maps authored decimal position text before validation. Native Rust XML position ToValue explicitly uses decimal strings, so text/binary payloads use the same native field representation.

Fresh final Office refresh passed all three DOCX/PPTX/XLSX managed TypeScript tests in 24.8 seconds. Independent Ajv verifies all three strict VML document payloads and rejects source markup. Native JSON snapshot XML position binding now precedes canonical PPTX/XLSX fixture admission. Log: `🗑️generated/office-vml-typescript-repaired-test.log`.
