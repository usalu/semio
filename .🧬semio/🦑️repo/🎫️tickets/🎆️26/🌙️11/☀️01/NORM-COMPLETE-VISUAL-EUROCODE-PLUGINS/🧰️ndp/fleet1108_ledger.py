"""📒 Writes the reviewed conflict ledger fixture: expected EN and NA-DE values per resolved cross-check conflict.

Run: python3 -I fleet1108_ledger.py FIXTURE_DIR. The values are typed here on purpose: the test compares them with the registry.
basis `library` = EN value independently encoded by a third-party library (row verified or EN verified per annex);
basis `public` = only public secondary sources; basis `open` = nothing found. Only `library` may carry a verified flag.
"""
import json, pathlib, sys

L, P, O = "library", "public", "open"
R = [
    ("E92-02", "ndp.en1992.m-01.alpha-cc-accidental", 1.0, 0.85, O),
    ("E92-02", "ndp.en1992.m-01.alpha-cc-persistent", 1.0, 0.85, L),
    ("E92-02", "ndp.en1992.m-01.alpha-ct", 1.0, 0.85, L),
    ("E92-05", "ndp.en1992.m-01.gamma-c-accidental", 1.2, 1.3, P),
    ("E92-13", "ndp.en1992.d-04.class-fck.xc1", 30.0, 30.0, L),
    ("E92-13", "ndp.en1992.d-04.class-fck.xc2-xc3", 35.0, None, L),
    ("E92-13", "ndp.en1992.d-04.class-fck.xc4-xd1-xd2-xs1", 40.0, None, L),
    ("E92-13", "ndp.en1992.d-04.class-fck.xd3-xs2-xs3", 45.0, None, L),
    ("E92-24", "ndp.en1992.k1-shear", 0.15, 0.12, L),
    ("E92-24", "ndp.en1992.crd-c-shear", 0.18, 0.15, L),
    ("E92-25", "ndp.en1992.vmin-shear", 0.035, 0.0525, L),
    ("E92-25", "ndp.en1992.vmin-shear-d800", 0.035, 0.0375, P),
    ("E92-26", "ndp.en1992.cot-theta-min", 1.0, 1.0, L),
    ("E92-26", "ndp.en1992.cot-theta-max", 2.5, 3.0, L),
    ("E92-39", "ndp.en1992.crd-c-punch", 0.18, 0.18, L),
    ("E92-39", "ndp.en1992.k1-punch", 0.1, 0.1, L),
    ("E92-43", "ndp.en1992.punch-cs-vrdc-factor", 0.75, 0.75, L),
    ("E92-43", "ndp.en1992.punch-cs-asw-factor", 1.5, 1.5, L),
    ("E92-43", "ndp.en1992.punch-fywd-ef-base", 250.0, 250.0, L),
    ("E92-43", "ndp.en1992.punch-fywd-ef-slope", 0.25, 0.25, L),
    ("E92-47", "ndp.en1992.fbd-eta", 2.25, 2.25, L),
    ("E92-59", "ndp.en1992.fatigue-straight", 162.5, 162.5, O),
    ("E92-80", "ndp.en1992.cone-k-cast-cracked", 8.9, 8.9, O),
    ("E92-80", "ndp.en1992.cone-k-cast-uncracked", 12.7, 12.7, O),
    ("E92-80", "ndp.en1992.cone-k-post-cracked", 7.7, 7.7, O),
    ("E92-80", "ndp.en1992.cone-k-post-uncracked", 11.0, 11.0, O),
    ("E92-82", "ndp.en1992.nu1-en-factor", 0.6, None, L),
    ("E92-82", "ndp.en1992.nu1-en-divisor", 250.0, None, L),
    ("E92-82", "ndp.en1992.nu1-factor", 1.0, 0.75, P),
    ("E92-82", "ndp.en1992.nu2-offset", None, 1.1, P),
    ("E92-82", "ndp.en1992.nu2-fck-divisor", None, 500.0, P),
    ("E92-PR", "ndp.en1992.p-r-sup.pre-or-unbonded", 1.05, None, L),
    ("E92-PR", "ndp.en1992.p-r-sup.post-bonded", 1.1, None, L),
    ("E92-PR", "ndp.en1992.p-r-inf.pre-or-unbonded", 0.95, None, L),
    ("E92-PR", "ndp.en1992.p-r-inf.post-bonded", 0.9, None, L),
    ("E95-01", "ndp.en1995.strength.c24.ft0k", 14.5, 14.5, L),
    ("E95-02", "ndp.en1995.strength.c24.ft90k", 0.4, 0.4, L),
    ("E95-04", "ndp.en1995.strength.c16.ft90k", 0.4, 0.4, L),
    ("E95-05", "ndp.en1995.strength.c30.ft90k", 0.4, 0.4, L),
    ("E95-05", "ndp.en1995.strength.c30.fc0k", 24.0, 24.0, L),
    ("E95-05", "ndp.en1995.strength.c30.ft0k", 19.0, 19.0, L),
    ("E95-06", "ndp.en1995.strength.gl24h.ft0k", 19.2, 19.2, P),
    ("E95-06", "ndp.en1995.strength.gl24h.gmean", 650.0, 650.0, P),
    ("E95-07", "ndp.en1995.strength.gl28h.ft0k", 22.3, 22.3, P),
    ("E95-07", "ndp.en1995.strength.gl28h.gmean", 650.0, 650.0, P),
    ("E95-08", "ndp.en1995.strength.gl32h.ft0k", 25.6, 25.6, P),
    ("E95-08", "ndp.en1995.strength.gl32h.gmean", 650.0, 650.0, P),
    ("E95-14", "ndp.en1995.gamma-m.glulam", 1.25, 1.3, P),
    ("E95-15", "ndp.en1995.gamma-m.lvl", 1.2, 1.3, P),
    ("E95-16", "ndp.en1995.gamma-m.plywood", 1.2, 1.3, P),
    ("E95-21", "ndp.en1995.gamma-m.clt", 1.25, 1.3, O),
    ("E95-44", "ndp.en1995.floor.f1-limit", 8.0, 6.0, P),
    ("E95-45", "ndp.en1995.floor.a-limit", 1.5, 0.5, O),
    ("E96-06", "ndp.en1996.k.gp.clay.g3", 0.35, 0.45, P),
    ("E96-07", "ndp.en1996.k.gp.clay.g4", 0.35, 0.4, P),
    ("E96-08", "ndp.en1996.k.tl.clay.g1", 0.75, 0.7, P),
    ("E96-09", "ndp.en1996.k.tl.clay.g2", 0.7, 0.7, P),
    ("E96-15", "ndp.en1996.fvk0.gp.clay.ge10", 0.3, 0.22, P),
    ("E96-16", "ndp.en1996.fvk0.gp.clay.ge5", 0.2, 0.18, P),
    ("E96-18", "ndp.en1996.fvk0.tl.clay.all", 0.3, 0.22, P),
    ("E96-36", "ndp.en1996.gamma-m.c3", 2.0, 1.7, P),
]
out = pathlib.Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
rows = [dict(conflict=c, id=i, en=en, naDe=na, basis=b) for c, i, en, na, b in R]
(out / "🔣️.json").write_text(json.dumps(dict(
    description="Reviewed ledger of the resolved cross-check conflicts (ticket audits ndp-crosscheck-*.md): expected EN and NA-DE registry values and the strongest evidence basis. library = EN value encoded by a third-party library, public = public secondary sources only, open = nothing found. A row with another basis than library must not carry a verified flag. Written by 🧰️ndp/fleet1108_ledger.py.",
    rows=rows), ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
