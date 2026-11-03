use super::*;
use crate::catalogue::piles::{concrete_modulus_pa, pile_elastic_modulus_pa, PILE_CONCRETE_CLASSES};
use semio_s_artifact_norm_en1997_model::PileConcreteClass;

/// 🛟️ Utilisation of the PILE-05 downdrag verdict (F_c,d + F_neg,d against R_c,d) of the first demo pile with a settling ground and the skin friction coefficient `beta`, with the pile's compression utilisation.
fn downdrag(beta: f64) -> (f64, f64) {
    let mut document = semio_s_artifact_norm_en1997_model::compliant_demo();
    let id = document.piles[0].id.clone();
    document.piles[0].settling_thickness = 4.0;
    document.piles[0].negative_friction_beta = beta;
    let report = check_project(&document);
    let find = |wanted: String| report.checks.iter().find(|check| check.id == wanted).unwrap_or_else(|| panic!("no check {wanted}")).clone();
    (find(format!("en1997.pile-05.downdrag.{id}")).limit.value, find(format!("en1997.7.6.2.compression.{id}")).utilization)
}

/// 🛟️ The downdrag verdict is linear in the drag load, `u = (F_c,d + γ_G F_neg,k) / R_c,d`: its intercept is the design compression action over the same design resistance as the 7.6.2 compression check, and the slope is positive.
#[test]
fn the_downdrag_verdict_adds_the_design_drag_force_to_the_action_side_and_divides_by_the_design_compressive_resistance() {
    let (u1, compression) = downdrag(100.0);
    let (u2, _) = downdrag(200.0);
    let (u4, _) = downdrag(400.0);
    assert!(u1 > 1.0 && u2 > u1 && u4 > u2, "the downdrag verdict must grow with beta: {u1} {u2} {u4}");
    assert!(((u4 - u2) - 2.0 * (u2 - u1)).abs() <= 1e-9 * u4, "u must be affine in the drag load");
    let intercept = 2.0 * u1 - u2;
    assert!(intercept > 0.0 && intercept <= compression * (1.0 + 1e-9), "intercept {intercept} must be F_c,d / R_c,d of the compression check {compression}");
}

/// 🧱️ E_cm of every pile concrete class from the shared registry equals the EN 1992-1-1 Table 3.1 formula 22 (f_cm / 10)^0.3 GPa with f_cm = f_ck + 8 MPa to the table rounding of 1 GPa.
#[test]
fn the_pile_concrete_moduli_of_the_shared_registry_follow_the_table_3_1_formula() {
    for class in PILE_CONCRETE_CLASSES {
        let f_ck: f64 = class[1..class.find('/').unwrap()].parse().unwrap();
        let formula = 22.0e9 * ((f_ck + 8.0) / 10.0).powf(0.3);
        let table = concrete_modulus_pa(class).unwrap_or_else(|| panic!("no E_cm for {class}"));
        assert!((table - formula).abs() <= 0.6e9, "{class}: table {table}, formula {formula}");
    }
    assert_eq!(concrete_modulus_pa(" c30/37 "), Some(33.0e9));
    assert_eq!(concrete_modulus_pa("C55/67"), None);
}

/// 🧱️ The pile modulus is the explicit E_c when it is above 0 (the override), else E_cm of the concrete class from the registry, else 0; the class designations equal the catalogue list and every class resolves.
#[test]
fn the_pile_modulus_is_the_explicit_override_else_the_class_modulus_else_zero() {
    let mut pile = semio_s_artifact_norm_en1997_model::compliant_demo().piles[0].clone();
    assert_eq!((pile.concrete_class, pile.elastic_modulus), (PileConcreteClass::C3037, 0.0));
    assert_eq!(pile_elastic_modulus_pa(&pile), 33.0e9);
    pile.elastic_modulus = 28.0e9;
    assert_eq!(pile_elastic_modulus_pa(&pile), 28.0e9);
    pile.elastic_modulus = 0.0;
    pile.concrete_class = PileConcreteClass::NotGiven;
    assert_eq!(pile_elastic_modulus_pa(&pile), 0.0);
    let designations: Vec<&str> = PileConcreteClass::ALL.iter().filter_map(|class| class.designation()).collect();
    assert_eq!(designations, PILE_CONCRETE_CLASSES);
    for class in PileConcreteClass::ALL {
        pile.concrete_class = class;
        assert_eq!(pile_elastic_modulus_pa(&pile) > 0.0, class != PileConcreteClass::NotGiven, "{class:?}");
    }
}

/// 🧱️ PILE-09 reads the class: the demo pile with C30/37 and no explicit modulus gives the same settlement verdict as the explicit 33 GPa, and a lower class (C12/15, E_cm 27 GPa) shortens more, so the utilisation rises.
#[test]
fn the_pile_settlement_verdict_follows_the_concrete_class_unless_an_explicit_modulus_overrides_it() {
    let utilization = |class: PileConcreteClass, modulus: f64| {
        let mut document = semio_s_artifact_norm_en1997_model::compliant_demo();
        document.piles[0].concrete_class = class;
        document.piles[0].elastic_modulus = modulus;
        let report = check_project(&document);
        let check = report.checks.iter().find(|check| check.id.starts_with("en1997.pile-09.")).expect("PILE-09 check").clone();
        check.utilization
    };
    let by_class = utilization(PileConcreteClass::C3037, 0.0);
    let explicit = utilization(PileConcreteClass::NotGiven, 33.0e9);
    let overridden = utilization(PileConcreteClass::C1215, 33.0e9);
    assert!((by_class - explicit).abs() <= 1e-12 * explicit.max(1.0), "{by_class} {explicit}");
    assert!((overridden - explicit).abs() <= 1e-12 * explicit.max(1.0), "{overridden} {explicit}");
    assert!(utilization(PileConcreteClass::C1215, 0.0) > by_class);
}
