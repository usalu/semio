# Launch entries for procedural 3d / 2d (2026-09-09)

Scope: `.vscode/🧩️launch.seed.jsonc` (source of truth), `.vscode/launch.json` (regenerated, never
hand-edited), `.claude/launch.json` (hand-maintained Browser-pane config). No cargo, no dev server, no
browser run, no git-modifying command.

## 0. What was already correct (checked before editing)

`🖥️launch.ts` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts:190-247`)
**synthesizes the whole ten-row Nx-target set for every playground variant in the registry** — it is not
per-variant seed text. So `generation3d` and `generation2d` already carried the exact same complete set
as `puzzle3d`, and no rows had to be hand-added:

```
🎮️generate🧩️<variant> session
🎮️prepare🧩️<variant>⚛️react dev | release
🎮️build🧩️<variant>⚛️react release
🎮️activate🧩️<variant>⚛️react dev | release
🎮️serve🧩️<variant>⚛️react dev | release
🎮️dev🧩️<variant>⚛️react dev | release
```

Verified after regeneration: all ten exist for `generation3d`, all ten for `generation2d`, all ten for
`puzzle3d` (see §4). The backing Nx targets are inferred for every variant by
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:785-799`
(`{serve,dev}-<variant>-react-<profile>`, `activate-…`, `prepare-…`, `build-<variant>-react-release`).

The per-variant `@generated:<variant>:{react,wgpu}` placeholders in the seed's `configurations` array are
filled from the seed's own `devLaunchers` table plus registry ports; the `🖥️wgpu native` row is
hand-authored seed text. Both existed for generation2d/3d already.

## 1. Seed changes — `.vscode/🧩️launch.seed.jsonc`

### 1.1 Dead port env replaced with the live one (`devLaunchers`)

`PROCEDURAL_2D_PLAY_PORT` / `PROCEDURAL_3D_PLAY_PORT` have **zero read sites repo-wide** (boot-path audit
§1). The live variable is `S_OS_PORT`, consumed by `frameworkOsPlaygroundDevEnv`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2617-2627`) and it is
also what the generator's own zero-touch path emits (`🖥️launch.ts:116-127`).

- `devLaunchers.generation2d.reactEnv` / `.wgpuEnv`: `PROCEDURAL_2D_PLAY_PORT` → `S_OS_PORT`
- `devLaunchers.generation3d.reactEnv` / `.wgpuEnv`: `PROCEDURAL_3D_PLAY_PORT` → `S_OS_PORT`

`SEMIO_RENDERER` was already correct (`react` / `wgpu`) on all four, and `namePrefix`/`command` were left
untouched so `interactivityAllAppLaunchCoverageFailures` (`📜️script.ts:8433-8466`) keeps matching.

The `🛠️dev🔧️procedural🏙️3d⚛️react` row keeps `bun nx run workspace:dev -- procedural 3d`: the react branch
is live post wave-R (boot-path audit §2 traces alias `"procedural 3d"` → variant `generation3d` →
`ServeScript` → `activate-generation3d-react-dev`), so only the env was wrong.

### 1.2 Dead `fixture hexagonal-column` rows converted to the env-var mechanism

`fixture <name>` is not parsed anywhere (`"fixture"` has zero hits in root `📜️script.ts` and in os-dev's
`📜️script.ts`); the value `hexagonal-column` does not even match a real example id (the folder is
`🍄️hexagonal-mushroom-column`). Example selection is `VITE_SEMIO_DEFAULT_EXAMPLE` (soft default, in-app
switcher stays) / `VITE_SEMIO_LOCKED_EXAMPLE` (hard lock), read at
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:32,41`. A launch entry sets the **process** name
`SEMIO_DEFAULT_EXAMPLE`, which `frameworkOsLockedPrefsEnv`
(`…📚️library/🎮️playground/🔒️preferences/🟦️.ts:11-17` + `🔣️.json`) projects into Vite's public env as
`VITE_SEMIO_DEFAULT_EXAMPLE`. Same convention the existing `🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream…` rows use.

