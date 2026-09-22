# 📓️ Play Runtime — activation-lane drift + acceptance strength (topic `play-runtime`)

## 2026-09-22 — session 6 (fleet v5)

TypeScript/bun only: no cargo, no wasm32, no `describe`/`activate`, no server start/stop/restart.
Scratch and logs: `🗑️generated/play-runtime/`.

## 1. Activation-lane merge no longer kills :6033 on peer re-activation

**Symptom it removes.** All 28 lanes stage into ONE `🔌️plugin-modules` root, so exactly one artifact per
component exists on disk. `mergePlayActivationReceipts` nevertheless refused the whole union whenever two
lane receipts disagreed about one component's `artifactSha256` — which is what a peer running
`activate-flow-react-dev` (or `gis2d`, or `demonstrator`) after the coordinator's full activation always
produces. The serve start calls `readPlayActivation` before Vite listens, so the refusal took the whole
:6033 down (three outages in two days; `📓️status.md` 01:45 "crash-looped again on the demonstrator/flow
lane sha conflict").

**Rule now.** A disagreement is resolved against the INSTALLED artifact:

- `playInstalledArtifactSha256(workspace)` recomputes, off the one staging root, the identity an activation
  of that component would record — `sha256(<support digest> + <component digest>)`, the identity
  `♻️activation/🏃️execution/🟦️.ts` publishes. Lazy and memoised: a fleet whose lanes agree never reads a byte.
- The row whose sha equals it wins; among equal rows the NEWEST receipt by mtime wins
  (`readPlayActivationLane` now carries `statSync(🔣️receipt.json).mtimeMs`); `rebuiltAt` still keeps the
  EARLIEST value among the agreeing rows, so a lane re-activating unchanged bytes never fakes a hot-swap.
- One warning line instead of a refusal:
  `[play] lane drift: <component> served from <lane> (<sha8>), stale in <lanes>` — printed by
  `publishPlayUnionReceipt` on the serve's own console, next to the `[stale] …` freshness lines, which
  still name the same lane through the extension install root of the lane that staged it.
- Only a disagreement where NO lane activated what is installed is still a refusal — then the served bytes
  genuinely have no receipt, and the message names every lane's sha and the installed one.

**Files.**
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts` — `🔏️PlayInstalledArtifact` region
  (`activationFilesDigestSync`, `playInstalledArtifactSha256`), `playServedActivationRow`,
  `mergePlayActivationReceipts(…, options)`, receipt mtime, warn-on-publish.
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playactivation/🟦️.ts` — six new laws (below).

**The digest is restated, never guessed.** `activationFilesDigest` lives in
`♻️activation/📥️installation/🟦️.ts`, is `async` and drags the whole materialization tool-chain in; play
publishes its union inside a Vite config factory and cannot await. The restatement is therefore pinned to
the framework's LIVE SOURCE by a law that extracts that function's body and executes it against the same
fixture (`new Function`, no module graph — importing the module costs 2.5 s in bun alone and blew the
suite's 15 s budget). Proven on real data as well:

```
$ bun 🗑️generated/play-runtime/check-merge.ts
lanes=28 components=60 variant=demonstrator ms=109
warnings=0
installed(animate) = e7f34229…369268ad  receipt = e7f34229…369268ad  match=true  ms=417
```

i.e. the sync digest reproduces the sha the 03:04 activation wrote, for a real component, in 417 ms
(support digest included), and today's 28 lane receipts carry no drift.

### New laws and their output

`cd 🏢️semio-tech/🎡️play && bun ./📜️script.ts test` (log `🗑️generated/play-runtime/unit-final.txt`)

| law | verdict |
|---|---|
| refuses lanes that disagree when nothing on disk decides between them (kept from before) | pass |
| says nothing at all when every lane agrees | pass |
| serves the lane whose row matches the installed artifact and names the stale ones | pass |
| breaks a tie between two lanes that both activated the installed artifact by the newer receipt, keeping the earliest `rebuiltAt` | pass |
| refuses a disagreement no lane activated, naming every lane and the installed artifact | pass |
| refuses a disagreement whose component is not staged at all | pass |
| digests staged files byte-for-byte like the framework activation does | pass |

## 2. Strict acceptance suite: visible content, not just `data-shell-ready`

`🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` — three assertions per pane on top of the boot beacon,
all HARD failures. `bun nx run @semio-tech/semio-tech-play:test-e2e` is unchanged (same runner, same
config, same 70 tests, `workers: 1` as before).

1. **The chrome names the curated example.** The navbar trigger `#playground.navbar.fixture` inside
   `[data-play-pane="<variant>"]` must render exactly the catalog's new `exampleLabel`. That field is the
   example's `label.native.en` — the exact prose the picker shows under play's `native`/`en` locks — and
   is pinned to the descriptor by a new unit law, so this compares LABEL to LABEL and never repeats the
   id-to-label guess `📓️default-example.md` retracted. A pane whose app declares no `setActiveExample`
   has no trigger and stays covered by the unit gate `names every pane whose curated example cannot reach
   its app`.
2. **The main window paints.** The largest canvas of the pane (≥ 10 000 px²) is screenshot by Playwright —
   never `drawImage` on the live context, which reads back blank for WebGL/WebGPU without
   `preserveDrawingBuffer` — decoded in-page and censused: 12-bit colours, pixels differing from the modal
   colour, outer border insetted. It must not be a single colour and must clear 0.1 % of its surface
   (floor 64 px). A pane with no canvas uses its largest `[data-slot="window-body"]`: non-empty text or
   ≥ 10 elements; a pane with neither fails with "the pane mounted no window".
3. **No asset answered by the SPA fallback.** Every response whose path ends in a media extension
   (`png|jpe?g|gif|webp|avif|svg|ico|bmp|glb|gltf|obj|stl|ply|3dm|mp3|mp4|webm|wav|ogg|woff2?|ttf|otf|pdf`)
   and whose `content-type` is `text/html` is collected and asserted empty, per pane and on the overview.

### Measured examples (all against :6033 serving the 2026-09-22 **03:04** activation — the new one was
### still in its describe phase, `🗑️generated/activation/describe-activate-0922-1121.txt`, no `activate-dev rc=`)

- `animate` **FAILS, correctly and newly**: `/%F0%9F%96%BC%EF%B8%8Fbauteilb%C3%B6rse.png → 200 text/html`
  — the missing deck figure the SPA fallback was answering with `index.html`, exactly the defect
  `📓️status.md` 04:10 found by eye. Log `🗑️generated/play-runtime/e2e-animate.txt`.
- `cad`, `raster`, `writer`, `stdio-tsv`, `block2d`, `architect`, `puzzle3d` pass all three.
- Negative proof of the label assertion (catalog `raster` label temporarily set to "Not The Demo",
  restored byte-identically): `Error: raster chrome must name its curated example demo` —
  `e2e-label-negative.txt`.
- Negative proof of the paint census (threshold temporarily 2 %, restored): `Error: raster main window
  paints nothing — canvas: canvas #0 909×628: 22 colours, 3351/561666 pixels differ from the modal colour
  (needs 11233); central 76 %: 1 colours, 0/362802` — `e2e-negative.txt`.
- Cost of the new work: 20 ms inventory + ~1.3 s screenshot/decode/census per pane (`probe-census.mjs`),
  i.e. ≈ +1.5 min on a 70-test run; the 03:55 run was 7.1 min.

### Why the paint census is a LIVENESS bound and not a content verdict

All 69 panes were censused on :6033 (`probe-batch.mjs`, `calibration-all.ndjson`, table below), at the
whole surface and again over the central 76 %. The strict variant (centre must not be one colour) reds
eight panes — `raster`, `puzzle2d`, `writer`, `mathematical`, `dag`, `trinity-jack`, `stdio-txt`,
`demonstrator` — and `puzzle2d`, `writer` and `stdio-txt` are CONTENT-verified in `📓️audit-visual-2.md`:
their content is simply not in the middle of their largest canvas. So the centre rule cannot be a gate.
Every pane that paints clears 0.59 % of its surface, which is why the shipped bound is 0.1 %: it can never
red a working pane, and a viewport that painted nothing has literally one colour. The centre figures ride
along in the failure message so an audit never has to re-measure them.

The five panes whose largest canvas is UNIFORM in the middle and that are known content defects or
expected-empty — `raster` (blank), `mathematical` (gated), `demonstrator` (expected empty), `dag`,
`trinity-jack` — are the standing work of the visual audits, not of this gate.

| pane | main window | surface colours | surface differ | centre colours | centre differ |
|---|---|---:|---:|---:|---:|
| `cad` | canvas 630×299 | 196 | 44.36 % | 147 | 43.93 % |
| `generation3d` | canvas 859×628 | 79 | 37.28 % | 78 | 35.53 % |
| `generation2d` | canvas 693×628 | 48 | 12.72 % | 48 | 12.18 % |
| `flow` | canvas 859×628 | 58 | 19.28 % | 48 | 15.96 % |
| `lowpoly` | canvas 1268×629 | 137 | 9.99 % | 21 | 6.20 % |
| `remodel` | canvas 884×629 | 112 | 6.46 % | 21 | 6.34 % |
| `draw` | canvas 1268×628 | 26 | 39.43 % | 6 | 31.40 % |
| `raster` | canvas 909×628 | 22 | 0.60 % | 1 | 0.00 % |
| `layout` | canvas 693×628 | 53 | 46.93 % | 40 | 46.07 % |
| `shooting` | canvas 858×629 | 151 | 15.74 % | 56 | 9.86 % |
| `puzzle2d` | canvas 628×629 | 19 | 1.94 % | 1 | 0.00 % |
| `puzzle3d` | canvas 418×629 | 8 | 5.36 % | 4 | 0.61 % |
| `puzzle5d` | canvas 757×629 | 106 | 12.78 % | 21 | 13.54 % |
| `block2d` | DOM body, 92 chars / 50 elements | – | – | – | – |
| `block3d` | canvas 1268×629 | 107 | 9.82 % | 22 | 8.39 % |
| `block5d` | DOM body, 52 chars / 50 elements | – | – | – | – |
| `wfc2d` | no window | – | – | – | – |
| `wfc3d` | no window | – | – | – | – |
| `grid2d` | canvas 630×628 | 67 | 27.40 % | 53 | 32.20 % |
| `grid3d` | no window | – | – | – | – |
| `bitmap` | canvas 630×628 | 43 | 27.48 % | 15 | 29.72 % |
| `fem2d` | canvas 630×628 | 53 | 20.48 % | 36 | 19.82 % |
| `fem3d` | canvas 630×629 | 111 | 6.71 % | 20 | 6.80 % |
| `energy` | canvas 693×629 | 114 | 2.86 % | 9 | 0.91 % |
| `process3d` | canvas 1268×629 | 119 | 6.70 % | 21 | 4.61 % |
| `sourcing` | canvas 413×629 | 118 | 4.65 % | 18 | 1.03 % |
| `gis2d` | canvas 1268×629 | 290 | 30.50 % | 258 | 32.31 % |
| `gis3d` | canvas 1268×629 | 112 | 49.26 % | 20 | 50.36 % |
| `architect` | canvas 374×628 | 28 | 9.63 % | 23 | 8.10 % |
| `din4108` | DOM body, 1886 chars / 395 elements | – | – | – | – |
| `din16798` | DOM body, 1680 chars / 497 elements | – | – | – | – |
| `din18599` | DOM body, 813 chars / 276 elements | – | – | – | – |
| `en1990` | DOM body, 1259 chars / 412 elements | – | – | – | – |
| `en1991` | DOM body, 668 chars / 259 elements | – | – | – | – |
| `en1992` | DOM body, 732 chars / 225 elements | – | – | – | – |
| `en1993` | DOM body, 1784 chars / 497 elements | – | – | – | – |
| `en1994` | DOM body, 506 chars / 191 elements | – | – | – | – |
| `en1995` | DOM body, 596 chars / 208 elements | – | – | – | – |
| `en1996` | DOM body, 597 chars / 208 elements | – | – | – | – |
| `en1997` | DOM body, 357 chars / 157 elements | – | – | – | – |
| `en1998` | DOM body, 782 chars / 276 elements | – | – | – | – |
| `en1999` | DOM body, 585 chars / 208 elements | – | – | – | – |
| `iso16757` | DOM body, 840 chars / 293 elements | – | – | – | – |
| `vdi3805` | DOM body, 3449 chars / 889 elements | – | – | – | – |
| `note` | DOM body, 61 chars / 111 elements | – | – | – | – |
| `writer` | canvas 1268×595 | 47 | 1.63 % | 1 | 0.00 % |
| `forms` | DOM body, 497 chars / 672 elements | – | – | – | – |
| `mathematical` | canvas 757×628 | 21 | 0.72 % | 1 | 0.00 % |
| `reasoning-wires` | canvas 1268×628 | 21 | 19.51 % | 2 | 18.78 % |
| `dag` | canvas 859×628 | 21 | 0.63 % | 1 | 0.00 % |
| `imperative` | DOM body, 35 chars / 60 elements | – | – | – | – |
| `playbook` | DOM body, 134 chars / 269 elements | – | – | – | – |
| `trinity-jack` | canvas 757×628 | 22 | 1.27 % | 1 | 0.00 % |
| `trinity-rewriting` | DOM body, 17 chars / 49 elements | – | – | – | – |
| `stdio` | canvas 1268×628 | 24 | 2.84 % | 20 | 0.42 % |
| `stdio-txt` | canvas 1268×628 | 24 | 0.71 % | 1 | 0.00 % |
| `stdio-csv` | DOM body, 69 chars / 67 elements | – | – | – | – |
| `stdio-tsv` | DOM body, 602 chars / 59 elements | – | – | – | – |
| `stdio-json` | DOM body, 163 chars / 351 elements | – | – | – | – |
| `stdio-json-i` | DOM body, 179 chars / 277 elements | – | – | – | – |
| `stdio-xml` | DOM body, 198 chars / 180 elements | – | – | – | – |
| `stdio-xml-valid` | DOM body, 213 chars / 200 elements | – | – | – | – |
| `stdio-html` | canvas 1268×628 | 24 | 1.34 % | 20 | 0.06 % |
| `animate` | canvas 1268×628 | 81 | 30.44 % | 46 | 27.91 % |
| `sequence` | canvas 628×628 | 24 | 8.84 % | 10 | 7.94 % |
| `vcs` | DOM body, 19 chars / 51 elements | – | – | – | – |
| `demonstrator` | canvas 1268×628 | 24 | 0.59 % | 1 | 0.00 % |
| `home` | DOM body, 76 chars / 58 elements | – | – | – | – |
| `space` | DOM body, 89 chars / 66 elements | – | – | – | – |

(`wfc2d`, `wfc3d`, `grid3d` show "no window": they never reach `data-shell-ready` — the known wasm trap at
app registration — so the acceptance suite already fails them on the boot beacon.)

## 3. Play unit suite

| run | result | log |
|---|---|---|
| baseline, before any edit of mine | 61 / 63 | `🗑️generated/play-runtime/unit-baseline.txt` |
| after the merge work | 67 / 69 | `unit-merge-2.txt` |
| final (merge + catalog `exampleLabel` + its two laws) | **69 / 71** | `unit-final.txt` |

The two reds are the SAME two I found before touching anything, and they are not mine:
`✏️s/🔌️plugins/🗄️stdio/🔣️.json` (mtime 2026-09-21 11:19) still publishes a single example (`demo` for
`s.stdio.md`), so the nine stdio panes' curated `demo` reads as "not published by
`s.stdio.txt@utf-8/*#editor` (descriptor: none)" and `stdio` reads as a plugin whose app cannot switch
examples. The nine examples exist in source (`…/📚️examples/🎬️demo/🦀️.rs`, all
`LocalizedLabel::native("Demo", "Demo")`); only a `describe` of `🗄️stdio` refreshes the descriptor, and
describe is the coordinator's (stdio is LAST in `📜️describe-serial-then-activate.sh`; at 11:32 the pass
was still on `animate`). Both laws go green by themselves the moment that descriptor lands — the catalog
already carries `"exampleLabel": "Demo"` for all nine panes, read from those sources.

## Files changed (absolute)

- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts` — installed-artifact
  identity + lane-drift resolution + warn-on-publish.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🧪️playactivation/🟦️.ts` — six new laws.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` — `exampleLabel` on all 52
  panes that curate an example.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧬️schema/🔣️.json` — `PlayExampleLabel`
  + `dependencies: example ↔ exampleLabel`.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts` — `PlayRuntimePane.exampleLabel`.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts` — two new laws pinning
  `exampleLabel` to the descriptor's `label.native.en` and to the presence of `example`.
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` — curated-example label,
  paint census, SPA-fallback asset guard.

## What remains

- The two stdio unit reds (above) — blocked on the coordinator's `describe` of `🗄️stdio`.
- `animate` is now a RED acceptance pane for a real reason (the missing `🖼️bauteilbörse.png`); that figure
  is `knowledge-children`'s open item.
- The full strict suite was NOT run by me (the coordinator's activation is in flight and owns that run);
  only single-pane `--grep` runs with one worker, against the 03:04 activation.

---

## 2026-09-22 16:00–16:45 — why 30 lanes read `[stale]` after the 15:42 describe pass

> `🗑️generated` was SWEPT at 16:35:58 (`🗑️generated/SWEPT-AT-16xx-marker.txt`) — every log the sections
> above cite is gone, including `serve-6033-supervised.txt`. The evidence quoted there was read and
> recorded before the sweep. Logs from 16:40 on are back under `🗑️generated/play-runtime/`.

### 1. A lane never copies the plugin descriptor — `materialize` re-emits it

`✏️s/🔌️plugins/<p>/🔣️.json` and `🛂️.descriptor.semio` are **the `describe` target's declared outputs**:

```
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json
  "describe": { …, "outputs": ["{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/🛂️.descriptor.semio",
                               "{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/🔣️.json"] }
```

The STAGED copy under `🔌️plugin-modules/<dir>/` is not that file. `materialize-<profile>`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:61-74`)
runs the descriptor PROBE against the component it just built, calls `finalizePluginDescriptor(pack,
pluginId, digest(artifact), digest(core.wasm))` and writes its OWN `🛂️.descriptor.semio` + `🔣️.json` into
the staging temp, then `stageArtifacts` them. Two independent emissions of the same descriptor.

