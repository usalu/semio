### 15.5 Glue `#[path]` convention (leaf-prefixed + grouping `#[path = "."]`)

This crate uses the **leaf-prefixed** convention: grouping modules reset with `#[path = "."]` so nested `snapshot` / `diff` keep the same `../../` leaf prefix (no extra `../`).

```rust
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/💠️lowpoly/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod objects_add {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️objects-add/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_remove {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️objects-remove/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_move {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/↔️objects-move/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod objects_patch {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹objects-patch/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod add_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➕️add-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/➖️remove-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_paint_layer {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🩹patch-paint-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod paint_stroke {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖌️paint-stroke/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/🗣️dsl/🦀️component.rs"]
        pub mod dsl;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/💠️lowpoly/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/💠️lowpoly/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🎨️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🧵️media/🦀️component.rs"]
            pub mod media;
            pub use media::{lowpoly_document_from_mesh, lowpoly_mesh_from_document, mesh_data_from_transfer, mesh_document_from_mesh, mesh_from_mesh_document};
            pub use paint::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
        }
    }
}
//#endregion 🗿️Artifacts
```

