"""✍️ Author 64: rewrites the `decisions` of volume sources from the actual solids (facts read from lists, rig and thermal regions) plus a hand-written note per asset (author-64-notes.py). Geometry is never touched.

Usage: python3 author-64-decisions.py <category/slug>... [--dry] [--force-age]
Skips a source touched in the last 20 minutes unless --force-age. Keeps decisions that start with "Minimal:" and are not listed in notes[asset]["rewrite_minimal"].
"""
import glob
import json
import os
import sys
import time

T = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
sys.path.insert(0, T)
import importlib.util

spec = importlib.util.spec_from_file_location("notes64", os.path.join(T, "author-64-notes.py"))
notes64 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(notes64)
NOTES = notes64.NOTES

KIND = {"box": ("box", "boxes", "Quader", "Quader"), "ellipsoid": ("ellipsoid", "ellipsoids", "Ellipsoid", "Ellipsoide"), "capsule": ("capsule", "capsules", "Kapsel", "Kapseln"),
        "revolve": ("revolved body", "revolved bodies", "Drehkörper", "Drehkörper"), "extrude": ("extrusion", "extrusions", "Extrusion", "Extrusionen"), "loft": ("loft", "lofts", "Loft", "Lofts")}


def short(t):
    for sep in (" and ", " und ", " / ", ", ", " (", " with ", " mit ", " on ", " auf ", " of ", " von "):
        if sep in t:
            t = t.split(sep)[0]
    return t.strip()


ADJ = {"Nahes", "Naher", "Nahe", "Nahen", "Fernes", "Ferner", "Ferne", "Fernen", "Linkes", "Linker", "Linke", "Rechtes", "Rechter", "Rechte", "Oberes", "Oberer", "Obere", "Unteres", "Unterer", "Untere", "Vorderes", "Vorderer", "Vordere", "Hinteres", "Hinterer", "Hintere", "Äußeres", "Äußerer", "Äußere", "Inneres", "Innerer", "Innere", "Kleines", "Kleiner", "Kleine", "Großes", "Großer", "Große"}


def low(t):
    return t if (len(t) > 1 and t[1].isupper()) or any(c.isdigit() for c in t) else t[0].lower() + t[1:]


def lowde(t):
    w = t.split(" ")
    if len(w) > 1 and (w[0] in ADJ or (w[1][:1].isupper() and w[0].endswith(("es", "er", "en", "em", "te", "ge", "ne", "se", "ve", "le")) and len(w[0]) > 4 and not w[0].endswith("-"))):
        return w[0][0].lower() + w[0][1:] + " " + " ".join(w[1:])
    return t


def join(items, lang, cap=9):
    items = list(items)
    more = len(items) - cap
    if more > 0:
        items = items[:cap]
        tail = f"and {more} more" if lang == "en" else f"und {more} weitere"
        return ", ".join(items) + " " + tail
    if len(items) <= 1:
        return "".join(items)
    word = "and" if lang == "en" else "und"
    return ", ".join(items[:-1]) + f" {word} " + items[-1]


def num(v, d=1):
    s = f"{v:.{d}f}"
    return s.rstrip("0").rstrip(".") if "." in s else s


def numde(v, d=1):
    return num(v, d).replace(".", ",")


def expanded(lst):
    return sum((x.get("repeat", {}).get("count", 1) if isinstance(x.get("repeat"), dict) else 1) for x in lst)


def catalog_asset(cat, slug):
    f = glob.glob(f"{CAT}/*{cat}/{slug}/*.json")
    return json.load(open(f[0]))["asset"]


def kinds(body, lang):
    counts = {}
    for x in body:
        k = x.get("kind", "ref")
        counts[k] = counts.get(k, 0) + 1
    out = []
    for k in ("loft", "revolve", "extrude", "box", "capsule", "ellipsoid"):
        if k in counts:
            c = counts[k]
            e = KIND[k]
            out.append(f"{c} {e[1] if c != 1 else e[0]}" if lang == "en" else f"{c} {e[3] if c != 1 else e[2]}")
    return ", ".join(out)


def geo(x):
    return json.dumps({k: v for k, v in x.items() if k not in ("id", "tone", "material", "role", "tags", "region", "layer", "member", "part")}, sort_keys=True)


def avg_sections(body):
    ls = [len(x["sections"]) for x in body if x.get("kind") == "loft"]
    return sum(ls) / len(ls) if ls else None


def facts(asset, vol):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    regs = {r["id"]: r for r in asset["thermal"]["regions"]}
    return bones, regs


