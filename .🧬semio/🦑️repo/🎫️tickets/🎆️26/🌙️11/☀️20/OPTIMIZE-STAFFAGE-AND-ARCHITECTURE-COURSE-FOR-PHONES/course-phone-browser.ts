import { chromium, devices, firefox, webkit } from "playwright";
import type { Device } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";
import phone from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/📱️phone/🔣️.json" with { type: "json" };

const executablePath = process.env.COURSE_CHROME_EXECUTABLE;
const port = Number(process.env.COURSE_PHONE_BROWSER_PORT ?? "6272");
const useWebKit = process.env.COURSE_PHONE_BROWSER === "webkit";
const useFirefox = process.env.COURSE_PHONE_BROWSER === "firefox";
const browser = useWebKit ? await webkit.launch() : useFirefox ? await firefox.launch() : await chromium.launch(executablePath !== undefined ? { executablePath } : process.env.COURSE_USE_CHROMIUM_SHELL === "true" ? {} : { channel: "chromium" });
try {
  const { activeOverview, answerRun, arrive, authorPreview, boxOf, card, expect, expectFeedback, identify, openTaskById, playQuiz, quizOf, readIntroduction, screen, submitRun, touchAssignClassification } =
    await import("../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts");
  const origin = `http://127.0.0.1:${port}`;
  const context = await browser.newContext({ ...devices["iPhone 13"], ...(useFirefox ? { isMobile: false } : {}), baseURL: origin, viewport: { width: phone.width, height: phone.height }, locale: "en-GB" });
  await globalThis.routeTicketBrowser?.(context, origin);
  context.setDefaultTimeout(12000);
  const page = await context.newPage();
  const pageErrors: string[] = [];
  const bootProblems: string[] = [];
  const pdfRequests: string[] = [];
  const crowdRequests: string[] = [];
  const pdfPath = new RegExp(phone.pdfPattern, "u");
  page.on("pageerror", (error) => {
    pageErrors.push(error.message);
    bootProblems.push(error.stack ?? error.message);
  });
  page.on("console", (message) => {
    if (message.type() === "error") bootProblems.push(message.text());
  });
  page.on("requestfailed", (request) => bootProblems.push(`${request.method()} ${request.url()} ${request.failure()?.errorText}`));
  page.on("response", (response) => {
    if (response.status() >= 400) bootProblems.push(`${response.status()} ${response.url()}`);
  });
  page.on("request", (request) => {
    const pathname = new URL(request.url()).pathname;
    if (pdfPath.test(pathname)) pdfRequests.push(request.url());
    if (pathname.endsWith("/crowd") || request.postData()?.includes('"course.crowd"')) crowdRequests.push(request.url());
  });
  const learner: Device = { context, page, locale: "en", problems: [], expectingFailures: async (body) => { await body(); return 0; } };
  try {
    await arrive(learner, 60_000);
    await readIntroduction(learner);
    await identify(learner, { kind: "anonymous" });
  } catch (error) {
    const shown = await page.locator("body").innerText({ timeout: 1000 }).catch(() => "");
    console.error("[DEBUG] Course browser boot", { url: page.url(), text: shown.slice(0, 1200), problems: [...new Set(bootProblems)].slice(-24) });
    throw error;
  }
  await authorPreview(learner);
  const picker = page.getByRole("combobox", { name: "Open a course page" });
  const pick = async (value: string) => {
    await picker.selectOption(value);
    await picker.locator("..").getByRole("button", { name: "Open", exact: true }).click();
  };
  await expect(picker).toBeVisible();
  await expect(page.locator("[data-course-grid]")).toHaveCount(phone.rootGrids);
  for (const control of await page.locator("[data-course-nav]").all()) {
    const target = await boxOf(control);
    expect(target.width).toBeGreaterThanOrEqual(phone.minimumTouchTarget);
    expect(target.height).toBeGreaterThanOrEqual(phone.minimumTouchTarget);
    await expect(control).toBeInViewport({ ratio: 1 });
  }
  await pick("clips");
  const clips = activeOverview(page);
  const poster = card(page, phone.visibleClip).locator("img");
  await expect(poster).toBeVisible();
  await expect.poll(() => poster.evaluate((image: HTMLImageElement) => image.naturalWidth)).toBeGreaterThan(0);
  await expect(clips.locator("[data-layered-overlay] [data-clip-poster] img")).toHaveCount(phone.maximumClipCardPosters);
  await pick(phone.directClip);
  await expect(page.locator(`[data-page="clip:${phone.directClip}"] img`)).toBeVisible();
  await page.locator('[data-course-nav="overview"]').tap();
  await pick("assignments");
  await pick(phone.assignment);
  const assignment = page.locator(`[data-page="assignment:${phone.assignment}"]`);
  await expect(assignment).toBeVisible();
  await expect(assignment.locator("iframe")).toHaveCount(0);
  await expect(assignment.getByRole("link", { name: "Download" })).toHaveAttribute("href", pdfPath);
  await expect(assignment.getByRole("link", { name: "Open in new tab" })).toHaveAttribute("href", pdfPath);
  expect(pdfRequests).toEqual([]);
  expect(crowdRequests).toHaveLength(phone.idleCrowdRequests);
  await page.setViewportSize(phone.landscape);
  await expect(activeOverview(page)).toHaveAttribute("data-mode", "swipe");
  await expect(picker).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  console.log("[DEBUG] narrow portrait and touch landscape navigation, clip posters, mobile PDF links and idle request counts passed");

  await page.setViewportSize({ width: 375, height: 560 });
  const quiz = quizOf("physics");
  await page.locator('[data-course-nav="overview"]').tap();
  await pick("quizzes");
  await pick("energy");
  await pick(quiz.id);
  await playQuiz(learner, quiz.id);
  await openTaskById(learner, quiz.tasks.find((task) => task.kind === "classification")!.id);
  const grip = screen(page, "task").locator(".course-chip [data-course-grip]").first();
  await grip.scrollIntoViewIfNeeded();
  expect((await boxOf(grip)).width).toBeGreaterThanOrEqual(44);
  if (!useWebKit && !useFirefox) {
    try {
      await touchAssignClassification(page);
    } catch (failure) {
      console.error("[DEBUG] touch drop layout", await page.evaluate(() => ({
        viewport: [window.innerWidth, window.innerHeight],
        scrollers: [...document.querySelectorAll<HTMLElement>(".course-app, #course-main, .course-run")].map((element) => ({
          style: getComputedStyle(element).overflowY,
          position: element.scrollTop,
          bounds: element.getBoundingClientRect().toJSON(),
          height: element.scrollHeight,
        })),
        zones: [...document.querySelectorAll<HTMLElement>("[data-course-drop]")].map((element) => ({
          zone: element.dataset.courseDrop,
          bounds: element.getBoundingClientRect().toJSON(),
        })),
      })));
      throw failure;
    }
    console.info("[DEBUG] touch drag assigned an offscreen category in the Physics quiz");
  }
  await answerRun(learner, quiz, "perfect");
  await submitRun(learner);
  expect((await expectFeedback(learner, quiz, "perfect")).score).toBe(100);
  expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  expect(pageErrors).toEqual([]);
  console.log(`[DEBUG] 375×560 ${browser.browserType().name()} full quiz through scored results passed`);
} finally {
  await browser.close();
}
