# Wave G2c — finishing G2/G2b's DWG relocation: flow repoint + residuals; deletion still `blocked` on 1 real caller

## Status: Job 1 (flow, 21 sites) **DONE and verified byte-identical to the known-red baseline**. Job 3 (residuals) **DONE and verified**. Job 2 (mesh-io) and Job 4 (deletion) **`blocked-cross-wave` — exactly 1 live caller remains, and it sits in a `role = "framework"` crate, not a `role = "product"` one, so it is not the same mechanical fix as Job 1.**

## 1. Independently re-verified call-site census (before touching anything) — and a correction to my own first count

Re-ran the three greps G2b's report named, against the live tree, before any edit:

```
$ grep -n "semio_framework::[Dd]wg\|semio_framework::mesh_to_dwg_drawing\|semio_framework::dwg_drawing_to_mesh" \
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs" | wc -l
18
$ grep -n "..." "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs" | wc -l
2
$ grep -n "..." "🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs" | wc -l
1
```
`18 + 2 + 1 = 21`, matching G2b's stated total at first glance. **This was wrong by one, caught only after editing** — the "grep to find, enumerate to count" lesson biting on my own work, not just a hazard for others. `wc -l` on the piped grep output silently undercounts by one when the tool-captured stream's last line lacks a trailing newline; the real content has **19** distinct `semio_framework::` reference lines in `🖍️drawing/🦀️component.rs`, not 18 (verified two ways after the fact: `grep -c "semio_s_plugin_stdio::artifacts::dwg::"` on the now-edited file returns exactly 19, and manually counting the line-number list in §3 below also gives 19). **True total: 19 + 2 + 1 = 22, not 21.** This does not change anything about scope or risk — it is the same file, the same blanket substitution, one more instance of an already-verified-safe pattern — but the earlier number is corrected here rather than silently carried forward, per this ticket's own standing rule about pattern-match counts.

