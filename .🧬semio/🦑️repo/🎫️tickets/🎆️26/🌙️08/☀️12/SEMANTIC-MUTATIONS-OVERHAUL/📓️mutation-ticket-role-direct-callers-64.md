# Mutation Ticket Role Direct Callers 64

## Scope and Method

Read-only caller audit after the retained role-forwarding RED. The search was bounded to regular authored TypeScript and TSX candidates using `rg` with exclusions for every `compose` segment, `.🧬semio` ticket output, `node_modules`, `dist`, `build`, `target`, and `⚡️cache`. It searched only direct references to `MutationTaxonomyInventoryOptions`, `mutationTaxonomySourceAdmission`, `mutationTaxonomySourceIndex`, and `inventoryMutationTaxonomy`.

This is an exact result for that bounded source filter, not an all-language or whole-repository consumer claim. No inventory, Git, source capture, native command, or Compose path was run or read.

The capture read root S at `fdb34f8e4a9d1696915dc18d804876ed80a1f46c6d09d365f92b24914c5a991d`, canonical package tests at `c226ed8a179841c80b8ba963e432a08af3bea709a2ef96c7b2ca64dd5c7056cc`, canonical source-index capture at `79829a5bb5b7dd5e6b1665fc787bdaaf5dec8ef8f620367991a71f132d534c94`, and the staged ticket probe at `02aaa60b53dd62688935d98098c03b25498d2f18507dd186b45b72a736f9477a`.

## Observed Direct Callers

| Location | Direct role | Cutover consequence |
| --- | --- | --- |
| [S option type](/Users/ueli/Documents/semio/📜️script.ts:20631) | Declares `ticketDir?: string`. | Direct rename to `explicitTicketDir?: string`. |
| [S admission bridge](/Users/ueli/Documents/semio/📜️script.ts:20752) | Sole property forwarder into shared N. | Forward `options.explicitTicketDir` as N’s unchanged `ticketDir`. |
| [S mutation runner](/Users/ueli/Documents/semio/📜️script.ts:21075) | Builds typed local inventory options from output `ticketDir`. | Remove exactly that forwarding; retain output publication and `assignmentLedgerPath`. |
| [S local inventory helpers](/Users/ueli/Documents/semio/📜️script.ts:20725), [source index](/Users/ueli/Documents/semio/📜️script.ts:20807), [snapshot](/Users/ueli/Documents/semio/📜️script.ts:20860), [inventory](/Users/ueli/Documents/semio/📜️script.ts:20943), and [verify](/Users/ueli/Documents/semio/📜️script.ts:21066) | Typed pass-through/default consumers only. | Receive the renamed type; no observed object literal needs a key rewrite. |
| [S structural policy defaults](/Users/ueli/Documents/semio/📜️script.ts:28189), [policy view](/Users/ueli/Documents/semio/📜️script.ts:28537), [policy breach view](/Users/ueli/Documents/semio/📜️script.ts:28731) | Calls source index with `{}`. | No change. |
| [Canonical package test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:6862) through `:7115` | Calls inventory and runner without an admission-ticket object literal; it separately supplies assignment-ledger fixtures. | No signature/key rewrite. Preserve those assignment-ledger tests. |
| [Canonical source-index capture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-index-capture/🟦️.ts:59) through `:104` | Dynamically resolves source index and calls it with `{}` or cancellation/progress only. | No admission-ticket key rewrite. |
| [Ticket role probe](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-ticket-role-split-64/📜️script.ts:147) | Current direct API sentinel fixture. | Staged to `{ explicitTicketDir }`; intentionally not run until root S release. |

No other direct regular TS/TSX caller was returned by the bounded search. The CLI parser and normal taxonomy route were intentionally outside the direct type-key rename: [CleanScript.runTaxonomy](/Users/ueli/Documents/semio/📜️script.ts:21180) keeps its output-ticket call into the runner.

## Staged Ticket Probe

The child probe now uses `explicitTicketDir` for its two direct API cases. This is preparation only. Running it against current S would leave N’s current `ticketDir` absent on those direct calls and could fall through the sentinel, so no GREEN attempt was made. The retained current RED remains [run-PUOuo3](../🧫️role-result-PUOuo3/receipt.json); it used the older direct key only to prove the current direct path worked while the output forwarding was defective.

## Proposed Canonical Regression Triplet

After root review, add one canonical neighbour without creating a new membership authority:

1. `🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🧬️schema/🔣️.json` — closed role input/output schema, retaining independent origin facts for output-path candidates.
2. `🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🔣️.json` — the four closed cases from this ticket, including ignored-generator and tracked candidates under the output ticket plus direct explicit input.
3. `🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🟦️.ts` — strict Ajv, independent option-role projection, and an isolated Bun child which preserves the real N export family, replaces only `inventoryTaxonomySources`, requires the sentinel binding before invoking S, and checks output versus direct key forwarding before capture.

The existing package entry [index.test.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:1) already mounts neighbouring mutation-inventory tests by direct import. The eventual single import belongs beside its current source-file facts/source-roster role imports. This proposal does not create that import or any canonical file yet.

## Boundary

The direct API retains intentional explicit-ticket capability through the renamed field. Output routing does not acquire an admission key. Independently admitted paths under the output ticket remain observable by their existing shared N origins; the proposed test asserts role forwarding only and does not fabricate a source roster.
