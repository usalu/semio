# Eurocode surface and example audit

## Diagram and tree publication

The EN 1990 browser repro failed with `scene-surface.encode` while admitting a 102,717-byte scene to the 32 KiB fixed scene document. EN 1992 and EN 1998 reported the same admission failure with 80,731- and 34,846-byte scenes, respectively. Canvas 2D now places nonempty diagram layers in a retained text lane. The host reconstructs the existing `layersJson` field after the lane arrives. Diagram data stays intact and the fixed scene document limit stays at 32 KiB. The browser conformance tests reconstructed all 500 layers from the shared 107,501-byte fixture. The restarted Play server returned the updated canvas lane module, the scene interpreter and the tree-window implementation as JavaScript.

EN 1991 and EN 1993 each have 35 registered outliner groups. EN 1991 has 34 insert tools and EN 1993 has 35. The fixed UI list holds 32 entries. The shared outliner now windows the groups and keeps the nested collection windows. Both Model windows use the host's tree-window requests for large Insert palettes. Select appears first in a windowed palette so it is immediately available for disarming an insert tool. Palettes that fit the fixed row count keep all their rows and their existing order. The standalone WASI toolbar check passed with all 32 fixed rows and returned the final requested row of a 35-row EN 1991 palette. The EN 1993 regression tests request the last Outliner and Insert rows through `ViewModel.tree_windows`.

The focused Play fixture requires current retained 3D geometry, a matching real draw receipt and visible model pixels. It delivers the blank and restore example actions through the picker, applies viewport zoom, observes the last paged Insert item for EN 1991 and EN 1993 and checks 2D diagram pixels for EN 1990 and EN 1992. The EN 1990 control run against the earlier guest reproduced the original fixed-capacity admission error before Play could publish a ready shell. EN 1990 and EN 1991 both reached `setActiveExample` on the fresh `wOltbvh` Eurocode core and failed at `original-turn.source-not-ready` before opening any surface. This blocks positive integrated Play verification for both routes.

The independent WASI fixture evaluates the current EN 1990–1993 samples and calls their scene producers, viewport lowering and retained scene carrier assembly. A standalone Chromium fixture loads those retained nodes through Play's React surface interpreter and concrete canvas hosts. All four models drew the current geometry, had visible nonuniform interior pixels and changed camera state on pointer-wheel zoom. EN 1990 drew 12 meshes and 59 instances through a 941-byte fixed scene. EN 1991 drew 73 and 699 through 960 bytes. EN 1992 drew 72 and 392 through 973 bytes. EN 1993 drew 48 and 81 through 940 bytes. The complete EN 1990 diagrams window admitted all 14 diagrams. Every EN 1990 diagram restored its full layer payload.

Chromium exposed a second defect after lane reconstruction. The Norm producer published nested drawing nodes and byte-scale color channels. Both React and native Canvas 2D read flat path and text fields and color channels in the unit interval. The shared Norm producer now writes those fields, uses the catalog's Share Tech Mono face with its recorded glyph width and cap height, and centers the pixel-space diagram in the viewport. The largest EN 1990 diagram used a 65,874-byte complete scene and a 165-byte retained scene spine. Its Combination matrix screenshot changed from an empty board grid with three interior colors to a painted chart with 3,351 interior colors. Some labels overlap in the dense matrix. The combined build had already compiled the earlier visual-chrome source when this fix landed.

The exact EN 1992 scene test reached a second fixed UI refusal in its complete diagrams window at `ui-node-list.admission`. The marker table used a fixed list for every diagram. Marker tables above the 32-entry fixed list limit now use the host's retained tree window. The regression covers a host request for marker 40 in a one-row slice. The same 32-entry limit governs the Insert palette, whose Select row also consumes one entry.

