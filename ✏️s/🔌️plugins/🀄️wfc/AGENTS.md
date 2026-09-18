---
emoji: 🀄
---

# WFC

WFC is the wave function collapse owner: five artifacts that author a tile vocabulary plus its
adjacency law and then let the shared solver (`⚙️engine`, crate `semio-s-plugin-wfc-engine`) collapse
it.

- `🗿️artifacts/🖼️bitmap` — overlapping model over a sample bitmap (`s.wfc.bitmap`).
- `🗿️artifacts/🔲️grid2d` — tiled 2D grid with explicit adjacency rules (`s.wfc.grid2d`).
- `🗿️artifacts/◻️2d` — 2D slot graph (`s.wfc.wfc2d`).
- `🗿️artifacts/🧱️grid3d` — tiled 3D grid with non-uniform cell sizes (`s.wfc.grid3d`).
- `🗿️artifacts/🧊️3d` — 3D slot graph (`s.wfc.wfc3d`).

## 🎮️ Play harness

Each artifact's editor is its own playground variant (`bitmap`, `grid2d`, `wfc2d`, `grid3d`,
`wfc3d`); the ports are declared in `📦️packages/🦀️rust/Cargo.toml` and nowhere else.
