# 📓️ audit-coverage-2 — plugin/app/pane/lane coverage re-audit (session 6, 2026-09-22 11:13, Sonnet, read-only)

Re-runs the 09-20 coverage audit (`📓️audit-artifacts.md`) against CURRENT disk state: the 34 plugin
directories, the 69-pane catalog (`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`), play's 28 activation
lanes (`📋️project.json` `activate-dev` `dependsOn`), and the 28 lane `🔣️receipt.json` files on disk from the
**03:04** activation (`🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/<lane>/activation/🔣️receipt.json`).
Method: read `🔣️.json` descriptors for all 34 plugin dirs directly, parsed `PLAYGROUND_BUILD_TARGETS` /
`PLUGIN_BUILD_TARGETS` / `EXTENSION_TARGETS` from the generated registry sources, cross-checked against
the play unit suite (`playpanecoverage`, `playpanedefaults`) which independently encodes the same law, and
read the 28 on-disk receipts to get the actually-BUILT component closure (not just the statically-computed
one). Also read `$T/🗑️generated/e2e/test-e2e-0340.txt` (the 03:40 strict acceptance run, 70 tests) and ran
the play unit suite (`cd 🏢️semio-tech/🎡️play && bun ./📜️script.ts test`).

## Headline: the 09-20 gaps are CLOSED at the static/coverage level

Both `playbook` and `stdio` — the two plugins `📓️audit-artifacts.md` (09-20) reported with **no committed
root `🔣️.json`** — now HAVE one (`playbook` since Sep 21 18:32, `stdio` since Sep 21 11:19). All 34 plugin
directories currently commit a descriptor; the play unit test "reads a committed descriptor for every
plugin directory" passes. Every one of the play unit suite's `playpanecoverage` assertions is GREEN right
now (`lists every playground app exactly once`, `shows every plugin directory in at least one pane`,
`reaches every editor app…`, `reaches every viewer app…`, `mounts every pane with exactly the shell props…`,
`activates every registry component the panes need`, `keeps stdio's component bounded…`). Independent
verification below confirms the same conclusion from source, not just from the test's own logic.

The one remaining red bucket is **curated-example staleness**, isolated to stdio (2 of 63 unit tests), and
a live plugin defect in `wfc` (3 of 70 strict-acceptance tests) plus a peer-owned `playbook` defect (1 of
70) — none of these are coverage/mapping gaps; the mapping itself is correct in every case.

## 1. Plugins × apps × panes × lanes (34 plugins, 69 panes, 28 lanes)

Every plugin directory now commits a descriptor. "Apps" = editor apps declared by that descriptor. "Panes"
= catalog panes whose `pluginId` is this plugin (host-only `s.space.studio` editor app is exempt by design,
same as `📓️audit-artifacts.md`). "Lane" = the play `activate-dev` lane whose on-disk 03:04 receipt actually
stages this plugin's wasm component (confirmed by reading `🔣️receipt.json`, not inferred).

