# ISO16757 Independent Source and Native-Parity Extents

Actual independent measurement completed through selected `bun nx exec --projects=@semio-tech/norm-iso16757-rs --excludeTaskDependencies -- bun <ticket-script>` with exit zero for both Source and Native-parity modes. This is an independent Source physical measurement, not a Native owning gate or Native budget receipt.

The retained [measurement script](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/iso16757-independent-extent/📜️script.ts) borrows the existing Source factory helper declarations from [actual Source tests](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts:11), constructs the complete populated typed Source model, calls the existing Source provider and physical exporter, then queries independent Bun SQLite. It does not import the test suite, run an owning test filter, or derive costs from provider cell counters.

For every table, `PRAGMA table_info` supplies actual columns. Independent SQL sums each actual cell by storage class: NULL zero; INTEGER/REAL eight; TEXT/BLOB `length(CAST(column AS BLOB))`. Authored INTEGER PRIMARY KEY is counted once as its declared column. Every measured case has all 55 tables populated, 208 rows, `PRAGMA integrity_check = ok`, and zero foreign-key failures. Runtime `[DEBUG]` output confirmed all literal totals.

| Case | Original Source Factory Bytes | Native-Parity Source Bytes |
|---|---:|---:|
| Complete base, 0.5 | 7394 | — |
| 0000000000000000 | 7394 | 7394 |
| 8000000000000000 | 7394 | 7394 |
| 7ff0000000000000 | 7924 | 7934 |
| fff0000000000000 | 7924 | 7934 |
| 7ff8000000000042 | 6811 | 6800 |
| 7ff0000000000042 | 6811 | 6800 |
| 0000000000000001 | 7394 | 7394 |
| 8000000000000001 | 7394 | 7394 |

The original Source factory replaces the main fixture's 0.5 leaves but constructs extensionCases Float from its original literal 0.5 word. The actual [Native every_word helper](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:17) also rewrites every intrinsic extension Float. The separately named Native-parity mode borrows the same Source factory then rewrites those extension Float leaves recursively to the selected word; it does not modify the original Source measurements. The additional present Float explains +10 infinity bytes and -11 NaN bytes. Native runtime equality and full/one-short admission remain required; these independent Source file totals provide the literal expected authority.

[Original Source proposed authority](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/iso16757-independent-complete-source-semantic-extent-proposed-authority.json) retains nine literal cases with every table's row/cell cost; [Source closed schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/iso16757-independent-complete-source-semantic-extent-proposed-schema.json) fixes the exact nested case corpus. [Native-parity proposed authority](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/iso16757-independent-complete-native-parity-semantic-extent-proposed-authority.json) retains eight separately named cases with all per-table costs; [Native-parity closed schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/iso16757-independent-complete-native-parity-semantic-extent-proposed-schema.json) fixes that exact corpus. These are proposed neutral authorities, not mounted production fixtures. No schema-validator gate was run.

The initial default-project Nx attempt stopped before measurement on the existing repo/repo-lib dependency cycle. A selected-project inline attempt reached execution but Nx shell argument reconstruction rejected inline quote syntax. Both produced zero observations. The final allowed ticket script avoids that shell reconstruction and both successful runs produced actual observations. No repository/provider files were changed.
