# Entity-Only SELECT Required Inputs

The actual source-bound type-input producer now supports complete, bounded entity-only SELECT declarations through the existing exact Reference input. Every declared member must resolve to an entity; mixed, nested, aggregate, empty or oversized unsupported domains refuse as a whole. Source entity identity and released subtype membership determine the selected member. The command carries an ordinary exact `reference`, without a fabricated typed entity constructor or an initial ID selection.

The released census finds 32 required direct occurrences across 24 entity-only SELECT declarations. The concrete neutral fixture exercises `IfcMeasureWithUnit.UnitComponent : IfcUnit` and the inherited/complex `IfcActor.TheActor : IfcActorSelect`. It includes actual `IfcSIUnit` subtype membership, three actor alternatives, nonmembers, dangling IDs and blank/partial/overflow drafts. The actor candidate includes the inherited object/root constituents and follows the existing complete-candidate structural and issued local-rule checks.

## Executed TypeScript Evidence

Schema, feature and closed source/case vectors preceded production. The new actual consumer initially produced 2 passes and 14 failures because entity SELECT input controls were unavailable. After the bounded kind classification change, the same suite passes 16 tests with 5630 assertions. Narrow strict TypeScript checking exits 0. The four actual reference SELECT, scalar SELECT, basic required-input and LIST consumer suites pass 64 tests with 15025 assertions and no failures in 111.89 seconds.

The external IfcOpenShell 0.9 oracle independently serializes the supplied complete values, resolves the complex inherited primary using its own schema and runs full EXPRESS validation. All nine physical admitted/refused membership outcomes agree; three incomplete drafts remain unserialized. Valid output retains exact reference IDs and source entity kinds. This physical oracle supports those fixture outputs, not native execution or universal SELECT support.

The new suite `type_reference_select_input.test.ts` remains unregistered until independent review. Parent owns registration. The current TypeScript source is held for the root's selected 127-suite snapshot; this report does not report that still-running snapshot as successful.

## Authored Native Source

The existing retained kind cursor now visits each declared member under its original turn. It remembers whether the complete declaration contains entities or supported scalar aliases, rejects a mixed domain, and returns the existing Reference kind only after all entity members are visited. It borrows static released metadata, creates no index/cache or new authority, and introduces no new profile hook. The previously audited profile pins already cover SELECT/alias hooks and the separate direct-count binding.

The original donated producer/controller, typed source loan, exact raw draft binding, strict private candidate, central `SetEntityArg`, diff and inverse paths are reused. Native tests consume the same language-neutral vectors and author central output/inverse, pure-demand/zero-heap, denied-axis/zero-fuel stationarity, missing/partial editability, cancellation and paid retirement laws. Adjacent controller tests author 24 missing/supplied modes through the actual original render hook and require source Arc retirement. These tests are mounted in the existing native source owners.

The new coherent native roster contains 64 source paths, including `constituent_roster.rs`, and 43 Rust files. All 43 parse with rustfmt; maximum absolute path length is 146 UTF-16 code units. Authentic Cargo exits 101 at the enforcing compiler version ingress before rustc. Native type checking, execution, physical allocation/copy sufficiency, receipts and cancellation/close laws remain unproved. No capability, readiness shortcut, public transport or publication flag is enabled.

## Exact Checkpoint

The distinct generated rosters are `type-reference-select-source.json` (five TypeScript/neutral paths) and `native-type-reference-select-source.json` (64 paths). Historical scalar SELECT58 and its initial pre-correction57 evidence remain unchanged. The input script `type-reference-select-checkpoint.py` records current hashes without rewriting those historical manifests.

Evidence lives under `🗑️generated`: `type-reference-select-input-red.txt`, `type-reference-select-input-tests.txt`, `type-reference-select-input-strict.txt`, `type-reference-select-consumers.txt`, `type-reference-select-census.json`, `type-reference-select-oracle.json`, `native-reference-select-syntax-*.txt` and `native-type-reference-select-cargo.txt`.

## Independent Closure

The subsequent independent report `type-reference-select-independent-audit.md` closes this bounded source checkpoint. The auditor matches all five TypeScript and 64 native hashes, independently replays the actual 16-test/5630-assertion consumer suite without failures, and finds no additional confirmed P1/P2. Native review remains source-only. The audit releases its source hold; the parent's broader selected-127 reader hold remains independently active. The new test is still unregistered at this report update, pending parent registration.
