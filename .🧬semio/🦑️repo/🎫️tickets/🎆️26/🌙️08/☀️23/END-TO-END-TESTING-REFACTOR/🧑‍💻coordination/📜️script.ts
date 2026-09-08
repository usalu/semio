/** 🧑‍💻️ Replays the retained framework runtime and discovery evidence without modifying sources. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const root = process.env.SEMIO_LAYOUT_REPO_ROOT ?? process.cwd(), ticket = dirname(import.meta.dir), lane = join(ticket, "🗑️generated", "framework-layout");
process.chdir(root);
mkdirSync(lane, { recursive: true });
{
  const resultPath = join(lane, "runtime-results.json");
  const markdown = readFileSync(join(ticket, "📓️test-layout-framework-files-2026-09-08.md"), "utf8");
  const manifest = JSON.parse(/```json\s*\n([\s\S]*?)\n```/u.exec(markdown)![1]);
  const results: { name: string; status: string; detail?: string }[] = [];
  for (const move of manifest.moves) {
    try {
      const module = await import(pathToFileURL(resolve(root, move.destination)).href);
      const count = await module[move.name]();
      results.push({ name: move.name, status: "passed", detail: count === undefined ? undefined : String(count) });
    } catch (error) { results.push({ name: move.name, status: "failed", detail: error instanceof Error ? error.stack : String(error) }); }
    console.log(`[DEBUG] ${move.name}: ${results.at(-1)!.status}`);
  }
  const plugin = spawnSync("bun", ["test", "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts", "--test-name-pattern", "Nx discovers"], { cwd: root, encoding: "utf8" });
  writeFileSync(join(lane, "plugin-discovery.log"), plugin.stdout + plugin.stderr);
  results.push({ name: "Nx canonical case discovery regression", status: plugin.status === 0 ? "passed" : "failed", detail: plugin.stdout + plugin.stderr });
  const code = "import importlib.util as u; from pathlib import Path; root=Path('♻️mit-bestand/🔎️recherche/_neo4j/netz/netz'); paths=[root/'data/🧪️tests/🔬️stage-one-parity/🐍️.py',root/'model/🧪️tests/🔬️stage-two-parity/🐍️.py',root/'🧪️tests/🪜️stage-regression/🐍️.py']; [compile(p.read_text(),str(p),'exec') for p in paths]; load=lambda i,p: (lambda s: (lambda m:(s.loader.exec_module(m),m)[1])(u.module_from_spec(s)))(u.spec_from_file_location('layout_stage_'+str(i),p)); modules=[load(i,p) for i,p in enumerate(paths)]; assert modules[0].PACKAGE == root.resolve(); assert modules[1].PACKAGE == root.resolve(); assert modules[0].GOLDEN.exists(); assert modules[1].GOLDEN.exists(); print('[DEBUG] 3 canonical stage imports and golden roots verified')";
  const python = spawnSync(join(root, ".venv", process.platform === "win32" ? "Scripts/python.exe" : "bin/python"), ["-c", code], { cwd: root, encoding: "utf8", env: { ...process.env, PYTHONPYCACHEPREFIX: join(lane, "python-cache") } });
  results.push({ name: "Python canonical stage imports", status: python.status === 0 ? "passed" : "failed", detail: python.stdout + python.stderr });
  writeFileSync(resultPath, JSON.stringify(results, null, 2) + "\n");
  writeFileSync(join(ticket, "📓️test-layout-framework-runtime-2026-09-08.md"), "# Framework Oracle Runtime Results\n\nPublic Bun/Nx execution of all twelve canonical exported fixture functions and focused discovery/import checks. Nx workspace: `" + (process.env.NX_WORKSPACE_ROOT_PATH ?? root) + "`. A private minimal Nx fixture is used if the full repository graph remains blocked; this proves function execution, not the full graph. Assertions are unchanged.\n\n```json\n" + JSON.stringify(results, null, 2) + "\n```\n");
  console.log(JSON.stringify(results, null, 2));
  process.exit(results.some(result => result.status === "failed") ? 1 : 0);
}