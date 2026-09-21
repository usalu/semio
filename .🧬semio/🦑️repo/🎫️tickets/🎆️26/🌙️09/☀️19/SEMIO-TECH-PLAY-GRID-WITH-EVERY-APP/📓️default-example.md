# 📓️ Wrong Default Example (`wfc3d`, `gis2d`) — topic `default-example`, 2026-09-21

## Verdict: NO DEFECT. `📓️audit-visual.md` §3 "Wrong default example shown" is RETRACTED — both panes boot exactly the curated example.

Both `wfc3d` and `gis2d` select the catalog's curated example on a cold boot. The audit's `MISMATCH`
verdict is a false positive of its own check, which word-matched the pane's **kebab-case example id**
against the **rendered English label** of the navbar trigger. Example labels are authored prose, not the
id spelled out, so the two panes whose curated example has a label unrelated to its id were reported wrong.

## Root cause of the false verdict

`📓️audit-visual.md` "Methodology notes": *"`default-example-ok` uses the navbar trigger's rendered label
text (`#playground.navbar.fixture`), matched word-wise against the catalog's kebab-case `example` id"*.

The trigger's text is the **label** of the selected row, by construction:
`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx:53-55`
("the trigger's own text is the active example's label and nothing else") and `:77-80`
(`SelectValue` inside `SelectTrigger id={id}`). The label comes from the plugin descriptor and, for play
(terminology `native`, locale `en` — `🏢️semio-tech/🎡️play/🪧️brand.ts:14,17`), is `label.native.en`:

| pane | curated id | `label.native.en` the navbar renders | id title-cased (what the audit expected) |
|---|---|---|---|
| `wfc3d` | `tower-stack` | **Tower With A Cantilever** | "Tower Stack" |
| `gis2d` | `demo` | **Reuse Map** | "Demo" |