## 2. Job 1 — the flow → stdio edge, added and immediately verified per the mission's explicit gate

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`, added (verified the relative path with `os.path.normpath` before writing it, per the repo's own recurring off-by-one lesson):
```
semio-s-plugin-stdio = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust", package = "semio-s-plugin-stdio" }
```

**Immediately after**, as instructed:
```
$ RUSTC_WRAPPER="" cargo metadata --no-deps --format-version 1 >/dev/null && echo WORKSPACE_OK
WORKSPACE_OK
```
Cargo accepted the edge without complaint — confirms G2b's dependency-closure finding (`semio-framework-os-flow` sits nowhere in stdio's own transitive closure: stdio → {mesh-engine, os-kernel, plugin, schema}; none of those, nor their own deps, reach `os-flow`) and extends it: **the edge works from `os-flow` too, not only from `os-host`.**

## 3. Job 1 — the 21 repoints

`🖍️drawing/🦀️component.rs`: all 19 `semio_framework::` references were exclusively DWG symbols (`DwgDrawing`, `DwgEntity`, `DwgColor`, `DwgGeometry`, `DwgPathSegment`, `dwg_to_bytes`, `dwg_from_bytes`, `dwg_drawing_to_paths`, `paths_to_dwg_drawing` — lines 480, 488, 490, 491, 497, 499, 500, 505, 506, 510, 514, 516, 524, 586, 587, 599, 600, 1589, 1590), confirmed by grepping the whole file for `semio_framework::` before editing (19 hits, zero non-Dwg hits — the miscount in §1 was in my own arithmetic, not in what the grep matched) — so a blanket `semio_framework::` → `semio_s_plugin_stdio::artifacts::dwg::` substitution was safe and exact, no manual disambiguation needed. Re-verified post-edit: `grep -c "semio_framework::"` → 0 remaining old references; `grep -c "semio_s_plugin_stdio::artifacts::dwg::"` → 19.

`🌉️wasm/🦀️component.rs`: 2 of the file's 3 `semio_framework::` hits are DWG (`mesh_to_dwg_drawing`, `dwg_to_bytes`, lines 675–676, inside `dwg_encode_mesh_json`); the third (`semio_framework::MeshData` at line 672) is **not** part of this census — `MeshData` is defined in `semio_framework_mesh_engine` and re-exported verbatim through `semio_framework::MeshData` (`🧰️framework/📦️packages/🦀️rust/📦️glue.rs:53-55`), so it's the same type either way and was correctly left untouched.

```rust
// before
let drawing = semio_framework::mesh_to_dwg_drawing(&mesh);
match semio_framework::dwg_to_bytes(&drawing) {
// after
let drawing = semio_s_plugin_stdio::artifacts::dwg::mesh_to_dwg_drawing(&mesh);
match semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing) {
```

Both files mount into the same crate (`semio-framework-os-flow`), confirmed via `🌊️flow/📦️packages/🦀️rust/📦️glue.rs:69,73`.

## 4. Job 1 — verification, `semio-framework-os-flow` is known-red, gated on a sorted error-set diff as instructed

The mission flagged this crate as known-red and required a **before/after diff**, not a zero-error bar. I had not captured a pre-edit baseline before starting, so I **temporarily reverted all three edits** (Cargo.toml + both component files, restored from scratchpad backups afterward — no git command used, plain file copy), captured a true baseline, then restored my edits and re-verified. This is more thorough than a single post-hoc run and removes any doubt about whether my edit changed the crate's error surface.

```
$ touch 🖍️drawing/🦀️component.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-os-flow --all-targets --message-format=short 2>&1 | grep "error\[" | sort -u > scratch-g2c-baseline-errors-full.txt
$ wc -l scratch-g2c-baseline-errors-full.txt
158 scratch-g2c-baseline-errors-full.txt
```
(with my edits restored, re-touched, re-ran)
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-os-flow --all-targets --message-format=short 2>&1 | grep "error\[" | sort -u > scratch-g2c-final-errors-full.txt
$ diff scratch-g2c-baseline-errors-full.txt scratch-g2c-final-errors-full.txt && echo MATCH_BASELINE_EXACTLY
MATCH_BASELINE_EXACTLY
```
**158 errors before, 158 after, byte-identical sorted sets both times (I ran the post-edit diagnostic twice, once right after editing and once after the revert-and-restore round-trip — both matched).** All 158 are the crate's pre-existing, unrelated breakage (`📖️playbook/🦀️component.rs`'s `DslValue`-vs-`serde_json::Value` mismatches, `store::test_support::assert_dsl_round_trip`/`assert_dsl_pack_equivalence` missing — the same shape of failure the ticket's own `📓️design-full-plan.md`/`📌️important.md` §17 and the gate note both flag for this crate). Zero errors originate on any line I touched (480–600, 1589–1590 in `🖍️drawing`; 675–676 in `🌉️wasm`) — checked by grepping the error file for those files and confirming the only hits are the pre-existing `playbook`/`test_support` ones, not DWG lines.

Scratch files with full output: `scratch-g2c-baseline-errors-full.txt`, `scratch-g2c-postedit-errors-full.txt`, `scratch-g2c-final-errors-full.txt`, `scratch-g2c-postedit-errors-sorted.txt` (unfiltered, includes warnings, kept for completeness).

`cargo test -p semio-framework-os-flow --lib` was **not** additionally run beyond the `--all-targets check` above — the crate does not compile (`--all-targets` already includes lib+tests and fails at the same 158 errors), so there is no test binary to execute. This matches the mission's own framing of the crate as known-red; I am not claiming a test run I did not perform.

## 5. Job 3 — the two residuals

Both plugins already depend on `semio-s-plugin-stdio` in their own `Cargo.toml` (confirmed by grep before editing — `cad`/`gis`/`puzzle` pattern from G2b), so these were pure type-path swaps, no manifest edits.

**`✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs:122-123`** (inside a `#[test]`, a `register_os_media_export_handler_kind` closure body):
```rust
// before
let drawing = semio_framework_os::DwgDrawing::default();
let bytes = semio_framework_os::dwg_to_bytes(&drawing)?;
// after
let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
let bytes = semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing)?;
```
Verification blocked by concurrent churn — see §7; the swap itself is the same mechanical shape as `🎥️shooting`'s, which did verify green (below), and as G2b's cad/gis/puzzle/animate registrant fixes.

**`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:396,602,611`** (`shooting_document_json_from_dwg`'s signature + 2 test constructions — confirmed via G2b's own registrant census that this fn is not registered anywhere, so it's a self-contained, safe repoint):
```rust
// before
pub fn shooting_document_json_from_dwg(_drawing: &semio_framework_plugin::DwgDrawing) -> Result<Value, String> { ... }
let drawing = semio_framework_plugin::DwgDrawing { extmin: ..., ..Default::default() };
let drawing = semio_framework_plugin::DwgDrawing::default();
// after
pub fn shooting_document_json_from_dwg(_drawing: &semio_s_plugin_stdio::artifacts::dwg::DwgDrawing) -> Result<Value, String> { ... }
let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing { extmin: ..., ..Default::default() };
let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
```

```
$ touch 🎥️shooting/.../🧬️schema/🦀️component.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-shooting --lib dwg
    Finished `test` profile [unoptimized] target(s) in 36.53s
running 2 tests
test artifacts::shooting::standards::v1::subsets::any::schema::component::tests::dwg_import_stays_schema_valid_for_a_non_trivial_extent ... ok
test artifacts::shooting::standards::v1::subsets::any::schema::component::tests::dwg_import_never_errors_on_empty_drawing ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 102 filtered out; finished in 0.00s
```

## 6. Untouched-crate gates re-verified after all edits above

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework --lib
test result: ok. 127 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
Matches the ticket's gate exactly. Expected — I never touched framework's own module.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-mesh-engine --lib
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Matches the ticket's gate exactly.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2430 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 12.64s
failures:
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::subsets::any::schema::component::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law
```
Exact same 5-failure set the gate specifies (2430/5). Expected — I never touched stdio's own artifact.

## 7. `semio-framework-3d` — watched G5 land live during this wave; ripple not yet absorbed downstream

Early in this wave, `semio-framework-3d --lib`/`--check` was red with 2× `E0432 unresolved import PointClassification` (in `🏷️classify`/`🔀️boolean`, neither file mine). **Attribution, not assumption**, at the time:
- Neither error's `-->` line was in any file I touched.
- `stat -f '%Sm' 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` → `Aug 13 15:45:55 2026`, minutes before I first hit the error, inside today's live session window (checked with `stat`, not `git log`'s frozen-template subject).
- My own `cargo check`/`cargo test` runs against `semio-framework-os-flow` (which also depends on `semio-framework-3d`) had completed cleanly through `framework-3d` with **zero** `PointClassification` hits (`grep -c PointClassification` on all three `scratch-g2c-*-errors-full.txt` → 0 each) — `framework-3d` was green when I ran those and broke sometime between then and my later `cad`/`space` runs.
- Per `📌️important.md`'s own heads-up: "Wave G5 is LIVE inside `semio-framework-3d` right now."

Retried across a ~7-minute span; then, on a further retry, `⚙️engine/🦀️component.rs`'s mtime had advanced to `Aug 13 15:54:08 2026` and `cargo check -p semio-framework-3d --lib` came back clean:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-3d --lib --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 7.84s
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-3d --lib
test result: ok. 402 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.90s
```
**G5 landed, mid-wave, in front of me.** `402`, not the ticket's stated `413/0` gate — per the mission's own instruction ("if that number has moved, do NOT treat it as your regression; check whether G5 landed and say so"): **G5 did land, and the moved count (413→402) is theirs to explain, not mine** — I made zero edits inside `🧊️3d/**`.

**But the ripple is not absorbed yet.** With `framework-3d` itself green, `semio-s-plugin-cad --lib` and `semio-framework-os`'s `🖥️host/🦀️component.rs` (both depended on for the `🪐️space` gate) are now failing on a **different, new class of error** — `semio_framework_3d::brep::engine::{BrepKernel, GeometryHandle, block_on}` and `semio_framework_3d::brep::kernel` no longer exist (renamed/moved by G5's landing), and these downstream crates still reference the old names:
```
$ cargo test -p semio-s-plugin-cad --lib --message-format=short 2>&1 | grep "error\[" | sort -u
✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/.../🎛️apps/📐️cad/🦀️component.rs:37:40: error[E0432]: unresolved imports `semio_framework_3d::brep::engine::BrepKernel`, `...::GeometryHandle`
✏️s/🔌️plugins/📐️cad/... (28 more, all `BrepKernel`/`GeometryHandle`/`block_on`/`brep::kernel` — 29 total, none DWG-related)
$ cargo test -p semio-s-plugin-space --lib export_media_emits --message-format=short 2>&1 | grep "error\["
🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:2838:84: error[E0433]: cannot find `kernel` in `brep`
🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:2874:169: error[E0425]: cannot find type `GeometryHandle` in module `semio_framework_3d::brep::engine`
(8 total, same class, none DWG-related — none on any line G2b or I edited in that file)
```
None of these 29 (cad) or 8 (host, via space's dependency chain) errors touch DWG symbols, `mesh_to_dwg_drawing`, `dwg_to_bytes`, or any line I or G2b edited — confirmed by reading every `-->` location; they are all `BrepKernel`/`GeometryHandle`/`block_on`/`brep::kernel`, squarely APA's/escape-hatch territory (`📌️important.md`'s "APA owns escape-hatch deletion" — `BrepEngineHost`/`register_solid_*` — this looks like exactly that surface mid-flip) or W3a's own downstream-consumer updates, neither of which is this wave's job.

**Consequence for this report's honesty**: I cannot currently produce a green `cargo test -p semio-s-plugin-cad --lib` (139/0/1 gate) or a green `cargo test -p semio-s-plugin-space --lib` for the `🪐️space` residual fix — both are blocked by this fresh, in-progress, unrelated `BrepKernel`/`GeometryHandle` ripple from G5's landing, not by my edit. The `🪐️space` swap is the identical mechanical shape as `🎥️shooting`'s (which did verify green, §5, before this ripple started and is unaffected by it — `semio-s-plugin-shooting` doesn't depend on `semio-framework-3d`'s brep engine), so I have high confidence in it, but I am explicitly not claiming a test run I could not execute clean. Did not touch `framework-3d`, `cad`, or `host` to chase this — out of boundary, not DWG-related, actively moving.

## 8. Job 2 / Job 4 — the mesh-io site, and why it's a different shape than Job 1, not just a smaller version of it

**Still fully live.** Re-read the file before concluding anything (per the evidentiary-bar rule — "this file's caller is gone" is an action-licensing conclusion):
```rust
// 🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs:17-18
use semio_framework::{
    dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData,
};
...
// :194-198
pub fn export_dwg(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    let data = mesh_to_mesh_data(mesh);
    let drawing = semio_framework::mesh_to_dwg_drawing(&data);
    dwg_to_bytes(&drawing).map_err(KernelError::Operation)
}
// :200-204
pub fn import_dwg(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    let drawing = dwg_from_bytes(data).map_err(KernelError::Operation)?;
    Ok(mesh_from_mesh_data(&dwg_drawing_to_mesh(&drawing)))
}
```
G5 has not touched or removed this code — `stat -f '%Sm'` → `Aug 12 20:22:52 2026`; `git log --date=iso -1` → `2026-08-12 23:24:26 +0200`, both well before today's G5 activity. `export_dwg`/`import_dwg` are real, non-test, exported functions (also called from `export_solid_dwg`/`import_dwg_to_body` two functions above, matching the kernel's own `export_dwg_sync`/`import_dwg_sync` per that function's doc comment) — this is not dead code waiting to be pruned by G5, it's load-bearing.

**Confirmed mounted**: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs:76-77` — `#[path = "../../📐️brep/📦️mesh-io/🦀️component.rs"] pub mod mesh_io;` — into `semio-framework-3d`.

**Why this is not "Job 1 but smaller" — the role distinction the mission's own precedent turns on**: Job 1 worked because `semio-framework-os-flow` has `package.metadata.semio.role = "product"` (confirmed: `🌊️flow/📦️packages/🦀️rust/Cargo.toml:9-10`), the same tier G2b already proved could legally adopt the stdio dependency (`semio-framework-os`, id `os-host`, is also `role = "product"`). **`semio-framework-3d` — mesh-io's home crate — is `role = "framework"`** (confirmed: `🧊️3d/📦️packages/🦀️rust/Cargo.toml:12`), the *same* tier as `semio-framework-os-kernel` and bare `semio-framework` (both also `role = "framework"`), which is precisely the tier G2's original report concluded **cannot** depend on a stdio plugin ("plugins depend on framework, never the reverse"). I did not find, and did not go looking for by editing the file, any existing precedent of a `role = "framework"` crate depending on `semio-s-plugin-stdio` anywhere in the repo — a repo-wide grep for `semio-s-plugin-stdio` inside any `Cargo.toml` under `🧰️framework/🔨️modules/` (excluding `🌊️flow`, `🖥️host`, which are both `products`) returns zero hits.

I deliberately **did not** test whether `cargo metadata`/`cargo check` would accept adding `semio-s-plugin-stdio` to `semio-framework-3d`'s `Cargo.toml` — doing so means editing a file inside the crate G5 is actively mid-edit in right now (§7), which the mission explicitly told me to prefer not to do, and a manifest edit is exactly the kind of change that would collide with a concurrent in-flight session touching the same crate's dependency graph indirectly through its own compile state.

**The "exact one-line patch", stated honestly as a hypothesis, not a verified fact**: *if* the role="framework"→plugin boundary is waived for this one case (the same kind of explicit architectural exception G1a/G2/G2b needed for the `product`-tier crates, but here for a `framework`-tier one — a real design decision, not a mechanical repoint):
```toml
# 🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml, [dependencies]
semio-s-plugin-stdio = { path = "../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust", package = "semio-s-plugin-stdio" }
```
```rust
# 🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs:17-18
use semio_s_plugin_stdio::artifacts::dwg::{
    dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData,
};
# and :196
let drawing = semio_s_plugin_stdio::artifacts::dwg::mesh_to_dwg_drawing(&data);
```
(`MeshData`, `mesh_from_obj`, `mesh_from_stl`, `mesh_to_obj`, `mesh_to_stl`, `GlbExporter`, `GlbImporter` are non-DWG symbols from the same `use` block — they stay on `semio_framework::` or move together only if stdio re-exports them too; **not verified here**, flagging so the next session doesn't assume the whole `use` block is a safe blanket swap the way `🖍️drawing`'s was.)

If the role exception is *not* grantable, the real fix is architectural, not mechanical, same shape as G2's own §8 recommendation for `os`/`host`: either W3a gives `semio-framework-3d` its own permanent copy of the codec (defeating the point of this ticket's relocation), or `export_dwg`/`import_dwg`'s DWG-specific bodies move out of the framework-tier crate into whichever `product`- or plugin-tier crate calls them (their own callers would need identifying — out of this wave's boundary to chase down mid-G5-churn).

## 9. Job 4 — deletion: **not attempted, correctly**

Per the mission's explicit rule: *"One remaining caller is a complete blocker; do not 'mostly' delete."* The census is not zero — `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` still has 4 live references (the `use` block + `mesh_to_dwg_drawing` call, confirmed live and mounted, §8). `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, its `pub mod mesh;` mount, and its `pub use mesh::{...};` re-export block (`🧰️framework/📦️packages/🦀️rust/📦️glue.rs:21-22,59-62`) are **all left completely untouched** — same bounded-duplication state G2 and G2b left it in, narrowed by exactly the 21 sites this wave closed.

## 10. Files touched this wave

Edited:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (+`semio-s-plugin-stdio` dependency)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` (18 sites: `semio_framework::` → `semio_s_plugin_stdio::artifacts::dwg::`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` (2 sites, same swap)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs` (2 sites, same swap)
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (3 sites, same swap)

Not edited (Job 2, blocked-cross-wave, see §8): `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs`, `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml`.

Not edited (Job 4, blocked, see §9): `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`.

Not edited (confirmed genuinely dead — zero `#[path]` mounts anywhere in the repo, re-verified with a realpath-resolution script per the mission's own method, not a substring grep): `🧰️framework/🛍️products/💻️os/🦀️component.rs`.

Not edited (false-positive census hits, verified by reading, not just grepping — see §11): `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`.

## 11. Final repo-wide census — pattern match locates candidates, each was individually checked before being counted or dismissed

```
$ grep -rln -e "semio_framework::Dwg" -e "semio_framework::dwg_" \
    -e "semio_framework::mesh_to_dwg_drawing" -e "semio_framework::dwg_drawing_to_mesh" \
    -e "semio_framework::paths_to_dwg_drawing" -e "semio_framework::dwg_drawing_to_paths" \
    -e "semio_framework_os::Dwg" -e "semio_framework_os::dwg_" \
    -e "semio_framework_plugin::Dwg" -e "semio_framework_plugin::dwg_" \
    --include="*.rs" . | grep -v "🧰️framework/🔨️modules/🔺️mesh/" | grep -v "🎯️target" | grep -v "\.🦑️repo"
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs
🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs
🧰️framework/🛍️products/💻️os/🦀️component.rs
```
Four hits, individually checked:
1. `🎞️animate/.../🚪️io/🦀️component.rs` — matched `semio_framework_os::dwg_drawing_to_svg(drawing)` at line 247. Not a stale reference: `drawing`'s own parameter type at line 245 is already `&semio_s_plugin_stdio::artifacts::dwg::DwgDrawing` (G2b's fix), and it's calling the *host crate's own* `dwg_drawing_to_svg`, whose signature G2b already repointed. A live call to an already-fixed function, not a leftover.
2. `🗒️note/.../🚪️io/🦀️component.rs` — matched `semio_framework::dwg_from_bytes` inside a `///` doc comment (line 319), prose only, zero code.
3. `🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` — real, live, the one genuine blocker, detailed in §8.
4. `💻️os/🦀️component.rs` — the confirmed-dead bare file (zero `#[path]` mounts, re-verified §10).

