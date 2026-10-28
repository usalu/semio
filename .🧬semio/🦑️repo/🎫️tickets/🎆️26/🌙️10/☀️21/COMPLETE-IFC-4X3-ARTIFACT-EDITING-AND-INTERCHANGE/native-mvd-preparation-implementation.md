# Canonical Native MVD Preparation and Terminal Borrowed Clauses

The source checkpoint replaces Direct and Terminal's separate request/source preparation pipelines with `concepts/preparation.rs`. Both public work types now retain their local clause, small scheduler metadata, and this same generic preparation owner. The sole MVD-root `source_index` module supplies the actual index type to population, request preparation, Direct, Terminal, and the concurrent Body implementation. No second catalog or whole-source clone was added.

## Contract and Original Custody

`IfcMvdRequest` extends the existing borrowed `IfcSourceIds` and repository-owned retirement contract with a bounded, immutable, allocation-free `valid_row(ordinal)` policy. The preparation owner admits the request count, retains one paid snapshot Arc alias before fallible constructor/posting, seals the original request Arc, builds its exact positive-u64/ordinal uniqueness index, and validates each closed domain row. Request validation precedes all source graph reads. It then moves those same snapshot/request owners into `IfcMvdPopulationWork::receive_donated`; it creates no second source alias at handoff.

`demands`, `check_resume`, `advance`, `ready`, `epoch` and `status` are scheduler interfaces. `ready()` is not source admission authority. The actual original caller, exact request/source addresses, source epoch, shape completion/validity, and stable population Box index govern `with_prepared`. `with_unavailable` lends only the actual unavailable request route. Neither route can be fabricated from a caller boolean.

Both HRTB callbacks receive a one-turn request-index view. Its opaque pin includes the actual request pin, completed row buffer identity, original caller, and, for the prepared route, actual population pin. `matches_prepared`/`matches_unavailable` correlate the request index to the exact borrowed route. Terminal checks this correlation on advance and take, retaining the same pin between turns; it cannot mix equal-content indices from separate preparations. Direct needs only the population view, while its wrapper still uses the common request-admission owner.

The shared owner retains request-index/continuation metadata before original population/source/request owners in retirement order. Clause metadata, queries, diagnostics and partial reports retire before preparation. Denied grants are stationary; cancellation refuses ordinary work but leaves paid original retirement available. Empty/invalid-row failure metadata posts its actual per-turn fixed metadata prefix once. Child index/population work posts its own accepted progress; the parent does not synthesize capacity or replay cumulative child progress. Constructor partial owners remain retained before fallible posts.

## Terminal Migration

`alignment/terminal-clause.rs` now holds only ordered-terminal continuation state, its once-per-source relation roster, owner-qualified attribute query, shared numeric/member cursors, diagnostic/report prefixes and opaque borrowed pins. It borrows source and source-index lookups from the prepared view and target-ordinal lookups from the qualified request-index view. The existing 38 business rows, 29 owner/query rows, official counterpart inputs, LIST ordering, all-member examination and late-unknown priority remain the contract. This is the selected terminal predicate, not a full Alignment verdict.

The new five-row `terminal-population-cases.json` exercises actual reference endpoint preparation, invalid global headers/IDs/references, retained non-applicable target rows and late unknown discovery. Native authored laws additionally move the wrapper between turns, compare two actual local clauses sharing one prepared index, reject mixed prepared/request-index routes before heap/receipt/metadata changes, and retain the first paid alias until same-alias donation or paid close.

## Evidence and Limits

- Test-first reference evidence: preparation 11 tests/27 assertions and terminal-population 6 tests/47 assertions passed before the native fold. The new neutral files are pending root's selected-registration addition at this checkpoint.
- Fresh focused actual reference run: 224 tests passed, 1,220 assertions, seven files (preparation, terminal-population, existing Alignment, owner query, Direct, Direct-population and shared population). Strict TypeScript check for the same graph exited 0.
- Independent `native-preparation-oracle.py` uses a repository-owned interface around SQLite 3.50.4. SQL primary-key/check constraints independently match all ten canonical request/count/uniqueness/closed-kind admission outcomes. This is reference-input evidence, not native-output comparison.
- Fresh existing independent terminal interface: IfcOpenShell 0.9.0.post1 matched 21 released metadata/official-input observations. It does not execute this native work or establish full physical complex-entity validity.
- Four new canonical native law functions are mounted by `preparation.rs`; existing Direct and Terminal laws and adjacent prepared-clause laws remain mounted. They cover all ten neutral requests, actual source/request index roles, six denied grant axes, source/request/epoch/caller substitutions, each cancellation prefix, exact heap/receipt comparisons, unavailable request-only access, and rejected-row receipt stationarity. These laws are authored, unexecuted.
- Syntax-only rustfmt parsing passed 14 owned Rust files. This does not establish Rust typing. Authentic Cargo `--lib native_mvd_canonical` exited 101 because configured repository compiler ingress refuses `-vV` before rustc. No compiler override was used.

Generated logs and the fresh source manifest remain under `🗑️generated`. The previous population10, Direct15 and Terminal11 audits are historical source checkpoints, not current hashes or native proof. The new manifest distinguishes owned frozen files from shared/current references (the MVD root and concurrent Body/common-type extraction).

Native compile, runtime, actual receipt/heap/scheduler execution, external native-output comparison, full Alignment/MVD conformance, public transport, mounting/capability and semantic publication remain unverified/refused. The next source integration is Body's local clause on this same canonical preparation API, owned separately by root; broader joint-request applicability or a full Alignment verdict is not inferred from successful standalone clauses.

## Independent Source Review

`native-mvd-preparation-independent-audit.md` closed this bounded source checkpoint: all21 owned frozen hashes matched, and seven shared references matched at the observation time. The review found no new confirmed P1/P2 in the checked admission, same-alias handoff, request-index route correlation, Terminal continuation and paid-close source. This is source-review evidence and does not remove any native proof gates above.