SIDE_EN = ("near ", "far ", "left ", "right ")
SIDE_DE = ("nah", "fern", "link", "recht")


def strip_side(t, lang):
    if lang == "en":
        for p in SIDE_EN:
            if t.startswith(p):
                return t[len(p):], True
        return t, False
    w = t.split(" ", 1)
    if len(w) == 2 and w[0].lower().startswith(SIDE_DE):
        return w[1], True
    return t, False


def bone_names(bones, ids, lang, cap=9, mark=True):
    names, sided = [], False
    for i in ids:
        t = bones[i]["title"][lang] if i in bones else i
        t = low(short(t)) if lang == "en" else lowde(short(t))
        t, sd = strip_side(t, lang)
        sided = sided or sd
        if lang == "de" and sd:
            t = t[0].upper() + t[1:] if t and t[0].islower() and not t.split(" ")[0].islower() else t
        names.append(t)
    seen = []
    for n in names:
        if n not in seen:
            seen.append(n)
    out = join(seen, lang, cap)
    if sided and mark:
        out += " (each side)" if lang == "en" else " (beidseitig)"
    return out


def body_bones(body):
    out = []
    for x in body:
        b = x.get("bone")
        if b and b not in out:
            out.append(b)
    return out


def body_entry(asset, key, s, vol, fine, note):
    bones, regs = facts(asset, vol)
    body = vol["body"]
    n = expanded(body)
    bb = body_bones(body)
    plain = note.get(("plain", "all"), False)
    if plain:
        en = [f"Body at 1:{s}: {n} solids ({kinds(body, 'en')})."]
        de = [f"Körper bei 1:{s}: {n} Körper ({kinds(body, 'de')})."]
    else:
        en = [f"Body at 1:{s}: {n} solids ({kinds(body, 'en')}) standing for {len(bb)} of the {len(bones)} rig bones: {bone_names(bones, bb, 'en')}."]
        de = [f"Körper bei 1:{s}: {n} Körper ({kinds(body, 'de')}) für {len(bb)} der {len(bones)} Bones des Skeletts: {bone_names(bones, bb, 'de')}."]
    a = avg_sections(body)
    if fine is not None:
        fb = fine["body"]
        fbb = body_bones(fb)
        gone = [b for b in fbb if b not in bb]
        new = [b for b in bb if b not in fbb]
        ids_f, ids_s = {x["id"] for x in fb}, {x["id"] for x in body}
        dropped, added = len(ids_f - ids_s), len(ids_s - ids_f)
        fa = avg_sections(fb)
        fs = fine["denominator"]
        fmap = {x["id"]: x for x in fb}
        ident = sum(1 for x in body if x["id"] in fmap and geo(x) == geo(fmap[x["id"]]))
        shared = len(ids_f & ids_s)
        if plain:
            en.append(f"Against 1:{fs} ({expanded(fb)} solids): gone {dropped}, new {added}, same name {shared} (of which {ident} are geometrically identical and {shared - ident} are re-cut from this scale's own outlines).")
            de.append(f"Gegenüber 1:{fs} ({expanded(fb)} Körper): entfallen {dropped}, neu {added}, gleicher Name {shared} (davon {ident} geometrisch identisch und {shared - ident} aus den Umrissen dieses Maßstabs neu geschnitten).")
        elif gone:
            en.append(f"Against 1:{fs} ({expanded(fb)} solids) {dropped} solids are gone. Left without a solid of their own: {bone_names(bones, gone, 'en')}, absorbed by the neighbouring mass.")
            de.append(f"Gegenüber 1:{fs} ({expanded(fb)} Körper) entfallen {dropped} Körper. Ohne eigenen Körper bleiben: {bone_names(bones, gone, 'de')}, in der Nachbarmasse aufgegangen.")
        else:
            en.append(f"Every bone that has a solid at 1:{fs} keeps one, but {dropped} solids are merged or dropped; solids drawn anew at this scale: {added}.")
            de.append(f"Jede Bone mit Körper bei 1:{fs} behält einen, aber {dropped} Körper sind zusammengefasst oder entfallen; in diesem Maßstab neu gezeichnete Körper: {added}.")
        if new and not plain:
            en.append(f"New at this scale: {bone_names(bones, new, 'en')}.")
            de.append(f"Neu in diesem Maßstab: {bone_names(bones, new, 'de')}.")
        if a and fa and not plain:
            en.append(f"Lofts carry {num(a)} sections on average (1:{fs}: {num(fa)}), each re-cut from this scale's own outlines.")
            de.append(f"Lofts tragen im Mittel {numde(a)} Schnitte (1:{fs}: {numde(fa)}), jeder neu aus den Umrissen dieses Maßstabs geschnitten.")
    elif a:
        en.append(f"The finest scale: lofts carry {num(a)} sections on average.")
        de.append(f"Der feinste Maßstab: Lofts tragen im Mittel {numde(a)} Schnitte.")
    nt = note.get(("body", str(s))) or note.get(("body", "50" if fine is None else "C"))
    if nt:
        en.append(nt[0])
        de.append(nt[1])
    return {"en": " ".join(en), "de": " ".join(de)}


