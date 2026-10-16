# 📝️ Applies the WP0 reconciliation to PLAN-RENDERER-WGPU-CRATES.md in place: drifted numbers (marked "was ..."), the hand-off contract 10.4, appendices F, G, H. Idempotent: refuses to run twice.
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = HERE + '/../PLAN-RENDERER-WGPU-CRATES.md'
S = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/planparts'
text = open(PLAN, encoding='utf-8').read()
assert '### 10.4 WP0 hand-off contract' not in text, 'already applied'
W = '**[WP0 change]**'
def rep(old, new, count=1):
    global text
    assert text.count(old) == count, f'anchor count {text.count(old)} != {count}: {old[:90]!r}'
    text = text.replace(old, new)
def in_section(heading, old, new):
    global text
    a = text.index(heading); b = text.index('\n#### ', a + 10) if '\n#### ' in text[a + 10:] else len(text)
    seg = text[a:b]
    assert seg.count(old) == 1, f'{heading}: {old[:60]!r} x{seg.count(old)}'
    text = text[:a] + seg.replace(old, new) + text[b:]

# header
rep("Notation. `E/` =", "**WP0 executed 2026-10-15 (phase 0, `RESULT-RENDERER-WP0.md`).** Every place this plan changed against the live tree is marked **[WP0 change]**; the hand-off contract is section 10.4, the frozen crate, hub and lift tables are Appendices F, G and H. The numbers of sections 1 to 9 keep the planner's values and show the WP0 re-measure as `(was …)` where it moved.\n\nNotation. `E/` =")

# section 0
rep("1. Today the library closure is **146 files / 5,861,374 bytes / 101,831 lines** (native), 130 files / 5,459,031 B / 91,304 lines (wasm32-unknown-unknown), **2 files / 10,079 B / 221 lines** (wasm32-wasip2: only `📮️runtime-mailbox-core`). The test closures are **505 files / 11,217,952 B / 157,068 lines** (native, 223 embedded files) and 375 files / 9,383,249 B / 123,640 lines (wasm32). The `OUT_DIR` icons include and the embedded icon SVGs are gone (a peer finished the runtime-asset channel while this plan was written; the closure above is the real tree, no overlay).",
    "1. Today the library closure is **402 files / 5,966,814 bytes / 102,014 lines** (native; was 146 / 5,861,374 / 101,831), 386 files / 5,564,510 B / 91,472 lines (wasm32-unknown-unknown; was 130 / 5,459,031 / 91,304), **2 files / 9,943 B / 219 lines** (wasm32-wasip2: only `📮️runtime-mailbox-core`). The test closures are **756 files / 11,325,874 B / 157,346 lines** (native, 473 embedded files; was 505 / 11,217,952 / 157,068 / 223) and 626 files / 9,489,058 B / 123,883 lines (wasm32; was 375 / 9,383,249 / 123,640) as measured before the hubs (the test rows cannot be re-measured at the moment, 10.4.8). " + W + " The `OUT_DIR` include is gone, but the embedded icon SVGs are **not**: a peer replaced the build-script copies by a checked-in generated file `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (255 lines) that `include_str!`s the 250 raw icon SVGs, mounted by `IconRenderHost`. That is +251 files and 261 embedded files in the lib closure and breaks the file budget of `icon-host` (risk 7, section 13).")
rep("**46 non-Shell lib crates (45 new + the existing crate reduced to the thin top crate) + 24 Shell lib crates + 17 companion test crates (4 non-Shell, 12 Shell, 1 shared test-support) = 87 crates**",
    "**46 non-Shell lib crates (45 new + the existing crate reduced to the thin top crate) + 24 Shell lib crates + 19 companion test crates (4 non-Shell, 14 Shell, 1 shared test-support) = 89 crates, 88 of them new** " + W + " (the planner counted 12 Shell bins and 87 crates; Appendix D lists 14 bins)")
