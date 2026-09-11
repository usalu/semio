# Unknown kind after wasm restage (2026-09-11)

## Runtime (this turn)

Playground `http://127.0.0.1:6018/?plugin=generation3d` was hung (LISTEN, HTTP timeout). Restarted `screen g3dreact`. Vite serves the restaged descriptor **220 930 B** and wasm **94 138 842 B** (2026-09-11 14:15), not the 09-10 23:44 copy.

Shared `🔍️browser-probe.ts` still crashes on `JSON.stringify` of BigInt after snapshots. Early settle at 17.8 s was pre-contributions (`flow.extension-not-contributed`, `contributed: []`).

`🐍️restage-eval-probe.mjs` polled 2 s ticks until unknown-kind or meshes:

| t | meshes | preview |
| --- | --- | --- |
| 11 s | 0 | `extension-not-contributed` |
| 13 s | 0 | host `setContributions` settles, deferred `flowEvalTick` |
| 36 s | 0 | `widgetErrors`: `unknown kind: brep.curve.polygon`, `math.vector`, `brep.solid.extrude` |

Host logs (same run):

- scoped pack **248 635** chars, `hasManifestJson: true`, `hasPolygon: true`
- `invokeExtension dispatch failed {extensionId: flow-extension-math, capability: evaluate, error: typed-operation failed: window-transient publication is retiring a rejected authority}`

Graph labels after the pack: Polygon / Vector / ExtrudeCurve (display catalogue resolved). Numbers evaluate. `evalLen=779`. `meshes=0`.

Evidence: `🗑️generated/restage-eval/eval.json` and `final.png`.

## Native tests are not the served shape

`set-contributions` unit tests call `test_support::lock()` → `flow_operators::installed()` which **registers linked brep/math installers**. The served guest **links nothing**. Catalogue assertions can pass from linked packs while `ContributedExtensionStub` never lands.

The unlinked law `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` is the real native analogue (meshes ≥ 1 in-process via `🔬️brep-extension`). It does not cover the host `invokeExtension` typed-operation / window-transient path.

`register_contributed_manifest` still `return`s on parse failure. `invalidate_for_flow_extension_registry` sweeps the neural cache and clears `eval_json` but does **not** take `previous_snapshot` / `previous_channels` — `FlowHost::evaluate_step` can then skip re-dispatch when the tree is unchanged.

## Next

1. Make set-contributions laws unlinked + `FlowHost::evaluate` → `PendingExtension`, not catalogue-only.
2. Fix host `invokeExtension` so evaluate does not publish through a window-transient authority retired by the pre-contribution `flow.extension-not-contributed` tick (`🏛️ShellHost` comment at pending_effects).
3. Clear incremental eval baseline on registry-generation invalidation.
4. Re-probe 6018 for `meshes > 0` and no `unknown kind`. Do not mark the goal complete.

Do not raise the 64-page / 262 144 B ingress ceiling. 512 MiB wasm budget untouched.
