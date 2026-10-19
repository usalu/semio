//! 🧱️ Dimension-specific actual material and explicit authored reference value.
/// 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
/// `nu` (dimensionless, drives `Tet4` solid elements), and density `rho` (kg/m³, drives self-weight via
/// `Bar3`/`Frame3`/`Tet4`'s `mass()`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "material")]
pub struct FemMaterialValue {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub g: f64,
    pub nu: f64,
    pub rho: f64,
}

super::native_material_row!(FemMaterial,FemMaterialValue,4,{e:2,g:3,nu:4,rho:5});
#[cfg(test)]#[path="../../🧪️tests/🦀️.rs"]mod native_material_tests;
