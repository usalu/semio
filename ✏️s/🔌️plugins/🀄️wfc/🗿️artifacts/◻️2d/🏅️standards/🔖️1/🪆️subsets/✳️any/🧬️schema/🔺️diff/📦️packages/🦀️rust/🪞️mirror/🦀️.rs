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
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    pub mod diff;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "../../../../🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            #[path = "../../../../🧬️mutations/↔️move-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/↔️move-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/↔️move-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            #[path = "../../../../🧬️mutations/📐️resize-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/📐️resize-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📐️resize-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            #[path = "../../../../🧬️mutations/🔗️connect-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🔗️connect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🔗️connect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            #[path = "../../../../🧬️mutations/✂️disconnect-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/✂️disconnect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/✂️disconnect-slots/↩️inverse/🦀️.rs"]
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
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod transient {
            pub use semio_s_artifact_wfc_2d_transient::editor::wfc2d::transient::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
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
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
