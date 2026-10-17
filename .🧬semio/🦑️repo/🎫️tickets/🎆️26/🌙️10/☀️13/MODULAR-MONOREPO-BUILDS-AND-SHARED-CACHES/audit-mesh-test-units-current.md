# Independent Mesh Test Units Audit

## Actionable Findings

- **P3 documentation:** `🧰️framework/🔨️modules/🧊️3d/🌀️rigid/🦀️.rs:6` still describes collision tests as “same crate, different file”. Collision and rigid now have separate defining crates and only collision declares the parry3d dev dependency. Update the ownership explanation; this is stale documentation, not a demonstrated runtime defect.
- **Evidence qualification:** `.vscode/launch.json` differs from the frozen identity. Both independent inventories see 8744208 bytes and SHA256 `2f28e3f94b703591fc70f9077b4009f920e3ea8ad12811df0bc563f938695009`, versus frozen `9c493da7e3e3440a0599dd01a91f0357c027a956a00f08b6c1dff25e0307182b`. Treat editor-row evidence as shared coedit drift requiring separate reconciliation. No attribution is inferred.

No additional actionable implementation defect was established by this Source-only review.

## Scope and Identity

Read-only Source/evidence review; only this report and before/after Markdown inventories were written. No tests, compiler, Cargo, metadata, Native/cache/SDK/browser/network/OS sampling, Git/worktree, ticket or goal actions were performed. A local Python read attempt found no tomllib; manifest checks instead read literal paths directly, without installing anything or running Cargo.

Frozen report SHA256 matches `22fb03491297f67c4ab5feed92c6ba188c05d0cebf2fbad7633db4ad9df38c3a`. All 87 declared identities (26 owned +48 observed +13 logs) were read and hashed with exact byte sizes before and after; 86 match frozen bytes. The launch file above is the only mismatch and is stable across this audit. See `audit-mesh-test-units-inventory-before.md` and `audit-mesh-test-units-inventory-after.md`.

## Independent Source Findings

Five distinct explicit integration targets resolve to real files; autotests=false closes implicit test discovery. The test library contains only a domain docstring. Its one canonical higher-library dev dependency activates mesh-test-observation. Literal library, test, dependency and workspace relative paths in the three new manifests resolve correctly. Framework lists all three members exactly once at Cargo.toml:306–308. This validates authored manifests, not Cargo resolution or macro expansion.

Higher root reexports actual rigid and collision crates at lines29 and36. Collision imports rigid at collision/🦀️.rs:16 and its only production dependency is rigid; rigid has no production dependencies. No return edge was found. Private closest_point_on_triangle differential coverage remains at collision/🧪️tests/🧿️collision/🦀️.rs:304 under its defining owner's cfg(test) mount. Integration targets import the canonical library and contain no production path mounts or production implementation copies.

The inspection module is feature-gated at modeling/🦀️.rs:15–17 and its public reexport at mesh/🦀️.rs:7–8. Its approximately4KB view exposes scalar facts and shared immutable references to first-party data. No Work or private algorithm type, &mut, constructor, clone, advance or step is exported. Inspection methods match immutable data; they do not grant job execution. Reconstruction's feature-owned MeshTopologyInspection at lines841–848 borrows mesh/soup and returns copied scalar state, without a dependency on higher mesh. Shared Vec/container references permit ordinary observation; no independent bound on their contents is claimed. Private algorithm state remains private.

Independent literal mount traversal reproduces the reported cfg-possible closures exactly:

| Root | Files | Bytes | Nonblank Lines |
|---|---:|---:|---:|
| Higher 3D | 7 | 211324 | 2028 |
| Document | 25 | 242824 | 3181 |
| Reconstruction | 3 | 111263 | 900 |
| Rigid | 2 | 10661 | 261 |
| Collision | 2 | 50258 | 999 |
| Jobs target | 1 | 219238 | 731 |

No bound was increased:262144 bytes/32 files/4000 nonblank lines. These are conservative literal-mount Source counts, not resolved rustc compilation-unit measurements.

All171 declared preserved function names occur exactly once across their consumed owners plus the moved reconstruction owner. The seven former-owner files contain1131 assertion macros; the moved snapshot-frontier law adds22, totaling1153. The moved law is at reconstruction/🧪️tests/🦀️.rs:30. Async custom-test attributes account for62 unit names; a plain #[test]-only census would incorrectly omit them. This proves current literal census consistency with the preserved manifest, not independent historical equivalence of every assertion expression: no baseline source bodies were available in this review. No duplicate declared law names or dropped declared names were found.

## Evidence Limits

Both green6 logs contain their separate10-pass/216-expectation adapter prerequisite and13-pass/1045-expectation mesh Source result. Their hashes match the freeze. Three strict logs are empty and hash to the empty-file SHA256; emptiness alone is not exit0, retained-root, serial execution, cache-at-birth or terminal-handle proof. The frozen report claims final strict handles28916/11309 exit0 and all handles terminal, but this reviewer did not observe those handle completions and cannot independently establish them from empty files. No blanket compiler qualification follows.

Rust remains authored and uncompiled; Cargo.lock remains unrefreshed. Source parsing, literal ownership, census and logged TS/Ajv/SQLite/Three outcomes do not establish Rust type checking, proc-macro behavior, runtime allocator behavior or execution correctness.
