# Generation 259 Direct Existence Check

The retained parent manifest `a09127b692d0a7b02a627d07329d84db479232b01a05dc7446bc89894da27601` names 5,100 owner fragments. A bounded read-only file-existence check found all 5,100 corresponding `o/<fragment>.json.br` files, and found the named `f/<frontier>.json.br` and `x/<factIndex>.facts.br` files. Existence does not authenticate the members or prove closure.

The manifest names parent `b77756204a6bfd671c523aee7a056859c6a7a3e52e1d3add61329e41684c51ab`; its corresponding `m/<parent>.json` file was absent. This proves missing ancestry only; it does not establish that ancestry is required by closure validation. No restore candidate was authenticated by these checks.

The generation loader’s `settleRestoredClosuresV1` uses missing rows from a restored outcome and settles their `build-input-closures` or `build-lock-closures` keys; an unsettled row can cause the `closure-missing` invalidation. The manifest’s historical parent is therefore not identified as that missing leaf by this evidence. `readManifestAt` explicitly allows a retired manifest to be absent. The direct manifest fields did not reveal a concrete missing closure key.

The retained manifest's global fragment was also present and its compressed bytes matched the exact digest in the manifest. This verifies that single stored member and does not establish the complete closure or a signed current head.
