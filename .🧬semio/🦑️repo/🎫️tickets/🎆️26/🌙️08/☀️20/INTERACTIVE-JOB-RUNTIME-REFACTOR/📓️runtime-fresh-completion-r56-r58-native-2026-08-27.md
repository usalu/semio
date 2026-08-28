# Fresh Component and Binding Completion — Actual RED/GREEN

R56 stopped before tests because the new fixture incorrectly assumed an existing `UiBindingsCopy::source()` accessor. The fixture was corrected to inspect the already-public actual candidate length/source allocation. R57 then executed the intended behavioral RED: **0 passed, 1 failed, 108 skipped**, 0.099s. Both fresh component and binding children had already transferred/cleared their roots in their completion turn.

The repair retains each completed child first, returns the original source in a later turn, returns the candidate in another turn, then clears the empty cursor separately. Component allocation is now its own zero-payload-work step. Binding allocation similarly receives zero copy credit; its exact existing 32768-byte binding work limit is unchanged. Binding root-return bits preserve completion authority after either root is transferred. No limits were raised.

R58 actual canonical runtime suite: **5 passed, 104 skipped**, 0.271s; process 51225 exited 0. It includes the new fresh component/binding completion law plus the previous four canonical/read-pressure/unwind laws.

Raw logs: `🧪️member-runtime-fresh-completion-red-r56-native-2026-08-27.txt`, `🧪️member-runtime-fresh-completion-red-r57-native-2026-08-27.txt`, `🧪️member-runtime-fresh-completion-green-r58-native-2026-08-27.txt`. Canonical route uses explicit `SEMIO_COVERAGE=0`, exhaustive native profile and the existing master target.

Fresh partial-return unwind and full shared/native regression remain queued. This result is not a full resident-census, transaction or whole-runtime completion claim.
