# Retained Dispatch Census Binding

## Exact Scope

The full census still required worker construction, retained-wire dispatch and one initial worker turn inside `dispatch_typed_command_inner`, even though the executed registered dispatch route delegates these steps to `start_typed_command_operation`. The existing helper-aware proof already binds pipeline admission, exact direct helper call, pending latest-wins storage and the real reserved operation slot before preparation. The repaired census reuses that proof rather than introducing a generic exception.

The new check requires one production dispatcher declaration, one exact helper declaration, one exact helper call, and exactly one retained-wire dispatch, worker construction, fixed operation insertion and initial pump in order. A direct worker/dispatch or reducer in the dispatcher, or a run-to-completion path in either body, rejects. This does not certify the generic command implementation, payload cleanup or arbitrary child publication; their independent failures remain intact.

## Test-Driven Evidence

The language-neutral `🔣️mounted-dispatch-binding.json` fixture and strict schema contain the unchanged route and nine hostile mutations. Ajv independently checks the valid mutation identity; each mutation is applied to its exact production function, not a same-named token elsewhere in the file. Cases cover wrong, missing or duplicate helpers; missing pipeline admission; duplicate sessions or initial turns; direct reducers/dispatch; and run-to-completion.

The first canonical run stopped before these tests on a peer's temporary reserved `package` binding in repository discovery (`🧪️member-dispatch-binding-red-r1-selftest-2026-08-27.txt`). That peer fixed the binding without edits from this executor. The second run reproduced the intended RED: the unchanged production route was rejected (`🧪️member-dispatch-binding-red-r2-selftest-2026-08-27.txt`). The first two green attempts exposed fixture mutations hitting earlier unrelated guard/session occurrences; the fixtures now target the exact dispatcher/helper bodies. No runtime guard was weakened.

The canonical Nx selftest then passed918 checks,33 exact factory owners,255 custom rows and25 generic rows (`🧪️member-dispatch-binding-green-r3-selftest-2026-08-27.txt`). Full census r4 emitted `📊️member-mounted-source-full-census-r4-2026-08-27.json` and its paired text output. It remains RED:773 rows,351 source-bounded,315 batch-only,2 forbidden,269 remaining,918 selftests and12 top-level failures. Exactly the stale single-turn-dispatch failure disappeared; command classifications and unresolved counts did not change. No all-app completion or latency claim follows from source validation.

## Changed Files

- Root `📜️script.ts`: exact helper-aware full-census predicate and hostile tests.
- Plugin retained-command `🧬️schema/🔣️mounted-dispatch-binding.schema.json`.
- Plugin retained-command `🧪️fixtures/🔣️mounted-dispatch-binding.json`.

No Rust runtime or app source changed in this verifier packet.

## Adjacent Audit, Not Yet Changed

`toolJobPeerInteractionRootsExact` still extracts `dispatch_typed_command_inner` and then requires six immutable-capture expressions now located in the already-bound `start_typed_command_operation`. Its failure therefore contains a stale source-location requirement. The same predicate does not yet require the newly native-verified exact peer base-root/installed-factory publication checks. A follow-up must bind the exact helper and add those stronger obligations and hostile mutations while preserving every existing ingress/channel check. The current packet leaves this separate failure intact; the twelve passing Store Presence laws do not alone certify every plugin roster-ingress path.

## Peer Capture Follow-Up

The coordinator authorized that follow-up. Canonical RED reproduced rejection of the real helper/base/factory source (`🧪️member-peer-capture-binding-red-r1-selftest-2026-08-27.txt`). The resolver now reuses the exact single-turn/helper proof for the six unchanged immutable-capture expressions and additionally requires the existing peer publication authority predicate. That predicate is shared with the four language-neutral, native-executed peer-commit cases rather than duplicated.

Authority checks require private commit fields, exact retained base identity, the installed factory identity, rejection before root replacement, transfer of the displaced root before base-alias release, and retention/explicit release of the rejected commit's original base. Ten whole-predicate hostile source mutations exercise wrong helper, whole-roster capture, stale base, foreign factory, public/forged authority, premature alias release, lost rejection base, removed roster freshness validation and missing ingress admission. Every other prior ingress, cancellation and close condition remains present.

Canonical GREEN passed929 checks,33 exact factory owners,255 custom rows and25 generic rows (`🧪️member-peer-capture-binding-green-r1-selftest-2026-08-27.txt`). Full census r5 emitted `📊️member-mounted-source-full-census-r5-2026-08-27.json`: RED773 rows,351 source-bounded,315 batch-only,2 forbidden,269 remaining,929 selftests and11 top-level failures. Only the stale capture-source failure disappeared; category counts did not change. This only repairs source resolution and strengthens its authority obligations; it adds no new runtime coverage beyond the separately recorded native gates.
