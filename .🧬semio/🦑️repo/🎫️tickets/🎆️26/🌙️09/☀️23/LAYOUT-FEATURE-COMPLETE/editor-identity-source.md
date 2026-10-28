# Canonical Child Editor Identity

Editor::builder in framework plugin declaration/surface/surface-builders uses the shared semio_framework::surface_app_id(&ArtifactDialect,AppRole::Editor). Genuine Draw registration must call that canonical owner rather than assume a #editor suffix. The framework facade explicitly reexports this function from semio_framework_manifest_app_surface. This source research only reads the canonical builders and does not claim a child command executor is registered.

The concrete canonical spelling owner is manifest/app-surface/app-definition: surface_app_id formats dialect.to_coordinate() followed by the role. MemberStoreProducer lives in store/initialization/owners/producer-member/schema (actual path: 🏪️store/🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs), reexported by semio_framework_os_store_engine_owners::member_producer.
