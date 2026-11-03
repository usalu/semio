//! 🧱️ Ductility gate of the plastic global analysis: a member analysed plastically needs a material with f_u/f_y, the elongation at failure and eps_u that meet EN 1993-1-1 3.2.2 (V-011 feeds V-029). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use semio_s_artifact_norm_en1993_model::AnalysisKind;

fn rule(id: &str) -> f64 {
    semio_s_plugin_norm_compliance::ndp_common(id)
}

/// 🧱️ Appends one gate check per plastically analysed member: the governing ratio of the ductility requirements to the values of its material.
pub fn check_ductility_gate(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for member in document.members.iter().filter(|m| m.analysis == AnalysisKind::Plastic && scope.admits("members", &m.id)) {
        let Some(material) = find_material(document, &member.material_id) else { continue };
        if material.fy <= 0.0 || material.fu <= 0.0 {
            continue;
        }
        let certificate = document.material_ductility.iter().find(|d| d.material_id == material.id);
        let mut terms = vec![(rule("ndp.en1993.1-1.ductility.fu-fy") * material.fy / material.fu, "f_u/f_y")];
        if let Some(c) = certificate {
            if c.elongation > 0.0 {
                terms.push((rule("ndp.en1993.1-1.ductility.elongation") / c.elongation, "elongation"));
            }
            if c.ultimate_strain > 0.0 && material.e_modulus > 0.0 {
                terms.push((rule("ndp.en1993.1-1.ductility.eps-u-factor") * material.fy / material.e_modulus / c.ultimate_strain, "eps_u"));
            }
        }
        let (u, governing) = terms.iter().fold((0.0, "f_u/f_y"), |a, t| if t.0 > a.0 { (t.0, t.1) } else { a });
        let given = if certificate.is_some() { "with the elongation and eps_u of the certificate" } else { "elongation and eps_u not given" };
        let given_de = if certificate.is_some() { "mit Bruchdehnung und eps_u des Zeugnisses" } else { "Bruchdehnung und eps_u nicht angegeben" };
        let target = subject_member(member, &format!("members[id={}].analysis", member.id));
        let mut b = CheckResult::assess(format!("en1993.5.6.ductility.{}", member.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "5", "5.6"), target.clone(), loc("Ductility of the material for plastic analysis", "Duktilität des Werkstoffs für plastische Berechnung"))
            .annex(annex)
            .ndp("ndp.en1993.v-011.1")
            .utilization(dimensionless(u), dimensionless(1.0))
            .explanation(loc(
                &format!("Material {}: requirement over value {u:.3} for {governing} ({given}); every requirement of 3.2.2(1) must hold for plastic global analysis.", material.id),
                &format!("Werkstoff {}: Anforderung zu Wert {u:.3} für {governing} ({given_de}); für die plastische Berechnung müssen alle Anforderungen nach 3.2.2(1) erfüllt sein.", material.id),
            ))
            .ndp_reads();
        if u > 1.0 {
            b = b.remedy(Remedy::one_of(target, vec!["Elastic".into()], loc("Analyse the member elastically or choose a more ductile material.", "Das Bauteil elastisch berechnen oder einen duktileren Werkstoff wählen.")));
        }
        report.push(b.build());
    }
}
