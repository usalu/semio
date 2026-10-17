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
                        pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::io::binary::snapshot;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod pin_slot {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            #[path = "../../../../🔓️unpin-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔓️unpin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔓️unpin-slot/↩️inverse/🦀️.rs"]
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
                        #[path = "."]
                        pub mod change_tile_media {
                            #[path = "../../../../🎨️change-tile-media/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🎨️change-tile-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🎨️change-tile-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
    pub mod wfc2d {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_wfc_2d_config::editor::wfc2d::config::*;
            #[cfg(test)]
            #[path = "../../../../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod graph {
                        pub use semio_s_artifact_wfc_2d_diff::editor::wfc2d::modes::edit::windows::graph::*;
                        #[cfg(test)]
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🧪️tests/🔬️unit/🦀️.rs"]
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
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "../../../../../../📚️examples/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../../../../../📚️examples/🚪️two-room-corridor/🦀️.rs"]
    pub mod two_room_corridor;
    #[path = "../../../../../../📚️examples/🧱️wall-roof-facade-strip/🦀️.rs"]
    pub mod wall_roof_facade_strip;
    #[path = "../../../../../../📚️examples/🔷️hex-ring/🦀️.rs"]
    pub mod hex_ring;
    #[path = "../../../../../../📚️examples/🗺️terrain-ring/🦀️.rs"]
    pub mod terrain_ring;
}
