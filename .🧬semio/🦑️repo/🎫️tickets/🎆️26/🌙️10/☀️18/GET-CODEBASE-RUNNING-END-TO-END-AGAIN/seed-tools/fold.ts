import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { projects, resolveAll } from "./graph.ts";

const R = "/Users/ueli/Documents/semio/";
const T = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/";
const OUT = T + "🗑️generated/seed-run/";
const LIB = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/";
const catalog = await import(LIB + "🚀️launch-catalog/🟦️.ts");
const routes = await import(LIB + "📦️crate-tree/🚀️launch-routes/🟦️.ts");
const protocolCrate = JSON.parse(readFileSync(LIB + "📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json", "utf8")).definitions.Protocol.const;

type E = Record<string, any>;
const canon = (v: any): string => v === undefined ? "U" : Array.isArray(v) ? "[" + v.map(canon).join(",") + "]" : v && typeof v === "object" ? "{" + Object.keys(v).sort().map((k) => JSON.stringify(k) + ":" + canon(v[k])).join(",") + "}" : JSON.stringify(v);
const stripOrder = (e: E) => canon({ ...e, presentation: e.presentation ? { group: e.presentation.group } : undefined });
const stripName = (e: E) => canon({ ...e, name: undefined });

const lText = readFileSync(R + ".vscode/launch.json", "utf8");
const sText = readFileSync(R + ".vscode/🧩️launch.seed.jsonc", "utf8");
const hash = (t: string) => createHash("sha256").update(t).digest("hex").slice(0, 16);
export const sources = { launch: hash(lText), seed: hash(sText) };
const L = Bun.JSONC.parse(lText) as { version: string; configurations: E[]; compounds: E[]; inputs: E[] };
const S = Bun.JSONC.parse(sText) as { configurations: (E | string)[]; compounds: E[]; inputs: E[]; devLaunchers: Record<string, any> };
const registry = catalog.readPlaygroundRegistry(R);
const log: Record<string, any> = { sources, repairs: {} as Record<string, number>, dropped: [] as { name: string; reason: string }[], notes: [] as string[] };
const note = (s: string) => log.notes.push(s);

