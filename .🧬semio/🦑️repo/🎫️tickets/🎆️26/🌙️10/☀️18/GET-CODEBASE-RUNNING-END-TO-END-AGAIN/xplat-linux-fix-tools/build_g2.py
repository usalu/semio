import json, os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import live, emit
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..")))
from winpath import resolve
FW = "🧰️framework"
NB = FW + "/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
CONTRACT = NB + "/🌐️platform/📋️contract/🟦️.ts"
FACADE = FW + "/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"
LIB = FW + "/🛍️products/🦑️repo/🔨️modules/📚️library"
edits, migrated = [], []

def rel(from_file, target):
    path = os.path.relpath(os.path.join(LIVE_ROOT, target), os.path.dirname(os.path.join(LIVE_ROOT, from_file)))
    return path if path.startswith(".") else "./" + path
LIVE_ROOT = "/Users/ueli/Documents/semio"

# contract: used bytes and space come from the shared usage; the new statfs surface is re-exported
text = live(CONTRACT)
edits.append((CONTRACT, 'import {nativeDirectoryProtocolV1,observeNativeDirectoryStateV1,measureNativeDirectoryUsageV1} from "../../📁️directory/🟦️.ts";',
 'import {nativeDirectoryProtocolV1,observeNativeDirectoryStateV1,readNativeDirectoryAnchorFileV1} from "../../📁️directory/🟦️.ts";\nimport {nativeDirectoryUsedBytesSyncV1} from "./📐️statfs/🟦️.ts";\nexport {nativeDirectoryUsedBytesSyncV1,nativePartitionOfV1,nativeStatfsSyncV1,nativeStatfsV1,type NativePartitionV1} from "./📐️statfs/🟦️.ts";'))
edits.append((CONTRACT, 'if(nativeImageDirectoryHostV1())return measureNativeDirectoryUsageV1(root).allocatedBytes;const filesystem=statfsSync(root,{bigint:true});',
 'if(nativeImageDirectoryHostV1())return nativeDirectoryUsedBytesSyncV1(root,readNativeDirectoryAnchorFileV1(dirname(root)).maximumPhysicalBytes);const filesystem=statfsSync(root,{bigint:true});'))
edits.append((CONTRACT, 'bavail:Math.max(0,Math.floor((declared-measureNativeDirectoryUsageV1(root).allocatedBytes)/bsize))',
 'bavail:Math.max(0,Math.floor((declared-nativeDirectoryUsedBytesSyncV1(root,declared))/bsize))'))
edits.append((FACADE, 'nativeImageUsedBytesV1, nativeProtectedArtifactInputGroupProtocolV1,', 'nativeImageUsedBytesV1, nativePartitionOfV1, nativeProtectedArtifactInputGroupProtocolV1,'))
edits.append((FACADE, 'nativeProtectedMaintenanceRosterTransitionV1, readNativeImageMaintenanceRequestV1,', 'nativeProtectedMaintenanceRosterTransitionV1, nativeStatfsSyncV1, nativeStatfsV1, readNativeImageMaintenanceRequestV1,'))
edits.append((FACADE, 'export type { NativeImageMaintenanceCustodyV1,', 'export type { NativePartitionV1, NativeImageMaintenanceCustodyV1,'))

