import sys
p=sys.argv[1];s=open(p,encoding="utf8").read()
def sub(a,b):
    global s
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
old_start=s.index('const STAGE_INFER_V1 = { en: "inferring: in a farm child')
old_end=s.index(', STAGE_REFS_V1 =',old_start)
s=s[:old_start]+'''const STAGE_INFER_V1 = { en: "inferring in this process (one thread: closure walks and hashing hold it, a busy machine slows it)", de: "Inferenz in diesem Prozess (ein Thread: Closure-Läufe und Hashes halten ihn, eine ausgelastete Maschine bremst ihn)" }, STAGE_FARM_V1 = { en: "waiting for a farm child to infer the owner, or for one to start (the host capacity governor holds a child while the machine lacks free memory)", de: "wartet auf einen Farm-Kindprozess, der den Owner ableitet, oder auf dessen Start (die Kapazitätssteuerung hält einen Kindprozess, solange der Maschine freier Speicher fehlt)" }, STAGE_RESUME_V1 = { en: "the farm child is done; waiting for the memory governor to let the owner back into the process", de: "der Farm-Kindprozess ist fertig; wartet darauf, dass die Speichersteuerung den Owner wieder in den Prozess lässt" }'''+s[old_end:]
sub('const stageV1 = (name, text) => { stagesV1.set(name, text); };\n','''const stageV1 = (name, text) => { stagesV1.set(name, text); };

/** 🧭️ Runs the farm preparation of one owner through `away` and names each wait of it for the progress line, in both languages: first the farm child (or its start), then the memory governor that lets the owner back into the process, then the process itself. */
export async function stagedPrepareV1(name, away, work) {
  stageV1(name, STAGE_FARM_V1);
  try { return await away(async () => { try { return await work(); } finally { stageV1(name, STAGE_RESUME_V1); } }); }
  finally { stageV1(name, STAGE_INFER_V1); }
}
''')
sub('return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));','return stagedPrepareV1(name,away,()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));')
open(p,"w",encoding="utf8").write(s)
