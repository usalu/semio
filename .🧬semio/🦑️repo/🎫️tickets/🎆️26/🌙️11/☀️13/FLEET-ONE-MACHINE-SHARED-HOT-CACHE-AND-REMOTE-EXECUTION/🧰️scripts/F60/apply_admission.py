import os,re,sys
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/"
S=os.path.dirname(os.path.abspath(__file__))+"/"
def load(p):return open(p,encoding="utf8").read()
def save(p,t):
    tmp=p+".f60.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
def once(t,old,new):
    assert t.count(old)==1,(old[:80],t.count(old));return t.replace(old,new)

# ceiling
p=D+"🏔️ceiling/🟦️.ts";t=load(p)
if "decideGrantGrowthV1" not in t:
    t=once(t,"export function unitBeyondHostV1(policy: CompileBudgetPolicyV1, unit: Readonly<{ crate: string; target: string }>, learnedMegabytes: number | undefined, hostMegabytes: number): boolean {","export function unitBeyondHostV1(policy: CompileBudgetPolicyV1, unit: Readonly<{ crate: string; target: string }>, learnedMegabytes: number | undefined, hostMegabytes: number, holdableMegabytes?: number): boolean {")
    t=once(t,"  return peak > budget.finalMegabytes && hostMegabytes <= budget.hostMegabytes;\n}","  return peak > budget.finalMegabytes && hostMegabytes <= budget.hostMegabytes || holdableMegabytes !== undefined && peak > holdableMegabytes;\n}")
    t=once(t,"a machine larger than the class, and a unit never measured, answer `false`.","a machine larger than the class, and a unit never measured, answer `false`, except that a unit whose peak is above what the machine can ever hold beside its reserves (`holdableMegabytes`, see `hostHoldableMegabytesV1`) is always beyond it.")
    t=t.rstrip("\n")+"\n"+load(S+"ceiling_add.ts")
    save(p,t)

