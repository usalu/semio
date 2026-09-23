# 📓️ host-vendor — `GET /🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts.bin` → 404 on every pane

## 2026-09-23 05:35 — root cause + fix landed (source), awaiting :6033 recycle for proof

### Who requests it
`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` `defaultGuestSlimAssetFetcher` (≈l.2208): every activation registry
fetches the typst default font pack ONCE from `<plugin module url>/../🪞️vendor/🔤️guestslim-typst-fonts.bin`
and attaches it to the guest's `instance-open` event. So every pane that activates any plugin requests it.

### Where it is produced / supposed to come from
- Producer: Nx `semio-framework-os-infinite:fonts` (`♾️infinite/📦️packages/🦀️rust/📜️script.ts`) stages it to
  `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts/🔤️guestslim-typst-fonts.bin`
  (present: 8 757 072 bytes, 2026-09-23 02:45). The dev activation only VALIDATES it (`🧰️preparation`,
  `🏃️execution` digest); by design (`🧊️live-activation` law) it never copies fonts into the staging root.
- Release: `playRuntimeAssetSources` copies that dir to `🔌️plugin-modules/🪞️vendor` (owner `infinite:fonts`).
- Dev serve: play's Vite config mounted it as an OVERLAY static-dir at `/🔌️plugin-modules/🪞️vendor`.

### Root cause (not a missing build, not a union exclusion)
Play's dev Vite config (`🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`) registered TWO static-dir mounts on the SAME
route `/🔌️plugin-modules/🪞️vendor`: first the staging-root `🪞️vendor` dir (from `pluginModuleDirNames`, which
starts with `MODULE_VENDOR_DIRECTORY`), then the font dir. `createStaticDirMiddleware` (ui-styling builder)
deliberately answers 404 for a file missing under its route (never SPA fallback), so the first mount answered
404 for the font and the font mount was unreachable. Measured before the fix: `curl :6033/…/🔤️guestslim-typst-fonts.bin`
→ 404, `…/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js` → 200. It looked "new on this activation" because
the staging `🪞️vendor` dir was re-created at 02:42 (it previously also held a copy of the font from the old
`ensureGuestSlimTypstFontsAsset` build path, which masked the shadowing). The os/dev Vite config
(`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`) had the identical double mount.

### Fix (schema-first, laws in the owning packages)
1. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` — new `staticDirMountVitePlugins(repoRoot, specs)`:
   ONE route table over several static-dir roots; refuses a route claimed twice (`Two static-dir roots claim one
   route: …`), orders serve halves most-specific-route first (a parent mount otherwise swallows a nested one) and
   build copies parent first.
2. Fixture + schema: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/🗂️static-dir-mounts/🔣️.json`,
   `…/🗂️static-dir-mounts/🧬️schema/🔣️.json` (mounts declared parent-first adversarially, trailing-slash route,
   9 requests, 1 double-claim refusal).
3. Law `static-dir mount table` in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`: Ajv validates the
   fixture; real Vite dev server with the table vs the third-party oracle = native Vite `publicDir` over the
   union tree (byte parity on every 200, 404 exactly where the union lacks the file); build halves' copy equals
   the union; double claim refused. `bun test … -t "static-dir mount table"` → 1 pass, 21 expects.
   Negative control (nesting sort disabled) → fails on `preview2-shim/cli.js` 404; restored.
4. Play: `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📦️assets/🟦️.ts` new `playDevStaticDirMounts(workspace, extensionDir)`
   = exactly the release sources (`playRuntimeAssetSources(…, "dev")`) mounted in place (preview2 shim at its own
   nested route, fonts at `🪞️vendor`), extensions from their activated install dirs. Play Vite config uses
   `staticDirMountVitePlugins(repoRoot, playDevStaticDirMounts(repoRoot, extensionDir))` in serve mode.
   Law `🏢️semio-tech/🎡️play/🧪️tests/🧪️playdevmounts/🟦️.ts` (registered from the assets module; assets module added
   to play's vitest `includeSource`): 63 unique mounts, font pack + shim + plugin + extension served over HTTP
   through the real middleware chain, missing vendor file → 404. `bun ./📜️script.ts test 📦️assets` → 1/1 pass.
5. os/dev Vite config: same table, staging mount narrowed to `PREVIEW2_VENDOR_RELATIVE` (no browser-bundle file
   edited; only its constant imported).

Play unit suite: 70/72 — the 2 reds are `play pane example defaults` (flow boots "demo" by accident;
generation2d example unreachable), untouched by this change (peer/pane-defaults territory).
tsc over the changed files: no new errors (the play config's `OwnedBuildPlugin` vs Vite `Plugin` TS2769 and the
suite's older TS errors pre-exist; my lines are clean).

No re-activation needed (host Vite config only). The running chain `describe-activate-0923-0502-d` ends with a
:6033 recycle, which loads the new config; proof probe follows.
