import { expect, test } from "@playwright/test";

type PreviewWindow = typeof globalThis & {
  __STORYBOOK_PREVIEW__?: {
    currentRender?: { id: string; phase: string };
    channel: { emit(event: string, payload: { storyId: string; viewMode: string }): void };
  };
};

test("catalog", async ({ page, request, baseURL }) => {
  test.setTimeout(600000);
  const response = await request.get("index.json");
  expect(response.ok()).toBe(true);
  const index = await response.json() as { entries: Record<string, { type: string; title: string }> };
  const stories = Object.entries(index.entries).filter(([, entry]) => entry.type === "story" && /^(🖱️ui⚛️react|🎨️styling)(\/|$)/u.test(entry.title) && (!process.env.STORYBOOK_AUDIT_PATTERN || new RegExp(process.env.STORYBOOK_AUDIT_PATTERN).test(entry.title)));
  expect(stories.length).toBeGreaterThan(0);
  const failures: { id: string; errors: string[] }[] = [];
  let errors: string[] = [];
  let rejectRender: ((error: Error) => void) | undefined;
  page.on("pageerror", error => { errors.push(error.message); rejectRender?.(error); });
  page.on("console", message => { if (message.type() === "error") errors.push(message.text() + " " + message.location().url); });
  page.on("response", resource => {
    if (resource.status() >= 400) {
      const failure = `${resource.status()} ${resource.url()}`;
      errors.push(failure);
      rejectRender?.(new Error(failure));
    }
  });
  page.on("requestfailed", resource => errors.push(`${resource.failure()?.errorText} ${resource.url()}`));
  for (const [id] of stories) {
    await test.step(id, async () => {
      console.log(`Checking ${id}`);
      errors = [];
      const failed = new Promise<never>((_, reject) => { rejectRender = reject; });
      void failed.catch(() => {});
      try {
        if (id === stories[0][0]) {
          await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(id)}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
        } else {
          await page.evaluate(storyId => {
            (globalThis as PreviewWindow).__STORYBOOK_PREVIEW__!.channel.emit("setCurrentStory", { storyId, viewMode: "story" });
          }, id);
        }
        await Promise.race([failed, page.waitForFunction(storyId => {
          const render = (globalThis as PreviewWindow).__STORYBOOK_PREVIEW__?.currentRender;
          return render?.id === storyId && render.phase === "finished";
        }, id, { timeout: 30000 })]);
        const body = await page.locator("body").innerText();
        if (/Failed to load the Storybook preview|Couldn't find story matching|Unable to render story/i.test(body)) errors.push(body.slice(0, 2000));
        const exception = page.locator(".sb-errordisplay");
        if (await exception.isVisible()) errors.push((await exception.innerText()).slice(0, 2000));
      } catch (error) {
        errors.push(String(error));
      }
      rejectRender = undefined;
      if (errors.length) {
        failures.push({ id, errors: [...new Set(errors)] });
        console.log(JSON.stringify(failures.at(-1)));
        if (id === stories[0][0]) expect(errors).toEqual([]);
      }
    });
  }
  console.log(`Validated ${stories.length} registered UI/styling variants; ${failures.length} failed.`);
  expect(failures).toEqual([]);
});

test("autodocs", async ({ page, request, baseURL }) => {
  test.setTimeout(60000);
  const response = await request.get("index.json");
  const index = await response.json() as { entries: Record<string, { type: string; title: string }> };
  const docs = Object.entries(index.entries).find(([, entry]) => entry.type === "docs" && entry.title === "🖱️ui⚛️react/Button");
  expect(docs).toBeDefined();
  const errors: string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
  page.on("response", resource => { if (resource.status() >= 400) errors.push(`${resource.status()} ${resource.url()}`); });
  await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(docs![0])}&viewMode=docs`, { waitUntil: "domcontentloaded" });
  await expect(page.locator(".sbdocs-wrapper")).toBeVisible({ timeout: 60000 });
  await expect(page.locator(".sbdocs-wrapper")).toContainText("Button");
  expect(errors).toEqual([]);
});
