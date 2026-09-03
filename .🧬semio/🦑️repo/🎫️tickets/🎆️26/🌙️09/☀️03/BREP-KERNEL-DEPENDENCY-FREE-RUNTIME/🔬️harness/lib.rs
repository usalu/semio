//! 🧪 Standalone test harness for the brep KERNEL layer — see TICKET/📓️h0-harness.md for the full
//! scope rationale, how to run, and the architecture finding this file's history led to. Mounts
//! the REAL source files under
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{✳️brep,✳️base}` verbatim
//! via `#[path]`, at exactly the module positions the real stdio crate root uses
//! (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`), so every `use crate::artifacts::semio::…`
//! line in those files resolves unchanged. Never copies a source line — nine parallel workers keep
//! editing the real files and this harness picks up their edits on the next `cargo test`.
//!
//! SCOPE (after two widen/narrow passes — see 📓️h0-harness.md "Widening and narrowing" for the
//! full trace): the pure KERNEL algorithm files only.
//!   snapshot: vector, curve, polynomial, surface, arena, tolerance, error, topology
//!   diff:     primitives, boolean, euler, intersect, offset, blend, sweep
//!   inferences: classification, bounding_volume, mass_properties, tessellation
//!   engine:   ONLY `⚙️engine/🔖️contract/🦀️.rs` (the neutral `MeshTransfer`/`Aabb`/… types W1-A
//!             relocated there) — mounted as `schema::engine` so `use …::schema::engine::{…}`
//!             resolves, WITHOUT the full `Brep`/`BrepKernel` façade (see "Not mounted" below).
//!   base:     ONLY `schema::geometry`, `schema::triples` (the only two `base::schema::*` paths
//!             anything in ✳️brep's kernel imports).
//!
//! NOT MOUNTED, and why (all discovered by trying to widen the harness — see 📓️h0-harness.md):
//!   - `🔺️diff/🧵️sew/🦀️.rs` — its real (non-test) `heal_solid` calls `inferences::
//!     validation_report::validate_body`. Nothing else in scope imports `diff::sew`.
//!   - `💡️inferences/✅validation-report/🦀️.rs` — `validate_body`'s home file also implements
//!     `store::InferredField<SemioBrepSnapshot>`, which needs: the artifact-layer `SemioBrepSnapshot`
//!     (`📸️snapshot/🦀️.rs` + `schema::ArtifactSchema` + `base::schema` beyond geometry/triples),
//!     `brep::io::check_brep_referential_integrity` (`🚪️io/🦀️.rs`'s `derived_composition`, which
//!     needs `semio_framework_plugin` AND the brep-owned STEP serializers
//!     `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/📐️step/…`), and THOSE
//!     need the separate standalone `crate::artifacts::step` artifact (its own subsystem). This
//!     one call chain pulls in most of the stdio crate's artifact/schema/io/STEP graph — confirmed
//!     by actually mounting it and hitting ~180 further unresolved-module errors from base's
//!     generic cross-artifact `io`/`snapshot`/`diff`/`mutations` registry (which references EVERY
//!     other semio subset: animation, audio, cad, document, drawing, flow, graph, image, kit,
//!     mesh, model, object, presentation, table, text, value, video). NOT trivially mountable.
//!     Recommend to W1-F / the ticket lead: split `validate_body` (a pure `&Body ->
//!     Vec<ValidationIssue>` fn, no `SemioBrepSnapshot` in its signature) out of
//!     `✅validation-report/🦀️.rs` into its own kernel-scope file, decoupled from the
//!     `InferredField` artifact-layer wrapper.
//!   - The full `⚙️engine/🦀️.rs` (`Brep`/`BrepKernel`), its `📦️mesh-io` (needs
//!     `crate::artifacts::dwg`, which itself needs `crate::registry` + its own `standards::
//!     v_ac1018::subsets::any::schema` — another cascade) and `📄️step` submodules, `🚪️io/🦀️.rs`,
//!     `🧬️mutations/🦀️.rs` — all excluded per the validation-report finding above (engine.rs
//!     `use`s `validate_body` directly) plus the mesh-io/dwg cascade on top.
//!   - `viewer/**`, `editor/**` (Wave 3A) and the flow extension crate — never referenced by
//!     anything above.
//!
//! KNOWN CURRENTLY-FAILING (transient, from concurrent Wave-1 work in progress — NOT harness
//! bugs, do not chase): see 📓️h0-harness.md "Known failing tests" for the live, re-checked list.

// Same crate aliases the real stdio root declares (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`
// lines 21-25), so `dsl::…`/`store::…`/`protocol::…`/`value_derive::…` resolve exactly as they do
// in the real crate. `schema` (semio_framework_schema, for `ArtifactSchema`) is deliberately NOT
// aliased here — nothing in this harness's mounted scope needs it (see "Not mounted" above).
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub mod artifacts {
    pub mod semio {
        pub mod standards {
            pub mod v1 {
                pub mod subsets {
                    #[path = "."]
                    pub mod brep {
                        #[path = "."]
                        pub mod schema {
                            //#region Engine — contract types only, see module doc above.
                            #[path = "."]
                            pub mod engine {
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🔖️contract/🦀️.rs"]
                                pub mod contract;
                                pub use contract::*;
                            }
                            //#endregion Engine

                            #[path = "."]
                            pub mod snapshot {
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➡️vector/🦀️.rs"]
                                pub mod vector;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs"]
                                pub mod curve;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/〰️polynomial/🦀️.rs"]
                                pub mod polynomial;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs"]
                                pub mod surface;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️.rs"]
                                pub mod arena;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/📏️tolerance/🦀️.rs"]
                                pub mod tolerance;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🚨️error/🦀️.rs"]
                                pub mod error;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs"]
                                pub mod topology;
                            }

                            #[path = "."]
                            pub mod diff {
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧱️primitives/🦀️.rs"]
                                pub mod primitives;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs"]
                                pub mod boolean;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs"]
                                pub mod euler;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️.rs"]
                                pub mod intersect;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs"]
                                pub mod offset;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs"]
                                pub mod blend;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/🦀️.rs"]
                                pub mod sweep;
                            }

                            #[path = "."]
                            pub mod inferences {
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs"]
                                pub mod classification;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️.rs"]
                                pub mod bounding_volume;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs"]
                                pub mod mass_properties;
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs"]
                                pub mod tessellation;
                            }
                        }
                    }

                    #[path = "."]
                    pub mod base {
                        #[path = "."]
                        pub mod schema {
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧮️geometry/🦀️.rs"]
                            pub mod geometry;
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧬️schema/🧰️triples/🦀️.rs"]
                            pub mod triples;
                        }
                    }
                }
            }
        }
    }
}
