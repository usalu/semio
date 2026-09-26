#!/usr/bin/env python3
"""🎯 Retarget tickets from old goals onto the running goal tree.

Dry-run by default. Pass --apply to rewrite the goal field in place.
"""
import json
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(".🧬semio/🦑️repo")
GOALS = ROOT / "🎯️goals"
TICKETS = ROOT / "🎫️tickets"
PREFIX = "🎯"
APPLY = "--apply" in sys.argv

def flat(value: str) -> str:
    out = []
    for ch in value:
        if (ch.isascii() and ch.isalnum()) or ord(ch) > 0x7F:
            out.append(ch.lower() if ch.isascii() else ch)
    return "".join(out)

def compose(path: str) -> str:
    return "".join(PREFIX + flat(part) for part in path.split("/") if part)

NEW = {}
for path in GOALS.rglob("🎯️goal.json"):
    rel = path.parent.relative_to(GOALS).as_posix()
    if rel == "RUNNING-FRAMEWORK" or rel.startswith("RUNNING-FRAMEWORK/"):
        NEW[rel] = compose(rel)

BASE = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-OS/RUNNING-S/RUNNING-PLUGINS"

def plugin(name: str) -> str:
    return f"{BASE}/RUNNING-{name}"

def artifact(plugin_name: str, artifact_name: str) -> str:
    return f"{plugin(plugin_name)}/{artifact_name}"

S = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-OS/RUNNING-S"
REPO = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-REPO"
OS = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-OS"
HUB = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-SERVER/RUNNING-HUB"
SERVER = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-SERVER"
PRINT = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-PRINT"
PRESENTATION = "RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-PRESENTATION"
FRAMEWORK = "RUNNING-FRAMEWORK"

PLUGINS = {
    "animate": plugin("ANIMATE"),
    "architect": plugin("ARCHITECT"),
    "block": plugin("BLOCK"),
    "cad": plugin("CAD"),
    "dag": plugin("DAG"),
    "demonstrator": plugin("DEMONSTRATOR"),
    "draw": plugin("DRAW"),
    "energy": plugin("ENERGY"),
    "fem": plugin("FEM"),
    "flow": plugin("FLOW"),
    "forms": plugin("FORMS"),
    "gis": plugin("GIS"),
    "imperative": plugin("IMPERATIVE"),
    "layout": plugin("LAYOUT"),
    "lowpoly": plugin("LOWPOLY"),
    "mathematical": plugin("MATHEMATICAL"),
    "norm": plugin("NORM"),
    "note": plugin("NOTE"),
    "playbook": plugin("PLAYBOOK"),
    "procedural": plugin("PROCEDURAL"),
    "process": plugin("PROCESS"),
    "puzzle": plugin("PUZZLE"),
    "raster": plugin("RASTER"),
    "reasoning": plugin("REASONING"),
    "remodel": plugin("REMODEL"),
    "sequence": plugin("SEQUENCE"),
    "shooting": plugin("SHOOTING"),
    "sourcing": plugin("SOURCING"),
    "space": plugin("SPACE"),
    "stdio": plugin("STDIO"),
    "trinity": plugin("TRINITY"),
    "vcs": plugin("VCS"),
    "wfc": plugin("WFC"),
    "writer": plugin("WRITER"),
}

