/** 🔍️ WG11 exploration (ticket-local, expendable): what the wasm32 shell exposes for one open hub document after unfolding its
 * window's Actions pane and opening the History panel — node keys + the structure dump's text.
 * Usage: bun wg11-explore-wasm32.ts <serveUrl> <hub> <space> <document> <windowCamel> <tag>   (credentials from the environment) */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { hubCollaborationHumans, Wasm32Shell } from "./harness/🤝️hub-collaboration/🟦️.ts";

const [serve, hub, space, document, windowCamel, tag] = process.argv.slice(2);
const out = (name: string) => join(import.meta.dir, "generated", `explore-${tag}-${name}`);
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const started = Date.now();
try {
  const shell = await Wasm32Shell.boot(browser, hubCollaborationHumans()[0], serve!, hub!, "en", () => Date.now() - started, () => document!);
  console.log("signIn", JSON.stringify(await shell.signIn()));
  console.log("attach", JSON.stringify(await shell.attach(space!, document!)));
  await shell.page.waitForTimeout(15_000);
  const structure = await shell.page.evaluate(async () => (await (globalThis as any).semioWgpuIntrospection?.dumpStructure?.()) ?? "");
  writeFileSync(out("structure.json"), structure);
  console.log("structure bytes", structure.length, (structure.match(/Handle Kind[^"]*/gu) ?? []).slice(0, 5));
  console.log("actions", await shell.activate(`framework.window.${windowCamel}.engagement.toggle`, 3_000));
  writeFileSync(out("projection-actions.json"), JSON.stringify(await shell.projection(), null, 1));
  console.log("history", await shell.activate("framework.panel.history", 3_000));
  writeFileSync(out("projection-history.json"), JSON.stringify(await shell.projection(), null, 1));
  await shell.page.screenshot({ path: out("screen.png") });
  writeFileSync(out("console.txt"), shell.lines.join("\n"));
} catch (error) {
  console.log("ERROR", String(error).slice(0, 600));
} finally {
  await browser.close();
}
