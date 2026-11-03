//! 🗣️ Coded, bilingual reasons: every catalogue evaluator explains an insufficient input with one rule of [`RULES`] (stable code, English and German template) instead of a formatted string.

/// 🧾️ One localised reason rule: a stable `code`, an English pattern whose `{}` holes capture parameters, and the German template that places them as `{0}`, `{1}`, ….
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReasonRule {
    pub code: &'static str,
    pub en: &'static str,
    pub de: &'static str,
}

/// 🗣️ A reason resolved against [`RULES`]: its code, captured parameters and both texts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reason {
    pub code: &'static str,
    pub params: Vec<String>,
    pub en: String,
    pub de: String,
}

/// 🚫️ Code of a reason that no rule covers; its German text repeats the English one and the reason test fails on it.
pub const UNMAPPED_REASON: &str = "reason.unmapped";

const fn rule(code: &'static str, en: &'static str, de: &'static str) -> ReasonRule {
    ReasonRule { code, en, de }
}

/// 📚️ Every reason the Eurocode catalogue evaluators raise, most specific first.
pub const RULES: &[ReasonRule] = &[
    rule("input.missing-runway", "missing inputs: give sum_q_r and runway_beams, or xi, m_drive and a, or skew_rad, or phi_7, v_1, m_c and s_b", "Eingaben fehlen: sum_q_r und runway_beams, oder xi, m_drive und a, oder skew_rad, oder phi_7, v_1, m_c und s_b angeben"),
    rule("input.missing",  "missing input {}", "Eingabe {0} fehlt"),
    rule("input.missing-any", "missing inputs: {}", "Eingaben fehlen: {0}"),
    rule("input.positive", "input {} must be positive", "Eingabe {0} muss positiv sein"),
    rule("input.non-negative", "input {} must not be negative", "Eingabe {0} darf nicht negativ sein"),
    rule("input.at-least-one", "input {} must be at least 1", "Eingabe {0} muss mindestens 1 sein"),
    rule("input.exceed-one", "input {} must exceed 1", "Eingabe {0} muss größer als 1 sein"),
    rule("input.range", "input {} must lie within {} to {}", "Eingabe {0} muss zwischen {1} und {2} liegen"),
    rule("input.one-of-three", "input {} must be {}, {} or {}", "Eingabe {0} muss {1}, {2} oder {3} sein"),
    rule("catalogue.unknown-entry", "unknown catalogue entry {}", "unbekannter Katalogeintrag {0}"),
    rule("value.unknown-structure-admissible", "unknown structure {}; admissible: {}", "unbekanntes Tragwerk {0}; zulässig: {1}"),
    rule("value.unknown-structure", "unknown structure {}", "unbekanntes Tragwerk {0}"),
    rule("value.unknown-distribution", "unknown distribution {}", "unbekannte Verteilung {0}"),
    rule("value.unknown-test-distribution", "unknown test distribution {}", "unbekannte Versuchsverteilung {0}"),
    rule("value.unknown-use", "unknown use {}", "unbekannte Nutzung {0}"),
    rule("value.unknown-situation", "unknown situation {}", "unbekannte Bemessungssituation {0}"),
    rule("value.unknown-psi-category", "unknown ψ category {}", "unbekannte ψ-Kategorie {0}"),
    rule("value.unknown-psi-category-action", "variable action {} has unknown ψ category {}", "veränderliche Einwirkung {0} hat die unbekannte ψ-Kategorie {1}"),
    rule("value.unknown-material-admissible", "unknown material {}; admissible: {}", "unbekannter Werkstoff {0}; zulässig: {1}"),
    rule("value.unknown-material", "unknown material {}", "unbekannter Werkstoff {0}"),
    rule("value.unknown-material-bare", "unknown material", "unbekannter Werkstoff"),
    rule("value.unknown-analysis", "unknown analysis {}", "unbekannte Berechnungsart {0}"),
    rule("value.unknown-category-admissible", "unknown category {}; admissible: {}", "unbekannte Kategorie {0}; zulässig: {1}"),
    rule("value.unknown-tyres", "unknown tyres {}; admissible: {}", "unbekannte Bereifung {0}; zulässig: {1}"),
    rule("value.unknown-snow-zone", "unknown snow zone {}; admissible: {}", "unbekannte Schneelastzone {0}; zulässig: {1}"),
    rule("value.unknown-cen-zone", "unknown CEN zone {}; admissible: {}", "unbekannte CEN-Zone {0}; zulässig: {1}"),
    rule("value.unknown-snow-state", "unknown snow state {}; admissible: {}", "unbekannter Schneezustand {0}; zulässig: {1}"),
    rule("value.unknown-curve", "unknown curve {}; admissible: {}", "unbekannte Brandkurve {0}; zulässig: {1}"),
    rule("value.unknown-occupancy-admissible", "unknown occupancy {}; admissible: {}", "unbekannte Nutzungsart {0}; zulässig: {1}"),
    rule("value.unknown-occupancy", "unknown occupancy", "unbekannte Nutzungsart"),
    rule("value.unknown-terrain-category", "unknown terrain category {}; admissible: {}", "unbekannte Geländekategorie {0}; zulässig: {1}"),
    rule("value.unknown-wind-zone", "unknown wind zone {}; admissible: {}", "unbekannte Windzone {0}; zulässig: {1}"),
    rule("value.unknown-mode", "unknown mode {}; admissible: {}", "unbekannte Tragwerksart {0}; zulässig: {1}"),
    rule("value.unknown-load-group", "unknown load group {}; admissible: {}", "unbekannte Lastgruppe {0}; zulässig: {1}"),
    rule("value.unknown-member-admissible", "unknown member {}; admissible: {}", "unbekanntes Bauteil {0}; zulässig: {1}"),
    rule("value.unknown-member-kind", "unknown member kind", "unbekannte Bauteilart"),
    rule("value.unknown-direction", "unknown direction {}; admissible: {}", "unbekannte Richtung {0}; zulässig: {1}"),
    rule("value.unknown-model", "unknown model {}; admissible: {}", "unbekanntes Modell {0}; zulässig: {1}"),
    rule("value.unknown-hoist-class-admissible", "unknown hoist class {}; admissible: {}", "unbekannte Hubklasse {0}; zulässig: {1}"),
    rule("value.unknown-hoist-class", "unknown hoist class {}", "unbekannte Hubklasse {0}"),
    rule("value.unknown-deck-type-admissible", "unknown deck type {}; admissible: {}", "unbekannter Überbautyp {0}; zulässig: {1}"),
    rule("value.unknown-deck-type", "unknown deck type", "unbekannter Überbautyp"),
    rule("value.unknown-exposure-class", "unknown exposure class", "unbekannte Expositionsklasse"),
    rule("value.unknown-exposure", "unknown exposure {}", "unbekannte Exposition {0}"),
    rule("value.unknown-strength-class-for", "unknown strength class for {}", "unbekannte Festigkeitsklasse für {0}"),
    rule("value.unknown-strength-class", "unknown strength class {}", "unbekannte Festigkeitsklasse {0}"),
    rule("value.unknown-charring-material", "unknown charring material", "unbekannter Abbrandwerkstoff"),
    rule("value.unknown-lamella-class", "unknown lamella class", "unbekannte Lamellenklasse"),
    rule("value.unknown-system", "unknown system {}", "unbekanntes System {0}"),
    rule("value.unknown-gamma-m-material", "unknown γM material", "unbekannter γM-Werkstoff"),
    rule("value.unknown-mortar", "unknown mortar {}", "unbekannter Mörtel {0}"),
    rule("value.unknown-action-class", "unknown action class {} in {}", "unbekannte Einwirkungsklasse {0} in {1}"),
    rule("value.set-geo", "set must be B, C (GEO-3) or C-geo2, got {}", "Satz muss B, C (GEO-3) oder C-geo2 sein, erhalten: {0}"),
    rule("value.category-table", "category {} belongs to Table {}, not Annex {}", "Kategorie {0} gehört zu Tabelle {1}, nicht zu Anhang {2}"),
    rule("value.rc-range", "rc must be 1..3", "rc muss 1..3 sein"),
    rule("value.rc-range-years", "rc must be 1..3 and t_years positive", "rc muss 1..3 sein und t_years positiv"),
    rule("value.category-range", "category must be 1..5", "Kategorie muss 1..5 sein"),
    rule("value.forklift-class", "forklift class must be 1 to 6", "Staplerklasse muss 1 bis 6 sein"),
    rule("value.alpha-classification", "classification factor alpha must be one of {}", "Anpassungsfaktor alpha muss einer der Werte {0} sein"),
    rule("value.zone-roof-table", "no c_pe table for zone {} and roof type {}", "keine c_pe-Tabelle für Zone {0} und Dachform {1}"),
    rule("value.no-barrier-load", "no barrier line load for category {}", "keine Absturzsicherungs-Linienlast für Kategorie {0}"),
    rule("value.wall-load-range", "wall load {} kN/m exceeds the equivalent-load range; apply the wall as an individual line load", "Wandlast {0} kN/m überschreitet den Ersatzlastbereich; Wand als einzelne Linienlast ansetzen"),
    rule("value.probability-range", "annual exceedance probability must lie within (0, 1)", "jährliche Überschreitungswahrscheinlichkeit muss in (0, 1) liegen"),
    rule("value.whole-tests", "n must be a whole number of tests, got {}", "n muss eine ganze Anzahl Versuche sein, erhalten: {0}"),
    rule("value.test-count", "n must be at least 1 (V known) or 3 (V unknown)", "n muss mindestens 1 (V bekannt) oder 3 (V unbekannt) sein"),
    rule("value.table-d1", "Table D1 has no k_n for n = {} with V_X {}", "Tabelle D1 enthält kein k_n für n = {0} mit V_X {1}"),
    rule("value.must-be-positive-finite", "{} must be a positive finite number", "{0} muss eine positive endliche Zahl sein"),
    rule("value.must-be-within", "{} must be within {}..{}", "{0} muss zwischen {1} und {2} liegen"),
    rule("value.must-be-either", "{} must be {} or {}", "{0} muss {1} oder {2} sein"),
    rule("series.action-row", "action row {} needs id|class|value", "Einwirkungszeile {0} benötigt id|class|value"),
    rule("series.action-value", "bad action value {} in {}", "ungültiger Einwirkungswert {0} in {1}"),
    rule("series.spectrum-row", "spectrum row {} needs range:cycles", "Kollektivzeile {0} benötigt range:cycles"),
    rule("series.bad-range", "bad range in {}", "ungültige Schwingbreite in {0}"),
    rule("series.bad-cycles", "bad cycles in {}", "ungültige Lastspielzahl in {0}"),
    rule("series.empty-spectrum", "empty spectrum", "leeres Kollektiv"),
    rule("series.no-actions", "no actions", "keine Einwirkungen"),
    rule("series.situations-empty", "situations is empty", "Bemessungssituationen sind leer"),
    rule("series.spectrum-bin", "spectrum bin (Δσ = {}, n = {}, Δσ_C = {}) needs finite non-negative Δσ and n and a positive Δσ_C", "Kollektivstufe (Δσ = {0}, n = {1}, Δσ_C = {2}) benötigt endliche, nicht negative Δσ und n sowie ein positives Δσ_C"),
    rule("series.equal-length", "ax and ay differ in length", "ax und ay haben unterschiedliche Länge"),
    rule("series.increasing", "times_min and temps_c must be strictly increasing series of equal length ≥ 2", "times_min und temps_c müssen streng steigende Reihen gleicher Länge ≥ 2 sein"),
    rule("series.non-empty-equal", "p_h, p_d_h and consequence must be non-empty series of equal length", "p_h, p_d_h und consequence müssen nicht leere Reihen gleicher Länge sein"),
    rule("series.vectors-equal", "vectors l_m, t_m, x_m, vrd_n must have equal non-zero length", "Vektoren l_m, t_m, x_m, vrd_n müssen gleiche Länge größer null haben"),
    rule("series.bars-equal", "bar_y_m and bar_a_m2 must have equal non-zero length", "bar_y_m und bar_a_m2 müssen gleiche Länge größer null haben"),
    rule("series.tendons", "tendon lists t_y, t_z, t_a are required", "Spanngliedlisten t_y, t_z, t_a sind erforderlich"),
    rule("series.edges", "edges must be four letters of F, S, X (top, bottom, left, right)", "Ränder müssen vier Buchstaben aus F, S, X sein (oben, unten, links, rechts)"),
    rule("series.layers-moduli", "layer series h1…hn and e1…en required", "Schichtreihen h1…hn und e1…en erforderlich"),
    rule("series.layers-moduli-raft", "layer series h1…hn and e1…en below the equivalent raft required", "Schichtreihen h1…hn und e1…en unter dem Ersatzfundament erforderlich"),
    rule("series.layers-unit-weights", "series h1…hn and gamma1…gamma_n required", "Reihen h1…hn und gamma1…gamma_n erforderlich"),
    rule("series.stiffness-stress", "series e1…en and sigma1…sigman (n ≥ 3) required", "Reihen e1…en und sigma1…sigman (n ≥ 3) erforderlich"),
    rule("series.triaxial-pairs", "series p1…pn and q1…qn (n ≥ 2) required", "Reihen p1…pn und q1…qn (n ≥ 2) erforderlich"),
    rule("series.shear-pairs", "series sigma1…sigman and tau1…taun (n ≥ 3) required", "Reihen sigma1…sigman und tau1…taun (n ≥ 3) erforderlich"),
    rule("series.triaxial-or-shear", "series p1…pn and q1…qn (triaxial) or sigma1…sigman and tau1…taun (shear), n ≥ 3, required", "Reihen p1…pn und q1…qn (triaxial) oder sigma1…sigman und tau1…taun (Scherversuch), n ≥ 3, erforderlich"),
    rule("series.heights-vertical-stress", "series h1…hn and sv1…svn required", "Reihen h1…hn und sv1…svn erforderlich"),
    rule("series.settlement-times", "series ts1…tsn (strictly increasing settlement) and tq1…tqn of equal length (n ≥ 2) required", "Reihen ts1…tsn (streng steigende Setzung) und tq1…tqn gleicher Länge (n ≥ 2) erforderlich"),
    rule("series.samples", "series x1…xn of samples required", "Reihe x1…xn der Proben erforderlich"),
    rule("series.measured-resistances", "series rm1…rmn of measured resistances required", "Reihe rm1…rmn gemessener Widerstände erforderlich"),
    rule("series.dynamic-resistances", "series rm1…rmn of dynamic test resistances required", "Reihe rm1…rmn dynamischer Probebelastungswiderstände erforderlich"),
    rule("series.pull-out-resistances", "series ra1…ran of suitability test pull-out resistances required", "Reihe ra1…ran der Herausziehwiderstände aus Eignungsprüfungen erforderlich"),
    rule("series.base-shaft", "series rb1…rbn and rs1…rsn of equal length required", "Reihen rb1…rbn und rs1…rsn gleicher Länge erforderlich"),
    rule("series.rt-or-series", "rt_k or series rt1…rtn required", "rt_k oder Reihe rt1…rtn erforderlich"),
    rule("series.required", "series {} required", "Reihen {0} erforderlich"),
    rule("series.required-with-n", "series {} and {} (n ≥ {}) required", "Reihen {0} und {1} (n ≥ {2}) erforderlich"),
    rule("series.layer", "layer series {} required", "Schichtreihen {0} erforderlich"),
    rule("series.layer-depth", "evaluation depth exceeds the layer series", "Auswertungstiefe überschreitet die Schichtreihe"),
    rule("input.give-moments", "give g_mean and v_g, or q_mean and q_cov", "g_mean und v_g oder q_mean und q_cov angeben"),
    rule("input.give-bridge-sls", "give at least one of acc_v, acc_h, deck_acc, twist with v_kmh, deflection with span", "mindestens eines von acc_v, acc_h, deck_acc, twist mit v_kmh, deflection mit span angeben"),
    rule("input.statistics", "test statistics need a positive mean, a non-negative standard deviation and V_X and a positive η_d", "Versuchsstatistik benötigt einen positiven Mittelwert, eine nicht negative Standardabweichung, V_X und ein positives η_d"),
    rule("input.reliability-moments", "{}: mean and a positive standard deviation are required", "{0}: Mittelwert und eine positive Standardabweichung sind erforderlich"),
    rule("input.snow-factors", "mu must not be negative and C_e, C_t must be positive", "mu darf nicht negativ sein und C_e, C_t müssen positiv sein"),
    rule("input.snow-region", "exceptional snow load applies to the North German lowland only", "außergewöhnliche Schneelast gilt nur im Norddeutschen Tiefland"),
    rule("input.height-above-origin", "height z must lie above the virtual origin", "Höhe z muss über dem virtuellen Ursprung liegen"),
    rule("input.curve-needs-compartment", "curve needs compartment data", "Kurve benötigt Brandabschnittsdaten"),
    rule("input.category-h", "category H missing", "Kategorie H fehlt"),
    rule("input.no-execution", "no execution input given", "keine Ausführungsangabe vorhanden"),
    rule("input.k-def", "k_def not allowed", "k_def nicht zulässig"),
    rule("input.k-mod", "k_mod not allowed for service class", "k_mod für die Nutzungsklasse nicht zulässig"),
    rule("input.notch", "notch geometry requires 0 < h_ef/h ≤ 1 and x ≥ 0", "Ausklinkungsgeometrie erfordert 0 < h_ef/h ≤ 1 und x ≥ 0"),
    rule("input.moment-line", "the moment line of the member is unknown: give the span l and the load case", "Die Momentenlinie des Bauteils ist unbekannt: Spannweite l und Lastfall angeben"),
    rule("input.load-case-moment-only", "load case {} only defines a moment line", "Der Lastfall {0} legt nur eine Momentenlinie fest"),
    rule("input.stiffness", "stiffness sum must be positive", "Steifigkeitssumme muss positiv sein"),
    rule("input.polygon", "polygon needs three points", "Polygon benötigt drei Punkte"),
    rule("input.stiffness-pair", "k_s with ei, or gamma with l and phi_deg required", "k_s mit ei oder gamma mit l und phi_deg erforderlich"),
    rule("resistance.missing", "missing resistances", "Widerstände fehlen"),
    rule("resistance.no-tension-anchor", "no tension anchor", "kein Zuganker"),
    rule("resistance.sigma-s-range", "σ_s outside Tables 7.2N and 7.3N", "σ_s außerhalb der Tabellen 7.2N und 7.3N"),
    rule("resistance.uncracked", "section is uncracked: w_k = 0", "Querschnitt ungerissen: w_k = 0"),
    rule("resistance.fck-scope", "f_ck above 90 MPa is outside the scope of EN 1992-1-1", "f_ck über 90 MPa liegt außerhalb des Anwendungsbereichs von EN 1992-1-1"),
    rule("resistance.fire-ned", "N_Ed,fi outside the fire resistance", "N_Ed,fi außerhalb des Feuerwiderstands"),
    rule("resistance.truss", "truss is not statically determinate or unstable", "Fachwerk ist nicht statisch bestimmt oder instabil"),
    rule("resistance.mu-max", "μ_Eds above its maximum: section cannot carry the moment", "μ_Eds über dem Höchstwert: Querschnitt kann das Moment nicht aufnehmen"),
    rule("resistance.compression-steel", "compression steel d2 lies below the neutral axis at the strain limit", "Druckbewehrung d2 liegt an der Dehnungsgrenze unterhalb der Nulllinie"),
    rule("resistance.biaxial-range", "N_Ed outside the biaxial resistance range", "N_Ed außerhalb des zweiachsigen Tragfähigkeitsbereichs"),
    rule("resistance.ned-range", "N_Ed outside range", "N_Ed außerhalb des Bereichs"),
    rule("resistance.ned-section", "N_Ed outside the section resistance", "N_Ed außerhalb der Querschnittstragfähigkeit"),
    rule("resistance.cracked-m", "section uncracked at M_max", "Querschnitt bei M_max ungerissen"),
    rule("resistance.no-load", "section cannot carry any load", "Querschnitt kann keine Last aufnehmen"),
    rule("resistance.load-capacity", "load exceeds the sum of shaft and base capacity", "Last überschreitet die Summe aus Mantel- und Fußtragfähigkeit"),
    rule("resistance.horizontal-vc", "horizontal action exceeds V + A′c′cot φ′", "Horizontalkraft überschreitet V + A′c′cot φ′"),
    rule("resistance.horizontal-cu", "horizontal action exceeds A′·c_u", "Horizontalkraft überschreitet A′·c_u"),
    rule("resistance.drained-phi", "drained bearing needs φ′ > 0.5° (use the undrained entry)", "dränierter Grundbruch benötigt φ′ > 0,5° (undränierten Eintrag verwenden)"),
    rule("resistance.din4017-phi", "DIN 4017 entry needs φ′ > 0.5°", "Eintrag nach DIN 4017 benötigt φ′ > 0,5°"),
    rule("resistance.din4017-hv", "H/V must stay below 1 for DIN 4017", "H/V muss für DIN 4017 unter 1 bleiben"),
    rule("geometry.no-limit-equilibrium", "no limit equilibrium: φ − θ − β < 0", "kein Grenzgleichgewicht: φ − θ − β < 0"),
    rule("geometry.slip-circle", "slip circle does not cut the slope", "Gleitkreis schneidet die Böschung nicht"),
    rule("geometry.embedment", "no equilibrium embedment found", "keine Gleichgewichts-Einbindetiefe gefunden"),
    rule("geometry.well-level", "water level in the well must be below the undisturbed head", "Wasserstand im Brunnen muss unter dem ungestörten Wasserstand liegen"),
    rule("geometry.pit-level", "water level in the pit must be below the undisturbed head", "Wasserstand in der Baugrube muss unter dem ungestörten Wasserstand liegen"),
    rule("geometry.flow-net", "flow net needs ΔH > 0 and a layer thicker than the embedment", "Potentialnetz benötigt ΔH > 0 und eine Schicht, die dicker als die Einbindetiefe ist"),
    rule("geometry.base-width", "base width must exceed toe length plus stem thickness", "Fundamentbreite muss größer als Fußlänge plus Stielstärke sein"),
    rule("geometry.zero-net-pressure", "no depth of zero net pressure found below the dredge level", "unter der Baugrubensohle wurde keine Tiefe mit verschwindendem Netto-Erddruck gefunden"),
    rule("geometry.prop-depth", "prop depth must lie above the dredge level", "Stützentiefe muss über der Baugrubensohle liegen"),
    rule("geometry.kranz", "degenerate Kranz geometry", "entartete Kranz-Geometrie"),
    rule("geometry.observation-wells", "observation wells must satisfy r2 > r1 and h2 > h1", "Beobachtungsbrunnen müssen r2 > r1 und h2 > h1 erfüllen"),
    rule("geometry.t2-exceeds-t1", "t2 must exceed t1", "t2 muss größer als t1 sein"),
    rule("value.gamma-sat", "γ_sat must exceed γ_w", "γ_sat muss größer als γ_w sein"),
    rule("value.exit-gradient", "the exit gradient must be positive", "der Austrittsgradient muss positiv sein"),
    rule("value.net-cone-resistance", "net cone resistance must be positive", "der Netto-Spitzenwiderstand muss positiv sein"),
    rule("value.tendon-modulus-area", "tendon modulus and area must be positive", "E-Modul und Fläche des Spannglieds müssen positiv sein"),
    rule("value.raft-depth-factor", "raft_depth_factor must be below 1", "raft_depth_factor muss kleiner als 1 sein"),
    rule("value.lower-confidence-slope", "the lower confidence slope must stay positive (and below 1 for sin φ′)", "die untere Vertrauensgrenze der Steigung muss positiv bleiben (und für sin φ′ unter 1)"),
    rule("input.profile-pointer", "at least one profile pointer p1_h1, p1_qs1, p1_qb required", "mindestens ein Profilzeiger p1_h1, p1_qs1, p1_qb erforderlich"),
    rule("input.profile-series", "each profile needs equally long series pJ_h1…, pJ_qs1…", "jedes Profil benötigt gleich lange Reihen pJ_h1…, pJ_qs1…"),
    rule("input.creep-segments", "creep segments lv1…, lh1… or i_exit or d_embed required", "Sickerstrecken lv1…, lh1… oder i_exit oder d_embed erforderlich"),
    rule("action.no-accidental", "no accidental action (class A)", "keine außergewöhnliche Einwirkung (Klasse A)"),
    rule("action.no-seismic", "no seismic action (class E)", "keine Erdbebeneinwirkung (Klasse E)"),
    rule("form.not-converged", "FORM did not converge in {} iterations", "FORM ist nach {0} Iterationen nicht konvergiert"),
    rule("form.not-converged-indeterminate", "FORM did not converge in {} iterations (indeterminate, no verdict)", "FORM ist nach {0} Iterationen nicht konvergiert (unbestimmt, kein Urteil)"),
];

