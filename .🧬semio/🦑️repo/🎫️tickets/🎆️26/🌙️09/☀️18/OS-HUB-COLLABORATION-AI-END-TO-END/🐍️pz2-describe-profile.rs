// 🔬️ PZ2's native describe-path profiler for 🧩️puzzle (2026-09-22). NOT compiled from here: paste
// this region into `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️surface/🦀️.rs` and run
//   CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-pz2 \
//   cargo test -p semio-s-plugin-puzzle --lib -- --exact surface_tests::pz2_describe_total --nocapture
// It times exactly what the owned guest's `__semio_describe_component` runs — install_bundle →
// plugin_manifest → describe_plugin — so the fuel cliff can be attributed without a wasm32 build
// or the fleet mutex. Kept OUT of the crate because it asserts nothing; the laws that do are the
// three `shipped_*_kinds_are_the_two_examples_own_catalog_rows` in the editors' own unit tests.
// Numbers it produced: 📓️pz2-puzzle-describe-under-budget.md §2.

//#region 🔖️Pz2DescribeProfile
/// ⏱️ Times one phase of the describe path and prints it as a `pz2-phase` row for
/// `🗑️generated/pz2-native-profile-*.txt`.
fn pz2_phase<T>(label: &str, body: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let value = body();
    println!("pz2-phase {label} {}ms", start.elapsed().as_millis());
    value
}

/// 🗂️ Prints every select-shaped action argument of one `AppDefinition` as `pz2-options` rows, so
/// the example-derived option sets an `AppDefinition` carries can be read without a describe.
fn pz2_dump_options(label: &str, app: &semio_framework_plugin::AppDefinition) {
    for action in &app.actions {
        for arg in &action.args {
            if let semio_framework_plugin::manifest::ArgSchema::String { options, .. } = &arg.schema {
                if !options.is_empty() {
                    println!("pz2-options {label} {} {} {} [{}]", action.id, arg.id, options.len(), options.iter().map(|option| format!("{}|{:?}", option.value, option.label)).collect::<Vec<_>>().join(", "));
                }
            }
        }
    }
}

/// 🔬️ PZ2's native attribution of `🧩️puzzle`'s `describe()` cost: every phase the owned guest runs
/// between `semio_plugin_install_bundle` and the encoded `PackageDescriptor`, timed on a native
/// build so the fuel cliff can be located without a wasm32 build or the fleet mutex. Run with
/// `cargo test -p semio-s-plugin-puzzle --lib pz2_describe_profile -- --nocapture --test-threads=1`.
#[test]
fn pz2_describe_profile() {
    let projection = pz2_phase("parse-dsl-5d-nakagin", || semio_s_artifact_puzzle_5d::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(semio_s_artifact_puzzle_5d::examples::puzzle5d::nakagin_capsule_tower::DSL_TEXT).expect("5d nakagin dsl parses"));
    drop(projection);
    let json = pz2_phase("document-json-5d-nakagin", || semio_s_artifact_puzzle_5d::examples::puzzle5d::nakagin_capsule_tower::SOURCE.document_json());
    println!("pz2-size json-5d-nakagin {}", json.len());
    drop(json);
    pz2_phase("create-puzzle2d-app", semio_s_artifact_puzzle_2d::editor::puzzle2d::create_puzzle2d_app);
    pz2_phase("create-puzzle2d-viewer", semio_s_artifact_puzzle_2d::viewer::puzzle2d::create_puzzle2d_viewer);
    pz2_phase("create-puzzle3d-app", semio_s_artifact_puzzle_3d::editor::puzzle3d::create_puzzle3d_app);
    pz2_phase("create-puzzle3d-viewer", semio_s_artifact_puzzle_3d::viewer::puzzle3d::create_puzzle3d_viewer);
    pz2_phase("create-puzzle5d-app", semio_s_artifact_puzzle_5d::editor::puzzle5d::create_puzzle5d_app);
    pz2_phase("create-puzzle5d-viewer", semio_s_artifact_puzzle_5d::viewer::puzzle5d::create_puzzle5d_viewer);
    pz2_phase("create-puzzle5d-app-warm", semio_s_artifact_puzzle_5d::editor::puzzle5d::create_puzzle5d_app);
    pz2_dump_options("puzzle2d", &semio_s_artifact_puzzle_2d::editor::puzzle2d::create_puzzle2d_app());
    pz2_dump_options("puzzle3d", &semio_s_artifact_puzzle_3d::editor::puzzle3d::create_puzzle3d_app());
    pz2_dump_options("puzzle5d", &semio_s_artifact_puzzle_5d::editor::puzzle5d::create_puzzle5d_app());
    pz2_phase("artifact-declaration-2d", semio_s_artifact_puzzle_2d::artifact::<crate::PuzzleApps>);
    pz2_phase("artifact-declaration-3d", semio_s_artifact_puzzle_3d::artifact::<crate::PuzzleApps>);
    pz2_phase("artifact-declaration-5d", semio_s_artifact_puzzle_5d::artifact::<crate::PuzzleApps>);
    pz2_phase("plugin-builder-warm", || crate::plugin().expect("puzzle plugin assembles"));
    pz2_phase("install-plugin-bundle", crate::__semio_install_plugin_bundle);
    let manifest = pz2_phase("plugin-manifest", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime))));
    println!("pz2-size manifest-examples {}", manifest.examples.len());
    println!("pz2-size manifest-apps {}", manifest.apps.len());
    pz2_phase("plugin-manifest-again", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime))));
    let descriptor = pz2_phase("describe-plugin", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(runtime))));
    println!("pz2-size descriptor-bytes {}", descriptor.len());
}

