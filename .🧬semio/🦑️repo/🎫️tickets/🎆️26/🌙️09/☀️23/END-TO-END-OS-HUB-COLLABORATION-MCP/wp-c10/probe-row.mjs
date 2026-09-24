/** 🔎️ C10 probe: dumps the attributes of the Home table row that carries a given text and its ancestors. */
import { boot, openSessions, signIn } from "./c10-lib.mjs";
const text = process.argv[2];
const { browser, sessions } = await openSessions(["http://127.0.0.1:6520/"]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.getByText(text).first().waitFor({ state: "visible", timeout: 90000 });
console.log(JSON.stringify(await A.page.evaluate((needle) => {
  const cell = [...document.querySelectorAll("*")].find((el) => el.children.length === 0 && (el.textContent ?? "").includes(needle));
  const chain = [];
  for (let n = cell; n && chain.length < 8; n = n.parentElement) chain.push(`${n.tagName} ${[...n.attributes].map((a) => `${a.name}=${a.value.slice(0, 60)}`).join(" ").slice(0, 400)}`);
  const buttons = cell?.closest("tr,[role=row]")?.querySelectorAll("button") ?? [];
  return { chain, buttons: [...buttons].map((b) => [...b.attributes].map((a) => `${a.name}=${a.value.slice(0, 60)}`).join(" ")) };
}, text), null, 1));
await browser.close();
