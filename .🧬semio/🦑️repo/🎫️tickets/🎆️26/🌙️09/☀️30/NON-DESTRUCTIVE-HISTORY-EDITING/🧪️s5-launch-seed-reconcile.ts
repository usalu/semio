/** 🧷️ Moves hand-added `.vscode/launch.json` rows into `.vscode/🧩️launch.seed.jsonc` before `plugin-registry:generate` re-renders the
 * launch file from the seed (peers register launchers in the output; a render would drop them). Input: the generator preview
 * (`cd <registry> && bun ./📜️script.ts preview-generated > preview.json`). A row the preview no longer renders is inserted after its
 * nearest preceding neighbour the seed owns; a row whose body (everything but name and presentation) the preview already renders
 * under another name is a rename and is skipped. Usage: `bun <this> <preview.json> [--check]`. */
import { assertLaunchSeedPlacement } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧱️placement/🟦️.ts";

type Row = Record<string, unknown>;
const root = "/Users/ueli/Documents/semio";
const seedPath = `${root}/.vscode/🧩️launch.seed.jsonc`;
const check = process.argv.includes("--check");
const previewPath = process.argv.slice(2).find(argument => !argument.startsWith("--"));
if (!previewPath) throw new Error("usage: <preview.json> [--check]");
const preview = JSON.parse(await Bun.file(previewPath).text()) as { nodes: { path: string; bytesBase64: string }[] };
const rendered = Bun.JSONC.parse(Buffer.from(preview.nodes.find(node => node.path === ".vscode/launch.json")!.bytesBase64, "base64").toString("utf8")) as { configurations: Row[]; inputs: Row[] };
const current = Bun.JSONC.parse(await Bun.file(`${root}/.vscode/launch.json`).text()) as { configurations: Row[]; inputs: Row[] };
const raw = await Bun.file(seedPath).text();
const seed = Bun.JSONC.parse(raw) as { configurations: (Row | string)[]; inputs: Row[] };
/** 🧬️ Identifies a launcher by what it runs, ignoring its name and menu placement. */
const body = (row: Row): string => JSON.stringify(Object.fromEntries(Object.entries(row).filter(([key]) => key !== "name" && key !== "presentation")));
const renderedNames = new Set(rendered.configurations.map(row => row.name));
const renderedBodies = new Set(rendered.configurations.map(body));
const owned = new Set(seed.configurations.filter((row): row is Row => typeof row === "object").map(row => row.name as string));
const lines = raw.split("\n");
const compoundsAt = lines.indexOf('  "compounds": [');
const inputsAt = lines.indexOf('  "inputs": [');
if (lines[1] !== '  "version": "0.2.0",' || lines[2] !== '  "configurations": [' || compoundsAt === -1 || inputsAt < compoundsAt) throw new Error("seed layout drifted");
/** 📍️ Finds the closing line of the configuration row that carries a name. */
function rowEnd(name: string): number {
  const needle = `      "name": ${JSON.stringify(name)}`;
  const at = lines.findIndex((line, index) => index < compoundsAt && (line === needle || line === `${needle},`));
  if (at === -1) throw new Error(`seed row not found: ${name}`);
  for (let index = at; index < compoundsAt; index++) if (lines[index] === "    }," || lines[index] === "    }") return index;
  throw new Error(`seed row has no end: ${name}`);
}
/** 🧱️ Renders one row the way the seed writes rows. */
const rowLines = (row: Row, last: boolean): string[] => JSON.stringify(row, null, 2).split("\n").map((line, index, all) => `    ${line}${index === all.length - 1 && !last ? "," : ""}`);
const insertions = new Map<number, string[][]>();
const renames: string[] = [];
const moved: string[] = [];
let anchor: string | undefined;
for (const row of current.configurations) {
  const name = row.name as string;
  if (renderedNames.has(name)) { if (owned.has(name)) anchor = name; continue; }
  if (renderedBodies.has(body(row))) { renames.push(name); continue; }
  const at = anchor === undefined ? 2 : rowEnd(anchor);
  insertions.set(at, [...(insertions.get(at) ?? []), JSON.stringify(row, null, 2).split("\n")]);
  moved.push(name);
}
const renderedInputs = new Set(rendered.inputs.map(row => row.id));
const lostInputs = current.inputs.filter(row => typeof row.id === "string" && !renderedInputs.has(row.id));
const out: string[] = [];
lines.forEach((line, index) => {
  const pending = insertions.get(index);
  if (!pending) { out.push(line); return; }
  if (index === 2) { out.push(line); for (const rowText of pending) out.push(...rowText.map((text, at) => `    ${text}${at === rowText.length - 1 ? "," : ""}`)); return; }
  const closesArray = line === "    }";
  out.push("    },");
  pending.forEach((rowText, at) => out.push(...rowText.map((text, lineAt) => `    ${text}${lineAt === rowText.length - 1 && !(closesArray && at === pending.length - 1) ? "," : ""}`)));
});
if (lostInputs.length) {
  let end = out.indexOf('  "inputs": [');
  while (out[end] !== "  ]," && out[end] !== "  ]") end++;
  if (out[end - 1] !== "    }") throw new Error("seed inputs do not end with a row");
  out[end - 1] = "    },";
  out.splice(end, 0, ...lostInputs.flatMap((row, at) => rowLines(row, at === lostInputs.length - 1)));
}
const next = out.join("\n");
const after = Bun.JSONC.parse(next) as { configurations: (Row | string)[]; inputs: Row[] };
assertLaunchSeedPlacement(after);
if (after.configurations.length !== seed.configurations.length + moved.length || after.inputs.length !== seed.inputs.length + lostInputs.length) throw new Error("row counts do not match the insertions");
const kept = new Set(after.configurations.map(row => JSON.stringify(row)));
if (seed.configurations.some(row => !kept.has(JSON.stringify(row)))) throw new Error("a seed row changed");
console.log(`${check ? "would move" : "moved"} ${moved.length} configuration row(s) and ${lostInputs.length} input(s) into the seed; ${renames.length} rename(s) skipped`);
console.log(`renames skipped: ${renames.join(" | ") || "-"}`);
console.log(`inputs: ${lostInputs.map(row => row.id).join(", ") || "-"}`);
if (!check && (moved.length || lostInputs.length)) {
  if ((await Bun.file(seedPath).text()) !== raw) throw new Error("seed changed while reconciling; re-run");
  await Bun.write(seedPath, next);
}