| plugin (dir) | pluginId | descriptor | editor apps | panes (n) | staged by lane |
|---|---|---|---|---|---|
| 🎞️animate | animate | ✅ | 1 | 1: animate | `animate` (own) |
| 🏛️architect | architect | ✅ | 1 | 1: architect | `architect` (own) |
| 🧱️block | block | ✅ | 3 | 3: block2d, block3d, block5d | `block2d` (own; shared component) |
| 📐️cad | cad | ✅ | 1 | 1: cad | `demonstrator` (shared) |
| 🕸️dag | dag | ✅ | 1 | 1: dag | `dag` (own) |
| 🎪️demonstrator | demonstrator | ✅ | 7 (4 re-exported) | 1 real (`demonstrator`) + 6 excluded `entwerfen-mit-bestand-*` brand rows by design | `demonstrator` (own) |
| 🖍️draw | draw | ✅ | 1 | 1: draw | `draw` (own) |
| 🔋️energy | energy | ✅ | 1 | 1: energy | `energy` (own) |
| 🏗️fem | fem | ✅ | 2 | 2: fem2d, fem3d | `fem2d` (own; shared component) |
| 🌊️flow | flow | ✅ | 1 | 1: flow | `flow` (own; **peer-owned guest code**) |
| 📋️forms | forms | ✅ | 1 | 1: forms | `forms` (own) |
| 🌍️gis | gis | ✅ | 2 | 2: gis2d, gis3d | `demonstrator` (shared) |
| 📜️imperative | imperative | ✅ | 1 | 1: imperative | `imperative` (own) |
| 📏️layout | layout | ✅ | 1 | 1: layout | `layout` (own) |
| 💠️lowpoly | lowpoly | ✅ | 1 | 1: lowpoly | `lowpoly` (own) |
| ➗️mathematical | mathematical | ✅ | 1 | 1: mathematical | `mathematical` (own) |
| 📕️norm | norm | ✅ | 15 | 15: din4108/din16798/din18599/en1990-1999/iso16757/vdi3805 | `din4108` (shared; **peer-owned**) |
| 🗒️note | note | ✅ | 1 | 1: note | `note` (own) |
| 📖️playbook | playbook | ✅ (now) | 1 | 1: playbook | `playbook` (own; **peer-owned guest code**) |
| 🌀️procedural | procedural | ✅ | 2 | 2: generation2d, generation3d | `demonstrator` (shared; **peer-owned**) |
| 🏭️process | process | ✅ | 1 | 1: process3d | `demonstrator` (shared) |
| 🧩️puzzle | puzzle | ✅ | 3 | 3: puzzle2d, puzzle3d, puzzle5d | `demonstrator` (shared) |
| 🖨️raster | raster | ✅ | 1 | 1: raster | `raster` (own) |
| 💡️reasoning | reasoning | ✅ | 1 | 1: reasoning-wires | `reasoning-wires` (own) |
| 📸️remodel | remodel | ✅ | 1 | 1: remodel | `remodel` (own) |
| 🎬️sequence | sequence | ✅ | 1 | 1: sequence | `sequence` (own; **peer-owned guest code**) |
| 🎥️shooting | shooting | ✅ | 1 | 1: shooting | `shooting` (own) |
| 🪵️sourcing | sourcing | ✅ | 1 | 1: sourcing | `demonstrator` (shared) |
| 🪐️space | space | ✅ | 3 (1 host-exempt) | 2: home, space | `home` (own; **peer-owned guest code**) |
| 🗄️stdio | stdio | ✅ (now, **STALE** — see §3) | 9 | 9: stdio, stdio-csv/-tsv/-txt/-json(-i)/-xml(-valid)/-html | `stdio` (own) |
| 🔱️trinity | trinity | ✅ | 2 | 2: trinity-jack, trinity-rewriting | `trinity-jack` (own; shared component) |
| 🌿️vcs | vcs | ✅ | 1 | 1: vcs | `vcs` (own) |
| 🀄️wfc | wfc | ✅ | 5 | 5: bitmap, grid2d, grid3d, wfc2d, wfc3d | `wfc2d` (own; shared component — **live defect, see §4**) |
| ✒️writer | writer | ✅ | 1 | 1: writer | `writer` (own) |

Totals: 34/34 plugins have a committed descriptor and ≥1 pane; 76 editor apps declared, 69 catalog panes,
28 lanes. **No plugin directory and no editor app is unreachable.** The union of the 28 on-disk lane
receipts' `plugins[].pluginId` (60 entries: 34 build targets + 26 extension targets) exactly equals the
full `PLUGIN_BUILD_TARGETS ∪ EXTENSION_TARGETS` id set — verified directly from the receipts, zero missing,
zero extra (beyond `space` appearing via the `home` lane, which is correct: `home`/`space` both need the
`space` plugin component). The `s` host lane is NOT one of play's 28 lanes and play's merge never reads
it — play never depends on the peer's host-shell receipt.

## 2. Extensions (26 targets) — all activated

`playRuntimeComponentIds()` ∪ receipts on disk cover cad×4, flow×9 (bim/brep/dictionary/draw/list/logic/
math/primitive/text — `draw` only via the `flow` lane, not `demonstrator`, but still covered), imperative×5,
playbook-module-procedural×1, process×4, sourcing-module×3 = 26/26. No extension is orphaned (no
`contributes` with no matching `consumes` closure); the previously-fixed `imperative.module` closure
(session 3 finding, "5 imperative-extension-* components in NO activation union") stays fixed.

## 3. Missing / invalid curated examples

**No pane is entirely without a curated example that should have one.** 17 panes carry no `example` field
by design, per `📓️default-example.md`'s law and the play unit test's `EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS`:
`demonstrator` (1), `flow` (1), and all 15 `norm` panes — these plugins' examples never reach the navbar
picker, so the catalog intentionally curates none. Verified: none of these 17 have `example` set, and the
unit test asserting exactly this passes.

**Invalid**: 8 of 9 stdio panes (`stdio-txt`, `stdio-csv`, `stdio-tsv`, `stdio-json`, `stdio-json-i`,
`stdio-xml`, `stdio-xml-valid`, `stdio-html`) curate `example: "demo"`, but the currently-committed
`✏️s/🔌️plugins/🗄️stdio/🔣️.json` (mtime **Sep 21 11:19**) publishes exactly ONE example total
(`demo`, dialect `s.stdio.md@commonmark/*` only) — the play unit test `names only examples the pane's own
app publishes` fails with all 8 as `"demo" is not published by <app> (descriptor: none)`.

