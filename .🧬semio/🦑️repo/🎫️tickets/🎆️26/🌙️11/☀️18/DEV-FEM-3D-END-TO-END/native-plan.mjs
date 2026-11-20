import {readFileSync, readdirSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";

/** 🗺️ Orders the dependsOn closure of one task from the last sealed selected graph: `node native-plan.mjs <project:target> [out.json]`. */
const [root, out] = process.argv.slice(2);
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const selected = join(workspace, ".cache/b/g/m/selected");
const [rootProject, rootTarget] = [root.slice(0, root.lastIndexOf(":")), root.slice(root.lastIndexOf(":") + 1)];
const graph = readdirSync(selected).filter(name => /^[a-f0-9]{64}\.json$/.test(name)).map(name => JSON.parse(readFileSync(join(selected, name), "utf8"))).find(value => value.projects?.[rootProject]?.targets?.[rootTarget]);
const projects = graph?.projects ?? {}, edges = graph?.dependencies ?? {};
const authored = new Map(execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "*📋️project.json"], {cwd: workspace, encoding: "utf8", maxBuffer: 1 << 28}).split("\n").filter(Boolean).flatMap(path => {try {const value = JSON.parse(readFileSync(join(workspace, path), "utf8")); return value.name ? [[value.name, {...value, root: dirname(path)}]] : [];} catch {return [];}}));
for (const [name, value] of authored) if (!projects[name]) projects[name] = {...value, authoredOnly: true};
if (!graph) {
  const target = authored.get(rootProject)?.targets?.[rootTarget];
  if (!target || !Array.isArray(target.dependsOn) || target.dependsOn.length) throw Error(`no sealed selected graph holds ${root}`);
}
const projectDeps = name => (edges[name] ?? []).map(edge => typeof edge === "string" ? edge : edge.target).filter(name => projects[name]);
const expand = (project, entry) => {
  const spec = typeof entry === "string" ? entry : entry.projects ? null : (entry.dependencies ? `^${entry.target}` : entry.target);
  if (spec === null) return (Array.isArray(entry.projects) ? entry.projects : [entry.projects]).map(name => `${name}:${entry.target}`);
  if (spec.startsWith("^")) return projectDeps(project).filter(name => projects[name].targets?.[spec.slice(1)]).map(name => `${name}:${spec.slice(1)}`);
  return spec.includes(":") ? [spec] : [`${project}:${spec}`];
};
const order = [], state = new Map();
const visit = id => {
  if (state.get(id) === "done") return;
  if (state.get(id) === "open") throw Error(`cycle at ${id}`);
  state.set(id, "open");
  const at = id.lastIndexOf(":"), project = id.slice(0, at), name = id.slice(at + 1), target = projects[project]?.targets?.[name];
  if (!target) {state.set(id, "done"); order.push({id, missing: true}); return;}
  for (const entry of target.dependsOn ?? []) for (const next of expand(project, entry)) visit(next);
  state.set(id, "done");
  const current = authored.get(project)?.targets?.[name]?.options?.command;
  const command = current ?? target.execution?.command ?? target.options?.command ?? null;
  const cwd = target.options?.cwd ? target.options.cwd.replace("{projectRoot}", projects[project].root) : projects[project].root;
  order.push({id, authoredOnly: !!projects[project].authoredOnly, cwd, command, env: target.execution?.env ?? target.options?.env ?? {}, executor: target.executor ?? null, continuous: !!target.continuous});
};
visit(root);
if (out) writeFileSync(out, JSON.stringify(order, null, 1));
for (const row of order) console.log(`${row.id}\t${row.cwd}\t${row.command}`);
