//! 🧰️ Shared helpers of the IFC export tests: the committed house model and readers over the generated Part-21 document.

use super::{model_to_part21, Schema};
use crate::standards::v1::subsets::any::schema::snapshot::{AreaMeasure, AreaScheme, Axis, Beam, Ceiling, CeilingType, Column, CurtainPanel, CurtainPanelOverride, Layer, LayerFunction, LocationLine, ModelSnapshot, Opening, OpeningKind, Phase, Point2, Profile, Ramp, Slab, Slope, TopConstraint, Vertex, Wall, WallSide, WallSweep, Zone};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 🏠️ The committed export fixture: one site, one building, four storeys and every element family.
pub const HOUSE: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 🏠️ The decoded house model.
pub fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
}

/// 🏷️ The committed psets fixture: the house with three classification systems (tables with parents), several systems per element and per type, type-level property sets and the property set templates of the IFC4 library.
pub const PSETS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🏗️ifc/🏷️psets/📸️snapshot/🔣️.json");

/// 🏷️ The decoded psets model.
pub fn psets() -> ModelSnapshot {
    from_json_str(PSETS, JsonMemberPolicy::Reject).expect("the committed psets model decodes")
}

/// 🪧️ The committed annotated room: dimensions, tags, a note and a leader around four walls, a window, a grid line and a column.
pub const NOTATED: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json");

/// 🪧️ The decoded annotated room.
pub fn notated() -> ModelSnapshot {
    from_json_str(NOTATED, JsonMemberPolicy::Reject).expect("the committed room decodes")
}

/// 🔲️ The committed ceilings: a board ceiling with a hole, a sloped and a diagonally sloped tile ceiling and one with a half-round end, on two storeys.
pub const CEILINGS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json");

/// 🔲️ The decoded ceilings model.
pub fn ceilings() -> ModelSnapshot {
    from_json_str(CEILINGS, JsonMemberPolicy::Reject).expect("the committed ceilings decode")
}

/// 🪑️ The committed components model: every category of family, free, hosted, rotated and mirrored components, overrides, terminals of several systems and ducts, pipes and trays on two storeys.
pub const COMPONENTS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🏗️ifc/🪑️components/📸️snapshot/🔣️.json");

/// 🪑️ The decoded components model.
pub fn components() -> ModelSnapshot {
    from_json_str(COMPONENTS, JsonMemberPolicy::Reject).expect("the committed components model decodes")
}

/// 📄️ The IFC 2x3 Part-21 document of a model.
pub fn document(model: &ModelSnapshot) -> Part21Document {
    document_in(Schema::Ifc2x3, model)
}

/// 📄️ The Part-21 document of a model in `schema`.
pub fn document_in(schema: Schema, model: &ModelSnapshot) -> Part21Document {
    model_to_part21(schema, model).expect("the model exports").0
}

/// 🧩️ Every instance of an entity type with its argument list.
pub fn rows<'a>(document: &'a Part21Document, entity: &'a str) -> Vec<(&'a Part21Instance, &'a Vec<Part21Value>)> {
    document.by_type(entity).filter_map(|instance| instance.entity(entity).map(|args| (instance, args))).collect()
}

/// 🔢️ How many instances of an entity type the document holds.
pub fn count(document: &Part21Document, entity: &str) -> usize {
    document.by_type(entity).count()
}

/// 🏷️ The string at argument `index`.
pub fn string(args: &[Part21Value], index: usize) -> Option<String> {
    args.get(index).and_then(|value| value.as_str()).map(str::to_string)
}

/// 🔢️ The real at argument `index`.
pub fn real(args: &[Part21Value], index: usize) -> f64 {
    args.get(index).and_then(|value| value.as_real()).unwrap_or(f64::NAN)
}

/// 🔗️ The instance that argument `index` refers to.
pub fn target<'a>(document: &'a Part21Document, args: &[Part21Value], index: usize) -> Option<&'a Part21Instance> {
    args.get(index).and_then(|value| document.resolve(value))
}

