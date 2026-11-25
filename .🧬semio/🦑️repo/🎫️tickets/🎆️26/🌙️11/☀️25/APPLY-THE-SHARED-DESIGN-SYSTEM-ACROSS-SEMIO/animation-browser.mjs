import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { chromium } from 'playwright';

const folder = '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine';
const styles = '🧰️framework/🔨️modules/🖱️ui/🎨️styling';
const [palette, css] = await Promise.all([readFile(`${styles}/🎨️palette/🎨️.css`, 'utf8'), readFile(`${folder}/🎨️presentation.css`, 'utf8')]);
const stylesheet = process.argv[2] ? await readFile(process.argv[2], 'utf8') : palette.replaceAll('@theme {', ':root {') + css;
const emittedHtml = process.argv[3] ? await readFile(process.argv[3], 'utf8') : null;
const browser = await chromium.launch();
try {
  for (const viewport of [{ width: 1280, height: 720 }, { width: 375, height: 812 }, { width: 1920, height: 1080 }]) {
    for (const colorScheme of ['light', 'dark']) {
      const page = await browser.newPage({ viewport, colorScheme });
      if (emittedHtml) {
        await page.route('http://animate.test/**', route => {
          const path = new URL(route.request().url()).pathname;
          if (path === '/') return route.fulfill({ contentType: 'text/html; charset=utf-8', body: emittedHtml });
          if (path === '/styles.css') return route.fulfill({ contentType: 'text/css; charset=utf-8', body: stylesheet });
          return route.fulfill({ status: 404 });
        });
        await page.goto('http://animate.test/');
        assert.equal(await page.locator('html').getAttribute('lang'), '');
        assert.equal(await page.locator('title').getAttribute('lang'), 'en');
        assert.equal(await page.locator('main').getAttribute('data-level'), 'window');
        assert.equal(await page.locator('main > canvas').getAttribute('data-window-frame-content'), '');
        const deck = await page.locator('#animate-presentation-deck').textContent();
        assert.equal(JSON.parse(deck).title, 'A & B < C · Wärme');
      } else {
        await page.setContent(`<style>${stylesheet}</style><main id="animate-presentation-root" data-level="window"><canvas id="animate-presentation-canvas" data-window-frame-content width="1280" height="720"></canvas></main>`);
      }
      const measure = await page.evaluate(() => {
        const frame = document.querySelector('main'), canvas = document.querySelector('canvas');
        const style = getComputedStyle(frame), canvasStyle = getComputedStyle(canvas), box = canvas.getBoundingClientRect();
        return { frame: [frame.offsetWidth, frame.offsetHeight], width: box.width, height: box.height,
          radius: style.borderRadius, frameBorder: style.borderWidth, canvasBorder: canvasStyle.borderWidth,
          base: getComputedStyle(document.documentElement).backgroundColor, window: style.backgroundColor };
      });
      assert.deepEqual(measure.frame, [viewport.width, viewport.height]);
      assert(Math.abs(measure.width / measure.height - 16 / 9) < 0.001);
      assert(measure.width <= viewport.width - 2 && measure.height <= viewport.height - 2);
      assert.equal(measure.radius, '0px');
      assert.equal(measure.frameBorder, '1px');
      assert.equal(measure.canvasBorder, '0px');
      assert.equal(measure.base, colorScheme === 'dark' ? 'rgb(0, 17, 23)' : 'rgb(247, 243, 227)');
      assert.notEqual(measure.window, measure.base);
      await page.close();
    }
  }
  console.log('[DEBUG] Animate static frame passed at three viewports in light and dark');
} finally {
  await browser.close();
}
