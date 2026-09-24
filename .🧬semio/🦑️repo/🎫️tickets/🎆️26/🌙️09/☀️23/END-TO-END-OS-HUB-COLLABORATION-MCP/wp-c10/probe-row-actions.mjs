/** 🔎️ C10 probe: dumps one Home/Space table row (by `data-ui-node-key`) and every interactive descendant with its title,
 * aria-label, id and bindings, so the collab harness can address row actions by what the shell actually renders. */
import { boot, openSessions, signIn } from "./c10-lib.mjs";
const [key, path = "/"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([`http://127.0.0.1:6520${path}`]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.locator(`[data-ui-node-key="${key}"]`).first().waitFor({ state: "visible", timeout: 90000 });
console.log(JSON.stringify(await A.page.evaluate((needle) => {
  const row = document.querySelector(`[data-ui-node-key="${needle}"]`);
  const attrs = (el) => [...el.attributes].filter((a) => !["class", "style"].includes(a.name)).map((a) => `${a.name}=${a.value.slice(0, 80)}`).join(" ");
  const interactive = [...(row?.querySelectorAll("button,[role=button],[role=menuitem],a,[title],[aria-label],[data-ui-node-key]") ?? [])];
  return { row: row ? attrs(row) : null, rowText: row?.textContent?.slice(0, 200), interactive: interactive.slice(0, 60).map((el) => `${el.tagName} ${attrs(el)} :: ${(el.textContent ?? "").trim().slice(0, 40)}`) };
}, key), null, 1));
await browser.close();
