# Result CRATE-HEADROOM (prefix `hdr`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/hdr-tools/` (`hdr-census.ts` full census with a fresh manifest list, `closure.ts`/`cen.sh` per-crate file lister and quick census, `classify.py` threshold + owner classification, `fixt.ts` run-time fixture rewriter, `install-reader.py`, `done.py` status recorder, `report.py` regenerates this file from `status.json`, `pos/` copy of the typeck oracle with sandboxes under `/Users/ueli/.cache/hdr/`). Logs: `F/🗑️generated/hdr-*`.

## 0. Verdict

**PARTIAL (static evidence only; no `COMPILE ADMITTED` line in `📓️FLEET.md`).** Start (15:1x census, 2,873 crates): 241 crates at >= 0.90 of a limit. Final census (20:0x, 3,033 crates, 62 of them new from this slice): **122 of the 241 are now below 0.90; 155 crates are still >= 0.90 (142 free, 13 owned by running slices: r9a bim-model, r8a puzzle-2d and puzzle-5d, bd10 raster and process3d-host; r5b pdf and r7c writer/curation/sequence finished and their crates are mine now); none of the 62 new crates is above 0.90.** About 105 of the 142 free crates sit exactly at 29 of 32 files (0.906) because the carve generators packed to 29: they are one file over the 0.90 line, not near the gate. Fixed since the previous version of this file (all `--check-applied` exit 0 with onlyCombined 0/0, section 4): the coordinator hand-overs `stdio-pdf-tables` (0.95 -> 0.48, Adobe Glyph List split into `stdio-pdf-glyph-list`), `flow-flow-unit-tests` (0.97 -> 0.54, harness behind `testkit`, companion `unit-tests-2` 0.44), `layout-layout-editor-commands` (lib 32/32 -> 0.625, its `-2` part 0.50), `semio-framework-dsl` (1.001 -> 0.639), `os-directory-client-tests`, plugin `tests-05` and `tests-07-2` (0.938 -> 0.64, new bin `tests-07-3`), `tests-18` (0.906 -> 0.78), plus three crates found over or at the gate after the carves landed: `stdio-semio-presentation-schema` (1.125 -> 0.31), `sequence-sequence-schema` (1.000 -> 0.59), `stdio-semio-document-schema` (0.969 -> 0.38). Not fixed: `job` (0.991), `3d-brep-boolean` (0.988), `process3d-host` (1.084, bd10), the white-box hub roots and the mutation-aggregate crates in section 5.

