# Result c6plug (census 6 packages P46, P50, P51, P53, P56, P57)

Executor Sonnet 5.5, foreground, no sub-agents, no cargo / `bun repo` / nx / bazel, no modifying git command, no ticket tool. `C` = this folder. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c6plug-tools/`. Log: `F/🗑️generated/c6plug-verify.log`. Sandboxes and stages: `/Users/ueli/.cache/c6plug/`.

## 0. Verdict

**DONE (static evidence, nothing compiled).** All 8 files of the 6 packages are under 1,000 lines (largest child 407, every root 49 to 167). 8 files (10,695 lines) became 8 roots + 53 children, in 8 independent installs. No new file uses a glob: no child has `use super::*;`, no root has `pub use x::*;`. The four pre-existing `use crate::*;` of the ISO 16757 inline parts are now explicit imports (`glob use lines orig/new` 4/0 for `is`, 0/0 elsewhere); the only glob left is the untouched `#[cfg(test)] use super::*;` inside gb's `native_codec` and the `use super::*;` of the mounted gb tests, pdf `canonical`/`octets` and gltf `canonical` files, which were already there.

Needs the coordinator's compile (section 6). Four things in the original tree are broken independent of this split and are reported, not repaired (section 5).

## 1. What was cut

`PL` = `✏️s/🔌️plugins`. Tag = key in `F/c6plug-tools/files.tsv`.

| Pkg | Tag | File (tail) | Before | Root after | Children (lines) |
| --- | --- | --- | ---: | ---: | --- |
| P46 energy | en | `🔋️energy/…/🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs` (crate root) | 1,410 | 84 | 🧫️case 41, 🛠️constructors 116, ⚖️laws 231, 🏷️kinds 305, 🗂️directories 305, 🧵️probe-corpus 407 |
| P50 norm | vd | `📕️norm/…/🏭️vdi3805/…/🧬️schema/📸️snapshot/🦀️.rs` (crate root) | 1,543 | 107 | 🔤️localized-text-and-units 101, 🏷️product-identity-and-links 65, 🛡️limits-and-code-tables 90, 🗂️sheet-registry 246, 🧾️record-families 266, 🧮️sheet-attributes 300, 📚️catalog-model 229, 🩺️diagnostics-and-edition-profile 48, 🧪️reference-fixture 170 |
| P50 norm | is | `📕️norm/…/📇️iso16757/…/🧬️schema/📸️snapshot/🦀️.rs` (crate root) | 1,036 | 87 | 🧱️catalogue-core 155, 📚️part-1-catalogue 316, 📐️part-2-geometry 182, 📖️part-4-dictionary 139, 🔄️part-5-exchange 124, 🧾️artifact-definition 54 (the four `pub mod part_N { … }` inline modules became file modules of the same names) |
| P51 procedural | gb | `🌀️procedural/…/🌀️generation2d/…/🚪️io/💾️binary/📸️snapshot/🦀️.rs` (module `snapshot` of `…-2d-io-binary`) | 1,078 | 121 | 🏷️mounted-owner-state 139, 🔤️owner-string-targets 234, 🧮️owner-values-and-dictionaries 218, 🗃️owner-containers-and-widgets 214, 🔌️owner-pack-protocol 225 (the 631-line `impl Generation2dMountedTypedSnapshotOwner` was cut into three `impl` blocks at method boundaries, lines 387 and 593) |
| P53 process | p3 | `🏭️process/…/🧊️process3d/…/🧬️schema/📸️snapshot/🦀️.rs` (module of `…-process3d-model`) | 1,068 | 112 | 🪚️machines-and-catalogs 367, 📐️document-records 238, 🧱️brep-converters 217, 🔀️flow-and-scene-converters 229 |
| P56 remodel | rm | `📸️remodel/…/📸️remodeling/…/🧬️schema/📸️snapshot/🦀️.rs` (module of `…-remodeling-model`) | 1,317 | 167 | 🧩️asset-handles-and-durable-content 290, 🕸️mesh-handles 213, 🧮️buffers 96, 🎞️media-and-calibration 180, ⚙️reconstruction-params 280, 📊️reconstruction-results 184 |
| P57 stdio | pdf | `🗄️stdio/…/📖️pdf/…/🧱️base/🧬️schema/📸️snapshot/📐️model/🦀️.rs` (crate root) | 1,951 | 49 | 🔗️object-model 330, 🖋️content-operators 181, 🌈️colour-and-graphics-state 231, 🔤️font-model 309, 🖼️xobjects 118, 🎯️navigation 178, 📌️annotations-and-forms 232, 📄️document-metadata 279, 📃️page-model 128 |
| P57 stdio | gltf | `🗄️stdio/…/🧊️gltf/…/🧬️schema/📸️snapshot/🦀️.rs` (module of `…-gltf-model-snapshot`) | 1,292 | 114 | 🧵️source-form-and-json 88, 🔢️attribute-map-and-defaults 104, 🌳️scene-graph 196, 🧮️accessors-and-buffers 148, 🎨️materials-and-textures 282, 🎬️skins-and-animations 161, 📷️cameras 148, 📄️document-and-decoded-input 155 |

