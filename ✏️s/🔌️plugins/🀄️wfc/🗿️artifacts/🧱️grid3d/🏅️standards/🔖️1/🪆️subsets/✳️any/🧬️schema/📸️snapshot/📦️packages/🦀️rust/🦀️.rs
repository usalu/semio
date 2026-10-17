//! 📸️ snapshot of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
pub use crate::standards::v1::subsets::any::schema::mutations::change_seed::artifact_items_3::{mesh_child_handle};

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
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🔺️diff/🦀️.rs"]
                    pub mod diff;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "../../../🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_cell_sizes {
                            #[path = "../../../🧬️mutations/📏️change-cell-sizes/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/📏️change-cell-sizes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/📏️change-cell-sizes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            #[path = "../../../🧬️mutations/🔁️change-periodicity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/🔁️change-periodicity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🔁️change-periodicity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "../../../../📚️examples/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
