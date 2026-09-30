# Final Shared Contracts Audit

Read-only Light audit of the actual shared policy, plugin deployment contracts, neutral reconcile envelopes, local-session broker, and provider/owner handoff. The existing ticket implementation notes were read. No source edits, git mutations, ticket transitions, broad suites, or runtime tests were performed. Findings below are source-established and are not passing-test claims.

## Actionable Findings

### P2: Reject Explicit Null Space Kinds in the Rust Policy Parser

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🛡️access-policy/🧬️schema/🦀️.rs:85-86,103-109`

`#[serde(default)] Option<Vec<String>>` accepts both an absent field and `"spaceKinds": null` as `None`. The subsequent validation only inspects `Some`, so a policy such as `{"schema":"semio.os.directory.access-policy/v1","grants":[{"effect":"allow","roles":["authenticated"],"actions":["document.read"],"spaceKinds":null}]}` is accepted as an unrestricted grant by Rust. JSON Schema requires an array when present, and the TypeScript parser sends every present field through the nonempty-array validator. Distinguish absent from explicit null during native deserialization; add this refusal to the common language-neutral fixture and all three validators.

### P2: Use the Broker Profile Grammar for Provider Defaults

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🟦️.ts:17`

The provider schema/parser permits defaults such as `human-` or `human.`. The directory broker schema/parser requires a final alphanumeric character. `ensureDevLocalHub` chooses that accepted default at `🏃️execution/🟦️.ts:438` and passes it to the broker at line 443, so a contribution can validate successfully yet never obtain its default session. Align the provider JSON Schema and parser with the directory-owned profile grammar, and add trailing-dot/trailing-hyphen negative fixtures.

### P2: Handle Provider Spawn Errors Before Detaching

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts:393-401`

The new generic contribution accepts any nonempty executable name. If that program is absent or cannot execute, Node emits the child's asynchronous `error` event; this child has no error listener. Returning `child.pid ?? null` does not consume that event, so the serve process terminates from an unhandled event instead of using the existing typed no-owner/local-only outcome. Await spawn success/error or attach an explicit handler and expose failure through the generic owner handoff. Cover a missing contributed executable with a focused owner test.

## Inspection Boundaries

The neutral job result delegates payload decoding to its application; the Rust envelope deliberately validates fields after application transport decoding. The plugin deployment owner preserves catalog/route JSON exports and declares their JSON-only format support separately from manifest/index TypeScript/Rust exports. No additional actionable issue was established for those boundaries by this Light source inspection. Fixture hashes were inspected in source, not recomputed; aggregate runtime validation remains the coordinator's responsibility.

The generic execution leaf still contains catalog-header/freshness helpers and concrete trusted-catalog filenames, consumed by the hub owner. Cancellation is also absent from `ensureDevLocalHub` and its readiness loops. These are further architectural review areas, not additional runtime-verified regressions in this report.
