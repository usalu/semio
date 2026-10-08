# Schema Custody Read-Only Audit

The deleting actor is **unproven**. This finite audit inspected source and a process snapshot without restoring schemas, modifying infrastructure, invoking heavy checks, or changing Git.

## Confirmed Observations

- `🗑️generated/sol-2026-10-08/geometry-port-oracle-1.log` shows the actual Nx ownership target importing the test and reaching AJV compilation at test line 7. Its error is unsupported draft 2020-12, not a missing-file error. The later disappearance is reported by the parent task; this audit did not independently capture the unlink event.
- At audit time the geometry schema leaf is absent while its fixture and TypeScript/Rust tests are present. The TypeScript reader has concurrently changed to a fixture-only oracle. This is evidence of live source edits, not proof that those edits removed the schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json:168–176` declares the ownership target with `cache:false`, `dependsOn:[]`, and `outputs:[]`. The package script imports the ownership test. There is no target-defined output restoration that explains the removed leaf.
- Root `📜️script.ts:15027–15042` schema generation renders the catalog and writes only the taxonomy catalog path. It does not recursively prune schema source directories. Root direct `rmSync` sites are an executable alias at line 228 and the cache directory at line 14699.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:117` directly removes an install marker. The reviewed bootstrap does not directly delete schema leaves.

## Proven Deletion Risks, Not Proven Incident Causes

All following infrastructure paths are beneath `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

- `🧹️normalization/🟦️.ts:8675–8680`: rollback recursively removes each started regeneration's declared `outputRoots`, then reconstructs pre-output directories and backups. A concurrently authored leaf beneath a declared output root can therefore be removed. Lines 9704–9713 compare pre-output state before execution and exact outputs afterwards; an unexpected concurrent addition can trigger failure, followed by rollback. No inspected plan or process proves the missing geometry/UI/Mesh leaves belonged to such a running regeneration.
- `🧹️normalization/🟦️.ts:8525–8532`: source-parent pruning requires the actual directory to be empty and uses `rmdirSync`; this cannot itself delete a nonempty schema leaf.
- `⚡️caching/🧹️pruning/🟦️.ts:258–276`: `deleteUnit` unlinks or recursively removes a unit beneath its supplied area root. Its Cargo path caller at line 208 uses the build directory. No observed call supplies the geometry schema directory.
- `🧼️workspace-cleanup/🗑️removal/🟦️.ts:32–46`: the cleanup gate can recursively delete approved candidates. `🔍️candidate-discovery/🟦️.ts` collects misplaced metadata, root transients, Windows-illegal paths, large ignored files, ticket generated output, and build artifacts. No schema-specific candidate rule was found; the named schema path does not match the reviewed misplaced or Windows-illegal rules.

## Process Evidence and Limits

A read-only `ps` snapshot found concurrent root `verify mutation-outcome-law` commands, Nx bootstraps, managed native test commands, and a dashboard generation command. It did not show a schema-generation, taxonomy-apply, or cleanup command. A snapshot cannot establish historical absence or attribute a past deletion. Other agents' shell processes visibly edit source; their unrelated work was left untouched.

## Safe Repair Proposal

Keep authored schema inputs under explicit source ownership; reserve regeneration output roots for exclusively generated artifacts. In the original normalization owner, make rollback removal conditional on the exact transaction-owned postimage and preserve unexpected concurrently authored descendants, reporting custody drift instead of deleting an entire changed output root. Add a language-agnostic law fixture plus an independent filesystem oracle for concurrent source insertion during failed regeneration. This is a justified hardening proposal for a proven generic risk, not an established fix for this incident.

Before naming an incident cause, capture a narrowly scoped deletion event with responsible PID and command, or find a transaction journal explicitly listing the disappeared path. Do not repeatedly restore schemas or alter hashing to conceal absence. No runtime custody fix was applied or tested by this audit.
