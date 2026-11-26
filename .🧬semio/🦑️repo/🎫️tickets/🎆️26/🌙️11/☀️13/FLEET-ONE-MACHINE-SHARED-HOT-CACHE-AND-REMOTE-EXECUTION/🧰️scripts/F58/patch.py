import os,sys,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1:sys.exit("anchor count %d: %s"%(s.count(old),old[:80]))
        s=s.replace(old,new)
    t=path+".f58.tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,path)
which=sys.argv[1]
if which=="observation":
    edit(B+"👁️observation/🧬️schema/🔣️.json",[
     ('        "settleRounds": 64,\n','        "settleRounds": 64,\n        "conflictRounds": 5,\n        "conflictPauseMilliseconds": 25,\n'),
     ('          "settleRounds": "The most rounds a computation is repeated while the files it read keep changing.",\n','          "settleRounds": "The most rounds a computation is repeated while the files it read keep changing.",\n          "conflictRounds": "How many times a computation that read one source in two states (a concurrent edit by the replica) is repeated after a pause before the conflict counts as genuine and fails the owner.",\n          "conflictPauseMilliseconds": "The pause before the first repetition after a conflicting read; each further repetition waits four times as long.",\n'),
     ('          "settleRounds": "Die meisten Runden, die eine Berechnung wiederholt wird, solange sich die gelesenen Dateien weiter ändern.",\n','          "settleRounds": "Die meisten Runden, die eine Berechnung wiederholt wird, solange sich die gelesenen Dateien weiter ändern.",\n          "conflictRounds": "Wie oft eine Berechnung, die eine Quelle in zwei Zuständen las (gleichzeitige Änderung durch das Replikat), nach einer Pause wiederholt wird, bevor der Konflikt als echt gilt und den Owner scheitern lässt.",\n          "conflictPauseMilliseconds": "Die Pause vor der ersten Wiederholung nach einem widersprüchlichen Lesen; jede weitere Wiederholung wartet viermal so lange.",\n'),
    ])
    edit(B+"👁️observation/🟨️.d.mts",[("settleRounds:number;","settleRounds:number;conflictRounds:number;conflictPauseMilliseconds:number;")])
    edit(B+"👁️observation/🟨️.mjs",[
     (' let previousFailure;\n for(let round=0;;round++){\n  signal?.throwIfAborted();\n  const nested=new LayeredScopeV1()',' let previousFailure,conflictRounds=0;\n for(let round=0;;round++){\n  signal?.throwIfAborted();\n  const nested=new LayeredScopeV1()'),
     ('  if(failed&&(failure?.code==="BUILD_AUTHORITY_CHANGED"||failure?.code==="BUILD_COLLECTED_ERRORS"||signature===previousFailure)){accept(rows,captured);throw failure;}\n  previousFailure=signature;\n',
      '  const conflicting=failed&&failure?.code==="BUILD_SOURCE_CONFLICT"&&conflictRounds<OBSERVATION_LIMITS_V1.conflictRounds;\n  if(failed&&(failure?.code==="BUILD_AUTHORITY_CHANGED"||failure?.code==="BUILD_COLLECTED_ERRORS"||signature===previousFailure&&!conflicting)){accept(rows,captured);throw failure;}\n  previousFailure=signature;\n  if(conflicting){\n   try{await settlementDelay(OBSERVATION_LIMITS_V1.conflictPauseMilliseconds*4**conflictRounds++,undefined,{signal});}\n   catch(error){signal?.throwIfAborted();throw error;}\n  }\n'),
    ])
else:
    edit(B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json",[
     ('"(?:refused|invalid|missing) process\\\\.source\\\\.", ','"(?:refused|invalid|missing) process\\\\.source\\\\.", "BUILD_SOURCE_CONFLICT\\\\]: Conflicting inference observations", '),
    ])
