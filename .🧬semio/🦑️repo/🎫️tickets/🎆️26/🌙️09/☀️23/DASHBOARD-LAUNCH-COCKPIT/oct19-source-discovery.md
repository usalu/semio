# October 19 Current Source Discovery

## Implemented Design

Text parameters can declare neutral `suggestions` with source `projects`, `scripts`, `tests` or `configs`, plus optional `projectParameter` and `directoryParameter` relationships. The registry schema rejects suggestions on flags/closed choices and extra/unknown fields. Native declaration validation checks related fields are other text parameters. The registry JSON exposes the suggestion declaration; suggestions remain editable values and grant no execution authority.

The current canonical registered project roots supply project identities. A physical source value belongs to its deepest current registered owner. Script suggestions identify the permanent `📜️script.ts` filename. Config suggestions identify `tsconfig*.json`. Test suggestions require direct runtime bindings imported from `bun:test`; comments, strings, regular expression examples, full/named type-only imports and side-effect-only imports are excluded. Mixed type/runtime imports remain discoverable. No historical editor catalog, duplicate command registry or legacy adapter supplies these choices.

The native registry lazily starts a standard-library worker when a source field needs it. Owner-specific scopes avoid an eager whole-workspace scan during ordinary registry construction, listing, resolution and project selection. Views subscribe to a shared scope; cancelling or closing one view releases only that subscriber. Last-subscriber removal stops the worker. Explicit refresh requests a fresh scan on the existing shared index without discarding other subscribers. A subsequent form does not inherit cancellation. Physical directory/candidate-file modification times invalidate the index, with bounded metadata work.

The source scanner skips declared concrete generated/cache roots and explicit dependency/generated/fixture boundaries. Authored plain `build`, `dist` and nested `cache` remain traversed. Exact repository cache and temporary roots are excluded, including `.🧬semio/🦑️repo/⚡️cache` and `.🧬semio/🦑️repo/🧹️tmp`. Symbolic links and Windows reparse points/junctions are rejected before directory traversal and candidate-file reads. Portable UTF16 path capacity and control-character/path-component checks apply.

The schema-owned bounds are one million inspected entries, 100,000 source rows, 64 KiB per candidate read, 32 MiB total reads, 32 visible matches, 1,024 query characters, 16 query words, eight active scopes and at most 1,024 metadata checks per scope every 500 ms. The UI displays scanning counts, cancellation, errors and incomplete/limited results. Large/partially read files explicitly mark the result incomplete. This bounds work; it does not establish measured global-workspace latency. Explicit owner selection and refresh remain available.

The existing launcher form renders searchable values with their project and registered owner directory. Dedicated customizable suggestion actions use Alt+Up, Alt+Down and Alt+Enter by default; Ctrl+R refreshes and Ctrl+G cancels. The actual configured bindings appear in English/German help. Ordinary text cursor keys remain editor inputs. Pointer acceptance and keyboard acceptance call the same transition, and the existing UI event loop receives compact background-change tokens so completed discovery can repaint without another keypress.

Accepting a source fills its related project and, when still derived/unset, its actual registered owner root as the working-directory default. It does not infer working directory from the script filename. Selecting another source updates a derived directory; manual edits are retained. Changing a project clears dependent automatically selected source/directory values while retaining deliberately typed overrides. This preserves explicit user control and prevents silently retaining an old automatically selected source under a new owner.

Root metadata remains owned by the capability agent: project/source suggestions on run-script and bun-test, current directory wording and a config picker on the typed compiler owner. Root source tools remain literal argument-vector declarations.

## Executed Validation

- Independent Ajv validates the shared source contract and seven positive/negative parameter declarations. TypeScript's own parser independently identifies runtime `bun:test` imports; fast-glob independently verifies the physical fixture inventory and excluded boundaries.
- An actual Rust regression first failed because the old lexical detector incorrectly included regex examples and type-only imports. The shared vectors and production detector were corrected together; mixed type/runtime imports, named-only types, side effects and standalone regex expressions are now covered. A final auditor-supplied valid division expression before an import passed the TypeScript AST oracle and failed the actual Rust scanner, then passed after adding expression-context slash handling.
- The direct rustc harness compiles and executes exact production registry suggestion/subscriber/refresh methods, scanner, ownership/filter functions, watcher chunks and form acceptance/invalidation methods. The harness supplies small model records and the schema-owned policy values; it does not compile the entire dashboard dependency graph or prove its Serde/model integration.
- Actual native assertions cover all shared owner/search vectors and owner-directory tuples; cancellation with two subscribers; reopening; refresh preserving the shared state/users; a newly added physical source invalidating the index; cancelled traversal; incomplete per-file reads; bounded metadata chunks and query limits; automatic directory reselection; manual directory preservation including deliberately empty cwd; project changes clearing derived fields; and explicit overrides surviving project changes. The empty manual-directory vector first reproduced an unwanted autofill, then passed after distinguishing an untouched default from an explicitly edited empty value. Final audit also found generic form normalization discarded that value before acceptance. The harness was extended to execute actual normalization between edits and selection, reproduced the failure, and passed after preserving empty values for declaratively related directory fields. Other text-default normalization remains unchanged.
- A real Windows directory junction from the fixture workspace to a sibling outside root was created. The actual Rust scanner refused its source. A shared selected owner nested behind that junction first failed by offering its outside source, then passed after validating every selected-owner ancestor and root before traversal. Five shared reparse/symlink attribute vectors also ran. Fixtures and generated compiler/harness outputs were removed by bounded cleanup after each run.
- The focused source suite passed after the fixes: two tests, 23 Bun assertions plus the Rust assertions, with `[DEBUG]` runtime evidence. Final full non-native authority result is recorded below after convergence.
- Native launcher unit tests now specify pointer/keyboard equivalence and localized custom binding help. They have not executed in an admitted full native dashboard build. Parent's final read-only rustfmt parsing accepted both the scanner and launcher including final normalization; this is parser evidence only.

