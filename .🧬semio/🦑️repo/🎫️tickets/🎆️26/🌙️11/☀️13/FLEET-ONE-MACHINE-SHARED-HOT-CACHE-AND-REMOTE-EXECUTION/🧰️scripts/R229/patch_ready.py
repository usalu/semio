import sys
RD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
ts=f"mirror/{RD}/🟦️.ts"; js=f"mirror/{RD}/🧬️schema/🔣️.json"
s=open(ts).read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old)); s=s.replace(old,new)
rep('import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";',
 'import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";\nimport{phaseLedgerRunLinesV1}from"../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧮️phase-ledger/🟦️.ts";')
rep('/** 🪦️ The line that says a stage process vanished:',
'''/** 📏️ The measurement lines of a stage (`[phase]`, `[cargo-timings]`, `[units]`, `[nx-task]`): the ones its log holds in the last `failureWindowBytes`, else the newest ledger tables its tasks wrote since the stage started (a task that was stopped prints nothing, its ledger file does). They are printed once at the end of a run, so the tail window of the log mirror drops them under the machine rows that follow; the mirror keeps them first, the last `measurementMaximumLines` of them cut to `width`.
 * @see ../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧮️phase-ledger/🟦️.ts */
export function readyMeasurementLinesV1(log:string,root:string,sinceMs:number,width:number=P.mirrorLineCharacters):string[]{
 const patterns=P.measurementLines.map(pattern=>new RegExp(pattern,"u"));let found:string[]=[];
 try{
  const size=statSync(log).size,start=Math.max(0,size-P.failureWindowBytes),buffer=Buffer.alloc(size-start),handle=openSync(log,"r");
  try{readSync(handle,buffer,0,buffer.length,start);}finally{closeSync(handle);}
  found=buffer.toString("utf8").split(/\\r?\\n/u).filter(line=>patterns.some(pattern=>pattern.test(line)));
 }catch{}
 if(found.length===0)try{found=[...phaseLedgerRunLinesV1(root,sinceMs,(Date.now()-sinceMs)/1000)];}catch{}
 return found.slice(-P.measurementMaximumLines).map(line=>line.slice(0,width));
}

/** 🪞️ What the log mirror shows of a running or ended stage: its measurement lines, then the newest lines of the log tail that are not measurement lines. */
function readyMirrorLinesV1(log:string,root:string,sinceMs:number):string[]{
 const kept=readyMeasurementLinesV1(log,root,sinceMs),tail=logTailV1(log,P.mirrorLines,P.mirrorLineCharacters);
 if(kept.length===0)return tail;
 const patterns=P.measurementLines.map(pattern=>new RegExp(pattern,"u"));
 return[...kept,...tail.filter(line=>!patterns.some(pattern=>pattern.test(line))).slice(-(P.mirrorLines-kept.length))];
}

/** 🪦️ The line that says a stage process vanished:''')
rep('failure:[provenance(),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length,P.mirrorLineCharacters),...note]',
    'failure:[provenance(),...readyMeasurementLinesV1(inflight.log,input.root,inflight.startedMs),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length-P.measurementMaximumLines,P.mirrorLineCharacters),...note]')
rep('input.mirror?.(inflight.stage,logTailV1(inflight.log,P.mirrorLines,P.mirrorLineCharacters));','input.mirror?.(inflight.stage,readyMirrorLinesV1(inflight.log,input.root,inflight.startedMs));')
rep('input.mirror(inflight.stage,logTailV1(inflight.log,P.mirrorLines,P.mirrorLineCharacters));','input.mirror(inflight.stage,readyMirrorLinesV1(inflight.log,input.root,inflight.startedMs));')
rep('ports.mirror?.(`${row.name}-stalled`,result.tail);','ports.mirror?.(`${row.name}-stalled`,result.failure??result.tail);')
open(ts,"w").write(s)
j=open(js).read()
old='"mirrorLines": 200,'
assert j.count(old)==1
j=j.replace(old,old+'\n        "measurementMaximumLines": 80,\n        "measurementLines": ["^\\\\[phase\\\\] ", "^\\\\[cargo-timings\\\\] ", "^\\\\[units\\\\] ", "^\\\\[nx-task\\\\] "],')
open(js,"w").write(j)
