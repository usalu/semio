import json, re, sys, pathlib
T = pathlib.Path(__file__).resolve().parent.parent
d = {e["id"]: e for e in json.load(open(sys.argv[1]))["entries"]}
for e in d.values(): e.setdefault("evidence", "")
def add(i, code, part, clause, sym, rec, de, unit, doc, loc, ref, written, ev):
    d[i] = dict(id=i, code=code, part=part, clause=clause, symbol=sym, recommended=rec, naDe=de, unit=unit, source=dict(document=doc, edition="to be recorded at verification", locator=loc), status="unverified", specRef=ref, asWritten=written, evidence=ev)
SRC = "https://www.studiomadera.it/news/660-legno-conifera; https://www.fh-bielefeld.de Leporello 3 (2020-05-25); https://www.hasslacher.com/data/_dateimanager/broschuere/HNT-Brettschichtholz-EN.pdf"
del d["ndp.en1995.t-01.1"]
cls = {"c16": ("C16", "EN 338:2016", [10, 0.4, 17, 2.2, 500], [8.5, 0.4, None, 2.2, 500], "single-source: ft0 8.5 (MAD)"),
 "c24": ("C24", "EN 338:2016", [14.5, 0.4, 21, 2.5, 690], [14.5, 0.4, 21, 2.5, 690], "verified-public: spec correct (L3, MAD)"),
 "c30": ("C30", "EN 338:2016", [18, 0.4, 23, 2.7, 750], [19.0, 0.4, 24, 2.7, 750], "verified-public: ft0 19.0, fc0 24 (L3, MAD)"),
 "gl24h": ("GL24h", "EN 14080:2013", [16.5, 0.4, 24, 2.7, 650], [19.2, 0.5, 24, 2.5, 650], "verified-public: EN 14080 ft0g 19.2, ft90 0.5, fc90 2.5, G 650; spec values are EN 1194 era (HASS, L3, CIV24)"),
 "gl28h": ("GL28h", "EN 14080:2013", [19.5, 0.45, 26.5, 3.0, 650], [22.3, 0.5, 28, 2.5, 650], "verified-public: EN 14080 22.3, 0.5, 28, 2.5, 650; spec is EN 1194 era (HASS, L3)"),
 "gl32h": ("GL32h", "EN 14080:2013 (2023 differs)", [22.5, 0.5, 29, 3.3, 650], [25.6, 0.5, 32, 2.5, 650], "verified-public: EN 14080 25.6, 0.5, 32, 2.5, 650 (HASS, CIV32)")}
props = [("ft0k", "f_t,0,k", 0), ("ft90k", "f_t,90,k", 1), ("fc0k", "f_c,0,k", 2), ("fc90k", "f_c,90,k", 3), ("gmean", "G_mean", 4)]
for k, (name, doc, spec, pub, note) in cls.items():
    for pid, sym, j in props:
        pv = pub[j]
        ev = f"{note}. Public value for {sym}: {pv if pv is not None else 'not found'}. Sources: {SRC}. Non-authoritative."
        add(f"ndp.en1995.t-01.{k}.{pid}", "EN 1995", "1-1", "Table 3.1 / EN 338 / EN 14080", f"{sym} {name}", spec[j], spec[j], "N/mm2", f"DIN EN 1995-1-1/NA ({doc})", f"{doc} strength table, {name}", "en1995-en1996-target.md:28", f"{name} {sym} {spec[j]} (spec value used)", ev)