/// 🧮️ The value of base quantity `name` of property set `set` of the products and spaces written for model element `id` (`Tag`, or `ObjectType` of a space; a window or door shares its id with its opening element).
pub fn quantity(document: &Part21Document, id: &str, set: &str, name: &str) -> Option<f64> {
    let elements: Vec<u64> = document.instances.iter().filter(|instance| instance.primary().is_some_and(|(entity, args)| string(args, if entity == "IFCSPACE" { 4 } else { 7 }).as_deref() == Some(id))).map(|instance| instance.id).collect();
    document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES")).filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id().is_some_and(|target| elements.contains(&target))))).find_map(|args| {
        let definition = document.resolve(&args[5])?.entity("IFCELEMENTQUANTITY").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
        definition[5].as_list()?.iter().filter_map(|item| document.resolve(item)).find_map(|row| row.entities.iter().find(|(_, args)| string(args, 0).as_deref() == Some(name)).map(|(_, args)| real(args, 3)))
    })
}

/// 🏷️ The sorted `Tag` strings (argument 7) of every product of an entity type.
pub fn tags(document: &Part21Document, entity: &str) -> Vec<String> {
    let mut tags: Vec<String> = rows(document, entity).iter().filter_map(|(_, args)| string(args, 7)).collect();
    tags.sort();
    tags
}

/// 🧗️ The committed attic: gable and hip roofs with walls under them, a wall standing on a sloped slab, baseboards, a hand rail, a door, a window with a reveal and a plain window.
pub const ATTIC: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json");

/// 🧗️ The decoded attic model.
pub fn attic() -> ModelSnapshot {
    from_json_str(ATTIC, JsonMemberPolicy::Reject).expect("the committed attic decodes")
}

/// 🧮️ The value of base quantity `name` of property set `set` of the product `element`.
pub fn quantity_of(document: &Part21Document, element: u64, set: &str, name: &str) -> Option<f64> {
    document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES")).filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element)))).find_map(|args| {
        let definition = document.resolve(&args[5])?.entity("IFCELEMENTQUANTITY").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
        definition[5].as_list()?.iter().filter_map(|item| document.resolve(item)).find_map(|row| row.entities.iter().find(|(_, args)| string(args, 0).as_deref() == Some(name)).map(|(_, args)| real(args, 3)))
    })
}

/// 🏷️ The rows of property set `set` of every product tagged `id` that carries it, by property name with the typed value as written.
pub fn property_sets(document: &Part21Document, id: &str, set: &str) -> Vec<BTreeMap<String, Part21Value>> {
    let tagged: Vec<u64> = document.instances.iter().filter(|instance| instance.primary().is_some_and(|(_, args)| string(args, 7).as_deref() == Some(id))).map(|instance| instance.id).collect();
    document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id().is_some_and(|target| tagged.contains(&target)))))
        .filter_map(|args| {
            let definition = document.resolve(&args[5])?.entity("IFCPROPERTYSET").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
            Some(definition[4].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (string(row, 0).unwrap_or_default(), row[2].clone())).collect())
        })
        .collect()
}

