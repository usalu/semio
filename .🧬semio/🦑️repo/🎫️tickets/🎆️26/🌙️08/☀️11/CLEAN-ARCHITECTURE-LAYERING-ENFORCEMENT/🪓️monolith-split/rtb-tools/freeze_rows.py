"""🧊️ Refreshes 🗑️generated/rwp0-crates.json from the manifests: first-party and external dependency sets, rows for the new crates and a topological re-rank of the test side. DRY=1 only reports."""
import json, os, re
from lib import *

TABLE = GEN + "/rwp0-crates.json"
NEW = {
    "fixtures": dict(kind="support", wp=4, desc="Shell fixtures of the renderer tests: host shells, apps, pointer records and tour shells behind the testkit feature"),
    "frame-tests-1": dict(kind="companion", wp=3, desc="Companion test bin of the wgpu renderer: frame core and renderer laws (1)"),
}


def tables(text):
    out = {}
    sec = None
    for line in text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and re.fullmatch(r"(target\..+\.)?dependencies", sec) and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip().split(".")[0]
            if re.match(r"^[A-Za-z0-9_\-]+$", key):
                pk = re.search(r'package\s*=\s*"([^"]+)"', line)
                out[pk.group(1) if pk else key] = line
    return out


def main():
    d = json.load(open(TABLE, encoding="utf8"))
    cr = crate_table()
    for suffix in NEW:
        if suffix not in d:
            folder = cr[suffix]
            rel_dir = rel(T + "/" + folder)
            d[suffix] = dict(key=suffix, kind=NEW[suffix]["kind"], wp=NEW[suffix]["wp"], cfg="both", deps=[], ext=[], tests=False, dir=rel_dir, pkg=rel_dir + "/📦️packages/🦀️rust", folder=folder, name=PKG_PREFIX + suffix, id="os-renderer-wgpu-" + suffix, project=f"@semio-tech/os-renderer-wgpu-{suffix}-rs", rank=999)
    byname = {v["name"]: k for k, v in d.items()}
    changed = []
    for k, v in d.items():
        mp = REPO + "/" + v["pkg"] + "/Cargo.toml"
        deps = tables(read(mp))
        first = sorted({byname[n] for n in deps if n in byname})
        ext = sorted(n for n in deps if n not in byname and not n.startswith(PKG_PREFIX))
        old_first = sorted(v["deps"])
        if first != old_first:
            changed.append((k, sorted(set(first) - set(old_first)), sorted(set(old_first) - set(first))))
            order = {x: i for i, x in enumerate(v["deps"])}
            v["deps"] = [x for x in v["deps"] if x in first] + [x for x in first if x not in order]
        v["ext"] = ext if ext else v["ext"] if k not in NEW else ext
    for k, add, rem in changed:
        print(f"deps {k}: +{add} -{rem}")
    test_side = [k for k, v in d.items() if v["rank"] >= 140 or k in NEW]
    prio = {k: (d[k]["rank"] if k not in NEW else (143.5 if k == "fixtures" else 158.5)) for k in test_side}
    done, order = set(), []
    pending = sorted(test_side, key=lambda k: prio[k])
    while pending:
        for k in pending:
            if all(x in done or x not in prio for x in d[k]["deps"]):
                order.append(k)
                done.add(k)
                pending.remove(k)
                break
        else:
            raise SystemExit(f"cycle among {pending}")
    for i, k in enumerate(order):
        if d[k]["rank"] != 140 + i:
            d[k]["rank"] = 140 + i
    for k, v in d.items():
        for x in v["deps"]:
            assert d[x]["rank"] < v["rank"], (k, x)
    print("ranks", [(k, d[k]["rank"]) for k in order])
    if os.environ.get("DRY") != "1":
        backup(TABLE)
        edit(TABLE, json.dumps(d, indent=1, ensure_ascii=False) + "\n")
        print("written", len(d))


main()
