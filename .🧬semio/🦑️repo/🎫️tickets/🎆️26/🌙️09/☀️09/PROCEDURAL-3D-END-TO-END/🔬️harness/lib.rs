//! 🧪 Standalone harness that mounts the REAL BREP kernel sources at exactly the module positions
//! the `semio-s-artifact-stdio-semio` crate root gives them (`crate::standards::v1::subsets::…`),
//! so every `use crate::standards::…` line in those files resolves unchanged and no source line is
//! ever copied. See `Cargo.toml` for why this exists (root-target lock contention plus the module
//! ROOTS' `semio-framework-plugin`/`-schema` dependency, which neighbouring sessions routinely
//! leave uncompilable).
//!
//! NOT mounted: the `📸️snapshot`/`🔺️diff`/`💡️inferences` component ROOT files (the artifact-layer
//! `SemioBrepSnapshot`/`SemioBrepDiff`/`ArtifactInferrer` wrappers) — this file supplies its own
//! roots holding only the geometry submodules (which is also why `✉️base` is absent — only those
//! unmounted roots reach for it); and `✅validation-report`'s own root, of which only
//! the kernel-scope `🧪️body` split is mounted (the real crate root does the same re-export, so
//! `inferences::validation_report::validate_body` resolves identically in both trees).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub mod standards {
    pub mod v1 {
        pub mod subsets {
            pub mod brep {
                pub mod schema {
                    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs"]
                    pub mod engine;

                    pub mod snapshot {
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➡️vector/🦀️.rs"]
                        pub mod vector;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs"]
                        pub mod curve;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/〰️polynomial/🦀️.rs"]
                        pub mod polynomial;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs"]
                        pub mod surface;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️.rs"]
                        pub mod arena;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📏️tolerance/🦀️.rs"]
                        pub mod tolerance;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🚨️error/🦀️.rs"]
                        pub mod error;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs"]
                        pub mod topology;
                    }

                    pub mod diff {
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🧱️primitives/🦀️.rs"]
                        pub mod primitives;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs"]
                        pub mod boolean;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs"]
                        pub mod euler;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/✂️intersect/🦀️.rs"]
                        pub mod intersect;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs"]
                        pub mod offset;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs"]
                        pub mod blend;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/➡️sweep/🦀️.rs"]
                        pub mod sweep;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔁️transform/🦀️.rs"]
                        pub mod transform;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🧵️sew/🦀️.rs"]
                        pub mod sew;
                    }

                    pub mod inferences {
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🏷️classification/🦀️.rs"]
                        pub mod classification;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️.rs"]
                        pub mod bounding_volume;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs"]
                        pub mod mass_properties;
                        #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs"]
                        pub mod tessellation;

                        pub mod validation_report {
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs"]
                            pub mod body;
                            pub use body::validate_body;
                        }
                    }
                }
            }
        }
    }
}
