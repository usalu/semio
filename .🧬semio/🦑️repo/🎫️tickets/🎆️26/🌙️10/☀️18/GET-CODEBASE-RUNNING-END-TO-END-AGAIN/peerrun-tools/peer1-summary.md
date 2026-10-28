## Peer edits (peer1)

| # | stage | when | exit | wall s | executed | Bazel | graph | plan | restarts | edit during run | verdict | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 00 | control:course |  | 0 | 517.7 | 0 | 2 processes: 6 action cache hit, 1 disk cache hit, 1 interna | planned | again | 0 | - | **PASS** |  |
| 01 | control:course |  | 0 | 201.5 | 0 | 1 process: 1 internal. | planned | reused | 0 | - | **PASS** |  |
| 02 | peer:p1:course:graph | graph | 0 | 1726.4 | 7 | 8 processes: 1 internal, 7 worker. | planned | again | 0 | yes | **FAIL** | 7 re-execution(s), bound 0; noise: 60 other file(s) changed during the run |
| 03 | control:course |  | 0 | 294.6 | 0 | 8 processes: 7 disk cache hit, 1 internal. | planned | again | 0 | - | **PASS** |  |
| 04 | peer:p1:course:bazel | bazel | 0 | 623.2 | 0 | 1 process: 7 action cache hit, 1 internal. | planned | reused | 0 | yes | **PASS** |  |
| 05 | control:course |  | 0 | 537.0 | 0 | 1 process: 1 internal. | planned | again | 0 | - | **PASS** |  |
| 06 | peer:p3:course:graph | graph | 0 | 176.2 | 0 | 1 process: 1 internal. | planned | reused | 0 | yes | **PASS** |  |
| 07 | control:course |  | 0 | 237.3 | 0 | 1 process: 1 internal. | planned | reused | 0 | - | **PASS** |  |
| 08 | peer:p3:course:bazel | bazel | 0 | 72.4 | 0 | 1 process: 1 internal. | planned | reused | 0 | yes | **PASS** |  |
| 09 | control:course |  | 0 | 39.9 | 0 | 1 process: 1 internal. | memo | reused | 0 | - | **PASS** |  |
| 10 | peer:p2:course:graph | graph | 0 | 481.9 | 1 | 2 processes: 1 internal, 1 worker. | memo | reused | 0 | yes | **PASS** |  |
| 11 | control:course |  | 0 | 120.9 | 0 | 2 processes: 1 disk cache hit, 1 internal. | planned | again | 0 | - | **PASS** |  |

**ACCEPTANCE peer edits: FAIL** (12 stage(s), 5 peer stage(s) {'PASS': 4, 'FAIL': 1}, 5 with the edit during the run)
