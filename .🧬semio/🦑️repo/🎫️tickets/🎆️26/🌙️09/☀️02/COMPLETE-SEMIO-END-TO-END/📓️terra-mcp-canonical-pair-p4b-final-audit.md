# MCP Canonical Pair P4-B Final Audit

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Verdict

**Accepted within P4-B's bounded, materialized-response scope.** The initial
independent invocation was red before its assertions because of concurrent
plugin-host compile breakage, but the retained isolated target was rerun after
that repair and independently passed every registered reader, route, neutral
oracle, and all-feature check below.

## Final-source audit

The source implements the intended bounded, server-selected active-pair
boundary:

- `VerifiedActiveCheckpointPairReader` selects the exact active public/private
  checkpoint projection, checks descriptor digest and scope equality, preflights
  nonzero checked pair lengths/record count before CAS reads, verifies both part
  hashes/lengths and aggregate, and does not fall back to a changed active
  checkpoint (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:304-390`).
- The binary receiver requires the canonical big-endian header, pack-before-SPR
  ordered contiguous records, an exact Complete terminal, no trailing bytes,
  and both per-part/aggregate digests (`…/🛰️lag-rebootstrap/🦀️.rs:461-654`).
  The TypeScript/AJV/Node oracle reconstructs the framing and independently
  recomputes hashes and the domain-separated ETag
  (`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:50-121`).
- The route allows only the exact path with no query, no `Range`, exactly one
  exact media `Accept`, and exactly one bounded `Authorization: Bearer` whose
  typed capability is Session or Share
  (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1426-1458`). It admits only a current
  member session or exact-document share—never public fallback—and rechecks
  authorization after metadata, before every record, and before terminal
  (`:1059-1069,1624-1680`).
- Request ownership uses an atomic cancellation/active control and bounded
  stage slots. The raw-TCP disconnect, deadline, and progress law is present
  (`:1475-1576,4607-4687`). Responses are deliberately materialized before
  publication, so revocation yields an empty error rather than an accepted
  partial frame. This is a bounded materialization design, **not** a claim of
  incremental HTTP streaming after headers/body publication.
- Registration is correct in `🌎️hub/📦️packages/🦀️rust/📋️project.json:79-85`,
  `📜️script.ts:1008-1014`, and both launch files at gate order `411.11`.

No source blocker was found in reader selection, framing, route admission,
authorization, request race handling, oracle independence, or registration.

## Independent gate evidence

The normal uncached invocation first waited on the shared Cargo build lock and
was cancelled before it began (exit 130), so it is not test evidence. The
independent isolated invocation was:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/canonical-pair-audit-target \
RUSTC_WRAPPER='' CARGO_BUILD_RUSTC_WRAPPER='' \
bun nx run os-hub:canonical-pair-check --skip-nx-cache
```

It ran cold from the current shared source and exited **1**. Its first command,
`cargo test --manifest-path Cargo.toml --lib canonical_pair -- --test-threads=1`,
did not reach canonical tests because `semio-framework-plugin-host` failed to
compile with unrelated concurrent errors:

- E0308: `turn_fault_message` receives `&PluginHostError` instead of
  `&TurnFault` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1113,1260`.
- E0499: overlapping mutable `self` borrows at `:1760,1765`.
- E0609: missing `close_requested` on `GuestRelayMountedSession` in
  `…/🔌️plugin/🖥️host/🦀️.rs:3871,3886,3962,3971,3987,4013,4020`.
- E0599: `GuestColdRelayJob::terminal_is_empty` trait method not in scope at
  `…/🔌️plugin/🖥️host/🦀️.rs:3700`.

Consequently this independent run produced no reader/route/oracle assertion
result and no all-feature hub check result. The ticket-local Cargo target is
historical only: after the final rerun below, concurrent external cleanup
removed `🗑️generated/canonical-pair-audit-target`. That removal did not alter
the recorded terminal session `63310` evidence and is not represented as a new
test run.

## Initial-Red Acceptance Hold (Historical)

Before the shared plugin-host compile repair, the initial-red invocation required
the exact registered uncached rerun recorded below before P4-B could be
accepted. That historical condition is now satisfied. The separate
SocketGrant/MCP carrier, cache/mount/catalog, and tail-barrier work remains
outside this P4-B server snapshot verdict.

## Final independent rerun — accepted

After the shared plugin-host repair, this audit reran the exact registered gate
from the retained ticket-local target with wrappers disabled:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/canonical-pair-audit-target \
RUSTC_WRAPPER='' CARGO_BUILD_RUSTC_WRAPPER='' \
bun nx run os-hub:canonical-pair-check --skip-nx-cache
```

Terminal session `63310` exited **0**. Its actual assertions were:

- Rust verified-reader/framing: **3 passed, 0 failed** — cancellation without
  terminal acceptance, neutral pack-then-SPR framing, and preflight-before-
  allocation/record bound.
- Raw-TCP route/lifecycle: **3 passed, 0 failed** — exact member/share route
  admission, non-path/ambiguous-header rejection before work, and request-owned
  disconnect/deadline/progress cleanup.
- Independent TypeScript/AJV/Node neutral oracle: **1 passed, 0 failed**.
- Final all-feature `semio-hub` `cargo check`: **exit 0**.

The terminal Nx output included its advisory `Nx detected a flaky task
os-hub:canonical-pair-check`, but no assertion in this rerun failed. It follows
the earlier compile-blocked invocation recorded above and is not treated as a
test failure or silently discarded. This acceptance remains limited to the
source and checks enumerated here: bounded materialization, not post-publication
incremental streaming; no claim for the separate SocketGrant/MCP carrier,
cache/mount/catalog activation, tail cursor, or backend runtime topologies not
exercised by this packet.
