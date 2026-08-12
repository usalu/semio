# W4b — Daemon / Workflow Wiring

## Done
- `📜️script.ts`: `daemon` and `workflow` BundleScript commands forwarding to `semio`.
- `📋️project.json`: `daemon` and `workflow` nx targets with `forwardAllArgs`.
- `.vscode/🧩️launch.seed.jsonc`: dashboard daemon start/attach + workflow entries in `3_dev`.
- Regenerated `.vscode/launch.json`.
- Rust dispatch stub for `semio workflow` (W5 fills implementation).

## Verify
Launch seed names present in `launch.json`:
- `🛠️dev🎛️dashboard🌀daemon▶️start`
- `🛠️dev🎛️dashboard🌀daemon📎attach`
- `🛠️dev🎛️dashboard🌊️workflow`
