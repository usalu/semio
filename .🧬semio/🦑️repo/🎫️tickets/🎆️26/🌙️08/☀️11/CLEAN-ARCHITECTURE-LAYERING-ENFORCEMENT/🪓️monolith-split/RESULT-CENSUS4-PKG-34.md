# Result CENSUS4-PKG-34 (prefix `c4p34`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `A` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`. Tools: `F/c4p34-tools/` (descendant of `c4p20-tools`). Logs: `F/🗑️generated/c4p34-*` (nothing deleted except the seven duplicates of section 6.4). Stable sandbox/stage root (survives a scratchpad wipe): `/Users/ueli/.cache/c4p34/` (`stage/`, `sbx/`, `orc/`, `ca-*`).

## 0. Verdict

DONE (static evidence, 2026-10-17 09:2x), with one flagged gate effect (section 6.1) and four installed files later edited by concurrent carves (section 6.2). All 7 files are split and installed: 10,172 lines became 7 wiring parents (73 to 301 lines as installed) plus 48 children (longest 409 lines; two of them are the nested module shells of `pptx` and `dxfbin`). Evidence, taken on the tree as installed (08:43 to 08:55): lossless by line multiset 7 of 7 (missing 0), region markers balanced per file, rustfmt parse 55 of 55, every child mounted once, taxonomy resolves 48 of 48, scoped patched rustc differential identical for all six crate roots (lib, `--test`, `wasm32-wasip2`: 18 of 18 runs), POS typeck oracle pre = post (lib 395 = 395 rows, test 528 = 528, 0 new, privacy 0), E0451 scan 0 sites, dxf facet leaf equal to the module tree (9 files, 109,833 bytes), `--check-applied` identical to the baseline for png, jpg, avi, pptx, dxf (exit 0) and deflate (the same foreign exit 1), `lib-wiring-budget` unchanged (2 pass, 1 foreign fail). Not run: a real `bun repo cargo check` (`📓️FLEET.md` carries no `COMPILE ADMITTED` line, checked 09:1x; the fleet mutex was held by `cad`); the exact commands are in section 7.

**Flag (needs a coordinator decision):** the `semio-s-artifact-stdio-png-mutations` crate was at 30 files (lib) and 32 files (`--test`) of the 32-file source-boundary budget; the png io split adds 7 files, so it is now 37 and 39 and `overLimit` is true. Every other measured crate keeps its status (jpg, dxf and deflate artifact roots were over the file budget before; avi-tests 7 > 12 / 13 > 18 and the pptx test-oracle 4 > 11 lib stay inside). If the png crate must stay inside until its own carve, undo only png (restore `c4p34-tools/orig/png.rs.txt` over the png io file and delete the 7 child folders); nothing else depends on it.

## 1. Found state (fresh slice, 2026-10-17 08:00)

- No earlier c4p34 work: no `c4p34-tools`, no `🗑️generated/c4p34-*`, no RESULT file.
- The 7 files are byte-identical to `HEAD b84bf526c7b` (git blob hashes equal) and have the census line counts (1,459 + 1,555 + 1,331 + 1,073 + 1,001 + 2,192 + 1,561 = 10,172). Pre-edit snapshots: `F/c4p34-tools/orig/<tag>.rs.txt`, `SHA256` in the same folder.

| tag | file (tail under `A/`) | lines | SHA256 (first 16) |
| --- | --- | ---: | --- |
| png | `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs` | 1,459 | `07e93d53c85e1fe8` |
| jpg | `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs` | 1,555 | `a50d2a55cea14739` |
| avi | `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs` | 1,331 | `e137add75ff34c61` |
| pptx | `📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` | 1,073 | `529963ab30c95d1b` |
| dxfbin | `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🦀️.rs` | 1,001 | `8dad8da12ce9bdcd` |
| dxfschema | `🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🦀️.rs` | 2,192 | `4e3df1143c237ee2` |
| zlib | `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🦀️.rs` | 1,561 | `2deb9fcd527b53f3` |

