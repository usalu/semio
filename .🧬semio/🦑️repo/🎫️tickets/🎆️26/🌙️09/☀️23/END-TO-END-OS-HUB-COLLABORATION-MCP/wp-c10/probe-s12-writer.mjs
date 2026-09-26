/** 🔎️ C10 s12 probe: user1 creates (or opens) a writer document on the given serve and reports what its window body is
 * made of (editable surfaces, tag census, execution-target notices) every 15 s for up to 4 minutes. */
import { activate, boot, dialog, openSessions, read, selectOption, signIn, submitDialog } from "./c10-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, openSpace, until } from "./c10-journey.mjs";
const [url] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(5000);
const name = `C10 Writer ${Date.now() % 100000}`;
await activate(A.page, "s-home-create-space");
await dialog(A.page).waitFor({ state: "visible", timeout: 20000 });
await A.page.locator("#name").fill(name);
await selectOption(A.page, "kind", /studio/i);
await selectOption(A.page, "visibility", /public/i);
await submitDialog(A.page);
const spaceId = await until(async () => A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key")?.slice(6) ?? null, name), 90000);
await openSpace(A, spaceId);
const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === "text.document");
const artifactId = await createArtifact(A, "Writer probe", kind);
console.log("space", spaceId, "artifact", artifactId);
for (let tick = 0; tick < 16; tick += 1) {
  await A.page.waitForTimeout(15000);
  const state = await read(A.page);
  const census = await A.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => {
    const tags = {};
    for (const element of body.querySelectorAll("*")) tags[element.tagName.toLowerCase()] = (tags[element.tagName.toLowerCase()] ?? 0) + 1;
    return { window: body.closest("[data-window-id]")?.getAttribute("data-window-id"), editable: body.querySelectorAll('textarea, [contenteditable="true"], input[type=text]').length, slots: [...new Set([...body.querySelectorAll("[data-slot]")].map((e) => e.getAttribute("data-slot")))].slice(0, 20), tags };
  }));
  console.log(`t+${(tick + 1) * 15}s`, JSON.stringify({ exec: state.executionTarget, windows: state.windows, census }).slice(0, 1500));
  if (census.some((body) => body.editable > 0)) break;
}
console.log(A.lines.filter((l) => /error|refus|fault|warn/i.test(l) && !/typed-operation|bridge|u5 focused|stderr|Failed to load/.test(l)).slice(-15).join("\n"));
await browser.close();
