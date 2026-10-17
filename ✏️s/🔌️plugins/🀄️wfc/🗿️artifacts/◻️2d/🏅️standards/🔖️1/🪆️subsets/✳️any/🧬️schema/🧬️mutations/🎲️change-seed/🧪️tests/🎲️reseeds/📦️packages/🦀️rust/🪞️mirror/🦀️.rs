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
                    pub mod binary {
                        #[path = "../../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🧩️create-slot/🧪️tests/🧩️inserts/🦀️.rs"]
                            pub mod tests_inserts_slot_d_in_canonical_order;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🕳️delete-slot/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_slot_a_and_cascades_edge_ab;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../↔️move-slot/🧪️tests/↔️drags-slot-b-down/🦀️.rs"]
                            pub mod tests_drags_slot_b_down;
                        }
                        #[path = "."]
                        pub mod drag_slots {
                            #[path = "../../../../../../✋️drag-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../✋️drag-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../✋️drag-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-slots/🧪️tests/✋️drags-slots-a-and-b-together/🦀️.rs"]
                            pub mod tests_drags_slots_a_and_b_together;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-slots/🧪️tests/⚠️skips-a-slot-the-board-lacks/🦀️.rs"]
                            pub mod tests_skips_a_slot_the_board_lacks;
                        }
                        #[path = "."]
                        pub mod set_slot_positions {
                            #[path = "../../../../../../🎯️set-slot-positions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🎯️set-slot-positions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🎯️set-slot-positions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🎯️set-slot-positions/🧪️tests/🎯️sets-slots-a-and-c/🦀️.rs"]
                            pub mod tests_sets_slots_a_and_c;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../📐️resize-slot/🧪️tests/📐️widens-slot-b/🦀️.rs"]
                            pub mod tests_widens_slot_b;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🔗️connect-slots/🧪️tests/🔗️joins-slot-a-to-slot-c/🦀️.rs"]
                            pub mod tests_joins_slot_a_to_slot_c;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../✂️disconnect-slots/🧪️tests/✂️severs-edge-ab/🦀️.rs"]
                            pub mod tests_severs_edge_ab;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../📌️pin-slot/🧪️tests/📌️pins-slot-a-to-the-roof-tile/🦀️.rs"]
                            pub mod tests_pins_slot_a_to_the_roof_tile;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🔓️unpin-slot/🧪️tests/🔓️releases-the-slot-c-pin/🦀️.rs"]
                            pub mod tests_releases_the_slot_c_pin;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🀄️create-tile/🧪️tests/🀄️adds-the-window-tile/🦀️.rs"]
                            pub mod tests_adds_the_window_tile;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete-tile/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_the_wall_tile_and_cascades_rules_and_pins;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../⚖️change-tile-weight/🧪️tests/⚖️raises/🦀️.rs"]
                            pub mod tests_raises_the_wall_tile_bias;
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
                }
            }
        }
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
