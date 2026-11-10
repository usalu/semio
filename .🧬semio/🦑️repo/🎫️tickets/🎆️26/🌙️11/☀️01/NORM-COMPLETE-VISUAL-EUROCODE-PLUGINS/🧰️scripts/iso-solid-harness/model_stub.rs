pub mod part_2 {
    use std::collections::BTreeMap;
    #[path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📐️part-2-geometry/🧊️solid-volume/🦀️.rs"] pub mod solid;
    #[derive(Clone, Copy, Debug, PartialEq)] pub struct BoundingBox { pub min: [f64;3], pub max: [f64;3] }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum BooleanOperator { Union, Intersection, Difference }
    #[derive(Clone, Debug)] pub enum GeometryNode {
        Primitive { kind: String, parameters: BTreeMap<String,f64> },
        Transform { translation: [f64;3], rotation_deg: [f64;3], child: Box<GeometryNode> },
        Boolean { operator: BooleanOperator, children: Vec<GeometryNode> },
        Reference { geometry_id: String } }
    #[derive(Clone, Debug)] pub struct GeometryObject { pub shape: Option<GeometryNode> }
    #[derive(Default)] pub struct GeometryCatalogue { pub objects: BTreeMap<String, GeometryObject> }
}
