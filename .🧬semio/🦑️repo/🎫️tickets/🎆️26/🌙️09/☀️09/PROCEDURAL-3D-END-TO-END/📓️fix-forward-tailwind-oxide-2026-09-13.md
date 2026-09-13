# 🩹 Fix forward: `@tailwindcss/oxide` in the browser dependency optimizer (2026-09-13)

Lane: react playground on `:6018` (`?plugin=generation3d`). Outputs under `🗑️generated/fix-forward-tailwind/`.

## 1. What the defect actually was

The reported symptom was right, the traced cause was not. Diffing the two suspected files
(`git diff 8add1df147..HEAD -- 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts 🧰️framework/🔨️modules/🖱️ui/🎨️styling/💨️tailwind/🟦️.ts`)
shows the peer's `b2064cc237` only **renamed** `💨️tailwind/🎨️tailwind.config.ts` → `💨️tailwind/🟦️.ts` byte-for-byte and
repointed the theme's re-export. That module imports `@tailwindcss/typography`, whose closure
(`tailwindcss/plugin` → 201 bytes, no native binding) never reaches oxide — an esbuild metafile scan of
`💨️tailwind/🟦️.ts` returns 38 modules and zero oxide hits.

The real importer chain was found by running the serve under `DEBUG=vite:deps` and crawling the served module
graph over HTTP (`🗑️generated/fix-forward-tailwind/crawl.mjs`):

```
2026-09-13T04:54:28Z vite:deps new dependencies found: @tailwindcss/postcss
✘ [ERROR] No loader is configured for ".node" files: node_modules/@tailwindcss/oxide-darwin-arm64/…node

/🟦️.ts
 -> 📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx
 -> 📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx
 -> 📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
 -> ♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx
 -> 🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.mts        ← the PostCSS tool-config entry
 -> 🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts
 -> @tailwindcss/postcss → @tailwindcss/node → @tailwindcss/oxide → *.node
```

`⚛️footer.tsx` imported its React entry **extensionless** (`…/📦️packages/🟦️typescript/🟦️`). A peer landed the
PostCSS owner's package entry as `🟦️.mts` in that same directory at 2026-09-13 04:19 (tool-configuration-ownership
fixture, `packageEntryPath`). Vite's `resolve.extensions` default is `[".mjs", ".js", ".mts", ".ts", ".jsx", ".tsx", ".json"]`
— `.mts` sorts **ahead of** `.ts`/`.tsx`, so the extensionless specifier silently flipped from the browser entry
(`🟦️.ts`, which is what resolved during the last good boot at 2026-09-12 ~23:00) to the node-only PostCSS config.
`enhanced-resolve` on the same extension order reproduces the flip exactly, and only `@tailwindcss/vite` and
`@tailwindcss/postcss` reference oxide anywhere in `node_modules`.

## 2. Fix forward at the owning layer

1. **`♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx`** — both imports now name the extension
   (`…/📦️packages/🟦️typescript/🟦️.tsx`, the public React entry rather than the `🟦️.ts` test adapter it used to land
   on by accident). 2-line diff; the peer's `🟦️.mts` contract is untouched.
2. **Theme / Tailwind split per the taxonomy** — `🖱️ui/🎨️styling/🌓️theme/🟦️.ts` no longer re-exports `tailwindConfig`
   (and `🎨️styling/📦️packages/🟦️typescript/🟦️.ts` no longer re-exports its `default`). The theme barrel is
   browser-served through `@semio-tech/ui-styling`; the build-time Tailwind config stays behind its declared owner
   `🖱️ui/🎨️styling/💨️tailwind/🟦️.ts`, and the build-time PostCSS/Vite integration stays in
   `🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts`. No `optimizeDeps` exclusion was added. Nothing in the
   repo imports the `@semio-tech/ui-styling` default, and the peer's behaviour (the config itself) is unchanged.
3. **Peer contract updated, not reverted** — `🦑️repo/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts` asserted
   `themeModule.default === tailwindModule.default`. That assertion *is* the leak, so it now asserts the opposite
   boundary (the browser theme barrel carries no Tailwind config) with a comment naming the 504. Every other
   assertion of that suite is untouched; `9 pass / 0 fail`.

## 3. Language-neutral guard

New fixture `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🌐️browser-graph.json` (entry, alias map,
`resolveExtensions`, `extensionAlias`, `denyPackages`, `denyModules`, `requireModules`, `ambiguousDirectories`) plus a
`describe("browser entry module graph")` block appended to `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts`, following the existing
`vite config module graph` / `dev server watch policy` pattern in that file.

- Ours: a BFS of the browser entry's relative + aliased closure under the declared resolve order, recording every
  bare specifier as a leaf edge (exactly what Vite hands its optimizer).
