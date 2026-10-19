# Packages that TS suites import but that do not resolve from the suite location (for the deps executor)

Layout cause: `bunfig.toml` uses `linker = "isolated"` with empty hoist patterns, so a suite outside a workspace package only sees the 54 root devDependencies. No manifest was edited by this task (freeze respected). `version` is the highest version present in the current store generation (`.cache/b/d/m/j/441fd20cf4b20f6dcafe`); `NOT IN STORE` means the package is in no generation at all and needs a lock/sync.

| package | store version | suites needing it |
|---|---|---|
| `brepjs-opencascade` | NOT IN STORE | 2 |
| `execa` | NOT IN STORE | 2 |
| `tree-kill` | NOT IN STORE | 2 |
| `cli-truncate` | NOT IN STORE | 1 |
| `color` | NOT IN STORE | 1 |
| `retry` | NOT IN STORE | 1 |
| `rxjs` | NOT IN STORE | 1 |
| `@testing-library/react` | 16.3.3 | 89 |
| `@testing-library/user-event` | 14.6.8 | 27 |
| `jsonc-parser` | 3.3.1 | 23 |
| `fast-check` | 3.23.2 | 16 |
| `immer` | 10.1.1 | 16 |
| `@noble/hashes` | 2.4.0 | 14 |
| `gl-matrix` | 3.4.3 | 14 |
| `dom-accessibility-api` | 0.6.3 | 12 |
| `xstate` | 5.33.3 | 12 |
| `opentype.js` | 1.3.4 | 11 |
| `pngjs` | 7.0.0 | 11 |
| `@tailwindcss/node` | 4.3.3 | 9 |
| `i18next` | 25.10.10 | 7 |
| `jsdom` | 24.1.3 | 7 |
| `polygon-clipping` | 0.15.7 | 7 |
| `fast-json-stable-stringify` | 2.1.0 | 6 |
| `json5` | 2.2.3 | 6 |
| `minimatch` | 10.2.6 | 6 |
| `smol-toml` | 1.6.1 | 6 |
| `react-i18next` | 16.6.6 | 5 |
| `@tailwindcss/vite` | 4.3.3 | 4 |
| `@vitejs/plugin-react` | 5.2.0 | 4 |
| `@webassemblyjs/leb128` | 1.13.2 | 4 |
| `commander` | 14.0.3 | 4 |
| `d3-hierarchy` | 3.1.2 | 4 |
| `mathjs` | 14.0.0 | 4 |
| `whatwg-url` | 14.2.0 | 4 |
| `@react-three/fiber` | 9.8.2 | 3 |
| `aria-query` | 5.3.0 | 3 |
| `d3-array` | 3.2.4 | 3 |
| `diff` | 8.0.4 | 3 |
| `lightningcss` | 1.32.0 | 3 |
| `pathe` | 2.0.3 | 3 |
| `postcss` | 8.5.30 | 3 |
| `yargs-parser` | 21.1.1 | 3 |
| `@bytecodealliance/jco` | 1.39.0 | 2 |
| `@modelcontextprotocol/sdk` | 1.32.1 | 2 |
| `base64-js` | 1.5.1 | 2 |
| `colord` | 2.10.0 | 2 |
| `cross-spawn` | 7.0.6 | 2 |
| `d3-ease` | 3.0.1 | 2 |
| `d3-scale` | 4.0.2 | 2 |
| `find-up` | 5.0.0 | 2 |
| `lodash-es` | 4.17.23 | 2 |
| `parse5` | 7.3.0 | 2 |
| `path-to-regexp` | 8.4.2 | 2 |
| `tree-sitter-wasms` | 0.1.13 | 2 |
| `@bytecodealliance/preview2-shim` | 0.30.0 | 1 |
| `@csstools/css-calc` | 2.1.4 | 1 |
| `@csstools/css-tokenizer` | 3.0.4 | 1 |
| `@gltf-transform/core` | 4.5.1 | 1 |
| `@iarna/toml` | 2.2.5 | 1 |
| `@tailwindcss/oxide` | 4.3.3 | 1 |
| `@testing-library/dom` | 10.4.2 | 1 |
| `@xyflow/react` | 12.12.0 | 1 |
| `ajv` | 8.20.0 | 1 |
| `bidi-js` | 1.1.0 | 1 |
| `check-error` | 2.1.3 | 1 |
| `clsx` | 2.1.1 | 1 |
| `cockatiel` | 3.2.1 | 1 |
| `color-convert` | 2.0.1 | 1 |
| `color-string` | 1.9.1 | 1 |
| `d3-color` | 3.1.0 | 1 |
| `d3-polygon` | 3.0.1 | 1 |
| `detect-port` | 2.1.0 | 1 |
| `enhanced-resolve` | 5.26.0 | 1 |
| `es-module-lexer` | 2.3.2 | 1 |
| `fflate` | 0.8.3 | 1 |
| `fresh` | 2.0.0 | 1 |
| `glob` | 13.0.6 | 1 |
| `iconv-lite` | 0.7.3 | 1 |
| `ieee754` | 1.2.1 | 1 |
| `jstat` | 1.9.6 | 1 |
| `lines-and-columns` | 2.0.3 | 1 |
| `lru-cache` | 11.5.3 | 1 |
| `p-limit` | 3.1.0 | 1 |
| `p-map` | 7.1.1 | 1 |
| `reveal.js` | 5.2.1 | 1 |
| `saxes` | 6.0.0 | 1 |
| `string-argv` | 0.4.0 | 1 |
| `string-width` | 7.2.0 | 1 |
| `undici` | 7.30.1 | 1 |
| `ws` | 8.22.0 | 1 |

