# Recovery Process Audit

Observed 2026-11-05 09:44:22.307 UTC (`atMs=1793871862307`) using the read-only command `repo doctor --only=processes,leases,capacity --facts --lang=en`. All three probes reported `ok`. No gates were started or cancelled.

| Physical PID | Parent | Current command / evidence |
| --- | --- | --- |
| 34296 | 30896 | Repository wrapper: `repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long` |
| 29656 | 34296 | Captured implementation runner: `run @semio-tech/draw-drawing-rs:test --lang=en --parallel=1 -- long`; age 22392 seconds, RSS 1142 MB |

PID 34296 held a `run` capacity grant with one job, 3339 MB reserved, 1172 MB observed and age 22393 seconds. PID 29656 held the exclusive `javascript-dependency-build:C:\git\semio` lease, shared native image leases for `.cache/b/o` and `.cache/b/d`, and six shared JavaScript dependency pin leases. This is the existing native Draw gate and dependency producer; retain it and avoid a duplicate native run.

The physical process inventory contained no command invoking `draw-js:test`, `framework-process:test-root-family-preservation`, or `draw-plugin:guest-instance-check`. Their old tool session identifiers 48719, 97196 and 72068 do not establish physical process identity. None of those numbers, nor native tool session identifier 22389, appeared as a physical PID in this snapshot. The native process instead has the concrete PIDs above.

Absence here is limited to the repository doctor's current process inventory and command matching. It does not supply successful test results or recover old tool session output. Previously reported guest and root-family log failures remain separate evidence. Long captured runner command lines embed implementation source; inspect their final task arguments when matching routes.

The repository-owned doctor interface supplies PID, parent PID, command, age, RSS, leases and capacity grants. It exposes no attachment address for recovering a lost tool output session. Do not infer completion from stale ticket logs or unknown tool-session lookups.

Housekeeping: the audit's redundant read-only directory-print query (`repo tools exec --light python`, tool session 10442) was cancelled through its exact tool handle after its path was supplied independently. The terminal result was exit code 1; its last output remained `boot.loaded` at 110 seconds. No product gate or peer process was changed.
