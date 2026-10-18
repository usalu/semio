//! 🧩️ entities of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod sheet_layout {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::title_block;
                        }
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify;
                        pub use semio_s_artifact_bim_model_mut_any_174::standards::v1::subsets::any::schema::mutations::set_wall_top;
                        pub use semio_s_artifact_bim_model_mut_any_63::standards::v1::subsets::any::schema::mutations::set_railing;
                        pub use semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space;
                        pub use semio_s_artifact_bim_model_mut_any_5::standards::v1::subsets::any::schema::mutations::set_slab_boundary;
                        pub use semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening;
                        pub use semio_s_artifact_bim_model_mut_any_17::standards::v1::subsets::any::schema::mutations::set_project_info;
                        pub use semio_s_artifact_bim_model_mut_any_86::standards::v1::subsets::any::schema::mutations::rename_element;
                        pub use semio_s_artifact_bim_model_mut_any_89::standards::v1::subsets::any::schema::mutations::create_ceiling_type;
                        pub use semio_s_artifact_bim_model_mut_any_135::standards::v1::subsets::any::schema::mutations::create_ceiling;
                        pub use semio_s_artifact_bim_model_mut_any_26::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary;
                        pub use semio_s_artifact_bim_model_mut_any_27::standards::v1::subsets::any::schema::mutations::set_ceiling;
                        pub use semio_s_artifact_bim_model_mut_any_136::standards::v1::subsets::any::schema::mutations::create_ramp;
                        pub use semio_s_artifact_bim_model_mut_any_137::standards::v1::subsets::any::schema::mutations::create_view;
                        pub use semio_s_artifact_bim_model_mut_any_94::standards::v1::subsets::any::schema::mutations::create_annotation_style;
                        pub use semio_s_artifact_bim_model_mut_any_138::standards::v1::subsets::any::schema::mutations::create_dimension;
                        pub use semio_s_artifact_bim_model_mut_any_30::standards::v1::subsets::any::schema::mutations::set_dimension;
                        pub use semio_s_artifact_bim_model_mut_any_139::standards::v1::subsets::any::schema::mutations::create_tag;
                        pub use semio_s_artifact_bim_model_mut_any_140::standards::v1::subsets::any::schema::mutations::create_text_note;
                        pub use semio_s_artifact_bim_model_mut_any_141::standards::v1::subsets::any::schema::mutations::create_leader;
                        pub use semio_s_artifact_bim_model_mut_any_142::standards::v1::subsets::any::schema::mutations::create_zone;
                        pub use semio_s_artifact_bim_model_mut_any_143::standards::v1::subsets::any::schema::mutations::create_area_scheme;
                        pub use semio_s_artifact_bim_model_mut_any_100::standards::v1::subsets::any::schema::mutations::set_area_scheme;
                        pub use semio_s_artifact_bim_model_mut_any_102::standards::v1::subsets::any::schema::mutations::create_schedule;
                        pub use semio_s_artifact_bim_model_mut_any_35::standards::v1::subsets::any::schema::mutations::set_schedule;
                        pub use semio_s_artifact_bim_model_mut_any_178::standards::v1::subsets::any::schema::mutations::set_wall_end_join;
                        pub use semio_s_artifact_bim_model_mut_any_145::standards::v1::subsets::any::schema::mutations::create_sheet;
                        pub use semio_s_artifact_bim_model_mut_any_146::standards::v1::subsets::any::schema::mutations::create_viewport;
                        pub use semio_s_artifact_bim_model_mut_any_147::standards::v1::subsets::any::schema::mutations::create_sheet_revision;
                        pub use semio_s_artifact_bim_model_mut_any_108::standards::v1::subsets::any::schema::mutations::create_property_template;
                        pub use semio_s_artifact_bim_model_mut_any_148::standards::v1::subsets::any::schema::mutations::create_classification_system;
                        pub use semio_s_artifact_bim_model_mut_any_41::standards::v1::subsets::any::schema::mutations::set_classification_system;
                        pub use semio_s_artifact_bim_model_mut_any_149::standards::v1::subsets::any::schema::mutations::create_family;
                        pub use semio_s_artifact_bim_model_mut_any_186::standards::v1::subsets::any::schema::mutations::create_wall_sweep;
                        pub use semio_s_artifact_bim_model_mut_any_188::standards::v1::subsets::any::schema::mutations::set_wall_base_slab;
                        pub use semio_s_artifact_bim_model_mut_any_115::standards::v1::subsets::any::schema::mutations::create_option_group;
                        pub use semio_s_artifact_bim_model_mut_any_116::standards::v1::subsets::any::schema::mutations::create_design_option;
                        pub use semio_s_artifact_bim_model_mut_any_117::standards::v1::subsets::any::schema::mutations::create_workset;
                        pub use semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option;
                        pub use semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset;
                        pub use semio_s_artifact_bim_model_mut_any_150::standards::v1::subsets::any::schema::mutations::create_clash_set;
                        pub use semio_s_artifact_bim_model_mut_any_151::standards::v1::subsets::any::schema::mutations::create_rule;
                        pub use semio_s_artifact_bim_model_mut_any_152::standards::v1::subsets::any::schema::mutations::create_issue;
                        pub use semio_s_artifact_bim_model_mut_any_153::standards::v1::subsets::any::schema::mutations::create_issue_comment;
                        pub use semio_s_artifact_bim_model_mut_any_154::standards::v1::subsets::any::schema::mutations::create_component;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::remove_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_157::standards::v1::subsets::any::schema::mutations::create_mep_element;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod entities {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod schedule {
                        pub use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit;
                    }
                    #[path = "."]
                    pub mod family {
                        pub use semio_s_artifact_bim_model_viewer::editor::bim::modes::edit::windows::family::edit;
                        pub use semio_s_artifact_bim_model_wall_layout::editor::bim::modes::edit::windows::family::vocabulary;
                    }
                }
            }
        }
    }
}