Techniques applied so far (each lossless, tool-driven, no compat shims):
1. Descriptor facets served at run time: `new leaves <descriptor.rs> --rust` (generator extended to the `inference` export, test added) plus the default `SchemaLeafReader` install; `resolved_leaf_body` added to the schema registry (+ unit test) and used by the component's GraphQL SDL readers so externalised leaves stay served.
2. Embedded test fixtures read at run time (`fixt.ts` over the generator's tested `fixturesToRuntime`): 150+ test files, every fixture existence and sha256 logged in `F/🗑️generated/hdr-fixt-*.log`; `include!`d fragments and one file shared with the pi2 closure were skipped/reverted.
3. Public-API tests moved to `[[test]]` targets (dsl-record).
4. Leaf-family crates split into sibling part crates (`famsplit.py`; flat, wrapper-chain and inline-module shapes; leaf-reference components kept together; `[[test]]` case targets redistributed by `fixtests.py`); the old crate re-exports the moved leaf modules with an explicit `pub use`, so no consumer changes. Parts are registered in the artifact workspace (member + dependency table) and the owner rows; `new crates <pkg> --check-applied` is the layout gate (see section 3).
5. Companion test bins (`semio-framework-plugin-tests-NN`) split by test root into `-2`/`-3` bins (`famsplit.py --sibling --no-reexport`), registered as root-workspace members with project.json, owner rows and six launch routes each.
6. Framework carves: `semio-framework-dsl-syntax` (grammar model, lexer, parsers) out of `semio-framework-dsl`, registered by `newfw.py`; typeck oracle `pos` family `hdr-dsl`: NEW 0 / GONE 0 for lib and test, negative control caught.
7. Descriptor facets also for app descriptors: the `new leaves` generator now serves the `config` and `presence` exports of an `AppSchemaDescriptor` (test added, crate-tree suite 101 pass); the component's app GraphQL SDL reader resolves them through `resolved_leaf_body`. Applied to block 2d/5d editors and the animate presentation viewer; the artifact descriptors of writer, flow, sequence, semio presentation and semio document were converted the same way.
8. Harness behind the testkit idiom (flow unit tests): the 220-line `context` module became its own file mounted as `unit_tests::context` under `cfg(any(test, feature = "testkit"))`; `testkit` forwards the dev-only features of plugin (`artifact-app-testing`) and os-kernel (`protocol-laws`, `mutation-testing`); the companion crate dev-depends on it with the feature (`flowsplit.py`).
9. Wiring roots: helper code that famsplit copied into a part root was moved into child modules (`layout commands-2`: `actions`, `interaction`, `native`), the descriptor functions of the generation2d and lowpoly schema roots into child modules, so no pinned lib-wiring root grew; the ratchet pin of the old layout commands root was removed (52 lines now).
10. Plugin companion bin split by mounted child roots (`binsplit.py`): the time-travel test root mounts `gesture` and `folder-reload-route` as independent children; they now live in `plugin-tests-07-3` (member, project.json, owner row, six routes in launch.json and the seed).

## 1. Crates at >= 0.90 of any limit, not owned by a running slice (151)

Before columns: lib and test closure as files/bytes/active lines (cgc7 reader, `hdr-census.ts`, 2026-10-17 15:1x, 2,873 crates). Ratio = worst of the three limits over lib and test, binding limit in brackets. `after` is the re-measured worst ratio.

| crate | lib | test | before | state | after | note |
| --- | --- | --- | --- | --- | --- | --- |
| `F:job` | 23/223519/2367 | 27/259692/2647 | 0.991 (bytes) | PARTIAL | 0.991 (lib 23/223519/2367; test 27/259692/2647) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:vcs-vcs-editor` | 25/117041/2027 | 31/186692/3184 | 0.969 (files) | OPEN | 0.969 (lib 25/117041/2027; test 31/186692/3184) |  |
| `A:forms-forms-commands` | 31/78702/1534 | 31/78702/1559 | 0.969 (files) | OPEN | 0.969 (lib 31/78702/1534; test 31/78702/1559) |  |
| `A:stdio-pptx-base-model` | 30/155140/2770 | 31/156317/2819 | 0.969 (files) | PARTIAL | 0.969 (lib 30/155140/2770; test 31/156317/2819) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:stdio-semio-table-mutations` | 30/63237/1126 | 31/74250/1346 | 0.969 (files) | PARTIAL | 0.969 (lib 30/63218/1126; test 31/74231/1346) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) | fixtures read at run time through the shared test-fixtures reader macros (fixt.ts, dev-dependency added) |
| `A:stdio-semio-document-examples` | 25/115928/2282 | 31/121112/2426 | 0.969 (files) | TODO |  |  |
| `A:stdio-semio-audio-mutations` | 30/49908/947 | 31/56901/1097 | 0.969 (files) | PARTIAL | 0.969 (lib 30/49889/947; test 31/56882/1097) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:repo-model` | 10/141350/3874 | 10/141350/3874 | 0.969 (lines) | OPEN | 0.969 (lib 10/141350/3874; test 10/141350/3874) |  |
| `semio-s-plugin-energy-engine-sim` | 7/149581/3019 | 8/195985/3871 | 0.968 (lines) | OPEN | 0.968 (lib 7/149581/3019; test 8/195985/3871) |  |
| `A:stdio-svg-snapshot` | 20/240647/3842 | 20/240647/3870 | 0.968 (lines) | OPEN | 0.968 (lib 20/240661/3842; test 20/240661/3870) |  |
| `F:os-kernel-db-sync` | 1/141733/2666 | 2/205392/3855 | 0.964 (lines) | PARTIAL | 0.964 (lib 1/141733/2666; test 2/205392/3855) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:gis-gismap-gis2d` | 15/152707/2401 | 20/242185/3849 | 0.962 (lines) | OPEN | 0.962 (lib 15/152707/2401; test 20/242185/3849) |  |
| `A:layout-layout-engine-export` | 1/122435/2338 | 2/204795/3846 | 0.962 (lines) | OPEN | 0.962 (lib 1/122371/2338; test 2/204731/3846) |  |
| `F:ui-prepared` | 1/144127/2584 | 4/215214/3832 | 0.958 (lines) | PARTIAL | 0.958 (lib 1/144127/2584; test 4/215214/3832) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:3d-brep-construction` | 6/155549/2870 | 14/210117/3832 | 0.958 (lines) | PARTIAL | 0.958 (lib 6/155549/2870; test 14/210117/3832) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:plugin-instance-app` | 9/241072/3057 | 10/250865/3171 | 0.957 (bytes) | PARTIAL | 0.957 (lib 9/241072/3057; test 10/250865/3171) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:quiz-tests` | 1/786/0 | 15/250510/3740 | 0.956 (bytes) | OPEN | 0.956 (lib 1/786/0; test 15/250510/3740) |  |
| `F:os-renderer-wgpu-dock` | 2/109951/2158 | 14/205482/3806 | 0.952 (lines) | PARTIAL | 0.944 (lib 2/109951/2158; test 7/203725/3777) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:procedural-3d-editor` | 4/217389/3052 | 6/249012/3613 | 0.950 (bytes) | PARTIAL | 0.949 (lib 4/217197/3050; test 6/248820/3611) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:ui-contract-values` | 6/126908/2931 | 13/169495/3797 | 0.949 (lines) | PARTIAL | 0.949 (lib 6/126908/2931; test 13/169495/3797) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:puzzle-3d-editor` | 9/162668/2013 | 15/248230/3550 | 0.947 (bytes) | OPEN | 0.947 (lib 9/162603/2012; test 15/248230/3550) |  |
| `A:cad-cad` | 1/8914/130 | 16/247567/3047 | 0.944 (bytes) | OPEN | 0.945 (lib 1/8914/130; test 16/247598/3047) |  |
| `A:wfc-bitmap-bitmap` | 18/204478/3545 | 22/216586/3763 | 0.941 (lines) | OPEN | 0.941 (lib 18/204478/3545; test 22/216586/3763) |  |
| `A:reasoning-wires-wires` | 14/111150/1869 | 21/246636/3693 | 0.941 (bytes) | OPEN | 0.940 (lib 14/111150/1869; test 21/246393/3691) |  |
| `F:os-mcp-hub-remote` | 10/135734/2324 | 12/223104/3757 | 0.939 (lines) | OPEN | 0.939 (lib 10/135734/2324; test 12/223104/3757) |  |
| `A:mathematical-equation-snapshot` | 20/179996/3307 | 28/196715/3751 | 0.938 (lines) | OPEN | 0.938 (lib 20/179996/3307; test 28/196715/3751) |  |
| `A:stdio-gif-editor` | 18/84475/1351 | 30/123118/2027 | 0.938 (files) | OPEN | 0.938 (lib 18/84475/1351; test 30/123118/2027) |  |
| `A:stdio-binary-snapshot` | 27/71990/904 | 30/94526/1027 | 0.938 (files) | OPEN | 0.938 (lib 27/71971/904; test 30/94507/1027) |  |
| `A:stdio-xlsx-transitional` | 22/84337/1487 | 30/107312/1849 | 0.938 (files) | PARTIAL | 0.938 (lib 22/84337/1487; test 30/107312/1849) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:stdio-docx-base-mutations` | 30/224915/3364 | 30/224915/3478 | 0.938 (files) | OPEN | 0.938 (lib 30/224985/3364; test 30/224985/3478) |  |
| `A:stdio-xml-mutations` | 23/62470/1184 | 30/79576/1532 | 0.938 (files) | OPEN | 0.938 (lib 23/62470/1184; test 30/79576/1532) |  |
| `A:stdio-semio-text-mutations` | 27/55101/998 | 30/65000/1199 | 0.938 (files) | PARTIAL | 0.938 (lib 27/55082/998; test 30/64981/1199) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) | fixtures read at run time through the shared test-fixtures reader macros (fixt.ts, dev-dependency added) |
| `F:ui-contract` | 16/136838/2968 | 22/174144/3749 | 0.937 (lines) | PARTIAL | 0.937 (lib 16/136838/2968; test 22/174144/3749) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:value-intrinsic` | 16/158380/2640 | 22/237122/3745 | 0.936 (lines) | PARTIAL | 0.936 (lib 16/158380/2640; test 22/237122/3745) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:stdio-dwg-sqlite-objects` | 18/244909/926 | 18/244909/926 | 0.934 (bytes) | OPEN | 0.934 (lib 18/244909/926; test 18/244909/926) |  |
| `A:gis-gismap-demo-asset` | 2/243819/5 | 2/243819/5 | 0.930 (bytes) | OPEN | 0.930 (lib 2/243819/5; test 2/243819/5) |  |
| `A:cad-cad-play` | 2/243202/191 | 2/243202/191 | 0.928 (bytes) | OPEN | 0.928 (lib 2/243230/191; test 2/243230/191) |  |
| `A:stdio-dwg-read-objects` | 6/242787/3187 | 6/242787/3187 | 0.926 (bytes) | OPEN | 0.926 (lib 6/242787/3187; test 6/242787/3187) |  |
| `A:stdio-dxf-diff` | 21/208121/3693 | 21/208121/3693 | 0.923 (lines) | OPEN | 0.923 (lib 21/208233/3693; test 21/208233/3693) |  |
| `F:os-kernel-db-io-driver` | 1/91649/1761 | 3/188087/3693 | 0.923 (lines) | PARTIAL | 0.923 (lib 1/91649/1761; test 3/188087/3693) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:process-process3d-model` | 24/232306/3238 | 25/241779/3419 | 0.922 (bytes) | PARTIAL | 0.922 (lib 24/232302/3238; test 25/241775/3419) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `F:3d-mesh-document` | 12/176268/2997 | 25/241713/3440 | 0.922 (bytes) | OPEN | 0.921 (lib 12/176268/2997; test 25/241379/3438) |  |
| `F:plugin-interpreter` | 3/175356/3293 | 6/216295/3687 | 0.922 (lines) | PARTIAL | 0.922 (lib 3/175356/3293; test 6/216295/3687) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:puzzle-5d-interaction` | 20/198851/3398 | 22/218727/3679 | 0.920 (lines) | TODO |  |  |
| `F:3d-brep-queries` | 4/142032/2927 | 10/180016/3679 | 0.920 (lines) | OPEN | 0.920 (lib 4/142032/2927; test 10/180016/3679) |  |
| `F:plugin-child-group-wire-laws` | 0/0/0 | 27/241181/1975 | 0.920 (bytes) | OPEN | 0.920 (test 27/241181/1975) |  |
| `F:value-derive` | 9/227493/3551 | 11/237023/3676 | 0.919 (lines) | PARTIAL | 0.919 (lib 9/227493/3551; test 11/237023/3676) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) | fixtures read at run time through the shared test-fixtures reader macros (fixt.ts, dev-dependency added) |
| `F:3d-brep-offset` | 3/181084/3224 | 5/205353/3674 | 0.918 (lines) | OPEN | 0.918 (lib 3/181084/3224; test 5/205353/3674) |  |
| `F:os-kernel-db-io-core` | 1/145294/3316 | 4/161560/3671 | 0.918 (lines) | PARTIAL | 0.918 (lib 1/145294/3316; test 4/161560/3671) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:norm-en1993-checks` | 5/147116/2969 | 6/178321/3661 | 0.915 (lines) | OPEN | 0.915 (lib 5/147116/2969; test 6/178363/3661) |  |
| `A:draw-drawing-editor` | 13/239195/2665 | 14/239843/2670 | 0.915 (bytes) | TODO |  |  |
| `F:os-services` | 15/200954/3662 | 15/200954/3662 | 0.915 (lines) | OPEN | 0.915 (lib 15/200954/3662; test 15/200954/3662) |  |
| `A:puzzle-2d-mutations` | 26/181027/2312 | 29/239822/3062 | 0.915 (bytes) | TODO |  |  |
| `F:os-infinite-world-model` | 7/151082/3299 | 8/167655/3647 | 0.912 (lines) | PARTIAL | 0.912 (lib 7/151082/3299; test 8/167655/3647) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:procedural-generation3d` | 1/59416/582 | 22/238781/1693 | 0.911 (bytes) | OPEN | 0.911 (lib 1/59416/582; test 22/238781/1693) |  |
| `F:os-renderer-wgpu-shell-tests-input-1` | 1/4540/0 | 23/238816/2593 | 0.911 (bytes) | PARTIAL | 0.911 (lib 1/4540/0; test 23/238816/2593) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:wfc-3d-wfc3d` | 14/152345/2776 | 19/204981/3637 | 0.909 (lines) | OPEN | 0.909 (lib 14/152345/2776; test 19/204981/3637) |  |
| `F:job-worker` | 8/123533/1617 | 16/238318/2605 | 0.909 (bytes) | PARTIAL | 0.909 (lib 8/123533/1617; test 16/238318/2605) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) | fixtures read at run time through the shared test-fixtures reader macros (fixt.ts, dev-dependency added) |
| `F:plugin-decl-views` | 13/215589/3445 | 18/238197/3598 | 0.909 (bytes) | PARTIAL | 0.909 (lib 13/215589/3445; test 18/238197/3598) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:wfc-grid3d-editor` | 11/166963/2985 | 17/205397/3633 | 0.908 (lines) | OPEN | 0.908 (lib 11/166963/2985; test 17/205397/3633) |  |
| `A:gis-gismap-tests` | 16/108948/1899 | 29/231385/3634 | 0.908 (lines) | OPEN | 0.908 (lib 16/108948/1899; test 29/231385/3634) |  |
| `A:stdio-xml-snapshot` | 22/215096/3587 | 23/217686/3630 | 0.907 (lines) | OPEN | 0.907 (lib 22/215306/3587; test 23/217896/3630) |  |
| `F:pets-contract` | 8/114570/2743 | 11/172381/3629 | 0.907 (lines) | PARTIAL | 0.907 (lib 8/114570/2743; test 11/172381/3629) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:writer-writer-editor` | 28/64557/1223 | 29/67766/1340 | 0.906 (files) | TODO |  |  |
| `A:writer-writer-operations` | 16/40697/840 | 29/110087/1805 | 0.906 (files) | TODO |  |  |
| `A:writer-writer-tests-2` | 22/152518/2518 | 29/202612/3270 | 0.906 (files) | TODO |  |  |
| `A:writer-writer-inferences` | 25/61045/1021 | 29/64934/1101 | 0.906 (files) | TODO |  |  |
| `A:writer-writer-rename-writer` | 19/41050/780 | 29/52768/1013 | 0.906 (files) | TODO |  |  |
| `A:mathematical-equation-update-graph-algorithm` | 25/59951/1323 | 29/69728/1521 | 0.906 (files) | OPEN | 0.906 (lib 25/59951/1323; test 29/69728/1521) |  |
| `A:wfc-2d-diff` | 28/48659/1065 | 29/50414/1098 | 0.906 (files) | OPEN | 0.906 (lib 28/48659/1065; test 29/50414/1098) |  |
| `A:wfc-2d-mutations` | 13/54888/933 | 29/191693/2962 | 0.906 (files) | OPEN | 0.906 (lib 13/54888/933; test 29/191693/2962) |  |
| `A:wfc-2d-pin-slot` | 21/49193/1029 | 29/66124/1387 | 0.906 (files) | OPEN | 0.906 (lib 21/49193/1029; test 29/66124/1387) |  |
| `A:wfc-grid2d-diff` | 26/58389/1260 | 29/63136/1379 | 0.906 (files) | OPEN | 0.906 (lib 26/58389/1260; test 29/63136/1379) |  |
| `A:wfc-grid2d-create-tile` | 27/43352/895 | 29/51642/1058 | 0.906 (files) | OPEN | 0.906 (lib 27/43352/895; test 29/51642/1058) |  |
| `A:wfc-3d-mutations` | 13/52029/854 | 29/187840/3025 | 0.906 (files) | OPEN | 0.906 (lib 13/52029/854; test 29/187827/3025) |  |
| `A:wfc-grid3d-mutations` | 16/49898/858 | 29/163711/2563 | 0.906 (files) | OPEN | 0.906 (lib 16/49898/858; test 29/163711/2563) |  |
| `A:flow-flow-editor-leaves` | 25/84947/1526 | 29/95341/1704 | 0.906 (files) | OPEN | 0.906 (lib 25/84947/1526; test 29/95341/1704) |  |
| `A:gis-gismap-mutations` | 27/70676/1093 | 29/87116/1331 | 0.906 (files) | OPEN | 0.906 (lib 27/70676/1093; test 29/87116/1331) |  |
| `A:animate-presentation-mutations` | 14/80687/1376 | 29/198749/3038 | 0.906 (files) | OPEN | 0.906 (lib 14/80660/1376; test 29/198722/3038) |  |
| `A:shooting-shooting-tests` | 22/58170/1174 | 29/79220/1580 | 0.906 (files) | OPEN | 0.906 (lib 22/58170/1174; test 29/79220/1580) |  |
| `A:shooting-shooting-mutations` | 10/53288/730 | 29/175941/2387 | 0.906 (files) | OPEN | 0.906 (lib 10/53288/730; test 29/175941/2387) |  |
| `A:shooting-shooting-set-active-asset` | 27/46031/909 | 29/47631/944 | 0.906 (files) | OPEN | 0.906 (lib 27/46031/909; test 29/47631/944) |  |
| `A:sequence-sequence-inferences` | 25/48321/778 | 29/51925/855 | 0.906 (files) | TODO |  |  |
| `A:sequence-sequence-tests` | 21/105489/1959 | 29/135588/2460 | 0.906 (files) | TODO |  |  |
| `A:forms-forms-forms` | 10/107281/1669 | 29/232298/3521 | 0.906 (files) | OPEN | 0.906 (lib 10/107281/1669; test 29/232298/3521) |  |
| `A:forms-forms-tests-2` | 19/88417/1575 | 29/111683/2006 | 0.906 (files) | OPEN | 0.906 (lib 19/88417/1575; test 29/111683/2006) |  |
| `A:forms-forms-diff` | 29/80237/1491 | 29/80237/1491 | 0.906 (files) | OPEN | 0.906 (lib 29/80237/1491; test 29/80237/1491) |  |
| `A:norm-din18599-editor` | 18/61640/1083 | 29/88171/1576 | 0.906 (files) | PARTIAL | 0.906 (lib 18/61640/1083; test 29/88171/1576) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:norm-din4108-io` | 25/83490/961 | 29/87787/1067 | 0.906 (files) | OPEN | 0.906 (lib 25/83490/961; test 29/87835/1067) |  |
| `A:norm-en1996-editor` | 18/56743/1029 | 29/98688/1717 | 0.906 (files) | PARTIAL | 0.906 (lib 18/56743/1029; test 29/98688/1717) | embedded test fixtures read at run time (fixt.ts / fixturesToRuntime) |
| `A:trinity-rewriting-operations` | 22/78977/1315 | 29/151399/2278 | 0.906 (files) | OPEN | 0.906 (lib 22/78977/1315; test 29/151399/2278) |  |
| `A:trinity-rewriting-mut-any` | 27/44151/887 | 29/47158/951 | 0.906 (files) | OPEN | 0.906 (lib 27/44151/887; test 29/47158/951) |  |
| `A:stdio-html-mutations` | 21/91890/1564 | 29/137066/2293 | 0.906 (files) | OPEN | 0.906 (lib 21/90365/1567; test 29/134585/2293) |  |
| `A:stdio-zip-schema` | 25/94625/1988 | 29/101537/2144 | 0.906 (files) | OPEN | 0.906 (lib 25/94625/1988; test 29/101537/2144) |  |
| `A:stdio-svg-handcrafted-diff-codec-tests` | 27/29716/643 | 29/35209/723 | 0.906 (files) | OPEN | 0.906 (lib 27/29716/643; test 29/35209/723) |  |
| `A:stdio-svg-mutations` | 26/62002/1255 | 29/70960/1426 | 0.906 (files) | OPEN | 0.906 (lib 26/62002/1255; test 29/70960/1426) |  |
| `A:stdio-svg-tests` | 16/33775/647 | 29/44665/878 | 0.906 (files) | OPEN | 0.906 (lib 16/33775/647; test 29/44665/878) |  |
| `A:stdio-pdf-tests` | 23/92515/1262 | 29/104046/1483 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-snapshot` | 29/35850/666 | 29/35850/666 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-diff` | 29/75958/1455 | 29/75958/1455 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-14` | 3/39100/612 | 29/100120/1619 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-x-2` | 2/17382/201 | 29/28074/473 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-10` | 5/36238/559 | 29/46142/819 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-9` | 8/47889/791 | 29/56079/1002 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-13` | 3/34920/569 | 29/94427/1560 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-15` | 8/46389/802 | 29/92217/1556 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-base-4` | 28/50042/775 | 29/50798/790 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-8` | 27/105620/1787 | 29/107700/1825 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-conformance-support` | 17/142566/2438 | 29/185338/3076 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-inferences` | 24/152574/2958 | 29/169253/3227 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-schema-4` | 9/48450/818 | 29/90851/1668 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-model` | 28/167377/3226 | 29/169121/3242 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-pdf-snapshot-value-tests` | 10/76299/1329 | 29/214735/3084 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-testkit` | 20/105229/1929 | 29/206517/3272 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-base-8` | 2/14058/179 | 29/63264/1027 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-base-9` | 2/14076/176 | 29/62685/1048 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-12` | 5/73233/1155 | 29/155760/2449 | 0.906 (files) | TODO |  |  |
| `A:stdio-pdf-mut-family-11` | 10/38154/605 | 29/49210/854 | 0.906 (files) | TODO |  |  |
| `A:stdio-docx-base-model` | 29/166402/2716 | 29/166402/2809 | 0.906 (files) | OPEN | 0.906 (lib 29/166675/2716; test 29/166675/2809) |  |
| `A:stdio-md-diff` | 24/120084/2159 | 29/124907/2353 | 0.906 (files) | OPEN | 0.906 (lib 24/120198/2159; test 29/125021/2353) |  |
| `A:stdio-xml-schema-2` | 23/90224/1583 | 29/104769/1850 | 0.906 (files) | OPEN | 0.906 (lib 23/90224/1583; test 29/104769/1850) |  |
| `A:stdio-jpg-schema-2` | 17/66232/1292 | 29/122169/2155 | 0.906 (files) | OPEN | 0.906 (lib 17/66232/1292; test 29/122169/2155) |  |
| `A:stdio-tiff-snapshot` | 29/83463/734 | 29/83463/734 | 0.906 (files) | OPEN | 0.906 (lib 29/83463/734; test 29/83463/734) |  |
| `A:stdio-gltf-inferences-clearance` | 10/24920/362 | 29/31054/533 | 0.906 (files) | OPEN | 0.906 (lib 10/24920/362; test 29/31054/533) |  |
| `A:stdio-gltf-mutations-mut-any-5` | 2/14180/157 | 29/49065/709 | 0.906 (files) | OPEN | 0.906 (lib 2/14180/157; test 29/49065/709) |  |
| `A:stdio-gltf-mutations-mut-any-9` | 2/14360/161 | 29/37522/487 | 0.906 (files) | OPEN | 0.906 (lib 2/14360/161; test 29/37522/487) |  |
| `A:stdio-gltf-mutations-mut-any-6` | 2/15327/174 | 29/49632/625 | 0.906 (files) | OPEN | 0.906 (lib 2/15327/174; test 29/49632/625) |  |
| `A:stdio-gltf-mutations-mut-any-8` | 2/13183/149 | 29/36165/478 | 0.906 (files) | OPEN | 0.906 (lib 2/13183/149; test 29/36165/478) |  |
| `A:stdio-gltf-mutations-mut-any-7` | 2/11830/129 | 29/35424/463 | 0.906 (files) | OPEN | 0.906 (lib 2/11830/129; test 29/35424/463) |  |
| `A:stdio-semio-value-mutations` | 29/65041/1085 | 29/65041/1129 | 0.906 (files) | PARTIAL | 0.906 (lib 29/65022/1085; test 29/65022/1129) | fixtures read at run time through the shared test-fixtures reader macros (fixt.ts, dev-dependency added) |
| `A:stdio-semio-kit-mutations` | 27/61871/1102 | 29/63303/1162 | 0.906 (files) | OPEN | 0.906 (lib 27/61878/1102; test 29/63310/1162) |  |
| `A:note-note-diff` | 24/82966/1634 | 29/90199/1785 | 0.906 (files) | OPEN | 0.906 (lib 24/82966/1634; test 29/90199/1785) |  |
| `A:note-note-mutations` | 20/144571/1716 | 29/182387/2413 | 0.906 (files) | OPEN | 0.906 (lib 20/144571/1716; test 29/182387/2413) |  |
| `A:note-note-tests-hides-the-grid` | 22/63771/1140 | 29/129217/2016 | 0.906 (files) | OPEN | 0.906 (lib 22/63771/1140; test 29/129217/2016) |  |
| `A:note-note-delete-blocks` | 28/62200/1334 | 29/62944/1352 | 0.906 (files) | OPEN | 0.906 (lib 28/62200/1334; test 29/62944/1352) |  |
| `A:puzzle-5d-store-preparation` | 29/57625/1016 | 29/57625/1016 | 0.906 (files) | TODO |  |  |
| `A:puzzle-5d-diff` | 23/136800/2505 | 29/158759/2951 | 0.906 (files) | TODO |  |  |
| `A:puzzle-5d-mut-any-5` | 24/114750/1701 | 29/146636/2188 | 0.906 (files) | TODO |  |  |
| `A:puzzle-5d-mut-any-7` | 4/10394/138 | 29/215509/2934 | 0.906 (files) | TODO |  |  |
| `A:sourcing-curation-operations` | 23/59254/1115 | 29/80638/1539 | 0.906 (files) | TODO |  |  |
| `A:sourcing-curation-inferences` | 27/87532/1498 | 29/102641/1752 | 0.906 (files) | TODO |  |  |
| `A:sourcing-curation-domain` | 28/144748/2293 | 29/148492/2364 | 0.906 (files) | TODO |  |  |
| `F:os-config` | 29/103232/2166 | 29/103232/2166 | 0.906 (files) | OPEN | 0.906 (lib 29/103232/2166; test 29/103232/2166) |  |
| `F:artifact-workflow-run` | 29/96203/1417 | 29/96203/1417 | 0.906 (files) | OPEN | 0.906 (lib 29/96203/1417; test 29/96203/1417) |  |
| `A:puzzle-2d-interaction` | 28/68863/1211 | 29/91621/1489 | 0.906 (files) | TODO |  |  |
| `A:puzzle-2d-mut-any-7` | 5/14101/169 | 29/215458/2812 | 0.906 (files) | TODO |  |  |
| `F:os-run` | 10/175089/2551 | 11/236919/3501 | 0.904 (bytes) | OPEN | 0.904 (lib 10/175089/2551; test 11/236919/3501) |  |
| `A:puzzle-5d-dispatch` | 26/225021/3607 | 26/225021/3607 | 0.902 (lines) | TODO |  |  |
| `F:plugin-tests-15` | 1/771/11 | 26/236547/2201 | 0.902 (bytes) | TODO |  |  |
| `A:stdio-pdf-tests-2` | 22/205068/3510 | 25/208245/3603 | 0.901 (lines) | TODO |  |  |
| `A:stdio-pdf-schema-5` | 17/226676/3367 | 20/235994/3519 | 0.900 (bytes) | TODO |  |  |

