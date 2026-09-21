# 📓️ play-stdio — stdio in the play grid (session 5, 2026-09-21)

Topic report (tracked; scratch, logs and screenshots under `🗑️generated/play-stdio/`).

## Result

`semio-tech play`'s unit suite is **62/62 green** (was 58/62). All four reds were the missing stdio
coverage and all four are gone. The nine stdio editor apps each have their own pane, all nine boot to
`data-shell-ready` on :6033 with **zero console errors and zero page errors**, and the landing grid
reports **69 apps across 9 groups**.

## The "unlinkable stdio component" blocker no longer exists

A predecessor had already bounded the shipped component: `default = ["plugin-root"]` →
`component-app-assembly` pulls exactly seven artifact crates (csv, tsv, txt, json, xml, md, html =
9 editor + 9 viewer apps); the 88-app `full-app-catalog` is library-only and unreachable from
`default`. The unit law "keeps stdio's component bounded to the app fleet it can link" already pinned
that and already passed.

Evidence the component links (WebAssembly **component** magic `00 61 73 6d 0d 00 01 00`, not a core
module `…01 00 00 00`), all read from disk today:
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_stdio.wasm` — 245.9 MB, 2026-09-21 10:46
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3c/wasm32-wasip2/wasm-release/semio_s_plugin_stdio.wasm` — 48.4 MB, 2026-09-21 13:58 (the peer's hub-bootstrap `wasm-release` run finished successfully)
- staged, jco-transpiled dev module `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🗄️stdio/` — complete (`🌉️bridge.js`, `🔣️.json`, `🛂️.descriptor.semio`, core wasm), 2026-09-20 14:41

**I ran no wasm build at all.** The `stdio` react-dev activation lane receipt already existed
(`…/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/stdio/activation/🔣️receipt.json`, 2026-09-20 03:02),
so no `activate-*-react-dev` lane run and no mutex slot was needed. Nothing was requested via
`🗑️generated/activate.request/`.

## Why NINE panes and not one

`🧪️tests/🧪️playpanecoverage/🟦️.ts` carries two laws that together force one pane per app:
- *reaches every editor app a plugin descriptor declares* — a pane whose registry row pins `app`
  reaches only that app; a row pinning none reaches every editor of the directory.
- *reaches every viewer app through its pane's editor⇄viewer switch* — it declares **every** viewer of
  a paned plugin directory but reaches only the boot app's viewer.

stdio's committed descriptor declares 18 apps (9 editor / 9 viewer, 1:1 by dialect coordinate). One
pane without `app` satisfies the editor law and fails the viewer law by 8; one pane with `app` fails
the editor law by 8. Nine app-pinned rows satisfy both — exactly how `block` (block2d/3d/5d) and
`puzzle` (2d/3d/5d) already ship several playgrounds from one crate.

## What landed (absolute paths)

1. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`
   8 new `[[package.metadata.semio.playground]]` rows. `variant = "stdio"` stays the markdown row (it is
   the only app of the fleet publishing an example and the lane play activates the component from); new
   `stdio-txt`, `stdio-csv`, `stdio-tsv`, `stdio-json`, `stdio-json-i`, `stdio-xml`, `stdio-xml-valid`,
   `stdio-html` on react **6211–6218** / wgpu **6311–6318** — the first contiguous free block
   (`🗿️taxonomy-validation` claims one global TCP namespace, and every port below 6211 is already a
   react or wgpu launcher of another variant).
2. Registry regenerated with `bun ./📜️script.ts generate` in
   `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`
   → "60 plugin crates, **76 playgrounds**, 54 framework packages" + `/Users/ueli/Documents/semio/.vscode/launch.json`.
   `check-generated` afterwards: "generated catalog and launch bytes are fresh."
   Nx now infers `activate-stdio-react-dev` and the 8 new lanes on `@semio-tech/framework-os-dev`
   (verified with `nx show project @semio-tech/framework-os-dev --json`).
3. `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`
   new group `documents` ("Documents", 9 panes, markdown first) between `knowledge` and `media`.
   No schema change, so no `schema generate` was needed; the Ajv law still passes.
4. `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/📋️project.json`
   `stdio` added to `prepare-dev`, `prepare-release` and `activate-dev` `dependsOn` (28 lanes now — the
   greedy cover picks exactly one stdio lane because all nine stdio panes share the same closure).
5. `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts`
   `stdio` added to `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` (its md editor declares no
   `setActiveExample` — same bucket as the 13 panes already listed) and the now-wrong
   "the one pane play shows of it" comment rewritten.

No other file was touched. `🪧️brand.ts` and `🟦️.tsx` needed no edit: pane specs, brands and grid
geometry all derive from the catalog (`playGridDimensions(69)` → 9 × 8 with a centred trailing row).

## Evidence from runs I made

| run | result | log |
|---|---|---|
| play unit, before | 58/62 (4 red, all stdio) | `🗑️generated/play-stdio/unit-run1.txt` |
| play unit, after the catalog+registry work | 60/62 | `🗑️generated/play-stdio/unit-run2.txt` |
| play unit, final | **62/62** | `🗑️generated/play-stdio/unit-final.txt` |
| `play prepare dev` | "Prepared play dev: 69 panes, 60 components" | — |
| `play activate` | "Activated play dev: 60 completed components for 69 panes" (merged union receipt now carries `stdio`) | — |
| static boot resolution (`🧪️probe-boot.ts`) | all 9 stdio variants resolve their app, 1 plugin, 0 dependency errors | — |
| headless browser, :6033 (recycled 14:21:59 via `serve-restart.request`) | 9/9 `data-shell-ready`, 9–12 s each, **0 console errors, 0 page errors** | `🗑️generated/play-stdio/probe/*.txt` + `*.png` |
| landing overview | 69 cards, "69 apps", 0 console errors | `🗑️generated/play-stdio/probe/landing.txt/.png` |

## What is left, and why

1. **No pane boots a curated example — every stdio pane opens its app's genesis document.**
   `stdio` (Markdown) shows a Text window with the single line `semio stdio.md.dsl v1`; `stdio-csv`
   shows a Table window reading "No data"; the others are the same shape. Two separate causes, both in
   stdio **guest Rust**, both needing a 243 MB stdio wasm rebuild + `describe` + re-activation:
   - No stdio editor declares `ActionDefinition::new("setActiveExample", …)` (verified: zero hits under
     `✏️s/🔌️plugins/🗄️stdio`), so `appSwitchesExamples` gates the picker off and the curated `demo`
     stays inert. Identical to the 13 panes already inventoried in `play-defaults`' `EMPTY-PANES.md`.
   - The committed descriptor `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔣️.json` (2026-09-21 11:19)
     declares **one** example in total (`demo` on `s.stdio.md@commonmark/*`), although the other
     subsets ship authored ones on disk — e.g.
     `🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/📚️examples/🎬️demo/{🟦️.ts,🦀️.rs}` — and mount them
     (`🗿️artifacts/📊️csv/🦀️.rs:207` + the `pub use … ::examples` re-export at line 328, versus md's
     top-level `pub mod examples` at `🗿️artifacts/📝️md/🦀️.rs:352`). Only md's reaches the manifest.
   I deliberately did **not** touch stdio guest Rust: the peer's hub bootstrap
   (26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END) is publishing this exact crate, holds the wasm mutex
   until ~15:00 and finished its `wasm-release` link at 13:58. Changing the crate now would invalidate
   that build. Hand this to whoever owns the stdio crate next.
2. **Pre-existing red, NOT caused by this work:** `@semio-tech/plugin-registry`'s
   `🧪️tests/📖️generated-projection/🟦️.ts` → "the generated-module host predicate is semantically
   identical to the projection one" fails with `live home: expected true to be false`
   (`🗑️generated/play-stdio/projection-test.txt`). `projectedHostPluginFilter`
   (`📇️registry/📖️catalog-view/🟦️.ts:28`) has an extra `if (variantRow?.app !== undefined) return false;`
   that `isHostPlaygroundFilter` (`📇️registry/🟦️.ts:56`) does not, so the app-pinned `home`/`space`
   rows of the host plugin `space` disagree. It came in with the 09-20 home/space work, not with stdio
   (stdio declares no `host` table, so both sides answer `false` for every stdio variant). Owner:
   whoever owns the space host rows; one of the two implementations has to adopt the other's clause
   and `🧫️fixtures/🏠️host-filter.json` has to gain a vector for it.
   The same run also reports the suite exceeding its 15 s budget — also pre-existing.
3. `bun ./📜️script.ts check` in the registry still reports repo-wide taxonomy-tree violations
   (stdio gltf test files, note subsets, block 2d schema, space demo example). All pre-existing; none
   mentions playgrounds or ports (`grep -c playground` on the log: 0).
4. The strict acceptance suite (`bun nx run @semio-tech/semio-tech-play:test-e2e`) now has 69 panes to
   boot; it is coordinator-owned and was not run here.
