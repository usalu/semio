# Box Shell Preview — `shell-orientation-inward` was a leftover scaffold solid, not a wasm divergence (2026-09-12)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "box shell orientation". Repo MCP was down for the
whole run (`invalid initialize params` / `Connection closed`); ticket bookkeeping is on disk and no
ticket was opened, closed or reopened.

Outputs: `🗑️generated/box-shell/`. Runtime probe: `🐍️mesh-delivery-probe.mjs` with
`SEMIO_PROBE_PICK="Box Shell Preview"` → `🗑️generated/box-shell/runtime-after-republish/`.

---

## 1. Verdict

**Both premises the lane was handed were wrong, and the truth is simpler than either.**

* The served wasm was **not stale** — it was the 08:29 build, byte-identical in the staging root and
  in the runtime install root the page reads (§2).
* The kernel does **not** differ between native and `wasm32-wasip2` — there is no `cfg`, no `libm`,
  no `f32` path and no hash-order sensitivity that changes this result, and the identical law now
  passes on **both** targets (§5).

The defect was in the kernel, on **every** target, and it was reproducible natively in one
14-line test the moment the right question was asked. `↔️offset::shell_solid` built the
`-thickness` offset through `offset_solid_with_corner`, which wraps its faces in a `Solid` **of its
own**; `shell_solid` then flipped every one of those faces to face into the cavity, handed them to
the real shelled solid — and **left the wrapper in the body**. `−4.096` is exactly `−1.6³`: that
orphan's own outer shell, now inside out.

`validate_body` walks every solid in the body, and `Brep::validate_gate_sync` refuses on any
error-class issue **anywhere** in it — so one orphan refused every preview taken from that kernel
afterwards, including handles that never went near the shell.

The native `example-geometry` oracle stayed green (17/17, `+3.904`) only because its harness
disposes handles between examples, and `dispose` runs the arena GC (`Brep::compact_unreachable`)
that swept the scaffold before anything validated. The browser never gets that GC between the
`shell` node evaluating and the preview gate running. Same bytes, same arithmetic, different
lifecycle — which is why a rebuild could never have fixed it.

Fix: `🔺️euler::retire_solid_scaffold` (new, the exact inverse of `add_solid`/`add_shell` for faces
that are being handed on), called from `shell_solid`. One line at the call site, pinned by a
fixture-driven law that runs on native **and** `wasm32-wasip2` under wasmtime.

---

## 2. Which bytes the page loads (deliverable 1)

The react serve mounts extensions from the **runtime install root**, not the staging root:
`⚙️vite.config.ts:34` `installedExtensionsDir = path.join(runtimeRoot, "extensions")`, served under
`MODULE_EXTENSION_ROUTE = "/🧩️extension-modules"` (`📇️registry/📦️deployment/🛣️routes.json`) by
`semioExtensionStoreVitePlugin` (`🔌️plugin/🏪️store/📥️store.ts:374`). The plugin route
(`/🔌️plugin-modules`, `⚙️vite.config.ts:184`) filters to `role === "plugin"` and therefore never
serves an extension at all — which is why the three "obvious" URLs all return the 2 428-byte SPA
`index.html` with a `200`, not the wasm.

The real route, fetched and hashed:

```
GET http://127.0.0.1:6018/%F0%9F%A7%A9%EF%B8%8Fextension-modules/%F0%9F%A7%8A%EF%B8%8Fflow-extension-brep/semio_s_plugin_flow_extension_brep_component.core.wasm
200 · application/wasm · 20 438 438 B
sha256 d69fb26642d135a97b1176170e27ecbde4a93a50fba3c8779cb0b648ef159c56
```

Against the staged copies, at the time of the defect:

