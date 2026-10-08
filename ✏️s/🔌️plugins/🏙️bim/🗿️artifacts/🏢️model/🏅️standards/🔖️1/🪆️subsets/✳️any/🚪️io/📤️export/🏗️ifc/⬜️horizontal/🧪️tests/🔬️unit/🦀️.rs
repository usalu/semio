use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, tags, target};

#[test]
fn level_slabs_extrude_their_outline_with_holes_by_the_layer_thickness_below_the_storey_level() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCSLAB"), ["r-1:layer0", "r-1:layer1", "sl-balcony", "sl-first", "sl-ground"]);
    let (_, ground) = rows(&document, "IFCSLAB").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("sl-ground")).expect("the ground slab");
    assert_eq!(ground[8].as_enum(), Some("FLOOR"));
    let shape = target(&document, ground, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the representation");
    let solid = document.resolve(&representation[3].as_list().expect("items")[0]).and_then(|instance| instance.entity("IFCEXTRUDEDAREASOLID")).expect("the solid");
    assert!((real(solid, 3) - 0.25).abs() < 1e-9, "wood 0.05 plus concrete 0.2");
    assert!(target(&document, solid, 0).is_some_and(|profile| profile.is_type("IFCARBITRARYPROFILEDEFWITHVOIDS")), "the slab has a hole");
    let placement = target(&document, ground, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axes = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let location = document.resolve(&axes[0]).and_then(|instance| instance.entity("IFCCARTESIANPOINT")).expect("the location");
    assert!((location[0].as_list().expect("coordinates")[2].as_real().expect("z") + 0.25).abs() < 1e-9, "the top face lies on the storey elevation");
}

#[test]
fn a_sloped_slab_is_a_faceted_brep_of_its_inferred_solid() {
    let document = document(&house());
    let (_, balcony) = rows(&document, "IFCSLAB").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("sl-balcony")).expect("the balcony");
    let shape = target(&document, balcony, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the representation");
    assert_eq!(string(representation, 2).as_deref(), Some("Brep"));
    assert!(count(&document, "IFCFACETEDBREP") >= 1);
}

#[test]
fn a_roof_aggregates_one_roof_slab_per_layer_and_names_its_shape() {
    let document = document(&house());
    let (roof, args) = rows(&document, "IFCROOF").into_iter().next().expect("a roof");
    assert_eq!(args[8].as_enum(), Some("GABLE_ROOF"));
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(roof.id)).expect("the roof aggregate");
    let parts = aggregate[5].as_list().expect("parts");
    assert_eq!(parts.len(), 2, "two roof layers");
    for part in parts {
        let slab = document.resolve(part).and_then(|instance| instance.entity("IFCSLAB")).expect("a slab part");
        assert_eq!(slab[8].as_enum(), Some("ROOF"));
    }
}

#[test]
fn the_take_off_rows_of_a_solid_map_back_to_its_groups_by_layer_and_material() {
    use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidGroup};
    use crate::standards::v1::subsets::any::schema::inferences::quantities::LayerQuantity;
    let group = |layer: u32, material: &str| SolidGroup { part: "roof".into(), material: material.into(), layer };
    let row = |material: &str, volume: f64| LayerQuantity { material: material.into(), volume, ..LayerQuantity::default() };
    let solid = ElementSolid { groups: vec![group(1, "tile"), group(0, "wood"), group(0, "felt")], ..ElementSolid::default() };
    let rows = [row("felt", 2.0), row("wood", 3.0), row("tile", 5.0)];
    assert_eq!(super::group_volumes(&solid, &rows), [5.0, 3.0, 2.0], "rows are ordered by (layer, material): felt, wood, tile");
    assert_eq!(super::group_volumes(&solid, &rows[..1]), [0.0, 0.0, 2.0], "a missing row measures nothing");
}
