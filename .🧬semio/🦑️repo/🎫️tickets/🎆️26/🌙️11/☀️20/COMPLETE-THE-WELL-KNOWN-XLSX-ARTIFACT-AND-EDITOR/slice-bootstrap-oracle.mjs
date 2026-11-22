/** 🧪 Verifies the neutral slice restart cases against Node's native ESM loader. */
import assert from "node:assert/strict";
import {spawnSync} from "node:child_process";
import {mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {join} from "node:path";

const [corpusPath, outputRoot] = process.argv.slice(2);
const corpus = JSON.parse(readFileSync(corpusPath, "utf8"));

for (const vector of corpus) {
  const root = mkdtempSync(join(outputRoot, "slice-oracle-"));
  try {
    const consumer = join(root, "consumer.mjs");
    const provider = join(root, "provider.mjs");
    writeFileSync(consumer, 'import "./provider.mjs";');
    if (vector.initialSource !== null) writeFileSync(provider, vector.initialSource);
    const restarts = [];
    let starts = 0;
    let result;
    while (result === undefined) {
      starts++;
      const run = spawnSync(process.execPath, [consumer], {encoding: "utf8", timeout: 5000});
      assert.ifError(run.error);
      if (run.status === 0) result = {exit: null, code: null};
      else if (!run.stderr.includes("ERR_MODULE_NOT_FOUND")) result = {exit: run.status, code: null};
      else if (starts === 3) result = {exit: 75, code: "slice-worker-failing"};
      else {
        restarts.push(75);
        if (vector.repairSource !== null) writeFileSync(provider, vector.repairSource);
      }
    }
    assert.deepEqual({result, starts, restarts}, {result: vector.result, starts: vector.starts, restarts: vector.restarts}, vector.name);
    console.log(vector.name);
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
}