del d["ndp.en1992.m-01.1"]
E92 = "https://www.frilo.eu/wp-content/uploads/DE/Manuals/nachweise_am_stahlbeton-querschnitt.pdf; https://lbv.brandenburg.de/download/Bautechnik/bautechnik_Tipp_15-06.pdf; https://www.dlubal.com/de/downloads-und-infos/dokumente/online-handbuecher/rf-concrete-members/004608. Non-authoritative."
for pid, sym, cl, rec, de, ev in [("alpha-cc-persistent", "α_cc", "3.1.6(1)", 1.0, 0.85, "verified-public: DE 0.85 default"),
 ("alpha-cc-accidental", "α_cc accidental", "3.1.6(1)", 1.0, 1.0, "conflict: public sources give 0.85 for DE, 1.0 only in justified cases; repo uses DE 0.85, EN 1.0; spec 1.0"),
 ("alpha-ct", "α_ct", "3.1.6(2)", 1.0, 0.85, "verified-public: DE 0.85"),
 ("gamma-c-persistent", "γ_c", "2.4.2.4", 1.5, 1.5, "verified-public: 1.5"),
 ("gamma-c-accidental", "γ_c accidental", "2.4.2.4", 1.2, 1.2, "verified-public: DE accidental 1.3 (Table NA.2.1DE), EN 1.2; spec line 28 (1.2) is the EN value")]:
    add(f"ndp.en1992.m-01.{pid}", "EN 1992", "1-1", cl, sym, rec, de, "", "DIN EN 1992-1-1/NA", f"Table 2.1DE / {cl}", "en1992-target.md:28", f"{sym} EC {rec} / DE {de}", f"{ev}. Sources: {E92}")
for e in d.values():
    c = e["code"]; s = e["source"]
    if c == "EN 1997":
        s["document"] = "DIN 1054:2021-04"; s["edition"] = "2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check)"
    elif c == "EN 1998":
        s["document"] = "DIN EN 1998-1/NA:2023-11"; s["edition"] = "2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete)"
    elif c == "EN 1999":
        s["document"] = "DIN EN 1999-1-1/NA"; s["edition"] = "2013-05 and 2017-05 are marked withdrawn; current edition to be checked"
for line in (T / "🔍️audits/ndp-public-verification.md").read_text().splitlines():
    if not line.startswith("| `ndp."): continue
    c = [x.strip() for x in line.strip("|").split("|")]
    if len(c) < 7: continue
    for i in re.findall(r"`(ndp\.[^`*]+)`", c[0]):
        if i in d: d[i]["evidence"] = f"{c[5]}: {c[3]} (sources: {c[4]}). Non-authoritative, see 🔍️audits/ndp-public-verification.md."[:700]
import subprocess
ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
cited = set()
for f in ART.rglob("🦀️.rs"):
    cited |= set(re.findall(r'"(ndp\.en199\d\.[a-z0-9.\-]+)"', f.read_text(errors="ignore")))
SPECFILE = {"en1990": "en1990-en1991-target.md", "en1991": "en1990-en1991-target.md", "en1992": "en1992-target.md", "en1993": "en1993-target.md", "en1994": "en1994-en1999-target.md", "en1999": "en1994-en1999-target.md", "en1995": "en1995-en1996-target.md", "en1996": "en1995-en1996-target.md", "en1997": "en1997-en1998-target.md", "en1998": "en1997-en1998-target.md"}
for i in sorted(cited - set(d)):
    fam, item = i.split(".")[1], i.split(".")[2]
    fn = SPECFILE.get(fam, "?"); ref, written, sym, cl = "cited by artifact", "cited by an artifact check; add the exact value from the spec item", "?", "?"
    if fn != "?":
        for n, line in enumerate((T / "📐️specs" / fn).read_text().splitlines(), 1):
            if re.match(r"\*\*" + re.escape(item.upper()) + r"\b", line.strip()):
                ref = f"{fn}:{n}"; written = re.sub(r"\s+", " ", line)[:240]
                m = re.search(r"`([^`]{1,24})`", line); sym = m.group(1) if m else "?"
                m = re.search(r"Table [A-Z]?\d[\w.]*|(?<![\d.])[1-9]\d?(?:\.\d{1,2}){1,4}", line); cl = m.group(0) if m else "?"
                break
    add(i, "EN " + fam[2:], "", cl, sym, None, None, "", "DIN EN " + fam[2:] + "/NA", cl, ref, written, "")
print("cited", len(cited))
out = sorted(d.values(), key=lambda e: e["id"])
json.dump(dict(entries=out), open(sys.argv[2], "w"), ensure_ascii=False, indent=1)
print(len(out), sum(bool(e["evidence"]) for e in out))
