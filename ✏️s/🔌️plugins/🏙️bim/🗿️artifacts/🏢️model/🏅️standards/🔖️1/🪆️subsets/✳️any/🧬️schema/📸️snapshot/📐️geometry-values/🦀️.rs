//! 📐️ Geometry value types of `s.bim.model@1`: points, vertices, profiles, solid shapes, anchors and view frames.

/// 📍️ Which end of a wall axis an anchor names: the start or the end of the axis.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AnchorEnd {
    Start,
    End,
}

/// ↔️ Which face of a wall, looking along its axis from start to end: the left or the right face.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum WallSide {
    Left,
    Right,
}

/// 🧭 The local axis a revolved profile turns around: the profile lies in the plane through that axis, `u` is the distance from the axis and `v` the position along it.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SolidAxis {
    X,
    Y,
    Z,
}

/// 📐️ Which area of a space an area scheme adds up: the gross area of its outline or the net floor area without the columns.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AreaMeasure {
    Gross,
    Net,
}

/// 🔷️ A closed loop of bulged vertices (counter-clockwise).
pub type Loop = Vec<Vertex>;

/// ⚓️ What an annotation is attached to: a free point, or an element of the model (a wall face, axis or end, the centre of an opening or column, a grid line). An element anchor follows its element by inference, a point stays where it was put.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AnnotationAnchor {
    Point {
        #[dsl(block)] point: Point2,
    },
    WallFace {
        wall: String,
        side: WallSide,
    },
    WallAxis {
        wall: String,
    },
    WallEnd {
        wall: String,
        end: AnchorEnd,
    },
    OpeningCentre {
        opening: String,
    },
    Grid {
        grid: String,
    },
    ColumnCentre {
        column: String,
    },
}

/// ▭ A closed cross-section whose dimensions are formulas: a rectangle, a circle, an I-shape or a polygon of formula points (counter-clockwise, in the local `u` right and `v` up plane).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ParametricProfile {
    Rectangle {
        width: String,
        depth: String,
    },
    Circle {
        diameter: String,
    },
    IShape {
        width: String,
        depth: String,
        web: String,
        flange: String,
    },
    Polygon {
        #[dsl(table)] points: Vec<ExprPoint>,
    },
}

/// 🧊 How a family solid is built: a profile extruded over a height, a profile revolved about an axis, a profile swept along a plan path at the elevation of the solid offset, or an axis-aligned cuboid.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SolidShape {
    Extrusion {
        #[dsl(statements, block)] profile: ParametricProfile,
        base: String,
        height: String,
    },
    Revolution {
        #[dsl(statements, block)] profile: ParametricProfile,
        axis: SolidAxis,
        angle: String,
    },
    Sweep {
        #[dsl(statements, block)] profile: ParametricProfile,
        #[dsl(table)] path: Vec<ExprPoint>,
    },
    Cuboid {
        x: String,
        y: String,
        z: String,
        width: String,
        depth: String,
        height: String,
    },
}

/// 〰️ Wall axis: a line, or an arc whose bulge is tan(sweep / 4); a curved wall is an axis whose bulge is set.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Axis {
    Line {
        #[dsl(block)] start: Point2,
        #[dsl(block)] end: Point2,
    },
    Arc {
        #[dsl(block)] start: Point2,
        #[dsl(block)] end: Point2,
        bulge: f64,
    },
}

/// ▭️ Cross-section of a linear element.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Profile {
    Rectangle {
        width: f64,
        depth: f64,
    },
    Circle {
        diameter: f64,
    },
    IShape {
        width: f64,
        depth: f64,
        web: f64,
        flange: f64,
    },
    Custom {
        #[dsl(table)] outline: Vec<Vertex>,
    },
    Family {
        family: String,
    },
}

/// 🏠️ How a space outline is given: inferred from the walls around a seed, or explicit.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SpaceBoundary {
    Bounded {
        #[dsl(block)] seed: Point2,
    },
    Explicit {
        #[dsl(table)] outline: Vec<Vertex>,
    },
}

/// 📍 A planar point whose coordinates are formulas.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ExprPoint {
    pub x: String,
    pub y: String,
}

/// 📍 A point in space whose coordinates are formulas.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ExprPoint3 {
    pub x: String,
    pub y: String,
    pub z: String,
}

/// 📍️ A point in space in metres.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🧊️ An axis-aligned box in building-datum metres: the section box of a viewpoint hides everything outside it.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SectionBox {
    /// The corner with the smallest coordinates.
    #[dsl(block)] pub min: Point3,
    /// The corner with the largest coordinates.
    #[dsl(block)] pub max: Point3,
}

/// 📍️ Planar point in metres.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

/// 🎨️ Linear colour, components in 0..1.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// 🔷️ Loop vertex; bulge shapes the segment to the next vertex.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Vertex {
    #[dsl(block)] pub point: Point2,
    pub bulge: f64,
}

/// 📐️ Slope of a plane: fall direction and angle in radians.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Slope {
    pub direction: f64,
    pub angle: f64,
}

/// 📐️ The vertical plane of a section or elevation as the line it stands on in plan: from `start` to `end`, the viewer stands on the right of the line and looks to its left; `u` runs along the line from `start`.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ViewPlane {
    #[dsl(block)] pub start: Point2,
    #[dsl(block)] pub end: Point2,
}

/// ✂️ The crop rectangle of a view in its own drawing coordinates: building metres of a plan, or `u` along the plane and elevation above the building datum for a section or elevation.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ViewCrop {
    #[dsl(block)] pub min: Point2,
    #[dsl(block)] pub max: Point2,
}