**Nx inputs (checked, not assumed).** The plugin's Nx project is `…/📦️packages/🦀️rust`, and its
`namedInputs.default` is `{workspaceRoot}/✏️s/🔌️plugins/<p>/**/*.rs` + `{projectRoot}/**/*`.
`materialize-<profile>` (generated in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:880`) takes
`["production", "^production", {dependentTasksOutputFiles}, …web bundler sources…]`, and `component-<profile>`
takes `nativeSources`. So the plugin-root descriptor pair is **not** an input of either — deliberately, and
it must stay that way: it is the output of a target that consumes the same crate, so making it an input
would make every describe invalidate the component build it was produced from, forever. The pair is also
**tracked in git** (`git ls-files` → tracked, `git check-ignore` → not ignored), so the gitignored-inputs
blindness is not in play here.

### 2. The staged descriptors are NOT behind — the `[stale]` lines were a false alarm

Source (describe, 15:42) vs staged (materialize, 14:0x–14:23), parsed and compared key by key
(`🗑️generated/play-runtime/` scratch):

| plugin | differing paths | of which `hashes.*` | of which number formatting (`5.0` vs `5`) | **manifest differences** |
|---|---:|---:|---:|---:|
| `🗄️stdio` | 0 | 0 | 0 | **0** |
| `🏛️architect` | 7 | 3 | 4 | **0** |
| `🧩️puzzle` | 242 | 3 | 239 | **0** |
| `🌊️flow` | 8 | 3 | 5 | **0** |
| `📐️cad` | 3 | 3 | 0 | **0** |
| `🖨️raster` | 7 | 3 | 4 | **0** |

The only differences are the three `hashes.*` fields — which identify the component BUILD each emission was
made from, and the describe target builds its own — and JSON number formatting between the two emitters
(the describe side writes `5.0` where the materialize side writes `5`; 239 occurrences in puzzle). So the
served descriptors carry exactly today's manifests. Nothing needs restaging on the descriptor's account,
and **I did NOT touch `🗑️generated/activate.request/ALL`**.

### 3. The real defect: the freshness walk counted a generated artifact as a source

`newestComponentSourceMtime(sourceRoot)` walks the plugin owner tree for the newest file mtime, skipping
only `dist`/`target`/`node_modules`/`pkg`/`.git`. The describe outputs sit at the owner ROOT next to the
sources, so every describe made every component instantly "source-newer" — the ~30 `[stale]` lines at 15:57
name nothing but `✏️s/🔌️plugins/<p>/🔣️.json`.

**Fixed** in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`:
new exported `GENERATED_COMPONENT_OWNER_FILES = ["🔣️.json", "🛂️.descriptor.semio"]`, skipped by the walk
**only at the owner root** — a nested `🔣️.json` is a schema or a fixture and still decides freshness. A real
source edit still reports, because the `.rs` that changed is still walked. Content comparison was NOT the
fix: the two files are legitimately different encodings of the same descriptor (above), so comparing their
bytes would report a difference forever.

Fixture + laws (`🧫️fixtures/🔌️staging-root.json` `freshness.walk.generatedOwnerFiles`,
`🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`):

| law | verdict |
|---|---|
| treats the owner root's describe outputs as build output and a nested `🔣️.json` as source | pass |
| names exactly the two files the describe target declares as its outputs (pinned to `DESCRIPTOR_*_FILENAME` source text) | pass |

`bun 🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts test staging-root` → **48 passed (48)**
(`🗑️generated/play-runtime/osdev-staging-root.txt`). Proven load-bearing: with the one skip line removed the
first law fails `expected 9000000 to be 1000000` — the descriptor mtime shadowing the real source — and the
line was restored immediately (`osdev-staging-root-negative.txt`).

### 4. What IS genuinely stale (the same walk, with the fix)

`🗑️generated/play-runtime/true-staleness.txt` — **9 of 60** components, every one of them because a fleet
agent edited a `.rs` after the 14:0x materialize, none because of a describe:

| component | newest source |
|---|---|
| `cad` | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs` 14:03 |
| `playbook`, `playbook-module-procedural` | `📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` 15:56 |
| `process` | `…/🧊️process3d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` 15:59 |
| `puzzle` | `🧩️puzzle/🧪️tests/🔬️surface/🦀️.rs` 16:34 |
| `reasoning` | `…/🔌️wires/🧪️tests/🔬️unit/🦀️.rs` 15:58 |
| `sequence` | `…/🎬️sequence/…/✏️editor/🦀️.rs` 16:10 |
| `space` | `…/🪐️space/…/✏️editor/🦀️.rs` 16:15 |
| `writer` | `…/🎮️commands/🔍️lint-document/🧪️tests/🔬️unit/🦀️.rs` 15:58 |

**Follow-up for the coordinator to decide (I did not change it):** 4 of those 9 (`cad`, `puzzle`,
`reasoning`, `writer`) are stale only through a `🧪️tests/**/🦀️.rs` edit, which `materialize`'s own
`production` input EXCLUDES — so Nx will not rebuild for them and the `[stale]` line asks for a restage that
would change nothing. Mirroring `production`'s `🧪️tests`/`🧫️fixtures` exclusion in the walk would drop those
four, but a crate that `include_str!`s a fixture would then stop reporting, so it is a judgement call, not a
bug fix. The five that remain (`playbook`, `playbook-module-procedural`, `process`, `sequence`, `space`) are
real restage candidates whenever their agents are done editing.

### 5. The 15:57 serve did load the drift-tolerant merge

- The supervisor's events file recorded `15:57:03 started serve`; the merge landed on disk at 11:14 and was
  auto-committed at 11:20 (`f2585a4fdb`). Bun reads the module at import, so that process cannot have run
  the old one.
- In the log (read at 16:05, before the sweep) the last generation printed its `[stale]` block — the one
  naming the NEW 13:42Z–13:56Z descriptors — and then `Play ready`, with **no** `union activation receipt
  refused` line after it. The refusals earlier in the file carry the OLD message text (`is <sha> in
  demonstrator but <sha> in flow`), which the current code cannot produce; they belong to the 03:20
  generation that ran all day.
- The drift path was not exercised, because the 15:43 `activate-dev` left the lanes consistent: a read-only
  merge of the 28 live receipts right now reports `lanes=28 components=60 drift warnings=0` and
  `components with disagreeing lane receipts: 0` (`🗑️generated/play-runtime/merge-readonly.ts`).

### 6. Play unit suite is GREEN

With the regenerated descriptors the two reds of this morning are gone by themselves, exactly as predicted:
`cd 🏢️semio-tech/🎡️play && bun ./📜️script.ts test` → **71 passed (71)**, 4.34 s
(`🗑️generated/play-runtime/unit-71-of-71.txt`). That includes the new `exampleLabel` pin over all 52 curated
panes — the nine `stdio` labels I had read out of the Rust example sources are confirmed by the regenerated
descriptor (`label.native.en = "Demo"` for all nine).

### 7. :6033 is DOWN (not by me)

At 16:38–16:45 `http://127.0.0.1:6033/` does not answer and no process listens on the port; the sweep at
16:35:58 also removed `🗑️generated/serve-6033-supervised.txt` and its events file. A
`🗑️generated/serve-restart.request` dated 16:36 already exists (not mine). I started, stopped and restarted
nothing, and I did not run activation. The acceptance assertions of task 2 therefore could not be
re-measured against the 15:43 activation.

### Files changed in this task

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts` —
  `GENERATED_COMPONENT_OWNER_FILES` + the owner-root skip in `newestComponentSourceMtime`.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json` —
  `freshness.walk.generatedOwnerFiles`.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts` —
  two new laws + the fixture type.

---

## 2026-09-22 17:00–18:15 — the canvas census was a false green; the paint witness now reads the host

The raster agent was right, and the measurements below say why both pixel routes are dead ends:

- **An element screenshot is a PAGE screenshot clipped to the element box.** A full-bleed viewport has the
  window header, gutters and overlays inside that box, so the census counted chrome as paint: a visually
  BLANK `raster` composite scored 22 distinct colours and 0.60 % differing pixels — above any bound that
  does not also red working panes.
- **A `drawImage` readback of the live canvas is blank by construction here.** Probing the contexts on
  :6033: `raster`'s composite and `cad`'s viewports answer `getContext("webgpu")` — a WebGPU (or WebGL
  without `preserveDrawingBuffer`) surface reads back transparent whether or not it drew.

### What the host publishes, measured on :6033 (15:47 activation)

| window kind | witness | where |
|---|---|---|
| 3D world (`🌐️World3dHost`) | `data-meshes-json`, `data-instances-json` on the surface element | DOM attribute, e.g. `energy` 4 542 chars / `fem3d` 2 meshes + 234 instances |
| raster (`🖌️Paint2dHost`) | `scene.documentSyncJson` layers + `scene.assetsJson` textures | host props (React fiber of `[data-surface-id]`) |
| everything else | text + element count of the window body | DOM |

`readPaintWitnesses` asks every mounted window in its OWN terms in one `page.evaluate` (≈20 ms/pane, the
slowest measured 334 ms), and the pane passes when ANY window paints. Any-window, not largest-window, is
forced by the data: `sourcing`'s 3D viewport is correctly near-empty (`📓️audit-visual-2.md`: "perspective
grid only") while the pane is full of content, and `energy` mounts an empty table window next to its model.

Two traps the measurements exposed and the implementation now handles:

1. **A retained lane can land after the boot settle.** `energy` read `0 meshes + 0 instances` at 2.5 s and
   `4 542 characters` of `data-meshes-json` at 4 s; a single read reds a healthy pane. The witness is
   polled every 250 ms up to 4 s and a painted pane answers on the first read.
2. **Not every pane mounts a surface host.** The 15 `norm` codes, `stdio-json*`, `stdio-xml*`, `block2d`,
   `block5d`, `home`, `space` and `vdi3805` have no `[data-surface-id]` at all — their content is the
   window body. A first version returned "the pane mounted no window at all" for all 23 of them
   (`🗑️generated/play-runtime/e2e-full.txt`, 24 failed); the body fallback is now a declared branch and
   all 23 pass (`e2e-witness2.txt`).

### Why `raster` fails, exactly

```
Error: raster paints nothing — window:raster-composite [paint2d] EMPTY — 2 layers, 1 assets:
       backdrop→semio-emblem 2×2; window:raster-navigator [paint2d] EMPTY — 2 layers, 1 assets: …
```

Its document declares a **1024×1024 visible `pixel` layer** whose `imageKey` is `semio-emblem`, and the
scene's only asset is a **2×2** PNG (104 base64 characters, read from the IHDR without decoding the image).
The pane screenshot (`🗑️generated/play-runtime/pane-raster.png`) is an empty composite. So
"the document has a layer and the scene has an asset" — which is all `assetsJson != "{}"` says, and the
raster agent's fix did land: `assetsJson` now carries `semio-emblem` where `📓️raster.md` recorded `"{}"` —
still paints nothing. The witness therefore requires a visible image layer whose asset decodes to at least
`MINIMUM_IMAGE_ASSET_EDGE` (8) pixels a side: far below any authored demo, far above a placeholder.

### Verification (`:6033`, 15:47 activation, one worker)

| pane | verdict | witness |
|---|---|---|
| `raster` | **FAIL** (required) | `2 layers, 1 assets: backdrop→semio-emblem 2×2` |
| `cad` | pass | `1 meshes, 1 instances` (+3 more world3d windows) |
| `puzzle3d` | pass | `4 meshes, 1 instances` |
| `stdio-tsv` | pass | `TableHost: 390 characters, 77 elements` |
| `energy` | pass | `0 meshes, 0 instances` on the model window, table window paints |
| `sourcing` | pass | viewport empty, `790 characters, 434 elements` next to it |
| `block2d`, `din4108`, `vdi3805`, `stdio-json`, `home`, `space` | pass | window-body fallback |

Logs: `e2e-witness.txt` (6 panes, 5 passed / raster failed, 42 s), `e2e-witness2.txt` (8 panes incl. the
body fallback, 7 passed / raster failed, 59 s), `e2e-full.txt` (the whole suite before the fallback:
46 passed / 24 failed, 10.3 min), `e2e-full2.txt` (the whole suite after it).

Cost: the witness replaced a ~1.3 s screenshot+decode per pane with a ~20 ms host read, so the suite got
CHEAPER, not slower; only a pane that paints nothing pays the 4 s poll.

### Proposed diff (not applied — `🖌️Paint2dHost` is the raster agent's file)

`🌐️World3dHost` publishes its retained geometry on the surface element as `data-meshes-json` /
`data-instances-json`, which is why the 3D witness needs no React internals. `🖌️Paint2dHost` publishes
nothing comparable, so the raster witness reads the host's `scene` prop through the React fiber of
`[data-surface-id]` — test-only, but an internal. One attribute would make it public and identical in kind
to the 3D one:

```tsx
// 🖌️Paint2dHost/🟦️.tsx, on the `semio-paint-2d-canvas-surface` div
data-layers-json={scene.documentSyncJson}
data-assets-json={scene.assetsJson}
```

### Files changed in this task

- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` — the screenshot census is
  gone; `readPaintWitnesses` + `paintWitness` (per-window-kind predicate, polling, body fallback),
  `MINIMUM_IMAGE_ASSET_EDGE`, `PAINT_WITNESS_DEADLINE_MS`/`PAINT_WITNESS_POLL_MS`. The curated-example
  label assertion and the SPA-fallback asset guard are unchanged.

### Full strict suite with the honest witness — **69 passed, 1 failed, 8.9 min**

`bun node_modules/playwright/cli.js test --config 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
--workers 1` against `:6033` (15:47 activation), log `🗑️generated/play-runtime/e2e-full2.txt`:

- the ONLY red is `raster`, for the reason above;
- all 13 world-3d panes, all 45 DOM panes, the 9 `stdio` panes and the overview pass;
- `wfc2d`, `wfc3d` and `grid3d`, the three shell-error panes of the 03:04 run, now boot and paint
  (`grid3d`: 3 meshes + 27 instances);
- `animate` passes now — the `/🖼️bauteilbörse.png → 200 text/html` SPA-fallback red of this morning is
  gone on this activation;
- 8.9 minutes for 70 tests with one worker, inside the ~10 min budget.
