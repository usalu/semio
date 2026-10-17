//! 🧬️ schema of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationComputedChild, EquationGeometry, EquationGraph, EquationNotationChild, EquationResultsChild};

#[path = "."]
pub mod cas {
    pub use semio_s_artifact_mathematical_equation_examples::cas::fnkind;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::expr;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::assume;
    pub use semio_s_artifact_mathematical_equation_snapshot::cas::visit;
    #[path = "../../💡️inferences/🌿️cas-internals/🧩️pattern/🦀️.rs"]
    pub mod pattern;
    #[path = "../../💡️inferences/🌿️cas-internals/📐️trig/🦀️.rs"]
    pub mod trig;
    #[path = "../../💡️inferences/🌿️cas-internals/📈️diff/🦀️.rs"]
    pub mod diff;
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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
