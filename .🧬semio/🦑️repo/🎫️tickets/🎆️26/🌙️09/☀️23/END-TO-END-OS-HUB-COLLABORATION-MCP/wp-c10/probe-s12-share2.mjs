/** 🔎️ C10 s12 probe: user1 shares its first hub space with user2 as author and decodes every worker request the Shell
 * posted meanwhile, to name the directory command the share dialog produced. */
import { boot, clickRowAction, dialog, openSessions, rowIds, selectOption, signIn, submitDialog } from "./c10-lib.mjs";
import { decodeBackboneWorkerRequest } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
const { browser, sessions } = await openSessions([process.argv[2]]);
const [A] = sessions;
await A.page.addInitScript(() => {
  globalThis.__c10posted = [];
  const original = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (message, ...rest) {
    if (message && message.wire instanceof Uint8Array) globalThis.__c10posted.push(Array.from(message.wire));
    return original.call(this, message, ...rest);
  };
});
await boot(A);
await signIn(A);
await A.page.waitForTimeout(5000);
const id = [...(await rowIds(A.page, "space"))].find((key) => key !== "space:default").slice(6);
await clickRowAction(A.page, "space", id, /^(share|teilen)\b/iu);
await dialog(A.page).waitFor({ state: "visible", timeout: 20000 });
await A.page.evaluate(() => (globalThis.__c10posted.length = 0));
await A.page.locator("#email").fill("user2@semio.dev");
await selectOption(A.page, "role", /author/i);
await submitDialog(A.page);
await A.page.waitForTimeout(4000);
for (const wire of await A.page.evaluate(() => globalThis.__c10posted)) {
  try {
    const request = decodeBackboneWorkerRequest(Uint8Array.from(wire));
    if (/directory|command/i.test(request.kind)) console.log(JSON.stringify(request).slice(0, 600));
  } catch (error) {
    console.log("undecodable", String(error).slice(0, 200));
  }
}
console.log(A.lines.filter((l) => /refus|fail|invalid|error/i.test(l)).slice(-10).join("\n"));
await browser.close();