- Third-party twins: **esbuild** re-walks the same entry under the same extension order and alias map and must agree
  on the denied modules/packages (and must not reach a module our walk missed); **enhanced-resolve** must agree
  file-for-file on every relative specifier we resolved, and independently proves the `🟦️` → `🟦️.mts` hazard.
- Teeth check: re-introducing the extensionless import in `⚛️footer.tsx` fails **4 of the 7** new tests with
  `🟦️.mts is browser-served …` and `@tailwindcss/postcss ← …/🛠️build-tooling/🎨️styling/🟦️.ts`. Restored → 7 pass.

## 4. Test runs (all foreground, quoted verbatim)

| target | result |
| --- | --- |
| `🧑‍💻dev` vitest, `-t "browser entry module graph"` | `Tests 7 passed | 154 skipped (161)` — `[DEBUG] browser entry graph: 466 modules, 40 bare packages, 5 unbuilt artifacts` |
| `@semio-tech/ui-styling:test-quick` (`bun ./📜️script.ts test quick`) | `45 pass / 0 fail`, 1440 expect() calls |
| `@semio-tech/repo-lib:test-tool-configuration-ownership` | `9 pass / 0 fail`, 172 expect() calls |
| `@semio-tech/framework-renderer-react` `test quick` | `Test Files 1 passed (1) / Tests 5 passed (5)` |

### Red I did not cause (not bisected away, attributed)

- `🧑‍💻dev` `🧹️config` full file: `3 failed | 34 passed (37)`. All three are in the **pre-existing** describes —
  `vite config module graph > stays inside the declared module and source-byte bounds` (`expected 57 to be less than
  or equal to 40`: the *config* graph grew under peer churn, none of my files are in it, the deny/require tests still
  pass), `agrees with Bun's independent bundler …` (esbuild sees two modules bun does not), and
  `dev server watch policy > replays an atomic save …` (macOS `add` vs `change`, the flake its own comment documents).
- `@semio-tech/ui-react` `test quick`: `13 failed | 718 passed (731)` — icon keyframes / celebrate CSS / UIDialog
  focus / Diagram force, plus `ENOENT … /semio/🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json` (a peer's
  broken relative fixture path). None of these tests touch `tailwindConfig` or the `@semio-tech/ui-styling` default.

## 5. Boot on `:6018`

Serve restarted with the ticket's own script (`screen -S g3dreact -X quit; screen -dmS g3dreact 📜️serve-generation3d-react-direct.sh`);
`🗑️generated/s13-serve-react-direct.txt` now contains **no** `ERROR`, no `No loader`, no `error while updating dependencies`.

First probe after the optimizer fix surfaced a second, independent blocker that the 504s had been masking:
`ReferenceError: Cannot access 'uiLocale' before initialization` in `FrameworkOsShellInner`. In
`🏛️ShellHost/🟦️.tsx` the `spacePrograms` `useMemo` dependency array read `uiLocale`/`uiTerminology` at line 2093 while
the `const { … uiLocale, uiTerminology … } = shellState.uiPrefs` destructure sat at line 2142 — a temporal-dead-zone
throw on first paint, present identically at `HEAD` (so a peer regression between the last good boot and now, not the
working tree). Fixed by hoisting that one destructure next to the sibling `shellState.pluginRuntime` destructure at
line 1951, with a comment stating why; no name collides in between.

Measured, `SEMIO_PROBE_SECONDS=60 SEMIO_PROBE_OUT=fix-forward-tailwind/boot bun 🐍️console-dump-probe.mjs` and a
90 s `surface-probe.mjs` that counts HTTP 504s directly:

```
504 responses: 0
{ "surfaces": [], "windows": ["procedural-main", "procedural-preview"], "canvases": 0 }
```

- **0 × 504** — at the HTTP level and in the console dump (`grep -c 504 boot/console.txt` → `0`). ✅
- **Both windows mount** — `procedural-main` and `procedural-preview` are in the DOM. The shell boots, the plugin
  activates, and contributions publish (`contributions publish … {"status":"installed","chars":248635, 16 kinds}`). ✅
- **`data-surface-id` / preview `meshes`: NOT reached — red, not mine.** No surface host ever mounts because the
  wasm guest wedges: `shard 0 terminated by the host watchdog: the worker was silent for 23922 ms; outstanding:
  turn procedural#1 started 23951 ms ago`, then the same for `procedural#2` at ~50 s — a guest turn that never
  yields, in a restart loop. That is the staging/guest lane (this brief forbids restaging wasm), and a peer's own
  probe run in this ticket at 07:16 today (`🗑️generated/boot-check-11/console.txt`, `hosts.json` = `[]`) records the
  identical watchdog kill on the same server. The serve log also still lists `[stale] … unactivated/unstaged` for
  the extension and sourcing modules, pointing at the pending activation receipt.

## 6. Files touched

- `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🌐️browser-graph.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (TDZ hoist only)
