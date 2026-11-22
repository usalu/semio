/** 🕰️ Waits for a newly published Note guest before verifying Pencil and both erasers through dev. */
import { join } from "node:path";

const receiptPath = ".cache/b/p/m/browser/prototype/modules/🗒️note/🧾️receipt.json";
const baseline = process.env.SEMIO_NOTE_WATCH_FROM;
if (!baseline) throw Error("Provide the canonical Note source digest to wait beyond");
const expires = Date.now() + Number(process.env.SEMIO_NOTE_WATCH_SECONDS ?? "1800") * 1000;
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6080/?plugin=note&role=editor";
const runner = join(import.meta.dir, "🐍️note-acceptance.mjs");

while (Date.now() < expires) {
  const current = await Bun.file(receiptPath).json().catch(() => undefined);
  if (current?.sourceTreeDigest && current.sourceTreeDigest !== baseline) {
    const ready = await fetch(url, { signal: AbortSignal.timeout(3000) }).catch(() => undefined);
    if (ready?.ok && (ready.headers.get("Content-Type") ?? "").includes("text/html")) {
      console.log("[DEBUG] Canonical Note receipt changed", current.sourceTreeDigest, current.wasmSha256, current.wasmBytes);
      const environment = {
        ...process.env, SEMIO_PROBE_URL: url, SEMIO_PROBE_PENCIL_ONLY: "1", SEMIO_PROBE_ERASERS: "1",
        SEMIO_PROBE_SETTLE: "55", SEMIO_PROBE_NATIVE_REFUSAL: "1", SEMIO_PROBE_WASM_STACK: "1",
        SEMIO_PROBE_OUT: process.env.SEMIO_PROBE_OUT ?? "note-published-erasers",
      };
      delete environment.SEMIO_PROBE_LINKED_CAPTURE;
      delete environment.SEMIO_PROBE_PICKER_EXPERIMENT;
      const browser = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", runner], {
        env: environment, stdout: "inherit", stderr: "inherit",
      });
      process.exit(await browser.exited);
    }
  }
  await Bun.sleep(2500);
}

console.log("[DEBUG] No newly published Note guest became reachable before the observation window ended");
process.exit(3);