/// 🔤️ The string inside a typed property value.
pub fn text_of(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<String> {
    rows.get(name)?.as_typed()?.1.first()?.as_str().map(str::to_string)
}

/// 🔢️ The number inside a typed property value.
pub fn number_of(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<f64> {
    rows.get(name)?.as_typed()?.1.first()?.as_real()
}

/// 📍️ The location of the placement of a product (argument 5) relative to its parent.
pub fn location(document: &Part21Document, args: &[Part21Value]) -> Option<Vec<f64>> {
    let placement = target(document, args, 5)?.entity("IFCLOCALPLACEMENT")?;
    let axis = document.resolve(&placement[1])?.entity("IFCAXIS2PLACEMENT3D")?;
    let point = document.resolve(&axis[0])?.entity("IFCCARTESIANPOINT")?;
    Some(point[0].as_list()?.iter().filter_map(Part21Value::as_real).collect())
}

/// 🏗️ The house with a leaning column, an arc beam, an inclined beam and a beam joined to two columns, on the storey of its first column.
pub fn leaning() -> ModelSnapshot {
    let mut model = house();
    let template = model.columns.values().next().cloned().expect("a column");
    let beam_type = model.beams.values().next().map(|beam| beam.beam_type.clone()).expect("a beam");
    let storey = template.storey.clone();
    let at = |x: f64, y: f64| Point2 { x, y };
    model.columns.insert("c-lean".into(), Column { position: at(30.0, 30.0), rotation: 0.0, tilt: Some(Slope { direction: 0.5, angle: 0.2 }), ..template.clone() });
    model.columns.insert("c-west".into(), Column { position: at(40.0, 30.0), tilt: None, ..template.clone() });
    model.columns.insert("c-east".into(), Column { position: at(46.0, 30.0), tilt: None, ..template });
    let beam = |axis: Axis, end_top_offset: Option<f64>| Beam { storey: storey.clone(), beam_type: beam_type.clone(), axis, top_offset: 0.0, end_top_offset, phase: crate::standards::v1::subsets::any::schema::snapshot::Phase::New, name: String::new() };
    model.beams.insert("b-arc".into(), beam(Axis::Arc { start: at(30.0, 40.0), end: at(36.0, 40.0), bulge: 0.4 }, None));
    model.beams.insert("b-incline".into(), beam(Axis::Line { start: at(30.0, 50.0), end: at(36.0, 50.0) }, Some(-0.8)));
    model.beams.insert("b-joined".into(), beam(Axis::Line { start: at(40.0, 30.0), end: at(46.0, 30.0) }, None));
    model
}

/// 🪟️ The house whose curtain wall has a door in its first cell and an open second cell.
pub fn framed() -> ModelSnapshot {
    let mut model = house();
    let door_type = model.door_types.keys().next().cloned().expect("a door type");
    model.curtain_panel_overrides.insert("ov-door".into(), CurtainPanelOverride { curtain: "cw-1".into(), u: 0, v: 0, panel: CurtainPanel::Door { door_type } });
    model.curtain_panel_overrides.insert("ov-open".into(), CurtainPanelOverride { curtain: "cw-1".into(), u: 1, v: 0, panel: CurtainPanel::Empty });
    model
}

/// 🏘️ The house with two zones, two area schemes and finished spaces: the living room in the day zone, the bedroom in the night zone.
pub fn zoned() -> ModelSnapshot {
    let mut model = house();
    model.zones.insert("z-day".into(), Zone { name: "Day zone".into(), category: "Ventilation".into(), occupancy_density: 0.1 });
    model.zones.insert("z-night".into(), Zone { name: "Night zone".into(), category: "Fire compartment".into(), occupancy_density: 0.05 });
    model.zones.insert("z-empty".into(), Zone { name: "Empty zone".into(), category: "Tenant".into(), occupancy_density: 0.0 });
    let living = model.spaces.get_mut("sp-1").expect("the living room");
    (living.zone, living.floor_finish, living.wall_finish, living.ceiling_finish) = (Some("z-day".into()), Some("m-wood".into()), Some("m-brick".into()), Some("m-conc".into()));
    let bedroom = model.spaces.get_mut("sp-2").expect("the bedroom");
    (bedroom.zone, bedroom.floor_finish) = (Some("z-night".into()), Some("m-wood".into()));
    model.area_schemes.insert("as-nsa".into(), AreaScheme { name: "Net use area".into(), measure: AreaMeasure::Net, usages: vec!["living".into(), "bedroom".into()], zones: Vec::new() });
    model.area_schemes.insert("as-day".into(), AreaScheme { name: "Day zone area".into(), measure: AreaMeasure::Gross, usages: Vec::new(), zones: vec!["z-day".into()] });
    model
}

pub fn vertex(x: f64, y: f64, bulge: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge }
}

/// 🛝️ The house with a 10 m ramp on its first storey: 1.5 m landings at both ends, 0.5 m of rise.
pub fn with_ramp(left: bool) -> ModelSnapshot {
    let mut model = house();
    let storey = model.storeys.iter().min_by_key(|(_, storey)| storey.level).map(|(id, _)| id.clone()).expect("a storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    model.ramps.insert(
        "rp-1".into(),
        Ramp {
            storey,
            path: vec![vertex(0.0, -3.0, 0.0), vertex(10.0, -3.0, 0.0)],
            width: 1.2,
            landing_start: 1.5,
            landing_end: 1.5,
            landing_turn: 1.5,
            max_slope: 1.0 / 12.0,
            thickness: 0.2,
            material,
            base_offset: 0.0,
            top: TopConstraint::Unconnected { height: 0.5 },
            railing_left: left,
            railing_right: false,
            name: "Entrance Ramp".into(),
        },
    );
    model
}

pub fn rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Vertex> {
    vec![vertex(x0, y0), vertex(x1, y0), vertex(x1, y1), vertex(x0, y1)]
}

/// 🧗️ The part of the house the import understands plus three walls on the ground storey: one under a level ceiling with a baseboard and a window with a reveal, one under a sloped ceiling (a brep) with a hand rail, one standing on a level slab.
pub fn with_depth() -> ModelSnapshot {
    let mut model = house();
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    let ground = model.storeys.iter().find(|(_, storey)| storey.level == 0).map(|(id, _)| id.clone()).expect("the ground storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    let (wall_type, slab_type, window_type) = (model.wall_types.keys().next().cloned().expect("a wall type"), model.slab_types.keys().next().cloned().expect("a slab type"), model.window_types.keys().next().cloned().expect("a window type"));
    model.ceiling_types.insert("ct-board".into(), CeilingType { name: "Board".into(), layers: vec![Layer { material: material.clone(), thickness: 0.0125, function: LayerFunction::Finish }] });
    let ceiling = |boundary: Vec<Vertex>, slope: Option<Slope>, name: &str| Ceiling { storey: ground.clone(), ceiling_type: "ct-board".into(), boundary, holes: Vec::new(), offset: 0.2, slope, name: name.into() };
    model.ceilings.insert("ce-flat".into(), ceiling(rectangle(100.0, 0.0, 108.0, 6.0), None, "Flat"));
    model.ceilings.insert("ce-raked".into(), ceiling(rectangle(100.0, 10.0, 108.0, 16.0), Some(Slope { direction: 0.0, angle: 0.1 }), "Raked"));
    model.slabs.insert("sl-foot".into(), Slab { storey: ground.clone(), slab_type, boundary: rectangle(100.0, 20.0, 108.0, 26.0), holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: "Foot".into() });
    let wall = |y: f64, top: TopConstraint, base_slab: Option<&str>| Wall {
        storey: ground.clone(),
        wall_type: wall_type.clone(),
        axis: Axis::Line { start: Point2 { x: 100.0, y }, end: Point2 { x: 108.0, y } },
        location: LocationLine::Center,
        base_offset: if base_slab.is_some() { 0.05 } else { 0.0 },
        top,
        phase: Phase::New,
        start_join: None,
        end_join: None,
        name: String::new(),
        base_slab: base_slab.map(str::to_string),
    };
    model.walls.insert("w-ceil".into(), wall(3.0, TopConstraint::Ceiling { ceiling: "ce-flat".into(), offset: 0.05 }, None));
    model.walls.insert("w-raked".into(), wall(13.0, TopConstraint::Ceiling { ceiling: "ce-raked".into(), offset: 0.0 }, None));
    model.walls.insert("w-foot".into(), wall(23.0, TopConstraint::StoreyTop { offset: 0.0 }, Some("sl-foot")));
    model.wall_sweeps.insert("ws-board".into(), WallSweep { host: "w-ceil".into(), side: WallSide::Left, profile: Profile::Rectangle { width: 0.02, depth: 0.1 }, height: 0.0, inset: 0.0, material: material.clone(), name: "Baseboard".into() });
    model.wall_sweeps.insert("ws-rail".into(), WallSweep { host: "w-raked".into(), side: WallSide::Right, profile: Profile::Rectangle { width: 0.05, depth: 0.04 }, height: 0.9, inset: 0.01, material: material.clone(), name: String::new() });
    model.openings.insert(
        "o-rev".into(),
        Opening { host: "w-ceil".into(), kind: OpeningKind::Window { window_type }, offset: 4.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: String::new(), reveal_depth: Some(0.1), reveal_material: Some(material) },
    );
    model
}
