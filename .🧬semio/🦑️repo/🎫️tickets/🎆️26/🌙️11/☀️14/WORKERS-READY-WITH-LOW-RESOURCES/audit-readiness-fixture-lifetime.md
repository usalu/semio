# Detached readiness fixture lifetime

The readiness containment test starts detached Bun grandchildren with a fixed synthetic lease request. An assertion could fail before the test copied its process IDs into the cleanup list. The cleanup then missed the children, which ran an interval indefinitely.

The private fixture now exits when its script disappears and has a 125-second lifetime as a fallback for abruptly interrupted tests. The containment scenario also reads each private registration in its cleanup. A real detached child stayed alive after its fixture directory was removed on the old code and exited in the corrected run. The lifetime, normal adoption, and waiting-holder tests passed together with 21 assertions.

The coordinator had 42 positively identified abandoned test children. Each belonged to the local user, was reparented to PID 1, was older than 30 minutes, carried the exact synthetic request and had neither its launch script nor its private registration directory. Their aggregate sampled resident size was approximately 200 MiB. After a fresh per-process identity check they received SIGTERM. The follow-up process list contained zero matching expired fixtures. Current tests and ordinary build processes were outside the selection.
