# 🔎️ artifact_create `s.note.note` fails with BUDGET_EXCEEDED

## Reproduction (2026-11-26, probe.py, two creates on one fresh server)

- `e2e-x1` failed after 168 s, `e2e-x2` after 155 s: `BUDGET_EXCEEDED: guest poll reached its wall budget before it settled`, retryable, plugin `note`.
- The message has no ` (poll stage=..., ...)` suffix although `poll_deadline_fault` in `🔁️native-poll/🦀️.rs` always appends one.

## Findings

1. The server that answered is a STALE publication. `repo dev mcp stdio` serves the previous published executable
   (`.cache/b/r/m/published/6d3390d7b4aeb20c3436/prototype/semio-os-mcp`, built 2026-11-25 18:06) from a private copy
   while the refresh runs. The sources of the poll path are newer (`🔁️native-poll/🦀️.rs` 2026-11-26 01:18,
   `🖥️host/🔁️poll/🎛️session/🦀️.rs` 00:28) and the running binary does not contain the `loan header=` diagnostic format.
   So the exact message is produced by the older code, not by the current `poll_deadline_fault`.
2. The refresh of that publication did not complete in this session: `repo run @semio-tech/framework-os-mcp-rs:build`
   ran 1 h 19 min and failed in the resident service with `BUILD_SOURCE_CONFLICT: Conflicting inference observations`
   (closure-graph root `.../🔌️plugin/🏪️store/📦️packaging/🔌️provider/🟦️.ts`), and a retry sits queued on host memory
   capacity behind other agents. The probe therefore keeps talking to the stale binary.
3. The stale binary is the debug (`prototype`) build; in it the guest runs on the in-repo interpreter: stderr shows the
   guest at 734 M fuel in phase Poll with tens of thousands of `operation-clock-us` host calls per turn. The wall budgets
   (`COMMAND_RESUME_WALL_BUDGET` 60 s, `INSTANCE_OPEN_WALL_BUDGET` 210 s, rationale in
   `💰️budgets-and-limits/README.md`) were written for compiled-runtime turns, so an unoptimised interpreter run cannot
   settle in them. This is a hypothesis; it could not be confirmed against current sources because no fresh binary
   could be built.

## Status

No source change was made: the failing code could not be reproduced on current sources, and inflating the constants
is not justified by the evidence. Next step once `repo run @semio-tech/framework-os-mcp-rs:build` succeeds: rerun
`python3 🐍️probe.py calls.jsonl 900`; a deadline fault will now carry the `poll stage=` suffix that names the phase.