| copy | sha256 | mtime |
|---|---|---|
| **served** (`GET /🧩️extension-modules/…`) | `d69fb266…` | — |
| runtime install root `…/🧑‍💻dev/…/dist/runtime/dev/generation3d/extensions/🧊️flow-extension-brep/` | `d69fb266…` | 2026-09-12T08:23:53 |
| one staging root `…/🔌️plugin/…/dist/dev/🔌️plugin-modules/🧊️flow-extension-brep/` | `d69fb266…` | 2026-09-12T08:29:33 |
| crate dist `…/📐️brep/📦️packages/🦀️rust/dist/component-dev/` (pre-jco) | `18d481b4…` | 2026-09-12T08:29:32 |
| retired tree B `…/🧑‍💻dev/🔌️plugin-modules/` (no longer on any code path) | `8472f353…` | 2026-09-12T04:48:24 |

**Served == install root == staging root.** The bytes were fresh. `🗑️generated/restage-7/activate.txt`
confirms the 08:29 run really recompiled the kernel into them (`:152`/`:488` `Compiling
semio-s-artifact-stdio-semio`, `:499` `Compiling semio-s-plugin-flow-extension-brep`,
`:1356 built program flow-extension-brep (wasm32-wasip2, wasm-dev)`), and `strings` finds the
`shell-orientation-inward` literal compiled into the served module.

So **deliverable 3 does not apply** — nothing was stale, and no publish/staging path needed
repairing. `📓️preview-mesh-delivery-2026-09-12.md` §2.3's "stale staged wasm" reading is hereby
retired; its two-different-handles observation was real but is explained by §3, not by staleness.

---

## 3. Root cause (deliverable 2)

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs`,
`shell_solid`, as it stood at line 1167–1178:

```rust
let outer_faces = body.solid_faces(solid);
let corner = if is_planar_only(body, solid) { OffsetCorner::Sharp } else { OffsetCorner::Round };
let inner_solid = offset_solid_with_corner(body, solid, -thickness, corner, rec)?;   // ← wraps its faces in a Solid
let inner_faces = body.solid_faces(inner_solid);
for &f in &inner_faces {
    if let Some(fd) = body.faces.get_mut(f) { fd.flipped = !fd.flipped; }            // ← and now that Solid lies
}
let outer_shell = add_shell(body, outer_faces, rec);
let inner_shell = add_shell(body, inner_faces, rec);
Ok(add_solid(body, outer_shell, vec![inner_shell], rec))                             // ← inner_solid never retired
```

`offset_solid_with_corner` ends in `🧱️primitives::finish_solid` (`:83`), which is
`add_shell` + `add_solid` — a complete `Solid` record, not a bag of faces. `shell_solid` only ever
wanted the faces. The wrapper survives, and once its faces are flipped its outer shell integrates
to the cavity's volume with the wrong sign.

Measured directly, on a `Body` that is never disposed:

```
[DEBUG] box faces=[Face 0..5]
[DEBUG] shelled solid=solid-2-0 outer=Shell 2 inners=[Shell 3]
[DEBUG] outer shell faces=[Face 0..5]            → signed volume = 8
[DEBUG] void shell 3 faces=[Face 6..11]          → signed volume = -4.096000000003073
[DEBUG] solid signed volume = 3.903999999996927
[DEBUG] validation issues = [ValidationIssue { entity: "solid-1",
          code: "shell-orientation-inward",
          message: "outer shell's signed volume is negative (-4.096000000003073); …" }]
