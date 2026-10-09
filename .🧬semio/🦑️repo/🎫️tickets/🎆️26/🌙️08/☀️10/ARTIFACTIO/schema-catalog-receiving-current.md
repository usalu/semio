# Immutable Catalog Receiving Floor

Actual full Kernel12 terminated before tests because the canonical Schema public surface reexported EntityKindCatalog while its Rust projection produced only EntityKind and ENTITY_KINDS. The canonical Rust projection now defines the schema-declared immutable catalog as a static borrowed slice, matching the existing TypeScript readonly array and actual neutral EntityKindCatalog schema. Its generated Rust projection has the same definition, with no allocation or replacement catalog. Whole Kernel receiving remains required.