## Final Point-in-Time Verification

The final focused source suite passed **2 tests, 23 Bun assertions and the exact-production Rust assertions**, including selected-owner ancestor junction refusal and normalization preserving an explicit empty cwd. The subsequent complete non-native authority run passed **31 tests**, failed two, filtered the installed-native test and executed **1,363 assertions** in 41.49 seconds. Its source discovery, current registry declarations, 1,447 owners, 3,792 targets, 3,791 canonical script invocations, root metadata and typed Bazel controls passed. The failures were course README launch instructions and the empty `.vscode` directory.

After the parent reported the course README corrected, the final scoped documentation/catalog/directory command executed **three tests: zero passed, three failed, 31 filtered, 172 assertions, 24.15 seconds**. The observed files had been concurrently restored again: course README lines 50/212 instructed `.vscode/launch.json`, with Run and Debug operations also at 95/114; the catalog existed; the inventory refused exactly `.vscode` and `.vscode/launch.json`. The independent writer and conflicting instructions are recorded in `oct19-authority-execution.md`. No test was weakened, document exempted or blocked deletion retried through another API. These are actual final failures, not a green exclusivity claim.

The final scoped command from the repository root was:

```powershell
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts' --test-name-pattern '^(developer instructions agree|active editor launch projections|every active source directory)'
```

The caller-owned `SEMIO_TEST_ARTIFACT_DIR` was the current ticket's `🗑️generated` directory. Earlier converged all-green authority runs are historical snapshots and do not supersede the final observed failures.

## Genuine Remaining Limits

Source discovery does not reconstruct the former purpose, accepted command modes, required domain arguments or help for every historical helper. Generic paths are now discoverable and correctly associated with current owners; specific mode/argument forms still belong beside each actual owner. Canonical script discovery intentionally does not label every arbitrary TypeScript helper as a permanent executable script. Explicit path entry is retained. A file that only aggregates other test modules without a direct runtime `bun:test` import is not automatically labelled a Bun test; conversely a runtime import alone cannot prove the file registers nonzero cases without executing it. The bounded native lexical detector is validated against the shared TypeScript AST cases; it is not a complete JavaScript/TypeScript parser.

The owner root is a visible current default, not proof of a historical helper's required cwd. Humans can explicitly override it; individual historical invocation acceptance remains distinct from discovery. Large global scopes can reach the declared limits, and their real-workspace latency has not been benchmarked. Background invalidation is bounded polling rather than an instantaneous native filesystem notification.

The admitted installed dashboard remains unavailable because native store provisioning is blocked. Whole-dashboard compilation, real terminal pointer/keyboard interaction, installed registry dry-runs and all-platform native integration remain unverified. No storage bypass, alternate native publication or permission workaround was added.

## Changed Files

- Dashboard registry JSON schema; new source-discovery JSON schema, shared fixture, TypeScript regression and Gherkin feature.
- Registry module, declarations, model, validation, builder, record, lookup and JSON projection; new source-discovery Rust module.
- Terminal launcher, native launcher unit tests, controls, event loop and bilingual labels.
- Default keymap and shared keymap fixture.
- Authority test registration, source-tool assertions and current explicit project in the bun-test native dry-run vector.
- This retained report.

Exact source-discovery paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`:

```text
🧬️schema/🎮️registry/🔣️.json
🧬️schema/🔎️source-discovery/🔣️.json
🧪️tests/🧫️fixtures/🔎️source-discovery/🔣️.json
🧪️tests/🔎️source-discovery/🟦️.ts
🧪️tests/🔎️source-discovery/🥒️.feature
🎮️registry/🦀️.rs
🎮️registry/🧾️declarations/🦀️.rs
🎮️registry/🧬️model/🦀️.rs
🎮️registry/🔍️lookup-and-search/🦀️.rs
🎮️registry/📚️registry-record/🦀️.rs
🎮️registry/🧶️build-and-derive/🦀️.rs
🎮️registry/🛂️tokens-and-validation/🦀️.rs
🎮️registry/🩺️check-and-json/🦀️.rs
🎮️registry/🔎️source-discovery/🦀️.rs
🖥️terminal/🦀️.rs
🖥️terminal/⌨️controls/🦀️.rs
🖥️terminal/🌐️labels/🦀️.rs
🖥️terminal/🚀️launcher/🦀️.rs
🖥️terminal/🚀️launcher/🧪️tests/🔬️unit/🦀️.rs
⚙️preferences/⌨️keymap/🔣️.json
🧪️tests/🧫️fixtures/⌨️keymap/🔣️.json
🧪️tests/🧭️authority/🟦️.ts
🧪️tests/🧫️fixtures/🧭️authority/🔣️.json
```

The prior authored Go compiler overlay input was restored outside generated output at `oct19/go-authority-overlay.json`; it references checked-in sources and copies none of them. Input retention supersedes the earlier deletion statement in the authority execution report.