def area(v, lang):
    if v < 0.1:
        t = num(v * 10000, 0 if v * 10000 >= 10 else 1) + " cm2"
    else:
        t = num(v, 2) + " m2"
    return t if lang == "en" else t.replace(".", ",")


def own_phrase(th, regs, lang):
    emit, surf = [], {}
    for x in th:
        if "of" in x:
            continue
        tags = x.get("tags") or []
        if "emitter" in tags:
            n = next((t for t in tags if t != "emitter"), x["id"]).replace("-", " ")
            if n not in emit:
                emit.append(n)
        else:
            r = x.get("region")
            surf[r] = surf.get(r, 0) + 1
    parts = []
    if emit:
        parts.append(("emitters: " if lang == "en" else "Emitter: ") + join(sorted(emit), lang, 4))
    if surf:
        names = []
        for r, c in surf.items():
            t = regs[r]["title"][lang] if r in regs else str(r)
            names.append(f"{low(t) if lang == 'en' else lowde(t)} ({c})")
        parts.append(("surface solids for " if lang == "en" else "Flächenkörper für ") + join(names, lang, 4))
    return "; ".join(parts)


def layer_phrase(regs, th, lang):
    seen, out = set(), []
    for x in th:
        r = x.get("region")
        if r in regs and r not in seen and regs[r].get("layers"):
            seen.add(r)
            out.append((regs[r]["areaM2"], r))
    out.sort(reverse=True)
    groups = {}
    for _, r in out[:4]:
        t = regs[r]["layers"][-1]["title"][lang]
        groups.setdefault(t, []).append(low(regs[r]["title"][lang]) if lang == "en" else lowde(regs[r]["title"][lang]))
    ph = [f"{low(t) if lang == 'en' else t} on {join(rs, lang, 3)}" if lang == "en" else f"{lowde(t)} auf {join(rs, lang, 3)}" for t, rs in groups.items()]
    return join(ph, lang, 3)


def thermal_entry(asset, key, s, vol, fine, note):
    bones, regs = facts(asset, vol)
    th = vol["thermal"]
    byreg = {}
    for x in th:
        byreg.setdefault(x.get("region") or "-", []).append(x)
    refs = sum(1 for x in th if "of" in x)
    own = len(th) - refs
    layered = sorted({x["layer"] for x in th if x.get("layer")})
    parts_en, parts_de = [], []
    for rid, items in byreg.items():
        if rid in regs:
            t = regs[rid]["title"]
            r = regs[rid]
            parts_en.append(f"{low(t['en'])} ({len(items)}, {area(r['areaM2'], 'en')})")
            parts_de.append(f"{lowde(t['de'])} ({len(items)}, {area(r['areaM2'], 'de')})")
    en = [f"Thermal at 1:{s}: {len(th)} solids over {len(byreg)} regions: {join(parts_en, 'en', 12)} (solids per region, declared area)."]
    de = [f"Thermik bei 1:{s}: {len(th)} Körper in {len(byreg)} Regionen: {join(parts_de, 'de', 12)} (Körper je Region, deklarierte Fläche)."]
    en.append(f"Shared with body solids by reference: {refs}; with geometry of their own: {own}" + (": " + own_phrase(th, regs, "en") if own else "") + ".")
    de.append(f"Per Verweis mit Körpern des Volumens geteilt: {refs}; mit eigener Geometrie: {own}" + (": " + own_phrase(th, regs, "de") if own else "") + ".")
    lp = layer_phrase(regs, th, "en")
    if lp:
        en.append(f"Outer layers of the largest regions: {lp}.")
        de.append(f"Außenschichten der größten Regionen: {layer_phrase(regs, th, 'de')}.")
    multi = [r for r in regs.values() if len(r.get("layers") or []) > 1]
    if s == "50" and multi:
        en.append(f"Regions with several layers are nested shells, innermost first: {join([low(short(r['title']['en'])) for r in multi], 'en', 8)}.")
        de.append(f"Mehrschichtige Regionen sind geschachtelte Schalen, innen zuerst: {join([short(r['title']['de']) for r in multi], 'de', 8)}.")
    elif multi:
        en.append("Layers are merged into the outer surface of each region at this scale; their thickness is far below the smallest feature.")
        de.append("Schichten sind in diesem Maßstab zur Außenfläche jeder Region zusammengefasst; ihre Dicke liegt weit unter der kleinsten Einzelheit.")
    shared = []
    for rid, items in byreg.items():
        refs_ = [x["of"] for x in items if "of" in x]
        if refs_ and rid in regs:
            shared.append(f"{rid}: {', '.join(refs_[:3])}" + (f" +{len(refs_) - 3}" if len(refs_) > 3 else ""))
    if shared:
        en.append("Body solids reused per region: " + "; ".join(shared[:6]) + ("; ..." if len(shared) > 6 else "") + ".")
        de.append("Wiederverwendete Körper je Region: " + "; ".join(shared[:6]) + ("; ..." if len(shared) > 6 else "") + ".")
    tasm = (asset.get("thermal") or {}).get("assumptions") or []
    if tasm and isinstance(tasm[0], dict) and tasm[0].get("en") and tasm[0].get("de"):
        en.append("The asset's thermal basis: " + tasm[0]["en"].rstrip(".") + ".")
        de.append("Die thermische Grundlage des Assets: " + tasm[0]["de"].rstrip(".") + ".")
    nt = note.get(("thermal", "all"))
    if nt:
        en.append(nt[0])
        de.append(nt[1])
    return {"en": " ".join(en), "de": " ".join(de)}


