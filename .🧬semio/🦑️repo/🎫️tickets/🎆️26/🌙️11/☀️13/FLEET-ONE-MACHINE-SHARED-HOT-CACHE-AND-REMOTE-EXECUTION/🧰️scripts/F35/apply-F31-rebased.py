import json,os,re,sys,hashlib
root=os.getcwd()
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P=f"{B}/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool"
D=f"{B}/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail"
S=f"{B}/⚡️caching/🚀️bootstrap/📜️script.ts"
dry="--dry" in sys.argv
def edit(path,old,new,count=1):
    text=open(path,encoding="utf8").read()
    if new in text and old not in text: print("already",path[-40:]);return
    assert text.count(old)==count,(path,old[:60],text.count(old))
    if not dry: open(path,"w",encoding="utf8").write(text.replace(old,new))
    print("edited",path[-40:])
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
files=[f"{P}/🟦️.ts",f"{P}/🧒️child/🟨️.mjs",f"{P}/🏃️runner/🟨️.mjs",f"{P}/🧪️tests/🟦️.ts",f"{D}/🟦️.ts",f"{D}/🧬️schema/🔣️.json",f"{D}/🧪️tests/🧾️summary/🟦️.ts",S]
print({f[-30:]:sha(f) for f in files})
edit(f"{P}/🟦️.ts","export async function pullOwnerPoolV1<","""/** 🕳️ How the pool files one failed owner: a native dependency without a build project owner row is the registry's gap and not the owner's fault, so it is skipped (code `ownerless`, never published as a deterministic failure note that other machines would honour after the row exists) and reported by its own code; every other failure keeps its code and determinism. */
export function ownerPoolFailureClassV1(input:Readonly<{message:string;deterministic:boolean;code?:string}>):Readonly<{code:string;deterministic:boolean;skipped:boolean}>{
 const ownerless=(input.code===undefined||input.code==="")&&/Native dependency has no build project owner: /u.test(input.message);
 return Object.freeze(ownerless?{code:"ownerless",deterministic:false,skipped:true}:{code:input.code??"",deterministic:input.deterministic,skipped:false});
}
""" + "export async function pullOwnerPoolV1<")
edit(f"{P}/🧒️child/🟨️.mjs","import { OWNER_POOL_POLICY_V1 as P, ownerPoolFitsV1, ownerPoolHeavyV1, ownerPoolRecycleV1, pullOwnerPoolV1 }","import { OWNER_POOL_POLICY_V1 as P, ownerPoolFailureClassV1, ownerPoolFitsV1, ownerPoolHeavyV1, ownerPoolRecycleV1, pullOwnerPoolV1 }")
edit(f"{P}/🧒️child/🟨️.mjs","""fail: (name, message, deterministic, code) => { write({ t: "fail", name, message: message.slice(0, 300), deterministic, code: code ?? "" }); opening.fail?.(name, message, deterministic); },""","""fail: (name, message, deterministic, code) => { const filed = ownerPoolFailureClassV1({ message, deterministic, code }); write({ t: "fail", name, message: message.slice(0, 300), deterministic: filed.deterministic, code: filed.code, skipped: filed.skipped }); opening.fail?.(name, message, filed.deterministic); },""")
edit(f"{P}/🏃️runner/🟨️.mjs","""status?.update({ lastError: `${line.name}: ${line.message}`.slice(0, 300), failures: { ...failures } }); }""","""status?.update({ lastError: `${line.skipped === true ? "skipped, no build owner row / übersprungen, keine Build-Owner-Zeile: " : ""}${line.name}: ${line.message}`.slice(0, 300), failures: { ...failures } }); }""")
edit(f"{D}/🧬️schema/🔣️.json","""            "silent": "the task printed no output at all; its exit status and signal are in the stage log"
""","""            "silent": "the task printed no output at all; its exit status and signal are in the stage log",
            "lateSignal": "[nx] {signal} reached this run after Nx had already ended with exit status {code} (failed: {tasks}); the failure is the cause and keeps its status, the signal is only reported and its sender is unknown (this run sends only SIGTERM and SIGKILL)",
            "signal": "[nx] {signal} reached this run while Nx was still running; exit status {status}; the sender is unknown (this run sends only SIGTERM and SIGKILL)"
""")
edit(f"{D}/🧬️schema/🔣️.json","""            "silent": "die Aufgabe gab gar nichts aus; Exit-Status und Signal stehen im Stufenprotokoll"
""","""            "silent": "die Aufgabe gab gar nichts aus; Exit-Status und Signal stehen im Stufenprotokoll",
            "lateSignal": "[nx] {signal} erreichte diesen Lauf, nachdem Nx schon mit Exit-Status {code} geendet hatte (fehlgeschlagen: {tasks}); der Fehlschlag ist die Ursache und behält seinen Status, das Signal wird nur gemeldet und sein Absender ist unbekannt (dieser Lauf sendet nur SIGTERM und SIGKILL)",
            "signal": "[nx] {signal} erreichte diesen Lauf, während Nx noch lief; Exit-Status {status}; der Absender ist unbekannt (dieser Lauf sendet nur SIGTERM und SIGKILL)"
""")
edit(f"{D}/🟦️.ts","/** 🛑️ {@link settleNxRunV1} that also says","""/** 🛬️ The exit status of an Nx invocation whose process was signalled: a signal that arrived only after the Nx process had ended in a failure (`cancelledAtMs` not before `closedAtMs`) is a consequence and never the cause, so the failure keeps its own status and the lines name the failed tasks and say in the declared languages that the signal came afterwards; a signal during the run keeps 130 (SIGINT) or 143, and says that its sender is unknown.
 * @see ../../../../../../../../../📜️docs */
export function nxFinalStatusV1(input:Readonly<{code:number;signal?:string;cancelledAtMs?:number;closedAtMs:number;tail:string}>,environment:Readonly<Record<string,string|undefined>>=process.env):Readonly<{status:number;lines:readonly string[]}>{
 if(input.signal===undefined)return Object.freeze({status:input.code,lines:Object.freeze([])});
 const late=input.code!==0&&(input.cancelledAtMs??Number.POSITIVE_INFINITY)>=input.closedAtMs,status=late?input.code:input.signal==="SIGINT"?130:143,tasks=nxFailureSummaryOfTailV1(input.tail)?.failed.map(row=>row.task).join(", ")||"-";
 const fill=(language:string,key:string)=>(summaryPolicy.texts as Record<string,Record<string,string>>)[language]![key]!.replace(/\\{([a-z]+)\\}/gu,(_,name:string)=>String({signal:input.signal,code:input.code,tasks,status}[name as"signal"]));
 return Object.freeze({status,lines:Object.freeze(churnLanguagesV1(environment).map(language=>fill(language,late?"lateSignal":"signal")))});
}
/** 🛑️ {@link settleNxRunV1} that also says""".replace("\n * @see ../../../../../../../../../📜️docs",""))
edit(S,"import {createNxFailureTailV1,settleNxRunReportedV1,type NxFailureTailV1}","import {createNxFailureTailV1,nxFinalStatusV1,settleNxRunReportedV1,type NxFailureTailV1}")
edit(S,"""    let cancelled: NodeJS.Signals | undefined, cancellationDeadline = 0, watchFailure = 0, finishing = false;""","""    let cancelled: NodeJS.Signals | undefined, cancellationDeadline = 0, watchFailure = 0, finishing = false, cancelledAtMs: number | undefined;""")
edit(S,"""      if (cancelled) return;
      cancelled = signal;""","""      if (cancelled) return;
      cancelled = signal;
      cancelledAtMs = Date.now();""")
