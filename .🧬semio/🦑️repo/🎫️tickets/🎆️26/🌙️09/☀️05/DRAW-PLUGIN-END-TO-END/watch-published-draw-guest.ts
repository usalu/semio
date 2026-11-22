/** 🧪️ Verifies the original native guest from the same publication the Draw playground serves. */
import { join } from "node:path";
import { drawPublicationHasCurrentDescriptor } from "./draw-current-publication.ts";

const generated = Bun.argv[2];
if (!generated) throw Error("Expected the Draw ticket generated directory");
const receipt = join(generated, "draw-native-" + (process.env.DRAW_PROBE_COMPONENT_STAGE ?? "component") + ".log");
const deadline = Date.now() + 10_800_000;
const complete = /\[component\] semio:draw: built, described and materialized/;
let ready = false;
while (Date.now() < deadline) {
  const log = Bun.file(receipt);
  if (await log.exists() && complete.test(await log.slice(Math.max(0, log.size - 65_536)).text()) && await drawPublicationHasCurrentDescriptor()) {
    ready = true;
    break;
  }
  await Bun.sleep(2000);
}
if (!ready) throw Error("Draw component publication did not complete");
const processArgs = [
  "repo", "tools", "exec", "--light", "bun", "--", join(generated, "../draw-native-dev.ts"),
  generated, process.env.DRAW_PROBE_GUEST_STAGE ?? "guest-instance-check-current", "🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/📜️script.ts", "guest-instance-check",
];
const child = Bun.spawn(processArgs, { stdout: "inherit", stderr: "inherit" });
const interrupt = () => child.kill("SIGINT");
const terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
process.exitCode = await child.exited.finally(() => {
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
});
