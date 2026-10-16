"""🟦️ Builds two TypeScript sandboxes (baseline and current) of the 🧰️framework TS/JSON sources with node_modules symlinked,
writes a scoped tsconfig listing the touched TS files of each tree, and runs `tsc --noEmit` on both; prints the error
sets with paths mapped from old to new locations so the two runs compare.
Usage: ts_sandbox.py BASELINE_DIR OUT_JSON
"""
import os, sys, shutil, subprocess, pathlib, json, re
REPO = pathlib.Path("/Users/ueli/Documents/semio")
B = pathlib.Path(sys.argv[1])
OUT = sys.argv[2]
S = pathlib.Path("/private/tmp/claude-501/-Users-ueli-Documents-semio/35792af2-9c1e-4f70-b4e3-d5fa94336422/scratchpad/ts-sandbox")
EXT = (".ts", ".tsx", ".json", ".d.ts", ".cjs", ".mjs", ".css")
MOVES = {
    "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️history-edit-actions/🟦️.ts": "🧰️framework/🔨️modules/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️tool-run-actions/🟦️.ts": "🧰️framework/🔨️modules/⏯️tool-run/🕹️actions/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🛂️manifest/🎬️tutorial/🏠️local-interaction/🟦️.ts": "🧰️framework/🔨️modules/🎓️onboarding/🏠️local-interaction/🟦️.ts",
}
CURRENT = ["🧰️framework/📦️packages/🟦️typescript/🟦️.ts", "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts", "🧰️framework/🔨️modules/🛂️manifest/🗺️window-layout/🟦️.ts", "🧰️framework/🔨️modules/🖥️platform/🟦️.ts", "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts", "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts", "🧰️framework/🔨️modules/⏪️time-travel/🕹️actions/🟦️.ts", "🧰️framework/🔨️modules/⏯️tool-run/🕹️actions/🟦️.ts", "🧰️framework/🔨️modules/🎓️onboarding/🟦️.ts", "🧰️framework/🔨️modules/🎓️onboarding/🏠️local-interaction/🟦️.ts", "🧰️framework/🔨️modules/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/⏯️tool-run/🕹️actions/🧪️tests/🟦️.ts"]
BASE = ["🧰️framework/📦️packages/🟦️typescript/🟦️.ts", "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts", "🧰️framework/🔨️modules/🖥️platform/🟦️.ts", "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts", "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts"] + list(MOVES)

def mirror_node_modules(src, dest, sandbox):
    dest.mkdir(parents=True, exist_ok=True)
    for entry in os.listdir(src):
        target = src / entry
        if entry.startswith("@") and target.is_dir() and not target.is_symlink():
            scope = dest / entry
            scope.mkdir(exist_ok=True)
            for pkg in os.listdir(target):
                link = target / pkg
                real = pathlib.Path(os.path.realpath(link))
                try:
                    rel = real.relative_to(REPO / "🧰️framework")
                    os.symlink(sandbox / "🧰️framework" / rel, scope / pkg)
                except ValueError:
                    os.symlink(real, scope / pkg)
        else:
            os.symlink(target, dest / entry)

def copy_tree(dest):
    if dest.exists():
        shutil.rmtree(dest)
    pending = []
    for root, dirs, files in os.walk(REPO / "🧰️framework"):
        rel = pathlib.Path(root).relative_to(REPO)
        if "node_modules" in dirs:
            (dest / rel).mkdir(parents=True, exist_ok=True)
            pending.append((pathlib.Path(root) / "node_modules", dest / rel / "node_modules"))
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "🎯️target", "dist", ".nx", "🗑️generated")]
        for f in files:
            if f.endswith(EXT):
                (dest / rel).mkdir(parents=True, exist_ok=True)
                shutil.copy2(pathlib.Path(root) / f, dest / rel / f)
    for src_nm, dest_nm in pending:
        mirror_node_modules(src_nm, dest_nm, dest)
    mirror_node_modules(REPO / "node_modules", dest / "node_modules", dest)
    shutil.copy2(REPO / "tsconfig.json", dest / "tsconfig.json")
    shutil.copy2(REPO / "package.json", dest / "package.json")

def run(dest, files):
    cfg = {"extends": "./tsconfig.json", "compilerOptions": {"noEmit": True, "types": ["bun-types", "vitest/importMeta"] if False else None}, "files": files, "include": []}
    cfg["compilerOptions"] = {"noEmit": True}
    (dest / "tsconfig.relo.json").write_text(json.dumps(cfg, ensure_ascii=False))
    res = subprocess.run(["perl", "-e", "alarm 1500; exec @ARGV", "--", "node", str(REPO / "node_modules/typescript/bin/tsc"), "-p", "tsconfig.relo.json", "--pretty", "false"], cwd=dest, capture_output=True, text=True)
    errs = [l for l in res.stdout.split("\n") if re.search(r"error TS\d+", l)]
    return errs

cur = S / "cur"
copy_tree(cur)
base = S / "base"
copy_tree(base)
for path in B.rglob("*"):
    if path.is_file() and str(path.relative_to(B)).startswith("🧰️framework") and path.suffix in (".ts", ".tsx", ".json"):
        target = base / path.relative_to(B)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(path, target)
for new in CURRENT[6:]:
    p = base / new
    if p.exists() and new not in BASE:
        p.unlink()
ec = run(cur, CURRENT)
eb = run(base, BASE)
def norm(line):
    for old, new in MOVES.items():
        line = line.replace(old, new)
    return re.sub(r"\(\d+,\d+\)", "", line)
sc, sb = sorted(map(norm, ec)), sorted(map(norm, eb))
import collections
cc, cb = collections.Counter(sc), collections.Counter(sb)
json.dump({"current": ec, "baseline": eb, "new": list((cc - cb).elements()), "gone": list((cb - cc).elements())}, open(OUT, "w"), ensure_ascii=False, indent=1)
print("baseline errors", len(eb), "current errors", len(ec), "new", sum((cc - cb).values()), "gone", sum((cb - cc).values()))
for l in list((cc - cb).elements())[:40]:
    print("NEW", l[:300])
for l in list((cb - cc).elements())[:20]:
    print("GONE", l[:300])
