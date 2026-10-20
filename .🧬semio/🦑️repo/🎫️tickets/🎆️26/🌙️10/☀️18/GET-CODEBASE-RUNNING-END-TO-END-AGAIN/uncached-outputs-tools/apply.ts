import { existsSync, readFileSync, renameSync, statSync, writeFileSync, chmodSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ROOT, TICKET } from "./targets.ts";
import { apply, parse, prop, setProps, type Edit, type Node } from "./jsonspan.ts";
import type { Decision } from "./decide.ts";

const DANGLING = { manifest: "✏️s/🧑‍💻dev/📋️project.json", target: "typecheck", entry: "@semio-tech/trinity-jack-lsp:wasm" };
export type Action = { id: string; action: "outputs-added" | "cache-disabled" | "already-declared" | "already-uncached" | "missing-at-apply" | "dependsOn-removed" | "skipped"; detail?: string };

function targetNode(root: Node, name: string): Node | undefined {
  const targets = prop(root, "targets");
  return targets && targets.value.type === "object" ? prop(targets.value, name)?.value : undefined;
}
const valueOf = (n: Node): unknown => n.type === "string" || n.type === "number" || n.type === "literal" ? n.value : undefined;

function arrayItemRemoval(text: string, array: Node, index: number): Edit {
  if (array.type !== "array") throw Error("array expected");
  const item = array.items[index]!;
  if (index > 0) return { start: array.items[index - 1]!.end, end: item.end, text: "" };
  if (array.items.length > 1) return { start: item.start, end: array.items[1]!.start, text: "" };
  return { start: array.start + 1, end: array.end - 1, text: "" };
}

export function planManifest(text: string, manifest: string, decisions: Decision[], dangling: boolean): { edits: Edit[]; actions: Action[]; expect: (json: any) => void } {
  const root = parse(text);
  const edits: Edit[] = [], actions: Action[] = [], todo: ((json: any) => void)[] = [];
  for (const d of decisions) {
    const node = targetNode(root, d.target);
    if (!node || node.type !== "object") { actions.push({ id: d.id, action: "missing-at-apply" }); continue; }
    const outputs = prop(node, "outputs"), cache = prop(node, "cache");
    if (d.resolution === "verdict") {
      if (outputs) { actions.push({ id: d.id, action: "already-declared" }); continue; }
      if (cache && cache.value.type === "literal" && cache.value.value === false) { actions.push({ id: d.id, action: "already-uncached" }); continue; }
      edits.push(...setProps(text, node, [["outputs", []]], ["inputs", "cache", "executor"]));
      todo.push((json) => { json.targets[d.target].outputs = []; });
      actions.push({ id: d.id, action: "outputs-added" });
    } else if (d.resolution === "uncache") {
      if (cache && cache.value.type === "literal" && cache.value.value === false) { actions.push({ id: d.id, action: "already-uncached" }); continue; }
      edits.push(...setProps(text, node, [["cache", false]], ["executor"]));
      todo.push((json) => { json.targets[d.target].cache = false; });
      actions.push({ id: d.id, action: "cache-disabled" });
    } else actions.push({ id: d.id, action: "skipped", detail: d.resolution });
  }
  if (dangling) {
    const node = targetNode(root, DANGLING.target);
    const deps = node && node.type === "object" ? prop(node, "dependsOn")?.value : undefined;
    if (deps && deps.type === "array") {
      const index = deps.items.findIndex((item) => valueOf(item) === DANGLING.entry);
      if (index >= 0) {
        edits.push(arrayItemRemoval(text, deps, index));
        todo.push((json) => { json.targets[DANGLING.target].dependsOn = json.targets[DANGLING.target].dependsOn.filter((x: unknown) => x !== DANGLING.entry); });
        actions.push({ id: `${manifest}#${DANGLING.target}.dependsOn`, action: "dependsOn-removed", detail: DANGLING.entry });
      }
    }
  }
  return { edits, actions, expect: (json) => todo.forEach((f) => f(json)) };
}

const canon = (v: unknown): unknown => Array.isArray(v) ? v.map(canon) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).sort(([a], [b]) => (a < b ? -1 : 1)).map(([k, x]) => [k, canon(x)])) : v;
const same = (a: unknown, b: unknown) => JSON.stringify(canon(a)) === JSON.stringify(canon(b));

export function run(decisions: Decision[], write: boolean): { actions: Action[]; manifests: string[] } {
  const byManifest = new Map<string, Decision[]>();
  for (const d of decisions) if (d.resolution === "verdict" || d.resolution === "uncache") (byManifest.get(d.manifest) ?? byManifest.set(d.manifest, []).get(d.manifest)!).push(d);
  if (!byManifest.has(DANGLING.manifest)) byManifest.set(DANGLING.manifest, []);
  const actions: Action[] = [], touched: string[] = [];
  for (const [manifest, rows] of [...byManifest].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
    const path = join(ROOT, manifest);
    for (let attempt = 0; ; attempt++) {
      if (attempt > 3) throw Error("manifest keeps changing: " + manifest);
      const before = readFileSync(path, "utf8");
      const plan = planManifest(before, manifest, rows, manifest === DANGLING.manifest);
      if (!plan.edits.length) { actions.push(...plan.actions); break; }
      const after = apply(before, plan.edits);
      const expected = JSON.parse(before); plan.expect(expected);
      if (!same(JSON.parse(after), expected)) throw Error("edit changed more than intended: " + manifest);
      if (write) {
        if (readFileSync(path, "utf8") !== before) continue;
        const mode = statSync(path).mode, temp = path + ".uncached-outputs.tmp";
        writeFileSync(temp, after); chmodSync(temp, mode);
        if (readFileSync(path, "utf8") !== before) continue;
        renameSync(temp, path);
      }
      touched.push(manifest); actions.push(...plan.actions);
      break;
    }
  }
  return { actions, manifests: touched };
}

if (import.meta.main) {
  const write = process.argv.includes("--write");
  const decisions: Decision[] = JSON.parse(readFileSync(join(TICKET, "🗑️generated", "uncached-outputs", "decisions.json"), "utf8"));
  if (decisions.some((d) => d.basis === "UNREVIEWED")) throw Error("unreviewed decisions remain");
  const { actions, manifests } = run(decisions, write);
  const counts: Record<string, number> = {}; for (const a of actions) counts[a.action] = (counts[a.action] ?? 0) + 1;
  const dir = join(TICKET, "🗑️generated", "uncached-outputs"); mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, write ? "apply-actions.json" : "dryrun-actions.json"), JSON.stringify({ counts, manifests, actions }, null, 1));
  console.log(write ? "WRITTEN" : "DRY RUN", counts, manifests.length, "manifests");
}