Sources: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🔣️.json:23420` and
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🔣️.json:6554`; authored at
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🗼️tower-stack/🦀️.rs:20`
and `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs:7`.

Every hypothesis the brief listed was checked and ruled out:

- **Curated id not declared by the descriptor** — ruled out. Reading the two descriptors the way the shell
  does (app of the registry row, examples of the app's dialect coordinate, manifest order) gives
  `s.wfc.wfc3d@1/*#editor` → `two-room-corridor`, `wall-roof-facade-strip`, **`tower-stack`** and
  `s.gis.gismap@1/*#editor` → **`demo`** (its only example). Both curated ids are declared, and both apps
  declare `setActiveExample`.
- **Brand default ignored** — ruled out, and positively disproven for `wfc3d`: `tower-stack` is the LAST of
  three published examples, so `resolveBootExampleId`'s fallback
  (`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:292-301`,
  `return exampleOptions[0]?.id`) would have shown "Two Rooms And A Corridor". The browser shows the
  third row checked, i.e. `defaults.exampleId` from `playPaneBrand` (`🪧️brand.ts:76-78`) won.
- **Persisted last-used example winning** — ruled out twice over: the probe used a FRESH browser context
  (localStorage held only `ui.chrome.locale` and `semio.os.hub-device-instance.v1`, no shell state), and
  play's brands are `ephemeral: true` (`🪧️brand.ts:77`), so no shell state is durable at all.

## Browser evidence (runs I made, 2026-09-21 ~14:50, `:6033`, untouched)

Probe `$T/🗑️generated/default-example/probe.mjs` — repo playwright,
`PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, chromium headless
`--use-angle=metal --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`, 1440×900, ONE page at a
time, a FRESH browser + context per variant, `http://127.0.0.1:6033/#<variant>`, wait for the pane's own
`data-shell-ready`, then settle 6 s, then read the trigger and open the dropdown.

```
wfc3d  outcome=ready  trigger="Tower With A Cantilever"
       options: "No example" unchecked | "Two Rooms And A Corridor" unchecked
                | "Wall And Roof Facade Strip" unchecked | "Tower With A Cantilever" CHECKED
       localStorage keys: ui.chrome.locale, semio.os.hub-device-instance.v1 ; sessionStorage: []
gis2d  outcome=ready  trigger="Reuse Map"
       options: "No example" unchecked | "Reuse Map" CHECKED
       localStorage keys: ui.chrome.locale, semio.os.hub-device-instance.v1 ; sessionStorage: []
```

`data-state="checked"` on the curated row (and NOT on `exampleOptions[0]` for `wfc3d`) is the direct
proof that the brand default, not the first-example fallback, decided the boot example.

No dev server was started, stopped or recycled: nothing served changed (the only edit is a test file).

## Fix

**Nothing to fix in the catalog or the host.** The catalog ids are right, `resolveBootExampleId`'s
precedence is right, and the peer-owned `ExampleDefinition`/`ExampleSource` and ShellHost mount paths were
not involved, so no proposed diff is owed to a peer.

### The gate that already covers the brief's "wrong catalog id" branch, proven load-bearing

`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts` → `it("names only examples the pane's own app
publishes")` already fails on a curated id the descriptor does not declare. I proved it rather than
assuming it: temporarily setting `wfc3d`'s catalog `example` to `tower-stack-typo` turned the suite red
with `wfc3d: "tower-stack-typo" is not published by s.wfc.wfc3d@1/*#editor (descriptor:
two-room-corridor, wall-roof-facade-strip, tower-stack)` — 1 failed | 61 passed. The catalog was restored
byte-identical (verified with `diff`). So that branch cannot recur and needs no new law.

### The gap that DID let this happen, now closed

Nothing in the repo stated what text a curated example RENDERS, so the audit inferred it from the id and
was wrong for 2 of the 44 panes with a picker. Added to the same play unit gate
(`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts`):

- `publishedExamples` now also resolves each offered example's `label.native.en` (the exact text play's
  navbar renders, given `PLAY_LOCALE`/`PLAY_TERMINOLOGY`), documented in its docstring.
- New law `renders every curated example under the label.native.en a boot check reads instead of the id`
  — every curated example must publish that label, so a pane can never boot its picker on the bare
  "Example" placeholder.
- New module docstring `CURATED_EXAMPLE_LABEL_SOURCE` records the trap in the codebase itself: the picker
  shows authored prose, never the id spelled out; `tower-stack` → "Tower With A Cantilever", `demo` →
  "Reuse Map"; **a boot check compares label to label.**

Also proven load-bearing rather than assumed: temporarily blanking `"en"` on line 6554 of the gis
descriptor turned the suite red with `gis2d: "demo" publishes no label.native.en, so its navbar picker
would boot on the placeholder` — 1 failed | 62 passed. The descriptor was restored byte-identical
(verified with `diff`; `git status` clean for that path afterwards).

## Test evidence (runs I made)

`cd /Users/ueli/Documents/semio/🏢️semio-tech/🎡️play && DEVELOPER_DIR=/Library/Developer/CommandLineTools bun ./📜️script.ts test`

| run | result |
|---|---|
| baseline, before my edit | **4 files, 62/62 passed** |
| negative test, `wfc3d` curated id → `tower-stack-typo` | 1 failed / 61 passed (the inventory law) — restored |
| after adding the label law | **4 files, 63/63 passed** |
| negative test, gis `demo` label blanked | 1 failed / 62 passed (the new label law) — restored |
| final, after both restores | **4 files, 63/63 passed** |

## Files changed

- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts` — the only production
  change (test-side): `Published.labels`, the `label.native.en` resolution and docstring, the
  `CURATED_EXAMPLE_LABEL_SOURCE` docstring, and the new label law.
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📓️audit-visual.md`
  — one dated retraction line in §3, pointing here.
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📓️default-example.md`
  — this report.

Touched and restored byte-identical (both verified with `diff`, both clean in `git status` afterwards):
`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`, `✏️s/🔌️plugins/🌍️gis/🔣️.json`.

## Consequence for the audit's other findings

`default-example-ok` in `📓️audit-visual.md` compares the id to the label, so it reports `MISMATCH` for
every pane whose curated example is labelled with anything but its title-cased id. Only two panes hit that
in this catalog and both are retracted here. Its `OK` verdicts are far more likely to be sound (they
require the rendered label to word-match the curated id), but they are not proof either — a pane that fell
back to a different row whose label happens to word-match the curated id would also read `OK`. Re-running
the other 42 picker panes label-to-label was not done by this topic; the new play unit law now states the
id→label mapping for all of them, so that re-check is a pure lookup rather than a guess.
