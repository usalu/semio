//! 🧪 Standalone test harness for the brep KERNEL layer — see TICKET/📓️h0-harness.md for the full
//! scope rationale, how to run, and the architecture finding this file's history led to. Mounts
//! the REAL source files under
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{✳️brep,✳️base}` verbatim
//! via `#[path]`, at exactly the module positions the real stdio crate root uses
//! (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`), so every `use crate::artifacts::semio::…`
//! line in those files resolves unchanged. Never copies a source line — nine parallel workers keep
//! editing the real files and this harness picks up their edits on the next `cargo test`.
//!
//! SCOPE (after several widen/narrow passes — see 📓️h0-harness.md for the full trace): the
//! pure KERNEL algorithm files, PLUS the full engine façade (widened per coordinator follow-up
//! so the harness can surface engine.rs's own compile errors, e.g. W1-C's `Entity` arity
//! mismatches).
//!   snapshot: vector, curve, polynomial, surface, arena, tolerance, error, topology
//!   diff:     primitives, boolean, euler, intersect, offset, blend, sweep, transform, sew
//!   inferences: classification, bounding_volume, mass_properties, tessellation
//!   engine:   the FULL `⚙️engine/🦀️.rs` (`Brep`/`BrepKernel`), which self-mounts its own
//!             `mesh_io`/`step`/`contract` submodules internally.
//!   base:     ONLY `schema::geometry`, `schema::triples` (the only two `base::schema::*` paths
//!             anything in ✳️brep's kernel or engine imports).
//!
//! `sew` is mounted only because `engine.rs` itself `use`s `diff::sew::{…}` — the module must
//! exist for that import to resolve, even though `sew.rs`'s own `heal_solid` still needs
//! `validate_body` (not mounted, see below) and so does not fully compile on its own.
//!
//! TWO SYMBOLS ARE DELIBERATELY LEFT UNRESOLVED (confirmed non-blocking — see 📓️h0-harness.md
//! "Known failing" for the experiment that proved this): each produces exactly one localized
//! `E0432` at its own `use` site, without suppressing typecheck errors elsewhere in the same
//! file (rustc checks every item independently):
//!   - `crate::artifacts::dwg` (`mesh_io.rs`'s `export_solid_dwg`/`import_dwg_to_body`) — `dwg`
//!     needs `crate::registry`, which references ~69 OTHER artifact types (gltf, binary, txt,
//!     xml, deflate, zip, json, csv, …). Not mounted.
//!   - `…::inferences::validation_report` (`engine.rs:62`, `sew.rs:14`, and the `#[cfg(test)]`
//!     modules of `euler.rs`/`sweep.rs`/`primitives.rs`) — its home file's `validate_body` lives
//!     alongside `impl store::InferredField<SemioBrepSnapshot>`, which needs the artifact-layer
//!     `SemioBrepSnapshot` (+ `schema::ArtifactSchema` + more of `base::schema`),
//!     `brep::io::check_brep_referential_integrity` (needs `semio_framework_plugin` + brep's own
//!     STEP serializers, which need the SEPARATE standalone `crate::artifacts::step` artifact).
//!     Actually mounting this whole chain was tried and reverted: `base::schema`'s component
//!     root re-exports a generic cross-artifact `io`/`snapshot`/`diff`/`mutations` registry that
//!     references EVERY other semio subset (animation, audio, cad, document, drawing, flow,
//!     graph, image, kit, mesh, model, object, presentation, table, text, value, video) — ~180
//!     further unresolved-module errors, not remotely "trivially mountable." Recommend to W1-F:
//!     split `validate_body` (pure `fn(&Body) -> Vec<ValidationIssue>`) into its own kernel-scope
//!     file, decoupled from the `InferredField` artifact-layer wrapper.
//!
//! NOT MOUNTED at all: `viewer/**`, `editor/**` (Wave 3A) and the flow extension crate — never
//! referenced by anything above.
//!
//! KNOWN CURRENTLY-FAILING (transient concurrent-work, or the two by-design gaps above — NOT
//! harness bugs, do not chase): see 📓️h0-harness.md "Known failing" for the live, re-checked
//! list with owning worker per error.

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
                            //#region Engine — FULL ⚙️engine/🦀️.rs (Brep/BrepKernel façade),
                            // widened from contract-only per coordinator follow-up (see
                            // 📓️h0-harness.md "Widened to full engine.rs"). Self-mounts its own
                            // `mesh_io`/`step`/`contract` submodules via relative `#[path]` inside
                            // the file, same as the real crate root's single-line mount.
                            // `mesh_io`'s `use crate::artifacts::dwg::{…}` is left UNRESOLVED
                            // (dwg needs `crate::registry`, which itself references ~69 other
                            // artifact types — not mounted, out of scope) — this is a LOCALIZED
                            // E0432 inside `mesh_io`'s own functions, it does not stop the rest of
                            // `engine.rs` (including the `Entity`/`BrepKernel` code the coordinator
                            // wants visible) from being typechecked independently.
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs"]
                            pub mod engine;
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
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔁️transform/🦀️.rs"]
                                pub mod transform;
                                // `sew.rs` mounted because `engine.rs` itself `use`s
                                // `diff::sew::{convert_to_nurbs, defeature, heal_solid,
                                // sew_faces}` — the module must exist for that `use` line to
                                // resolve at all. `heal_solid`'s real body still calls
                                // `inferences::validation_report::validate_body` (NOT mounted,
                                // see doc above), so `sew.rs` itself does NOT fully compile — a
                                // LOCALIZED E0432/E0425 inside `sew.rs` and at `engine.rs`'s
                                // `heal_solid`/`defeature`/etc. call sites, not a blocker for the
                                // rest of the file.
                                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🧵️sew/🦀️.rs"]
                                pub mod sew;
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

                                // `validation_report` here is a harness-only shim: it mounts ONLY
                                // the kernel-scope `🧪️body/🦀️.rs` split (pure `fn(&Body) ->
                                // Vec<ValidationIssue>`, no `SemioBrepSnapshot`/artifact-layer
                                // dependency), not the real `✅validation-report/🦀️.rs` root file
                                // (which stays unmounted — its `BrepValidationReport` still needs
                                // the full artifact/STEP/plugin chain documented above). The real
                                // crate's root file does the equivalent `#[path] mod body; pub use
                                // body::validate_body;`, so `inferences::validation_report::
                                // validate_body` resolves identically in both trees — engine.rs's
                                // and sew.rs's existing `use` lines need no change.
                                #[path = "."]
                                pub mod validation_report {
                                    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs"]
                                    pub mod body;
                                    pub use body::validate_body;
                                }
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

    // 🖊️ Harness-only stub for `crate::artifacts::dwg` (ticket
    // `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 1): `⚙️engine/🦀️.rs`'s `DwgExporter`/
    // `DwgImporter` need these 4 free-function signatures to compile (they implement
    // `semio_framework_mesh_engine::MeshExporter`/`MeshImporter` over the real `crate::artifacts::
    // dwg` codec) — mounting the REAL `dwg` artifact would pull `crate::registry`, which itself
    // references ~69 OTHER artifact types (gltf, binary, txt, xml, deflate, zip, json, csv, …),
    // exactly the cascade `📓️h0-harness.md` "Not mounted" already ruled out for
    // `validation_report`'s old chain. Nothing in the kernel-layer test suite exercises DWG
    // import/export (that is an artifact/IO-layer concern, out of this ticket's wave-1 scope), so
    // a thin harness-local stand-in — not a copy of any real source line, just the 4 signatures —
    // is the pragmatic substitute; the real stdio crate root mounts the genuine `🖊️dwg` artifact.
    pub mod dwg {
        pub struct DwgDrawing;
        // 🚫️async: E1 harness stub, never executed by any mounted test — see module doc above
        pub fn mesh_to_dwg_drawing(_mesh: &semio_framework_mesh_engine::MeshData) -> DwgDrawing {
            DwgDrawing
        }
        // 🚫️async: E1 harness stub, never executed by any mounted test — see module doc above
        pub fn dwg_to_bytes(_drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
            Err("dwg codec is not mounted in the isolated kernel test harness".to_string())
        }
        // 🚫️async: E1 harness stub, never executed by any mounted test — see module doc above
        pub fn dwg_from_bytes(_bytes: &[u8]) -> Result<DwgDrawing, String> {
            Err("dwg codec is not mounted in the isolated kernel test harness".to_string())
        }
        // 🚫️async: E1 harness stub, never executed by any mounted test — see module doc above
        pub fn dwg_drawing_to_mesh(_drawing: &DwgDrawing) -> semio_framework_mesh_engine::MeshData {
            semio_framework_mesh_engine::MeshData::default()
        }
    }
}
