#!/usr/bin/env python3
"""🛬️ T14 window-3 landing: ONE compile-atomic pass of every set the overlay proved together, in this order — F9 content ids
(`f9/content-id.py`) + its carrier map (`f9/apply-map.py` over the recorded `s14b-f9-map-*.tsv`), G12's authoring-seed builder
pass, the class fix (`p8class/follow-children.py`), P8 orphan, H9-L labels, 5b A+B1 (`5b/dsl-value.py`) + B2
(`5b/dsl-value-b2.py`), item 6 (`item6/fallback-wrappers.py`).

`--check` dry-runs every set on the live tree. `--write` first copies the live bytes of every file the overlay proof changed
(`generated/s14b-overlay-changed-*.txt`, newest) to `.🧬semio/🌐hub/s14-t14-land/before/`, runs the sets in order, then compares
every such file with the proven overlay (the store file without the overlay-only id recorder): a file a peer edited after the
overlay sync differs and is listed for a look, never overwritten from the overlay. `--restore` puts back only the files still
byte-equal to what this landing wrote. Compile gates run through the fleet lanes separately.
usage: land-w3.py --check | --write | --restore | --status"""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
OVERLAY = ROOT / ".🧬semio/🌐hub/s14-t14-overlay"
LOGS = ROOT / ".🧬semio/🌐hub/s14-t14-logs"
STATE = ROOT / ".🧬semio/🌐hub/s14-t14-land"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
PROCESS3D = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs"
MAPS = [str(path) for path in sorted(LOGS.glob("s14b-f9-map-*.tsv"))]
SETS = [
    ("f9", [sys.executable, str(HERE / "f9/content-id.py")], "--write", None),
    ("f9-carriers", [sys.executable, str(HERE / "f9/apply-map.py"), *sum((["--map", path] for path in MAPS), [])], "--write", None),
    ("g12", [sys.executable, str(ROOT / ".tmp-ticket/wp-g12/g12-authoring-seed.py")], "--write", "--dry-run"),
    ("class-fix", [sys.executable, str(HERE / "p8class/follow-children.py")], "--write", None),
    ("p8-orphan", [sys.executable, str(ROOT / ".tmp-ticket/wp-p8/patches/p8-orphan.py")], "--write", "--dry-run"),
    ("h9l", [sys.executable, str(HERE / "h9l/kind-label-patch.py")], "--apply", None),
    ("5b", [sys.executable, str(HERE / "5b/dsl-value.py")], "--write", None),
    ("5b-b2", [sys.executable, str(HERE / "5b/dsl-value-b2.py")], "--write", None),
    ("item6", [sys.executable, str(HERE / "item6/fallback-wrappers.py")], "--write", None),
]


def run(command, flag):
    return subprocess.run(command + ([flag] if flag else []), cwd=ROOT, capture_output=True, text=True)


def proven_files():
    listings = sorted((HERE / "generated").glob("s14b-overlay-changed-*.txt"), key=lambda path: int(path.stem.rsplit("-", 1)[1]))
    return [line for line in listings[-1].read_text(encoding="utf-8").split("\n") if line and line != ".t14-stamp"]


def expected_bytes(rel):
    data = (OVERLAY / rel).read_bytes()
    if rel not in (STORE, PROCESS3D):
        return data
    constants = {}
    source = (HERE / "f9/record-instrument.py").read_text(encoding="utf-8")
    exec(compile(source.split('if __name__ != "__main__":', 1)[0], "record-instrument", "exec"), constants)
    pairs = [(constants["NEW"], constants["OLD"])] if rel == STORE else [(constants["P3_NEW"], constants["P3_OLD"]), (constants["P3_FLOW_NEW"], constants["P3_FLOW_OLD"])]
    text = data.decode("utf-8")
    for instrumented, original in pairs:
        text = text.replace(instrumented, original)
    return text.encode("utf-8")


def check():
    clean = True
    for name, command, _, dry in SETS:
        if name == "f9-carriers" and not MAPS:
            print(f"[{name}] no recorded map yet")
            clean = False
            continue
        result = run(command, dry)
        tail = (result.stdout + result.stderr).strip().splitlines()[-2:]
        print(f"[{name}] rc={result.returncode}: {' | '.join(tail)}")
        clean &= result.returncode == 0 or name in ("g12", "class-fix", "p8-orphan", "5b-b2")
    return clean


def write():
    if (STATE / "manifest.json").exists():
        raise SystemExit(f"already landed by this tool ({STATE}); --restore first")
    files = proven_files()
    before = STATE / "before"
    for rel in files:
        if (ROOT / rel).is_file():
            (before / rel).parent.mkdir(parents=True, exist_ok=True)
            (before / rel).write_bytes((ROOT / rel).read_bytes())
    for name, command, flag, _ in SETS:
        result = run(command, flag)
        print(f"[{name}] rc={result.returncode}: {' | '.join((result.stdout + result.stderr).strip().splitlines()[-2:])}")
        if result.returncode != 0:
            print(f"[{name}] FAILED — nothing after it ran; --restore puts every earlier set back")
            break
    manifest, differing = [], []
    for rel in files:
        live = ROOT / rel
        if not live.is_file():
            continue
        manifest.append({"rel": rel, "created": not (before / rel).is_file()})
        if live.read_bytes() != expected_bytes(rel):
            differing.append(rel)
    (STATE / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=1), encoding="utf-8")
    (STATE / "after").mkdir(exist_ok=True)
    for row in manifest:
        (STATE / "after" / row["rel"]).parent.mkdir(parents=True, exist_ok=True)
        (STATE / "after" / row["rel"]).write_bytes((ROOT / row["rel"]).read_bytes())
    print(f"{len(manifest)} files recorded; {len(differing)} differ from the proven overlay")
    for rel in differing:
        print("  differs", rel)


def restore():
    manifest = json.loads((STATE / "manifest.json").read_text(encoding="utf-8"))
    kept = 0
    for row in manifest:
        path, after = ROOT / row["rel"], STATE / "after" / row["rel"]
        if not path.exists() or path.read_bytes() != after.read_bytes():
            kept += 1
            print("kept (edited since):", row["rel"])
            continue
        if row["created"]:
            path.unlink()
        else:
            path.write_bytes((STATE / "before" / row["rel"]).read_bytes())
    print(f"restored {len(manifest) - kept}, kept {kept}")


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
    if mode == "--check":
        sys.exit(0 if check() else 1)
    if mode == "--write":
        write()
    elif mode == "--restore":
        restore()
    elif mode == "--status":
        print("landed" if (STATE / "manifest.json").exists() else "not landed", f"maps: {MAPS}")