After marker paging, EN 1992 encountered the aggregate diagrams child limit at `ui.node.children`. The shared diagram gallery pages card collections through the real host tree. The final grouping admitted seven of 61 cards in 95 retained records initially and returned the last card in 24 records. All 61 cards were reachable by an individual host request and each used at most 68 records. The registered WASI fixture closed its arena successfully. Every diagram restored its full layer payload. The largest complete scene measured 79,122 bytes and its fixed spine fit in 165 bytes. Chromium painted that scene, issued the final-card request on scroll and displayed the last retained card on a 900 × 450 pixel canvas.

The first grouped implementation still exposed more than the retained document's 128 records on some assets. The unpaged EN 1990 default produced 243 records. The diagram producer now charges exact subtree sizes against the shared budget and pages every multi-diagram collection. The tightened WASI fixture checks the initial page and every one-card request against the 128-record ceiling and exports the initial and final pages for browser verification. The final EN 1990 default admitted four of 14 cards in 98 retained records. A host request for the final card admitted it in 17 records. The largest individually requested card needed 50 records.

The React interpreter previously rendered a chart nested in a tree row in the same narrow value column used by fields. In a 900-pixel pane the live chart measured 160 × 80 pixels. The interpreter now places a surface or a container with a direct surface under the disclosure label as expanded content. It keeps ordinary controls in the value column. Chromium on the restarted Play server drew the real Combination matrix on a 900 × 450 pixel canvas and requested the final card after scrolling. The same browser check then passed with the exact initial and final galleries authored by the final EN 1990 producer. Three focused framework interpreter tests passed.

## Curated examples

The fixes preserve engineering input values while aligning column positions, record nesting, enum tags and quantity spelling with the current typed DSL. The EN 1991 office examples also declare their required empty tables.

The parser audit links the repository's typed snapshot models into a WASI command and reads the committed source assets at runtime. It parses each asset, prints canonical DSL, parses it again and compares the two canonical strings. EN 1992, EN 1993 and EN 1994 pass for all 13 committed examples, including their showcase assets. All three EN 1991 assets and all five EN 1998 assets pass. The four EN 1998 picker examples also equal their code-built snapshots. All four EN 1997 assets and all three EN 1999 assets pass. The EN 1997 decoder audit exposed the `directShear` spelling in the demo, compliant and noncompliant examples. Those examples now use `direct-shear`. The EN 1995 audit found four examples using a display string for the ultimate limit state equation. All four pass with the native `eq610` enum tag. All three EN 1996 assets pass. The EN 1990 road-bridge showcase now explicitly quotes empty text and supplies the neutral defaults for the optional second action and theoretical mean. All nine EN 1990 assets pass. The audit verified all 44 curated DSL assets for the ten Eurocode families. Browser component verification remains in progress.

All 20 checked-in Pack twins of the edited DSL examples differed. The typed `ArtifactPack` writer rebuilt them in both cohorts. A separate check decoded every written Pack and confirmed it matches the current authored DSL. The six EN 1992–1994 twins used isolated builds of those exact model sources and their 13 static schema leaves against the warmed joint component dependencies.

## Compound quantities

The scalar lexer emits a separate multiplication token in `N*m`. Quantity decoding now consumes a glued product suffix and verifies its exact symbol against the unit registry. Both record decoders and the standalone quantity parser accept the canonical spelling. The shared fixture passed through the ordinary and bounded Rust codecs in WASI. Math.js independently verified the three accepted conversions. The isolated WASI test executed the ordinary decoder, bounded decoder, both writers and standalone parser against the shared fixture. The regular Rust compound-quantity unit test passed after the test fixture adopted the current retirement effect API. The fixture now compares actual allocation extent and performed work with the reported retirement progress, and verifies that a zero grant leaves the selected retirement demand untouched. The broader retained-writer filter executed nine cases after the fixture adopted the producer invocation and reply API. Two passed. The seven remaining failures expose two existing runtime defects in `schema/🛫️encoding/🦀️.rs`. The internal nested writer close at `advance_unit` calls retirement without its original producer invocation. `RetainedRecordWriter::close_into` requires at least one child depth even when the selected empty transition quotes zero. The fixture continues to use the exact quoted depth. The full suite also found a stale relative path to the borrowed-text corpus JSON, which now resolves to the existing schema fixture. The focused rerun was canceled while waiting for compiler admission. The separate `native_retirement` integration test still has stale direct calls to the older close signature.

