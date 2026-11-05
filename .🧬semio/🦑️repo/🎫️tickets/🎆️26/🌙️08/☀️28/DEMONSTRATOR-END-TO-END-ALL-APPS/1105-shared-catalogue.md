# Shared Operator Catalogue Owner

The existing Flow instance catalogue owner now belongs to the defining semio-os-flow-operators package at registry/catalogue-owner. Flow imports the shared FlowOperatorCatalogueOwner directly; the private implementation was removed. Only module paths, the type name, and public visibility changed. Derivation, registry identity/generation matching, paid alias retention, retirement order, deadlines, source limits and bootstrap semantics remain identical.

The operators package now declares its existing first-party semio-framework-job dependency explicitly. Runtime confirmed both source packages lie outside the active lower Ordered/Equality capture. No external runtime dependency or Bun package change was introduced.

The initial owning package census was 96,725 bytes / seven source files. Final census and syntax validation follow. Native typing and laws remain pending on the sole Runtime-owned queue.

Final static census: 105,554 bytes / eight files, zero missing sources, below the existing 262,144-byte / 32-file limits. Both touched Rust source files parsed with zero errors. Native typing remains unrun.
