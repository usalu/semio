//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement;
                        pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify;
                        pub use semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height;
                        pub use semio_s_artifact_bim_model_mut_any_159::standards::v1::subsets::any::schema::mutations::create_column;
                        pub use semio_s_artifact_bim_model_mut_any_5::standards::v1::subsets::any::schema::mutations::set_slab_boundary;
                        pub use semio_s_artifact_bim_model_mut_any_7::standards::v1::subsets::any::schema::mutations::set_roof_footprint;
                        pub use semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening;
                        pub use semio_s_artifact_bim_model_mut_any_161::standards::v1::subsets::any::schema::mutations::set_wall_axis;
                        pub use semio_s_artifact_bim_model_mut_any_177::standards::v1::subsets::any::schema::mutations::split_wall;
                        pub use semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements;
                        pub use semio_s_artifact_bim_model_mut_any_87::standards::v1::subsets::any::schema::mutations::set_element_property;
                        pub use semio_s_artifact_bim_model_mut_any_87::standards::v1::subsets::any::schema::mutations::remove_element_property;
                        pub use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::set_element_classification;
                        pub use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::remove_element_classification;
                        pub use semio_s_artifact_bim_model_mut_any_144::standards::v1::subsets::any::schema::mutations::set_element_storey;
                        pub use semio_s_artifact_bim_model_mut_any_179::standards::v1::subsets::any::schema::mutations::copy_elements;
                        pub use semio_s_artifact_bim_model_mut_any_189::standards::v1::subsets::any::schema::mutations::mirror_elements;
                        pub use semio_s_artifact_bim_model_mut_any_180::standards::v1::subsets::any::schema::mutations::array_elements;
                        pub use semio_s_artifact_bim_model_mut_any_181::standards::v1::subsets::any::schema::mutations::align_elements;
                        pub use semio_s_artifact_bim_model_mut_any_182::standards::v1::subsets::any::schema::mutations::offset_wall;
                        pub use semio_s_artifact_bim_model_mut_any_183::standards::v1::subsets::any::schema::mutations::trim_extend_wall;
                        pub use semio_s_artifact_bim_model_mut_any_184::standards::v1::subsets::any::schema::mutations::split_slab;
                        pub use semio_s_artifact_bim_model_mut_any_185::standards::v1::subsets::any::schema::mutations::split_beam;
                        pub use semio_s_artifact_bim_model_mut_any_146::standards::v1::subsets::any::schema::mutations::create_viewport;
                        pub use semio_s_artifact_bim_model_mut_any_37::standards::v1::subsets::any::schema::mutations::set_viewport;
                        pub use semio_s_artifact_bim_model_mut_any_164::standards::v1::subsets::any::schema::mutations::set_column_tilt;
                        pub use semio_s_artifact_bim_model_mut_any_167::standards::v1::subsets::any::schema::mutations::set_curtain_wall_grid;
                        pub use semio_s_artifact_bim_model_mut_any_168::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_169::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_107::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_40::standards::v1::subsets::any::schema::mutations::set_property_template;
                        pub use semio_s_artifact_bim_model_mut_any_41::standards::v1::subsets::any::schema::mutations::set_classification_system;
                        pub use semio_s_artifact_bim_model_mut_any_152::standards::v1::subsets::any::schema::mutations::create_issue;
                        pub use semio_s_artifact_bim_model_mut_any_54::standards::v1::subsets::any::schema::mutations::set_issue;
                        pub use semio_s_artifact_bim_model_mut_any_154::standards::v1::subsets::any::schema::mutations::create_component;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::remove_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_158::standards::v1::subsets::any::schema::mutations::set_mep_element;
                        pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::set_space_conditions;
                        pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::remove_space_conditions;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod sheets {
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::testkit;
                        }
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
        #[path = "."]
        pub mod gestures {
            #[path = "../../../../../🧵️gestures/🏗️frame/🦀️.rs"]
            pub mod frame;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::opening;
            #[path = "../../../../../🧵️gestures/🫧️overlay/🦀️.rs"]
            pub mod overlay;
            pub use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane;
            #[path = "../../../../../🧵️gestures/🎯️select/🦀️.rs"]
            pub mod select;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::snap;
            #[path = "../../../../../🧵️gestures/🔪️split/🦀️.rs"]
            pub mod split;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::transform;
            #[path = "../../../../../🧵️gestures/👯️duplicate/🦀️.rs"]
            pub mod duplicate;
            #[path = "../../../../../🧵️gestures/✂️reshape/🦀️.rs"]
            pub mod reshape;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::typed;
            #[path = "../../../../../🧵️gestures/🪑️place/🦀️.rs"]
            pub mod place;
            #[path = "."]
            pub mod viewports {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::viewports::*;
                #[cfg(test)]
                #[path = "../../../../../🧵️gestures/🖼️viewports/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::flip_walls;
            #[path = "."]
            pub mod move_storey {
                #[path = "../../../🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod split_wall {
                #[path = "../../../../✂️split-wall/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_property {
                #[path = "../../../../🧾️set-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_property {
                #[path = "../../../../🗃️remove-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod select_findings {
                #[path = "../../../../🎯️select-findings/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod coordinate {
                #[path = "../../../../🤝️coordinate/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_classification {
                #[path = "../../../../🗂️set-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_classification {
                #[path = "../../../../🗄️remove-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod apply_template {
                #[path = "../../../../🧰️apply-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod conditions {
                #[path = "../../../../🌡️conditions/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_template {
                #[path = "../../../../🧮️edit-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_classification {
                #[path = "../../../../📚️edit-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod search_classification {
                #[path = "../../../../🔍️search-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_elements {
                #[path = "../../../../📍️place-elements/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_grid_columns {
                #[path = "../../../../🏛️place-grid-columns/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod browse_families {
                #[path = "../../../../🪑️browse-families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_override {
                #[path = "../../../../🎚️set-override/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
