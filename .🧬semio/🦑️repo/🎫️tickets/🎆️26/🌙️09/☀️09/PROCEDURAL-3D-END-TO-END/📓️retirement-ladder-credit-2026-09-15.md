# 🪜️ Retirement ladder credit — lane `retirement-ladder-credit`, 2026-09-15

React procedural 3d playground, port 6022. Closes the item
`📓️flow-surface-followup-2026-09-15.md` §2.4 / §7 handed on: **the session retirement ladder moved per
byte.** A 32 KB retained document needed 5 718 close turns against a declared bound of 4 096
(`🧫️fixtures/🧹️session-close/🔣️.json` `close.maximumTurns`) — a live contract violation for any
document over ~20 KB.

## 1. Result

| retained document | close turns before | close turns after |
|---|---:|---:|
| 1 KB (8 rows, scaled payload) | 1 393 | **11** |
| 32 KB (8 rows, scaled payload) | 1 417 | **11** |
| 256 KB (8 rows, scaled payload) | 1 585 | **11** |
| 24 rows (`MEASURED_ROWS`, the followup's own document) | 2 627 | **11** |
| 64 rows (~32 KB, the followup's own document) | **5 719** | **11** |

The 2 627 / 5 719 readings reproduce `📓️flow-surface-followup-2026-09-15.md` §2.4's 2 626 / 5 718
exactly (±1 poll: this harness counts the terminal poll too), so the two measurements are of the same
thing. After the fix the count is **11 turns at every size and every row count** — 520× fewer for the
64-row document, and 372× under the declared bound of 4 096 instead of 1.4× over it.

The React host's own owner ladder (`closeUiOwner`) had the same mispricing and is fixed the same way.
On the converged hexagonal column, retiring the editor instance:

| | before | after |
|---|---:|---:|
| close turns, 14 retained surfaces | 11 207 | **29** = 2 × 14 + 1 |
| close turns, 7 retained surfaces | 2 828 | **15** = 2 × 7 + 1 |
| `plugin-ui.owner-close-budget-exhausted` over 8 switches | — | **0** |

Exactly one turn per retained item: one wire release and one surface release per surface, plus the
instance close. The 11 207 ladder steps are still taken — they are the `free()` — they simply no
longer each cost a budget unit.

## 2. The ladder, printed

`🐍️`-free: the census is a law, `the_retirement_ladder_names_every_turn_it_spends`, which drives one
real `FlowDomainAdapter` to terminal and names the phase every close turn anchors on.

### Before — 1 416 turns for a 32 KB document

```
Vcs(DocumentVersion) x1      ← pop one document version onto the retirement frontier
Vcs(Backing)       x120      ← 120 turns draining that version's payload, one owner per turn
Vcs(Document)        x1
Host(Dag)          x615      ← 615 turns walking the DAG retirement, one owner per turn
Host(Widgets)        x1  Host(Domain) x41   ← pop one widget, then 41 turns freeing its bytes
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x41
Host(Widgets)        x1  Host(Domain) x48
Host(Widgets)        x1  Host(Domain) x59
Host(Widgets)        x1  Host(Domain) x40
Host(Synapses)       x1  Host(Domain) x64
Host(Synapses)       x1  Host(Domain) x64
Host(Layout)         x1  Host(Domain) x56
Host(Schema)         x1  Host(Domain)  x1
Host(Bridges)        x1
Host(Cache)          x2
Host(Complete)       x1
```

**What each close turn did:** exactly one rung. `advance_session_close` calls `domain.close_step`
once per `poll`, and `close_step` took one rung and returned — `FlowRetainedVcs::close_retired_step`
decomposing one owner, or `FlowHostRetirement::close_page(1, …)` popping one. The `Backing` and
`Domain` runs are the smell the scroll lane found in the ABI credit, one layer over: **780 of the
1 416 turns (55%) are pure payload-frontier drains** (`Vcs(Backing)` 120 + `Host(Domain)` 660), and a
further **615** are the DAG retirement walking one owner per turn — **1 395 of 1 416 (98.5%) are turns
that hand NOTHING back across the ABI**, each granted 4 096 bytes of credit and spent freeing one
`Vec`. Only 21 turns cross a retained item.

(This census drives a bare `FlowDomainAdapter`, so it has no bridge poll of its own; the 1 417 / 11 in
§1 are the same ladder counted through the real `FlowBridge`, which adds the terminal poll.)

### After — 10 turns for the same document

```
Vcs(DocumentVersion) x1
Vcs(Document)        x1
Host(Dag)            x1
Host(Widgets)        x1
Host(Synapses)       x1
Host(Layout)         x1
Host(Schema)         x1
Host(Bridges)        x1
Host(Cache)          x1
Host(Complete)       x1
```

One turn per retained ITEM. No payload phase anchors a turn any more — `Vcs(Backing)`,
`Host(Domain)` and `Host(Neural)` are spent inside the turn that fed them.

## 3. Design

**A byte credit is the wrong currency for a close.** Nothing crosses the ABI while a session retires:
the only message the whole ladder produces is the terminal event at the end. What a close turn costs
is the ROUND TRIP the host pays to ask for it — a `postMessage` and a poll — and the quantity worth
one round trip is a retained item, not a `free()`.

So the ladder is anchored on its own structure. Both stages of a Flow domain now name the rung they
would take next:

- `FlowRetainedVcs::close_phase() -> FlowVcsClosePhase` — `Operations`, `Backing`, `RetiredSurface`,
  `RetiredAction`, `History`, `DocumentSurface`, `DocumentVersion`, `Document`, `Complete`, in
  `close_retired_step`'s own branch order. `Backing` is the only rung that moves payload.
- `FlowHostRetirement::close_phase() -> FlowHostClosePhase` — `Dag`, `Domain`, `Neural`, `Widgets`,
  `Synapses`, `Layout`, `Schema`, `Outputs`, `Exports`, `EvalJson`, `Catalogue`, `KindInfos`,
  `PreviousSnapshot`, `PreviousChannels`, `HistoryBaseline`, `PendingEvals`, `Bridges`, `Cache`,
  `HistoryStore`, `Faulted`, `Complete`, in `close_page`'s own branch order. `Domain` and `Neural`
  are the payload frontiers every other rung pushes onto.

`FlowDomainAdapter::close_step` then loops: it anchors the turn on the first NON-payload phase it
sees and keeps stepping while the phase is a payload drain or equals that anchor; it ends the turn
the moment a DIFFERENT retained item comes up. The turn count is therefore the number of retained
items — surfaces, history entries, document versions, host owners — and reads the payload nowhere.

Three properties kept deliberately:

- **A rung that makes no progress ends the turn, as it always did.** `close_retired_step` answering
  `ClosePending` (a live operation credit), or `Ok(true)` while the VCS is not terminal (a wedge),
  yields the turn instead of spinning inside it. `FlowRetirement::close_page` answering `Blocked` —
  which `close_retired_step` used to DISCARD, returning `Ok(false)` as though it had progressed — is
  now propagated as `ClosePending`, so a blocked drain yields rather than looping on a grant it
  cannot use.
- **`FLOW_CLOSE_RUNGS_PER_TURN` (1 << 20) is a hang guard, not the bound.** A turn that reaches it
  yields and the driver re-enters with a fresh anchor; it never fails the close. The bound is the
  phase count.
- **The byte credit keeps its old job**: it still sizes each individual `release_root_backing`
  draw-down and each `close_page` grant. It simply no longer decides how many ROUND TRIPS a close
  costs.

### The React host twin — `🔌️PluginRuntime/🟦️.tsx`

`closeUiOwner` bounded itself with `retainedUiCloseStepCeilingV1(surfaces)` — derived, correctly,
from the retained SURFACES (`📓️react-final-sweep-2026-09-15.md` §4b) — but compared it against a
count of ladder STEPS. `closeChild` caps every step at ONE item however large the grant (`uiGrant`
offers 256 items and 65 536 bytes), so a step count is a count of an instance's retained BYTES: the
number and the ceiling measured different quantities, which is exactly why a converged generation3d
instance threw `plugin-ui.owner-close-budget-exhausted` on 1 of 4 role switches
(`📓️role-switch-regression-2026-09-14.md` §6) while naming neither a phase nor a count.

The same rule now applies, extracted as an exported accountant so the rule under law is the rule
production runs:

```ts
createRetainedUiCloseLadderV1(retainedSurfaces) → { ceiling, turns, admit(step) }
```

**A turn is one step in `OwnedUiInstance.closeStep`'s OWN branch namespace** — the `instance-` phases:
a lookup, a work-queue release, an input retirement, a receipt outbox, a wire, a surface, the instance
itself. Every other phase arrives through `closeChild` from a descendant and is payload UNDER the item
the ladder is on. That discriminator is not guessed; it is the measured vocabulary of a real close on
this app (`🗑️generated/react-retire/phases/`, instance 1, 14 surfaces, 11 207 steps):

```
instance-wire-release:14   instance-surface-release:14   instance-close:1      ← the retained items
surface-root-close:14  surface-nodes-release:14  surface-bindings-release:14  surface-close:14
node-index-close:479   node-release:310   node-retire:310   node-field-retire:1085
typed-release:1085     typed-retire:1085  typed-object-retire:1282  typed-bytes-retire:53
binding-index-close:479  scene-binding-release:155  scene-binding-close:155
prepared-scene-source-close:3920  prepared-scene-record-close:250   …            ← the payload
```

29 of 11 207 steps are instance-level. An intermediate version of this fix anchored on *runs of the
same phase string* and read 5 861 turns — the node ladder alternates phases every step or two, so a
run-length rule is still a byte count. Naming the owner's own namespace is what makes the count
structural.

The byte-aware `PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS` stall rule stays on the STEPS, because a stall is
a property of a step and not of a turn, and the fault still names its phase. The macrotask yield stays
on the steps too, so responsiveness is unchanged. `closeUiOwner` now emits
`[DEBUG] plugin-ui close ladder instance=… surfaces=… turns=… steps=… ceiling=… phases=…` — the seam
the gate probe reads.

## 4. Files

Product:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs` — `FlowDomainAdapter::close_step`
  (the anchored turn), `close_phase`, `close_rung`, `FlowCloseLadderPhase`, `FlowCloseRung`,
  `FLOW_CLOSE_RUNGS_PER_TURN`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs` — `FlowVcsClosePhase`,
  `FlowRetainedVcs::close_phase`, `close_retired_step` propagates a `Blocked` backing drain
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowHostClosePhase`,
  `FlowHostRetirement::close_phase`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` —
  `createRetainedUiCloseLadderV1`, `closeUiOwner` anchored on it, the `[DEBUG] plugin-ui close ladder`
  seam, `retainedUiCloseStepCeilingV1`'s docstring
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/` — rebuilt,
  `nx run semio-framework-os-flow-core:wasm`, `NX_SKIP_NX_CACHE=true`, 4 m 8 s

Laws:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs` —
  region `🪜️RetirementLadder`: `scaled_document_json`, `row_document_json`, `close_ladder_turns`,
  `close_ladder_census`, and the two laws; `MEASURED_ROWS`' docstring re-measured
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` —
  four new rows in `plugin ui close ladder budget`

Ticket:

- `🐍️retirement-ladder-probe.mjs` (new), `📓️retirement-ladder-credit-2026-09-15.md` (this report)
- evidence: `🗑️generated/retire/`, `🗑️generated/react-retire/`

## 5. Laws, with output

### 5.1 Rust — the ladder is independent of the payload

```
cargo test -p semio-framework-os-flow --lib a_retained_document_retires_in_turns -- --nocapture
[DEBUG] flow close ladder: document 1024 B (1941 B of json) retired in 11 turns
[DEBUG] flow close ladder: document 32768 B (33765 B of json) retired in 11 turns
[DEBUG] flow close ladder: document 262144 B (263189 B of json) retired in 11 turns
test result: ok. 1 passed; 0 failed
```

`a_retained_document_retires_in_turns_that_do_not_count_its_bytes` drives three documents of the SAME
eight-row structure and 256× different payload through the real `FlowBridge` — open, synchronize,
read back `documentJson`, close — and asserts the counts are EQUAL, that the payloads really differ by
two orders of magnitude, and that the largest still fits its declared bound with 32× of margin. Before
the fix: `left: 1393, right: 1585`.

### 5.2 Rust — the ladder names every turn it spends

```
cargo test -p semio-framework-os-flow --lib the_retirement_ladder_names_every_turn -- --nocapture
[DEBUG] flow close ladder census (32 KB document): 10 turns
[DEBUG]   Vcs(DocumentVersion) x1 … Host(Complete) x1     (§2, in full)
[DEBUG] flow close ladder: the 24-row measured document retired in 11 turns
[DEBUG] flow close ladder: the 64-row measured document retired in 11 turns
test result: ok. 1 passed; 0 failed
```

`the_retirement_ladder_names_every_turn_it_spends` asserts the census is under 64 turns and that NO
payload phase (`Backing`, `Domain`, `Neural`) ever anchors a turn. Before the fix: 1 416 turns.

### 5.3 Rust — the session close itself, and the retirement family

```
cargo test -p semio-framework-os-flow --lib retirement -- --test-threads=1
test result: ok. 11 passed; 0 failed; 0 ignored; 235 filtered out
```

Eleven, including the whole `host::session_retirement_tests` family (`host_retirement_reports_no_credit
_and_retained_fault_without_false_pending`, `session_close_dag_host_retirement_preserves_exact_owner
_and_byte_grants`, `live_session_drop_is_rejected_without_recursive_payload_destruction`, …) and
`the_retirement_ladder_names_every_turn_it_spends`.

### 5.4 TypeScript — the host ladder's twin

```
SEMIO_TEST_LEVEL=long bunx vitest run --config ./🟦️.ts \
  --testNamePattern="plugin ui close ladder budget"
Test Files 1 passed | 50 skipped (51)     Tests 6 passed | 1306 skipped (1312)
```

- `counts one turn per retained item and never reads the bytes under it` — three retained items
  carrying 1, 64 and 4 096 descendant steps each all answer 3 turns.
- `spends no turn on the descendant phases closeChild forwards, however many arrive` — 100 000
  alternating `prepared-scene-source-close`/`typed-retire` steps answer 0 turns; the next
  `instance-surface-release` answers 1. This is the row that fails on the run-length rule.
- `cannot exhaust its budget on a converged instance, at any payload` — a whole ceiling's worth of
  retained items raises no fault; item `ceiling + 1` raises exactly
  `plugin-ui.owner-close-budget-exhausted:instance-close after N turns over 1 retained surfaces`.
- `names the phase of a ladder that releases nothing, long before the backstop` — 32 zero-byte
  descendant steps pass, the 33rd raises `plugin-ui.owner-close-stalled:node-field-retire released
  nothing for 33 steps`, with the turn count still at 0.
- Plus the two rows `📓️react-final-sweep-2026-09-15.md` §8.4b landed, still green.

### 5.5 Rust — the compiled session close itself

```
cargo test -p semio-framework-os-flow --lib compiled_session_close -- --nocapture
[DEBUG] Flow native compiled session close: real VCS and host retired, terminal-empty=true
[DEBUG] Flow real adapter session A retired with its exact receipt; sibling B completed selection
        before global terminal close
test result: ok. 2 passed; 0 failed
```

Both drive `close_bridge`, which is bounded by the fixture's own `close.maximumTurns`.

```
cargo check -p semio-framework-os-flow --all-targets --keep-going  → exit 0, no errors
```

## 6. Gate

### 6.1 `🐍️retirement-ladder-probe.mjs` — 8 role switches on the converged hex column

```
cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d&example=hexagonal-mushroom-column \
  SEMIO_PROBE_OUT=react-retire/ladder SEMIO_PROBE_GATE=1 bun 🐍️retirement-ladder-probe.mjs   → exit 0

1-boot            ok=true  windows=["procedural-preview"]       meshes=3
2..9 switch × 8   ok=true  viewer/editor alternating            meshes=3 every step
verdict red=0 unconverged=[] closes=8 maxTurns=29

{instance:1, surfaces:14, turns:29, steps:11207, ceiling:4480000}
{instance:2, surfaces: 7, turns:15, steps: 2828, ceiling:2240000}
… the remaining six closes repeat those two shapes exactly

budgetExhausted 0  stalled 0  blocked 0  pageErrors 0  roleSwitchFailed 0
```

Eight real retirements of a converged instance. Turns are exactly `2 × surfaces + 1` in every one of
them, `owner-close-budget-exhausted` is **0**, and so are the stall, blocked, page-error and
role-switch-failure counts. The same probe on the run-length rule read `maxTurns 5861`.

### 6.2 Battery

```
cd <ticket> && SEMIO_BATTERY_URL=http://127.0.0.1:6022/?plugin=generation3d \
  SEMIO_BATTERY_ROOT=react-retire bun 🐍️react-battery.mjs --only=role-switch,journey
[DEBUG] BATTERY DONE green=2/2 red=[] 154s pageerrors=0
```

- `journey` — all eight examples load with live preview meshes in the editor and the viewer, generate
  mode adds a generation, the viewer role and `No example` are clean; `no page errors`, `no shell
  faults`.
- `role-switch` — 9/9 steps, `no \`no actor for instance\``, `no page errors`, `no shell faults`.

## 7. Not claimed

- **The Rust close is proven natively, not in the browser.** Both laws and the census drive the real
  `FlowBridge`/`FlowDomainAdapter` in-process. `flow_core_bg.wasm` was rebuilt so the guest the page
  loads carries the fix, but no browser reading of a FLOW SESSION close turn count was taken — the
  flow session close has no console seam of its own, and adding one was out of this lane's scope. The
  browser evidence in §6 is of the React host's owner ladder, which is a different ladder with the
  same defect.
- **`FLOW_CLOSE_RUNGS_PER_TURN` has never been reached.** Every measured close ends on a phase change
  or on `Complete`; the ceiling is a hang guard that no run in this lane exercised. It is also the one
  place where a pathological single phase could still cost a long turn — a 10 MB DAG would free inside
  one turn, and this lane did not measure such a document.
- **The TS turn discriminator is a phase-name prefix.** `RETAINED_UI_CLOSE_OWNER_PHASE_PREFIX` is
  `"instance-"`, which is `OwnedUiInstance.closeStep`'s own branch namespace by construction, and the
  measured vocabulary of a real close agrees (§3). It is still a string convention: a new instance-level
  branch named outside that namespace would silently stop costing a turn. The structurally exact fix
  would be an `owner` flag on `RetainedUiWireStep`, which `closeChild` forwards verbatim across every
  retained-UI module — a much wider change this lane did not make.
- **`plugin-ui.owner-close-budget-exhausted` was not reproduced before the fix either.** Like
  `📓️react-final-sweep-2026-09-15.md` §4b, what is proven is that the count and its ceiling now
  measure the same quantity and that 8 real retirements of a converged instance raise nothing; the
  09-14 sighting itself has not recurred on this tree since, so this lane cannot claim it removed that
  exact byte.
- **The intermediate run-length rule is recorded because it is a trap, not because it shipped.**
  Anchoring on runs of the same phase string reads 5 861 turns for 11 207 steps on this app: the node
  ladder alternates phases every step or two, so run-length is still a byte count wearing a structural
  name. The shipped rule names the owner's namespace.
- **Pre-existing reds this lane did not touch, and proved it did not cause.**
  `semio-framework-os-flow --lib domain_laws::` is 14 passed / 7 failed: five are the
  `ordered-map root must be explicitly retired before drop` family already named by
  `📓️flow-surface-followup-2026-09-15.md` §7, one is
  `production_reachability_fixture_and_hostile_source_census_reject_the_old_route` comparing a bundled
  `🌐️flow-browser.js` against its source (named as pre-existing by
  `📓️react-final-sweep-2026-09-15.md` §11), and one is
  `synchronized_document_json_is_the_exact_retained_document` failing on a `layout` map that round-trips
  empty. `--lib flow_vcs_tests` is 12 passed / 21 failed, 16 of them the same OrderedMap-drop family.
  None of them calls `close_step`, `close_phase` or `close_retired_step`, and the whole
  `flow_vcs_tests` filter reads **identically (12/21) with and without** this lane's `Blocked`
  propagation — measured by disabling that branch and re-running. Both of this lane's new Rust laws,
  `--lib retirement` (11/11) and `--lib compiled_session_close` (2/2) are green.
- **Two rows of the whole `🔌️plugin-runtime` suite are red and neither is this lane's.** The file is
  `2 failed | 125 passed (127)`: `host effect address decoding · reads every request-carrying effect
  out of its nested WIT params record` (a `wireEffectToFriendly` / `🎯️host-effect-address.json`
  mismatch) and `leftover brush guest hover retain · keeps the leftover vortex id on an armed brush
  window` (a 5 s timeout). This lane's diff to `🔌️PluginRuntime/🟦️.tsx` touches only `closeUiOwner`,
  the new ladder accountant and two docstrings — neither the effect decoder nor the brush hover path —
  and all six `plugin ui close ladder budget` rows are green.
- **`FlowVcsFeature::close_cursor_step` still spends one rung per turn.** That is the OPERATION-cancel
  ladder, not the session close; it retires one operation's cursor and page, a quantity that does not
  scale with the document. It was read and deliberately left alone.
- **`OwnedUiInstance.closeStep`'s one-item cap is unchanged.** `closeChild` still rejects any step
  reporting more than one item, so the 256-item `uiGrant` is still spent one item at a time. This lane
  stopped that cap from setting the BUDGET; making a UI close step actually consume its item grant is a
  retained-UI contract change and a lane of its own.
- **`close.maximumTurns` is left at 4 096.** It is the contract's CEILING, not a target, and the
  measured cost is now 11; tightening it to the measured number would turn a backstop into a
  regression detector for a quantity the laws already pin directly. The fixture is untouched.
- **Only macOS was exercised**, and only port 6022. Windows and Linux are reasoned: nothing in either
  ladder is platform-dependent.
- **`interact` and the other battery rows were not run.** The gate is `role-switch,journey` plus the
  new probe, as scoped; the reds `📓️react-final-sweep-2026-09-15.md` §11 lists are neither confirmed
  nor cleared here.
