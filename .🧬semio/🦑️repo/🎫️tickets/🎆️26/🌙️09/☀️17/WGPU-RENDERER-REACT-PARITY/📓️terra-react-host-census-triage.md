# React and Native Host Census Triage

Read-only triage on 2026-09-20. No production edits, builds, or tests were run. This classifies the exact failures recorded in `🗑️generated/astra-palette-geometry/ui-react-census.log` and `🗑️generated/astra-runtime/native-host-cancel-2.log` against the current source.

## Scope finding

The React receipt has **17 failures / 804 tests**. It contains no `ShellSearch`, command-palette, WGPU renderer, or pointer-cancel assertion. The focused React ShellSearch receipt cited by `📓️astra-sol-palette-geometry.md` is 6/6 passing. These failures therefore are not evidence against the current command-palette geometry or WGPU parity packet.

The native receipt has **9 failures / 79 host tests**. All nine are in `enqueue::input_admission_tests` or `enqueue::input_root_tests`; none enter the browser/Winit cancellation transport. They are current UI-host input-admission defects, outside this ticket's renderer/palette/cancellation scope.

## React: three independent repair packets

| Receipt failures | Verified cause | Smallest repair | Scope / confidence |
| --- | --- | --- | --- |
| Ten CSS-reading cases: celebrate (2), UIIntroduction (2), icon animation (5), ContextMenu U-gap, and inactive chrome hover | `🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:391` resolves `../../../../🎨️styling/…`, which is the absent `🧰️framework/🎨️styling/…` path named in every receipt. The actual CSS exists at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`; from the test directory it is `../../🎨️styling/…`. | Correct that shared CSS resolver (and its equivalent direct reads) to the existing module-local stylesheet. No CSS behavior change. | Test-harness path only; external to WGPU parity. **High** |
| TutorialLocalInteraction | The test's source/schema imports at `…owned-locale-detector-retirement/🟦️.tsx:11457-1161` ascend three levels to `🔨️modules/🛂️manifest`, but its fixture read at :11458 ascends five and resolves the absent `🧰️framework/🛂️manifest/…`. The fixture is present at `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`. | Change only the fixture-relative path to the same module depth as its import. | Test-harness path only; external. **High** |
| UIDialog nested picker | The test definition permits only `map` (`…UIDialog/🧪️tests/🧩️component/🟦️.tsx:23-33`), then its nested Select offers and selects `terrain` (:138-156). Current UIDialog deliberately clears choices outside `dialog.args` (:60-75) and disables submit while unresolved (:71, :117). Receipt's absent submit is the intended outcome for the declared schema. | Make the test schema and ordinary test renderer include `terrain`, or retain the one-value schema and expect `map`. Do not remove UIDialog validation. | React component test law, external. **High** |

## React: WindowChrome-focused follow-up

These four receipt cases share React chrome code but do not establish a WGPU divergence. They should be assigned as one focused React-only packet, with a current receipt before changing production because the census log may precede concurrent source changes.

| Receipt failure | Current source fact | Bounded decision / repair |
| --- | --- | --- |
| `Window merges the ad-hoc actionPane…` | `Window` shares `actionsFolded` between the Actions pane and Search (:180-187, :205-215), passes the inverse to `Pane` (:370-387), and `Pane` passes no body when folded (`react/🟦️.tsx:10170-10194`). The receipt says `actionPane` remained after the second toggle at test :6823. | Reproduce only this current React test. If current behavior still retains the content, repair that fold transition; if it unmounts, retain the source and update the stale receipt/test. This is a real current React code-path mismatch, but no source-only proof selects which side. **Medium** |
| `WindowChrome stamps data-dim on the body glass surface…` | `window-chrome-body-surface` is intentionally conditional on measured `geometry.bodyRegion` (`react/🟦️.tsx:8153-8162`); the content plane is always present and already stamps `data-dim` (:8163-8175). The test expects the conditional surface after a synthetic measurement (:8164-8191); receipt retained `data-silhouette-state=pending`. | Keep the test's rectangle contract and repair the geometry settling path if it remains pending, or assert the permanent content plane when no body region is measurable. This is geometry-test/React target work, not WGPU. **High on the condition; medium on the chosen repair.** |
| fullscreen trailing selector | Navbar renders `NavbarTrailingChromeSlot` (`🧱️elements/🔝️Navbar/🟦️.tsx:187-202`). That slot emits `data-slot=navbar-trailing-chrome` and a Toggle with stable `id=ui.fullscreen.toggle` (`react/🟦️.tsx:9494-9546`), never `navbar-fullscreen-toggle`. The failed assertion demands the nonexistent latter selector at test :10291-10295. | Select the stable control id (and trailing-chrome parent if location matters), or deliberately introduce a new slot contract. Test selector update is the smallest evidence-backed repair. **High** |
| panel chip-cap and controls | `WindowChrome` only renders `window-chrome-controls` when it has `capRightChips`, `enlarge`, or `close` (`react/🟦️.tsx:8123-8147`). The tested Panel supplies none, but test :10631-10641 dereferences the absent node. | Make this test provide a control when it is testing the controls cell, or assert the documented absence for a no-control Panel. No production control should be synthesized merely for this selector. **High** |

## Native host: two production repair packets

| Receipt failures | Verified current source cause | Smallest independent repair | Scope / confidence |
| --- | --- | --- | --- |
| `input_admission_full_refusal_preserves_generation`; `input_admission_generation_exhaustion_does_not_wrap`; `input_admission_metrics_generation_exhaustion_preserves_queue` | `InputGeneration::next` is `wrapping_add` at `🖥️host/📥️enqueue/🦀️.rs:110-115`. Both `enqueue` and `enqueue_metrics` assign it before admission/overflow checks (:283-305). This produces receipt values 257 on full rejection and 0 at `u64::MAX`, directly contradicting the neutral laws at `📥️input/🎟️admission/🧪️tests/🎟️admission/🦀️.rs:27-63`. | Make generation reservation fallible and perform it only as part of successful admission; at exhaustion preserve the queue and return the typed refusal already required by the event path. Apply the same rule to coalesced metrics. | Current host defect; independent of cancellation. **High** |
| `input_admission_constructor_has_no_unadmitted_backing`; `…terminal_requires_empty_backing`; the four `input_root…` capacity/allocation failures | `EventQueue::new` allocates `VecDeque::with_capacity(DISCRETE_QUEUE_CAPACITY)` at `…enqueue/🦀️.rs:255-259`. Receipt observes exactly 256 slots / 24,576 bytes. The neutral fixture requires zero initial backing and terminal backing (`📥️input/🎟️admission/🧫️fixtures/🔣️.json:13-22`); root tests require zero allocation/capacity before and after root admission (`📥️input/🎟️admission/🪪️root/🧪️tests/🪪️root/🦀️.rs:44-62, 95-183`). | Reconcile the implementation with the existing fixture: defer discrete backing allocation until successful event admission, and release it at terminal retirement. Preserve the explicit bounded logical capacity checks. Do not alter the fixture to bless preallocation without an ownership-contract decision, since it would weaken seven independent assertions. | Current host defect; independent of renderer cancellation. **High** |

## Existing kernel note

The separate AJV receipt `🗑️generated/astra-runtime/kernel-pool-fixture-schema-2.log` reports `kernel-pool-future strict schema accepted 2 neutral traces`. It clears the fixture-validation concern raised in the earlier kernel audit; it does not change the ResponseSlot lost-wake analysis or the renderer packet's scope.

## Priority

1. Host generation admission/exhaustion: user-visible stale-input correctness and a compact isolated fix.
2. Host deferred backing/terminal retirement: one contract-driven ownership packet, affects seven receipt failures.
3. React shared CSS/fixture relative paths: restores eleven harness failures without behavior work.
4. React UIDialog fixture alignment and WindowChrome focused receipt: independent React cleanup, not a blocker for palette/WGPU parity.
