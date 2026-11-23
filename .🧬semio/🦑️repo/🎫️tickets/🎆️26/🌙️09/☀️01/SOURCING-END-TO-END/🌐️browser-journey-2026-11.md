# 🌐️ Sourcing browser journey (2026-11-23)

## Status: BLOCKED at boot, journey not yet exercised against the real DOM

Polled up to the 60 min budget. Port 6081 came up (vite pid 11215, 200) but the sourcing plugin never became loadable.

## Probe evidence (`🗑️generated/probe/*-en-US.*`)
- beacon `error:sourcing`; body shows "Plugin Recovery / This program crashed." with Restart App / Disable Plugin; zero `[data-window-id]`, tables, canvases.
- Console: `http 404 /🔌️plugin-modules/🪵️sourcing/🔣️.json`, then `program module unavailable sourcing ... descriptor answered HTTP 404; ... extensions: no registry entry`.
- Root cause: the sourcing wasm and descriptor were never staged. `.cache/b/p/m/browser/*/modules/` still holds only `wfc`, `shard`, `vendor` (as of 06:2x). Activation (cargo wasm32-wasip2 build of `semio-hub-sourcing-component` plus transpile) had not finished; cargo/rustc were still compiling flow/stdio crates. This is an activation/build gap, not a Rust or TS app bug in sourcing.

## Files
- `probe-sourcing.ts`: loads page, waits for beacon, dumps console, page errors, failed requests, aria snapshot, DOM summary and screenshot.
- `sourcing-journey.ts`: 16 steps per language (en, de): boot faults, windows, ten kinds, grid canvas, add, set count, undo/redo, preview, remove, drag both ways, query/module/typology/min-availability filters, sort, Demo-Empty-Demo, reload persistence. Locators are role/label based (labels from `🗣️terminology/🦀️.rs`) but are UNVERIFIED guesses (stepper as spinbutton, module toggles as checkbox/button, selector as combobox/button, `data-window-id="sourcing-*"`). Adapt after the first boot with the probe's aria/dom dumps. The app exposes no known language switch; locales are driven by the browser locale (en-US / de-DE).
- `poll-6081.sh`: 30 s poller.

## Resume
Once `curl http://127.0.0.1:6081/🔌️plugin-modules/🪵️sourcing/🔣️.json` returns 200: run `bun probe-sourcing.ts`, fix locators, run `bun sourcing-journey.ts`.
