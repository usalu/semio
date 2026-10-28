# Bounded Publisher Status Commit

Root traced the terminal publisher failure to record(): Windows denied atomic publish.new→publish.json replacement while a reader had the old status file open. This failure propagated from a worker; the fair coordinator then correctly cancelled its owned peers. Algorithms' abrupt closing log was that peer cancellation, not evidence of a story defect.

Portable contract: inspection-record-cases.json. The actual old record() reproduced exact WinError5 with a real native CreateFileW reader. The concurrent-reader test also reproduced all four writer failures. Before the fix, five tests were RED in0.178s: three failures and two errors.

Only the ticket helper inspection-publish.py changed. Its existing serialized _LOCK, single candidate write, and atomic replacement are preserved. Replacement alone retries PermissionError up to50attempts with at most49×10ms waits. The old JSON remains untouched until a successful atomic commit. Other errors propagate immediately; permanent permission denial still propagates after the bounded retry and cancels active peers.

GREEN against the actual helper:
- Five status controls0.527s: native Windows reader releases after0.12s, deterministic transient failures, permanent50-attempt bound and unchanged old JSON, four concurrent writers/readers validated independently by existing Node JSON/assert, and permanent record failure cancelling a blocked peer before pool join.
- Six fair scheduler controls0.569s.
- Four owner coherence controls0.362s.

No production Python, body, compositor, duration, certificate, or semantic validation changed. Actual helper SHA d0765b0013bc29d01b384c3bb7cfe11e2156f712106fe4b9176ba696cb0ae9e0. Compositor and collection remain1304ba6990e8251a1352c202cee42e80ac0e12772395f89ef3d47d6c20c8c165 /6242f2457ffbf5bfe6ab7d15ebbc7323d517fc18074e04dbebc9befe3e534695.

Doctor facts1793143825273 show grants[]/waiters[], no oldPID27984, publisher, or owned native encoder/browser/Manim children. Only the existing independent chapter verifier, beat-sheet watcher, and body-failure observer remain. Evidence: generated/ifair/record-old-owner-doctor.json. Root owns the fresh sole-publisher launch; valid media remains reusable.

Inputs: inspection-record-test.py, inspection-record-oracle.mjs, inspection-record-cases.json. Logs: generated/ifair/record-{red,green,fair-green,owner-green}.log. The original fairness candidate remains historical input only; test drivers import the actual helper by default.
