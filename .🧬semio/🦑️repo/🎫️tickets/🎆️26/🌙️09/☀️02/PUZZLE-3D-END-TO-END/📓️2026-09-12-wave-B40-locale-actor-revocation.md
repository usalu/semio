# Wave B40 — the locale switch that reloaded every plugin, the projection camera, the delete census

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · wasm #55 on `:6013` (coordinator's battery).
Written incrementally while the wave ran. Every command quoted below ran in the FOREGROUND with its tail.

Inputs read first: `📓️2026-09-12-wave-B38-probe-recipes-export-segments.md` §1.3/§1.5/§1.7,
`📓️2026-09-11-wave-B13-export-history-locale.md` §3, `📓️2026-09-11-wave-B17-reactor-close-trap.md`.

---

## 1 The locale switch revoked the plugin actor — FIXED

### 1.1 Root cause — the locale is a dependency of the plugin INSTALL graph

Not a `reloadPlugin` bug, not `update_view`, not the guest. The hop chain, `file:line`, all in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`:

| hop | where | what |
|---|---|---|
| 0 | `:8057` `setLocale` → `:7817` `commitUiPreference` → `SET_UI_LOCALE` | `uiLocale` changes — the intended, correct half |
| 1 | `:3411` `establishPrimarySession` (pre-fix `:3387`–`:3435`) | `useCallback(…, [hostConfig, appId, appRole, pluginFilter, uiTerminology, uiLocale])` — it reads `uiTerminology`/`uiLocale` directly (`:3401`, `:3423`) only to label the layout seed, so **its identity changes on every locale switch** |
| 2 | `:3469` `establishPrimaryWithShardRetry` → `:3491` `installPlugin` → `:3546` `reloadPlugin` | all three inherit that identity churn through their dep arrays |
| 3 | the `PluginSource` subscription effect, now `:4355`–`:4398` | `useEffect(…, [registry, pluginSource, installPlugin, reloadPlugin])` — a new `reloadPlugin` **tears the effect down and re-runs it** |
| 4 | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2373` `createDevPluginSource.subscribe` | every `subscribe` opens a **fresh `EventSource(watchUrl)`**, and the contract at `:2348` says so outright: *"Fires an immediate `snapshot` on subscribe … the dev source's SSE endpoint always sends one"* |
| 5 | the pump, now `:4368` | the replayed snapshot names ~20 already-built plugins; each reads `alreadyLoaded === true` and therefore routes to **`reloadPlugin`**, not `installPlugin` |
| 6 | `:3590` in `reloadPlugin` | for the session-owning plugin: `await current.handle.destroyApp(activeSession.instanceId)` → **`actor-activation.revoked`** (`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1583`), then `establishPrimarySession(newHandle)` mints a brand-new instance with an EMPTY document → `program puzzle: no channel for instance 1` for everything still addressed to the old one |

So B38 §1.7's signature is exact and its reading was right — the guest dies — but the cause is not the
locale reaching the guest at all. **Changing the UI language hot-swapped every loaded plugin.** The same
defect fires for `uiTerminology`, and fired for any other locale-carrying identity that ever leaked into
that dep chain.

### 1.2 The fix

Two changes, one root and one invariant.

**(a) `establishPrimarySession` is locale-stable.** It now reads `uiLocaleRef.current` /
`uiTerminologyRef.current` (`:2307`–`:2310`, the refs the surrounding code already uses at `:3315`,
`:3706`, `:4352` for exactly this reason) instead of the reactive values, and drops both from its dep
array. Correct, not a workaround: the seed labels must reflect the locale at the MOMENT a session is
established, which is what the ref gives, and the already-existing retitle effect (`:4855`, keyed `[uiTerminology, uiLocale]`) is the thing that keeps an established session's baked-in titles in
the current language. With that, `installPlugin` / `reloadPlugin` / the subscription effect no longer
change identity on a locale switch, so hop 3 never fires.