```

The shelled solid itself was **always right** — `+8`, `−4.096`, `+3.904`, exactly the fixture's
`analytic outer 2^3 minus inner (2-2*0.2)^3`. The issue is reported against `solid-1`, the orphan.
The browser reported `solid-2` because its body carries one more solid by then; `−4.096000000003073`
is the same number to the last digit, which is what identifies it.

### The cascade

Through the same `Brep` façade the wasm extension calls (`⚙️engine/🦀️.rs:1106 shell_sync` →
`shell_solid_with_open_faces` → `shell_solid`, `openFaces` being unwired in the example so the
closed path is taken):

```
[DEBUG] facade gate on shelled = Err([… solid-1 shell-orientation-inward …])
[DEBUG] facade gate on box     = Err([… solid-1 shell-orientation-inward …])
```

The gate refused the **untouched box handle** too. `validate_gate_sync` (`⚙️engine/🦀️.rs:1527`)
validates the whole body, so the scaffold is not a local defect — it poisons the session.

### Why native looked green

`example-geometry`'s harness evicts the mesh cache and disposes handles between examples;
`dispose` runs `compact_unreachable` (`⚙️engine/🦀️.rs:598`), whose reachability walk starts from live
handles — and the scaffold has none. It was collected before anything validated. The browser's
`flowEvalTick` runs the gate while the scaffold is still there.

### Ruled out, with evidence

* **Hash-order nondeterminism.** `↔️offset/🦀️.rs:929/:1011/:1246` iterate `HashSet`s, so face/edge
  construction order (hence arena ids) varies per process seed. Twenty fresh runs of
  `box_shell_preview_evaluates_to_a_hollow_solid`, each with a new `RandomState`: 20/20 identical.
  Order does not reach this result.
* **Target divergence.** No `cfg(target_arch = "wasm32")`, no `#[cfg(not(…))]`, no `libm`, and no
  `sort_unstable` in the brep subset's production code; every `f32` is an output-boundary downcast
  in the mesh transfer contract. The law in §5 now runs the same cases on both targets and both
  agree to `1e-6`.
* **A stale artifact.** §2.

---

## 4. The fix

### `🔺️euler::retire_solid_scaffold` — new

`…/🧊️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs`, in `#region 🔖️Make` beside `add_shell`/`add_solid` whose
inverse it is. It drops a solid and its shells and **leaves every face, loop, edge and vertex
alone** — the caller owns them now. Deliberately *not* `🔀️boolean`'s `remove_solid_and_orphans`,
which also deletes faces the caller no longer wants; these are two different operations and the new
one is the one `shell_solid` needs.

### `↔️offset::shell_solid` — one call

```rust
    retire_solid_scaffold(body, inner_solid, rec);
```
at `↔️offset/🦀️.rs:1186`, between the flip loop and `add_shell`. Both docstrings now carry the
defect and the ticket, so the next reader knows why the wrapper must go.

### Deliberately NOT changed

`validate_gate_sync` validating the whole body is arguably too wide — one bad entity anywhere
refuses every preview. That is a real sharp edge and this lane proved it, but widening or narrowing
the gate would have hidden the actual defect rather than fixed it, and the gate's breadth is what
made the problem visible at all. Left as is, recorded here.

### An adjacent defect found, NOT in scope

The operator matrix probe turned up a second, pre-existing failure the same way:

```
[DEBUG] sphere+shell(round): gate REFUSED with 3 issue(s)
    coedge-5 same-parameter-violated — pcurve and 3D curve disagree by 1.5845662237736027 …
    coedge-7 same-parameter-violated — …
    solid-1-void-shell-2 void-shell-not-inverted — …
```

The **`Round`** corner path (any solid with a curved face) produces a shell whose cavity is not
inverted and whose seam p-curves disagree with their 3D curves. No bundled `generation3d` example
reaches it — all eight are planar or boolean — so it is recorded here rather than fixed in this
lane. It is present with and without this lane's change.

---

## 5. The law (deliverable 2), on both targets

`…/🧊️brep/🧪️tests/🐚️shell-orientation/🦀️.rs` + fixture `🔣️.json`, declared as
`[[test]] name = "brep_shell_orientation"` in the crate manifest (emoji file names never compile as
an implicit `tests/` target).

Three fixture cases — the example's own `2.0³ / 0.2`, a `1.0³ / 0.1` thin wall, and a `3×2×1 / 0.25`
anisotropic slab whose three wall spans differ off one thickness. Each asserts, on a `Brep` that is
**never disposed and never GC-ed** (the browser's situation, not the native harness's):

1. `validate_gate_sync` admits the shelled solid **and** the untouched input handle — the gate is
   whole-body, so checking only the shelled one would have missed the cascade entirely.
