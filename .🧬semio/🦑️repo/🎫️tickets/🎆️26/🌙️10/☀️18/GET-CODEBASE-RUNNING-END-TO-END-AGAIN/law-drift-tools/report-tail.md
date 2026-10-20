
## Real code/data defects found by the laws (code-is-wrong; not law drift)

Non-library (owner in brackets; none fixed because they are Rust/data work in flight or need cargo):
- fem analyses (`✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses`): dev-deps on the stdio plugin, stdio-base and teaching-proctor-base because an import fixer bound `AssemblyPlan` to `semio_s_plugin_stdio` (the real one is `pub(super)` in `🧩️assembly`) [fem/rs-sweep].
- replication partition: unused `semio-framework-replication-error` dep in history-index; orphan (never mounted) Rust files `retained/🧱️row` (+tests), `retained/🔺️diff/🪢️sequence/🧪️tests`, `transition/🧪️tests/🔬️unit`; mutation-messages native-test closure 232,899 B > 230,000; `impl HistoryFoldSet` in the history crate while the type lives in history-index [rs-sweep].
- General package `@semio-tech/framework` package.json exports 22 `os/*` entries to Specific stubs; General replication `🧳️retained` test imports os dsl/derive fixtures [exports-stage / package.json owner].
- mesh facade lock edges: 31 Cargo.lock files still list journal/surface/quality edges removed from the `semio-framework-3d` manifest [lockref].
- mesh/document had `semio-framework-test-fixtures` as a production dependency through an uncfg'd `use` (FIXED before the pause: `#[cfg(test)] use`, dependency left in dev-dependencies). 277 Cargo.toml still list test-fixtures under `[dependencies]` (test companions are fine, production crates are not) [rs-sweep/deps].
- db crate-boundaries: unused `semio-framework-pack` dependency in db artifact-engine, io-registry, io-driver; `serde_json` (tests only) as production dep of io-core [rs-sweep].
- 🧪️test module: 2 schemas not draft-07, schema catalogs stale in 5 dirs, owner-ineligible `📦️packages/🟦️typescript`; question-dictionary.json outside a 🧬️schema dir (mutation index cannot publish it) [test module owner].
- Server coordinator builder script closure 526 files (cap 192) [server owner].
- Fixture placement: 11 os plugin fixture files outside a 🧪️tests owner; 14+ in teaching/architecture course, hub observability and ✏️s dev fem stories [owners of those splits].
- plugin registry semantic source pairs: a committed descriptor JSON/pack pair disagrees (regenerate).
- Dashboard authority: `AGENTS.md` lost the pinned `launch.json` instruction (user decision pending with the `.vscode` retirement).
Library (frozen, so staged or reported):
- `nativeHostArtifactPathV1` derives the binary path from `outputs[0]`, which is now the store receipt (`.🧾️store/native-build.json/<binary>`); expected `dist/native-<mode>/<binary>` (law staged, stays red until fixed).
- `caching/📦️artifacts/📋️native-orchestration` imports bare `@semio-tech/process/build-mode` (CLI-closure rule + no workspace authority).
- 11 plugin lib roots exceed their wiring ratchet pins (puzzle jobs 466 -> 613, energy, fem app-surface, generation3d host, ...), process3d retained-cursors 665 lines > 150.
- Indexed generated output: git tracks the jcoprobe jco browser bundles (known, user decision).
- Shared paint-editing fixture bytes changed vs the pinned sha256 and violate their own schema (`selectionTransport.cases[1].spans`).
- generation2d `🧬️schema/🟦️.ts` lacks the document declaration `Generation2dArtifact` (generated schema TS stale).
- Framework root Rust package entry classifies as implementation (expected declaration).

## Staged patches

All 24 applied (22 before the second pause; `cc-project` and `cc-test` after the full lift with `ALLOW_PROJECT=1`). Live reruns (r4): 11 suites flipped to green with the patches (61, 65, 66, 72, 74, 84, 100, 104, 105, 108, 114); 59, 82, 86, 106, 107 stay red on real defects.

