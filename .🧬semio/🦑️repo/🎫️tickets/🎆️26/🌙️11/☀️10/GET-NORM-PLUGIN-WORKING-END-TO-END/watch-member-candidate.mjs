import { createReadStream } from "node:fs";
import { access, readdir, stat } from "node:fs/promises";
import { resolve, join } from "node:path";
import { spawn } from "node:child_process";
import { setTimeout } from "node:timers/promises";

const generation = process.argv[2];
if (!/^\w+$/u.test(generation ?? "")) throw new Error("Expected one component generation ID");
const run = resolve(".cache/b/r/m/components/runs", generation);
const marker = '"name":"semio_s_artifact_norm_din18599_editor"';
const offsets = new Map();

async function candidate(directory) {
  for (const name of await readdir(directory).catch(() => [])) {
    if (!/^cargo-prototype-.*\.jsonl$/u.test(name)) continue;
    const log = join(directory, name);
    const size = (await stat(log)).size;
    const start = offsets.get(log) ?? 0;
    if (size <= start) continue;
    let read = start;
    let unfinished = Buffer.alloc(0);
    for await (const chunk of createReadStream(log, { start, end: size - 1 })) {
      const bytes = unfinished.length ? Buffer.concat([unfinished, chunk]) : chunk;
      let lineStart = 0;
      for (let end = bytes.indexOf(10); end !== -1; end = bytes.indexOf(10, lineStart)) {
        const line = bytes.subarray(lineStart, end).toString("utf8");
        lineStart = end + 1;
        if (line.includes(marker)) {
          const event = JSON.parse(line);
          if (event.reason === "compiler-artifact" && event.filenames.some(path => path.includes("/wasm32-wasip2/"))) return true;
        }
      }
      read += lineStart;
      unfinished = bytes.subarray(lineStart);
    }
    offsets.set(log, read);
  }
  return false;
}

const deadline = Date.now() + 3 * 60 * 60 * 1_000;
while (Date.now() < deadline) {
  if (await candidate(join(run, "l"))) {
    console.log("[DEBUG] Detected current DIN V 18599 editor WASI cohort", generation);
    const child = spawn("repo", ["tools", "exec", "python3", "--", join(import.meta.dirname, "watch-member-archive.py"), generation], { stdio: "inherit" });
    process.exitCode = await new Promise((resolve, reject) => {
      child.once("error", reject);
      child.once("exit", code => resolve(code ?? 1));
    });
    process.exit();
  }
  if (await access(join(run, "r", "report.md")).then(() => true, () => false)) throw new Error("Build finished without current DIN V 18599 editor WASI cohort: " + generation);
  await setTimeout(5000);
}
throw new Error("Component watcher timed out: " + generation);
