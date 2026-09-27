/** 🔴️ F3 — red check of the yielding law: the HEAD (pre-F3) async freshness resolver over a tree without an index. */
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const mod = await import(process.argv[2]!);
const resolve = mod.resolveBootSourceContentHashesAsync ?? mod.resolveBootSourceContentHashes;
const sandbox = mkdtempSync(join(tmpdir(), "f3-red-"));
try {
  const sourceRoot = join(sandbox, "source");
  for (let index = 0; index < 1500; index += 1) {
    const directory = join(sourceRoot, `m${index % 25}`);
    mkdirSync(directory, { recursive: true });
    writeFileSync(join(directory, `f${index}.rs`), `fn f${index}() { ${"x".repeat(index % 97)} }`);
  }
  let ticks = 0;
  const ticker = setInterval(() => { ticks += 1; }, 0);
  const started = performance.now();
  const pending = resolve({ sourceRoot, moduleDirectory: join(sandbox, "staged") });
  const synchronousMs = performance.now() - started;
  await pending;
  const totalMs = performance.now() - started;
  clearInterval(ticker);
  const share = synchronousMs / totalMs;
  console.log(`${share <= 0.1 && ticks > 0 ? "PASS" : "FAIL"} synchronous ${synchronousMs.toFixed(1)} of ${totalMs.toFixed(1)} ms (share ${share.toFixed(3)}, bound 0.1), timer ticks ${ticks}`);
} finally {
  rmSync(sandbox, { recursive: true, force: true });
}
