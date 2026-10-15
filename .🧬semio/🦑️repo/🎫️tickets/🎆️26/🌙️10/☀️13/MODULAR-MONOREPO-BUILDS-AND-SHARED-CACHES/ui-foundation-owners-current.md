# UI Foundation Native Owners

Three real foundation crates now sit below the aggregate UI contract, with explicit Framework workspace members, canonical source projects, owner registry records and package Scripts:

| Package | Physical owner | Production body |
| --- | --- | --- |
| semio-framework-ui-contract-budget | contract/📏️budget | Schema-owned scalar capacities and shared document/page/value/tree-window formulas |
| semio-framework-ui-contract-text | contract/🔤️text | UiText, UiTextOriginalCopy, all inherent/conversion/formatting/Serde/Value implementations and bounded inline byte operations |
| semio-framework-ui-contract-identity | contract/🆔️identity | SurfaceId, UiNodeId, UiRevision, UiNodeIdAllocator and all their nominal derives/inherent/conversion implementations |

Budget is dependency-free in production. Text depends on budget, the canonical Value owner and existing Serde interoperability. Identity depends on text, Value/its derive owner and Serde. None depends on the aggregate UI package or a higher UI concrete type.

The aggregate contract composes these public owners directly. Document, builder, action and component now consume the same lower budget authority; action no longer reads builder's capacity, and builder no longer reads document's constant definition. UI_VALUE_PAGE_ROWS and UI_BUILT_CHILDREN_MAX derive from the shared page width. Document node/binding/patch counts and tree-window fixed/body budgets have one owner. Representation-sized arena/resident totals remain beside their real private representations; they were not fabricated as low-level constants.

UiText bytes and length remain private. The original pointer-bound copy cursor moved completely with the nominal type. Higher typed copy/compare/retirement visitors retain their existing local traits and concrete owner dependencies; their trait implementations are legal local-trait implementations over foundation types. The comparison inherent method moved into text, and bounded copy-prefix/retire-bytes methods replace the higher field mutations. No raw arena storage, handle, slot or mutex was exported. No forwarding module or duplicate foreign nominal Serde/Value/inherent implementation was added.

## Tests And Runtime

The schema and language-neutral fixture were authored before the native bodies moved. The actual original source-owner gate first failed on the absent native owners: one pass / one fail, 15 assertions. The independent SQLite arithmetic/UTF8 cases already passed in that RED run.

Final neutral prototype/release both pass two tests / 32 assertions (139 / 81 ms test runtime). They compare schema-owned capacities and fixture formulas with SQLite, UTF8 byte lengths with the host byte implementation, revision overflow with BigInt, physical package declarations with Bun TOML and independent @iarna/toml, and verify the higher nominal definitions/upward capacity edge were removed.

Ingress's same-Original Framework 31785 offline lock refresh and locked fetch passed after these declarations settled. This lane did not edit any Cargo lock file.

Actual native prototype 46612 reached the real compiler and exposed an ambiguous empty-array assertion in the new text test. The assertion was corrected to an emptiness predicate; no implementation, policy or capacity was widened to make it pass. Prototype successor 98856 passed all three native test binaries: budget one law, identity one law and text two laws. Total task runtime was 18.4 seconds; Cargo reported 2.05 seconds after the earlier failed compilation had already built some artifacts. This is not a cold-build measurement.

Actual native release 77840 passed the same four laws in 49.1 seconds total; Cargo reported 14.98 seconds for the optimized compilation. The real tests cover budget formulas, ASCII/empty/multibyte boundary admission and clipping, bare-string Serde/Value interoperability, exact original source refusal, one-byte copy/retirement grants, nominal identity codecs, monotonic allocation and u64 revision/allocator exhaustion. Both modes used the canonical issued compiler/toolchain/store route and unchanged source/capacity ceilings. No ambient Cargo writer, stub dependency or cap exception was used.

Focused ES2022 strict/noUncheckedIndexedAccess passed for the neutral gate and all three new package Scripts. The test and native commands, plus the strict command, are registered through the permanent ticket headroom Script/project and both editor launch catalogs. Public package source targets are authored; a new full ordinary public Nx discovery run was not used as this proof.

Physical production body sizes are 36 / 195 / 79 lines and 2499 / 8069 / 3351 bytes (budget / text / identity). These are physical Source measurements, not a native active-source census. Successful actual native compilation establishes that these selected owners passed the current real admission path; no aggregate UI admission result is inferred from those numbers.

## Remaining Frontier

Aggregate UI still contains the action value arena, component/document/builder/validation and their typed/built retirement SCC. It has not been requalified as a compiling aggregate. These three foundations are the completed checkpoint, not a claim that the giant UI contract is solved. The next architecture work must preserve owner-private arena/retirement semantics and move complete nominal implementations with their actual concrete prerequisites.

The separate priority remains fresh coherent SDK publication and ordinary Rust seven-phase qualification after Storage's detached daemon checkpoint. This work did not edit shared compiler/tooling/graph/Nx-patch runtime or activate an SDK. Foundation production Source and manifests are stable now; no further cuts are planned before that handoff.
