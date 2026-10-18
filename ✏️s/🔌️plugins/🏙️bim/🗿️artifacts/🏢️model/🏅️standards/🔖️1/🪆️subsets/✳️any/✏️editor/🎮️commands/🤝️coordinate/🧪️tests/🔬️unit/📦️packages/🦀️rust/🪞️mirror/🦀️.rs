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
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        pub use semio_s_artifact_bim_model_mut_any_40::standards::v1::subsets::any::schema::mutations::set_property_template;
                        pub use semio_s_artifact_bim_model_mut_any_41::standards::v1::subsets::any::schema::mutations::set_classification_system;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::remove_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::set_space_conditions;
                        pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::remove_space_conditions;
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
        pub use semio_s_artifact_bim_model_bim::editor::bim::*;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        #[path = "."]
        pub mod gestures {
            pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures::*;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::typed;
        }
        pub use semio_s_artifact_bim_model_tests::editor::bim::presence;
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_bim_model_bim::editor::bim::fixture;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::*;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property;
            #[path = "."]
            pub mod coordinate {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::coordinate::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::coordinate::*;
                #[cfg(test)]
                #[path = "../../../🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_classification {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_classification::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_classification::*;
                #[cfg(test)]
                #[path = "../../../../../../🗂️set-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod remove_classification {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_classification::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_classification::*;
                #[cfg(test)]
                #[path = "../../../../../../🗄️remove-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod apply_template {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::apply_template::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::apply_template::*;
                #[cfg(test)]
                #[path = "../../../../../../🧰️apply-template/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod conditions {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::conditions::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::conditions::*;
                #[cfg(test)]
                #[path = "../../../../../../🌡️conditions/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_template {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_template::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_template::*;
                #[cfg(test)]
                #[path = "../../../../../../🧮️edit-template/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_classification {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_classification::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_classification::*;
                #[cfg(test)]
                #[path = "../../../../../../📚️edit-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod search_classification {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::search_classification::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::search_classification::*;
                #[cfg(test)]
                #[path = "../../../../../../🔍️search-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod cursor_keys {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys::*;
                #[cfg(test)]
                #[path = "../../../../../../🧭️cursor-keys/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place_elements {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_elements::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_elements::*;
                #[cfg(test)]
                #[path = "../../../../../../📍️place-elements/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place_grid_columns {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_grid_columns::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_grid_columns::*;
                #[cfg(test)]
                #[path = "../../../../../../🏛️place-grid-columns/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod engagement_input {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_input::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_input::*;
                #[cfg(test)]
                #[path = "../../../../../../⌨️engagement-input/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod browse_families {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::browse_families::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::browse_families::*;
                #[cfg(test)]
                #[path = "../../../../../../🪑️browse-families/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_override {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_override::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_override::*;
                #[cfg(test)]
                #[path = "../../../../../../🎚️set-override/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod gesture_keys {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::gesture_keys::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::gesture_keys::*;
                #[cfg(test)]
                #[path = "../../../../../../🔑️gesture-keys/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod engagement_submit {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit::*;
                #[cfg(test)]
                #[path = "../../../../../../📨️engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