const slug = (t: string) => t.replace(/^(?:[0-9#*]\uFE0F?\u20E3|\p{Extended_Pictographic}|\p{Emoji_Presentation}|\p{Emoji_Modifier}|\uFE0F|\u200D)+/u, "");
const inputsAllRaw: E[] = [...L.inputs, ...S.inputs.filter((i) => i.id && !L.inputs.some((j) => j.id === i.id))];
const inputsAll: E[] = inputsAllRaw.map((input) => {
  const m = /^projectTarget\.(.+)$/.exec(input.id);
  return m && Array.isArray(input.options) ? { ...input, options: input.options.filter((p: string) => [...(projects.get(p)?.targets ?? [])].some((t) => slug(t) === m[1])) } : input;
});

const dashboardRepair = (e: E): E | "drop" | null => {
  const m = /^bun repo run @semio-tech\/repo-cli-rs:(run|daemon|install|preferences|workflow|test)\b(.*)$/s.exec(e.command ?? "");
  if (!m) return null;
  const [, target, rest] = m as unknown as [string, string, string];
  const dashboardRow = e.name.includes("🎛️dashboard");
  if (target === "workflow") return "drop";
  if (dashboardRow) return { ...e, command: `bun repo run @semio-tech/repo-dashboard-rs:${target}${rest}` };
  if (target === "run") return { ...e, command: `bun repo run @semio-tech/repo-cli-rs:repo${rest}` };
  return null;
};
const repairs: ((e: E) => E | "drop" | null)[] = [
  dashboardRepair,
  (e) => (/^bun repo run-many -t describe --projects=@semio-tech\/(process|sourcing)-extension-/.test(e.command ?? "") ? { ...e, command: e.command.replace("run-many -t describe", "run-many -t package") } : null),
  (e) => (e.command === "bun repo run @semio-tech/repo-lib:lint-framework-module-product-direction" ? { ...e, command: "bun repo run @semio-tech/repo-lib:lint-dependency-direction" } : null),
];

const staleTargets = new Set<string>();
const live = (e: E, inputs: E[]): string | null => {
  const rs = resolveAll([e], inputs);
  for (const r of rs) {
    if (r.status === "project-missing" && !/[${*"]/.test(String(r.project))) return `project-missing ${r.project}`;
    if (r.status === "target-missing") return `target-missing ${r.project}:${r.target}`;
  }
  const sub = (s: string) => s.replaceAll("${workspaceFolder}", R.slice(0, -1));
  if (typeof e.cwd === "string") { const p = sub(e.cwd); if (p.startsWith("/") && !existsSync(p)) return `cwd-missing ${e.cwd.replace("${workspaceFolder}/", "").slice(-80)}`; }
  const cmd: string = e.command ?? "";
  for (const m of cmd.matchAll(/\b(?:bun|python3?)\s+"?(\$\{workspaceFolder\}\/[^"\s]+\.(?:ts|mjs|js|py|sh|tsx))"?/g)) if (!existsSync(sub(m[1]!))) return `script-missing ${m[1]!.replace("${workspaceFolder}/", "").slice(-90)}`;
  return null;
};

const repairEntry = (e: E): { entry: E | null; reason?: string } => {
  let cur = e;
  for (const rule of repairs) {
    const next = rule(cur);
    if (next === "drop") return { entry: null, reason: "no successor target (dashboard workflow)" };
    if (next) { log.repairs[rule === dashboardRepair ? "repo-cli-rs->dashboard/repo" : rule.toString().slice(0, 40)] = (log.repairs[rule === dashboardRepair ? "repo-cli-rs->dashboard/repo" : rule.toString().slice(0, 40)] ?? 0) + 1; cur = next; }
  }
  const reason = live(cur, inputsAll);
  return reason ? { entry: null, reason } : { entry: cur };
};

const devRenderers = [{ id: "react", icon: "⚛️react", orderKey: "order", portKey: "react" }, { id: "wgpu", icon: "🧊️wgpu🌐️wasm", orderKey: "wgpuOrder", portKey: "wgpu" }] as const;
const environment = { BUN_RUNTIME_TRANSPILER_CACHE_PATH: "0", NODE_DISABLE_COMPILE_CACHE: "1" };

const byName = new Map<string, E[]>();
for (const c of L.configurations) byName.set(c.name, [...(byName.get(c.name) ?? []), c]);

const devLaunchers: Record<string, any> = {};
const devSlot = new Map<string, string>();
const matchesGenerated = (row: any, rend: (typeof devRenderers)[number], entry: E, launcher: any, expectedName?: string) => {
  const generated = catalog.expandLaunchCatalog({ version: "0.2.0", configurations: [`@generated:${row.variant}:${rend.id}`], compounds: [], inputs: [], devLaunchers: { [row.variant]: { namePrefix: entry.name.slice("🛠️dev".length, entry.name.length - rend.icon.length), order: entry.presentation.order, wgpuOrder: entry.presentation.order, ...(launcher.env ? { env: launcher.env } : {}) } }, crateRoutes: { subjects: [] } }, registry).configurations[0];
  return canon(generated) === canon(entry);
};
for (const row of registry) {
  const curated = S.devLaunchers[row.variant];
  let launcher: any = undefined;
  for (const rend of devRenderers) {
    let found: E | undefined;
    if (curated && (rend.id === "react" || curated.wgpuOrder !== undefined)) found = (byName.get(`🛠️dev${curated.namePrefix}${rend.icon}`) ?? [])[0];
    if (!found) {
      const cands = L.configurations.filter((c) => typeof c.command === "string" && c.command.startsWith(`bun repo run @semio-tech/framework-os-dev:dev-${row.variant}-${rend.id}-prototype `) && c.env?.SEMIO_PLUGIN === row.variant && c.name.startsWith("🛠️dev") && c.name.endsWith(rend.icon) && c.presentation?.group === "3_dev" && matchesGenerated(row, rend, c, curated ?? {}));
      cands.sort((a, b) => a.name.length - b.name.length);
      found = cands[0];
      if (cands.length > 1) note(`dev ${row.variant} ${rend.id}: ${cands.length} identical candidates, took ${cands[0]!.name}`);
    }
    if (!found) continue;
    const prefix = found.name.slice("🛠️dev".length, found.name.length - rend.icon.length);
    launcher ??= { namePrefix: prefix, order: undefined };
    if (launcher.namePrefix !== prefix) { note(`dev ${row.variant}: react prefix ${launcher.namePrefix} != ${rend.id} prefix ${prefix}`); }
    if (rend.id === "react") launcher.order = found.presentation.order;
    else launcher.wgpuOrder = found.presentation.order;
    devSlot.set(found.name, `@generated:${row.variant}:${rend.id}`);
  }
  if (launcher) {
    if (launcher.order === undefined) { note(`dev ${row.variant}: no react entry`); }
    if (curated?.env) launcher.env = curated.env;
    devLaunchers[row.variant] = launcher;
  }
}
const usersFor = S.devLaunchers["s"]?.users;
if (usersFor && devLaunchers["s"]) {
  devLaunchers["s"].users = { namePrefixPattern: "🪐️space👤️{N}", env: usersFor.env };
  const generated = catalog.expandLaunchCatalog({ version: "0.2.0", configurations: ["@generated:s:users"], compounds: [], inputs: [], devLaunchers: { s: devLaunchers["s"] }, crateRoutes: { subjects: [] } }, registry).configurations;
  let firstUser = true;
  for (const g of generated) {
    const hit = (byName.get(g.name) ?? [])[0];
    if (!hit) { note(`users: ${g.name} absent in launch`); continue; }
    if (canon(hit) !== canon(g)) note(`users: ${g.name} differs from launch`);
    devSlot.set(g.name, firstUser ? "@generated:s:users" : "");
    firstUser = false;
  }
}
for (const row of registry) if (!devLaunchers[row.variant]) note(`dev ${row.variant}: no entries in launch (new variant)`);
const unmatched = [...devSlot].length;
note(`dev slots ${unmatched}`);

const famName = /^🦀️ (\S+) (🏗️build|🔎️check|🧪️test) (🛠️prototype|🚀️release)$/u;
const famCommand = /^bun repo run (\S+):(build|check|test) --configuration=(prototype|release) --parallel=1 -- (.*)$/u;
const family = new Map<string, E[]>();
for (const c of L.configurations) {
  const n = famName.exec(c.name ?? ""), m = famCommand.exec(c.command ?? "");
  if (n && m) family.set(n[1]!, [...(family.get(n[1]!) ?? []), c]);
}
const subjects: any[] = [];
const subjectEntries = new Set<E>();
let literalFamily = 0;
for (const [crate, entries] of family) {
  const project = famCommand.exec(entries[0]!.command)![1]!;
  const targetsPresent = [...new Set(entries.map((e) => famCommand.exec(e.command)![2]!))];
  const argsOf = (t: string) => [...new Set(entries.filter((e) => famCommand.exec(e.command)![2] === t).map((e) => famCommand.exec(e.command)![4]!))];
  if (targetsPresent.some((t) => argsOf(t).length !== 1)) { literalFamily += entries.length; note(`crate ${crate}: ambiguous arguments, kept literal`); continue; }
  const shapes = Object.entries(protocolCrate.shapes) as [string, Record<string, string[]>][];
  let best: { shape: string; hits: number } | undefined;
  for (const [shape, args] of shapes) {
    const hits = targetsPresent.filter((t) => args[t]!.join(" ") === argsOf(t)[0]).length;
    if (!best || hits > best.hits) best = { shape, hits };
  }
  const subject: any = { name: crate };
  if (project !== crate) subject.project = project;
  subject.shape = best!.shape;
  if (targetsPresent.length !== 3) subject.targets = ["build", "check", "test"].filter((t) => targetsPresent.includes(t));
  const overrides: Record<string, string[]> = {};
  for (const t of targetsPresent) if ((protocolCrate.shapes[best!.shape][t] as string[]).join(" ") !== argsOf(t)[0]) overrides[t] = argsOf(t)[0]!.split(" ");
  if (Object.keys(overrides).length) subject.arguments = overrides;
  const regenerated = routes.crateLaunchRoutes([subject], routes.nextLaunchRouteOrders([]));
  const a = entries.map(stripOrder).sort().join("|"), b = regenerated.map(stripOrder).sort().join("|");
  if (a !== b) { literalFamily += entries.length; note(`crate ${crate}: regeneration differs, kept literal`); continue; }
  subjects.push(subject);
  for (const e of entries) subjectEntries.add(e);
}
note(`crate subjects ${subjects.length} covering ${subjectEntries.size} entries; literal family entries ${literalFamily}`);

const items: (E | string)[] = [];
const droppedRows: { name: string; reason: string }[] = log.dropped;
let crateAnchor = false;
const usersEmitted = new Set<string>();
const dashboardTicketOk = new Map<string, boolean>();
for (const c of L.configurations) {
  if (subjectEntries.has(c)) { if (!crateAnchor) { items.push("@generated:crate-routes"); crateAnchor = true; } continue; }
  const slot = devSlot.get(c.name);
  if (slot !== undefined) {
    if (slot === "" ) continue;
    if (slot === "@generated:s:users") { if (usersEmitted.has(slot)) continue; usersEmitted.add(slot); }
    items.push(slot);
    continue;
  }
  const { entry, reason } = repairEntry(c);
  if (!entry) { droppedRows.push({ name: c.name, reason: reason! }); continue; }
  items.push(entry);
}
if (!crateAnchor && subjects.length) items.push("@generated:crate-routes");

const lastDev = items.reduce((at, item, index) => (typeof item === "string" && /^@generated:[^:]+:(react|wgpu|users)$/.test(item) ? index : at), -1);
const newVariants = registry.filter((row) => !devLaunchers[row.variant]);
const bimPrefix: Record<string, string> = { bim: "🏬️bim🏢️model" };
const added: string[] = [];
for (const row of newVariants) {
  const prefix = bimPrefix[row.variant];
  if (!prefix) { note(`variant ${row.variant}: needs a namePrefix`); continue; }
  devLaunchers[row.variant] = { namePrefix: prefix, order: 420 + registry.indexOf(row) * 0.01, wgpuOrder: Math.round((420 + registry.indexOf(row) * 0.01 + 0.001) * 1000) / 1000 };
  added.push(`@generated:${row.variant}:react`, `@generated:${row.variant}:wgpu`);
}
items.splice(lastDev + 1, 0, ...added);

const misplaced: E[] = [...S.inputs.filter((i) => !i.id), ...S.compounds.filter((c) => !Array.isArray(c.configurations)), ...S.compounds.flatMap((c) => (Array.isArray(c.configurations) ? c.configurations.filter((m: any) => typeof m !== "string") : []))];
note(`seed compounds array holds ${misplaced.length} misplaced configuration objects`);
const sOnly = [...S.configurations.filter((c): c is E => typeof c !== "string" && !byName.has(c.name)), ...misplaced.filter((c) => !byName.has(c.name))];
const lEquivalent = new Set(L.configurations.map(stripName));
const regenerated = (() => { try { return new Set(catalog.expandLaunchCatalog({ version: "0.2.0", configurations: items.filter((i) => typeof i === "string"), compounds: [], inputs: [], devLaunchers, crateRoutes: { subjects } }, registry).configurations.map(stripName)); } catch (e) { note("regeneration for equivalence failed: " + (e as Error).message); return new Set<string>(); } })();
let seedOnlyRenamed = 0, seedOnlyKept = 0;
const sPos = new Map<string, number>(S.configurations.map((c, i) => [typeof c === "string" ? c : c.name, i]));
const itemNames = () => new Map(items.map((c, i) => [typeof c === "string" ? c : c.name, i]));
for (const c of sOnly) {
  const key = stripName(c);
  if (lEquivalent.has(key) || regenerated.has(key)) { seedOnlyRenamed++; continue; }
  const { entry, reason } = repairEntry(c);
  if (!entry) { droppedRows.push({ name: c.name, reason: "seed-only: " + reason }); continue; }
  let at = items.length;
  if (sPos.has(c.name)) {
    const before = S.configurations.slice(0, sPos.get(c.name)!).reverse().find((x) => typeof x !== "string" && itemNames().has(x.name));
    if (before) at = itemNames().get((before as E).name)! + 1;
  } else {
    const sibling = items.map((x, i) => [x, i] as const).filter(([x]) => typeof x !== "string" && x.name.startsWith(c.name.slice(0, 20))).at(-1);
    if (sibling) at = sibling[1] + 1;
  }
  items.splice(at, 0, entry);
  seedOnlyKept++;
}
note(`seed-only: ${sOnly.length} (renamed duplicates ${seedOnlyRenamed}, kept ${seedOnlyKept})`);

const seen = new Map<string, E>();
const finalItems: (E | string)[] = [];
for (const item of items) {
  if (typeof item === "string") { finalItems.push(item); continue; }
  const prior = seen.get(item.name);
  if (!prior) { seen.set(item.name, item); finalItems.push(item); continue; }
  if (canon(prior) === canon(item)) { note(`duplicate identical ${item.name}: second removed`); continue; }
  if (item.name === "🧱️artifact-io-⚖️installed-service🧰️os🟦️" && /inference-service-receiving-check/.test(item.command)) {
    const renamed = { ...item, name: "🧱️artifact-io-⚖️inference-service-receiving-check🌍️gis🗺️gismap🟦️", presentation: { ...item.presentation, order: item.presentation.order + 0.0001 } };
    note(`duplicate DIFFERENT ${item.name}: second renamed to ${renamed.name}`);
    seen.set(renamed.name, renamed);
    finalItems.push(renamed);
    continue;
  }
  throw new Error(`unresolved duplicate ${item.name}`);
}

const finalNames = new Set(finalItems.filter((i): i is E => typeof i !== "string").map((i) => i.name));
for (const g of [`@generated:s:users`]) void g;
const generatedNames = (() => { const doc = catalog.expandLaunchCatalog({ version: "0.2.0", configurations: finalItems.filter((i) => typeof i === "string"), compounds: [], inputs: [], devLaunchers, crateRoutes: { subjects } }, registry); return new Set(doc.configurations.map((c: E) => c.name)); })();
const known = (n: string) => finalNames.has(n) || generatedNames.has(n);
const compounds: E[] = [];
const compoundNames = new Set<string>();
for (const c of [...L.compounds, ...S.compounds.filter((x) => Array.isArray(x.configurations))]) {
  if (compoundNames.has(c.name)) continue;
  compoundNames.add(c.name);
  const members: string[] = [];
  for (const m of c.configurations) {
    const name = typeof m === "string" ? m : m?.name;
    if (typeof m !== "string") { note(`compound ${c.name}: inline configuration ${name} removed from members`); continue; }
    if (!known(name)) { note(`compound ${c.name}: member ${name} unresolved, removed`); continue; }
    members.push(name);
  }
  if (!members.length) { note(`compound ${c.name}: no resolvable member, dropped`); continue; }
  compounds.push({ ...c, configurations: members });
}

const inputs: E[] = [];
for (const input of inputsAll) {
  const m = /^projectTarget\.(.+)$/.exec(input.id);
  if (!m || !Array.isArray(input.options)) { inputs.push(input); continue; }
  const options = input.options.filter((p: string) => [...(projects.get(p)?.targets ?? [])].some((t) => slug(t) === m[1]));
  if (options.length !== input.options.length) note(`input ${input.id}: removed stale options ${input.options.filter((p: string) => !options.includes(p)).join(", ")}`);
  if (!options.length) { note(`input ${input.id}: no option left, dropped`); continue; }
  inputs.push({ ...input, options });
}

const commandRows: E[] = [
  ["📦️generate-launch", "workspace:generate-launch", "4_build", 900.0013],
  ["⚖️generate-launch-check", "workspace:generate-launch-check", "4_gate", 900.0013],
  ["📦️generate-config", "workspace:generate-config", "4_build", 900.0014],
  ["⚖️generate-config-check", "workspace:generate-config-check", "4_gate", 900.0014],
  ["⚖️test-launch-catalog📚️library🟦️", "@semio-tech/repo-lib:test-launch-catalog", "4_gate", 900.0364],
].map(([name, project, group, order]) => ({ name, type: "node-terminal", request: "launch", command: `bun repo run ${project}`, cwd: "${workspaceFolder}", env: { ...environment }, presentation: { group, order } }));
const present = new Set(finalNames);
const firstTaxonomy = finalItems.findIndex((i) => typeof i !== "string" && i.name === "📦️generate-taxonomy-duplicates");
const toAdd = commandRows.filter((r) => !present.has(r.name));
finalItems.splice(firstTaxonomy < 0 ? finalItems.length : firstTaxonomy + 1, 0, ...toAdd);

const seed = { version: "0.2.0", configurations: finalItems, compounds, inputs, devLaunchers, crateRoutes: { subjects } };
const problems = catalog.launchCatalogProblems(seed, registry, R.slice(0, -1));
console.log("PROBLEMS", problems.length, problems.slice(0, 15));
await Bun.write(OUT + "draft-seed.json", JSON.stringify(seed));
await Bun.write(OUT + "fold-log.json", JSON.stringify(log, null, 1));
if (!problems.length) {
  const expanded = catalog.expandLaunchCatalog(seed, registry);
  await Bun.write(OUT + "new-launch.json", catalog.renderLaunchDocument(expanded));
  await Bun.write(OUT + "new-seed.jsonc", catalog.renderLaunchDocument(seed));
  const outByName = new Map<string, E>(expanded.configurations.map((c: E) => [c.name, c]));
  const droppedNames = new Set(droppedRows.map((d) => d.name));
  const stat: Record<string, number> = {};
  const examples: Record<string, string[]> = {};
  const bump = (k: string, n: string) => { stat[k] = (stat[k] ?? 0) + 1; (examples[k] ??= []).length < 5 && examples[k]!.push(n); };
  for (const c of L.configurations) {
    if (droppedNames.has(c.name) && !outByName.has(c.name)) { bump("dropped", c.name); continue; }
    const o = outByName.get(c.name);
    if (!o) { bump("MISSING", c.name); continue; }
    if (canon(o) === canon(c)) bump("identical", c.name);
    else if (stripOrder(o) === stripOrder(c)) bump(c.name.startsWith("🦀️") ? "crate-order-only" : "order-only", c.name);
    else bump("changed", c.name);
  }
  console.log("VERIFY", stat, JSON.stringify(examples));
  console.log("expanded", expanded.configurations.length, "L", L.configurations.length);
}
console.log(JSON.stringify({ items: finalItems.length, placeholders: finalItems.filter((i) => typeof i === "string").length, literal: finalItems.filter((i) => typeof i !== "string").length, subjects: subjects.length, devLaunchers: Object.keys(devLaunchers).length, compounds: compounds.length, inputs: inputs.length, dropped: droppedRows.length, repairs: log.repairs }, null, 1));
const reasons: Record<string, number> = {};
for (const d of droppedRows) { const k = d.reason.replace(/ .*/, ""); reasons[k] = (reasons[k] ?? 0) + 1; }
console.log("dropped by reason", reasons);
for (const n of log.notes) console.log("NOTE", n);
