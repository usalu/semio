# Independent IDF Registration Audit

## Critical Declaration Gap

At audit time, energy model `definition()` declared native/ZIP/JSON composers but no IDF composer. Its `io_registry::entries()` now contains an IDF export entry writing `s.stdio.idf@25.2/*`.

The framework artifact declaration builder's `composers_async` requires every entry's written dialect to have a declared composer capability (`require_declared_capability_or_record`). Consequently the missing declaration can reject the whole Energy artifact during `try_build()`, rather than merely hiding IDF. Reported to the IDF lane for correction; no concurrent IDF source changes made by this audit.

## Confirmed Static Wiring

- Stdio IDF format identity is `s.stdio.idf`, standard `25.2`, profile any. Both energy import dependency and export composer use the exact same `25.2/*` coordinate.
- Format representation publishes `.idf`, not the native `.energy` extension. Shared format descriptor generation derives short ID `idf` from the first extension.
- Stdio format contribution uses `definition_only_assembly` and intentionally has no generic document editor. This is appropriate: energy owns conversion into/from the energy artifact.
- Deployment-generated `🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/🧩️composition/🦀️.rs` includes `semio_s_artifact_stdio_idf::contribution()` in its full selected roster. The corresponding deployment Cargo dependencies exist.
- Energy native declaration passes `io_registry::entries()` to `.composers(...)`, making the import/export table the right selector/registry path after the missing capability is corrected.
- Host export-extension resolution uses the selected format descriptor's first extension, so this descriptor resolves `.idf` when requested through the format short ID `idf`.

## Verification Limits

Static registration evidence is necessary but not a rendered selector or a successful plugin assembly. Full Energy declaration compilation/registration still depends on the shared OS store/replay repair. A runtime test should assert declaration succeeds, IDF appears in import/export IO keys, and export filename uses `.idf`.

## Mutation Regression Failure-Before Analysis

The previous production row applier spliced inserted rows directly into the collection and had no uniqueness check. Thus both fixture cases with duplicate surviving or inserted keys would previously have returned success; their new expected refusals are specific to the correction. Uniqueness checks operate within each keyed collection only; sharing the same numeric ID across different collections is unaffected. Removing a key before reintroducing it remains legal and is explicitly exercised by the native inverse matrix.

The previous slot absorber binary-searched an unchecked `[index 2, index 0]` vector. Absorbing index 0 inserted another index-0 entry before that unsorted vector, leaving the old index-0 value last; the final result was 10 instead of expected 30. The fixture regression is specific to a wire-accepted order and later-write semantics. The native production-source harness passes the corrected behavior and all legal replacement/inverse cases. Baseline failure was established by direct source/algorithm inspection, not by temporarily reverting shared production files.

## Declaration Correction Revalidated

The IDF lane added `s.energy.model.composer.idf` with the matching `s.stdio.idf@25.2/*` dialect claim. Read-only inspection confirms the capability is now present in energy model `definition()`.

## Host Export Route Still Needs Proof

The host's older `registry_export_media_via_composer_registry` path requires the selected export entry to advertise a read dialect with standard `rfc8259`, then passes source-document JSON through that bridge. The energy export entries, including IDF, advertise only `MODEL_DIALECT`; their internal `rebuild_native_snapshot` contains a JSON bridge branch, but that branch is not advertised in the entry. Thus the legacy host path cannot select a source bridge for these entries. The newer host IO-mechanism path may avoid that dependency, but the current energy declaration inspected here registers composers rather than an explicit new IO route. This remains an integration concern requiring a real host export test or a proper route implementation; converter/schema tests alone do not prove the user's file export works. Reported to parent and IDF lane without editing concurrent files or adding compatibility bridges.

## Canonical JSON Source Advertising Review

Advertising the already-supported canonical JSON source on the energy export entries resolves the host's missing-source-selection branch locally. However, advertising JSON as a read on the energy JSON writer creates a global self-composition key collision: `s.stdio.json@rfc8259/*` to itself is already owned by Stdio's `JsonRawAnyComposer`. `composer_entries_by_key` generates Import and Export keys for every read/write pair and rejects differing compose functions for the same key. Reported to coordinator before proceeding to native simulation restoration. A presence-only read-dialect test is insufficient to establish cross-plugin registry preflight.
