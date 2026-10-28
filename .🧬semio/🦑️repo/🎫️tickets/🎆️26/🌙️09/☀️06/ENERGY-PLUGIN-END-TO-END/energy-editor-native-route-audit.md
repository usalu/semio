# Independent Native Editor Route Audit

## Identity Chain

The new route uses the authored `EnergyModelMutation::DESCRIPTORS` as its source. Each native descriptor's `aggregate_variant` is appended to both the retained tool roster and the artifact-document tool roster. The same retained roster drives manifest interactive-job migration rows, `OpBinary::TOOL_JOB_IDS`, factory keys, and bounded first-step proof rows naming the exact `EnergyModelCommandJobFactory` type.

Native action forms use that same aggregate variant as action ID. The args bridge maps it back to the descriptor's semantic kind and serializes the actual submitted argument object. `ApplyNativeMutation::action_id` derives the matching aggregate action ID again; factory admission rejects tool/command identity mismatches. The reducer parses the object and calls the actual aggregate mutation's `from_payload_value`, emitting a native mutation rather than rewriting the snapshot. The existing central diff and semantic inverse paths therefore remain the publication mechanism.

## Remaining Concerns Reported to Editor Lane

- The retained extent guard counts only zones, spaces, surfaces, fenestrations, materials, and constructions, while the new full vocabulary includes all other collections and potentially large schedule/time-series/nested-array payloads. Returning one work item after this limited census does not prove the retained step bound for the whole vocabulary. A complete authored extent bound or incremental retained preparation is needed.
- Native display names are currently used for both English and German. This publishes localized slots but does not provide translated German labels.
- The native args bridge preserves numeric strings, unlike the explicit authored bridge's numeric coercion. Actual schema-derived control submission should be tested to ensure numeric/reference inputs emit the native numeric values the mutation payload expects.
- Form construction zips descriptors and schema rows. The new roster-count regression detects truncation on the current vocabulary, but registration completeness still requires actual declaration/dispatch execution rather than source presence alone.

No concrete mismatch was found in the source-level exact action/proof/factory identity chain. Native catalog form and semantic inverse tests are authored by the editor lane. This audit does not claim those tests executed or that every form rendered successfully.
