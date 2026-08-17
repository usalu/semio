# stdio g5 — declarative conversion of mp4, avi, mp3

## Converted (all 3)
All three artifacts fully converted, no imperative registration left for them.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🦀️component.rs` — added `declaration()`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🦀️component.rs` — added `declaration()`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🦀️component.rs` — added `declaration()`

Each `declaration()` does `.schema(...)` + `.inferences([...])` + `.composers(...)` +
`.document_codec_bare::<Snapshot, Mutation>(...)`, reproducing exactly what the removed
`register()` chain did (`schema descriptor registration` + `inference descriptor registration` +
`register_composer_entries` + `store::register_document_codec(store::ArtifactCodec::of::<..>())`).
None of the three had a `languages`/`formats`/`subset_validators`/`migrations`/`capability` call
anywhere in their register chain, so those builder methods were not needed.

`.composers(...)` for all three is fully qualified to the STANDARD-level engine `io_registry`
(e.g. `crate::artifacts::mp4::standards::isobmff::engine::io_registry::entries()`), which returns
`&'static [ComposerEntry]` — the type `.composers()` requires. The artifact root's OWN shadowing
`io_registry` module (unchanged, left in place) returns `&'static [&'static ComposerEntry]` and
would silently rebind/mistype if called bare — avoided by full qualification in all three files.

## Plugin root (`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`)
- Removed 3 lines: `crate::artifacts::{mp4,avi,mp3}::standards::<std>::engine::register();`
- Added `.artifact(crate::artifacts::{mp4,avi,mp3}::declaration())` immediately after each
  matching `.artifact_kind(...)` call (kept `.artifact_kind(...)` in place, per the `🗒️note`
  exemplar's shape).
- Edited only these 3 register lines + 3 new `.artifact(...)` lines; did not touch any sibling's
  lines. Re-read the file immediately before each edit.

## Left imperative
None. All 3 assigned artifacts converted; nothing in their register chains needed a field the
builder doesn't have.

## `⚙️engine` directories
Untouched — no moves, renames, or deletions inside any of the 3 artifacts' `⚙️engine` trees.
`declaration()` only ADDS a reference to what `engine::io_registry` already exposes.

## Silent-rebind check
`grep -rn "io_registry::entries"` across all 3 artifacts: 3 hits, all fully qualified
(`...::engine::io_registry::entries()`), zero bare calls.

## Verify — `cargo check -p semio-s-plugin-stdio --all-targets`
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-stdio --all-targets
Finished `dev` profile [unoptimized] target(s) in 2m 10s
```
Exit 0. `grep -c "^error"` on the full output: 0. 695/787 warnings are pre-existing, crate-wide
(unrelated identifiers, not mp4/avi/mp3/g5 lines) — not attributed to this change since they
predate it and touch files this session never opened.

Full log: `scratch-g5-check-1.txt` in this ticket folder.
