//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_wfc_3d_inferences::host::inferences::*;
        #[cfg(test)]
        #[path="../../../../../../../../🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::tower_stack;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::two_room_corridor;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::wall_roof_facade_strip;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::inverse;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::inverse;
                        }
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;
                        }
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                    }
                    pub use semio_s_artifact_wfc_3d_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_wfc_3d_config::editor::wfc3d::config;
        #[path = "."]
        pub mod transient {
            pub use semio_s_artifact_wfc_3d_transient::editor::wfc3d::transient::*;
            #[cfg(test)]
            #[path = "../../../🫧️transient/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod tools {
                    #[path = "../../../🎭️modes/✏️edit/🛠️tools/📑️fill/🦀️.rs"]
                    pub mod fill;
                    #[path = "."]
                    pub mod drag {
                        pub use semio_s_artifact_wfc_3d_inferences::editor::wfc3d::modes::edit::tools::drag::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_3d_schema::editor::wfc3d::modes::edit::windows::graph;
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧊️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc3d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
