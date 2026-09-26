# WP-R9 — AGENTS.md Compliance of the Runtime Paths

Slice: R9 (session 13). Source: `📓️audit-s13-rules.md`. Ports 8170–8179 / 6670–6679. Private cargo: `.tmp-ticket/wp-r9/target`.
Durable captures: `.🧬semio/🌐hub/s13-r9-captures/`. Expendable captures: `.tmp-ticket/wp-r9/generated/`.

## Session 13

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | launch.json registration: executable-command rule, generator + law | in progress | — |
| 2 | `[DEBUG]`-tagged permanent status lines (hub `📜️script.ts`) | applied (hub + MCP trees); compiling | §Item 2 |
| 3 | CRUD `DELETE` hub auth routes → commands (hub + all clients) | edited (H11 go 19:4x); compiling | §Item 3 |
| 4 | `🔌️plugin/🦀️.rs` fallback / weak-linkage shim | applied (forbidden: implicit 2nd init path, removed); compiling | §Item 4 |
| 5 | `@emoji` docstring tag investigation (report only) | **DONE** — residue, not a convention; codemod + law recommended | §Item 5 |
| 7 | (coordinator 21:0x) T13 repo-contract HIGHs: ui `event-feed-time-format` inline test, os schema-oracle JSON in `🧪️tests` | moved to canonical layout; compiling | §Item 7 |
| 6 | comments inside definitions: parser count + clean hub/MCP; codemod plan for guest-linked | hub + MCP cleaned (tree-sitter census 0 left); compiling; plan for guest-linked pending | §Item 6 |

### Session 13 log

- 19:36 slice start; read AGENTS.md, preambles 13/12, `audit-s13-rules.md`, `wp-r8.md` (launch resolver probe P1-6).
- 19:4x H11 answered "go now" for item 3 (not editing those handlers; asked to move its H9 revocation-fence laws and the
  two-client e2e too, fence byte-identical). V1 asked which launch mechanism to use → told: seed row + identical rendered row for
  now; R9 converts every seed row itself if the source of truth moves.