2. `body.solids.len() == 2` and `body.shells.len() == 3` — **the assertion that actually pins the
   fix.** Every volume below was already correct while the scaffold was there.
3. `validate_body` is empty.
4. Outer shell signed volume positive and equal to the outer box; the single void shell's negative
   and equal to the cavity; the solid's their sum. Closed form, never read back from the kernel.
5. `parry3d` — the third-party oracle, `[dev-dependencies]`-only, the same crate and version
   `🧲️procedural-example-booleans` already uses — recomputes that signed volume from the tessellated
   triangle soup alone, so the sign this law is named after never rests on our own arithmetic.

The fixture is `include_str!`-ed rather than read from disk precisely so the wasm run needs no
preopened directory.

### Negative control — the law fails without the fix

```
test shelling_a_box_leaves_no_scaffold_solid_behind ... FAILED
  assertion `left == right` failed: box-shell-preview: validate_body reported
    [ValidationIssue { entity: "solid-1", code: "shell-orientation-inward",
      message: "outer shell's signed volume is negative (-4.096000000003073); …" }]
test the_preview_validate_gate_admits_a_shelled_box_and_its_input ... FAILED
  assertion `left == right` failed: box-shell-preview: the preview gate must admit the shelled solid
test result: FAILED. 0 passed; 2 failed
```

The law reproduces the browser's diagnostic to the last digit. The control was reverted; no
`[DEBUG]` line survives in any source this lane touched.

### Results — every command run, verbatim

```
# native
cargo test -p semio-s-artifact-stdio-semio --test brep_shell_orientation
  test shelling_a_box_leaves_no_scaffold_solid_behind ... ok
  test the_preview_validate_gate_admits_a_shelled_box_and_its_input ... ok
  test result: ok. 2 passed; 0 failed

# wasm32-wasip2 under wasmtime 46.0.1 — the target the served component is built for
CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime \
cargo test -p semio-s-artifact-stdio-semio --target wasm32-wasip2 --test brep_shell_orientation
  Running …/build/wasm32-wasip2/debug/…/brep_shell_orientation-….wasm
  test shelling_a_box_leaves_no_scaffold_solid_behind ... ok
  test the_preview_validate_gate_admits_a_shelled_box_and_its_input ... ok
  test result: ok. 2 passed; 0 failed

# the two registered launch entries, through nx
bun nx run @semio-tech/stdio-semio-rs:test --args="--test brep_shell_orientation -- --nocapture"          → ok
bun nx run @semio-tech/stdio-semio-rs:test --args="--target wasm32-wasip2 --test brep_shell_orientation…" → ok

# surrounding brep suites, native
brep_procedural_example_booleans   2 passed
brep_extrude_orientation          12 passed
brep_tessellation_jobs            10 passed
brep_analytic_blend                7 passed

# the procedural example oracle (scipy/parry3d rows for this example)
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry
  test result: ok. 17 passed; 0 failed
  (box_shell_preview_evaluates_to_a_hollow_solid and delivery_box_shell_preview among them)
```

`cargo test -p semio-s-artifact-stdio-semio --lib` reports **2469 passed / 46 failed**. Those 46 are
**pre-existing and unrelated** — measured identically, 2469/46, with the fix and with the negative
control in place. They are snapshot-round-trip and mutation-fixture laws (`sphere`/`cylinder`/`torus`
round trips, `mutations::component::fixture_tests`, `value::schema::diff`), none of which touch
`shell_solid`; `box_round_trips_through_snapshot` passes.

### Launch registration

