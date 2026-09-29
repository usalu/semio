/** 🎬️ Site walk against a running site (default the dev site on 6061 with the dev proctor behind it): introduction → pseudonym →
 * a perfect physics run → submit → results → leaderboard → a second device (empty storage) recalling the same pseudonym with
 * different case and spacing. Saves one screenshot per step and every console error / failed request into
 * `🗑️generated/site-infra-screens-<width>/`, flagging horizontal overflow. Certificate errors are ignored for a local stack
 * behind Caddy's internal CA. `bun site_infra_e2e.ts [origin] [WxH]`. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Page } from "playwright";

const origin = process.argv[2] ?? "http://localhost:6061";
const [width, height] = (process.argv[3] ?? "1280x900").split("x").map(Number) as [number, number];
const out = join(import.meta.dir, "🗑️generated", `site-infra-screens-${width}`);
mkdirSync(out, { recursive: true });
const handle = `Walk ${Date.now().toString(36)}`;
const problems: string[] = [];
const browser = await chromium.launch({ channel: "chrome" });
const shot = async (page: Page, name: string) => { const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth); if (overflow > 0) problems.push(`${name}: horizontal overflow ${overflow}px at ${width}px`); await page.screenshot({ path: join(out, `${name}.png`), fullPage: true }); console.log(`[DEBUG] ${name}`); };
const watch = (page: Page, label: string) => {
  page.on("console", (message) => { if (message.type() === "error") problems.push(`${label} console: ${message.text()}`); });
  page.on("requestfailed", (request) => problems.push(`${label} failed: ${request.method()} ${request.url()} ${request.failure()?.errorText}`));
  page.on("response", (response) => { if (response.status() >= 400) problems.push(`${label} ${response.status()}: ${response.request().method()} ${response.url()}`); });
};
const sortOrder = (page: Page) => page.$$eval("main button[aria-label^='Move '][aria-label$=' up']", (buttons) => buttons.map((button) => button.getAttribute("aria-label")!.slice(5, -3)));
const sortBy = async (page: Page, ascending: readonly string[]) => {
  const present = await sortOrder(page);
  const target = ascending.filter((label) => present.includes(label));
  if (target.length !== present.length) throw new Error(`unknown sorting items: ${present.filter((label) => !target.includes(label)).join(", ")}`);
  for (let index = 0; index < target.length; index++) {
    while ((await sortOrder(page)).indexOf(target[index]!) > index) await page.getByRole("button", { name: `Move ${target[index]} up`, exact: true }).click();
  }
};

const first = await (await browser.newContext({ viewport: { width, height }, locale: "en-GB", ignoreHTTPSErrors: true })).newPage();
watch(first, "device-1");
await first.goto(origin);
await first.getByRole("button", { name: "Continue" }).waitFor();
await shot(first, "01-introduction");
await first.getByRole("button", { name: "Continue" }).click();
await first.getByRole("radio", { name: "Pseudonym" }).check();
await shot(first, "02-identity");
await first.getByRole("textbox").fill(handle);
await first.getByRole("button", { name: "Continue" }).click();
await first.getByRole("button", { name: "Start quiz" }).first().waitFor();
if (!(await first.locator("main").innerText()).includes(handle)) throw new Error(`home does not greet ${handle}`);
await shot(first, "03-home");
await first.getByRole("button", { name: "Start quiz" }).first().click();
await first.getByText("Task 1 of 3").waitFor();
const energy = ["Annual electricity use", "Food energy", "Annual yield", "Usable capacity", "Energy content", "Primary energy demand", "One full smartphone charge", "Annual heating demand"];
const powers = ["Burning tea light (heat)", "Person sitting still (body heat)", "Sunlight on 1 m² at noon on a clear summer day", "Electric kettle", "Wall box charging an electric car", "Design heating load of an unrenovated 1960s single-family house", "Car engine at full throttle (136 PS)", "Modern onshore wind turbine at rated wind speed", "ICE 3 high-speed train at full power", "Nuclear power plant unit (electrical, Isar 2)", "Germany's average electricity consumption", "Humanity's average primary energy use", "Sunlight intercepted by the Earth", "Total radiant power of the Sun"];
const energies = ["One full smartphone charge", "Heating 1 litre of water from 20 °C to boiling", "Food energy of a 100 g chocolate bar", "Daily food energy of an adult", "One litre of heating oil", "Usable capacity of an electric car battery", "A full 50-litre tank of petrol", "Annual electricity use of a two-person household", "Annual heating demand of an unrenovated 1960s single-family house", "Annual yield of a modern onshore wind turbine", "Germany's annual primary energy consumption", "The world's annual primary energy consumption"];
for (let task = 1; task <= 3; task++) {
  await first.getByText(`Task ${task} of 3`).waitFor();
  if ((await first.locator("main select").count()) > 0) {
    for (;;) {
      const open = first.locator("main select").filter({ has: first.locator("option:checked[value='']") }).first();
      if ((await open.count()) === 0) break;
      const label = (await open.getAttribute("aria-label")) ?? "";
      await open.selectOption(energy.some((prefix) => label.includes(prefix)) ? "energy" : "power");
    }
  } else {
    const present = await sortOrder(first);
    await sortBy(first, present.every((label) => powers.includes(label)) ? powers : energies);
  }
  await shot(first, `04-run-task-${task}`);
  if (task < 3) await first.getByRole("button", { name: "Next task →" }).click();
}
await first.getByRole("button", { name: "Submit quiz" }).click();
await first.getByRole("button", { name: "Submit now" }).click();
await first.getByText("Your score:").waitFor();
await shot(first, "07-results");
console.log(`[DEBUG] score line: ${await first.getByText("Your score:").innerText()}`);
await first.getByRole("button", { name: "Open the leaderboard" }).click();
await first.locator("main table").getByText(handle).waitFor();
await shot(first, "08-leaderboard");

const second = await (await browser.newContext({ viewport: { width, height }, locale: "de-DE", ignoreHTTPSErrors: true })).newPage();
watch(second, "device-2");
await second.goto(origin);
await second.getByRole("button", { name: "Weiter" }).waitFor();
await shot(second, "09-second-device-introduction-de");
await second.getByRole("button", { name: "Weiter" }).click();
await second.getByRole("radio", { name: "Pseudonym" }).check();
await second.getByRole("textbox").fill(`  ${handle.toUpperCase().replace(" ", "   ")} `);
await second.getByRole("button", { name: "Weiter" }).click();
await second.getByRole("button", { name: "Letztes Ergebnis ansehen" }).waitFor();
if (!(await second.locator("main").innerText()).includes(handle)) throw new Error(`second device does not greet ${handle}`);
await shot(second, "10-second-device-recalled-de");
const learners = await Promise.all([first, second].map((page) => page.evaluate(() => JSON.parse(localStorage.getItem("semio.quiz.architecture.learner") ?? "null")?.id)));
console.log(`[DEBUG] learner ids: ${learners.join(" ")} same=${learners[0] === learners[1]}`);
writeFileSync(join(out, "problems.txt"), problems.join("\n") + "\n");
console.log(`[DEBUG] problems: ${problems.length}\n${problems.join("\n")}`);
await browser.close();