**Root cause, verified on disk, not guessed**: this is a STALE DESCRIPTOR, not a missing feature.
`📓️stdio-examples.md` (session 5, 2026-09-21 14:30–17:xx) landed a real `demo` example directory with
`🦀️.rs`+`🟦️.ts` under EVERY one of the other 8 artifact subsets (`📊️csv/…/🎬️demo`, `📑️tsv/…/🎬️demo`,
`🔤️txt/…/🎬️demo`, `🧾️json/…/base+i-json/🎬️demo`, `📰️xml/…/base+valid/🎬️demo`, `🌐️html/…/🎬️demo` — all
confirmed present on disk with a `🟦️.ts`). But every `describe` re-run for stdio since then has failed
(status.md 20:30 "puzzle owned describe() hit the 30-min epoch deadline"; 22:45 "stdio describes in 1764s
on a quiet machine… loses the guest epoch"; 03:35 "describe run 3 actually refreshed 30/34… failed: cad,
stdio, gis, puzzle") — so the committed descriptor is still the pre-stdio-examples one. Once a successful
`describe` lands for stdio, all 8 should resolve (the disk evidence is unambiguous).

Additionally, the 9th stdio pane (`stdio` itself, md/commonmark) DOES find its curated `demo` example in
the descriptor, but the play unit test's second failure (`names every pane whose curated example cannot
reach its app…`) flags it as INERT: the committed descriptor's `s.stdio.md@commonmark/*#editor` app
declares no `setActiveExample` action (app-level actions list and its one window kind's actions both omit
it), so `appSwitchesExamples` gates the picker to `[]` and the pane boots the genesis doc. This is the SAME
stale-descriptor root cause: `📓️stdio-examples.md` §"What landed" states all nine editors gained the
action via a shared `set_active_example_action()` helper in
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:1022` (confirmed present in the guest source), but
the committed descriptor predates it. `stdio` is currently OUTSIDE the unit test's documented
`PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` allow-list (that list only names generation2d, lowpoly, layout,
gis3d, mathematical, sequence, vcs, home, space, architect, dag, imperative, trinity-rewriting) — so this
is a genuinely new red the allow-list has not yet absorbed, correctly surfaced as a fresh failure rather
than silently passing.

Net: **9 stdio panes total have an inert/invalid curated example**, entirely attributable to one stale
descriptor; no other plugin has an invalid curated-example id right now.

## 4. Panes the strict acceptance spec does NOT test

**None.** `🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` is GENERATED from the same
`🔨️modules/🧩️runtime/🔣️.json` catalog the panes come from (`playPanes()` reads the catalog file directly,
then `for (const pane of playPanes()) test(\`boots ${pane.label} (${pane.variant})\`, …)`) — so structurally
every catalog pane gets exactly one acceptance test; drift between "69 catalog panes" and "tested panes" is
not possible without editing the spec itself. The 03:40 run (`$T/🗑️generated/e2e/test-e2e-0340.txt`) shows
70 total tests = 69 per-pane `boots …` tests + 1 `lists one overview card per app` aggregate test.
Cross-checked the 69 unique pane labels/variants extracted from the log against the 69 catalog panes:
exact match, zero missing either direction.

Of the 69 per-pane tests, 4 currently FAIL (66 passed, 7.1 min):
- **`boots WFC 2D (wfc2d)`**, **`boots WFC 3D (wfc3d)`**, **`boots Grid 3D (grid3d)`** — shell outcome
  `"error"` (detail string equals the variant id) at app registration. This is a live `wfc` plugin defect,
  not a mapping gap: the pane IS correctly cataloged and IS correctly mapped to the `wfc2d` lane's shared
  `wfc` component (confirmed — receipt lists `wfc`); the plugin fails to register these 3 of its 5 apps at
  runtime. The other 2 wfc-family panes (`grid2d`, `bitmap`) pass. Routed to the `design` topic per
  status.md.
- **`boots Playbook (playbook)`** — console error `"setContributions command failed playbook presence
  local read requires a live exact local retirement owner"`. Playbook is correctly cataloged and correctly
  staged by play's own `playbook` lane; the defect is in playbook's peer-owned guest code (routed to peer
  S10 per status.md, unresolved as of 04:00).

Neither failure is a coverage/mapping defect — both panes exist, are correctly cataloged, and are staged by
the correct lane; they fail live for plugin-internal reasons outside this audit's coverage question.

## 5. Peer-owned plugins (space, norm, procedural, playbook, sequence, flow)

All six have their guest Rust/wasm code owned by the peer session (`End-to-end repo completion`,
26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END) per brief v4/v5's no-touch list. Coverage-wise, all six ARE
correctly reached by play — the question is which lane stages them and whether that lane is play's own or
the peer's `s` host lane:

| plugin | panes play needs | staged by | play-owned lane? | current live state |
|---|---|---|---|---|
| flow | flow | `flow` lane | play's own (28-lane list) | strict e2e passes; guest diffs already applied with peer approval (session 5 16:50) |
| sequence | sequence | `sequence` lane | play's own | strict e2e passes; native crate has 44 test-side reds (guest peer-owned, not a play-boot blocker) |
| playbook | playbook | `playbook` lane | play's own | strict e2e **FAILS** (see §4) — peer-owned guest defect, routed S10 |
| space | home, space | `home` lane | play's own | strict e2e passes for both; curated-example inert for both (`home`/`space` declare no `setActiveExample` — allow-listed, known) |
| norm | 15 din4108/din16798/…/vdi3805 panes | `din4108` lane (shared — ONE norm component for all 15) | play's own | strict e2e passes for all 15; exempt from the example-picker law by design, no gap |
| procedural | generation2d, generation3d | `demonstrator` lane (shared) | play's own | strict e2e passes for both; `generation2d`'s curated example is inert (editor declares no `setActiveExample` — allow-listed, known); `generation3d` unaffected |

**Play never depends on the peer's `s` host lane.** All six peer-owned plugins are staged through lanes
`activate-dev` itself triggers and that play's own fleet controls the activation timing for (subject to the
shared wasm mutex) — the peer's `s` lane build is a separate, parallel thing play's merged receipt does not
read. The only place "peer ownership" matters for coverage is: if a fix is needed inside one of these six
plugins' guest code (as with playbook's live console error, or stdio's — stdio is NOT peer-owned, it is
play's own scope), play's fleet can only hand over a proposed diff, per the no-touch list.

## 6. Play unit suite (confirmed by running `cd 🏢️semio-tech/🎡️play && bun ./📜️script.ts test`)

**63 tests total, 61 passed, 2 failed** (both in `play pane example defaults`, both stdio-descriptor
staleness — see §3). Breakdown by describe block (from the run): `playpanecoverage` all green,
`playactivation` green, `playgrid`/`playmaptiles` green, `playpanedefaults` 2 red. This is a regression in
absolute count from session 5's "63/63" (15:05) and session 4's "62/62" — NOT a new coverage defect, but
the stdio descriptor going stale again after a `describe` re-run failed to complete for it (per §3).

## Summary of gaps (plain text)

- **No plugin, no app, no extension, and no pane is missing coverage right now.** All 34 plugin
  directories commit a descriptor and have ≥1 pane; all 76 editor apps (minus the one host-exempt) are
  reachable; all 71 viewer apps are reachable; all 60 registry components (34 plugins + 26 extensions) are
  in the union of the 28 on-disk lane receipts from the 03:04 activation; all 69 catalog panes have exactly
  one strict-acceptance test (by construction, since the spec is generated from the same catalog file).
- **9 stdio panes carry an invalid or inert curated `example`**, all traceable to ONE stale plugin
  descriptor (`✏️s/🔌️plugins/🗄️stdio/🔣️.json`, committed Sep 21 11:19, before the `stdio-examples` guest
  fix that session landed ~14:30–17:50 the same day). The guest-side fix is verifiably on disk (demo
  example dirs + the `setActiveExample` action via a shared helper); only a successful `describe` pass for
  stdio is missing — every attempt since has failed on the guest-epoch/timeout issues status.md logs.
  `stdio` itself (the md pane, 1 of the 9) is a NEW inert-example red not yet in the unit test's documented
  allow-list.
- **4 of 70 strict-acceptance tests fail live, none are coverage gaps**: `wfc2d`/`wfc3d`/`grid3d` (wfc
  plugin app-registration defect, 3 of the wfc component's 5 apps, routed to `design`) and `playbook`
  (peer-owned guest console error, routed to peer S10).
- **17 panes have no curated example by design** (demonstrator ×1, flow ×1, all 15 norm panes) — confirmed
  intentional per the example-picker-exempt law, not a gap.
- **Peer-owned plugins (space, norm, procedural, playbook, sequence, flow) are all correctly staged by
  play's own 28 lanes**, never through the peer's `s` host lane; the only open issue among them is
  playbook's live console-error defect (§4/§5), which is peer-owned and already routed.
- Play unit suite: 61/63 (2 red, both the stdio descriptor staleness above) — down from session 5's 63/63
  because a later `describe` attempt for stdio never landed.
