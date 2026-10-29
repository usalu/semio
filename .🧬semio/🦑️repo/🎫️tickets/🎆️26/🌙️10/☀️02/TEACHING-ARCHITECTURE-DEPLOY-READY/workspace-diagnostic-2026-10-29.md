# Root Workspace Diagnostic Dependency — 2026-10-29

The actual Part21 canonical Cargo attempt failed before compilation because composition-publication inherits `semio-framework-diagnostic` from the repository root workspace, which omitted that dependency. This was a manifest ownership/declaration failure, not a missing registry package.

## Change

Added exactly one declaration under root `Cargo.toml` `[workspace.dependencies]`:

`semio-framework-diagnostic = { path = "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust" }`

The diagnostic package keeps its canonical name and framework workspace ownership. Publication remains an explicit root workspace member. No alias, ownership transfer, compatibility layer, dependency fetch, source-cap change, lockfile edit, or runtime dependency was introduced.

## Scoped inheritance audit

The neutral corpus checks every dependencies/dev-dependencies/build-dependencies table, including target-specific tables, of these actual callers against its declared owning workspace and membership/exclusions:

| Member | Workspace | Inherited occurrences |
|---|---|---:|
| OS plugin | Root | 8 |
| Composition publication | Root | 4 |
| Stdio contract editing | Stdio | 3 |

Only publication's diagnostic entry was missing. The other eleven root occurrences and all three stdio occurrences resolve. This scope does not claim a repository-wide inheritance audit.

## Regression and evidence

Schema-first neutral corpus and focused test are under:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🧬️inheritance`

The test uses independent `@iarna/toml` and `Bun.TOML.parse` parsers, Ajv schema validation, and existing picomatch membership semantics. Both parsers reproduce the exact single missing row when the declaration is removed from their parsed root document. They then require all actual scoped inheritance to resolve, the exact canonical path declaration, canonical package name, diagnostic framework workspace ownership and membership.

- Red: 2 passed, 1 failed; the positive current-tree check reported exactly publication → dependencies → diagnostic.
- Green: **3 passed, 0 failed, 63 assertions**, exit 0.
- Command: `SEMIO_LANGUAGE=en repo tools exec --light bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🧬️inheritance/🟦️.ts'`.
- Retained generated evidence: `🗑️generated/workspace-inheritance-red.log`, `🗑️generated/workspace-inheritance-green.log`.
- Original Cargo diagnostic: `🗑️generated/part21-visible-lock-diagnostic.log`.

One initial invocation was refused by the existing Bun explicit-operand guard; it was corrected to an explicit `./` path. Test implementation's initial root-coordinate assertion was corrected before the causal red run. Neither transient result is presented as the regression baseline.

## Changed files and remaining gate

- `Cargo.toml`: canonical root dependency declaration.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🧬️inheritance/🔣️.json`: language-neutral inheritance corpus.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🧬️inheritance/🧬️schema/🔣️.json`: closed corpus schema.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🧬️inheritance/🟦️.ts`: independent-parser regression.
- This report.

Source is ready and frozen. No Cargo process was launched by this agent for this repair; frontend owns the active full semantic run. Actual canonical Part21 lock preparation, compiler admission, and the named runtime law must still execute in the coordinated queue. Focused manifest success is not claimed as Part21 Rust execution.

## Canonical Part21 retry 58811

After parent authorized the sole compiler slot, the unchanged exact owning law was run from `✏️s/🔌️plugins/🗄️stdio` with release/shared/parallelism1 and ticket environment. **Authoritative terminal exit: 1**. The prior manifest inheritance failure did not recur. Native Source admission completed 101 of 382 units, then refused unit102 `semio-framework-os-kernel`: `Unbalanced Rust source delimiter`. No compiler or Part21 runtime law executed.

Log: `🗑️generated/part21-workspace-fixed.log`.

A bounded read-only reproduction using the canonical source reader (approximate host cfg, not a substitute compiler admission) identified `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🦀️.rs`. Canonical token pairing reports an unmatched opening brace at line125, offset23672. Probe input: `kernel-source-location.ts`. No edit, cap relaxation, source bypass, automatic retry, or peer process action was performed. The file may be under another owner's active work; coordinator was notified for ownership before any repair.
