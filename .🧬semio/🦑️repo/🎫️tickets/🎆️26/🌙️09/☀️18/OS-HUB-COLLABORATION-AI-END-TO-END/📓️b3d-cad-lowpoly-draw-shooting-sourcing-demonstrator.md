# B3d — 📐️cad · 💠️lowpoly · 🖍️draw · 🎥️shooting · 🪵️sourcing · 🎪️demonstrator, plus the re-verification of the seven proven editors

Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, slice **B3d**, session 5b (2026-09-20, started
~08:05, load ≈ 97, 33 GiB free).

**Inheritance: none.** An earlier B3d worker was launched on 2026-09-19 ~00:20 and died without a
report. `ls 🗑️generated/b3d-*` is EMPTY and `📓️b3d-*.md` did not exist — the only B3d artefacts in
the tree are four probe scripts and two shell scripts written by that worker before it was cut
(`🐍️b3d-cad-probe.mjs`, `🐍️b3d-plugin-probe.mjs`, `🐍️b3d-cad-extension-probe.mjs`,
`🐍️b3d-interaction-probe.mjs`, `🐍️b3d-sourcing-modules-probe.mjs`, `📜️b3d-activate.sh`,
`📜️b3d-serve.sh`, all dated 09-19 00:17–01:14). Nothing was measured by it. This report starts from
zero measurements.

## 1. The bar and the method

Five clauses per artifact, all measured headless, nothing inferred:

1. **boots** — `data-semio-os-ready="<variant>"`, no `data-semio-os-error`, no window fault,
2. **example loads** — a non-empty document witness (pane, UiNode body, or app panel rows),
3. **one real mutation** — an Actions-rail verb dispatches and appends an APPLIED row to the
   framework history ledger *and* moves the uncommitted-edit count on `#s-checkin`,
4. **undo → redo** — the ledger row retires and comes back, edit count `n → n-1 → n`,
5. **zero console fault lines** over the whole run.

Tooling, per the coordinator's direction:

- the SHARED probe `🐍️b3a-interaction-probe.mjs` (B3a2 fixed four scoring defects in it today:
  UiNode surfaces, app panels, docked-panel-over-the-rail, stale witness),
- headless chromium `--use-angle=metal`, 1600×1000,
- `SEMIO_PROBE_SETTLE_MS=60000` under fleet load (see §2 — the knob did not exist in the shared
  probe and was added here),
- activate ONCE per variant (`📜️b3d-activate.sh <variant> <port>`), serve detached with nohup
  (`📜️b3d-serve.sh <variant> <port>`), never `dev <variant>` watch mode.

Ports are the ones the `.vscode/launch.json` rows name (`bun nx run workspace:dev -- <variant>`,
`S_OS_PORT`): cad 6020, lowpoly 6078, draw 6064, shooting 6019, sourcing 6081, demonstrator 6107,
raster 6060, forms 6058, note 6080, fem2d 6086, fem3d 6087, energy 6106, layout 6079, remodel 6063.
One exception, recorded because it matters for reproduction: **note was served on 6180, not 6080** —
6080 is held by a vite serve started 2026-09-19 11:20 by a session that is long dead, and preamble
rule 22 says a worker kills only what its own captures prove is its own, so it was left running.

## 2. Six probe fixes landed here in the SHARED probe (affects every B-slice)

B3a2 fixed four scoring defects in `🐍️b3a-interaction-probe.mjs` this morning. Six more turned up in
this slice, every one of them the same family — **the probe scored a working app as broken** — and
all six are now fixed in the shared file. The first three are the ones that changed a verdict:

1. **No settle budget knob** (`:124`). The three `until(…)` budgets were the literal `25_000`, and
   B3b had already measured trinity jack landing its undo at 112 s and its redo at 136 s at load
   ≈ 110. A slow guest and a stolen undo are indistinguishable through a fixed budget. Added
   `SEMIO_PROBE_SETTLE_MS` (default unchanged at 25 s); every B3d run in this report used 60 s.
2. **An Actions-rail ROW can be covered, not just the engagement toggle** (`clickUncovered`, `:119`).
   B3a2's fix retires panels that cover the *toggle*. It does not help once the rail is open and a
   docked panel covers an individual row: `force: true` clicks the row's centre point anyway, the
   press lands on whatever is painted on top, and the call still returns `ok`. Measured on fem2d with
   the artifact **and** inspection panels docked: the `action.undo` press journalled a `Select`
   chrome note (the inspection tree got the click) and the edit count sat at 1 for the whole budget.
   `clickUncovered` now checks `elementFromPoint` at the row centre, retires only the panels that
   geometrically cover that row, and presses again. Used for the mutation row, `undo` and `redo`.
3. **A multi-window app mounts every action row once per window rail, under the same element id**
   (`activeScoped`, `:152`). Census `🐍️b3d-undo-row-census.mjs` on fem2d
   (`🗑️generated/b3d-undo-row-census-fem2d.txt`): **all 34 rows exist twice**, `fem2d-model` at
   x ≈ 13 and `fem2d-results` at x ≈ 811, and the ACTIVE window is `fem2d-results`. A document
   mutation reaches the document from either rail, but `undo` is dispatched in the *active* window,
   so `locator(…).first()` — the model rail's copy — undoes nothing. `undo`/`redo` are now scoped to
   `[data-slot="window"][data-active="true"]` with a fallback to the first copy.

Defects 2 and 3 are the reason fem2d/fem3d first read `undone: false`. Scoping `undo` to the active
window was NOT enough on its own — the rail that actually reaches the document is neither reliably
the active one nor reliably the first in the DOM — so the step now presses candidates in order
(`pressUntil`) and stops the moment the edit count moves. Three more followed from apps that publish
their verbs somewhere other than an open rail:

4. **`rowSelector`** — 🖨️raster contributes its mutations as ARTIFACT-PANEL tree rows
   (`…raster-play-layers.add.pixel`), so its `actionCount` is 0 however hard the rail is pressed. The
   probe can now be pointed at that row; the app panel is raised only for the press and the History
   panel (which shares the dock, and takes `#s-checkin` with it) is put back before any witness.
5. **A keyboard `key:Meta+z` route** — the last resort for a rail-less app, which is exactly how
   raster's own end-to-end ticket drives its undo.
6. **Rail recovery when SOME rail mounted but the verb did not** — 📏️layout mounted the
   `layout-preview` rail's 17 generic rows while `layout-blueprint`'s stayed folded, and
   `addPage`/`addFrame` exist ONLY there, so layout read as an app with no mutation at all. The
   recovery condition changed from "zero rows" to "the wanted verb is absent". **This alone moved
   layout from a fabricated FAIL to a measured 5/5 PASS.**

**Every earlier "did not clear the bar" verdict is suspect if the app has more than one window, or
publishes verbs in a panel, or has more than one rail** — those three shapes cover most of the
remaining B2c/B3c reds and they should be re-run before anyone treats them as plugin defects.

New in this slice, all in the ticket folder: `🐍️b3d-bar-probe.mjs` (one CLI wrapper for all fourteen
variants, census mode when no verb is given), `🐍️b3d-undo-row-census.mjs`,
`🐍️b3d-fem-undo-diagnose.mjs`, `📜️b3d-activate-chain.sh` (serial one-cargo-at-a-time activation).

## 3. Bar matrix — measured only

Rows appear only once a run has produced a `SUMMARY` line. `—` means not run yet in this session.

| # | artifact | variant | port | boots | example | mutation (verb) | undo | redo | faults | bar |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 🖨️raster | raster | 6060 | ✅ | ✅ 2 windows, artifact tree | ✅ `add.pixel` (panel row) | ✅ **trap FIXED** | ❌ **trap moved to redo** | **3** | ❌ **FAIL** ᴮ³ᶠ |
| 2 | 📋️forms | forms | 6058 | ✅ | ✅ 35 verbs | ✅ `addStep` | ✅ | ✅ | **0** | ✅ **PASS** |
| 3 | 🗒️note | note | **6180** | ✅ | ✅ 19 verbs, 2 windows | ✅ `addBlock` | ✅ | ✅ | **0** | ✅ **PASS** |
| 4 | 🏗️fem 2d | fem2d | 6086 | ✅ | ✅ 34 verbs, 2 windows | ✅ `addNode x=3.5 y=4.5` | ✅ | ✅ | **0** | ✅ **PASS 5/5** ᴮ³ᶠ |
| 5 | 🏗️fem 3d | fem3d | 6087 | ✅ | ✅ 34 verbs | ✅ `addSupport` | ❌ **no-op, twice** | ❌ | **0** | 🟡 **4/5** ᴮ³ᶠ |
| 6 | 🔋️energy | energy | 6106 | ✅ | ✅ 33 verbs | ✅ `rename-zone zone=1 newName=ProbeZone` | ✅ | ✅ | **0** | ✅ **PASS** |
| 7 | 📏️layout | layout | 6079 | ✅ | ✅ 19 verbs, 2 windows | ✅ `addPage` | ✅ | ✅ | **0** | ✅ **PASS** |
| 8 | 📸️remodel | remodel | 6063 | ✅ | ✅ 40 verbs, 2 windows | ✅ `addStream` | ✅ | ✅ | **0** | ✅ **PASS** |
| 9 | 🖍️draw | draw | 6064 | ✅ | ✅ 17 verbs | ✅ `addLayer` | ✅ | ✅ | **0** | ✅ **PASS** |
| 10 | 💠️lowpoly | lowpoly | 6078 | ✅ | ✅ 59 verbs, artifact tree | ✅ `addPrimitive kind=box` | ✅ | ✅ | **0** | ✅ **PASS** ᴮ³ᵉ |
| 11 | 📐️cad | cad | 6020 | ✅ **4 windows** | ✅ 37 verbs | ✅ `addNode` | ✅ | ✅ | **0** | ✅ **PASS** |
| 12 | 🎥️shooting | shooting | 6019 | ✅ 2 windows | ✅ 49 verbs, both panes | ✅ `addShot` | ✅ | ✅ | **0** | ✅ **PASS** ᴮ³ᵉ |
| 13 | 🪵️sourcing | sourcing | 6081 | ✅ **4 windows** | ✅ 16 verbs, Pool table + 3 modules | ✅ `curationSetCount` (Pool stepper cell, `delta +1`) | ✅ | ✅ | **0** | ✅ **PASS 5/5** ᴮ³ᶠ |
| 14 | 🎪️demonstrator | demonstrator | 6107 | ✅ | ✅ 11 verbs, text window | ✅ `changeSchema` | ✅ | ✅ | **0** | ✅ **PASS 5/5** ᴮ³ᶠ |

