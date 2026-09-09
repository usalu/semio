import assert from "node:assert/strict";
import { join } from "node:path";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const serverPath = join(root, "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/📜️script.ts");
const child = Bun.spawn([Bun.argv[0]!, serverPath], { env: { ...Bun.env, SEMIO_JCO_PROBE_PORT: "0" }, stdout: "pipe", stderr: "inherit" });
const reader = child.stdout.getReader();
const timeout = setTimeout(() => child.kill(), 30_000);
try {
  let output = "";
  while (!output.includes("http://localhost:")) {
    const item = await reader.read();
    if (item.done) throw new Error("JCO server ended before readiness");
    output += new TextDecoder().decode(item.value);
  }
  const url = output.match(/http:\/\/localhost:\d+/)![0];
  const response = await fetch(url + "/");
  const text = await response.text();
  console.log(`[DEBUG] actual JCO root status=${response.status} HTML=${text.startsWith("<!doctype html>")}`);
  assert.equal(response.status, 200);
  assert.match(text, /new Worker/);
  const references = [...text.matchAll(/new Worker\("([^"]+)"/g)].map(row => row[1]!);
  for (const path of references) {
    const worker = await fetch(new URL(path, url + "/"));
    assert.equal(worker.status, 200);
    assert.match(worker.headers.get("content-type")!, /javascript/);
    const source = await worker.text();
    const module = source.match(/from "([^"]+)"/)![1]!;
    const example = await fetch(new URL(module, worker.url));
    assert.equal(example.status, 200);
    for (const row of (await example.text()).matchAll(/from '([^']+)'/g)) {
      const support = await fetch(new URL(row[1]!, example.url));
      assert.equal(support.status, 200);
    }
  }
  console.log("[DEBUG] actual JCO HTML, worker, guest example and four support imports resolve");
} finally {
  clearTimeout(timeout);
  child.kill();
  await child.exited;
  reader.releaseLock();
}
