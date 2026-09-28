# CSV, TSV, and ZIP Checkpoint Audit

**Ticket:** `26/09/26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE`  
**Date:** 2026-09-28  
**Method:** read-only source audit of `✏️s/🔌️plugins/🗄️stdio`; no source changes, Cargo jobs, or TypeScript jobs were started.

## P1 — Changing a legacy CP437 archive comment can make the ZIP edit unsaveable

The ZIP editor accepts any UTF-8 archive-comment text, but the emitted mutation cannot select a new archive-comment encoding. A source ZIP whose EOCD comment was decoded with `comment_utf8 = false` therefore retains CP437 encoding after an edit. Replacing that comment with a character absent from CP437, such as `🎒`, reaches ZIP serialization and fails.

Static call chain:

1. `decode_best_effort_text` selects CP437 fallback and persists `comment_utf8 = false` for a non-UTF-8 EOCD comment in [ZIP I/O](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs:232).
2. The Rust editor checks only the UTF-8 byte length and emits a string-only `SetArchiveComment` at [lines 42–59](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🦀️.rs:42). The TypeScript retained editor has the same string-only output at [lines 78–95](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🟦️.ts:78).
3. `SetArchiveComment` contains only `comment` in [the mutation leaf](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💬set-archive-comment/🦀️.rs:12). Its diff changes `comment` while leaving `comment_utf8` unset at [lines 285–287](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:285), and the inverse similarly restores only text at [lines 124–127](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:124).
4. The retained post-copy explicitly assigns `source.comment_utf8` after copying the changed comment at [lines 463–475](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:463), then validates the result at [lines 166–169](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:166).
5. That validator encodes with CP437 when the persisted flag is false; an unmappable character returns `ZipError::Utf8` ([encoding](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs:206), [validation](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs:723)).

This is a current user-visible editor path, rather than a schema-only concern. The existing rich codec fixture proves that CP437 archive comments are preserved on the wire ([lines 369–371](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs:369)), but it does not replace one.

**Required correction:** make the target encoding an explicit part of the archive-comment mutation, its diff, retained post-copy, and inverse. The native and TypeScript editors must choose a serializable encoding for the replacement; promotion to UTF-8 is required when CP437 cannot represent it. The inverse must restore both the original comment and original encoding. Add Rust mutation/preparation/codec proof and TypeScript emission parity for a CP437 source comment changed to a Unicode replacement, including save-and-reopen equality.

## TSV — Current correction is source-consistent; runtime verification remains outstanding

The prior checkpoint's first-record-as-header problem has been corrected in the current shared checkout. The renderer now creates synthetic column labels and renders every record as a data row at [lines 34–80](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:34); it also removes the unsupported `set-header` action at [lines 18–21](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:18). The reducer maps cell and row ordinals directly to `records`, and structural operations no longer reserve row zero ([lines 300–339](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs:300)).

The new focused render tests cover a one-record TSV and reject `set-header` ([lines 15–34](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs:15)); structural reducer coverage is present at [lines 173–197](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:173). These are source observations only: no compilation or retained UI dispatch was run during this audit.

## P1 — WGPU still falls back from childless table-row activation to its first action

The WGPU projection's childless `TableRow` branch turns activation into the first `row_actions` binding when the record has no explicit activation. The same first action selects the row icon at [lines 1051–1063](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1051). If that first action is `remove-row`, activating the row dispatches deletion.

The current CSV/TSV editable-row shape has declarative cell children, so it follows the separate stack branch, which retains only an explicitly declared row activation ([lines 1039–1050](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1039)). The current regression proves that childful case does not activate `Remove row` ([lines 35–77](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs:35)). The fallback nevertheless remains in current source and is unsafe for any childless table row with a destructive first action.

**Required correction:** remove the first-row-action activation fallback. Project every action as an explicit, labelled, focusable trailing control, and add a childless-row law that proves activation does not dispatch `remove-row`. This framework correction is owned by the table/UI workstream; no source was changed by this audit.

## CSV and ZIP metadata preservation — no additional immediate static blocker found

CSV's TableRow reducer consistently accounts for the optional header in cell and row translation, including the no-header to header transition at [lines 279–378](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs:279). I found no second concrete behavior failure in the current CSV source.

ZIP decoding captures entry serialization metadata, and retained preparation copies local and central extra fields, legacy-name/comment byte payloads, central comments, timestamp and attribute scalars, compression method, and data-descriptor signature ([copy](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:547), [finish](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:656)). I found no additional immediate metadata loss in that static trace. This conclusion excludes the archive-comment encoding blocker above.

## P2 — Proto declarations do not encode the Rust/TypeScript semantic defaults

The ZIP model defaults compression to Deflate, sets UTF-8 flags and DOS date fields, and defaults `comment_utf8` to true ([Rust defaults](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:33)). The snapshot and artifact Proto3 declarations give omitted scalars Proto3 zero values instead ([snapshot declaration](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto:15), [artifact declaration](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🛰️.proto:3)).

The scoped search found no ZIP protobuf decode/encode consumer or supported round-trip path beyond these declarations. This is therefore schema/default drift, not a demonstrated runtime data change, and should be addressed when a protobuf consumer is introduced or identified. At that point, define presence/default behavior explicitly and prove Rust/TypeScript/protobuf parity.

## Verification record

`git diff --check` over the CSV, TSV, ZIP, and inspected WGPU source completed with no diagnostics. No Cargo, Bun, Nx, or TypeScript test command was run, so this audit makes no compile or runtime-pass claim.
