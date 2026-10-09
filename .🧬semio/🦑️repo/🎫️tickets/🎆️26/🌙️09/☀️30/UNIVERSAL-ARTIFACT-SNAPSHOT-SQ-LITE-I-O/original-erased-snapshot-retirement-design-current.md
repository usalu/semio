# Original Erased Snapshot Retirement Design

Read-only source audit; no tests or runtime qualification.

Store `🦀️.rs:12257–12262` decodes P, wraps OwnedSqliteSnapshot, projects SQL, validates and returns database. Import12273–12278 reconstructs P, validates, encodes, returns payload. Actual wrapper `📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs:18–23` drops P via nongranted retire_sqlite_snapshot. This does not retain snapshot custody when original physical close is refused.

## Genuine Existing Ports

NativeSnapshotDecodeOwner.receive and EncodeOwner.receive use original native allocation/callback/recipient plus cumulative five-axis wallet. IO `⏱️control/🛫️snapshot/🦀️.rs:106–139` pre-admits a concrete frame, holds intermediate and output separately, retires intermediate, and publishes output only after terminal-empty and granted frame release. Failure returns the actual frame to original recipient. This is the correct conservation pattern.

But NativeDecodeControl.with_retirement_owner at Value `🛬️decode/🦀️.rs:54–60` refuses a reserved or occupied sole slot. Calling P::decode_sqlite_snapshot_native inside another receive would nest the same reserved slot and fail for genuine existing P decoders. Do not resolve this with a second recipient/controller. A first-party receiving API needs to transfer successfully decoded P from the existing decode frame into its continuation, or admit a same-original successful-value custody frame before transferring P, returning P unchanged on every pre-admission failure. Existing receive has no such input-return error API: passing already owned P into a closure then failing frame admission would drop the captured P outside the recipient.

Project semantic SQL while P is immutably borrowed from that actual frame; retain P through SQL validation. Install completed output into the same frame before closing P. On projection/validation/close cancellation keep P plus any unpublished output in original custody. Preserve cumulative native allocation and five-axis receipts; neither a descriptive demand nor SQL maximum is authority. Whole successful output publishes only after P and auxiliary fields are terminal-empty, then funded receiving-frame release.

Import must likewise adopt its actual incoming SqliteDatabase before fallible reconstruction/validation so it is not dropped on refusal. from_sqlite_database returning only completed P remains an internal partial reconstruction frontier; original-control destination reconstruction is required for complete qualification.

## Real Output Types

IoPayload at `🚪️io/📦️payload/🧬️schema/🦀️.rs:9–13` has genuine RetireOwned derive and actual Text(String)/Binary(Vec<u8>) backing. It is eligible for typed output guard; its encoder local construction still needs retained partial production.

SqliteDatabase `🚪️io/🪶️sqlite-snapshot/🦀️.rs:58`, SqliteTable45, SqliteRow32, and SqliteValue declaration have no RetireOwned implementation found. Their actual nested table/name/sql/rows/value/string/blob fields need first-party explicit generated/declared retirement, with measured original child/scaffold costs. No blanket or opaque owner is legitimate.

IoOutcome<T> `🚪️io/🧬️schema/🦀️.rs:170–173` has actual Vec<Diagnostic> alongside value and no RetireOwned implementation found. Validation diagnostics are owned output too. Guarding only database/payload while a local diagnostics Vec drops on close denial is incomplete. Genuine Diagnostic backing and outcome owner must be supported, or diagnostics kept in an actual original frame field with real retirement.

## Necessary Contract

The narrow useful first-party contract is an already-owned input adoption operation that returns the same input on admission failure and thereafter retains it in the same original recipient through borrower projection and close, paired with a decoder continuation transfer to avoid occupied-slot nesting. Current drive_cursor also owns caller Option and pre-admits frame before taking it, but is Encode-only and loops granted work; its admission-before-take ordering is reusable, not a compatibility shim or new unrelated owner. P must genuinely implement supported RetireOwned; public codec bounds need to state that requirement. Output types and semantic projector destination ownership must be authored before declaring erased import/export qualified.
