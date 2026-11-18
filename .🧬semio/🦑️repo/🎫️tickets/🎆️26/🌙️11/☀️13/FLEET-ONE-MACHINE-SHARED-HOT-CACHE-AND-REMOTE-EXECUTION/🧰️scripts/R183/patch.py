#!/usr/bin/env python3
"""R183: anchored edits applied to a directory of files f1..f7 (f1 process-tree ts, f2 its schema, f3 containment ts, f4 its schema, f5 ready ts, f6 ready schema, f7 graph server ts). Usage: patch.py <dir>; every anchor must match exactly once."""
import sys,os,re
d=sys.argv[1]
def edit(name,old,new,count=1):
    p=os.path.join(d,name);s=open(p,encoding='utf8').read()
    if s.count(old)!=count: sys.exit(f"{name}: anchor found {s.count(old)}x, want {count}: {old[:70]!r}")
    open(p,'w',encoding='utf8').write(s.replace(old,new))

# ---------- f2: process-tree policy
edit('f2','"holderWaitPatterns": ["holds the dependency store since", "waiting for it \\\\(", "[Ww]aiting for (?:the )?(?:holder|lease)"],\n        "holderWaitRestartMilliseconds": 600000,',
'''"holderWaitPatterns": ["holds the dependency store since", "hält den Abhängigkeitsspeicher seit", "waiting for it \\\\(", "wird darauf gewartet \\\\(", "[Ww]aiting for (?:the )?(?:holder|lease)"],
        "holderNamedPatterns": [
          "Process (?<pid>\\\\d+) \\\\((?<command>.*)\\\\) holds the dependency store since (?<since>\\\\S+?)(?: and has not renewed its registration for (?<silent>\\\\d+) s, so it may be blocked)?; waiting for it \\\\((?<seconds>\\\\d+) s of at most (?<limit>\\\\d+) s\\\\)",
          "Der Prozess (?<pid>\\\\d+) \\\\((?<command>.*)\\\\) hält den Abhängigkeitsspeicher seit (?<since>\\\\S+?)(?: und hat seine Anmeldung seit (?<silent>\\\\d+) s nicht erneuert, er ist womöglich blockiert)?; es wird darauf gewartet \\\\((?<seconds>\\\\d+) s von höchstens (?<limit>\\\\d+) s\\\\)"
        ],
        "holderSilentMilliseconds": 300000,
        "holderYieldMilliseconds": 120000,
        "holderCheckMilliseconds": 10000,
        "foregroundFreshMilliseconds": 600000,
        "readRecord": [".cache", "f", "graph-server", "read-record.json"],
        "holderWaitRestartMilliseconds": 600000,''')

# ---------- f1: process-tree code
edit('f1','holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),staleStageMilliseconds',
 'holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),holderSilentMilliseconds:scaledMillisecondsV1(policy.holderSilentMilliseconds,factor),holderYieldMilliseconds:scaledMillisecondsV1(policy.holderYieldMilliseconds,factor),staleStageMilliseconds')
