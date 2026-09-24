/** 🔎️ C10 probe: user1 opens a space, opens Create Artifact, dumps the kind picker options' DOM attributes. */
import { activate, boot, dialog, openSessions, signIn } from "./c10-lib.mjs";
const space = process.argv[2];
const { browser, sessions } = await openSessions(["http://127.0.0.1:6520/"]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.goto(new URL(`/spaces/${space}`, A.url).href, { waitUntil: "domcontentloaded" });
await A.page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 120000 });
await activate(A.page, "s-space-create-artifact");
await dialog(A.page).waitFor({ state: "visible", timeout: 15000 });
await A.page.locator('[id="kindChoice"]').click();
await A.page.waitForTimeout(800);
console.log(JSON.stringify(await A.page.getByRole("option").evaluateAll((els) => els.map((el) => [...el.attributes].map((a) => `${a.name}=${a.value.slice(0, 160)}`).join(" ") + " text=" + el.textContent)), null, 1));
await browser.close();
