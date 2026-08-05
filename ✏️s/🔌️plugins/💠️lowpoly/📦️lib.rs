//! 💠️ Lowpoly plugin — mesh + paint editor bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod lowpoly {
        #[path = "🗿️artifacts/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🗿️artifacts/💠️lowpoly/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "🗿️artifacts/💠️lowpoly/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "🗿️artifacts/💠️lowpoly/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "🗿️artifacts/💠️lowpoly/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "🗿️artifacts/💠️lowpoly/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "🗿️artifacts/💠️lowpoly/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "🗿️artifacts/💠️lowpoly/⚙️engine/🦀️paint.rs"]
            pub mod paint;
            #[path = "🗿️artifacts/💠️lowpoly/⚙️engine/🦀️media.rs"]
            pub mod media;
            pub use media::{lowpoly_document_from_mesh, lowpoly_mesh_from_document, mesh_data_from_transfer, mesh_document_from_mesh, mesh_from_mesh_document};
            pub use paint::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod lowpoly {
        #[path = "🎛️apps/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🎛️apps/💠️lowpoly/🦀️config.rs"]
        pub mod config;
        #[path = "🎛️apps/💠️lowpoly/🦀️terminology.rs"]
        pub mod terminology;
        #[path = "🎛️apps/💠️lowpoly/🦀️view.rs"]
        pub mod view;
        #[path = "🎛️apps/💠️lowpoly/🦀️session.rs"]
        pub mod session;

        #[path = "."]
        pub mod commands {
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/➕️add-primitive/🦀️component.rs"]
            pub mod add_primitive;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/✏️patch-object/🦀️component.rs"]
            pub mod patch_object;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🔷️mesh-edit/🦀️component.rs"]
            pub mod mesh_edit;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🧵️uv/🦀️component.rs"]
            pub mod uv;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🧲️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🖌️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🌍️world/🦀️component.rs"]
            pub mod world;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/📄️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "🎛️apps/💠️lowpoly/🎮️commands/👁️chrome/🦀️component.rs"]
            pub mod chrome;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🎛️apps/💠️lowpoly/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🎛️apps/💠️lowpoly/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs"]
                    pub mod model;
                }
            }

            #[path = "."]
            pub mod paint {
                #[path = "🎛️apps/💠️lowpoly/🎭️modes/🎨️paint/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🎛️apps/💠️lowpoly/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️component.rs"]
                    pub mod uv;
                }
            }
        }

        #[path = "."]
        pub mod options {
            #[path = "🎛️apps/💠️lowpoly/🎚️options/👁️show-edges/🦀️component.rs"]
            pub mod show_edges;
            #[path = "🎛️apps/💠️lowpoly/🎚️options/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "🎛️apps/💠️lowpoly/🎚️options/🧲️snap/🦀️component.rs"]
            pub mod snap;
            #[path = "🎛️apps/💠️lowpoly/🎚️options/🗂️select/🦀️component.rs"]
            pub mod select;
            #[path = "🎛️apps/💠️lowpoly/🎚️options/🖌️paint-params-brush/🦀️component.rs"]
            pub mod paint_params_brush;
            #[path = "🎛️apps/💠️lowpoly/🎚️options/🧽️paint-params-eraser/🦀️component.rs"]
            pub mod paint_params_eraser;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🎛️apps/💠️lowpoly/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "🎛️apps/💠️lowpoly/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "🎛️apps/💠️lowpoly/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "🎛️apps/💠️lowpoly/📌️panels/🗂️layers/🦀️component.rs"]
            pub mod layers;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
/// 🔌️ One call per `MeshExporter`/`MeshImporter` format so the OS workflow VFS auto-populates from
/// `required_os_media_export_formats`/`required_os_media_import_formats`; also registers the
/// `DocumentPack` codec so `.pack`/`.ops` sync/storage paths can encode/decode `LowpolyProjection`.
fn register_lowpoly_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::lowpoly::LowpolyPlayApp>(artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.lowpoly", "lowpoly", artifacts::lowpoly::engine::lowpoly_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.lowpoly", artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.lowpoly", artifacts::lowpoly::engine::lowpoly_document_from_mesh);
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.mesh", "mesh", artifacts::lowpoly::engine::mesh_from_mesh_document);
    semio_framework_os::register_mesh_importer("3d.mesh", artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.mesh", artifacts::lowpoly::engine::mesh_document_from_mesh);
}

semio_framework_plugin::semio_plugin! {
    id: "lowpoly", label: "Lowpoly", version: "0.1.0",
    setup: register_lowpoly_exports,
    apps: [ apps::lowpoly::create_lowpoly_app => apps::lowpoly::LowpolyPlayApp ],
}
//#endregion 🔖️Plugin
