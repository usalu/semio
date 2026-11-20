# Selected source input traversal

A g1-m11 physical slice identified source-input matching in a Rust owner as its active operation. Inspection found a reproducible separate cost in the same matcher. A declared workspace-root `*.rs` input in the repository TypeScript package made an exhaustive recursive walk of the checkout even though its wildcard cannot cross a directory separator. The eventual empty match depended on observations from unrelated subtrees.

The matcher now plans the directory depth needed by all positive globs sharing a root. A basename glob using ordinary single stars and question marks needs only that root listing. Recursive forms, directory components, alternations and character classes retain the existing recursive behavior. A shallow and a recursive glob at the same root still share one physical walk. Matching and exclusions remain per target and the matcher still compiles them lazily.

The minimatch and Ajv fixtures cover a file matching at the root, a same-named file deeper in the tree and a mixed recursive scan. On the original code the shallow case incorrectly read two descendant directories. On the real checkout, the actual production matcher checked the root once, collected three observations, and found zero Rust files in 11 ms. Independent Python directory enumeration also found zero. This measures one pattern; it does not assign the physical worker’s elapsed time to it.
The four adjacent source-demand and selection suites passed together with 42 tests and 1,897 assertions.
