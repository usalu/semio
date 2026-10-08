# 📋️ Audit: stdio small artifacts, diff-only mutations

Read-only audit against `📋️design.md`: laws L1–L5, codes V1–V4, plus the Rulings on disk (Minimality, Absorb soundness, Generic seams deleted, V5-ABSORB). No source file edited, no cargo/bun build, no git modifying command. Scripts and intermediate tables are in `🗑️generated/stdio-small-audit/` (to be swept at ticket close).

Scope: every `MutationKind` leaf under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` except 🧿️semio, 📖️pdf, 🧊️gltf, 📐️step, 📜️docx, 🎞️gif, 🏗️ifc, 🎨️svg, 📽️pptx, 📕️xlsx. Stdio outside `🗿️artifacts` holds no leaf kinds in scope; the only macro template is `📇️registry/🧬️contract/✏️editing/🦀️.rs:1389` (`snapshot_patch_leaf!`) and the editing macro at :1401.

## 1. Summary

- **265 kinds** in 26 artifact dirs (`🔣️.json` holds none). 241 are literal `impl … MutationKind<…>` sites (199 delegate to a per-artifact `agg_diff`/`agg_inverse` match, 42 are standalone leaves). 24 are macro-generated `patch-snapshot` kinds (`snapshot_patch_leaf!`), which a plain `impl MutationKind` grep misses. So the "≈240" in the request is 265 in practice.
- **113 kinds violate** at least one code, **11 are borderline** (no code, flagged for the `between(` gate), **141 are clean** at leaf level.
- Code hits (kinds): V1-SNAPSHOT-DIFF 61, V1-GENERIC-DIFF 49, V2-DIFF-DERIVED-INVERSE 11 strict plus 17 weak guard-only, V2-RESTORE-INVERSE 76, V2-EMPTY-INVERSE 0, V3-LEAF-APPLY 33, V3-HAND-MUTATION 1 (tiff editor config, not a MutationKind), V4-LAW-UNTESTED 0 raised (16 kinds lack kind-named test evidence, see §8).
- All 30 `set-snapshot` kinds (every in-scope artifact) are V1-SNAPSHOT-DIFF + V2-RESTORE-INVERSE: diff = `X::between(base, snapshot)`, inverse = `SetSnapshot(base.clone())`.
- Shared seams: `snapshot_patch_leaf!` (24 kinds) clones base, applies patch steps, then `between`; its inverse is a generic patch replay. The per-artifact `agg_diff`/`agg_inverse` (24 modules, 199 callers) hide per-variant logic behind a shared match outside each leaf's `↩️inverse/🦀️.rs` (L2 placement; no listed code).
- Diff-level: 39 sites in `🔺️diff` modules (`apply_*_diff_unchecked`, `Self::between(&applied, base)`) back `DiffAlgebra::inverse` in 17 artifacts. Not leaves, but the same clone-apply-difference shape (V1 and V3); see §4.
- `MutationDiff::apply` and `fn absorb` exist in all 26 diff types; `DiffAlgebra` (`inverse`/`between`/`is_empty`) in all 26.
- Hand-written `impl Mutation<P>` in scope: 1 (tiff editor config). The other `impl Mutation`/`MutationDiff` in 📇 are test-only (`🩹️patch/🧪️tests`).
- Seams still present (ruling: delete): `Restore(…)` inverse variants (xml-base 6 leaves, json-base 5 leaves); `SetSnapshot` inverse variants (30); 31 `apply_*(&mut …)` helpers (mutation mods and `⚙️operations`; apply_in_place shape); xml-base `apply_to(&mut Vec<XmlNode>)` in `🔺️diff`.

## 2. Per-artifact summary

Counts are kinds per code. "Clean" = no code and not borderline. Diff-level rows (§4) are not in these counts. "weak guard" is shown separately. V3-HAND-MUTATION (tiff editor config, not a kind) is listed in §5 and not counted in the table.

| Artifact | Kinds | V1-SNAPSHOT-DIFF | V1-GENERIC-DIFF | V2-DIFF-DERIVED-INVERSE | weak guard | V2-RESTORE-INVERSE | V2-EMPTY-INVERSE | V3-LEAF-APPLY | V3-HAND-MUTATION | V4-LAW-UNTESTED | Borderline | Clean | Kinds with a violation |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ☁️las | 15 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 1 | 12 | 2 |
| 🌐️html | 9 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 8 | 1 |
| 🌦️epw | 13 | 2 | 3 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 9 | 4 |
| 🎒️zip | 14 | 3 | 1 | 0 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 11 | 3 |
| 🎥️mp4 | 10 | 2 | 3 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 6 | 4 |
| 🎵️mp3 | 5 | 2 | 4 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 |
| 💬️bcf | 14 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 12 | 2 |
| 💾️binary | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 1 |
| 📊️csv | 6 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 4 | 2 |
| 📑️tsv | 7 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 5 | 2 |
| 📝️md | 5 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 4 | 1 |
| 📰️xml | 17 | 4 | 4 | 6 | 0 | 12 | 0 | 2 | 0 | 0 | 0 | 5 | 12 |
| 📷️png | 5 | 5 | 1 | 0 | 0 | 5 | 0 | 4 | 0 | 0 | 0 | 0 | 5 |
| 📸️jpg | 21 | 4 | 3 | 0 | 8 | 5 | 0 | 2 | 0 | 0 | 0 | 8 | 13 |
| 📼️avi | 13 | 2 | 4 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 8 | 5 |
| 🔊️wav | 6 | 2 | 4 | 0 | 0 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 6 |
| 🔤️txt | 6 | 1 | 0 | 0 | 5 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 6 |
| 🔺️stl | 7 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 5 | 2 |
| 🖊️dwg | 3 | 3 | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 3 |
| 🖋️dxf | 19 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 6 | 11 | 2 |
| 🖼️tiff | 11 | 5 | 3 | 0 | 4 | 5 | 0 | 3 | 0 | 0 | 0 | 1 | 10 |
| 🗜️deflate | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 1 |
| 🗽️obj | 22 | 2 | 6 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 4 | 11 | 7 |
| 🧱️ply | 10 | 2 | 1 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 8 | 2 |
| 🧾️json | 15 | 2 | 4 | 5 | 0 | 8 | 0 | 1 | 0 | 0 | 0 | 7 | 8 |
| 🪟️bmp | 4 | 4 | 1 | 0 | 0 | 4 | 0 | 3 | 0 | 0 | 0 | 0 | 4 |
| **Total** | **265** | **61** | **49** | **11** | **17** | **76** | **0** | **33** | **0** | **0** | **11** | **141** | **113** |

## 3. Shared generic helpers

| Helper | file:line | Callers (in scope) | Violation |
|---|---|---|---|
| `snapshot_patch_leaf!` macro | `📇️registry/🧬️contract/✏️editing/🦀️.rs:1389` | 24 leaves | V1-SNAPSHOT-DIFF (`diff` at :1406 = `between(base, apply_clone(base))`); V1-GENERIC-DIFF (JSON-pointer patch payload); V2-RESTORE-INVERSE (inverse at :1411 replays a generic patch); V3-LEAF-APPLY |
| `apply_snapshot_patch_checked` / `apply_snapshot_patch` / `apply_validated_snapshot_patch` | `📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs:600, 589, 691` | macro (24) | V3-LEAF-APPLY: `let mut next = snapshot.clone()` then `apply_step(&mut next, …)` |
| `inverse_snapshot_patches` (+ `_within`) | `📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs:751, 761` | macro (24) | V2-RESTORE-INVERSE (generic replay); V3 (`snapshot.clone()` + `apply_step` at :793–794) |
| `agg_diff` / `agg_inverse` (per-artifact mutations `🦀️.rs`) | see agg table below | 199 AGG kinds | No own code (L2 placement). The codes come from their arms (§5) |
| `diff_set_snapshot` (per-artifact `🔺️diff`) | 26 modules (xml-base, jpg, zip have 2 variants) | 1–2 each | V1-SNAPSHOT-DIFF (`X::between(base, next)`) |
| `apply_<art>_diff_unchecked` + `Self::between(&applied, base)` | `🔺️diff` of las, epw, zip, mp4, avi, txt, stl, dwg, dxf, jpg, binary, json, ply, csv, tsv, obj, tiff (see §4) | 1 per module (`DiffAlgebra::inverse`) | V1-SNAPSHOT-DIFF + V3-LEAF-APPLY (diff level) |
| `paint_*_controlled` (`⚙️operations`) | png ×3, bmp ×2, tiff ×1 (`paint_region/samples`) | 1 each | V1-SNAPSHOT-DIFF + V3-LEAF-APPLY (`let mut next = snapshot.clone()` then `&mut` writes; bmp `⚙️operations/🦀️.rs:73, 84`) |
| `value_diff_between` | json-base `🔺️diff/🦀️.rs:375` | 1 (set-member) | V1-GENERIC-DIFF (generic JSON value diff) |
| `diff_at_path` / `attribute_diff_at_path` | html `🔺️diff/🦀️.rs:148`, xml-base `🔺️diff/🦀️.rs:186`, md `🔺️diff/🦀️.rs:184`; html `mutations/🦀️.rs:111` | 6 / 6 / 4 / 1 | Clean (sparse typed path diff; `Vec::new()` is an empty list) |
| `contribute` | per leaf (not shared): jpg ×9, tiff ×4 | 1 each | Clean, except jpg `replace-pixels` (whole pixel array, V1-GENERIC-DIFF) |
| `apply_*_mutation(&mut P)` (mod-level) | 31 in-scope (mutation mods and `⚙️operations`; e.g. mp4 `mutations/🦀️.rs:71`) | tests and store glue | Seam (apply_in_place shape, ruling: delete). Calls `MutationDiff::apply` |
| `Restore(…)` variant | xml-base 6 leaves, json-base 5 leaves; diff arms `Self::Restore(diff) => diff.clone()` | 11 | V2-RESTORE-INVERSE (wraps a derived diff); seam (ruling) |
| `absorb_*` keyed helpers | `absorb_named` (bcf), `absorb_indexed` (mp4), `absorb_entries` (zip), `absorb_triangles` (stl), `absorb_splices` (binary), `absorb_ifds_opt` (tiff) | several | V5 review (§9) |

agg table (`agg_diff` line / `agg_inverse` line, callers): las 126/153 (14); html 140/160 (9); epw 126/151 (12); zip-iso21320 127/156 (7); zip-base 84/102 (6); mp4 92/116 (9); mp3 90/101 (4); bcf 179/224 (13); binary 72/88 (4); csv 86/104 (5); tsv 84/101 (6); md 125/144 (5); xml-valid 214/274 (8); jpg-baseline 185/250 (9); avi 80/103 (12); wav 76/93 (4); stl 97/114 (6); dwg 74/96 (2); dxf 201/250 (18); tiff-baseline 17/22 (3); deflate 74/84 (4); obj 235/287 (21); ply 126/159 (9); json-i-json 282/304 (9). Sum 199.

## 4. Diff types and trait coverage

All 26 artifacts: `impl MutationDiff<S> for D` has `fn apply` and `fn absorb`, and `impl DiffAlgebra<S>` is present.

`DiffAlgebra::inverse` class: **A** = `between(apply_unchecked(d, base), base)` (differencing a mutated clone); **B** = field-wise `base.x.clone()` restore, no between; **C** = walks the diff via an `inverse_*` helper.

| Artifact | Diff type (line of impl) | apply | absorb | DiffAlgebra | inverse class |
|---|---|---|---|---|---|
| ☁️las | `LasDiff` (`🎩️header/🔺️diff/🦀️.rs:739`) | ✓ | ✓ | ✓ | A (`apply_header_diff` clones header) |
| 🌐️html | `HtmlDiff` (`🔺️diff/🦀️.rs:158`) | ✓ | ✓ | ✓ | C (`inverse_node_diff`) |
| 🌦️epw | `EpwDiff` (`🔺️diff/🦀️.rs:203`) | ✓ | ✓ | ✓ | A |
| 🎒️zip | `ZipDiff` (`🧱️base/🔺️diff/🦀️.rs:162`) | ✓ | ✓ | ✓ | A |
| 🎥️mp4 | `Mp4Diff` (`🔺️diff/🦀️.rs:464`) | ✓ | ✓ | ✓ | A (`between` + `apply`, :506) |
| 🎵️mp3 | `Mp3Diff` (`🔺️diff/🦀️.rs:26`) | ✓ | ✓ | ✓ | B |
| 💬️bcf | `BcfDiff` (`🖊️markup/🔺️diff/🦀️.rs:327`) | ✓ | ✓ | ✓ | C (`inverse_named`) |
| 💾️binary | `BinaryDiff` (`🔺️diff/🦀️.rs:55`) | ✓ | ✓ (`absorb_splices`, §9) | ✓ | A |
| 📊️csv | `CsvDiff` (`🔺️diff/🦀️.rs:258`) | ✓ | ✓ | ✓ | A |
| 📑️tsv | `TsvDiff` (`🔺️diff/🦀️.rs:184`) | ✓ | ✓ | ✓ | A |
| 📝️md | `MdDiff` (`🔺️diff/🦀️.rs:237`) | ✓ | ✓ | ✓ | C (`inverse_blocks_diff`) |
| 📰️xml | `XmlDiff` (`🧱️base/🔺️diff/🦀️.rs:196`) | ✓ | ✓ | ✓ | C, plus `apply_to(&mut)` at :145 |
| 📷️png | `PngDiff` (`🔺️diff/🦀️.rs:17`) | ✓ | ✓ | ✓ | B |
| 📸️jpg | `JpgDiff` (`🧾️document/🔺️diff/🦀️.rs:912`) | ✓ | ✓ | ✓ | A |
| 📼️avi | `AviDiff` (`🎛️hdrl/🔺️diff/🦀️.rs:360`) | ✓ | ✓ | ✓ | A (between + base clone, :439) |
| 🔊️wav | `WavDiff` (`🔺️diff/🦀️.rs:25`) | ✓ | ✓ | ✓ | B (its `apply` helper is used by `patch-data`) |
| 🔤️txt | `TxtDiff` (`🔺️diff/🦀️.rs:243`) | ✓ | ✓ | ✓ | A (:316–320) |
| 🔺️stl | `StlDiff` (`🔺️diff/🦀️.rs:339`) | ✓ | ✓ | ✓ | A (:359–364) |
| 🖊️dwg | `DwgDiff` (`🔟ac1024/🔺️diff/🦀️.rs:59`) | ✓ | ✓ | ✓ | A (:129–132) |
| 🖋️dxf | `DxfDiff` (`🔺️diff/🦀️.rs:1662`) | ✓ | ✓ (`DxfHeaderVarsDiff::absorb`) | ✓ | A (:1651–1703) |
| 🖼️tiff | `TiffDiff` (`🧾️document/🔺️diff/🦀️.rs:395`) | ✓ | ✓ | ✓ | A (:465–468) |
| 🗜️deflate | `DeflateDiff` (`🔺️diff/🦀️.rs:42`) | ✓ | ✓ | ✓ | B |
| 🗽️obj | `ObjDiff` (`📐️geometry/🔺️diff/🦀️.rs:988`) | ✓ | ✓ (`ObjVerticesDiff::absorb` etc.) | ✓ | A (:946–1061) |
| 🧱️ply | `PlyDiff` (`🔺️diff/🦀️.rs:637`) | ✓ | ✓ | ✓ | A (:610–663) |
| 🧾️json | `JsonDiff` (`🧱️base/🔺️diff/🦀️.rs:126`) | ✓ | ✓ | ✓ | A (:160–178); `value_diff_between` at :375 |
| 🪟️bmp | `BmpDiff` (`🔺️diff/🦀️.rs:17`) | ✓ | ✓ | ✓ | B |

Diff-level rows with a code (`DiffAlgebra::inverse` builders and diff-module `apply`/`&mut` helpers; 39 rows):

| file:line | artifact | code | evidence |
|---|---|---|---|
| `🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:519` | 🎥️mp4 | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:162` | 🔺️stl | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:359` | 🔺️stl | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:364` | 🔺️stl | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🔺️diff/🦀️.rs:439` | 📼️avi | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🦀️.rs:1651` | 🖋️dxf | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🦀️.rs:1703` | 🖋️dxf | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:287` | 🌦️epw | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:430` | 🌦️epw | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:1155` | 📸️jpg | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:1157` | 📸️jpg | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:1158` | 📸️jpg | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:245` | 🎒️zip | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:245` | 🎒️zip | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:145` | 📰️xml | V3-LEAF-APPLY | diff module &mut mutation helper (apply_to) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:465` | 🖼️tiff | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:467` | 🖼️tiff | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs:468` | 🖼️tiff | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:316` | 🔤️txt | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:319` | 🔤️txt | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:320` | 🔤️txt | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:86` | 💾️binary | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:101` | 💾️binary | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:129` | 🖊️dwg | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:131` | 🖊️dwg | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:132` | 🖊️dwg | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:160` | 🧾️json | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:165` | 🧾️json | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:178` | 🧾️json | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:610` | 🧱️ply | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:663` | 🧱️ply | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🦀️.rs:844` | ☁️las | V3-LEAF-APPLY | diff-level inverse/diff calls .apply(base) |
| `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🦀️.rs:857` | ☁️las | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:326` | 📊️csv | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:472` | 📊️csv | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:255` | 📑️tsv | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:377` | 📑️tsv | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🦀️.rs:946` | 🗽️obj | V3-LEAF-APPLY | apply_*_diff_unchecked clones base and writes fields (diff-level inverse builder) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🦀️.rs:1061` | 🗽️obj | V1-SNAPSHOT-DIFF | diff-level inverse = between(apply(d,base), base) (differencing a mutated copy) |

## 5. Violating kinds (one row per kind)

Paths are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. Codes: V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE, V3-LEAF-APPLY, V3-HAND-MUTATION. "weak guard" = the inverse only checks the forward diff for emptiness and then builds a concrete inverse from `base`.

| file:line | kind | codes | evidence |
|---|---|---|---|
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 🖋️dxf `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: DxfDiff::between / HtmlDiff::between; inverse SetSnapshot(base.clone()) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🕳️set-unknown-statements/🦀️.rs:22` | 🗽️obj `🕳️set-unknown-statements` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Vec<ObjUnknownStatement> |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🖌️set-usemtl/🦀️.rs:21` | 🗽️obj `🖌️set-usemtl` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Vec<ObjUsemtlRange> |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:31` | 🗽️obj `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/📦set-object/🦀️.rs:22` | 🗽️obj `📦set-object` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole faces Vec<u64> list |
| `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 🔺️stl `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🧵set-smoothing-groups/🦀️.rs:22` | 🗽️obj `🧵set-smoothing-groups` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Vec<ObjSmoothingRange> |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🏷️set-group/🦀️.rs:22` | 🗽️obj `🏷️set-group` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole faces Vec<u64> list |
| `🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-track-codec/🦀️.rs:18` | 🎥️mp4 `🎛️set-track-codec` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Mp4Codec sub-document |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🎬set-main-header/🦀️.rs:15` | 📼️avi `🎬set-main-header` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole AviMainHeader record |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🎨set-stream-format/🦀️.rs:16` | 📼️avi `🎨set-stream-format` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole AviStreamFormat record |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 📼️avi `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: AviDiff::between(base,snapshot); inverse SetSnapshot(base.clone()) |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🎞️set-stream-header/🦀️.rs:16` | 📼️avi `🎞️set-stream-header` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole AviStreamHeader record |
| `🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-ftyp/🦀️.rs:16` | 🎥️mp4 `🏷️set-ftyp` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Mp4Ftyp record |
| `🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:16` | 🎥️mp4 `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: between_indexed over whole snapshot (set-snapshot arm, agg_diff mod.rs:95); inverse: SetSnapshot(base.clone()) (mod.rs:120) |
| `💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/🗃️set-snapshot/🦀️.rs:14` | 💬️bcf `🗃️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 🌐️html `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: DxfDiff::between / HtmlDiff::between; inverse SetSnapshot(base.clone()) |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:23` | 📑️tsv `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:18` | 📊️csv `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:15` | 🎒️zip `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff::diff_set_snapshot -> ZipDiff::between; inverse: SetSnapshot(base.clone()) |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:23` | 🎒️zip `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff::diff_set_snapshot -> ZipDiff::between; inverse: SetSnapshot(base.clone()) |
| `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:19` | ☁️las `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff::diff_set_snapshot -> LasDiff::between(base,next); inverse: SetSnapshot(base.clone()) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 📸️jpg `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: cross-subset document::diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:28` | 🗜️deflate `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🪓️remove-huffman/🦀️.rs:19` | 📸️jpg `🪓️remove-huffman` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🔁️change-restart/🦀️.rs:19` | 📸️jpg `🔁️change-restart` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🔲️replace-pixels/🦀️.rs:19` | 📸️jpg `🔲️replace-pixels` | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | Minimality: whole pixel array (pixels: base.pixels != pixels then_some(pixels)); inverse ReplacePixels{pixels: base.pixels.clone()} |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📥️insert-other/🦀️.rs:20` | 📸️jpg `📥️insert-other` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🗑️remove-other/🦀️.rs:19` | 📸️jpg `🗑️remove-other` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🌳️replace-huffman/🦀️.rs:19` | 📸️jpg `🌳️replace-huffman` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on DiffAlgebra::is_empty(self.diff(base)) then builds concrete inverse from base (guard only) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:15` | 📸️jpg `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🧹️remove-quant/🦀️.rs:19` | 📸️jpg `🧹️remove-quant` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🪪️change-jfif/🦀️.rs:23` | 📸️jpg `🪪️change-jfif` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only use of forward diff in inverse; concrete inverse from base |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📊️replace-quant/🦀️.rs:19` | 📸️jpg `📊️replace-quant` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on DiffAlgebra::is_empty(self.diff(base)) then concrete inverse from base (guard only) |
| `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 🧱️ply `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-id3v2/🦀️.rs:14` | 🎵️mp3 `🏷️set-id3v2` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Option<Id3v2Tag> sub-document |
| `🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 🎵️mp3 `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️set-frames/🦀️.rs:14` | 🎵️mp3 `🎼️set-frames` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Vec<Mp3Frame> |
| `🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️set-id3v1/🦀️.rs:14` | 🎵️mp3 `🔖️set-id3v1` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Option<Id3v1Tag> sub-document |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🦀️.rs:21` | 🪟️bmp `🎨️paint-indexed-region` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: paint_indexed_region_controlled (clone base, &mut) then BmpDiff::between; inverse SetSnapshot(base.clone()) |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🦀️.rs:24` | 🪟️bmp `🖌️paint-direct-region` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: paint_direct_region_controlled (clone base, &mut) then BmpDiff::between; inverse SetSnapshot(base.clone()) |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:15` | 🪟️bmp `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: BmpDiff::between(base,&snapshot); inverse SetSnapshot(base.clone()) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 🧾️json `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: JsonDiff::between(base,next); inverse SetSnapshot(base.clone()) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/🌳set-top-level/🦀️.rs:14` | 🧾️json `🌳set-top-level` | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | Minimality: whole document root sub-tree; inverse restores whole root (SetTopLevel{root: base.value.clone()}) |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-native-samples/🦀️.rs:19` | 📷️png `🎨️paint-native-samples` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: paint_native_region_controlled (clone base, &mut) then PngDiff::between; inverse SetSnapshot(base.clone()) |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:15` | 📷️png `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: PngDiff::between(base,&self.snapshot); inverse SetSnapshot(base.clone()) |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs:17` | 📷️png `🌗️change-gamma` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: set_gamma_chunk_controlled (clone base, &mut) then PngDiff::between(base,&next); inverse SetSnapshot(base.clone()) |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs:24` | 📷️png `🩹️patch-pixels` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: paint_rgba8_region_controlled (clone base, &mut) then PngDiff::between; inverse SetSnapshot(base.clone()) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-version-info/🦀️.rs:18` | 🖊️dwg `🏷️set-version-info` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff = DwgDiff::between(base, &version_info_next(base,..)) (whole next snapshot built from base.clone-style literal) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 🖊️dwg `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: Cow::Borrowed(snapshot) = whole-snapshot diff; inverse SetSnapshot(Box(base.clone())) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔢️set-scalar/🦀️.rs:24` | 🧾️json `🔢️set-scalar` | V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | diff Replace{value} whole subtree; inverse derived via DiffAlgebra::inverse in Restore(diff) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-array-element/🦀️.rs:25` | 🧾️json `📥️insert-array-element` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore(inverse) |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-fmt/🦀️.rs:14` | 🔊️wav `🎚️set-fmt` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole WavFmt record |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-member/🦀️.rs:24` | 🧾️json `🗑️remove-member` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore(inverse); diff has Restore(diff) arm |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️set-other-chunks/🦀️.rs:15` | 🔊️wav `📎️set-other-chunks` | V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff arm calls protocol::MutationDiff::apply(outcome.diff(), base); inverse SetSnapshot(base.clone()) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📤️remove-array-element/🦀️.rs:24` | 🧾️json `📤️remove-array-element` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore(inverse) |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-data/🦀️.rs:89` | 🔊️wav `🩹️patch-data` | V1-GENERIC-DIFF, V3-LEAF-APPLY | diff = apply(&base.data, payload) (data.clone() + splice) then diff_set_data(whole data); inverse calls apply() |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/✏️set-member/🦀️.rs:25` | 🧾️json `✏️set-member` | V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | diff uses schema::diff::value_diff_between (generic JSON value diff); inverse = JsonDiff::inverse(outcome.diff()) wrapped in Restore(inverse) |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 🔊️wav `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse SetSnapshot(base.clone()) |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔊️set-data/🦀️.rs:14` | 🔊️wav `🔊️set-data` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole WavData sample collection |
| `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:28` | 💾️binary `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍set-location/🦀️.rs:16` | 🌦️epw `📍set-location` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole EpwLocation sub-record |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:16` | 🌦️epw `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot -> EpwDiff::between; inverse: SetSnapshot(base.clone()) |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅set-data-periods/🦀️.rs:16` | 🌦️epw `📅set-data-periods` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole EpwDataPeriods list/record |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️set-trailing-newline/🦀️.rs:25` | 🔤️txt `↩️set-trailing-newline` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on self.diff(base) then concrete inverse |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🦀️.rs:24` | 🔤️txt `🗑️remove-line` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on self.diff(base).lines then concrete inverse |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🦀️.rs:30` | 🔤️txt `🔚️set-line-ending` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on self.diff(base).line_ending then concrete inverse |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:23` | 🔤️txt `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: TxtLinesDiff::between(&base.lines, &snapshot.lines); inverse SetSnapshot(base.clone()) |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🦀️.rs:25` | 🔤️txt `✏️set-line` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on self.diff(base).lines then concrete inverse |
| `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line/🦀️.rs:25` | 🔤️txt `📥️insert-line` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on self.diff(base) then concrete inverse |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 🖼️tiff `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: TiffDiff::between(base,&snapshot); inverse SetSnapshot(base.clone()) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🔢️set-bits-per-sample/🦀️.rs:14` | 🖼️tiff `🔢️set-bits-per-sample` | V1-GENERIC-DIFF | Minimality: set-<field> emits whole Vec<u16> bits list |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🦀️.rs:25` | 🖼️tiff `🎨️paint-region` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | diff: paint_tiff_region_controlled (clone base, &mut) then between; inverse SetSnapshot(base.clone()) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📤️remove-ifd/🦀️.rs:19` | 🖼️tiff `📤️remove-ifd` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only forward-diff use in inverse |
| `📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:22` | 📝️md `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🏷️replace-tag/🦀️.rs:21` | 🖼️tiff `🏷️replace-tag` | V2-DIFF-DERIVED-INVERSE (weak guard) | inverse guards on DiffAlgebra::is_empty(self.diff(base)) then concrete inverse |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📥️insert-ifd/🦀️.rs:20` | 🖼️tiff `📥️insert-ifd` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only forward-diff use in inverse |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🗑️remove-tag/🦀️.rs:20` | 🖼️tiff `🗑️remove-tag` | V2-DIFF-DERIVED-INVERSE (weak guard) | guard-only forward-diff use in inverse |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:15` | 🖼️tiff `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: TiffDiff::between(base,&snapshot); inverse SetSnapshot(base.clone()) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📣️set-declaration/🦀️.rs:26` | 📰️xml `📣️set-declaration` | V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse derived from forward diff in Restore; Minimality: whole Option<XmlDeclaration> sub-document |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🏷️set-attribute/🦀️.rs:29` | 📰️xml `🏷️set-attribute` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore; diff has Restore(diff) arm |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/✍️set-text/🦀️.rs:27` | 📰️xml `✍️set-text` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) wrapped in Restore(Box<inverse>); diff has Restore(diff) arm |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📜️set-doctype/🦀️.rs:26` | 📰️xml `📜️set-doctype` | V1-GENERIC-DIFF, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse derived from forward diff in Restore; Minimality: whole Option<XmlDoctype> sub-document |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-element/🦀️.rs:27` | 📰️xml `🗑️remove-element` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/📜declare-doctype/🦀️.rs:15` | 📰️xml `📜declare-doctype` | V2-RESTORE-INVERSE | inverse falls back to SetSnapshot{base.clone()} when base has no doctype (declare-doctype/🦀️.rs inverse) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/🏳️set-standalone/🦀️.rs:14` | 📰️xml `🏳️set-standalone` | V2-RESTORE-INVERSE | inverse falls back to SetSnapshot{base.clone()} when base has no declaration (set-standalone/🦀️.rs inverse) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:14` | 📰️xml `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: diff_set_snapshot (between) with blocked_snapshot_violation; inverse: SetSnapshot(base.clone()) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-element/🦀️.rs:29` | 📰️xml `📥️insert-element` | V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | inverse = DiffAlgebra::inverse(outcome.diff(), base) in Restore |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs:11` | 📰️xml `📸️set-snapshot` | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff: crate::schema::diff::diff_set_snapshot (between); inverse: SetSnapshot(base.clone()) |
| `💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 💬️bcf `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:14` | 🎥️mp4 `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🔺️stl `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📼️avi `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🖋️dxf `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🌦️epw `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:12` | 🔊️wav `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🪟️bmp `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🎵️mp3 `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📸️jpg `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📸️jpg `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:17` | 🎒️zip `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📰️xml `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📰️xml `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:18` | 🖼️tiff `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🖼️tiff `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📷️png `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🖊️dwg `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🧾️json `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 🧱️ply `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | ☁️las `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📊️csv `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15` | 📑️tsv `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:17` | 🗽️obj `🩹️patch-snapshot` | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-LEAF-APPLY | macro snapshot_patch_leaf! (📇 contract/editing/🦀️.rs:1389): diff = apply_snapshot_patch_checked clone+steps (editing/🩹️patch/🦀️.rs:600) then DiffAlgebra::between(base,&next); inverse = inverse_snapshot_patches replay of generic JSON-pointer patches (🩹️patch/🦀️.rs:751) |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎚️config/🦀️.rs:53` | 🖼️tiff editor config (not a MutationKind) | V3-HAND-MUTATION, V1-SNAPSHOT-DIFF | hand-written `impl Mutation<TiffEditorConfig>`; `type Diff = TiffEditorConfig` (whole config as the diff) |

## 6. Borderline kinds (no code)

| file:line | kind | codes | note |
|---|---|---|---|
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🎚️set-layer/🦀️.rs:15` | 🖋️dxf `🎚️set-layer` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🪡set-linetype/🦀️.rs:15` | 🖋️dxf `🪡set-linetype` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🏷️set-header-var/🦀️.rs:16` | 🖋️dxf `🏷️set-header-var` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🔧set-entity/🦀️.rs:15` | 🖋️dxf `🔧set-entity` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🖌️set-style/🦀️.rs:15` | 🖋️dxf `🖌️set-style` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🔲set-block/🦀️.rs:15` | 🖋️dxf `🔲set-block` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/📍set-vertex/🦀️.rs:23` | 🗽️obj `📍set-vertex` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🧭set-texcoord/🦀️.rs:23` | 🗽️obj `🧭set-texcoord` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🧲set-normal/🦀️.rs:23` | 🗽️obj `🧲set-normal` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔶set-face/🦀️.rs:23` | 🗽️obj `🔶set-face` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |
| `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/✏️set-point/🦀️.rs:18` | ☁️las `✏️set-point` | — | entity-level `between(payload, base)` field compare (declarative, sparse); no code, flagged for the `between(` policy gate |

## 7. Totals by code

- V1-SNAPSHOT-DIFF: 61 kinds = 30 set-snapshot + 24 patch-snapshot + 6 paint kinds (png 3, bmp 2, tiff 1) + dwg set-version-info. The tiff editor config (§5) is an extra non-kind case.
- V1-GENERIC-DIFF: 49 kinds. Minimality: set-<field> with a whole list or sub-document (epw set-data-periods/set-location; mp4 set-ftyp/set-track-codec; mp3 set-frames/set-id3v2/set-id3v1; avi set-stream-header/format/main-header; wav set-fmt/set-data; obj set-group/set-object/set-smoothing-groups/set-unknown-statements/set-usemtl; tiff set-bits-per-sample; xml-base set-doctype/set-declaration; json-base set-member/set-scalar; json-i-json set-top-level; jpg replace-pixels). Plus 24 patch-snapshot kinds (generic patch).
- V2-DIFF-DERIVED-INVERSE: 11 strict (xml-base 6, json-base 5). Weak guard-only: 17 (jpg 8, txt 5, tiff 4).
- V2-RESTORE-INVERSE: 76 (30 set-snapshot, 24 patch replay, 11 Restore carriers, 3 SetSnapshot fallbacks in xml-valid set-standalone/declare-doctype and wav-set-other-chunks, plus jpg replace-pixels, json-i-json set-top-level, paint and dwg kinds).
- V2-EMPTY-INVERSE: 0. Every `Vec::new()` in an inverse is an absent-target branch (`None => Vec::new()`) or a refused/empty forward diff, so it is a no-op.
- V3-LEAF-APPLY: 33 kinds = 24 patch-snapshot macro, 3 png paint, 2 bmp paint, 1 tiff paint, 2 wav (set-other-chunks, patch-data), 1 dwg set-version-info. Plus diff-level rows in §4.
- V3-HAND-MUTATION: 1 (tiff editor config, §5).
- V4-LAW-UNTESTED: 0 raised (see §8).

## 8. L3 coverage (V4) and the sum-of-inverses law

- Method: a kind is covered when a test file in its artifact (with a `#[…test]` or `async_test` attribute) mentions its type or dsl keyword, calls `inverse`, calls `apply` or `absorb`, and asserts equality on a base-like value. Generic laws (`inverse_law`, `absorb_law` over `demo_mutation_cases()` / `variants()`) count when the kind is in the demo list. Heuristic, not a proof.
- Literal kinds with kind-named evidence: 225 of 241.
- Literal kinds without kind-named evidence (16): 🔤️txt `📸️set-snapshot`, 🖋️dxf `🎚️set-layer`, 🖋️dxf `🎨insert-style`, 🖋️dxf `🏷️set-header-var`, 🖋️dxf `📦insert-block`, 🖋️dxf `🔲set-block`, 🖋️dxf `🖌️set-style`, 🖋️dxf `🧵insert-linetype`, 🖋️dxf `🧹remove-header-var`, 🖋️dxf `🧽remove-style`, 🖋️dxf `🪓remove-block`, 🖋️dxf `🪚remove-linetype`, 🖋️dxf `🪡set-linetype`, 🧱️ply `🗑️remove-comment`, 🧾️json `🔢️set-scalar`, 🪟️bmp `📸️set-snapshot`. They are covered as follows: all 18 dxf kinds are in `demo_mutation_cases()` (`🖋️dxf …/🧬️mutations/🦀️.rs:344`) and run through generic `inverse_law` / `absorb_law` (`🧪️tests/🔬️unit/🦀️.rs:129–160`); ply `remove-comment` is in the generic `inverse_law` demo list (`🚪️io/🧪️tests/🔬️unit/🦀️.rs:168`); json `set-scalar` has an inverse spec in `🧱️base/🧪️tests/🔀️mutate-json-rfc8259/🦀️.rs:99–101`; bmp `set-snapshot` is asserted in `✏️editor/🧬️publication/🧪️tests/🦀️.rs:17`; txt `set-snapshot` has no kind-named test (unverified).
- Patch-snapshot kinds (24): the macro has no per-leaf L3 test. The shared `📇…/🩹️patch/🧪️tests` tests a test-only publication mutation. Only mp4's leaf has an inline test (`🩹️patch-snapshot/🦀️.rs`) that applies the inverse and compares to base. The other 23 are unverified.
- Sum of inverse diffs with `absorb`: no kind-level test sums per-mutation inverse diffs. `absorb` tests compose forward diffs (las `absorb_law` :84, dxf :149–160). mp4 (`🔺️diff/🧪️tests` :165) and dwg (example :185) absorb one diff-level inverse. The design's `Σ == m.diff(base).diff().inverse(base)` is therefore not exercised per kind.

## 9. Absorb soundness (V5-ABSORB, not yet a code hit)

- Most `absorb` impls are field-level last-writer-wins (`if other.x.is_some() { self.x = other.x }`). That is sound for whole-field replacement.
- Keyed collections use helpers (`absorb_named`, `absorb_indexed`, `absorb_entries`, `absorb_triangles`, `absorb_blocks_diff`, `absorb_ifds_opt`, `ObjVerticesDiff::absorb`, `DxfHeaderVarsDiff::absorb`). Their coalescing rules (patch∘patch, create∘delete, delete∘create) were not verified line by line.
- Needs review: `binary` `absorb` = `absorb_splices(&self.splices, &other.splices)` (no visible coalescing). `ply` `absorb` pushes unmatched changes and replaces same-name fields. `tsv` resizes rows then overwrites cells. These may be sound but are unverified.

## 10. Caveats

- Method is regex and text based over the Rust sources (no compiler). Representative and flagged leaf and helper bodies were read. The 141 clean kinds are clean by the same pattern checks, not by line-by-line reading.
- The tiff-baseline `set-bits-per-sample` arm was not extracted by the script; its code comes from the payload type (`Vec<u16>`).
- Minimality ruling applied by payload type: `Vec<…>` or a struct record = violation; `String`, numbers, bools and enums = fine; set-<entity> with a named record (dxf layer/style/linetype/block/entity/header-var, obj vertex/face/normal/texcoord, las point) = borderline, not counted.
- Not checked: `🔣️.json` artifact (no kinds).
