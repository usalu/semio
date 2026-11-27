# Fresh Draw Component, Descriptor and Staging — 26 November

## Verdict

A fresh `wasm32-wasip2` Draw component was linked from the current tree, described, materialized for the browser, prepared and activated. No Draw source change was needed. The registered `@semio-tech/draw-plugin:describe` target does not exist (`Unknown admitted build target`); the canonical producer is the component driver (`component --manifest <hub draw component Cargo.toml>`), which builds the cdylib, runs the descriptor emitter and materializes model plus browser module.

| Item | Value |
|---|---|
| Component SHA-256 | `75f75e49029cd9a90db2e51dc09bd17c17745ce69da4fed64eba89fcaaa94024` (164 798 435 bytes) |
| Semantic descriptor SHA-256 | `d1547e6294d0ff47d507cee7c0143001b3fe75e8ce9cd2d0fede5ccf07ea3884` (the `🛂️.descriptor.semio` bytes) |
| Descriptor JSON SHA-256 | `6023b1884a5af2fddfa0be7dec69d110b090d4d55e304fa20ac30e220dcd0da0` |
| Source tree digest | `958e6497f68679225bd5f95ce47c303ec6c779112444e21c2340e1537ca9eec7` |
| Toolchain | rustc 1.99.0-nightly (9f36de775bc6) |
| Native build | 911 units, 0 failures, 1685 s, report `.cache/b/r/m/components/runs/w5KmajH/r/report.md` |
| Model dir | `.cache/b/p/m/browser/prototype/components/285067b377def16ed6e8c388` |
| Browser module dir | `.cache/b/p/m/browser/prototype/modules/🖍️draw` (receipt identical to the model receipt) |

The regenerated pair is byte-identical to the committed pair in `🌎️hub/🧩️compositions/🖍️draw/🧩️component/` (`🔣️.json`, `🛂️.descriptor.semio`), so nothing under version control changed.

## Verification

- Draw registry gate (`verify-draw-registry-26.ts`, actual `validateDescriptors` on the generated Draw row): 1/1 crates own a verified pair, 0 errors, 1 warning (no shipped release component, expected for prototype). Draw-only; not a global catalog verdict.
- Receipt reconciliation (`verify-published-draw-receipt-26.ts`): wasm SHA and size equal the receipt, model receipt equals browser receipt, descriptor pair equals the committed pair, semantic SHA equals the receipt, deployed browser descriptor equals `finalizePluginDescriptor` of the Pack. Exit 0.
- Staging (`draw-stage-26.sh`): `support` ok, `prepare` ("1 of 1 components, session, browser support and 17 fonts"), `activate` ("1 completed components (changed)"). Activation receipt: `.cache/b/p/m/browser/prototype/runtime/react/draw/activation/🔣️receipt.json`.

## What went wrong on the way (no code defects)

1. First two driver runs: descriptor emitter prerequisite rebuild; one run failed with no compiler error (host SIGTERM under swap).
2. A peer's concurrent reactor refactor briefly left `semio_plugin_instance_app` calling `semio_plugin_reactor_app::maintenance_starved_mark` before the helper landed; the next run saw the function (E0425 gone on its own).
3. Repeated host-warden SIGTERM of `draw-member-decode` and `actor-input-wire` (1.3-2 GB rustc) while peers held 6-7 GB swap; the build owner retried internally and `component-retry-26.sh` wraps reruns.
4. `cannot update the lock file ... --locked` and `product lane ... is held` arose from a peer editing `Cargo.toml`/`Cargo.lock` and from overlapping own runs; resolved by killing overlapping retry loops and starting one run.
5. The repo MCP graph was unhealthy (`closure-missing`), so `show_project` and `run` could not plan; the native driver route was used (SEMIO_OFFLOAD=never, fleet CAS refuses with 507).

## How to serve (coordinator)

From `/Users/ueli/Documents/semio`, either use the launch entry `draw-react-native` in `.claude/launch.json` (preview_start name `draw-react-native`, port 6064) or run:

```sh
sh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/serve-draw-native.sh"
```

The script names its stage `serve-HHMMSS`, so the launcher's engine-log gating (exact stage `serve` only) does not apply and it starts at once. Port 6064 was free when staging finished. The serve registry pin is read at serve start, so serve after the activation above (done, 18:23).

## Files

New ticket scripts: `component-retry-26.sh`, `draw-stage-26.sh`, `verify-draw-registry-26.ts`, `verify-published-draw-receipt-26.ts`, this report. Logs under `🗑️generated/` (`draw-native-component-r1-w.log`, `draw-native-registry-w3.log`, `draw-native-receipt-w2.log`, `draw-native-{support,prepare,activate}-s26.log`); the earlier failed run logs `…c26*.log`, `…r*-x/y/z.log` are deletable.