# Vitest configs (`vitest run --config`, `--testTimeout 120000`)

plain = vitest as the package launcher would run it; hoisted = same with an alias wrapper that resolves the missing packages from the emulated hoist (harness only).

| config | plain | hoisted | cause |
|---|---|---|---|
| `✏️s/🔌️plugins/🎞️animate/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@tailwindcss/vite' imported from /Users/ueli/Documents |
| `✏️s/🔌️plugins/🏗️fem/…/config` | green 11 passed (11) | same |  |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@tailwindcss/vite' imported from /Users/ueli/Documents |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/…/config` | green 1 passed (1) | same |  |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/…/config` | red no tests | red no tests | Error: Cannot find package 'brepjs-opencascade/src/brepjs_single.wasm?url' imported from /Users/ueli/Docu |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/…/config` | red no tests | red no tests | Error: Cannot find package 'brepjs-opencascade/src/brepjs_single.wasm?url' imported from /Users/ueli/Docu |
| `✏️s/🔌️plugins/📸️remodel/…/config` | green 1373 passed (1373) | same |  |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/…/config` | red no tests | red no tests | ReferenceError: self is not defined |
| `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/…/config` | green 1 passed (1) | same |  |
| `✏️s/🔌️plugins/🧩️puzzle/…/config` | green 47 passed (47) | green 47 passed (47) |  |
| `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/…/config` | red no tests collected | red no tests collected | TypeError [ERR_INVALID_ARG_TYPE]: The "paths[0]" argument must be of type string. Received undefined |
| `✏️s/🧑‍💻dev/📐️cad/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@tailwindcss/vite' imported from /Users/ueli/Documents |
| `✏️s/🧑‍💻dev/🧩️catalog/…/config` | red 4 ⎯⎯⎯⎯⎯⎯⎯ | red 4 ⎯⎯⎯⎯⎯⎯⎯ | TypeError: Module "file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%94%A8%EF%B8%8Fmod |
| `🌎️hub/🔨️modules/🛡️admin/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@vitejs/plugin-react' imported from /Users/ueli/Docume |
| `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/…/config` | green 16 passed (16) | same |  |
| `🌎️hub/…/config` | green 70 passed | 4 skipped (74) | same |  |
| `🏢️semio-tech/🎡️play/…/config` | red 7 ⎯⎯⎯⎯⎯⎯⎯ | red 7 ⎯⎯⎯⎯⎯⎯⎯ | Error: Cannot find module 'yargs-parser' |
| `…/config` | red no tests collected | red no tests collected | ? |
| `🧰️framework/🔨️modules/⏳️async/…/config` | green 27 passed (27) | same |  |
| `🧰️framework/🔨️modules/◻️2d/…/config` | red 1 ⎯⎯⎯⎯⎯⎯⎯ | red 1 ⎯⎯⎯⎯⎯⎯⎯ | Error: browserType.launch: Executable doesn't exist at /Users/ueli/Library/Caches/ms-playwright/chromium_ |
| `🧰️framework/🔨️modules/🎠️kernel/…/config` | green 91 passed (91) | green 91 passed (91) |  |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🏷️localized-label-fixture/🎚️config/🟦️.ts` | green 4 passed (4) | same |  |
| `🧰️framework/🔨️modules/🎭️actor/…/config` | green 104 passed (104) | green 104 passed (104) |  |
| `🧰️framework/🔨️modules/📐️geometry/…/config` | green 26 passed (26) | same |  |
| `🧰️framework/🔨️modules/📡️replication/…/config` | green 21 passed (21) | green 21 passed (21) |  |
| `🧰️framework/🔨️modules/🔄️machine/…/config` | green 3 passed (3) | same |  |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/…/config` | red 2 ⎯⎯⎯⎯⎯⎯⎯ | red 2 ⎯⎯⎯⎯⎯⎯⎯ | Error: Cannot find module 'color-string' |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/…/config` | red 256 passed (256) | red 256 passed (256) | Error: Failed to resolve import "@react-three/fiber" from "../../../../🧱️elements/🔌️Ports/🟦️.tsx". Does t |
| `🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/…/config` | red no tests | red no tests | Error: Failed to resolve import "@react-three/fiber" from "../../../../🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx". D |
| `🧰️framework/🔨️modules/🧊️3d/…/config` | green 17 passed (17) | same |  |
| `🧰️framework/🔨️modules/🧬️schema/…/config` | green 29 passed (29) | same |  |
| `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@vitejs/plugin-react' imported from /Users/ueli/Docume |
| `🧰️framework/🛍️products/❓️quiz/…/config` | red 344 passed (344) | red 344 passed (344) | Error: Cannot find module 'jstat' |
| `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@vitejs/plugin-react' imported from /Users/ueli/Docume |
| `🧰️framework/🛍️products/🎤️presentation/…/config` | green 56 passed (56) | same |  |
| `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@vitejs/plugin-react' imported from /Users/ueli/Docume |
| `🧰️framework/🛍️products/🐾️pets/…/config` | green 1133 passed (1133) | same |  |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/…/config` | red no tests collected | red no tests collected | Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@tailwindcss/vite' imported from /Users/ueli/Documents |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/…/config` | red 22 passed (22) | red 22 passed (22) | Error: semio-os-mcp binary gate failed at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modul |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/…/config` | red + //! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io. | red + //! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io. | Error: browserType.launch: Executable doesn't exist at /Users/ueli/Library/Caches/ms-playwright/chromium_ |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/…/config` | red + //! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io. | red + //! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io. | Error: browserType.launch: Executable doesn't exist at /Users/ueli/Library/Caches/ms-playwright/chromium_ |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/…/config` | green 9 passed (9) | same |  |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/…/config` | red 1 ⎯⎯⎯⎯⎯⎯⎯ | red 1 ⎯⎯⎯⎯⎯⎯⎯ | Error: No test suite found in file /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️pl |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition/🎚️config/🟦️.ts` | red 5 ⎯⎯⎯⎯⎯⎯⎯ | red 5 ⎯⎯⎯⎯⎯⎯⎯ | Error: Failed to resolve import "@react-three/fiber" from "../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️ |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/…/config` | green 13 passed (13) | same |  |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/…/config` | green 7 passed (7) | same |  |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/…/config` | red 9 ⎯⎯⎯⎯⎯⎯⎯ | red 9 ⎯⎯⎯⎯⎯⎯⎯ | Error: Build failed with 1 error: |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` | red no tests collected | red no tests collected | Error: Build failed with 1 error: |
| `🧰️framework/🛍️products/💻️os/…/config` | red 749 passed (749) | red 749 passed (749) | Error: Cannot find module 'retry' |
| `🧰️framework/🛍️products/🖥️server/…/config` | green 18 passed (18) | same |  |
| `🧰️framework/…/config` | green 306 passed (306) | same |  |
