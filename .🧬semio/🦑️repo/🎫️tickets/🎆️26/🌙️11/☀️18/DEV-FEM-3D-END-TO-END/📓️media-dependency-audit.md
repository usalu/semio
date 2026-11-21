# Media Dependency Audit

Read-only scan of current Rust references to `semio_plugin_decl_media` and plugin Cargo manifests. No additional missing direct declaration was found. Current reactor-lifecycle (`⚛️reactor/📨️pending/📦️packages/🦀️rust/Cargo.toml`) and instance-render (`🧵️instance/🖼️render/📦️packages/🦀️rust/Cargo.toml`) both declare it.

Apparent nearest-directory exceptions were checked against actual inclusions: `⚛️reactor/🚪️lifetime/🦀️.rs` belongs to reactor-lifecycle, runtime command custody manifest is included by runtime-ops, `📜️declaration/👷️job/🦀️.rs` and its child files are included by decl-app, and the declaration-media publication test is included by companion tests-03. Their owning manifests declare the dependency. Root mounts are included by the plugin root manifest, which also declares it. Other direct production consumers and the relevant test companions found by the scan declare it.

This is a source/manifest scan, not Cargo planning or compilation; it cannot protect a plan against later concurrent edits or prove every feature-specific include is exercised.
