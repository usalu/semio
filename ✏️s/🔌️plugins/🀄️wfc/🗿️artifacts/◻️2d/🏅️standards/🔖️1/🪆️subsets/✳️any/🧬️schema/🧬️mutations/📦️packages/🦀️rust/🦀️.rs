//! 🧬️ mut-any of `semio-s-artifact-wfc-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
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
                pub mod schema {
                    pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_slot {
                            #[path = "../../🧩️create-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧩️create-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧩️create-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            #[path = "../../🕳️delete-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🕳️delete-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🕳️delete-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "../../🀄️create-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🀄️create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🀄️create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "../../🗑️delete-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗑️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "../../🚦️create-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "../../❌delete-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../❌delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../❌delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