# main
p=D+"🟦️.ts";t=load(p)
if "GrantGrowthPortV1" not in t:
    t=once(t,'import { carvingOfUnitV1, unitBudgetKindV1, unitBudgetMegabytesV1, unitKeyPartsV1, type CompileBudgetKindV1, type CompileBudgetPolicyV1 } from "./🏔️ceiling/🟦️.ts";','import { carvingOfUnitV1, decideGrantGrowthV1, hostHoldableMegabytesV1, unitBudgetKindV1, unitBudgetMegabytesV1, unitKeyPartsV1, type CompileBudgetKindV1, type CompileBudgetPolicyV1, type GrowthHostV1 } from "./🏔️ceiling/🟦️.ts";')
    t=once(t,'export type {CompileBudgetClassV1,CompileBudgetKindV1,CompileBudgetPolicyV1,CompileCarvingEntryV1,CompileCarvingFamilyV1,RuntimeCeilingV1} from "./🏔️ceiling/🟦️.ts";','export type {CompileBudgetClassV1,CompileBudgetKindV1,CompileBudgetPolicyV1,CompileCarvingEntryV1,CompileCarvingFamilyV1,GrantGrowthVerdictV1,GrowthHostV1,RuntimeCeilingV1} from "./🏔️ceiling/🟦️.ts";')
    t=once(t,'export {UNIT_CEILING_POLICY_V1,carvingOfUnitV1,compileUnitOfCommandV1,runtimeCeilingOfCommandV1,unitBudgetKindV1,unitBeyondHostV1,','export {UNIT_CEILING_POLICY_V1,carvingOfUnitV1,compileUnitOfCommandV1,decideGrantGrowthV1,hostHoldableMegabytesV1,runtimeCeilingOfCommandV1,unitBudgetKindV1,unitBeyondHostV1,')
    t=once(t,'import { readHostCapacityPolicyV1 } from "../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";','import { hostFloorMegabytesV1, hostInteractiveReserveMegabytesV1, readHostCapacityPolicyV1 } from "../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";')
    anchor="/** 🌡️ The reason a compiler unit starts or waits under the host memory policy. */"
    port='''/** 📈️ What the broker needs of the host to grow a parent grant: the memory facts of this machine now (`local`, `undefined` when unknown), those of the other machines of the fleet (`fleet`, `undefined` when unknown) and `acquire`, which books `extraMegabytes` more beside the grant in the fair queue of the host capacity gate, waits at most `waitMilliseconds`, ends at once when `signal` aborts and returns the booking to give back (`release`) or why there is none.
 * @see ./🏔️ceiling/🟦️.ts (`decideGrantGrowthV1`) */
export type GrantGrowthPortV1 = Readonly<{ local: () => GrowthHostV1 | undefined; fleet: () => readonly GrowthHostV1[] | undefined; acquire: (request: Readonly<{ unit: string; extraMegabytes: number; signal: AbortSignal; waitMilliseconds: number; write: (line: string) => void }>) => Promise<Readonly<{ granted: true; release: () => void }> | Readonly<{ granted: false; reason: "timeout" | "cancelled" }>> }>;

/** 🧱️ The most memory reservations can ever hold on a machine of `totalMegabytes` under the host-capacity policy of the build (its total less the free-memory floor and the interactive reserve, times the share reservations may take): a unit whose peak is above it never runs there, whatever else is idle.
 * @see ./🏔️ceiling/🟦️.ts (`hostHoldableMegabytesV1`) */
export function localHoldableMegabytesV1(totalMegabytes: number): number {
  const policy = readHostCapacityPolicyV1(buildPolicy.hostCapacity), host = { totalMegabytes };
  return hostHoldableMegabytesV1({ totalMegabytes, floorMegabytes: hostFloorMegabytesV1(policy, host), interactiveReserveMegabytes: hostInteractiveReserveMegabytesV1(policy, host), budgetBasisPoints: policy.memory.budgetBasisPoints });
}

/** 📣️ Describes one step of a grant growth (`Messages.growth`) in the given language. */
export function formatGrowthV1(language: "en" | "de", name: keyof typeof schema.definitions.Messages.const.en.growth, values: Readonly<Record<string, string | number>>): string {
  return schema.definitions.Messages.const[language].growth[name].replace(/\\{([a-z]+)\\}/gu, (match, key: string) => key in values ? String(values[key]) : match);
}

'''
    t=once(t,anchor,port+anchor)
    t=once(t,"type Entry = { request: AdmitRequestV1;","type Entry = { grown?: Readonly<{ extraBytes: number; release: () => void }>; request: AdmitRequestV1;")
    t=once(t,"parentMemoryAllowanceBytes?: number; history: ReadonlyMap<string, readonly UnitObservationV1[]>; endpoint?: string;","parentMemoryAllowanceBytes?: number; growth?: GrantGrowthPortV1; history: ReadonlyMap<string, readonly UnitObservationV1[]>; endpoint?: string;")
    t=once(t,"reserve=options.reserveBytes??admissionReserveBytesV1(physicalBytes),parentAllowance=options.parentMemoryAllowanceBytes;\n  if(!Number.isSafeInteger(reserve)||reserve<0||reserve>physicalBytes||parentAllowance!==undefined&&(!Number.isSafeInteger(parentAllowance)||parentAllowance<0))","reserve=options.reserveBytes??admissionReserveBytesV1(physicalBytes);\n  let parentAllowance=options.parentMemoryAllowanceBytes;\n  if(!Number.isSafeInteger(reserve)||reserve<0||reserve>physicalBytes||parentAllowance!==undefined&&(!Number.isSafeInteger(parentAllowance)||parentAllowance<0))")
    # forget releases growth
    t=once(t,"""    if (index >= 0) waiting.splice(index, 1);
    running.delete(entry);
    schedule();""","""    if (index >= 0) waiting.splice(index, 1);
    running.delete(entry);
    giveBackGrowth(entry);
    schedule();""")
    t=once(t,"""  const forget = (entry: Entry | undefined): void => {""","""  const giveBackGrowth = (entry: Entry): void => {
    const grown = entry.grown;
    if (!grown) return;
    entry.grown = undefined;
    parentAllowance = Math.max(0, (parentAllowance ?? 0) - grown.extraBytes);
    try { grown.release(); } catch { return; }
  };
  const forget = (entry: Entry | undefined): void => {""")
    t=once(t,"""          running.delete(entry);
          lastActivityMs = now();
          socket.end();""","""          running.delete(entry);
          giveBackGrowth(entry);
          lastActivityMs = now();
          socket.end();""")
    # growParent closure before server
    growth='''  type GrowthOutcome = Readonly<{ kind: "grown"; extraBytes: number; release: () => void } | { kind: "refused"; reason: "beyond-host" | "no-host-fits" | "parent-memory"; allowanceBytes: number } | { kind: "cancelled" }>;
  const growParent = async (unit: string, predictedBytes: number, signal: AbortSignal, queued: () => void): Promise<GrowthOutcome> => {
    const port = options.growth!, policy = ADMISSION_POLICY_V1.growth, mebi = 1048576, allowance = parentAllowance!, held = (): GrowthOutcome => ({ kind: "refused", reason: "parent-memory", allowanceBytes: allowance });
    const safely = <T>(read: () => T): T | undefined => { try { return read(); } catch { return undefined; } };
    const say = (name: keyof typeof schema.definitions.Messages.const.en.growth, values: Readonly<Record<string, string | number>>): void => { try { options.log?.(`[admission] ${formatGrowthV1("en", name, values)} / ${formatGrowthV1("de", name, values)}`); } catch { return; } };
    const local = safely(port.local);
    if (!local) return held();
    const need = Math.ceil(predictedBytes / mebi), grant = Math.floor(allowance / mebi), base = { unit, predicted: need, grant };
    const verdict = decideGrantGrowthV1({ needMegabytes: need, grantMegabytes: grant, local, fleet: safely(port.fleet) });
    if (verdict.kind === "fits") return { kind: "grown", extraBytes: 0, release: () => undefined };
    if (verdict.kind === "beyond-host") { say("beyondHost", { ...base, holdable: verdict.holdableMegabytes, largest: verdict.largestMegabytes }); return { kind: "refused", reason: "beyond-host", allowanceBytes: verdict.holdableMegabytes * mebi }; }
    if (verdict.kind === "refused") { say("refused", { ...base, origin: verdict.largestOrigin, largest: verdict.largestMegabytes }); return { kind: "refused", reason: "no-host-fits", allowanceBytes: verdict.largestMegabytes * mebi }; }
    const extra = verdict.extraMegabytes, startedMs = now(), minutes = Math.ceil(policy.waitMilliseconds / 60000), seconds = (): number => Math.round((now() - startedMs) / 1000);
    say("asking", { ...base, extra, minutes });
    queued();
    const ticker = setInterval(() => { const facts = safely(port.local); say("waiting", { ...base, extra, available: facts?.availableMegabytes ?? "?", reserve: facts ? facts.floorMegabytes + facts.interactiveReserveMegabytes : "?", seconds: seconds() }); }, policy.progressMilliseconds);
    ticker.unref?.();
    try {
      const answer = await port.acquire({ unit, extraMegabytes: extra, signal, waitMilliseconds: policy.waitMilliseconds, write: (line) => { try { options.log?.(line); } catch { return; } } });
      if (answer.granted) { say("grown", { ...base, granted: need, seconds: seconds() }); return { kind: "grown", extraBytes: extra * mebi, release: answer.release }; }
      if (answer.reason === "cancelled") { say("cancelled", { ...base, seconds: seconds() }); return { kind: "cancelled" }; }
      say("timeout", { ...base, minutes });
      return held();
    } finally { clearInterval(ticker); }
  };
'''
    t=once(t,"  const server = createServer((socket) => {\n    let entry: Entry | undefined, finished = false, chain = Promise.resolve();\n    socket.on(\"error\", () => undefined);\n",growth+"  const server = createServer((socket) => {\n    let entry: Entry | undefined, finished = false, chain = Promise.resolve();\n    const gone = new AbortController();\n    socket.on(\"error\", () => undefined);\n    socket.once(\"close\", () => gone.abort());\n")
    old='''          const fleetOnly=pass.placement==="fleet-only",allowanceBytes=fleetOnly?0:parentAllowance;
          if (allowanceBytes!==undefined&&(fleetOnly||predictedBytes>allowanceBytes)) {
            finished = true;
            lastActivityMs = now();
            if(fleetOnly)placementFailures.add(pass.id);
            const refusal=admitDefinition("RefusalV1",{v:1,op:"refused",reason:fleetOnly?"fleet-placement":"parent-memory",predictedBytes,allowanceBytes});
            if (!socket.destroyed) socket.end(JSON.stringify(refusal)+"\\n");
            return;
          }
          entry = { request,'''
    new='''          const fleetOnly=pass.placement==="fleet-only";
          let grown: Entry["grown"], refusal: Readonly<{ reason: "parent-memory" | "fleet-placement" | "beyond-host" | "no-host-fits"; allowanceBytes: number }> | undefined;
          if (!fleetOnly&&options.growth&&parentAllowance!==undefined&&parentAllowance>0&&predictedBytes>parentAllowance) {
            const outcome = await growParent(request.crate === buildScriptCrateV1 ? key : request.crate, predictedBytes, gone.signal, () => verdict(socket, false));
            if (outcome.kind === "cancelled") { finished = true; socket.destroy(); return; }
            if (outcome.kind === "refused") refusal = outcome;
            else if (gone.signal.aborted) { outcome.release(); finished = true; return; }
            else { grown = { extraBytes: outcome.extraBytes, release: outcome.release }; parentAllowance = (parentAllowance ?? 0) + outcome.extraBytes; }
          }
          const allowanceBytes=fleetOnly?0:refusal?.allowanceBytes??parentAllowance;
          if (allowanceBytes!==undefined&&(fleetOnly||refusal!==undefined||predictedBytes>allowanceBytes)) {
            finished = true;
            lastActivityMs = now();
            if(fleetOnly)placementFailures.add(pass.id);
            const wire=admitDefinition("RefusalV1",{v:1,op:"refused",reason:fleetOnly?"fleet-placement":refusal?.reason??"parent-memory",predictedBytes,allowanceBytes});
            if (!socket.destroyed) socket.end(JSON.stringify(wire)+"\\n");
            return;
          }
          entry = { grown, request,'''
    t=once(t,old,new)
    save(p,t)