**(b) An availability event for an artifact already loaded is not a rebuild.** `pluginAvailabilityRouteV1`
in `🛠️ShellHelpers/🟦️.tsx` (beside `pluginShouldEstablishSession`) is now the ONE place that decides
whether an event may destroy a live instance — `install` | `hot-swap` | `drop` — with
`pluginArtifactRebuiltAtRef` in ShellHost recording what each loaded artifact was built from: the pump
hot-swaps only when the event's `rebuiltAt` is strictly newer than the `rebuiltAt` the loaded handle was
built from. A snapshot replay — reconnect, SSE drop, or any future effect re-run — is
then idempotent by construction and can never destroy a live instance and its document.

### 1.3 Files touched

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `establishPrimarySession` reads locale/terminology through `uiLocaleRef`/`uiTerminologyRef` and drops both from its deps; new `pluginArtifactRebuiltAtRef` + `recordPluginArtifactRebuiltAt`, recorded at both acquisition points and forgotten on uninstall (plugin and extension); the availability pump routes through `pluginAvailabilityRouteV1` and `continue`s on `drop` without consuming a worker slot |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | new `pluginAvailabilityRouteV1` / `PluginAvailabilityRouteV1`, beside `pluginShouldEstablishSession` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | re-exports both |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🔁️plugin-availability-route/🔣️.json` | new — 8 routing rows plus the 3-connect snapshot-replay scenario |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | new law `never hot-swaps a live plugin for a replayed availability snapshot` in the `plugin session ownership` region |

### 1.4 The law, run in the foreground

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config .../⚛️react/vitest.config.ts --reporter=verbose --silent=false -t "hot-swaps a live plugin"
stdout | ../../../../🧪️tests/🔬️engine-contract/🟦️.ts > plugin session ownership > never hot-swaps a live plugin for a replayed availability snapshot
[DEBUG] plugin availability route: rows=8 connects=3 installs=3 hot-swaps=0 drops=6
 ✓ ... > plugin session ownership > never hot-swaps a live plugin for a replayed availability snapshot 2ms
 Test Files  1 passed | 29 skipped (30)
      Tests  1 passed | 998 skipped (999)
```

Three connects replaying the same three-plugin snapshot: 3 installs, **0 hot-swaps**, 6 drops.

### 1.5 There is no `update_view` law to add — the revoke never crossed into the guest

