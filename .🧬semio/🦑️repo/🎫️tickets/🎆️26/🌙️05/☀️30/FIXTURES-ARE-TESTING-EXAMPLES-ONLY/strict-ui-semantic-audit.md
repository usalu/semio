# UI Moved Fixture Semantic Audit

Current physical source read; no source edits, Cargo, runtime or test execution. Hashes in 🗑️generated/strict-ui-semantic-inputs.json.

UI host schema `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json:31` cites browser-host-limits.tsv in a JSON Schema description. This is annotation, not a file resource expression. Actual consuming reader is host/🧪️tests/🔬️event-unit/🦀️.rs:5 include_str!(../🧫️fixtures/📐️browser-host-limits.tsv). Its parent host/📡️event/🦀️.rs:763-765 mounts the source under #[cfg(test)] and the fixture exists in host/🧪️tests/🧫️fixtures. This source edge is test-only, not production runtime.

Scalar schema exact selectors:

- /$defs/ScalarDeclaration/properties/closePlan/const/reader/path
- /$defs/ScalarDeclaration/properties/closePlan/const/readerAlias/path
- /$defs/ScalarDeclaration/properties/closePlan/const/page/path

Unlike description annotation these are normative const fields of the ScalarDeclaration source declaration contract. They constrain declared evidence coordinates; they do not themselves open files. The matching scalar/🔣️.json declaration reports status runtime-unmounted-source-declaration and registeredTestProposal.status source-oracle-contract. It is a frozen source model with test registration/evidence, not observed live scalar runtime.

Actual reader is OS renderer element UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx:36-42 importing declaration, schema, scalar fixture, reader raw fixture and page raw fixture. exactSchemaDefinitionV1 selects ScalarDeclaration at :38, AJV checks declaration at :134-135. childSources :136-139 keys are the moved reader/page paths. runScalarCloseModelCold at :157-158 indexes these keys using declaration closePlan[name].path, SHA-256 verifies raw bytes, parses original reader/page fixture grants and compares the declared close-plan slices. Hence these fields serve actual test-only original-source evidence bindings, not inert labels and not production asset mounts.

UiDocumentStore production source :689-691 gates dynamic test registration on import.meta.vitest. Static source indicates test-only ownership; proof that release bundling folds this gate requires actual selected build inputs/compile receipts, not this grep.

Potential schema concern should be classified separately: ScalarDeclaration includes frozen source oracle model/registered test proposal and example evidence path consts. This is not a runtime-fixture edge merely because the contract mentions a fixture. Its full source-declaration contract authority may warrant review under no-independent-corpus-schema policy; the inspected reader validates declaration (not scalarTestsJson against a separate fixture schema), then compares ordinary test examples with independent oracles. No name-only violation conclusion is made.

Physical declaration bindings checked with Python SHA-256 (not a test/runtime run):
- reader: target exists=True; declared digest matches=True.
- readerAlias: target exists=True; declared digest matches=True.
- page: target exists=True; declared digest matches=True.