- 20:0x item 3 edits (see §Item 3); 20:15 `cargo check -p semio-hub --lib --bins --tests` started (queued behind a peer's hub check).
- 20:2x item 5 measured (see §Item 5).
- 20:3x rule 25: my `cargo check -p semio-hub` waited 28 min with no rustc child (convoy) → stopped (pid 62168).
- 20:4x TS side of item 3 measured: relay admission law **4/4** (`bun test`), `hub-sign-in-spaces-check` **EXIT 0 — oracle 15 checks
  clean (incl. the new sign-out-path pin), 3 files / 98 tests pass** (`generated/hub-sign-in-spaces-check-1.txt`).
- 20:53 item 4 applied (`wp-r9/plugin-link-shim-removal.py`, dry run clean first). 20:54 combined native check launched on the
  shared build-dir → convoy (8 min, no rustc child) → stopped at 21:03 per rule 25/26; relaunched on `build-fleet-b`
  (pid 5396, capture `.🧬semio/🌐hub/s13-r9-captures/native-check-2.txt`).
- 21:0x item 2 applied (`wp-r9/debug-residue-removal.py`): 56 test prints deleted in 15 hub files, 26 hub `📜️script.ts` tags dropped,
  the `native-catalog-payload=` channel renamed on both ends. Item 7 (coordinator) applied. Item 6: tree-sitter census + codemod
  (`wp-r9/comment-census.ts`, `wp-r9/comment-hoist.ts`) applied to hub + MCP: 221 blocks.

## Item 3 — hub auth `DELETE` routes → commands

Design: queries stay `GET`, every state change is a `POST` command on the resource (the hub's existing `…/cancel`,
`…/redeem`, `…/approval` shape). The two `DELETE` routes become:

| was | now | handler |
|---|---|---|
| `DELETE /auth/sessions/me` | `POST /auth/sessions/me/sign-out` (empty body, `DefaultBodyLimit::max(0)`, non-empty → 400) | `post_session_sign_out` |
| `DELETE /auth/agent-delegations/{id}` | `POST /auth/agent-delegations/{id}/revoke` (empty body, non-empty → `malformed-request`) | `post_agent_delegation_revoke` |

Both still append their facts (`session-revoked`, `agent-delegation-revoked`) exactly as before; the delegation fence (hold every
revoked session's binding exclusively before invalidating grants/answering) is byte-identical (only a body check was added in
front). No hub route answers `DELETE` any more (a law asserts `DELETE /auth/sessions/me` → 405), CORS `allow-methods` drops
`DELETE`, the `HttpMethod::Delete` variant is gone from the kernel directory client (queries `GET`, commands `POST`).

Files: hub `🔐️auth/🦀️.rs` (`SESSION_SIGN_OUT_ROUTE`), `🔐️auth/🤖️agent/🦀️.rs` (`AGENT_DELEGATION_REVOKE_ROUTE`, module doc),
`🔐️auth/🧬️schema/🟦️.ts` (`AUTH_SESSION_SIGN_OUT_ROUTE`), `🏗️bootstrap/🦀️.rs` (handlers, route table, rate-limit class, CORS),
`🧪️tests/🔬️bin-unit/🦀️.rs` (13 direct calls + 7 raw HTTP calls incl. H9's three revocation-fence laws, +405 law),
`🧫️fixtures/🚧️hostile-input-v1/🔣️.json` (2 rows, empty-body vectors), `📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json`
(self-revoke step + route inventory), `📦️packages/🦀️rust/📜️script.ts` (4 probes, journey method taxonomy, relay body read),
`🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` (+405 check), `🧪️tests/🤝️two-client-document/🟦️.ts`, `🚀️local-relay/🧭️routing/🟦️.ts`
+ its admission law. Clients: kernel `📇️directory/🔌️client/🦀️.rs` (`sign_out` → POST command; native wgpu shell uses it),
wgpu `📇️directory-door/🦀️.rs` + its law, wgpu `🔐️HubSignIn` (`HUB_SESSION_SIGN_OUT_PATH_V1` + fixture law), React
`🔗️HubConnection/🟦️.tsx` (sign-out + revoke), `📇️directory/🔐️sign-in/🟦️.ts` (`HUB_SESSION_SIGN_OUT_PATH_V1`, re-exported from
`💻️os/🟦️.ts`), sign-in contract fixture + schema (`signOutPath`, pinned by the TS law, the wgpu law and the hub-auth oracle in the
React `📜️script.ts`), `📇️directory/🤖️delegations/🟦️.ts` (`…/revoke`) + AgentDelegations law, MCP laws
`🤖️hub-agent-participant`, `🤝️hub-edit-durability`. The MCP Rust side has no revoke/sign-out call (verified by grep).

## Item 5 — the `@emoji` docstring tag

Verdict: **residue of an authoring style, not a machine-readable convention.** Recommend a codemod (strip the tag) + a law.

Evidence (all measured):
- **No consumer.** Every occurrence of `@emoji` in the repo's code (224 057 source files scanned, 9 188 hits,
  `generated/at-emoji-all-1.txt`) is inside a comment/docstring except (a) three generators that EMIT it into generated files
  (`🖱️ui/🎨️styling/📽️projection/🟦️.ts` ×2, `📇️registry/📽️projection/🟦️.ts` ×3, wgpu `🏗️builder/🦀️.rs`) and (b) two
  `indexOf("/** @emoji 🛂️ …")` / `indexOf("/// @emoji 🧬️ …")` text anchors in `🌎️hub/…/📜️script.ts:13126` and
  `💻️os/🖥️host/…/📜️script.ts:1024`. No parser, taxonomy law, docs extractor or uniqueness check reads the tag; nothing checks
  docstring emojis at all. AGENTS.md (then and now) says "start with a unique emoji" — never `@emoji`.
- **Not introduced by the 09-02 rename.** 21fbcd3538f only moved files: its parent already had the same lines (e.g.
  `🏪️store/🦀️component.rs:1761`). Per-revision history of that file: `@emoji` present from its creation (2026-08-06, 183 → 377).
  Tree-wide bisect (first-parent, `.rs/.ts/.tsx`): 0 at 7e243651ba (≤ 2026-05-01) → **287 at 17c93d6fe6 ("2026-04-16", kit
  JS/React `semio/js/index.ts`)**, 5 870 by mid-June, 8 901 by August — an agent JSDoc habit that spread by copy.
- **Minority, mixed style.** Docstring openings today (`at-emoji-census.py`, first line of each `///` block / `/** */`):
  Rust 71 336 bare emoji vs **4 093 `@emoji`** (1 002 other), TS 18 462 bare vs **4 664 `@emoji`** (2 909 other) → 8.9 % of
  emoji docstrings; 741 files carry it, 385 of them mix both styles. Top: `🏪️store/🦀️.rs` 411 (+279 bare), ui React 403,
  spatial-kernel geometry 218, `🛂️manifest/🦀️.rs` 195, `🔌️plugin/🦀️.rs` 177 (+1 361 bare), hub bootstrap 161 (+90 bare).
- **It breaks the language-native docstring in TypeScript.** TypeScript's own compiler (5.9.3, `at-emoji-jsdoc-probe.ts`)
  parses `/** @emoji 🧹️ Tagged summary. */` as documentation `""` + block tag `emoji` = "🧹️ Tagged summary." — IDE hover,
  quick info and TypeDoc show NO summary for 4 664 TS definitions. `/** 🧹️ Plain summary. */` → documentation "🧹️ Plain summary.".
  Rustdoc renders the literal "@emoji" as the first word of the summary line.

Recommendation (next cycle, after PUBLISH DONE — it touches guest-linked crates, 741 files): one codemod in a ticket folder that
rewrites only a docstring's first token `@emoji ` (Rust `///`/`//!`, TS `/** @emoji`, `* @emoji` first content line, the
generators' templates and the two `indexOf` anchors — anchors first, or those laws break), dry-run diff reviewed per file
(memory: codemod incidents on emoji-heavy files), compile-atomic per crate; plus a repo-lib law "a docstring's first token is an
emoji, never `@`" (and, optionally, per-file uniqueness) so it cannot regrow.
