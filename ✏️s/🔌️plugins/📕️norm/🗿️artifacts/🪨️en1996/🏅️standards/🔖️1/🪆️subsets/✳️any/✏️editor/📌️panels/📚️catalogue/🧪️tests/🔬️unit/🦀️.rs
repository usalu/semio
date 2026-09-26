use super::*;
use crate::app_surface::CatalogueCell;
use crate::artifact_schema::{evaluate_building, fire_min_thickness_m};
use crate::editor::en1996::unit_tests::context;
use crate::{En1996Snapshot, UnitMaterial};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_CATALOGUE));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
}

#[semio_framework_async_macros::async_test]
async fn renders_this_standards_catalogue_headline() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, BODY_CATALOGUE).await.contains("catalogue"));
    context::close(&mut app);
}

#[test]
fn renders_reference_tables_with_examples() {
    let node = render(Vec::new(), semio_framework_plugin::Locale::En, "norm.catalogue").expect("catalogue");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project");
    assert!(json.contains("norm-catalogue.examples") || json.contains("Examples") || json.contains("Beispiele") || json.contains("catalogue"), "{json}");
    let tables = reference_tables();
    assert!(!tables.is_empty(), "reference_tables must publish normative tables");
    for table in &tables {
        assert!(!table.id.is_empty());
        assert_ne!(table.title_en, table.title_de, "table {} needs distinct en/de titles", table.id);
        assert!(
            json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id),
            "missing table {}: {json}",
            table.id
        );
        for col in &table.columns {
            if col.unit.is_some() {
                assert!(!col.unit.unwrap().is_empty(), "column {} unit must be non-empty when set", col.id);
            }
        }
    }
}

#[test]
fn reference_tables_cells_match_evaluate_sources() {
    let tables = reference_tables();
    assert!(tables.len() > 0, "reference_tables must be non-empty");

    let fire = tables.iter().find(|t| t.id == "fire-min-thickness-rei90").expect("fire table");
    let clay = fire.rows.iter().find(|r| r.id == "rei90-clay").expect("clay REI 90 row");
    let CatalogueCell::Number { value: t_min, .. } = &clay.cells[1] else {
        panic!("clay t_min cell must be Number");
    };
    assert_eq!(*t_min, fire_min_thickness_m(90, UnitMaterial::Clay));
    assert!((*t_min - 0.140).abs() < 1e-12);

    let gamma = tables.iter().find(|t| t.id == "de-gamma-m-by-class").expect("γ_M table");
    let class1 = gamma.rows.iter().find(|r| r.id == "Class1").expect("Class1 row");
    let CatalogueCell::Number { value: g_persist, .. } = &class1.cells[1] else {
        panic!("γ_M persistent cell must be Number");
    };
    assert_eq!(*g_persist, crate::MasonryClass::Class1.gamma_m_de(false));

    let psi = tables.iter().find(|t| t.id == "psi0-imposed-categories").expect("ψ₀ table");
    let cat_e = psi.rows.iter().find(|r| r.id == "E").expect("category E");
    let CatalogueCell::Number { value: psi0, .. } = &cat_e.cells[1] else {
        panic!("ψ₀ cell must be Number");
    };
    assert_eq!(*psi0, crate::artifact_schema::psi0_imposed("E"));

    let cap = tables.iter().find(|t| t.id == "f-vk-de-cap").expect("f_vk cap table");
    let row = &cap.rows[0];
    let CatalogueCell::Number { value: ratio, .. } = &row.cells[1] else {
        panic!("cap ratio cell must be Number");
    };
    assert_eq!(*ratio, crate::artifact_schema::F_VLT_OVER_FB_DE);

    let fk = tables.iter().find(|t| t.id == "fk-factors-de-en").expect("f_k factors");
    let de_clay = fk.rows.iter().find(|r| r.id == "de-clay-g1-gp").expect("DE clay G1");
    let expected = crate::artifact_schema::fk_factors(
        crate::document::AnnexChoice::De,
        UnitMaterial::Clay,
        crate::UnitGroup::Group1,
        crate::MortarType::GeneralPurpose,
    );
    let CatalogueCell::Number { value: k, .. } = &de_clay.cells[1] else {
        panic!("K cell must be Number");
    };
    assert_eq!(*k, expected.k);

    let mut doc = En1996Snapshot::compliant_clay_wall();
    let wall_id = doc.walls[0].id.clone();
    let base_g = doc.walls[0].load_cases[0].g_k_slab_n;
    let mut matched = false;
    for mult in 1..=80 {
        doc.walls[0].load_cases[0].g_k_slab_n = base_g * (mult as f64);
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
        let check = report
            .checks
            .iter()
            .find(|c| c.id == format!("en1996.1-2.fire.{wall_id}"))
            .expect("fire check from evaluate_building");
        if (check.limit.value - *t_min).abs() < 1e-12 {
            assert_eq!(check.limit.value, *t_min);
            matched = true;
            break;
        }
    }
    assert!(
        matched,
        "evaluate_building fire limit never matched catalogue cell {t_min} (need α in (0.6, 1.0])"
    );
}