rep("10. Parallelism: WP0 (serial, first: skeletons for all 87 crates, registries, root split into one mount file per WP)", "10. Parallelism: WP0 (serial, first: skeletons for all 88 new crates, registries, root split into one mount file per WP; **executed**, section 10.4)")
rep("12. Peers are changing the tree under this plan (the `🔤️compiler-fonts` to `🔤️runtime-assets` rename and the icon-pack runtime channel landed during the session; the old `📋️` / `🗑️generated` files are untouched). Re-run the census (section 12) at WP0 start and diff against section 1.",
    "12. Peers are changing the tree under this plan (the `🔤️compiler-fonts` to `🔤️runtime-assets` rename and the icon-pack runtime channel landed during the session; the old `📋️` / `🗑️generated` files are untouched). Re-run the census (section 12) at WP0 start and diff against section 1. " + W + " The re-run found: `runtime-assets` grew to 88 lines, the icon SVG embeds above, `IconRenderHost` root 185 to 179 lines, a one-line `use ui_gpu::{GpuContext};` in the root, and a handful of Shell/frame files that moved by 1 to 170 bytes (Appendix A). No file moved between packages.")

# 1.1
rep("| lib | native (aarch64-apple-darwin, `not(wasm32)`) | 146 | 5,861,374 | 101,831 | 11 |\n| lib | `wasm32-unknown-unknown` | 130 | 5,459,031 | 91,304 | 8 |\n| lib | `wasm32-wasip2` (only `runtime_mailbox_core` mounts) | 2 | 10,079 | 221 | 0 |\n| lib + tests | native, `--cfg test` | 505 | 11,217,952 | 157,068 | 223 |\n| lib + tests | wasm32, `--cfg test` | 375 | 9,383,249 | 123,640 | 155 |",
    "| lib | native (aarch64-apple-darwin, `not(wasm32)`) | 402 (was 146) | 5,966,814 (was 5,861,374) | 102,014 (was 101,831) | 261 (was 11) |\n| lib | `wasm32-unknown-unknown` | 386 (was 130) | 5,564,510 (was 5,459,031) | 91,472 (was 91,304) | 258 (was 8) |\n| lib | `wasm32-wasip2` (only `runtime_mailbox_core` mounts) | 2 | 9,943 (was 10,079) | 219 (was 221) | 0 |\n| lib + tests | native, `--cfg test` | 756 (was 505) | 11,325,874 (was 11,217,952) | 157,346 (was 157,068) | 473 (was 223) |\n| lib + tests | wasm32, `--cfg test` | 626 (was 375) | 9,489,058 (was 9,383,249) | 123,883 (was 123,640) | 405 (was 155) |\n\n" + W + " Re-measured 2026-10-15 17:58 (lib rows, after the five mount hubs: +5 files, the renderer root keeps 665 active lines) and 17:30 (test rows, before the hubs; the test closure cannot be re-measured since a peer removed `🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`). The 251 new files are the icon embeds (250 SVGs + the generated index) plus the five hubs; the lines moved by the hubs are neutral (about +25 lines for the banners, the five `include!` lines and the repeated region markers). `closure.ts` of the ticket folder reproduces every row.")

# 1.3
rep("They are included in every measurement here.", "They are included in every measurement here. " + W + " During phase 0 a peer also added `use ui_gpu::{GpuContext};` to the renderer root and removed `🖱️ui/…/🧊️gpu/🦀️.rs`; `🖼️IconRenderHost/…/🦀️.rs` is staged with the icon-embeds change.")

# 2.1
rep("* `PKG/📚️library/🦀️.rs` (23 lines): mounts `R/📇️registry` (non-wasi), `T/⌨️native-entrypoint/📦️modules` (native), `T/📮️runtime-mailbox-core` (wasi),", "* `PKG/📚️library/🦀️.rs` (23 lines; " + W + " 20 lines now): mounts `R/📇️registry` (non-wasi; stays here, 10.4.1), ~~`T/⌨️native-entrypoint/📦️modules` (native)~~ (moved into the WP1 hub), `T/📮️runtime-mailbox-core` (wasi),")
rep("* `R/🦀️.rs` (the included root, 921 lines): `extern crate` aliases", "* `R/🦀️.rs` (the included root, 921 lines; " + W + " 665 active lines after phase 0, the module mounts, the 12 partitions and the root-level test mounts now live in the five hubs `R/📥️mounts/<hub>/🦀️.rs`, `include!`d from the root, Appendix G): `extern crate` aliases")

