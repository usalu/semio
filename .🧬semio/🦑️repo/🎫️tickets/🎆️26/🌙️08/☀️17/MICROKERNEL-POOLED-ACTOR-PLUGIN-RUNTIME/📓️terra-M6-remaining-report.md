# 📓️ terra M6-remaining report

Packet: **M6-remaining** — tail of W3 (14 named plugins) + repair of `📏️layout`. Draft, extended
in place as acceptance runs complete; see `📌️important.md`, `📓️design-abi.md` §6,
`📓️terra-M0-stdio-report.md`, `📓️terra-M1-small-plugins-report.md` for context.

## Part A — `📏️layout` DWG repair: ✅ COMPLETE, verified

Root cause confirmed by reading the real `DwgSnapshot` at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:3891`:
the struct no longer carries `bytes`/`section_names`/`sections`/`decode_status`, and
`DwgDecodeStatus` no longer exists anywhere in the tree (confirmed by repo-wide grep — the only
hit was the broken file itself). Same class of stale drift E2 already fixed inside `🗒️note`.

**Fixed (3 real bugs found, all inside `📏️layout`'s own owned tree):**
1. `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` (the file named in
   the packet) — old body constructed the removed sentinel fields directly. `DwgSnapshot` has no
   raw-byte field left, so I routed through the same honest path `🗒️note`'s sibling serializer
   uses: `print_dsl` → this leaf's own DSL text is SVG (confirmed against the sibling `🎨️svg`
   serializer in the same directory) → `semio_framework_os::svg_to_dwg_bytes` → `decode_dwg`. A
   genuine (if minimal) R2004+ decode, not a fabricated status.
2. `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` — same stale-drift
   bug, mirrored on the import side (`&from.bytes`). Fixed by re-materializing real DWG bytes via
   `encode_dwg(from)`, then reusing the existing (unbroken) `deserialize_bytes` path unchanged.
3. `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs` — unrelated stale
   field: `SvgSnapshot` no longer has `lexical` (verified against the real struct — only `schema`+
   `doc` remain). Dropped the field from the struct literal.
4. `✏️editor/🌉️wasm/🦀️component.rs` `render_frame`/`hit_test` — pre-existing (not part of the named
   bug, found only once `--lib` was green and `--target wasm32-wasip2` was tried) `E0502` ×4:
   `SceneQuery` held several `&inner.*` borrows alongside `&mut inner.layout_engine`/
   `&inner.document_json` in the same call; `RefMut::deref_mut`/`deref` can't be split by rustc
   across two calls in one statement even though the underlying fields are disjoint. Fixed by
   cloning the query inputs (`page_id: String`, `selected_ids: Vec<String>`, `hovered_id:
   Option<String>`, `camera: Camera` (Clone), `viewport: Viewport` (Clone+Copy), `document_json:
   String`) into owned locals before the mutable borrow — no behavior change, just removes the
   aliasing.

**Verified (coordinator-grade, pasted, not claimed):**
```
$ export CARGO_TARGET_DIR=.../🎯️target-m6
$ cargo check -p semio-s-plugin-layout --lib
    Finished `dev` profile [unoptimized] target(s) in 2m 57s
$ echo $?
0                                    (110 pre-existing warnings, unrelated to this fix)

