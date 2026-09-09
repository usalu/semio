# Wave 1 — procedural 3d react boot report (2026-09-09)

**Result: `http://127.0.0.1:6018/?plugin=generation3d` SERVES (HTTP 200).**
Evidence line (19:37:43 CEST):

```
$ curl -s -o /dev/null -w '%{http_code}' 'http://127.0.0.1:6018/?plugin=generation3d'
200
```

Lane: `bun nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev` under `nohup`
(`$S/boot-react.sh`), shared default cargo target dir, env
`CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000
SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react`.
Final log copied to `🗑️generated/boot-react.txt`; failed attempts kept in the scratchpad as
`boot-react-attempt{2,3,4,5}.log`.

## 1. Timeline

| time (CEST) | attempt | outcome |
|---|---|---|
| 17:56 | 2 (adopted from coordinator) | started; 33/39 tasks green |
| 18:03 | 2 | **failed** — `flow-plugin:component-dev` (2 compile errors), `flow-extension-bim-rust:materialize-dev`, `flow-extension-draw-rust:materialize-dev` (descriptor probe panic) |
| 18:07–18:11 | — | diagnosed + fixed F1, F2, F3 (below) |
| 18:11 | 3 | started; flow-plugin, bim, draw now green |
| 18:29 | 3 | **failed** — `flow-extension-dictionary-rust:component-dev`: `semio-s-artifact-stdio-semio` 7 errors (peer mid-edit) |
| 18:34 | 4 | restarted after 5-min settle on peer file |
| 18:44 | 4 | **failed** — same crate, `align_pcurve_branch` not found (peer still mid-edit) |
| 18:53 | 5 | restarted after 6-min settle |
| 19:03 | 5 | **failed** — `puzzle-plugin:wasm`: `planar_pcurve_sense` not found in a sibling peer file |
| 19:13 | — | tried a standalone `cargo check -p semio-s-artifact-stdio-semio --target wasm32-unknown-unknown` as a cheap gate; **not a valid oracle** (fails on `semio-framework-ui` `web_sys` feature-gating outside the lane's own feature set) — abandoned |
| 19:17 | 6 | restarted |
| 19:35:59 | 6 | `Activated generation3d react dev: 11 completed components (changed)` |
| 19:36:01 | 6 | `VITE v7.3.6 ready in 788 ms` — `Local: http://127.0.0.1:6018/` |

Attempt 6 ran all 39 dependency tasks green: 3 generators, `plugin-registry:generate` +
`session-generation3d`, `flow-core:wasm`, `infinite:font-tool`/`fonts`, `surface-rs:wasm`,
`editor-rs:wasm`, `puzzle-plugin:wasm`, `framework-plugin-web:support-dev`,
`component-dev`+`materialize-dev` for all 11 closure crates, then `prepare` → `activate` → `serve`.

## 2. Fixes made in this lane

### F1 — `semio-s-artifact-flow-flow` E0425: `_context` never renamed
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:858` +
use site `:898`.

`ArtifactCommandWork::extent` declared the parameter `_context` while the body did
`let context = context?;`. A dangling half-edit from the pre-reboot extension round-trip lane
(file mtime 14:29, untouched 3h38m — nobody mid-edit). Renamed the parameter to `context`.

### F2 — `semio-s-artifact-flow-flow` E0423: unit-struct literal fallout
Same file, `:1631`.

```rust
FlowCommand::FlowEvalResolve(command) => flow_eval_resolve::handle(command, &view, &ConfigView { snapshot: &NoConfig, window: None }, session),
```

`NoConfig` is `pub struct NoConfig {}` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9000`),
so `&NoConfig` is a type path, not a value. Changed to `&NoConfig {}`. Same class as the known
struct-literal codemod fallout.

### F3 — bim + draw descriptor probe abort: registry dropped outside a cold boundary
`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs:608,615` and
`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs:671,678`.

Symptom (both crates, `materialize-dev`):

```
Plugin descriptor failed for flow-extension-bim:
thread '<unnamed>' (1) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:101:13:
final Dictionary ownership must be explicitly retired or owned by a cold boundary
RuntimeError: unreachable
```

