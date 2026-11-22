/** 🔬 Samples the isolated native conflict case while its retained close has a stable retirement quote. */
import { join } from "node:path";

const [generated, binary] = process.argv.slice(2);
const peer = Bun.spawn([binary, "conflict_history_edits_end_to_end", "--nocapture"], {
  env: { ...process.env, SEMIO_LANGUAGE: "en", RUST_MIN_STACK: "67108864", SEMIO_XLSX_ARCHIVE_PROGRESS: "1", RUST_BACKTRACE: "1" },
  stdout: "inherit", stderr: "inherit"
});
const finished = peer.exited.then(() => true);
for (const [index, pause] of [9000, 9000, 9000, 7000].entries()) {
  let timer = null;
  const elapsed = new Promise(resolve => { timer = setTimeout(() => resolve(false), pause); });
  if (await Promise.race([finished, elapsed])) { clearTimeout(timer); break; }
  const sample = Bun.spawn(["/usr/bin/sample", String(peer.pid), "2", "5", "-file", join(generated, "conflict-close-" + index + ".stack")], {stdout:"ignore",stderr:"inherit"});
  await sample.exited;
}
process.exit(await peer.exited);
