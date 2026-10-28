# Current Source Failure Triage

The source-only refresh reported 110 certified sources of 117 and seven unsuccessful entries. No duplicate full source render was started.

| Source | Reported failure | Current finding |
| --- | --- | --- |
|118 Exchange | Changed authored inputs after native completion | Normal guarded retry |
|119 Consistency | Missing _identity | Current rewrite includes _identity and all 20 beats; preserve and retry |
|120 Extrema | Changed authored inputs after native completion | Normal guarded retry |
|121 Sampling | Missing _worst_bar consumer | Current rewrite removed the consumer; preserve and retry |
|122 Experiments | Changed authored inputs after native completion | Normal guarded retry; preserve root updater repair |
|124 Pareto | Two-component cross mark point | Current source already places it at explicit x,y,0; preserve and retry |
|125 Algorithms | Retired room/table widgets | Correct to current two rooms and heating/cooling/lighting ledger |

## Algorithms Repair

A language-neutral widget ownership fixture and native Manim consumer controls live under algorithms/🧪️tests/🧩️widgets. After test setup corrections, RED reproduced missing room in English/German and independently exposed undefined ROOM_AT in the closing block.

Genetic transitions now consume/reveal the existing room pair, bar and ledger members. The closing beat reuses these current widgets and note; its answer remains in the vacated map column. No aliases or shared helper changes. Narrated beats and concurrent source changes are preserved.

Atomic source hashes: c3beab6961f6a0a5957c36210e7fd29ad4fa977a955d3d409f869587eb19f442 → 5245355f1f26b4f9666a7a7ae8ce571ff5770d42f7622db6c2a82f9129b9abc4. Focused GREEN is pending. The root publisher remains the sole actual source output owner.

Focused GREEN: two tests passed in 0.154 seconds, with English/German subcases. Native Manim setup/begin/interpolate/finish/cleanup validates genetic consumption and reveal of actual current widget identities. The actual closing AST block reuses the same room pair, bar, ledger members and note. The fixture/driver is registered in authoring/📜️script.ts. No full source render duplicated the root publisher.

Fresh owner27984: algorithms125 attempt1793143300764491500/PID37644 exited1 after142.850s. Its log stops at animation159 FadeOut(Rectangle), frame4/8; no traceback/error/exception/abort/critical/cancel markers occur anywhere. It correctly yielded waiting_for_input_change outside the worker pool. The authored leaf changed after exit at1793143488.774 toSHA109358dec65e297cb839053a7f9e6265107d4ba1d10db94bae6aea5ff46dc55b. A normal retry under those new bytes is legitimate. Termination cause remains undiagnosed; no speculative source/helper change or duplicate encoder was made.

Superseding diagnosis of the125 abrupt exit: root found terminal publisher46266 PermissionErrorWinError5 in record() atomic status replacement. The fair coordinator cancelled active peers before joining. Thus the abrupt125 log is the expected owned peer cancellation, not a story defect. The status-only bounded retry fix and native RED/GREEN are recorded in inspection-record-commit.md. No algorithms source change was made for that event.
