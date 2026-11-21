import hashlib
import json
import re
import sys
from pathlib import Path


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def locate(run, module):
    base = f"semio_hub_{module}_component"
    units = json.loads((run / "r/units.json").read_bytes())
    stages = {s["stage"] for s in units["stages"] if s["ok"] and s["component"] == f"semio-hub-{module}-component"}
    if not {"stage", "transpile"} <= stages:
        raise ValueError("run has no successful component transpilation")
    raws = [p for p in (run / "w").glob("*/component.wasm") if re.fullmatch("[0-9a-f]{24}", p.parent.name)]
    if len(raws) != 1:
        raise ValueError("expected one staged component")
    raw_hash = digest(raws[0])
    candidates = []
    for path in (run.parents[1] / "stages/transpile").glob("*/🧾️entry.json"):
        entry = json.loads(path.read_bytes())
        key = entry["key"]
        if key["stage"] != "transpile" or key["tools"].get("base") != base or key["inputs"].get("component") != raw_hash:
            continue
        canonical = json.dumps(key, sort_keys=True, ensure_ascii=False, separators=(",", ":")).encode()
        if path.parent.name != entry["digest"] or hashlib.sha256(canonical).hexdigest() != entry["digest"]:
            raise ValueError("transpile key digest differs")
        records = {f["path"]: f for f in entry["files"]}
        names = {p.name for p in path.parent.glob("*.wasm")} | {base + ".js", base + ".d.ts"}
        if base + ".core.wasm" not in names or names != {n for n in records if n.endswith(".wasm")} | {base + ".js", base + ".d.ts"}:
            raise ValueError("transpiled file inventory differs")
        for name in names:
            file = path.parent / name
            if file.is_symlink() or file.stat().st_size != records[name]["bytes"] or digest(file) != records[name]["sha256"]:
                raise ValueError("transpile bytes differ for " + name)
        candidates.append(path.parent)
    if len(candidates) != 1:
        raise ValueError("expected one cache entry for the staged component")
    return candidates[0]


if __name__ == "__main__":
    try:
        print(locate(Path(sys.argv[1]), sys.argv[2]))
    except (OSError, ValueError, KeyError) as error:
        sys.exit(str(error))