The two React surface lane conformance tests passed. The updated Rust Canvas 2D producer test passed in a standalone WASI harness. The shared Rust outliner and toolbar compiled for WASI. The complete WASI toolbar library also compiled after the EN 1993 wiring. The final fixed-or-windowed toolbar type-checked against the joint build’s current dependencies.

## Build verification

The restarted structural component build had failed with Rust E0492 in the tool-machine follower. It tried to extend a shared borrow of a temporary interior-mutable seed to `'static`. The current source replaces that borrow with field-by-field seeding through `NativeSeedTemplate`. An exact production `wasm32-wasip2` metadata compile against the running extension cohort passed. The narrow check included the new native-storage dependency and used an isolated response file in the ticket.

The final standalone scene checks used the extension build's coherent `p4cd526` dependency cohort. The structural fixture compiled the 46 missing EN 1992, EN 1993 and EN 1994 model, check and scene crate units in scratch using the structural Cargo graph. Its selected EN 1992 and EN 1993 crate inputs and the shared diagram producer match their production source paths. The standalone WASI and Chromium checks passed. Integrated Play route verification still depends on the fresh browser components.

## Changed files

### Production and conformance tests

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🔲️canvas2d-scene/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🛤️lanes/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️plane-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/📋️data/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🪢️views/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🧬️window-data-and-granularity-picks/🟦️.tsx`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🌲️outliner/🪟️window/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧊️viewport3d/🧰️toolbar/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🖼️canvas-lowering/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪟️window/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌲️outliner/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🖋️notation/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🗺️shape-parsing/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️decoding/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️testing/♻️retirement/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🪆️refusal/🚪️public/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧫️fixtures/🧮️compound-quantity.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌲️outliner/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

### Curated assets

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️road-bridge-showcase/🏢️road-bridge-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏗️compliant/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🚨noncompliant/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️failing-under-reinforced/❌️failing-under-reinforced/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢compliant-office-frame/🏢compliant-office-frame/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️rc-building-showcase/🏢️rc-building-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/💥failing-prestressed-beam/💥failing-prestressed-beam/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🧵compliant-prestressed-beam/🧵compliant-prestressed-beam/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/✅️heb240-compliant/✅️heb240-compliant/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏗️steel-showcase/🏗️steel-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢composite-floor-beam/🏢composite-floor-beam/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢composite-floor-beam-failing/🏢composite-floor-beam-failing/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️multi-fail-masonry/❌️multi-fail-masonry/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏘️masonry-showcase/🏘️masonry-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️multi-fail-timber/❌️multi-fail-timber/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏚️noncompliant-multi-fail/🏚️noncompliant-multi-fail/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏠️aluminium-showcase/🏠️aluminium-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️seismic-multipart-fail/⚠️seismic-multipart-fail/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️seismic-rc-frame-fail/⚠️seismic-rc-frame-fail/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-multipart/🏢️seismic-multipart/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-showcase/🏢️seismic-showcase/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️overloaded-footbridge/⚠️overloaded-footbridge/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🌉️glulam-footbridge/🌉️glulam-footbridge/🗣️.dsl.semio`

