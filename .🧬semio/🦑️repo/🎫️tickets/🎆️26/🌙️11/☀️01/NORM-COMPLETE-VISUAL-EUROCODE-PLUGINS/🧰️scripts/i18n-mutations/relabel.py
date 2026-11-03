"""Recompose mixed-language DE mutation labels from the leaf's own bilingual schema labels.

`python -I relabel.py [--write]` prints (and with --write applies) `<entity de>: <field de> ändern`
for every `LocalizedLabel::native` whose DE side leaks English, using `x-semio-ui.label` of the
target `id` property (entity) and the single `new*` property (field) of `🧬️schema/🔣️.json`.
"""
import json, pathlib, re, sys

ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
NATIVE = re.compile(r'(LocalizedLabel::native\("(?:[^"\\]|\\.)*", ")((?:[^"\\]|\\.)*)("\))')
EN = set("""kind member source name permanent height wall label backfill weld detail destabilizing stabilizing shape activity face
material direction class process length stress pressure movement friction surcharge governing layer stem thickness embedment
vertical horizontal concrete variable total pore structure outer bolts fabrication exponent deck change width depth angle
area force load ratio factor value type level limit strength diameter spacing number count slope bearing check design
group phi deg bend bends""".split())


COGNATE = set("parallel moment detail form element system position radius ring normal relative operative station isolator block first net tests gamma alpha delta sigma".split())


def words(name):
    return {w.lower() for w in re.findall(r"[A-Za-z][a-z]*|[A-Z]+(?![a-z])", re.sub(r"[-_]", " ", name))}


def leaks(de, field):
    toks = {t.lower() for t in re.findall(r"[^\W\d_]+", de) if t.isascii() and t[0].islower()}
    names = words(field)
    return bool(toks & EN) or any(len(n) > 3 and n in toks and n not in COGNATE for n in names)


def run(write):
    changed = 0
    for rs in sorted(ART.rglob("🧬️mutations/*/🦀️.rs")):
        sj = rs.parent / "🧬️schema" / "🔣️.json"
        text = rs.read_text(encoding="utf-8")
        if not sj.exists() or "MutationKind<" not in text:
            continue
        props = json.loads(sj.read_text(encoding="utf-8"))["properties"]
        news = [k for k in props if k.startswith("new")]
        m = NATIVE.search(text)
        if not m or len(news) != 1 or not leaks(m.group(2), news[0]):
            continue
        lab = lambda p: props[p]["x-semio-ui"]["label"]
        targets = [k for k, v in props.items() if (v.get("x-semio-ui") or {}).get("role") == "target"]
        prefix = m.group(2).split(": ")[0] if ": " in m.group(2) else None
        if not targets and not prefix:
            print("NO-ENTITY", rs.parent.name, file=sys.stderr)
            continue
        field, ent = lab(news[0]), {"de": prefix or lab(targets[0])["de"]}
        if field["de"] == field["en"] and field["de"].isascii() and field["de"].lower() in EN | words(news[0]):
            print("NO-DE", rs.parent.name, field, file=sys.stderr)
            continue
        de = "%s: %s ändern" % (ent["de"], field["de"])
        if de == m.group(2):
            continue
        print(m.group(2), "->", de)
        if write:
            rs.write_text(text[:m.start(2)] + de + text[m.end(2):], encoding="utf-8")
        changed += 1
    print("changed", changed)


run("--write" in sys.argv)