Children are `<emoji>️<kebab>/🦀️.rs` next to their root, mounted by `#[path = "<folder>/🦀️.rs"] mod <name>;`. `p3` was first cut into 7 children and re-cut into 4 because its crate closure reached exactly 32 files; the 7-child spec is kept at `/Users/ueli/.cache/c6plug/p3-spec-7children.py`.

## 2. Method (differs from census 5 in one point: explicit imports)

New engine `F/c6plug-tools/xsplit.py` (descendant of `modsplit.py`: same item scanner, region handling and visibility widening `widen2.py`):

- **Child imports are computed, never globbed.** For every moved item the engine collects the identifiers it reads from the module scope (not field or method access, not path tails, not declaration or binding names; names shadowed by `let`/`for`/closure/typed parameters count only when called; string literals inside attributes that name a function or module, such as `default = "…"`, `with = "…"`, `retire_with = "…"`, count; inline format arguments count). Each name is resolved to: a root item (`use super::Name;`), a sibling item (`use super::sibling::Name;`), or the original `use` path of the root binding (copied with its attributes). Imports are gated with the `#[cfg(…)]` of the defining item or, when every using item has one, of the users. Result is sorted by `rustfmt`.
- **Root re-exports are explicit lists**: `pub use child::{A, B};` for every `pub` item, `pub(crate) use` only when something else names it, `use` for private items the root or a glob consumer needs. cfg attributes of the items are kept.
- **Root `use` lines** are pruned to what the root, a glob consumer (mounted `use super::*` files: gb tests, pdf `canonical`/`octets`, gltf `canonical`) or a dark trait import still needs. Consumer-only bindings and re-exports get `#[cfg(test)]` when all consumers are test mounts.
- `SNAPSHOT_LEAVES` (vd, is, p3, rm): `rust: include_str!("🦀️.rs")` became `rust: concat!(include_str!("🦀️.rs"), include_str!("<child>/🦀️.rs"), …)` so the Rust leaf still carries the whole source (same pattern as `INTERPRETER_SOURCE` of census 5).
- Visibility: private names used in another file are widened to `pub(super)` (`widen2.py`); `pub(super)` of files that are modules is rebased with `nest`.
- `include!`/`include_str!` literals of moved items get one more `../` per level (p3 sqlite native fields include, is `📜️artifact-definition.json`).

## 3. Evidence (all run on the live tree after install; log `F/🗑️generated/c6plug-verify.log`)

| Check | Result |
| --- | --- |
| Live files equal the staged tree | 61 files, 0 differences |
| Lossless (`xsplit.py lossless`: line multiset orig vs new, minus declared generated and removed lines, region markers compared by count) | `LOSSLESS OK` x8, `missing 0`, `extra 0`; every removed or generated line is a `use`/wiring/doc/`include` literal/header/`}` of an inlined module (declared by the engine) |
| Structure (`vmod.py live`) | `VMOD OK`: every child mounted once, docstring emoji = folder emoji / unique / different from the root's / no sibling emoji clash, path length (max 244 UTF-8 bytes absolute, 176 chars, repo-relative at most 240 bytes), brace depth, rustfmt parse, include targets exist, `#[test]` multiset equal, module-name clash |
| Cross-file private leftovers (`defscan.py`) | 0 in all 8 |
| Name clash (`modclash.py`) | OK |
| Taxonomy | `rgi.mjs` `bad=0` for 53 folder names; `tax.ts` through the live `semanticDirectoryKindId`: 53 resolve (`crate-module-children`, fixtures `members-of-tests`), 0 unresolved. Two names first resolved to unrelated kinds (`🔤️fonts` → `fonts`, `📃️page` → `page`) and were renamed `🔤️font-model`, `📃️page-model` |
| Source-boundary gate (`census-own.ts`, canonical `readRustSourceClosureV1`, crate roots) | files / bytes / active lines ratio (of 32 / 262,144 / 4,000): en 7 / 0.396, vd 14 / 0.606, is 12 / 0.553, gb 14 to 17 / 0.531, **p3 28 to 29 / 0.934 (bytes bind)**, rm 19 to 20 / 0.843, pdf 12 to 13 / 0.575, gltf 18 / 0.579 |
| Formatting debt (`fmt.py`, rustfmt hunks original/new) | en 15/11, vd 31/30, is 32/15, gb 10/10, p3 8/6, rm 36/33, pdf 18/18, gltf 13/11; generated imports and wiring add no hunk |
| Scoped patched differential (`oracle6.py`, `rustc +nightly-2026-07-20 --emit=metadata` on the real crate root, original restored in a sandbox, lib and `--test`, the mounted io/test files pulled on demand) | see below |