ᴮ³ᶠ re-measured by slice B3f. 🪵️sourcing on the same staged guest after the React table-stepper fix
(§B3f.2), `🗑️generated/b3f-sourcing-bar2.txt`. 🎪️demonstrator on a guest B3f REBUILT
(`activate-demonstrator-react-dev` exit=0 at 16:56, 64 nx tasks), `🗑️generated/b3f-demonstrator-bar.txt`
— B3d's eight store owner/disposer declarations are now proven at runtime (§B3f.5). 🏗️fem2d/🏗️fem3d
on a guest B3f REBUILT (`activate-fem2d-react-dev` exit=0 at 17:44), `🗑️generated/b3f-fem{2,3}d-bar*.txt`
(§B3f.6). 🖨️raster on a guest B3f REBUILT (`activate-raster-react-dev` exit=0 at 18:00),
`🗑️generated/b3f-raster-bar.txt` (§B3f.7).

ᴮ³ᵉ measured by slice B3e on the variant's STAGED component (2026-09-19), not on a guest rebuilt
from today's tree — see §B3e.6 and §B3e.7. Rows 1 (raster), 4, 5 (fem) and 14 (demonstrator) still
carry their PRE-fix values: the fixes exist, the rebuild does not (§B3e.7).

Captures: `🗑️generated/b3d-<variant>-bar*.txt` (the numbered files are successive runs as the probe
defects of §2 were removed — the LAST numbered run of each variant is the one the row reports) plus
`🗑️generated/b3a-<variant>/report.json` and `🗑️generated/b3a-<variant>-console.txt`.

### 3.0 The re-verification verdict for the seven previously-proven editors

This is the question the coordinator asked: are the seven editors that were proven before the peer
renames (`🎚️options`→`☑️options`, `⚙️config`→`🎚️config`) still green? Measured, all seven:

| editor | verdict | note |
|---|---|---|
| 📋️forms | ✅ **still green** | 5/5, guest staged 08:00 today |
| 🗒️note | ✅ **still green** | 5/5, on a guest this slice rebuilt (07:20, 14 min) |
| 🔋️energy | ✅ **still green** | 5/5, guest staged 07:36 today |
| 📏️layout | ✅ **still green** | 5/5, on a guest this slice rebuilt (07:06, 10 m 23 s) |
| 📸️remodel | ✅ **still green** | 5/5, on a guest this slice rebuilt (07:35, 15 min) |
| 🏗️fem 2d/3d | 🟡 **4/5 each** | the ONLY failing clause is "zero console fault lines", and both lines are fem's own `[DEBUG]` eprintlns (§3.3), removed at source here |
| 🖨️raster | ❌ **REGRESSION** | `undo` traps the guest (§3.1), on a guest rebuilt at 06:51 today |

**No rename-caused failure was found in any of the seven.** The one red (raster) is a store
retirement trap, not a taxonomy drift, and the two yellows are logging, not behaviour.

**Guest freshness.** draw (07:27), demonstrator (07:24), forms (08:00), fem (07:42) and energy
(07:36) were staged by S2 this morning and needed no activation — their served guest IS the tree.
raster and layout were rebuilt by this slice's chain (`raster` exit=0 at 06:51 after 43 min,
`layout` exit=0 at 07:06 after 10 m 23 s of `component-dev`). note / remodel / lowpoly / cad /
shooting / sourcing were all source-newer than their staged component (M5a's 191-declaration sweep
landed in every `✏️editor/🦀️.rs` between 00:10 and 02:50) and are queued behind the chain.

### 3.1 🖨️raster — `undo` TRAPS the guest (P0, real, on a guest built at 06:51 today)

raster is the only artifact in this slice whose failure is a wasm trap. It mutates fine (the artifact
tree's `Add Pixel` row → `addLayer {kind:"pixel"}`, edit count moves, the tree grows a row), and then:

```
error  thread '<unnamed>' (1) panicked at ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../🦀️.rs:301:9:
warning input #24 undo refused: dispatch-failed (user window=raster-navigator) — unreachable
warning input #25 noteShellCommand refused: dispatch-failed … {"code":"plugin.internal","message":"runtime …
warning input #26 redo refused: dispatch-failed …
```

`…/🦀️.rs:301` is `RasterOwnedMap`'s fail-closed destructor:

```rust
impl<V> Drop for RasterOwnedMap<V> {
    fn drop(&mut self) {
        assert!((self.length == 0 && self.pages.iter().all(Option::is_none)) || std::thread::panicking(), "Raster owned map reached Drop before every entry and page backing was explicitly retired");
```

So the undo path drops a `RasterSnapshot` whose paged owner still holds entries — the guest aborts,
and **every** later dispatch in that shell is refused (`noteShellCommand`, `redo`, …), i.e. the first
undo kills the session. This is the "close every store before drop" law applied to raster's own map;
the assertion is correct and the retirement before it is missing. Not fixed here: it is a
retirement-path change inside raster's store lane, each attempt costs a ~40 min wasm rebuild, and the
evidence above is enough for the owner to cut it precisely. **Capture: `🗑️generated/b3a-raster-console.txt`
lines 5–8, `🗑️generated/b3d-raster-bar5.txt`.**

### 3.2 🎪️demonstrator — its ONE mutation was refused; root fix landed

`changeSchema` is demonstrator's only document mutation, and it never reached the document:

```
warning input #6 changeSchema refused: dispatch-failed (user window=framework.window.text)
  — typed-operation failed: validation failed: returned snapshot read requires its exact owned-snapshot retirement factory
```

After that refusal the shell itself goes to `data-semio-os-error="demonstrator"`. The cause is the
exact defect 📖️playbook carries a comment about: the editor declared
`build_artifact_store_one_item_preparation_factory` and **no store owners or disposers at all**.
Fixed at source in
`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:310`
(eight declarations: bounded document owners + disposer, `no_config_*`, `no_draft_*`,
`no_presence_store_disposer`, `no_transient_store_disposer`). **Compiled: no. Re-measured: no** — the
activation is queued behind the chain. The row above is the pre-fix measurement and stays until a
rebuilt guest is probed.

### 3.3 🏗️fem — two `[DEBUG]` eprintlns are the only thing between fem2d and a green bar

fem2d clears boot / example / mutation / undo / redo / panel round-trip. Its only two "fault lines"
are its own debug logging:

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/…/✏️editor/🧵️session/🦀️.rs:1020` — `eprintln!("[DEBUG] fem2d session fault at {:?}: {}", …)`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🧵️session/🦀️.rs:2950` — `eprintln!("[DEBUG] fem3d session fault: {}", …)`

Both sit in a `fail()` that already returns `JobStep::Failed(detail)`, so the line adds nothing and
violates the "no `[DEBUG]` logs left behind" rule. **Removed at source here; not rebuilt, so the
matrix still shows the pre-fix fault counts.** All three edited Rust files parse clean
(`rustfmt --edition 2021 --emit stdout`, no diagnostics); nothing beyond that is compiled — see §6.

**Six more `[DEBUG]` eprintlns remain in 🏗️fem and were deliberately NOT touched** — they print boot
snapshots and live-value dumps, none of them matches the probe's fault pattern, so none of them
blocks the bar, and stripping multi-line `eprintln!` bodies mechanically is exactly the change that
has fused `if` heads in this repo before. For fem's owner:
`🧊️3d/…/👁️viewer/🦀️.rs:73`, `🧊️3d/…/✏️editor/🦀️.rs:894`,
`🧊️3d/…/✏️editor/🧵️session/🦀️.rs:3691`,
`🧊️3d/…/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs:25`,
`◻️2d/…/🧬️schema/🦀️.rs:181` and `:194`.

fem3d also taught the slice something about *choosing* the measured verb: `addNode` / `addMaterial` /
`addSection` all leave the model a mechanism, the solver then repeats
`stiffness matrix is singular — model is a mechanism or under-constrained` every turn, and the undo
never lands. `addSupport` — which does not destabilise the solve — undoes cleanly. That is a
**measurement** caveat, not proof fem3d's undo is sound for every verb; the honest statement is
"fem3d undoes a support; it does not undo a free node while its solver is faulting".

## 4. 📐️cad's 4 extensions and 🪵️sourcing's 3 modules

First, a correction to the slice brief's framing: **none of the seven has a dev variant of its own.**
`.vscode/launch.json` has exactly one `workspace:dev -- cad` row (6020) and one
`workspace:dev -- sourcing` row (6081); the only other cad/shooting rows are FIXTURE launches
(`workspace:dev -- cad fixture concrete`, `… shooting fixture base-icon`) that reuse the same port,
and the `mit-bestand` rows are a different product flavour (`koordinator` 6028, `aussuchen` 6030).
An extension is not separately bootable — it contributes windows and verbs INTO the host plugin's
session, so its bar is measured through the host's variant.

**📐️cad's four extensions are live, and the witness is the window list.** The cad shell boots four
windows, one per extension contribution:

```
windowIds: ["cad-play-shape", "cad-play-building", "cad-play-energy", "cad-play-structure-classic"]
```

— `🔷️spatial-shape`, `🏢️aec-building`, `🔥️aec-building-energy`, `🏟️aec-building-structure`. All four
components are staged (`🔷️` / `🔥️` / `🏟️` at 07:13 today, `🏢️` at 00:55), and the cad serve's only
staleness line for the cad variant names `cad-extension-aec-building` **source-newer against a TEST
file** (`🧩️extensions/🏢️aec-building/🧪️tests/🔬️unit/🦀️.rs`, 09-19 23:44), which does not enter the
wasm. Zero console faults over the whole run. The cad row in §3 is therefore a measurement of the
host **and** its four extensions in one session.

Not measured: an extension-OWNED mutation (the inherited `🐍️b3d-cad-extension-probe.mjs` drives the
Building pane's "Place Column" model-definition interaction, which is `🏢️aec-building`'s own action).
It never ran — the cad serve only came up at 11:55 and the slice's remaining time went to the
unmeasured plugins. That probe is ready to run against :6020 as-is.

**🪵️sourcing's three modules** (`🪜️beams`, `🧇️slabs`, `🪟️windows`) are all staged (all three at
09-19 16:58) but sourcing itself is one of the variants whose activation had not finished when this
report was written, so nothing about them is measured here. The inherited
`🐍️b3d-sourcing-modules-probe.mjs` reads exactly the right witness — the Pool window's rows carry the
module each stock kind came from, plus the boot ledger's `Set Contributions` entry — and is ready to
run against :6081.