ARTIFACTS = {
    ("animate", "presentation"): artifact("ANIMATE", "RUNNING-PRESENTATION"),
    ("architect", "program"): artifact("ARCHITECT", "RUNNING-PROGRAM"),
    ("block", "2d"): artifact("BLOCK", "RUNNING-BLOCK-2D"),
    ("block", "3d"): artifact("BLOCK", "RUNNING-BLOCK-3D"),
    ("block", "5d"): artifact("BLOCK", "RUNNING-BLOCK-5D"),
    ("demonstrator", "playground"): artifact("DEMONSTRATOR", "RUNNING-PLAYGROUND"),
    ("draw", "drawing"): artifact("DRAW", "RUNNING-DRAWING"),
    ("energy", "model"): artifact("ENERGY", "RUNNING-MODEL"),
    ("fem", "2d"): artifact("FEM", "RUNNING-FEM-2D"),
    ("fem", "3d"): artifact("FEM", "RUNNING-FEM-3D"),
    ("gis", "map"): artifact("GIS", "RUNNING-GIS-MAP"),
    ("gis", "gismap"): artifact("GIS", "RUNNING-GIS-MAP"),
    ("gis", "terrain"): artifact("GIS", "RUNNING-GIS-TERRAIN"),
    ("gis", "gisterrain"): artifact("GIS", "RUNNING-GIS-TERRAIN"),
    ("imperative", "procedure"): artifact("IMPERATIVE", "RUNNING-PROCEDURE"),
    ("mathematical", "equation"): artifact("MATHEMATICAL", "RUNNING-EQUATION"),
    ("procedural", "generation2d"): artifact("PROCEDURAL", "RUNNING-GENERATION-2D"),
    ("procedural", "2d"): artifact("PROCEDURAL", "RUNNING-GENERATION-2D"),
    ("procedural", "generation3d"): artifact("PROCEDURAL", "RUNNING-GENERATION-3D"),
    ("procedural", "3d"): artifact("PROCEDURAL", "RUNNING-GENERATION-3D"),
    ("process", "process3d"): artifact("PROCESS", "RUNNING-PROCESS-3D"),
    ("process", "3d"): artifact("PROCESS", "RUNNING-PROCESS-3D"),
    ("puzzle", "2d"): artifact("PUZZLE", "RUNNING-PUZZLE-2D"),
    ("puzzle", "3d"): artifact("PUZZLE", "RUNNING-PUZZLE-3D"),
    ("puzzle", "5d"): artifact("PUZZLE", "RUNNING-PUZZLE-5D"),
    ("reasoning", "wires"): artifact("REASONING", "RUNNING-WIRES"),
    ("remodel", "remodeling"): artifact("REMODEL", "RUNNING-REMODELING"),
    ("sourcing", "curation"): artifact("SOURCING", "RUNNING-CURATION"),
    ("space", "home"): artifact("SPACE", "RUNNING-HOME"),
    ("trinity", "jack"): artifact("TRINITY", "RUNNING-JACK"),
    ("trinity", "rewriting"): artifact("TRINITY", "RUNNING-REWRITING"),
    ("wfc", "2d"): artifact("WFC", "RUNNING-WFC-2D"),
    ("wfc", "3d"): artifact("WFC", "RUNNING-WFC-3D"),
    ("wfc", "bitmap"): artifact("WFC", "RUNNING-BITMAP"),
    ("wfc", "grid2d"): artifact("WFC", "RUNNING-GRID-2D"),
    ("wfc", "grid3d"): artifact("WFC", "RUNNING-GRID-3D"),
}

NORM = {
    "din16798": artifact("NORM", "RUNNING-DIN-16798"),
    "din18599": artifact("NORM", "RUNNING-DIN-18599"),
    "din4108": artifact("NORM", "RUNNING-DIN-4108"),
    "iso16757": artifact("NORM", "RUNNING-ISO-16757"),
    "vdi3805": artifact("NORM", "RUNNING-VDI-3805"),
}
for year in range(1990, 2000):
    NORM[f"en{year}"] = artifact("NORM", f"RUNNING-EN-{year}")