`grep` over `🔨️modules/🔌️plugin/🦀️.rs`: there is **no `update_view`** entry point at all, and no locale
in it. The locale a guest renders against lives on `ViewModel::locale`
(`🔨️modules/🛂️manifest/🦀️.rs:4389` — non-optional, "the shell always resolves one … so 'nobody set the
locale' is unrepresentable"), which programs read through their own `app_labels!` resolver
(`✏️editor/🗣️terminology/🦀️.rs:132` `puzzle3d_labels`). So a locale change IS already a pure view-context
update on the wire: nothing in the guest can retire an instance in response to one, and the revoke
measured by B38 was entirely host-manufactured (§1.1). That is why there is no guest-side subject for a
"`update_view` with a changed locale re-renders, never retires" law — the laws that actually bound this
defect are §1.4 (the routing invariant) and the live verdicts in §4.

What the guest DOES owe is that its bodies come back in the new language once asked, and the host was
never asking — a separate, second defect that only became visible once this one was fixed. It is §3.

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib locale -- --test-threads=1
test component::app::artifact_definition_contract_tests::identities_and_locales_are_explicit_and_conflicts_do_not_overwrite ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 674 filtered out

$ RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib update_view -- --test-threads=1
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 675 filtered out
```

(`cargo test … --lib locale update_view` is rejected outright — `error: unexpected argument 'update_view' found`; cargo takes one TESTNAME. Run as two invocations above.)

---

## 2 `projection-repaints-camera` — the guest owes it, and delivers it

### 2.1 The measurement that decides it

From B38's own captured run (`🗑️generated/b38-probe-six-lanes.txt`, lines 7–12), the whole hop in four
timestamps:

```
[20.1s] projection measure candidates=[… {"id":"puzzle3d-measure-projection-orthographic-view","slot":"select-trigger","role":"combobox","isControl":true,"inPerspective":true,"text":""} …]
[21.5s] nudge puzzle3d-measure-projection-orthographic-view select options=["Plan","Top",…] current= picking=0
[22.4s] projection nudge … {"before":{…,"value":"","text":""},"after":{…,"value":"","text":"Plan"},"waitedMs":86}
[22.5s] verdict projection-repaints-camera FAIL before={"position":[9.17,-5.67,4.2575],…} after={"position":[9.17,-5.67,4.2575],…}
```

Two things settle it:

1. **`after.value` is still `""` while `after.text` is `"Plan"`.** That is the signature of
   `useWindowMeasureDraft` (`🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx:28`), the rail's own optimistic
   draft. Its docstring states the round trip it exists to cover: *"Measured on the live `:6013` shell
   (wave B12): 0.7 s on an idle app, several seconds on a busy one."* So `projection-control-flips`
   PASSed on the DRAFT — it is not evidence the guest answered.
2. The camera was read **~100 ms** after the click (`[22.4s]` → `[22.5s]`), with no settle. Compare the
   `camera-gestures` lane in the same probe, which polls `cameraSettled(...)` for every one of its three
   verdicts. `projection-repaints-camera` has no such wait.

So the red is a **probe timing defect**, not a product one: it samples the pre-dispatch pose.
(The locator is right — the candidate dump shows exactly ONE `…-orthographic-view` control and it is
`inPerspective: true`, so B36/B38's locator work stands.)

### 2.2 The guest half, proved

Traced end to end and found correct at every hop:

| hop | where | state |
|---|---|---|
| 0 | `🎚️measure-controls/🟦️.tsx:48` | `onValueChange` → `onAction({…measure.onChange, args:{ field:"orthographicView", value:"plan" }})` |
| 1 | `🛠️ShellHelpers/🟦️.tsx` `windowMeasuresChrome` | `taggedOnAction` stamps the owning `windowId` — so the flip targets the pane it is rendered in |
| 2 | `✏️editor/🦀️.rs:2353` | `setProjection` → `Puzzle3dScopeClass::WindowOption` |
| 3 | `✏️editor/🦀️.rs:2236` `puzzle3d_window_option_scope` | `window_bodies: [main::BODY_KEY]` — the world body IS repainted, so `cameraJson` re-encodes |
| 4 | `🎮️commands/📽️set-projection/🦀️.rs:10` | `world3d_projection_action_moves_pose` true for `orthographicView`; `apply_world3d_projection_action` sets `kind`/`orthographic_view`; `world3d_projection_pose` re-derives position and up around the unchanged target |
| 5 | `✏️editor/🦀️.rs:7199` | publication contract `[WindowConfig]`, and `🪟️window/🦀️.rs:30` shows `Puzzle3dWindowConfig` carries `camera` — so the new pose really is what the lane publishes |

New law, `📷️OpeningCamera` region of
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`:
`flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone`. It reads the
published pose off the world lane through the existing `published_camera` helper (the exact string
`World3dHost` parses into `data-camera-json`), asserts the pane starts on its own `threePoint` default,
flips to an orthographic plan, and requires: the published `projection/mode/kind` and
`projection/orientation/view` to follow, the POSITION to move, the UP vector to be re-derived, the plan
eye to stand **directly above** its target on the same x/y with a greater z, the SIBLING pane's camera
lane to be byte-identical, and the flip back to `threePoint` to move the pose off the plan axis again.

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib projection -- --test-threads=1
running 5 tests
test editor::puzzle3d::component::tests::flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone ... ok
test editor::puzzle3d::component::tests::puzzle3d_play_projection_pack_round_trips ... ok
test editor::puzzle3d::component::tests::puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example ... ok
test editor::puzzle3d::precompute::component::tests::set_scene_with_applied_fill_projection_preserves_slider_session ... ok
test standards::v1::subsets::any::schema::snapshot::text::tests::puzzle3d_projection_dsl_round_trips ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 728 filtered out; finished in 0.23s
```

Green on the FIRST run against the unchanged guest — no product change was needed or made for item 2.
**Verdict: `projection-repaints-camera` is a probe defect.** Handover, one line, to whoever owns
`🔍️browser-probe.ts` (read-only for this wave): replace the bare `cameraOf(...)` after `nudgeMeasure`
with the `cameraSettled("puzzle3d-main-perspective", cameraBefore)` the `camera-gestures` lane already
uses, and score `projection-control-flips` on the measure's `value`, not its `text`, so the rail's draft
cannot pass it on its own.

---

## 3 The second half of the locale switch: nothing asked the GUEST to re-render

Found only once §1 was fixed — until then the guest was dead by the time these labels were read, which
is exactly why B38 §1.7 could not tell the two defects apart.

### 3.1 Root cause, `file:line`

`🏛️ShellHost/🟦️.tsx:7210` — the UI-preference projection effect, keyed `[uiLocale, uiTheme, scope]` —
calls `scope.i18n.changeLanguage`, `syncShellLabelLocale`, sets `documentElement.lang` and repaints the
theme. `:4855` — the retitle effect, keyed `[uiTerminology, uiLocale]` — retitles the window layout
nodes and the extra window instances. **Neither requests a `refreshUi`.**

But every label inside a guest-authored BODY is resolved by the program against `ViewModel::locale`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4389`, non-optional and documented as such) at the moment it
rendered — `puzzle3d_labels(view_state)` in
`✏️editor/🗣️terminology/🦀️.rs:132`, consumed by the outliner at
`✏️editor/📌️panels/🗿️artifact/🦀️.rs:316` (`ui_label(labels.objects.as_str())`), the inspection panel at
`🔍️inspection/🦀️.rs:189`, the catalogue at `🛍️catalogue/🦀️.rs:153`. So a language switch left every one
of those bodies in the previous language until something unrelated happened to dirty them.

