//! 🧮️ Facts: the numbers the scene derives from the same clause functions the checks use (alloy band, plate strips of the HAZ and of the local buckling reduction, element classification), so drawing and verification cannot drift apart.

use super::checks::part_1_1::{self, AlloyProps, EffCtx, Role, SectionClass};
use super::model::{AluminiumMember, AluminiumSection, En1999Snapshot, PlateElement};
use super::norm::CheckReport;

/// 🟫️ One rectangular plate strip of the section in outline coordinates (centre `y`, `z`; gross extents `dy`, `dz`) with the thickness factor it carries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Band {
    pub y: f64,
    pub z: f64,
    pub dy: f64,
    pub dz: f64,
    pub factor: f64,
    pub thin_z: bool,
    pub web: bool,
}

impl Band {
    pub fn effective(&self) -> (f64, f64) {
        if self.thin_z { (self.dy, self.dz * self.factor) } else { (self.dy * self.factor, self.dz) }
    }
}

/// 🔩️ The alloy band of a member's material at the thickest wall of its section: the very lookup the member checks perform.
pub fn alloy_of(document: &En1999Snapshot, material_id: &str, section: &AluminiumSection) -> Option<AlloyProps> {
    let material = document.materials.iter().find(|m| m.id == material_id)?;
    part_1_1::resolve_alloy_for_thickness(&material.designation, part_1_1::max_thickness(section))
}

pub fn member_parts<'a>(document: &'a En1999Snapshot, member: &AluminiumMember) -> Option<(&'a AluminiumSection, AlloyProps)> {
    let section = document.sections.iter().find(|s| s.id == member.section_id)?;
    Some((section, alloy_of(document, &member.material_id, section)?))
}

fn shift(section: &AluminiumSection) -> (f64, f64) {
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => ((section.width - section.web_thickness) / 2.0, 0.0),
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => (section.width / 2.0, section.height / 2.0),
        _ => (0.0, 0.0),
    }
}

fn bands(section: &AluminiumSection, ctx: &EffCtx) -> Vec<Band> {
    if part_1_1::is_tube(section) {
        return Vec::new();
    }
    let (sy, sz) = shift(section);
    part_1_1::strips(section, ctx).into_iter().map(|s| Band { y: s.yc - sy, z: s.zc - sz, dy: s.dy, dz: s.dz, factor: s.factor, thin_z: s.thin_z, web: s.role == Role::Web }).collect()
}

fn context(alloy: &AlloyProps, eta: f64, local: bool, haz: bool) -> EffCtx {
    EffCtx { f_o_pa: alloy.f_o_pa, class_a: alloy.buckling_class_a, eta_web: eta, local, haz_rho: haz.then_some(alloy.rho_o_haz), haz_extents: &[] }
}

/// 🌡️ Every gross plate strip of the section in outline coordinates, in the order of `part_1_1::strips` (the order of `part_1_2::temperature_field`).
pub fn gross_bands(section: &AluminiumSection, alloy: &AlloyProps) -> Vec<Band> {
    bands(section, &context(alloy, 1.0, false, false))
}

pub fn haz_bands(section: &AluminiumSection, alloy: &AlloyProps) -> Vec<Band> {
    bands(section, &context(alloy, 1.0, false, true)).into_iter().filter(|b| b.factor < 1.0 - 1e-9).collect()
}

pub fn local_bands(section: &AluminiumSection, alloy: &AlloyProps, eta: f64) -> Vec<Band> {
    bands(section, &context(alloy, eta, true, false)).into_iter().filter(|b| b.factor < 1.0 - 1e-9).collect()
}

pub fn effective_bands(section: &AluminiumSection, alloy: &AlloyProps, eta: f64) -> Vec<Band> {
    bands(section, &context(alloy, eta, true, true))
}

/// 📈️ The stress-gradient factor η the member checks applied to internal elements, recovered exactly from the reported β/ε of the first internal element.
pub fn eta_of(report: &CheckReport, member: &str, section: &AluminiumSection, alloy: &AlloyProps) -> f64 {
    section
        .elements
        .iter()
        .filter(|e| !e.outstand)
        .find_map(|e| {
            let id = format!("en1999.6.1.4.class.{member}.{}", e.id);
            let reported = report.checks.iter().find(|c| c.id == id)?;
            let raw = part_1_1::element_beta_over_eps(e, alloy.f_o_pa, 1.0);
            (raw > 1e-12).then(|| (reported.computed.value / raw).clamp(0.05, 1.0))
        })
        .unwrap_or(1.0)
}

/// 🏷️ Classification of one plate element: β/ε, limits, class and ρ_c.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Classified {
    pub beta_over_eps: f64,
    pub limits: [f64; 3],
    pub class: SectionClass,
    pub rho: f64,
}

pub fn classify(element: &PlateElement, alloy: &AlloyProps, eta: f64) -> Classified {
    let beta_over_eps = part_1_1::element_beta_over_eps(element, alloy.f_o_pa, eta);
    let limits = part_1_1::class_limits(alloy.buckling_class_a, element.outstand, element.welded);
    Classified { beta_over_eps, limits, class: part_1_1::class_of(beta_over_eps, limits), rho: part_1_1::rho_c(beta_over_eps, alloy.buckling_class_a, element.outstand, element.welded) }
}

pub fn class_number(class: SectionClass) -> f64 {
    f64::from(class as u8)
}

/// 🌡️ b_haz of a plate of thickness `t` [m], the table value of EN 1999-1-1 6.1.6.3.
pub fn haz_extent(t: f64) -> f64 {
    part_1_1::haz_width(t)
}
