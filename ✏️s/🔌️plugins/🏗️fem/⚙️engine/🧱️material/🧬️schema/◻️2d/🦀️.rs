//! 🧱️ Dimension-specific actual material and explicit authored reference value.
/// 🧱️ An isotropic material — Young's modulus `e` in Pascals, Poisson's ratio `nu`, density `rho`
/// in kg/m³ (the latter two required for continuum `FemRegion` elements and self-weight).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemMaterialValue {
    pub id: String,
    pub name: String,
    #[dsl(unit = "Pa")]
    pub e: f64,
    pub nu: f64,
    #[dsl(unit = "kg/m3")]
    pub rho: f64,
}

super::native_material_row!(FemMaterial,FemMaterialValue,3,{e:2,nu:3,rho:4});
#[cfg(test)]#[path="../../🧪️tests/🦀️.rs"]mod native_material_tests;
