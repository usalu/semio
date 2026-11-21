import { chromium, devices } from "playwright";
import { PAR, activeOverview, answerRun, answerTask, arrive, boxOf, card, expect, expectFeedback, goHome, handle, identify, openTask, playQuiz, quizOf, readIntroduction, restingPage, screen, shownBest, shownTask, shownText, submitRun, swipeTo, taskOf, touchSwipe, unresolvedLabels, type Device } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";

const phase = process.env.COURSE_PHONE_PHASE ?? "all";
const viewport = { width: 375, height: 812 };
const origin = `http://127.0.0.1:${process.env.COURSE_PHONE_BROWSER_PORT ?? "6277"}`;
const executablePath = process.env.COURSE_CHROME_EXECUTABLE;
const browser = await chromium.launch(executablePath === undefined ? { channel: "chromium" } : { executablePath });

async function verify(locale: Device["locale"], visit: (device: Device) => Promise<void>): Promise<void> {
  const context = await browser.newContext({ ...devices["iPhone 13"], baseURL: origin, viewport, locale: locale === "de" ? "de-DE" : "en-GB" });
  await globalThis.routeTicketBrowser?.(context, origin);
  context.setDefaultTimeout(12000);
  try {
    const page = await context.newPage();
    const problems: string[] = [];
    page.on("pageerror", (error) => problems.push(error.message));
    const learner: Device = { context, page, locale, problems, expectingFailures: async (body) => { await body(); return 0; } };
    await arrive(learner, 60000);
    await readIntroduction(learner);
    await identify(learner, locale === "de" ? { kind: "anonymous" } : { kind: "pseudonym", handle: handle("Phone Guesser") });
    await visit(learner);
    expect(problems).toEqual([]);
  } finally {
    await context.close();
  }
}

async function noSideways({ page }: Device): Promise<void> {
  const overflow = await page.evaluate(() => Math.max(
    0,
    document.documentElement.scrollWidth - window.innerWidth,
    ...[...document.querySelectorAll<HTMLElement>("#course-main, #course-main *")]
      .filter((box) => box.closest("[inert]") === null && /auto|scroll/u.test(getComputedStyle(box).overflowX) && box.firstElementChild?.tagName !== "TABLE")
      .map((box) => box.scrollWidth - box.clientWidth),
  ));
  expect(overflow).toBe(0);
}