### Curated binary Pack assets

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢de-office-compliant/🏢de-office-compliant/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️multi-fail-timber/❌️multi-fail-timber/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏠️glulam-floor-beam/🏠️glulam-floor-beam/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️overloaded-footbridge/⚠️overloaded-footbridge/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🌉️glulam-footbridge/🌉️glulam-footbridge/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️multi-fail-masonry/❌️multi-fail-masonry/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🧱️loadbearing-wall/🧱️loadbearing-wall/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏗️compliant/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🚨noncompliant/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-rc-frame/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏚️noncompliant-multi-fail/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏠️aluminium-roof-purlin/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️failing-under-reinforced/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢compliant-office-frame/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🛢️liquid-retaining-fem-anchor/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🔩️high-strength-connection/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🌉️composite-bridge-girder/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢composite-floor-beam/🎒️.pack.semio`

### Ticket inputs and report

- `audit-eurocode-assets.rs`
- `audit-eurocode-packs.rs`
- `audit-eurocode-scenes.rs`
- `link-eurocode-scenes.py`
- `probe-eurocode-retained-scenes.mjs`
- `probe-eurocode-viewports.mjs`
- `audit-compound-units.mjs`
- `eurocode-surface-and-example-audit.md`

## Viewport Scene Pages and Shortcut Help

The retained UI node census counts the whole authored panel recursively, including closed sections and serialized World3d lane carriers. Before the change, EN 1992 office-frame used 273 records (172 in the world, 65 in closed shortcut help and 35 in the toolbar). The liquid-retaining model used 171 and failing-under-reinforced used 269. The retained document budget is 128.

The model sets its current geometry page through the existing `viewportAct` local transient. The shared viewport producer clamps the selected page to the model bounds, and combination or traffic-group changes reset it. Scoped shortcut rows materialize only on a real TreeWindow open request. Each materialized shortcut row includes its platform chord and localized scope. The panel reserves 16 shortcut rows so opening help preserves the same geometry page boundaries and camera. Geometry and controls together remain within the retained budget. All live editor and viewer entries supply the actual body `ViewModel` to this logic.

The authored EN 1993 steel showcase used 109 records with help closed and 125 with the first help page open. A real pager action advanced from page one to page two, and an oversized page request resolved to the last page at 90 records. Both other EN 1993 examples and expanded help also passed. `audit-eurocode-scenes.rs` and `probe-eurocode-retained-scenes.mjs` record the typed and Chromium checks. `link-norm-viewport-panels.py` rebuilds the narrow WASI panel proof against the selected dependency cohort. The scene geometry partitioner and its independent audits are listed in the companion Eurocode window admission report.

The final viewport census passed all 9 EN 1990, 3 EN 1991, 6 EN 1992 and 3 EN 1993 curated files. The largest retained viewport with help open used 127 records. EN 1990 road-bridge-showcase first exposed an additional 73-entry Field menu that overflowed the fixed list. Oversized Field and Playback menus now build only their requested host window. The authored showcase reached Field 73 of 73 and Playback 80 of 80 through `ViewModel.tree_windows`. The toolbar shares a 64-row allowance across Field, Playback, and Insert. With deliberately oversized requests for Field, Playback, and Help, the showcase admitted 127 records and retained a navigable model page. Chromium exercised the EN 1993 model page click from 301 to 67 instances with the same camera and a current geometry draw. It also reported the disclosure and scroll requests and reached shortcut 60 of 60. The staged 03:32 EN 1990 browser core refused the document transfer before rendering any window. The exact refreshed and activated 04:36 receipt `91a300a6a98f780a379a7434673585069dfe4aa716c0d579b91ab20682e3e6d6` removed the immediate refusal. Its official Chromium route still stopped on “Loading document: 118 of 118” and displayed 205 seconds without an interaction response before the 240-second route timeout. No model, diagram or rejection was published. The dedicated full-route probe is `probe-eurocode-viewports.mjs`.


The stricter scene producer accounts for retained item credits and backing bytes as well as nodes. It exposed a 130,442-byte sphere in EN 1991 bridge-showcase that could not share the original unsplit page with 91 reserved chrome nodes. The final producer split that mesh by whole triangles. The independent scene proof covered all 1,440 original instances in 16 pages and compared the split positions, normals and colors. The full authored panel proof reached Insert 35 of 35 and the last model page, with at most 111 viewport records including expanded help. The refreshed Chromium EN 1993 fixture changed from 89 to 54 instances with an identical camera and still reached shortcut 60 of 60.

The final partitioned WASI run completed all 21 files with expanded Help.

| Family | Authored panels | Maximum with Help open |
| --- | ---: | ---: |
| EN 1990 | 9 | 109 |
| EN 1991 | 3 | 111 |
| EN 1992 | 6 | 90 |
| EN 1993 | 3 | 93 |

The exact `91a300a6` browser component was also exercised on EN 1991, EN 1997 and EN 1998. Each reported idle without its requested viewport. The EN 1991 browser observer recorded a `refresh-ui` request including Model and patches for every other requested window. EN 1990 stayed on archive command settlement, reporting `more-work` with empty patch and effect lists before the Model request. Both observations use the live receipt from 04:36. The subsequent paging compiler cohort was checked separately against the authored viewport panels.


## Editor and viewer viewport state

The Play role button opens a new viewer actor for the existing artifact dialect. It captures the editor document archive before creating the viewer, restores the archive and attaches the destination before publishing it, then seeds the viewer's default view state. The Norm viewer creates its viewport from that actor's local `ViewLane`. `scene_page` is ephemeral, and shortcut windows are addressed by body key. The editor's page, camera zoom, and help disclosure therefore belong to the editor's viewport. The viewer offers model page controls under the viewer action scope and draws geometry without the editor's Insert or gesture actions.

The headed Chromium fixture uses the current Play interpreter, the production `prepareDocumentSurfaceV1` ordering with fixture ports, and retained editor and viewer trees rendered by the current WASI Norm producer for the same EN 1993 steel document. It verifies wheel zoom survives a page change from 89 to 54 instances and opening scoped Help preserves page two and its camera. The host scroll request for the last Insert entry and the request for the last shortcut both leave model page two and its camera intact. The viewport reserves the toolbar's bounded authored maximum for every host range so scrolling controls leaves page geometry stable. The current stable reserve passed all six EN 1992 and three EN 1993 authored panels in WASI, with maxima of 90 and 93 records including Help. The mounted viewer begins at page one, with the model's initial camera and Help closed. Its Next row dispatches `viewportSetScenePage` to `#viewer` and advances the readonly model. An optional live check in `probe-eurocode-viewports.mjs` clicks the actual viewer button after the route editor reaches a drawn model. Live role transfer awaits the mounted route's document archive acknowledgement.


