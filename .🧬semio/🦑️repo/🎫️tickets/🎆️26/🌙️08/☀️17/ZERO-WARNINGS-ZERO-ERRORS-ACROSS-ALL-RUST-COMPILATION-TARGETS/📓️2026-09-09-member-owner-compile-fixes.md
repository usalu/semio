# Member Owner Compile Fixes

WASI 1096 stopped at nine shared store errors. Import the existing value derive macros alongside the traits for OwnerRef. Route the typed initial-member opener through the store's inherent bounded close methods; the erased SpaceMember facade now requires composition traversal, but typed store retirement itself does not. Extract the exact existing terminal predicate into the inherent store method and have both callers use it, preserving disposer and read-lease checks. These changes address the observed missing derives and unrelated facade bounds without widening generic snapshot requirements. Fresh strict validation and retained lifecycle runtime tests are pending.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs
