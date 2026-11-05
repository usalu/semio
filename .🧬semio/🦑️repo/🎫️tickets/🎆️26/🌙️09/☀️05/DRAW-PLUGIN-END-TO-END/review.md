# Draw Change Review

Read-only audit of IO descriptors, composed guest opening contracts, and browser acceptance on 2026-11-04. No source changes or runtime checks performed; root owns execution checks.

## Actionable Findings

- Browser journey reload only waits for a matching layer count before success. Require ready=draw and no semioOsError after reload, preferably also a visible canvas, so restored status text cannot mask a guest startup failure. File: ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs, reload block. Runtime owner notified.
- Browser journey fixture declares export.format but runner always uses the PDF action/parser/path without consulting it. Assert format=pdf or make this field control dispatch, otherwise a future fixture format change passes against the wrong export contract. Runtime owner notified.
- Composed Draw default input additions use recursive **/*.json over the package dist directory as well as owned fixtures. This can hash produced component/cache JSON as source and cause avoidable invalidation; replace broad patterns with owned component descriptor and guest test paths or explicit output exclusion. File: 🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/📋️project.json. Existing recursive Rust/projectRoot inputs already warrant a broader ownership review.

## Verified By Inspection

- IO TypeScript rows now use the shared structured dialect wire contract and follow the native eight-entry registration ordering, fidelity, and sniff declarations. Neutral fixture checks supported and unsupported pairs; Ajv and owned parser cover invalid descriptors. Native fixture comparison provides the implementation bridge.
- Guest scenarios cover editor and viewer with distinct actors. Both native deserialization and schema reject malformed fields; schema constrains app/role pairing, uniqueness, and required role coverage. Native checks tie scenarios to the shipped descriptor.
- Guest open loop now rejects faulted turns and requires a lifecycle receipt before idle success. The receipt type/correlation is not independently asserted, but fresh instances receive only InstanceOpen before acknowledgement, so no concrete false-positive path was identified.
- New browser target is live/noncached and routed through the owned cancellation boundary. Real pointer, undo/redo, download parsing, and reload exercise useful behavior. Full browser execution remains essential because UI selectors and localized status parsing are implementation-coupled.

## Scope Limits

Browser source was still being edited by the runtime owner during review. Review is advisory against the inspected snapshot; no claim that browser acceptance has passed. Root retains ticket lifecycle ownership.
