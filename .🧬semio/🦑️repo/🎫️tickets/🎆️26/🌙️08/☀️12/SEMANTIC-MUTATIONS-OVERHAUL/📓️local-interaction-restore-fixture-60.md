# Local Interaction Restore Full and Sparse Fixture

## Scope

This preparation packet changes only ticket-owned files:

- [`📜️script.ts`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/📜️script.ts)
- [`🧫️fixtures/🔣️schema.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️fixtures/🔣️schema.json)
- [`🧫️fixtures/🔣️vectors.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️fixtures/🔣️vectors.json)

No Plugin, Interaction, protocol, schema-production, native, command, publication, or renderer source changed. No native compiler or runtime command ran.

## Neutral contract

The fixture schema references the actual protocol schema `semio:local-interaction` for every base identity, restore, and three-map state. It covers:

- exact-base `full` replacement of all three maps;
- sparse all-null removal of only one domain while unrelated domains remain intact;
- explicit values for a non-broadcast domain, including an anchor and literal comma-containing ID;
- empty sparse identity;
- stale generation and stale topology rejection before the cold function can write;
- four live-only cancellation/rejection phase obligations.

Each direct input is first-read with nofollow ancestor checks, SHA-256 captured, re-read before result publication, and retained alongside the receipt.

## Executed source/reference result

The scoped command executed was:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/📜️script.ts' check
```

It exited `0`; the retained [corrected receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/check-9e76bc5c-da99-4ded-a42d-3914b34d156e/🔣️result.json) records 58 counted assertions across six cases and four live obligations, with no input drift. The prior [60-count receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/check-138eed72-acf3-4582-83c4-904219d60f2e/🔣️result.json) is preserved as historical evidence: its `let assertions = 10` seed overcounted the eight initial envelope/id/hostile assertions by two. The controller now counts each check, equality, deep equality, and throw through a helper; it has no manual assertion seed.

The controller executes the actual canonical TypeScript `applyLocalInteractionRestoreCold` and compares each successful result with two independent implementations: `jsonc-parser` edit/apply over a JSON representation and Immer immutable production. Ajv rejects missing nullable patch fields, duplicate IDs, unknown fields, non-string generations, and unknown restore kinds. This is a cold-model/schema comparison only.

## Deliberately unexecuted obligations

`LocalInteractionRootUpdate` was not instantiated or executed. The retained candidate-close path, topology/registry validation, cancellation during every phase, ordinary-command admission, Store-root replacement, and publication remain live-runtime obligations. This fixture neither creates an ABI/tag nor infers a cold fallback route.

## Reusable Interaction-fixture proposal

No production fixture filename or leaf location is proposed. Taxonomy must choose the semantic directory and empty-leaf convention before any canonical Interaction fixture is published; this ticket does not authorize an ad hoc named leaf.

That future Interaction fixture must retain the actual `semio:local-interaction` schema and full `LocalInteractionIdentity` unchanged. It can reuse the exact-base/full-or-sparse shape, explicit-null removal, expected cold result, and separately labeled live obligations from this ticket fixture, but it must not substitute a graph-domain or a new ABI. Two independent reference calculations remain required, and stale-base/cancellation must remain unexecuted until the retained native Interaction owner is mounted. A successful immutable model comparison is not a complete live mutation proof.

## Final cross-platform controller receipt

The ticket controller now normalizes `relative(workspace, resolved)` with the native `node:path` separator before applying its existing slash-based lexical and Compose checks. This preserves Windows-native path handling without changing vectors or source semantics. The final scoped check exited `0`; its [receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/check-b20b7e39-9b33-40fd-9ed5-16f603cd7097/🔣️result.json) has the unchanged six cases, four live obligations, 58 counted assertions, no drift, and final controller SHA-256 `adfceefe2e150f55cba2b283efbacdca66aeb3298845df85115709233f5f9bb4`.

## Required non-broadcast fixture role

The initial fixture schema made `nonbroadcastDomains` optional even though the successful-case controller iterates it. The controller now constructs an exact missing-field mutant and validates it through the same independent Ajv instance. The retained [pre-fix RED](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/red-a95164a2-1db8-4d7d-90f7-3101ae600081/🔣️result.json) exited `1` only after recording that the actual fixture schema accepted the mutant (`missingNonbroadcastDomainsAccepted: true`).

The fixture case schema now requires `nonbroadcastDomains`; no existing vector was changed. The scoped [post-fix GREEN](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/check-9f897866-86f7-405f-99dd-3c335bc6acdc/🔣️result.json) exited `0`: 59 counted assertions, six cases, four live obligations, no drift, `missingNonbroadcastDomainsAccepted: false`. Its stable endpoint hashes are canonical source schema `9b200e30396f6637f08b6b3a7d5017eac938a8edc88258ade34e907e5a87348e`, actual cold source `2467d16665faefbdf3aa301bb7d38e41dfa92d59c98fd0b66644b21005069497`, vectors `fde9726b2457662e1fd88c572a4743556d3cfdedef8b2564813876dfd0945fd3`, fixture schema `fe128e22ea0607c202975f6c23796c57a4d1a8af2c4c0e5edb867e74a0cfa939`, and controller `70702fcc875c13a6e0c30c847d82a747d12657ef54a2175219823cf18db1d82d`.

## Desired-law replay correction

The earlier `red-a951…` receipt is retained as a defect reproducer only: its red mode asserted that acceptance was expected and then deliberately threw. It was not a desired-law RED.

The controller now has one shared assertion implementation and executes the same desired law in both modes: the fixture schema must reject an otherwise identical vector with `nonbroadcastDomains` omitted. `red` selects only the exact retained pre-fix schema bytes captured at `red-a95164a2-1db8-4d7d-90f7-3101ae600081/🧬️fixture.schema.json`; it never restores or overwrites the current schema or vectors. The bounded [desired-law RED replay](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/red-37f8356f-fa83-490e-a037-18a57c465531/🔣️result.json) exited `1` with one recorded failure, `fixture schema must reject missing nonbroadcastDomains`; it identifies its fixture authority as `retained-pre-fix-red-a95164a2`, preserves the current schema endpoint hash, and records `missingNonbroadcastDomainsAccepted: true`.

The same controller and assertion then ran against the current schema. The [current-schema GREEN](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️local-interaction-restore-fixture-60/🧫️runs/check-1f43e1fc-be73-49c6-be4c-20b9110dcb23/🔣️result.json) exited `0` with 59 assertions, no failures, no drift, and `missingNonbroadcastDomainsAccepted: false`. Both replay receipts use unchanged vector hash `fde9726b2457662e1fd88c572a4743556d3cfdedef8b2564813876dfd0945fd3`; the final controller hash is `651610d6f61432221fb5ddc4b91120a1ddedbedac5d12f709437a9eb5552a273`.

## Root Review And Current Release

Root read the complete changed controller, desired-law RED replay and GREEN receipt. The nonthrowing accumulated `check` helper now returns `void`, not an unsound `asserts value` narrowing. No expected values, six source cases, live obligations, runtime code or schema semantics changed in that correction. Root then independently ran the same `check` through Bun/Nx: exit 0, 59 assertions, six cases, four declared live obligations, no failures and no input drift, retained at `🧫️runs/check-8868b42e-4f84-41d8-aee3-44bcc2eff914/🔣️result.json`. The current controller hash is `065eda7a09389503d7a2b84fdd5123dee8cca1b3c2a93f04a15162f60305ea12`; fixture schema remains `fe128e22ea0607c202975f6c23796c57a4d1a8af2c4c0e5edb867e74a0cfa939`. All prior captures remain historical evidence; no native/live or canonical publication claim is added.
