/** 🔎️ C10 s12 probe: user1 opens the share dialog of its first hub space row and dumps the dialog's controls and role options. */
import { boot, clickRowAction, dialog, openSessions, rowIds, signIn } from "./c10-lib.mjs";
const { browser, sessions } = await openSessions([process.argv[2]]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(5000);
const id = [...(await rowIds(A.page, "space"))].find((key) => key !== "space:default").slice(6);
console.log("row", id, await clickRowAction(A.page, "space", id, /^(share|teilen)\b/iu));
await dialog(A.page).waitFor({ state: "visible", timeout: 20000 });
console.log(JSON.stringify(await dialog(A.page).evaluate((root) => [...root.querySelectorAll("input,button,select,[role=combobox]")].map((el) => [...el.attributes].map((a) => `${a.name}=${a.value.slice(0, 100)}`).join(" ") + " text=" + (el.textContent ?? "").trim().slice(0, 40))), null, 1));
for (const trigger of await dialog(A.page).locator("button[role=combobox], [role=combobox]").all()) {
  await trigger.focus();
  await A.page.keyboard.press("Enter");
  await A.page.waitForTimeout(700);
  console.log("options", JSON.stringify(await A.page.locator('[role="option"]').evaluateAll((els) => els.map((el) => `${el.getAttribute("data-value")} | ${(el.textContent ?? "").trim()}`))));
  await A.page.keyboard.press("Escape");
  await A.page.waitForTimeout(300);
}
console.log(A.lines.filter((l) => /refus|fail|invalid|error/i.test(l)).slice(-10).join("\n"));
await browser.close();
