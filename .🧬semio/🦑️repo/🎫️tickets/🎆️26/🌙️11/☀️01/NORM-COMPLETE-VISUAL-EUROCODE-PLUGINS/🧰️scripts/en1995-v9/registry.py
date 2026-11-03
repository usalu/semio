"""EN 1995 v9: NDP registry rows. Idempotent. Run: python -I registry.py, then the ticket codegen."""
import json, pathlib
REG = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json")
d = json.loads(REG.read_text(encoding="utf-8"))
rows = {e["id"]: e for e in d["entries"]}
def new(i, clause, symbol, rec, unit, evidence, part="1-1"):
    if i in rows: return
    rows[i] = {"id": i, "code": "EN 1995", "part": part, "clause": clause, "symbol": symbol, "recommended": float(rec), "naDe": float(rec), "unit": unit, "source": {"document": "DIN EN 1995-1-1/NA", "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": "en1995 v9", "asWritten": symbol, "evidence": evidence}
new("ndp.en1995.kh.density-limit", "EN 1995-1-1 3.2(3)", "characteristic density up to which the size factor k_h of solid timber applies (rho_k <= 700 kg/m3)", 700, "kg/m3",
    "adjudicator 2026-11-03 read the clause in BS EN 1995-1-1:2004+A1:2008 (3.2 Expression 3.1: k_h applies to solid timber with rho_k <= 700 kg/m3, page 101/102); no library holds it, so the row stays unverified; DE column repeats the value, NA text not available.")
CLT = "value as used by the artifact before wave 2 v9 (hard-coded default of T-97); CLT is not covered by EN 1995-1-1:2004, the value rests on product approvals (EN 16351 / ETA) and has no source in the repository; DE column repeats the value."
new("ndp.en1995.clt.rolling-shear-strength", "EN 16351 / ETA (product value)", "characteristic rolling shear strength f_r,k of the cross layers of CLT", 1.0, "N/mm2", CLT)
new("ndp.en1995.clt.rolling-shear-modulus", "EN 16351 / ETA (product value)", "rolling shear modulus G_R of the cross layers of CLT", 50.0, "N/mm2", CLT)
new("ndp.en1995.clt.gross-shear-strength", "EN 16351 / ETA (product value)", "characteristic gross shear strength f_v,CLT,k of CLT", 5.0, "N/mm2", CLT)
DECK = "value as used by the artifact before wave 2 v9 (hard-coded in T-122 and in the oracle); the clause of EN 1995-2 for stress-laminated decks is not in the repository (the catalogue cites Annex A, itself unverified), so neither value nor locator has a source; DE column repeats the value."
new("ndp.en1995.deck.min-prestress", "EN 1995-2 stress-laminated decks (Annex A cited by the catalogue)", "minimum residual prestress of a stress-laminated deck after all losses", 0.35, "N/mm2", DECK, "2")
new("ndp.en1995.deck.default-loss", "EN 1995-2 stress-laminated decks (Annex A cited by the catalogue)", "total prestress loss assumed when no staged losses are given", 0.55, "", DECK, "2")
new("ndp.en1995.deck.friction-coefficient", "EN 1995-2 stress-laminated decks (Annex A cited by the catalogue)", "friction coefficient between the laminations against slip", 0.3, "", DECK, "2")
new("ndp.en1995.deck.k-c90", "EN 1995-2 stress-laminated decks (Annex A cited by the catalogue)", "factor k_c,90 on the compressive strength perpendicular to the grain under the anchor plate", 1.5, "", DECK, "2")
d["entries"] = sorted(rows.values(), key=lambda e: e["id"])
REG.write_text(json.dumps(d, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
