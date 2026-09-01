# Mutation Ticket Role Split 64

## Scope

Schema-first mutation-only caller design. Current root S and shared N source were inspected, but neither was written. No inventory, Git enumeration, filesystem census, Compose path, or native test was run.

This follows [Mutation Ticket Output and Explicit Authorship 64](./mutation-ticket-output-vs-explicit-authorship-64.md). The problem is a root caller role conflation; the shared normalizer remains the sole source-membership authority.

The current-source sentinel probe captured root S at `fdb34f8e4a9d1696915dc18d804876ed80a1f46c6d09d365f92b24914c5a991d` and shared N at `2e6966bff543b330451de80cfd9ba088081b790d88b66d84f7b9b2c714a740b1`; all three owned S slices were stable before/after. Its final schema/vector/controller hashes are retained in [run-PUOuo3](../🧫️role-result-PUOuo3/receipt.json); the raw child observation is retained in [child probe output](../🧫️role-probe-LYiBDQ/child.stdout.log). The first provisional CLI-flag/overlap proposal is superseded below and was not selected for implementation.

## Current Boundary

The mutation root currently has one role-bearing value:

| Current boundary | Current field | Present effect |
| --- | --- | --- |
| [TaxonomyCliOptions](/Users/ueli/Documents/semio/📜️script.ts:19979) and [taxonomyCliOptions](/Users/ueli/Documents/semio/📜️script.ts:19993) | `ticket` / `--ticket` | Resolves one ticket directory. |
| [CleanScript.runTaxonomy](/Users/ueli/Documents/semio/📜️script.ts:21168) | `ticketDir` | Passes the resolved directory to the mutation runner and retains it as the normal CLI output destination. |
| [runMutationTaxonomyCli](/Users/ueli/Documents/semio/📜️script.ts:21058) | `ticketDir` | Builds mutation inventory input, assignment-ledger default, and output artifact locations from one value. |
| [MutationTaxonomyInventoryOptions](/Users/ueli/Documents/semio/📜️script.ts:20630) | `ticketDir` | Represents an admission input despite its caller supplying an output directory. |
| [mutationTaxonomySourceAdmission](/Users/ueli/Documents/semio/📜️script.ts:20751) | `options.ticketDir` | Forwards that value to `inventoryTaxonomySources`, where shared N correctly treats it as `explicit-ticket`. |

The normal taxonomy CLI path at [taxonomyCliInventoryOptions](/Users/ueli/Documents/semio/📜️script.ts:20516) remains outside this mutation-only repair. It retains its existing `--ticket` ownership and shared inventory call. No shared N option, union, or source root is added.

## Selected Minimal Caller Contract

Keep the existing CLI and `--ticket` exactly as the mutation runner’s output destination. Do not add `--authored-ticket`, a new positional runner parameter, or any normal-taxonomy CLI option. The existing CLI had no distinct explicit authored-source authority, so this bounded repair removes its accidental source-admission authority rather than inventing one.

The exact owned mutation API becomes:

```ts
export interface MutationTaxonomyInventoryOptions {
  readonly explicitTicketDir?: string;
  readonly assignmentLedger?: unknown;
  readonly assignmentLedgerPath?: string;
  readonly cancelFile?: string;
  readonly progress?: /* existing inventory progress */;
  readonly scope?: string;
  readonly afterFactsCollected?: () => void;
}
```

`MutationTaxonomyInventoryOptions.ticketDir` cuts directly to `explicitTicketDir`; no alias remains. `mutationTaxonomySourceAdmission` passes only `explicitTicketDir` to unchanged shared N `inventoryTaxonomySources({ ticketDir })`. N retains the legitimate explicit-ticket union and authority without any new field.

`runMutationTaxonomyCli` keeps its current output `ticketDir` parameter and uses it only for `taxonomyCliArtifactPath`, output publication, and the existing output-ticket `assignmentLedgerPath`. It does not put that output directory in `MutationTaxonomyInventoryOptions`. Direct library callers retain the only explicit source path, now named `explicitTicketDir`.

