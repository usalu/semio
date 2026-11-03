import { expect, test } from "@playwright/test";
import fixture from "../🧫️fixtures/📐️visual-layout/🔣️.json" with { type: "json" };

test("geometry", async ({ page, request, baseURL }) => {
  test.setTimeout(180000);
  const index = await (await request.get("index.json")).json() as { entries: Record<string, { type: string; title: string }> };
  const errors: string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  const open = async (subject: string): Promise<void> => {
    const entries = Object.entries(index.entries).filter(([, entry]) => entry.type === "story" && entry.title === `🖱️ui⚛️react/${subject}`);
    const story = entries.find(([id]) => id.endsWith("--default")) ?? entries[0];
    expect(story, subject).toBeDefined();
    await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(story![0])}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
    await page.waitForFunction(id => {
      const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
      return render?.id === id && render.phase === "finished";
    }, story![0], { timeout: 60000 });
    await page.evaluate(() => document.fonts.ready);
  };
  await open(fixture.badge.subject);
  const spacing = await page.evaluate(() => {
    const probe = document.createElement("span");
    probe.style.cssText = "position:absolute;visibility:hidden;width:var(--spacing-single);height:1px";
    document.body.append(probe);
    const width = probe.getBoundingClientRect().width;
    probe.remove();
    return width;
  });
  expect(spacing).toBeGreaterThan(0);
  const badge = await page.locator('[data-slot="badge"]').evaluate(element => {
    const style = getComputedStyle(element);
    return { start: parseFloat(style.paddingInlineStart), end: parseFloat(style.paddingInlineEnd), gap: parseFloat(style.gap) };
  });
  for (const value of [badge.start, badge.end]) expect(Math.abs(value - spacing * fixture.badge.paddingUnits)).toBeLessThan(fixture.tolerance);
  expect(Math.abs(badge.gap - spacing * fixture.badge.gapUnits)).toBeLessThan(fixture.tolerance);

  await open(fixture.actions.subject);
  const actions = await page.locator('[data-slot="action-group"]').evaluate(group => {
    const shell = group.getBoundingClientRect();
    return { top: shell.top, bottom: shell.bottom, height: shell.height, items: Array.from(group.querySelectorAll('[data-slot="action-group-item"]')).map(item => {
      const box = item.getBoundingClientRect();
      return { top: box.top, bottom: box.bottom, left: box.left, right: box.right, icons: Array.from(item.querySelectorAll("svg")).map(icon => {
        const rect = icon.getBoundingClientRect();
        return { top: rect.top, bottom: rect.bottom, left: rect.left, right: rect.right, width: rect.width, height: rect.height };
      }) };
    }) };
  });
  expect(Math.abs(actions.height - spacing * fixture.actions.heightUnits)).toBeLessThan(fixture.tolerance);
  expect(actions.items.length).toBeGreaterThan(0);
  for (const item of actions.items) {
    expect(item.top).toBeGreaterThanOrEqual(actions.top - fixture.tolerance);
    expect(item.bottom).toBeLessThanOrEqual(actions.bottom + fixture.tolerance);
    expect(item.icons.length).toBeGreaterThan(0);
    for (const icon of item.icons) {
      expect(Math.abs(icon.width - spacing * fixture.actions.iconUnits)).toBeLessThan(fixture.tolerance);
      expect(Math.abs(icon.height - spacing * fixture.actions.iconUnits)).toBeLessThan(fixture.tolerance);
      expect(icon.left - item.left).toBeGreaterThan(0);
      expect(item.right - icon.right).toBeGreaterThan(0);
      expect(icon.top - item.top).toBeGreaterThan(0);
      expect(item.bottom - icon.bottom).toBeGreaterThan(0);
      expect(Math.abs(icon.top + icon.bottom - item.top - item.bottom)).toBeLessThan(fixture.tolerance * 2);
    }
  }

  await open(fixture.presence.subject);
  const avatars = await page.locator('[data-slot="avatar"]').evaluateAll(elements => elements.map(element => {
    const box = element.getBoundingClientRect();
    return { left: box.left, right: box.right, top: box.top, height: box.height };
  }));
  expect(avatars.length).toBeGreaterThan(1);
  for (let index = 1; index < avatars.length; index++) {
    expect(avatars[index]!.left - avatars[index - 1]!.right).toBeGreaterThanOrEqual(spacing * fixture.presence.gapUnits - fixture.tolerance);
    expect(Math.abs(avatars[index]!.top - avatars[0]!.top)).toBeLessThan(fixture.tolerance);
  }

  await open(fixture.avatar.subject);
  const initials = await page.locator('[data-slot="avatar-fallback"]').evaluateAll(elements => elements.filter(element => element.textContent?.trim()).map(element => {
    const style = getComputedStyle(element);
    const box = element.getBoundingClientRect();
    const range = document.createRange();
    range.selectNodeContents(element);
    const text = range.getBoundingClientRect();
    return { fontSize: style.fontSize, lineHeight: style.lineHeight, horizontal: (box.width - text.width) / 2, vertical: (box.height - Math.max(text.height, parseFloat(style.lineHeight))) / 2 };
  }));
  expect(initials.length).toBeGreaterThan(1);
  expect(new Set(initials.map(value => value.fontSize)).size).toBe(1);
  expect(new Set(initials.map(value => value.lineHeight)).size).toBe(1);
  for (const value of initials) {
    expect(value.horizontal).toBeGreaterThanOrEqual(fixture.avatar.minimumTextClearance - fixture.tolerance);
    expect(value.vertical).toBeGreaterThanOrEqual(fixture.avatar.minimumTextClearance - fixture.tolerance);
  }

  await open(fixture.label.subject);
  const row = page.locator('[data-slot="property-row"]').first();
  const measure = () => row.evaluate(element => {
    const label = element.querySelector('[data-slot="property-label"]')!.getBoundingClientRect();
    const input = element.querySelector("input")!.getBoundingClientRect();
    return { centerOffset: Math.abs((label.top + label.bottom - input.top - input.bottom) / 2), stacked: input.top >= label.bottom - 0.5, indent: input.left - element.getBoundingClientRect().left };
  });
  for (const width of fixture.label.inlineWidths) {
    await row.evaluate((element, width) => { (element as HTMLElement).style.width = `${width}px`; }, width);
    await expect.poll(async () => (await measure()).centerOffset).toBeLessThan(fixture.tolerance);
    expect((await measure()).stacked).toBe(false);
  }
  await row.evaluate((element, width) => { (element as HTMLElement).style.width = `${width}px`; }, fixture.label.stackedWidth);
  await expect.poll(async () => (await measure()).stacked).toBe(true);
  expect(Math.abs((await measure()).indent)).toBeLessThan(fixture.tolerance);
  expect(errors).toEqual([]);
});

