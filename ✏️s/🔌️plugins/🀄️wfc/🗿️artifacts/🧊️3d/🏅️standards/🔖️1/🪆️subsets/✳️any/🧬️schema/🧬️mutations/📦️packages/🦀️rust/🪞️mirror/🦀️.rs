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
                pub mod examples {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::two_room_corridor;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../🧩️create-slot/🧪️tests/🧩️inserts/🦀️.rs"]
                            pub mod tests_inserts_room_c_at_the_sorted_position;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../🕳️delete-slot/🧪️tests/🕳️removes/🦀️.rs"]
                            pub mod tests_removes_room_a_and_cascades_its_edge;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../🚚️move-slot/🧪️tests/🚚️lifts-room-b-one-storey/🦀️.rs"]
                            pub mod tests_lifts_room_b_one_storey;
                        }
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;
                            #[cfg(test)]
                            #[path = "../../../✋️drag-slots/🧪️tests/✋️lifts-both-rooms-two-storeys/🦀️.rs"]
                            pub mod tests_lifts_both_rooms_two_storeys;
                            #[cfg(test)]
                            #[path = "../../../✋️drag-slots/🧪️tests/⚠️skips/🦀️.rs"]
                            pub mod tests_skips_a_slot_the_corridor_lacks;
                        }
                        #[path = "."]
                        pub mod set_slot_positions {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::inverse;
                            #[cfg(test)]
                            #[path = "../../../🎯️set-slot-positions/🧪️tests/🎯️sets/🦀️.rs"]
                            pub mod tests_sets_the_corridor_and_room_b;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../📐️resize-slot/🧪️tests/📐️widens/🦀️.rs"]
                            pub mod tests_widens_room_a_to_a_double_bay;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                            #[cfg(test)]
                            #[path = "../../../🔗️connect-slots/🧪️tests/🔗️joins/🦀️.rs"]
                            pub mod tests_joins_room_a_to_room_b_beside;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;
                            #[cfg(test)]
                            #[path = "../../../✂️disconnect-slots/🧪️tests/✂️severs/🦀️.rs"]
                            pub mod tests_severs_the_corridor_b_edge;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../📌️pin-slot/🧪️tests/📌️pins-room-a-to-the-room-tile/🦀️.rs"]
                            pub mod tests_pins_room_a_to_the_room_tile;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;
                            #[cfg(test)]
                            #[path = "../../../📍️unpin-slot/🧪️tests/📍️releases-the-pin-on-room-a/🦀️.rs"]
                            pub mod tests_releases_the_pin_on_room_a;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "../../../🀄️create-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🀄️create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🀄️create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🀄️create-tile/🧪️tests/🀄️adds/🦀️.rs"]
                            pub mod tests_adds_a_stair_tile_to_the_catalogue;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "../../../🗑️delete-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🗑️delete-tile/🧪️tests/🗑️drops/🦀️.rs"]
                            pub mod tests_drops_the_corridor_tile_and_its_rule;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                            #[cfg(test)]
                            #[path = "../../../⚖️change-tile-weight/🧪️tests/⚖️raises/🦀️.rs"]
                            pub mod tests_raises_the_room_tile_selection_bias;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                            #[cfg(test)]
                            #[path = "../../../🖼️change-tile-media/🧪️tests/🖼️swaps/🦀️.rs"]
                            pub mod tests_swaps_the_corridor_box_for_a_wedge;
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
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
