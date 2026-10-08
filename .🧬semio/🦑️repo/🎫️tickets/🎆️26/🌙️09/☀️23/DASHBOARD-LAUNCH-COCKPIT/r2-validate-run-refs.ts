#!/usr/bin/env bun
// 🔎 Round 2 audit helper (read-only): resolves every `nx run` / `nx run-many` / `nx exec` reference of the
// developer config surfaces against the published Nx project graph, and writes the result under 🗑️generated.
const ROOT = "C:/git/semio";
const T = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT`;
const GEN = `${T}/🗑️generated`;

const graph = JSON.parse(await Bun.file(`${ROOT}/.nx/workspace-data/project-graph.json`).text());
const nodes: Record<string, { data: { targets?: Record<string, unknown> } }> = graph.nodes;
const targetsOf = (project: string): Set<string> | undefined => {
  const node = nodes[project];
  return node ? new Set(Object.keys(node.data.targets ?? {})) : undefined;
};

type Ref = { source: string; row: string; ref: string; status: "ok" | "missing-project" | "missing-target" | "unchecked" };
const refs: Ref[] = [];

function resolve(source: string, row: string, project: string, target: string | undefined, ref: string): void {
  const targets = targetsOf(project);
  if (!targets) return void refs.push({ source, row, ref, status: "missing-project" });
  if (target === undefined) return void refs.push({ source, row, ref, status: "ok" });
  refs.push({ source, row, ref, status: targets.has(target) ? "ok" : "missing-target" });
}

function scan(source: string, row: string, command: string): void {
  for (const m of command.matchAll(/nx run (?!-)([^\s"'`]+)/g)) {
    const [project, target] = m[1]!.split(":");
    if (!project || project.includes("{")) continue;
    resolve(source, row, project, target, m[1]!);
  }
  for (const m of command.matchAll(/nx run-many[^\n]*?(?:-t|--target)[= ]([^\s"'`]+)[^\n]*?--projects[= ]([^\s"'`]+)/g)) {
    for (const project of m[2]!.split(",")) {
      if (project.includes("*") || project.startsWith("!") || project.includes("{")) {
        refs.push({ source, row, ref: `run-many ${m[1]} ${project}`, status: "unchecked" });
        continue;
      }
      resolve(source, row, project, m[1], `run-many ${m[1]} ${project}`);
    }
  }
  for (const m of command.matchAll(/nx exec[^\n]*?--projects[= ]([^\s"'`]+)/g)) {
    for (const project of m[1]!.split(",")) {
      if (project.includes("*") || project.startsWith("!") || project.includes("{") || project === "workspace") continue;
      resolve(source, row, project, undefined, `exec --projects=${project}`);
    }
  }
}

function strings(value: unknown): string[] {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return value.map((v) => (Array.isArray(v) ? v.join(" ") : String(v)));
  return [];
}

async function jsoncRows(path: string): Promise<{ name: string; command: string }[]> {
  const doc = Bun.JSONC.parse(await Bun.file(path).text()) as { configurations?: unknown[] };
  const rows: { name: string; command: string }[] = [];
  for (const entry of doc.configurations ?? []) {
    if (!entry || typeof entry !== "object") continue;
    const e = entry as Record<string, unknown>;
    const name = String(e.name ?? "");
    const command = [typeof e.command === "string" ? e.command : "", ...strings(e.runtimeArgs)].join(" ");
    if (command.trim()) rows.push({ name, command });
  }
  return rows;
}

// 1. Launch files (the surfaces the dashboard and the editor read today)
for (const path of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc", ".claude/launch.json"]) {
  for (const row of await jsoncRows(`${ROOT}/${path}`)) scan(path, row.name, row.command);
}

// 2. Root package.json scripts
const pkg = JSON.parse(await Bun.file(`${ROOT}/package.json`).text()) as { scripts: Record<string, string> };
for (const [name, command] of Object.entries(pkg.scripts)) scan("package.json", name, command);

// 2b. Other automation that invokes Nx (CI workflow, devcontainer lifecycle, editor settings)
for (const path of [".github/workflows/architecture-quiz.yml", ".devcontainer/devcontainer.json", ".vscode/settings.json", ".vscode/mcp.json", ".mcp.json"]) {
  const text = await Bun.file(`${ROOT}/${path}`).text();
  for (const line of text.split("\n")) if (/nx (run|exec)/.test(line)) scan(path, "line", line);
}

// 3. Root and owner project manifests: target commands and dashboard declarations
const manifests = (await Bun.file(`${T}/🗑️generated/project-manifests.txt`).text()).split("\n").filter(Boolean);
let manifestCommands = 0;
for (const rel of manifests) {
  const doc = JSON.parse(await Bun.file(`${ROOT}/${rel}`).text()) as Record<string, any>;
  for (const [target, def] of Object.entries<any>(doc.targets ?? {})) {
    for (const command of [def?.options?.command, ...(def?.options?.commands ?? []).map((c: any) => c?.command)]) {
      if (typeof command === "string" && /nx (run|exec|run-many)/.test(command)) {
        manifestCommands++;
        scan(rel, `${doc.name}:${target}`, command);
      }
    }
  }
  const dash = doc.metadata?.semio?.dashboard;
  if (!dash) continue;
  for (const tool of dash.tools ?? []) scan(rel, `tool ${tool.id}`, tool.command.join(" "));
  for (const compound of dash.compounds ?? []) {
    for (const member of compound.members ?? []) {
      const ref = String(member.run ?? "");
      if (/^(tool|playground|group|compound|ticket|repo|script):/.test(ref)) {
        refs.push({ source: rel, row: `compound ${compound.id}`, ref, status: "unchecked" });
      } else {
        const [project, target] = ref.split(":");
        resolve(rel, `compound ${compound.id}`, project!, target, ref);
      }
    }
  }
  for (const group of dash.groups ?? []) {
    for (const target of group.targets ?? [group.target]) {
      for (const project of group.projects ?? []) {
        if (project.includes("*") || project.startsWith("!")) {
          refs.push({ source: rel, row: `group ${group.id}`, ref: `${project}:${target}`, status: "unchecked" });
        } else resolve(rel, `group ${group.id}`, project, target, `${project}:${target}`);
      }
    }
  }
}

const byStatus = refs.reduce<Record<string, number>>((acc, r) => ((acc[r.status] = (acc[r.status] ?? 0) + 1), acc), {});
const missing = refs.filter((r) => r.status.startsWith("missing"));
await Bun.write(`${GEN}/run-resolution.json`, JSON.stringify({ graphNodes: Object.keys(nodes).length, manifestCommands, byStatus, missing, refs }, null, 2));
console.log(JSON.stringify({ graphNodes: Object.keys(nodes).length, manifestCommands, total: refs.length, byStatus }));
const uniq = new Map<string, number>();
for (const r of missing) uniq.set(`${r.status} | ${r.source} | ${r.ref}`, (uniq.get(`${r.status} | ${r.source} | ${r.ref}`) ?? 0) + 1);
for (const [key, count] of [...uniq].slice(0, 400)) console.log(`${count}x ${key}`);
