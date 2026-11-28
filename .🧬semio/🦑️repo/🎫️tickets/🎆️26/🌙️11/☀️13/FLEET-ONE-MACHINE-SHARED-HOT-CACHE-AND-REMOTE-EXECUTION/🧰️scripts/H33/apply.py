#!/usr/bin/env python3
import os, shutil, sys

ROOT = "/Users/ueli/Documents/semio/"
LIB = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FLIGHT = LIB + "🏗️build/🌐️remote-cache/🧶️nx/🚦️single-flight/"
SCRIPT = LIB + "⚡️caching/🚀️bootstrap/📜️script.ts"
TEXTS = ROOT + "🧰️framework/🔨️modules/🏃️process/📖️texts/🔣️.json"

STALL_IMPORT = '''import stallSchema from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧬️schema/🔣️.json" with {type:"json"};
'''

STALL_FN = '''
/** ⏳️ The stall period an Nx run needs while this front can park it: a task whose hash another machine computes is awaited for up to the wait budget, and Nx prints nothing and has no child process then, so the 10 minute liveness rule of the Script process would kill the run (exit 124) while its peer is still working. Returns the environment that lifts the stall period to the wait budget plus the grace; empty when the period already is that long or the front never waits.
 * @see ./🧬️schema/🔣️.json */
export function nxFlightStallEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Record<string,string>{
 const names=stallSchema.definitions.Policy.const,needed=nxFlightTimingV1(environment).waitBudgetMilliseconds+policy.stallGraceMilliseconds,asked=Number(environment[names.stallEnvironment]),current=Number.isSafeInteger(asked)&&asked>=1000?asked:names.stallMilliseconds;
 return needed>current?{[names.stallEnvironment]:String(needed)}:{};
}
'''


def edit(path, steps):
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    for old, new in steps:
        if text.count(old) != 1:
            sys.exit("anchor not unique (%d) in %s: %s" % (text.count(old), path, old[:80]))
        text = text.replace(old, new)
    backup = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig", os.path.basename(os.path.dirname(path)) + "-" + os.path.basename(path))
    os.makedirs(os.path.dirname(backup), exist_ok=True)
    if not os.path.exists(backup):
        shutil.copyfile(path, backup)
    temp = path + ".h33.tmp"
    with open(temp, "w", encoding="utf-8") as handle:
        handle.write(text)
    os.replace(temp, path)


edit(FLIGHT + "🧬️schema/🔣️.json", [
    ('        "finishPath": "/flight/finish"\n', '        "stallGraceMilliseconds": 120000,\n        "finishPath": "/flight/finish"\n'),
])

edit(FLIGHT + "🟦️.ts", [
    ('import schema from "./🧬️schema/🔣️.json" with {type:"json"};\n', 'import schema from "./🧬️schema/🔣️.json" with {type:"json"};\n' + STALL_IMPORT),
    ('const {version:_version,environment:_names,finishPath:_path,...timing}=policy,', 'const {version:_version,environment:_names,finishPath:_path,stallGraceMilliseconds:_grace,...timing}=policy,'),
    ('\nconst empty=():Response=>', STALL_FN + '\nconst empty=():Response=>'),
    ('timing?:Partial<NxFlightTimingV1>;fetch?:typeof fetch;report?:(event:NxFlightEventV1)=>void;port?:number}>;', 'timing?:Partial<NxFlightTimingV1>;fetch?:typeof fetch;report?:(event:NxFlightEventV1)=>void;waiting?:()=>()=>void;port?:number}>;'),
    ('  const served=await wait(key,request.signal);\n  return served??empty();', '  const leave=options.waiting?.()??(()=>undefined);let served:Response|undefined;\n  try{served=await wait(key,request.signal);}finally{leave();}\n  return served??empty();'),
    ('options:Readonly<{report?:(event:NxFlightEventV1)=>void;fetch?:typeof fetch;timing?:Partial<NxFlightTimingV1>}>={}', 'options:Readonly<{report?:(event:NxFlightEventV1)=>void;waiting?:()=>()=>void;fetch?:typeof fetch;timing?:Partial<NxFlightTimingV1>}>={}'),
    ('...(options.report?{report:options.report}:{}),fetch:options.fetch??remoteCacheFetchV1(environment)});', '...(options.report?{report:options.report}:{}),...(options.waiting?{waiting:options.waiting}:{}),fetch:options.fetch??remoteCacheFetchV1(environment)});'),
])

edit(SCRIPT, [
    ('Object.assign(env,flight.environment());hold.stopFlight=()=>flight.stop();', 'Object.assign(env,flight.environment(),flightModule.nxFlightStallEnvironmentV1(env));hold.stopFlight=()=>flight.stop();'),
    ('if(status!==undefined&&status!==0)throw new Error(`Owned Nx invocation failed (${status}): nx ${args.join(" ")}`);',
     'if(status!==undefined&&status!==0)throw status===124?refuseProcessV1("stalled","nx.run-stalled",{status,command:"nx "+args.join(" ")}):refuseProcessV1("failed","nx.run-failed",{status,command:"nx "+args.join(" ")});'),
    ('{report:event=>{process.stderr.write(', '{waiting:enterScriptProcessWaitV1,report:event=>{process.stderr.write('),
    ('import {processPidExistsV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\n',
     'import {processPidExistsV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\nimport {refuseProcessV1} from "../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";\nimport {enterScriptProcessWaitV1} from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";\n'),
])

edit(TEXTS, [
    ('''      "de": "Rust-Crate {crate} überschreitet vor dem Compiler seine Quellgrenze: {files} von {maxFiles} Dateien, {bytes} von {maxBytes} Bytes"
    }
''', '''      "de": "Rust-Crate {crate} überschreitet vor dem Compiler seine Quellgrenze: {files} von {maxFiles} Dateien, {bytes} von {maxBytes} Bytes"
    },
    "nx.run-stalled": {
      "en": "The Nx run ({command}) was stopped by the liveness watchdog (exit status {status}): it printed nothing for the stall period and had no live child process, for example while it waited for another machine to finish the same task; running it again is safe",
      "de": "Der Nx-Lauf ({command}) wurde vom Lebenszeichen-Wächter beendet (Exit-Status {status}): er gab während der Stillstandsfrist nichts aus und hatte keinen lebenden Kindprozess, zum Beispiel weil er auf eine andere Maschine für dieselbe Task wartete; ein erneuter Lauf ist unbedenklich"
    },
    "nx.run-failed": {
      "en": "The Nx run ({command}) failed with exit status {status}",
      "de": "Der Nx-Lauf ({command}) schlug mit Exit-Status {status} fehl"
    }
'''),
])
print("applied")