One thing already known about them from a peer's capture: B3a's block2d serve recorded
`[stale] sourcing-module-windows: unactivated — staged 2026-09-19T14:58:27.361Z but absent from the
activation receipt`, i.e. the three module components are staged but do not appear in a non-sourcing
variant's receipt. That is expected (receipts are per-variant), not a defect.

## 5. G3's leftover `🪵️sourcing/…/🪟️windows/🎚️options` directory — RESOLVED

Measured first. Repo-wide, outside `node_modules`/`dist`, there were exactly **two** `🎚️options`
directories left against **353** `☑️options` ones, and only one of the two is in product source:

- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options/📌️.empty.md` (tracked; G3's leftover)
- `.🧬semio/…/🎫️tickets/🎆️26/🌙️07/☀️12/WINDOW-APP-PANEL-CONTRACTS/🧪️window-policy-fixture/…/🎚️options`
  (a closed ticket's frozen fixture — deliberately NOT touched; it is another slice's evidence)

The sourcing one is a scaffolding marker: `🎚️options`, `🎬️actions`, `👥️presence` and `🪛️utilities`
each contain nothing but `📌️.empty.md`, none of the four is referenced from the extension's own
`🦀️.rs`, and the two SIBLING sourcing extensions (`🧱️slabs`, `🪵️beams`) carry none of them at all.
Since `🎚️` now means **config** after the peer rename (`⚙️config`→`🎚️config`), leaving an `options`
directory under that emoji is a live taxonomy collision, so it was renamed rather than deleted —
deleting the scaffolding for one extension and not its siblings is a decision that belongs to
sourcing's owner, renaming is the narrow fix G3 asked for:

`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options` → `☑️options`

After it, `find ✏️s -type d -name "🎚️options" | wc -l` is **0**.

## 6. cargo deadlock at 11:54–12:30 (preamble rule 23) — why four rows are still `—`

**cargo deadlock at 11:54.** Reported per rule 23. The `🎥️shooting` activation reached
`plugin-registry:session-shooting` at 11:54 and then produced nothing for 23 minutes. Rule-23 check:

```
pgrep -c rustc                      → 0        (no rustc anywhere on the machine)
ps | grep cargo                     → 6 cargos, all 0.0 % CPU, 12–42 min old
sample 17716 1 | grep prebuild_lock → 5 hits
    cargo::core::compiler::prebuild_lock_exclusive
    cargo::core::compiler::locking::LockManager::lock
    flock  (in libsystem_kernel.dylib)
```

pid 17716 was mine (a descendant of this slice's `activate-shooting-react-dev` nx node, verified by
walking `pgrep -P`), so it was killed by pid at 12:17 — no peer process was touched. The chain moved
on to `sourcing`; **ten further minutes produced no rustc at all**, so the cycle is machine-wide and
not mine to break. This is the same fine-grain-locking deadlock the coordinator swept at 06:12 and
again at 11:12 today.

Consequence, stated plainly: **💠️lowpoly, 🎥️shooting, 🪵️sourcing and the demonstrator RE-measure
are not measured.** Each needs a wasm rebuild that cannot start. Their state:

| variant | why it needs a build | chain result |
|---|---|---|
| 💠️lowpoly | editor `🦀️.rs` 09-20 02:02 > staged 09-19 18:02 | `exit=130` — cut at 09:11 by the session's own death, never retried |
| 🎥️shooting | editor `🦀️.rs` 09-20 02:03 > staged 09-19 18:02 | `exit=130` — the rule-23 kill above |
| 🪵️sourcing | editor `🦀️.rs` 09-20 02:03 > staged 09-19 16:58 | started 10:17, deadlocked, no output |
| 🎪️demonstrator | carries this slice's §3.2 root fix | queued in `📜️b3d-activate-chain2.sh` |
| 🏗️fem 2d/3d | carry this slice's §3.3 `[DEBUG]` removals | queued in `📜️b3d-activate-chain2.sh` |

`📜️b3d-activate-chain2.sh` (pid 94153) is armed behind chain 1 and will run
`lowpoly → demonstrator → fem2d → fem3d` the moment chain 1 exits — no further cargo is started by
this slice in parallel with it. Each writes `🗑️generated/b3d-<variant>-activate.txt`, so whoever
picks this up can see which finished. **`🎥️shooting` and `🪵️sourcing` are NOT in chain 2** (chain 1
still owns them); if chain 1 dies before they finish, re-run
`📜️b3d-activate-chain.sh shooting sourcing`.

Once a variant's activation exits 0, its whole bar is one command:

```
📜️b3d-serve.sh <variant> <port>          # detached, nohup
curl -s -o /dev/null "http://127.0.0.1:<port>/?plugin=<variant>"   # warm the vite transform first —
                                                                   # a cold first load exceeds the probe's 30 s goto budget
SEMIO_PROBE_SETTLE_MS=60000 bun 🐍️b3d-bar-probe.mjs <plugin> <variant> <port>              # census: dumps the live verb list
SEMIO_PROBE_SETTLE_MS=60000 bun 🐍️b3d-bar-probe.mjs <plugin> <variant> <port> <verb> [k=v] # the bar
```

## 7. Files changed

Product source (3 files, all root fixes, none of them yet compiled — see §6):

- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:310`
  — eight store owner/disposer declarations added (§3.2).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs:1020`
  — `[DEBUG]` eprintln removed (§3.3).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs:2950`
  — `[DEBUG]` eprintln removed (§3.3).

Taxonomy (§5): `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options` → `☑️options`.

Shared probe (`🐍️b3a-interaction-probe.mjs`, owned by B3a but shared by B2c / B3b / B3c / B3d) —
six additive changes, every one of them the cure for a **false negative**, default behaviour
unchanged where a knob exists: `SEMIO_PROBE_SETTLE_MS`; `clickUncovered`; per-rail `undo`/`redo`
candidates with `pressUntil` + `activeScoped` + a `key:Meta+z` keyboard route; a broadened
execute-control fallback; `config.rowSelector` for apps that publish verbs as panel tree rows; and
rail recovery that also fires when the rail mounted but the wanted verb did not.

New scratch files in the ticket folder: `🐍️b3d-bar-probe.mjs`, `🐍️b3d-undo-row-census.mjs`,
`🐍️b3d-fem-undo-diagnose.mjs`, `📜️b3d-activate-chain.sh`, `📜️b3d-activate-chain2.sh`.

## 8. Honest gaps and handoffs

