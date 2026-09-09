# Schema Policy Owner Discovery Audit

The generic artifact schema policy in the root script still enumerates artifact roots and appends `🧬️schema`, `📸️snapshot/🧬️schema`, and `🔺️diff/🧬️schema`. Authored contracts actually live under each standard/subset, with snapshot and diff nested inside `🧬️schema`. Consequently, the generic policy reports misplaced completeness failures and skips real parity/state checks. The ticket's separate 156-facet field parity audit uses the correct locations and remains the current evidence source.

The policy family also requires an `artifact` whole-replacement field for every diff, even though Rewriting's native diff contract does not expose one. This generic requirement is semantically incorrect: a diff vocabulary belongs to the owning schema and must agree across representations, while whole replacement is a domain-specific supported mutation choice. The diff checker should validate coverage against the declared contract without inventing operations.

Existing `policyListTopLevelSubsetDirs` supplies plugin standard/subset owners. It can replace artifact-root enumeration for this policy family. Framework-owned artifacts also need discovery rather than a plugin-only hard-coded roster. A regression fixture must exercise multiple standards and subsets, a concrete misplaced artifact-root schema, and snapshot/diff children under the canonical schema owner. Existing parity extraction has independent TypeScript AST and Ajv oracles.

This report is a read-only finding. No policy discovery implementation or passing regression is claimed yet.

## Correction and Validation

The root policy now discovers artifact standard/subset owners across both framework products and plugins, then checks canonical `🧬️schema`, `🧬️schema/📸️snapshot`, and `🧬️schema/🔺️diff` facets. It no longer invents a required whole-artifact replacement operation for every diff. Discovery is shared across the five validators in each call.

`artifact-schema-owner-red-1.log` reproduced the misplaced Writer artifact-root scope. `artifact-schema-owner-green-1.log` passed in 19.6 seconds: all 192 discovered owners agreed with an independent fast-glob directory oracle, all reported schema scopes belong to those owners, and no generic replacement operation was invented. Existing AST/Ajv field-extractor checks also passed. This establishes correct inspection scope, not that all 192 schemas are free of other violations.
