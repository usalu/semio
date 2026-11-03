#!/usr/bin/env python3
"""Replaces site.seismicZone (DeSeismicZone) by site.sApR (f64, m/s2) in the snapshot model and its five schema twins."""
import re, sys
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
def rd(p): return open(A + p, encoding="utf-8").read()
def wr(p, s): open(A + p, "w", encoding="utf-8").write(s)
def sub(s, old, new, count=1):
    assert s.count(old) >= 1, old[:80]
    return s.replace(old, new) if count == 0 else s.replace(old, new, count)

p = "🧬️schema/📸️snapshot/🦀️.rs"; s = rd(p)
s = sub(s, "(zone 2, B-R, RC DCH frame, 4 storeys)", "(S_aP,R 1.5 m/s², B-R, RC DCH frame, 4 storeys)")
s = sub(s, "seismic_zone: DeSeismicZone::Zone2,", "s_ap_r: 1.5,")
a = s.index("/// 🌩 DE seismic zone per DIN EN 1998-1/NA")
b = s.index("/// 🪨 DE-NA ground/subsoil combination")
s = s[:a] + s[b:]
s = sub(s, "    pub seismic_zone: DeSeismicZone,\n    /// 📏️ EN annex reference PGA [m/s²]. For DE annex, evaluate derives a_gR from `seismic_zone` (Table NA.1).",
 "    /// 🌋️ DE annex plateau spectral response acceleration S_aP,R [m/s²] of subsoil A-R at T_NCR = 475 a (DIN EN 1998-1/NA:2023-11). Ignored under EN annex.\n    #[dsl(unit = \"m/s2\")]\n    pub s_ap_r: f64,\n    /// 📏️ EN annex reference PGA a_gR [m/s²]. Ignored under DE annex, which uses `s_ap_r`.")
s = sub(s, "/// 🌚️ Seismic site parameters (DE zones 0–3 + DE-NA ground/subsoil, or EN A–E spectrum).", "/// 🌚️ Seismic site parameters (DE plateau acceleration S_aP,R + DE-NA ground/subsoil, or EN a_gR and A–E spectrum).")
wr(p, s)

for p in ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json"):
    s = rd(p)
    s = sub(s, '        "seismicZone",\n        "aGr",', '        "sApR",\n        "aGr",')
    old = '''        "seismicZone": {
          "$ref": "#/$defs/DeSeismicZone",
          "x-semio-ui": {
            "label": {
              "en": "Seismic zone",
              "de": "Erdbebenzone"
            }
          }
        },'''
    new = '''        "sApR": {
          "type": "number",
          "x-semio-ui": {
            "label": {
              "en": "Plateau spectral acceleration S_aP,R (subsoil A-R, 475 a)",
              "de": "Plateau-Spektralbeschleunigung S_aP,R (Untergrund A-R, 475 a)"
            },
            "widget": "stepper",
            "unit": "m/s\\u00b2",
            "precision": 2,
            "step": 0.05
          }
        },'''
    new = new.replace("\\u00b2", "²")
    s = sub(s, old, new)
    a = s.index('    "DeSeismicZone": {')
    b = s.index('    "DeGroundCombo": {')
    s = s[:a] + s[b:]
    wr(p, s)

p = "🧬️schema/📸️snapshot/🛰️.proto"; s = rd(p)
s = sub(s, "  string seismic_zone = 1;", "  double s_ap_r = 1;"); wr(p, s)
p = "🧬️schema/📸️snapshot/🔗️.graphql"; s = rd(p)
s = sub(s, "enum DeSeismicZone { zone0 zone1 zone2 zone3 }\n", ""); s = sub(s, "  seismicZone: DeSeismicZone!", "  sApR: Float!"); wr(p, s)
for p in ("🧬️schema/📸️snapshot/🟦️.ts", "🧬️schema/🟦️.ts"):
    s = rd(p)
    s = sub(s, "  seismicZone: DeSeismicZone;", "  sApR: number;")
    s = sub(s, 'export type DeSeismicZone = "zone0" | "zone1" | "zone2" | "zone3";\n\n', "")
    s = sub(s, "seismicZone: normWireRequired(normWireRef(() => parseDeSeismicZone))", "sApR: normWireRequired(normWireNumber)")
    s = sub(s, 'export const parseDeSeismicZone: NormWireReader<DeSeismicZone> = normWireLiteral("zone0", "zone1", "zone2", "zone3");\n', "")
    wr(p, s)
print("ok")