1. **🖨️raster `undo` traps the guest (P0, §3.1).** Diagnosed to the line, not fixed. The snapshot
   type embeds `assets: RasterOwnedMap<RasterAssetChild>` whose `Drop` is fail-closed, raster DOES
   declare `RasterSnapshotRetirementFactory` for the document lane
   (`…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:3451`), and `RasterOwnedMap::retire()` has exactly two
   call sites (`…/🗿️artifacts/🖨️raster/🦀️.rs:387` and `:419`, both inside the map's own clone path).
   So some snapshot on the undo path is dropped WITHOUT going through `retire_owned`. Finding which
   one needs a debugger-grade read of the undo state machine (`HashInverse` → `ApplyForward` →
   `CommitApplied`) plus a ~40 min wasm rebuild per attempt; out of reach in this slice's remaining
   budget, and the evidence above should make the cut short for raster's owner.
2. **🎪️demonstrator and 🏗️fem fixes are unverified.** Both are one activation from a measurement
   (chain 2), and until that runs the §3 rows stay at their pre-fix values. Do not promote them.
3. **💠️lowpoly, 🎥️shooting, 🪵️sourcing: zero measurements.** Nothing at all is claimed about them.
4. **Extension-owned mutations are not measured** (§4): cad's four extensions are proven *mounted*
   (four windows, zero faults), not proven to *mutate*. `🐍️b3d-cad-extension-probe.mjs` and
   `🐍️b3d-sourcing-modules-probe.mjs` are written and ready and have still never been run.
5. **fem3d's undo depends on the verb** (§3.3): green for `addSupport`, never lands for `addNode` /
   `addMaterial` / `addSection` while the solver repeats `stiffness matrix is singular`. Whether the
   solver fault legitimately blocks the undo or merely starves it is not established here.
6. **The §2 probe fixes invalidate old verdicts.** Any earlier "does not clear the bar" on a
   multi-window app, an app whose verbs live in a panel, or an app with only one rail unfolded was
   possibly scored by a probe that could not reach the control. Worth a re-run for B2c/B3c rows.
7. **Servers left running** (none to be killed by others; all this slice's, by port): 6020 cad,
   6058 forms, 6060 raster, 6063 remodel, 6064 draw, 6079 layout, 6086 fem2d, 6087 fem3d,
   6106 energy, 6107 demonstrator, 6180 note. Port 6080 holds a note serve from 2026-09-19 11:20
   that is NOT this slice's — it was left alone and note was served on 6180 instead.

---

## B3e

Slice **B3e** (2026-09-20, from 12:44, load ≈ 90, 35 GiB free). Continues B3d's batch: outcome-1 depth
— raster's undo trap, the demonstrator re-measure, fem2d/fem3d re-measure, and the three unmeasured
plugins (💠️lowpoly, 🎥️shooting, 🪵️sourcing).

**Inherited live state at 12:44** (nothing restarted, nothing killed):

- `📜️b3d-activate-chain.sh` pid **99987** still alive, on its LAST variant `sourcing` (started
  12:17 local — the chain file's stamps are UTC), with `📜️b3d-activate-chain2.sh` pid **94153**
  armed behind it for `lowpoly → demonstrator → fem2d → fem3d`. Both are B3d's, both are doing
  exactly this slice's work, so they were left running and this slice started no competing wasm
  activation. `pgrep -fl rustc | wc -l` = 19 at 12:44, so the machine was NOT in the 06:12/11:12
  deadlock state.
- All eleven B3d serves from §8 still listening (6020 cad, 6058 forms, 6060 raster, 6063 remodel,
  6064 draw, 6079 layout, 6086 fem2d, 6087 fem3d, 6106 energy, 6107 demonstrator, 6180 note).
- `🎥️shooting` is in NEITHER chain (B3d's rule-23 kill at 12:17 left it unactivated).

### B3e.1 The `🎨️paint` collision (coordinator's note)

Checked first, as instructed. The two `paint_gesture_layer` / `paint_gesture_before` declarations in
`🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs` are **not** a duplicate: after the coordinator's own
repair they sit on two DIFFERENT structs — `RasterHost` (`:380`) and its retirement twin
`RasterHostRetirement` (`:1058`). The file parses clean (`rustfmt --edition 2021 --emit stdout`,
exit 0) and the coordinator's `cargo check -p semio-framework-surface` Finished, so nothing was
edited here.

What that `+60/−5` working-tree edit actually is: a **live paint-stroke undo capture** — the host
records each touched pixel's before-RGBA for the duration of one gesture and `pointer_cancel_screen`
(new, bound as `pointerCancelScreen`) restores them. It is a CANVAS-lane gesture cancel, not the
document lane: raster's document vocabulary has no paint mutation at all
(`🧬️schema/🧬️mutations/` is twelve layer/asset verbs; pixels reach the document only as a layer's
image asset), so a stroke is not an undoable ledger edit and `action.undo` can never reverse one.
It is therefore **not** the other half of the guest trap — that one is B3e.2, in the plugin guest,
and reproduced and fixed independently below. The paint-gesture capture is complete on the Rust side
(fields, `RasterHostRetirement` destructuring, `close_step` drain, `terminal_is_empty`); what is
still missing is any caller of `pointerCancelScreen` in the React canvas host, so the capture is
currently written and never read. Left for its author rather than half-wired here.

### B3e.2 🖨️raster `undo` traps the guest — root cause and fix

**Reproduced natively in 0.42 s, not inferred.** B3d's §3.1 had the assertion but not the drop site.
A mounted law on a POPULATED document (`🗑️generated/b3e-raster-repro.txt`) gives the whole chain:

```
panicked at 🗿️artifacts/🖨️raster/🦀️.rs:301: Raster owned map reached Drop before every entry and page backing was explicitly retired
 2: <RasterOwnedMap<DslValue> as Drop>::drop
 4: drop_glue::<RasterLayerNode>            ← the adjustment layer's `params`
 8: drop_glue::<RasterSnapshot>
 9: core::mem::drop::<RasterSnapshot>
10: <RasterDiff as MutationDiff<RasterSnapshot>>::retire_projection      ← HERE
11: os_store::retire_replayed_projection::<RasterSnapshot, RasterMutation>
```

The seam is `protocol::MutationDiff::retire_projection`, whose docstring
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:141`, ticket 26/09/09/PROCEDURAL-3D) states the
law outright: the default IS a bare drop, and *"a technology whose delta owns a fail-closed root …
MUST override it"*, because `undo`/`redo` and every `.pack`/`.spr` reload walk
`base → mid₁ → … → head` and throw every intermediate projection away through it. `🌊️flow` overrides
it. **raster did not** — so every displaced projection of a document carrying a populated
`RasterOwnedMap` (the demo carrier's `assets`, an adjustment layer's `params`) aborted the guest, and
after the abort the shell refuses `redo`, `noteShellCommand` and everything else.

Root fix, four edits, all in raster:

| file:line | change |
|---|---|
| `🗿️artifacts/🖨️raster/🦀️.rs:571` | new `retire_raster_layer` / `retire_raster_layers` — closes an `Adjustment`'s `params` and recurses through `Group` children |
| `…/🧬️schema/📸️snapshot/🦀️.rs:44` | new `retire_raster_snapshot` — the layer forest plus the `assets` map |
| `…/🧬️schema/🔺️diff/📝️text/🦀️.rs:381` | `RasterDiff::retire_projection` (→ `retire_raster_snapshot`) **and** `retire_cold` (the whole-artifact replacement and every `added` insertion's layer) |
| `…/🧬️schema/🧬️mutations/🦀️.rs:39,68` | `#[mutations(…, retire_cold = retire_raster_mutation)]` + the function — `CreateLayer` is the one leaf owning a layer subtree, and the store cold-retires decoded arrivals, replay clones and rebased inverses through it |

**Three new laws, all measured green** (`🗑️generated/b3e-raster-retirement-laws.txt`), in
`…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`:

```
retire_projection_closes_every_owned_map_in_a_displaced_projection ... ok
retire_cold_closes_every_owned_map_in_a_displaced_diff             ... ok
retire_cold_closes_the_layer_a_create_layer_operation_owns         ... ok
```

They cover a projection with a populated `assets` map, an adjustment layer, AND a nested group
hiding a second one — every shape the live demo document has. Before the fix the first of them is the
exact panic quoted above; after it, all three pass and the reproduction law no longer reaches
`RasterOwnedMap::drop` at all (the run's next failure is a different, pre-existing one, below).

**One more real fix on the way there.** `✏️editor/🧪️tests/🔬️unit/🦀️.rs`'s `context::app()` never called
`bind_instance_id`, so **every** typed dispatch in raster's editor suite failed closed with
`interactive-job.live-instance: typed command 'addLayer' does not belong to the mounted live app
instance` — 16 tests, including the pre-existing `add_layer_action_appends_and_undo_removes`. Fixed
by mounting the fixture the way 🔱️trinity and 📸️remodel already do
(`app.bind_instance_id(meta("local").instance_id).await`).

**Honest bounds on the live proof.** The fix is proven by the three laws and by the disappearance of
the trap in the mounted reproduction; it is **not yet proven on the live bar**, because re-measuring
🖨️raster needs a wasm re-activation (~43 min) and raster is queued last in `📜️b3e-activate-chain3.sh`.

**Two defects found next to it that are NOT this slice's and were deliberately not chased** (both
reproduce with raster's own committed tests, and `git status` shows a peer holding uncommitted edits
in `🧬️mutations/💾️binary/🦀️.rs`, `📸️snapshot/🦀️.rs` (+157 lines that are not this slice's) and
`✏️editor/🦀️.rs` right now — this is their lane):

1. **`RasterOwnedRetirement` does not terminate for a snapshot carrying an `ArtifactChild` asset.**
   raster's own `raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant`
   (`💾️binary/🧪️tests/🔬️unit/🦀️.rs:978`) fails with *"nested Raster retirement did not reach
   terminal"* after 10 000 bounded `close_step(1, RASTER_OWNED_FIELD_BYTES)` grants, and
   `raster_owned_map_cap_plus_one_…` runs past 60 s in the same family. Consequence downstream: the
   editor fixture `context::semio_app()` cannot dispose the envelope it prints its pack from —
   `ArtifactEnvelope::Drop` asserts, and draining it through
   `raster_envelope_decode_owner_bundle().retire_envelope(…)` spins on `Pending { 0, 0 }` forever.
   That is why this slice's app-level undo law was retargeted at the retirement seam itself rather
   than at `semio_app()`.
2. **A populated raster document cannot be built through commands in a unit fixture.**
   `MutationDiff::apply` refuses outright once `snapshot.assets` is non-empty (*"populated Raster maps
   require the retained initialization authority"*), so `setActiveExample demo` plants the emblem
   asset and then every following `create-layer` in the same batch is refused — measured: the demo
   example loads with `assets: []`. Only the retained SPR/`load_document_pack` lane can hydrate one.

### B3e.3 📐️cad's extensions now MUTATE — B3d gap 4 closed (no build needed)

B3d proved cad's four extensions *mounted* (four windows) but never that one of them can drive the
document, and left `🐍️b3d-cad-extension-probe.mjs` written and unrun. It runs against the serve B3d
left on :6020, so it needed no activation and was measured first, while the build queue moved.

`SEMIO_PROBE_SECONDS=90 SEMIO_PROBE_OUT=b3e-cad-extension bun 🐍️b3d-cad-extension-probe.mjs`
(`🗑️generated/b3e-cad-extension.txt`, `🗑️generated/b3e-cad-extension/`):

| witness | value |
|---|---|
| surface | `window:cad-play-building` — the `🏢️aec-building` extension's own pane |
| interaction | **Place Column**, a model-definition action only that extension contributes |
| meshes before → after | **11 → 12**, `instances` 11 → 12 |
| new object | `object-1` (alongside the two committed `concrete-forest` BIM objects) |
| HUD | `Step: Committed 1 object(s) OK` |
| `ok` | **true** |
| console faults | **0** plugin faults (one `404` for a static resource; no `refused`, no `Fault`) |

Screenshots of the started / rubber-band / committed states are in
`🗑️generated/b3e-cad-extension/`. So **📐️cad's extension lane is proven end to end**: an extension
contributes a window, an action and an engagement session, and its commit reaches the document.

Noted in passing, NOT fixed here because it would invalidate every plugin's build while the fleet is
queued on one wasm mutex: the framework leaves a `[DEBUG]` line in every app's console —
`[DEBUG] typed-operation slots instance=1 live=1/64 peak=1` — which violates the "no `[DEBUG]` logs
left behind" rule. It is a `console.debug`, so it does not enter any bar's fault count.

### B3e.4 🎪️demonstrator rebuild + re-measure — NOT DONE

B3d's root fix (eight store owner/disposer declarations,
`🎪️playground/…/✏️editor/🦀️.rs:310`) is still unbuilt. `📜️b3e-activate.sh demonstrator` was queued
through the fleet mutex at 13:58 and had not been granted the lock by the end of this slice
(§B3e.7). The §3 row therefore still carries its PRE-fix measurement and must not be promoted.
Nothing about the fix is claimed here.

### B3e.5 🏗️fem2d / 🏗️fem3d re-measure — NOT DONE

Same cause: both carry B3d's `[DEBUG]` eprintln removals, both need one activation
(`🏗️fem` is ONE component for both variants, so the second is a cache hit), and both sat behind
`demonstrator` in the mutex queue. Their serves (6086 / 6087) are still up on the pre-fix guests, so
the re-measure is one `📜️b3e-activate.sh fem2d` plus two probe runs once the mutex is free. The §3
rows keep their `[DEBUG]` fault counts.

### B3e.6 💠️lowpoly · 🎥️shooting · 🪵️sourcing — the unmeasured three, now measured

All three had staged components from 2026-09-19 that were never served. Rather than wait for the
build queue (§B3e.7), each was served from its staged component and put through the full five-clause
bar. **Every number below is measured on the STAGED guest**, whose `✏️editor/🦀️.rs` is older than the
tree (M5a's declaration sweep landed 09-20 02:02–02:03); the staleness is a declaration-metadata
sweep, not a behaviour change, but it is stated here rather than hidden.

Serves started by this slice (detached, `📜️b3e-serve.sh`): lowpoly **6078**, shooting **6019**,
sourcing **6081** — all three ports were free.

| artifact | port | boots | example | mutation | undo | redo | faults | bar |
|---|---|---|---|---|---|---|---|---|
| 💠️lowpoly | 6078 | ✅ 1 window, 59 verbs | ✅ artifact panel: `Hexagonal Cut Concrete Forest Left obj-1`, Vertices 71 / Edges 154 / Faces 57 | ✅ `addPrimitive kind=box` | ✅ | ✅ | **0** | ✅ **PASS 5/5** |
| 🎥️shooting | 6019 | ✅ 2 windows (Scene, Icon), 49 verbs | ✅ both panes render (`256×256 · rectangle`, `Overview Svg · 256×256 · SVG`) | ✅ `addShot` | ✅ | ✅ | **0** | ✅ **PASS 5/5** (panel round-trip too) |
| 🪵️sourcing | 6081 | ✅ **4 windows** (Pool, Curated, Preview, Grid), 16 verbs | ✅ Pool table with 673 chars of real stock rows | ❌ **not reachable in the React host** (below) | — | — | **0** | 🟡 **3/5** |

Captures: `🗑️generated/b3e-{lowpoly,shooting,sourcing}-{census,bar}.txt`,
`🗑️generated/b3e-sourcing-curation.txt`, `🗑️generated/b3e-sourcing-curation/`.

**🪵️sourcing's three modules are proven mounted** without needing the module probe: the Pool pane's
own text carries the module filter chips and the per-row module path —

```
Beams Windows Slabs Reuse | All Typologies | Restock From Catalogue
Glulam GL24h 200×400   beams / solid-timber / glulam   24
KVH C24 100×200        beams / solid-timber / kvh      60
Steel IPE 200          beams / steel / ipe             12
```

— and the history ledger's first entry is `Set Contributions`, the boot record that plants
`🪜️beams` / `🧇️slabs` / `🪟️windows`. That is exactly the witness
`🐍️b3d-sourcing-modules-probe.mjs` was written to read.

**Why sourcing cannot mutate — a REACT-HOST gap, measured, not a plugin defect.** sourcing's sixteen
rail verbs contain no undoable document edit: `stockFromCatalogue` is declared
`ArtifactToolPublicationLane::HostOnly` and answers with `Effect::LoadDocument`
(`🗂️curation/…/✏️editor/🦀️.rs:386`), and the probe measured it exactly so —
`clicked: ok, submitted: absent, mutated: false`. The real curation vocabulary
(`curationAdd` / `curationRemove` / `curationSetCount` / `dropOnPool` / `dropOnCurated`) is bound to
the Pool TABLE's cells: the `curated` column is a `TableCell::Stepper` per stock kind bound to
`curationSetCount` on `Trigger::Change`
(`…/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs:68`). In the React host that cell renders as:

```html
<div data-slot="input-root" data-detail-panel-control="fill" data-dim="true">
  <input data-slot="input" id="window:sourcing-pool.beam-glulam-gl24h.curated" readonly="" value="0">
</div>
```

**`readonly`, dimmed, and with no increment/decrement control at all** — although the React target
does export a `Stepper` component. So the only path to a sourcing document edit is not operable:
`fill` times out, and there is no sibling button to press (both measured,
`🗑️generated/b3e-sourcing-curation.txt`). Two new scratch probes carry the evidence:
`🐍️b3e-sourcing-dom.mjs` (the DOM census above) and `🐍️b3e-sourcing-curation-probe.mjs` (the
five-clause bar driven through the table cell instead of the rail). **Handoff:** this belongs to
whoever owns `TableCell::Stepper` in `🖱️ui/🎯️targets/⚛️react` — it is one renderer, and fixing it
turns sourcing's 3/5 into a measurable 5/5 with no wasm rebuild.

### B3e.7 The build queue under preamble rule 27

Inherited state at 12:44: B3d's chain 1 (pid 99987) was on its last variant `sourcing`, chain 2
(pid 94153) armed behind it for `lowpoly → demonstrator → fem2d → fem3d`, and `shooting` was in
neither. What happened, in order, all by pid and all recorded:

| time | event |
|---|---|
| 13:23 | `sourcing`'s cargo (pid 40815) had sat **64 min** in `prebuild_lock_exclusive → flock` with no `rustc` child. Killed by pid; nx retried it, and the retry (pid 72666) starved the same way for 15 min |
| 13:38 | stopped the whole `sourcing` activation subtree by pid (72666, then 40048/40049/40050/40057) so chain 1 could finish — `== sourcing exit=143`, `chain done` |
| 13:39 | chain 2 started `lowpoly` |
| 13:40 | armed `📜️b3e-activate-chain3.sh` (pid 86485) behind chain 2 for `shooting → sourcing → raster` |
| 13:57 | `lowpoly` `exit=130` after 17 m — the coordinator had killed its `cargo rustc` (pid 87682) under the new rule-27 check |
| 13:58 | **preamble rule 27 arrived.** Stopped chain 2 and chain 3 by pid (94153, 86485) and the in-flight demonstrator nx run, because neither chain went through the fleet mutex |
| 13:58 → | every activation now runs through `📜️wasm-build-mutex.sh` via the new `📜️b3e-activate.sh <variant>`, ONE at a time, foreground |

`📜️b3e-activate.sh demonstrator` was queued at 13:58 and was **still waiting on the mutex at 15:00**
(held by `s3` 13:58–14:17, then by `a3` from 14:17). The mutex is a `mkdir` spin with no queue, so a
waiter has no priority; this slice never held it. That is the whole reason 🎪️demonstrator, 🏗️fem2d,
🏗️fem3d and 🖨️raster are not re-measured here, and it is also why the three unmeasured plugins were
measured on their staged guests instead of waiting (§B3e.6) — that work needed no wasm at all.

**Scripts left for the next worker** (all in the ticket folder, all mutex-correct):
`📜️b3e-activate.sh <variant>` — one activation through the fleet mutex, its own capture at
`🗑️generated/b3e-<variant>-activate.txt`; `📜️b3e-serve.sh <variant> <port>` — detached serve.
`📜️b3e-activate-chain3.sh` is SUPERSEDED by rule 27 and must not be used as-is (it does not take the
mutex).

### B3e.8 Files changed, gaps, servers

**Product source (4 files, one root fix, compiled and covered by three green laws):**

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` — `retire_raster_layer` / `retire_raster_layers`.
- `…/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — `retire_raster_snapshot`.
- `…/🧬️schema/🔺️diff/📝️text/🦀️.rs` — `RasterDiff::retire_projection` + `RasterDiff::retire_cold`.
- `…/🧬️schema/🧬️mutations/🦀️.rs` — `#[mutations(…, retire_cold = retire_raster_mutation)]` + the function.

**Tests (2 files):**

- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `context::app()` now calls `bind_instance_id`, which unblocks
  every typed dispatch in raster's editor suite (16 tests were failing closed on
  `interactive-job.live-instance`).
- `…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — the three cold-retirement laws (§B3e.2).

**Ticket folder (new, all mutex-correct where they build):** `📜️b3e-activate.sh`,
`📜️b3e-serve.sh`, `📜️b3e-activate-chain3.sh` (superseded by rule 27, do not use),
`🐍️b3e-sourcing-curation-probe.mjs`, `🐍️b3e-sourcing-dom.mjs`. Captures: `🗑️generated/b3e-*`.

**Nothing else was touched.** In particular `🗺️surface/🎨️paint/🦀️.rs` was read and left alone
(§B3e.1), and no `🗑️generated` file this slice did not create was removed.

**Honest gaps, in priority order:**

1. **🖨️raster's bar is not re-measured.** The fix is proven by three laws and by the trap's
   disappearance in a mounted reproduction; it is NOT proven live. One `📜️b3e-activate.sh raster`
   (~43 min) plus `📜️b3e-serve.sh raster 6060` and the shared probe closes it.
2. **🎪️demonstrator / 🏗️fem2d / 🏗️fem3d re-measures** (§B3e.4, §B3e.5) — one activation each,
   queued and never granted.
3. **🪵️sourcing's mutation clause** (§B3e.6) is blocked on a React `TableCell::Stepper` renderer,
   not on the plugin. No wasm needed to fix it.
4. **💠️lowpoly / 🎥️shooting / 🪵️sourcing were measured on 09-19 staged components.** The delta to
   the tree is M5a's declaration sweep; a rebuild would make the three rows unconditional.
5. **Two raster retirement defects left for their owner** (§B3e.2): `RasterOwnedRetirement` never
   reaches terminal for a snapshot carrying an `ArtifactChild`, and a populated raster document
   cannot be built through commands in a unit fixture. A peer holds uncommitted edits in exactly
   those files right now.
6. **The framework leaves `[DEBUG] typed-operation slots …` in every app's console** (§B3e.3).

**Servers this slice started** (detached, `📜️b3e-serve.sh`, kill only by pid from
`🗑️generated/b3e-<variant>-serve.txt`): **6078 lowpoly, 6019 shooting, 6081 sourcing**. B3d's eleven
serves from §8 were all still alive at 12:44 and were reused, not restarted, and none was killed.
**Processes this slice stopped, all by pid, all inherited from B3d's own chains:** the starved
`sourcing` cargos 40815 / 72666 and their nx subtree (40048/40049/40050/40057), chain 1's successor
chains 94153 and 86485, and the un-mutexed demonstrator run 97619 — plus this slice's own
`cargo test` pids 64665 and 65-something. No peer process was touched.

---

## B3f

Slice **B3f** (2026-09-20, from 14:51, load ≈ 90, 45 GiB free). Closes B3d/B3e's batch: the
🪵️sourcing `TableCell::Stepper` defect and the four re-measures that were still queued behind the
fleet wasm mutex.

**Inherited live state at 14:51** (nothing restarted, nothing killed): mutex held by `a3` since
14:50; serves alive on 6078 lowpoly, 6019 shooting, 6081 sourcing, 6060 raster, 6086 fem2d,
6087 fem3d, 6107 demonstrator, 6020 cad; `pgrep -fl rustc | wc -l` = 5.

### B3f.1 🪵️sourcing's stepper — B3e's verdict CORRECTED

B3e reported the cell as *"`readonly`, dimmed, and with no increment/decrement control at all"* and
handed it on as a missing renderer. **That is wrong, and the DOM census proves it**
(`🗑️generated/b3f-stepper-dom.txt`, taken on B3e's own serve on :6081 BEFORE any edit of this
slice): the cell renders `button(minus) · input · button(plus)`, the minus disabled because the
value sits on `min: 0`, the plus enabled.

What B3e actually printed was the `Input` element's OWN wrapper. `Input` renders
`<div data-slot="input-root" data-detail-panel-control="fill" data-dim>` around its `<input>`
(`🖱️ui/🧱️elements/✏️Input/🟦️.tsx:459`), so `stepper.parentElement` is that wrapper, not the cell,
and the two buttons are the input's **uncles**. The same mistake made the probe's locator
`[id="…curated"] ~ button` unmatchable — a sibling combinator over an element that has no button
siblings — which is why both presses timed out and the third fallback clicked the readonly input
itself. `readonly` on the readout is correct and deliberate: the value is guest-owned.

**Measured before any change of this slice** (`🗑️generated/b3f-sourcing-before.txt`, the same
five-clause probe with the control located correctly): pressing the real increment button gives
`mutated: true` — ledger row `create-curated-item item { object-id=beam-glulam-gl24h … }`,
`Check In (1)`, `edits 0 → 1`. **🪵️sourcing's document mutation was reachable in the React host all
along.** The honest defect is narrower and still real: the control had **no accessible name, no
role, no keyboard route and no bounds semantics** — three tiny buttons and a nameless readonly box.

### B3f.2 The React table stepper, implemented

`TableStepperCell` (new) in `📊️Table/🟦️.tsx` replaces the inline three-element `<div>`:

| clause | what it does now |
|---|---|
| dispatch contract | unchanged and now locked by a law: the cell's own `ActionDescriptor` with `{ delta: ±step }` merged into its args — byte-for-byte what `render_table_cell` / `table_cell_hit` do in the wgpu host (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2915`, `:3026`) |
| role | the readout carries `role="spinbutton"` + `aria-valuenow` / `aria-valuemin` / `aria-valuemax` / `aria-valuetext`, live from the node's `value`/`min`/`max` |
| keyboard | ↑/→ `+step`, ↓/← `−step`, PageUp/PageDown `±10·step`, Home → `min`, End → `max`; the dispatched delta is CLAMPED so a key can never cross a bound, and a no-op key dispatches nothing |
| bounds | decrement disabled at `min`, increment disabled at `max` (both directions measured) |
| labels | `ui.tableStepper.decrement` / `.increment` / `.value`, **en + de** (`Decrease`/`Verringern`, `Increase`/`Erhöhen`), composed with the **column's own label from the node** → `aria-label="Increase Curated"`, `aria-label="Curated"` on the readout |
| phone width | the buttons keep `data-slot="button-group"` / `"button-group-item"`, which is exactly what `.touch :is([data-slot="button-group-item"], …) { min-height: var(--layout-touch-min) }` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:689`, 2.75 rem) grows at phone width — a law asserts the slots so a rewrite cannot silently drop out of that rule |
| probe handle | `data-slot="table-stepper"` on the group and `data-stepper-control="decrement"|"value"|"increment"` + `data-stepper-for="<cell id>"` on the three controls |

No schema change was needed: `TableCell::Stepper` already carries `value/min/max/step/action`, and
the accessible name comes from the column label the scene already publishes — so nothing had to be
regenerated and no plugin needed a wasm rebuild.

**Seven new laws, all green** (`🗑️generated/b3f-stepper-laws.txt`, `…-laws2.txt`, run at
`SEMIO_TEST_LEVEL=long` over `🔬️engine-contract/🟦️.ts`), plus the four pre-existing table/stepper
laws re-run green beside them:

```
✓ renders a stepper cell as a spinbutton with decrement, live value and increment controls
✓ dispatches the cell's own descriptor with the wgpu host's {delta} patch on increment and decrement
✓ disables the stepper control that would leave the cell's min/max bounds
✓ steps a stepper cell from the keyboard: arrows by one step, Home and End onto the bounds
✓ never dispatches a keyboard step that would cross a bound
✓ keeps the stepper buttons on the button-group slots the touch stylesheet sizes at phone width
✓ tableStepperKeyDelta and tableStepperClampedDelta agree with the wgpu segment contract
```

A `StepperBounds` story was added to the host's existing storybook entry (`📖️stories/🧪️.story.tsx`),
showing a row on `min` and a row on `max` beside a free one.

**Verified live on the running serve, no rebuild:** the TS-only change was picked up by the vite
serve on :6081 and re-censused — `data-slot="table-stepper"` present, `role="spinbutton"`,
`aria-label` `Decrease Curated` / `Curated` / `Increase Curated`, minus disabled at 0
(`🗑️generated/b3f-stepper-dom2.txt`).

### B3f.3 Which other plugins emit `TableCell::Stepper`

**Exactly one: 🪵️sourcing — and none of the other thirteen B3d variants.** Grep over every `.rs` in
the tree gives three sites, two of them sourcing's own windows and the third a framework round-trip
test:

| site | cell | action |
|---|---|---|
| `🪵️sourcing/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs:68` | `curated` | `curationSetCount` (this slice's measured control) |
| `🪵️sourcing/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs:82` | `count` | `curationSetCount` — the SAME cell renderer, so the fix covers it too |
| `🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-value-round-trip/🦀️.rs:101` | — | value round-trip fixture |

Two near-misses that are NOT this cell and were checked rather than assumed: 📐️cad's
`{ kind: "stepper", label, value, min, max }` (`✏️editor/⚙️engine/🎬️actions/🟦️.ts:1335`, `:1364`,
`📺️renderer/🟦️.tsx:4660`) is a `WindowEngagementControl`, rendered by `🛠️ShellHelpers` and already
working; and `🪜️Stepper` in `🖱️ui/🧱️elements` is the detail-panel control, a different component
with its own draft/commit lifecycle. So the renderer gap fixed here blocked one plugin only, and
`TableCell::Stepper` is the least-exercised cell kind in the vocabulary.

### B3f.4 The shared probe's blind witness — the OTHER half of sourcing's 3/5

Running the fixed control through the SHARED probe (`🐍️b3a-interaction-probe.mjs`, `rowSelector`
mode) still scored `mutated: false` on the first attempt — while the very same report's render
witness showed the Curated pane going
`No data` → `Glulam GL24h 200×400 24` → `No data` → `Glulam GL24h 200×400 24` across
invoke / undo / redo. Every ledger witness in that run read `edits: -1` with an empty ledger.

Cause, and it is a probe defect that can mis-score ANY app: **raising the History tab is a toggle,
not "bring to front"**. In `rowSelector` mode the probe raises the app panel, presses the row, then
presses the History tab — but `open-actions` had already left History as the raised tab of that dock,
so the press CLOSED it, and `#s-checkin` plus the whole `framework.history.entry.*` list went with
it. Fixed additively in the shared probe: a new `raiseHistory()` reads the live panel set and presses
only when History is not already open; all four raise sites now go through it
(`🐍️b3a-interaction-probe.mjs:126`, `:242`, `:331`, `:458`, `:484`).

**🪵️sourcing, re-measured on :6081, 5/5** (`🗑️generated/b3f-sourcing-bar2.txt`,
`🗑️generated/b3a-b3f-sourcing/`):

```
SUMMARY {"ready":"sourcing","error":null,"exampleRendered":true,"actionCount":16,
         "mutated":true,"undone":true,"redone":true,"panelRoundTrip":true,
         "faultLines":0,"interactionBar":true}
```

with the ledger witness now readable: `edits 0 → 1`, applied row
`framework.history.entry.19: create-curated-item item { object-id=beam-glulam-gl24h count…`, then
`entry.20: Undo` (`edits → 0`), then `entry.21: Redo` (`edits → 1`). Undo and redo both landed on
the FIRST candidate, the active window's own `action.undo`/`action.redo` row.

**No regression from the probe fix**: 💠️lowpoly re-run through the same probe on :6078 is still
`mutated/undone/redone: true`, `faultLines: 0`, `interactionBar: true`
(`🗑️generated/b3f-lowpoly-regression.txt`).

**sourcing's SECOND stepper cell measured too** (`🗑️generated/b3f-curated-stepper.txt`): the Pool
press creates the curated item at `count 1`, then the Curated window's own `count` stepper raises it
to `2` and lowers it back to `1`, each control carrying `role="spinbutton"`, a live `aria-valuenow`
and `aria-label="Count"`, zero console faults. One renderer, both windows, proven rather than assumed.

### B3f.5 🎪️demonstrator rebuilt and re-measured — B3d's fix proven at runtime, 5/5

The mutex was granted to this slice at **16:22**; `📜️b3f-activate.sh demonstrator` finished
**exit=0 at 16:56**, 64 nx tasks (mostly cache hits behind B3e's interrupted 46 —
`🗑️generated/b3f-demonstrator-activate.txt`). B3d's serve on :6107 picked the new guest up without a
restart, and the shared probe gives (`🗑️generated/b3f-demonstrator-bar.txt`,
`🗑️generated/b3a-b3f-demonstrator/`):

```
SUMMARY {"ready":"demonstrator","error":null,"exampleRendered":true,"actionCount":11,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

`changeSchema` — demonstrator's ONE document mutation, which B3d measured being **refused** with
*"returned snapshot read requires its exact owned-snapshot retirement factory"* and which then
poisoned the shell into `data-semio-os-error="demonstrator"` — now applies, undoes and redoes with
**zero fault lines**. **B3d's eight store owner/disposer declarations
(`🎪️playground/…/✏️editor/🦀️.rs:310`) are the cure, and this is their first runtime proof.**

### B3f.6 🏗️fem2d / 🏗️fem3d rebuilt and re-measured — fem2d 5/5, fem3d's failing clause MOVED

`📜️b3f-activate.sh fem2d` (one component serves both variants) **exit=0 at 17:44**, 12 nx tasks,
`@semio-tech/fem-plugin:component-dev` 2 m 47 s (`🗑️generated/b3f-fem2d-activate.txt`). Both serves
(:6086, :6087) took the new guest without a restart.

| variant | SUMMARY | verdict |
|---|---|---|
| 🏗️fem2d | `mutated:true, undone:true, redone:true, panelRoundTrip:true, faultLines:0` | ✅ **5/5** |
| 🏗️fem3d | `mutated:true, undone:false, redone:false, faultLines:0` — **twice**, two independent runs | 🟡 **4/5** |

**B3d's `[DEBUG]` removal is confirmed for BOTH**: `faultLines: 0` on each. The `[DEBUG]` lines still
in the console (`fem3d editor boot snapshot`, `live_visual reconcile spawn`, `setActiveExample`) are
the six B3d deliberately kept — they are `console.debug`, they carry no `fault`/`refused`/`panicked`
token, and they do not enter any bar's fault count. So the thing that held fem2d at 4/5 is gone and
fem2d clears the bar outright.

**fem3d's remaining red is now UNDO, not logging, and it is not flaky — it is a silent no-op.**
The measured chain (`🗑️generated/b3a-b3f-fem3d-console.txt`, `🗑️generated/b3f-fem3d-bar{,2}.txt`):

```
entry.8: create-support support=support id=sup8 node-id="" fixed=[ Tx …     ← edits 0 → 1
[DEBUG] fem3d live_visual reconcile spawn: … revision=12856258927874554027 generation=2
[DEBUG] fem3d live_visual reconcile spawn: … revision=12856258927874554027 generation=2   (×5)
```

Every undo route was pressed and ACCEPTED — the active window's own `action.undo` row, its per-rail
copy, `Meta+z` and `Control+z` all answered `ok` — and across the whole 60 s budget the guest
republished the **identical revision** five times running: no new `Undo` ledger row, `edits` stuck at
1, and **zero refusals or faults in the console**. So the press reaches the guest and the guest's
undo lane silently declines to move the document; this is not the probe failing to find a control
(that failure mode prints `absent`, and here it printed `ok`), and not the dispatch being refused
(that prints `dispatch-failed`). B3d measured `addSupport` undoing cleanly on the PRE-rebuild guest,
so this is either a regression in today's tree or a state-dependent lane — **the honest statement is
that fem3d, on a guest built at 17:44 from today's tree, does not undo the support it just created,
twice, without saying why.** Handoff for fem's owner; not chased here (a wasm rebuild per attempt and
🖨️raster was queued behind this one).

### B3f.7 🖨️raster rebuilt and re-measured — B3e's undo fix WORKS, the trap moved to redo

`📜️b3f-activate.sh raster` **exit=0 at 18:00** (`🗑️generated/b3f-raster-activate.txt`,
`@semio-tech/framework-surface-rs:wasm` 1 m 46 s), measured on B3d's serve :6060 through the
artifact-tree row `Add Pixel` (`SEMIO_PROBE_ROW='[role="treeitem"]:has-text("Add Pixel")'`):

```
SUMMARY {"ready":"raster","exampleRendered":true,"mutated":true,
         "undone":true,"redone":false,"faultLines":3}
```

**The undo trap B3d found (§3.1) and B3e fixed (§B3e.2) is gone, proven live**: the `create-layer`
row applies (`edits → 3`), `Meta+z` lands, the ledger gains `entry.21: Undo` and `edits → 2`. Under
B3d's measurement the first undo aborted the guest and every later dispatch in that shell was
refused; none of that happens now. **B3e's four retirement edits are confirmed at runtime.**

**What is still red is REDO, on the same assertion, and it is a NEW finding** — the three fault
lines are all from the redo press:

```
error   thread '<unnamed>' (1) panicked at 🗿️artifacts/🖨️raster/📦️packages/🦀️rust/../../🦀️.rs:301:9:
warning input #22 redo refused: dispatch-failed (user window=raster-navigator) — unreachable
warning input #23 redo refused: dispatch-failed (user window=raster-navigator) — {"tag":"fault",…
```

`🦀️.rs:301` is the same `RasterOwnedMap` fail-closed `Drop`. B3e overrode all THREE retirement seams
the trait offers — `MutationDiff::retire_projection`, `MutationDiff::retire_cold` and
`Mutation::retire_cold` (`📡️replication/🎮️mutation/🦀️.rs:128`, `:141`, `:186`) — and the undo walk
now goes through them cleanly, so **the redo lane drops an owned raster value through a fourth path
that is none of those three**. Not chased here: locating it needs a native stack the way B3e got the
undo one (a mounted law, 0.42 s once the crate is built) plus one ~40 min wasm rebuild per attempt,
and this slice's mutex slot was spent on the four activations. **Handoff for raster's owner, with
the cut already narrowed from "undo kills the session" to "redo drops one value off the retirement
path".**

### B3f.8 The fleet wasm mutex — all four activations landed

All four queued re-measures got their activation. What was measured about the queue, all by pid:

| time | event |
|---|---|
| 14:51 | inherited: mutex held by `a3`; B3e's `📜️b3e-activate.sh demonstrator` (pid 98359, queued 13:58) still ALIVE and waiting |
| 15:05 | B3e's orphan was granted the mutex and started `activate-demonstrator-react-dev` — this slice left it alone, it is exactly this work |
| 15:35 | that run was **SIGINT-ed from outside this slice** (`exit=130`) after 29 m 27 s with **46 of 55 nx tasks ✔** and 9 `component-dev` tasks failed/interrupted (`process-extension-*`, `puzzle-plugin`, `sourcing-*`) — `🗑️generated/b3e-demonstrator-activate.txt`. A re-run is therefore mostly a cache hit |
| 15:35 | this slice queued its own `📜️b3f-activate.sh demonstrator` (detached, pids 69920/69925, capture `🗑️generated/b3f-demonstrator-activate.txt`) |
| 15:37→16:15 | `tc1` held the mutex for 38 min at 0 % CPU with `pgrep rustc` = **0** — the rule-23(a) signature, but on a PEER's process (`trusted-stdio-gis-bootstrap`, pid 43985/71296), so nothing was killed. Reported here for tc1's owner |
| 16:22 | **granted to b3f** — `demonstrator` built, exit=0 at 16:56 (§B3f.5) |
| 16:57 | queued `fem2d`; `tc1` took the lock again at 17:00 and sat another 18 min at 0 % CPU with `pgrep rustc` = 0 — same peer, same signature, again left alone |
| 17:37 → 17:44 | **b3f** built `fem2d` (one component, both variants), exit=0 (§B3f.6) |
| 17:44 → 18:00 | **b3f** built `raster`, exit=0 (§B3f.7) |

**One peer problem to report rather than touch:** `tc1`'s `trusted-stdio-gis-bootstrap` (pids
43985/71296, then 1327/29938) held the fleet mutex twice — 15:37→16:15 and 17:00→17:18 — each time
at **0 % CPU with zero `rustc` anywhere on the machine**, i.e. preamble rule 23(a)'s deadlock
signature. It is not this slice's process, so nothing was killed (rule 15); its owner should check
it, because while it holds the lock the whole fleet's wasm queue stops.

**A detail worth keeping**: B3e's own `demonstrator` activation orphan (pid 98359, queued 13:58)
outlived B3e's session by 1 h 20 m, was granted the lock at 15:05 and was **SIGINT-ed from outside
this slice at 15:35** with 46 of 55 nx tasks already ✔. Because nx cached those, this slice's re-run
finished in 34 min instead of starting cold — so the interrupted run was not wasted.

### B3f.9 Files changed, gaps, servers

**Product source (4 files, one root fix — the React table stepper; every one shared with peers who
hold their own uncommitted edits in the same files, so the line counts below are this slice's own):**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx` — `TABLE_STEPPER_PAGE`, `tableStepperKeyDelta`, `tableStepperClampedDelta`, `TableStepperCell`; `renderTableCell` now takes the column label (+102 −9).
- `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` — re-exports the two helpers beside `TableHost` (+2 −2).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` — `ui.tableStepper.{decrement,increment,value}` in the catalog interface (+10).
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — the en AND de entries for those three keys (+10).

**Tests / stories (2 files):**

- `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — seven stepper laws (§B3f.2), all green, plus the two helper imports.
- `…/🧱️elements/📊️Table/📖️stories/🧪️.story.tsx` — the `StepperBounds` story.

**Shared probe (1 file, additive, owned by B3a):** `🐍️b3a-interaction-probe.mjs` — `raiseHistory()`
and its four call sites (§B3f.4). Default behaviour unchanged where History was closed; proven
non-regressive on 💠️lowpoly.

**Ticket folder (new):** `📜️b3f-activate.sh`, `🐍️b3f-stepper-dom.mjs`, `🐍️b3f-stepper-click.mjs`,
`🐍️b3f-sourcing-stepper-probe.mjs`, `🐍️b3f-curated-stepper-probe.mjs`. Captures: `🗑️generated/b3f-*`,
`🗑️generated/b3a-b3f-*`. **No `🗑️generated` file this slice did not create was removed**, and
`📌️important.md` / `🎫️ticket.json` were not touched.

**Honest gaps, in priority order:**

1. **🖨️raster's redo still traps** (§B3f.7) — the undo half is fixed and proven; redo drops an owned
   raster value through a path that is none of the trait's three retirement seams. Narrowed, not fixed.
2. **🏗️fem3d does not undo `addSupport`** (§B3f.6) — silently, twice, with the guest republishing an
   identical revision five times and no refusal. B3d measured this undo as green pre-rebuild, so it
   is either a regression in today's tree or a state-dependent lane. Not chased.
3. **💠️lowpoly / 🎥️shooting / 🪵️sourcing are still measured on 09-19 STAGED components**
   (B3e gap 4 stands) — sourcing's 5/5 here is on that staged guest plus this slice's TS-only React
   fix, which needs no wasm; a rebuild would make the three rows unconditional.
4. **The React stepper is proven on 🪵️sourcing only**, because sourcing is the only plugin that emits
   `TableCell::Stepper` (§B3f.3) — both of its cells were driven, but no second plugin exercises it.
5. **`@semio-tech/framework-renderer-react`'s `typecheck` has 132 pre-existing errors** (`ImportMeta.dir`,
   `Bun` globals, a peer's in-flight `pointerCancelScreen`), none of them in any file this slice
   touched (`🗑️generated/b3f-typecheck.txt`); the slice's own code is covered by the seven green laws.
6. **`tc1`'s mutex stall** (§B3f.8) — a peer's, reported not touched.

**Servers**: none started and none killed by this slice. B3d's and B3e's serves were reused as they
stood and all are still listening — 6019 shooting (pid 15046), 6060 raster (98222), 6078 lowpoly
(15037), 6081 sourcing (15048), 6086 fem2d (823), 6087 fem3d (851), 6107 demonstrator (866), plus
6020 cad. The three rebuilt guests (demonstrator, fem, raster) were picked up by their existing
serves without a restart. **Processes this slice started**: three detached activations through the
fleet mutex (`📜️b3f-activate.sh`, pids 69920/69925, 28473/28477, 59482) — all three exited 0 and are
gone. Nothing of a peer's was stopped.

