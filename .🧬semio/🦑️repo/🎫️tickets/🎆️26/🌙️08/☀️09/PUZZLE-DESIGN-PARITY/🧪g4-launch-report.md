# G4 Launch / Package Wiring

## package.json
- `dev:puzzle:5d:capsule-dream` → `bun ./📜️script.ts dev 5d`
- `build:puzzle:5d:capsule-dream` → `bun nx run @semio-tech/framework-os-dev:build -- puzzle5d`

## .vscode/launch.json
- `🛠️dev…5d🎛️capsule🌙️dream⚛️react` (port 6015, locks `capsule-dream`)
- `🛠️dev…5d🎛️capsule🌙️dream️wgpu🌐️wasm` (port 6115, locks `capsule-dream`)
- `📦️build…5d🎛️capsule`

Env locks: `PLAYGROUND_LOCKED_EXAMPLE_ID` + `SEMIO_DEFAULT_EXAMPLE` = `capsule-dream`.


## Seed persistence
Entries live in `.vscode/🎩️launch.seed.jsonc` (regenerated into `launch.json` via `@semio-tech/plugin-registry:generate`).
