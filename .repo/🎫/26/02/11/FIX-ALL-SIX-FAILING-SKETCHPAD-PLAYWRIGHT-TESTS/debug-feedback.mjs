import { chromium } from 'playwright';

const browser = await chromium.launch({
  headless: true,
  channel: 'chrome',
  args: ['--no-sandbox', '--disable-gpu', '--disable-dev-shm-usage', '--disable-setuid-sandbox']
});
const page = await browser.newPage();
await page.goto('http://localhost:5173/feedback');
await page.waitForTimeout(5000);

const stacks = await page.evaluate(() => document.querySelectorAll('.lm_stack').length);
console.log('GL stacks:', stacks);

const items = await page.evaluate(() => document.querySelectorAll('.lm_item_container').length);
console.log('GL item containers:', items);

const formCount = await page.evaluate(() => {
  return document.querySelectorAll('[id="compose.sketchpad.app.feedback.form.kind"]').length;
});
console.log('form.kind elements:', formCount);

const glConfig = await page.evaluate(() => {
  const stacks = document.querySelectorAll('.lm_stack');
  const items = [];
  stacks.forEach((stack, i) => {
    const contents = stack.querySelectorAll(':scope > .lm_items > .lm_item');
    const tabs = stack.querySelectorAll('.lm_tab');
    const feedbackDivs = stack.querySelectorAll('#feedback');
    items.push({ stack: i, contentItems: contents.length, tabs: tabs.length, feedbackDivs: feedbackDivs.length });
  });
  const allFeedbackDivs = document.querySelectorAll('#feedback');
  return { stacks: items, totalFeedbackDivs: allFeedbackDivs.length };
});
console.log('GL config:', JSON.stringify(glConfig, null, 2));

await browser.close();
