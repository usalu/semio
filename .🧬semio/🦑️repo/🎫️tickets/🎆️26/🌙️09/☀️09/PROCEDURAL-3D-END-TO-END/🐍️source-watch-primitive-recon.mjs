/** 🛰️ Exercises the raw `node:fs` recursive watch the dev server's source watcher is built on, under the
 * runtime that actually serves (`bun`), across write styles and module depths, so a missed dev-server
 * invalidation can be attributed to the watcher primitive rather than to Vite's module graph.
 *
 * Reports, per write style, how often the EDITED PATH itself was named by an event — an event naming a
 * temp file or the parent directory never invalidates the module Vite keyed by the edited path.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts */
import { watch, readFileSync, writeFileSync, renameSync, readdirSync, mkdirSync, rmSync } from "node:fs";
import { join, relative } from "node:path";
import { execFileSync } from "node:child_process";

const repoRoot = "/Users/ueli/Documents/semio";
const UNWATCHED = [".git", ".nx", ".vscode", ".🧬semio", "node_modules", "dist", "target", "🤖️generated", "🗑️generated"];
const excluded = new Set(UNWATCHED);
const roots = readdirSync(repoRoot, { withFileTypes: true }).filter((e) => e.isDirectory() && !excluded.has(e.name)).map((e) => join(repoRoot, e.name)).sort();

const targets = [
  { label: "depth8", path: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.stale-guard-probe.ts") },
  { label: "depth4", path: join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.stale-guard-probe.ts") },
  { label: "depth1", path: join(repoRoot, "🌎️hub/🟦️.stale-guard-probe.ts") },
];

const events = [];
const handles = roots.map((root) => watch(root, { recursive: true, persistent: true }, (eventType, name) => {
  if (name === null) return;
  events.push({ eventType, path: join(root, name) });
}));

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const drain = () => events.splice(0, events.length);

const styles = {
  "in-place-append": (path, body) => writeFileSync(path, body),
  "sed -i '' (temp+rename)": (path) => execFileSync("sed", ["-i", "", `$a\\
// [DEBUG] stale-guard ${Date.now()}`, path]),
  "rename-into-place": (path, body) => {
    const tmp = `${path}.tmp-probe`;
    writeFileSync(tmp, body);
    renameSync(tmp, path);
  },
  "truncate+rewrite": (path, body) => {
    writeFileSync(path, "");
    writeFileSync(path, body);
  },
};

const REPEATS = 5;
const table = [];
for (const target of targets) {
  mkdirSync(join(target.path, ".."), { recursive: true });
  writeFileSync(target.path, "export const probe = 0;\n");
  await sleep(1200);
  drain();
  for (const [style, run] of Object.entries(styles)) {
    let named = 0;
    let dirNamed = 0;
    let any = 0;
    for (let i = 0; i < REPEATS; i += 1) {
      drain();
      run(target.path, `export const probe = ${Date.now()};\n`);
      await sleep(1600);
      const got = drain();
      any += got.length;
      if (got.some((e) => e.path === target.path)) named += 1;
      if (got.some((e) => e.path === join(target.path, ".."))) dirNamed += 1;
    }
    table.push({ depth: target.label, style, namedTarget: `${named}/${REPEATS}`, namedParentDir: `${dirNamed}/${REPEATS}`, totalEvents: any });
  }
  rmSync(target.path, { force: true });
  await sleep(500);
  drain();
}

console.table(table);
for (const handle of handles) handle.close();
process.exit(0);
