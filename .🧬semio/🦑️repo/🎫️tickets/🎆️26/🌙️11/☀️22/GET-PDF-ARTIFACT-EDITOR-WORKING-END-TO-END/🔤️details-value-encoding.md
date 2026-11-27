# Details value encoding for `setSnapshotValue`

## Verdict
The journey was wrong; the framework contract is correct and tested. No product change.

## Contract (evidence)
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/📨️action-bridge/🦀️.rs:77-83` (`snapshot_edit_event_from_action`):
`value` is a raw `any` value unless `valueEncoding == "json"`, in which case the text is decoded by `parse_snapshot_json` (invalid JSON yields a snapshot-edit fault via `snapshot_edit_fault`).
`valueEncoding` is a declared optional text arg (line 114) of `setSnapshotValue` / `insertSnapshotValue`.
Tests in `.../✏️editing/🧪️tests/🔬️unit/🦀️.rs`: `action_json_and_operation_codecs_preserve_typed_values` (json-encoded object decodes) and `chunked_rfc6901_paths_...` (no encoding: string `updated` stays the raw string).
Default is raw, not JSON; changing it would break those tests and the generated Details controls, which send `valueEncoding=json` explicitly.

## Root cause
The journey (`pdf-pane-journey.ts` `details`, line 241) typed `JSON.stringify(title)` into `value` without setting `valueEncoding=json`, so the quoted text was stored raw: `/Title("Details Title")`.

## Fix
Journey now types the raw title (lines 241 and 252). To type JSON instead, also fill `valueEncoding` with `json`.
Typing `42` without encoding yields the string "42"; with `valueEncoding=json` a number. Invalid JSON with `json` encoding is rejected with a fault, no raw fallback.

## Checks / wasm
No code changed outside the journey; no tests needed re-run, no wasm rebuild needed.
Limitation: the shell form has no per-arg JSON toggle; the encoding is a separate text field.
