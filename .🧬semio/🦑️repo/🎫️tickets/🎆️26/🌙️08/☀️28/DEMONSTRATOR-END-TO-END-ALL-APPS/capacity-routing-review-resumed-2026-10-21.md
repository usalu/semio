# Capacity Routing Review — Resumed 2026-10-21

Read-only source review. No builds, tests, process control, policy changes, or receipt edits were performed.

## Concrete finding

The initial reviewed `runBuildCliV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` uses `parseBuildInvocationV1(...).args` to recognize its early capacity/leases route. That parser strips tool/workers/graph-policy but retains the public global `--lang` flag. Consequently `repo --lang=en capacity status`, `repo --lang de leases status`, and corresponding prefixed help requests leave a language option at diagnostic[0] and miss the graphless early branch. The neutral birth fixture presently exercises tool prefixes only, so its new regression does not cover this public spelling. Sent immediately to acceptance_hardening and parent; fix should normalize diagnostic selection only, retaining the full normalized original args for ordinary builds.

## Confirmed boundaries

- `⚙️operations/🟦️.ts` derives neutral bootstrap classification directly from the operation catalogue: requires none AND both implementation entries neutral. Unknown names fail closed to dependency-requiring behavior. The birth test independently queries the catalogue with SQLite and validates its neutral fixture with Ajv. No hardcoded capacity-only exemption.
- `📥️caller/📤️child/🟦️.ts` omits Nx bootstrap launches for classified neutral operations, while preserving existing special tool/reset/watch behavior and closed capability validation.
- `normalizeBuildOptionsV1` stops at the first `--`; `parseBuildInvocationV1` and graph policy extraction preserve the tail. New dispatch passes diagnostic arguments only to the matching control operation and passes original normalized args through admission for every other operation. Thus tool/workers options remain available to downstream ordinary build resolution.
- Capacity run still splits its own header/tail at the first separator, sizes a request, enters `withHostCapacityV1`, and only then spawns its own child under the grant. Its signal forwarding targets that owned child. Status uses the existing sampler owner. No new arbitrary release, kill, cancellation, or foreign receipt mutation operation was added.
- Lease ingress still delegates to the existing status-only owner. Its catalogue and command module expose no new retirement, release, or kill command. Existing stale-entry reclamation remains the owner implementation, not newly privileged CLI authority.

## Validation limits

Source-only review; the already-owned native birth/mode checks are the execution authorities. No duplicate checks were queued. Report will receive a follow-up if the language-prefix fix is saved during this audit.

## Public Cargo Test Entry Completion Review

The saved public entry at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🧪️testing/📦️executable/🧪️testing/🌿️/🟦️.ts` now imports the existing owner `🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️runtime-selection/🟦️.ts`. Filesystem path resolution confirms the seven parent traversals land exactly at that owner. The package export `./cargo/testing/executable/testing` still maps to the public entry; no API rename is introduced.

All six relative dependencies and source reads in the split runtime-selection test resolve to existing files, including its neutral fixture, schema, implementation, dependency schema, and dependency implementation. The owner contains actual Bun test registrations using shared vectors and SQL/Ajv oracles; the source test parent also imports this split owner. Both public entry and target have absolute path lengths164 JavaScript string units, below256. No concrete repair defect found by source review. Native execution remains owned by parent.

## Saved Global Language Fix Completion Review

The earlier concrete --lang prefix finding is resolved in current source. `buildOperationArgumentsV1` derives consumed required global flags from the neutral catalogue, normalizes separate values, and preserves the separator and complete opaque tail. `parseBuildInvocationV1` applies this projection only after existing tool, worker, and graph-policy validation. `runBuildCliV1` passes the already resolved language into a diagnostic-only environment for control commands and help; ordinary admitted operations still receive their full normalized original arguments.

Crosschecked all13 current diagnostic fixture rows against source and the schema: separate English language before capacity; German language with tool, workers and graph-policy before leases; German capacity help; ordinary build retaining language/workers; and an opaque child whose German language remains untouched. The regression now uses the real language resolver and asserts forwarded language as well as route/args. The optional fixture language is restricted to en/de by schema. No concrete new selector, tail, language, or mutation-authority defect found. This is source/fixture review only; native tests already owned by other agents remain pending execution authorities.