## Long check explanations

The paired Structural `wIKruMq` preview used its matched generated bindings and core. Chromium drew the EN 1994 composite floor beam with 104 meshes, 104 instances and matching current geometry revisions. During the initial example command, the guest reported `ui.fixed-capacity` at `tree-item-description.description`. The authored Inspection panel reproduced that exact error for the fire bending check at 730 English and 754 German UTF-8 bytes and for slab insulation at 930 and 946 bytes. The retained text field is 512 bytes. The Checks and Inspection panel details share the same explanation row author.

Paired Structural previews drew the initial models on EN 1992, EN 1993 and EN 1994. EN 1992 drew 17 meshes and 221 instances plus a diagram, then stalled behind the note-shell command while changing examples. EN 1993 drew its current model after a live-cleanup zero-progress refusal on the initial example command. EN 1994 drew its current model and reproduced the long explanation fault before publication of this correction. These runs paired the private JCO bindings and core through the canonical preview2 imports. The normal Play acceptance uses the newly published component pair.

The short explanation remains an inline description. A longer one uses the existing UTF-8 text chunk renderer inside a labeled disclosure. The Interpreter places text-only stacks in the row's full-width revealed content. The report, recommendation, and check quantities retain their original text. The browser harness applies the current Interpreter function privately while 6033 serves its frozen predecessor. The headed Chromium probe reconstructed the exact 730 bytes from two paragraphs across the 900-pixel panel, then closed by mouse and reopened with Enter.

