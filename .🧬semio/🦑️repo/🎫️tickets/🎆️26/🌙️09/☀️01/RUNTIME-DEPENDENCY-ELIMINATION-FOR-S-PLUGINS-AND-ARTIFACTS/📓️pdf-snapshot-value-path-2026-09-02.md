# `PdfSnapshot` first-party value path — 2026-09-02

## Scope and discovery

The public `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot` re-export resolves to the PDF 1.7
base schema at:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs`.

Before this slice it used `value_derive::ToValue` and `value_derive::FromValue`. As with the
`MeshData` precedent, using the derive is the wrong dependency direction for a public artifact
value boundary because the derive expands through the OS-kernel path. The replacement is therefore
hand-written directly against `pack::value`; this slice adds no dependency on
`semio-framework-value-derive` or `semio-framework-os-kernel`.

The repository goal resource could not be read because the configured `repo` MCP server failed its
initialization handshake. Work continued inside the explicitly named existing ticket; the ticket
was neither reopened nor closed.

## Implementation

- Replaced `PdfSnapshot`'s derived value codecs with hand-written `pack::value::ToValue` and
  `pack::value::FromValue` implementations.
- `ToValue` emits exactly the previous camelCase object shape: `schema`, `declaredVersion`,
  `pages`, `info`, `objects`, and `trailer`.
- `FromValue` requires `schema`, decodes the same camelCase keys, preserves the former
  field-level `#[value(default)]` behavior for every other field, and attaches the field name to
  nested `ValueError` paths. In particular, an omitted `declaredVersion` remains `String::default()`
  (empty), matching the old field attribute rather than `PdfSnapshot::default()`'s authored
  `"1.7"` value.
- Added four `#[cfg(test)]` regression tests: populated round-trip, missing-field defaults,
  required-schema diagnostics, and a camelCase JSON-shape comparison through `serde_json` as the
  third-party oracle. No production serde use was added. Stdio's manifest still has broader
  production serde dependencies for unrelated, unfinished conversions already documented in the
  ticket, so moving those dependencies is outside this slice.

## Consumer conversions

The four named animate/shooting files already had concurrent unstaged edits replacing the failing
serde calls with direct `dsl::ToValue`/`dsl::FromValue` conversion when the baseline was captured.
This slice preserved that work and aligned the final form with the established os-flow precedent:

- encoding uses infallible `dsl::json::to_json_string` (the OS-kernel re-export of
  `pack::json::to_json_string`), with the dead serde encoding error branch removed;
- decoding uses `dsl::json::from_json_str`, mapping only the real `ValueError` decode failure into
  the surrounding `IoError`/`TextError`.

Files:

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs`

## Verification

Both snapshots used the requested warm isolated target and an empty compiler wrapper:

```sh
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target2
export RUSTC_WRAPPER=""
cargo check -p semio-s-plugin-stdio -p semio-s-plugin-animate -p semio-s-plugin-shooting --message-format short
```

Errors were counted exactly with `grep -cE ': error(\[|:)'`:

| Measure | Before | After |
|---|---:|---:|
| All errors in the three-package command | 637 | 637 |
| `PdfSnapshot: serde::Serialize` / `DeserializeOwned` trait-bound errors | 0 | 0 |
| Errors located in the four named PDF consumer files | 2 | 2 |

The zero baseline count for the serde trait bounds is intentional and real, not reconstructed from
`HEAD`: the concurrent call-site conversion was already present before the baseline run. The two
remaining errors in the named files are both unrelated async-convention churn in shooting:

- export line 14 applies `?` to an `impl Future<Output = Result<PdfSnapshot, TextError>>`;
- import line 15 returns a future where `Result<ShootingSnapshot, TextError>` is expected.

The final check compiled `semio-s-plugin-stdio` successfully and reported no warning or error in the
`PdfSnapshot` source file. Animate and shooting still fail with the same 637-error aggregate from
concurrent repository churn, including missing mutation submodules, async `found future`/`not a
future` mismatches, and UI-builder API mismatches. None of those were changed in this slice.

A focused `cargo test -p semio-s-plugin-stdio pdf_snapshot_value_tests --lib` was also attempted in
the same isolated target. The crate-wide `#[cfg(test)]` build failed before running the filter due to
unrelated existing test-build failures, beginning with TIFF's `_`-in-expression error and followed
by many removed-serde oracle sites elsewhere in stdio. The run was cancelled after rustc had
established those blockers; no claim is made that the four new tests ran or passed.

## Files changed by this slice

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs`
- the four consumer files listed above
- this report

No commit, stash, checkout, worktree, ticket-state mutation, or goal-state mutation was performed.