STDIO = {
    "avi": "RUNNING-AVI", "bcf": "RUNNING-BCF", "binary": "RUNNING-BINARY", "bmp": "RUNNING-BMP",
    "commands": "RUNNING-COMMANDS", "contract": "RUNNING-CONTRACT", "csv": "RUNNING-CSV",
    "deflate": "RUNNING-DEFLATE", "docx": "RUNNING-DOCX", "dwg": "RUNNING-DWG", "dxf": "RUNNING-DXF",
    "epw": "RUNNING-EPW", "gif": "RUNNING-GIF", "gltf": "RUNNING-GL-TF", "graph": "RUNNING-GRAPH",
    "html": "RUNNING-HTML", "ifc": "RUNNING-IFC", "inventory": "RUNNING-INVENTORY", "jpg": "RUNNING-JPG",
    "jpeg": "RUNNING-JPG", "json": "RUNNING-JSON", "las": "RUNNING-LAS", "md": "RUNNING-MD",
    "mp3": "RUNNING-MP3", "mp4": "RUNNING-MP4", "obj": "RUNNING-OBJ", "pdf": "RUNNING-PDF",
    "ply": "RUNNING-PLY", "png": "RUNNING-PNG", "pptx": "RUNNING-PPTX", "semio": "RUNNING-SEMIO",
    "step": "RUNNING-STEP", "stl": "RUNNING-STL", "svg": "RUNNING-SVG", "tiff": "RUNNING-TIFF",
    "tsv": "RUNNING-TSV", "txt": "RUNNING-TXT", "wav": "RUNNING-WAV", "xlsx": "RUNNING-XLSX",
    "xml": "RUNNING-XML", "zip": "RUNNING-ZIP",
}

REPO_SEGMENTS = {
    "aioptimizedrepo", "repo", "repoclient", "repobinary", "repocli", "repoclifilters", "repomcp",
    "repomcptools", "repomcpprompts", "repomcpresources", "repomechanisms", "repofilemechanism",
    "reposectionmechanism", "repofoldermechanism", "repobundlemechanism", "repocommitmechanism",
    "repoticketmechanism", "repogoalmechanism", "repopolicymechanism", "repodefinitionmechanism",
    "repodraftmechanism", "repocontributormechanism", "repoprojectmechanism", "repotodomechanism",
    "repolicensemechanism", "repovscodeextension", "reposerver", "repoapi", "sandboxedrepo",
    "zerotouchdevcontainer", "singlefilerepo", "consistentsections", "consistentrepohistory",
    "repogo", "repotooling", "clifoundation",
}

SKETCHPAD_SEGMENTS = {
    "runningsketchpad", "sketchpad", "sketchpadimprovements", "sketchpadapps", "sketchpadfeatures",
    "runningdesignapp", "runningtypeapp", "runningkitapp", "runningdocsapp", "runninghomeapp",
    "designapp", "typeapp", "kitapp", "docsapp", "homeapp", "apps",
}

CATCHALL = {"r2602", "r2603", "r26021", "r26002", "r2602", "aioptimizedrepo"}

def segments(goal: str) -> list:
    text = goal.replace("\ufe0f", "").replace("\u200d", "").strip()
    if not text:
        return []
    if "🎯" in text:
        return [part for part in text.split("🎯") if part]
    if "/" in text:
        return [flat(part) for part in text.split("/") if part and part not in {"🎯"}]
    return [flat(text)]

def deepen(path: str, plugin_name: str, tail: list) -> str:
    if plugin_name == "norm":
        for part in tail:
            if part in NORM:
                return NORM[part]
        return path
    if plugin_name == "stdio":
        for part in tail:
            if part in STDIO:
                return artifact("STDIO", STDIO[part])
        return path
    if plugin_name == "puzzle":
        for part in tail:
            if part in {"2d", "3d", "5d"}:
                return ARTIFACTS[("puzzle", part)]
            if part in {"puzzle2d", "2dboard"}:
                return ARTIFACTS[("puzzle", "2d")]
            if part in {"puzzle3d", "puzzl3d", "3dworld"}:
                return ARTIFACTS[("puzzle", "3d")]
            if part in {"puzzle5d", "5dtimeline"}:
                return ARTIFACTS[("puzzle", "5d")]
        return path
    for part in tail:
        hit = ARTIFACTS.get((plugin_name, part))
        if hit:
            return hit
    return path

