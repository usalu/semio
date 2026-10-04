# Full Catalogue Runtime Verification — 2026-10-04

The required gate is `bun nx run @semio-tech/print:test-viz-full --skip-nx-cache --parallel=1 --output-style=static`. It builds all 81 registered visualization documents in light and dark, then independently opens every published PDF with PDF.js, reads every page and checks readable text plus the authored API text contract. Metadata/import coverage is insufficient for completion.

At handoff, the previous process was gone and only `viz-2` was published under `dist/documents`. Its previous log does not prove the full gate completed. This run preserves successful document cache and publication ownership while restarting the canonical dependency graph.

Current cached graph commands were checked against current project commands and all 81 catalogue owners and `build-viz` dependencies were present. Permanent consumption progress in the existing pipeline test now records every independently read PDF and its page count, with a final total.

Run status: waiting for the native verification initial font staging handoff before starting the canonical catalogue gate. Generated logs remain in ticket-owned `🗑️generated`; final inventory and outcome will be recorded below.

## Cache-Preserving Parallel Continuation

The first owned serial run published `viz-2` light and dark successfully. The light compile took approximately 126 seconds, making a serial 81-document run substantially slower than the initial estimate. The parent authorized continuation at parallelism 8 after initial producer font staging completed.

Inspection of installed Nx 23.2 confirmed that `--skip-nx-cache` disables writes as well as reads (`postRunSteps` receives `doNotSkipCache`); the first successful `viz-2` therefore had no new cache entry. The published consumer PDFs remain intact. Only the verified owned Nx process tree (PID 70980) was stopped; native verification and unrelated developer processes were preserved.

The resumed canonical gate is `bun x --no-install nx run @semio-tech/print:test-viz-full --parallel=8 --output-style=static`, with cache enabled, current validated target metadata and ticket-owned workspace data. This creates caches for successful unchanged documents so failures can resume. The changed per-PDF consumption verifier has never passed or been cached, so the final gate must perform the actual PDF.js reads. Generated log: `🗑️generated/print-viz-rendered-parallel-2026-10-04.log`. Execution session: 88985.

## Compiler Defect Exposed by the Actual Catalogue

The parallel run reached eight simultaneous Tectonic children and exposed `viz-64.tex:28`, `\SemioVizChart{commit-timeline}`: `Unknown authored layout option 'order'`. Its authored catalogue options include `layout=arc,order=name` on the graph family.

The graph family calls `semio_viz_layout_run` expecting positions in network kernel x/y arrays. The new generic dispatcher selects the authored-table `arc` backend because the graph data is also a registered table; that backend has a different options and output contract. Admission of `order` alone would not address the downstream output mismatch. The canonical native grammar owner was informed with exact locations; catalogue rendering continues so independently successful documents can cache.

## Discovery Wave Interrupted for Shared-Source Freeze

The parent requested interruption after shared LaTeX fixes changed the catalogue input closure. Only the verified owned parallel Nx process tree (PID 8660) was stopped; execution session 88985 exited 1 intentionally. The actual discovery log recorded seven published light/dark pairs: `viz-2`, `viz-14`, `viz-23`, `viz-33`, `viz-38`, `viz-43`, `viz-56`. It also recorded pre-fix failures `viz-64` (`order`), `viz-9` (`weight`) and `viz-61` (`order`) in authored layout admission. This is partial runtime evidence, not full-gate success.

The native owner corrected graph/flow family structure backend dispatch and a native band-kind expansion. The final gate will start only after an explicit shared LaTeX source freeze, in a fresh ticket-owned `nx-catalogue-final` workspace data directory so file hashes reflect those final sources. Cached graph/file maps from before the fixes are not trusted to establish current-input PDF validity.

## Current Taxonomy Path Verification

The current product root is `🧰️framework/🛍️products/📓️print`. The source sets of all 81 visualization catalogue documents and every declared library source (`🖋️latex`, `🔨️modules/🕸️graph/📐️.tex`) exist under that root. Current `test-viz-full`, `build-viz`, `fonts`, `deps-tectonic` and `deps-tex` commands all resolve their existing script owners from the current product package cwd. Concurrent taxonomy relocation is preserved; final fresh graph discovery will confirm the inferred targets and current file hashes again.

## Additional Native Family Verification During Freeze Preparation

