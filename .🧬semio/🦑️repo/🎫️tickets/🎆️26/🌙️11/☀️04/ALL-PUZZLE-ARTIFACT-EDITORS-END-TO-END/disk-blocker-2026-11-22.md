# Disk Blocker (2026-11-22)

`/home` (465 GB) is at 100% with 0.6–2.3 GB free. `repo run …:activate-puzzle5d-react-prototype` refuses with
"Partition /home has 607.5 MiB free, below the 12 GiB this command needs"; repo cache collectors free 0 B. A fixture write by L5 was
truncated to 0 bytes by ENOSPC (restored from `git show HEAD:` plus its edits).

## Largest consumers (ticket `🗑️generated` build outputs)

| Path | Size | Last write |
| --- | --- | --- |
| `🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/rust-build/debug/incremental` | 89 GB | 2026-11-21 03:47 |
| `🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/rust-build/debug/build` | 32 GB | 2026-11-21 03:47 |
| `🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target` | 27 GB | 2026-11-21 22:25 |
| `🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/w` | 27 GB | 2026-11-21 22:38 |
| `🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated/cargo-puzzle-clipboard-build` | 19 GB | 2026-11-21 22:38 |
| `.cache/b` (repo build cache) | 70 GB | live |

Cargo `incremental` directories are pure accelerators. Deleting the REMODEL one (89 GB, idle about 20 h, no process holding it) loses only
incremental reuse. The coordinator's deletion was refused by the agent permission classifier because the folder belongs to another ticket.
It needs owner approval.
