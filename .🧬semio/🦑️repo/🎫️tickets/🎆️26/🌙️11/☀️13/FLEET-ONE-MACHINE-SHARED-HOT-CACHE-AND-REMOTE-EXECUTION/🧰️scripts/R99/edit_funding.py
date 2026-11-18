import sys
funding_src,funding_dst,texts_src,texts_dst=sys.argv[1:5]
def edit(src,dst,pairs):
    text=open(src,encoding="utf8").read()
    for old,new in pairs:
        if text.count(old)!=1: raise SystemExit("anchor count %d for %r"%(text.count(old),old[:90]))
        text=text.replace(old,new)
    open(dst,"w",encoding="utf8").write(text)
edit(funding_src,funding_dst,[
('holderStartToleranceMilliseconds:5000,','holderStartToleranceMilliseconds:5000,heartbeatStaleMilliseconds:300000,'),
('export type DependencyStoreHolderV1=Readonly<{pid:number;command:string;sinceMs:number;mode:"shared"|"exclusive";ancestor:boolean}>;','export type DependencyStoreHolderV1=Readonly<{pid:number;command:string;sinceMs:number;silentMs?:number;mode:"shared"|"exclusive";ancestor:boolean}>;'),
('osStartedAtMs?(pid:number):number|undefined;ancestors()','osStartedAtMs?(pid:number):number|undefined;now?():number;ancestors()'),
('''    const self=entry.pid===ports.pid,holder=(ancestor:boolean):DependencyStoreHolderV1=>Object.freeze({pid:entry.pid,command:entry.command.slice(0,160),sinceMs:hold.sinceMs,mode:hold.mode,ancestor});
    if(!self&&(!ports.alive(entry.pid)||dependencyStorePidReusedV1(entry.startedAtMs,ports.osStartedAtMs?.(entry.pid)))){ports.remove(path);gone.push(holder(false));continue;}''',
'''    const self=entry.pid===ports.pid,silentMs=Number.isFinite(entry.updatedAtMs)?Math.max(0,(ports.now?.()??Date.now())-entry.updatedAtMs):0,holder=(ancestor:boolean):DependencyStoreHolderV1=>Object.freeze({pid:entry.pid,command:entry.command.slice(0,160),sinceMs:hold.sinceMs,silentMs,mode:hold.mode,ancestor});
    if(!self&&!dependencyStoreHolderPresentV1(ports.alive(entry.pid),entry.startedAtMs,ports.alive(entry.pid)?ports.osStartedAtMs?.(entry.pid):undefined,parents.has(entry.pid)?0:silentMs)){ports.remove(path);gone.push(holder(false));continue;}'''),
('''const osStartedAtMsOfProcessV1=''','''/** 🫀️ Whether a registered holder is still a holder: its process is alive, is not a later process under a reused pid, and, when the operating system could not tell its start (a loaded Windows host often times out the lookup, which left the dead owner of a reused pid in the way for hours), has renewed its registration within `heartbeatStaleMilliseconds`; a holder whose start is known and matches stays whatever its heartbeat, it only reports how long it has been silent, and an ancestor of this process is never judged by its heartbeat (it blocks in a synchronous spawn while its child waits here). */
export function dependencyStoreHolderPresentV1(alive:boolean,registeredStartedAtMs:number|undefined,osStartedAtMs:number|undefined,silentMs:number):boolean{return alive&&!dependencyStorePidReusedV1(registeredStartedAtMs,osStartedAtMs)&&!(osStartedAtMs===undefined&&silentMs>DEPENDENCY_STORE_WAIT_V1.heartbeatStaleMilliseconds);}
/** 🗝️ The text a wait line uses: no holder known, a holder that renews its registration, or one that has been silent past the heartbeat bound and may be blocked. */
export function dependencyStoreWaitKeyV1(holder?:DependencyStoreHolderV1):"dependency-store.waiting"|"dependency-store.waiting-for-holder"|"dependency-store.waiting-for-silent-holder"{return holder===undefined?"dependency-store.waiting":(holder.silentMs??0)>DEPENDENCY_STORE_WAIT_V1.heartbeatStaleMilliseconds?"dependency-store.waiting-for-silent-holder":"dependency-store.waiting-for-holder";}
/** 📝️ The key and the exact values of one wait line: only the silent text takes `silent`, because the catalog refuses a value without a placeholder. */
export function dependencyStoreWaitLineV1(seconds:number,limit:number,holder?:DependencyStoreHolderV1):Readonly<{key:ReturnType<typeof dependencyStoreWaitKeyV1>;values:Readonly<Record<string,string|number>>}>{const key=dependencyStoreWaitKeyV1(holder);return holder===undefined?{key,values:{seconds,limit}}:{key,values:{seconds,limit,...holderValuesV1(holder),...key==="dependency-store.waiting-for-silent-holder"?{silent:Math.round((holder.silentMs??0)/1000)}:{}}};}
const osStartedAtMsOfProcessV1='''),
('say:(seconds,limit,holder)=>holder?sayCachingV1("dependency-store.waiting-for-holder",{seconds,limit,...holderValuesV1(holder)}):sayCachingV1("dependency-store.waiting",{seconds,limit})','say:(seconds,limit,holder)=>{const line=dependencyStoreWaitLineV1(seconds,limit,holder);sayCachingV1(line.key,line.values);}'),
])
edit(texts_src,texts_dst,[
('''    "dependency-store.waiting-for-holder": {''','''    "dependency-store.waiting-for-silent-holder": {
      "en": "Process {pid} ({command}) holds the dependency store since {since} and has not renewed its registration for {silent} s, so it may be blocked; waiting for it ({seconds} s of at most {limit} s)",
      "de": "Der Prozess {pid} ({command}) hält den Abhängigkeitsspeicher seit {since} und hat seine Anmeldung seit {silent} s nicht erneuert, er ist womöglich blockiert; es wird darauf gewartet ({seconds} s von höchstens {limit} s)"
    },
    "dependency-store.waiting-for-holder": {'''),
])
print("ok")
