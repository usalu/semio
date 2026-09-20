# Stepper Press and Pointer Cancellation

The actual Puzzle3D Settings document supplies four retained NumberSteppers. Root added a language-neutral gesture schema and fixture with plus release, minus release outside, plus cancellation and center-value press. The same fixture drives the real React Interpreter and the retained WGPU engine.

The new native law failed before production changes: plus press emitted zero actions instead of one. WGPU now emits the step on primary press and suppresses an additional release commit. The existing absolute-versus-delta binding law was updated to inspect that press boundary and require release silence. UI 52 passed all 641 tests. The actual React oracle passed six tests including schema validation, and demonstrates identical absolute values and action counts at press and terminal cleanup.

Cancellation now has an explicit lossless browser wire variant and distinct `DispatchEvent::PointerCancel` / `UiEvent::PointerCancel`. The browser registers `pointercancel`; Rust wire decoding projects it; both native TouchPhase::Cancelled ingress paths preserve cancellation rather than creating a successful release. The normalized dispatcher retires only the named pointer's capture. Its two-pointer regression law passed in the 137-test UI render suite.

The App interaction handler retires its pointer owner, calls Shell cancellation and cancels active World relocate state. Shell cancellation retires retained capture, canvas/list transfers, pending dock/tree/chrome releases and aggregate drag state. The retained router clears ACTIVE, capture, hover, thumb and press state without firing a value or click command. The old Canvas Escape cleanup now uses this explicit retained cancellation event. Shell integration laws additionally cover cancelled Display transfer and exactly one Canvas cancelled terminal action; these still await the full native renderer census.

Wire and transport tests passed 67/67, including the new shared DOM→wire→normalized cancellation fixture. A first run only required the expected event-kind census to include the new variant. Actual application pointercancel acceptance awaits activation 16. Existing special scene owners beyond Canvas and World relocate require the route audit's final review; this report does not imply accepted Ink/Graph/Board cancellation.

No new runtime dependency or background script was introduced. The test adapter adds the existing Testing Library mouseLeave event behind the repository's own test interface. Temporary outputs remain under this ticket's generated directory.
