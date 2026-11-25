import os, sys, hashlib, tempfile
ROOT = "/Users/ueli/Documents/semio/"
G = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
TAKE = G + "📥️take/"
def edit(path, pairs):
    src = open(path, encoding="utf8").read()
    before = hashlib.sha256(src.encode()).hexdigest()[:12]
    for old, new in pairs:
        assert src.count(old) == 1, (path, old[:80], src.count(old))
        src = src.replace(old, new)
    d = os.path.dirname(path)
    fd, tmp = tempfile.mkstemp(dir=d, prefix=".f43-")
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(src)
    os.chmod(tmp, os.stat(path).st_mode & 0o7777)
    os.replace(tmp, path)
    print(path.split("/")[-2], before, "->", hashlib.sha256(src.encode()).hexdigest()[:12])

# schema
edit(TAKE + "🧬️schema/🔣️.json", [
 ('"strongCores", "cpuProgressPercent"],', '"strongCores", "cpuProgressPercent", "noProgressCeilingMilliseconds", "loadStretchMaximum", "idleLoadPercentPerCore", "stuckCpuMilliseconds", "quarantineMilliseconds"],'),
 ('        "cpuProgressPercent": { "type": "integer", "minimum": 1, "maximum": 100 },\n',
  '        "cpuProgressPercent": { "type": "integer", "minimum": 1, "maximum": 100 },\n'
  '        "noProgressCeilingMilliseconds": { "type": "integer", "minimum": 1, "description": "EN: The hard ceiling of the no-progress watch: an owner silent this long is stopped however busy the host is. DE: Die harte Obergrenze der Fortschrittswache: ein so lange stiller Projekt-Eigner wird gestoppt, wie ausgelastet der Rechner auch ist." },\n'
  '        "loadStretchMaximum": { "type": "integer", "minimum": 1, "description": "EN: The most the no-progress window and the CPU progress threshold stretch with the host load per core (load average divided by cores); a starved process is not a stuck one. DE: Wie weit sich Fortschrittsfenster und CPU-Schwelle mit der Rechnerlast pro Kern höchstens dehnen; ein ausgehungerter Prozess hängt nicht." },\n'
  '        "idleLoadPercentPerCore": { "type": "integer", "minimum": 1, "description": "EN: Below this load per core (percent) the host has idle CPU, so a process without CPU use is stuck and not starved. DE: Unter dieser Last pro Kern (Prozent) hat der Rechner freie CPU, ein Prozess ohne CPU-Verbrauch hängt also und ist nicht ausgehungert." },\n'
  '        "stuckCpuMilliseconds": { "type": "integer", "minimum": 1, "description": "EN: CPU milliseconds of the work of a read between two samples above which it is alive, not stuck. DE: CPU-Millisekunden der Arbeit eines Lesens zwischen zwei Messungen, ab denen es lebt und nicht hängt." },\n'
  '        "quarantineMilliseconds": { "type": "integer", "minimum": 1, "description": "EN: How long an owner stopped as stuck stays quarantined: it runs last and alone in a child of the next pass. DE: Wie lange ein als hängend gestoppter Projekt-Eigner in Quarantäne bleibt: er läuft im nächsten Durchgang zuletzt und allein in einem Kindprozess." },\n'),
 ('"cpuProgressPercent": 10,', '"cpuProgressPercent": 10, "noProgressCeilingMilliseconds": 3600000, "loadStretchMaximum": 12, "idleLoadPercentPerCore": 70, "stuckCpuMilliseconds": 50, "quarantineMilliseconds": 7200000,'),
])