The native retry6 actual graph PDF compiled, while its test inspection failed because a protected node-id macro and internal floating-point tokens remained unexpanded in probe records. Existing painted geometry/text records also showed the authored custom graph labels in first-appearance order (`b,c,a`) despite `order=name`; this possible semantic defect was reported to the native owner and parent. No full catalogue wave was launched on the strength of compilation alone. The parent separately preflighted current standalone `@semio-tech/print:test-viz` without touching staged fonts.

## Standalone Metadata Preflight Reported by Parent

The parent reported terminal exit 0 for the registered standalone `@semio-tech/print:test-viz --skip-nx-cache`: inference quick 466/466, taxonomy coverage 1,966/1,966 via 1,738 kinds, and authored API 131/131. Evidence log: `🗑️generated/print-coverage-final-retry-2026-10-04.log`. This preceded additional typed nice/guide work; the final full target will replay the latest checks. It does not replace the required 162 actual PDF consumption checks.

## Source-Frozen Build Phase

Native full registered runtime target completed with terminal exit 0 before this final phase. The parent confirmed compiler/paint closure frozen after its type-only bridges. Final metadata discovery uses fresh ticket-owned `nx-catalogue-final` data, without forced cached graph reuse. A source snapshot covers 274 authored gallery/API and shared LaTeX/compiler/paint/font/staging files; sorted path/content digest SHA-256: `377a0422bd687ab63d5d71c5dc4c081d20080d90270c1a2cd181cb315f08e999`. The snapshot remains generated scratch evidence; final integrity will be compared before completion.

The canonical `build-viz` target will publish all 162 real PDFs first; after typed guide/strict closure the canonical `test-viz-full` target will replay current tests and independently consume those correctly hashed document outputs.

Final canonical build invocation: `bun x --no-install nx run @semio-tech/print:build-viz --parallel=8 --output-style=static`, execution session 9940, log `🗑️generated/print-catalogue-build-final-2026-10-04.log`. The fresh project graph resolved all 81 current per-document script commands, source sets, shared library sources and exclusive output directories. Forced cached graph reuse is unset for this phase.

## Final Build Runtime Milestone

At 2026-10-04 03:10 Europe/Berlin, the final fresh-hash build had observed eight simultaneous ticket-owned Tectonic compilers. Its log published the frozen-source light/dark pairs for viz-64, viz-38, viz-43, viz-56 and viz-14. The corrected commit-timeline catalogue document (viz-64) is therefore available for the parent's actual visual QA. This is a partial build milestone; full PDF consumption remains pending.

## Visual Counterexample Reopened Shared-Source Freeze

The parent's actual raster inspection of viz-64 page 2, light and dark, exposed directed graph arrow tips covered by their target glyphs: 1.1 mm tips terminated at node centre, while the 1.5 mm filled target radius obscured them. The native owner reopened the stylesheet freeze and is correcting boundary-aware graph edge termination in the existing graph owner, with actual compiler/raster proof.

At 03:17 Europe/Berlin, only the verified owned catalogue Bun process 72728 subtree was stopped. All 26 captured descendants were confirmed absent afterward. Session 9940 intentionally terminated; the partial build did not pass. Eight document pairs had published: viz-64, viz-38, viz-43, viz-56, viz-14, viz-2, viz-33 and viz-9. These outputs and cache history are preserved as evidence, but they cannot be certified final after the forthcoming shared-style fix. All affected documents require fresh input hashes and canonical rebuilding after the next explicit native freeze.

## Source Hash Boundary Before Final Dispatch

The refreshed graph contains 81 exclusive current document targets and the required fonts, compiler, locked bundle and token-generation prerequisites. Its exact owner input inventory exposed a redundant historical routing path in four authored print target rows; the valid framework router is already included. Root owns removing those obsolete metadata rows.

Document owners inherit the broad font-catalog input glob, including newly derived TypeScript font metrics and their tests. Although these do not change the TeX payload, edits still change all document cache keys. Root and the native lane are stabilizing the final registered metric derivation and authoritative metadata before dispatch, so correct successful document caches remain reusable for the final full PDF consumption gate. Source snapshots will include the final generic artifact publication owner, whose post-read/final-EOF cancellation regression has just been corrected by root.

## Final Canonical Dispatch

After focused family green6 and final generic publisher correction, a second fresh graph (`nx-catalogue-runtime`) validated all concrete source inputs exist. The redundant obsolete routing rows were removed by root. Its current metadata contains 81 exclusive build owners at the relocated print product root.

