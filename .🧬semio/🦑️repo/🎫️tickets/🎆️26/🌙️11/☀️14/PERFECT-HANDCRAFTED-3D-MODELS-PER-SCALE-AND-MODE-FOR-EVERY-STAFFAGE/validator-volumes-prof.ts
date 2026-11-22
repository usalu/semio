/** ⏱️ Scratch harness for the speed of the measured volume rules (`checkVolumes`): freezes an asset as the tools read it and times every phase of one run.
 *
 * Usage: `bun validator-volumes-prof.ts freeze <category>/<slug>` writes the catalog asset with the `T/volumes` source merged over it to `🗑️generated/validator/sp-<slug>/🔣️.json` (a frozen input for `--file`);
 * `bun validator-volumes-prof.ts run <asset document json> <out json> [--scale N] [--rules a,b] [--measure]` runs `checkVolumes` once (with `--measure`: `measureVolumes`, every measurement of every rule at full precision instead of the findings), prints wall and CPU seconds per `operation.progress` label and writes the findings (the `VOLUMES` lines of `r3-validate.ts` and every failing measurement at full precision) to `<out json>`.
 * @see ./validator.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

import { checkVolumes, measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { GENERATED, type Json } from "./volumes-lib.ts";
import { load } from "./r3-lib.ts";

const USAGE = "validator-volumes-prof.ts freeze <category>/<slug> | run <asset document json> <out json> [--scale N] [--rules a,b] [--measure]";
const [mode, first, second] = process.argv.slice(2);
const option = (name: string): string | undefined => (process.argv.indexOf(name) < 0 ? undefined : process.argv[process.argv.indexOf(name) + 1]);

if (mode === "freeze" && first !== undefined) {
  const { entry, asset } = load(first, USAGE);
  const document = JSON.parse(readFileSync(entry.path, "utf8"));
  const path = join(GENERATED, "validator", `sp-${entry.slug}`, "🔣️.json");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify({ ...document, asset }));
  console.log(path);
} else if (mode === "run" && first !== undefined && second !== undefined) {
  const asset: Json = JSON.parse(readFileSync(resolve(first), "utf8")).asset;
  const phases: { label: string; wall: number; cpu: number }[] = [];
  let [label, wall, cpu] = ["analytic rules", performance.now(), process.cpuUsage()];
  const [wall0, cpu0] = [wall, cpu];
  const close = (next: string): void => {
    const used = process.cpuUsage(cpu);
    phases.push({ label, wall: (performance.now() - wall) / 1000, cpu: (used.user + used.system) / 1e6 });
    [label, wall, cpu] = [next, performance.now(), process.cpuUsage()];
  };
  const scale = option("--scale");
  const rules = option("--rules");
  const findings = (process.argv.includes("--measure") ? measureVolumes : checkVolumes)(asset as never, { ...(scale === undefined ? {} : { scale: Number(scale) }), ...(rules === undefined ? {} : { rules: rules.split(",") }) } as never, { progress: (_done: number, _total: number, text: { en: string }) => close(text.en) } as never) as readonly Json[];
  close("end");
  for (const p of phases) console.log(`${p.label.padEnd(20)} wall ${p.wall.toFixed(1).padStart(7)} s  cpu ${p.cpu.toFixed(1).padStart(7)} s`);
  const total = process.cpuUsage(cpu0);
  console.log(`${"total".padEnd(20)} wall ${((performance.now() - wall0) / 1000).toFixed(1).padStart(7)} s  cpu ${((total.user + total.system) / 1e6).toFixed(1).padStart(7)} s  findings ${findings.length}`);
  mkdirSync(dirname(resolve(second)), { recursive: true });
  writeFileSync(resolve(second), JSON.stringify({ lines: process.argv.includes("--measure") ? [] : findings.map((f) => `VOLUMES ${f.rule}${f.scale === undefined ? "" : ` 1:${f.scale}`} ${f.code} ${f.path}: ${f.message.en}`), findings }, null, 1));
} else {
  process.stderr.write(`usage: ${USAGE}\n`);
  process.exit(2);
}
