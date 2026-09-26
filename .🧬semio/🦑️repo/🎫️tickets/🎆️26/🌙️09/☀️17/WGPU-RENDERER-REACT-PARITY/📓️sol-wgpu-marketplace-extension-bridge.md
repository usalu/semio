# WGPU Marketplace Dynamic Extension Bridge

## Boundary

The extension Store remains the only owner of installed-package inventory. Its canonical record is schema-backed and contains exactly `extensionId`, `directoryName`, `version`, `label`, `extends`, `moduleUrl`, `packageHash`, and `installedAt`. Install responses and list responses now return that same full record; uninstall accepts only the public extension identity.

The WGPU page I/O door exposes four bounded operations: list, install from URL, install from file, and uninstall. The file path transfers package bytes directly from the page picker to the Store endpoint. Rust never receives package bytes. URL admission accepts an authored URL or asks through the page's localized prompt; prompt and file cancellation settle as `{cancelled:true}` and do not call the Store.

The Rust Shell keeps a bounded 256-entry projection of Store records with local `enabled`, `load_status`, and admitted program state. Failed packages and packages whose declared host is missing remain visible. The Marketplace surface mirrors React's URL/file install rows, host nesting, missing-host sections, enable/disable action, and uninstall action in English and German.

Dynamic extensions use their own `semioWgpuInstallExtension(recordJson)` and `semioWgpuRetireExtension(extensionId)` doors in both the frame Worker and direct-page boot. They do not overload the static plugin-id loader. Admission validates the Store record before loading, then validates the loaded manifest's plugin id and version against that record. A rejected manifest retires the actor it just mounted.

Disable removes the extension program before contributions are republished. Uninstall removes contributions, retires the JavaScript actor, asks the Store to delete the package, and removes the projection only after Store acceptance. If actor retirement or Store deletion refuses, the Shell re-admits the Store record and republishes its contributions, preserving the visible projection and enabled state.

## Schema and neutral fixture

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json`

The Store suite validates the fixture with Ajv 2020. The TypeScript dynamic-loader laws and the Rust Marketplace projection laws consume the same fixture.

## Principal implementation files

- Store authority: `🔌️plugin/🏪️store/📥️installation/🟦️.ts`
- Page boundary: `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts`
- Shared dynamic admission: `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️dynamic-extension/🟦️.ts`
- Worker and direct-page installs: `🎞️frame-worker/🟦️.ts`, `🎬️renderer-boot/🟦️.ts`
- Rust bridge: `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- Shell ledger, actions, and surface: `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`

The new browser module is registered in the repository source taxonomy and in the WGPU TypeScript project's browser inputs. The final frame-worker generator passed in 1m40s and the final browser-boot generator passed in 2m51s after the retirement door and browser source registration were complete.

## Validation

The Store law suite passed:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/plugin-extension-store:test
Test Files  1 passed (1)
Tests       8 passed (8)
Duration    9.37s
```

The focused dynamic installation and retirement suite passed after the final bridge changes:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test -- --run /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️wasm-plugin-install/🟦️.ts
Test Files  1 passed (1)
Tests       11 passed (11)
Duration    4.95s
```

The WGPU page-door bundle ran 171 laws. All twelve non-Chromium files passed, including the five extension Store door laws. Three unrelated laws in `wgpu-worker-step-budget` could not start because the local Playwright Chromium executable is absent:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker -- 🧪️tests/🧩️wgpu-extension-store-door/🟦️.ts
Test Files  12 passed, 1 failed
Tests       168 passed, 3 failed
Failure     Playwright chromium_headless_shell-1234 executable does not exist
```

A later attempt to exclude the three Chromium laws was inconclusive because the WGPU task expanded the configured suite and its 15-second test-process budget killed the combined command. It reported no extension-door assertion failure before termination.

Root's live React runtime oracle opened Marketplace at port 6013 with no warning or error logs. It showed disabled Store records grouped beneath their declared hosts, Enable and Uninstall actions, the loaded Puzzle 0.1.0 plugin with Reload, development/extension/hub source groups, and both file and URL install rows. The oracle deliberately did not mutate the pre-existing Store inventory.

`git diff --check` passed. The focused native Shell Marketplace laws remain assigned to the root-owned Cargo session.