Canonical `@semio-tech/print:build-viz --parallel=8 --output-style=static` is live under session14064. `NX_FORCE_REUSE_CACHED_GRAPH` is unset; daemon and plugin isolation remain disabled. Ticket-owned workspace graph and compiler scratch/log paths are used. Successful unchanged document caches remain enabled.

Source-bound compiler closure snapshot: 294 actual files, joined sorted path:SHA256 digest `0bd38d025a91b93956c4a1005023dee7890774c5a0ef3c44579faa3bf56f2d44`. A replay immediately after producer staging found zero differences. Actual observed catalogue Tectonic concurrency is eight, independent of the native probe compiler. Initial active document owners: viz-2,56,38,33,14,30,43,64. No completion is claimed yet.

| Key Frozen Owner | SHA256 |
| --- | --- |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts | `4a90b596be3674c603c0b2a091f9f937284bff9173e89f4f070573895dc29907` |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json | `68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb` |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json | `4eaf526eacd253559604ec9a4aaf9b4319abaff487bd7116e611e8537b36a8bf` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-flowchart.sty | `1b4f0f70fbece1c296913a63b89bce1760179f158272847398ff2891c89ca30d` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-process.sty | `b4e830d9a6ae944a7326a1088b1c78d15ed757f1651549483ded56db06594cb7` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-flow-sankey.sty | `b734b8aa5db1165687aecaa38a7acc236b39a579106cd7c47de2437f9ab7d32f` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-chemistry.sty | `d5049e9c4121b6c2a92e76ac53f4d810558e727c5b0cb2284005497b1a259d3b` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-field.sty | `3087d2ff69c4cb68de6ad32b8b8ae8e7f8d5e23c13a9b503e23fd87f33feb62f` |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-physics.sty | `9f37c3e89a237eda877e9e374bce8efc8d500a02eb94de7ce1511d31b5642a39` |

### Evaluation Neutral Compiler Counterexample

Registered `@semio-tech/print:test-native-grammar -- --families-only` with `PRINT_NATIVE_FAMILY_PHASE=evaluation` terminated with exit 1 in 1m 5s (session 88817). The actual neutral `regions.tex:11` compiler diagnostic is `Missing ) inserted for expression`, matching catalogue `viz-24.tex:96`. The existing decision-region palette passes a comparison and ternary into integer arithmetic. Two language-neutral cases retain independent D3 cell geometry and threshold classification expectations for both themes; their actual compiler replay must pass after correction. No evaluation production owner was edited during this discovery wave.

The current wave has additionally exposed `viz-25.tex:44` (`riemann-surface-visualization`, `Use of \??? doesn't match its definition`). Current published count is 33 document pairs; this mixed-revision discovery wave is not final certification.

### Discovery Handle Loss

On resuming, session 14064 returned `Unknown process id`; no catalogue Bun/Nx/Tectonic process remained. Its retained log ends at publication of viz-72, contains 34 published document pairs and five actual compiler failures (viz-24, viz-79, viz-37, viz-17 and viz-25), and has no Nx terminal summary. This is incomplete discovery, not an observed terminal completion. The subsequent catalogue replay requires fresh graph/file hashes after the coordinated native fixes.

### Current Fresh Catalogue Graph And Source Binding

Fresh isolated `nx-catalogue-frozen` show-project completed exit0 (59843). Its aggregate `build-viz` dependency list exactly matches current catalog visualizations: eighty numbered documents plus API,81 independent cacheable source/output owners. Current repo plugin `printDocumentTargets` at library/🟨️.mjs line994 replaces literal project aggregate dependencies with catalog document owners; line991 generates each owned build command. Thus the declared `complete visualizations` aggregate command runs only after all81 owners. Fresh graph inputs/commands match the earlier repaired graph for every visualization owner; no source path or stale router mismatch remains. `test-viz-full` depends on that aggregate and consumes transitive PDF outputs.

The current compiler-input inventory contains294 exact files admitted by the complete81-owner set, including literal command closure and expanded pinned library/font/toolchain sources. Current sorted inventory SHA256 is `2779978781637d6c3c9f13fee4b924a1bf6972a172483c8f9b45cd90d7047fdd`; it will be compared again before dispatch and after all actual publications. Seven native stylesheet owners differ from the mixed discovery-wave snapshot: finance, statistics, architecture, guide, plot, mathematics and showcase. Current mathematical source is `740dd6a211654a74b0b60dd07a0a4086b973d94371feb2bf0258d86731485a5c`.

