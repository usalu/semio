# Tutorial Local Interaction Consumer Handoff

## Exact Current Contracts

`replication/📡️wire/🏠️local-interaction/🟦️component.ts` defines `LocalInteractionState` as selection, activeMode and activeGranularity maps. Captures pair that state with current identity; full restore replaces all maps, while sparse domain patches explicitly use null to remove an entry. Historical tutorial content must not preserve a capture identity for a future restore.

The authored manifest still exposes `TutorialUiSnapshot.interactionSelection` and a selection-only change with granularity/ids. The UI composition helper copies only that map. ShellHost's diff also loses deleted domains and anchors and compares ids by comma joining. ShellHelpers still reads/writes the deleted `selectionJson`, accounting for five of the seven current strict errors. The other two are the demonstrator authored base snapshot and the renderer fixture missing the old required map.

## Real Producer Boundary

`AppChannelClient.readLocalInteractionPages(consume, signal)` exists in OS source and resolves its captured identity only after closed/retired query completion. The renderer PluginRuntime facade currently exposes neither that method nor a complete structured capture. The native restore command is not mounted. `applyLocalInteractionRestoreCold` is only a cold composition helper and cannot replace that live producer.

`captureTutorialUiSnapshot` is currently synchronous and is called by record start, every shell-state diff and periodic full snapshots. Correct capture requires an asynchronous, cancellation-aware query that consumes actual pages and preserves the exact current session; it cannot pull local state from filtered Presence or invent an empty map. Restore must obtain a fresh current identity, then issue one typed atomic full/domain request through the interaction authority before reporting success. The old snapshot identity cannot serve as that request's base.

## Owned Next Packet

UI now owns the authored TypeScript tutorial snapshot/change shape, composition/diff helpers, ShellHelpers/ShellHost consumers, demonstrator base and matching fixtures. Dag retains native Interaction/Store/restore and has confirmed no overlapping TypeScript edits. The approved authored shape is required `localInteraction: LocalInteractionState` and `kind: "localInteractionDomain"` with exact `LocalInteractionDomainPatch`; no old alias remains.

Schema-first cases must cover comma-bearing ids, anchors, selection modes, nonbroadcast domains, explicit removal, full replacement, sparse preservation, historical state versus fresh authority, stale query/session cancellation and restore refusal. Exact authored fixture values may contain an explicitly empty initial state, but live capture must never default absent producer data to that state.

The coordinated producer seam still needed is a real renderer-accessible capture plus typed live restore. Adding an unavailable method declaration, optional success callback, no-op restore or cold fallback would hide this boundary. The next source patch will preserve that distinction and request the exact producer join from the owner rather than claim live tutorial restoration from typechecking alone.

## Pure Composition TDD Staging

The five neutral cases now have a collected `TutorialLocalInteraction` test in the existing UI React tutorial test region. The new authored tutorial module initially contains only the exact change type. Its planned cold diff/composition helpers are not yet implemented or mounted into live recording; the test compares all three maps with an Immer Map oracle and validates the canonical local-interaction schema through Ajv.

Renderer R1 failed before executing this test: Vite could not resolve the peer's in-progress `kernel/return/content/input/🟦️component.ts` import from ShardClient. Four suites failed collection, zero bodies executed. This is neither a tutorial semantic RED nor a renderer behavior regression. The complete output is retained in `🧪️renderer-tutorial-local-red-r1-2026-08-27.txt`. An isolated canonical UI target is being checked separately; no result is assumed.

## Executed Cold Composition Boundary

The isolated default UI tier hit its unchanged 15-second process watchdog before a body result. Long R3 then reached the fixture validator and exposed that the compiled JSON import did not preserve the hostile `__proto__` domain as the actual JSON file does. The fixture now uses Node filesystem text plus `JSON.parse`, with a concrete path (the first DOM URL-object attempt was rejected by the filesystem boundary). Renderer R4 discovered 645 tests but skipped all of them: this test is owned by the separate UI React source target, so that exit-0 result is explicitly not a pass.

The canonical UI source selector finally produced the intended R6 RED: **1 failed, 547 skipped**, because the new helper exports were absent. R7 then passed **1 test, 547 skipped, 548 total**, exit 0, 15.82 seconds. It executes all five neutral cases against Immer Map composition, Node deep equality and strict Ajv. The helpers preserve comma-bearing ordered IDs, anchors, modes, granularity, sparse deletion, private domains and prototype-spelled keys without mutating their input. Both helpers are explicitly named `Cold`; they perform ordinary collection traversal and are not mounted into live recording or restoration.

Exact selector: `bun x nx run @semio-tech/ui-react:test-long --skip-nx-cache --args='--run 📦️index.tsx -t TutorialLocalInteraction'`. Logs `🧪️renderer-tutorial-local-ui-red-r6-2026-08-27.txt` and `🧪️renderer-tutorial-local-ui-green-r7-2026-08-27.txt` retain the actual semantic RED/GREEN. The seven live/tutorial joins remain open; the authored manifest shape and live consumers have not been changed by this isolated helper gate.

## Deterministic Cross-Language Ordering

The contract now specifies unsigned UTF-16-code-unit lexicographic domain order, with no locale comparison or Unicode normalization. Implementations using scalar strings must compare their UTF-16 encoding. The sixth neutral case includes integer-looking domains `2` and `10`, a supplementary-plane emoji and a BMP private-use character. R8 actually failed because insertion/own-key order emitted `2` before `10`; R9 passed **1 test, 547 skipped, 548 total**, 7.54 seconds after sorting the authored domain union. Logs are `🧪️renderer-tutorial-local-order-red-r8-2026-08-27.txt` and `🧪️renderer-tutorial-local-order-green-r9-2026-08-27.txt`.

The Node assertion fixture imports the concrete default export with the owned `node:assert` type, removing the reported TS2322 test boundary. The coordinator independently reran the canonical selector: **1 passed, 547 skipped, 548 total**, 13.73 seconds with seven stable source/config hashes; see `📓️coordinator-tutorial-local-r1-2026-08-27.md`. These six cases remain cold authored composition. No full-map spread/diff is installed in live capture, record or restore callbacks, and the seven live joins remain open pending the real fresh-identity producer.
