"""🔩 One-shot fix of the converted en1994 stud rows: the declared count must stay inside the designed span (the old checks counted every declared stud), so a layout whose stations (count / studs per rib) times the declared spacing overflows the span is compressed to the pitch span / stations, centred on the same middle; a layout that fits keeps its spacing, and a spacing above the 6.6.5.5 maximum stays (the fixture exists to fail the spacing rule). Idempotent. Run: python3 -I fit_pitch.py [--apply]"""
import json, os, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 9))
ART = os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994")


def stations(rows):
    out, first = [], None
    for row in sorted(rows, key=lambda r: (r["xM"], r["id"])):
        if first is None or row["xM"] - first > 1e-3:
            out.append([])
            first = row["xM"]
        out[-1].append(row)
    return out


def fit(doc):
    changed = 0
    for beam in doc["beams"]:
        rows = [s for s in doc["studs"] if s["beamId"] == beam["id"]]
        groups = stations(rows)
        if len(groups) < 2:
            continue
        pitch = groups[1][0]["xM"] - groups[0][0]["xM"]
        span = beam["spanM"]
        s_max = min(6 * max(beam["slabThicknessM"] - beam["sheeting"]["heightM"], 0.04), 0.8)
        if len(groups) * pitch <= span + 1e-9 or pitch > s_max:
            continue
        centre = (groups[0][0]["xM"] + groups[-1][0]["xM"]) / 2
        fitted = span / len(groups)
        for k, group in enumerate(groups):
            for row in group:
                row["xM"] = round(centre + (k - (len(groups) - 1) / 2) * fitted, 9)
        changed += 1
    return changed


def walk(node, hits):
    if isinstance(node, dict):
        if isinstance(node.get("beams"), list) and isinstance(node.get("studs"), list) and node["studs"] and isinstance(node["studs"][0], dict) and "beamId" in node["studs"][0]:
            hits.append(node)
        for value in node.values():
            walk(value, hits)
    elif isinstance(node, list):
        for value in node:
            walk(value, hits)


if __name__ == "__main__":
    files = beams = 0
    for dp, _, fs in os.walk(ART):
        for name in fs:
            if not name.endswith(".json") or "/target/" in dp:
                continue
            path = os.path.join(dp, name)
            raw = open(path, encoding="utf8").read()
            if '"studs"' not in raw or '"beamId"' not in raw:
                continue
            try:
                doc = json.loads(raw)
            except json.JSONDecodeError:
                continue
            hits = []
            walk(doc, hits)
            n = sum(fit(h) for h in hits)
            if not n:
                continue
            files += 1
            beams += n
            if "--apply" in sys.argv:
                ind = 2 if raw.startswith('{\n  ') else (1 if raw.startswith('{\n ') else None)
                out = json.dumps(doc, ensure_ascii=False, indent=ind)
                if ind is None:
                    out = json.dumps(doc, ensure_ascii=False, separators=(",", ":"))
                open(path, "w", encoding="utf8").write(out + ("\n" if raw.endswith("\n") else ""))
    print(files, "files,", beams, "beams compressed")