/// 🔬️ The describe path exactly as the owned guest runs it, in a process where nothing has warmed
/// a static: `semio_plugin_install_bundle` → `plugin_manifest` → `describe_plugin`. Run ALONE
/// (`cargo test -p semio-s-plugin-puzzle --lib pz2_describe_total -- --nocapture`) so the filter
/// leaves every `LazyLock` cold, which is the only way this number is the guest's number.
#[test]
fn pz2_describe_total() {
    let start = std::time::Instant::now();
    pz2_phase("install-plugin-bundle-cold", crate::__semio_install_plugin_bundle);
    let manifest = pz2_phase("plugin-manifest-cold", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime))));
    println!("pz2-size manifest-examples {}", manifest.examples.len());
    let descriptor = pz2_phase("describe-plugin-cold", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(runtime))));
    println!("pz2-size descriptor-bytes {}", descriptor.len());
    println!("pz2-phase describe-path-total {}ms", start.elapsed().as_millis());
}

/// 🔬️ The SAME describe path with the six example documents materialised first — which is exactly
/// what `create_puzzle{2d,3d,5d}_app()` used to do while assembling their kind selects, so this is
/// the pre-PZ2 total reconstructed in one process rather than estimated. Run ALONE.
#[test]
fn pz2_describe_total_with_example_documents() {
    let start = std::time::Instant::now();
    pz2_phase("force-example-documents", || {
        drop(semio_s_artifact_puzzle_2d::editor::puzzle2d::concrete_forest_example_json());
        drop(semio_s_artifact_puzzle_2d::editor::puzzle2d::nakagin_example_json());
        drop(semio_s_artifact_puzzle_3d::editor::puzzle3d::default_fixture());
        drop(semio_s_artifact_puzzle_3d::editor::puzzle3d::nakagin_fixture());
        drop(semio_s_artifact_puzzle_5d::editor::puzzle5d::concrete_forest_example_document());
        drop(semio_s_artifact_puzzle_5d::editor::puzzle5d::nakagin_example_document());
    });
    pz2_phase("install-plugin-bundle-cold", crate::__semio_install_plugin_bundle);
    pz2_phase("plugin-manifest-cold", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime))));
    pz2_phase("describe-plugin-cold", || crate::__SEMIO_PLUGIN_RUNTIME.with(|runtime| semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(runtime))));
    println!("pz2-phase describe-path-total-with-example-documents {}ms", start.elapsed().as_millis());
}
//#endregion 🔖️Pz2DescribeProfile
