/** 🔎️ C10 s12 probe: user1 opens space <spaceId> on <url>, activates Create Artifact and dumps the dialog's text and fields
 * over 30 s — the miss where the dialog opens without its kind picker. */
import { activate, boot, dialog, openSessions, signIn } from "./c10-lib.mjs";
import { openSpace } from "./c10-journey.mjs";
const [url, spaceId] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(5_000);
await openSpace(A, spaceId);
await A.page.waitForTimeout(3_000);
await activate(A.page, "s-space-create-artifact");
await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
for (let second = 0; second <= 30; second += 5) {
  const content = await dialog(A.page).evaluate((element) => ({ text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 400), ids: [...element.querySelectorAll("[id]")].map((node) => node.id).slice(0, 30) })).catch((error) => ({ error: String(error) }));
  console.log(`+${second}s`, JSON.stringify(content));
  await A.page.waitForTimeout(5_000);
}
console.log(A.lines.filter((line) => /error|warn|artifact-creations|creation/iu.test(line) && !/59773\/bridge/u.test(line)).map((line) => line.slice(0, 300)).join("\n"));
await browser.close();
