import { chromium } from "playwright";
import { activeOverview, arrive, authorPreview, boxOf, card, expect, goHome, identify, pane, readIntroduction, type Device } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts";

const origin = `http://127.0.0.1:${process.env.COURSE_PHONE_BROWSER_PORT ?? "6279"}`;
const browser = await chromium.launch({ channel: "chromium" });
try {
  const context = await browser.newContext({ baseURL: origin, viewport: { width: 1440, height: 900 }, locale: "en-GB" });
  await globalThis.routeTicketBrowser?.(context, origin);
  context.setDefaultTimeout(12000);
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("pageerror", (error) => problems.push(error.message));
  const learner: Device = { page, context, locale: "en", problems, expectingFailures: async (body) => { await body(); return 0; } };
  await arrive(learner, 60000);
  await readIntroduction(learner);
  await identify(learner, { kind: "anonymous" }, true);
  const overview = activeOverview(page);
  const onGrid = async (active: string) => {
    await expect(page.locator(`[data-course-grid="${active}"][data-grid-active]`)).toBeVisible();
    await expect(overview).toHaveAttribute("data-mode", "strip");
    await expect(overview).toHaveAttribute("data-pan", "pointer");
  };
  const openCard = async (id: string) => {
    await card(page, id).locator(`a[href="#${id}"]`).click();
    await expect(pane(page, id)).toHaveAttribute("data-opened", "");
    await expect(pane(page, id)).not.toHaveAttribute("inert", "");
    await expect.poll(() => new URL(page.url()).hash).toBe(`#${id}`);
  };
  await onGrid("root");
  await expect(page.getByRole("combobox", { name: "Open a course page" })).toBeHidden();
  const frame = await boxOf(overview);
  const strip = overview.locator(":scope > [data-layered-strip]");
  const veil = overview.locator(":scope > [data-layered-veil]");
  await page.mouse.move(frame.x + 2, frame.y + 2, { steps: 4 });
  const before = await strip.getAttribute("style");
  await page.mouse.move(frame.x + frame.width - 2, frame.y + frame.height - 2, { steps: 4 });
  await expect(veil).toHaveAttribute("data-veil", "whole");
  await expect.poll(() => strip.getAttribute("style")).not.toBe(before);
  await page.mouse.move(frame.x + 2, frame.y + 2, { steps: 4 });
  await expect(pane(page, "quizzes")).toHaveAttribute("inert", "");
  await openCard("quizzes");
  await onGrid("quizzes");
  await openCard("energy");
  await onGrid("energy");
  await openCard("physics");
  await page.keyboard.press("Escape");
  await expect.poll(() => new URL(page.url()).hash).toBe("#energy");
  await expect(card(page, "physics").getByRole("link")).toBeFocused();
  await goHome(learner);
  await onGrid("root");
  console.info("[DEBUG] Desktop course strip panned, opened nested grids and restored keyboard focus on Escape");

  await authorPreview(learner);
  await openCard("assignments");
  await onGrid("assignments");
  await openCard("parametrization");
  const assignment = page.locator('[data-page="assignment:parametrization"]');
  await expect(assignment).toBeVisible();
  const path = "/assignments/architecture-parametric-parametrization-en.pdf";
  await expect(assignment.locator("iframe")).toHaveAttribute("src", `${path}#view=FitH`);
  await expect(assignment.getByRole("link", { name: "Open in new tab" })).toHaveAttribute("href", path);
  await expect(assignment.getByRole("link", { name: "Download" })).toHaveAttribute("href", path);
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(1440);
  expect(problems).toEqual([]);
  console.info("[DEBUG] Desktop course assignment mounted the inline iframe and localized PDF links");
} finally {
  await browser.close();
}
