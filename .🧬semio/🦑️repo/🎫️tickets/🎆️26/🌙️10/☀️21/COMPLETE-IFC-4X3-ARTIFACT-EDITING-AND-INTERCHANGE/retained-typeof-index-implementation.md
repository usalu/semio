# Retained TYPEOF Immutable Parent Index Checkpoint

The actual retained TYPEOF continuation now consumes the root-owned immutable metadata::select_parents API instead of scanning every released type declaration/member for every emitted seed. The native SELECT semantics remain those in typeof-select-cases.json. No metadata/index generator, cold evaluator, common Frame contract or TS implementation was changed by this worker.

## Continuation And Custody

Only machine-population.rs stage5 changes production behavior. It borrows the current original queue seed, strips only the exact IFC4X3_ADD2 prefix and reborrows its immutable direct parent slice. The existing frame.position addresses one actual parent. Exhaustion returns to stage4; a parent starts the existing TextCopy, Contains, append and stage8 continuations. Stage8 now increments a direct parent position rather than a catalog position. The same live linked SET queue discovers newly appended SELECTs, including append after the current tail. Existing deduplication prevents repeated insertion.

Frame has no borrowed parent-slice field, so the immutable slice is reborrowed for each parent transition through the bounded normalization/binary lookup. No request container, reverse-index construction, global lazy state, cache funding or new ownership owner is introduced. Fixed bounded metadata lookup work remains part of the existing semantic transition; no fictitious scan ticks or copy receipts are added. Genuine text allocation/copy, equality/deduplication, queue append, source/environment pins, grants, fuel, cancellation and retirement continue through their existing original boundaries. Stage4 and stages6–8 source custody are unchanged.

The native declaration scan cost is removed from this route. No native total turn count, original wallet outcome or full population timing is claimed because the source cannot currently execute through authentic ingress.

## Test-First Contract And Laws

The language-neutral typeof-select-cases.json already supplies thirteen exact expressions, three explicit runtime seed cases and both MaxOneColour population outcomes. It is unchanged. Root's shared static-index assertion checks every generated edge against the released language-neutral catalog.

Before the production edit, machine-typeof-tests.rs gained two native laws. The first compares immutable direct slices with an independent released declaration/member scan for every entity and type, and checks the lookup itself requests/releases no heap bytes. The second executes actual retained TYPEOF over RGB, records each stage5 (original edge,parent position), rejects positions beyond the original seed's direct parents and compares visited boundaries with the actual final unique SET. It requires transitive IfcColour/IfcFillStyleSelect membership. Its original measured advancement helper denies each insufficient actual funding axis before the funded single-fuel transition and closes through original retirement. Existing thirteen-vector, two actual population, foreign-source pin and stage4–8 cancellation/close tests remain mounted.

These native tests are authored only. Test-side sets/vectors record expectations outside measured production transitions; they are not runtime containers or substituted production inputs.

## Verification

Executed `bun test <validation>/typeof-select.test.ts`: **21 pass, 0 fail, 47 assertions**. This includes closed neutral schema, immutable native index direct-edge coverage, all expression/explicit seed cases, both actual TS population cases and work/cancellation budgets. It does not execute Rust.

`rustfmt --edition 2024 --check --config skip_children=true` passed for both changed native files. Authentic `cargo test --manifest-path <IFC4x3>/📦️packages/🦀️rust/Cargo.toml --lib native_typeof -- --nocapture` refused at its -vV enforcing ingress with `Original compiler ingress requires the repository launcher and authentic enforcing store`, before rustc. No bypass was attempted. Native compilation, type checking, test execution, actual scheduler/heap/receipt/close and total population performance flags remain **false**.

The existing independent IfcOpenShell/NumPy named-seed graph evidence is documented in typeof-select-implementation.md and typeof-select-independent-audit.md; this worker did not rerun or extend those oracle totals. Root separately audits the new immutable index and cold evaluator. Wider EXPRESS conformance and native semantic admission remain outside this optimization.

## Changed Roster

Source prefix: `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/`.

- `machine-population.rs`: TYPEOF stage5 indexed direct-parent continuation.
- `machine-typeof-tests.rs`: immutable direct membership/no-heap law and actual original queue visit law.
- Ticket `retained-typeof-index-plan.md`, this report and appended `shared-coordination.md`.

No generated output files were persisted, no shared dependencies removed and no lifecycle, Git, worktree, AGENTS, UI or unrelated source changes were made.
