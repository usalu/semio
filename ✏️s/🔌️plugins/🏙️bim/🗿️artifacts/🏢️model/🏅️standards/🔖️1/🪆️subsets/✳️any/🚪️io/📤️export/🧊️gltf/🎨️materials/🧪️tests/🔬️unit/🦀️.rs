use semio_s_artifact_bim_model_tests_5::render::world::family_color;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::{parts, SolidFamily, SolidGroup};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::MaterialCategory;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document::GltfMaterial;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_tests_5::render::world::group_color;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::gltf::materials::{key_of, MaterialKey, Palette};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::testkit::house;

fn group(part: &str, material: &str) -> SolidGroup {
    SolidGroup { part: part.into(), material: material.into(), layer: 0 }
}

#[test]
fn a_group_is_keyed_by_its_material_then_by_glazing_then_by_its_family() {
    let model = house();
    let known = model.materials.keys().next().expect("a material").clone();
    assert_eq!(key_of(&model, SolidFamily::Wall, &group("layer", &known)), MaterialKey::Model(known));
    assert_eq!(key_of(&model, SolidFamily::Window, &group(parts::GLASS, "")), MaterialKey::Glass);
    assert_eq!(key_of(&model, SolidFamily::Window, &group(parts::FRAME, "")), MaterialKey::Family(SolidFamily::Window));
    assert_eq!(key_of(&model, SolidFamily::Slab, &group("body", "no-such-material")), MaterialKey::Family(SolidFamily::Slab));
}

#[test]
fn the_palette_numbers_materials_in_order_of_first_use_and_reuses_them() {
    let model = house();
    let known = model.materials.keys().next().expect("a material").clone();
    let mut palette = Palette::default();
    let glass = palette.index_of(&model, SolidFamily::Window, &group(semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::parts::GLASS, ""));
    let wall = palette.index_of(&model, SolidFamily::Wall, &group("layer", &known));
    assert_eq!((glass, wall), (0, 1));
    assert_eq!(palette.index_of(&model, SolidFamily::Door, &group(parts::GLASS, "")), 0);
    assert_eq!(palette.index_of(&model, SolidFamily::Wall, &group("layer", &known)), 1);
    assert_eq!(palette.materials.len(), 2);
}

#[test]
fn model_materials_take_their_colour_and_glazing_is_translucent() {
    let model = house();
    let mut palette = Palette::default();
    for (id, material) in &model.materials {
        let index = palette.index_of(&model, SolidFamily::Wall, &group("layer", id));
        let defined = &palette.materials[index];
        assert_eq!(defined.name, material.name);
        assert_eq!(defined.color[..3], [material.color.r as f32, material.color.g as f32, material.color.b as f32]);
        assert_eq!(defined.blend, material.category == MaterialCategory::Glass);
        assert_eq!(defined.metallic, if material.category == MaterialCategory::Metal { 1.0 } else { 0.0 });
    }
    let index = palette.index_of(&model, SolidFamily::Window, &group(semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::parts::GLASS, ""));
    let glazing = &palette.materials[index];
    assert!(glazing.blend && glazing.color[3] < 1.0 && glazing.roughness < 0.2);
}

#[test]
fn faces_without_a_material_take_the_family_colour_of_the_viewer() {
    let model = house();
    let mut palette = Palette::default();
    let index = palette.index_of(&model, SolidFamily::Roof, &group("body", ""));
    assert_eq!(palette.materials[index].color, family_color(SolidFamily::Roof));
    assert_eq!(palette.materials[index].name, "Roof");
}
