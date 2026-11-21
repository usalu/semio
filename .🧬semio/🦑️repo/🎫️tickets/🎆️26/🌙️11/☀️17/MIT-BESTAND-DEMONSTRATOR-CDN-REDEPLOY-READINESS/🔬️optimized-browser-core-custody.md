# Optimized Browser Core Custody

The component receipt records the linked raw Wasm SHA-256 and the semantic descriptor SHA-256. Each browser module's publication marker declares its owner and file names. The site copier transforms the relative Preview 2 imports for extensions, then copies the declared files. The original batch gate verified equality between staged and site files and held the site digest across browser acceptance. An exact staged-to-site comparison could still accept an altered core if both copies carried the same edit before the accepted site was hashed.

A disposable reproduction used the confirmed Flow Extension Dictionary release. The old gate passed the real copies, and then accepted both copies with the same additional valid Wasm custom section. All receipts remained unchanged. The older Energy-only `wt48zqN` audit already took the stronger route by binding its linked raw Wasm to its transpile output and its optimized stage artifact.

The queue recorder now certifies every optimized core against retained production stage records. Transpile evidence selects the recorded raw component and package base. If the release contains DWARF, the recorder hashes the authenticated retained model after applying the production debug-strip function. Optimize evidence binds each extracted core digest to the exact optimized file. A browser probe binds the complete core set to the same semantic descriptor. Only one resulting inventory is admissible. The recorder rehashes the staged files and atomically commits the receipt and inventory in one confirmation JSON. The existing pair wrapper holds these files privately until both recordings succeed.

Final verification checks the complete staged and site Wasm inventories against the recorded paths, sizes and SHA-256 values. Final verification reads no stage-cache records, so later stage-cache collection preserves confirmation usability. Existing authored descriptor verification, extension import transformation checks, site acceptance and archive checks remain in the release path.

The five existing confirmations were rechecked against their matching live staged raw receipts and certified using retained stage evidence. No module build was needed. GIS was following the canonical publication path and had no confirmation at the handover audit, so its normal queue recording will use this certificate path.

| Confirmed module | Optimized core bytes | SHA-256 |
| --- | ---: | --- |
| Energy | 266,429,018 | `32db45ad1079e2205425f17238a3de2b53e74f23ddccf1647242cadc8d66680f` |
| FEM | 91,802,095 | `90345f9e9be2050eea3ffb11cb8b82f5a685317a16d25624429d37acd88e4e2c` |
| Flow Extension BIM | 26,059,931 | `686297e69eb19649a39e82c9dfdec8190e3dcb462647f5b06b348d93fed15c9d` |
| Flow Extension Dictionary | 25,984,975 | `476c131ee13a6fc3cdad1cdffd3d25284f7d5dcbe5e26ff30c30f5085ea8a604` |
| Procedural | 111,726,765 | `dbcf6d7182aae784c3dce596a133b62d292b4dfa58d7e3d5923dabe4ef52c62e` |

The real Dictionary fixture re-certified the exact bundle, verified it with access to stage evidence intentionally withdrawn, and rejected the same paired substitution. The paired queue fixture supplied two raw receipts with one missing core proof. Pair confirmation stayed private and the wrapper completed its singleton fallback. The synthetic full-roster fixture exercised all 32 receipt slots and retained the mismatch checks for staged receipts, extension imports, and copied site files. Runtime acceptance for the rebuilt 32-root candidate is tracked separately.

[Release receipt gate](release-receipts.py) · [Stage lineage certification](verify-browser-core-lineage.ts) · [Real component regression](test-sealed-browser-cores.py)
