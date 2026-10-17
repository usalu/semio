# 📏️ Wave 7 Crate Source-Gate Census

Read-only census of every first-party Rust crate in every workspace against the compiler-ingress source boundary. Auditor: Haiku `cgc7`, fresh run on 2026-10-17. No source file, git state or cargo build touched; nothing run under cargo or rustc except `rustup which rustc` and `rustc --print cfg`.

## Method

- Reader: the canonical `readRustSourceClosureV1` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`) with the host cfg, `debug_assertions` and each crate's enabled features. The census tool is `🗑️generated/cgc7-census.ts`; the report is rendered by `🗑️generated/cgc7-report.ts`.
- Limits: 32 files, 262,144 bytes (`include_bytes!`/`include_str!` counted), 4,000 active lines. A crate's own `rust-source-boundary` metadata can only tighten them.
- Workspaces: all 287 `[workspace]` roots on disk (root, `🧰️framework`, `🌎️hub`, `🎓️teaching`, and every `✏️s/**` plugin/artifact/dev workspace). Crates: every `[package]` manifest outside `.🧬semio`, `🗑️generated`, `target`, `node_modules`, plus every path dependency reached from them.
- Lib closure: the lib, bins and build script with the non-test cfg. Test closure: the lib with `cfg(test)` plus every `tests/*.rs` (or `[[test]]`) target, also with `cfg(test)`.
- Dependency reach: lib reach follows normal and build dependencies. Test reach also starts from dev-dependencies. A crate is "blocked" by an oversized dependency when that dependency is in its lib reach (blocks lib) or its test reach (blocks test). Test-only oversize blocks nothing outside its own test build.
- Own vs pulled-in: "Own" is the oversized closure itself (test closure for test-only rows). "Pulled-in" is the number of first-party crates in the lib reach and the sum of their lib-closure bytes.
- Sort: blocked count (any), then lib before test-only, then ratio.

## Totals

| Metric | Count |
| --- | ---: |
| Workspaces (`[workspace]` roots) | 287 |
| Crates measured | 2419 |
| Workspace members | 2373 |
| Non-member crates reached through path dependencies | 46 |
| Compilable (lib closure within limits) | 1868 |
| Compilable (test closure within limits) | 1811 |
| Oversized lib (non-test closure over a limit) | 58 |
| Oversized test-only (test closure over, lib within limits) | 43 |
| of which sized over the limit | 34 |
| of which reader-refused (test closure cannot be produced) | 9 |
| Oversized lib and test together | 58 |
| Crates whose closure the reader refuses (any target) | 10 |
| Path dependencies resolving to a missing directory | 1 |
| Directories with a manifest but no `[package]` (virtual or skipped) | 144 |

Compilable counts exclude crates that depend on an oversized lib (their lib reach includes an oversized crate). The reader refusal covers crates whose closure cannot be read, mostly missing `include_str!` JSON inputs or compiler-generated Rust inputs; the ingress will refuse them as well, but this census cannot size them.

## Oversized crates (sorted by blocked count)

| # | Crate | Workspace | Kind | Files | Bytes | Active lines | Binding limit (ratio) | Own bytes (embedded) | Pulled-in crates / bytes | Blocks: any (lib / test) | Claim |
| ---: | --- | --- | --- | ---: | ---: | ---: | --- | --- | --- | --- | --- |
| 1 | semio-s-artifact-stdio-semio-mesh | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 118 | 449273 | 6306 | files (3.69) | 449273 (74915) | 331 / 23,265,769 | 314 (289 / 314) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 2 | semio-s-artifact-stdio-xml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml | lib | 122 | 636835 | 9359 | files (3.81) | 636835 (119449) | 341 / 23,679,931 | 301 (273 / 301) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 3 | semio-s-artifact-stdio-deflate | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate | lib | 74 | 352521 | 5701 | files (2.31) | 352521 (52412) | 335 / 23,377,986 | 300 (272 / 300) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 4 | semio-s-artifact-stdio-semio-drawing | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 120 | 517593 | 6757 | files (3.75) | 517593 (99429) | 332 / 23,290,994 | 299 (276 / 299) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 5 | semio-s-artifact-stdio-zip | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip | lib | 109 | 503942 | 7746 | files (3.41) | 503942 (71535) | 343 / 24,669,287 | 288 (261 / 288) | c4p16 reported at baseline (not carved) |
| 6 | semio-s-artifact-stdio-json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json | lib | 116 | 568082 | 8135 | files (3.63) | 568082 (63045) | 341 / 23,679,931 | 283 (254 / 283) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 7 | semio-s-artifact-stdio-semio-brep | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 109 | 549374 | 7143 | files (3.41) | 549374 (100202) | 341 / 24,082,288 | 246 (225 / 246) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 8 | semio-s-artifact-stdio-semio-model | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 97 | 388552 | 5522 | files (3.03) | 388552 (60667) | 332 / 23,290,994 | 219 (197 / 219) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 9 | semio-s-artifact-stdio-semio-image | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 97 | 285165 | 4462 | files (3.03) | 285165 (41174) | 332 / 23,290,994 | 199 (176 / 199) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 10 | semio-s-artifact-stdio-semio-object | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 93 | 236364 | 3509 | files (2.91) | 236364 (37083) | 345 / 25,264,982 | 195 (171 / 195) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 11 | semio-s-artifact-stdio-semio-kit | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 111 | 380090 | 4892 | files (3.47) | 380090 (82201) | 347 / 25,889,898 | 194 (170 / 194) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 12 | semio-s-artifact-stdio-semio-flow | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 102 | 324397 | 4979 | files (3.19) | 324397 (45673) | 333 / 23,808,587 | 188 (168 / 188) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 13 | semio-framework-3d-brep-engine | 🧰️framework | lib | 8 | 272018 | 3934 | bytes (1.04) | 272018 (0) | 102 / 7,366,269 | 177 (160 / 177) | unclaimed |
| 14 | semio-s-artifact-stdio-pdf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf | lib | 722 | 3744187 | 52805 | files (22.56) | 3744187 (440077) | 336 / 23,730,507 | 175 (151 / 175) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 15 | semio-s-artifact-stdio-semio-document | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 114 | 460138 | 6703 | files (3.56) | 460138 (59238) | 332 / 23,290,994 | 171 (148 / 171) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 16 | semio-s-artifact-stdio-svg | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg | lib | 145 | 645163 | 9703 | files (4.53) | 645163 (101651) | 342 / 24,316,766 | 169 (146 / 169) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 17 | semio-s-artifact-stdio-semio-cad | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 108 | 353605 | 4786 | files (3.38) | 353605 (61633) | 333 / 23,808,587 | 168 (145 / 168) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 18 | semio-s-artifact-stdio-semio-presentation | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 105 | 418809 | 5841 | files (3.28) | 418809 (64781) | 333 / 23,751,132 | 168 (145 / 168) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 19 | semio-s-artifact-stdio-semio-animation | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 99 | 315117 | 4586 | files (3.09) | 315117 (51529) | 332 / 23,290,994 | 168 (145 / 168) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 20 | semio-s-artifact-stdio-semio-graph | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 127 | 579721 | 8750 | files (3.97) | 579721 (83894) | 333 / 23,449,816 | 166 (143 / 166) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 21 | semio-s-artifact-stdio-semio | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio | lib | 150 | 513724 | 4880 | files (4.69) | 513724 (192505) | 385 / 31,104,378 | 165 (142 / 165) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 22 | semio-s-artifact-stdio-dxf | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf | lib | 84 | 627271 | 8626 | files (2.63) | 627271 (130992) | 341 / 23,679,931 | 157 (135 / 157) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 23 | semio-s-artifact-stdio-gltf-model | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf | lib | 69 | 612387 | 8194 | bytes (2.34) | 612387 (64169) | 347 / 24,816,261 | 139 (118 / 139) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 24 | semio-s-artifact-stdio-jpg | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg | lib | 99 | 417479 | 6384 | files (3.09) | 417479 (46683) | 335 / 23,377,986 | 82 (73 / 82) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 25 | semio-s-artifact-stdio-gltf-mutations | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf | lib | 131 | 732261 | 12910 | files (4.09) | 732261 (563) | 348 / 25,428,648 | 79 (64 / 79) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 26 | semio-s-artifact-stdio-gltf-inferences | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf | lib | 365 | 523640 | 6655 | files (11.41) | 523640 (159679) | 348 / 25,428,648 | 77 (62 / 77) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 27 | semio-s-artifact-stdio-tiff | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff | lib | 116 | 487946 | 5819 | files (3.63) | 487946 (86667) | 354 / 26,267,842 | 60 (51 / 60) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 28 | semio-s-artifact-gis-gismap | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap | lib | 203 | 1058280 | 12314 | files (6.34) | 1058280 (316604) | 452 / 43,096,086 | 36 (27 / 36) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 29 | semio-s-artifact-gis-gisterrain | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain | lib | 100 | 294291 | 4431 | files (3.13) | 294291 (28675) | 486 / 48,701,511 | 30 (22 / 30) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 30 | semio-s-plugin-wfc-engine | ✏️s/🔌️plugins/🀄️wfc | lib | 43 | 398750 | 9356 | lines (2.34) | 398750 (0) | 192 / 13,318,485 | 13 (13 / 13) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 31 | semio-s-artifact-trinity-jack | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack | lib | 132 | 716259 | 12082 | files (4.13) | 716259 (39227) | 340 / 24,019,860 | 12 (11 / 12) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 32 | semio-s-artifact-sourcing-curation | ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation | lib | 128 | 491278 | 6943 | files (4.00) | 491278 (69530) | 420 / 35,646,416 | 12 (12 / 12) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 33 | semio-s-artifact-vcs-vcs | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs | lib | 128 | 321118 | 4833 | files (4.00) | 321118 (48307) | 352 / 26,213,813 | 10 (7 / 10) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 34 | semio-s-artifact-space-home | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home | lib | 112 | 369320 | 5467 | files (3.50) | 369320 (24928) | 450 / 34,089,447 | 6 (5 / 6) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 35 | semio-s-artifact-writer-writer | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer | lib | 155 | 428307 | 7483 | files (4.84) | 428307 (21141) | 432 / 40,881,162 | 5 (4 / 5) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 36 | semio-s-artifact-sequence-sequence | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence | lib | 93 | 465539 | 7657 | files (2.91) | 465539 (30995) | 483 / 39,584,397 | 5 (4 / 5) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 37 | semio-s-plugin-flow-brep | ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep | lib | 7 | 303720 | 4040 | bytes (1.16) | 303720 (3440) | 497 / 37,803,518 | 5 (3 / 5) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 38 | semio-s-artifact-puzzle-2d | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d | lib | 340 | 2643423 | 25218 | files (10.63) | 2643423 (991722) | 489 / 42,094,001 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 39 | semio-s-artifact-puzzle-5d | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d | lib | 329 | 1953983 | 26190 | files (10.28) | 1953983 (371001) | 512 / 45,802,787 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 40 | semio-s-artifact-note-note | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note | lib | 248 | 820105 | 12501 | files (7.75) | 820105 (103173) | 399 / 34,906,460 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 41 | semio-s-artifact-shooting-shooting | ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting | lib | 219 | 667228 | 9695 | files (6.84) | 667228 (118769) | 431 / 38,781,606 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 42 | semio-s-artifact-forms-forms | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms | lib | 201 | 718945 | 10479 | files (6.28) | 718945 (73818) | 503 / 42,549,102 | 4 (3 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 43 | semio-s-artifact-animate-presentation | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation | lib | 170 | 722436 | 14096 | files (5.31) | 722436 (61834) | 479 / 44,162,205 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 44 | semio-s-artifact-mathematical-equation | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation | lib | 169 | 841766 | 14789 | files (5.28) | 841766 (82132) | 411 / 33,600,456 | 4 (4 / 4) | c4p17 reported (needs carve, not claimed) |
| 45 | semio-s-artifact-trinity-rewriting | ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting | lib | 161 | 499196 | 7068 | files (5.03) | 499196 (62812) | 492 / 46,243,161 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 46 | semio-s-artifact-wfc-3d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d | lib | 147 | 539917 | 8867 | files (4.59) | 539917 (63316) | 346 / 24,802,797 | 4 (4 / 4) | c4p17 reported (needs carve, not claimed) |
| 47 | semio-s-artifact-wfc-2d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d | lib | 138 | 563910 | 8624 | files (4.31) | 563910 (100288) | 449 / 34,245,653 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 48 | semio-s-artifact-wfc-bitmap | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap | lib | 119 | 528893 | 8725 | files (3.72) | 528893 (51864) | 322 / 23,125,571 | 4 (4 / 4) | c4p17 reported (needs carve, not claimed) |
| 49 | semio-s-artifact-dag-dag | ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag | lib | 112 | 317308 | 5041 | files (3.50) | 317308 (30426) | 495 / 42,371,002 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 50 | semio-s-artifact-wfc-grid2d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d | lib | 110 | 508995 | 8348 | files (3.44) | 508995 (59443) | 349 / 25,241,533 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 51 | semio-s-artifact-wfc-grid3d | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d | lib | 110 | 499575 | 7841 | files (3.44) | 499575 (74086) | 407 / 29,935,597 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 52 | semio-s-artifact-reasoning-wires | ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires | lib | 99 | 331671 | 4830 | files (3.09) | 331671 (27030) | 473 / 38,367,211 | 4 (4 / 4) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 53 | semio-s-artifact-bim-model | ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model | lib | 1300 | 6287123 | 88482 | files (40.63) | 6287123 (1302633) | 363 / 31,534,019 | 3 (3 / 3) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 54 | semio-s-artifact-architect-program | ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program | lib | 179 | 1604545 | 23545 | bytes (6.12) | 1604545 (242905) | 418 / 35,406,429 | 3 (3 / 3) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 55 | semio-s-artifact-flow-flow | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow | lib | 131 | 530825 | 8480 | files (4.09) | 530825 (18155) | 504 / 41,412,133 | 3 (3 / 3) | artifact-flow-flow (FLEET-CLAIMS, W4 in progress) |
| 56 | semio-s-artifact-demonstrator-playground | ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground | lib | 74 | 160138 | 2435 | files (2.31) | 160138 (29904) | 360 / 26,839,680 | 3 (3 / 3) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 57 | semio-s-artifact-energy-model-schema | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model | lib | 57 | 269592 | 212 | files (1.78) | 269592 (258146) | 345 / 24,041,894 | 3 (3 / 3) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 58 | semio-s-artifact-stdio-tiff-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust | test-only | 12 | 17545228 | 1069 | bytes (66.93) | 17545228 (-) | 2 / 69,812 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 59 | semio-framework-plugin | . | test-only | 368 | 4310854 | 45731 | bytes (16.45) | 4310854 (-) | 318 / 22,417,988 | 0 (0 / 0) | pi2 (PLUGIN-INTEGRATION-2; test closure per lead) |
| 60 | semio-s-artifact-stdio-ifc-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust | test-only | 19 | 3483962 | 3032 | bytes (13.29) | 3483962 (-) | 3 / 73,381 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 61 | semio-s-artifact-stdio-mp4-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust | test-only | 3 | 2746784 | 504 | bytes (10.48) | 2746784 (-) | 2 / 69,812 | 0 (0 / 0) | c4p16 reported at baseline (not carved) |
| 62 | semio-s-artifact-layout-layout-mutations-frames | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout | test-only | 202 | 593071 | 5249 | files (6.31) | 593071 (-) | 323 / 22,840,041 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 63 | semio-s-artifact-layout-layout-mutations-content | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout | test-only | 167 | 473802 | 4605 | files (5.22) | 473802 (-) | 323 / 22,840,041 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 64 | semio-framework-plugin-child-group-wire-laws | 🧰️framework | test-only | 42 | 1175981 | 1965 | bytes (4.49) | 1175981 (-) | 0 / 0 | 0 (0 / 0) | pi2 (PLUGIN-INTEGRATION-2; test closure per lead) |
| 65 | semio-s-artifact-draw-drawing-scene | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 78 | 991269 | 2412 | bytes (3.78) | 991269 (-) | 323 / 23,072,549 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 66 | semio-framework-expression | 🧰️framework | test-only | 5 | 886692 | 225 | bytes (3.38) | 886692 (-) | 0 / 0 | 0 (0 / 0) | unclaimed |
| 67 | semio-s-artifact-draw-drawing-transform | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 78 | 152742 | 1806 | files (2.44) | 152742 (-) | 324 / 23,132,567 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 68 | semio-s-artifact-draw-drawing-editor | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 51 | 601238 | 5618 | bytes (2.29) | 601238 (-) | 398 / 35,777,942 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 69 | semio-s-artifact-draw-drawing-style | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 69 | 122220 | 1277 | files (2.16) | 122220 (-) | 324 / 23,132,567 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 70 | semio-s-artifact-stdio-jpg-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust | test-only | 5 | 520017 | 715 | bytes (1.98) | 520017 (-) | 2 / 69,812 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 71 | semio-s-artifact-stdio-pptx-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust | test-only | 12 | 505487 | 1589 | bytes (1.93) | 505487 (-) | 3 / 204,657 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 72 | semio-framework-time-travel | 🧰️framework | test-only | 9 | 470603 | 1957 | bytes (1.79) | 470603 (-) | 48 / 2,942,619 | 0 (0 / 0) | pi2 (time-travel listed in its dispatch) |
| 73 | semio-s-artifact-draw-drawing-editor-commands | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 51 | 220258 | 1743 | files (1.59) | 220258 (-) | 349 / 25,181,117 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 74 | semio-s-artifact-draw-drawing-write | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 26 | 406710 | 1955 | bytes (1.55) | 406710 (-) | 392 / 35,215,539 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 75 | semio-s-artifact-draw-drawing-editor-interaction | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 48 | 401313 | 3112 | bytes (1.53) | 401313 (-) | 346 / 24,849,268 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 76 | semio-s-artifact-layout-layout-editor-commands | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout | test-only | 46 | 263763 | 4482 | files (1.44) | 263763 (-) | 341 / 24,167,157 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 77 | semio-s-artifact-draw-drawing-model | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 46 | 302551 | 3381 | files (1.44) | 302551 (-) | 321 / 22,792,495 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 78 | semio-s-artifact-draw-drawing-host | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 27 | 362035 | 4818 | bytes (1.38) | 362035 (-) | 347 / 25,040,316 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 79 | semio-framework-3d-brep-boolean | 🧰️framework | test-only | 8 | 344819 | 4000 | bytes (1.31) | 344819 (-) | 91 / 6,320,537 | 0 (0 / 0) | unclaimed |
| 80 | semio-framework-3d-mesh-tests | 🧰️framework | test-only | 6 | 339178 | 779 | bytes (1.29) | 339178 (-) | 0 / 0 | 0 (0 / 0) | unclaimed |
| 81 | semio-s-artifact-draw-drawing-structure | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 41 | 105552 | 946 | files (1.28) | 105552 (-) | 324 / 23,132,567 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 82 | semio-s-artifact-stdio-docx-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust | test-only | 12 | 327120 | 1993 | bytes (1.25) | 327120 (-) | 3 / 204,657 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 83 | semio-s-artifact-draw-drawing-geometry | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing | test-only | 39 | 242463 | 1948 | files (1.22) | 242463 (-) | 322 / 22,972,520 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 84 | semio-s-artifact-stdio-pptx | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx | test-only | 31 | 294111 | 868 | bytes (1.12) | 294111 (-) | 352 / 25,851,382 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 85 | teaching-proctor | 🎓️teaching | lib | 11 | 238110 | 4457 | lines (1.11) | 238110 (0) | 58 / 3,612,544 | 0 (0 / 0) | unclaimed |
| 86 | semio-framework-os-kernel | . | test-only | 31 | 288289 | 2225 | bytes (1.10) | 288289 (-) | 182 / 12,813,021 | 0 (0 / 0) | os-kernel carve (FLEET-CLAIMS, WP2-WP8 pending) |
| 87 | semio-framework-number | 🧰️framework | test-only | 18 | 143601 | 4325 | lines (1.08) | 143601 (-) | 0 / 0 | 0 (0 / 0) | unclaimed |
| 88 | semio-framework-plugin-publication-mounted | . | test-only | 26 | 283026 | 2381 | bytes (1.08) | 283026 (-) | 267 / 17,554,269 | 0 (0 / 0) | pi2 (PLUGIN-INTEGRATION-2; test closure per lead) |
| 89 | semio-s-artifact-block-2d-editor | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d | test-only | 34 | 145736 | 2240 | files (1.06) | 145736 (-) | 356 / 25,473,312 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 90 | semio-s-artifact-block-5d-editor | ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d | test-only | 34 | 140156 | 2220 | files (1.06) | 140156 (-) | 356 / 26,007,968 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 91 | semio-s-artifact-stdio-ply-inferences | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply | test-only | 33 | 209841 | 3062 | files (1.03) | 209841 (-) | 341 / 23,679,931 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 92 | semio-framework-3d-mesh-document | 🧰️framework | test-only | 28 | 264421 | 3427 | bytes (1.01) | 264421 (-) | 80 / 5,281,109 | 0 (0 / 0) | unclaimed |
| 93 | semio-hub-artifact-creation | 🌎️hub | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 253 / 16,891,621 | 0 (0 / 0) | unclaimed |
| 94 | semio-hub-stdio | 🌎️hub | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 646 / 59,123,370 | 0 (0 / 0) | unclaimed |
| 95 | semio-hub-directory-laws | 🌎️hub | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 0 / 0 | 0 (0 / 0) | unclaimed |
| 96 | semio-hub-directory-backend-laws | 🌎️hub | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 0 / 0 | 0 (0 / 0) | unclaimed |
| 97 | semio-s-artifact-procedural-2d-io-sqlite | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 424 / 30,847,471 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 98 | semio-s-artifact-stdio-pdf-test-oracle | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 3 / 204,657 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 99 | semio-s-composition-laws | ✏️s/🧑‍💻dev/🧩️composition | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 192 / 13,329,693 | 0 (0 / 0) | unclaimed |
| 100 | semio-s-artifact-procedural-3d-io-sqlite | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 522 / 37,419,633 | 0 (0 / 0) | plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) |
| 101 | semio-framework-os-shell | . | test-only | 0 | 0 | 0 | refused (refused) | 0 (-) | 230 / 15,022,699 | 0 (0 / 0) | unclaimed |

Reader-refused rows (ratio shown as "refused"):

- `semio-hub-artifact-creation` (test-only): semio_hub_artifact_creation: Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️artifact-genesis-v1/🔣️.json
- `semio-hub-stdio` (test-only): native_openable_provider: Missing Rust source input: 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json
- `semio-hub-directory-laws` (test-only): semio_hub_directory_laws: Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json
- `semio-hub-directory-backend-laws` (test-only): semio_hub_directory_backend_laws: Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json
- `semio-s-artifact-procedural-2d-io-sqlite` (test-only): semio_s_artifact_procedural_2d_io_sqlite: Missing Rust source input: ✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🧪️tests/🧫️fixtures/📏️semantic-cells.json
- `semio-s-artifact-stdio-pdf-test-oracle` (test-only): semio_s_artifact_stdio_pdf_test_oracle: Generated Rust input requires compiler ownership: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs
- `semio-s-composition-laws` (test-only): generation3d-app-laws: Missing or ambiguous Rust module: laws
- `semio-s-artifact-procedural-3d-io-sqlite` (test-only): semio_s_artifact_procedural_3d_io_sqlite: Missing Rust source input: ✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🧪️tests/🧫️fixtures/📏️semantic-cells.json
- `semio-framework-os-shell` (test-only): semio_framework_os_shell: Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🤖️generated/🟦️.ts

## Family summary

| Family | Crates | Oversized lib | Oversized test-only (sized) | Reader-refused test-only |
| --- | ---: | ---: | ---: | ---: |
| plugin (✏️s/🔌️plugins) | 1364 | 56 | 24 | 3 |
| root | 520 | 0 | 3 | 1 |
| framework | 354 | 1 | 7 | 0 |
| hub | 152 | 0 | 0 | 4 |
| ✏️s other | 28 | 0 | 0 | 1 |
| teaching | 1 | 1 | 0 | 0 |

## Claims from running slices

| Claim status | Oversized crates |
| --- | ---: |
| plugin wave 0 (FLEET-CLAIMS generic, unverified per crate) | 77 |
| unclaimed | 13 |
| c4p17 reported (needs carve, not claimed) | 3 |
| pi2 (PLUGIN-INTEGRATION-2; test closure per lead) | 3 |
| c4p16 reported at baseline (not carved) | 2 |
| artifact-flow-flow (FLEET-CLAIMS, W4 in progress) | 1 |
| pi2 (time-travel listed in its dispatch) | 1 |
| os-kernel carve (FLEET-CLAIMS, WP2-WP8 pending) | 1 |

Claim sources: `📓️FLEET.md` (last 60 lines; `c4p17` reported `equation`, `wfc-bitmap`, `wfc-3d`; `pi2` owns `semio-framework-plugin`), `📓️FLEET-CLAIMS.md` (plugin wave 0 generic), and the census-4 log lines (c4p16 zip and mp4-test-oracle at baseline). A "reported" status means the slice named the crate but did not claim a carve; it is not a claim on the crate.

## Top 15 oversized crates

1. semio-s-artifact-stdio-semio-mesh: 314 blocked, lib, 449273 bytes, 118 files, 6306 lines
2. semio-s-artifact-stdio-xml: 301 blocked, lib, 636835 bytes, 122 files, 9359 lines
3. semio-s-artifact-stdio-deflate: 300 blocked, lib, 352521 bytes, 74 files, 5701 lines
4. semio-s-artifact-stdio-semio-drawing: 299 blocked, lib, 517593 bytes, 120 files, 6757 lines
5. semio-s-artifact-stdio-zip: 288 blocked, lib, 503942 bytes, 109 files, 7746 lines
6. semio-s-artifact-stdio-json: 283 blocked, lib, 568082 bytes, 116 files, 8135 lines
7. semio-s-artifact-stdio-semio-brep: 246 blocked, lib, 549374 bytes, 109 files, 7143 lines
8. semio-s-artifact-stdio-semio-model: 219 blocked, lib, 388552 bytes, 97 files, 5522 lines
9. semio-s-artifact-stdio-semio-image: 199 blocked, lib, 285165 bytes, 97 files, 4462 lines
10. semio-s-artifact-stdio-semio-object: 195 blocked, lib, 236364 bytes, 93 files, 3509 lines
11. semio-s-artifact-stdio-semio-kit: 194 blocked, lib, 380090 bytes, 111 files, 4892 lines
12. semio-s-artifact-stdio-semio-flow: 188 blocked, lib, 324397 bytes, 102 files, 4979 lines
13. semio-framework-3d-brep-engine: 177 blocked, lib, 272018 bytes, 8 files, 3934 lines
14. semio-s-artifact-stdio-pdf: 175 blocked, lib, 3744187 bytes, 722 files, 52805 lines
15. semio-s-artifact-stdio-semio-document: 171 blocked, lib, 460138 bytes, 114 files, 6703 lines

## Caveats

- The tree moved while the census ran (peer carving continues). Per-crate numbers differ from the 2026-10-16 CSV (`🗑️generated/crate-census.csv`), which is root-only and has no test closure; compare only after a fresh run.
- The reader refusal is a reader limit, not a proof of compiler refusal. Missing JSON `include_str!` inputs (some with emoji-only file names) and compiler-generated inputs are the observed causes.
- One path dependency points at a missing directory (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧿️test-support/📦️packages/🦀️rust`); the crates behind it are not measured.
- Outputs: `🗑️generated/cgc7-crate-census.csv` (one row per crate, all columns), `🗑️generated/cgc7-census-summary.json` (totals, oversized list, per-workspace counts), `🗑️generated/cgc7-census.out`, `🗑️generated/cgc7-census.err`.
