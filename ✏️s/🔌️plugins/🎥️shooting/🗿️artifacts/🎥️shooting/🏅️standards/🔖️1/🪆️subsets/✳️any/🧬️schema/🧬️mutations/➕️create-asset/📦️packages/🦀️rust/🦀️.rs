//! ➕️ mut-family of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{ShootingAsset, ShootingCamera};
pub use semio_s_artifact_shooting_shooting_schema::{ShootingAssetPatch};

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
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_asset {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            #[path = "../../../🗑️delete-asset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️delete-asset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️delete-asset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_asset_url {
                            #[path = "../../../🌐️change-asset-url/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🌐️change-asset-url/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🌐️change-asset-url/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_assets {
                            #[path = "../../../🔀️reorder-assets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔀️reorder-assets/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔀️reorder-assets/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
pub use semio_s_artifact_shooting_shooting_schema::diff;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod hexagonal_cut_concrete_forest_left {
        #[path = "../../../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_shooting::editor::shooting::*;
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/👥️presence/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            pub mod contract_vectors;
            #[path = "../../../../../✏️editor/👥️presence/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod shooting {
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
