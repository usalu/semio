# 🧭️ 1112 Audit Activation Scope (read-only, 2026-11-12)

Scope: which Rust packages and targets must compile for the mit-bestand demonstrator (`@semio-tech/mit-bestand-demonstrator`, label "Entwerfen mit Bestand") to activate and serve on 6029. Read-only: no source edits, no git writes, no builds, no cargo, no sub-agents.

Caveat: the generated catalog (`@semio-tech/s-dev-catalog-ts/generated/plugins`, `playgrounds`) is not on disk, and no activation receipt exists under `.cache/b/p/m/browser/prototype` or `♻️mit-bestand/🧺️demonstrator/dist`. The closure below is therefore derived from the hub/ext scripts, the component descriptors (`🧩️component/🔣️.json` `manifest.pluginId` and `dependencies`), and the 2026-09-16 memory note, not from a live run.

## 1. Hub components (13) and extension components (21)

Hub, from `1105b-tools/hub.sh` (workspace `🌎️hub/Cargo.toml`, `cargo check --target wasm32-wasip2 --keep-going -p <pkg>`). Manifest paths below are the package manifests under `🌎️hub/🧩️compositions/`:

| # | Package | Manifest (relative to `🌎️hub/🧩️compositions/`) |
|---|---|---|
| 1 | semio-hub-cad-component | `📐️cad/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 2 | semio-hub-demonstrator | `🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` |
| 3 | semio-hub-demonstrator-component | `🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 4 | semio-hub-energy-component | `🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 5 | semio-hub-fem-component | `🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 6 | semio-hub-flow-component | `🌊️flow/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 7 | semio-hub-gis-component | `🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 8 | semio-hub-playbook-component | `📖️playbook/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 9 | semio-hub-procedural-component | `🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 10 | semio-hub-process-component | `🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 11 | semio-hub-puzzle | `🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` |
| 12 | semio-hub-puzzle-component | `🧩️puzzle/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| 13 | semio-hub-sourcing-component | `🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml` |

Extensions (21), from `1105b-tools/value/ext.sh` (one manifest each, `cargo check --target wasm32-wasip2 --keep-going -p <pkg>`). Paths are relative to `✏️s/🔌️plugins/`:

- flow (9): `🌊️flow/🧩️extensions/{🏗️bim, 📐️brep, 📖️dictionary, 📃️list, 🧠️logic, 🧮️math, 🔤️primitive, 📝️text, 🖍️draw}/🧩️component/📦️packages/🦀️rust/Cargo.toml`
- playbook (1): `📖️playbook/🧩️extensions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml` (package `semio-s-plugin-playbook-procedural-component`)
- process (4): `🏭️process/🧩️extensions/{🧱️concrete, 🔩️metal, 🤖️robotic, 🪵️wood}/🧩️component/📦️packages/🦀️rust/Cargo.toml`
- cad (4): `📐️cad/🧩️extensions/{🏢️aec-building, 🔥️aec-building-energy, 🏛️aec-building-structure, 📐️spatial-shape}/🧩️component/📦️packages/🦀️rust/Cargo.toml`
- sourcing (3): `🪵️sourcing/🧩️extensions/{🪵️beams, 🧱️slabs, 🪟️windows}/🧩️component/📦️packages/🦀️rust/Cargo.toml`

Note: the 21 in `ext.sh` include `playbook-module-procedural`. The 2026-09-16 memory closure (28 = 8 plugins + 9 flow + 4 process + 4 cad + 3 sourcing) does not. Whether the playbook extension is in the current closure is not verified (the playbook plugin is not in the plugin-dependency closure, see section 3).

Gap: `stdio` is a plugin in the closure (see section 3), but its component `semio-hub-stdio-component` is in neither `hub.sh` nor `ext.sh`. It is not checked by any ticket script. The activation receipt gate would still fail on it if it is not compiled.

## 2. Host-side packages and checks

