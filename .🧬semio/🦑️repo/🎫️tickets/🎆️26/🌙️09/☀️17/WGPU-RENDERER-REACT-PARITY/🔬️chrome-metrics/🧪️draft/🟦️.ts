import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { expect, test } from "@playwright/test";

const fixture = JSON.parse(
  readFileSync(resolve(process.cwd(), "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json"), "utf8"),
) as {
  readonly exampleControlMetrics: {
    readonly minimumRem: number;
    readonly maximumRem: number;
    readonly cases: readonly {
      readonly name: string;
      readonly rootRemPixels: number;
      readonly availablePixels: number;
      readonly expectedPixels: number;
    }[];
  };
};

test("the mounted React example trigger resolves authored rem bounds in actual browser layout", async ({ page }) => {
  await page.goto("/?plugin=puzzle3d", { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => document.getElementById("playground.navbar.fixture") instanceof HTMLElement, undefined, { timeout: 120_000 });

  for (const row of fixture.exampleControlMetrics.cases) {
    const measurement = await page.evaluate(
      ({ rootRemPixels, availablePixels }) => {
        const trigger = document.getElementById("playground.navbar.fixture");
        const host = trigger?.parentElement;
        if (!(trigger instanceof HTMLElement) || !(host instanceof HTMLElement)) return null;
        document.documentElement.style.fontSize = `${rootRemPixels}px`;
        host.style.width = `${availablePixels}px`;
        host.style.maxWidth = "none";
        host.style.padding = "0";
        host.style.flex = "none";
        const style = getComputedStyle(trigger);
        return {
          rootRemPixels: Number.parseFloat(getComputedStyle(document.documentElement).fontSize),
          minimumPixels: Number.parseFloat(style.minWidth),
          maximumPixels: Number.parseFloat(style.maxWidth),
          widthPixels: trigger.getBoundingClientRect().width,
        };
      },
      { rootRemPixels: row.rootRemPixels, availablePixels: row.availablePixels },
    );
    expect(measurement, row.name).not.toBeNull();
    expect(measurement!.rootRemPixels, row.name).toBeCloseTo(row.rootRemPixels, 3);
    expect(measurement!.minimumPixels, row.name).toBeCloseTo(fixture.exampleControlMetrics.minimumRem * row.rootRemPixels, 3);
    expect(measurement!.maximumPixels, row.name).toBeCloseTo(fixture.exampleControlMetrics.maximumRem * row.rootRemPixels, 3);
    expect(measurement!.widthPixels, row.name).toBeCloseTo(row.expectedPixels, 2);
  }
});