The new WASI inspection fixture checked localized explanations by selecting every check from every curated structural asset. It verified 2,602 EN 1992, 938 EN 1993 and 1,012 EN 1994 selections. The fixture matched all 188 expanded explanations byte for byte, including 2,359 UTF-8 bytes in EN 1993 steel-showcase. It also checks every retained Inspection panel against the document node limit.

Production and fixture paths for this correction

- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/✅️checks/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx`
- `audit-long-check-descriptions.rs`
- `audit-eurocode-windows.rs`
- `link-norm-viewport-panels.py`

## Canonical structural Play and EN 1993 retirement

Play at port 6033 served the structural component `71dce6d59c32431356eb22877d79b9ae158f0beee14b304c53dfd304352dd4c0` and the updated tree interpreter. EN 1994 mounted its initial 104-mesh, 104-instance World3d scene with a successful draw and no UI fault. The real Checks action to open the fire bending inspection waited behind `Note Shell Command`. The separate retained-scene fixture fed the exact 730-byte authored inspection answer through the same served React interpreter. Both text paragraphs used the full 900-pixel row, and mouse collapse and keyboard Enter worked.

EN 1993 drew its initial model, then its browser command returned `plugin.internal.zero-progress` at epoch 41803, followed by a tree-window refresh refusal. The exact canonical runtime and the captured 2,058-byte Pack and 266-byte Spr reproduced the fault without rendering or shell commands. The captured archive contains no members. Validation and old member retirement ran, and by turn 10589 the replacement had committed and entered `RetiringCommittedStore`. The pump reported zero progress at turn 12662. Giving the same fixture 256 KiB of mounted copy authority still faulted. A CPU profile confined to turns 12500–12662 located the stalled quote during `SteelJoint` field retirement. A WASI observer on the production quote recorded 32 copy bytes, 16 capacity bytes, 0 release bytes and depth 65 under the caller's grant of depth 64.

The RetireOwned derive now emits a balanced binary tree over each struct or variant's fields. Traversal preserves declaration order, and the existing paid field pair enforces each quote. The new shared law retires 66 strings with unique release sizes under the existing depth 64. The previous macro reproduced the depth 65 refusal in WASI. The rebuilt proc macro completed all 66 ordered releases and exactly 662,145 net bytes. Bun ran the schema through Ajv and computed the same counts and bytes with SQLite. The remaining integrated check is to re-run the captured archive and actual Play EN 1993 with the next structural component.

Ticket inputs for this verification are `probe-en1994-inspection.mjs`, `capture-norm-archive.mjs`, `check-original-archive-context.py`, `profile-en1993-retirement.mjs`, `inspect-en1993-retirement.py`, and `check-wide-retirement.py`. Browser and native logs are `🗑️generated/en1994-inspection-canonical.log`, `🗑️generated/en1994-description-canonical-renderer.log`, `🗑️generated/capture-en1993-canonical.log`, `🗑️generated/en1993-canonical-runtime-cleanup.log`, `🗑️generated/en1993-canonical-runtime-wide.log`, `🗑️generated/en1993-quote-stall.log`, and `🗑️generated/wide-retirement-wasi.log`.

The retirement changes affect these tracked sources and tests.

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🧬️generic-owner/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🧬️generic-owner/retirement.test.ts`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🧫️fixtures/🧬️generic-owner/🧬️.json`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json`
- `probe-eurocode-retained-scenes.mjs`
- `probe-eurocode-viewports.mjs`

Production and source-test paths for the viewport control and lane changes

- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧊️viewport3d/🧰️toolbar/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧵️lane-runtime/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧵️lane-runtime/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/👁️viewer-surface/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/👁️viewer-surface/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️viewport-panels/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`

Ticket input added for the narrow compile

- `link-norm-viewport-panels.py`