- Checked by a script: `1105b-tools/hostcheck.sh` (and `hostcheck/run-1107a.sh`), native `cargo check --tests` over `🧰️framework/Cargo.toml`, plus `hostcheck-pkgs.txt`: `semio-plugin-host` and 18 `semio-plugin-host-*` packages, `semio-plugin-runtime-ops`, `semio-plugin-instance-app`. `run-1107a.sh` also checks `semio-framework-artifact-reference`, `semio-os-store-link-schema`, `semio-os-store-child`. These are not part of the activation target.
- Not checked by any ticket script: `host-base`, the wgpu shell, and the `semio-os` host crate (no hit for `host-base` in the ticket tools; `wgpu` and `semio-os` only appear in replay/probe scripts, not as a compile check).
- `wasm32-unknown-unknown`: no source reference under `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` (grep with exclusions, no hit). No ticket script checks it. Note: the `🧰️framework/🛍️products/🦑️repo/.../🕸️graph/📥️inference/🟨️.mjs` path named in memory has no `wasm32` or `activate-` hit.
- What the activation target runs instead: `activate-prototype` (`♻️mit-bestand/🧺️demonstrator/📋️project.json`, lines 260-295) `dependsOn` `prepare-prototype` and `activate-lanes-prototype`. `prepare-prototype` `dependsOn` nine `@semio-tech/framework-os-dev:prepare-<variant>-react-prototype` (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen) and four wasm targets: `framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm`. `activate-lanes-prototype` `dependsOn` `framework-os-dev:activate-{energy,fem3d,generator}-react-prototype`. These wasm/browser builds are inferred by nx, not declared in any `project.json` on disk, so their exact triples and packages are not verified here.

## 3. Families in the demonstrator closure (yes/no, with evidence)

Plugin closure, computed by a read-only script over component descriptors from `demonstrator` and `procedural` (`dependencies` in `manifest`): cad, demonstrator, gis, procedural, process, puzzle, sourcing, stdio. Evidence: `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/🔣️.json` lists dependencies cad, gis, procedural, process, puzzle, sourcing; `🗄️stdio/🧩️component/🔣️.json` is reached through the plugin graph.

