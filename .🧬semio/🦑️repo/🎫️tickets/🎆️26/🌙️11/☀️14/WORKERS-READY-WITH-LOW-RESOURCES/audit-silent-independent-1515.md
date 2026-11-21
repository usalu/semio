# Independent Evidence for Silent Workers at 15:15 UTC

Read-only inspection on 2026-11-21 at 15:15 UTC of the coordinator local mirror `/Users/Shared/semio-store/v1/origins`, `heads`, and named fleet records. No host administration or credentials were accessed. No fresh independent worker record was found for m04, m06, or m09 in the inspected surfaces. This cannot establish their current power, network, processes, readiness, or exact Git stage.

- m04: newest non-auth origin record is `pool.json` at 2026-11-20 23:53:12 UTC, followed by `ready.json` at 23:53:10 and tools log 23:53:08. The historical pool record reported memory pressure and a cache that did not answer; those are old observations, not a current cause. Its most recent head is from the same night.
- m06: newest non-auth origin record is `attest.json` at 2026-11-20 18:25:12 UTC; `pool.json` and `ready.json` were at 18:25:11, workspace log at 18:25:05, and graph `slice-memory.json` at 18:24:35. The historical pool had a non-answering cache and was starting a child. None is fresh evidence of the present gate.
- m09: no non-auth origin records and no corresponding head. Enrollment is not a worker heartbeat.

The declared `repo remote-execution workers` read was attempted but exited 1 because no scheduler URL was configured; it supplied no scheduler worker data. The inspected fleet directories contain no filenames naming these three origins, but hash-named artifact records were not exhaustively attributed to hosts. Absence in this bounded inspection does not prove that no other scheduler or host-side record exists. Fresh host-originated attestation/Ready or independently timestamped scheduler evidence is necessary before acceptance can assert these three are current or identify a present blocking gate.
