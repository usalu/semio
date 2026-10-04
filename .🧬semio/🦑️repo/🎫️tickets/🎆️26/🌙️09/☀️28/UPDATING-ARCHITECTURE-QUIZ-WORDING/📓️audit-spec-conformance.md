# 🔍️ Spec Conformance Audit — Quiz Cores, Wire Contract, Proctor, Client

Five read-only Sonnet sub-audits against `📓️design.md` and `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json`,
compiled by the coordinator. Every finding lists its resolution.

## 1. Randomness, sheet, scoring, badges (TS ↔ Rust ↔ design §3–§7)

No divergence on valid input: FNV-1a seed, MT19937, rejection-sampled uniform, Fisher–Yates, sheet RNG consumption
order and rotation rule, pair-concordance summation order, half penalty for matching ties, classification `d_max`,
badge semantics (catalog order, held skip, empty selector never awards) are equivalent.

| Finding | Severity | Resolution |
|---|---|---|
| TS `mean([])` → NaN, Rust → 0 | medium (unreachable after validation) | TS now 0 (`📓️core-ts-report.md`, follow-up) |
| Incomplete category profile poisons the TS task's `d_max` with NaN; Rust excludes the category | medium (unreachable after validation) | TS excludes it like Rust |
| Matching item missing a dimension value: TS `undefined` card, Rust NaN | low | TS NaN card |
| TS `scoreTask`/`scoreRun` throw, Rust returns `None` | low | TS returns `undefined`; `submitRun` maps it to `run-incomplete` |

## 2. Validation, lifecycle, views, schema twins

No issues: answer validity/completeness, handle normalisation, roster recall, stale-revision voiding/rejection,
latest-wins answers, badge awarding, leaderboard ordering (`reachedAt` updated only on a strictly greater best), tagged
unions and optional fields match the schema.

## 3. Wire contract (proctor ↔ React client, design §9a)

No mismatch: command/query kinds `quiz.*`, targets (`quiz-roster/roster`, `quiz-learner/<learner>`), version 1,
`commandId = idempotencyKey = Command.id`, UTF-8 JSON payloads and view snapshots, rejection detail tokens. The gateway
resolves every request to `Principal::Anonymous` (no resolvers, by design: identity without passwords).

**Consequence found by the coordinator:** a learner id is the only credential of an anonymous learner, and the public
leaderboard published it. Resolution: `LeaderboardRow.learner` → `tag` = `fnv1a32(learner)` as 8 lowercase hex digits
(schema, both cores, vectors, proctor e2e asserting no id in the served board, client own-row highlight by tag; design
§13).

## 4. Proctor internals and the framework rehydration fix

Confirmed: atomic projection commits with checkpoint, idempotent folding across restarts, WAL + `synchronous=FULL`,
append-only event/receipt/outbox tables, traversal-safe static hosting with canonicalisation, production gating as a
true AND, exact-match CORS allowlist. The shared `CommandBus` rehydration (`🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs`)
closes a pre-existing availability bug for every instance after a restart (hub included) and has a framework unit test.

| Finding | Severity | Resolution |
|---|---|---|
| Non-GET/HEAD on static routes → 404 | low | 405 with `Allow: GET, HEAD` |
| SPA fallback 404s a route whose last segment contains a dot | low | kept: the client has no dotted routes; documented |
| `X-Forwarded-Proto` trust relies on the proxy stripping client headers | informational | documented in the proctor README / Caddyfile |

## 5. Client local-first behaviour

| Finding | Severity | Resolution |
|---|---|---|
| Two tabs overwrite each other's persisted slices (lost answers) | critical | per-record storage keys + `storage`-event merge; two-tab test |
| Cancelled submission may race a server commit and leave the run "open" | high | always reconcile the run view after cancel |
| Delivered local answers override newer proctor answers; `open()` skipped refresh for cached runs | medium | only undelivered answers override; `open()` refreshes |
| Backoff window treats the retried head as coalescable | low | harmless (latest-wins); documented |
