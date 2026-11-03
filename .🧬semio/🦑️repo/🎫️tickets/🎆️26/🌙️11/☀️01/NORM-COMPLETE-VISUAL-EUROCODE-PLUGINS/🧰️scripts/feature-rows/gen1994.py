"""EN 1994 feature-rows fixtures derived from the committed oracle documents: shear (vpl), construction, shear connection (prd), variable deflection, column N_pl and slab shear failures. Run: .venv/bin/python -I gen1994.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
load = lambda n: json.load(open(DOCS / n / "🔣️.json"))


def tweak(name, fn):
    d = copy.deepcopy(load(name)); fn(d); return d


def act(d, key, i, value, kind="beams"):
    next(a for a in d[kind][0]["actions"] if a["id"] == i)[key] = value


def beam(d, **kw):
    d["beams"][0].update(kw)


FIXTURES = {
    "beam-shear-fail": tweak("beam-end-span", lambda d: act(d, "qAreaPa", "Q-office", 40000.0)),
    "beam-construction-fail": tweak("beam-end-span", lambda d: act(d, "qAreaPa", "Q-constr", 18000.0)),
    "beam-connection-pass": tweak("beam-partial-connection", lambda d: (act(d, "qAreaPa", "Q-office", 12000.0), d["beams"][0]["studs"].update(totalCount=80))),
    "beam-connection-fail": tweak("beam-partial-connection", lambda d: (act(d, "qAreaPa", "Q-office", 12000.0), d["beams"][0]["studs"].update(totalCount=16))),
    "beam-variable-deflection-fail": tweak("beam-deflection-fail", lambda d: act(d, "qAreaPa", "Q-office", 9500.0)),
    "column-resistance-fail": tweak("column-encased", lambda d: act(d, "nKN", "Q-col", 14000000.0, "columns")),
    "slab-shear-resistance-fail": tweak("slab-continuous", lambda d: act(d, "qAreaPa", "Q-slab", 60000.0, "slabs")),
}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        if "--write" in sys.argv:
            (DOCS / name).mkdir(exist_ok=True)
            (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        print(name, [(c["id"].split(".", 2)[2], round(c["utilization"], 3)) for c in json.loads(out.stdout)["checks"]] if out.stdout else out.stderr[-300:])