- Mount chains (the module context each file compiles in, found by `git grep` of the `#[path]` lines): png io is `mod component` inside `pub mod io` of the `semio-s-artifact-stdio-png-mutations` lib root; jpg io, dxf io and dxf schema diff and deflate io are mounted from the artifact lib root `<artifact>/🦀️.rs` through nested `#[path = "."]` modules; avi io is mounted only by the `semio-s-artifact-stdio-avi-tests` crate (`🪞️mirror`); the pptx oracle file is mounted by the `semio-s-artifact-stdio-pptx-test-oracle` crate (`pptx/🔮️oracles/🦀️.rs`, excluded from the artifact workspace).
- Only one source-text reader exists: the diff facet leaf `rust: include_str!("🔺️diff/🦀️.rs")` in `A/🖋️dxf/.../📰️header/🧬️schema/🦀️.rs` (it reads the dxfschema file). The other six files have no `include_str!` reader of their `.rs` text.
- `dxfbin` and `pptx` hold one big inline module each (`mod diff_codec` 913 lines, unindented body; `mod oracles` 981 lines), so they need a nested split (module becomes a file module, its items go into grandchildren).

## 2. Baselines (taken before the first install)

- Scoped patched rustc differential (`oracle.py`, sandbox of the whole artifact, original restored, native lib, `--test`, `wasm32-wasip2`): the baseline is the `orig` side of each run (errors orig: png 373/417/373, jpg 1027/1524/1027, avi 286/458/286, pptx 36/38/36, dxf 1244/1453/1244, zlib 701/1035/698; lib/test/p2). Lib roots: png `mutations` crate, jpg, avi `tests` crate, pptx `test-oracle` crate, dxf, deflate.
- `--check-applied` (`bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813`, `ulimit -n 65536`): `python3 c4p34-tools/cadigest.py 🗑️generated/c4p34-checkapplied-before-*.log`.
- Reader/facet probe: `bun c4p34-tools/facet-check34.ts <dxf schema descriptor> <dxf diff root>` = `{"files":1,"facetBytes":107335,"treeBytes":107335,"equal":true}` (`c4p34-facet-before.log`).
- `lib-wiring-budget`: `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` in `📚️library`: 2 pass, 1 fail; the failure is the foreign `🔋️energy/…/🔋️model/…/🧬️schema/🦀️.rs: grew from 165 to 214 lines` (`c4p34-wiring-before.log`). None of the seven files is a lib root, so no pin applies.
- Census (`census.sh before`, canonical `readRustSourceClosureV1`, `c4p34-census-before.jsonl`): png-mutations 30 files lib / 32 test (limit 32, inside); jpg 99/161, dxf 84/105, deflate 74/97 files (already over before); avi-tests 7/13; pptx test-oracle 4/12.
- POS typeck oracle `pre` (`run-oracle.sh pre lib,test`, 8 crates, absolute classification, canary E0308 E0616 E0624 reached in lib and test): `c4p34-pos-pre.log`.

## 3. What changed

Method: item-level splitter `c4p34-tools/c4p34.py` (descendant of `c4p20.py`, with `widen2.py` and `rsitems.py`; new: the `inner` mode that turns one inline module into a file module and splits its body into grandchildren). Every top-level item keeps its text. A child is `<folder>/🦀️.rs` next to its parent, mounted by `#[path = "<folder>/🦀️.rs"] mod <snake>;` followed by `pub use <snake>::*;` (a `pub` glob only where a child item is `pub` and named outside the file; `pub(crate) use` where the child's pub items have no user outside; `use` otherwise) and starts with a docstring `//! <folder emoji> ... See [`super`].`, then `use super::*;` (dropped in the 3 children that need nothing from the parent: jpg `transforms-and-tables`, zlib `adler-bit-io-and-huffman` and `lz77-matching`). All `use` items, the `derived_*` and `io_registry` inline modules, the `#[path]` mounts of submodules (`binary`, `text`, `native`, `sqlite`, `dynamic`, `publication`, tests) and the cfg(test) mounts stay in the parent, so every relative `#[path]` and every module path keeps its meaning. Private definitions that another file of the split names became `pub(super)` (planner `widen2.py`, driven by identifier presence in the other children, the parent and the files of the module's own subtree; `.widen.tsv` per file in `🗑️generated/c4p34-widen-<tag>.tsv`).