test("branches", async ({ page, request, baseURL }) => {
  test.setTimeout(120000);
  const index = await (await request.get("index.json")).json() as { entries: Record<string, { type: string; title: string }> };
  for (const vector of fixture.connectors) {
    const story = Object.entries(index.entries).find(([id, entry]) => entry.type === "story" && entry.title === `🖱️ui⚛️react/${vector.subject}` && id.endsWith("--default"));
    expect(story).toBeDefined();
    await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(story![0])}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
    await page.waitForFunction(id => {
      const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
      return render?.id === id && render.phase === "finished";
    }, story![0], { timeout: 60000 });
    for (const label of vector.expand) await page.locator('#storybook-root [data-slot="tree-label"]:visible').filter({ hasText: label }).click();
    for (const [ids, last] of [[vector.last, true], [vector.continuing, false]] as const) for (const id of ids) {
      const row = page.locator(`[data-slot="tree-item-row"][id="${id}"]`);
      await expect(row).toBeVisible();
      const geometry = await row.evaluate(element => {
        const row = element.getBoundingClientRect();
        const parent = element.querySelector('[data-slot="tree-parent-guide"]')!.getBoundingClientRect();
        const elbow = element.querySelector('[data-slot="tree-branch-elbow"]')!.getBoundingClientRect();
        return { rowBottom: row.bottom, guideBottom: parent.bottom, anchor: (elbow.top + elbow.bottom) / 2, guideHeight: parent.height };
      });
      expect(geometry.guideHeight).toBeGreaterThan(0);
      expect(Math.abs(geometry.guideBottom - (last ? geometry.anchor : geometry.rowBottom))).toBeLessThanOrEqual(fixture.tolerance);
    }
  }
});

test("floor", async ({ page, request, baseURL }) => {
  test.setTimeout(120000);
  const index = await (await request.get("index.json")).json() as { entries: Record<string, { type: string; title: string }> };
  const story = Object.entries(index.entries).find(([id, entry]) => entry.type === "story" && entry.title === `🖱️ui⚛️react/${fixture.grid.subject}` && id.endsWith(fixture.grid.storySuffix));
  expect(story).toBeDefined();
  await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(story![0])}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForFunction(id => {
    const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
    return render?.id === id && render.phase === "finished";
  }, story![0], { timeout: 60000 });
  const canvas = page.locator("canvas").first();
  await expect(canvas).toBeVisible();
  const withGrid = await canvas.screenshot();
  await page.evaluate(async storyId => {
    const channel = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__: { channel: { on(event: string, handler: (payload: { storyId: string; args: { showGrid?: boolean } }) => void): void; off(event: string, handler: unknown): void; emit(event: string, payload: unknown): void } } }).__STORYBOOK_PREVIEW__.channel;
    await new Promise<void>(resolve => {
      const changed = (payload: { storyId: string; args: { showGrid?: boolean } }) => {
        if (payload.storyId !== storyId || payload.args.showGrid !== false) return;
        channel.off("storyArgsUpdated", changed);
        resolve();
      };
      channel.on("storyArgsUpdated", changed);
      channel.emit("updateStoryArgs", { storyId, updatedArgs: { showGrid: false } });
    });
    await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  }, story![0]);
  const withoutGrid = await canvas.screenshot();
  const changed = await page.evaluate(async ({ before, after, contrast }) => {
    const decode = async (encoded: string) => {
      const image = await createImageBitmap(await (await fetch(`data:image/png;base64,${encoded}`)).blob());
      const canvas = new OffscreenCanvas(image.width, image.height);
      const context = canvas.getContext("2d")!;
      context.drawImage(image, 0, 0);
      image.close();
      return { width: canvas.width, height: canvas.height, pixels: context.getImageData(0, 0, canvas.width, canvas.height).data };
    };
    const first = await decode(before);
    const second = await decode(after);
    let changed = 0;
    for (let y = Math.floor(first.height * 0.25); y < first.height * 0.85; y++) for (let x = Math.floor(first.width * 0.1); x < first.width * 0.3; x++) {
      const offset = (y * first.width + x) * 4;
      const difference = Math.abs(first.pixels[offset]! - second.pixels[offset]!) + Math.abs(first.pixels[offset + 1]! - second.pixels[offset + 1]!) + Math.abs(first.pixels[offset + 2]! - second.pixels[offset + 2]!);
      if (difference > contrast) changed++;
    }
    return changed;
  }, { before: withGrid.toString("base64"), after: withoutGrid.toString("base64"), contrast: fixture.grid.contrast });
  expect(changed).toBeGreaterThan(fixture.grid.minimumChangedPixels);
});
