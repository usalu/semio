# E2E Apps Status

## Fixed
- `parseShellRoute` imported in ShellHost (was ReferenceError crash)
- ShellHelpers exports for symbols ShellHost needs
- Playground catalog path: `plugin/⚡️implementations` → `plugin/📦️packages`
- Backbone worker path + `./📦️index` → `./🟦️component`
- DevScript no longer falls through unknown apps to compose-desktop
- `runViteBunxDev`/`spawnBunx` now keep the process alive (Vite no longer dies immediately)
- BREP kernel mangled dispose/retain/registry_len + Vec3/EVec3 impl mismatch
- Mindmap extension wrong `infinite_board_normal_directed` → kernel alias
- Puzzle glue misplaced `//!` docs; puzzle2d PlayApp pure-handle reconstruction

## Serving (SKIP_PLUGIN_BUILD=1, react)
- cad http://127.0.0.1:6020/
- animate http://127.0.0.1:6051/

## Still blocked for full wasm plugin rebuild
- Puzzle and many plugins still need cargo/wasm green (CLT env: `DEVELOPER_DIR=/Library/Developer/CommandLineTools`)
- Widespread `self.`→ephemeral RefCell corruption in puzzle3d/5d app hosts needs same pure-handle treatment
- Xcode license blocks default `cc` without CLT DEVELOPER_DIR

## Ticket
Continue under `26/08/06/S-AND-PLUGINS-END-TO-END` / goal Running Sketchpad.
