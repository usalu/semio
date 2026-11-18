import sys, os, shutil
R = "🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
T = R + "🟦️.ts"
J = R + "🧬️schema/🔣️.json"
EDITS = [
 (T, 'import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../🐢️speed/🟨️.mjs";\n', 'import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../🐢️speed/🟨️.mjs";\nimport{inspectTypescriptCompilerV1}from"../../../🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";\n'),
 (T, '|"stage-failed"|"stage-timeout"|"stage-stalled"|', '|"stage-failed"|"tooling-compiler"|"stage-timeout"|"stage-stalled"|'),
 (T, 'disk?(stage:string):ReadyDiskShortV1|null;\n', 'disk?(stage:string):ReadyDiskShortV1|null;compiler?():Readonly<{en:string;de:string}>|null;\n'),
 (T, '   const memory=ports.memory(),lease=ports.lease.peek();\n',
  '   if(done.dependencies!==undefined){const wrong=ports.compiler?.()??null;if(wrong!==null){const at=ports.now(),attempts=state.attempts+1,{dependencies:_dropped,...rest}=state.stages;state={...state,stages:rest,failure:{digest,stage:"dependencies",attempts,exit:null,code:"tooling-compiler",nextAttemptMs:at+readyBackoffV1(attempts,ports.jitter),atMs:at,tail:[wrong.en,wrong.de]},attempts};ports.state.write(state);delete done.dependencies;}}\n   const memory=ports.memory(),lease=ports.lease.peek();\n'),
 (T, '    const stalledOut=result.stalled&&!result.timedOut&&!result.cancelled&&!signal.aborted,outcome=result.exit===0&&!result.timedOut&&!result.spawnFailed&&!(row.name==="native"&&!ports.repoPresent())?"ok"',
  '    const wrong=row.name==="dependencies"&&result.exit===0&&!result.timedOut&&!result.spawnFailed?ports.compiler?.()??null:null;\n    const stalledOut=result.stalled&&!result.timedOut&&!result.cancelled&&!signal.aborted,outcome=result.exit===0&&!result.timedOut&&!result.spawnFailed&&!(row.name==="native"&&!ports.repoPresent())&&wrong===null?"ok"'),
 (T, 'if(result.exit===0&&!result.timedOut&&!result.spawnFailed&&!binaryMissing){', 'if(result.exit===0&&!result.timedOut&&!result.spawnFailed&&!binaryMissing&&wrong===null){'),
 (T, 'const failedTail=readyTailWithDiskCauseV1(', 'const failedTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1('),
 (T, 'code:ReadyCodeV1=stallExhausted?"stage-stall-exhausted":', 'code:ReadyCodeV1=wrong!==null?"tooling-compiler":stallExhausted?"stage-stall-exhausted":'),
 (T, '   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1}),\n',
  '   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1}),\n   compiler:()=>{const seen=inspectTypescriptCompilerV1({root:input.root});return seen.ok||(seen.refusal.code==="typescript-compiler-declaration"&&seen.refusal.found===null&&!existsSync(seen.refusal.resolved))?null:{en:seen.refusal.en,de:seen.refusal.de};},\n'),
 (J, '"memory-low", "stage-failed", "stage-timeout", "stage-stalled", "stage-stall-exhausted", "stage-code-changed", "store-full", "disk-short", "loop-failed", "spawn-failed", "interrupted", "server"]}', '"memory-low", "stage-failed", "tooling-compiler", "stage-timeout", "stage-stalled", "stage-stall-exhausted", "stage-code-changed", "store-full", "disk-short", "loop-failed", "spawn-failed", "interrupted", "server"]}'),
 (J, '          "stage-failed": "Stage {stage} failed with exit status {exit}; retrying with back-off.",\n', '          "stage-failed": "Stage {stage} failed with exit status {exit}; retrying with back-off.",\n          "tooling-compiler": "Stage {stage} ended, but the TypeScript compiler of the tooling is not the declared one: resolved path, found version and wanted version are in the last lines of the tail. Retrying with back-off; if it stays, the dependency install of this machine has to be repaired (repo dependencies sync --all).",\n'),
 (J, '          "stage-failed": "Stufe {stage} schlug mit Exit-Status {exit} fehl; neuer Versuch mit Wartezeit.",\n', '          "stage-failed": "Stufe {stage} schlug mit Exit-Status {exit} fehl; neuer Versuch mit Wartezeit.",\n          "tooling-compiler": "Stufe {stage} ist zu Ende, aber der TypeScript-Compiler der Werkzeuge ist nicht der deklarierte: aufgelöster Pfad, gefundene und verlangte Version stehen in den letzten Zeilen des Protokollausschnitts. Neuer Versuch mit Wartezeit; bleibt es so, muss die Abhängigkeitsinstallation dieser Maschine repariert werden (repo dependencies sync --all).",\n'),
]
def main(mode, base, backup):
    files = sorted({e[0] for e in EDITS})
    if mode == "apply":
        os.makedirs(backup, exist_ok=True)
        texts = {f: open(os.path.join(base, f), encoding="utf8").read() for f in files}
        for f, old, new in EDITS:
            if texts[f].count(old) != 1: print("ANCHOR", f, texts[f].count(old), old[:70]); return 3
            texts[f] = texts[f].replace(old, new)
        for f in files:
            b = os.path.join(backup, f); os.makedirs(os.path.dirname(b), exist_ok=True); shutil.copy2(os.path.join(base, f), b)
        for f in files: open(os.path.join(base, f), "w", encoding="utf8").write(texts[f])
        print("applied", len(EDITS), "edits in", len(files), "files"); return 0
    texts = {f: open(os.path.join(base, f), encoding="utf8").read() for f in files}
    for f, old, new in reversed(EDITS):
        if texts[f].count(new) != 1: print("INVERSE", f, texts[f].count(new)); return 4
        texts[f] = texts[f].replace(new, old)
    for f in files: open(os.path.join(base, f), "w", encoding="utf8").write(texts[f])
    print("restored", len(files), "files"); return 0
sys.exit(main(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else ""))
