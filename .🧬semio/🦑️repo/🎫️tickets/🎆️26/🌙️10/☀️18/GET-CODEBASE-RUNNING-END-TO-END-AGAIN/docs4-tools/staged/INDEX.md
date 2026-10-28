# Staged doc batches (docs4)

Each batch: `python3 -I staged/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, exit 2 on drift, writes nothing then). Laws: `sh staged/<any>/verify.sh <mirror root>`.

| # | batch | document | edits | adds |
|---|---|---|---|---|
| 1 | `graph-memo` | `📓️build-guide.md` | 4 | The graph memo: when a repeated request is served without a graph child, the two lines it prints, the switch, the refusal reasons. |
| 2 | `plan-reuse` | `📓️build-guide.md` | 8 | Bazel plan reuse: what keeps a plan, what ends it, the two lines, the troubleshooting rows; the 'not measured yet' sentence becomes the measurement. |
| 3 | `action-env` | `📓️build-guide.md` | 6 | The action environment allowlist, how a task declares a variable, runtime rows keyed by output only, the warm-cache row and the troubleshooting row. |
| 4 | `lock-order` | `📓️build-guide.md` | 4 | The lease classes with their ranks, the [lease-order] line, the switch and the registry; the troubleshooting row. |
| 5 | `capacity` | `📓️build-guide.md` | 4 | The quiet reservation of a hot no-op, the bound of what it grows into, the partial grow line and its troubleshooting row. |
| 6 | `rust-path` | `📓️build-guide.md` | 2 | The Rust test and build path: the preparation witness, the lease order of the compiler writer, the admission memos. |
| 7 | `measuring` | `📓️build-guide.md` | 4 | A new section on measuring a build (clean environment, the third run, the proof lines) and the 10-27 measurements in the status. |
| 8 | `readme-bazel` | `README.md` | 4 | README, Bazel implementation: plan reuse and the action environment allowlist, EN and DE, pointing at the operator guide. |
| 9 | `g34` | `📓️build-guide.md` | 10 | The G34 group: memo stamps for unread siblings, the journal's new place, runtime rows without a child, the plan walk skipping ignored directories. |