NEW = r'''
/** 📈️ How many times longer than on an idle host a process may stay silent: the load average per core (1 on or below one runnable thread per core), never above `loadStretchMaximum`; a starved process is not a stuck one.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeLoadStretchV1(load, cores, policy = TAKE_POLICY_V1) {
  const perCore = Number.isFinite(load) && Number.isFinite(cores) && cores > 0 ? load / cores : 0;
  return Math.min(policy.loadStretchMaximum, Math.max(1, perCore));
}

/** 🕰️ The window without progress on an idle host of this speed: the authored `noProgressMilliseconds` times the speed factor of this machine. */
export function takeNoProgressWindowV1() { return scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds, localMachineSpeedFactorV1()); }

/** 🌡️ The load average of the last minute and the cores of this host, the input of `takeLoadStretchV1` and `takeStopVerdictV1`. */
export function takeHostLoadV1() { return Object.freeze({ load: loadavg()[0], cores: availableParallelism() }); }

/** ⚖️ Whether a silent read stops now: never before the window stretched by the host load, always at the hard ceiling (`ceiling`), else only when the work used no CPU between two samples (below `stuckCpuMilliseconds`) while the host has idle CPU (`starved` and `alive` extend, `stuck` stops).
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeStopVerdictV1({ silentMilliseconds, windowMilliseconds, load, cores, cpuDeltaMilliseconds }, policy = TAKE_POLICY_V1) {
  if (silentMilliseconds >= policy.noProgressCeilingMilliseconds) return Object.freeze({ stop: true, reason: "ceiling" });
  if (silentMilliseconds < windowMilliseconds * takeLoadStretchV1(load, cores, policy)) return Object.freeze({ stop: false, reason: "stretched" });
  if (Number.isFinite(cpuDeltaMilliseconds) && cpuDeltaMilliseconds >= policy.stuckCpuMilliseconds) return Object.freeze({ stop: false, reason: "alive" });
  const idle = cores > 0 && load / cores * 100 < policy.idleLoadPercentPerCore;
  return idle ? Object.freeze({ stop: true, reason: "stuck" }) : Object.freeze({ stop: false, reason: "starved" });
}

/** 🚧️ The owners of the quarantine file `path` that are still quarantined at `now` (names), none where the file is missing or unreadable. */
export function takeQuarantineReadV1(path, now = Date.now(), policy = TAKE_POLICY_V1) {
  try {
    const rows = JSON.parse(readFileSync(path, "utf8"));
    return Array.isArray(rows) ? rows.filter(row => typeof row?.name === "string" && now - row.at < policy.quarantineMilliseconds).map(row => row.name) : [];
  } catch { return []; }
}

/** 🚧️ Quarantines `names` in the file `path` (atomically, temp file and rename), keeping the rows that are still valid: the next pass runs them last and alone in a child instead of in the parent. */
export function takeQuarantineWriteV1(path, names, reason, now = Date.now(), policy = TAKE_POLICY_V1) {
  try {
    let rows = [];
    try { rows = JSON.parse(readFileSync(path, "utf8")); } catch {}
    const kept = (Array.isArray(rows) ? rows : []).filter(row => typeof row?.name === "string" && now - row.at < policy.quarantineMilliseconds && !names.includes(row.name));
    const temp = path + "." + process.pid + ".tmp";
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(temp, JSON.stringify([...kept, ...names.map(name => ({ name, at: now, reason }))]));
    renameSync(temp, path);
    return true;
  } catch { return false; }
}
'''
edit(TAKE + "🟨️.mjs", [
 ('import { closeSync, constants, mkdirSync, openSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeSync } from "node:fs";\nimport { join } from "node:path";',
  'import { closeSync, constants, mkdirSync, openSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";\nimport { availableParallelism, loadavg } from "node:os";\nimport { dirname, join } from "node:path";'),
 ('/** 🔥️ A sign of CPU progress for the watch:', NEW.lstrip("\n") + '\n/** 🔥️ A sign of CPU progress for the watch:'),
 # fallback: unreadable children never hide the own CPU
 ('  return children === undefined ? undefined : children + takeOwnCpuV1(usage ?? process.cpuUsage());',
  '  return (children ?? 0) + takeOwnCpuV1(usage ?? process.cpuUsage());'),
 ('export function createCpuProgressV1({ sample, now = () => performance.now(), policy = TAKE_POLICY_V1 }) {\n  let previous, at = now(), count = 0;\n  return Object.freeze({\n    async tick() {\n      const used = await sample(), time = now();\n      if (used !== undefined && previous !== undefined && time > at && (used - previous) * 100 >= (time - at) * policy.cpuProgressPercent) count++;\n      if (used !== undefined) previous = used;\n      at = time;\n      return count;\n    },\n  });',
  'export function createCpuProgressV1({ sample, now = () => performance.now(), policy = TAKE_POLICY_V1, stretch = () => 1 }) {\n  let previous, at = now(), count = 0, delta;\n  return Object.freeze({\n    async tick() {\n      const used = await sample(), time = now();\n      delta = used !== undefined && previous !== undefined ? used - previous : undefined;\n      if (delta !== undefined && time > at && delta * 100 * Math.max(1, stretch()) >= (time - at) * policy.cpuProgressPercent) count++;\n      if (used !== undefined) previous = used;\n      at = time;\n      return count;\n    },\n    delta() { return delta; },\n  });'),
])
edit(TAKE + "🟨️.d.mts", [
 ('export function takeOwnCpuV1(', 'export function takeLoadStretchV1(load: number, cores: number, policy?: Readonly<Record<string, number>>): number;\nexport function takeNoProgressWindowV1(): number;\nexport function takeHostLoadV1(): Readonly<{ load: number; cores: number }>;\nexport function takeStopVerdictV1(input: Readonly<{ silentMilliseconds: number; windowMilliseconds: number; load: number; cores: number; cpuDeltaMilliseconds?: number }>, policy?: Readonly<Record<string, number>>): Readonly<{ stop: boolean; reason: "ceiling" | "stretched" | "alive" | "stuck" | "starved" }>;\nexport function takeQuarantineReadV1(path: string, now?: number, policy?: Readonly<Record<string, number>>): string[];\nexport function takeQuarantineWriteV1(path: string, names: readonly string[], reason: string, now?: number, policy?: Readonly<Record<string, number>>): boolean;\nexport function takeOwnCpuV1('),
])

