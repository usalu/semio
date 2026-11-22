"""🌱 Species re-authoring helpers shared by the PW-09 plants: ids, tones, texts, loads, sections and the manifest row.

@see ./morph.py
@see ../../📋️brief.md
"""
import math
import re

from morph import each_shape, each_solid, rename, replace_words, texts


def rename_ids(doc, mapping):
    """🏷 Renames slugs in the whole document (ids, references, track targets, dict keys)."""
    doc["asset"] = rename(doc["asset"], mapping)


def retone(asset, rule):
    """🎨 Sets `tone` of every shape and solid through `rule(item, tone) -> tone`."""
    for _, s in each_shape(asset):
        if "tone" in s:
            s["tone"] = round(rule(s, s["tone"]), 3)
    for _, s in each_solid(asset):
        if "tone" in s:
            s["tone"] = round(rule(s, s["tone"]), 3)


def set_texts(asset, overrides):
    """📝 Replaces texts by path (as `morph.texts` names them); a path that does not exist is an error."""
    found = dict(texts(asset))
    for path, (en, de) in overrides.items():
        if path not in found:
            raise KeyError(path)
        found[path]["en"], found[path]["de"] = en, de


def leftovers(asset, words):
    """🔎 Texts that still name the template species."""
    out = []
    for path, t in texts(asset):
        for w in words:
            if re.search(w, t["en"]) or re.search(w, t["de"]):
                out.append((path, w, t["en"][:120]))
    return out


def section_titles(asset):
    """⭕ Rewrites the diameter in every section title from the section area."""
    for b in asset["rig"]["bones"]:
        if "section" not in b:
            continue
        d = math.sqrt(4 * b["section"]["areaM2"] / math.pi)
        t = b["section"].get("title")
        if t is None:
            continue
        t["en"] = re.sub(r"d = [0-9.,]+ (m|cm)", f"d = {d:.2f} m", t["en"])
        t["de"] = re.sub(r"d = [0-9.,]+ (m|cm)", f"d = {d:.2f} m".replace(".", ","), t["de"])


def scale_loads(asset, kind, factor, poses=None):
    """⚖️ Multiplies the forces of all loads of one kind (per pose id or all poses)."""
    for p in asset["poses"]:
        f = factor if not isinstance(factor, dict) else factor.get(p["id"])
        if f is None or (poses is not None and p["id"] not in poses):
            continue
        for ld in p.get("loads", []):
            if ld["kind"] != kind:
                continue
            for key in ("fxN", "fyN", "fzN"):
                if key in ld:
                    ld[key] = round(ld[key] * f, 2)
            if "distributed" in ld:
                for key in ("wxNPerM", "wyNPerM", "wzNPerM"):
                    if key in ld["distributed"]:
                        ld["distributed"][key] = round(ld["distributed"][key] * f, 3)


def scale_pose_angles(asset, pose_id, factor):
    """📐 Scales the bone offsets of one pose (a droop authored proportional to its load)."""
    for p in asset["poses"]:
        if p["id"] == pose_id:
            for off in p["bones"].values():
                for key in ("angleDeg", "abductionDeg", "twistDeg"):
                    if key in off:
                        off[key] = round(off[key] * factor, 3)


def manifest_row(asset, distinction, manifest_template):
    """📥 One manifest entry in the shape of the existing rows, derived from the asset."""
    row = {}
    for key in manifest_template:
        if key == "id":
            row[key] = asset["id"]
        elif key == "category":
            row[key] = asset["category"]
        elif key == "title":
            row[key] = asset["title"]
        elif key == "heightM":
            row[key] = asset["dimensions"]["heightM"]
        elif key == "widthM":
            row[key] = asset["dimensions"]["widthM"]
        elif key == "powerW":
            row[key] = 0
        elif key == "poses":
            row[key] = [p["id"] for p in asset["poses"]]
        elif key == "clips":
            row[key] = [c["id"] for c in asset["clips"]]
        elif key == "representations":
            row[key] = {k: v["status"] for k, v in asset["representations"].items()}
        elif key == "symbols":
            row[key] = [s["id"] for s in asset["symbols"]]
        elif key == "exports":
            row[key] = asset["exports"]
        elif key == "distinction":
            row[key] = distinction
        elif key == "completion":
            row[key] = asset["completion"]
        else:
            row[key] = manifest_template[key]
    return row


__all__ = ["rename_ids", "retone", "set_texts", "leftovers", "section_titles", "scale_loads", "scale_pose_angles", "manifest_row", "replace_words"]


EN_NUM = "zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen".split()
EN_TENS = "_ _ twenty thirty forty fifty sixty seventy eighty ninety".split()
DE_NUM = "null eins zwei drei vier fünf sechs sieben acht neun zehn elf zwölf dreizehn vierzehn fünfzehn sechzehn siebzehn achtzehn neunzehn".split()
DE_TENS = "_ _ zwanzig dreißig vierzig fünfzig sechzig siebzig achtzig neunzig".split()


def en_words(n):
    """🔢 English number words below one hundred."""
    if n < 20:
        return EN_NUM[n]
    return EN_TENS[n // 10] + ("" if n % 10 == 0 else "-" + EN_NUM[n % 10])


def de_words(n):
    """🔢 German number words below one hundred (`ein` in compounds, `eins` alone)."""
    if n < 20:
        return DE_NUM[n]
    unit = n % 10
    return DE_TENS[n // 10] if unit == 0 else ("ein" if unit == 1 else DE_NUM[unit]) + "und" + DE_TENS[n // 10]


def count(asset, scale, view, pred):
    """🔢 Number of shapes of one view of one authored scale that `pred` selects."""
    return sum(1 for s in asset["representations"][scale]["geometry"][view] if pred(s))


def count_solids(asset, scale, lst, pred):
    """🔢 Number of solids of one list of one authored volume that `pred` selects."""
    return sum(1 for s in asset["volumes"][scale][lst] if "kind" in s and pred(s))
