import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
R = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery"
CLOCK, SCHEMA = R + "/🕰️clock/🟦️.ts", R + "/🧬️schema/🔣️.json"
edits = []
edits.append((SCHEMA, '          "linux": {\n            "file": "/proc/stat",\n            "pattern": "^btime ([0-9]{9,11})$"\n          },',
 '          "linux": {\n            "file": "/proc/stat",\n            "pattern": "^btime ([0-9]{9,11})$",\n            "namespaceFile": "/proc/1/stat",\n            "clockTicksPerSecond": 100\n          },'))
old = 'const bootEpochMs=parseNativeBootEpochV1(platform,text);if(bootEpochMs===null)throw Error("Exact boot epoch text required");'
new = 'const hostEpochMs=parseNativeBootEpochV1(platform,text);if(hostEpochMs===null)throw Error("Exact boot epoch text required");const bootEpochMs=platform==="linux"?hostEpochMs+linuxNamespaceOffsetMsV1(readFileSync(epoch.linux.namespaceFile,"utf8")):hostEpochMs;'
assert live(CLOCK).count(old) == 1
edits.append((CLOCK, old, new))
edits.append((CLOCK, 'let primed:NativeBootClockV1|undefined,priming:Promise<NativeBootClockV1>|undefined;\n',
 '''let primed:NativeBootClockV1|undefined,priming:Promise<NativeBootClockV1>|undefined;

/** 🐧️ Milliseconds between the host boot and the start of process 1 of this pid namespace, read from the start tick of `/proc/1/stat`: zero on a machine, the age of the container at its start inside one, so records of a restarted container count as records of an earlier boot even though the host never rebooted. */
export function linuxNamespaceOffsetMsV1(initStat:string):number{
 const close=initStat.lastIndexOf(")"),ticks=Number(initStat.slice(close+2).trim().split(/\\s+/u)[19]);
 if(close<0||!Number.isSafeInteger(ticks)||ticks<0)throw Error("Exact init start tick required");
 return Math.round(ticks*1000/epoch.linux.clockTicksPerSecond);
}
'''))
new_files = [R + "/🕰️clock/🧪️tests/🟦️.ts", R + "/🕰️clock/🧪️tests/🧫️fixtures/🔣️.json"]
emit("g5", "boot epoch of a Linux container is the start of its pid namespace, so restarted devcontainers retire the leftovers of the previous run", [CLOCK], edits, new_files, os.path.join(HERE, "groups"))
print("g5 written")
