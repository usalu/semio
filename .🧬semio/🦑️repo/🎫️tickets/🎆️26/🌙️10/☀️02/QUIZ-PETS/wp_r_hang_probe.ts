/** 🪤️ Ticket tool of work package R: hunts the page that stopped answering while the pet spec waited for a walk (one
 * of thirty-nine runs of the reduced-motion test ended with "Target crashed" after five seconds without a frame). A
 * learner whose device asks for reduced motion opens the preferences and switches between still and calm pets again and
 * again at the test tempo. A watchdog beside the learner asks the page for an answer (and for the size of its heap)
 * several times a second through the debugger's own session; when no answer comes within a second and a half it pauses
 * the page and writes down where it is — three times, so a loop shows — together with what the learner was doing and
 * how the heap grew.
 *
 * `WP_R_ROUNDS` is the number of still → calm rounds per learner (default 12), `WP_R_WATCH_MS` how long calm pets are
 * watched per round (default 8000), `WP_R_PAGE` the page of the overview the learner watches on (default `prefs`; an
 * empty value watches on the overview), `WP_R_RELOAD=1` reloads before every round but the first (a new seed for the
 * stage each time) and `WP_R_STILL_MS` is how long the pets stay still before they turn calm (default 0).
 * `WP_R_TRANSITIONS=1` gives the pet layer back the transitions its stylesheet forbids since this work package (a style
 * element of the probe's own, dev topology only: the release build's policy refuses inline styles) — the state in
 * which the crash was seen: every write of a frame starts a transition of a hundredth of a millisecond.
 *
 * Usage (from the repository root, with the stack of `wp_r_stack.sh` up):
 *   PLAYWRIGHT_BASE_URL=http://127.0.0.1:6197 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR=http://127.0.0.1:8927 node node_modules/playwright/cli.js test --config ".../wp_r_hang.config.ts" --repeat-each 8 --output <dir>
 */
import { enter, expect, pane, test } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";

const ROUNDS = Number(process.env.WP_R_ROUNDS ?? 12);
const WATCH_MS = Number(process.env.WP_R_WATCH_MS ?? 8000);
const PAGE = process.env.WP_R_PAGE ?? "prefs";
const RELOAD = process.env.WP_R_RELOAD === "1";
const STILL_MS = Number(process.env.WP_R_STILL_MS ?? 0);
const TRANSITIONS = process.env.WP_R_TRANSITIONS === "1";

interface CallFrame {
  readonly functionName: string;
  readonly url: string;
  readonly location: { readonly lineNumber: number; readonly columnNumber?: number };
}

const pause = (milliseconds: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, milliseconds));

function within<T>(work: Promise<T>, milliseconds: number, otherwise: T): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  return Promise.race([work.catch(() => otherwise), new Promise<T>((resolve) => (timer = setTimeout(() => resolve(otherwise), milliseconds)))]).finally(() => clearTimeout(timer));
}

test("calm pets at the test tempo never stop the page", async ({ device }, info) => {
  const learner = await device("en");
  const page = learner.page;
  const said: string[] = [];
  let doing = "arriving";
  await page.addInitScript(() => {
    const mark = (): boolean => {
      document.documentElement?.setAttribute("data-pets-tempo", "8");
      return document.documentElement !== null;
    };
    if (mark()) return;
    const watch = new MutationObserver(() => {
      if (mark()) watch.disconnect();
    });
    watch.observe(document, { childList: true });
  });
  if (TRANSITIONS)
    await page.addInitScript(() => {
      window.addEventListener("DOMContentLoaded", () => {
        const sheet = document.createElement("style");
        sheet.textContent = "html .pet-layer, html .pet-layer * { transition-property: all !important; }";
        document.head.append(sheet);
      });
    });
  await page.emulateMedia({ reducedMotion: "reduce" });
  await enter(learner, { kind: "anonymous" });
  await expect(page.locator(".pet-layer svg.pet")).not.toHaveCount(0, { timeout: 30_000 });
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  const choices = pane(page, "prefs").getByRole("group", { name: "Pets", exact: true }).getByRole("button");
  const debug = await page.context().newCDPSession(page);
  await debug.send("Debugger.enable");
  let paused: ((frames: readonly CallFrame[]) => void) | null = null;
  debug.on("Debugger.paused", (event) => paused?.(event.callFrames as unknown as readonly CallFrame[]));
  const where = async (): Promise<string> => {
    const frames = await within(
      new Promise<readonly CallFrame[]>((resolve) => {
        paused = resolve;
        debug.send("Debugger.pause").catch(() => resolve([]));
      }),
      4000,
      [],
    );
    paused = null;
    await within(debug.send("Debugger.resume").then(() => undefined), 1000, undefined);
    if (frames.length === 0) return "    (the page did not pause)";
    return frames
      .slice(0, 16)
      .map((frame) => `    ${frame.functionName || "(anonymous)"} ${decodeURIComponent(frame.url.split("/").slice(-4).join("/"))}:${frame.location.lineNumber + 1}:${(frame.location.columnNumber ?? 0) + 1}`)
      .join("\n");
  };
  const heaps: string[] = [];
  let hung = false;
  let over = false;
  const watchdog = (async (): Promise<void> => {
    while (!over) {
      const asked = Date.now();
      const heap = await within(
        debug.send("Runtime.evaluate", { expression: "performance.memory ? performance.memory.usedJSHeapSize : 0", returnByValue: true }).then((answer) => Number(answer.result.value)),
        1500,
        -1,
      );
      if (over) return;
      if (heap < 0 && doing.startsWith("reloading")) {
        await pause(250);
        continue;
      }
      if (heap < 0) {
        hung = true;
        said.push(`while ${doing}: the page does not answer; heap before, MB (newest last): ${heaps.slice(-12).join(" ")}`);
        for (let look = 0; look < 3; look++) said.push(`  look ${look}, ${Date.now() - asked} ms after the question:\n${await where()}`);
        return;
      }
      heaps.push((heap / 1048576).toFixed(0));
      await pause(250);
    }
  })();
  try {
    for (let round = 0; round < ROUNDS && !hung; round++) {
      if (RELOAD && round > 0) {
        doing = `reloading before round ${round}`;
        await page.reload();
        await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
        await expect(page.locator(".pet-layer svg.pet")).not.toHaveCount(0, { timeout: 30_000 });
      }
      doing = `choosing still in round ${round}`;
      await choices.nth(1).click();
      await expect(page.locator(".quiz-app")).toHaveAttribute("data-pets", "still");
      doing = `resting still in round ${round}`;
      await pause(STILL_MS);
      doing = `choosing calm in round ${round}`;
      await choices.nth(2).click();
      await expect(page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
      if (PAGE === "" && round === 0) {
        await page.keyboard.press("Escape");
        await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
      }
      doing = `watching calm pets in round ${round}`;
      const until = Date.now() + WATCH_MS;
      while (Date.now() < until && !hung) await pause(250);
      if (PAGE === "" && !hung) {
        await page.evaluate(() => (window.location.hash = "prefs"));
        await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
      }
    }
  } catch (error) {
    said.push(`while ${doing}: ${String(error).split("\n")[0]}`);
    await pause(6000);
  }
  over = true;
  await within(watchdog, 20_000, undefined);
  await info.attach("hang", { body: said.join("\n"), contentType: "text/plain" });
  process.stdout.write(`${said.length === 0 ? "[DEBUG] no hang" : `[DEBUG] ${said.join("\n")}`}\n`);
  expect(said).toEqual([]);
});