## 2. Excluded (owned by a running slice) (1)

| crate | lib | test | ratio | owner |
| --- | --- | --- | --- | --- |
| `A:bim-model` | 1303/5104913/87522 | 1705/9072869/124243 | 53.281 (files) | r9a |

## 1b. Crates brought under 0.90 (before -> after, final census; includes crates that were owned by a slice at the start and were freed or fixed later) (127)

| crate | before (worst) | after lib f/b/l | after test f/b/l | after (worst) |
| --- | --- | --- | --- | --- |
| `A:stdio-pdf` | 40.219 | 3/62519/853 | 8/174444/1543 | 0.665 |
| `A:puzzle-2d` | 36.688 | 1/38958/578 | 3/68965/808 | 0.263 |
| `A:puzzle-5d` | 26.281 | 1/46340/649 | 5/84468/1077 | 0.322 |
| `F:plugin` | 16.140 | 14/155806/2472 | 21/181166/2810 | 0.703 |
| `A:flow-flow` | 6.813 | 1/17974/309 | 4/47850/593 | 0.183 |
| `A:sourcing-curation` | 5.844 | 2/16975/278 | 16/168898/2034 | 0.644 |
| `A:raster-raster-editor` | 3.656 | 11/140463/2019 | 11/140463/2019 | 0.536 |
| `A:raster-raster-mutations-layers` | 3.094 | 19/29265/590 | 19/29265/590 | 0.594 |
| `A:raster-raster-model` | 1.844 | 20/140780/1978 | 22/143587/2039 | 0.688 |
| `A:raster-raster-mutations-pixels` | 1.063 | 15/85673/1424 | 15/85673/1424 | 0.469 |
| `A:raster-raster-io` | 1.031 | 26/55539/923 | 28/78294/1275 | 0.875 |
| `A:procedural-3d-editor-commands` | 1.000 | 16/25517/436 | 16/25517/436 | 0.500 |
| `A:sequence-sequence-schema` | 1.000 | 19/76094/1116 | 20/77926/1156 | 0.625 |
| `A:lowpoly-lowpoly-model` | 1.000 | 24/196308/1439 | 27/202613/1571 | 0.844 |
| `A:layout-layout-editor-commands` | 1.000 | 20/134506/2447 | 20/134506/2545 | 0.636 |
| `A:stdio-las-schema` | 1.000 | 10/42026/477 | 10/42026/477 | 0.313 |
| `A:stdio-ply-schema` | 1.000 | 10/38545/445 | 13/43148/535 | 0.406 |
| `A:stdio-semio` | 1.000 | 5/86080/634 | 14/189213/1341 | 0.722 |
| `F:dsl` | 0.999 | 7/81852/1735 | 10/125897/2555 | 0.639 |
| `F:os-directory` | 0.996 | 1/14692/215 | 1/14692/215 | 0.056 |
| `F:3d-brep-boolean` | 0.987 | 3/177257/3334 | 4/186819/3511 | 0.878 |
| `F:dsl-record` | 0.986 | 8/113307/1477 | 14/226141/2799 | 0.863 |
| `A:stdio-mp3-test-oracle` | 0.982 | 2/29041/503 | 3/36977/633 | 0.158 |
| `A:procedural-2d-mutations` | 0.969 | 2/44949/681 | 6/50839/735 | 0.194 |
| `A:procedural-3d-io-text` | 0.969 | 23/106698/2031 | 27/130297/2368 | 0.844 |
| `A:procedural-3d-mutations` | 0.969 | 2/25417/436 | 6/51443/730 | 0.196 |
| `A:fem-2d-load` | 0.969 | 16/26939/500 | 16/26939/500 | 0.500 |
| `A:fem-3d-load` | 0.969 | 16/26366/492 | 16/26366/492 | 0.500 |
| `A:norm-en1991-mut-traffic` | 0.969 | 16/16298/330 | 16/16298/330 | 0.500 |
| `A:norm-en1991-mut-silos` | 0.969 | 16/16102/330 | 16/16102/330 | 0.500 |
| `A:norm-vdi3805-examples-failing` | 0.969 | 11/12440/110 | 16/15739/185 | 0.500 |
| `A:norm-iso16757-mut-products` | 0.969 | 16/21297/419 | 16/21297/419 | 0.500 |
| `A:norm-en1993-mut-general` | 0.969 | 13/14000/256 | 13/14000/256 | 0.406 |
| `A:norm-din4108-mut-elements` | 0.969 | 16/18773/345 | 16/18773/345 | 0.500 |
| `A:norm-en1996-mut-loads` | 0.969 | 16/17917/348 | 16/17917/348 | 0.500 |
| `A:norm-en1996-mut-walls` | 0.969 | 16/16321/318 | 16/16321/318 | 0.500 |
| `A:norm-en1995-mut-members` | 0.969 | 16/17761/276 | 16/17761/276 | 0.500 |
| `A:norm-en1998-mut-buildings` | 0.969 | 16/18096/373 | 16/18096/373 | 0.500 |
| `A:remodel-remodeling-mut-media` | 0.969 | 10/18079/332 | 14/13387/247 | 0.438 |
| `A:remodel-remodeling-mut-calib` | 0.969 | 10/19355/338 | 12/11465/212 | 0.375 |
| `A:stdio-mp4-tests` | 0.969 | 7/141464/2582 | 10/165852/2984 | 0.746 |
| `A:stdio-step-base` | 0.969 | 13/42638/526 | 16/54562/662 | 0.500 |
| `A:stdio-docx-base` | 0.969 | 20/110966/1872 | 25/162775/2577 | 0.781 |
| `A:stdio-jpg-mutations` | 0.969 | 26/101572/1909 | 28/124652/2105 | 0.875 |
| `A:stdio-ply-inferences` | 0.969 | 27/199976/2919 | 28/201316/2998 | 0.875 |
| `A:stdio-semio-base-examples` | 0.969 | 17/14023/176 | 17/14023/176 | 0.531 |
| `A:block-2d-mut-handles` | 0.969 | 16/21961/423 | 16/21961/423 | 0.500 |
| `A:block-5d-mut-representations` | 0.969 | 16/24398/442 | 16/24398/442 | 0.500 |
| `A:block-3d-mut-representations` | 0.969 | 16/24398/442 | 16/24398/442 | 0.500 |
| `F:value-retirement` | 0.969 | 12/93885/1300 | 20/136944/1552 | 0.625 |
| `F:io-sqlite-snapshot-tests` | 0.969 | 1/2813/0 | 12/137334/799 | 0.524 |
| `F:os-vcs` | 0.969 | 21/153633/2568 | 26/196245/3106 | 0.813 |
| `semio-hub-directory-laws` | 0.954 | 1/719/12 | 14/205301/2185 | 0.783 |
| `semio-framework` | 0.950 | 1/24110/147 | 7/151513/2485 | 0.621 |
| `F:ui-retained-tree` | 0.948 | 2/108503/1695 | 3/136759/2489 | 0.622 |
| `A:stdio-dwg-snapshot` | 0.947 | 15/131330/1613 | 19/229859/2368 | 0.877 |
| `F:os-directory-client-tests` | 0.946 | 1/3648/0 | 7/106744/1727 | 0.432 |
| `A:fem-3d-mutations` | 0.942 | 8/24759/410 | 16/150979/1921 | 0.576 |
| `A:writer-writer-schema` | 0.938 | 16/75316/1304 | 16/75316/1304 | 0.500 |
| `A:procedural-2d-editor` | 0.938 | 11/162456/2545 | 14/192706/3070 | 0.767 |
| `A:procedural-2d-model` | 0.938 | 11/118910/1263 | 15/131203/1519 | 0.500 |
| `A:fem-2d` | 0.938 | 8/68613/257 | 10/93361/356 | 0.356 |
| `A:fem-3d` | 0.938 | 8/62915/255 | 10/88554/353 | 0.338 |
| `A:norm-iso16757-io` | 0.938 | 14/140692/1010 | 16/142368/1050 | 0.543 |
| `A:norm-en1995-io` | 0.938 | 14/44530/427 | 16/48011/497 | 0.500 |
| `A:remodel-remodeling-mut-results` | 0.938 | 7/18339/310 | 8/8049/139 | 0.250 |
| `A:stdio-step-snapshot` | 0.938 | 22/198770/2759 | 28/221646/3129 | 0.875 |
| `A:stdio-txt-schema` | 0.938 | 15/67759/731 | 16/83307/811 | 0.500 |
| `A:stdio-dwg-sqlite-drawing` | 0.938 | 11/149870/388 | 11/149870/388 | 0.572 |
| `A:puzzle-3d-commands-runtime` | 0.938 | 15/19441/347 | 15/19441/347 | 0.469 |
| `A:block-2d-editor` | 0.938 | 21/97078/1563 | 24/137952/2205 | 0.750 |
| `F:value-native-decoding` | 0.938 | 9/44279/382 | 16/118334/850 | 0.500 |
| `F:plugin-testing-fixtures` | 0.938 | 19/64243/955 | 26/83194/1259 | 0.813 |
| `F:os-directory-tests-1` | 0.934 | 1/482/0 | 2/28142/448 | 0.112 |
| `A:fem-2d-mutations` | 0.933 | 8/23303/412 | 16/161615/1975 | 0.617 |
| `A:raster-raster-host` | 0.933 | 5/144463/2610 | 6/165550/2990 | 0.748 |
| `F:replication-error` | 0.930 | 2/8472/133 | 4/21685/264 | 0.333 |
| `A:procedural-3d-preview-eval` | 0.926 | 8/163349/2425 | 9/189732/2774 | 0.724 |
| `A:stdio-dxf-schema` | 0.924 | 6/46004/222 | 6/46004/222 | 0.188 |
| `F:os-renderer-wgpu-engine-attach-tests` | 0.923 | 1/2441/0 | 3/57234/1097 | 0.274 |
| `A:stdio-gltf-mutations-mut-any-4` | 0.920 | 2/17370/284 | 2/17370/285 | 0.071 |
| `F:value-tests` | 0.915 | 1/2072/0 | 19/221100/2336 | 0.843 |
| `F:os-kernel` | 0.914 | 5/111459/1394 | 13/235360/2226 | 0.898 |
| `A:process-process3d-host` | 0.912 | 6/153415/2640 | 7/197781/3328 | 0.832 |
| `A:fem-3d-editor` | 0.910 | 19/151378/2202 | 16/180928/2163 | 0.690 |
| `A:mathematical-equation-inferences` | 0.906 | 21/70780/1361 | 26/87317/1663 | 0.813 |
| `A:wfc-grid2d-inferences` | 0.906 | 20/107708/2029 | 26/159132/2841 | 0.813 |
| `A:gis-gisterrain-inferences` | 0.906 | 23/92714/1163 | 26/98489/1273 | 0.813 |
| `A:vcs-vcs-inferences` | 0.906 | 25/56627/948 | 26/57256/957 | 0.813 |
| `A:vcs-vcs-tests` | 0.906 | 15/43676/728 | 27/65744/1133 | 0.844 |
| `A:animate-presentation-viewer` | 0.906 | 22/34941/610 | 23/37232/649 | 0.719 |
| `A:shooting-shooting-inferences` | 0.906 | 25/94570/1357 | 26/98139/1416 | 0.813 |
| `A:layout-layout-mutations-content` | 0.906 | 15/53478/925 | 15/53478/927 | 0.469 |
| `A:layout-layout-mutations-frames` | 0.906 | 10/42008/697 | 15/51849/863 | 0.469 |
| `A:norm-en1990-io` | 0.906 | 14/48732/563 | 16/53951/691 | 0.500 |
| `A:norm-din18599-io` | 0.906 | 14/47548/478 | 16/49801/533 | 0.500 |
| `A:norm-en1997-io` | 0.906 | 14/45510/437 | 16/47400/476 | 0.500 |
| `A:norm-din16798-io` | 0.906 | 14/73122/952 | 16/73994/979 | 0.500 |
| `A:norm-vdi3805-io` | 0.906 | 14/93819/778 | 16/95392/814 | 0.500 |
| `A:norm-en1996-io` | 0.906 | 14/43904/445 | 16/45807/484 | 0.500 |
| `A:norm-en1999-io` | 0.906 | 14/52851/465 | 16/53756/489 | 0.500 |
| `A:norm-en1998-io` | 0.906 | 13/76335/792 | 15/78088/829 | 0.469 |
| `A:playbook-playbook-inferences` | 0.906 | 19/132463/2169 | 26/147675/2485 | 0.813 |
| `A:imperative-procedure-inferences` | 0.906 | 19/75428/1427 | 26/87379/1691 | 0.813 |
| `A:remodel-remodeling-mut-params` | 0.906 | 7/11855/218 | 8/7578/138 | 0.250 |
| `A:dag-dag-inferences` | 0.906 | 24/61993/1033 | 26/65696/1098 | 0.813 |
| `A:draw-drawing-editor-commands` | 0.906 | 15/49043/558 | 15/49043/564 | 0.469 |
| `A:stdio-zip-inferences` | 0.906 | 22/80420/990 | 26/86719/1120 | 0.813 |
| `A:stdio-gif-schema-2` | 0.906 | 11/132691/2034 | 14/153291/2371 | 0.593 |
| `A:stdio-svg-mutations-2` | 0.906 | 20/99279/1671 | 28/145331/2410 | 0.875 |
| `A:stdio-bcf-inferences` | 0.906 | 24/180829/2847 | 26/184448/2931 | 0.813 |
| `A:stdio-csv-mutations` | 0.906 | 22/97761/1829 | 28/135245/2504 | 0.875 |
| `A:stdio-docx-transitional` | 0.906 | 19/82952/1457 | 27/111557/1921 | 0.844 |
| `A:stdio-xml-inferences` | 0.906 | 25/31835/495 | 26/32038/503 | 0.813 |
| `A:stdio-pptx-transitional` | 0.906 | 20/78882/1405 | 28/105063/1825 | 0.875 |
| `A:stdio-txt-tests` | 0.906 | 11/79983/1337 | 23/147901/2317 | 0.719 |
| `A:stdio-dxf-inferences` | 0.906 | 21/37608/704 | 26/51738/928 | 0.813 |
| `A:stdio-tiff-inferences` | 0.906 | 19/130290/1608 | 26/159405/1897 | 0.813 |
| `A:stdio-deflate-schema` | 0.906 | 9/27749/309 | 14/60547/722 | 0.438 |
| `A:stdio-json-schema` | 0.906 | 23/218027/2847 | 26/225346/3018 | 0.860 |
| `A:stdio-json-mutations` | 0.906 | 22/100898/1959 | 27/143431/2709 | 0.844 |
| `A:stdio-semio-video-mutations` | 0.906 | 27/51270/962 | 28/69610/1268 | 0.875 |
| `A:stdio-bmp-mutation-regressions` | 0.906 | 8/13679/301 | 14/34650/638 | 0.438 |
| `A:block-5d-editor` | 0.906 | 20/93828/1557 | 23/131946/2170 | 0.719 |
| `A:space-home-inferences` | 0.906 | 25/39337/658 | 26/40020/674 | 0.813 |
| `F:trace` | 0.906 | 13/94361/2079 | 20/140770/3035 | 0.759 |
| `A:gis-gismap` | 0.900 | 16/112155/1467 | 28/233615/3178 | 0.891 |

