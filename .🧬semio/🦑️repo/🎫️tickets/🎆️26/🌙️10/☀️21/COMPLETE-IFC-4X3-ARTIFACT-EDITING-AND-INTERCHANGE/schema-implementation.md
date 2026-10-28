# IFC4x3 Schema Implementation

## Retained Normative Input

The exact [official IFC4X3_ADD2 EXPRESS schema](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp) is retained in the new standard's `🧬️schema/📚️catalog/IFC4X3_ADD2.exp`. The standards host required a Cloudflare browser challenge, so the bytes were obtained from the [buildingSMART Standards-builds official mirror](https://raw.githubusercontent.com/buildingSMART/Standards-builds/main/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp). SHA-256: `f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860`. The schema identifier is exactly `IFC4X3_ADD2`, version 4.3.2.0, published as ISO 16739-1:2024 according to the [official specifications database](https://technical.buildingsmart.org/standards/ifc/ifc-schema-specifications/).

The retained input and derived catalog contain all 876 entity declarations, 436 named types, 48 FUNCTION declarations and 2 global RULE declarations. Entity records include abstractness, single supertype, direct attributes, flattened inherited attribute order, derived redeclarations, inverse declarations and WHERE/UNIQUE source. Type expressions preserve nested aggregates, literal and contextual bound expressions, optional/unique flags, primitive widths, enums and selects. Functions and global rules are indexed by name with their full declaration body. No source was copied into the ticket. The reproducible input script `schema-build.py` stays in this ticket; its production outputs are catalog JSON and dependency-free Rust static initialization data.

## Implemented Scope

| Capability | TypeScript | Rust | Verification |
| --- | --- | --- | --- |
| Complete declaration lookup and subtype resolution | Implemented | Implemented | All entity inheritance/abstractness/attribute order/optional/derived positions compared with IfcOpenShell |
| Enum/select member catalogs | Implemented | Implemented | All members compared with IfcOpenShell |
| Required and derived markers, positional arity | Implemented | Implemented | Language-neutral cases and focused tests |
| Named types, entity reference target type, nested selects | Implemented | Implemented | Valid property SELECT and incorrect reference tests |
| Literal aggregate bounds and uniqueness | Implemented | Implemented | Empty coordinate and unit assignment cases |
| Primitive domains and fixed STRING widths | Implemented | Implemented | REAL values remain typed Binary64 in TypeScript |
| BINARY | Implemented in TS validator and transport schema | Implemented in shared graph, codecs, SQLite and catalog; compilation unverified | See binary-implementation.md for 23 passing TS persistence tests and Rust syntax evidence |
| WHERE/UNIQUE entity rules, inverse rules | Retained; not evaluated | Retained; not evaluated | Explicit capability flags false |
| FUNCTION/global RULE execution | Retained; not evaluated | Retained; not evaluated | 48 functions/2 rules indexed, capability flags false |
| Context-dependent bounds | Retained; not evaluated | Retained; not evaluated | Capability flag false |
| Complex instances | Retained in graph; validator rejects unsupported validation | Same | No false claim of conformance |
| MVD constraints | Not implemented here | Not implemented here | See status below |

The JSON schema `🧬️schema/🔣️.json` describes the actual recursive tagged graph transport, not the former text-only schema. Native `bigint` identifiers/integers and Binary64 `bits` are transported as decimal strings; runtime bounds still require core validation. Its `$defs/IfcValue`, `IfcEntity`, `IfcHeader` and `IfcSnapshot` are suitable references for the new diff/mutation transport schema.

Public TS exports: `ifc4x3Catalog`, `ifcAttributeDeclarations`, `ifcIsSubtype`, `validateIfc4x3Snapshot`, declaration interfaces and diagnostics. Public Rust exports: `ifc4x3_catalog`, `ifc_attribute_declarations`, `ifc_is_subtype`, `validate_ifc4x3_snapshot`, own public declaration structs. Rust uses std-only static data plus the existing IFC graph type; no new runtime parser or library dependency. Diagnostics include a stable code, message, entity id and recursive argument path.

