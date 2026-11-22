# Owner Admission Metadata Sharing

Worker m02 reported 307,657 distinct source observations, 1,410 completed owner checks and 745,408 verified memo rows at approximately 19:17 UTC. At 19:23 it had completed 1,930 owner checks and reported 1,136,128 verified memo rows. Worker m10 reported 308,542 distinct source observations and 1,228,352 verified memo rows at 4,154 owner checks. These counters measure memo work across owners. They do not count physical filesystem requests.

The admission verifier already shares verdicts for the same observation object in one scope round. A physical stat, file-type query and directory-type query about the same path still used three system calls when held as different observations. The verifier now shares the recorded followed type or exact stat error per rebased path in that round. The policy caps its cache at 8,192 paths. Negative file and directory answers, precise stat errors, and separate no-follow link observations retain their prior meaning. The final seal retains its independent physical verification. The empty stable-directory case also avoids constructing a parent path on every row.

The SQLite oracle supplied independent verdicts and exact expected metadata calls for a file, directory, absent path, access error and unrelated no-follow link. The red fixture recorded twelve followed calls. The same fixture recorded four with the change, and the final seal immediately rejected a changed target type. A broader test run exposed two current Source exports missing from its neutral roster and a fixture filesystem port that also intercepted Source ancestry. The roster now matches both the runtime and declarations. The test port delegates paths outside its fixture to the actual filesystem. The final focused owner-admission and inventory suite passed 13 tests and 256 assertions.

The native filesystem benchmark used 512 unique private paths and three independent owner questions per path, alternating order over eight measured runs after warmup. The separate physical checks made 1,536 stat calls, with median 9.87 ms elapsed and 12.20 ms CPU. The owner-admission round made 512 calls, with median 5.73 ms elapsed and 5.64 ms CPU. The comparison measures that exact fixture on this coordinator. The whole fleet effect depends on the actual mix of row kinds and storage.

The complete observation suite subsequently passed 69 tests and 382 assertions.

[Native measurement script](measure-admission-stat-sharing.ts)