## 3. Method, tools and verification

All tools are in `F/hdr-tools/`; logs in `F/🗑️generated/hdr-*`.

| technique | tool | what it does | verification |
| --- | --- | --- | --- |
| descriptor facets served at run time | `bun ./📜️script.ts new leaves <descriptor.rs> --rust` (generator extended to the `inference` export, test `serves the facet companions of an inference descriptor...`, crate-tree suite 87 pass) + `install-reader.py` | typescript/graphql/proto/rust companions of 27 schema descriptors and 16 inference descriptors are registered documents instead of `include_str!`; json_schema stays embedded; the default `SchemaLeafReader` is installed in the descriptor fn. Registry got `resolved_leaf_body` (+ unit test) and the component's GraphQL SDL readers use it, so externalised leaves stay served (found: they read the empty inline field). lowpoly-model had `const` leaves and was converted by hand to register-and-serve functions | rustfmt parse on every touched file; census; `fixcheck` |
| leaf-family split | `famsplit.py`, `fixtests.py` | a crate whose lib root is a list of leaf mounts (flat, nested in a wrapper chain, or one inline module per leaf) becomes the crate plus one or two sibling part crates; leaves that name each other stay together; the old crate re-exports the moved leaf modules with an explicit `pub use` (no consumer changes); `[[test]]` case targets are redistributed; parts registered in the artifact workspace (member + dependency table) and the owner rows | `new crates <pkg> --check-applied`: onlyCombined 0 for lib and test on the artifacts listed in section 4; taxonomy resolves every new directory (165 paths, 0 unresolved); longest path 236 bytes (two dirs renamed `calib-N` for that); owners scan has no row of mine missing |
| companion test bins | `famsplit.py --sibling --no-reexport` | the plugin companion bins `tests-NN` hold test roots; they are distributed over `tests-NN-2`/`-3`; each part is a root-workspace member with project.json, owner row and six launch routes in `.vscode/launch.json` and the seed | census; routes parse |
| public-API tests detached | `mkcomp2.py`, `rebasefix.py` | 18 hoisted inverse-law tests of the stdio semio facade moved to the new companion crate `semio-s-artifact-stdio-semio-inverts-tests`; `crate::` became the owner path; run-time fixture literals were re-based to the new manifest directory (`fixcheck.ts` found 5 more in `procedural-3d-editor-commands-2`, fixed) | the oracle was the guard for a white-box attempt: moving the `os-mcp-hub-remote` unit tests produced 45 new E0603/E0616/E0624 rows in `pos` (`hdr-hubremote`), so that move was reverted completely (files, member, owner row, six routes) |
| module peel | `peel2.py` | one directly mounted module tree becomes a producer crate rooted at its own file; parent keeps `pub use <crate> as <module>`; restricted items of the module root widened to pub. Applied to `ui-retained-tree::tree` and to the `text` codec subtree of 10 `norm-*-io` crates | `pos` family `hdr-retained`: lib and test NEW 0 / GONE 0; `--check-applied` for the norm artifacts |
| framework carve | `newfw.py` | `semio-framework-dsl-syntax` (grammar model, lexer, parsers) out of `semio-framework-dsl`: member, owner row, six routes; `prim_fixed_width` moved to the model; `GKind`, `GToken`, `lex`, `is_protocol_source` widened to pub | `pos` family `hdr-dsl`: NEW 0 / GONE 0 for lib and test, negative control (private `is_protocol_source`, `lex`) caught as E0603/E0364 |
| test targets split | `splittarget.py` | pure mount-list `[[test]]` targets cut into sibling targets: fem-2d/3d-mutations (`mesh_topology_kinds`), procedural-2d-editor (`editor-routes`), fem-3d-editor (`editor_app`) | census |
| fixtures at run time | `fixt.ts` over the generator's `fixturesToRuntime` | embedded test fixtures of 150+ test files read at run time (sha256 and existence logged in `hdr-fixt-*.log`); `include!`d fragments and one file shared with another crate's closure were skipped or reverted. At 17:50 the FIXTURE-READER-DEDUPE peer introduced the shared crate `semio-framework-test-fixtures` and the macro form; the helper copies written here before that are for the dedupe sweep to convert | census |
| public-API tests to `[[test]]` | by hand (dsl-record) | refusal/number-refusal/optional-field/required-inline became integration targets; `pos` `hdr-dslrecord` lib NEW 0, test NEW 0 | |

