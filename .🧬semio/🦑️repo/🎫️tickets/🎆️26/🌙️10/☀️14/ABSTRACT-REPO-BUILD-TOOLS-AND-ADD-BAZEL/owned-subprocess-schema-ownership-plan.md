# Owned Subprocess Wire Schema Ownership Plan

Planning only, subordinated to the released Windows payload implementation. No owned-execution Source/schema/declaration or facade changed. This concretizes [Low's earlier headroom review](Source-headroom-review.md); historical savings are not current capacity acceptance.

## Complete Component Fold

All five current documents are under `process/🎛️owned-execution/🧬️schema`. They use the SAME draft07 dialect, contain no `$id`, no `$ref`, no nested dialect/resource/anchor and no custom vocabulary. Keep `📬️output.json` as canonical document, preserving its entire root title/type/additionalProperties/required/properties and numeric constraints exactly. Add four whole sibling bodies as named definitions, removing only their redundant dialect declarations:

| Retired document | Canonical definition | Complete contract retained |
| --- | --- | --- |
| `📤️stdout.json` | `StdoutDestination` | title and exact inherit/ignore/stderr enum; absent type stays absent |
| `📤️stream.json` | `Stream` | title/string type and exact stdout/stderr enum |
| `📥️consumer.json` | `ConsumerProtocol` | title, closed object, all required version/synchronous/finiteOutput/chargeBeforeConsume/drainBeforeReject/undefinedFailure constants |
| `📪️exit.json` | `ExitFixture` | closed cases object, min1/max64, complete nested closed item with name/code/signal/stdout/stderr constraints and union types |

Output limits retain closed required maximumBytes/maximumLines integer1..9007199254740991. No value/type coercion, default, fallback, alias, field omission or old resource ID is introduced. Stdout/stream/root output limit values remain separate selected schemas; do not intersect a nested string enum with the canonical object root. Use canonical full-root validator context/ref wrappers even though these current subcontracts have no local refs. Preserve every title/annotation/scalar/array order. If a peer adds refs before implementation, re-read/protect and rebase every local pointer recursively rather than applying this zero-ref assumption blindly.

## Exact Consumer Census And Changes

Current literal Source census finds:

- `owned-execution/🟦️.ts` imports stdout and output: retain the canonical output import and privately select StdoutDestination. Its existing enum check and output validation bodies stay unchanged apart from selected contract identity/full context.
- `owned-execution/📥️capture/🟦️.ts` validates the output root: same existing canonical import; no runtime body change.
- `owned-execution/🧪️tests/🟦️.ts` imports exit: select ExitFixture from canonical root and validate it with that full root.
- `owned-execution/🧪️tests/📤️stream/🟦️.ts` and `📤️stdout/🟦️.ts` select their named definitions/full-root Ajv wrapper.
- `owned-execution/🧪️tests/📬️output/🟦️.ts` validates the canonical root, which remains the same output-limits contract.
- Repo `⚡️caching/📦️artifacts/💾️storage/🧪️tests/🟦️.ts` dynamically imports consumer.json: manually import canonical output and select ConsumerProtocol. Its actual charge-before-consume/undefined/async refusal assertions remain.
- Repo `⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts` reads output JSON by path: path/root contract stays canonical, no new file.
- Public `process/📦️packages/🟦️typescript/🎛️owned-execution/🧬️schema/🟦️.ts` explicitly exports `ownedExecutionOutputSchema`: retain that facade/export and its canonical path. It is an intentional package boundary, not a count-saving deletion.

Do a fresh all-consumer path/ID census before edit; keep stdout/stream/exit/consumer fixture DATA at their existing paths. Exact OS DSL document index read found none of these no-ID documents; no invented index entry/removal. Actual reader declaration contains stdout and output consecutively: after an implementation release remove exactly stdout, retain output once at its existing position, then Root verifies actual complete reader membership/order. Do not concurrently change Root's reader alignment.

## Protected Planning Evidence And Capacity

`🗑️generated/owned-wire-schema-planning.json` retains five current protected Source FD/stamp/SHA, entire normalized-document/body hashes and actual membership in the explicitly named retained receipts. All five appear in complete backend pair469, whose actual first capture is512files/3,564,682 bytes. Only output/stdout appear in each actual retained tooling role (reader171/runtime252/compiler132); stream/consumer/exit do not. Those membership facts are historical samples, not an assertion that Source bytes or a later backend count remain current.

Root reports a later511/512 point. No retained511 artifact was located by the bounded report lookup in this lane; use Root's exact later receipt before describing its import frontier. The mathematical projection is four fewer schema documents in any complete closure currently containing all five, and one fewer in each sampled tooling role. It is NOT a measured postfold507 or runtime251 result. Adding real Windows producer imports can introduce X/C/H edges and other dependencies; no schema folder deletion, code/type/facade deletion, scanner exclusion, externalization, manifest omission, quota/grant increase or cap increase is allowed.

## Schema-First Proof And Actual Gates

Before production mutation, protect all five full schemas again and retain full normalized hashes/ref locations, including output root. In the existing owned test/corpus owner add closed language-neutral contract cases and premove hashes, using the canonical `withBuildTestInvocationV1` incoming Original ownership. Actual Source RED must fail at absent canonical StdoutDestination/Stream/ConsumerProtocol/ExitFixture. The pure selector must avoid loading/running unrelated child-process or Native/SDK/graph fixtures.

Positive and negative rows cover output bounds at1/MAX_SAFE, zero/fraction/overflow/missing/extra fields; every stdout/stream enum and non-string/null/unknown; all required ConsumerProtocol constants, absent/false/extra/undefinedFailure alternatives; exit corpus counts0/1/64/65, nullable code/signal, negative code, empty signal/name and nested extra keys. Original fixture bytes and process output/exit behavior remain independently tested.

After movement, normalize each named definition back to its original whole body and compare all five full keyword hashes (canonical root compared excluding ONLY added definitions). Independent Python sorted JSON/SHA, Ajv draft07 full-root wrappers, first-party validator and SQLite JSON/type/count/bounds projections must agree across the closed corpus. No broad hash update conceals a constraint change. Remove only four schema docs; manually handalign all actual consumers. Fresh Source text census must find no retired schema path/import/ID, while fixture DATA remains.

Run current permanent owned output/stdout/stream/exit/consumer and capture laws that exercise affected contracts using actual bounded owned Node/Bun children, with no Native/store runtime. Keep existing finite controls/cancel/drain/assertions. New pure test obeys actual one-Original ownership; do not add receive/remint per law. Focused strict covers actual owned owner/capture/package schema facade, all changed direct tests plus the Repo consumer and test-body readers. Preserve arbitrary undefined/frozen failure and ordered body/cleanup behavior; this schema fold changes no execution algorithm or API.

Final modified/removed protected FD/SHA manifest and independent Low full normalized schema/context review are required. Root then runs unchanged-budget complete backend/tooling captures, exact reader handalignment/FD comparison and SDK provenance before physical acceptance. No new command, script, launcher or migration is needed. This plan remains unimplemented pending a separate release.
