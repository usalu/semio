# UI Canonical Reconciliation

Replaced stale semio-framework-ui package references and semio_framework_ui Rust identifiers with the actual semio-ui package and semio_ui crate naming across 182 current Cargo and Rust files. Collapsed identical duplicate dependency keys in UI contract text, lowpoly editor and WFC bitmap manifests. Updated native build path fixture package/crate values. No aliases or lockfile edits.

Fresh Bun TOML parsing covered 4,456 manifests and verified 3,407 dependency paths resolving inside UI against their target package declarations: zero mismatches and zero parse errors. Scoped source search finds no old UI Cargo or Rust identifiers. This validates manifest and source naming only; Cargo lock refresh, semantic compilation and native tests remain pending.