# 2.3 embedded assets
rep("* **Embedded assets left in the lib closure** (11):", "* **Embedded assets left in the lib closure** (" + W + " 261 now: the 11 below plus the 250 raw icon SVGs of `🖼️assets/🔣️icons`, embedded through the generated `🧩️icon-embeds/🦀️.rs`; the 11 were):")

# 6.1 rows
rep("| 2 | `host-base` | 1 | 2015 | 112 | 11 | 425 / 30 / 6 |  | native + wasm32 |", "| 2 | `host-base` | 1 | 2039 (was 2015) | 114 (was 112) | 11 | 425 / 30 / 6 |  | native + wasm32 |")
rep("| 25 | `icon-host` | 2 | 2289 | 101 | 7 | 675 / 62 / 12 |", "| 25 | `icon-host` | 2 | 2538 (was 2289) | 121 (was 101) | 8 (was 7) + 251 embedded | 675 / 62 / 12 |")
rep("Every row satisfies the budget with its unit tests except those flagged in 9.3", W + " `icon-host` satisfies lines and bytes (2,538 lines, 121 KB) but not files: 8 sources + 251 embedded files = 259 against 28 (hard ceiling 32), see 6.2 and risk 7. Every other row keeps the planner's value; the measured per-file drift is below 0.3 percent (Appendix A).\n\nEvery row satisfies the budget with its unit tests except those flagged in 9.3")

# 6.2
in_section("#### `host-base` (WP1", "`web_sys`, `semio_framework_ui_locale`", "`web_sys`, `semio_framework_ui_locale`, `semio_framework_canvas` " + W + " (`T/🔤️runtime-assets` installs the Canvas runtime assets)")
for name in ("engine-registry", "engine-board", "engine-graph"):
    pass
in_section("#### `engine-registry` (WP2", "`semio_framework_os_kernel`\n", "`semio_framework_os_kernel`, `semio_framework_canvas` " + W + " (`use semio_framework_canvas as canvas;` in the surface registry, prepared canvas and tutorial geometry files)\n")
in_section("#### `engine-board` (WP2", "`semio_framework_surface`\n", "`semio_framework_surface`, `semio_framework_canvas` " + W + "\n")
in_section("#### `engine-graph` (WP2", "`ui_styling`\n", "`ui_styling`, `semio_framework_canvas` " + W + "\n")
in_section("#### `engine-attach` (WP2", "`semio_framework_io_base64`\n", "`semio_framework_io_base64`, `semio_framework_canvas`, `framework_editor`, `semio_framework_os_node_graph`, `semio_framework_surface` " + W + " (named by the live source)\n")
in_section("#### `scene-state` (WP2", "`semio_framework_trace`\n", "`semio_framework_trace`, `semio_framework_os_renderer_camera` (`camera_storage`) " + W + "\n")
in_section("#### `icon-host` (WP2", "* cfg: native + wasm32\n", "* cfg: native + wasm32\n* " + W + " **File budget.** The lib closure of this crate today is 8 sources (7 plus the generated `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs`, 255 lines, 20,119 B) and 251 embedded files (the 250 raw icon SVGs and the 682-byte logo), 259 files against 28 (ceiling 32) and 164 KB of embeds. The embeds must leave the crate before `icon-host` can pass the gate: feed the atlas from the Canvas icon pack installed through the runtime asset port (`semio_framework_canvas::RuntimeAsset::IconPack`, installed by `host-base` `runtime-assets`) or add a raw-icon asset to that port; the 250 `include_str!` list is a build-time index of checked-in sources, not data that has to be compiled in. Decision for WP2 with the owner of the Canvas assets.\n")
in_section("#### `icon-host` (WP2", "(the catalogue now comes from the Canvas icon pack installed through the runtime asset port; no SVG is embedded)", "(" + W + " the plan assumed the catalogue comes from the Canvas icon pack installed through the runtime asset port; the live tree still embeds the 250 SVGs through `ICON_SVGS`, see the file budget above)")