def from_segments(segs: list):
    if not segs or segs[0] == "runningframework":
        return None
    if any(part.startswith("testgoal") for part in segs):
        return None
    joined = set(segs)
    if joined & {"runningnet", "grasshopper", "testednet", "pureccomponents", "testedgrasshoppercomponents"}:
        return None
    if "updateddocs" in joined or "updateddevdocs" in joined or "updateduserdocs" in joined:
        return None
    for index, part in enumerate(segs):
        if part in PLUGINS:
            return deepen(PLUGINS[part], part, segs[index + 1 :])
        if part in {"puzzle2d", "puzzle3d", "puzzle5d", "puzzl3d"}:
            dim = "2d" if part == "puzzle2d" else "5d" if part == "puzzle5d" else "3d"
            return ARTIFACTS[("puzzle", dim)]
    if "hub" in joined or "oshub" in joined:
        return HUB
    if "print" in joined or "runningprint" in joined:
        return PRINT
    if "presentation" in joined:
        return PRESENTATION
    if "os" in joined or "runningos" in joined:
        return OS
    if joined & REPO_SEGMENTS and "aioptimizedrepo" in joined:
        if len(segs) > 1:
            return REPO
    if segs == ["repo"] or (len(segs) > 1 and segs[0] == "repo"):
        return REPO
    if "framework" in joined and not (joined & set(PLUGINS) or joined & {"playground", "ui", "platform"}):
        if "os" in joined:
            return OS
        if "presentation" in joined:
            return PRESENTATION
        if segs == ["framework"] or segs[-1] == "framework":
            return FRAMEWORK
    if joined & SKETCHPAD_SEGMENTS or "runningsketchpad" in joined:
        return S
    if segs == ["s"] or (segs and segs[0] == "s"):
        return S
    return None

TITLE_RULES = [
    (re.compile(r"\bpuzzle\s*5d\b|\bpuzzle5d\b", re.I), ARTIFACTS[("puzzle", "5d")]),
    (re.compile(r"\bpuzzle\s*3d\b|\bpuzzle3d\b|\bpuzzl3d\b", re.I), ARTIFACTS[("puzzle", "3d")]),
    (re.compile(r"\bpuzzle\s*2d\b|\bpuzzle2d\b", re.I), ARTIFACTS[("puzzle", "2d")]),
    (re.compile(r"\bpuzzle\b", re.I), plugin("PUZZLE")),
    (re.compile(r"\biso\s*16757\b", re.I), NORM["iso16757"]),
    (re.compile(r"\bvdi\s*3805\b", re.I), NORM["vdi3805"]),
    (re.compile(r"\bdin\s*16798\b", re.I), NORM["din16798"]),
    (re.compile(r"\bdin\s*18599\b", re.I), NORM["din18599"]),
    (re.compile(r"\bdin\s*4108\b", re.I), NORM["din4108"]),
    (re.compile(r"\ben\s*1990\b", re.I), NORM["en1990"]),
    (re.compile(r"\ben\s*1991\b", re.I), NORM["en1991"]),
    (re.compile(r"\ben\s*1992\b", re.I), NORM["en1992"]),
    (re.compile(r"\ben\s*1993\b", re.I), NORM["en1993"]),
    (re.compile(r"\ben\s*1994\b", re.I), NORM["en1994"]),
    (re.compile(r"\ben\s*1995\b", re.I), NORM["en1995"]),
    (re.compile(r"\ben\s*1996\b", re.I), NORM["en1996"]),
    (re.compile(r"\ben\s*1997\b", re.I), NORM["en1997"]),
    (re.compile(r"\ben\s*1998\b", re.I), NORM["en1998"]),
    (re.compile(r"\ben\s*1999\b", re.I), NORM["en1999"]),
    (re.compile(r"\barchitect\b", re.I), plugin("ARCHITECT")),
    (re.compile(r"\banimate\b", re.I), plugin("ANIMATE")),
    (re.compile(r"\benergy engine\b|\benergyplus\b", re.I), plugin("ENERGY")),
    (re.compile(r"\bcad\b", re.I), plugin("CAD")),
    (re.compile(r"\bwfc\b|\bwave function collapse\b", re.I), plugin("WFC")),
    (re.compile(r"\btrinity\b", re.I), plugin("TRINITY")),
    (re.compile(r"\bprocedural\b", re.I), plugin("PROCEDURAL")),
    (re.compile(r"\blowpoly\b", re.I), plugin("LOWPOLY")),
    (re.compile(r"\bgis\b", re.I), plugin("GIS")),
    (re.compile(r"\bstdio\b", re.I), plugin("STDIO")),
    (re.compile(r"\bplaybook\b", re.I), plugin("PLAYBOOK")),
    (re.compile(r"\bfem\b", re.I), plugin("FEM")),
    (re.compile(r"\bwires\b", re.I), ARTIFACTS[("reasoning", "wires")]),
    (re.compile(r"\bos-hub\b|\bhub server\b", re.I), HUB),
    (re.compile(r"\bsketchpad\b", re.I), S),
    (re.compile(r"\bshooting\b", re.I), plugin("SHOOTING")),
    (re.compile(r"\bwriter\b", re.I), plugin("WRITER")),
    (re.compile(r"\bprint product\b|\blatex\b", re.I), PRINT),
]

