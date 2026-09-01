# Mutation Ticket Output and Explicit Authorship 64

## Scope

Read-only option-flow audit. No source admission, CLI inventory, ticket traversal, receipt read, or filesystem census was run.

## Observed Current Flow

There is one overloaded `ticketDir` field at every relevant boundary:

| Boundary | Current meaning passed onward |
| --- | --- |
| [TaxonomyCliOptions](/Users/ueli/Documents/semio/📜️script.ts:19979) | CLI `--ticket` is an identifier, resolved to one ticket directory. |
| [CleanScript.runTaxonomy](/Users/ueli/Documents/semio/📜️script.ts:21173) | `taxonomyCliTicket(this.root, options.ticket)` creates `ticketDir`; mutation operations call `runMutationTaxonomyCli(..., ticketDir, ...)`. |
| [runMutationTaxonomyCli](/Users/ueli/Documents/semio/📜️script.ts:21058) | uses the same `ticketDir` both for `inventoryOptions.ticketDir` and for `📊️taxonomy-*` output writes; it also derives the optional assignment-ledger path under that directory. |
| [MutationTaxonomyInventoryOptions](/Users/ueli/Documents/semio/📜️script.ts:20630) | exposes only `ticketDir?: string`; no output/admission distinction exists. |
| [mutationTaxonomySourceAdmission](/Users/ueli/Documents/semio/📜️script.ts:20751) | forwards `options.ticketDir` unchanged to shared `inventoryTaxonomySources`. |
| [TaxonomyInventoryOptions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:273) | also exposes only `ticketDir?: string`. |
| [sourceAdmissionPrepareOptions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2746) | resolves/validates that field as `prepared.ticketDir`. |
| [collectTaxonomySourceAdmission](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2861) | walks `prepared.ticketDir` and adds every eligible result with origin `explicit-ticket` at `:2892–2894`. |

The normal CLI follows the same shape: [taxonomyCliInventoryOptions](/Users/ueli/Documents/semio/📜️script.ts:20516) forwards its output `ticketDir` into shared `inventoryTaxonomy`.

Therefore the current root mutation CLI conflates two distinct authorities:

1. `--ticket` as an output destination for generated inventory/plan/verify/apply JSON and Markdown artifacts; and
2. `ticketDir` as an explicit authored-source root for source admission.

There is no guard which distinguishes a generated `📊️taxonomy-*` or `📓️taxonomy-*` output from an authored explicit-ticket source. A first inventory call writes after it has collected admission, so it does not necessarily admit its newly written receipt during that same call. However, a later invocation with the same CLI ticket can walk already-present generated outputs and mark them `explicit-ticket`; controller/log writes already present before admission have the same exposure.

## Existing Safeguards, and Their Limit

The shared collector is not unsafe in the path sense:

- `sourceAdmissionPrepareOptions` validates lexically, checks repository fences, and rejects unsafe paths before its walker;
- `sourceAdmissionWalk` applies opaque/exclusion/scope checks, no-follow observation, cancellation, nested-repository fencing, nested `.git` stopping, and directory identity rechecks;
- source admission preserves `explicit-ticket` as a legitimate declared origin in both its TypeScript union and canonical normalizer schema.

Those safeguards govern *how* an explicit ticket path is read. They do not distinguish the ticket’s authored input from the root CLI’s own generated output role. The shared abstraction is correctly capable of explicit-ticket authorship; the root caller is supplying an output directory to that input parameter.

## Precise Future Neutral Regression

Use a single mocked/captured admission fixture with two separately named paths; do not add a root list or generic skip list.

1. **Output-only ticket.** Give the root mutation CLI an output ticket containing pre-existing `📊️taxonomy-inventory/🔣️.json`, `📓️taxonomy-inventory/📝️.md`, a controller receipt, and a log. Its source-admission request carries no explicit authored-ticket input. Expected: none of those paths appears in admitted observations with `explicit-ticket`, source capture, source roster, coverage, or digest input; the command may still write its output artifacts under the output ticket after inventory completion.
2. **Explicit authored ticket.** Separately supply an explicit authored-ticket input with a small regular source file and schema/document evidence, plus an independent output destination. Expected: the authored file is observed once with origin `explicit-ticket`, obeys scope/opaque/no-follow rules, and may enter the source capture; generated output destination files remain absent from admission.
3. **Same physical ticket, distinct roles.** Where a command intentionally needs both, supply the two role fields explicitly. Expected: only the declared authored sub-input is admitted; output artifact paths are not reclassified as authored merely because they share a ticket ancestor.
4. **Negative safety controls.** A compose-segment, escaping, symlink-ancestor, or repository-fence ticket input rejects before traversal; this preserves existing shared source-admission safety rather than treating output filtering as an exemption.

The fixture must assert origins and captured-roster/digest membership, not only that files happen not to be parsed. It should run against a direct captured source-admission input and a root mutation-CLI output writer separately, because one validates authorship selection and the other validates output routing.

## Boundary

This report identifies a real option-flow conflation; it does not select names for split fields or authorize an API change. It does not propose another membership authority, new roots, or a global skip list. Explicit-ticket authorship remains valid; generated ticket evidence/output must not gain that origin merely through the root CLI’s output destination.
