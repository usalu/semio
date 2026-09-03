# P4-C Canonical Pair Cache/Mount Current Audit

## Current source verdict

**ACCEPTED for the bounded P4-C cache/mount scope.** This report is the durable no-variation-selector ticket copy. A prior variation-selector report is intentionally preserved.

`finish_mount_return` takes `pair_actor` and keeps it through the final actor-identity and binding/authority-generation test (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs:644-678`). `begin_refresh`, `invalidate`, and `revoke` take that same lock before changing generation/authority and invalidating the actor (`🏠️workspace/🔗️remote/🦀️.rs:244-280`). Therefore a return holding the actor lock linearizes before an invalidation, while an invalidation which wins first is detected by the final fence. The deterministic cache-hit/revoke law pauses after lookup and before that shared fence, then requires `StaleCompletion` and `Revoked` state (`pair/🦀️.rs:1321-1344`). This closes the former cache-return race.

The registered P4-C check lists each suffix, requires exactly one fully-qualified Cargo subject, exact-runs all four subjects (including that race law), then runs the independent Node parser oracle and all-feature MCP/hub checks (`🌉️mcp/📦️packages/🦀️rust/📜️script.ts:49-76`). This closes the former selector-vacuity concern.

The Nx subject is explicitly noncached and invokes only the registered script (`🌉️mcp/📦️packages/🦀️rust/📋️project.json:52-58`); the matching seed and generated launch entries use the same uncached project target at gate order 411.12 (`.vscode/🧩️launch.seed.jsonc:3095-3103`; `.vscode/launch.json:4443-4451`).

The Node oracle is structurally independent for the fixture frame reader: it uses its own frame parser, fatal `TextDecoder`, Node SHA-256/WebCrypto, AJV, and mutations (`pair/🧪️oracle/🟦️.ts:1-133`). It remains a corpus/parser oracle, not a real protected-pool cancellation or race execution. The explicit header ceiling is still indirect in this oracle: its total-wire calculation includes `headerBytes`, while it does not separately compare first-frame length to that limit (`:51-56`). Current fixed field grammar prevents an oversized structurally valid header, but the direct bound should be retained in the independent oracle if the framing grammar expands.

No P4-C uncached registered gate terminal is claimed by this audit after the source repair.

## Cache-hit deadlock repair re-read

The prior cache-hit law could deadlock before its shared-lock fence: the `if let` scrutinee held
the temporary `pair_actor` mutex guard while evaluating the body, and `finish_mount_return` tried
to take that same non-reentrant lock. Current source first materializes the hit in a scoped
binding, then enters the `if let` only after the guard is dropped
(`🏠️workspace/🔗️remote/🧩️pair/🦀️.rs:527-532`). This is a genuine lifetime fix rather than a
timeout, larger stack, or skipped race.

The revocation linearization remains sound. The repaired cache hit releases the actor lock before
`finish_mount_return`; that function later reacquires the same lock and checks mounted identity
plus binding/authority generations (`:645-678`). `revoke`, `invalidate`, and `begin_refresh` take
that lock before clearing the actor and changing either generation (`🏠️workspace/🔗️remote/🦀️.rs:244-280`).
Thus a revoke that wins the shared lock is observed as `StaleCompletion`; a return that wins it
linearizes before revoke. The deterministic law pauses exactly between cache lookup and the final
fence, revokes, releases, and requires `StaleCompletion` (`pair/🦀️.rs:1322-1345`).

The current registered script still exact-selects the same four current source suffixes, requires
one `: test` match for each, and exact-runs all four (`🌉️mcp/📦️packages/🦀️rust/📜️script.ts:87-106`).
Session 12342 is implementation-run evidence only until independently terminal; no Cargo command
was run by this audit.

## Coordinator-observed registered terminal

After the deadlock repair above, coordinator session **12342** terminated with exit `0` on the
current source. Per the coordinator's terminal report, the registered uncached script completed
all four exact-selected Rust laws, the independent oracle's 3 positive and 15 negative vectors,
the MCP checks, and the hub all-feature check. This is coordinator-observed runtime evidence, not
a Cargo command run by this audit. Together with the independent source re-read, it accepts the
bounded canonical-pair receiver/cache/mount/revocation scope only; it does not certify unrelated
MCP workspace, catalog-currentness, or end-to-end document-open behavior.
