/** 🔬️ S18 §14c (G12 relay): the directory bootstrap state a clean profile reaches after signing in through the shell, per
 * locale — on Home and after landing in a space (`/spaces/<id>`): `[data-directory-bootstrap]` state + code over time,
 * fault/refusal console lines and the hub requests of the directory lane.
 * usage: bun s18-probe-directory-bootstrap.mjs <url> <locale> [spaceId] */
import { USERS, boot, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/", locale = "de-DE", spaceId = ""] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { locale, users: [USERS[0]] });
const [A] = sessions;
const status = () => A.page.evaluate(() => [...document.querySelectorAll("[data-directory-bootstrap]")].map((e) => `${e.getAttribute("data-directory-bootstrap")}:${e.getAttribute("data-directory-bootstrap-code") ?? ""}:${(e.textContent ?? "").trim().slice(0, 60)}`));
const timeline = [];
const sample = async (label, seconds) => {
  for (let i = 0; i < seconds; i += 1) {
    const now = JSON.stringify(await status());
    if (timeline.at(-1)?.now !== now) timeline.push({ at: `${label}+${i}s`, now });
    await A.page.waitForTimeout(1_000);
  }
};
try {
  await boot(A);
  await sample("boot", 3);
  await signIn(A);
  await sample("signed-in", 20);
  if (spaceId) {
    await A.page.goto(new URL(`/spaces/${spaceId}`, url).href, { waitUntil: "domcontentloaded" });
    await sample("space", 30);
  }
  console.log("TIMELINE", JSON.stringify(timeline, null, 0));
  for (const line of A.lines.filter((l) => /fault|refus|directory|bootstrap|error|UiText|capacity/iu.test(l) && !/status of 404|DevTools/u.test(l)).slice(-40)) console.log("LINE", line.slice(0, 500));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 600));
} finally {
  await browser.close();
}