$ cargo check -p semio-s-plugin-layout --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 1m 31s
$ echo $?
0
```

## Part B — plugin migrations (14 crates)

Declarations (item 1: `.activation`/`.execution`/`.requests`, quota only where measured) applied
to all 14 crates' root `🦀️component.rs`, mirroring `🗒️note`'s/M0's/M1's exact shape. Wiring
verified for all 14 (`component-guest` feature + `plugin_exports!`) — **one gap found and fixed**:
`🔋️energy` was missing both (same class M0 found on stdio), now matches `🗒️note` verbatim.

`HostEffect`/`pending_effects`/`fn tick(` all confirmed **zero** across all 14 crates by grep
before any edit — items 2/3 of the migration are no-ops everywhere, same finding M0/M1 made for
their crates (A3's repo-wide mechanical rename already covered this fleet).

Acceptance status per crate below — filled in as each completes. **This section is authoritative;
do not trust anything above it if it conflicts.**

| crate | `.activation` kinds | `--lib` | `--target wasm32-wasip2` | descriptor |
|---|---|---|---|---|
| 🏗️fem | 2 (computation.fem2d, computation.fem3d) | pending | not run | not attempted |
| 🏛️architect | 1 (program) | not run | not run | not attempted |
| 🔱️trinity | 2 (jack, rewrite) | not run | not run | not attempted |
| 🧱️block | 3 (block2d/3d/5d) | not run | not run | not attempted |
| 🪐️space | 2 (home, space) | not run | not run | not attempted |
| 🌿️vcs | 1 | not run | not run | not attempted |
| 🎞️animate | 1 (present) | not run | not run | not attempted |
| 🎥️shooting | 1 | not run | not run | not attempted |
| 🎬️sequence | 1 | not run | not run | not attempted |
| ✒️writer | 1 | not run | not run | not attempted |
| 💡️reasoning | 1 (wires) | not run | not run | not attempted |
| 🕸️dag | 1 | not run | not run | not attempted |
| 🔋️energy | 1 (model) | not run | not run | not attempted |
| 📕️norm | 15 (din/en/iso/vdi families) | not run | not run | not attempted |

### 🪐️space — `host_now_ms` finding

The brief flagged `🪐️space` as "the only crate calling `host_now_ms` directly." Verified against
the actual code: this is **already fixed**, not by this packet. `grep -rn host_now_ms` under
`✏️s/🔌️plugins/🪐️space` → **zero hits**. The two real call sites
(`⚙️engine/🪐️space/🦀️component.rs:139,157`) already call `host::now_ms()` where `host` is imported
from `semio_framework_plugin::host` (`⚙️engine/🪐️space/🦀️component.rs:33-37`) — the new SDK's async
host module (`🔌️plugin/🌐host/🦀️component.rs:335`), exactly the route the brief asks for. Some
earlier packet in this ticket's wave (A2/A5/B1b territory) already did this rename. No action
needed; recorded here so it isn't mistaken for an open item.

### 🏗️fem — job migration: investigated, NOT done, real reason (not mine to invent)

Verified the solver is genuinely synchronous inside render/export, not hypothetical:
`fem3d_solve_all`/`fem2d_solve_all` (`✏️s/🔨️modules/🏗️fem/⚙️engine/{🧊️3d,◻2d}/🦀️component.rs`, the
shared FE engine module, mounted into the plugin via `📦️glue.rs` `#[path]`) are called directly
from `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs` — the doc comment there says
outright "`fem2d_solve`/`fem2d_solve_all` run fresh inside `render()`/`export_media` whenever the
results panel is opened." A real unbounded-fuel risk, exactly as the brief predicted.

**Why it isn't moved to a job in this packet.** I read the actual job-dispatch mechanism
(`🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`, `start_job`/`step_job`/`cancel_job`) rather than
assuming it exists generically. It does not: `step_job` is a hardcoded `match record.kind.as_str()`
covering exactly two absorbed io kinds (`semio.io-run`, `semio.io-sniff`) with **no
plugin-extensible registration point** — no `.job_handler(kind, fn)` on the `Plugin` builder, no
per-plugin dispatch table. The module's own doc comment anticipates precisely this gap: "a future
kind that genuinely spans multiple `step-job` calls (WFC, FEM solve, SfM, brep tessellation — see
design-abi.md §6) will need real fuel/deadline bookkeeping in `JobRecord`; not needed by anything
landing in this wave." Building that generic extension point is `🔌️plugin/⚛️reactor/**` — framework
SDK work, not `✏️s/🔌️plugins/**` — outside this packet's `path_scope`, and per `📌️important.md`
rule 3 that means a `lease-request`, not a unilateral edit to a shared crate five other packets
depend on. **Recorded as a real, verified gap and a concrete follow-up** (add a plugin-registrable
job-kind dispatch to the reactor's jobs module, then give fem's results-window render path a
"cached or spawn `fem.solve` job, render a placeholder until `job-completed`" path) rather than
silently left undone or hacked in-place.

## Files touched so far

**Part A** (`📏️layout`, all within its own owned tree):
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs`
- `✏️editor/🌉️wasm/🦀️component.rs`

**Part B** (each crate's own root `🦀️component.rs`, plus `🔋️energy`'s `Cargo.toml`+`📦️glue.rs`):
`🏗️fem`, `🏛️architect`, `🔱️trinity`, `🧱️block`, `🪐️space`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`,
`🎬️sequence`, `✒️writer`, `💡️reasoning`, `🕸️dag`, `🔋️energy`, `📕️norm`.

**Not touched**: `🔌️plugin/⚛️reactor/**` (fem's job-dispatch gap — flagged, not hacked in), any
peer-owned or `🧰️framework/**` file outside the two `📏️layout`-owned bugs above, other plugins.