Bifurcation green4 produced an actual light PDF after three successful TeX passes (8m13s): all7,280 neutral stock/custom numerical records matched D3 canvas/stable-logistic expectations. Actual Poppler views covered pages2/3; full stock cascade and custom short tails are visible and contained. The target then correctly stopped at a test decoding mistake: current PDF.js folds fill opcodes into constructPath args[0], whereas the first assertion counted standalone fill only. The assertion was corrected using the installed PDF.js worker implementation; both-theme green5 and broader stock25 remain live. No native production change followed that first published PDF.

### Source Supersession After Actual Visual Discovery

The provisional frozen81-owner wave84476 is explicitly superseded as final certification. Actual source-covered viz33 molecular-orbital labels contained missing U+FFFF glyph boxes and outer fragment rungs exceeded the frame. Root reproduced eight neutral before failures and is correcting the existing chemistry owner, including preserving literal math records in its slash parser. Independently, actual paired viz7 tree pages placed roots and descendants below their frames, through the footer; native sibling owns the neutral compiler/geometry repair. These are actual visual blockers, not warning-only speculation. The current wave continues to discover every remaining compiler owner failure; no successful owner is canceled solely for a warning. Later staged sources may include corrections, so it is an incremental mixed-source discovery wave. Its294-file dispatch inventory remains historical evidence; fresh current chemistry/tree hashes and a new complete canonical81/162 replay are required after those owners freeze.

Old discovery viz33 completed paired publication and removed its stage; no matching live builder/compiler processes remained before root’s current all-stock33 proof. Current discovery reached24 published pairs with zero new compiler errors and no missing code points beyond sigma. Glyph consumer neutral/actual regression now rejects the observed failure and all67 forbidden extraction scalars; final full PDF consumption must use it.

### Continuing Discovery Publication Census

Owned live handle84476 has published 39 distinct document pairs at 2026-10-04 14:42:20 +02:00. Actual compiler error count is 0; the only missing-glyph code point in the retained log is historical U+03C3 from old staged viz33. Viz1 and50 are now available to the independent visual audit. This census is progress evidence, not terminal or current-source full certification. Successful cache/output history is retained while root chemistry and native tree corrections close.

Published IDs: viz-64, viz-38, viz-56, viz-43, viz-14, viz-2, viz-33, viz-23, viz-9, viz-27, viz-7, viz-34, viz-61, viz-35, viz-6, viz-45, viz-18, viz-73, viz-5, viz-74, viz-24, viz-53, viz-44, viz-62, viz-79, viz-28, viz-37, viz-42, viz-63, viz-3, viz-50, viz-17, viz-54, viz-57, viz-19, viz-16, viz-41, viz-1, viz-52.


Chemistry source2dd740454e7640da268fa5f7db9027ddfea0c2f93065efaee6ea2552f8401442 is frozen after root paired focused58497 and current all-stock3388722 terminal0 (6m58s). Both current eight-page/22-caption PDFs passed independent pypdf all-page invalid-glyph checks; actual paired page2 and four focused pages were viewed. Root retained exact current hashes and evidence in 📓️chemistry-runtime-2026-10-04.md. Final fresh catalogue replay still awaits native tree containment freeze.


### Published Physics And Grammar Inspection Binding

Physics30 and grammar79 have terminal paired publication in wave84476. Their specific native family owners remain unchanged; this is source-covered visual evidence within the globally mixed discovery wave. Identical grammar byte publication preserved prior timestamps.

| Document | SHA256 |
| --- | --- |
| ⚛️viz-30-dark.pdf | `6efd466ff47bca97b5c31ca391704191cffae408950e1e3a1ab45d244a911e1c` |
| ⚛️viz-30.pdf | `9d11c8a2910f2f4b66ee454c21b7878c7ddbc0f8902ddb5d3a8db4d413ae3494` |
| 🏗️viz-79-dark.pdf | `b1a5c04d452dfea17cf4a94f4b8a532464fa9af75029c0693f507cb0ab9bf324` |
| 🏗️viz-79.pdf | `9039b62816448d8624fb19336c895f67e36cbfd1fb548c44a44c6bb3164ed945` |
Discovery84476 is terminal with outer status130 and Nx failure summary after85m58s:all80 numbered pairs published, API failed publication after both actual PDFs compiled. Generic stageArtifacts line83 raised EPERM on directory rename; root owns its neutral publisher regression/correction. This is not a full81 or162 PASS.


