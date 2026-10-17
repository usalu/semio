//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_2d_inferences::host::inferences;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::inverse;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::inverse;
                        }
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;
                        }
                    }
                    pub use semio_s_artifact_wfc_2d_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc2d {
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_wfc_2d_config::editor::wfc2d::config;
        pub use semio_s_artifact_wfc_2d_transient::editor::wfc2d::transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_2d_diff::editor::wfc2d::modes::edit::windows::graph;
                    #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
                #[path = "."]
                pub mod tools {
                    #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🦀️.rs"]
                    pub mod fill;
                    #[path = "."]
                    pub mod drag {
                        pub use semio_s_artifact_wfc_2d_inferences::editor::wfc2d::modes::edit::tools::drag::*;
                        #[cfg(test)]
                        #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc2d {
        pub use semio_s_artifact_wfc_2d_inferences::viewer::wfc2d::*;
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::*;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::two_room_corridor;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::wall_roof_facade_strip;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::hex_ring;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::terrain_ring;
}