fn capture(pattern: &str, text: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = pattern.split("{}").collect();
    let mut rest = text.strip_prefix(parts[0])?;
    let mut values = Vec::with_capacity(parts.len() - 1);
    for (index, literal) in parts.iter().enumerate().skip(1) {
        if index == parts.len() - 1 {
            let value = rest.strip_suffix(literal)?;
            if value.is_empty() {
                return None;
            }
            values.push(value.to_string());
            return Some(values);
        }
        let at = rest.find(literal).filter(|at| *at > 0 && !literal.is_empty())?;
        values.push(rest[..at].to_string());
        rest = &rest[at + literal.len()..];
    }
    rest.is_empty().then_some(values)
}

fn german_params(value: &str) -> String {
    value.replace("(or ", "(oder ").replace(" or ", " oder ").replace(" and ", " und ").replace(" with ", " mit ")
}

fn fill(template: &str, values: &[String]) -> String {
    let mut text = template.to_string();
    for (index, value) in values.iter().enumerate() {
        text = text.replace(&format!("{{{index}}}"), value);
    }
    text
}

/// 🗣️ Resolves an English evaluator reason against [`RULES`]; a reason without a rule keeps its text and the [`UNMAPPED_REASON`] code.
pub fn explain_reason(reason: &str) -> Reason {
    let reason = reason.trim().trim_end_matches('.');
    for rule in RULES {
        if let Some(values) = capture(rule.en, reason) {
            let german: Vec<String> = values.iter().map(|value| german_params(value)).collect();
            return Reason { code: rule.code, en: reason.to_string(), de: fill(rule.de, &german), params: values };
        }
    }
    Reason { code: UNMAPPED_REASON, params: Vec::new(), en: reason.to_string(), de: reason.to_string() }
}

