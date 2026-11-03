# Patch: registry-backed law constructors publish the framework schema documents

Root cause: `new_app_with_registry_and_members` / `new_registered_app_with_members` build a `VcsArtifactApp` from a manifest but never run `PluginRuntimeRegistry::publish_framework_schema_documents()`, which every production app construction runs (`🧵️instance/📥️staging/🚪️ingress/🦀️.rs:66`). Any input schema `$ref`-ing `/link` etc. (energy `bind-weather-file`) is refused `RefUnresolved` when the pane is born through the law helper.

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`
(crate already depends on `semio_framework_plugin_decl_registry`.)

In both fns, first statement of the body:

    semio_framework_plugin_decl_registry::artifact_registration::PluginRuntimeRegistry::publish_framework_schema_documents().expect("the OS's own schema documents never conflict");

(`new_app_with_registry_and_members`: before `let definition = manifest().definition;`; `new_registered_app_with_members`: before `let definition = manifest.await.definition;`.) The catalog tolerates exact duplicates, so it is idempotent. After applying, drop the same line from per-pane test `manifest()` fns (energie `🔬️mounted-bestest/🦀️.rs`).