## Run 8 regression is environmental, not from the patches (read this first)

After the second pause the dependency sync switched `node_modules` to generation 70825028305173c7f252, which lacks workspace links that sources now import: `@semio-tech/framework-transient` (32 suites), `@semio-tech/framework-hash` (6), `framework-filesystem/observation/descriptor` (2), `framework-filesystem/testing/fixtures`, `framework-manifest`; and the framework package exports now point at moved/missing stub files (`📦️packages/🟦️typescript/📁️filesystem/🟦️.ts`), which also invalidates the blessed `nativeEntries` roster again (suite 81, derived data). 12 suites that were green in r6 (2, 8, 34, 40, 49, 55, 65, 75, 84, 89, 100, 114) fail only on those missing links or on timing. Fix = package.json/bun.lock dependency owner links these packages, then re-bless suite 81's roster (see Node-loadability below) and rerun the runner.

An apply.sh bug (idempotence for pairs whose new text contains the old anchor) double-applied 6 patches once; detected, repaired from the validated staged outputs, and fixed in `stage.py` (`b in s` check). Currently `apply.sh --dry-run` reports every row DONE.

## Node-loadability sweep (suite 81, applied live)

- Roster `nativeEntries` blessed to the real 55-entry closure.
- Relative JSON imports/exports gained `with { type: "json" }` (Node requires it); bare package specifiers keep none when the package export target is a `.ts` stub (the attribute makes Bun parse the stub as JSON). A first blanket pass over 175 files was corrected: 16 files had the attribute removed again from bare specifiers.
- Constructor parameter properties converted to explicit fields in: caching/pruning (+measurement), bootstrap script (NxOutputSpoolV1), discovery/schema-reads (ReadScope, ReadReader), normalization/repository-authority.
- `import.meta.dir` -> `import.meta.dirname` in caching/artifacts/typescript.
- `jsonattr.ts --apply` was run after the pause (2 bare specifiers resolving to real `.json`: wgpu `host-io`, library `storage`). Re-bless of the roster is needed again because peers moved package stub files.

## Files changed (applied)

As in the first list plus: the 22 staged patches (library/hub/db/ui/dsl fixtures and laws), the 175-file JSON-attribute sweep (net: relative specifiers only), the six parameter-property files, `🏃️process/…/native-build/📥️inputs` and `📤️publication/📥️sources` schema imports.

## Verification

- Runner: `python3 C/law-drift-tools/run.py <list> <out.jsonl> 450 120` (single `./file` per run, 2 concurrent, finite per-suite envelope, artifact dir `C/🗑️generated/l/<8>`): r1 2/117, r3 44, r4 55 (after patches), r6 54 (suite 20 timed out at load; 81 needs jsonattr).
- Node load check of all 55 entries: `law-drift-tools/nodeload.py` (only the policy-gated vitest config fails by design). `bun build --target=bun` of 300 files touched by the sweep: 0 failures.
- Not run: cargo/wasm/browser, `bun repo run`, git modification, package.json/bun.lock/project.json edits.

## Open items

1. After the dependency owner links the missing workspace packages: re-bless suite 81's `nativeEntries` (debug-copy method in the report history) and rerun `run.py` (expect >= the r6 set plus 114 with its project.json inputs).
2. Owners: taxonomy-normalizer churn laws (79, 80, 83, 92, 93, 94, 101), launcher/envelope-gated (62, 64, 67, 70, 111, 16, 17), needs cargo (1, 3, 36, 38, 77, 95, 109), timing (19/20/69), silent suite 76, hash/roster ledgers (52, 87, 88, 90, 91, 103), the real defects listed above.
3. 102 files still use constructor parameter properties (not strip-only); only the Node-load closure of the 55 entries was fixed.
4. The `[DEBUG] General command …` progress line in `🧰️framework/🔨️modules/🏃️process/📜️script.ts` is a permanent debug-prefixed log (not removed here).
