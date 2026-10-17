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
                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "../../../../🗑️delete-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🗑️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🗑️delete-tile/↩️inverse/🦀️.rs"]
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
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "../../../../🚦️create-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "../../../../❌delete-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../❌delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../❌delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            #[path = "../../../../📌️pin-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📌️pin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📌️pin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            #[path = "../../../../📍️unpin-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📍️unpin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📍️unpin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json::*;
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid2d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_wfc_grid2d_diff::viewer::grid2d::modes::view::windows;
            }
        }
    }
}
