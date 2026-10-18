"""📝 C5G source-text reader re-points for the five cut host files (effects, guest-runtime, routers, shard, guest-cold-relay): every text read of a cut root becomes `readModuleTreeSource(<same path>)` (the module tree is the text the unsplit file held). Each edit asserts its expected match count; the import of the module-tree reader is added with a repo-relative path where the file lacks it. usage: tsedit6.py [--apply]"""
import os, re, sys
R = "/Users/ueli/Documents/semio/"
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
PL = M + "🔌️plugin/"
H = PL + "🖥️host/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
TREE = R + "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
HOSTLIST = r'\.map\(s=>readFileSync\(new URL\("(\.\./(?:\.\./)+)"\+s,import\.meta\.url\),"utf8"\)\)'
HOSTLIST_NEW = r'.map(s=>(s.includes("guest-runtime")||s.includes("routers"))?readModuleTreeSource(new URL("\1"+s,import.meta.url)):readFileSync(new URL("\1"+s,import.meta.url),"utf8"))'
# (path, regex, replacement, expected count)
EDITS = [
    (H + "⏱️deadline/🧪️tests/🟦️.ts", HOSTLIST, HOSTLIST_NEW, 1),
    (H + "♻️publication/🧪️tests/🟦️.ts", HOSTLIST, HOSTLIST_NEW, 1),
    (H + "🔔️wake/🧪️tests/🟦️.ts", HOSTLIST, HOSTLIST_NEW, 2),
    (H + "🎟️context/🤝️admission/🧪️tests/🟦️.ts", HOSTLIST, HOSTLIST_NEW, 1),
    (H + "🎟️context/🤝️admission/🧪️tests/🟦️.ts", r'readFileSync\((new URL\("\.\./\.\./\.\./[^"/]*shard/[^"/]+\.rs",import\.meta\.url\)),"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (H + "🧪️tests/🧱️ports/🟦️.ts", r'readFileSync\((join\(owner, "[^"]*guest-runtime", "[^"]*\.rs"\)), "utf8"\)', r'readModuleTreeSource(\1)', 1),
    (H + "🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests/🟦️.ts", r'readFileSync\((join\(host,"[^"]*guest-runtime","[^"]*\.rs"\)),"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (H + "🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests/🟦️.ts", r'readFileSync\((resolve\(import\.meta\.dir,"\.\./\.\./\.\./\.\.","[^"]*guest-runtime","[^"]*\.rs"\))," ?utf8"\)', r'readModuleTreeSource(\1)', 1),
    (H + "🧵️shard/🎟️grant/🧪️tests/🟦️.ts", r'readFileSync\((new URL\("\.\./\.\./[^"/]+\.rs",import\.meta\.url\)),"utf8"\)', r'readModuleTreeSource(\1)', 3),
    (H + "🧵️shard/🎟️grant/🧪️tests/🎟️supplier/🟦️.ts", r'readFileSync\((new URL\("\.\./\.\./\.\./[^"/]+\.rs",import\.meta\.url\)),"utf8"\)', r'readModuleTreeSource(\1)', 2),
    (M + "🛎️services/🧪️tests/🧮️compute-retained/🟦️.ts", r'read\(("\.\./[^"/]*plugin/[^"/]*host/[^"/]*effects/[^"/]+\.rs")\)', r'readModuleTreeSource(new URL(\1,base))', 2),
    (PL + "🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts", r'readFileSync\(rustUrl, "utf8"\)', r'readModuleTreeSource(rustUrl)', 1),
    (LIB + "⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts", r'policyReadFileSafe\((root), ("[^"]*host/[^"]*shard/[^"/]+\.rs")\)', r'policyReadModuleTreeSource(\1, \2)', 1),
    (LIB + "⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts", r'policyReadFileSafe\((repoRoot), ("[^"]*host/[^"]*shard/[^"/]+\.rs")\)', r'policyReadModuleTreeSource(\1, \2)', 1),
    (LIB + "🎛️commands/✅️verify/🟦️.ts", r'policyReadFileSafe\((this\.root), ("[^"]*host/[^"]*shard/[^"/]+\.rs")\)', r'policyReadModuleTreeSource(\1, \2)', 1),
    (H + "📦️packages/🦀️rust/📜️script.ts", r'readFileSync\((join\(hostRoot, "[^"]*guest-runtime", "[^"]*\.rs"\)), "utf8"\)', r'readModuleTreeSource(\1)', 2),
    (H + "📦️packages/🦀️rust/📜️script.ts", r'const tree = parser\.parse\(readFileSync\(join\(owner, relative\), "utf8"\)\);', 'const tree = parser.parse(/^\\.\\.\\/[^/]*shard\\/[^/]*\\.rs$/u.test(relative) ? readModuleTreeSource(join(owner, relative)) : readFileSync(join(owner, relative), "utf8"));', 1),
]
apply = "--apply" in sys.argv
bad = 0
by_path = {}
for path, rx, rep, n in EDITS:
    by_path.setdefault(path, []).append((rx, rep, n))
for path, edits in by_path.items():
    p = R + path
    text = open(p, encoding="utf-8").read()
    for rx, rep, n in edits:
        found = len(re.findall(rx, text))
        if found != n:
            print("COUNT", found, "!=", n, path.split("/")[-3], rx[:70]); bad += 1; continue
        text = re.sub(rx, rep, text)
        print("ok", n, "x", path.split("/")[-3] + "/" + path.split("/")[-2], rx[:55])
    if "readModuleTreeSource" in text and not re.search(r"^\s*import\b[^\n]*readModuleTreeSource", text, flags=re.M):
        rel = os.path.relpath(TREE, os.path.dirname(p))
        imp = f'import {{ readModuleTreeSource }} from "{rel}";'
        lines = text.split("\n")
        idx = max((i for i, l in enumerate(lines) if re.match(r"^import\b", l) and l.rstrip().endswith(";")), default=-1)
        lines.insert(idx + 1, imp)
        text = "\n".join(lines)
        print("   + import", rel[-40:])
    if "policyReadModuleTreeSource" in text and path.startswith(LIB):
        m = re.search(r"^import \{([^}]*)\} from (\"[^\"]*source-access/[^\"]*\");", text, flags=re.M)
        if m and "policyReadModuleTreeSource" not in m.group(1):
            print("   ! needs policyReadModuleTreeSource in import:", m.group(0)[:120]); bad += 1
        elif not m:
            print("   ! no source-access import found", path.split("/")[-3])
    if apply and not bad:
        open(p, "w", encoding="utf-8").write(text)
print("bad", bad, "applied" if apply and not bad else "dry")
