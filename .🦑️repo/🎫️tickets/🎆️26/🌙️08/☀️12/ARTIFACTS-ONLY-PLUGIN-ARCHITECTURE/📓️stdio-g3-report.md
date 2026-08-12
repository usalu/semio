# stdio g3 (tiff gif jpg png bmp svg) — declarative conversion report

## Group note
Highly uniform group as expected: tiff/jpg/png/svg/bmp share one identical `register()` shape
(schema descriptor → inferences → pilot languages → \[schema_specs\] → document codec →
\[baseline/tiny/basic subset validator(s)\]), and gif duplicates that shape across two standards
(87a legacy + 89a canonical). None were byte-for-byte character-identical (each has its own
type names / dialect strings / grammar module paths), but jpg/png/svg/tiff's `register()` bodies
are structurally identical modulo those substitutions — same call order, same fields.

## Converted (4): tiff, jpg, png, svg
Added `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` at each artifact root
(`🗿️artifacts/<x>/🦀️component.rs`), covering schema + inferences + composers (`.composers(...)`,
fully qualified to `standards::vX::…::engine::io_registry::entries()` — never the bare/local
shadowing `io_registry` module) + languages (`.languages(pilot_languages())`, 5-role
Document/Ops/Diff/Pack/Spr moved verbatim from each engine's `register_pilot_languages`) +
`.document_codec_bare::<Snapshot, Mutation>(...)` (stdio is a headless library plugin, no apps) +
`.subset_validators(...)` for tiff (✳️baseline) and svg (✳️tiny + ✳️basic), re-derived via
`subset_validator_entry_of::<…Validator>()` since each subset's own cached `validator_entry()` fn
is private. png has no subset validator and no `register_schema_specs()` call — the cleanest of
the six. jpg's `register_schema_specs()` is dropped: verified its body is a real, unconditional
no-op `{}` (documented in-file: `JpgSnapshot`/`JpgDiff`/`JpgMutation` all fail `dsl`'s derive
machinery), so dropping it changes zero runtime behaviour.

Removed `crate::artifacts::{tiff,jpg,png,svg}::engine::register();` from
`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, added `.artifact(crate::artifacts::<x>::declaration())`
immediately after each one's `.artifact_kind(...)` in the builder chain. Only these 4 lines/pairs
touched at the shared plugin root — re-read the file immediately before each edit; found it
already mutated twice by siblings (json/xml/csv/md, then mp4/avi/mp3) between my reads, re-located
by content each time rather than by remembered line number. gif and bmp's `engine::register()`
lines were deliberately left untouched (see below).

Each artifact's own root-level shadowing `io_registry` module (returning `&[&ComposerEntry]`) is
left in place as orphaned dead code, matching `🔋️energy`/`🗒️note`'s own precedent for their
orphaned wrappers — not touched, not called from the new `declaration()`.

`⚙️engine` directories: zero edits anywhere in any of the six artifacts. `declaration()` only
*references* what `⚙️engine` already exposes (schema descriptor fns, inference descriptor fns,
`io_registry::entries()`, subset `Validator` structs) via fully-qualified paths.

## Left imperative (2): gif, bmp — genuine mechanism gap, not invented around
Both standards of gif (87a, 89a) and bmp's single standard call the real
`dsl::registry::register_schema_spec(...)` inside their `register()` (non-wasm branch) —
`ArtifactDeclaration` has no field for the `dsl` `FullResolver` insertion registry (only
`.languages()` covers `dsl::register_language`, a different registry). Per instructions: did not
invent a field, did not drop the call — left gif and bmp's `crate::artifacts::{gif,bmp}::engine::
register();` lines and `.artifact_kind(...)` calls exactly as found at the stdio plugin root, no
`declaration()` added for either. Field that would cover it: something like
`.schema_specs(&'static [(&'static str, fn() -> dsl::registry::RecordSpec)])` threading through to
`dsl::registry::register_schema_spec` — not invented here per instructions.

## Verify
- `grep -rn "fn declaration" 🗿️artifacts/<a>` — present at artifact root for tiff/jpg/png/svg;
  absent for gif/bmp (intentional).
- Plugin root: my 4 `register();` lines removed and replaced by 4 `.artifact(...)` calls; gif/bmp
  `engine::register()` lines and both artifacts' `.artifact_kind(...)` calls untouched.
- `grep -rn "io_registry::entries"` across all 6 artifacts — every real call site fully qualified
  through `standards::vX::…::engine::io_registry::entries()`; grepped the whole repo for a bare
  (non-`::`-prefixed, non-comment) `io_registry::entries()` call — **0** matches, same as the
  ticket's stated baseline.
- `#[path]` entries in `📦️glue.rs`: file never touched by this pass; all 111 tiff/jpg/png/svg
  references in it are pre-existing and unaffected.
- `RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-stdio --all-targets`
  → **GREEN**: `Finished \`dev\` profile [unoptimized] target(s) in 1.20s`, exit 0, 0 `^error` lines
  (695+787 warnings, all pre-existing/unrelated dead-code and unused-import lints across other
  artifacts, none touching tiff/jpg/png/svg/gif/bmp declaration code). Full log:
  `scratch-stdio-g3-check-final.txt` in this ticket folder.

No errors encountered this pass — nothing to attribute upstream.
