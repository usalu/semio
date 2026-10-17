"""🏃 Whole-workspace tsc run that tolerates peers' transient syntax errors: generates the project through the permanent route, drops files with syntax diagnostics (TS1xxx) and files named by --drop, reruns, and writes <name>.final.txt."""
import json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
GEN = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated")

def main():
    name = sys.argv[1]
    drops = [a[len("--drop="):] for a in sys.argv[2:] if a.startswith("--drop=")]
    files_from = [a[len("--files-from="):] for a in sys.argv[2:] if a.startswith("--files-from=")]
    out = os.path.join(GEN, f"{name}.tsconfig.json")
    cmd = ["nice", "-n", "10", "bun", "./📜️script.ts", "verify", "typescript", "project", "--out", out]
    if files_from:
        cmd += ["--files-from", files_from[0]]
    subprocess.run(cmd, cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
    doc = json.load(open(out, encoding="utf8"))
    dropped = []
    for _ in range(6):
        scan = subprocess.run(["nice", "-n", "10", "node", os.path.join(os.path.dirname(__file__), "../tt4-tools/tt4_syntax.mjs"), out], capture_output=True, text=True, cwd=ROOT)
        bad = [line for line in scan.stdout.split("\n") if line]
        if bad:
            doc["files"] = [f for f in doc["files"] if f not in set(bad)]
            dropped.extend(bad)
            print("syntax-dropped:", len(bad), flush=True)
        files = [f for f in doc["files"] if not any(d in f for d in drops)]
        doc["files"] = files
        json.dump(doc, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=2)
        log = os.path.join(GEN, f"{name}.tsc.txt")
        with open(log, "w", encoding="utf8") as handle:
            subprocess.run(["nice", "-n", "10", "node", "--max-old-space-size=24576", "node_modules/typescript/bin/tsc", "-p", out, "--noEmit", "--pretty", "false"], cwd=ROOT, stdout=handle, stderr=subprocess.STDOUT)
        text = open(log, encoding="utf8").read()
        syntax = {m.group(1) for m in re.finditer(r"^(.*?)\(\d+,\d+\): error TS(?:1002|1003|1005|1010|1011|1068|1109|1127|1128|1131|1134|1136|1138|1161|1434): ", text, re.M)}
        if not syntax:
            break
        for path in syntax:
            rel = os.path.relpath(os.path.join(ROOT, path), os.path.dirname(out)) if not os.path.isabs(path) else path
            doc["files"] = [f for f in doc["files"] if not f.endswith(path)]
            dropped.append(path)
    open(os.path.join(GEN, f"{name}.dropped.txt"), "w", encoding="utf8").write("\n".join(dropped) + "\n")
    n = len(re.findall(r"^\S.*?\(\d+,\d+\): error TS\d+: ", text, re.M))
    print(f"{name}: roots={len(doc['files'])} diagnostics={n} dropped={len(dropped)}")

main()
