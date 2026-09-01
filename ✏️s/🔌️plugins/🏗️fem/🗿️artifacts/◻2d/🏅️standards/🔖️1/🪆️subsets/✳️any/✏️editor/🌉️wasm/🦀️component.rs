//! 🌉️ Fem2d play app — the wasm-bindgen `Fem2dSnapshotVcs` VCS bridge that used to live here was
//! deleted — nothing ever built it for `wasm32-unknown-unknown` (no engine entry, no `wasm` script
//! target) — see `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    /// 🧪️ The store type aliases live in `crate::artifacts::fem2d::mutations` (`Fem2dEnvelope`/`Fem2dStore`) and
    /// are exercised by that node's own tests plus `crate::artifacts::fem2d::spr`'s
    /// `fem2d_document_text_round_trips_through_the_store`.
    #[test]
    fn fem2d_store_type_alias_constructs_from_an_empty_envelope() {
        let store =
            semio_framework_plugin::resolve_ready(crate::artifacts::fem2d::mutations::Fem2dStore::new(store::create_document_envelope(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", crate::artifacts::fem2d::schema::empty_fem2d_snapshot(), None)))
                .expect("valid store");
        assert!(semio_framework_plugin::resolve_ready(store.snapshot()).expect("snapshot").nodes.is_empty());
    }
}
// #endregion 🧪️Tests
