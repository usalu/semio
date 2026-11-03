"""Adjudicator v2 registry rows. Idempotent. Run: python -I registry_v2.py, then the ticket codegen."""
import json, pathlib
REG = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json")
d = json.loads(REG.read_text(encoding="utf-8"))
rows = {e["id"]: e for e in d["entries"]}
SPEC = "adjudicator wave 2 v2 (oracle vs Rust disagreement)"
e = rows["ndp.en1996.gamma-m.accidental"]
e["asWritten"] = "γ_M DE 1.3 for the accidental situation and for the transient situation (Montagezustand); EN code uses the class value"
e["specRef"] = SPEC
e["evidence"] = "adjudicator 2026-11-03: repo spec en1995-en1996-target.md M-07 records the NA-DE values 'Montagezustand 1.3; außergewöhnlich 1.3' (marked [V], not verified against the print). Montagezustand is the construction (transient) situation, so DE reduces gamma_M for Transient, Accidental and Seismic alike; the seismic application is an extension (EN 1998-1 refers to the accidental value) and unverified. Rust (AnnexParams::for_document) and both oracles read this row for all three situations."
def new(i, clause, symbol, rec, note, evidence, code="EN 1994", part="1-1"):
    if i in rows: return
    rows[i] = {"id": i, "code": code, "part": part, "clause": clause, "symbol": symbol, "recommended": float(rec), "naDe": float(rec), "unit": "", "source": {"document": "EN 1994-1-1" if part == "1-1" else "EN 1994-2", "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": SPEC, "asWritten": note, "evidence": evidence}
new("ndp.en1994.1-1.tension-stiffening", "7.4.3 Eq. (7.5)", "factor of the tension-stiffening increment Δσ_s = k f_ctm / (α_st ρ_s) in crack control", 0.4, "Δσ_s = 0.4 f_ctm/(α_st ρ_s), α_st = A I / (A_a I_a), ρ_s = A_s/A_ct", "adjudicator 2026-11-03: repo spec en1994-en1999-target.md C-43 (cl. 7.4.3) prints 0.4 f_ctm/(α_st ρ_s); Rust delta_sigma_ts and the oracle agree. The EN 1994-2 6.8.5.4 fatigue increment is a separate row; its factor 0.2 is NOT in the repo text. Unverified against the print.")
new("ndp.en1994.2.fatigue-tension-stiffening", "6.8.5.4", "factor of the tension-stiffening increment Δσ_s in the reinforcement fatigue stress range of a bridge", 0.2, "Δσ_s = 0.2 f_ctm/(α_st ρ_s) as recalled in the wave 2 brief; the fatigue check still uses the 7.4.3 factor 0.4", "adjudicator 2026-11-03: the repo holds no text of EN 1994-2 6.8.5.4; the value 0.2 rests on a recollection in the brief and no check reads this row yet (the fatigue caller keeps the 7.4.3 factor, the safe larger increment is not assumed). Open for a human with the printed clause.", part="2")
d["entries"] = sorted(rows.values(), key=lambda e: e["id"])
REG.write_text(json.dumps(d, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
