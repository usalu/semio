# Replica Rollout and m07 Dependency Transition

On 2026-11-21 at 19:56:38 UTC the coordinator and m01, m02, m03, m08, m10, m11 and m12 freshly attested executing replica digest `06102e8b3eb19c67dd0eca3234e9920da806b8ff8288b41cc1f10d8d431d5516`. At 19:57:31 m07 also freshly attested that digest with zero incoming. m05 still attested `b0cc3cfb71fb` with 5,072 incoming, later 5,075 at 19:59:23. Source delivery and executing code are distinct observations.

The m07 dependencies attempt 6 began at 18:54:09.032 UTC and completed at 19:58:14.119 UTC after 3,845 seconds, exit 0, outcome ok and end evidence exit-receipt. Its timing digest was `26a898eac637`; this stage digest is not the replica digest. Ready advanced to workspace and remained false. At 19:59:23 the workspace reported waiting behind one checkout load, whose leader was checking the file inventory with 69,539 observations and 62,528 verified memo rows. This identifies upstream work, not workspace completion or Ready.

No process, replica or stage was changed in this observation.
