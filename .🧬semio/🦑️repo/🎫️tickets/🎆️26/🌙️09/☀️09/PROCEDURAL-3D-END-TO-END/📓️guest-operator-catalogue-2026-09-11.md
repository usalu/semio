# Guest operator catalogue vs unknown kind

Date: 2026-09-11.

## Native law (this turn)

`set-contributions` unit tests now require the registry catalogue to contain `brep.curve.polygon` and `math.vector` after install — not only `flow_extension_invocation_address`.

Run: `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib set_contributions`

| Test | Result |
| --- | --- |
| `a_paged_run_installs_the_contributed_registry` (now asserts catalogue kinds) | **ok** |
| `a_one_page_host_shaped_run_indexes_contributed_operators` (JSON.stringify-shaped page 0 of 1) | **ok** |
| `the_packaged_brep_manifest_parses_as_a_flow_extension_manifest` | first-party `FromValue` succeeds; initial run panicked on `Dictionary` Drop (retired in a follow-up edit) |
| third-party `serde_json` of the same packaged brep `manifestJson` names `brep.curve.polygon` | yes |

The native guest path **does** register contributed stubs. `unknown kind` is not a host scoping miss and not a JSON.stringify vs ToValue miss.

## Served wasm

`semio_s_plugin_procedural_component.core.wasm` in the dev plugin-modules tree is dated **2026-09-10 23:44**. Host TS (example-scoped pack, deferred `flowEvalTick`) is live via Vite. The browser is still running last night's guest, which can install addresses and still evaluate `unknown kind`.

Restage: `bun nx run @semio-tech/procedural-plugin:component-dev`. Do not raise the 64-page / 262 144 B ingress ceiling.

## After restage (required)

Reload `http://127.0.0.1:6018/?plugin=generation3d` (do not restart 6018 unless it is down). Probe for:

- no `unknown kind: brep.curve.polygon`
- `invokeExtension` for `flow-extension-brep` / `flow-extension-math`
- `meshes > 0`

## Not done

Meshes, all 9 examples, generate, viewer, assembly, wgpu. Goal stays open.
