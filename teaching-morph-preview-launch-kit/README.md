# Teaching morphs and fast previews: Steam Deck / Bash launch kit

Implement a declarative morph mechanism that preserves rich full-quality teaching transformations for final renders while making previews inexpensive. Preview translates/scales the source, then atomically replaces it with the target at the transition's terminal scheduled sample. The task also covers canonical lower-LoD staffage, wider preview optimization, deterministic timing, resource cleanup, actual course integration, benchmarks, and repairs to encountered fundamental defects.

## Extract and run

Extract the ZIP directly into the repository root, preserving the folder:

`teaching-morph-preview-launch-kit/`

From your existing Steam Deck/Linux Bash terminal at that repository root:

```bash
bash ./teaching-morph-preview-launch-kit/run.sh
```

The launcher uses the kit directory's parent as the repository working directory. Keep the folder directly inside that root, without an extra extraction layer. Calling through `bash` avoids depending on the ZIP extractor preserving executable permission bits.

It checks required files, resolves the installed Claude executable, supplies the prompt through standard input, sets medium effort and the 20-subagent cap, and passes through output and exit status. Environment/directory changes stay inside its process. It performs no installation or authentication/billing changes.

## Explicit command

Equivalent one-line Bash invocation from the repository root:

```bash
CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20 CLAUDE_CODE_EFFORT_LEVEL=medium claude -p --model claude-opus-5-5 --effort medium --agents ./teaching-morph-preview-launch-kit/agents.json --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' < ./teaching-morph-preview-launch-kit/prompt.txt
```

Multiline form:

```bash
CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20 \
CLAUDE_CODE_EFFORT_LEVEL=medium \
claude -p \
  --model claude-opus-5-5 \
  --effort medium \
  --agents ./teaching-morph-preview-launch-kit/agents.json \
  --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' \
  < ./teaching-morph-preview-launch-kit/prompt.txt
```

Each backslash must be the final character on its line. Keep the entire kit together: the prompt asks Claude and its workers to read `acceptance.md` and `fleet.md`.

## Required behavior

| Concern | Preview | Final |
| --- | --- | --- |
| Before the terminal sample | Source only, translated/scaled toward target placement | Real authored morph intermediates |
| Terminal sample | Exact target replaces source atomically | Exact authored target |
| Asset representation | Canonical lower LoD chosen before expensive loading | Required final detail |
| Morph preparation | No full-morph work attributable to the preview job | Full preparation and execution |
| Shared meaning | Same timeline, references, narration, endpoints, and analytical meaning | Same |

The prompt defines the boundary against scheduled time/frames, independent of frame delivery and easing. It covers reverse seeking, dropped frames, loops, one/zero-frame transitions, adjacent morphs, trimmed ranges, and output-rate differences. Source/target pose sampling and placement are explicitly authored.

Render profile, authored drawing scale, view, viewport zoom, and asset LoD remain distinct. Profile selection and resource ownership belong to shared render infrastructure, so clip data stays declarative.

The implementation must avoid hidden expensive preview work in loaders, prefetch, caches, workers, and offscreen preparation. Final exports explicitly select full quality and cannot inherit a preview toggle or consume preview results as final output.

## Parallel fleet and resources

| Role | Model | Effort | Available profiles |
| --- | --- | --- | --- |
| Main coordinator | Opus 5.5 | Medium | 1, additional to workers |
| Implementation | Sonnet 5.5 | Medium | 14 |
| Read-only exploration/audits | Haiku 5.5 | Medium | 4 |
| Bounded registrations/documentation | Haiku 5.5 | Medium | 2 |

Twenty worker profiles are available; dispatch adapts to useful independent work and observed capacity. The prompt prioritizes fastest verified completion, with shared contracts settled early and new work assigned as dependencies clear. It does not keep workers busy merely to fill slots.

All profiles set a full model ID, medium effort, background execution, and denial of the Agent tool. Instructions also prohibit spawning through shell or separate Claude sessions. The four readers have an explicit read-only tool list.

`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20` is a session cap, not a dispatch command, an observed count, or the account's maximum. Existing runtime/account constraints may limit capacity. The prompt requires accurate task/execution records. Avoid a forced global subagent model if you want the Sonnet/Haiku split.

Local builds/browsers/renders share a coordinator-managed queue. It starts with one heavy verification job or shared browser/server session on the Steam Deck, then expands when measured headroom and independent outputs justify it. This workflow policy is not a process limit enforced by the launcher. Benchmarks use controlled, comparable conditions.

## Verification and completion

The task requires per-job evidence that previews perform zero full-morph work and final jobs perform the real path; exact boundary/determinism tests; persisted definitions; actual course routes and exported media; lower-LoD asset coverage; and stable retained resources across repeated lifecycle cycles.

Performance claims must come from recorded before/after measurements, including cold/warm behavior, frame-time tails, seek responsiveness, and resource retention where measurable. No speedup is assumed in this kit.

Confirmed fundamental defects encountered along the way, including unrelated ones, must be reproduced, repaired, and verified under explicit ownership. The handoff must include these repairs alongside the migration/coverage matrix, commands, proof media, measured results, and concrete remaining blockers.

## Requirements and files

Use the authenticated Claude Code installation already available in your terminal. The `--agents` JSON-file form with print mode requires **Claude Code v2.1.281 or later**. Check `claude --version`; update an older supported installation with `claude update`. Requested models must be available through your account/provider.

The allowed-tools argument preapproves named tools; it is not an exclusive tool allowlist. Existing runtime permissions and managed settings still apply.

- `prompt.txt`: complete end-to-end implementation task.
- `acceptance.md`: exact transition behavior and verification contract.
- `agents.json`: 20 named worker profiles.
- `fleet.md`: ownership, scheduling, and execution-record rules.
- `run.sh`: UTF-8/LF Bash launcher.
- `README.md`: this guide.

Official references: [CLI flags](https://code.claude.com/docs/en/cli-reference), [custom subagents](https://code.claude.com/docs/en/sub-agents), and [environment variables](https://code.claude.com/docs/en/env-vars).