Canonical final source-conditioned runtime25313 started onfresh isolated generated/nx-catalogue-current-final after show-project90192 and294-source snapshot869e572d… . Exact current81 owners plus86 dependencies are scheduled by test-viz-full, cacheenabled,parallel8,stream output. All294 input hashes were rechecked immediately before dispatch withzero drift. Firstobserved catalogue TeX owners are2/56/38/33/14/30/43/64, eight concurrent catalogue compiler children; independent same-source stock30 proof88613 remains live. Physics focused16-case/32-reference/20-page proof66417 is terminal0 and14 current paired frame views clear. This is still a live conditional gate, not final certification. Final terminal/162 consumption and stock30 paired views remain required.

## Current Route25313 Superseded By Observed Physics Defect

Route25313 was intentionally canceled with terminal−1 after33 numbered owner pairs/66 PDFs; no full PDF consumer ran. Exact owned process root70108 plus28 descendants were stopped only after verifying independent stock88613 root59260 was excluded. All owned processes are absent. The retained cancellation ownership JSON records this scope. Stock88613 subsequently completed exit0 and published both physics245c16ff PDFs in40m3s.

Actual stock30 page4 exposes potential curve/n=4 caption intersection. This is a new declared-caption defect; the current294 snapshot and current publications are now before/reference evidence. Existing physics neutral tests are being expanded to actual PDF stroke–caption geometry before source correction. Valid caches and all prior publication history remain intact. A fresh current source graph and ordinary cache-enabled81-owner/162-consumer route are still required after this correction and API freshness gate integration freeze.

## Canceled Route Publication Inventory

