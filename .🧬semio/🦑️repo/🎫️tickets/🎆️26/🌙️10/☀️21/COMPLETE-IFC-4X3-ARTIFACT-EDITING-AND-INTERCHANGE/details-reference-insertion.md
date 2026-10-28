# Exact Reference Collection Insertion

The IFC4x3 details provider now exposes a localized `appendReference` input on editable reference aggregates. It accepts an exact positive decimal u64 entity ID through the existing revisioned text input, SetValue action and retained snapshot edit authority. The editor resolves that virtual view address to InsertValue on the actual aggregate; the existing edit rules yield one SetEntityArg or SetEntityComplex mutation and central diff application remains the publication authority. The virtual field never enters the persisted IFC graph.

This permits insertion after all candidates in the 256-entity suggestion window are already used, and reaches targets beyond that window. Admission checks the declared entity/select type, actual target existence, collection capacity, ARRAY/derived restrictions, and SET/UNIQUE duplication before resolving the mutation. Existing bounded suggestions remain cheap. The input and its capabilities disappear where append is not permitted.

## Verification

- Added the language-neutral scenario and `tests.json` reference-insertion fixture before implementation.
- Wrote a native host-route regression using an IfcPropertySet with 256 existing properties and 300 available targets: localized editable input, target #301, one sparse argument mutation, central forward/inverse equality, duplicate/wrong-type/missing/zero/invalid/out-of-u64 refusal, and exact target #9007199254740993.
- Native regression and changed editor/provider parse and format successfully with rustfmt. Authentic compiler ingress is still unavailable, so this does not claim Rust execution or browser end-to-end verification.
- Extended retained `details-oracle.py`: IfcOpenShell 0.9.0.post1 independently creates the same 301-entity property graph and appends target #301. It reports 257 property references and **zero structural or EXPRESS-rule errors** with `express_rules=True`. The same run verifies five official inherited attribute controls and three compressed GlobalId vectors.
- The oracle wheel requires Python 3.13; it runs with `py -3.13` on this Windows host and an existing short path into ticket generated dependencies. Initial attempts with the bundled Python 3.12 and PATH Python 3.14 could not load the cp313 wheel; these were dependency ABI mismatches, not executed conformance checks.

## Files and Limits

Changed only IFC4x3 editor, provider, written tests/neutral fixtures and retained ticket oracle/report inputs. No generic framework UI API, build infrastructure, AGENTS file or Git state was changed.

Actual native UI event execution, asynchronous large-graph reference indexing/search, retained source IO cancellation, complete complex semantic validation and all specialized IFC geometry interactions remain open work. This input removes a known reachability limit without claiming those broader capabilities.