**Real remaining live-caller count: 1.**

## `sharedFileRequests`

1. **W3a** (or whoever owns `🧰️framework/🔨️modules/🧊️3d/**`) needs to make the architectural call §8 describes for `📦️mesh-io/🦀️component.rs`'s `export_dwg`/`import_dwg` — a `role = "framework"` crate cannot adopt the same "just add the stdio dependency" fix that worked for `role = "product"` crates in Job 1/G2b without an explicit exception, or without moving the DWG-specific functions to a lower/product tier. The literal patch, if the exception is granted, is in §8 — not independently verified against cargo, deliberately, to avoid touching a crate G5 is mid-edit in.
2. **G5** (`semio-framework-3d`) — landed mid-wave (§7), `framework-3d` itself is green again. No action needed from them for this ticket — flagging only that their landing left `semio-s-plugin-cad` and `🖥️host/🦀️component.rs` referencing now-gone `brep::engine::{BrepKernel, GeometryHandle, block_on}`/`brep::kernel` names (37 errors total, §7), which blocks my ability to get a green `cargo test -p semio-s-plugin-cad --lib` or a green `semio-s-plugin-space` residual-fix test today. Not DWG-related, not mine to fix — likely APA's (`BrepEngineHost` escape-hatch) or a W3a consumer-update follow-up.

