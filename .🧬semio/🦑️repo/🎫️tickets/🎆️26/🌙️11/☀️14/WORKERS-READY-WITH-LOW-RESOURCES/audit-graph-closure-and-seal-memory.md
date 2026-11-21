# Graph closure recovery and seal memory

Generation 260 still has a null manifest and the recorded reason `closure-missing`. Its retained metadata lacks the exact input leaf, so this investigation cannot determine whether the active cold rebuild followed either reproduced path. It has not promoted the historical parent.

## Invalidation custody

In private real graph runs, a second builder published a healthy generation while the first request was settling a missing closure input. The former repair invalidated whichever pointer it read after that wait. It replaced the newer valid graph with a null pointer and caused a cold rebuild. This occurred both after local expansion and after a restore. An earlier restore report could also start settling an already replaced manifest.

Invalidation now compares the diagnosed generation and manifest digest inside the existing fenced pointer operation. The restored case takes the exact local digest returned by the successful install and confirms it before settling leaves. A restored leaf that is successfully refetched needs no invalidation. A real healthy successor remains current even if it was published while the original repair waited. Signed graph adoption still uses its existing verification and publication checks.

The neutral fixture exercises local and restored concurrent publication, a superseded restore report and a refetched leaf. It checks the exact live pointer, requires zero reinference by the older request and independently compares the resulting graph with Python. The three initial race cases failed before the change. A native cohort covering graph, loader, head-only adoption and remote closure leaves passed 142 tests and 8,781 assertions.

## Fragment payload lifetime

A cold build previously kept the full compute result and the fragment persistence callbacks through index construction and packing, even after all fragment writes finished. A controlled native fixture forced garbage collection at the existing pre-index storage step and still found the original owner node alive. The builder now retains the source row disposer separately. Once fragments finish, it drops the full compute result and completed callbacks, leaving the rows and small metadata used by index construction.

The native fixture now observes the owner payload released at that step. The source row disposer still runs after sealing, and the resulting graph matches the Python oracle. The focused run of both changes passed five tests and 24 assertions. The overall rerun passed 157 tests before six existing claim-progress tests failed to start because the direct launcher omitted the fixture's required artifact directory. Those six tests then passed with 35 assertions after the existing environment was supplied. This verifies one memory lifetime boundary. No native 8 GiB whole-graph peak or newly sealed physical generation was measured.
