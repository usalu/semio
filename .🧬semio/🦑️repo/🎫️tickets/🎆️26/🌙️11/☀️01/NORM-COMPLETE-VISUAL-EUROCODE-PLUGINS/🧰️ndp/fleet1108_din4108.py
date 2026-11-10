"""🌡️ Adds the DIN 4108 / EN ISO 6946 NDP rows requested by lane din4108-fix. Run: python3 -I fleet1108_din4108.py REGISTRY.json. Idempotent; every row stays unverified (no library encodes them)."""
import json, pathlib, sys

PEND = "to be recorded at verification"
NOTE = "fleet-1108 ndp 2026-11-08: UNVERIFIED. No third-party library in the venv (blue-prints, structuralcodes, groundhog, concreteproperties, sectionproperties) encodes this constant; value taken from the lane din4108-fix request and the oracle audit, not from the printed standard."


def row(rid, code, doc, edition, locator, symbol, value, unit, written, spec):
    return dict(id=rid, code=code, part="", clause=locator, symbol=symbol, recommended=value, naDe=value, unit=unit,
                source=dict(document=doc, edition=edition, locator=locator), status="unverified", specRef=spec, asWritten=written, evidence=NOTE)


D3, D2, I6 = ("DIN 4108-3", "DIN 4108-3:2018-10", "2018-10"), ("DIN 4108-2", "DIN 4108-2", PEND), ("EN ISO 6946", "EN ISO 6946", PEND)
A = "Annex A"
ROWS = [
    row("ndp.din4108-3.annex-a.condensation.theta_i", D3[0], D3[1], D3[2], A, "theta_i condensation period", 20.0, "degC", "Glaser condensation period interior 20 degC.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.condensation.phi_i", D3[0], D3[1], D3[2], A, "phi_i condensation period", 0.5, "", "Glaser condensation period interior relative humidity 50 %.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.condensation.theta_e", D3[0], D3[1], D3[2], A, "theta_e condensation period", -5.0, "degC", "Glaser condensation period exterior -5 degC.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.condensation.phi_e", D3[0], D3[1], D3[2], A, "phi_e condensation period", 0.8, "", "Glaser condensation period exterior relative humidity 80 %.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.condensation.duration", D3[0], D3[1], D3[2], A, "t condensation period", 2160.0, "h", "Condensation period 90 d = 2160 h; taken from the oracle audit.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.evaporation.theta", D3[0], D3[1], D3[2], A, "theta evaporation period", 12.0, "degC", "Glaser evaporation period 12 degC on both sides.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.evaporation.phi", D3[0], D3[1], D3[2], A, "phi evaporation period", 0.7, "", "Glaser evaporation period relative humidity 70 % on both sides.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.annex-a.evaporation.duration", D3[0], D3[1], D3[2], A, "t evaporation period", 2160.0, "h", "Evaporation period 90 d = 2160 h; taken from the oracle audit.", "DIN 4108-3 Annex A"),
    row("ndp.din4108-3.delta0", D3[0], D3[1], D3[2], "Annex A", "delta_0", 2.0e-10, "kg/(m s Pa)", "Water vapour permeability of air delta_0 = 2.0e-10 kg/(m s Pa) (EN ISO 13788 value; DIN 4108-3 may state 1/delta_0 in m h Pa/kg).", "DIN 4108-3 Annex A / EN ISO 13788"),
    row("ndp.din4108-3.saturation.water.p0", D3[0], D3[1], D3[2], "Annex A", "p_0 over water", 610.5, "Pa", "Magnus p_sat = p_0 exp(a theta/(b + theta)) over water, p_0 = 610.5 Pa.", "EN ISO 13788 Eq. 4"),
    row("ndp.din4108-3.saturation.water.a", D3[0], D3[1], D3[2], "Annex A", "a over water", 17.269, "", "Magnus over water, a = 17.269 (theta >= 0 degC).", "EN ISO 13788 Eq. 4"),
    row("ndp.din4108-3.saturation.water.b", D3[0], D3[1], D3[2], "Annex A", "b over water", 237.3, "degC", "Magnus over water, b = 237.3 degC.", "EN ISO 13788 Eq. 4"),
    row("ndp.din4108-3.saturation.ice.p0", D3[0], D3[1], D3[2], "Annex A", "p_0 over ice", 610.5, "Pa", "Magnus p_sat = p_0 exp(a theta/(b + theta)) over ice, p_0 = 610.5 Pa.", "EN ISO 13788 Eq. 5"),
    row("ndp.din4108-3.saturation.ice.a", D3[0], D3[1], D3[2], "Annex A", "a over ice", 21.875, "", "Magnus over ice, a = 21.875 (theta < 0 degC).", "EN ISO 13788 Eq. 5"),
    row("ndp.din4108-3.saturation.ice.b", D3[0], D3[1], D3[2], "Annex A", "b over ice", 265.5, "degC", "Magnus over ice, b = 265.5 degC.", "EN ISO 13788 Eq. 5"),
    row("ndp.din4108-3.mass-limit.general", D3[0], D3[1], D3[2], "4.2", "m_W,T limit", 1.0, "kg/m2", "Condensate limit 1.0 kg/m2 (carried over from the previous code, not re-checked).", "DIN 4108-3 4.2"),
    row("ndp.din4108-3.mass-limit.wood-light", D3[0], D3[1], D3[2], "4.2", "m_W,T limit wood and light elements", 0.5, "kg/m2", "Condensate limit 0.5 kg/m2 for wood and light elements (carried over from the previous code, not re-checked).", "DIN 4108-3 4.2"),
    row("ndp.din4108-2.mould.r_si", D2[0], D2[1], D2[2], "6.2", "R_si mould check", 0.25, "m2K/W", "Interior surface resistance for the f_Rsi check R_si = 0.25 m2K/W.", "DIN 4108-2 6.2"),
    row("ndp.din4108-2.mould.theta_i", D2[0], D2[1], D2[2], "6.2", "theta_i mould check", 20.0, "degC", "Interior air temperature 20 degC.", "DIN 4108-2 6.2"),
    row("ndp.din4108-2.mould.theta_e", D2[0], D2[1], D2[2], "6.2", "theta_e mould check", -5.0, "degC", "Exterior air temperature -5 degC.", "DIN 4108-2 6.2"),
    row("ndp.din4108-2.mould.f_rsi_min", D2[0], D2[1], D2[2], "6.2", "f_Rsi minimum", 0.7, "", "f_Rsi >= 0.70, equal to theta_si >= 12.6 degC at 20 / -5 degC (checked by arithmetic only).", "DIN 4108-2 6.2"),
    row("ndp.en-iso-6946.combined.max_ratio", I6[0], I6[1], I6[2], "6.7.2.4", "R_upper/R_lower validity limit", 1.5, "", "Combined method is valid for R_upper/R_lower <= 1.5; wording unverified.", "EN ISO 6946 6.7.2"),
]
path = pathlib.Path(sys.argv[1])
doc = json.loads(path.read_text(encoding="utf-8"))
by_id = {e["id"]: e for e in doc["entries"]}
for r in ROWS:
    by_id.setdefault(r["id"], r)
doc["entries"] = sorted(by_id.values(), key=lambda e: e["id"])
path.write_text(json.dumps(doc, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(doc["entries"]))