edit('f1','''/** 🔁️ Whether a stage that waits on a named holder is stopped to start again with the new code:''','''/** 🫱️ The holder a wait line of the dependency store names (English or German): who, since when, how long the waiter has waited, the bound it states and, when the waiter judged the holder silent, for how many seconds. */
export type ReadyHolderWaitV1=Readonly<{pid:number;command:string;since:string;silentSeconds:number|null;seconds:number;limitSeconds:number}>;

/** 🔎️ The holder the newest wait line of `lines` names, `undefined` when none does. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyHolderWaitOfV1(lines:readonly string[],policy:typeof P=P):ReadyHolderWaitV1|undefined{
 const expressions=policy.holderNamedPatterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--){
  const line=lines[index]!.replace(/\\r$/u,"");
  for(const expression of expressions){
   const groups=expression.exec(line)?.groups;
   if(groups!==undefined)return Object.freeze({pid:Number(groups.pid),command:groups.command!,since:groups.since!,silentSeconds:groups.silent===undefined?null:Number(groups.silent),seconds:Number(groups.seconds),limitSeconds:Number(groups.limit)});
  }
 }
 return undefined;
}

/** 🪪️ What the lease registry of the worker says about one holder: its pid, the start it registered, the last time it renewed its entry and its command. */
export type ReadyHolderEntryV1=Readonly<{pid:number;startedAtMs:number;updatedAtMs?:number;command:string}>;
/** ⚖️ The judgement of a wait: `progressing` (the holder lives with the start it registered and renews its entry, the stated bound is not spent), `silent` (it lives but has not renewed its entry for `holderSilentMilliseconds`, so it may be blocked), `gone` (no such process, or the pid belongs to another process: the waiter reclaims the lease by itself), `bound` (the stated bound is spent), `unknown` (the process table could not be read). */
export type ReadyHolderVerdictV1=Readonly<{kind:"progressing"|"silent"|"gone"|"bound"|"unknown";silentMs:number}>;

/** ⚖️ Judges a wait on a named holder from the process table, its registry entry and the clock: a waiter behind a live holder that renews its entry is not stalled however long it waits within the bound it states; a holder that is gone or whose pid was reused is reclaimed by the waiter and is no stall either; only a silent or a bound-spent holder is named as the problem. The liveness comes from the process table (on Windows the CIM rows with the creation time, so a reused pid has another start). A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyHolderCustodyV1(input:Readonly<{wait:ReadyHolderWaitV1;entry:ReadyHolderEntryV1|undefined;table:readonly ReadyProcessV1[];nowMs:number;policy?:typeof P}>):ReadyHolderVerdictV1{
 const policy=input.policy??P,wait=input.wait;
 if(wait.seconds>=wait.limitSeconds)return Object.freeze({kind:"bound",silentMs:0});
 if(input.table.length===0)return Object.freeze({kind:"unknown",silentMs:0});
 const identity=input.entry===undefined?(input.table.some(row=>row.pid===wait.pid)?"same":"gone"):readyIdentityV1({pid:wait.pid,startMs:input.entry.startedAtMs},input.table,policy.toleranceMilliseconds);
 if(identity!=="same")return Object.freeze({kind:"gone",silentMs:0});
 const silentMs=input.entry?.updatedAtMs!==undefined&&Number.isFinite(input.entry.updatedAtMs)?Math.max(0,input.nowMs-input.entry.updatedAtMs):(wait.silentSeconds??0)*1000;
 return Object.freeze({kind:silentMs>policy.holderSilentMilliseconds?"silent":"progressing",silentMs});
}

/** 🧾️ What the detached runner of a graph read keeps in `read-record.json`: the kind of read, the runner, the read, when the runner started, and the end status once it ended. */
export type ReadyReadRecordV1=Readonly<{reason:"request"|"refresh";runner:number;read:number|null;startedMs:number;status:number|null;endedMs:number|null}>;

/** 🧘️ The processes to stop so that a foreground stage gets the dependency store from a background reader of the machine's own graph server: the runner of the running read recorded by the graph server (alive with its recorded start, not ended) and everything below it, only when `holder` is among them. Nothing otherwise: a holder of a person, another agent or another checkout is never a victim. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyBackgroundReaderV1(input:Readonly<{record:ReadyReadRecordV1|undefined;holder:number;table:readonly ReadyProcessV1[];self:number;policy?:typeof P}>):Readonly<{root:ReadyKnownV1;plan:readonly number[];reason:"request"|"refresh"}>|undefined{
 const policy=input.policy??P,record=input.record;
 if(record===undefined||record.status!==null||record.endedMs!==null)return undefined;
 const row=input.table.find(each=>each.pid===record.runner);
 if(row===undefined||readyIdentityV1({pid:record.runner,startMs:record.startedMs},input.table,policy.toleranceMilliseconds)!=="same")return undefined;
 const root={pid:row.pid,startMs:row.startMs},plan=readyKillPlanV1({table:input.table,root,known:[],self:input.self,tolerance:policy.toleranceMilliseconds});
 return plan.includes(input.holder)&&!plan.includes(input.self)?Object.freeze({root,plan,reason:record.reason}):undefined;
}

/** 🔁️ Whether a stage that waits on a named holder is stopped to start again with the new code:''')

