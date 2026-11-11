# Morph and preview worker fleet

Coordinator: **Opus 5.5 / medium**, additional to the worker profiles below. The kit configures capacity for **20 worker subagents**; use the greatest useful parallelism supported by dependencies and available resources.

| # | Named profile | Model / effort | Responsibility |
| --- | --- | --- | --- |
| 1 | `sonnet-schema` | Sonnet 5.5 / medium | Declarative morph schema and shared contracts |
| 2 | `sonnet-timeline` | Sonnet 5.5 / medium | Canonical timeline and terminal-frame semantics |
| 3 | `sonnet-final-morph` | Sonnet 5.5 / medium | Full-quality final morph execution |
| 4 | `sonnet-preview` | Sonnet 5.5 / medium | Cheap preview translation, scaling, and target swap |
| 5 | `sonnet-staffage` | Sonnet 5.5 / medium | Canonical staffage references, poses, views, and LoD |
| 6 | `sonnet-profiles` | Sonnet 5.5 / medium | Render profiles, execution planning, and caches |
| 7 | `sonnet-playback` | Sonnet 5.5 / medium | Player, seeking, reverse, and live preview integration |
| 8 | `sonnet-authoring` | Sonnet 5.5 / medium | Morph authoring, settings, and persisted definitions |
| 9 | `sonnet-clips-a` | Sonnet 5.5 / medium | First disjoint clip migration batch |
| 10 | `sonnet-clips-b` | Sonnet 5.5 / medium | Second disjoint clip migration batch |
| 11 | `sonnet-exports` | Sonnet 5.5 / medium | Final exports, thumbnails, and generated media |
| 12 | `sonnet-lifecycle` | Sonnet 5.5 / medium | Cancellation, resource ownership, and leak repair |
| 13 | `sonnet-validation` | Sonnet 5.5 / medium | Benchmarks, regressions, and integrated verification |
| 14 | `sonnet-foundations` | Sonnet 5.5 / medium | Runtime and encountered fundamental defects |
| 15 | `haiku-inventory` | Haiku 5.5 / medium | Read-only clip, morph, and consumer inventory (read-only) |
| 16 | `haiku-timeline-audit` | Haiku 5.5 / medium | Read-only endpoint and deterministic-state audit (read-only) |
| 17 | `haiku-resource-audit` | Haiku 5.5 / medium | Read-only hidden-work, cache, and lifecycle audit (read-only) |
| 18 | `haiku-teaching-audit` | Haiku 5.5 / medium | Read-only teaching intent and final-media audit (read-only) |
| 19 | `haiku-registrations` | Haiku 5.5 / medium | Bounded declarative registrations and fixture updates |
| 20 | `haiku-documentation` | Haiku 5.5 / medium | Verified coverage, benchmark reports, and handoff docs |

## Dispatch and ownership

1. Start independent discovery, canonical-asset, lifecycle, profiling, and test analysis promptly. Settle shared schema, endpoint, profile, and ownership invariants early; scale parallel implementation as those dependencies clear.
2. Dispatch named profiles through actual background Agent calls, including task context, paths, ownership, dependencies, and required evidence. Workers do not automatically share the coordinator's conversation.
3. Use up to the configured 20-worker cap for real useful assignments. Reassign substantive remaining work and retire finished workers when no useful assignment remains. Do not maintain filler work to occupy capacity.
4. Keep a coordinator-owned task board with confirmed task IDs, assignments, dependency/status changes, and results. Separate available profiles, dispatched tasks, unfinished work, and observed runtime overlap. State when actual concurrency or runtime model/effort cannot be observed.
5. Shared owners integrate their contracts; two clip workers take disjoint batches from the actual inventory. Coordinate logical sections when files overlap. Preserve other agents' work, reread before edits, and reconcile conflicts without resets or blanket overwrites.
6. The validation owner coordinates the heavy-job queue; the foundations owner coordinates shared runtime/dependency repairs. Start with one heavy job or shared dev-server/browser session on the Steam Deck; increase with measured headroom and independent outputs. Reuse healthy services and keep competing loads out of benchmark comparisons. This is a workflow policy, not a process limit enforced by run.sh.
7. All profiles specify full model IDs, medium effort, background execution, and denial of the Agent tool. Instructions also prohibit shell-based spawning and separate Claude sessions. Four Haiku readers have only Read, Glob, Grep, WebSearch, and WebFetch. The two helpers make bounded agreed changes; complex implementation stays with Sonnet.
8. Confirmed fundamental defects, including unrelated ones encountered during this task, receive explicit owners, root-cause repairs, and regression evidence. Opus prioritizes and integrates these alongside the main result.

The environment variable controls the session's subagent cap; it does not dispatch workers, prove simultaneous execution, or define the account's maximum. Report actual capacity constraints and useful completed work honestly.

Reference: [Claude Code subagents](https://code.claude.com/docs/en/sub-agents).
