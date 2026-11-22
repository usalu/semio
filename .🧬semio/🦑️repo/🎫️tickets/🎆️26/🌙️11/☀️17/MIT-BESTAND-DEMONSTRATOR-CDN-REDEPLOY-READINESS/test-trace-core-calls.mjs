/** 🧪 Check diagnostic instrumentation against the original guest through V8. */
import assert from "node:assert/strict";
import {execFileSync} from "node:child_process";
import {mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {fileURLToPath} from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(join(ticket, "trace-core-law.json")));
const scratch = mkdtempSync(join(ticket, "🗑️generated", "trace-law-"));
try {
  const core = join(scratch, "baseline.wasm");
  writeFileSync(core, Buffer.from(fixture.coreHex, "hex"));
  const selected = join(scratch, "selected.json");
  writeFileSync(selected, JSON.stringify(fixture.selected));
  for (const mode of ["baseline", "entries", "selected"]) {
    const file = join(scratch, mode + ".wasm");
    if (mode !== "baseline") {
      const args = ["tools", "exec", "--light", "python3", "--", join(ticket, "trace-core-calls.py"), core, file];
      if (mode === "selected") args.push(selected);
      execFileSync("repo", args);
    }
    const {instance} = await WebAssembly.instantiate(readFileSync(file));
    const {increment, mem, tracePtr, traceMem} = instance.exports;
    const originalMemory = new Uint32Array(mem.buffer);
    originalMemory[1] = 932;
    if (tracePtr) tracePtr.value = 4;
    assert.deepEqual(fixture.calls.map(value => increment(value)), fixture.expected);
    assert.equal(originalMemory[1], 932);
    if (tracePtr) {
      const result = Array.from(new Uint32Array(traceMem.buffer, 4, tracePtr.value / 4 - 1));
      assert.deepEqual(result, mode === "selected" ? fixture.selectedTrace : fixture.entryTrace);
    }
  }
  console.log("Original guest, full entry trace and selected argument trace verified.");
} finally {
  rmSync(scratch, {recursive: true, force: true});
}
