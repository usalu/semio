/** 🧪️ Drives a live dev serve through every write style a developer or editor actually uses and reports,
 * per host module, whether the NEXT request served the edited bytes or a pre-edit transform.
 *
 * Appends a marked line, re-reads before every write and strips only its own marker afterwards, so a peer
 * editing the same module concurrently is never reverted.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts */
import { readFileSync, writeFileSync, renameSync, statSync } from "node:fs";
import { execFileSync } from "node:child_process";

const port = Number(process.argv[2] ?? 6029);
const repoRoot = "/Users/ueli/Documents/semio";
const MARKER = "[DEBUG] stale-guard";

const modules = [
  { label: "renderer/NodeGraph (depth 8)", path: `${repoRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` },
  { label: "ui/react runtime (depth 6)", path: `${repoRoot}/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts` },
  { label: "ui/react render (dep of dep)", path: `${repoRoot}/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts` },
];

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const url = (path) => `http://127.0.0.1:${port}/@fs${path}`;

async function served(path) {
  const response = await fetch(url(path), { headers: { accept: "*/*" } });
  const text = await response.text();
  return { status: response.status, text };
}

function stripMarkers(path) {
  const current = readFileSync(path, "utf8");
  const cleaned = current.split("\n").filter((line) => !line.includes(MARKER)).join("\n");
  if (cleaned !== current) writeFileSync(path, cleaned);
}

const styles = {
  "in-place append (>>)": (path, line) => writeFileSync(path, `${readFileSync(path, "utf8")}${line}\n`),
  "sed -i '' (macOS temp+rename)": (path, line) => execFileSync("sed", ["-i", "", `$a\\
${line}`, path]),
  "rename-into-place (editor save)": (path, line) => {
    const tmp = `${path}.stale-guard-tmp`;
    writeFileSync(tmp, `${readFileSync(path, "utf8")}${line}\n`);
    renameSync(tmp, path);
  },
  "import type only (rename-into-place)": (path) => {
    const tmp = `${path}.stale-guard-tmp`;
    const body = readFileSync(path, "utf8");
    writeFileSync(tmp, `import type { StaleGuardProbe${Date.now()} } from "node:fs";\n${body}`);
    renameSync(tmp, path);
  },
};

const rows = [];
for (const module of modules) {
  const before = await served(module.path);
  if (before.status !== 200) {
    rows.push({ module: module.label, style: "(not servable)", status: before.status, stale: "n/a" });
    continue;
  }
  for (const [style, write] of Object.entries(styles)) {
    const stamp = `${MARKER} ${style} ${Date.now()}`;
    const line = style.startsWith("import type") ? "" : `console.info(${JSON.stringify(stamp)});`;
    const baseline = await served(module.path);
    write(module.path, line);
    const mtime = statSync(module.path).mtimeMs;
    await sleep(2500);
    const after = await served(module.path);
    const changed = after.text !== baseline.text;
    rows.push({ module: module.label, style, mtimeMoved: mtime > 0, servedChanged: changed, stale: changed ? "fresh" : "STALE" });
    stripMarkers(module.path);
    if (style.startsWith("import type")) {
      const body = readFileSync(module.path, "utf8").split("\n").filter((l) => !l.startsWith("import type { StaleGuardProbe")).join("\n");
      writeFileSync(module.path, body);
    }
    await sleep(1500);
  }
  stripMarkers(module.path);
}

console.table(rows);
