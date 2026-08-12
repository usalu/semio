# W3 — `🖨️raster` (crate `semio-s-plugin-raster`) — APA plugin migration report

Plugin dir: `✏️s/🔌️plugins/🖨️raster/`. Matches census row (`📓️w0-census.md:162`): "same 3 stubs (no AGENTS.md/README)", no extras, no purity items, no escape-hatch calls, `semio-framework-os` 5 symbols, "Wave M singles lane running", size S–M.

## What changed

### Step 1 — dead facet directories deleted
All three were the 1-line doc-only marker (`//! 🔧️ Setup facet for ... hooked via '.setup(...)'.`-style comment, no code) and **unmounted** — confirmed with `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" "✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs"` returning zero matches before deletion.

- `✏️s/🔌️plugins/🖨️raster/🛂️manifest/🦀️component.rs` — deleted (1 line: `//! 🛂️ Manifest facet for '🖨️raster' — identity surfaces live on 'Plugin::builder' in the parent.`)
- `✏️s/🔌️plugins/🖨️raster/🎟️capabilities/🦀️component.rs` — deleted (1 line: `//! 🎟️ Capabilities facet for '🖨️raster' — declare rights via 'PluginBuilder::capability' / '.local_backbone_storage()'.`)
- `✏️s/🔌️plugins/🖨️raster/🔧️setup/🦀️component.rs` — deleted (1 line: `//! 🔧️ Setup facet for '🖨️raster' — codec/language/importer registration hooked via '.setup(...)'.`)

Root `🦀️component.rs:10` already calls `.setup(crate::artifacts::raster::engine::register)` directly — registration was already artifact-owned; the deleted facet dirs carried zero logic.

No `.DS_Store` / `node_modules` junk found at plugin root (`find . -maxdepth 1 -name ".*"` → only `.` itself).

### Step 2 — plugin root closed
`ls -a "✏️s/🔌️plugins/🖨️raster/"` now shows exactly 4 entries: `🦀️component.rs`, `🎛️apps`, `🗿️artifacts`, `📦️packages`. No `AGENTS.md`/`README.md` existed before or after (census already noted their absence). Nothing else needed relocating — packet's prediction of "nearly pure deletion" held.

### Step 3 — escape-hatch call sites
`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/🖨️raster/"` → **zero matches**. Nothing to relocate.

### Step 4 — dependency purge
**Not purged — left in place, correctly.** `📦️packages/🦀️rust/Cargo.toml:29` still declares `semio-framework-os`. Grep confirms 5 genuine `semio_framework_os::` symbol uses, all inside the sanctioned artifact engine file `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`:
- `DwgDrawing` — lines 433, 543, 664, 687
- `DwgGeometry` — lines 440, 441, 442, 669
- `DwgEntity` — line 666
- `DwgColor` — line 668
- `rasterize_svg_to_png_base64` — lines 548, 646

`w0-d-sdk-surface.md:504` independently confirms the same 5-symbol set for raster. None of the 5 has an equivalent re-export via `semio_framework_plugin` today (checked: `grep -rn "DwgColor\|DwgDrawing\|DwgEntity\|DwgGeometry\|rasterize_svg_to_png_base64" "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/"` → no matches). Per Step 4's explicit instruction, the dependency stays and the re-export need is filed below rather than forced.

### Step 5 — inventory only (no edits made)
- **Interior-mutable state**: `🎛️apps/🖨️raster/🌉️wasm/🦀️component.rs:16` — `store: RefCell<RasterStore>` inside `#[wasm_bindgen] pub struct RasterArtifactVcs`. This is **per-instance** state owned by each `wasm-bindgen`-constructed VCS bridge object (one `RefCell` per JS-side instance, not a process-global `thread_local!`/`static`), the exact shape the census separated out for `🖍️draw`'s identical `store: RefCell<DrawStore>` bridge field as distinct from Draft-lane scratch. **No thread_local!/OnceLock/Mutex/Cell app-level scratch found** — the only other interior-mutable finds are two `OnceLock<Vec<ComposerEntry>>` memoization caches (`🗿️artifacts/🖨️raster/🦀️component.rs:243`, `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:757`), both inside `🗿️artifacts` (sanctioned location), both pure read-only memoized computation, not mutation state — no proposed Draft snapshot fields needed for any of these.
- **`std::fs`/`std::env`/`std::process`/`Command::new`/network outside `#[cfg(test)]`**: none found (`grep -rn "std::fs\|std::env\|std::process\|Command::new\|reqwest\|TcpStream\|UdpSocket"` → zero matches in the whole plugin dir).
- **`fn seed(`**: none found.

## Files touched
- Removed: `✏️s/🔌️plugins/🖨️raster/🛂️manifest/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/🖨️raster/🎟️capabilities/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/🖨️raster/🔧️setup/🦀️component.rs` (dir removed)
- No files created, no files updated.

## Verification commands (real output pasted)

### Step 0 baseline
```
$ cd "/Users/ueli/Documents/semio" && CARGO_TARGET_DIR="…/🎯️target" cargo check -p semio-s-plugin-raster 2>&1 | tail -20
```
FILL_IN_BASELINE

### Step 6.1 — post-change check
```
FILL_IN_POSTCHECK
```

### Step 6.2 — tests
```
FILL_IN_TEST
```

### Step 6.3 — nx target
`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📋️project.json` confirms `bun nx run @semio-tech/raster-plugin:test-quick` is a real target (`name: "@semio-tech/raster-plugin"`, target `test-quick` → `bun ./📜️script.ts test quick`). FILL_IN_NX

### Step 6.4 — root closure
```
$ ls -a "✏️s/🔌️plugins/🖨️raster/"
.
..
🎛️apps
📦️packages
🗿️artifacts
🦀️component.rs
```
Exactly the 4 allowed entries present (no `AGENTS.md`/`README.md` existed before this wave either).

## sharedFileRequests
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/` (the `semio_framework_plugin` SDK surface). **Region**: wherever the curated re-export list lives (M3 per `📓️w0-census.md:187`). **Reason**: raster needs `DwgColor`, `DwgDrawing`, `DwgEntity`, `DwgGeometry`, `rasterize_svg_to_png_base64` re-exported so `semio-framework-os` can eventually be dropped from `📦️packages/🦀️rust/Cargo.toml:29`. Already tracked in `📓️w0-d-sdk-surface.md:504,509` as part of the 50-symbol union — no new patch filed, this just confirms raster's 5 are still needed and unaddressed as of this wave. **Patch file**: none (M3 is explicitly out of scope for W3 per the task's Step 4 instruction).

## Concurrent-churn observations
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🦀️component.rs` had a **staged (index) modification present before this session touched anything** (`git status --porcelain` showed `M ` against it at the start of this wave; `git diff --cached` shows a wire-format test-baseline rebase — dropping two leading mutation rows shifts binary ordinals — with a comment explicitly attributing it to `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`). File mtime `Aug 12 15:35:05 2026`, predating this session's start. Not touched by this wave; flagged only for visibility since it lives inside this plugin's `🎛️apps` tree. Left entirely alone.
- Heavy concurrent build load observed via `ps aux`: simultaneous `cargo check -p` invocations in flight for `semio-s-plugin-remodel`, `sequence`, `puzzle`, `note`, `energy`, `dag`, `space`, `sourcing`, `playbook`, `mathematical`, `trinity` at the time of this wave's baseline check — consistent with "the lock serializes concurrent agents" guidance. Baseline `cargo check -p semio-s-plugin-raster` queued behind the shared build lock for several minutes before producing output.

## apa-status: FILL_IN
