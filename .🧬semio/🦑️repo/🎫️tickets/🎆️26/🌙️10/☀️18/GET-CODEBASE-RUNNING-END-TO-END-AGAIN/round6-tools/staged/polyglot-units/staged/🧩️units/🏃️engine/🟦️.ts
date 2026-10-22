import {existsSync, mkdirSync, readFileSync, renameSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {sha256V1, interfaceClosureV1, outputClosureV1, unitKeyV1, type UnitResultFactsV1} from "../🔏️key/🟦️.ts";
import {unitsSchemaFailuresV1, type UnitPlanV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";

/** 🧾️ What a finished unit records; replayed on every hit. Failing results are cached too. */
export type UnitReceiptV1 = {readonly version: 1; readonly key: string; readonly unit: string; readonly ok: boolean; readonly diagnostics: string; readonly outputs: string; readonly interface: string | null; readonly exposes: readonly string[] | null; readonly milliseconds: number};
/** 💾️ Shared content-addressed receipts plus the latest facts per unit (for the cutoff statistic). Writers never lock: two sessions at worst compute the same key twice. */
export type UnitsStoreV1 = {
  readonly receipt: (key: string) => UnitReceiptV1 | undefined;
  readonly put: (receipt: UnitReceiptV1) => void;
  readonly latest: (unit: string) => {readonly interfaceClosure: string | null; readonly outputClosure: string} | undefined;
  readonly setLatest: (unit: string, facts: {readonly interfaceClosure: string | null; readonly outputClosure: string}) => void;
};
/** 🧠️ An in-memory store (tests, one-shot plans). */
export function memoryUnitsStoreV1(): UnitsStoreV1 & {readonly receipts: Map<string, UnitReceiptV1>} {
  const receipts = new Map<string, UnitReceiptV1>(), latest = new Map<string, {interfaceClosure: string | null; outputClosure: string}>();
  return {receipts, receipt: key => receipts.get(key), put: receipt => {receipts.set(receipt.key, receipt);}, latest: unit => latest.get(unit), setLatest: (unit, facts) => {latest.set(unit, {...facts});}};
}
const atomicWrite = (path: string, text: string): void => {mkdirSync(dirname(path), {recursive: true}); const temporary = `${path}.${process.pid}.${Date.now()}.${Math.random().toString(36).slice(2)}.tmp`; writeFileSync(temporary, text); renameSync(temporary, path);};
/** 💽️ A file store: `receipts/<aa>/<key>.json` and `latest/<24 hex of unit id>.json`, written by temporary file and rename. A corrupt or schema-violating file is no receipt. */
export function fileUnitsStoreV1(root: string): UnitsStoreV1 {
  const receiptPath = (key: string): string => join(root, "receipts", key.slice(0, 2), key + ".json"), latestPath = (unit: string): string => join(root, "latest", sha256V1(unit).slice(0, 24) + ".json");
  return {
    receipt: key => {
      const path = receiptPath(key);
      if (!existsSync(path)) return undefined;
      try {const value = JSON.parse(readFileSync(path, "utf8")) as UnitReceiptV1; return unitsSchemaFailuresV1("Receipt", value).length === 0 && value.key === key ? value : undefined;} catch {return undefined;}
    },
    put: receipt => atomicWrite(receiptPath(receipt.key), JSON.stringify(receipt)),
    latest: unit => {try {return JSON.parse(readFileSync(latestPath(unit), "utf8")) as {interfaceClosure: string | null; outputClosure: string};} catch {return undefined;}},
    setLatest: (unit, facts) => atomicWrite(latestPath(unit), JSON.stringify(facts)),
  };
}

export type ExecuteRequestV1 = {readonly unit: UnitV1; readonly key: string; readonly dependencies: readonly {readonly unit: string; readonly use: "interface" | "implementation" | "both" | "declared"; readonly interfaceClosure: string | null; readonly outputClosure: string; readonly artifact?: string | null}[]; readonly signal: AbortSignal};
/** 🏭️ What the language executor reports: success, diagnostics replayed on hits, the digest of all outputs and the digest of the interface artifact (null for a unit without one). */
export type ExecuteOutcomeV1 = {readonly ok: boolean; readonly diagnostics: string; readonly outputs: string; readonly interface: string | null; readonly exposes?: readonly string[] | null; readonly artifact?: string | null; readonly milliseconds?: number};
/** 🧰️ The effects a run needs: content digests of workspace files and the language executor. The repository wires its stat-keyed digest memo and process runner here. */
export type UnitsHostV1 = {readonly digest: (path: string) => string; readonly execute: (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1>};
export type UnitStatusV1 = "hit" | "ran" | "failed" | "blocked" | "cancelled";
export type UnitsProgressV1 = {readonly unit: string; readonly status: UnitStatusV1; readonly cutoff: boolean; readonly done: number; readonly total: number; readonly milliseconds: number};
export type UnitsRowV1 = {readonly id: string; readonly key: string | null; readonly status: UnitStatusV1; readonly cutoff: boolean; readonly milliseconds: number; readonly diagnostics: string};
export type UnitsRunResultV1 = {readonly units: readonly UnitsRowV1[]; readonly ran: number; readonly hit: number; readonly cutoff: number; readonly failed: number; readonly blocked: number; readonly cancelled: boolean};

/** 💬️ Looks up an EN/DE text of the policy and fills `{name}` placeholders. */
export function unitsMessageV1(policy: UnitsPolicyV1, language: string, name: keyof UnitsPolicyV1["messages"]["texts"], values: Readonly<Record<string, string | number>> = {}): string {
  const texts = policy.messages.texts[name], text = texts[language] ?? texts[policy.messages.languages[0]!] ?? "";
  return text.replace(/\{(\w+)\}/g, (_, key: string) => String(values[key] ?? ""));
}

/** 🚀️ Runs the units of a plan in dependency order with bounded concurrency. A unit whose key has a receipt replays it; otherwise the language executor runs once. A dependent's key folds only the interface closure of the dependencies it compiles against, so a unit that is rebuilt with an unchanged interface leaves every dependent a hit (early cutoff). Dependents of a failed unit run against its interface when it has one, otherwise they are blocked. */
export async function runUnitsV1(plan: UnitPlanV1, policy: UnitsPolicyV1, host: UnitsHostV1, store: UnitsStoreV1, options: {readonly concurrency: number; readonly only?: readonly string[]; readonly signal: AbortSignal; readonly onProgress?: (row: UnitsProgressV1) => void}): Promise<UnitsRunResultV1> {
  const needed = new Set<string>(), need = (id: string): void => {if (needed.has(id)) return; needed.add(id); for (const dependency of plan.byId.get(id)!.dependencies) need(dependency.unit);};
  for (const id of options.only ?? plan.order) {if (!plan.byId.has(id)) throw Error("unknown unit " + id); need(id);}
  const order = plan.order.filter(id => needed.has(id)), rows = new Map<string, UnitsRowV1>(), settled = new Map<string, Promise<{facts: UnitResultFactsV1; ok: boolean} | undefined>>();
  let finished = 0, running = 0;
  const waiting: (() => void)[] = [], slot = async (): Promise<void> => {while (running >= Math.max(1, options.concurrency)) await new Promise<void>(resolve => waiting.push(resolve)); running++;}, release = (): void => {running--; waiting.shift()?.();};
  const record = (row: UnitsRowV1): void => {rows.set(row.id, row); finished++; options.onProgress?.({unit: row.id, status: row.status, cutoff: row.cutoff, done: finished, total: order.length, milliseconds: row.milliseconds});};
  const run = (id: string): Promise<{facts: UnitResultFactsV1; ok: boolean} | undefined> => settled.get(id) ?? settled.set(id, (async () => {
    const unit = plan.byId.get(id)!, results = await Promise.all(unit.dependencies.map(async dependency => [dependency, await run(dependency.unit)] as const));
    const blockedBy = results.find(([dependency, result]) => dependency.use !== "declared" && (result === undefined || (!result.ok && (dependency.use === "implementation" || dependency.use === "both" || result.facts.interfaceClosure === null))));
    if (blockedBy) {record({id, key: null, status: blockedBy[1] === undefined && options.signal.aborted ? "cancelled" : "blocked", cutoff: false, milliseconds: 0, diagnostics: blockedBy[0].unit}); return undefined;}
    if (options.signal.aborted) {record({id, key: null, status: "cancelled", cutoff: false, milliseconds: 0, diagnostics: ""}); return undefined;}
    const facts = new Map<string, UnitResultFactsV1>(results.filter(([, result]) => result !== undefined).map(([dependency, result]) => [dependency.unit, result!.facts] as const));
    const key = unitKeyV1(unit, {digest: host.digest, dependencies: facts});
    let receipt = store.receipt(key), status: UnitStatusV1 = "hit", cutoff = false;
    if (!receipt) {
      await slot();
      try {
        if (options.signal.aborted) {record({id, key, status: "cancelled", cutoff: false, milliseconds: 0, diagnostics: ""}); return undefined;}
        const started = performance.now();
        const outcome = await host.execute({unit, key, signal: options.signal, dependencies: unit.dependencies.map(dependency => ({unit: dependency.unit, use: dependency.use, interfaceClosure: facts.get(dependency.unit)?.interfaceClosure ?? null, outputClosure: facts.get(dependency.unit)?.outputClosure ?? ""}))});
        if (options.signal.aborted) {record({id, key, status: "cancelled", cutoff: false, milliseconds: 0, diagnostics: ""}); return undefined;}
        if (unit.interface && outcome.ok && outcome.interface === null) throw Error(`unit ${id} declares an interface but its executor reported none`);
        if (!unit.interface && outcome.interface !== null) throw Error(`unit ${id} has no interface but its executor reported one`);
        receipt = {version: 1, key, unit: id, ok: outcome.ok, diagnostics: outcome.diagnostics, outputs: outcome.outputs, interface: unit.interface ? outcome.interface : null, exposes: outcome.exposes ?? null, milliseconds: Math.round(outcome.milliseconds ?? performance.now() - started)};
        store.put(receipt);
        status = receipt.ok ? "ran" : "failed";
      } finally {release();}
    } else if (!receipt.ok) status = "failed";
    const own: UnitResultFactsV1 = {interfaceClosure: receipt.interface === null ? null : interfaceClosureV1(unit, receipt.interface, facts, policy, receipt.exposes), outputClosure: outputClosureV1(unit, receipt.outputs, facts)};
    const before = store.latest(id);
    if (status === "ran") cutoff = before !== undefined && own.interfaceClosure !== null && before.interfaceClosure === own.interfaceClosure;
    if (before === undefined || before.interfaceClosure !== own.interfaceClosure || before.outputClosure !== own.outputClosure) store.setLatest(id, own);
    record({id, key, status, cutoff, milliseconds: status === "hit" ? 0 : receipt.milliseconds, diagnostics: receipt.diagnostics});
    return {facts: own, ok: receipt.ok};
  })()).get(id)!;
  await Promise.all(order.map(run));
  const list = order.map(id => rows.get(id)!), count = (status: UnitStatusV1): number => list.filter(row => row.status === status).length;
  return {units: list, ran: count("ran"), hit: count("hit"), cutoff: list.filter(row => row.cutoff).length, failed: count("failed"), blocked: count("blocked"), cancelled: options.signal.aborted};
}