/// 🗣️ The reason as a check explanation `"<id>: <reason>."` in both languages.
pub fn reason_copy(id: &str, reason: &str) -> crate::LocalizedCopy {
    let resolved = explain_reason(reason);
    crate::LocalizedCopy::new(format!("{id}: {}.", resolved.en), format!("{id}: {}.", resolved.de))
}

const ENGLISH_MARKERS: &[&str] = &[
    "the", "and", "of", "for", "not", "with", "is", "are", "to", "must", "missing", "input", "exceeds", "check", "value", "unknown", "required", "should", "than", "from", "which", "does", "has", "have", "below", "above", "within", "increase", "reduce", "raise", "limit", "load", "length", "width", "thickness", "height", "increases", "applies", "only", "outside", "range", "needs", "give", "series", "case", "member", "section", "resistance", "design",
];

const GERMAN_MARKERS: &[&str] = &[
    "der", "die", "das", "und", "oder", "nicht", "mit", "für", "von", "ist", "sind", "muss", "fehlt", "zu", "bei", "auf", "dem", "den", "ein", "eine", "unbekannte", "unbekannter", "unbekanntes", "erforderlich", "zulässig", "kein", "keine", "über", "unter", "zwischen", "als", "nach", "aus", "im", "wird", "werden", "erhöhen", "verringern", "dicke", "länge", "breite", "höhe", "last", "nachweis", "bauteil", "wert", "eingabe",
];