- renamed + converted: `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column⚛️react`
  → `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react`,
  command `bun nx run workspace:dev -- procedural 3d`,
  env `{S_OS_PORT: 6018, SEMIO_RENDERER: react, SEMIO_DEFAULT_EXAMPLE: hexagonal-mushroom-column}`
- renamed + converted: `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🌐️wasm`
  → `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm`,
  env `{S_OS_PORT: 6118, SEMIO_RENDERER: wgpu, SEMIO_DEFAULT_EXAMPLE: hexagonal-mushroom-column}`
- **removed**: `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🖥️native` — it carried no env at all and was
  a byte-identical duplicate of `🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native`; there is no env-var pin for the
  native renderer, so it could not be converted. The plain native row (which the app-launch-coverage
  gate looks for) is untouched.

Naming follows the seed's existing example-pin convention `🎛️<word>` + example emoji + `<word>` (cf.
`🎛️concrete🌲️forest`, `🎛️capsule🌙️dream`), with the artifact's own example emoji `🍄️`.

### 1.3 `4_build` row made symmetric with generation2d

- `📦️build🔧️procedural🏙️3d🎛️column` (`…framework-os-dev:build -- generation3d fixture hexagonal-column`,
  same dead segment) → `📦️build🔧️procedural🏙️3d`
  (`bun nx run @semio-tech/framework-os-dev:build-generation3d-react-release`), mirroring the existing
  `📦️build🔧️procedural🩻️2d` → `build-generation2d-react-release`.

### 1.4 Peer row rescued into the seed (not a procedural change)

A concurrent session had hand-added `📊️jack-results-window-transient` to the **generated**
`.vscode/launch.json` (present in the git index, absent from HEAD, absent from the seed). The first
regeneration dropped it. Rather than revert a peer's registration, the row was added to the seed next to
`🖱️wires-pointer-move` (group `4_gate`, order `411.1375`); its Nx target
`workspace:jack-results-window-transient` exists (`📋️project.json:1520-1524`). After the second
regeneration the row is back in `.vscode/launch.json` and is now regeneration-stable.

The four `🧪️{map,terrain}-document-contract[-native]` rows appearing in the `.vscode/launch.json` diff are
another peer's *seed* edit propagating through regeneration for the first time — not authored here, and
deliberately preserved.

## 2. `.claude/launch.json` changes

The Browser-pane config is hand-maintained (no generator references it anywhere in the repo).

- `procedural3d-react` **fixed**: was `bun ./📜️script.ts dev procedural 3d` on `port: 6019` with the dead
  `PROCEDURAL_3D_PLAY_PORT=6019` and **no `SEMIO_RENDERER`** — since `frameworkOsPlaygroundDevEnv` defaults
  the renderer to `wgpu`, it actually bound wgpu on 6118 and the port match could never fire. Now
  `bun nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev`, `port: 6018`,
  env `{SEMIO_RENDERER: react, S_OS_PORT: 6018}`. `serve-…` rather than `dev-…` because the `dev-` target
  needs the Nx daemon watcher (puzzle3d master plan, 2026-09-09 note).
- `procedural3d-wgpu` **added**: `bun nx run @semio-tech/framework-os-dev:dev -- generation3d`,
  `port: 6118`, env `{SEMIO_RENDERER: wgpu, S_OS_PORT: 6118}` — mirrors `puzzle3d-wgpu` exactly.
- `procedural3d-react-attach` **added**: attach-only (`url: http://localhost:6018`, `port: 6018`, no
  command), mirroring `puzzle3d-react-attach`, because the Browser pane refuses `preview_start {url}` for
  a server it did not start.

Open the booted react app at `http://127.0.0.1:6018/?plugin=generation3d` (bare root boots the full `s`
studio host instead).

## 3. Regeneration

```bash
cd /Users/ueli/Documents/semio
bun nx run @semio-tech/plugin-registry:generate
```

