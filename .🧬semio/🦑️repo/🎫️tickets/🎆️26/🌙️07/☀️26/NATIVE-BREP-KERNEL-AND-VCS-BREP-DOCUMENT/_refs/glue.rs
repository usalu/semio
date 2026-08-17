//! 🌐️ Shared 3D s-module: B-Rep kernel, half-edge mesh, scene math, and BVH spatial index.

//#region 🔖️Brep
/// @emoji 📐️ Boundary-representation kernel (native modules + brepkit backend + engine contracts).
#[cfg(feature = "brep")]
#[path = "."]
pub mod brep {
    #[path = "../../📐️brep/🚨️error/🦀️component.rs"]
    pub mod error;
    #[path = "../../📐️brep/➡️vector/🦀️component.rs"]
    pub mod vec;
    #[path = "../../📐️brep/🔢️matrix/🦀️component.rs"]
    pub mod mat;
    #[path = "../../📐️brep/📏️tolerance/🦀️component.rs"]
    pub mod tolerance;
    #[path = "../../📐️brep/⚖️predicates/🦀️component.rs"]
    pub mod predicates;
    #[path = "../../📐️brep/🔮️oracle/🦀️component.rs"]
    pub mod oracle;
    #[path = "../../📐️brep/〰️polynomial/🦀️component.rs"]
    pub mod poly;
    #[path = "../../📐️brep/🎢️bezier/🦀️component.rs"]
    pub mod bezier;
    #[path = "../../📐️brep/🪢️bspline/🦀️component.rs"]
    pub mod bspline;
    #[path = "../../📐️brep/➰️curve/🦀️component.rs"]
    pub mod curve;
    #[path = "../../📐️brep/✂️curve-ops/🦀️component.rs"]
    pub mod curve_ops;
    #[path = "../../📐️brep/🏄️surface/🦀️component.rs"]
    pub mod surface;
    #[path = "../../📐️brep/🪡️surface-ops/🦀️component.rs"]
    pub mod surface_ops;
    #[path = "../../📐️brep/🏟️arena/🦀️component.rs"]
    pub mod arena;
    #[path = "../../📐️brep/📜️history/🦀️component.rs"]
    pub mod history;
    #[path = "../../📐️brep/🕸️topology/🦀️component.rs"]
    pub mod topo;
    #[path = "../../📐️brep/🔺️euler/🦀️component.rs"]
    pub mod euler;
    #[path = "../../📐️brep/✅️validate/🦀️component.rs"]
    pub mod validate;
    #[path = "../../📐️brep/🌳bvh/🦀️component.rs"]
    pub mod bvh;
    #[path = "../../📐️brep/🧱primitives/🦀️component.rs"]
    pub mod primitives;
    #[path = "../../📐️brep/📏measure/🦀️component.rs"]
    pub mod measure;
    #[path = "../../📐️brep/🧩tessellate/🦀️component.rs"]
    pub mod tessellate;
    #[path = "../../📐️brep/✂️int-cc/🦀️component.rs"]
    pub mod int_cc;
    #[path = "../../📐️brep/✂️int-cs/🦀️component.rs"]
    pub mod int_cs;
    #[path = "../../📐️brep/✂️int-ss/🦀️component.rs"]
    pub mod int_ss;
    #[path = "../../📐️brep/🏷️classify/🦀️component.rs"]
    pub mod classify;
    #[path = "../../📐️brep/🖋️imprint/🦀️component.rs"]
    pub mod imprint;
    #[path = "../../📐️brep/🔀boolean/🦀️component.rs"]
    pub mod boolean;
    #[path = "../../📐️brep/🧵sew/🦀️component.rs"]
    pub mod sew;
    #[path = "../../📐️brep/🩹heal/🦀️component.rs"]
    pub mod heal;
    #[path = "../../📐️brep/➡️sweep/🦀️component.rs"]
    pub mod sweep;
    #[path = "../../📐️brep/↔️offset/🦀️component.rs"]
    pub mod offset;
    #[path = "../../📐️brep/🎨️blend/🦀️component.rs"]
    pub mod blend;
    #[path = "../../📐️brep/📄step/🦀️component.rs"]
    pub mod step;
    #[path = "../../📐️brep/📦mesh-io/🦀️component.rs"]
    pub mod mesh_io;
    #[path = "../../📐️brep/⚙️engine/🦀️component.rs"]
    pub mod engine;
    #[path = "../../📐️brep/🧰️kernel/🦀️component.rs"]
    pub mod kernel;
}
//#endregion 🔖️Brep

//#region 🔖️Mesh
/// @emoji 🥽️ Half-edge mesh kernel: topology, editing ops, tessellation, UV/decimation.
#[path = "../../🥽️mesh/🦀️component.rs"]
pub mod mesh;
//#endregion 🔖️Mesh

//#region 🔖️Scene
/// @emoji 🎬️ Generic 3D scene math: orbit camera, mesh instances, screen picking, draw descriptors.
#[path = "../../🎬️scene/🦀️component.rs"]
pub mod scene;

pub use scene::{project_point, ray_segment_distance, screen_segment_distance};
//#endregion 🔖️Scene

//#region 🔖️Spatial
/// @emoji 🗺️ BVH spatial index (AABB overlap, nearest point, ray queries).
#[cfg(feature = "brep")]
#[path = "../../🗺️spatial/🦀️component.rs"]
pub mod spatial;
//#endregion 🔖️Spatial
