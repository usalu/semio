# Fixed Script Semantic Enforcement Closure

## Confirmed Package Inventory Gap

The coordinator ran the public disposition classifier, filename check and normalizer against the actual OS dev package script on 2026-09-12. The existing nonpackage probe already established the same missing semantic enforcement at the repository root.

| Actual source | Direct disposition | Fixed contract | Inventory package role | Semantic violations |
| --- | --- | --- | --- | --- |
| Repository root 📜️script.ts | unresolved | root-script | not-package | none |
| OS dev 📦️packages/🟦️typescript/📜️script.ts | unresolved | root-script | configuration | none |

The second source was 355,418 bytes, SHA-256 56e0313b7bd461c1f595649e9ef0564ad3afeb3f6c7953b06da763f5a89fedd1. Inventory finished in 3,233.6 ms. This hash identifies the inspected source only; it is not a permanent body snapshot. Raw evidence is generated/coordinator/package-script-enforcement-probe.json.

The exact source is 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts. Its inventory row has fileKind:null, fixedContractId:root-script and no violations. The filename itself is correctly preserved by the user-mandated fixed contract.

## Why Both Paths Bypass

In normalization/🟦️.ts, canonicalFile returns fileKind:null for any selected fixed filename contract. classifyPackageRole first returns not-package outside package boundaries; inside a package it treats contractId plus no source kind as configuration before evaluating the source disposition. Thus even a fixed source inside a package bypasses this normalizer branch. Discovery's package walker separately derives source kind from the filename and does run its body recognizer, so that independent path must remain covered rather than conflated with normalizer acceptance.

This adds a second precise integration regression to 📓️nonpackage-script-enforcement-probe-2026-09-12.md. Correcting the lexical parser alone cannot resolve either integration bypass.

## Next Sol Gate Slice

After the active lexical parser correction is independently accepted, use a source-disposition check independent of package membership and preserved filename canonicalization. A mandatory command filename and a proven routing body are separate facts. Resolve the exact fixed contract and its declared grammar/validator, decode source safely and emit a clear semantic source finding for an unproven body. Keep real package roles and mandatory filenames unchanged; do not manufacture package metadata, relocate the executable name, or whitelist scripts by path/size.

Portable fixture coverage must include the same valid delegated router and hidden authored behavior in repository root, neutral domain, and language-package placements. Exercise direct classifier, package discovery where applicable, scoped normalization, ordinary registered enforcement and filename preservation. Include absent/present package manifest siblings, precise fixed-contract matching, unreadable/invalid source and the existing cancellation/progress protocol. Verify native TypeScript value/scope controls through the independent compiler oracle.

Canonical fixed metadata without a source-disposition contract remains metadata; this is not a proposal to parse every JSON/config/document as executable source. Maintain the catalog's read-only projection boundary and the manifestless discovery regression. Do not bury these checks in package-only collection or silently turn an unresolved source into configuration.

## Scope

This packet is semantic enforcement only. The separately queued source extractions remain necessary to make the repository pass the stronger gate. The 455-script census is review scope, not a list of 455 defects. Preserve imported opaque forwarding and valid native process routing, as settled in the router policy audit.

## Root Router Vocabulary Follow-Up

The completed bounded parser currently admits BundleScript plus four established terminals: runBundleScriptMain, runPolicyOnlyMain, runArtifactRustPackageMain and runArtifactTypeScriptPackageMain. Its 83/89 result is not an exhaustive statement about every repository router. The actual root and repo-test command compositions also use the owned Script base, and root dispatch uses runWorkspaceScriptMain. As the fixed-script gate expands to all 455 sources, model the actual neutral routing contracts with portable runtime/type-only pairs before deciding their disposition. Do not label a valid direct imported router defective merely because it was outside the earlier corpus, and do not exempt a body merely because it uses an additional base/terminal name. Preserve original imported export identity, scope, recursive arguments and the owned delegation contract.

## In-Progress Integration Review

Sol's implementation introduces a pure fixedSourceDispositionDecision keyed by the exact fixed contract and explicit grammarId. Normalization invokes it after decoding and exact fixed-name selection, before independent package-role consequences. Root read the implementation: this preserves fileKind:null and package configuration/not-package roles while surfacing unresolved or unreadable body findings. The explicit Script/runWorkspaceScriptMain vocabulary is also being added to the independent native oracle.

Root found two issues in the in-progress tests and sent them back before acceptance. First, the live root/Demonstrator/OS-dev integration assertion must not permanently require all three sources to remain unresolved: that would freeze present debt and reject future successful extraction or valid routing vocabulary. Use the independent native oracle for live expected decisions and keep controlled portable hostile/positive bodies as stable integration authority. Second, newly added cancellation/package controls must use ticket-owned generated artifact paths with no-follow ownership, rather than creating a cancellation folder directly in repository root or new controls in the operating-system temporary root. These are requested corrections, not a completed audit; the executor's final report must record their resolution.