edit('f1','''/** 🌲️ The processes to stop, leaf first''','''/** 🏔️ Whether a worker is busy with its readiness: its `ready.json` says not ready in a state of work (running, waiting, backing off) and was written within `foregroundFreshMilliseconds`, so a loop that died never holds a graph server back for ever. The graph server of the machine neither serves nor refreshes meanwhile: readiness is the foreground, a whole-workspace read is not. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyForegroundBusyV1(status:unknown,nowMs:number,policy:typeof P=P):boolean{
 if(status===null||typeof status!=="object")return false;
 const row=status as{ready?:unknown;state?:unknown;updatedMs?:unknown};
 return row.ready===false&&(row.state==="running"||row.state==="waiting"||row.state==="backoff")&&typeof row.updatedMs==="number"&&Number.isFinite(row.updatedMs)&&nowMs-row.updatedMs<=policy.foregroundFreshMilliseconds;
}

/** 🌲️ The processes to stop, leaf first''')

# ---------- f4: containment texts
edit('f4','''          "reason-restart": "the stage waited for a holder while its code changed and was stopped as a whole to start again with the new code.",''','''          "reason-restart": "the stage waited for a holder while its code changed and was stopped as a whole to start again with the new code.",
          "reason-yield": "a stage of the readiness loop had waited {seconds} second(s) for the dependency store behind this background graph read of the graph server of this machine; the read gives way (the waves it published are kept) and the graph server reads again when the store is free.",''')
edit('f4','''          "reason-restart": "die Stufe wartete auf einen Halter, während sich ihr Code änderte, und wurde als Ganzes gestoppt, um mit dem neuen Code neu zu starten.",''','''          "reason-restart": "die Stufe wartete auf einen Halter, während sich ihr Code änderte, und wurde als Ganzes gestoppt, um mit dem neuen Code neu zu starten.",
          "reason-yield": "eine Stufe der Bereitschaftsschleife wartete seit {seconds} Sekunde(n) hinter diesem Hintergrund-Graphlesen des Graph-Servers dieser Maschine auf den Abhängigkeitsspeicher; das Lesen weicht (die veröffentlichten Wellen bleiben erhalten) und der Graph-Server liest wieder, wenn der Speicher frei ist.",''')
edit('f4','''          "survivor": "Process {pid} ({name}) is still alive after {passes} stop pass(es).",''','''          "yielded-reader": "Stage {stage} waited {seconds} s for process {pid} ({name}), which holds the dependency store for a background graph read of this machine ({reason}); the read gives way to the stage and is read again later.",
          "survivor": "Process {pid} ({name}) is still alive after {passes} stop pass(es).",''')
edit('f4','''          "survivor": "Prozess {pid} ({name}) lebt nach {passes} Stopp-Durchgang/Durchgängen noch.",''','''          "yielded-reader": "Stufe {stage} wartete {seconds} s auf Prozess {pid} ({name}), der den Abhängigkeitsspeicher für ein Hintergrund-Graphlesen dieser Maschine hält ({reason}); das Lesen weicht der Stufe und wird später erneut gelesen.",
          "survivor": "Prozess {pid} ({name}) lebt nach {passes} Stopp-Durchgang/Durchgängen noch.",''')

