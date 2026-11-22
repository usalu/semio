/** 📦️ Verifies the exact Draw artifact released by the final registered build. */
import { join } from "node:path";
import { drawPublicationHasCurrentDescriptor } from "./draw-current-publication.ts";

const generated = Bun.argv[2];
if (!generated) throw Error("Expected the Draw ticket generated directory");
const models = join(process.cwd(), ".cache", "b", "p", "m", "browser", "prototype", "components");
const browser = join(models, "../modules/🖍️draw");
const final = join(generated, "draw-native-component-final.log");
const deadline = Date.now() + 10_800_000;
while (Date.now() < deadline) {
  const log = Bun.file(final);
  if (await log.exists() && /\[component\] semio:draw: built, described and materialized/.test(await log.slice(Math.max(0, log.size - 65_536)).text()) &&
      await drawPublicationHasCurrentDescriptor()) {
    const published = await Bun.file(join(browser, "🧾️receipt.json")).json();
    for await (const file of new Bun.Glob("*/🧾️receipt.json").scan({ cwd: models, type: "file" })) {
      const candidate = await Bun.file(join(models, file)).json();
      if (candidate.wasmSha256 !== published.wasmSha256 || candidate.semanticDescriptorSha256 !== published.semanticDescriptorSha256) continue;
      const model = join(models, file, "..");
      const test = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", join(generated, "../verify-published-draw-receipt.ts"), model, browser], { stdout: "inherit", stderr: "inherit" });
      const interrupt = () => test.kill("SIGINT");
      const terminate = () => test.kill("SIGTERM");
      process.once("SIGINT", interrupt);
      process.once("SIGTERM", terminate);
      process.exitCode = await test.exited.finally(() => {
        process.off("SIGINT", interrupt);
        process.off("SIGTERM", terminate);
      });
      console.log("[DEBUG] Draw final model", model);
      break;
    }
    if (process.exitCode !== undefined) break;
  }
  await Bun.sleep(2000);
}
if (process.exitCode === undefined) throw Error("No exact final Draw model matches the browser receipt");