def mass(kg, lang):
    t = f"{num(kg * 1000, 0)} g" if kg < 1 else f"{num(kg, 1)} kg"
    return t if lang == "en" else t.replace(".", ",")


def structure_entry(asset, key, s, vol, fine, note):
    bones, regs = facts(asset, vol)
    st = vol["structure"]
    parts = {"member": [], "joint": [], "mass": [], "support": []}
    for x in st:
        parts.setdefault(x.get("part", "member"), []).append(x)
    mem = parts["member"]
    mb = []
    for x in mem:
        if x["member"] not in mb:
            mb.append(x["member"])
    radii = []
    for x in mem:
        if x.get("kind") == "capsule":
            radii += [x["r0"], x["r1"]]
        elif x.get("kind") == "box":
            radii.append(min(x["size"]) / 2)
    rmin = min(radii) if radii else None
    en = [f"Structure at 1:{s}: {len(mem)} member {'solid' if len(mem) == 1 else 'solids'} along {bone_names(bones, mb, 'en', 4)}"]
    de = [f"Tragwerk bei 1:{s}: {len(mem)} Stabkörper entlang {bone_names(bones, mb, 'de', 4)}"]
    j, m, p = len(parts["joint"]), len(parts["mass"]), len(parts["support"])
    tail_en, tail_de = [], []
    if j:
        tail_en.append(f"{j} joint {'solid' if j == 1 else 'solids'} at the rig joints")
        tail_de.append(f"{j} Gelenkkörper an den Gelenken des Skeletts")
    elif fine is not None and any(x.get("part") == "joint" for x in fine["structure"]):
        tail_en.append("joints merged into the members")
        tail_de.append("Gelenke in die Stäbe zusammengefasst")
    if m:
        mbns = []
        for x in parts["mass"]:
            if x["member"] not in mbns:
                mbns.append(x["member"])
        tail_en.append(f"{m} mass {'solid' if m == 1 else 'solids'} at the centres of the heavy bones ({bone_names(bones, mbns, 'en', 6, False)})")
        tail_de.append(f"{m} Massenkörper in den Schwerpunkten der schweren Bones ({bone_names(bones, mbns, 'de', 6, False)})")
    if p:
        tail_en.append(f"{p} support {'solid' if p == 1 else 'solids'} at the declared supports")
        tail_de.append(f"{p} {'Auflagerkörper' if p == 1 else 'Auflagerkörper'} an den deklarierten Auflagern")
    else:
        tail_en.append("no support solids because the asset declares no supports")
        tail_de.append("keine Auflagerkörper, weil das Asset keine Auflager deklariert")
    en[0] += ", " + ", ".join(tail_en) + "."
    de[0] += ", " + ", ".join(tail_de) + "."
    if rmin is not None:
        thin = min(mem, key=lambda x: (min(x["r0"], x["r1"]) if x.get("kind") == "capsule" else min(x["size"]) / 2) if x.get("kind") in ("capsule", "box") else 9)
        thick = max(mem, key=lambda x: (max(x["r0"], x["r1"]) if x.get("kind") == "capsule" else max(x["size"]) / 2) if x.get("kind") in ("capsule", "box") else 0)
        tb = bone_names(bones, [thin["member"]], "en", 1, False)
        kb = bone_names(bones, [thick["member"]], "en", 1, False)
        en.append(f"The thinnest member radius is {num(rmin * 1000)} mm ({tb}); the thickest member: {kb}.")
        de.append(f"Der dünnste Stabradius beträgt {numde(rmin * 1000)} mm ({bone_names(bones, [thin['member']], 'de', 1, False)}); der dickste Stab: {bone_names(bones, [thick['member']], 'de', 1, False)}.")
    tot = sum((b.get("mass") or {}).get("kg", 0) for b in bones.values())
    if tot > 0:
        top, seen = [], set()
        for b in sorted(bones.values(), key=lambda b: -(b.get("mass") or {}).get("kg", 0)):
            if short(b["title"]["en"]) not in seen and len(top) < 2:
                seen.add(short(b["title"]["en"]))
                top.append(b)
        en.append(f"The rig carries {mass(tot, 'en')} in total; the heaviest bones are {join([low(short(b['title']['en'])) + ' (' + mass(b['mass']['kg'], 'en') + ')' for b in top], 'en')}.")
        de.append(f"Das Skelett trägt insgesamt {mass(tot, 'de')}; die schwersten Bones sind {join([lowde(short(b['title']['de'])) + ' (' + mass(b['mass']['kg'], 'de') + ')' for b in top], 'de')}.")
    sm = (asset.get("summary") or {})
    if isinstance(sm, dict) and sm.get("en") and sm.get("de"):
        en.append("Subject: " + sm["en"].split(". ")[0].rstrip(".") + ".")
        de.append("Gegenstand: " + sm["de"].split(". ")[0].rstrip(".") + ".")
    asm = (asset.get("structure") or {}).get("assumptions") or []
    if asm:
        en.append("The asset's structural basis: " + asm[0]["en"].rstrip(".") + ".")
        de.append("Die Tragwerksgrundlage des Assets: " + asm[0]["de"].rstrip(".") + ".")
    nt = note.get(("structure", "all"))
    if nt:
        en.append(nt[0])
        de.append(nt[1])
    return {"en": " ".join(en), "de": " ".join(de)}


