# Forms Validation Audit

Read-only inspection on 2026-11-04. Commands entered through repo tools exec --light bun. No tests were executed by this audit.

## Declared Owners

- repo run @semio-tech/forms-js:test: artifact TypeScript package script declares only SQLite snapshot, schema consumer, and JSON transport suites in publicOptions. The test command adds no extra suites. Owner: ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript/📜️script.ts.
- repo run @semio-tech/forms-js:verify-physical-codecs: runs JSON transport suite with Bun test and a 120-second owned-command budget.
- repo run @semio-tech/forms-forms-rs:verify-forms-document-contract: dynamically imports schema/🧪️tests/🪪️document/🟦️.ts, invokes testFormsDocumentContractOracle, and strict-checks schema TypeScript. Its optional native argument runs Rust forms_document_contract_ tests; native all removes the filter. Owner: artifact/🧪️tests/🔬️rust-package-commands/🟦️.ts.
- repo run @semio-tech/forms-forms-rs:test and test-snapshot-sqlite variants are declared by the Rust package project manifest.
- repo run @semio-tech/s-forms-composition-tests:verify-forms-try-window-ownership checks try window ownership; its optional native argument runs forms_try_window_ownership_ Rust tests. Owner: ✏️s/🧑‍💻dev/📋️forms/🧪️tests/🔬️typescript-package-commands/🟦️.ts.

## Shared Fixtures And Gaps

The subset owner is artifact/🏅️standards/🔖️1/🪆️subsets/✳️any. Shared JSON fixtures cover document import/blank/document/persistence contracts; answer validation; response submissions/events/timestamps; question choices/patches/visibility/extensions; builder actions; try controls and set-value vectors; placement/drop; authoring inspection; JSON transport; SQLite backing/preflight; editor config mutation before/after/diff/outcome; and document mutation before/after/diff/outcome. Language-agnostic Gherkin cases exist under subset/🧪️tests/🌵️mutate-forms-1 and 🎚️mutate-forms-forms-1-any-editor-config.

Existing authoring/demo-session/placement/patches/extensions/visibility TypeScript test files are outside the three explicitly declared forms-js public suites. The audit did not prove whether the generic package runner independently discovers these; inspect that runner before claiming complete execution.

The subset/🔮️oracles/🔣️.json registers fast-json-patch 3.1.1 for response transitions and fast-check 3.23.2 for change-block-field, both test-only. Its Python mutation oracle is a verified native second implementation, not third-party validation. The Python oracle docstring and registry acknowledge ten original vectors with only title change applying; the other nine pin diagnostics/no-op behavior. Successful create/delete/move/replace coverage needs independent verification or additional vectors. Registry rationale contains old child-scene and path descriptions; actual Python source is subset/🧪️tests/🌵️mutate-forms-1/🐍️.py.

## Browser End-To-End Route

repo run @semio-tech/semio-tech-play:test-e2e is the declared browser owner. Its dependencies are serve-e2e → prepare-e2e → activate-prototype plus workspace:deps-browsers. Prototype preparation includes @semio-tech/framework-os-dev:prepare-forms-react-prototype. Manifest: 🏢️semio-tech/🎡️play/📋️project.json.

Playwright config is 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts. It runs only 🧪️tests/🎭️acceptance/🟦️.ts, one Chromium worker, Desktop Chrome, and an already owned PLAYWRIGHT_BASE_URL. It does not spawn its own web server. PLAY_E2E_GPU=swiftshader selects software GPU. E2E command owner validates the live service session and writes reports beneath SEMIO_TICKET_DIR/🗑️generated when that environment variable is supplied.

The test-e2e-enduser owner restricts grep to 3d en-US/de-DE or language selection, so it cannot validate a forms workflow by itself. No literal forms reference was found in current play acceptance suite/config or forms-specific browser files. Generic pane census may mount forms through catalog data, but that is not evidence for authoring, input, submission, undo, persistence, or language workflows. Full user interactions need explicit acceptance coverage.