def from_title(text: str):
    hits = []
    for pattern, path in TITLE_RULES:
        if pattern.search(text):
            hits.append(path)
    if not hits:
        return None
    # Keep the deepest unique family. Conflicting plugins cancel the title signal.
    families = {hit.split("/RUNNING-PLUGINS/")[-1].split("/")[0] if "/RUNNING-PLUGINS/" in hit else hit for hit in hits}
    if len(families) > 1:
        specific = [hit for hit in hits if hit.count("/") == max(item.count("/") for item in hits)]
        plugins = {hit for hit in specific}
        if len(plugins) == 1:
            return specific[0]
        return None
    return max(hits, key=lambda hit: hit.count("/"))

def classify(goal: str, title: str, description: str):
    segs = segments(goal)
    if segs and segs[0] == "runningframework":
        return None
    goal_path = from_segments(segs)
    title_path = from_title(f"{title}\n{description}")
    if goal_path and title_path and title_path != goal_path and title_path.startswith(goal_path + "/"):
        return title_path
    if goal_path and title_path and title_path != goal_path:
        # A title that names a different plugin overrides a catch-all or a mismatched goal.
        goal_plugin = goal_path.split("/RUNNING-PLUGINS/")[-1].split("/")[0] if "/RUNNING-PLUGINS/" in goal_path else ""
        title_plugin = title_path.split("/RUNNING-PLUGINS/")[-1].split("/")[0] if "/RUNNING-PLUGINS/" in title_path else ""
        if goal_plugin and title_plugin and goal_plugin != title_plugin:
            return title_path
        if not goal_plugin and title_plugin:
            return title_path
    if goal_path:
        return goal_path
    catchall = bool(segs) and (segs[0] in CATCHALL or "r2602" in segs or "r2603" in segs or "aioptimizedrepo" in segs)
    if catchall and title_path:
        return title_path
    return None

def replace_goal(text: str, new_goal: str) -> str:
    match = re.search(r'("goal"\s*:\s*")([^"]*)(")', text)
    if not match:
        raise ValueError("goal field missing")
    return text[: match.start(2)] + new_goal + text[match.end(2) :]

moves = Counter()
examples = {}
changed = 0
skipped_bad = 0
for path in TICKETS.rglob("🎫️ticket.json"):
    raw = path.read_text(encoding="utf-8")
    try:
        doc = json.loads(raw)
    except json.JSONDecodeError:
        skipped_bad += 1
        continue
    goal = doc.get("goal") or ""
    target = classify(goal, doc.get("title") or "", doc.get("description") or "")
    if not target:
        continue
    if target not in NEW:
        raise SystemExit(f"unknown target {target}")
    new_goal = NEW[target]
    if goal.replace("\ufe0f", "") == new_goal:
        continue
    moves[target] += 1
    examples.setdefault(target, []).append(doc.get("title") or "")
    if APPLY:
        path.write_text(replace_goal(raw, new_goal), encoding="utf-8")
        changed += 1

print(("applied" if APPLY else "dry-run"), sum(moves.values()), "tickets")
print("bad json", skipped_bad)
for target, count in moves.most_common():
    print(f"{count:4} {target}")
    for title in examples[target][:3]:
        print(f"     - {title[:100]}")
