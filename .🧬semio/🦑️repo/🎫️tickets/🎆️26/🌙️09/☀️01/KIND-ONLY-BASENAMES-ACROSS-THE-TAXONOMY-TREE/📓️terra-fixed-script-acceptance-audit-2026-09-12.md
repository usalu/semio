# Fixed Script Semantic Enforcement Acceptance Audit

Date: 2026-09-12

## Decision

The live gate is accepted for its bounded behavior. `fixedSourceDispositionDecision` only acts when the supplied exact fixed contract has a fixed source disposition with an explicit grammar. The current catalog has one such entry: `root-script` with the TypeScript `command-router` validator. An absent contract and `root-package`, which has no declared grammar, both return no decision. A valid imported `ScriptRouter` plus `runWorkspaceScriptMain` router produces `tool-metadata` with no finding; direct `Bun.write` produces `unresolved` and `fixed-source-disposition-unresolved`.

Normalization calls the gate after exact fixed-name selection and decoded-content preparation, and before package-role classification. The result keeps `fileKind:null`, the fixed contract identity, the fixed filename and the real `not-package` or `configuration` role. The gate does not create package metadata or admit a script by its location.

## Independent Checks

The portable fixture contains ten fixed-script vectors: root runtime/type-only/hidden-I/O, domain valid/hidden-I/O, and manifest-present/manifest-absent package valid/hidden-I/O. They exercise the owned classifier, the TypeScript AST disposition oracle, public package discovery where applicable, and the installed TypeScript compiler diagnostics for type-only imports. The current source does not add vocabulary beyond imported `Script` and `runWorkspaceScriptMain`.

`fixedScriptControlRoot` resolves `SEMIO_TEST_ARTIFACT_DIR` inside this ticket's `🗑️generated` root, checks every ancestor with `lstat`, rejects a symbolic link or non-directory, allocates only a unique `mkdtemp` child, and removes that child in each control's `finally` block. The cancellation test uses that control root and fails immediately with `Taxonomy operation cancelled`.

I independently created a ticket-generated mode-`000` fixed script, supplied it through the explicit ticket source-admission path, and then ran `inventoryTaxonomy`. Its row retained `root-script`, `fileKind:null`, and `not-package`; it contained both `path-read-failed` and `fixed-source-content-unreadable`. This confirms the runtime read-failure integration. The temporary control is removed after this audit.

Direct current-suite verification:

```text
SEMIO_TEST_ARTIFACT_DIR=<ticket generated child> bun test ./.../package-boundary-classification/🟦️.ts --reporter=dot
120 pass, 0 fail, 525 expect() calls, 20.87 s
```

Registered current-suite verification, with isolated `NX_WORKSPACE_DATA_DIRECTORY`, `NX_CACHE_DIRECTORY`, and `SEMIO_TEST_ARTIFACT_DIR` below this ticket's generated child, and `--skip-nx-cache`:

```text
bun nx run @semio-tech/repo-lib:test-package-body-policy --skip-nx-cache --output-style=static
task status 0; 120 pass, 0 fail, 525 expect() calls, 21.29 s
```

`loadCatalogTaxonomy()` followed by `validateTaxonomy()` returned zero diagnostics.

## Actionable Coverage Finding

The durable suite has a pure unavailable-content assertion (`fixedSourceDispositionDecision("root-script", null, taxonomy)`) but no permanent inventory test that forces a source read failure. The Sol report's EACCES probe is an accurate runtime observation, and this audit reproduced it, but the generated probe was deliberately removed. Add a cross-platform injected read-failure seam or a platform-gated permission control to the registered test if the integration behavior must stay protected against future normalization changes. Do not hard-code the present unresolved decisions of live scripts.

## Shared-Checkout Drift

The earlier Sol report observed the plugin descriptor and registry scripts as unresolved. In the live checkout during this audit, both classify as `tool-metadata` with no fixed-source finding:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`

This is concurrent owner work; this audit made no product changes and did not broaden the router grammar.

## Inputs Reviewed

- `📓️fixed-script-enforcement-closure-packet-2026-09-12.md`
- `📓️sol-fixed-script-semantic-enforcement-2026-09-12.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` — `2cb3f56e7479cff1f12b3c137dc32b8a740d104ad629830f61a42c0e8a807f88`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — `84e9b8d79a5845a694387e2a6da9544675056cf100fd4933b1767fc30615814b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification/🔣️.json` — `5f92e8accdbe7c4dd0a0a181d884e34d53e90ab2b3da07497c998acd27bd418c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json` — `9c352b1748817d69c0f937dfcb3b8d2cd52e148627ae005cbb745685f5b5759c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` — `e5e6256083679cac5f1d05649cd67deb8d054dfaa84f1512a959d1583f66bbf9`

The hashes identify reviewed input revisions only. They are not behavioral assertions or source snapshots.
