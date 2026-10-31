// 🧭️ Read-only path-dependency walk: which 1029 failing crates each closure component transitively reaches.
// Input: component manifests (relative to repo root), failing crate -> batch map from 1029-audit-matrix.md section 2.
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = process.argv[2];
const components = readFileSync(join(TICKET, "🗑️generated/1030/component-manifests.txt"), "utf8").split("\n").filter(Boolean);

const failing: Record<string, string> = {
  "semio-framework-plugin-time-travel-turns": "hub", "semio-framework-plugin-instance-typed-ops": "hub", "semio-s-fem-node": "hub", "semio-s-fem-material": "hub",
  "semio-s-artifact-procedural-2d-editor-config": "hub", "semio-s-artifact-procedural-2d-editor-presence": "hub",
  "semio-s-artifact-cad-cad-model-diff": "cad-aec-building",
  "semio-framework-plugin-guest-contract": "cad-spatial-shape", "semio-framework-plugin-registration": "cad-spatial-shape", "semio-framework-2d-text": "cad-spatial-shape",
  "semio-s-artifact-stdio-step-snapshot": "flow-brep", "semio-framework-os-flow-extensions": "flow-brep", "semio-s-artifact-stdio-semio-brep-viewer": "flow-brep",
  "semio-s-plugin-flow-brep-mesh": "playbook-procedural",
  "semio-s-artifact-stdio-step-mutations": "process-concrete",
  "semio-s-artifact-process-process3d-model": "process-metal",
  "semio-s-artifact-stdio-step-cc4": "process-robotic", "semio-s-artifact-stdio-step-inferences": "process-robotic", "semio-s-artifact-stdio-semio-flow": "process-robotic", "semio-s-artifact-stdio-semio-brep": "process-robotic",
  "semio-s-artifact-stdio-semio-object-snapshot": "sourcing-beams", "semio-s-artifact-stdio-semio-presentation-snapshot": "sourcing-beams",
  "semio-s-artifact-stdio-semio-object-examples": "sourcing-slabs,sourcing-windows", "semio-s-artifact-stdio-semio-presentation-mutations": "sourcing-slabs,sourcing-windows",
};

type Pkg = { name: string; deps: { name?: string; path?: string; workspace?: boolean }[] };
const cache = new Map<string, { name: string; dir: string; deps: any; wsDeps: Record<string, any>; wsDir?: string; isWs: boolean }>();

function load(manifest: string) {
  if (cache.has(manifest)) return cache.get(manifest)!;
  const t = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const entry = { name: t.package?.name ?? "", dir: dirname(manifest), deps: { ...(t.dependencies ?? {}) }, wsDeps: t.workspace?.dependencies ?? {}, isWs: !!t.workspace, wsDir: undefined as string | undefined };
  cache.set(manifest, entry);
  return entry;
}

function workspaceDepsOf(manifest: string): Record<string, any> {
  let d = dirname(manifest);
  while (d.startsWith(ROOT)) {
    const m = join(d, "Cargo.toml");
    if (existsSync(m)) {
      const t = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
      if (t.workspace) return { dir: d, deps: t.workspace.dependencies ?? {} } as any;
    }
    const up = dirname(d); if (up === d) break; d = up;
  }
  return {};
}

function reach(start: string): Set<string> {
  const seen = new Set<string>(), queue = [start];
  while (queue.length) {
    const m = queue.pop()!;
    const e = load(m);
    for (const [key, spec0] of Object.entries(e.deps) as [string, any][]) {
      let spec = spec0 as any;
      if (typeof spec === "object" && spec.workspace === true) {
        const ws = workspaceDepsOf(m) as any;
        spec = { ...(ws.deps?.[key] ?? {}), ...spec };
        if (ws.dir && spec.path) spec.path = join(ws.dir, spec.path);
        else if (ws.dir === undefined && !spec.path) continue;
      }
      if (typeof spec !== "object" || !spec.path) continue;
      const depDir = resolve(e.dir, spec.path);
      const depManifest = join(depDir, "Cargo.toml");
      if (!existsSync(depManifest) || seen.has(depManifest)) continue;
      seen.add(depManifest);
      queue.push(depManifest);
    }
  }
  return seen;
}

const out: string[] = [];
for (const rel of components) {
  const manifest = join(ROOT, rel);
  const names = new Set<string>();
  const reached = reach(manifest);
  for (const m of [manifest, ...reached]) names.add(load(m).name);
  const hits = Object.keys(failing).filter(n => names.has(n));
  const byBatch = [...new Set(hits.map(n => failing[n]))].sort();
  out.push(`${load(manifest).name} | reachable=${names.size} | failing=${hits.length} | batches=${byBatch.join(",") || "none"} | ${hits.join(",")}`);
}
console.log(out.join("\n"));
