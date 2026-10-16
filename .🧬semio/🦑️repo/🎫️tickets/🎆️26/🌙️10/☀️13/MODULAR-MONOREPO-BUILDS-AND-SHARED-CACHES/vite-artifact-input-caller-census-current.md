# Vite Artifact Input Caller Census

2026-10-15. Source-only census; no physical execution or code edit. Paths below are repository-relative. Current producer helpers have changed ahead of several consuming legacy dist references; do not infer that those old references resolve to staged products.

## Actual Generic Production Calls

| Call | Actual Source |
|---|---|
| Generic build bridge | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1753`, `runViteBuild`, caller-selected config |
| OS distribution build | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts:52`, contribution config or OS config; renderer forced react |
| Play site build | `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts:54`, Play config; renderer react |
| Demonstrator site build | `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/📜️script.ts:18`, Demonstrator config; renderer react |
| Play runtime serve and E2E serve | `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts:45,71`, Play config |
| Demonstrator runtime serve and E2E serve | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts:62,87`, Demonstrator config |
| WGPU actual browser server | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:31`, selected composition, native config loader, renderer forced wgpu |

Repository-wide production `buildViteArtifact`/`serveVite` textual census found no additional direct calls. Tests are excluded. The app configs above are separate configs; they must not be mistaken for direct imports of the OS config.

## Actual Bypasses Requiring Closure

1. OS `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:55` calls `runViteBunxDev` with selected contribution config or `../../🏗️builder/🌐️vite/🟦️.ts`. Generic library `🟦️.ts:1941-1997` implements this by direct `spawnBunx(["vite",...])`, bypassing `serveVite` entirely. Add explicit input lifetime to this actual route or route it through the authorized service; adding inputs only to WGPU/serveVite leaves ordinary OS React dev exposed.
2. OS `🧑‍💻dev/🚚️distribution/📋️plan/🟦️.ts:132-137` resolves contribution/OS config, imports it, invokes config with command build, then directly calls imported Vite `build` with `write:false`. This is real input-witness construction and still reads staged/static assets. It bypasses `buildViteArtifact`; input custody must enclose config import and the complete build.
3. Generic `runViteBuild` does not infer OS ownership from config. Its caller-selected config can be any contribution. Explicit artifact request forwarding belongs in its options if OS callers continue through it.

No additional production direct Vite createServer consuming OS config was found. Core generic serveVite owns its own createServer. The relevant OS-config contribution imports are exactly Demonstrator `🏗️builder/🌐️vite/🐚️shell/🟦️.ts`, `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🏗️builder/🌐️vite/🟦️.ts`, and the equivalent GIS variant. Their `createFrameworkOsDevConfig` exports are consumed through selected contribution paths, including the two bypasses above.

## Exact Staged Producer Descriptors

Let P be the actual products store root and M the original prototype/release mode.

* Infinite fonts: `fontAssetPublicationRootV1(P,M)` returns `P/infinite/M/fonts`. Owner `infinite:fonts`, authored filename `🔤️guestslim-typst-fonts.bin`. Producer script is Infinite `📦️packages/🦀️rust/📜️script.ts:43`. Its reader protocol at `♾️infinite/🔤️fonts/👓️reading/🧬️schema/🔣️.json` specifies asset max134217728, manifest max1048576, maxFiles1. Reader maximumBytes must include manifest:135266304.
* Renderer: `rendererWasmPublicationV1(P,{version:1,owner:R,mode:M})` returns `P/renderer/sha256(R).slice(0,20)/M/wasm`, manifest owner `wgpu-renderer:M`. R is repository-relative actual Rust package root `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`. Producer `🏗️compiler/🌐️wasm/📜️script.ts:85-91` inventories actual Trunk output and requires package-name `.js` and `_bg.wasm`; it does not cap to only those two files. There is no renderer-specific maxbytes/files in its publication schema. Actual generic publisher caps are536870912 payload bytes,1048576 manifest bytes,16384 authored files: reader max537919488/maxFiles16384 covers that declared producer contract. Tool stdout's134217728 limit is unrelated to artifact bytes.
* Canvas: `runtimeAssetStageTrees(P,M)` returns six descriptors under `P/canvas/M/{port-assets,guest-assets}/{compiler-fonts,canvas-fonts,icon-pack}`. Port owner `canvas:runtime-assets`, guest owner `canvas:package-assets`. Port authored file counts are5/1/1. Guest counts are1/1/1: the guest compiler-font subset is NotoColorEmoji-subset.ttf. Canvas script skips empty trees. Whole port compiler-font set: LibertinusMath-Regular.otf, LibertinusSerif-Regular.otf, LibertinusSerif-Italic.otf, LibertinusMono-Regular.otf, NotoColorEmoji-subset.ttf, each with its actual emoji prefix from compiler world Source. Canvas label filename `🔤️MapLabelSans.ttf`, icons filename `🔣️.json`. Canvas staging schema has no narrower byte cap: use actual generic publisher byte/manifest bounds or author a separate explicit lower contract. Exact inventory can use descriptor.files.length for maximumFiles.

Fonts and Canvas are needed by both React and WGPU paths where mounted. WGPU renderer product should be conditional on renderer==wgpu or an actually selected consumer requiring its wasm alias, not unconditionally added to all React builds just because OS config declares an alias. Actual WGPU server checks JS/wasm existence in `🌐️server/🟦️.ts:104` and is an unconditional renderer consumer. Keep SH until actual server.close/optimizer drains, not merely readiness.

## Current Consumer Drift To Fix At The Same Time

OS config currently computes rendererModulesDir from repository `.../📦️packages/🦀️rust/dist/wasm-M` (line50) and fontsDir from repository `.../♾️infinite/📦️packages/🦀️rust/dist/fonts` (line57). Canvas dev mount uses `runtimeAssetDeliveries()` repository source roots, not staged roots. Runtime package asset routes also need the actual staged guest descriptors if the design requires all staged custody.

WGPU completed roots currently return the repository dist/wasm-M root (`🌐️server/🎚️config/🟦️.ts:17-23`), plus separate generated boot/library/worker roots; its compiled-root mount must be changed alongside the reader request. A lease on P/renderer/... cannot protect bytes served from old repository dist.

Play runtime asset Source currently still points Infinite fonts at repository dist and calls `canvasRuntimeAssetSources(workspace)` with no mode even though the current helper requires productsRoot and mode. Demonstrator has analogous legacy production asset-source logic. Play dev excludes staged Canvas rows and mounts source runtimeAssetDeliveries directly. Ensure app serve/build calls pass the same original M and actual P used by their updated config/asset-source providers; adding a correct request for unrelated roots is insufficient.

The census is finite textual/Source inspection, not browser/runtime evidence or a claim that current drifting callers typecheck. Root coedits may supersede the exact observed lines.
