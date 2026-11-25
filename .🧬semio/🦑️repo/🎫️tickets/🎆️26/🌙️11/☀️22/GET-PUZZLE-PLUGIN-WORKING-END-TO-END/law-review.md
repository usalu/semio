IN WORK

# 3D mutation latency

The existing eight-turn law measured 59 translate and 62 delete ladder turns in the frozen native build. The one-object translate ran 561 staging items within 17 publication calls. With a 64-item grant, those staging items alone need at least nine calls even with no page framing, scope overhead or forced yields. The 4 ms and 50 ms worker-wall experiments produced identical totals. The law and the 4 ms wall are unchanged.

The fixture runs until the entire app operation is quiescent. The owner trace in `p1-3d.md` records 674 maintenance items after publication, including 656 for mounted retirement. Those later items count toward the native law even though they follow the result. Other large contributors were retained worker preparation, action ingress retirement and store staging. The cancellation-custody wait is reported only after the fair maintenance scan finds no progress, so it does not independently establish that cancellation is blocking a runnable edit.

The 3D transform runs a read and a commit step, and the operation sends a single parametric mutation. The publication pump had reserved store receiving-scope headroom even for work that precedes the receiving scope. Updated it to reserve that headroom only on the receiving path, leaving the remaining turn items available to page framing, ACK retirement, accepted receipts and store retirement. The existing unit cases still cover denied scans, and a new case covers funding a non-receiving step with one remaining item. This removes a premature yield condition. The staging lower bound means the law still needs a wider change to shared ownership work.

## Verification

- `repo test-file` rejects the Rust integration source with exit 2 because its runner accepts JavaScript and TypeScript.
- Ran the frozen 3D editor gesture executable under `repo capacity run`. Five tests passed, including the one-operation emit and one-record residency laws. The run exited 124 at the 180-second limit during the later attribution cases, before reaching the eight-turn law.
- The shared native runner exited 101 for the 3D workspace because its Cargo lock needed refreshing. The 5D lock admitted resolution, but the framework crate is only a dependency of that workspace, so Cargo exited 101 without running its unit tests. Root workspace resolution succeeded. `-p semio-plugin-instance-typed-ops --lib mounted_run_and_pump` is waiting on the existing native build's artifact lock.
- After the coordinator refreshed the 3D lock, queued `-p semio-s-artifact-puzzle-3d-editor --test editor-gestures mutation_latency::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns -- --exact --nocapture`. It is waiting on the same artifact lock. The new native logs are `🗑️generated/law-pump-root.log` and `🗑️generated/law-move-current.log`.
- Scoped `git diff --check` passed via the repo tool registry.
- The new guard regression also passed as an isolated Rust smoke test, compiled from the exact helper and test source with the pinned compiler. This checked the grant decision independently of the pending crate build.