| File | Lines before | Parent after (as installed) | Children (lines) | Widened to `pub(super)` |
| --- | ---: | ---: | --- | ---: |
| png io | 1,459 | 278 | 149 (`🧾️records-and-chunk-io`) + 189 (`✅️header-and-structure`) + 129 (`🎞️filters-and-adam7`) + 147 (`🖼️unpack-and-ancillary`) + 235 (`🧭️layout-and-projection`) + 116 (`✍️native-encode`) + 274 (`🔓️native-decode`) | 27 |
| jpg io | 1,555 | 301 | 224 (`🧮️transforms-and-tables`) + 303 (`🔡️huffman-and-bit-io`) + 258 (`✍️encoder`) + 334 (`🔎️header-and-decode-entry`) + 186 (`🪜️step-decoder`) | 80 |
| avi io | 1,331 | 291 | 278 (`🧱️riff-and-headers`) + 104 (`📖️decoder`) + 140 (`🎬️encode-avi-and-cursor-state`) + 409 (`⏯️encode-cursor-steps`) + 139 (`📏️encoded-sizes`) | 42 |
| pptx oracle | 1,073 | 90 | 34 (`🧰️reference-implementation`) + 150 (`🧰️reference-implementation/🌳️xml-tree-parse-and-serialize`) + 189 (`🧰️reference-implementation/📦️package-and-presentation-model`) + 201 (`🧰️reference-implementation/🖼️render-and-write-presentation`) + 177 (`🧰️reference-implementation/🧾️json-wire-and-edit-helpers`) + 149 (`🧰️reference-implementation/🧬️mutation-apply`) + 132 (`🧰️reference-implementation/🔁️entry-points-and-plumbing`) | 43 |
| dxf binary diff | 1,001 | 91 | 33 (`🧬️diff-codec`) + 127 (`🧬️diff-codec/📦️primitives-and-values`) + 222 (`🧬️diff-codec/📐️group-codes-and-entities`) + 90 (`🧬️diff-codec/🗂️tables-and-blocks`) + 238 (`🧬️diff-codec/🏷️item-diffs`) + 168 (`🧬️diff-codec/🧵️triples-and-collection-diffs`) + 75 (`🧬️diff-codec/🚦️diff-binary-impl`) | 0 |
| dxf schema diff | 2,192 | 208 | 251 (`🧮️collection-algebra-core`) + 244 (`🏷️header-and-layer-diffs`) + 314 (`🎨️style-linetype-and-tables-diffs`) + 198 (`📐️entity-diff-records`) + 395 (`🔧️entity-diff-apply`) + 136 (`🧱️block-diffs`) + 270 (`🎯️dxf-diff-root`) + 129 (`🛠️diff-builders`) | 29 |
| zlib | 1,561 | 73 | 184 (`🔢️adler-bit-io-and-huffman`) + 111 (`🔎️lz77-matching`) + 219 (`🧵️deflate-encode-job`) + 187 (`📥️retained-decoder`) + 204 (`📤️retained-encoder`) + 160 (`🚚️publication-and-job-steps`) + 134 (`🔐️flat-close-and-checkpoint`) + 220 (`🎚️tuned-encode`) + 185 (`🔓️inflate-and-zlib-frame`) | 108 |

Special cases:
- `pptx` (`mod oracles`, 981 lines, indented) and `dxfbin` (`mod diff_codec`, 913 lines, unindented body): the inline module became a file module with the same module path (`#[allow(unused_imports)] #[path = "🧬️diff-codec/🦀️.rs"] mod diff_codec;` and `#[path = "🧰️reference-implementation/🦀️.rs"] mod oracles;`; the `pub use diff_codec::*;` line and the region wrappers stay). Its body is the shell file (the `use` items plus the wiring), its items went into 6 grandchildren each. The only lines that disappear are the two header and closing lines `mod X {` and `}` per file (declared in the lossless check). `dxfbin` has no region markers; `diff_wire_codec` (80 lines) stays inline.
- `dxfschema`: the 12 originally empty `HandcraftedDiffCodec` region pairs, the dangling doc comment, `demo_diff_cases` (cfg(test)) and the cfg(test) test mount stay in the parent; the facet leaf is re-pointed (section 4).
- `png`: the region `ExactAuthority` (closed by an `endregion Codec` in the original) is repeated balanced in every child it spans; the originally empty region `🧪️CodecTests` stays in the parent.
- `zlib`: the regions `DissolvedEngineCodec`, `DeflateCodec`, `StreamingEncode` and `TunedEncode` are repeated balanced in each child they span (10 region pairs became 33); `mod dynamic;`, `pub mod publication;` and `mod codec_tests;` stay in the parent (see section 6.2 for the peer's later edit of the last two).
- `avi`: the child is called `decoder`, not `decode_avi`, so that no module shares its name with the `pub fn decode_avi` it holds (Rust keeps modules and functions in different namespaces, but `modclash.py` flags it).
- Re-exports: no new alias, no compatibility layer and no caller outside the 7 files was touched. Each `pub use child::*` is the parent's own module wiring of items the file already exposed and that have users outside it (examples from `c4p34-pubusers.log`: `decode_png` 50 files, `encode_png` 44, `write_chunk` 16, `encode_jpg` 22, `decode_jpg` 18, `sniff_real_bytes` 17, `decode_avi` 13, `zlib_compress` 11, `DxfDiff` 12); re-pointing 50 callers to private child modules would make the child tree the public API. Two children keep `pub use` although nothing outside names them (zlib `retained-decoder`, `retained-encoder`: they are the module's public retained-codec types; without the wiring they would be dead private items). The shells of the two nested modules re-export with `pub(crate) use` (dxfbin) or `use` (pptx, plus one `pub use` for the entry points the outer file calls), because the shells are private modules.

