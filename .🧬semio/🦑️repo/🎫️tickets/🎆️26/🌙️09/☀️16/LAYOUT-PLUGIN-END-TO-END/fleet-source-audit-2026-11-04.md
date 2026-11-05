# Independent Build Source Closure Audit — 2026-11-04

Read-only audit of retained trace-build-source.cjs, generated fleet-build-source-trace-2026-11-04.json, and bounded actual imports. No builds, tests, production source edits or dependency operations.

## Actionable Ownership Finding

.storybook/📖️stories/🧭️coordination/🟦️.ts:20 imports only discoverPackages, readSemioMarkerSubTable and loadTaxonomy through the broad repo inventory barrel at 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts. Those functions are explicitly exported from repository-scan (barrel line19) and taxonomy-authority (line18). The same barrel also exports unrelated runtime actor verification (lines23-27), schema scope catalog (line33), dependency inventory (line37), schema field extraction (lines45-48), and normalization planning (lines52-53).

The common child caller line40 imports Storybook engines for descendant calculation; engines line18 imports scope coordination. Therefore a native build command admits the unrelated inventory exports even when its Storybook request filter ultimately returns no descendants. This is accidental executable coupling, not evidence that the Storybook engine lookup or discovery data can be discarded.

A narrow domain public entry containing only the three discovery exports (or existing exact public entries if available) would preserve all actual coordination behavior while ending the unrelated exports. Keep literal imports and normal companion/source admission; do not hide the graph behind computed import strings.

## Quantified Graph Check

The retained trace has1829files and17,172,879bytes. It contains exactly one importer of the broad inventory barrel: Storybook coordination. Counterfactual reachability from the same entry, using every retained edge, replacing only coordination-to-barrel with direct edges to existing repository-scan and taxonomy-authority implementations, makes181files unreachable. Their current physical sizes total2,122,362bytes. This suggests15,050,517bytes before any new narrow-entry overhead, comfortably below16MiB in this independent static trace. This calculation does not execute code, does not update the trace, and is not an authoritative admission receipt. Runtime data discovery remains additional to this trace.

Examples of removed unrelated paths include99,602byte runtime verification,85,330byte component-pass, and70,997byte schema-scope catalog implementation. The narrow edge preserves original package and taxonomy discovery and its transitive dependencies. No actual data needs deletion, minification, relabeling, copying, cap growth, or weakened Source observation.

## Other Largest Files

The largest traced387,204byte Bazel scopes data and293,383byte dependency storage implementation have explicit backend/bootstrap paths. Their size alone does not establish accidental ownership; removing them would require a separate backend dispatch contract. The Storybook inventory barrel offers a much narrower defensible reduction.

Sent the finding and quantified reachability to coordinator and layout_host_execution owner. Actual repair and authoritative normal admission remain with that owner.