# ---------- f3: containment code
edit('f3','export type ReadyKillReasonV1="orphan"|"holder"|"roster"|"stall"|"restart"|"timeout"|"cancel"|"settle";','export type ReadyKillReasonV1="orphan"|"holder"|"roster"|"stall"|"restart"|"yield"|"timeout"|"cancel"|"settle";')
edit('f3','kind:"reaped-roster"|"reaped-registry"|"reaped-holder"|"survivor",fields','kind:"reaped-roster"|"reaped-registry"|"reaped-holder"|"yielded-reader"|"survivor",fields')
edit('f3','/** 🪪️ Why the stage runner stops a process: `orphan`, `holder`, `roster`, `stall`, `restart`, `timeout`, `cancel` or `settle`. */','/** 🪪️ Why the stage runner stops a process: `orphan`, `holder`, `roster`, `stall`, `restart`, `yield` (a background graph read gives way to the stage that waits behind it), `timeout`, `cancel` or `settle`. */')
edit('f3','export type ReadyRegistryEntryV1=Readonly<{pid:number;startedAtMs:number;command:string;','export type ReadyRegistryEntryV1=Readonly<{pid:number;startedAtMs:number;updatedAtMs?:number;command:string;')

# ---------- f6: ready schema (refusal and reason)
edit('f6','''          "stage-code-stale": "Stage {stage} had run for at least {minutes} minutes when the code it runs changed ({file}); its whole process tree was stopped and the stage starts again at once with the new code, everything it finished stays stored (not counted as a failure).",''','''          "stage-code-stale": "Stage {stage} had run for at least {minutes} minutes when the code it runs changed ({file}); its whole process tree was stopped and the stage starts again at once with the new code, everything it finished stays stored (not counted as a failure).",
          "stage-holder-silent": "Stage {stage} waits for process {pid} ({command}), which holds the dependency store but has not renewed its registration for {silent} seconds, so it may be blocked; the stage was stopped after {minutes} minutes without progress and starts again in {seconds} seconds (stop {count} of {maximum} in a row, not counted as a failure). Stop that process to free the store.",''')
edit('f6','''          "stage-code-stale": "Stufe {stage} lief mindestens {minutes} Minuten, als sich der Code änderte, den sie ausführt ({file}); ihr ganzer Prozessbaum wurde beendet und die Stufe startet sofort neu mit dem neuen Code, alles Fertige bleibt gespeichert (zählt nicht als Fehler).",''','''          "stage-code-stale": "Stufe {stage} lief mindestens {minutes} Minuten, als sich der Code änderte, den sie ausführt ({file}); ihr ganzer Prozessbaum wurde beendet und die Stufe startet sofort neu mit dem neuen Code, alles Fertige bleibt gespeichert (zählt nicht als Fehler).",
          "stage-holder-silent": "Stufe {stage} wartet auf Prozess {pid} ({command}), der den Abhängigkeitsspeicher hält, aber seine Anmeldung seit {silent} Sekunden nicht erneuert hat und daher womöglich blockiert ist; die Stufe wurde nach {minutes} Minuten ohne Fortschritt beendet und startet in {seconds} Sekunden neu (Stopp {count} von {maximum} in Folge, zählt nicht als Fehler). Diesen Prozess beenden, um den Speicher freizugeben.",''')
edit('f6','"stage-code-stale", "store-full"','"stage-code-stale", "stage-holder-silent", "store-full"')
edit('f6','"stalled", "code-changed", "code-stale", "adopted", "binary-missing"]}','"stalled", "code-changed", "code-stale", "holder-silent", "adopted", "binary-missing"]}')
edit('f6','"adopted": "Adopted stage {stage}','"holder-silent": "Stage {stage} waited for a named holder of the dependency store that has not renewed its registration for more than 5 minutes (it may be blocked) and was stopped; started again after the back-off.", "adopted": "Adopted stage {stage}')
edit('f6','"adopted": "Stufe {stage} übernommen','"holder-silent": "Stufe {stage} wartete auf einen benannten Halter des Abhängigkeitsspeichers, der seine Anmeldung seit mehr als 5 Minuten nicht erneuert hat (womöglich blockiert), und wurde beendet; nach der Wartezeit neu gestartet.", "adopted": "Stufe {stage} übernommen')

exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'patch2.py'),encoding='utf8').read())