This is neither an output skip list nor a physical-path policy change. A candidate under the output ticket remains eligible when independently admitted by `tracked`, `nonignored-untracked`, or `ignored-generator`; it simply cannot acquire `explicit-ticket` from output routing. Assignment-ledger capture stays separate under the output ticket.

## Exact Direct Cutover Footprint

The current typed/property use is bounded to S:

- [MutationTaxonomyInventoryOptions](/Users/ueli/Documents/semio/📜️script.ts:20631) changes its property name only.
- [mutationTaxonomySourceAdmission](/Users/ueli/Documents/semio/📜️script.ts:20752) changes the sole root forwarding expression from `options.ticketDir` to `options.explicitTicketDir`, retaining shared N’s destination property name `ticketDir`.
- [runMutationTaxonomyCli](/Users/ueli/Documents/semio/📜️script.ts:21075) removes only the output `ticketDir` property from its local inventory-options construction. Its output writer and assignment-ledger expressions remain untouched.
- [CleanScript.runTaxonomy](/Users/ueli/Documents/semio/📜️script.ts:21180) keeps passing the output ticket to `runMutationTaxonomyCli`; no CLI parser/type/normal CLI edit belongs to this cutover.
- Existing public runner calls in [index.test.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:7062) through `:7115` keep their output-ticket parameter and need no signature rewrite. Existing canonical [source-index capture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-index-capture/🟦️.ts:86) supplies no ticket property and needs no rename.
- This ticket’s [child probe](../🧪️mutation-ticket-role-split-64/📜️script.ts) deliberately uses current direct `{ ticketDir }` for the retained RED. It is the only located test/client object literal that needs to cut to `{ explicitTicketDir }` for the later GREEN.

The exact-property search was limited to `📜️script.ts`, the canonical library TypeScript tests, and this ticket controller; it is not an exhaustive repository consumer claim.

## Proposed Neutral Contract

[role-split schema](../🧪️mutation-ticket-role-split-64/🧬️schema/🔣️.json) and [vectors](../🧪️mutation-ticket-role-split-64/🔣️vectors.json) are closed, ticket-only design inputs. They express four role outcomes:

1. output-only routing produces no shared-N ticket input, even when an ignored generator output exists beneath that output ticket;
2. a direct API `explicitTicketDir` reaches N once with its authored input and no output destination;
3. a direct explicit input has no assignment-ledger or output implication;
4. a tracked candidate below an output ticket remains independently eligible while the output route still produces no N ticket input.

The independent role reference projects only option identity: `{ nTicketDir, outputDestination, assignmentLedgerPath, explicitTicketPaths, independentOutputPaths, accepted, error }`. It does not fabricate a roster. The subject is [the ticket controller](../🧪️mutation-ticket-role-split-64/📜️script.ts): an isolated Bun child imports the real N export family (22 exports), replaces only `inventoryTaxonomySources` with a sentinel, proves that binding armed before S is invoked, records options, and throws before physical capture. It preserves the real N family rather than substituting a fake module.

`reference` passed strict Ajv and the independent option oracle for all four cases. The current `check` is intentionally RED: both mutation CLI output-only cases supplied N `ticketDir` values (`tickets/OUTPUT`, `tickets/ONE`) where the closed vectors expect `null`; both direct API explicit-input cases supplied `tickets/AUTHORED` as expected. The check exits nonzero only for this semantic mismatch; the child exited cleanly at the sentinel and never invoked a collector.

The root package test style already imports mutation CLI functions from [index.test.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:38); a focused neighbour under `🧪️tests/🧬️mutation-inventory/🧪️ticket-roles/` can use its existing Ajv-backed fixture pattern. The existing [source-file facts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts) demonstrates closed inputs, an independent reference, and a missing-subject failure; its supplied observations must not be mistaken for physical admission proof.

## Non-Changes

- No shared `TaxonomyInventoryOptions` or normalizer source-admission union change.
- No generic ticket-output exclusion, new source root, skip list, alias, compatibility path, or new CLI flag.
- No normal taxonomy CLI behavior change.
- The prior `--authored-ticket`/NFC-overlap design was provisional and rejected; it is not an implementation request.
