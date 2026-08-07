import { readdir, readFile, writeFile, rmdir } from "node:fs/promises";
import { join, relative } from "node:path";
import { rename } from "node:fs/promises";

const repo = "/Users/ueli/Documents/semio";
const ticket = join(
  repo,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
);
const ui = join(repo, "🧰️framework/🔨️modules/🖱️ui");
const elements = join(ui, "🧱️elements");
const core = join(elements, "🫀️core");

const LIFT = [
  "🆔ElementId",
  "🌈️Surface",
  "🎛️Chrome",
  "🏷️ClassNames",
  "🏷️Label",
  "🏷️UiLabel",
  "🐚️ShellScope",
  "🐹️ElementProps",
  "📚️I18n",
  "🚗️UiDriver",
  "🧭️Flow",
];

const PORTS_OLD = "🔌Ports";
const PORTS_NEW = "🔌️Ports";

async function liftFolders() {
  for (const name of LIFT) {
    const from = join(core, name);
    const to = join(elements, name);
    await rename(from, to);
  }
  await rename(join(core, PORTS_OLD), join(elements, PORTS_NEW));
  await rmdir(core);
}

function patchContent(text, fileRel) {
  let s = text;
  s = s.replaceAll(`../🫀️core/`, `../`);
  s = s.replaceAll(`🧱️elements/🫀️core/`, `🧱️elements/`);
  s = s.replaceAll(`../${PORTS_OLD}/`, `../${PORTS_NEW}/`);
  s = s.replaceAll(`/${PORTS_OLD}/`, `/${PORTS_NEW}/`);
  if (fileRel.includes("🏷️Label/🟦️component.tsx")) {
    s = s.replaceAll(`../../🪵Tree/`, `../🪵Tree/`);
  }
  if (fileRel.includes("🐚️ShellScope/🟦️component.tsx")) {
    s = s.replaceAll(`../../../📦️packages/`, `../../📦️packages/`);
  }
  return s;
}

async function walk(dir, files = []) {
  for (const ent of await readdir(dir, { withFileTypes: true })) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) {
      if (ent.name === "🫀️core") continue;
      await walk(p, files);
    } else if (/\.(tsx?|rs)$/.test(ent.name)) {
      files.push(p);
    }
  }
  return files;
}

async function patchUiTree() {
  const files = await walk(ui);
  let changed = 0;
  for (const f of files) {
    const rel = relative(ui, f);
    const before = await readFile(f, "utf8");
    const after = patchContent(before, rel);
    if (after !== before) {
      await writeFile(f, after);
      changed++;
    }
  }
  return changed;
}

const lifted = [];
for (const n of LIFT) lifted.push(n);
lifted.push(PORTS_NEW);

await liftFolders();
const changedFiles = await patchUiTree();

const report = {
  lifted,
  removedCore: true,
  portsRenamed: { from: PORTS_OLD, to: PORTS_NEW },
  patchedFiles: changedFiles,
};
await writeFile(join(ticket, "wave1-ui-elements-core.lift-log.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