# 9.4 and 10.2
rep("Shell (12), packed by theme", "Shell (14), packed by theme " + W + " (the planner wrote 12; the groups listed below and Appendix D are 14)")
rep("**Phase 0 (WP0, serial, before anyone else starts):**", "**Phase 0 (WP0, serial, before anyone else starts)** " + W + " **executed 2026-10-15; what it created and the rules that follow are in 10.4:**")
rep("1. Generate all 87 crate skeletons in one pass:", "1. Generate all 88 new crate skeletons (89 crates with the top crate) in one pass:")
rep("into five files `R/📥️mounts/wp1…wp5/🦀️.rs`, each owned by one package and `include!`d from the root;", "into five files `R/📥️mounts/<hub>/🦀️.rs` (hub folders `🌱️wp1-foundation`, `🎞️wp2-scenes`, `🧵️wp3-frame-core`, `🐚️wp4-shell-state`, `🎛️wp5-shell-views`), each owned by one package and `include!`d from the root (the module-tree helper of the TS readers follows `include!` since phase 0);")
rep("3. Create the 16 empty companion test crates (12 Shell, 4 others) and the `test-support` crate.", "3. Create the 18 empty companion test crates (14 Shell bins, 4 others) and the `test-support` crate " + W + " (16 and 12 were the planner's counts).")

# 12 and 13
rep("The programs fit in about 600 lines of Python", W + " The WP0 programs are in the ticket folder `🪓️monolith-split/rwp0-tools/` (`closure.ts`, `appendix_a_drift.py`, `extscan.py`, `shelledges.py`, `crates.py`, `gen_files.py`, `gen_registry.py`, `hubsplit.py`, `verify_static.ts`, `verify_hub.py`, `oracle.py`, `ts_compare.py`, `ts_vitest.py`, `tree_compare.ts`, `taxonomy-scope.ts`, `plan_appendix.py`, `plan_edit.py`).\n\nThe planner's programs fit in about 600 lines of Python")
rep("5. **87 crates is a lot of registry** (87 × 6 launch routes × 2 JSON files, owner rows, locks). That cost is mechanical and entirely WP0.", "5. **88 new crates is a lot of registry** (88 × 6 launch routes × 2 JSON files, owner rows, locks). That cost is mechanical and entirely WP0; phase 0 paid it except the lock files.")
rep("6. **Decision requested**:", "6. **Decision requested** (assumed yes by phase 0: the skeleton companions depend on production crates through `pub` API only):")
text = text.rstrip('\n') + "\n"
risks = ("\n7. " + W + " **`icon-host` and the embedded icon SVGs.** The tree embeds 250 raw icon SVGs plus the logo through a generated index (+251 files in every lib and test closure); with them `icon-host` is at 259 files against a 28 file budget and cannot pass the gate (6.2). Needs the runtime asset channel or a raw-icon asset; WP2 with the Canvas owner.\n"
         "8. " + W + " **Hubs are `include!`d.** Every macro-expanded `#[macro_export]` macro stays reachable by `crate::` paths only if its module is declared outside an `include!`d file (found: `renderer_registration`, kept in the library root). Any new `#[macro_export]` macro in a hub-mounted module has the same defect; declare such modules in the library root or re-export the macro from a real crate.\n"
         "9. " + W + " **The renderer root is still edited by peers** (`use ui_gpu::{GpuContext};` appeared while phase 0 ran). The WP0 differential supports `--baseline inverse` (the current root and hubs un-hubbed against the current tree) for exactly that reason; WP1 owns the root, so it reconciles such lines.\n"
         "10. " + W + " **No cargo evidence.** The launcher stops with `Finite original maintenance authority required`; the first real compile of any skeleton, and the first `Cargo.lock` refresh, happen after the infrastructure is back. Until then the static gate, the census and the differentials are the evidence.\n")
m = re.search(r"(6\. \*\*Decision requested\*\*[^\n]*\n)", text)
text = text[:m.end()] + risks.lstrip('\n') + text[m.end():]