| Owner | PDF | SHA256 |
|---|---|---|
| viz-2 | ⏱️viz-2-dark.pdf | 5082BC9DCC685960AA1C17C12D287295BD820969A56720014C64C57CE636B351 |
| viz-2 | ⏱️viz-2.pdf | 661375F5D5CE2AA5F29D0D90D88D574C2F9C918641B44302595D61F4A4A8D1FD |
| viz-3 | 🎻️viz-3-dark.pdf | 9890F85AB88A087C6FE055F8844567B425FD81DDD87FAF63E41712E53573F004 |
| viz-3 | 🎻️viz-3.pdf | 4ABF730E7348B480CEAB2A891521152825C5A890078C70A98526076AEEA5AE19 |
| viz-5 | 🎛️viz-5-dark.pdf | 1E1D06DDA4E54EBB9F2782F33BD1BF8C755A0D965C164EB602B27199C964BFF6 |
| viz-5 | 🎛️viz-5.pdf | 4E23537E820234C33986AE1015EA10D35AF46F63F16CC870CE059D05D450FD94 |
| viz-6 | 🍰️viz-6-dark.pdf | 44A279D16024552090C548FFE3134359C23CCF3878E8172B1699DAC99943735E |
| viz-6 | 🍰️viz-6.pdf | 3D6940A53A8D5804A79EEA3C48DE3047548F4AF494DC840C58C7E3EA91D8C6B9 |
| viz-7 | 🌳️viz-7-dark.pdf | 98D2C1F9BEBDACCA95237BC0EB4C41151CEDDEBEAB2C9CEB8C23A0CBADB49673 |
| viz-7 | 🌳️viz-7.pdf | CB8FD093FE060C9C4EEDEC5A5EBAE3E8F1B4002DAB2CA8659FCCB2983CD7C0C9 |
| viz-9 | 🌊️viz-9-dark.pdf | 475F21387C5DF69DD43A21C81F3473A88F37B18682DCE1EACC038994AC0A194D |
| viz-9 | 🌊️viz-9.pdf | 445C3D63BA1DA4E65E9F4C7236244F83D4B22294F15EFDD6EEA61C5D75E6C355 |
| viz-14 | ⚙️viz-14-dark.pdf | 375807F4902F8D7DC8B1C542032539C240259692A557852FDC8C3D088B88F647 |
| viz-14 | ⚙️viz-14.pdf | 07B70B15FC137063D84CDF46B3DAE6EBCBEBCEF68F51B4BD81BCF55A9A9EBAA0 |
| viz-17 | 💹️viz-17-dark.pdf | 1C926259DF11E2D29591E94239E364B95F1D4667B77CA3EC911058CA7C468010 |
| viz-17 | 💹️viz-17.pdf | 5ACBFC4BEAE17699E4F0BC42D67558E3DF68AA8B75371950A7C6F68363B4D4A5 |
| viz-18 | 🎚️viz-18-dark.pdf | E42A239FBBAD81F4916C067BC7389554488F4E095B5AFC63A89CDCE5BF98B7A6 |
| viz-18 | 🎚️viz-18.pdf | 99FC87A74686CB6184FDE3855F8EB5971CCE709B61EA880408267321DAC5DB62 |
| viz-23 | ❓️viz-23-dark.pdf | D90538144D4768EAC2C6BBD542E66C6C3C60F6A4B5594A4C0CB1F710A176858C |
| viz-23 | ❓️viz-23.pdf | BDA3C9ECF6FA9355D78005DBCE9CE952611DA71E9A1B1A284FD9F3510ECBAEC4 |
| viz-24 | 🎯️viz-24-dark.pdf | 0A11CD0AB4A2725BC94804B91E6C76A3E57F7E44EFFBCD9B8218DF4837E85A4F |
| viz-24 | 🎯️viz-24.pdf | FCB90FE952ABA2EABF15DB7328E33B58C9641CDBFD182DA40D5B548B60D8E0AF |
| viz-27 | ➡️viz-27-dark.pdf | 98E4A38475EC44E95CC1D976F34810CC647FE1CCB22D4458D97B12EC3C515245 |
| viz-27 | ➡️viz-27.pdf | 3A9AEA241722FD8ED55DC21CF2E2535FBCF1A797FE7680375609A15962835347 |
| viz-28 | 🏔️viz-28-dark.pdf | 3DE0609A3481A6A4E4D97AB4509E7C1D45BF55FA550613B7EECA4B1EE4B1BE94 |
| viz-28 | 🏔️viz-28.pdf | 9723F7801B711ACDCAC4D4F31CF7DC834EDE47D47D2C35C04FB8C3F2F4BE6644 |
| viz-33 | ⚗️viz-33-dark.pdf | 655B53C1E48706A7B7C808F26677AA32331792D46E00FFFACD47AD70517DDE4B |
| viz-33 | ⚗️viz-33.pdf | 5137C5DCC4CA7AE8B09E090888B95C8D0C94768DE5DF5A1456DEAB3B668C6D43 |
| viz-34 | 🌿️viz-34-dark.pdf | A3A31889F81112FD002CD9757F2E57BAED8710D789903900D3BC047881F78166 |
| viz-34 | 🌿️viz-34.pdf | 1D293A95357DAA390BEEFE1C41392B5067673517A33396AB712902F75DB4AB41 |
| viz-35 | 🌦️viz-35-dark.pdf | 06CEC4B8F6FE0D2ED775DD200898565044831A02B7447BCF41E29CC3E0DEE286 |
| viz-35 | 🌦️viz-35.pdf | FF89128031AD1361BB96B466EAEBB390EB292317903034291811268B5D43648C |
| viz-37 | 🏛️viz-37-dark.pdf | C7871B398CAA1523828B39F044ACB3B34493B2E45447E7DE11F298796E55D33E |
| viz-37 | 🏛️viz-37.pdf | AD3FA62A9983C2D3EB0CC27D4807C58F8ABCFDF68491D98BF1EF636D3EDA8FFF |
| viz-38 | ♾️viz-38-dark.pdf | B3DB9AA5E572F45A0E43EA83D4B9BA8691E6910196C1FFB64FC74BE5EFBE8F9B |
| viz-38 | ♾️viz-38.pdf | 9AFDC5000199EA6EDA6EE022647C24B42AEB8BD1D27FFFFA317997FDC7ECD7C3 |
| viz-42 | 🐟️viz-42-dark.pdf | 0DB155D705AC5700438794E339843913D94FD81395E2E555A1E063CAC2A2BA2A |
| viz-42 | 🐟️viz-42.pdf | 4EFB3FAB7F051AC763F47E39E314F4E161C48D62A9E61D5113F7F45943003EA7 |
| viz-43 | ⚽️viz-43-dark.pdf | F7B521CA58DEEA01813686C36EA97A132EC4A284B6F02CDEFE219844729158A8 |
| viz-43 | ⚽️viz-43.pdf | A3856621EE2AC463274E91B1131956BD06C10D84011B5A29BA3888C9979C130A |
| viz-44 | 🎼️viz-44-dark.pdf | 37C88D1B9D88369D1620FD8E096CBF3904148265DE3F58284AAB947D05C61D48 |
| viz-44 | 🎼️viz-44.pdf | C9E97DC428BE34EE070142DA91146DF474507FC4EFDCDA54D538BD5159E80DB3 |
| viz-45 | 🎓️viz-45-dark.pdf | 7AD6B4770FE1EDED6B6BE33E225B8D1E2426EA2A898AE1514926865F22082727 |
| viz-45 | 🎓️viz-45.pdf | A854D636A2C8365DB7AD66C1EF47109401C1641526B8683A1579D81038D96DAC |
| viz-50 | 💬️viz-50-dark.pdf | 0573FAC625477761488C8285EC830A83F5D779F7039EBE64C96BFC3DA7D3BA84 |
| viz-50 | 💬️viz-50.pdf | DCAB54CFA3D1B6BFA973E8B6D45438BD3651EBD32B71E8519D58D89A6B1FB30B |
| viz-53 | 🏁️viz-53-dark.pdf | FFD3AEFFB6BBB7BF5760004391349F3AC8D1B491AD7067B2FA4D858AD4FBBE33 |
| viz-53 | 🏁️viz-53.pdf | 3717D28ED33AA5E8DE7185F95514E421F35AF95E3CD0A1B9681E3ED84C60041A |
| viz-54 | 📅️viz-54-dark.pdf | 1A8A78524CC0822FCF97C5C016756D148C11BD297DF0CB64CB65AC4AA0B933C5 |
| viz-54 | 📅️viz-54.pdf | A5BC146650D2DDBD348F2A536F928B5365AD9DD5A2B698D0594F4F9A1F6571B5 |
| viz-56 | ♟️viz-56-dark.pdf | 34A1E80F51C21B778522E0599B62D58D2B958BB088E7F6FFD48160EA0C86FBDC |
| viz-56 | ♟️viz-56.pdf | 3EA3F9B4F57433495887A4CC12D376FF7C339DD002865B234A74F97C0FB1F206 |
| viz-61 | 🌀️viz-61-dark.pdf | 233A017A38E692A5636484D172B4C8522C56089DB95C2675BCCBBD2EF6F820CC |
| viz-61 | 🌀️viz-61.pdf | 66707B32ED4B435ABD8FD4FA73CA069C71C4A6A0A252C0AD6772FBA0D649717B |
| viz-62 | 🏎️viz-62-dark.pdf | ADA28883CEE97B8448F1D5BEEAC553182AD9257A806F6CC6DF0BC7A158E4027B |
| viz-62 | 🏎️viz-62.pdf | 5E5654C1CBB968A4020F777E6E10B86251BD7A5403A3EB146363275AB64095C6 |
| viz-63 | 👣️viz-63-dark.pdf | 948D214239364E9B68C0D0A1A0A6279CD009B16796F3405A8502F0490929C322 |
| viz-63 | 👣️viz-63.pdf | 93B3CF6EFFFB49BF3184AE1F353E1391CDAE44A018F00B7CE5E76AAB944C69AB |
| viz-64 | ⛓️viz-64-dark.pdf | BCCC291A59FE28D7E290599C21172E3F339E1862B975D04136CF47B234623DF3 |
| viz-64 | ⛓️viz-64.pdf | 8A2B945DC639C4487175F80B567D2C66768B0082A5343E45D0B2F1929BADBE15 |
| viz-73 | 🎞️viz-73-dark.pdf | 6609985509E063F3C1E0892882003DB7FFF686FC88955D5D06CC5391E301D04B |
| viz-73 | 🎞️viz-73.pdf | DF25E82042A69E97BF3412F31C2240DB2F5368FC5EDF8CEA6CDDAE5D5286B874 |
| viz-74 | 🎨️viz-74-dark.pdf | 4E2DF0935E89A79B83DF12F085B067001766CE198AC94ACEBFC6B8F8D1464580 |
| viz-74 | 🎨️viz-74.pdf | D8F828B6E523D1DAAC7FF83CE68D5EBE4B538F77CD347E9698715432EBC1B743 |
| viz-79 | 🏗️viz-79-dark.pdf | 8979434297DB1A1A2EE22A6152064874BD2940497EEE5004E9E162321419D2B4 |
| viz-79 | 🏗️viz-79.pdf | 0240BE5574ED2267B328E4EBD78964E5B997DC6D24F672C5F4B6142659D6FF61 |
