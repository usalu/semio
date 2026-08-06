# Important — UI Element Co-location

## Finish-pass verdict (2026-08-06)

Rust tui/wgpu element co-location + emoji `#[path]` resolution: **DONE**.
Per-target `Cargo.toml` must stay **absent**.

## Do not
- Recreate `🎯️targets/{⌨️tui,🧊️wgpu}/Cargo.toml`
- Undo family merge to `semio-framework-ui`
- Edit root `Cargo.toml` / root `package.json` from this ticket (registrar only)

## External blocker for cargo re-check
Root workspace member for repo CLI points at deleted Shape-V1 path.
Handoff: `📋️registrar-handoff.md`

## Remaining on master (inside ui, other slices)
- W3-interim still in ui-react barrel
- W6 activation gates + W7 residual stories