## 4. Source-text readers

- Searched with `git grep -F` over `✏️s/🔌️plugins` and `🧰️framework` for `include_str!`/`include_bytes!` of any `🦀️.rs` of the six artifacts and for literal `🚪️io/🦀️.rs`, `🔺️diff/🦀️.rs` and `🔮️oracles/🦀️.rs` readers in `.ts` files. The only reader of the text of one of the seven files is the dxf diff facet leaf in `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🦀️.rs` (`diff: FacetLeaves { rust: include_str!("🔺️diff/🦀️.rs"), .. }`), now `rust: concat!(include_str!("🔺️diff/🦀️.rs"), include_str!("🔺️diff/<child>/🦀️.rs") x 8)` in mount order. `facet-check34.ts` (tokenised `include_str!` list resolved against `readModuleTreeSource` of the diff root): before `{"files":1,"facetBytes":107335,"treeBytes":107335,"equal":true}`, after `{"files":9,"facetBytes":109833,"treeBytes":109833,"equal":true}`.
- The facet leaves of the six descriptors read only the `🦀️.rs` of the artifact's `snapshot`, `🔺️diff`, `🧬️mutations` and `💡️inferences` directories and the artifact root itself; of the seven files only the dxf schema diff is among them. None of the 7 files is listed in `POLICY_SPLIT_SOURCE_ROOTS`; `lib-wiring-budget` reads lib roots only (none of the 7 is one). `readModuleTreeSource` excludes `🧪️tests` mounts, so the facet text matches without them.

## 5. Verification (every command was run; results are the real ones)

Paths relative to `F/c4p34-tools/` unless stated; logs `F/🗑️generated/c4p34-*`. Nothing died with ENFILE, EMFILE or a Bun segfault; one `--check-applied` run (deflate, after) died on a peer's infra edit (`SyntaxError: Export named 'capturePersistentNativeProtectedMaintenanceV1' not found`) and was re-run once (note under the `--check-applied` table).

