# 🔏 Folder Archive Ownership ABI Fix

Corrected production identity ABI to required native parent_document_id (nullable string). Removed optional camelCase fallback. Archive reads now require exact document identity and current captured entry before and after asynchronous identity/archive reads. Numeric instance IDs are compared within session.pluginId, allowing distinct plugins with the same instance number. Missing archive reader still raises document-backbone.archive-reader-unavailable.

Updated neutral fixture includes matching/mismatched/null identities, cross-plugin equal instance numbers and same-plugin conflicting reuse. Existing tests now use real snake_case identity shape and plugin-scoped sessions; added mutation-during-identity-read regression.

Red baseline after correcting tests:4failed3passed7tests (312ms); old implementation accepted wrong document archives and read an archive after its entry was replaced. Green result:7passed0failed1file,exit0,duration231ms,tests38ms. Command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.archive-owner.config.mjs

Changed only archive-owner production module, existing folder-archive-owner tests, new neutral test fixture and ticket config/report.

## Persistence Design Notes Sent to Root

readAppDocumentIdentity is read-only and does not mint; actor documentId must equal imported store parent_document_id unless the complete event archive is explicitly rekeyed. Current LocalDocument six fields cannot retain exact editor standard/subset, so generic recent-file owner metadata needs a schema change or authoritative archive routing, not overloaded name/schema. localCatalogAdmissionV1 builds members:[] and must not be used to truncate recursive stdio document archives. FolderReadBack.absent triggers initial archive save; genuinely fresh folder targets prevent initial read-back replacing import with unrelated prior content. Existing reusable suites: os/🧪️tests/🧪️folder-read-back and 🧪️folder-archive-persistence.