**Differential reading.** The sandbox has no dependency crates, so unresolved imports of externals are errors on both sides; every child repeats the imports of the names it reads, so the new side has more of the same message. Everything below was classified from `c6plug-verify.log`:

| Tag | Only in new | Only in orig |
| --- | --- | --- |
| en | multiplicity of `unresolved import crate::snapshot` / `crate::mutations` (extern aliases removed by the sandbox), `E0603 committed is private` (artifact of the duplicate `committed`, section 5) | 2x `E0433 too many leading super` (fixed by the split, section 5) |
| vd | multiplicity of `unresolved import semio_s_plugin_norm_compliance` | `E0252 BTreeMap defined multiple times` and its unused-import warning (section 5) |
| is | multiplicity of the same import | `E0252 BTreeMap`, 4x `E0255 part_1/2/4/5 defined multiple times`, two unused-import warnings (section 5) |
| gb | multiplicity of `unresolved import store` and of `semio_s_artifact_procedural_2d_model` | none |
| p3 | multiplicity of `protocol`, `semio_framework_value_derive`, `semio_s_artifact_stdio_semio_brep`; `unresolved import machines_and_catalogs::MachineCatalogs` (the enum is produced by the `dyn_enum_close!` proc macro, which does not run in the sandbox) | none |
| rm | multiplicity of `semio_framework_value_derive`, `serde` | none |
| pdf | **none**: `errors 850/850` lib, `859/859` test, warnings equal (`ORACLE CLEAN`) | none |
| gltf | multiplicity of `serde`; 8/9 `unused import` warnings for the `default_*`/`is_*`/`present_json`/`ordered_attr_map` imports (the serde and value derives that read those attribute strings are unresolved in the sandbox, so the import looks unused there) | none |

The oracle caught one real defect in my output (pdf `📃️page` lacked `PdfTextArrayItem`, hidden by a `let` heuristic); fixed in the engine, all 8 stages re-generated, only that file changed. No other file changed between engine versions (`diff -r stage installed` empty).

Not run: the typeck-level oracle (`posrun.py`), `cargo`, `bun test` (the TS runtime cannot load `ajv`: `node_modules` is a dangling link). Missing trait imports for method calls (E0599) are not visible to the resolve-level differential; the only trait-only import in these files is `ToValue` in `en` (routed to `⚖️laws`), every other import is named in the code.

## 4. Source-text readers, registrations and pins changed

| Item | Change |
| --- | --- |
| `📏️lib-wiring-ratchet/🔣️.json` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/`) | 3 pins removed with `F/c6plug-tools/ratchet.py apply en vd is` (roots now 85, 108, 88 lines against budget 150). gb, p3, rm, gltf are not crate lib roots, pdf has no pin (root 50) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/…/🧱️base/🧬️schema/📸️snapshot/🧪️tests/♻️retirement/🟦️.ts` | read `📐️model/🦀️.rs` as plain text and asserts `retire_with="retire_pdf_*"` and `fn retire_pdf_*`; now `readModuleTreeSource(new URL(…))` with a relative import of `🌳️module-tree/🟦️.ts`. Python twin (`modtree.py`): 5 owners x 2 assertions hold over the tree (10 files) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/…/🧱️base/🧬️schema/🦀️.rs` (`register_schema_leaf_documents`) | lists every Rust source file of the snapshot export (the diff children are listed one by one); the 9 new `📐️model` children were added after the `📐️model/🦀️.rs` row |
| `✏️s/🔌️plugins/🌀️procedural/…/🌀️generation2d/…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` (crate `semio-s-artifact-procedural-2d-editor`, test `domain_local_static_verifier_rejects_raw_routes_and_proves_two_dimensional_coverage`) | read `🚪️io/💾️binary/📸️snapshot/🦀️.rs` as one text and asserts the mounted session route; now reads the root and the 5 children in mount order and concatenates. Twin: the four forbidden / three required / one scalar assertions give the same verdict before and after (the `decode_pack` and `decode_document` forbidden-word checks already fail on the original, section 5) |
| `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json` (p3 row) | no edit: the law reads `readModuleTreeSource(row.source)` and the `semio_framework_composition_admission::` token is in the tree |
| `📋️build-owners.json`, `🔒️dependencies.json`, Cargo manifests, `project.json`, `launch.json` | package-level rows only; no edit |
| `SNAPSHOT_LEAVES` rust leaf of vd, is, p3, rm | now the concatenated tree (section 2). No test or reader compares it with a file |

## 5. Pre-existing defects found (not caused by this split)

1. **en `fixtures` crate does not compile at HEAD**: two top-level functions named `committed` in the same module (private `fn committed(case, label, text)` at orig line 265 and `pub fn committed(case)` at 343): `E0428`. The split keeps both in `⚖️laws` (the error stays; the root `pub use laws::committed` then also reports a private-item error). Fix: rename the private one (3 call sites in `⚖️laws`), for example `committed_document`.
2. **en `super::KINDS` / `super::wire_probes()`** inside `assert_semantics` and the codec test (orig lines 444, 482) pointed above the crate root (`E0433`, left over from an inlined `laws` module). In the new layout `super` is the crate root, which re-exports `KINDS` and `wire_probes`, so the two errors disappear. The text is unchanged.
3. **vd and is**: `app_surface::artifact_kind_spec(…)` in `artifact_kind()` uses a name nothing imports (the root has `use semio_s_plugin_norm_app_surface;`, a crate-name import that binds nothing called `app_surface`): `E0433` in orig and new. Fix: `use semio_s_plugin_norm_app_surface as app_surface;` in the two roots.
4. **vd and is import duplicates (fixed by the split)**: `use std::collections::BTreeMap;` twice in the root (`E0252`), and in `is` `use crate::{part_1, part_2, part_4, part_5};` next to `pub mod part_N` (`E0255`). The engine drops a root binding that duplicates another binding or that imports a module the root itself defines, and declares it (`removed` in the log).
5. gb `native_codec` keeps its own `#[cfg(test)] use super::*;` (inline module of the root, unchanged). The `mounted-registry` test asserts `!contains("decode_pack")` over text that contains `decode_pack_with`, so it fails on the original and the new tree alike.

