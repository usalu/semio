# Native Borrowed Attribute Query Implementation

The native query and ADD2 adapter are source-complete for the existing shared attribute-query contract. Tessellation now consumes the same owner-qualified query in native and TypeScript implementations. This is metadata/slot query work, not value/domain validation or publication admission. Native execution remains unproven.

## Contract and ownership

The shared native implementation lives beside the existing TypeScript query and neutral laws. `IfcAttributeQueryPopulation` borrows actual source parts and values; `IfcAttributeQueryMetadata` borrows released static declarations/type expressions through bounded immutable accessors. ADD2 implements these interfaces against the actual `IfcSnapshot` and retained metadata; it does not prepare or duplicate the owned catalog.

`IfcAttributeQueryWork` retains fixed metadata only: normalized request words, exact original caller binding, source pointer/type/epoch, actual source row, exact request pointer/length identities, exact metadata profile identity, and at most two declaration/slot addresses. The enclosing original owner must keep the population immutable and alive through retirement. No source pointers are dereferenced unsafely. The epoch is a caller source-continuation epoch, not a canonical document hash or publication proof.

Each paid progression handles a name byte, constituent frontier, immutable metadata lookup, or attribute decision. Inputs use the shared 256-byte ASCII identifier grammar. Ordinary denial leaves state unchanged; resumed source/request/profile/caller substitution refuses before debit. There is no query heap allocation, eager value clone, catalog construction, or mutation. Fixed metadata demand is a conservative bound; physical native receipt precision has not been executed or measured.

The completed result borrows original values, exact static attribute/type-expression references and actual source ordinal/part/argument addresses. Unique declarations with absent serialized values remain unique with `None`. Missing, unique, ambiguous and unknown remain distinct. Every remaining constituent is looked up after the first two matches, so late unknown constituents dominate ambiguity. Result access is bounded and read-only, validates original source/request/context pins and cancellation, and does not grant value admission.

Cancellation refuses ordinary advance/result. Explicit close uses the original retirement ladder and permits paid close after cancellation/deadline. The cursor contains no heap/source owner; the enclosing retained owner keeps the real source lease and retires it after query metadata.

## Consumer

Native Tessellation replaces its duplicated positional scan with the shared query, using the actual source index and exact declaring owner `IFCREPRESENTATION` for RepresentationIdentifier, RepresentationType and Items. TypeScript Tessellation uses the existing shared query with the same owner. Unknown/ambiguous/missing preserve incomplete reports and explicit English/German diagnostics. A complex primary with flattened inherited arguments cannot substitute for declared constituent ownership. Alignment/direct-* and the common membership cursor were not changed.

## Laws and evidence

The native shared test module is mounted through the actual ADD2 catalog adapter. It consumes all ten shared neutral query cases at actual source ordinal 7, checks exact borrowed declaration/type/value identity, absent-value uniqueness, owner qualification and late unknown status. Additional authored native laws cover every-stage item/copy/depth/fuel denial, source/request/row/epoch/profile/caller substitution, invalid identifiers/ordinal, cancellation at every prefix including completion, and original paid close. Native Tessellation consumes four additional language-neutral consumer cases under its actual original work/take/report/source-close path.

The consumer cases and closed schema were authored before production changes. The actual repository selected test entry includes the new TypeScript consumer suite.

- Focused existing query, Tessellation and shared original-control suites: **86 passed, 455 assertions, 0 failures, four files**. Log: `🗑️generated/native-query-ts-tests.txt`.
- Strict TypeScript checking of the three query/Tessellation test roots and transitive source graph: **exit 0**. Log: `🗑️generated/native-query-ts-types.txt`.
- Independent repository-owned `MetadataOracle` test interface with **IfcOpenShell 0.9.0.post1**: all **10 neutral query outputs + 3 exact IFCREPRESENTATION owner/ordinal declarations matched**. Script: `native-attribute-query-oracle.py`; output: `🗑️generated/native-attribute-query/oracle.json`. This validates independent declaration metadata and slot-query outputs; malformed/duplicate complex fixtures are metadata-query laws, not claims that those are physically valid full IFC populations.
- Rustfmt syntax parsing: **seven native modules exit 0**; emitted source suppressed. Metadata-only log: `🗑️generated/native-attribute-query-syntax.json`.
- Authentic Cargo test selection: **exit 101 before rustc**, enforcing compiler `-vV` refusal requiring repository launcher/authentic enforcing store. Log: `🗑️generated/native-attribute-query-cargo.txt`. No native typecheck, test execution, heap measurement, runtime receipts or output evidence is inferred from syntax parsing.

## Source checkpoint

The fourteen-file SHA256 roster is `🗑️generated/native-attribute-query-source.json`. All paths are within the 256-character bound. Source files were not copied into the ticket.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🧪️tests.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/query.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/helpers.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🧪️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/types.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/query-cases.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/query.schema.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/query.test.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts`

No build/enforcement code, public transport, semantic gate, native capability, Store or publication path changed. Native/compiler/mount and semantic publication flags remain false.

Independent read-only review is recorded in `native-attribute-query-independent-audit.md`: no confirmed new P1/P2 source defect. All thirteen owned source hashes matched. The shared TypeScript package registration changed concurrently for root's additional control suite, so this is not a fourteen-of-fourteen unchanged claim. Source review cannot replace the missing genuine native execution.

