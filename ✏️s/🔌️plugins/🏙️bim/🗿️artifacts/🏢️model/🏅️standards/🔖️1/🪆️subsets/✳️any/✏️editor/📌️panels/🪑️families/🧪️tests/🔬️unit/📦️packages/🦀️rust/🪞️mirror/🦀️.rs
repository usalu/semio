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
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc;
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::io::export::gltf;
                        pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg;
                        #[path = "."]
                        pub mod csv {
                            pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec;
                        }
                        pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod view_linework {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::view_linework::frame;
                        }
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;
                        pub use semio_s_artifact_bim_model_mut_any_85::standards::v1::subsets::any::schema::mutations::delete_elements;
                        pub use semio_s_artifact_bim_model_mut_any_137::standards::v1::subsets::any::schema::mutations::create_view;
                        pub use semio_s_artifact_bim_model_mut_any_144::standards::v1::subsets::any::schema::mutations::set_element_storey;
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
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        #[path = "."]
        pub mod gestures {
            pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures::*;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
        }
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_bim_model_bim::editor::bim::fixture;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        pub use semio_s_artifact_bim_model_viewer::editor::bim::modes::edit::windows::family::edit;
                        pub use semio_s_artifact_bim_model_wall_layout::editor::bim::modes::edit::windows::family::vocabulary;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    #[path = "."]
                    pub mod schedule {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties;
            #[path = "."]
            pub mod families {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::families::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::families::*;
                #[cfg(test)]
                #[path = "../../../🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::*;
            #[path = "."]
            pub mod create_entity {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🏗️create-entity/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod create_view {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_view::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_view::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔭️create-view/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod delete_selection {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::delete_selection::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::delete_selection::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod rename_entity {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::rename_entity::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::rename_entity::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🏷️rename-entity/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_field {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🩹️set-field/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_view {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_view::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_view::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🪟️set-view/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_family {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_family::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_family::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🧬️edit-family/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_schedule {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_schedule::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_schedule::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📋️edit-schedule/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_schedule_csv {
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_schedule_csv::*;
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_schedule_csv::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📊️export-schedule-csv/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod analyse_model {
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::analyse_model::*;
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::analyse_model::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔎️analyse-model/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_model {
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::*;
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📤️export-model/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_sheets {
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_sheets::*;
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_sheets::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📄️export-sheets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_camera {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_camera::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_camera::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_pointer_down {
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_down::*;
                pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_down::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🖱️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_commit_draft {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_commit_draft::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_commit_draft::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/✅️canvas-commit-draft/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_escape {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_escape::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_escape::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🚪️canvas-escape/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod world_pointer_down {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_down::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_down::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🌍️world-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod world_pointer_move {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_move::*;
                pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_move::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🌐️world-pointer-move/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod arm_utility {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::arm_utility::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::arm_utility::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🛠️arm-utility/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod flip_walls {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::flip_walls::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::flip_walls::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔃️flip-walls/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod attach_walls {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::attach_walls::*;
                pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::attach_walls::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔗️attach-walls/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod move_storey {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::move_storey::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::move_storey::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🪜️move-storey/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod split_wall {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::split_wall::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::split_wall::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/✂️split-wall/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_property {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property::*;
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🧾️set-property/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
