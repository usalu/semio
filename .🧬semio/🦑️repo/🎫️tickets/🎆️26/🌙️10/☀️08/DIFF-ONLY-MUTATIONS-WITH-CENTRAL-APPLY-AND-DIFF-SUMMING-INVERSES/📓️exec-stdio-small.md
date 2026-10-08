# 📓️ exec-stdio-small

Scope: the 26 small stdio artifacts under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` plus the stdio-wide contract (`📇️registry/🧬️contract`).
Status 2026-10-08: everything below is WRITTEN BY HAND; compile/test status is in "Verification" (nothing was test-run; coordinator ruling "cargo check only").

## Shared contract (`📇️registry/🧬️contract`)
- `snapshot_patch_leaf!` macro DELETED (zero users repo-wide). `snapshot_edit_patch` + `snapshot_patch_*` helpers STAY: `📕️xlsx` (stdio-mid) still calls `snapshot_edit_patch` in its editor (2 sites). Delete them when xlsx is converted.
- Added `snapshot_edit_net_exact` / `net_leaves_exact` (editor snapshot edit -> concrete leaves via the artifact's `net_mutations(base, next)`, replayed through the central applier; an edit no leaf addresses is refused as `snapshot-edit.unaddressed`, never dropped).
- Added `import_media_as_load::<E>(port, media)` (natural-file open = genesis/load, replaces every `whole_document_operation -> SetSnapshot`). Framework gap: the framework's natural-file import job still calls `E::whole_document_operation`; artifacts that override only `import_media` keep working through the media path only.

## Per artifact (kinds, before -> after)
Common "before": every artifact carried `set-snapshot` (V1-SNAPSHOT-DIFF + V2-RESTORE-INVERSE) and `patch-snapshot` (same + V3), a per-artifact `agg_diff`/`agg_inverse` pair, and inverses of the form `between(apply(d, base), base)` or `SetSnapshot(base)`.
Common "after": both snapshot leaves and `agg_*` are deleted; each kind builds its sparse diff declaratively in its leaf, its inverse is concrete from `base` (absolute setters, original index for removes/moves per the wave-2 ruling), diff-level `DiffAlgebra::inverse` is concrete (no `apply`+`between`), the editor lowers snapshot edits through `net_mutations` + `snapshot_edit_net_exact`, and every artifact has a `*_mutation_inverse_sum_law_holds_for_every_leaf` test calling `assert_mutation_inverse_sum_law` (fixtures delete/insert/move a MIDDLE row).

| artifact | kinds | notes |
|---|---|---|
| ☁️las | set-point insert-point remove-point set-system-identifier set-scale-and-offset insert-vlr remove-vlr set-bounds set-points-by-return set-version set-creation-date set-vlr-data set-software-info | agg removed, indexed inverses |
| 🌐️html | set-raw-text set-text insert-node remove-node set-element-name set-comment set-doctype set-attribute | `html_net_mutations` answers nothing for unreachable replacements (refused) |
| 🌦️epw | 11 kinds (records, header blocks) | `inverse_records`, per-record `inverse` |
| 🎒️zip | base: set-entry-data add-entry remove-entry rename-entry set-archive-comment; iso21320: + add-stored/deflated-entry | agg removed, entry-keyed inverses |
| 🎥️mp4 | insert-track remove-track set-sample-sync set-track-codec set-movie(NEW) set-ftyp set-track-dimensions remove-sample insert-sample | `set-movie` added (binary tag 9) because the movie header had no concrete leaf; indexed inverses |
| 🎵️mp3 | set-frames set-id3v2 set-id3v1 | entity-replace kinds; oracle KINDS 3 |
| 💬️bcf | topics/comments/viewpoints insert/remove/set, set-version, set-topic-markup, ... | diff redesigned name-keyed -> index-keyed (`IndexedDiff`); insert kinds carry `index: Option<usize>` |
| 💾️binary | replace-byte-range append-bytes truncate-at | `absorb_splices` read and checked: touching splices fuse (RLE of the label simulation), inverse emits one shifted splice per forward splice, so Σ(inverse leaves) and `BinaryDiff::inverse` share the normal form (sum-law test present) |
| 📊️csv / 📑️tsv / 🔤️txt | set-field/-cell/-line, insert/remove row/record/line, header/newline/line-ending | csv/tsv structural editor commands emit remove+insert row pairs |
| 📝️md | set-inlines insert-block remove-block replace-block | nested-list paths in the law test |
| 📰️xml / 🧾️json | base+valid / base+i-json | `Restore(DiffAlgebra::inverse(..))` replaced by concrete inverses; xml `set-attribute`, json `set-member`, i-json `upsert-member` carry `index: Option<usize>` |
| 📼️avi | set-main-header, streams, chunks, unknown chunks | indexed triples |
| 🔺️stl | set-solid-name insert/remove-triangle set-triangle-normal/-vertices | |
| 🧱️ply | set-format, rows, elements, comments | comments became `IndexedDiff<String,String>` |
| 🗽️obj | 21 geometry kinds | `set-group`/`set-object` carry `index`; `remove-face` keeps a state-no-op multi-step inverse |
| 🖋️dxf | header vars, layers, styles, linetypes, blocks, entities | `set-header-var` carries `index` |
| 🖊️dwg (ac1024+ac1018) | set-version-info (only kind) | `set-snapshot`/`patch-snapshot` deleted; the proprietary container beyond the preamble has no leaf (oracle: "nothing beyond the preamble is expressible as fields"); `net_mutations` answers only the preamble triple. History fixtures rewritten to `setVersionInfo`. |
| 🖼️tiff document | insert-ifd remove-ifd replace-tag remove-tag paint-region replace-samples(NEW) | `TiffIfdDiff` gained sparse `runs` (`TiffSampleRun{block,offset,samples}`); `paint-region` no longer clones+mutates: its diff is the maximal differing runs and its inverse is `replace-samples` per run; diff-level inverse concrete (index transport over IFDs, tag-keyed, run restore); TS facets (`⚙️operations`, diff `🟦️.ts`, schemas) updated |
| 🖼️tiff baseline | set-photometric-interpretation set-bits-per-sample | module rewritten (it was codemod-corrupted); refuses when the page lacks the tag as SHORT words (inverse must exist); stale 7-kind unit test replaced |
| 🔊️wav | set-fmt set-data patch-data set-other-chunks | `WavDiff.data_splices` (sequential `WavSplice`, concatenation absorb) makes `patch-data` apply-free; `set-other-chunks` gained optional `chunkOrder` so its inverse is exact; serialization validation stays in the leaves (candidate built by struct update, not by applying the diff); `sparse_against` drops fields the base already carries |
| 🗜️deflate | set-compression-params set-preset-dictionary set-payload | |
| 📷️png / 🪟️bmp | replace-image change-gamma patch-pixels paint-native-samples / replace-image paint-indexed-region paint-direct-region | converted concurrently by another actor to the owned-image model (inverse = `replace-image(base.image)`); I only cleaned leftovers (ksy, feature prose) and added sum-law tests. See open issues. |
| 📸️jpg | change-jfif-header insert/remove-other-segment replace-pixels (+ replace-image from the concurrent actor) | leaf inverses made concrete (weak guards removed); see open issues |

## Verification
- `rustfmt --edition 2021` parse of every `.rs` touched in the last 10 h under the 26 artifacts: no syntax error.
- `cargo check` (lib, wasm32-wasip2): see the final line of this section.
- No test was run (coordinator: no test builds). Law/unit tests exist but are WRITTEN BUT UNRUN; exact commands once the framework compiles:
  `cd <artifact> && "$T/🚦️gate.sh" stdio-small -- cargo test -p semio-s-artifact-stdio-<name> --lib mutation_inverse_sum_law` (and `net_mutations_replay`).

## Open issues
1. png / bmp: paint (`patch-pixels`, `paint-native-samples`, `paint-indexed-region`, `paint-direct-region`) and `change-gamma` still produce a whole-image `image` replacement diff (`PngDiff { image }`, `BmpDiff { image }`) and invert through `replace-image(base.image)`. Sparse sample runs like tiff's would need new diff fields, canonical-JSON, retirement and publication changes inside another actor's freshly landed model (`RetireOwned`/`RetainedClone` plumbing), so I left it. The law holds (`{image: Some(base)}` both sides) but the diff is not minimal.
2. jpg: the document subset is mid-migration by another actor (snapshot gained `image: JpgImage`, `replace-image` added, `JpgDiff` still reads `base.width`/`base.pixels` and its `inverse` is still `between(apply(d), base)`); the leaf inverses are concrete, but `JpgDiff::inverse` and the sum-law test remain to be written once that diff stabilises.
3. dwg: only the preamble triple is mutable by design; summary/application/... edits in the snapshot-details editor are refused instead of patched.
4. xml-valid `declare-doctype`/`set-standalone` have no in-vocabulary inverse for the materialisation cases; i-json `set-top-level` has none for a scalar-root base.
5. bcf `parts`, avi `hdrl_extra`/`strl_extra`, dxf `other_tables`, ply element `count`-only changes, wav pad bytes and tiff schema stamp are unaddressable by `net_mutations` (edits are refused, not dropped).
6. Collection-valued setters kept as entity-replace kinds (obj set-usemtl/smoothing/unknown, mp3 set-frames, ...), accepted by the wave-2 ruling.
7. tiff editor config (`TiffEditorConfigMutation`) is already sparse (`config_diff!`) with an absolute inverse; its hand `impl Mutation` is kept (single leaf).
8. `.protocol.semio` tag documentation for mp4 `set-movie` and tiff `replace-samples` follows the declared `binaryTag`s (9 and 8); derived numbering was not re-run.
9. The framework natural-file import job still calls `E::whole_document_operation`; see "Shared contract".
10. Shared-build-dir lock cycle: several idle `cargo check`s (mine and two peers') were killed per the documented remedy to unblock the queue.
