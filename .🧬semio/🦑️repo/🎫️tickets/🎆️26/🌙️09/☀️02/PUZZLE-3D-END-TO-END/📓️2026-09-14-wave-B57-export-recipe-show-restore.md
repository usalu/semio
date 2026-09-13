# Wave B57 — the export press recipe, the example switch, and the Show restore

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-13 · wasm **#61** on `:6013` (host vite-live).
Written incrementally. Every command ran in the FOREGROUND with its tail quoted.

Inputs read first: `🗑️generated/battery-2026-09-13-64-6013-B55.txt` (the failing battery),
`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §0/§5/§7 (the export recipe),
`🔍️browser-probe.ts` (`activateWindowFileAction`, `export-import`, `example-switch`, `outliner-rows`),
`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (`instance_record_json`, `instance_record_fingerprint`,
`Puzzle3dInstanceResidency::refresh`), `🎮️commands/🔖️set-selection-flag/🦀️.rs`,
`🎮️commands/📥️import-fixture/🦀️.rs`, `✏️editor/🦀️.rs:1155` (`apply_puzzle3d_selection_flag`),
`📌️panels/🗿️artifact/🦀️.rs:126-150` (`flag_args`), `🔌️PluginRuntime/🟦️.tsx:2870-2915`
(`performInvocation`, the `importFixture ingress` tap).

---

## 0 Headline

