# Findings E61
1. Slice timers are 3-37x late (m08: 30 s timer fired after 1114 s) while the slice uses 6-12 % CPU: the process is starved/paged, not blocked by JS. A pool puller (5 x ~1 GB children) plus a 10 GB stage tree on a 16 GB Mac is the likely cause; admission of the puller while a stage reads should be gated on free memory (E56 area).
2. m01's fleet metadata waits (15.5 s average) are real waits for the fleet server, with an on-time loop; the server side under load 27-60 on the coordinator host is the candidate.