## Validation Evidence

- `🧪️tests/schema.feature` and `schema-cases.json` were written before validator implementation. They cover coordinates, cardinality, primitive mismatch, valid references, dangling references, wrong referenced subtype, abstract/unknown entities and duplicate ids.
- `bun test .../🧪️tests/schema.test.ts`: **11 tests, 22 assertions, all passed** after implementation. Additional checks cover SI unit inherited derived Dimensions and enum membership, property SELECT typed values and catalog counts.
- `schema-oracle.py`, using **IfcOpenShell 0.9.0.post1**, completed **4,693 independent checks with zero failures**: source checksum; entity count; every entity's abstractness, supertype, all attribute names/order/optionality/derived positions; every enum/select member; and seven shared structural snapshot acceptance/rejection cases validated with IfcOpenShell's own schema validator (`express_rules=False`). Unknown/duplicate parser cases are excluded from the seven acceptance comparisons; own tests cover them. The independent oracle exposed INTEGER tokens supplied to REAL attributes; fixtures and validators now preserve and require the REAL domain rather than silently widening the token type.
- The oracle does **not** claim exhaustive differential validation of every nested defined type expression or semantic rule. These remain additional work.
- `repo test` invoked with the standard path and package identity returned no test output. The cached `semio-repo.exe --help` has no general execution route; required source operations and explicit Bun/Python checks used necessary direct tools. No modifying Git operation or worktree was used.
- Full Rust crate checks remain **unverified**: a direct Cargo fallback was stopped before the compiler version probe by the repository's enforcement script: `Original compiler ingress requires the repository launcher and authentic enforcing store`. Enforcement was not bypassed. Existing workspace warnings were unrelated to the catalog.

IfcOpenShell was installed temporarily under ticket `🗑️generated/oracle`; the Windows native package cannot resolve Unicode paths through its codepage-dependent loader, so the test input script selects an existing Windows 8.3 path via `GetShortPathNameW`, without copying anything outside the ticket. Parent cleanup must remove the ticket generated folder after all sibling oracle work is complete. The audit evidence remains in this Markdown report.

## MVD Status and Remaining Full-Spec Work

The [IFC4.3 documentation introduction](https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/content/introduction.html) describes three implementation levels: Reference View (RV), Alignment Based View (AbV) and Design Transfer View (DTV). The [official 2025 AbV project publication notice](https://www.buildingsmart.org/ifc-4-3-alignment-based-view-definition-1-0-final-project-plan-is-now-is-now-out-for-review-by-the-standards-committee/) states RV is already final, AbV was submitted for final publication, and DTV had no significant market demand. The [official consultation record](https://www.buildingsmart.org/standards/standards-committee-consultation/) still exposes that historical review entry. The [release concept index](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/concepts/content.html) includes RV and Alignment-basedReferenceView tags; current development docs rename the latter Alignment-basedView.

The authoritative [MVD database](https://technical.buildingsmart.org/standards/ifc/mvd/mvd-database/) returned a verification challenge during this audit, so the **current final approval status and exact machine-readable deliverable URLs have not been independently verified here**. Do not infer present status from the 2025 notice or install IFC4 RV1.2 constraints as though they alone were complete IFC4x3 constraints. No invented IFC4x3 COBie/Coordination View profile was registered.

The user's full-spec goal therefore requires further implementation and independent tests for all EXPRESS WHERE/UNIQUE/inverse/contextual expressions/functions/global rules, correct complex instance validation, complete BINARY Rust codec support, exhaustive primitive/defined/aggregate type differential cases, and actual published MVD concept/constraint evaluation. The structural validator is deliberately separate from editing so an incomplete model can still be repaired. Large/cyclic invalid native input is guarded by depth limits, but aggregate uniqueness currently uses pairwise comparison and TS JSON serialization; cancellation/performance hardening remains needed for large analysis work. This report does not represent full IFC4x3 semantic conformance.