# hand-off contract before section 11
handoff = open(S + '/handoff.md', encoding='utf-8').read()
fixes = [("Ten vitest readers (`🧾️frame-action-ledger`, `🫀️settle-pump`, `🖱️wheel-application-point`, `🎮️wgpu-browser-input-wire`, `🎮️wgpu-runtime-mailbox-admission`, `🎯️presented-input-authority`, `📮️wgpu-frame-deferred-ownership`, `🔢️frame-generation-hold`, `🛍️app-catalogue-attempt`, `🪟️action-window-scope`) fail with `ENOENT` and three (`🌳️wgpu-document-reconcile`, `🔗️hub-projection`, `🚗️driver-editor`) fail resolving `@semio-tech/framework-os-node-graph-rs` identically before and after phase 0: they do not witness a carve until those peer faults are fixed.",
          "Ten vitest readers (`🧾️frame-action-ledger`, `🫀️settle-pump`, `🖱️wheel-application-point`, `🎮️wgpu-browser-input-wire`, `🎮️wgpu-runtime-mailbox-admission`, `🎯️presented-input-authority`, `📮️wgpu-frame-deferred-ownership`, `🔢️frame-generation-hold`, `🛍️app-catalogue-attempt`, `🪟️action-window-scope`) fail with `ENOENT` on `E/🧫️fixtures/<name>/🔣️.json` (the fixture folders are empty in the working tree) and three (`🌳️wgpu-document-reconcile`, `🔗️hub-projection`, `🚗️driver-editor`) fail resolving `@semio-tech/framework-os-node-graph-rs`; all thirteen fail identically before and after phase 0 (24 vitest files and 20 bun test files compared), so they do not witness a carve until those environment faults are fixed.")]
for o, n in fixes:
    assert handoff.count(o) == 1; handoff = handoff.replace(o, n)
rep("## 11. Verification and gates", handoff.rstrip('\n') + "\n\n## 11. Verification and gates")

# Appendix A rows
def row(n, cell, lines, nbytes, gate, tests, target): return f"| {n} | `{cell}` | {lines} | {nbytes} | {gate} | {tests} | {target} |"
upd = {
 "| 24 | `T/🔤️runtime-assets` | 64 | 3647 |  | 40 | host-base |": "| 24 | `T/🔤️runtime-assets` | 88 (was 64) | 5219 (was 3647) |  | 40 | host-base |",
 "| 29 | `T/🧊️renderer` | 921 | 49580 |  | 3602 | host-base |": "| 29 | `T/🧊️renderer` | 665 (was 921; the mounts moved to the hubs, rows 140 to 144) | 38042 (was 49580) |  | 3602 | host-base |",
 "| 20 | `T/📦️packages/🦀️rust/📚️library` | 23 | 990 |  | 166 | entrypoint |": "| 20 | `T/📦️packages/🦀️rust/📚️library` | 20 (was 23) | 854 (was 990) |  | 166 | entrypoint |",
 "| 38 | `T/🧊️renderer/📮️mailbox-impl` | 1533 | 88653 |": "| 38 | `T/🧊️renderer/📮️mailbox-impl` | 1535 (was 1533) | 89359 (was 88653) |",
 "| 45 | `T/🧊️renderer/🧠️app-runtime` | 1446 | 89096 |": "| 45 | `T/🧊️renderer/🧠️app-runtime` | 1446 | 89296 (was 89096) |",
 "| 46 | `T/🧊️renderer/🧾️frame-build` | 1463 | 84103 |": "| 46 | `T/🧊️renderer/🧾️frame-build` | 1463 | 84433 (was 84103) |",
 "| 66 | `🎞️Scenes/🎨️scene-render-entry` | 704 | 38787 |": "| 66 | `🎞️Scenes/🎨️scene-render-entry` | 704 | 38936 (was 38787) |",
 "| 71 | `🎞️Scenes/🖼️icon-host` | 454 | 21071 |": "| 71 | `🎞️Scenes/🖼️icon-render` (the planner's row said `icon-host`, a name slip) | 454 | 21079 (was 21071) |",
 "| 81 | `🐚️Shell/⌨️keyboard-and-palette` | 2246 | 131583 |": "| 81 | `🐚️Shell/⌨️keyboard-and-palette` | 2246 | 131728 (was 131583) |",
 "| 86 | `🐚️Shell/⚙️settings-and-dock-persistence` | 2030 | 123883 |": "| 86 | `🐚️Shell/⚙️settings-and-dock-persistence` | 2030 | 124053 (was 123883) |",
 "| 87 | `🐚️Shell/🎓️tutorial-runtime` | 823 | 45642 |": "| 87 | `🐚️Shell/🎓️tutorial-runtime` | 824 (was 823) | 45797 (was 45642) |",
 "| 97 | `🐚️Shell/🖱️pointer-input` | 2083 | 130723 |": "| 97 | `🐚️Shell/🖱️pointer-input` | 2083 | 130853 (was 130723) |",
 "| 98 | `🐚️Shell/🖼️chrome-frame-and-main-window` | 2052 | 119476 |": "| 98 | `🐚️Shell/🖼️chrome-frame-and-main-window` | 2052 | 119543 (was 119476) |",
 "| 102 | `🐚️Shell/🧰️window-actions-and-commands` | 2247 | 137329 |": "| 102 | `🐚️Shell/🧰️window-actions-and-commands` | 2247 | 137344 (was 137329) |",
 "| 103 | `🐚️Shell/🧱️chrome-paint-and-navbar` | 2278 | 137976 |": "| 103 | `🐚️Shell/🧱️chrome-paint-and-navbar` | 2278 | 138118 (was 137976) |",
 "| 114 | `🖼️IconRenderHost/T` | 185 | 8161 |": "| 114 | `🖼️IconRenderHost/T` | 179 (was 185) | 7898 (was 8161) |",
 "| 116 | `🖼️IconRenderHost/📤️export/🎬️scene` | 530 | 24534 |": "| 116 | `🖼️IconRenderHost/📤️export/🎬️scene` | 530 | 24917 (was 24534) |",
 "| 7 | `T/🌐️browser-worker` | 0 | 0 | target_arch = \"wasm32\" | 0 | browser-worker 99% |": "| 7 | `T/🌐️browser-worker` | 0 (wasm32: 935, 59962 B) | 0 | target_arch = \"wasm32\" | 0 | browser-worker 99% |",
}
for o, n in upd.items(): rep(o, n)
extra = ("| 139 | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds` " + W + " | 255 | 20119 |  | 0 | icon-host (+ 250 embedded SVGs, 164,569 B) |\n"
         "| 140 | `T/🧊️renderer/📥️mounts/🌱️wp1-foundation` " + W + " | 150 | 8195 |  | 0 | hub of WP1 |\n"
         "| 141 | `T/🧊️renderer/📥️mounts/🎞️wp2-scenes` " + W + " | 17 | 1058 |  | 0 | hub of WP2 |\n"
         "| 142 | `T/🧊️renderer/📥️mounts/🧵️wp3-frame-core` " + W + " | 98 | 5304 |  | 0 | hub of WP3 |\n"
         "| 143 | `T/🧊️renderer/📥️mounts/🐚️wp4-shell-state` " + W + " | 8 | 677 |  | 0 | hub of WP4 |\n"
         "| 144 | `T/🧊️renderer/📥️mounts/🎛️wp5-shell-views` " + W + " | 9 | 747 |  | 0 | hub of WP5 |\n")