# rust
p=D+"🦀️.rs";t=load(p)
if '"beyond-host"' not in t:
    t=once(t,'''                "fleet-placement" => Some("fleet-only''','''                "beyond-host" => Some(format!("compiler needs an estimated {} MiB but this host can hold at most {} MiB beside its reserves; the unit belongs to a larger machine of the fleet and is queued there, never compiled here / Der Compiler benötigt geschätzt {} MiB, dieser Host kann neben seinen Reserven höchstens {} MiB halten; die Einheit gehört auf eine größere Maschine der Flotte, wird dort eingereiht und hier nie kompiliert", predicted.div_ceil(mib), allowance / mib, predicted.div_ceil(mib), allowance / mib)),
                "no-host-fits" => Some(format!("compiler needs an estimated {} MiB but the largest host of the fleet can hold at most {} MiB; no machine can run this unit / Der Compiler benötigt geschätzt {} MiB, der größte Host der Flotte kann höchstens {} MiB halten; keine Maschine kann diese Einheit ausführen", predicted.div_ceil(mib), allowance / mib, predicted.div_ceil(mib), allowance / mib)),
                "fleet-placement" => Some("fleet-only''')
    t=once(t,'''        assert_eq!(decode_verdict(r#"{"v":1,"op":"refused","reason":"parent-memory","predictedBytes":0,"allowanceBytes":wrong}"#)''','''        let beyond = decode_verdict(r#"{"v":1,"op":"refused","reason":"beyond-host","predictedBytes":5368709120,"allowanceBytes":4294967296}"#).unwrap_err();
        assert_eq!(beyond.kind(), std::io::ErrorKind::PermissionDenied);
        assert!(beyond.to_string().contains("5120 MiB") && beyond.to_string().contains("4096 MiB") && beyond.to_string().contains("never compiled here"));
        let none = decode_verdict(r#"{"v":1,"op":"refused","reason":"no-host-fits","predictedBytes":5368709120,"allowanceBytes":4294967296}"#).unwrap_err();
        assert_eq!(none.kind(), std::io::ErrorKind::PermissionDenied);
        assert!(none.to_string().contains("no machine can run this unit"));
        assert_eq!(decode_verdict(r#"{"v":1,"op":"refused","reason":"parent-memory","predictedBytes":0,"allowanceBytes":wrong}"#)''')
    save(p,t)
print("applied")