edit(S,"""        if(pendingGeneration!==undefined&&code===0&&!cancelled&&!invocation.watch){""","""        const closedAtMs=Date.now();
        if(pendingGeneration!==undefined&&code===0&&!cancelled&&!invocation.watch){""")
edit(S,"""        process.exitCode=watchFailure||(cancelled?cancelled==="SIGINT"?130:143:code);""","""        const finalStatus=nxFinalStatusV1({code,signal:cancelled,cancelledAtMs,closedAtMs,tail:graphChanges.get(completed)?.output.text()??""});
        for(const line of finalStatus.lines)console.error(line);
        process.exitCode=watchFailure||(cancelled?finalStatus.status:code);""")

def append(path,marker,text):
    current=open(path,encoding="utf8").read()
    if marker in current: print("already",path[-40:]);return
    if not dry: open(path,"a",encoding="utf8").write(text)
    print("appended",path[-40:])
edit(f"{P}/🧪️tests/🟦️.ts",'import{runOwnerPoolV1}from"../🏃️runner/🟨️.mjs";','import{runOwnerPoolV1}from"../🏃️runner/🟨️.mjs";\nimport{ownerPoolFailureClassV1}from"../🟦️.ts";')
append(f"{P}/🧪️tests/🟦️.ts","ownerPoolFailureClassV1({message:text","""
test("a native dependency without a build project owner row is skipped under its own code and never filed as a deterministic failure note; every other failure keeps its code and determinism",()=>{
 const text="semio-x: Native dependency has no build project owner: a/b (required by semio-x:build; candidates=3)";
 expect(ownerPoolFailureClassV1({message:text,deterministic:true})).toEqual({code:"ownerless",deterministic:false,skipped:true});
 expect(ownerPoolFailureClassV1({message:"boom",deterministic:true,code:"unshareable"})).toEqual({code:"unshareable",deterministic:true,skipped:false});
 expect(ownerPoolFailureClassV1({message:text,deterministic:true,code:"no-record"})).toEqual({code:"no-record",deterministic:true,skipped:false});
 expect(ownerPoolFailureClassV1({message:"boom",deterministic:false})).toEqual({code:"",deterministic:false,skipped:false});
});
""")
append(f"{D}/🧪️tests/🧾️summary/🟦️.ts","nxFinalStatusV1(","""
test("a signal that reaches the run after Nx ended in a failure is reported as a consequence and the failure keeps its status; a signal during the run keeps 130 or 143; the lines are bilingual",()=>{
 const tail=fixture.cases[0]!.tail,reference=(row:{code:number;signal?:string;cancelledAtMs?:number;closedAtMs:number})=>row.signal===undefined?row.code:row.code!==0&&(row.cancelledAtMs??Infinity)>=row.closedAtMs?row.code:row.signal==="SIGINT"?130:143;
 for(const row of [{code:1,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100},{code:1,signal:"SIGINT",cancelledAtMs:50,closedAtMs:100},{code:1,signal:"SIGTERM",cancelledAtMs:50,closedAtMs:100},{code:0,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100},{code:1,closedAtMs:100}]){
  const own=nxFinalStatusV1({...row,tail},{});
  expect(own.status,JSON.stringify(row)).toBe(reference(row));
  expect(own.lines).toHaveLength(row.signal===undefined?0:2);
 }
 const late=nxFinalStatusV1({code:1,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100,tail},{});
 expect(late.lines[0]!.startsWith("[nx] SIGINT reached this run after Nx had already ended with exit status 1 (failed: ")).toBe(true);
 expect(late.lines[0]).toContain("workspace:deps-cargo:prototype");
 expect(late.lines[1]!.startsWith("[nx] SIGINT erreichte diesen Lauf, nachdem Nx schon mit Exit-Status 1 geendet hatte (fehlgeschlagen: ")).toBe(true);
 expect(nxFinalStatusV1({code:1,signal:"SIGINT",cancelledAtMs:50,closedAtMs:100,tail},{}).lines[0]).toContain("exit status 130");
});
""")
edit(f"{D}/🧪️tests/🧾️summary/🟦️.ts","nxFailureSummaryOfTailV1,settleNxRunReportedV1}","nxFailureSummaryOfTailV1,nxFinalStatusV1,settleNxRunReportedV1}")
print("done","dry" if dry else "applied")
