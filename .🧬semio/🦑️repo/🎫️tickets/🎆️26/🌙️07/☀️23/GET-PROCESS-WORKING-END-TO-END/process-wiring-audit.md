# 🔌️ Process Plugin Wiring Audit

Read-only audit of `✏️s/🔌️plugins/🏭️process` (base below as `P/`) and the hub and dev-variant wiring around it. No source file was edited and no git command was run.

Base paths: `P` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process`, `R` = `/Users/ueli/Documents/semio`.

## Method

- Scanned every `📋️project.json`, `Cargo.toml`, `🔣️.json` and `package.json` under `P` for path-like values (`./`, `../`, `{workspaceRoot}/`, and repo-relative prefixes) and checked existence.
- Parsed every `Cargo.toml` under `P` section-aware: 70 `path =` dependencies, `[lib]`/`[[bin]]` targets, `package.workspace`, and `[workspace] members` globs.
- Checked 49 relative TypeScript imports under `P`.
- Cross-checked the oracle file against the mutation directories on disk.
- Traced the dev route from `.claude/launch.json` through the root `📜️script.ts`, the `dev` command and the playground serving code.

## Verified OK

- All 70 Cargo `path` dependencies resolve to a `Cargo.toml` whose `[package] name` matches the dependency key or `package =` value. Exception: none. No duplicate package names.
- All `[workspace] members` globs resolve (`🗿️artifacts/🧊️process3d/Cargo.toml`, the four `🧩️extensions/*/Cargo.toml`, `🧩️extensions/🧪️tests/🔬️catalog-json/...`).
- `[lib] path` / `[[bin]] path` targets exist (`🦀️.rs` files in every crate).
- All 49 relative TS imports under `P` resolve.
- Dev route config `P/🧪️tests/🧭️dev-route/🔣️.json` resolves on every field:
  - `manifest` = `🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml` (exists).
  - `ownerRoot` = `✏️s/🧑‍💻dev/🎭️variants/🏭️process` (exists), with `🧬️schema/🔣️.json`, `🚀️entry/🟦️.ts` and `🏗️builder/🌐️vite/🟦️.ts` present.
  - `variant` `process3d`, `app` `s.process.process3d@1/*#editor` and `ports` react 6022 / wgpu 6122 match the hub row at `🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml:21-27`.
  - `extensions` lists 4 plugins, all present: `🧱️concrete`, `🔩️metal`, `🤖️robotic`, `🪵️wood`. Each has `process.machines` in its `🦀️.rs` and a `semio-s-plugin-process-*-component` crate at `🧩️component/📦️packages/🦀️rust/Cargo.toml`.
- Hub composition `🌎️hub/🧩️compositions/🏭️process`: `compositionConfigPath` `✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts` exists; `devContribution` exists.
- Symbols and subpaths imported by the dev route and dev variant exist and are exported: `bootFrameworkOsDev`, `composeSpecificOsCatalogV1`, `resolvePlaygroundBoot`, `@semio-tech/plugin-registry/catalog-view`, `@semio-tech/plugin-registry/playground/session`, `@semio-tech/process/artifacts/native-build/published`.
- `@semio-tech/s-dev-composition:generate` (dependsOn in `P/📦️packages/🟦️typescript/📋️project.json:9`) is defined by `✏️s/🧑‍💻dev/📋️project.json:2` and has a `generate` target at line 225.
- `process-js` workspace dependencies (`@semio-tech/framework`, `framework-3d-js`, `ui-react`, `ui-styling`, `infinite-world-r3f`) are all defined by a `package.json`.
- Oracle mutation directories exist for 15 of 16 entries (exception: `rename-machine`, F2). Scenario directories exist for 14 of 16 (exceptions: `rename-machine` via F2, `resize-stock` via F3).

## Findings (broken or inconsistent)

Severity: HIGH = breaks a consumer, MED = dangling reference that a consumer would hit, LOW = hygiene or prose.

### F1 (HIGH). Oracle `payloadSchema` points at a file that exists nowhere

- `P/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json` lines 270, 291, 312, 333, 353, 373, 393, 413, 433, 454, 475, 496, 517, 538, 559, 579 all say `"payloadSchema": "🧬️.schema.json"`.
- No file named `🧬️.schema.json` exists in the repository. Each mutation actually has `🧬️schema/🔣️.json` (verified for all 16 directories under `.../✳️any/🧬️schema/🧬️mutations/`).
- The oracle rationale (line 43 `specificationSource` and line 18 `rationale`) also says "each mutation's own 🧬️.schema.json", so the schema name and the on-disk name disagree.

### F2 (HIGH). Oracle `rename-machine` points at a non-existent mutation directory

- Oracle line 163: `"mutationDirectoryName": "✏️🏭️rename-machine"`. Not a directory.
- Real directory: `.../✳️any/🧬️schema/🧬️mutations/🔖rename-machine/` (with its `🧪️tests/🏷️retitles-the-saw/🦀️.rs`).
- The same entry's `sourceMutationDirectoryName` at line 162 is correct (`🔖rename-machine`), so the two fields disagree inside one entry. The `✏️` prefix looks like a stray edit.

### F3 (MED). Oracle scenario `keeps-width` has no directory

- Oracle line 233: `"directoryName": "🟰️keeps-width"` under mutation `resize-stock` (`📏resize-stock`).
- `.../🧬️schema/🧬️mutations/📏resize-stock/` has no `🧪️tests/` directory at all (it contains only `🔣️.json`, `🦀️.rs` and `🧬️schema/`).

### F4 (MED). Generated catalog `directoryName` for the four extensions does not match the source folders

- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🗂️sources.json` lines 608, 622, 636, 650: `🏙️process-extension-concrete`, `🔩️process-extension-metal`, `🤖️process-extension-robotic`, `🪓️process-extension-wood`.
- Source folders are `P/🧩️extensions/🧱️concrete`, `🔩️metal`, `🤖️robotic`, `🪵️wood`. The wood entry uses `🪓` where the folder uses `🪵`, which looks like an unpropagated rename.
- The same names appear in `🤖️generated/🔌️plugins.json`. The generated folder is stale or hand-edited relative to the source tree. I did not trace how `directoryName` is consumed, so the runtime impact is unverified. Regenerating with `repo ./📜️script.ts generate` should resolve it; I did not run it.
- The process hub entry (`sources.json:595`, `🏭️process`) matches the real folder.

### F5 (LOW). Extension `nativeSources` glob matches nothing

- `P/🧩️extensions/{🧱️concrete,🔩️metal,🤖️robotic,🪵️wood}/📦️packages/🦀️rust/📋️project.json` line 11: `.../🧩️component/📦️packages/🦀️rust/**/*`.
- That directory contains only `Cargo.toml`, so the input set is empty. Harmless for correctness, but it is a dead input.

### F6 (LOW). Oracle rationale path is wrong

- `P/.../✳️any/🔮️oracles/🔣️.json` line 18 (prose): `../../../🧪️tests/🏭️mutate-process3d-1/🐍️.py`. From `🔮️oracles/` this does not exist. The file is at `../🧪️tests/🏭️mutate-process3d-1/🐍️.py`.

### F7 (LOW). Undeclared dependencies of the dev-route test

- `P/🧪️tests/🧭️dev-route/🟦️.ts` imports `smol-toml` (line 4) and `@semio-tech/plugin-registry` (lines 6-7), but `P/📦️packages/🟦️typescript/package.json` declares neither.
- `smol-toml` is only in the root `R/package.json:298`. Resolution works only through hoisting.

### F8 (INFO). Dev-route test covers only react, and hardcodes the port

- `P/🧪️tests/🧭️dev-route/🟦️.ts:40` hardcodes `"http://localhost:6022/"`, and no assertion touches the wgpu port 6122, even though the fixture declares it.

### F9 (INFO). No launch entry for the wgpu dev route

- `R/.claude/launch.json` has `process3d-react` (lines 27-35, port 6022) but no wgpu counterpart for port 6122.

### F10 (INFO). Mutation id and directory differ on one oracle entry

- Oracle `replace-machine-capabilities` uses directory `🔁replace-machine` (oracle line near `mutationId` `replace-machine-capabilities`). Probably intentional (directory kept from an older name), but it is not documented.

## Dev route launch

From the repository root:

```
repo ./📜️script.ts dev playground process3d react prototype
```

Same as the `process3d-react` entry in `R/.claude/launch.json` (runtimeExecutable `repo`, port 6022). `repo` resolves to `/Users/ueli/.local/bin/repo` on this machine.

Dispatch path:

1. `R/📜️script.ts` routes `dev` to `DevScript` (`R/🧰️framework/.../📇️inventory/🎛️commands/🛠️dev/🟦️.ts`, re-exporting `R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`).
2. `DevScript.run` sends `segments[0] === "playground"` to `serveRootPlaygroundV1` (`.../🛠️dev/🎮️playground/🟦️.ts:258`), after `rootDevPlaygroundRequestV1` (`.../🛠️dev/🌱️birth/🟦️.ts:39`) validates `dev playground <variant> <react|wgpu|native> <prototype|release>`.
3. For `react`, `serveRootPlaygroundV1` runs `ensureBrowserSupport`, `ensureBootComponents`, `ActivationScript` (if needed) and `ServeScript` from `R/🧰️framework/.../🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`. The react port comes from the playground row (`🎠️playgrounds.json` and the hub `Cargo.toml:27`). I did not trace the exact line where the react port is applied.
4. For `wgpu`, the command is `repo ./📜️script.ts dev playground process3d wgpu prototype`. It goes through `serveWgpuPlayground` and passes `--port playground.ports.wgpu` (6122) at `.../🎮️playground/🟦️.ts:185`.

Not run: I did not start the server or the tests, since this audit is read-only.

## Ticket housekeeping

Temporary scripts (`check_paths.py`, `check_paths2.py`, `missing.txt`) were written under this ticket's `🗑️generated/` folder and are removed after this report was written.
