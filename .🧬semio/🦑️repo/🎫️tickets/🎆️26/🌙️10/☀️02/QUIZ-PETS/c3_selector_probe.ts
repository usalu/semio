/** 🔬️ C3 probe: whether jsdom (the quiz suites' environment) understands the props selector of the quiz glue, which
 * leaves out the marked rows inside a drag ghost. Run from the repository root: `bun <TK>/c3_selector_probe.ts`. */

import { JSDOM } from "jsdom";

const page = new JSDOM(`<div class="quiz-app"><ol><li data-pet-prop="a/b/c">row</li></ol><ol class="quiz-drag-ghost"><li data-pet-prop="a/b/c">ghost</li></ol><section data-pet-topic="a">card</section></div>`);
const found = [...page.window.document.querySelectorAll("[data-pet-prop]:not(.quiz-drag-ghost *)")].map((element) => element.textContent);
const matched = [...page.window.document.querySelectorAll("[data-pet-prop]")].map((element) => element.matches("[data-pet-prop]:not(.quiz-drag-ghost *)"));
process.stdout.write(`${JSON.stringify({ found, matched })}\n`);
