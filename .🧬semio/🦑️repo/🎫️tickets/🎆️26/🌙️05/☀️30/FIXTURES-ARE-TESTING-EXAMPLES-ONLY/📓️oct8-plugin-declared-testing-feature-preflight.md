# Declared Testing Feature Preflight

Read-only authored manifest scan using the existing Bun TOML parser. This is declared source selection evidence only; real Cargo compiler messages and shipped component bytes remain required. No manifest or invocation flag was modified.

Parsed 387 manifests; 0 parse refusals. Direct normal/build requests for the two named test capabilities: 0. Dev-dependency requests are excluded from this source inventory.


The explicit host testing component is a separate authored owner. Its declaration cannot establish a runtime component mount. Ordinary Cargo --lib checks and real component rustc builds must independently confirm their actual resolved features/profile and staged ownership. This read-only preflight does not infer a global graph or publication pass. Python standard tomllib was unavailable; the existing Bun parser was used without adding a dependency.
