/** 🧽️ Removes launch configuration rows that were duplicated into the `inputs` container of `.vscode/🧩️launch.seed.jsonc`
 * (activation s4-6: `invalid launch seed at $.inputs[21]`). Byte-range removal only: every removed row must deep-equal a row
 * that stays in `configurations`, everything else stays byte-identical. `--check` reports without writing. */
import { assertLaunchSeedPlacement } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧱️placement/🟦️.ts";

const seed = "/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc";
const check = process.argv.includes("--check");
const raw = await Bun.file(seed).text();
type Row = Record<string, unknown>;
const before = Bun.JSONC.parse(raw) as { configurations: unknown[]; inputs: Row[] };
const configured = new Set(before.configurations.filter(row => typeof row === "object").map(row => JSON.stringify(row)));
const misplaced = before.inputs.filter(row => !Object.hasOwn(row, "id"));
if (misplaced.length === 0) { console.log("seed inputs clean"); process.exit(0); }
if (misplaced.some(row => !configured.has(JSON.stringify(row)))) throw new Error("a misplaced row has no deep-equal twin in configurations");
const inputsAt = raw.indexOf('\n  "inputs": [');
if (inputsAt === -1 || raw.indexOf('\n  "inputs": [', inputsAt + 1) !== -1) throw new Error("inputs container anchor is not unique");
const lines = raw.split("\n");
const first = raw.slice(0, inputsAt + 1).split("\n").length;
const kept: string[] = lines.slice(0, first);
let row: string[] = [];
let depth = 0;
let removed = 0;
let index = first;
for (; index < lines.length; index++) {
  const line = lines[index]!;
  if (depth === 0 && line === "  ]," || depth === 0 && line === "  ]") break;
  row.push(line);
  if (line === "    {") depth = 1;
  else if (depth === 1 && (line === "    }," || line === "    }")) {
    depth = 0;
    const value = Bun.JSONC.parse(row.join("\n").replace(/,$/, "")) as Row;
    if (Object.hasOwn(value, "id")) kept.push(...row); else removed++;
    row = [];
  }
}
if (row.length !== 0 || removed !== misplaced.length) throw new Error(`row scan mismatch: removed ${removed}, expected ${misplaced.length}, dangling ${row.length}`);
const tail = lines.slice(index);
if (kept[kept.length - 1] === "    },") kept[kept.length - 1] = "    }";
const next = [...kept, ...tail].join("\n");
const after = Bun.JSONC.parse(next) as { configurations: unknown[]; inputs: Row[] };
assertLaunchSeedPlacement(after);
if (JSON.stringify(after.configurations) !== JSON.stringify(before.configurations)) throw new Error("configurations changed");
if (JSON.stringify(after.inputs) !== JSON.stringify(before.inputs.filter(row => Object.hasOwn(row, "id")))) throw new Error("inputs changed beyond the removal");
if (JSON.stringify({ ...after, inputs: 0 }) !== JSON.stringify({ ...before, inputs: 0 })) throw new Error("document changed outside inputs");
console.log(`${check ? "would remove" : "removed"} ${removed} misplaced configuration rows from inputs (${before.inputs.length} -> ${after.inputs.length}), ${raw.length - next.length} chars`);
if (!check) {
  if ((await Bun.file(seed).text()) !== raw) throw new Error("seed changed while deduplicating; re-run");
  await Bun.write(seed, next);
}
