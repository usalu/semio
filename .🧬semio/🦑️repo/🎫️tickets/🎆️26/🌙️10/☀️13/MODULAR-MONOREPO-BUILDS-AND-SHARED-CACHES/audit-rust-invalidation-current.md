# Current Rust Invalidation Audit

Static read-only audit. Read current producer qualification and warm profile reports; no builds, setup, Source mutation or gate promotion. Current producer policy explicitly declares `cache:true`; existing report records genuine both-mode 40/355 production qualification. This audit does not extend that runtime qualification to subsequent revisions.

## Findings

### P2: Every supported unit hashes the entire Cargo lock and workspace manifest

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts:32–33` unconditionally admits raw workspace `Cargo.toml` and `Cargo.lock` bytes into every selected unit configuration. `.../📦️units/🟦️.ts:31,36` includes those facts in semantic digest and native Nx file inputs. Consequently an unrelated lock package, comment-only lock edit, unrelated workspace member registration, or unrelated workspace dependency declaration invalidates every selected library even when effective unit/compiler/source/dependency facts are unchanged. This is a concrete remaining global invalidation surface, separate from installer-source isolation already achieved.

Preserve complete raw Source admission/rechecks for discovery, but derive artifact hash from the selected lock dependency closure and actual effective manifest/profile facts. Existing `.../🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs:25–27` already computes a selected lock closure digest for a different graph input owner. Add genuine ordinary unrelated-lock-entry and unrelated-member mutation oracles with unchanged selected hashes/receipts/output bytes before narrowing the producer.

### P2: Raw Cargo unit evidence leaks graph indices into artifact identity

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts:23` sets `compilationDigest:digest(unit)` on the raw Cargo unit, whose dependency references are numeric graph indices. The same line separately canonicalizes dependency edges to stable owned unit IDs. `.../📦️units/📥️descriptor/🕸️graph/🟦️.ts:18` hashes all observed unit facts into `SEMIO_CARGO_UNIT_EVIDENCE`, which is included in plan digest at `.../📦️units/🟦️.ts:31`. Therefore a planning result with identical compiler semantics but reordered unrelated units can alter a dependency-bearing producer hash through changed numeric indices. Root additions can also change the raw graph without changing that producer’s actual stable predecessor IDs.

Keep raw graph custody as admission evidence, but derive artifact evidence from normalized stable producer/dependency facts. Add a neutral two-equivalent-raw-graphs corpus and independent real Cargo multi-selection ordering oracle; no current installed Cargo ordering trigger was run here.

## Selector Check

Current declared root dependency selectors use positive `*` or `@semio-tech/repo-*` followed by `!workspace` / `!@semio-tech/repo-test-domain`. They agree with current portable selectors; no current declared ?/[] mismatch was found in the 698 inventoried project configuration files.

Installed Nx source at `.nx/installation/node_modules/nx/dist/src/tasks-runner/utils.js:99–111` expands target globs with minimatch; `.../utils/find-matching-projects.js:20–65,196–204` processes project patterns sequentially and uses minimatch. Portable `.../🏗️build/🧭️selection/🟦️.ts:19,24–25` supports only `*` and applies all exclusions last. Thus accepted CLI `--targets=test?` with a `test1` target selects nothing in portable admission but matches under installed Nx; accepted `--projects=*,!app,app` cannot reinclude app portably though installed Nx does. The argument validator at lines 97–104 admits these values without refusing unsupported pattern syntax. These are concrete accepted semantic mismatches, not claims that current authored dependency declarations use them. Either implement shared installed-Nx selector semantics or reject unsupported accepted syntax before native dispatch. Add actual Nx comparison corpus for both examples before a fix.

## Remaining Scope

Current unit configuration includes the dedicated compiler entry’s exact transitive defining closure, not the aggregate installer router. No direct raw mutable `.nx/installation` pointer path is present in the unit inputs examined. Environment includes PATH/RUSTUP_HOME and exact compiler identity, so machine/toolchain changes remain explicit rather than silently reusing artifacts. Global workspace membership still affects frontend admission/catalog work as described in the warm report; the last qualified approximately seven-second warm unit fixture is not complete full-workspace frontend qualification.