Other repository gates: `ctc-scan` (3,181 crates; the one group of mine, `norm-en1998-io-text` `demo_mutation_cases`, was gated `testkit` already, so the new crate got the `testkit` feature and the parent forwards it and dev-depends with the feature); `owners-tools/scan.ts` (only the 14 foreign rows are missing); both launch JSON files parse.

## 4. `--check-applied` results (layout differential vs `HEAD`, lib and test)

Exit 0 and onlyCombined 0/0 (logs `F/🗑️generated/hdr-chk-<artifact>.log`, drivers `hdr-chk-batch2..6.out`, `hdr-chk-pdf.out`): norm en1990, en1991, en1993, en1995, en1996, en1997, en1998, en1999, din4108, din16798, din18599, vdi3805, iso16757, remodel remodeling, block 2d, 3d, 5d, procedural generation3d, procedural generation2d, fem 2d/3d, layout, draw drawing (after fixing a peer-migration gap in the geometry translation test: `selection_tests` imported `fixture_path` instead of `fixture_text`), flow (after the harness extraction and again after the descriptor conversion), stdio pdf (after the glyph-list split), animate presentation, writer. Puzzle 3d passed on the re-run (the first run died on a transient loader error, `The argument 'file' cannot be empty`; block 3d and puzzle 3d share one log tag). Sequence (batch6) and stdio semio, stdio dwg and the stdio semio presentation/document schema edits: exit 0, onlyCombined 0/0 (`hdr-chk-semio-s60.log`, sandbox `chk-semio-s60`). Two earlier stdio semio runs (`hdr-chk-batch3.out`, `hdr-chk-batch7.out`: 50 lib extras, all `crate::io::Semio*Analyzer` imports of the subset `io` roots) were invalid: batch3 and batch6 ran the same artifact at the same time into the same sandbox directory (the sandbox is named after the last path segment), and batch7 reused the corrupted sandbox; a run into a fresh sandbox gives 0/0. Rule for the compile queue: never run two `--check-applied` of one artifact at once and use a fresh `--sandbox`.

