# Fixtures Are Testing Examples Only

Goal: fixtures are inert examples used exclusively for tests. Runtime assets, policy data, source implementations, schema facets and case definitions belong to their semantic owners. Do not preserve legacy names or aliases.

## Work Fleet

Coordinator handles canonicalization, integration and validation. Two GPT 6.1 Sol Low read-only exploration agents identify schema and runtime violations; GPT 6.1 Sol High implements enforcement. Execution lanes follow the audits. Maximum concurrency is four including coordinator. No fast service tier is selected.

## Plan

1. Inventory fixture schema facets, their consumers and live runtime dependencies.
2. Replace duplicate fixture wrapper schema validation with direct test expectations or actual domain-schema validation. Extract synthetic implementation/schema support into test support owners.
3. Extract actual runtime domain policy/source ownership and remove live fixture dependencies.
4. Enforce fixture/schema boundaries through language-neutral vectors and independent oracles.
5. Run targeted Nx tests and runtime/build checks, audit all remaining violations, repeat until closure.

## Ticket Infrastructure

Read repo://goals through local repository MCP. Reopening Assets Fixtures Separation with a changed title renamed its folder before reporting missing old path; current canonical ticket path is 26/05/30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY. MCP model allowlist does not contain gpt-6.1-sol; recorded codex identity instead of mislabeling another model. A duplicate current-date ticket was briefly created while diagnosing the rename; close it as duplicate.

## Validation

No passing claim until command execution. Generated output belongs in ticket 🗑️generated; preserve input scripts and markdown. No modifying git/worktrees, no AGENTS.md edits.
