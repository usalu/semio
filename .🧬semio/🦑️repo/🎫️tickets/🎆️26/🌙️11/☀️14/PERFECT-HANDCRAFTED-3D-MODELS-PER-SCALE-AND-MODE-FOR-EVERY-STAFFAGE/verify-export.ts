/** 📦️ Runs the batch exporter for chosen assets exactly as `export-models` does (same parser, same function), without the repo task wrapper. usage: bun verify-export.ts --out dir --asset a,b --view v,w --lang en */
import { exportModels, parseBatchArguments } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📦️batch/🟦️.ts";

const command = parseBatchArguments(process.argv.slice(2));
const controller = new AbortController();
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => controller.abort());
let last = -1;
const started = Date.now();
const { summary } = await exportModels({ ...command, started: (id, index, count) => process.stderr.write(`[asset ${index}/${count}] ${id}\n`) }, {
  signal: controller.signal,
  progress: (done, total, label) => {
    const percent = Math.floor((done / total) * 100);
    if (percent !== last) {
      last = percent;
      process.stderr.write(`[${percent} %] ${label[command.language]}\n`);
    }
  },
});
process.stdout.write(`${command.out}\n${JSON.stringify(summary)}\n`);
process.stderr.write(`[done] ${((Date.now() - started) / 1000).toFixed(1)} s\n`);
