import json, os, subprocess, sys, re, collections
SC = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pgp"
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
name = sys.argv[1]
out = subprocess.run(["diff", "-rq", f"{SC}/base", f"{SC}/{name}/ts"], capture_output=True, text=True).stdout
changed = []
for line in out.split("\n"):
    m = re.match(r"Files .*/base/(.*) and .*/ts/(.*) differ", line)
    if m and m.group(1).endswith("🟦️.ts"):
        changed.append(m.group(1))
print(len(changed), "changed TypeScript leaves")
results = {}
for tree in ("py", "ts"):
    cfg = {"extends": "/Users/ueli/Documents/semio/tsconfig.json", "compilerOptions": {"noEmit": True, "incremental": False, "skipLibCheck": True, "types": []}, "include": [], "files": [f"{SC}/{name}/{tree}/{p}" for p in changed]}
    cp = f"{F}/🗑️generated/pgp-tsconfig-{name}-{tree}.json"
    json.dump(cfg, open(cp, "w", encoding="utf-8"), ensure_ascii=False)
    r = subprocess.run(["node", "--max-old-space-size=8192", "/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc", "-p", cp, "--noEmit", "--pretty", "false"], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio")
    lines = [l for l in r.stdout.split("\n") if re.match(r".*\(\d+,\d+\): error", l)]
    open(f"{F}/🗑️generated/pgp-tsc-{name}-{tree}.txt", "w", encoding="utf-8").write("\n".join(lines))
    results[tree] = lines
    codes = collections.Counter(re.search(r"error (TS\d+)", l).group(1) for l in lines)
    print(tree, len(lines), "diagnostics", dict(codes.most_common(6)))
