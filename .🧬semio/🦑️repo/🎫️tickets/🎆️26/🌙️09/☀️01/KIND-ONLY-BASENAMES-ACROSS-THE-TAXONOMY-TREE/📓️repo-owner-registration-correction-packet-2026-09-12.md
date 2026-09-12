# Repository Owner Registration Correction

Terra’s independent repository audit accepts the 13 coordinator/VSCode contextual additions but found missing ancestor authority for two moved sources from the completed 22-source batch:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟦️.d.mts`

The live semantic member resolver returns null for the library, dependencies and runtime member names in their selected contexts. Terra observed the narrow member chain as library under `members-of-modules`, dependencies under `members-of-members-of-modules`, and runtime under `members-of-members-of-members-of-modules`. Re-read the actual current resolver and member registry before editing; these context strings are audit evidence, not permission to special-case whole paths or loosen generic membership.

The original Sol repository executor, currently working on script-policy precision, owns this small direct correction. Register the three exact semantic member identities in their real contexts, preserving existing library/package rules and shared edits. Extend the portable repository source-topology contract so it verifies both real source owner paths and all ancestor resolutions. The earlier 6/212 check omitted these chains and therefore passed despite the gap. Run the focused registered route, strict validation and scoped actual inventory/resolution. Do not pull every ambient normalization directory into this small repair; the root’s earlier normalization census and broader contextual packet retain those cases separately.

Retain exact changed paths and red/green resolver/native evidence in `📓️sol-repo-owner-registration-correction-2026-09-12.md` or an explicit standalone addendum referenced from the source report. Own scratch only ticket `🗑️generated/sol-repo-owner-registration`. No source compatibility exports, blanket library/dependencies/runtime admission, modifying Git, AGENTS or lifecycle changes. Terra should retain the finding as pending until the completed correction evidence is available.
