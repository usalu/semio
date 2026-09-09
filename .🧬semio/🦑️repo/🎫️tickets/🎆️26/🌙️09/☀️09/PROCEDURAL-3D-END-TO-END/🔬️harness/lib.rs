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

extern crate self as semio_s_artifact_stdio_semio;
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

/// 🧲 The crate's own `[[test]] brep_procedural_example_booleans` target, mounted here as a
/// unit-test module so the two example configurations run in the harness's ~30 s cycle instead of
/// the real crate's. `extern crate self as semio_s_artifact_stdio_semio` above makes the file's
/// own absolute `use` paths resolve unchanged.
#[cfg(test)]
#[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🧲️procedural-example-booleans/🦀️.rs"]
mod procedural_example_booleans;

/// 🔎 Temporary [DEBUG] probes for this ticket's offset investigation — face-by-face area and
/// volume-moment breakdown of the constructions whose totals came out wrong.
#[cfg(test)]
mod ticket_probe {
    use crate::standards::v1::subsets::brep::schema::diff::offset::{shell_solid_with_open_faces, thicken_face};
    use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_planar_face_from_points};
    use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::{face_area, solid_signed_volume};
    use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
    use crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;

    fn dump(body: &Body, solid: crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId) {
        for f in body.solid_faces(solid) {
            let fd = body.faces.get(f).unwrap().clone();
            let surf = body.surfaces.get(fd.surface).unwrap().clone();
            let kind = match &surf {
                Surface::Plane { frame } => format!("plane n={:?} o={:?}", (frame.z.x, frame.z.y, frame.z.z), (frame.origin.x, frame.origin.y, frame.origin.z)),
                Surface::Nurbs { .. } => "nurbs".into(),
                other => format!("{other:?}"),
            };
            eprintln!("[DEBUG] face {f} flipped={} area={:?} {kind}", fd.flipped, face_area(body, f, 1e-6));
            for cid in body.face_coedges(f) {
                let co = body.coedges.get(cid).unwrap().clone();
                let uv = co.pcurve.and_then(|id| body.curves2.get(id)).map(|pc| (pc.eval(co.prange.0), pc.eval(co.prange.1)));
                eprintln!("[DEBUG]   coedge edge={} fwd={} prange={:?} uv={:?}", co.edge, co.forward, co.prange, uv.map(|(a, b)| ((a.x, a.y), (b.x, b.y))));
            }
        }
        eprintln!("[DEBUG] signed volume = {:?}", solid_signed_volume(body, solid, 1e-6));
    }

    #[test]
    fn probe_thicken() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], &mut rec).unwrap();
        let solid = thicken_face(&mut body, face, 0.5, &mut rec).unwrap();
        dump(&body, solid);
    }

    #[test]
    fn probe_shell_open() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (a, b, c, t) = (2.0, 2.0, 2.0, 0.2);
        let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
        let top = *body.solid_faces(solid).iter().find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.z - c).abs() < 1e-9)).unwrap();
        let shelled = shell_solid_with_open_faces(&mut body, solid, t, &[top], &mut rec).unwrap();
        dump(&body, shelled);
    }

    #[test]
    fn probe_draft() {
        use crate::standards::v1::subsets::brep::schema::diff::offset::draft_angle;
        use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface as S;
        use crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let right = *body.solid_faces(solid).iter().find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), S::Plane { frame } if (frame.origin.x - 1.0).abs() < 1e-9)).unwrap();
        let drafted = draft_angle(&mut body, solid, &[right], Vec3::Z, (Pnt3::new(0.0, 0.0, 0.0), Vec3::Z), 0.2, &mut rec).unwrap();
        let mut seen = std::collections::BTreeSet::new();
        for f in body.solid_faces(drafted) {
            for cid in body.face_coedges(f) {
                let co = body.coedges.get(cid).unwrap();
                let e = body.edges.get(co.edge).unwrap();
                for v in [e.v0, e.v1] {
                    let p = body.vertices.get(v).unwrap().position;
                    seen.insert((format!("{:.5}", p.x), format!("{:.5}", p.y), format!("{:.5}", p.z)));
                }
            }
        }
        for v in &seen {
            eprintln!("[DEBUG] drafted vertex {v:?}");
        }
        dump(&body, drafted);
    }
}