(A `grep 'pub locale'` over `🔨️modules/🔌️plugin/🦀️.rs` finds nothing and invites the wrong conclusion —
`ViewModel` lives in `🔨️modules/🛂️manifest/🦀️.rs`, and it has carried `locale` all along. The wire
architecture was already right; the host simply never asked the guest to use it again.)

### 3.2 The fix

`🏛️ShellHost/🟦️.tsx:4855`–`:4887` — the retitle effect now also asks the guest:

```ts
const live = sessionRef.current;
if (live) void refreshUi(live, { kind: "full" }, undefined, true).catch(…);
```

`{ kind: "full" }` because a language change dirties every section there is, and `replaceBodies: true`
because it re-authors EVERY string: the refresh path is hash-conditional (`buildUiRefreshRequest` /
`applyUiRefreshResponseToCache`) and must not be allowed to keep a body it cached under the old locale,
and `replaceBodies` is also what calls `forceReloadLiveUiStoresV1(cache)` — those stores hold rendered
text of their own. `refreshUi` is a `useCallback(…, [])`, so listing it costs no dep churn, and the
effect reads the session through `sessionRef` rather than depending on it.

### 3.3 Laws

The guest half was already lawed and is green — no new guest law was needed:

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib german -- --test-threads=1
running 2 tests
test editor::puzzle3d::component::tests::app_definition_labels_resolve_german_reuse_branded_for_aggregator ... ok
test editor::puzzle3d::component::tests::document_and_kinds_trees_use_german_reuse_section_labels ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 731 filtered out
```

The HOST half is what had no law and no caller; its proof is the live verdict in §4.2.

---

## 4 Live verdicts on `:6013`, wasm #55

### 4.1 `--only=selection-keybindings` alone — item 3 needs no product change

`🗑️generated/b40-keys-only.txt`:

```
[52.1s] keybindings precondition attempt=0 via=pane-fraction ids=["object-1","object-1"] state=["puzzle3d-main-top=object-1","puzzle3d-main-perspective=object-1"] waitedMs=2
[52.1s] delete census count=2 +[] -[]
[53.7s] delete census count=1 +[] -["object-1"]
[53.7s] verdict delete-selection PASS
[57.7s] step selection-keybindings: ok windows=2 canvases=2 treeItems=4 newFaults=none
[57.8s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=16
```

`delete-selection` **PASS** on #55 with no `locale-switch` ahead of it — `count=2 → 1`, the deleted id
named in `-["object-1"]`, zero faults. So B38's red was never the delete lane: it was the §1 revoke,
whose `firstHardFaultAt=32.3` in B38's own run is the locale switch. B31's `deleteSelection` lane and
B36's `addObjectKind` Interaction-lane change are both intact on #55.

### 4.2 `--only=locale-switch,projection-options,selection-keybindings`

Before (B38 run 1, `🗑️generated/b38-probe-six-lanes.txt`) → after (`🗑️generated/b40-three-lanes-2.txt`):

| verdict | B38 on #53 | B40 on #55 |
|---|---|---|
| `step locale-switch` | `windows=0 canvases=0` + `plugin-ui.intake-rejected:intake:actor-activation.revoked` | **`windows=2 canvases=2 newFaults=none`** |
| `guest-alive-read` | FAIL `canvases=0 surfaces=[] guestDeathFaults=12` | **PASS** |
| `guest-alive-mutate` | FAIL `guestDeathFaults=9 first=… actor-activation.revoked firstHardFaultAt=32.3` | **PASS** |
| `battery-hard-faults` | FAIL `hard=9 collateral=3 distinct=6` | **PASS** |
| `battery-faults` | FAIL `raw=12 hard=9` | **PASS** |
| `delete-selection` | FAIL `before=5 after=5 removed=[] arrived=[] waitedMs=30566` | **PASS** |
| `duplicate-selection` / `duplicate-reselects-clone` / `focus-selection` | PASS (on drifting counts) | PASS |
| `locale-control-present` / `locale-no-english-leak` / `locale-switch-back-en` | PASS | PASS |
| `locale-flips-document-labels` | FAIL `en=…=de=OBJECTS \| … \| Hide \| Lock \| REFERENCES \| TARGET VOLUMES \| ATTRACTIONS` | **PASS** |
| `locale-de-document-section-label` | FAIL `de=OBJECTS \| … \| Hide \| Lock \| …` | FAIL — but now `de=OBJEKTE \| … \| Ausblenden \| Sperren \| REFERENZEN \| ZIELVOLUMINA \| ANZIEHUNGEN` (see §5) |
| `projection-repaints-camera` | FAIL | FAIL — probe timing, §2 |
| run totals | `faults=12 hard=9 collateral=3 first-hard-fault-at=32.3 guest-death-faults=9` | **`faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0`** |

```
[27.6s] step locale-switch: ok windows=2 canvases=2 treeItems=24 newFaults=none
[79.9s] battery verdict locale-flips-document-labels PASS
[79.9s] battery verdict delete-selection PASS
[79.9s] battery verdict guest-alive-mutate PASS
[79.9s] battery verdict battery-hard-faults PASS
[79.9s] battery verdict battery-faults PASS
[79.9s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=34
```

**The locale switch keeps the instance, its channel and its document, raises not one fault, and every
panel body comes back in German.** Zero guest-death faults in a run that contains the exact step which
produced twelve of them, and every downstream mutate step passes behind it — which is the whole of
B38 §1.7's handover discharged.

---

## 5 The two remaining reds, both explained, neither a product defect

1. **`projection-repaints-camera`** — probe timing (§2). The guest law is green; handover in §2.2.
2. **`locale-de-document-section-label`** — an **axis** error in the expectation, not a gap. It demands
   "Baukomponenten", which is the `reuse` TERMINOLOGY's word for `objects`
   (`🗣️terminology/🦀️.rs:11`: `objects: native_en "Objects", native_de "Objekte", reuse_en "Building
   components", reuse_de "Baukomponenten"`). The shell is on `native` terminology throughout this lane —
   the probe changes the LANGUAGE only — and `native_de` is `"Objekte"`, which is exactly what the
   outliner now publishes. The `reuse` cell is separately lawed and green
   (`document_and_kinds_trees_use_german_reuse_section_labels`, §3.3, which calls
   `set_label_axes(Locale::De, Terminology::Reuse)` first). Handover: this verdict must either switch the
   terminology to `reuse` before reading, or expect `"Objekte"`. Nothing in the product owes
   "Baukomponenten" under `native`, and per CLAUDE.md an unauthored axis must never fall back.

---

## 6 Verification, all foreground

| command | result |
|---|---|
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 27 passed (30)` / `Tests 15 failed \| 985 passed (1000)`. **No NEW failures** — the same 15 before and after this wave, all peer-owned live churn: 7 in `extension invocation completion ownership` + `noteShellCommand` (a peer added an `AbortSignal` to the invocation call — `expected "vi.fn()" to be called once with arguments … + "signal": AbortSignal`), 6 in `🧩️package-integration` (wgpu generated-worker bytes), 2 in `🔌️PluginRuntime` (`+ "ops": ""` on the document pack, and the packed window projection). `git status` confirms peers are live in `✏️s/🔌️plugins/🌊️flow/🧩️extensions/**` right now. The `+1` in passed is this wave's own law. |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json` | 861 errors, **unchanged before and after**, and **0** mention any symbol this wave introduced (`pluginAvailabilityRouteV1`, `pluginArtifactRebuiltAtRef`, `recordPluginArtifactRebuiltAt`, `uiLocaleRef`, `uiTerminologyRef`, the locale refresh). This package's typecheck is deeply red independently of this wave — the bulk is `🧪️tests/🧪️docklayoutstore/🟦️.ts` fast-check generics (`Type 'Command' is not generic`, ~hundreds) plus `ImportMeta.dir`. |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib locale -- --test-threads=1` | `1 passed; 0 failed` (674 filtered) — same as baseline |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib update_view -- --test-threads=1` | `running 0 tests` — no such law exists; see §1.5/§3.1 |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib projection -- --test-threads=1` | `5 passed; 0 failed` (728 filtered), including this wave's new `flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib delete -- --test-threads=1` | `24 passed; 0 failed` (709 filtered) |
| `RUST_MIN_STACK=134217728 cargo test … --lib delete_selection -- --test-threads=1` | `1 passed` — `delete_selection_shrinks_the_world_census_and_drops_the_deleted_id` (B31's) |
| `RUST_MIN_STACK=134217728 cargo test … --lib german -- --test-threads=1` | `2 passed; 0 failed` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile` — **0 errors** (88 pre-existing warnings) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile` — **0 errors** |
| `bun 🔍️browser-probe.ts --only=selection-keybindings --port=6013` | 16 verdicts, all PASS, 0 faults (§4.1) |
| `bun 🔍️browser-probe.ts --only=locale-switch,projection-options,selection-keybindings --port=6013` | 34 verdicts, 32 PASS / 2 FAIL (both §5), `faults=0 hard=0 collateral=0 guest-death-faults=0` (§4.2) |

### 6.1 Files changed by this wave

```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🔁️plugin-availability-route/🔣️.json  (new)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
```

Untouched on purpose: `🔍️browser-probe.ts` (read-only for this wave), `📃️UiDocumentStore` /
`🗣️Interpreter` / `🌐️World3dHost` record keying (wave B39), the plugin `🦀️.rs` files and the puzzle
editor's own command arms (peers). No temporary `[DEBUG]` logs were left behind: the two `[DEBUG]` lines
this wave adds are permanent evidence lines inside laws, matching the convention of both corpora.

### 6.2 One handover found while tracing, NOT fixed here

`framework.worldOrbit.projection` (`🌐️World3dHost/🟦️.tsx:4551`) is a hardcoded DOM id rendered once per
world surface, so a two-pane layout puts **two elements with the same `id`** in the document — B38's own
candidate dump shows both (`inPerspective: false` and `inPerspective: true`). Invalid HTML, and it makes
the pane unaddressable by id for `<label for>`, `aria-labelledby` and any automation. Measure ids escape
this only because the top pane's rail is folded, and only by luck: `world3d_projection_measures` takes a
kind-level `id_prefix` (`"puzzle3d"`), not the window instance, while `windowMeasuresChrome` already owns
the `windowId` it stamps onto every action. Not fixed here: World3dHost is wave B39's file, and
instance-qualifying measure ids would break the literal ids `🔍️browser-probe.ts` addresses
(`WINDOW_OPTION_IDS`, `puzzle3d-voxel-w`), which is read-only for this wave. It wants one packet that
changes the id scheme and the probe's locators together.