anchor = "| 138 | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh` | 115 | 5726 |  | 57 | host-base |\n"
rep(anchor, anchor + extra)

# appendices F G H at the end
app = json.load(open(HERE + '/../🗑️generated/rwp0-appendix.json', encoding='utf-8'))
tail = ("\n## Appendix F. Crate registry (frozen by WP0, generated by `rwp0-tools/crates.py` and `plan_appendix.py`)\n\n"
        "`T/` = `E/🎯️targets/🧊️wgpu/`. The package folder of a crate is `T/<folder>/📦️packages/🦀️rust`, its library root `T/<folder>/🦀️.rs`. `rank` is the position in the dependency order of 4.3 (shell crates in the order of 7.3/7.7; companions last). Dependencies are the direct first-party edges declared in the manifest; native-only dependents are reached through the native target table.\n\n"
        + app['F'] + "\n\n"
        "## Appendix G. Hub units (the 61 `mod` mounts of the five hubs, test modules included)\n\n"
        "The two `include!`d standalone test files of the WP3 hub (`🧪️tests/🧊️wgpu-renderer-standalone`, `🧪️tests/🐕️wgpu-present-stall-watch`) are no modules and are not listed; they go to `native-probe` and `frame-presentation`. A unit that maps to one crate is replaced by the line in the last column when the crate is carved (10.4.4).\n\n"
        + app['G'] + "\n\n"
        "## Appendix H. Item-move groups of 5.2 by package (the 12 groups that cross packages are lifted through `📥️lifted`)\n\n"
        + app['H'] + "\n")
text = text.rstrip('\n') + "\n" + tail
open(PLAN, 'w', encoding='utf-8').write(text)
print('plan updated;', len(text), 'chars')