Root cause: `Dictionary`'s `Drop` (`…🧠️neural/⚙️engine/🦀️.rs:98-104`) aborts whenever a **sole-owner,
non-empty** dictionary is dropped without going through explicit retirement —
`OrderedMap::release_shared` (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:136`) hands back an
`Err(Retirement)` in exactly that case. The escape hatch is `neural_engine::ColdOwner`
(`…⚙️engine/🧊️cold/🦀️.rs`), whose `Drop` runs `ColdRetire for Registry` → `ChannelSpec` →
`Value` → `Dictionary` drains.

The `extension_guest` modules of **bim** and **draw** were the only two of the eleven closure crates
still passing a bare `&module_registry()` temporary into `build_manifest_json` /
`evaluate_invoke_json`; every peer (list, math, dictionary, text, primitive, logic, brep, core) already
wrapped it. bim additionally carries `ChannelSpec::…with_default(Value::Dictionary(text_dictionary(default)))`
(`🏗️bim/🦀️.rs:114`) — the only `Value::Dictionary` channel default in the whole extension set — which
is what made the dictionary non-empty and the abort reachable.

Fix (4 call sites): `&module_registry()` → `&neural_engine::ColdOwner::new(module_registry())`,
matching the peer pattern exactly. Both crates' `component-dev` + `materialize-dev` are green from
attempt 3 onwards.

**Not a framework bug** — the `Dictionary::drop` invariant and `ColdOwner` were already committed
(`025ec86a42`, 2026-09-08 20:58, file untouched since Sep 8 19:51); bim and draw had simply never
been rebuilt against it (their previously staged wasm dated 2026-09-08 17:34 / 2026-09-06 09:15).

## 3. Peer breakage waited out (not edited by this lane)

All in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/**`,
owned by this ticket's live boolean-kernel agent. Per the concurrency rule these were observed,
timed, and waited out rather than repaired:

| attempt | error | file:line | resolution |
|---|---|---|---|
| 3 (18:29) | E0425/E0433 ×7: `Curve2`, `Vec2`, `closest_uv` not in scope | `🔀️boolean/🦀️.rs:1014,1017,1077` | peer added `use …snapshot::curve::Curve2;` (:49), `…vector::{Pnt2,Pnt3,Vec2,Vec3}` (:56), `…surface_ops::closest_uv` (:52) at 18:29:37 |
| 4 (18:44) | E0425: `align_pcurve_branch` not found | `🔀️boolean/🦀️.rs:1296` | peer added `fn align_pcurve_branch` (:1339) by 18:47:32 |
| 5 (19:03) | E0425 ×2: `planar_pcurve_sense` not found | `✂️intersect/🏄️surface-surface/🦀️.rs:106,111` | peer added `fn planar_pcurve_sense` (:152) by ~19:09 |

Gate used before each restart: newest `*.rs` mtime in the subtree older than 5–7 min **and** the
named missing symbol resolvable by grep.

## 4. Staged artifacts (profiled react tree)

Root: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/`

| dir | file | bytes | mtime |
|---|---|---|---|
| `🌀️procedural` | `semio_s_plugin_procedural_component.core.wasm` | 78,698,225 | 09-09 19:35 |
| `🌀️procedural` | `🛂️.descriptor.semio` | 235,626 | 09-09 19:35 |
| `🌀️procedural` | `🌉️bridge.js` | 9,447 | 09-09 19:35 |
| `🌀️procedural` | `🔣️.json` | 1,043,921 | 09-09 19:35 |
| `🌊️flow` | `semio_s_plugin_flow_component.core.wasm` | 92,919,260 | 09-09 19:25 |
| `🧊️flow-extension-brep` | `…brep_component.core.wasm` | 22,719,850 | 09-09 19:30 |
| `🧊️flow-extension-brep` | `🛂️.descriptor.semio` | 82,669 | 09-09 19:30 |
| `🏘️flow-extension-bim` | `…bim_component.core.wasm` | 15,637,751 | 09-09 19:27 |
| `🎨️flow-extension-draw` | `…draw_component.core.wasm` | 16,888,959 | 09-09 19:32 |

The remaining six extensions (`list`, `dictionary`, `text`, `logic`, `primitive`, `math`) staged in
the same 19:30–19:33 window. Every one of the eleven is dated **today 19:25–19:35** — the audit's
Sep 6–8 staleness (and `forms`' three-week-old wasm) is gone. Note the closure resolved to
`flow` + 9 flow-extensions + `procedural`; `forms` was **not** in the actual Nx dependency set, and
a `flow-extension-core` target appears that the audit did not list.

Activation receipt (all 11 with `sha256` + `rebuiltAt=1788975359512`):
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/generation3d/activation/🔣️receipt.json`
(1,672 B, 19:35). Sibling `…/generation3d/extensions/` holds the nine extension staging dirs.
Receipt `procedural` entry: `artifactSha256 = ed802e29ecb3ce80547cd3c977903dc9c6b3f799efdff8f2fc7073b365095c30`.

## 5. Freshness proof

Served URLs (all 200, byte sizes identical to the staged files):

```
200      2626 B  /?plugin=generation3d
200      9447 B  /🔌️plugin-modules/🌀️procedural/🌉️bridge.js
200    235626 B  /🔌️plugin-modules/🌀️procedural/🛂️.descriptor.semio
200  78698225 B  /🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm
200     49448 B  /🔌️plugin-modules/🧊️flow-extension-brep/🌉️bridge.js
```

Content freshness — `flowTessellateResolve` was added to the generation3d editor **today**
(`…✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs`, mtime 09-09 15:00, i.e. after the last
commit `599a5d8450` at 07:58):

```
staged wasm   : strings … | grep -c flowTessellateResolve  → 2
staged wasm   : strings … | grep -c flowEvalResolve        → 3
served wasm   : curl … | strings | grep -c flowTessellateResolve → 1   (>0; stream-chunked)
served descr. : curl … | strings | grep -c flow-tessellate-resolve → 1
```

Byte identity between disk and wire:

```
staged sha256 = daf0cf6070904d1e06b541dc3dd69903f594efbb6d6d0a4d1f38587c70434cf6
served sha256 = daf0cf6070904d1e06b541dc3dd69903f594efbb6d6d0a4d1f38587c70434cf6
descriptor    = a24a784dc82dd2cf9ab78b0ecbb8d2b29a7a07c198e947f1e74d11ab7c71d1c8
```

So the wasm on the wire is the one built at 19:35 and it contains the continuation route added at
15:00 — not a stale artifact.

## 6. Serve process tree

```
66860  66854  bun nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev
 66861 66860  bun …🚀️bootstrap/📜️script.ts nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev
  66862 66861 node node_modules/nx/dist/bin/nx.js run @semio-tech/framework-os-dev:serve-generation3d-react-dev
   91682 66862 bun ./📜️script.ts serve generation3d react dev
    91694 91682 bun node_modules/.bin/vite --configLoader bundle --config …
```

Unrelated peer vite at pid 79274 (different parent 77049) — not this lane's.

## 7. Open issues

1. The lane is a foreground-of-nohup `serve`; killing pid 66860 orphans nothing dangerous here, but
   the usual rule applies — kill by pid after checking ancestry, never by name.
2. `nx` reports `exited with code 130` on every failed attempt regardless of the real cause; the
   actual failure is only in the `Failed tasks:` block, not the exit code.
3. A standalone `cargo check -p semio-s-artifact-stdio-semio --target wasm32-unknown-unknown` is
   **not** a usable pre-flight gate for this lane — it dies in `semio-framework-ui`
   (`🎯️targets/🧊️wgpu/📦️prepared.rs:3232`, `web_sys` unresolved) because it does not reproduce the
   lane's per-manifest feature/target-dep set. Anyone wanting a cheap gate must replay the lane's own
   `native component dev --manifest <crate Cargo.toml>` invocation instead.
4. Runtime behaviour of the page (console, windows, 3D preview) was **not** verified here — the
   coordinator owns the browser pane. This report proves staging + serving + artifact freshness only.
5. The boolean-kernel agent was still editing the brep diff tree at 19:09; a later edit there will
   not affect the already-served artifacts but will invalidate the Nx cache for the next boot.