try {
  if (phase === "all" || phase === "swipe") await verify("de", async (learner) => {
    const { page } = learner;
    const root = activeOverview(page);
    await expect(root).toHaveAttribute("data-mode", "swipe");
    const cells = root.locator(":scope > [data-layered-overlay] > [data-layered-cell]");
    expect(await cells.evaluateAll((all) => all.map((cell) => {
      const element = cell as HTMLElement;
      return [element.dataset.layeredCell, Number(element.style.gridColumn) - 1, Number(element.style.gridRow) - 1];
    }))).toEqual([["learner", 0, 0], ["quizzes", 1, 0], ["assignments", 2, 0], ["clips", 0, 1], ["board", 1, 1], ["badges", 2, 1], ["prefs", 0, 2]]);
    expect(await cells.evaluateAll((all) => all.every((cell) => {
      const card = cell.querySelector<HTMLElement>("[data-layered-card] [data-card]")!;
      return getComputedStyle(cell).alignItems === "stretch" && card.scrollHeight === card.clientHeight;
    }))).toBe(true);
    const view = await boxOf(root);
    const learnerCard = await boxOf(cells.filter({ has: page.locator('[data-card="learner"]') }));
    expect([learnerCard.x, learnerCard.y, learnerCard.width, learnerCard.height].map(Math.round)).toEqual([Math.round(view.x), Math.round(view.y), viewport.width, Math.round(view.height)]);
    expect(await unresolvedLabels(learner)).toEqual([]);
    expect(await restingPage(page)).toBe("learner");
    const gestures = [[240, 0, "assignments"], [-240, 0, "learner"], [0, 325, "prefs"], [0, -325, "learner"], [-60, 0, "learner", 600]] as const;
    for (const [dx, dy, expected, duration] of gestures) {
      await touchSwipe(page, dx, dy, duration);
      await expect.poll(() => restingPage(page)).toBe(expected);
    }
    const hints = root.locator(":scope > [data-layered-neighbour]");
    const neighbours = () => hints.evaluateAll((all) => all.map((hint) => [(hint as HTMLElement).dataset.layeredNeighbour, (hint as HTMLElement).dataset.pane]));
    expect(await neighbours()).toEqual([["up", "prefs"], ["left", "assignments"], ["right", "quizzes"], ["down", "clips"]]);
    for (const hint of await hints.all()) await expect(hint).toBeInViewport({ ratio: 1 });
    await expect(root.locator('[data-layered-neighbour="right"]')).toHaveAttribute("aria-label", "Nach rechts zu Quizze");
    await root.locator('[data-layered-neighbour="down"]').tap();
    await expect.poll(() => restingPage(page)).toBe("clips");
    await expect.poll(neighbours).toEqual([["up", "learner"], ["left", "badges"], ["right", "board"], ["down", "prefs"]]);
    console.info("[DEBUG] German phone overview passed both directions, both wraparound axes and localized hint navigation");

    for (const id of ["learner", "quizzes", "energy"]) {
      await swipeTo(learner, id);
      if (id !== "learner") {
        await card(page, id).getByRole("link").tap();
        await expect(page.locator(`[data-course-grid="${id}"][data-grid-active]`)).toBeVisible();
      }
    }
    const quiz = quizOf("cooling");
    await swipeTo(learner, quiz.id);
    await expect(card(page, quiz.id).getByRole("heading")).toHaveText(quiz.title.de);
    await swipeTo(learner, "demand");
    expect(await restingPage(page)).toBe("demand");
    await swipeTo(learner, quiz.id);
    expect(await restingPage(page)).toBe(quiz.id);
    await playQuiz(learner, quiz.id);
    await noSideways(learner);
    await answerRun(learner, quiz, "perfect");
    await noSideways(learner);
    await submitRun(learner);
    expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
    await expect(screen(page, "results").getByRole("heading", { level: 1 })).toContainText(quiz.title.de);
    await noSideways(learner);
    await goHome(learner);
    await swipeTo(learner, quiz.id);
    expect(await shownBest(learner, quiz.id)).toEqual({ challenge: "medium", points: PAR.medium, par: PAR.medium });
    console.info("[DEBUG] German Cooling quiz passed sparse row navigation, 100% feedback and persisted best result");
  });

  if (phase === "all" || phase === "hard") await verify("en", async (learner) => {
    const { page } = learner;
    const quiz = quizOf("physics");
    await playQuiz(learner, quiz.id, "hard");
    const steps = screen(page, "run").locator("nav button");
    let fieldsChecked = 0;
    for (const index of quiz.tasks.keys()) {
      await openTask(learner, index);
      const current = screen(page, "task");
      await expect(current.locator('[aria-busy="true"]')).toHaveCount(0);
      const fields = await current.locator("[data-course-item] input").all();
      fieldsChecked += fields.length;
      for (const field of fields) {
        const { x, width, height } = await boxOf(field);
        expect(x).toBeGreaterThanOrEqual(0);
        expect(x + width).toBeLessThanOrEqual(viewport.width);
        expect(height).toBeGreaterThanOrEqual(44);
        await expect(field).toHaveCSS("font-size", "16px");
      }
      const task = taskOf(quiz, await shownTask(learner));
      for (const category of task.categories ?? []) {
        if (category.description !== undefined) expect(await shownText(learner)).not.toContain(category.description.en);
      }
      await answerTask(learner, quiz, "perfect");
      await expect(steps.nth(index).locator("[data-complete]")).toBeVisible();
      await noSideways(learner);
    }
    expect(fieldsChecked).toBeGreaterThan(0);
    await submitRun(learner);
    expect((await expectFeedback(learner, quiz, "perfect", "hard")).points).toBe(PAR.hard);
    await noSideways(learner);
    expect(await unresolvedLabels(learner)).toEqual([]);
    console.info(`[DEBUG] Hard Physics passed all tasks, ${fieldsChecked} guess fields and full credit`);
  });
} finally {
  await browser.close();
}
