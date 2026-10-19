//! 🎮️ `s.fem.fem3d` document commands: add, patch, remove and document settings.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem3d {
        #[path = "."]
        pub mod commands {
            #[path = "../../🏋️add-area-load/🦀️.rs"]
            pub mod add_area_load;
            #[path = "../../➖️add-bar/🦀️.rs"]
            pub mod add_bar;
            #[path = "../../🔗️add-combination/🦀️.rs"]
            pub mod add_combination;
            #[path = "../../🖼️add-frame/🦀️.rs"]
            pub mod add_frame;
            #[path = "../../📋️add-load-case/🦀️.rs"]
            pub mod add_load_case;
            #[path = "../../🧱️add-material/🦀️.rs"]
            pub mod add_material;
            #[path = "../../📏️add-member-udl/🦀️.rs"]
            pub mod add_member_udl;
            #[path = "../../📍️add-nodal-load/🦀️.rs"]
            pub mod add_nodal_load;
            #[path = "../../⚪️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../📐️add-section/🦀️.rs"]
            pub mod add_section;
            #[path = "../../🧊️add-solid/🦀️.rs"]
            pub mod add_solid;
            #[path = "../../🛡️add-support/🦀️.rs"]
            pub mod add_support;
            #[path = "../../🗂️remove-selection/🦀️.rs"]
            pub mod remove_selection;
            #[path = "../../📚️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🧮️set-analysis-settings/🦀️.rs"]
            pub mod set_analysis_settings;
            #[path = "../../⚖️set-self-weight/🦀️.rs"]
            pub mod set_self_weight;
            #[path = "../../🩹️patch-node/🦀️.rs"]
            pub mod patch_node;
            #[path = "../../🩹️patch-element/🦀️.rs"]
            pub mod patch_element;
            #[path = "../../🩹️patch-material/🦀️.rs"]
            pub mod patch_material;
            #[path = "../../🩹️patch-section/🦀️.rs"]
            pub mod patch_section;
            #[path = "../../🩹️patch-support/🦀️.rs"]
            pub mod patch_support;
            #[path = "../../🩹️patch-solid/🦀️.rs"]
            pub mod patch_solid;
            #[path = "../../🩹️patch-load/🦀️.rs"]
            pub mod patch_load;
            #[path = "../../🩹️patch-load-case/🦀️.rs"]
            pub mod patch_load_case;
            #[path = "../../🩹️patch-combination/🦀️.rs"]
            pub mod patch_combination;
        }
    }
}
