# Current public framework preflight hook and observer audit

Read-only source audit. No tests/compiler/edits. Source pair receipts27/27 and parser receipts are supplied by execution report; Native candidate12/12/16 remains unexecuted in this audit. Production preflight overrides remain absent in actual SpaceSnapshot, CollectionSnapshot and SpaceHistorySnapshot implementations.

## Actual capability and erased boundary distinction

Store trait default `🏪️store/🦀️.rs:11180–11183` performs EncodeNative checkpoint then returns typed UnsupportedOwner with owned message. Cancellation at that checkpoint returns Canceled instead. All three actual owner provider roots omit this hook; current ordinary public hook calls therefore select the default, not an unmounted draft.

Erased public codec export11200–11211 calls controlled native decode, typed to_database and check_database. Import11212–11227 calls from_database, semantic subset validation and controlled native encode. Neither invokes preflight_sqlite_snapshot_encoding. Targeted framework IO/Store scan found only trait definition; no automatic public erased call was found. Hence default hook absence directly proves missing explicit preflight capability, not that all erased public IO fails. Existing successful native output producers or History15 receipts remain compatible with that missing hook. Pairing an override alone does not newly integrate an erased preflight consumer.

## Test owner and exact scope

Space/Collection actual snapshot SQLite test files each contain12 prefixed laws; broad root scan's extra sqlite_snapshot_codec method is not a test. History candidate16 preserves existing14 fixture laws plus two cleanup/native laws. Selection must be measured by Root at existing @semio-tech/framework-space-space-rs:test-snapshot-sqlite-native, framework-space-collection-rs equivalent and framework-os-kernel:test-space-history-sqlite-native. Full typed owners, complete u64 sizes/ordered children/dictionary state/History clock words remain unchanged. New hooks are exercised directly on those actual owners in Text and Binary.

Shared helper `🪐️space/🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs:7–81` prepares actual native output and reconstructs full typed equality outside preflight observation. This is a whole borrowed-hook contract, not an encoded-output allocator test. Same-shape long/short owners must request equal scratch, while no copied payload/output authority is permitted. Conservative forecast may exceed actual encoded bytes; file-minus-one is a refusal bound and no exact-file-success guarantee is imposed. Physical scratch debit and max_value semantic field bound remain independent. Flat scans allocating zero scratch correctly succeed with zero allowance and skip positive scratch minus-one/cumulative exhaustion.

## Successful, failed and canceled observation

Successful long, short and exact scratch hooks are fully System-request observed with callback/control outside. Exact ledger=requests; successful scratch has dropped by hook return. Positive-scratch minus-one is separately observed and asserts observed requests=admitted scratch+actual returned error.message.capacity(). This is an explicit error-owner capacity relation, not guessed bytes/subtraction. It assumes current error uses one actual String owner allocation; any extra temporary diagnostic/failed reserve requests will correctly fail that law. It is not proof every failed result fits physical allowance including error memory: refused error is expressly outside scratch debit.

Cumulative two calls plus exhausted third enforce no scratch retirement refund, but are not independently System-observed. Other file/value/row/schema denial branches assert exact typed categories outside observer. Start cancellation observes only returned error message capacity and unchanged full scratch allowance. Real interior Unicode scan cancellation asserts reached phase/completed frontier and full unchanged owner, but occurs outside System observer: interior failed request/debit relation and release are unverified.

Observer type returns result+requested bytes only. There is no released-layout capacity census here, so completed hook drop does not prove no leak. No actual source override currently exists to inspect for a callback/allocation escape or numerical failure; the authentic next assertion RED should be default UnsupportedOwner rather than credit tuning. Current shared schema/Hasher accounting remains separately held and must not be inferred fixed from preflight design.

No concrete current source prerequisite identified in mounted law signatures/include paths after supplied final six-file pair. Compiler might still reveal a requirement. Preserve exact whole owner comparison and independent closed Ajv/BunSQLite UTF8/row/schema proofs; neither proves native runtime or compiled guest/live IO.