The typeck oracle (`pos`) has no baseline for the flow unit crates (they are not in `HEAD`), so family `hdr-flowunit` ran absolute: privacy and resolve rows that name the moved paths (`unit_tests::context`, `crate::FlowSnapshot`) are 0; the remaining rows are stub-layer noise of the 10-crate family.

## 5. Not done and why (free crates still at >= 0.90, grouped by cause)

* Hub roots with `use super::*` and white-box tests (a module cannot leave without changing visibility, which the oracle rejects): `semio-framework-job` (0.991, bytes, test), `3d-brep-boolean` (0.987), `ui-prepared`, `3d-brep-construction` (euler's tests need the primitives that depend on euler), `plugin-instance-app`, `os-kernel-db-sync`, `os-kernel-db-io-*`, `os-kernel`, `os-services`, `ui-contract`, `ui-contract-values`, `os-renderer-wgpu-dock`, `repo-model`, `energy-engine-sim`, `quiz-tests`, `value-intrinsic`, `value-derive`, `pets-contract`, `plugin-interpreter`, `plugin-decl-views`, `3d-mesh-document`, `3d-brep-queries/-offset`, `os-mcp-hub-remote` (white-box, reverted), `plugin-child-group-wire-laws` (inline tests with `include!` modules, no lib).
* Path-preserving mirror crates (`-tests`, `-mutations`, `-snapshot`, `-inferences`, about 50 at 0.906 to 0.94): the mirror nest is a multi-branch inline module tree whose unit tests use `super::*`; a splitter needs a tree partition, not a leaf list (attempted, abandoned: the top level of the nest has four branches that all name `standards`).
* Codec crates whose embedded files are protocol/grammar/SQL resources compiled into the codec (not test fixtures): `stdio-dwg-sqlite-*`, `stdio-dwg-read-objects`, `stdio-svg-snapshot`, `mathematical-equation-snapshot`.
* `procedural-2d/3d-mutations` (0.969): 25 embedded fixture JSONs of the `widget-kinds` target; waits for the fixture-reader sweep (the generator now emits the shared macros, which also need the dev-dependency).
* Over the gate and not mine: `process-process3d-host` test 1.084 (a peer extended `retained-laws` at 17:34; bd10 O-03 area). `plugin-tests-07-2` (0.938) and `tests-18` (0.906): one 165 KB test file (`time-travel-part-1`) / one test root; need a test-file split.
* `stdio-semio-document-examples`, `stdio-semio-table/audio-mutations`, `pptx-base-model`, `forms-commands` (0.969): the mutation leaves name the aggregate enum of their crate (`crate::schema::mutations::SemioTableMutation`), so the aggregate has to move down first (generic-leaf conversion of the owner slices); `famsplit` aborts with 'non-module items at the leaf level' and pinning the aggregate cannot be sound.
* About 105 crates at exactly 29 of 32 files (0.906: `stdio-pdf-*`, `wfc-*`, `writer-*`, `forms-*`, `norm-*`, `note-*`, `trinity-rewriting-*`, `sourcing-curation-*`, `flow-editor-leaves`, `flow-schema` ...) are generator packs one file above the 0.90 line; the embedded files among them are protocol/grammar/SQL resources (codec sources), not fixtures.
* No real compile (no `COMPILE ADMITTED`). Cargo.lock files of every touched workspace need `update --workspace --offline` in the compile window (62 new packages; none was added to a lockfile by hand). Dependency lists of `peel2` crates are the parent's full lists (safe superset); `famsplit` parts copy the parent's lists.

## 6. Commands for the compile queue

```
cd /Users/ueli/Documents/semio
bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline   # per touched workspace (norm artifacts, fem, remodel, block, puzzle-3d, procedural, stdio semio/dwg, lowpoly, framework, root, hub)
bun repo cargo --store=shared --configuration=prototype -- check --workspace --tests --keep-going --message-format=short   # per workspace
```

## 7. HDR-2 (coordinator follow-up 22:10: bd10 and r8a done, their crates are mine)

Census 22:53 (3,069 crates, 70 created by this slice, none >= 0.90): **0 crates over the gate except `bim-model` (r9a, excluded); 152 crates >= 0.90 (151 free)**. Fixed in this round (ratio before -> after, worst of lib and test):

| crate | before | after | how |
| --- | --- | --- | --- |
| `process3d-host` | 1.084 (lines, over the gate) | 0.832 | `retained-cursors` (656 lines, no sibling dependency) and `publication-authority` (338 lines, only three constants of the root) peeled into the producer crates `process3d-host-retained-cursors` and `process3d-host-publication-authority` (`hostpeel.py`); the consts moved with the authority, `cfg(test)` items became `cfg(any(test, feature = "testkit"))`, the host forwards `testkit` and dev-depends with the feature. `pos` family `hdr-p3dhost`: privacy 0, resolve 0 |
| `raster-io` | 0.969 (31/32 files) | 0.875 | `semio_bridge`, `composite_and_media`, `testing` and the one fn they needed from the construction module (`mint_raster_asset_child`) peeled into `raster-io-media-bridge` (`clumppeel.py`); the old root re-exports it with a glob so every `crate::` path of the 20 leaves stays |
| `raster-editor-commands` | 0.969 (lib 31/32) | 0.500 and `-2` 0.500 | `famsplit.py` extended with an aggregate pin (`--pin-word RasterCommand`) and wrapper containers with sibling items: payloads that name the closed command enum (and everything that names them) stay, the other 15 move |
| `raster-host` | 0.933 (lines) | 0.748 | `retirement` and `bounds_census` plus the four nesting constants peeled into `raster-host-retirement-bounds` (`clumppeel.py --precise`) |
| `puzzle-5d-v1` | 0.955 (bytes) | 0.281 | the `unit_tests` mount (six files, 172 KB) moved to `puzzle-5d-v1-unit-tests` with its own copy of the mirror (`mirrorsplit.py`, paths rebased); that companion is at 0.815 |
| `puzzle-2d-v1` | 0.951 (lines) | 0.772 | the three `cfg(test)` mounts inside the editor file (no `super::` use) moved to the mirror, then to `puzzle-2d-v1-editor-tests` (0.86) |
| `3d-brep-boolean` | 0.988 (lines) | 0.878 | the 452-line white-box unit test moved to the companion `3d-brep-boolean-unit-tests` (`testcomp.py`) with three thin accessors for private fns in a `cfg(any(test, feature = "testkit"))` module; `pos` family `hdr-boolean`: NEW 0 non-stub, privacy 0 |

New tools (all in `F/hdr-tools/`, backups of every touched file in `orig/`): `hostpeel.py` (one `use super::*` child -> crate rooted at its file), `clumppeel.py` (coupled children, lifted fns and consts, `--precise`, `--testkit`), `mirrorsplit.py` (test mounts of a path-preserving mirror -> companion with its own mirror), `testcomp.py` (white-box test module -> companion with testkit accessors), `binsplit.py`, `hubdeps.py` (which sibling and root definitions a child really uses), `chk2.sh` (one unique sandbox per artifact), `sample.py` (prints the sampled extras of a check log). `famsplit.py` gained `--pin-word`, wrapper containers with sibling items.

Not done:
* `job` (0.991, test bytes): the three white-box tests (retained-ownership, payload, builder; 25 KB) reach the private `JobPayloadOperationLedger`, the writer's private `staged`/`rejected` fields and `StepContext` internals; a testkit accessor surface would expose about 30 private members of the payload knot, and no child module can leave because every module names root items (`hubdeps.py`). The only sound cut is moving the knot itself (ledger, writer, step context, about 60 KB of the 75 KB root) into a lower crate; not started.
* `puzzle-5d-interaction` (0.92), `puzzle-2d-mutations` (0.915), `puzzle-5d-dispatch` (0.902 by 7 lines): the extra test lines sit in `pub mod tests` children of local nest modules (`use super::*`), and a mirror companion would duplicate the local lib code, which is the bound; they need a module peel like the process3d host.
* `process3d-model` 0.922 (bytes): one 9 KB test file under the binary snapshot module.
* Peer defect seen by the process3d check: the artifact's `✏️editor/🦀️.rs` imports `internal_action`, `PROCESS3D_EXAMPLE_PLATE`, `PROCESS3D_EXAMPLE_CONCRETE_FOREST` and `process3d_interaction_definition` twice (line 36 and lines 253-256, file touched 21:51), four E0252 in lib and test; not mine, not touched.
* The check logs of the media-bridge run showed unresolved `Dialect`, `IoDirection`, `SubsetId`, `IoPayload`: my `use` pruner dropped the `use { a::X, b::Y };` form. Fixed in `clumppeel.py`, `hostpeel.py`, `testcomp.py` and in the clump root; re-checked below.