def rewrite(key, dry, force):
    cat, slug = key.split("/")
    path = f"{T}/volumes/{key}.json"
    age = (time.time() - os.path.getmtime(path)) / 60
    if age < 20 and not force:
        print(f"SKIP {key}: touched {age:.1f} min ago")
        return False
    raw = open(path).read()
    src = json.loads(raw)
    asset = catalog_asset(cat, slug)
    note = NOTES[key]
    fine = None
    for s in ("50", "100", "200", "500"):
        vol = src["volumes"][s]
        if vol.get("status") != "authored":
            continue
        keep = [d for d in vol["decisions"] if (str(d.get("en", "")).startswith("Minimal:") and key not in notes64.REWRITE_MINIMAL) or str(d.get("en", "")).startswith("[")]
        fn = {k: v for k, v in note.items() if isinstance(k, tuple)}
        sn = dict(fn)
        entries = [body_entry(asset, key, s, vol, fine, sn), thermal_entry(asset, key, s, vol, fine, sn), structure_entry(asset, key, s, vol, fine, sn)]
        extra = note.get(("scale", s))
        if extra:
            entries.insert(1, {"en": extra[0], "de": extra[1]})
        vol["decisions"] = entries + keep
        fine = vol
    out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
    if dry:
        for s in ("50", "100", "200", "500"):
            v = src["volumes"][s]
            if v.get("status") == "authored":
                print("==", s)
                for d in v["decisions"]:
                    print("EN:", d["en"])
                    print("DE:", d["de"])
        return True
    if os.path.getmtime(path) != os.path.getmtime(path) or open(path).read() != raw:
        print(f"ABORT {key}: changed while rewriting")
        return False
    open(path, "w").write(out)
    print(f"WROTE {key}")
    return True


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    for k in args:
        rewrite(k, "--dry" in sys.argv, "--force-age" in sys.argv)
