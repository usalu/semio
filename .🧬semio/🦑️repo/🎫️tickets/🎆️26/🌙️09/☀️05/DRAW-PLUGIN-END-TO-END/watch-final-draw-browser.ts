/** 🧪️ Runs both localized editor journeys against the activated final Draw component. */
import { join } from "node:path";
import { drawPublicationHasCurrentDescriptor } from "./draw-current-publication.ts";

const generated = Bun.argv[2];
if (!generated) throw Error("Expected the Draw ticket generated directory");
const root = process.cwd();
const stage = join(generated, "draw-native-component-final.log");
const deadline = Date.now() + 10_800_000;
let expected: { wasmSha256: string; semanticDescriptorSha256: string } | undefined;
while (Date.now() < deadline) {
  const driver = Bun.file(stage);
  if (await driver.exists() && /\[component\] semio:draw: built, described and materialized/.test(await driver.slice(Math.max(0, driver.size - 65_536)).text()) && await drawPublicationHasCurrentDescriptor()) {
    expected = await Bun.file(join(root, ".cache/b/p/m/browser/prototype/modules/🖍️draw/🧾️receipt.json")).json();
    break;
  }
  await Bun.sleep(2000);
}
if (!expected) throw Error("Registered final Draw publication was not observed");

async function run(args: string[], environment: Record<string, string> = {}) {
  const task = Bun.spawn(["repo", ...args], { env: { ...process.env, ...environment }, stdout: "inherit", stderr: "inherit" });
  const interrupt = () => task.kill("SIGINT");
  const terminate = () => task.kill("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const status = await task.exited.finally(() => {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  });
  if (status) throw Error("Draw final stage exited " + status + ": " + args.join(" "));
}

const tool = ["tools", "exec", "--light", "bun", "--"];
const launcher = join(generated, "../draw-native-dev.ts");
const dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";
for (const phase of ["prepare", "activate"]) {
  await run([...tool, launcher, generated, phase + "-final", dev, phase, "draw", "react", "prototype"]);
}
const receiptPath = ["🔌️plugin-modules", "🖍️draw", "🧾️receipt.json"].map(encodeURIComponent).join("/");
while (Date.now() < deadline) {
  for (const port of [6064, 16064]) {
    const origin = "http://127.0.0.1:" + port;
    let served: typeof expected;
    try {
      const result = await fetch(origin + "/" + receiptPath, { signal: AbortSignal.timeout(3000), cache: "no-store" });
      if (result.ok) served = await result.json();
    } catch {}
    if (served?.wasmSha256 !== expected.wasmSha256 || served?.semanticDescriptorSha256 !== expected.semanticDescriptorSha256) continue;
    console.log("[DEBUG] Draw browser serves the final component receipt", origin, expected.wasmSha256);
    await run([...tool, join(generated, "../watch-draw-browser.ts"), generated, String(port)], { DRAW_BROWSER_STAGE: "final" });
    process.exit(0);
  }
  await Bun.sleep(2000);
}
throw Error("No live Draw playground served the final component");