| Check | Command | Result |
| --- | --- | --- |
| lossless line multiset, live tree | `python3 live.py` (`c4p34-post-live-live.log`, 08:44) | 7 of 7 `LOSSLESS OK`, missing 0, each installed file byte-equal to its stage. Extras are only wiring (`#[path]`, `mod`, re-exports, `use super::*;`), child docs and carried region markers (balanced: opens = closes in every file); non-blank lines orig > new: png 1,354 > 1,393, jpg 1,475 > 1,501, avi 1,225 > 1,253, pptx 989 > 1,022, dxfbin 925 > 955, dxfschema 1,951 > 1,993, zlib 1,421 > 1,510. `installed-sha256.txt` holds the SHA256 of all 55 installed files (= the stage). Later, `post.sh final` (09:18) shows live.py exit 1 for jpg and zlib only, because concurrent artifact carves edited 4 installed files (section 6.2); zlib is lossless modulo those declared edits (`python3 c4p34.py lossless zlib <live dir>`). |
| regions, mounts, docstring emoji, paths | `python3 check.py` (`c4p34-post-live-check.log`) | `CHECKS OK`: 48 children, longest child 409 lines (avi `encode-cursor-steps`), every child mounted exactly once and present, docstring emoji = folder emoji, unique among siblings and never the root's, regions balanced in every file, no `[DEBUG]`, longest repo-relative path 221 UTF-8 bytes (<= 240), longest absolute path 184 characters (<= 256) |
| rustfmt parse | `python3 rustfmt_live.py live` (`rustfmt +nightly-2026-07-20 --emit stdout --config skip_children=true`) | 55 of 55 files parse (also on the stage) |
| module-name clash | `python3 modclash.py <7 tags>` | OK (the avi `decode_avi` case was found and renamed before install) |
| private declarations used across files | `python3 defscan.py <tag>` x7 | 0 cross-file leftovers (private declarations scanned: 12, 14, 19, 8, 13, 34, 12 for png, jpg, avi, dxfbin, dxfschema, pptx, zlib) |
| taxonomy and emoji | `bun tax.ts newdirs.txt`, `bun ../c4p5-tools/rgi.mjs newnames.txt`, sibling scan in `post.sh` | 48 of 48 directories resolve (`unresolved: []`), 48 of 48 names RGI emoji + U+FE0F (`bad=0`), every emoji is already a folder prefix in the repo (`rf8-folder-emojis.tsv`), 0 sibling emoji clashes (also against the existing `💾️binary`, `📝️text`, `🧪️tests`, ... folders) |
| scoped patched rustc differential (`rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null`, sandbox of the whole artifact with the original restored, extern-crate and external-glob patches on both sides, `<SBX>` path normalisation) | `run-diff.sh live1 png jpg avi`, `run-diff.sh live2 pptx dxf zlib`, `run-diff.sh live3 dxf` (`C4P34_LIVE=1`; logs `c4p34-diff-live{1,2,3}-<group>.log`) | errors orig = new and only-new 0, warnings orig = new and only-new 0 in all 18 runs (lib / `--test` / `wasm32-wasip2`): png 373/417/373, jpg 1,027/1,524/1,027, avi 286/458/286, pptx 36/38/36, dxf 1,244/1,453/1,244, zlib 701/1,035/698. The first dxf live run showed 8 only-orig errors: the facet leaf re-pointed at the 8 children, which the baseline sandbox lacks by construction; `live3` restores the old leaf in the baseline sandbox and is clean. The stage runs (`c4p34-diff-stage*.log`) found and fixed four things before install: png E0425 for `decode_png_image_controlled` and `encode_png_image_controlled`, which the sibling submodule `🚦️native` reads (the widening scan now covers the module's whole subtree); three unused `use super::*;` (jpg `transforms-and-tables`, zlib `adler-bit-io-and-huffman` and `lz77-matching`: `glob=False`); an unused `pub(crate) use retained_encoder::*;` warning in zlib (`keep_pub`); the avi child name `decode_avi`. |
| oracle sensitivity | python mutation on the pptx `new` sandbox (removed `pub(super)` from `XNode::child`) | orig 36 errors, mutated 42: `+6 error[E0624]: method child is private`, i.e. the differential reaches method privacy |
| POS typeck oracle (8 crates, absolute classification, canary E0308/E0616/E0624 reached) | `run-oracle.sh pre lib,test` (before install) and `post` (after); `python3 posdiff-full.py pre post lib test` | lib 395 rows = 395, test 528 = 528, **new 0, gone 0**; privacy 0 non-stub in both; resolve 91/18, type 241/510, other 63/0 non-stub rows identical before and after (all foreign, section 6.3) |
| E0451 scan | `POS_PREFIX=c4p34 python3 rpv-tools/lit.py families/c4p34.json post` | 0 candidate sites (167 brace structs, 307 files in the module tree) |
| facet leaf | `bun facet-check34.ts <descriptor> <diff root>` | before 1 file equal, after 9 files equal (section 4) |
| duplicate mounts | `python3 ttc-tools/dupmounts.py <6 artifact dirs>` | 26 duplicate targets before and 26 after, identical set (all foreign directory targets, e.g. the `🗜️deflate/🔮️oracles` facade chains); 0 for the 48 children |
| lib wiring budget | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` in `📚️library` | before 2 pass / 1 fail, after 2 pass / 1 fail; the failure is foreign (`🔋️energy/…/🔋️model/…/🧬️schema/🦀️.rs: grew from 165 to 214 lines`) |
| source-boundary census (canonical `readRustSourceClosureV1`, native, wasm32-unknown-unknown, wasm32-wasip2, lib and cfg(test)) | `census.sh before` / `census.sh after` (`c4p34-census-{before,after}.jsonl`) | section 6.1 |
| `new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813` | `checkapplied-one.sh before|after <artifact>`, `python3 cadigest.py <logs>` | baseline = after for png, jpg, avi, pptx and dxf (exit 0, `onlyCombined` 0, admission and shared empty); deflate: exit 1 before and after for the same foreign rows, see the table below |

| artifact | exit before > after | crates | native base / combined / onlyCombined, before > after | `--test` base / combined / onlyCombined, before > after | admission, shared |
| --- | --- | ---: | --- | --- | --- |
| png | exit=0 > exit=0 | 6 > 6 | 877 / 877 / 0 > 877 / 877 / 0 | 1214 / 1218 / 0 > 1214 / 1218 / 0 | empty, empty |
| jpg | exit=0 > exit=0 | 0 > 0 | 876 / 876 / 0 > 876 / 876 / 0 | 1344 / 1344 / 0 > 1344 / 1344 / 0 | empty, empty |
| avi | exit=0 > exit=0 | 5 > 5 | 672 / 672 / 0 > 672 / 672 / 0 | 863 / 867 / 0 > 863 / 867 / 0 | empty, empty |
| pptx | exit=0 > exit=0 | 8 > 8 | 1265 / 1265 / 0 > 1265 / 1265 / 0 | 1448 / 1448 / 0 > 1448 / 1448 / 0 | empty, empty |
| dxf | exit=0 > exit=0 | 0 > 0 | 1145 / 1145 / 0 > 1145 / 1145 / 0 | 1352 / 1352 / 0 > 1352 / 1352 / 0 | empty, empty |
| deflate | exit=1 > exit=1 | 0 > 6 | 592 / 593 / 1 > 592 / 593 / 1 | 896 / 911 / 7 > 896 / 894 / 7 | empty, empty |

Logs `c4p34-checkapplied-{before,after}-<artifact>.log`, digest `python3 c4p34-tools/cadigest.py`. png, jpg, avi, pptx and dxf are identical before and after (exit 0, `onlyCombined` 0). deflate: exit 1 before and after for the same foreign rows (`DeflateSnapshot` unresolved in the deflate io root and its codec test, line numbers shifted by one); the combined `--test` count moved from 911 to 894 and `crates` from 0 to 6 because the peer's deflate carve (section 6.2) changed the working tree between the two runs. The first deflate `after` run (`...-after-deflate-run1-syntaxerror.log`) died on a peer's infra edit (`SyntaxError: Export named 'capturePersistentNativeProtectedMaintenanceV1' not found`) and was re-run once.


## 6. Flags, foreign errors, hand-offs

### 6.1 Source-boundary census (32 files / 262,144 bytes / 4,000 active lines) before > after

| crate (workspace) | lib files / bytes / lines | `--test` files / bytes / lines | over limit |
| --- | --- | --- | --- |
| `semio-s-artifact-stdio-png-mutations` | 30 / 123,150 / 2,197 > **37** / 125,607 / 2,262 | 32 / 131,327 / 2,331 > **39** / 133,784 / 2,396 | **false > true (binding: files)** |
| `semio-s-artifact-stdio-avi-tests` | 7 / 110,771 / 2,180 > 12 / 112,661 / 2,215 | 13 / 138,577 / 2,591 > 18 / 140,467 / 2,626 | false, false |
| `semio-s-artifact-stdio-pptx-test-oracle` (own workspace) | 4 > 11 files, bytes 74,848 (derived) > 73,819 | 12 > 19 files, 505,487 (derived) > 504,458 bytes | lib false; test already true (bytes) |
| `semio-s-artifact-stdio-jpg` | 99 > 104 / 417,479 > 419,864 / 6,384 > 6,440 | 161 > 166 | true, true (before and after) |
| `semio-s-artifact-stdio-dxf` | 84 > 99 / 627,271 > 632,365 / 8,626 > 8,637 | 105 > 120 | true, true |
| `semio-s-artifact-stdio-deflate` | 74 > 83 / 352,521 > 357,844 / 5,701 > 5,826 | 97 > 106 | true, true |
| `semio-s-artifact-stdio-png`, `-avi`, `-pptx` (top crates) | 1, 1, 3 files: unchanged | 8, 8, 3: unchanged | false |

The pptx test-oracle rows before are derived (the oracle was added to the census after the baseline run: after minus the byte delta of the 8 pptx files, -1,029 bytes because the dedent of the inline body shortens the lines). Native, wasm32-unknown-unknown and wasm32-wasip2 rows are identical.

The only crate that goes from inside to over the budget is `png-mutations`: it already sat at the file limit in the `--test` configuration (32 of 32), so any split of the png io file crosses it. It compiles nothing today anyway (its first-party closure contains the oversized framework crates that the gate refuses), the bytes and lines stay far below the limits, and it is the crate that the artifact carve has to cut next (the io codec is its single biggest module). Decision for the coordinator: keep (the split is what makes that carve possible) or undo png only (restore `c4p34-tools/orig/png.rs.txt` over the png io file and remove the 7 child folders).

### 6.2 Concurrent edits by artifact crate carves (not mine)

After my install at 08:43 peers ran the artifact crate carve over the deflate and jpg artifacts (new crates such as `semio-s-artifact-stdio-deflate-zlib` and `...-examples`, many modified files under `🗜️deflate/**`). They edited four of my installed files; I left all of it as found. All evidence above was taken at install time on the tree as installed (the 08:44 `live.py` run and the differentials of 08:46 to 08:50 precede every one of these edits).
- 08:51 zlib parent: the mounts `#[path = "../📤️publication/🦀️.rs"] pub mod publication;` and `#[cfg(test)] #[path = "../🧪️tests/🔬️codec/🦀️.rs"] mod codec_tests;` became `pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::zlib::publication;`. 09:02 zlib parent: added `pub use lz77_matching::{HASH_SIZE, WINDOW};`, and the child `🔎️lz77-matching` widened `WINDOW` and `HASH_SIZE` from `pub(super)` to `pub`. The zlib lossless check accounts for the 5 missing lines and the extra lines (`foreign_live` in `specs/zlib.py`).
- 09:14 jpg parent: `derived_composition`, `io_registry`, the `binary`/`text`/`sqlite` mounts, `derived_construction`, `derived_analysis` and the `derive_artifact_facets!` call were moved out by the carve (parent 5,080 bytes now), and `pub use binary::snapshot::encode_options::{..}` and two `binary::snapshot::observations::..` paths in my child `🔎️header-and-decode-entry` became absolute `crate::standards::..` paths.
- The deflate `--check-applied` baseline was already `exit 1` before my install (native 592/593 `onlyCombined` 1, `--test` 896/911 `onlyCombined` 7: `cannot find type DeflateSnapshot` in the deflate io root and its codec test; the working tree held the earlier carve edits of that artifact), so that gate is not at zero for a reason that is not the split.
- Hand-off: the carve keeps the zlib module file where it is (`pub mod zlib;` over `../../🦀️.rs` in the new crate) and my children follow through their own `#[path]` mounts; nothing in the split needs a change for it. `python3 c4p34-tools/live.py` compares against the installed stage, so it reports the four edited files; the stage and `installed-sha256.txt` are the reference.

### 6.3 Foreign errors (identical before and after)

- POS non-stub rows, lib: resolve 91, type 241, other 63; test: resolve 18, type 510 (mostly E0220 and E0599 on associated items of the stubbed framework traits, E0437/E0425 on stub-trait members; for example jpg `E0599 no method named diff found for enum JpgMutation` in the mutation test leaves). Privacy 0. Same multiset before and after.
- The sandbox differentials count 36 to 1,524 errors per root that are all unresolved external crates (`protocol`, `quick_xml`, `zip`, `semio_*`); the same multiset on both sides.
- `lib-wiring-budget`: 1 foreign failure (energy model schema root). `--check-applied` deflate: section 6.2.

### 6.4 Deviation from the folder rules

While copying the stage widen reports I created seven misnamed duplicates (`png.widen.tsv`, ... directly in `🗑️generated`, copies of the properly named `c4p34-widen-<tag>.tsv`) and removed those seven duplicates seconds later. Nothing else in a `🗑️generated` folder was deleted, and no content was lost.

## 7. Real-check commands for the compile queue (not run: `📓️FLEET.md` has no `COMPILE ADMITTED` line; the fleet mutex was held by `cad` at 09:00)

One launcher call per artifact workspace, under the fleet mutex, from `/Users/ueli/Documents/semio`:

```
SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate> --lib --tests --keep-going --message-format=short --manifest-path <A>/<artifact>/Cargo.toml
```

| artifact (`A/<dir>`) | `-p` crate |
| --- | --- |
| `📷️png` | `semio-s-artifact-stdio-png-mutations` |
| `📸️jpg` | `semio-s-artifact-stdio-jpg` |
| `📼️avi` | `semio-s-artifact-stdio-avi-tests` |
| `🖋️dxf` | `semio-s-artifact-stdio-dxf` |
| `🗜️deflate` | `semio-s-artifact-stdio-deflate` (after the peer's carve settles; the zlib module now sits in `semio-s-artifact-stdio-deflate-zlib`) |
| `📽️pptx/🔮️oracles/📦️packages/🦀️rust` (own workspace, `--manifest-path` its `Cargo.toml`) | `semio-s-artifact-stdio-pptx-test-oracle` |

Expect the source-boundary refusal for jpg, dxf, deflate and png-mutations (section 6.1) and the pre-existing foreign errors of section 6.3; the split itself is expected to add none (differential and POS identical).

## 8. New folders (48, each with a `🦀️.rs`; `A` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`)

- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧾️records-and-chunk-io`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/✅️header-and-structure`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🎞️filters-and-adam7`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🖼️unpack-and-ancillary`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧭️layout-and-projection`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/✍️native-encode`
- `A/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🔓️native-decode`
- `A/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🧮️transforms-and-tables`
- `A/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🔡️huffman-and-bit-io`
- `A/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/✍️encoder`
- `A/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🔎️header-and-decode-entry`
- `A/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪜️step-decoder`
- `A/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🧱️riff-and-headers`
- `A/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/📖️decoder`
- `A/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🎬️encode-avi-and-cursor-state`
- `A/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/⏯️encode-cursor-steps`
- `A/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/📏️encoded-sizes`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/🌳️xml-tree-parse-and-serialize`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/📦️package-and-presentation-model`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/🖼️render-and-write-presentation`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/🧾️json-wire-and-edit-helpers`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/🧬️mutation-apply`
- `A/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🧰️reference-implementation/🔁️entry-points-and-plumbing`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/📦️primitives-and-values`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/📐️group-codes-and-entities`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/🗂️tables-and-blocks`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/🏷️item-diffs`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/🧵️triples-and-collection-diffs`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🧬️diff-codec/🚦️diff-binary-impl`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🧮️collection-algebra-core`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🏷️header-and-layer-diffs`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🎨️style-linetype-and-tables-diffs`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/📐️entity-diff-records`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🔧️entity-diff-apply`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🧱️block-diffs`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🎯️dxf-diff-root`
- `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🛠️diff-builders`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🔢️adler-bit-io-and-huffman`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🔎️lz77-matching`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🧵️deflate-encode-job`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/📥️retained-decoder`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/📤️retained-encoder`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🚚️publication-and-job-steps`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🔐️flat-close-and-checkpoint`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🎚️tuned-encode`
- `A/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🔓️inflate-and-zlib-frame`

## 9. Files

Edited (8): the 7 parents of section 3 and, through the facet re-point, `A/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🦀️.rs` (one line). Created: the 48 children above. Tools: `F/c4p34-tools/` (`c4p34.py`, `specs/*.py`, `widen2.py`, `rsitems.py`, `oracle.py`, `run-diff.sh`, `run-oracle.sh`, `live.py`, `check.py`, `modclash.py`, `defscan.py`, `globuse.py`, `rustfmt_live.py`, `rf.sh`, `post.sh`, `installall.py`, `pubusers.py`, `pubusers.json`, `facet-check34.ts`, `census.sh`, `census.ts`, `checkapplied-one.sh`, `cadigest.py`, `posdiff-full.py`, `tax.ts`, `newdirs.py`, `families/c4p34.json`, `orig/*.rs.txt` with `SHA256`). Logs: `F/🗑️generated/c4p34-*`. Stage and sandboxes: `/Users/ueli/.cache/c4p34/` (`stage/<tag>`, `sbx/`, `orc/`; reproducible from the specs).