Two entries added to `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, directly after
`🧪️test🎨️brep🧊️analytic-blend`, following the existing order, grouping and naming:

* `🧪️test🐚️brep🧊️shell-orientation`
* `🧪️test🐚️brep🧊️shell-orientation🕸️wasm` (carries `CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime`)

---

## 6. Runtime (deliverable 4)

Rebuilt and republished the extension — `bunx nx run @semio-tech/framework-os-dev:plugin
--args="flow-extension-brep"`, exit 0, `Compiling semio-s-artifact-stdio-semio` then `Compiling
semio-s-plugin-flow-extension-brep`, `built program flow-extension-brep (wasm32-wasip2, wasm-dev)`.
The procedural plugin was **not** restaged and the 6018 serve was **not** restarted.

```
pre-fix  served  d69fb26642d135a97b1176170e27ecbde4a93a50fba3c8779cb0b648ef159c56
post-fix served  27dffe6811858ab3083678e47af6174500ea1438b8f3f1d6913d4cf6095ce64e
```

`GET /🧩️extension-modules/🧊️flow-extension-brep/…core.wasm` now returns `27dffe68…`, and both the
staging root and the runtime install root carry it. **No procedural restage was needed.**

One operational note for the next lane: immediately after `plugin` returned, the staging root held
`27dffe68…` while the install root still held `d69fb266…`. Publication into the install root is
**deferred** — it landed on the next page boot, via the extension store. A sha256 sampled seconds
after `plugin` can legitimately still show the old bytes; sample it after a reload. Relatedly, the
loud-staleness rule never flagged brep during that window: its `unpublished` verdict compares the
installed `📥️install.json` `packageHash` against the **receipt's** `artifactSha256` — both
`cb888870…`, in agreement — and never against the currently staged bytes, so a staging root that has
moved ahead of the install root is invisible to it. The only `[stale]` line the freshness pass
printed was the unrelated `flow: source-newer`.

### The probe

`🐍️mesh-delivery-probe.mjs`, `SEMIO_PROBE_PICK="Box Shell Preview"` →
`🗑️generated/box-shell/runtime-after-republish/`:

```
[DEBUG] pick:Box Shell Preview: [["window:procedural-main",0,0,null,null,null],
                                 ["window:procedural-preview",1,4339,"idle",1,null]]
```

Last preview sample:

```json
{"surfaceId": "window:procedural-preview",
 "meshCount": 1,
 "meshShapes": [{"id": "eval-shell@solid#0", "pos": 144, "idx": 72, "edge": 144}],
 "phase": "idle", "ratio": 1, "fault": null, "diagnostics": null}
```

* **`phase: "idle"`** — not `invalid`. The gate admits the shell.
* **`meshes = 1` (≥ 1)** — `eval-shell@solid#0`, `idx: 72` = **24 triangles**, exactly the fixture's
  `minTriangles: 24`, with 144 edge floats.
* **No orientation issue** — `grep -c 'shell-orientation-inward'` over both probe console captures
  returns **0**, as does `grep -c '"invalid"'`. `diagnostics: null`, `fault: null`.
* One tessellate round trip, `2 878 B` answered — against the `232 B` refusal envelope the
  coordinator-shell run recorded before the fix.

`2-pick-Box-Shell-Preview.png` shows the shelled solid painted in the preview window and the graph
reading `Shell Evaluated`.

The first two probe attempts timed out with `boot: []` on a blank page. That was a cold boot: the
republish rewrote eleven module directories and invalidated Vite's dependency optimization, so the
app tore down instance 1 (`actor-activation.revoked`) and rebooted. Once warm it booted normally and
the probe passed on the next run — not a defect, but worth knowing before chasing one.

---

## 7. Files

Changed:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs`
  — new `retire_solid_scaffold`.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs`
  — `shell_solid` retires the scaffold; import and docstring.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml`
  — `[[test]] brep_shell_orientation`.
* `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — two launch entries.

Added:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🐚️shell-orientation/🦀️.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🐚️shell-orientation/🔣️.json`

Evidence (`🗑️generated/box-shell/`): `served-wasm-sha256.txt`, `republish-brep.txt`,
`law-native.txt`, `law-wasm32-wasip2.txt`, `kernel-suites-native.txt`, `regression-native.txt`,
`freshness-after-republish.txt`, `runtime-probe-run.txt`, `runtime-after-republish/`.
