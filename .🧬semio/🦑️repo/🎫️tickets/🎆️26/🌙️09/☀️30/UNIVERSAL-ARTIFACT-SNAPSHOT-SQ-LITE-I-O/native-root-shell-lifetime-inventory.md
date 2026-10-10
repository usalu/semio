# Native Root Shell Lifetime Inventory

Read-only current source inventory; no builds or production changes. Root is correcting terminal pending-child Box copy demand separately.

Store `ArtifactSqliteSnapshot::sqlite_codec` and `IoRunControl` borrow caller native controls. They do not allocate a separate root recipient Box. `IoRunControl::{snapshot_decode,snapshot_encode}` reborrow the original native direction and cumulative grant/progress; borrowed owner Drop writes the accepted progress back. A stack recipient becoming terminal and leaving scope does not free a heap recipient allocation, so it must not receive an invented Box-release receipt.

The actual production boxed recipient container located in this scope is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs`:

- `SnapshotOperation` embeds its Decode/Encode recipient shells, detached forwarded receipt, and controlled returned output. Its Box birth is checked against the original frame capacity and reserved through the real forwarded operation allocation port.
- `SnapshotOperation::status` quotes final `size_of::<Option<Box<Self>>>()` copied bytes, `size_of::<Self>()` released bytes, one item, and depth one after both native/output owners become terminal.
- `snapshot_close` prechecks that exact final grant, calls the original operation finish port, and only then drops the original slot. The actual wrapper release is reported. No missing final shell-release pricing was found in this branch. Its fields' embedded recipient shells are part of this wrapper allocation, not separately allocated boxes.
- The same-instance thread-local slot admits only one live operation and returns its original ticket/demands. This is the existing explicit pending owner, not a new unpriced global result registry.

Remaining original ownership frontiers are outside that final wrapper branch:

| Path/function | Live owners after fallible work | Current unqualified frontier |
|---|---|---|
| Store `sqlite_codec` export/import | decoded/reconstructed typed snapshot, SQL database, native payload, validation diagnostics | Old OwnedSqliteSnapshot Drop invokes synchronous retirement; returned DB/payload/diagnostics remain ordinary locals. Root's typed export receiving frame is the current repair. Import's by-value database admission remains a separate caller-slot change. |
| OS `🚪️io/🦀️.rs:2574` run_snapshot_hop export | incoming payload, returned projected DB/diagnostics, metadata, emitted file | After provider export returns, schema/database validation and metadata/file writing can refuse while those owners are ordinary locals. The provider's inner frame does not retain the entire hop. |
| Same run_snapshot_hop import | input file bytes, decoded DB, extracted dialect/encoding metadata, returned payload/diagnostics | Database parsing/metadata validation precede provider adoption; by-value provider import and post-import size refusal still own locals outside a receiving frame. |
| OS typed io_export_sqlite_snapshot:2549 / io_import_sqlite_snapshot:2562 | projected/parsed DB, metadata diagnostics, reconstructed P or emitted bytes | APIs take SQL limits/progress only and call direct to/from_sqlite_database. There is no supplied native recipient/full grant or controlled result custodian. Generic codec repair does not repair these distinct APIs. |
| Probe workspace `🚪️io/🔤️json/🦀️.rs:78,90` plain parse/print | stack recipient, native receipt, local result | Existing 100000-turn closure and early `?` on close error can exit with a live original recipient. No heap root shell was allocated here, but pending owner lifetime still needs a genuine session/caller or proved finite funded synchronous closure. |

The local Value detached continuation laws genuinely allocate a recipient Box under native charge and release that exact original Box using `release_recipient_box`; that physical claim applies to those selected laws. It does not imply that stack recipient users need the same Box API or that the whole Store/OS hop already has that ownership boundary.

Additional admission point to review when adopting the whole component hop: `with_snapshot_authority` takes SnapshotInput by value before checking occupied slot/frame authority. Original input can therefore leave caller custody on pre-admission refusal. Frame birth is forwarded/charged, but the nested IoRunControl currently receives the full original grant while its native maximum is reduced by frame bytes; ensure the frame item/capacity/depth costs also debit the cumulative physical wallet rather than only the allocation ceiling. No runtime conservation failure is asserted without the selected law.
