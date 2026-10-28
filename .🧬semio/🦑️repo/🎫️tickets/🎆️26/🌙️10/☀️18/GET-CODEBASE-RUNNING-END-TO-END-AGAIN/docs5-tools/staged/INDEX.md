# Staged doc batches (docs5)

Each batch: `python3 -I staged/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, exit 2 on drift, writes nothing then). Laws: `sh staged/<any>/verify.sh <mirror root>`.

| # | batch | document | edits | adds |
|---|---|---|---|---|
| 1 | `test-run` | `📓️build-guide.md` | 6 | How to run one test file correctly: from the checkout root, why, how to read a bare red, and the troubleshooting row for the missing process invocation. |
| 2 | `test-envelope` | `📓️build-guide.md` | 2 | How a test file declares the process birth its suite grants so that a bare run gives the truth; what cannot be declared yet. |
| 3 | `test-scratch` | `📓️build-guide.md` | 2 | Where scratch and artifact directories must live and why: Cargo ancestors, observation ancestry, the 256-unit bound, sockets, images. |
| 4 | `providers` | `📓️build-guide.md` | 4 | Script providers: what a command-closure edit re-infers, how to read a refresh, how a consumer expands a provider reference. |
| 5 | `farm-memo` | `📓️build-guide.md` | 2 | The graph farm and its lines, why it is refused today, and the scoped verification of a slice with its derived-results line. |
| 6 | `capacity-lines` | `📓️build-guide.md` | 4 | How to read a capacity line: the reasons, the admission and growth lines, the blocker part; what the agents16 batches change for three agents on 16 GB. |
| 7 | `promote` | `📓️build-guide.md` | 2 | The promote rule in sealed mode: who promotes, when, what an exit 1 means, what to expect afterwards. |
| 8 | `verdict` | `📓️build-guide.md` | 2 | The group verdict recipe of the gate: stages, outputs, what the numbers say, expected minutes. |
| 9 | `readme-tests` | `README.md` | 2 | README, Test sandbox: running a single file from the root, declaring the birth, placing scratch (EN and DE), pointing at the operator guide. |