# project-nodes
edit(G + "🟨️.mjs", [
 ('createProgressWatchV1, createTailReporterV1, takeProgressSignV1,', 'createProgressWatchV1, createTailReporterV1, takeProgressSignV1, takeStopVerdictV1, takeNoProgressWindowV1, takeHostLoadV1, takeLoadStretchV1, takeQuarantineReadV1, takeQuarantineWriteV1,'),
 ('cpuProgress=createCpuProgressV1({sample:()=>takeWorkCpuV1()})', 'cpuProgress=createCpuProgressV1({sample:()=>takeWorkCpuV1(),stretch:()=>{const host=takeHostLoadV1();return takeLoadStretchV1(host.load,host.cores);}}),quarantinePath=join(workspaceDataDirectory(workspaceRoot),"owner-quarantine.json")'),
 ('hit=watch.check(sign),quiet=tail.check(sign);\n    if(quiet!==undefined&&hit===undefined&&!stalled){',
  'silent=watch.check(sign),quiet=tail.check(sign),verdict=silent===undefined?undefined:takeStopVerdictV1({silentMilliseconds:silent.sinceMilliseconds,windowMilliseconds:takeNoProgressWindowV1(),...takeHostLoadV1(),cpuDeltaMilliseconds:cpuProgress.delta()}),hit=verdict?.stop===true?silent:undefined;\n    if(quiet!==undefined&&hit===undefined&&!stalled){'),
 ('    const fields={minutes:Math.round(hit.sinceMilliseconds/60000),',
  '    if(takeQuarantineWriteV1(quarantinePath,[...active.keys()].filter(name=>jobs.some(job=>job.name===name)),verdict.reason))reasons.push("running owners are quarantined ("+verdict.reason+"): the next pass runs them last and alone in a child / laufende Projekte sind in Quarantäne ("+verdict.reason+"): der nächste Durchgang führt sie zuletzt und allein in einem Kindprozess aus");\n    const fields={minutes:Math.round(hit.sinceMilliseconds/60000),'),
 ('  const alone=new Set(enforcement.heavy?.(jobs.map(job=>job.name))??[]);const rank=',
  '  const alone=new Set(enforcement.heavy?.(jobs.map(job=>job.name))??[]);for(const name of takeQuarantineReadV1(join(workspaceDataDirectory(workspaceRoot),"owner-quarantine.json")))alone.add(name);const rank='),
])
