# Cargo Role and Contract Regression Audit

Read-only follow-up. No builds, native reruns, source or metadata edits, Git mutations, worktrees, ticket changes, or goal changes. Final Cargo owner verification remains pending at this initial inspection.

## Purpose-Based Role Review

All16 rows in `🛠️2026-09-30-cargo-role-declarations.md` match the current manifest role/id fields. Manifest descriptions, target kinds, entrypoint paths, and source module declarations agree with the declared purposes:

- The async, value, DSL, schema, dispatch, and machine macro crates expose generic code generation as proc-macro targets: framework.
- The generic `semio-framework-plugin` SDK exposes ArtifactApp/runtime interfaces rather than one concrete plugin: framework.
- Space, collection, playbook, workflow, and run source roots describe persisted document models: artifact.
- Draw FSM exposes generic static statechart tables, actor runtime and hosts; its sibling generates statechart tables: library. Both remain physically Draw-owned and therefore implementation-layer packages. These declarations do not resolve their placement.
- Imperative extension SDK exposes shared manifest/evaluation support from the imperative module: s-module.
- Font assets manifest declares the `dump-guestslim-typst-fonts` binary: tool.

The original metadata inventory's16 missing-role observation is a historical pre-edit result; the purpose table remains useful, but it must not be presented as current missing metadata after these declarations.

## Physical Ownership Independence

In `🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts`, the physical upward-rule branch compares classified package owner paths before semantic role checks. It does not filter on `library`, `artifact`, `product`, or `tool`; assigning Draw FSM library cannot hide a framework-to-Draw dependency. The semantic rules are added alongside physical violations. Missing/unknown roles and unclassified physical owners are separately reported as problems and fail the production execution path.

## Preliminary Contract Inspection

The production adapter requires Cargo metadata version1, unique IDs matching the complete member list, complete dependencies and activation declarations. Independent authored inventory reads every explicit root workspace member and direct, inherited, dev, build and platform-specific dependency declaration. Both normalized inventories must match exactly; missing importers or local targets and edited declaration kinds, platform, optionality or role cannot silently pass. Owner paths escaping the repository refuse. External declarations are deliberately terminals.

The neutral tests validate schemas through Ajv, serialize/parse authored fixtures through existing third-party TOML, and obtain actual offline Cargo metadata for each fixture. Fixture verdicts include missing and unknown roles, physical upward edges, semantic edges and allowed declaration kinds. Current corruption cases directly exercise the normalized report, while the production Cargo adapter additionally checks raw member/ID completeness.

No new actionable issue found in the reviewed declarations or preliminary contract. Final settled source and execution evidence must be checked after the owner handoff; this preliminary review claims no independently executed test pass and no root aggregate completion.

## Refreshed Evidence

The updated authored inventory follows the complete local dependency closure in addition to root members, matching Cargo's implicitly enrolled local packages, and the adapter verifies workspace_root belongs to the requested root. Physical enforcement remains independent of role labels.

Owner `🗑️generated/cargo-direction-fixtures-2.log` records four passing tests and150 assertions using actual Cargo metadata. `cargo-direction-final-verification.log` separately marks Cargo fixture and typecheck targets successful, but its combined run fails OS development composition ownership: the project inputs contain an extra `🧑‍💻dev/🧪️tests/🧹️layering-policy/🟦️.ts` path versus the portable ownership fixture at test line164. This concrete fixture/input discrepancy was sent to the coordinator; the combined run is not green.

Coordinator reported the standalone strict scan inventories275 packages and3121 local declarations, with zero metadata problems and21 actual violations (Flow10 physical, Procedural11 semantic). Those violations remain explicit cleanup work; declaring roles did not remove them. The final16-owner integrated canonical aggregate is independently confirmed successful in `🗑️generated/canonical-integrated-final.log`, taking7m49s. This result does not include the separate still-red Cargo checker or full renderer suite.

## Settled Checker Review

The checker owner explicitly relayed settled source readiness. Reinspection confirms the implicit-local-member fixture uses an authored root with only the first package and compares the discovered local closure with actual Cargo metadata. Portable schema bounds dependency kinds and activation fields; exact metadata/inventory signatures retain aliases, optionality and platform clauses. The generic plugin SDK role and physical upward enforcement are unchanged. No new actionable contract regression found. Owner reports the stale development-layering Nx input has been removed and is rerunning its focused regression; that repair's runtime result is not claimed here yet.

Settled follow-up: `🗑️generated/cargo-direction-final-owned-checks.log` records successful combined Cargo contract fixture, repo-lib typecheck, and generic47-owner OS development composition targets. The stale layering input discrepancy above is resolved. Checker source/config/TS completeness remain unchanged. The live graph's actual ownership violations and in-progress Flow composition edits remain separate from this successful checker contract verification.
