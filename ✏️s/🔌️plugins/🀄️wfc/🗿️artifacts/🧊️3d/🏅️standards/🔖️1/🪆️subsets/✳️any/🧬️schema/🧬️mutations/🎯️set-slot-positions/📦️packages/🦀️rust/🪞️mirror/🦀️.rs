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
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::tower_stack;
                    #[path = "."]
                    pub mod two_room_corridor {
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::two_room_corridor::*;
                        #[cfg(test)]
                        #[path = "../../../../../../📚️examples/🚪️two-room-corridor/🧪️tests/🧩️example/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod wall_roof_facade_strip {
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::wall_roof_facade_strip::*;
                        #[cfg(test)]
                        #[path = "../../../../../../📚️examples/🧱️wall-roof-facade-strip/🧪️tests/🧩️example/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod set_slot_positions {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            #[path = "../../../../📐️resize-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📐️resize-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📐️resize-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            #[path = "../../../../🔗️connect-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔗️connect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔗️connect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            #[path = "../../../../✂️disconnect-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✂️disconnect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✂️disconnect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            #[path = "../../../../📌️pin-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📌️pin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📌️pin-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            #[path = "../../../../📍️unpin-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📍️unpin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📍️unpin-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            #[path = "../../../../⚖️change-tile-weight/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../⚖️change-tile-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../⚖️change-tile-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
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
    pub mod wfc3d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod graph {
                        pub use semio_s_artifact_wfc_3d_schema::editor::wfc3d::modes::edit::windows::graph::*;
                        #[cfg(test)]
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
