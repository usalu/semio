# First Seven-Scope Independent Checkpoint Audit

All raw JSON rows independently parsed from first seven-scope full receipt: 5,472,124 bytes, SHA-256 `cecf2f1f1189913fcfdfb00ecb880ef781b246cc402833c9b204e4aa323ed75b`, 12,395 valid rows, zero malformed. Footer agrees: 4,477 modules, 3,630 scopes, 12,395 findings. Fixture/corpus findings are zero; all seven owned physical schema prefixes have zero rows; catalog findings are exactly one `schema-catalog-stale`. This is Nx1/70 seconds with unrelated repository diagnostics, not a global pass.

Generation log is 754 bytes/SHA-256 `c416e0fcd66de2e1c7af0658bd882eb226ce547239fca31fc38abcad5e097a49`; docs log is 710 bytes/SHA-256 `dd07fc9def1ec47e56fe597e2b8fbaa8dee5ad7b48a86779d9bd5e8f684b0ed9`. Both agree with the receipt and contain their successful target terminals. Generation reported 3,630 scopes/12,389 diagnostics; docs reported 3,630 scopes.

All seven receipt-bound physical schema bytes independently match their recorded digests. Current catalog path, recorded schema hash and complete sorted export names agree for hub.bootstrap, repo.discovery.runtime, os.plugin.browser-bundle, os.dev.distribution, os.dev.verification.publication, repo.library.workspaces.cargo.preparation.custody and s.dev.entry. All three original collection-owned schema paths remain physically absent with zero exact catalog references.

Current derived catalog has already changed during Root's second generation to SHA-256 `7a4522455571b21dcd49b7c89a3e975fe080bdf2a103eaa406c39b930d91a6b8`, 3,631 scopes. The first receipt's recorded catalog identity is therefore historical and is not relabelled current. A read-only comparison of current catalog recorded file hashes to actual disk bytes finds 23 drifts in peer Semio v1 diff facets: brep 4, cad 4, document 1, flow 4, image 3, mesh 3, model 2, presentation 2. No seven-scope binding drift was found. This concretely explains continuing concurrent derived-output risk without changing or adopting peer source.

Second-generation/docs/full/permanent receipts remain pending. No test, build, generation, producer or source edit was performed by this lane.
