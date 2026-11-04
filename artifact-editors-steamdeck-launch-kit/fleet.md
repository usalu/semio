# Artifact editor fleet

Main coordinator: **Claude Opus 5.5 / medium**, additional to **20 worker subagents**. The profiles below are defined in `agents.json`; the coordinator must dispatch them as actual background tasks.

| # | Named profile | Model / effort | Ownership |
| --- | --- | --- | --- |
| 1 | `sonnet-registry` | Sonnet 5.5 / medium | Canonical artifact registry and editor resolution |
| 2 | `sonnet-host` | Sonnet 5.5 / medium | Shared editor host and application integration |
| 3 | `sonnet-persistence` | Sonnet 5.5 / medium | Document models, saving, history, and data integrity |
| 4 | `sonnet-formats` | Sonnet 5.5 / medium | Shared import/export and round-trip compatibility |
| 5 | `sonnet-runtime` | Sonnet 5.5 / medium | Linux runtime, services, dependencies, and workers |
| 6 | `sonnet-validation` | Sonnet 5.5 / medium | Per-editor end-to-end verification and integration evidence |
| 7 | `sonnet-editors-a` | Sonnet 5.5 / medium | Editor implementation batch A |
| 8 | `sonnet-editors-b` | Sonnet 5.5 / medium | Editor implementation batch B |
| 9 | `sonnet-editors-c` | Sonnet 5.5 / medium | Editor implementation batch C |
| 10 | `sonnet-editors-d` | Sonnet 5.5 / medium | Editor implementation batch D |
| 11 | `sonnet-editors-e` | Sonnet 5.5 / medium | Editor implementation batch E |
| 12 | `sonnet-editors-f` | Sonnet 5.5 / medium | Editor implementation batch F |
| 13 | `sonnet-editors-g` | Sonnet 5.5 / medium | Editor implementation batch G |
| 14 | `sonnet-editors-h` | Sonnet 5.5 / medium | Editor implementation batch H |
| 15 | `haiku-inventory` | Haiku 5.5 / medium | Read-only canonical editor inventory and coverage audit (read-only) |
| 16 | `haiku-capabilities` | Haiku 5.5 / medium | Read-only capability and behavior-contract audit (read-only) |
| 17 | `haiku-data-audit` | Haiku 5.5 / medium | Read-only persistence, format, and data-loss audit (read-only) |
| 18 | `haiku-entry-audit` | Haiku 5.5 / medium | Read-only application entry-point and completion audit (read-only) |
| 19 | `haiku-registration` | Haiku 5.5 / medium | Bounded registry, metadata, fixture-reference, and manifest updates |
| 20 | `haiku-documentation` | Haiku 5.5 / medium | Verified coverage reports, commands, and handoff documentation |

## Coordination rules

1. Dispatch all 20 named profiles in the initial parallel wave. Give each worker the relevant acceptance criteria, exact assignment or discovery target, logical ownership, dependencies, and completion evidence. Ordinary subagents do not automatically inherit the coordinator's conversation.
2. Establish the repository-derived editor inventory and shared registry/model/host/storage contracts early. Allocate batches A-H from real editor types; do not invent eight categories. A batch can contain several editors. If fewer editors exist, assign independent subsystems or separate verification work.
3. The registry, host, persistence, formats, runtime, and validation workers own their respective shared integration points. Batch workers own editor-specific changes. Coordinate narrow edits when several owners touch one file.
4. Refill finished slots with meaningful work while the task remains active. Keep a coordinator-owned task board with real task IDs and observed concurrency. If runtime/account restrictions limit the fleet, report the constraint and continue at the permitted capacity.
5. Preserve other agents' work, reread files before editing, reconcile overlapping sections, and continue independent tasks. Never reset unrelated changes or treat a dirty tree as a reason to halt.
6. Keep all 20 model workers useful while bounding local process load. The validation owner schedules one heavy build/test/render/export job or one shared dev-server/browser verification session at a time initially. The runtime owner coordinates dependency installation and shared services. Other workers continue analysis, edits, audits, and preparing focused checks.
7. Every worker has `background: true`, a full model ID, `effort: medium`, and `disallowedTools: ["Agent"]`. Instructions also prohibit further spawning via shell or separate Claude launches. The four readers have only Read, Glob, Grep, WebSearch, and WebFetch. Helpers make bounded, agreed changes; complex implementation belongs to Sonnet.
8. Opus integrates the result, resolves conflicting contracts, and accepts only evidence-backed completion for each editor.

The concurrency environment variable sets a cap; it does not launch the workers or guarantee that all 20 can run under the current account/runtime constraints. The prompt requires actual dispatch and honest reporting.

Reference: [Claude Code subagents](https://code.claude.com/docs/en/sub-agents).
