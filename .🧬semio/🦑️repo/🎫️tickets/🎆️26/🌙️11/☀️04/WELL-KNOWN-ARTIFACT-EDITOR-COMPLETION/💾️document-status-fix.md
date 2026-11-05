# 💾 Document Status Failure Propagation

Fixed actual handleDocumentEventMessage status dispatch to preserve optional typed writeFailure, including message. Success receipts keep the property absent. Existing client-generation check still ignores stale receipts. Catalog handler in the same source file was not modified.

Schema-first neutral cases validated by Ajv run through the real worker handler and deriveSavedStateV1: failed persisted receipt remains unsaved, successful receipt saved with no failure property, stale client makes no dispatch. Initial regression1fail3pass demonstrated dropped field. Final focused Vitest:exit0,1file4tests passed,duration6.19s,tests40ms. Command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.document-status.config.mjs

New suite engine/🧪️tests/💾️document-status/🟦️.ts and fixture 🧫️fixtures/🔣️.json added to canonical broad host config; archive-owner suite also included. No full suite rerun performed here.