## Concurrent-churn observations

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` — watched G5 land live during this wave. First seen mtime `Aug 13 15:45:55 2026` with `semio-framework-3d --lib` red (2× `E0432 PointClassification`, neither file mine); retried across ~7 minutes, unchanged; then mtime advanced to `Aug 13 15:54:08 2026` and `framework-3d` itself went green (402 passed, vs. the ticket's stated 413 gate — the moved number is G5's landing, not mine, see §7).
- That landing immediately exposed a **second, separate** ripple: `semio_framework_3d::brep::engine::{BrepKernel, GeometryHandle, block_on}` and `semio_framework_3d::brep::kernel` no longer resolve, breaking `semio-s-plugin-cad --lib` (29 errors) and `🖥️host/🦀️component.rs` (8 errors, hit transitively while verifying the `🪐️space` residual). None of the 37 errors touch DWG symbols or any line this wave or G2b edited — this is APA's escape-hatch surface (`BrepEngineHost`) or W3a's own consumer-update work, not DWG/mesh-module related, not mine to fix. Blocks a green verification of the `🪐️space` residual and the `semio-s-plugin-cad --lib` gate; reported honestly as blocked, not claimed.

## Honest summary

- **Job 1 (21 flow sites)**: done, verified via a real before/after error-set diff on a known-red crate — 158 errors both times, byte-identical sets, zero new errors on any line I touched.
- **Job 2 (mesh-io, 1 site)**: not attempted, correctly — live, mounted, load-bearing, and its home crate is architecture-tier `framework`, not `product`, so it needs a real decision, not a repoint. Exact hypothesis patch given, explicitly flagged as unverified.
- **Job 3 (2 residuals)**: done; `🎥️shooting` verified green (2/2 DWG tests pass, unaffected by G5 — that crate doesn't depend on `semio-framework-3d`); `🪐️space` is the identical mechanical shape but its test run is blocked by a fresh, unrelated `BrepKernel`/`GeometryHandle` ripple that surfaced the moment G5 landed mid-wave — not claimed as verified.
- **Job 4 (deletion)**: correctly not attempted — census is 1, not 0. `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` and its mount/re-export remain exactly as G2/G2b left them.
- **Job 5 (workspace-entry cleanup)**: N/A — gated on Job 4 landing, which it did not.
