# Source Owner Readiness — 2026-11-03

## Finding

The root-scoped sync published generation `cf6c9ceed426c7b6a612868b32ca3b27061726ef7376e8d6575d5d91dedb522d`, epoch 28, then failed during all-scope preservation retirement. The complete-current reader has a real conditional guard; the expanded Bun source excerpt omits its condition and does not imply an unconditional throw.

Read-only physical receipt inspection establishes an exact mismatch. The ready receipts at all four nonroot Source endpoints (`✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`) are version 4, but each records its root owner reference as `bb4a213fb3a0a7162e700134c268465c6d0c1a15a71b4bc800602f9e3a8c8a11`. Each also has 133 root-owned members carrying that same old root generation. The live root endpoint instead resolves to newly published `cf6c9ceed426c7b6a612868b32ca3b27061726ef7376e8d6575d5d91dedb522d`. Thus the workspace is not a complete current Source-owner view.

## Contract

`readJavascriptDependencyCurrentBindingsV1` derives prospective plans for every declared scope, checks exact generation and canonical ready text, verifies every active external-owner reference against active generations, and checks declared provider execution before and after receipt replay. The ready writer does not serialize the execution fingerprint; its zero-fingerprint witness validates scope presence and is not evidence of the observed failure.

The existing owned consumer regression explicitly rejects a stale receipt and a changed member manifest. The ready codec and generation laws preserve the same current ownership and execution constraints. No evidence justifies relaxing those guards, manufacturing completion receipts or accepting an earlier input generation as current.

## Validation

Focused existing laws run through `repo tools exec --light bun -- test`:

- Source owner identities preserve unrelated generations.
- Source owner ready codec retains owned-only bindings.
- Source owner ready projection binds scoped execution.

Result: 3 pass, 0 fail, 83 expectations; 83 unrelated cases filtered out. Existing laws include language-neutral cases and independent Ajv/SQLite oracles.

## Next Canonical Operation

The demonstrator consumes framework, S, Hub and Mit Bestand scopes. A canonical `repo dependencies lock --all` followed by `repo dependencies sync --all` can restore their current common root references under the owning custody protocol. Parent coordinates that operation and the write freeze. No Source, schema, storage, receipt or guard was edited during this diagnosis. This report records a qualified refusal and an incomplete scoped synchronization, not a source-reader defect or a successful native dependency gate.
