import json, os, re, sys

ROOT = r"C:\git\semio"
CAT = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
EN_STOP = re.compile(r"\b(the|and|with|of|from|that|this|which|are|is|by|its|their|into)\b")
TERMS = ["Plattenwärmeübertrager", "Sorptionsrad", "Verdampfer", "Verflüssiger", "Scrollverdichter", "Expansionsventil", "Thermostatventil", "Umwälzpumpe", "Warmwasserspeicher", "Lüftungsgerät", "Pendellüfter", "Wärmerückgewinnung"]
BAD = re.compile(r"(?<![A-Za-zÄÖÜäöüß])(Fuer|fuer|Ueber|ueber|koennen|Koennen|Waerme|waerme|Geraet|geraet|Verfuegbar|Aender|Oel|oel|Aussen|aussen|Gruen|Strasse|Fussboden|Fuss|Zuerst|Muell)")

assets = sys.argv[1:]
for path in assets:
    with open(path, encoding="utf-8") as fh:
        a = json.load(fh)["asset"]
    rel = a["id"]
    print(f"## {rel}  title: en={a['title']['en']!r} de={a['title']['de']!r}")
    for kind in ("poses", "clips"):
        for x in a.get(kind, []):
            t = x.get("title", {})
            print(f"  {kind}.{x['id']}: en={t.get('en')!r} | de={t.get('de')!r}")
    strings = []
    def collect(node, p):
        if isinstance(node, dict):
            if "en" in node and "de" in node and isinstance(node["de"], str):
                strings.append((p, node["en"], node["de"]))
                return
            for k, v in node.items():
                collect(v, f"{p}.{k}")
        elif isinstance(node, list):
            for i, v in enumerate(node):
                collect(v, f"{p}[{i}]")
    collect(a, "asset")
    for p, en, de in strings:
        if EN_STOP.search(de) and not EN_STOP.search(en) is None:
            pass
        hits = EN_STOP.findall(de)
        if len(hits) >= 2:
            print(f"  EN-LEAK? {p}: {de[:140]!r}")
        m = BAD.search(de)
        if m:
            print(f"  ASCII-UMLAUT? {p}: {m.group(0)} in {de[:120]!r}")
        if de.strip() == en.strip() and len(en) > 12:
            print(f"  SAME? {p}: {de[:100]!r}")