| verdict | before (battery #64, B55) | after this wave | owner |
|---|---|---|---|
| `export-only` | FAIL `download=none` | **PASS** `nakagin-capsule-tower.json`, **145 714 B** | probe |
| `export-names-the-example` | FAIL | **PASS** | probe |
| `import-same-file-idempotent` | FAIL not reachable | **PASS** (see §2.3 — the PASS is weak by construction) | probe |
| `import-distinct` | FAIL not reachable | **FAIL, now reachable and diagnosed** — `paneObjects=180→180` | **product**, the host→guest payload hop (§2.3) |
| `import-distinct-records-history` | FAIL not reachable | **PASS** | probe |
| `example-switch` | FAIL `example=Concrete Forest` | **PASS** `label=Nakagin Capsule Tower bytes=266→54254` | probe |
| `outliner-show-restores` | FAIL `restored=false` | **PASS** `rowsRestored=true worldRestoresRowObject=true` | probe |

Nothing in this wave waits for #62: the two guest halves under suspicion (the un-hide's world
republication, the import's JSON decode) are both proven correct in the CURRENT source by laws (§3), and
the one remaining red is a host-side wire hop.

One blocker outside the ticket had to be cleared first (§4): a peer's in-flight edit in
`🌐️World3dHost/🟦️.tsx` left `sharedGumballTransformPreview` used 26 lines before its declaration, so every
boot died with `ReferenceError: Cannot access 'sharedGumballTransformPreview' before initialization` and
`canvases=0`.

---

## 1 `example-switch` — the retry sat BEHIND the verdict

**Root cause, `🔍️browser-probe.ts:686` (pre-edit):** the verdict was emitted, and only then the step
retried the switch. Battery #64's own log proves the lane was never broken:

```
[1390.8s] example options=[] pickerPresent=1 opened=false waitedMs=8048
[1413.8s] example after switch: Concrete Forest
[1413.8s] verdict example-switch FAIL example=Concrete Forest      ← written here
[1473.8s] example after nakagin retry: Nakagin Capsule Tower       ← fixed 60 s later, unreported
```

Two probe defects, both fixed:

1. **Ordering.** The switch is now attempted up to three times and the verdict is written after the loop.
2. **Route.** The listbox is opened POINTER first and KEYBOARD second (`focus()` + `Enter` on the
   `select-trigger`), and the option is committed pointer first, keyboard second (`ArrowDown` walks
   `[role="option"][data-highlighted]`, `Enter` commits). Battery #64 read `opened=false` on two forced
   presses with `mine:true` — nothing covers the trigger, the presses simply did not open it. The
   consolation `options.nth(1)` pick is gone: a switch that cannot find the wanted option must fail, not
   switch to whatever is second.
3. **Two observables, not one.** `dumpInstances` now also reports `bytes` (the published
   `data-instances-json` length), so the verdict reads the GUEST census next to the navbar label — B53
   hop 0's `266 → 54254`. A label that moves with no census move is a chrome-only switch; a census that
   moves under the old label is a switch the navbar did not render.
4. **Flag/lane mismatch.** The verdict was gated on `wantUndo`, so `--only=example-switch,export-import`
   ran the step and published no verdict for it at all. It is gated on `wantExample` now — the same class
   of defect the `family` comment at `:290` was written about.

**Live (run 5, `:6013`):**

```
[11.4s] example listbox route=pointer opened=true waitedMs=9 options=["No example","Concrete Forest","Nakagin Capsule Tower"]
[54.6s] example after switch attempt=0 route=pointer/pointer label=Nakagin Capsule Tower census={"count":180,"bytes":54254,…}
[54.6s] verdict example-switch PASS
[54.6s] verdict example-switch-instances PASS
```

Note `example=Concrete Forest census={"count":1,"bytes":266}` before the press and `180/54254` after: both
witnesses moved together.

---

## 2 The export/import lane — B53 §5/§7's recipe, ported, plus one hop B53 never reached

### 2.1 The press (B53's three budgets)

`activateWindowFileAction` now does, in order: `scrollIntoViewIfNeeded({timeout: 8000})` (the rail is an
`overflow-y:auto` scroller and `action.exportFixture` sits ~885 px below the fold, so `force: true` — which
waives the actionability CHECKS, never Playwright's scroll-and-hold — could never reach it), then a
hit-tested `click({timeout: 25000})` whose timeout is TOLERATED and logged (B53 §7: every check passes and
the press still hangs inside `performing click action`, yet it is delivered), and the `download` listener is
armed at **90 s** before the press (measured 3.7 s idle, 21-35 s loaded). `waitForEvent("filechooser")` went
25 s → 75 s for the same reason. `actionPaneState` now reports `rowCount` beside its 24-row slice — `Export`
is row 69 of 81 and looked "absent" for five waves.

### 2.2 The hop B53 did not measure: the pane never opens on the first press

Run 1 and run 2 of this wave both failed with the row ABSENT, not unreachable:

```
run 1  [340.5s] action-pane exportFixture rows=0 unfold={"unfolded":false,…} pane={…"folded":"true","rowCount":0,"rows":[]}
run 1  [390.2s] action-pane openImportFixture rows=1 unfold={"unfolded":true,…} pane={…"folded":null,"rowCount":81,…}   ← 16 s later, same pane, same toggle
run 2  [211.4s] action-pane exportFixture rowArrival=0 waitedMs=122498        ← 122 s of waiting changed nothing
```

So the toggle press is unreliable under the Nakagin document (`covered:false` — nothing is over it; the
renderer does not acknowledge it), and *waiting longer is not the fix*. The press has to be REPEATED. The
helper now runs up to three rounds — round 0 is `unfoldWindowPane` (pointer, B45's obstruction evidence
kept), rounds 1-2 press the toggle by keyboard (`focus()` + `Enter`) — each followed by a 45 s poll for the
row, and it skips the press entirely (with a log) when the row never arrives, instead of spending 33 s
clicking a locator that resolves to nothing. `unfoldWindowPane` re-reads `data-folded` before pressing, so
no round can fold the pane it needs. The log now prints the toggle press's own `click` outcome first: it was
being truncated off the end of the line by the obstruction chain.

**Live (run 4 and run 5 agree):**

```
run 4  [45.1s] action-pane exportFixture rowArrival round=0 rows=1 waitedMs=3
run 4  [45.2s] action-pane exportFixture rows=1 unfold={"unfolded":true,"click":"failed TimeoutError: click: Timeout 4000ms exceeded.","waitedMs":1731,"covered":false}
run 4  [47.3s] action-pane exportFixture press=ok
run 4  [69.9s] export download=nakagin-capsule-tower.json example=Nakagin Capsule Tower expected=nakagin-capsule-tower.json
run 4  [69.9s] verdict export-only PASS
run 4  [69.9s] verdict export-names-the-example PASS
```

`ls -l` on the saved file: **145 714** bytes — byte-for-byte B53 §0's figure. The toggle press still reports
`click: Timeout 4000ms exceeded` while the pane opens anyway (`unfolded:true waitedMs=1731`), which is the
product finding B54 owns, now recorded rather than fatal.

### 2.3 `import-distinct` — reachable, still red, and the guest is exonerated

Both imports now reach the guest. `import-same-file-idempotent` and `import-distinct-records-history` pass.
`import-distinct` fails with a reading no earlier run had:

```
run 5  [364.5s] verdict import-distinct FAIL [expect-41] before=180 after=180 paneObjects=180→180
               hostIngress=["[DEBUG] importFixture ingress {"name":"…-distinct.json","payloadType":"string","payloadLen":141574,"argKeys":["payload","name","windowId"]}"]
               hostSettled=[] documentTaps=["[DEBUG] history patch applied {…"labels":["Toggle Panel"]…}"]
run 4  same verdict, hostSettled present with frames:2 frameKinds:["Invocation","Ephemeral"] historyUpserts:0 effects:0
```

`paneObjects` is new in this wave: the Actions pane header renders the FIXTURE's own census
(`"180 Objects · 0 Attractions"`), so it answers independently of the world lane. It reads `180→180`. The
document did not grow — this is not a world-republication defect, the import did not apply.

What that leaves:

- the payload DOES cross the host boundary: `payloadLen=141574`, `argKeys:["payload","name","windowId"]`,
  exactly the shape `import_fixture` reads (`🎮️commands/📥️import-fixture/🦀️.rs:15-21`);
- there is NO refusal: no `Notify`, and `import_fixture`'s only failure mode is
  `ctx.notice(import_invalid) + ctx.abort`;
- the guest's decode is CORRECT for browser-written JSON — §3's new law pins the spellings
  `JSON.stringify` produces (`1` for a whole float, `-5.551115123125783e-17`, `"scale":null`, `·`) and
  passes, as does B9's existing exported-bytes round trip;
- in run 5 the invocation never settled at all (`hostSettled=[]`) while the same-file import in the SAME run
  settled in 20 212 ms.

**Leading hypothesis, for the plugin-host/reactor lane (B56's, untouched here):** the arg crosses as one
contiguous 141 574-char string through `encodePackValue` → `client.command`
(`🔌️PluginRuntime/🟦️.tsx:2914`), which is ~138 KiB in one guest allocation — far past the guest's
contiguous-request ceiling. The same-file import (144 766 chars) is the same size class and is *equally
unproven*: on the document it describes, an import that applies and an import that is dropped are the same
census. **`import-same-file-idempotent`'s PASS is therefore unfalsifiable by construction** and should be
re-cut against a witness the identity cannot fake (a history row, or the pane census after a round trip
through an EMPTY document) in whichever wave takes the wire hop.

Two stale-instrument defects were fixed while measuring this:

- the same-file settle waited on `[DEBUG] puzzle3d.import.apply`, **a tap that no longer exists anywhere in
  the guest** (B51's debug sweep removed it), so it could only burn its budget and report `applied=false` on
  an import that had landed. It now settles on the host's own `performInvocation settled` for
  `importFixture` (`applied=true waitedMs=20212` live);
- `guestTaps` filtered for the same dead tap and printed `guestTaps=[]` on every run, which reads as "the
  payload never crossed". It is `documentTaps` now and carries the document's `history patch applied` lines.

---

## 3 `outliner-show-restores` — the dump swept the History panel

**Root cause, `🔍️browser-probe.ts:3970` (pre-edit):** `rowDump` collected every
`[data-slot="tree-item"], [role="treeitem"]` in the DOCUMENT, so it carried the History panel's
`framework.history.entry.*` rows — which GROW while the step runs (`noteShellCommand` upserts one per
press). The Show verdict asks whether the dump came BACK to its pre-hide value, and a dump containing rows
that never come back can never answer yes. Battery #64's own note proves it: `restored=false waitedMs=30131`
with `afterShowHead` showing the `seed-left-001` row already reading `Hide` again.

The parent brief's hypothesis — that Show restores the row label but NOT the world scale, i.e. that the
residency key or the structural fingerprint misses `hidden` — is **disproven on both sides**:

- source: `instance_record_fingerprint` hashes `object.hidden`
  (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:256`), so `Puzzle3dInstanceResidency::refresh` re-serializes the
  record and `instance_record_json` re-emits the real scale (`:214`);
- live: with the dump scoped to the outliner's own id space and the verdict asking for the WORLD too,
  `worldRestoresRowObject=true`.

Fixes, both in the probe: `rowDump` is scoped to `puzzle3d-play-document` / `panel:puzzle3d-play-document/…`
(which also removes a false-green risk on `outliner-hide-applies`, whose inequality check was satisfiable by
History churn alone), and the Show verdict now polls rows AND `hiddenScales()` — symmetric with the Hide
half — and logs the dispatch tail.

**Live (`--only=outliner-rows --port=6013`):**

```
[14.1s] outliner hide clicked={…"text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"} rowObject=seed-left-001 waitedMs=2425
        worldHiddenBefore=[…"hidden":[]…] worldHiddenAfter=[{"surface":"window:puzzle3d-main-top","hidden":["seed-left-001"]},{"surface":"window:puzzle3d-main-perspective","hidden":["seed-left-001"]}]
[14.1s] verdict outliner-hide-applies PASS
[15.5s] outliner show console tail=[… "[DEBUG] history patch applied {…"labels":["change-object-hidden id=seed-left-001 new-hidden=false"],"canUndo":true}" …]
[15.5s] verdict outliner-show-restores PASS
        restored=true rowsRestored=true worldRestoresRowObject=true rowObject=seed-left-001
```

### 3.1 The law (new)

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
`outliner_show_restores_the_world_instance_scale_in_the_same_settle`: hide → the world lane publishes
`[0,0,0]`; show, same settle → it publishes `[1,1,1]` again and the projection reads `hidden=false`. It is
the restore half of the neighbouring `outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control`,
and it is the claim no row-text verdict can make.

```
running 2 tests
test …::outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control ... [DEBUG] outliner hide world lane scale=[0.0, 0.0, 0.0] rowIcon=Some("eye-off")
ok
test …::outliner_show_restores_the_world_instance_scale_in_the_same_settle ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 753 filtered out; finished in 0.44s
```

### 3.2 The second law (new), for §2.3

`a_browser_serialized_fixture_payload_imports_every_json_number_spelling`: the import payload is written by
a BROWSER, so it arrives spelled the way `JSON.stringify` writes it — a whole float as `1`, a tiny one as
`-5.551115123125783e-17`, an absent optional as `null`, a non-ASCII label as `·` — and never the way
this crate's writer spelled it on the way out. The pre-existing round-trip law only ever fed
`import_fixture` its own export text, so none of those spellings was under test, and
`import_fixture`'s abort path looks exactly like the browser's red (`effects:0 historyUpserts:0`, no notice).

```
running 2 tests
test …::a_browser_serialized_fixture_payload_imports_every_json_number_spelling ... ok
test …::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 754 filtered out; finished in 14.17s
```

---

## 4 The blocker: a peer's TDZ in `World3dHost`

Between 00:25 and 00:35 every probe died before its first step:

```
[17.8s] boot waiting… windows=2 canvases=0 faults=0     (×12, then booted=false)
error: [DEBUG] shell fault surface-world-3d ReferenceError: Cannot access 'sharedGumballTransformPreview' before initialization
```

`🌐️World3dHost/🟦️.tsx` used `sharedGumballTransformPreview` in two `useMemo`s at `:5332-5339` while its
`useSyncExternalStore` declaration stood at `:5359` — a temporal dead zone, so the component threw on every
render and no canvas ever mounted. The peer's own hunk was left byte-identical and MOVED below the
declaration (`previewVortices` / `previewAttractions` now sit at `:5356-5363`, directly after it); no
semantics, no dependency arrays and no hook set changed. Boot was green again immediately afterwards
(`canvases=2`, `verdict boot PASS`).

---

## 5 Verification — every command in the FOREGROUND, tails quoted

| command | tail |
|---|---|
| `bun 🔍️browser-probe.ts --only=boot --port=6013` | `done booted=true faults=0 hard=0 collateral=0 … verdicts=6` |
| `bun 🔍️browser-probe.ts --only=example-switch,export-import --port=6013` (run 4) | `export-only PASS`, `export-names-the-example PASS`, `import-same-file-idempotent PASS`, `import-distinct FAIL [expect-41]`, `import-distinct-records-history PASS`, `example-switch PASS`, `example-switch-instances PASS`, `guest-alive-replace PASS`, `battery-hard-faults PASS`, `battery-faults PASS` |
| same, run 5 (reproduction, with the `paneObjects` witness) | identical verdict set; `import-distinct … paneObjects=180→180` |
| `bun 🔍️browser-probe.ts --only=outliner-rows --port=6013` | `outliner-panel-opens PASS`, `outliner-hide-control-present PASS`, `outliner-hide-applies PASS`, `outliner-show-restores PASS`, `guest-alive-mutate PASS` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- outliner_show_restores outliner_hide_reaches --test-threads=1` | `test result: ok. 2 passed; 0 failed; … 753 filtered out` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- a_browser_serialized_fixture_payload exported_fixture_bytes_reimport --test-threads=1` | `test result: ok. 2 passed; 0 failed; … 754 filtered out` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- hidden --test-threads=1` | `test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 732 filtered out` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- residency --test-threads=1` | `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 754 filtered out` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished \`dev\` profile [unoptimized] target(s) in 38.09s` — 0 errors |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- import --test-threads=1` | `test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 745 filtered out; finished in 17.30s` — the WHOLE import lane, including the new §3.2 law and `leftover_import_fixture_replaces_live_document_with_distinct_two_object_json`, so the guest's distinct-import half is green in-process while the browser's `import-distinct` is red |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished \`dev\` profile [unoptimized] target(s) in 1m 58s` — 0 errors |

---

## 6 Files

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` — `activateWindowFileAction`
  (scroll, tolerated press budget, rounds + keyboard unfold, honest logs), `actionPaneState` (`rowCount`),
  `paneObjectCount` (new), `dumpInstances` (`bytes`), `example-switch` (routes, retry-before-verdict, two
  observables, `wantExample` gate), `export-import` (90 s download, 75 s choosers, live settle tap,
  `documentTaps`, `paneObjects` in the `import-distinct` note), `outliner-rows` (scoped `rowDump`, world in
  the Show verdict).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — two
  new laws (§3.1, §3.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — TDZ unblock
  (§4), a pure move of a peer's two `useMemo`s below the declaration they read.
- `🗑️generated/wave-B57-boot.txt`, `wave-B57-export-run1…5.txt`, `wave-B57-outliner-run1.txt` — run logs.

## 7 Handover

1. **`import-distinct` is the only red left in this lane** and it is a PRODUCT red on the host→guest
   payload hop, not on the press and not on the guest's decode (§2.3). It needs the wire lane: a ~138 KiB
   contiguous string arg through `encodePackValue` → `client.command`.
2. **`import-same-file-idempotent` needs re-cutting** — on the document it describes, an applied import and
   a dropped one are the same census, so its PASS proves nothing (§2.3).
3. The Actions-pane fold toggle still reports `click: Timeout 4000ms exceeded` while the pane opens anyway
   (B54's main-thread lane). The probe now survives it; the product finding stands.
