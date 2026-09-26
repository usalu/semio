/** 🔎️ C10 s12 probe: user1 opens a hub artifact, reads its windows (text, canvas hash, checkin, ledger), dispatches the
 * plugin's rail verb once, and reads again — does the author's own actor-rendered window move with its own edit? */
import { boot, canvasHash, openSessions, read, signIn } from "./c11-lib.mjs";
import { awaitMounted, hubHead, openRow } from "./c11-journey.mjs";
import { clickUncovered, readShell, submitStagedVerb } from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const [url, spaceId, artifactId, verb = "addBlock"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(4000);
await openRow(A, spaceId, artifactId);
try {
  await awaitMounted(A, 120000);
} catch (error) {
  console.log(String(error));
  console.log(A.lines.filter((l) => !/typed-operation slots|bridge|u5 focused/.test(l)).slice(-40).join("\n"));
  await browser.close();
  process.exit(1);
}
const snap = async (label) => {
  const shell = await readShell(A.page);
  const status = await read(A.page);
  const bodies = await A.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => `${body.closest("[data-window-id]")?.getAttribute("data-window-id")}: nodes=${body.querySelectorAll("*").length} canvas=${body.querySelectorAll("canvas").length} svg=${body.querySelectorAll("svg *").length} text=${(body.textContent ?? "").replace(/\s+/g, " ").slice(0, 120)}`));
  console.log(label, JSON.stringify({ checkin: shell.checkin, ledger: shell.ledger.slice(-3).map((e) => e.label), canvas: await canvasHash(A.page), bodies, pill: status.syncPill, head: await hubHead(artifactId) }, null, 1));
};
await snap("before");
console.log("click", await clickUncovered(A.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`));
await A.page.waitForTimeout(1000);
console.log("submit", await submitStagedVerb(A.page, verb));
await A.page.waitForTimeout(6000);
await snap("after");
console.log(A.lines.filter((l) => /refus|fault|error|patch|reject/i.test(l) && !/typed-operation slots|bridge/.test(l)).slice(-15).join("\n"));
await browser.close();
