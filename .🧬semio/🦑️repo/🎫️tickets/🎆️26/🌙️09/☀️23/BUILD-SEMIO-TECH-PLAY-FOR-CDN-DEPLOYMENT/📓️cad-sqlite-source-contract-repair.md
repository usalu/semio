# Cad SQLite Source Contract Repair

The complete registered source gate actually failed with 7 passing and 47 failing tests. All three newly added SQLite UTF-8 byte cases passed. The failures were retained as the red baseline in `🗑️generated/cad-paged-text-sqlite-source.log`.

The existing test cast the document schema parser owner to an invented SQLite projection interface. The actual canonical SQLite functions already exist in the adjacent projection module; the repair imports those actual functions and their owned CadSqliteSnapshot/CadSqliteReference types, removing the unchecked interface assertion. The document parser continues to be checked independently at its actual owner. No compatibility export or runtime behavior was added.

The literal reference-index test accidentally repeated top-level ownership/table assertions beneath a fixture object that has neither field. It now checks the declared native map shape, UTF-8 comparison, duplicate-key refusal and local SQL alias contract directly, retaining the independent SQLite BLOB ordering oracle and full literal-key round trip. The neutral snapshot schema check now actually invokes Ajv, validating the declared snapshot and rejecting an extra field.

All 54 original cases remain registered, including nine exact binary64 identities, malformed semantic edits, controlled cancellation, admission budgets, child relationships and intrinsic OBJ parsing against third-party SQLite/Ajv/Three.js oracles. The complete registered Nx source target completed with actual exit 0: 54 passing, 0 failing tests and 140 assertions. Independent schema admission, all malformed semantic edits, exact IEEE identities, cancellation, byte budgets, literal-key ordering and intrinsic OBJ parsing ran successfully. Console evidence also records the three SQLite UTF-8 corpus sizes 0, 3 and 7,800 bytes. The result log is `🗑️generated/cad-sqlite-contract-green.log`. Native serialization and component production remain separate pending gates.

Source attribution: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`.
