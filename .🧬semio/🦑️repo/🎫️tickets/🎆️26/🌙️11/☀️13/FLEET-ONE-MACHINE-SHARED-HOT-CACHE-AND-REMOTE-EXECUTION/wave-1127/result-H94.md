# Result H94: sealed tool snapshots (staged, not landed: freeze)

Design: `design-H94-sealed-tool-snapshots.md`. Module (new): `B/🏛️sealed/🧰️tool-snapshot` (schema, policy EN/DE, capture/materialize, `🚧️gate`, `🔁️promotion` + `🚀️flow`, `👁️watch`, `⌨️command`, `📜️script.ts`, README EN/DE, `🧪️tests` + `🐍️oracle.py`). Patches: `T/🧰️scripts/H94/sidecar.patch`, `sidecars.patch`. Stage for `repo land`: `scratchpad/H94/stage-H94` (15 files, 2 live bases).

## Verified
- Own tests 26/26 (also with SEMIO_LANGUAGE=de): schemas under repository validator and Ajv with mutations; digest and 400 random event-log folds equal an independent Python oracle (and `shasum`); capture rules, instability, materialize (read-only, concurrent processes), gate order/fail-fast/real syntax+imports, promotion, rejected-not-regated, rollback, auto-rollback on faults, claim races and lease takeover, GC, path limits, CLI EN/DE JSON; a real role keeps running sealed code (including lazy imports) while its live file is overwritten with garbage; real sidecar roles in a real snapshot: scheduler-only edit leaves the agent unchanged, agent edit restarts only the agent.
- Real tree (read only): capture 4 698 code files / 44 MB, syntax gate passes (3.7 s), imports gate passes (closure whole after adding closure extras), manifest-based role closure equals the live sidecar scanner exactly (agent 1159 = 1159, diff 0).
- `repo land check` of the stage: debug-lines, dangling-imports, typecheck (9 s) and tests passed, exit 0 (an earlier run was red on 6 real type errors, fixed).
- Patched sidecar: existing sidecar tests give the same results as live (handover, code-record, priority green; `🧮️closure` 6/1 and `🏊️pool`/`🔁️restart`/`🚰️drain` fail identically on live: module-not-found from a peer overlay path `scratchpad/H29/overlay`).

## Unverified
- Gates `typecheck`/`smoke`/`self-run` as the tool's own gate on a real overlay (swap 0.9 GB free, freeze); `repo land check` of a canary stage (assumed to accept an unchanged file).
- Promotion across real replicas / shared NFS store; Windows (junctions, long paths) not run.
- Catalogue verb `tool`, trampoline phase B, native registry seal: designed, not staged.

## Incident
My scratch tree `tree2` hard-linked live sources (nlink 2) and made the revision gate refuse commands; the coordinator removed it. Rebuilt with plain copies (`tree2.py` no longer uses `os.link`), verified no multi-link files. Module run folders hard-link only own blobs, never repo sources.

New folder emoji to check at taxonomy landing: 🧰️tool-snapshot, 🚧️gate, 🔁️promotion/🚀️flow, 👁️watch, ⌨️command, 🐍️oracle.py.