/// 🔎️ Whether `text` reads as an English sentence: ASCII-only words of three or more letters (symbols, units, ids and numbers are not words), at least one English marker word and no German marker word. Used by the German-rendering tests; symbols and units are the allowlist.
pub fn looks_english(text: &str) -> bool {
    let words: Vec<String> = text.split(|c: char| !c.is_alphabetic() && c != '\'').filter(|word| word.chars().count() >= 3).map(str::to_lowercase).collect();
    if words.len() < 2 || text.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) {
        return false;
    }
    words.iter().any(|word| ENGLISH_MARKERS.contains(&word.as_str())) && !words.iter().any(|word| GERMAN_MARKERS.contains(&word.as_str()))
}

/// 🇩🇪️ Every German text of `checks` (titles, explanations, remedy actions, subject labels) that still reads as English, as `check id: text`.
pub fn german_check_leaks(checks: &[crate::CheckResult]) -> Vec<String> {
    let mut leaks = Vec::new();
    for check in checks {
        let texts = [&check.title.de, &check.explanation.de, &check.subject.label.de].into_iter().chain(check.remedies.iter().map(|remedy| &remedy.action.de));
        for text in texts {
            if looks_english(text) {
                leaks.push(format!("{}: {text}", check.id));
            }
        }
    }
    leaks
}

/// 🇩🇪️ [`german_check_leaks`] over a whole report.
pub fn german_leaks(report: &crate::CheckReport) -> Vec<String> {
    german_check_leaks(&report.checks)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
