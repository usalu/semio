/** 🎚️ Ticket tool of work package R: counts the CSS transitions the browser starts inside the pet layer while calm pets
 * move, on a device that asks for reduced motion and on one that does not. The quiz shortens every transition under
 * reduced motion (`.quiz-app * { transition-duration: 0.01ms !important }`), and an element without a
 * `transition-property` of its own transitions every property: each write of a frame into a pet then starts a
 * transition. Before work package R pets never moved under reduced motion, so nobody saw it.
 *
 * Prints, per device, how many `transitionrun` events the layer saw in two seconds, on which properties and elements,
 * and how many animations the document holds at the end.
 *
 * Usage (from the repository root, with the stack of `wp_r_stack.sh` up):
 *   WP_R_PROBE=wp_r_transitions_probe.ts PLAYWRIGHT_BASE_URL=http://127.0.0.1:6197 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR=http://127.0.0.1:8927 node node_modules/playwright/cli.js test --config ".../wp_r_hang.config.ts" --output <dir>
 */
import { enter, expect, pane, test } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts";

function countTransitions(milliseconds: number): Promise<{ runs: number; properties: Record<string, number>; elements: Record<string, number>; animations: number; duration: string; property: string }> {
  return new Promise((resolve) => {
    const stage = document.querySelector(".pet-layer");
    const properties: Record<string, number> = {};
    const elements: Record<string, number> = {};
    let runs = 0;
    const heard = (event: Event): void => {
      runs += 1;
      const name = (event as TransitionEvent).propertyName;
      properties[name] = (properties[name] ?? 0) + 1;
      const tag = (event.target as Element).tagName.toLowerCase();
      elements[tag] = (elements[tag] ?? 0) + 1;
    };
    stage?.addEventListener("transitionrun", heard);
    window.setTimeout(() => {
      stage?.removeEventListener("transitionrun", heard);
      const pet = document.querySelector(".pet-layer svg.pet");
      const style = pet === null ? null : getComputedStyle(pet);
      resolve({ runs, properties, elements, animations: document.getAnimations().filter((animation) => (animation.effect as KeyframeEffect | null)?.target?.closest(".pet-layer") != null).length, duration: style?.transitionDuration ?? "", property: style?.transitionProperty ?? "" });
    }, milliseconds);
  });
}

for (const reducedMotion of ["reduce", "no-preference"] as const)
  test(`transitions inside the pet layer while calm pets move, reduced motion: ${reducedMotion}`, async ({ device }) => {
    const learner = await device("en");
    const page = learner.page;
    await page.emulateMedia({ reducedMotion });
    await enter(learner, { kind: "anonymous" });
    await expect(page.locator(".pet-layer svg.pet")).not.toHaveCount(0, { timeout: 30_000 });
    await page.evaluate(() => (window.location.hash = "prefs"));
    await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
    await pane(page, "prefs").getByRole("group", { name: "Pets", exact: true }).getByRole("button").nth(2).click();
    await expect(page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
    await page.keyboard.press("Escape");
    await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
    const counted = await page.evaluate(countTransitions, 2000);
    process.stdout.write(`[DEBUG] ${reducedMotion}: ${JSON.stringify(counted)}\n`);
  });