## 6. Crates to compile

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check \
  -p semio-s-artifact-energy-model-mutation-fixtures \
  -p semio-s-artifact-norm-vdi3805-model \
  -p semio-s-artifact-norm-iso16757-model \
  -p semio-s-artifact-procedural-2d-io-binary \
  -p semio-s-artifact-procedural-2d-mode-edit \
  -p semio-s-artifact-procedural-2d-mode-generate \
  -p semio-s-artifact-procedural-2d-editor \
  -p semio-s-artifact-process-process3d-model \
  -p semio-s-artifact-remodel-remodeling-model \
  -p semio-s-artifact-stdio-pdf-model \
  -p semio-s-artifact-stdio-pdf-schema-4 \
  -p semio-s-artifact-stdio-gltf-model-snapshot \
  --lib --tests --keep-going --message-format=short
```

`mode-edit` and `mode-generate` mount the generation2d binary snapshot file inside their `config` modules (`#[path="🚪️io/💾️binary/📸️snapshot/🦀️.rs"]`), `…-2d-editor` owns the re-pointed text test, `…-pdf-schema-4` owns the edited leaf registration. Expected pre-existing failures: en (`E0428`, item 1) and vd, is (`E0433 app_surface`, item 3). Dependents are unaffected: every `pub` item is re-exported under the same path.

## 7. Open risks

- **Nothing was compiled.** The resolve-level differential cannot see macro-generated names, derive-consumed attribute strings, method resolution or field privacy. Widening is by name occurrence (conservative: for example p3 `ContributedMachineCatalog.label` became `pub(super)`).
- p3 closure is at 0.934 of the byte gate (244,725 of 262,144); the byte count grew by about 9 KiB for imports and docstrings. Another split in that crate needs headroom.
- gb children are compiled in two more crates (`mode-edit`, `mode-generate`) under a different parent module; they use only `super::` siblings and extern-prelude paths, which exist in those crates for the original text too.
- The TS edit (pdf retirement test) and the Rust test edit (gb) are proven by Python twins only.
- `tax.ts` needed a path update (`🔐️taxonomy-authority/🟦️.ts`) because a peer is splitting the discovery library; the taxonomy json was invalid for a while (`generatorContracts[…].inputPatterns` ordering, not mine).

## 8. Tools (`F/c6plug-tools/`)

`xsplit.py` (stage, lossless, install, reinstall, uninstall, list), `usetree.py`, `oracle6.py` (real crate roots; `C6_STAGE_ORACLE=1` runs on the stage before install), `ratchet.py`, `readers2.py` (precise reader census), `census-own.ts` (`C6_FILES=` for crate roots), `specs/{en,vd,is,gb,p3,rm,pdf,gltf}.py`, snapshots `orig/*.rs`, plus the census 5 helpers (`vmod`, `modclash`, `defscan`, `fmt`, `rgi.mjs`, `tax.ts`, `modtree.py`). `fmt-tmp/` and `/Users/ueli/.cache/c6plug/` hold regenerable copies only.