- stdio mp4/avi/mp3: YES for the stdio plugin core (in plugin closure). NO for the `stdio-media` extension (`🗄️stdio/🧩️extensions/🎵️media`, not in `ext.sh` or the memory's 20-extension set). Format code for mp4/avi/mp3 was not located.
- raster: NO (`🖨️raster` is not in the plugin closure; no `semio-hub-raster` in hub.sh). epw: NO (no `epw` hit in the stdio or raster trees).
- layout: NO (`📏️layout` not in closure; not in hub.sh/ext.sh).
- draw: plugin NO (`🖍️draw` not in closure). `flow-extension-draw` YES (listed in `ext.sh` and in the memory's 9 flow extensions).
- remodel mesh-export: NO (`📸️remodel` not in closure; `stdio-mesh` extension not in the 20-set).
- puzzle 2d/3d/5d: YES (`puzzle` plugin in the closure; `semio-hub-puzzle` and `semio-hub-puzzle-component` in hub.sh). Individual 2d/3d/5d subsets not verified.
- block 2d/3d/5d: NO (`🧱️block` not in closure; not in hub.sh).
- lowpoly: NO (`💠️lowpoly` not in closure; not in hub.sh).
- sequence: NO (`🎬️sequence` not in closure; not in hub.sh).
- space home: NO (`🪐️space` not in closure; not in hub.sh).

Pane roots: the 8 pane runtime variants (generation3d, koordinator, aggregator, aussuchen, bearbeiten, verfolgen, energy, fem3d; `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`) map to playground build targets that are not on disk, so their plugin ids could not be resolved. Only `energy` resolved to a descriptor. The demonstrator's own closure is taken from `demonstratorRuntimeComponentIds` (`📦️assets/🟦️.ts`), which needs the generated catalog.

## 4. Exact command sequence (as written in 1110 checklist and 1112 audit section 3)

Header for every call:

```
cd /Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; D="/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator"
```

1. Gates (all GO before freeze; current state NO per 1112 section 2):
   - `bash "$C/1105b-tools/value/gate.sh" <n>` then `cat "$G/value-<n>-gate.rc"` (want `hub=0 ext=0`)
   - `"$C/1105b-tools/prefreeze.sh"` (`bash`), channel-pin-code 25/25/25/25
   - `git grep -n -I -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' ':!.🧬semio'` (must be 0 apart from accepted census literals)
2. Freeze: `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-pre.txt" 2>&1; echo rc=$?`, then `printf 1 > "$G/freeze"; printf 0 > "$C/1105b-tools/slots.k"; printf 1 > "$C/1105b-tools/slots.wextra"`
3. Descriptors (before canary): `( nohup "$C/1105b-tools/slot.sh" W-act-150 "$C/1105b-tools/canary/describe.sh" act-150 energy > "$G/W-act-150.txt" 2>&1 < /dev/null & )`, `cat "$G/act-150.rc"`; then the same with `W-act-200 ... act-200 closure`, `cat "$G/act-200.rc"`. Then re-run gate.sh and wait for a graph pass.
4. Canary (energy, 6106): `( nohup "$C/1105b-tools/slot.sh" W-canary-act-2 "$C/1105b-tools/canary/activate.sh" canary-act-2 > "$G/canary-act-2.slot.txt" 2>&1 < /dev/null & )`, `cat "$G/canary-act-2.rc"`, `"$C/1105b-tools/canary/serve.sh" start`, `"$C/1105b-tools/canary/probe.sh" "http://127.0.0.1:6106/?plugin=energy"; cat "$G/canary-probe/run.rc"`, `"$C/1105b-tools/canary/serve.sh" stop; lsof -nP -iTCP:6106 -sTCP:LISTEN` (must be empty).
5. `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-go.txt" 2>&1`
6. activate-prototype: `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-act-160 env SEMIO_PROGRESS_DEADLINE_TRACE=1 SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype; echo \$? > \"$G/W-act-160.rc\"" > "$G/W-act-160.txt" 2>&1 < /dev/null & )`. Success: rc 0, `$D/dist/♻️activation/prototype`, three receipts under `.cache/b/p/m/browser/prototype/runtime/react/{generator,energy,fem3d}/activation/🔣️receipt.json`.
   - Conflict: the checklist uses `repo --tool=bazel run`, the 1107 memory and `serve.sh` use `repo run`. Not reconciled here.
7. Serve on 6029: `"$C/1105b-tools/serve.sh" start; echo rc=$?` (exit 4 missing receipts, 5 early exit, 6 not 200, 7 Vite race; pid `$G/serve-6029.pid`).
8. e2e: `"$C/1105b-tools/e2eready/run.sh" --check; echo rc=$?`, then `E2E_ACTIONS_ALL_LANGUAGES=1 "$C/1105b-tools/e2eready/run.sh" http://127.0.0.1:6029/ --panes generator,koordinator,aggregator,energie,aussuchen,bearbeiten,verfolgen,statik --languages en,de` (rc in `$G/e2eready/run.rc`).
   - Note: `energie` and `statik` in this pane list do not match the 8 pane variants in `🔣️.json` (`energy`, `generation3d` and no `statik`). Not reconciled here.
9. test-e2e (50 executions): `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-e2e-160 env SEMIO_TICKET_DIR=\"$C\" repo run @semio-tech/mit-bestand-demonstrator:test-e2e; echo \$? > \"$G/W-e2e-160.rc\"" > "$G/W-e2e-160.txt" 2>&1 < /dev/null & )`
10. Release build and teardown: `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-build-160 env SEMIO_LANGUAGE=en repo run @semio-tech/mit-bestand-demonstrator:build; echo \$? > \"$G/W-build-160.rc\"" > "$G/W-build-160.txt" 2>&1 < /dev/null & )`, then `"$C/1105b-tools/serve.sh" stop; printf 8 > "$C/1105b-tools/slots.k"; printf 2 > "$C/1105b-tools/slots.wextra"; mv "$G/freeze" "$G/freeze.done"`

Current state (1112 section 2): no gate is GO. `ext-ok-1107` is absent. `hub-ok-1107` is stale. `sync-155.rc` is 0, but Cargo.toml mtimes are newer than `lock-155`. Descriptors still say channel 24 (1 of 142 files at 25). The `repo` MCP was down in this session, so graph generation status was not re-checked.