(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json` target `generate` →
`bun ./📜️script.ts generate`, cwd the registry package; freshness gate is
`bun nx run @semio-tech/plugin-registry:check`.) Ran twice — once after §1.1-1.3, once after §1.4.
Output: `59 plugin crates, 60 playgrounds, 47 framework packages` → `.vscode/launch.json regenerated`.

## 4. Verification

`git diff --stat .vscode/ .claude/` after the final regeneration:

```
 .claude/launch.json         | 33 ++++++++++++++++----
 .vscode/launch.json         | 65 +++++++++++++++++++++++---------------
 .vscode/🧩️launch.seed.jsonc | 76 ++++++++++++++++++++++++++++++---------------
 3 files changed, 118 insertions(+), 56 deletions(-)
```

Every `"name"` line in the `.vscode/launch.json` diff (nothing else changed names):

```
+ 🧪️map-document-contract                                    (peer seed edit, preserved)
+ 🧪️map-document-contract-native                             (peer seed edit, preserved)
+ 🧪️terrain-document-contract                                (peer seed edit, preserved)
+ 🧪️terrain-document-contract-native                         (peer seed edit, preserved)
- 🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column⚛️react
+ 🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react
- 🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🌐️wasm
+ 🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm
- 🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🖥️native
- 📦️build🔧️procedural🏙️3d🎛️column
+ 📦️build🔧️procedural🏙️3d
```

The remaining hunks are env-only (`PROCEDURAL_{2,3}D_PLAY_PORT` → `S_OS_PORT` on the four generated
procedural rows, plus the added `SEMIO_DEFAULT_EXAMPLE` on the two converted rows) and the one command
change on `📦️build🔧️procedural🏙️3d`. No peer hunk was reverted.

Parsed both generated files with `Bun.JSONC.parse` / `JSON.parse`: valid, **2445 configurations, 2445
unique names** (no collision introduced). Row inventory confirmed identical in shape across variants:

| variant | ten `🎮️` Nx rows | `🛠️dev…⚛️react` | `…🧊️wgpu🌐️wasm` | `…🧊️wgpu🖥️native` |
|---|---|---|---|---|
| `generation3d` | ✅ | ✅ | ✅ | ✅ |
| `generation2d` | ✅ | ✅ | ✅ | ✅ |
| `puzzle3d` (reference) | ✅ | ✅ | ✅ | ✅ |

Not run (out of the "no cargo / no dev server / no browser" scope of this task): the
`bun ./📜️script.ts verify interactivity apps` coverage gate and any actual boot. The gate's expectations
were read (`📜️script.ts:8412-8466`) and the edits deliberately leave `namePrefix`, `command`, `cwd` and
`SEMIO_RENDERER` of the three gate-checked rows per variant unchanged.

## 5. Launch names added / changed

**`.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json`**

| action | name |
|---|---|
| renamed + rewired | `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column⚛️react` → `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react` |
| renamed + rewired | `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🌐️wasm` → `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm` |
| removed | `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column🧊️wgpu🖥️native` |
| renamed + rewired | `📦️build🔧️procedural🏙️3d🎛️column` → `📦️build🔧️procedural🏙️3d` |
| env fixed (name unchanged) | `🛠️dev🔧️procedural🏙️3d⚛️react`, `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm`, `🛠️dev🔧️procedural🩻️2d⚛️react`, `🛠️dev🔧️procedural🩻️2d🧊️wgpu🌐️wasm` |
| added to seed (peer row, keeps regeneration stable) | `📊️jack-results-window-transient` |

**`.claude/launch.json`**

| action | name |
|---|---|
| fixed | `procedural3d-react` |
| added | `procedural3d-wgpu` |
| added | `procedural3d-react-attach` |

## Files touched

- `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc`
- `/Users/ueli/Documents/semio/.vscode/launch.json` (regenerated only)
- `/Users/ueli/Documents/semio/.claude/launch.json`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️launch-entries-2026-09-09.md`
