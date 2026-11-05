# Source Outcome Callers

Read-only current map. Source step/close currently mix selected work plus ObservedSource byte_at counts and native actual_work max. This can count demand/preflight reads before grant admission and duplicate semantic child work. Root owns replacing this instrumentation with explicit selected/delegated action outcomes.

Actual caller map:

- SPR 📜️history/🦀️.rs wrapper: admission331, ready transfer348, normal step352, fault transfer441, partial transfer445, close449 now capture turn.work before turn.result. Its with_control and consuming caller must debit that actual work once even on error; physical copied_items is not fuel.
- Store native snapshot receiving/📜️pair: phase4 admit remains stale Result syntax; phase5 fault transfer and phase6 ready transfer remain stale, as do close fault/partial branches. Capture turn.work before Result matching and retain full failure owner; normal Source step/close already extract outcome.work. Editor/runtime notified.
- Plugin document maintenance-ladder DecodeParent207 calls outer wrapper decoder.step and reads physical take_receipt but does not visibly read actual work at this callsite. Parent must consume wrapper last_turn_work including failed-prefix paths before propagating errors, without charging physical items as a substitute.
- Existing Source/frames native fixture loops need .result matching and independent actual work assertions after the signature port; model success is insufficient.

No Source/Metadata edits or native jobs performed.
