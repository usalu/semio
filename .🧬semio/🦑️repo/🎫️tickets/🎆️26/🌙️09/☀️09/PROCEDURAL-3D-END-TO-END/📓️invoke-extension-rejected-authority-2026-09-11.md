# InvokeExtension window-transient rejected authority (2026-09-11)

## Live fault

`invokeExtension dispatch failed {extensionId: flow-extension-math, capability: evaluate, error: typed-operation failed: window-transient publication is retiring a rejected authority}`

Source (guest typed-operation publication loop in `🔌️plugin/🦀️.rs`): a window-transient publication in `Closing` with `fault()` set used to `Err` on every incomplete `close_step`, instead of yielding `Ok(())` like a healthy closing publication. Host `PluginRuntime` then wraps that guest string as `typed-operation failed: …`.

That aborts math `evaluate` after contributions re-arm `flowEvalTick`. WidgetErrors still show `unknown kind` from the pre-contribution pass mixed into eval JSON.

## Source now (15:35)

`publication is retiring a rejected authority` is **gone** from `🔌️plugin/🦀️.rs`. Window-transient closing now:

- incomplete `close_step` → `Ok(())` (continue retiring next turn)
- `Complete` + terminal empty → `Ok(())` and drop pending
- `Complete` without emptiness → still a fault

Served wasm is still **14:15** (94 138 842 B). Restage required.

## Not the catalogue miss

`flow-extension-math` evaluate was dispatched, so stubs/`PendingExtension` did fire for math. Display labels (Polygon/Vector/ExtrudeCurve) also resolved. Native `set-contributions` catalogue laws still use linked packs via `test_support::lock()`; the unlinked law `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` is the real guest shape.

## Next

Restage procedural wasm, reload 6018, re-run `🐍️restage-eval-probe.mjs`. Expect no `is retiring a rejected authority`, then `meshes > 0` or a new precise fault.
