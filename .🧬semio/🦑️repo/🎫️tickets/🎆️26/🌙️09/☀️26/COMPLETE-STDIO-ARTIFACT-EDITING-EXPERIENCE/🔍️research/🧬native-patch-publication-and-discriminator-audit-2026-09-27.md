# Native Patch Publication and Discriminator Audit — 2026-09-27

Read-only audit of the in-progress compact native edit path. This report does
not restate the known unmounted full-snapshot rollout gap. No Cargo job was
started: the shared Rust lanes remain owned by the active implementation work.

## P1 — The direct editor route bypasses compact schema identity and publication validation

**Affected paths**

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:881-896`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:330-346`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:406-468`

The retained snapshot-edit reducer is the only caller which prepares the
native compact patch, validates it against the selected dialect schema, fixes
the schema identity, and validates replayed publication. `ArtifactEditor::handle`
for an `EditSnapshot` command calls the format's `snapshot_edit_emit` directly.
That emitter can call `apply_snapshot_edit`, which only checks typed
conversion; it does not call `apply_snapshot_edit_for_dialect` or the compact
publication validator.

**Reproduction**

1. Start with `WavSnapshot::default()`, whose `schema` is `stdio.wav`.
2. Invoke the WAV editor's direct `ArtifactEditor::handle` with
   `EditSnapshot(SetValue { path: "/schema", value: "other" })`.
3. Its generic fallback at the WAV editor lines 204-208 accepts the typed
   string and its emitter publishes `SetSnapshot`.
4. The resulting snapshot has `schema: "other"`.

The exact same event sent to the registered retained tool is rejected at the
shared reducer's `apply_snapshot_patch_for_dialect`, which first requires the
registered schema then rejects an identity change. The two command routes
therefore disagree, and the direct route may persist an artifact under an
unregistered identity.

Route every `EditSnapshot` command, including direct app handling, through one
shared routine that performs compact preparation, dialect-fragment validation,
and exact publication admission before it delegates to a format emitter.

## P1 — WAV exposes a variant discriminator edit that the native emitter silently discards

**Affected paths**

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:33-40`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:150-208`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:222-243`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs:537-546,1005-1028,1167-1177`

`WavData` is adjacently tagged by `kind` and `value`. The WAV details provider
special-cases only sample values (`/data/value/<index>`), so its metadata
provider exposes `/data/kind` as an editable string control. The direct patch
recognizer handles only `/data/value/<index>`. Its fallback deliberately
replaces the full `data` payload with an empty variant, performs a generic
edit, and then restores the original data. Editing `kind` therefore changes
the temporary empty value and is overwritten before emission.

**Reproduction**

1. Start with `WavSnapshot { data: WavData::Raw(vec![1, 2]), ..Default::default() }`.
2. Dispatch `SetValue { path: "/data/kind", value: "pcm8" }` through the
   direct editor route.
3. The fallback accepts the edit on `Raw([])`, restores `Raw([1, 2])`, and the
   emitter selects `SetSnapshot` with that unchanged data because its branch
   comparison at lines 379-385 sees no data difference.
4. The command reports a mutation but leaves the discriminator `raw`.

The compact retained route calculates the expected typed result as
`Pcm8([1, 2])`, then rejects the unchanged emitted snapshot with
`snapshot-edit.publication-mismatch`. This is an observable direct-route
false-success and shows that a discriminated native record cannot safely
expose its tag as an independent generic scalar edit.

Make the `kind` field read-only in the details provider and require a complete
valid `/data` replacement for a variant change, or implement a native variant
replacement that validates and publishes the complete new `WavData` value.

## Follow-up observed during the audit

At the time of inspection, the shared publication helper encoded inverse
mutations only to check their item size; it did not replay them to prove that
they restore the pre-forward state. The root implementation owner confirmed a
live correction: it is adding reverse replay plus an encoded-but-no-op inverse
regression. This item is recorded for traceability and is not an additional
open finding after that correction lands.

## Paths inspected without an additional defect

- A move removes from a cloned intermediate snapshot before normalizing an
  append destination (`🩹️patch/🦀️.rs:146-161`). Its inverse captures values
  sequentially and reverses the resulting edits (`332-350`). This handles a
  same-array move where removal shifts the destination parent.
- Fragment paths are captured against each sequential native state, then
  validated against the final post-patch state (`277-305`). Required object
  fields, union/discriminator transitions, and non-local array constraints
  request a bounded post-edit frontier rather than silently validating only a
  leaf.
- The large-array fast path requires a plain homogeneous array with no
  `contains`, `uniqueItems`, tuple items, union, or conditional constraints.
  It asks the native source only for the post-edit length and enforces
  `minItems` and `maxItems` (`✅️validator/🦀️.rs:542-596`). More involved
  arrays fall back to a full bounded context, so the inspected branch did not
  reveal a shape-validation bypass.
