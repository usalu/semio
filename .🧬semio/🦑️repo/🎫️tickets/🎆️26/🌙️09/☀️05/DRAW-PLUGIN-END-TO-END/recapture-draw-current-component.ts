/** 🔄️ Produces the final Draw component after the earlier native publisher has released its leases. */
import { join } from "node:path";
const [generated, previousPid, recoverOnly] = Bun.argv.slice(2);
const pid = Number(previousPid);
if (!generated || !Number.isSafeInteger(pid) || pid <= 1) throw Error("Expected the ticket output and previous Draw producer");
const deadline = Date.now() + 10_800_000;
while (Date.now() < deadline) {
  try { process.kill(pid, 0); }
  catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ESRCH") break;
    throw error;
  }
  await Bun.sleep(2000);
}
if (Date.now() >= deadline) throw Error("The preceding Draw compiler still owns its native stage");
if (recoverOnly === "recover") {
  const prior = Bun.file(join(generated, "draw-native-component-final.log"));
  if (await prior.exists() && (await prior.slice(Math.max(0, prior.size - 65_536)).text()).includes("[component] semio:draw: built, described and materialized")) process.exit(0);
}
console.log("[DEBUG] Draw component is recapturing the current owned export source and close policy");
const launcher = join(generated, "../draw-native-dev.ts");
const driver = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts";
const manifest = "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml";
const child = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", launcher, generated, "component-final", driver, "component", "--manifest", manifest], { stdout: "inherit", stderr: "inherit" });
const interrupt = () => child.kill("SIGINT");
const terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
process.exitCode = await child.exited.finally(() => {
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
});