core_edits = list(edits)
SITES = [
 (FW + "/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts", "internal"),
 (FW + "/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts", "internal"),
 (LIB + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/♻️recovery/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/🚀️bootstrap/💾️budget/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📥️physical/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts", "dynamic"),
 (LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🟦️.ts", "facade"),
 (LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts", "facade"),
 (LIB + "/🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs", "facade"),
 (LIB + "/🏗️build/📥️inputs/💾️storage/🟨️.mjs", "facade"),
]
for path, kind in SITES:
    text = live(path)
    target = CONTRACT if kind == "internal" else FACADE
    if path.startswith(NB) or "💾️cache/💾️volume" in path or "component-pass" in path:
        target = CONTRACT
    spec = rel(path, target)
    sync = "statfsSync" in text and "statfs(" not in text.replace("statfsSync(", "")
    names = []
    if kind == "dynamic":
        old = 'const{statfs}=await import("node:fs/promises"),fs=await statfs(store.root,{bigint:true}),unit='
        edits.append((path, old, 'const fs=await nativeStatfsV1(store.root,{bigint:true}),unit='))
        first = text.split("\n", 1)[0]
        edits.append((path, first, first + '\nimport{nativeStatfsV1}from"' + spec + '";'))
        migrated.append({"path": path, "uses": "nativeStatfsV1("})
        continue
    m = re.search(r'^import\s*(type\s*)?\{([^}]*)\}\s*from\s*"node:fs(/promises)?";?[ \t]*$', "\n".join(l for l in text.split("\n") if re.search(r'\bstatfs(Sync)?\b', l) and l.lstrip().startswith("import")), re.M)
    line = [l for l in text.split("\n") if l.lstrip().startswith("import") and re.search(r'\bstatfs(Sync)?\b', l) and 'node:fs' in l][0]
    inner = re.search(r'\{([^}]*)\}', line).group(1)
    parts = [p for p in re.split(r'\s*,\s*', inner.strip()) if p and p not in ("statfs", "statfsSync")]
    spaced = inner.startswith(" ")
    brace = "{ " + ", ".join(parts) + " }" if spaced else "{" + ",".join(parts) + "}"
    uses_sync = "statfsSync(" in text
    uses_async = re.search(r'(?<![A-Za-z0-9_.])statfs\(', text) is not None
    wanted = ("nativeStatfsSyncV1" if uses_sync else "") + ("," if uses_sync and uses_async else "") + ("nativeStatfsV1" if uses_async else "")
    nl = 'import {' + wanted.replace(",", ", ") + '} from "' + spec + '";' if spaced else 'import{' + wanted + '}from"' + spec + '";'
    if parts:
        new_line = re.sub(r'\{[^}]*\}', brace, line, count=1) + "\n" + nl
    else:
        new_line = nl
    edits.append((path, line, new_line))
    if uses_async:
        n = len(re.findall(r'(?<![A-Za-z0-9_.])statfs\(', text))
        edits.append((path, "statfs(", "nativeStatfsV1(", n))
    if uses_sync:
        n = text.count("statfsSync(")
        edits.append((path, "statfsSync(", "nativeStatfsSyncV1(", n))
    migrated.append({"path": path, "uses": "nativeStatfs" + ("SyncV1(" if uses_sync and not uses_async else "V1(")})

edits.append((LIB + "/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts", 'statfsSync:()=>({bavail:67108864,bsize:1})', 'nativeStatfsSyncV1:()=>({bavail:67108864,bsize:1})'))
new = [CONTRACT.replace("🟦️.ts", "📐️statfs/🟦️.ts"), CONTRACT.replace("🟦️.ts", "📐️statfs/🧪️tests/🟦️.ts"), CONTRACT.replace("🟦️.ts", "📐️statfs/🧪️tests/🧫️fixtures/🔣️.json")]
site_edits = edits[len(core_edits):]
fx = resolve(os.path.join(HERE, "staged", "files"), CONTRACT.replace("🟦️.ts", "📐️statfs/🧪️tests/🔒️sites/🧫️fixtures/🔣️.json"))
data = json.load(open(fx))
data["migrated"] = migrated
json.dump(data, open(fx, "w"), ensure_ascii=False, indent=2)
open(fx, "a").write("\n")
emit("g2", "virtual statfs for directory partitions: the declared envelope minus the shared measured usage, Windows host paths without statfs; the contract helpers follow", [], core_edits, new, os.path.join(HERE, "groups"))
emit("g11", "space queries of the build system ask the virtual statfs instead of the host disk below a partition root", [core_edits[0][0]], site_edits, [CONTRACT.replace("🟦️.ts", "📐️statfs/🧪️tests/🔒️sites/🟦️.ts"), CONTRACT.replace("🟦️.ts", "📐️statfs/🧪️tests/🔒️sites/🧫️fixtures/🔣️.json")], os.path.join(HERE, "groups"))
print("g2 written", len(core_edits), "core edits;", len(site_edits), "site edits;", len(migrated), "sites")
