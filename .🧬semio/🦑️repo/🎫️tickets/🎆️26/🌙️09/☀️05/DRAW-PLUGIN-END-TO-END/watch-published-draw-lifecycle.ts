/** 🧪️ Runs the original browser lifecycle from the exact registered Draw browser publication. */
import { join } from "node:path";
import { drawPublicationHasCurrentDescriptor } from "./draw-current-publication.ts";

const [generated, browsers] = Bun.argv.slice(2);
if (!generated || !browsers) throw Error("Expected the Draw ticket output and managed Chromium runtime");
const root = process.cwd();
const driverLog = join(generated, "draw-native-" + (process.env.DRAW_PROBE_COMPONENT_STAGE ?? "component") + ".log");
const modules = join(root, ".cache", "b", "p", "m", "browser", "prototype", "modules");
const deadline = Date.now() + 10_800_000;
for (;;) {
  const driver = Bun.file(driverLog);
  if (await driver.exists() && /\[component\] semio:draw: built, described and materialized/.test(await driver.slice(Math.max(0, driver.size - 65_536)).text()) && await drawPublicationHasCurrentDescriptor()) break;
  if (Date.now() >= deadline) throw Error("Registered Draw module publication remained pending");
  await Bun.sleep(2000);
}
const runner = join(generated, "../probe-linked-draw-lifecycle.ts");
const child = Bun.spawn(["repo", "capacity", "run", "--lang=en", "--memory=1300", "--cpu=1", "--", "repo", "tools", "exec", "--light", "bun", "--", runner, modules], {
  cwd: root, env: { ...process.env, DRAW_PROBE_PUBLISHED: "true", PLAYWRIGHT_BROWSERS_PATH: browsers },
  stdout: "inherit", stderr: "inherit",
});
const interrupt = () => child.kill("SIGINT");
const terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
process.exitCode = await child.exited.finally(() => {
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
});
