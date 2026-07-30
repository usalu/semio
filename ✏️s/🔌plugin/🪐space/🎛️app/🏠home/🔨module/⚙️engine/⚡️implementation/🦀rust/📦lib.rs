//! ⚙️ S Home launcher app — headless compute (constitutional: engine).
//!
//! 🕳️ `SHomeDocument` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own — the original monolith never factored
//! out a pure `empty_home_document()`/compute helper (every call site builds the literal
//! `SHomeDocument { schema: "s.home".into(), catalog_generation: N }` directly), so this layer is
//! deliberately empty. All of the Home launcher's actual document-adjacent logic (catalog/ephemeral
//! studio port plumbing) is host-effectful (backbone port I/O), not pure compute, and lives in
//! `home_ui` instead — see the constitutional split recipe's engine definition ("ALL PURE compute over
//! the document").
