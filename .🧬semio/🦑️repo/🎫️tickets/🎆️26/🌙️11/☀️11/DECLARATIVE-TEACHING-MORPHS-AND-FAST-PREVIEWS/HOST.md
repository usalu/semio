# Host Snapshot (2026-11-11, ~04:45 local)

- OS: Windows 11 Pro 10.0.22631, Git Bash + PowerShell.
- CPU: AMD Ryzen 7 5800X, 8 cores / 16 logical.
- RAM: 63.9 GB total, ~31 GB free at snapshot.
- GPU: NVIDIA RTX A2000 (driver 32.0.15.8216) plus Microsoft Remote Display Adapter (workstation is an RDP session: browsers may report reduced motion).
- Tools: uv (scoop shim), bun (npm shim), system python 3.14; teaching venv is uv-managed CPython 3.12 at `.🧬semio/🎓️teaching/⚡️cache/python`.

## python.exe processes at snapshot (do NOT kill; none started by this ticket)

| PID | Parent | Started | What |
| --- | --- | --- | --- |
| 52640 / 34420 | 53916 / 52640 | 11-10 12:25 | Another session's long course render: `clip/render/🐍️.py --language en --audience speaker --quality l --fps 15 --jobs 1 --video bim_objects ... bim_consistency` (teaching venv, uv python 3.12 child) |
| 5024 / 5692 | 34420 / 5024 | 11-10 21:07 | Its current manim worker: `manim ... videos/en/speaker/s/116 ... bim/clip/representations Beat3_SameObject` |
| 37796 | 72564 | 11-06 18:37 | Stale system-python 3.14 `-c` probe (framework/modules lookup), another session |
| 8236 | 48084 | 11-06 18:43 | Stale system-python 3.14 `-c` probe (rglob rs files), another session |
| 49372 | 45956 | 11-06 18:44 | Stale system-python 3.14 `-c` probe (receipt search), another session |
| 70888 | 51220 | 11-06 18:50 | Cursor agent store `resolve-batch09.py`, another session |

Consequence: an `en` `speaker` render is running (language `en`, quality `l`, fps 15), writing under `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker`. Do not start any `en` render or write to that output root; benchmarks on this host share CPU with it (Manim worker uses ~1 core).
