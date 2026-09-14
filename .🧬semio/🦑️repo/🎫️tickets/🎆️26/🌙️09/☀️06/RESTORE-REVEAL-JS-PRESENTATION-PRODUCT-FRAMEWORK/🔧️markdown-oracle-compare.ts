import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import { compileOwnedMarkdownToHtml } from "../../../../../../../🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts";
const proc = unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeStringify);
const t = "`";
const cases: [string, string][] = [
  ["paragraph", "A plain paragraph of prose."],
  ["paragraph-soft-break", "First line of the paragraph\nsecond line of the same paragraph."],
  ["emphasis-star", "A *small* and **strong** phrase."],
  ["emphasis-underscore", "A _small_ and __strong__ phrase."],
  ["heading-1-6", "# One\n\n## Two\n\n### Three\n\n#### Four\n\n##### Five\n\n###### Six"],
  ["heading-inline", "## A *slide* **title**"],
  ["ul", "- alpha\n- beta\n- gamma"],
  ["ul-plus", "+ alpha\n+ beta"],
  ["ul-star", "* alpha\n* beta"],
  ["ol", "1. first\n2. second"],
  ["ol-start", "3. third\n4. fourth"],
  ["ol-paren", "1) first\n2) second"],
  ["ul-nested", "- alpha\n- beta\n  - nested"],
  ["ul-inline", "- an *emphasised* item\n- a **strong** item"],
  ["link", "See [docs](https://example.com/a?q=1&b=2)."],
  ["link-title", 'See [docs](https://example.com "The docs").'],
  ["link-inline", "See [**bold** docs](https://example.com)."],
  ["link-mailto", "Write [us](mailto:team@example.com)."],
  ["autolink", "Visit <https://example.com/x> now."],
  ["code-inline", "Use " + t + "x < y" + t + " here."],
  ["code-inline-ticks", "Use " + t.repeat(2) + "a " + t + " b" + t.repeat(2) + " here."],
  ["fenced-lang", t.repeat(3) + "ts\nconst x = 1;\n" + t.repeat(3)],
  ["fenced-plain", t.repeat(3) + "\nplain text\n" + t.repeat(3)],
  ["fenced-escape", t.repeat(3) + "html\n<div>a & b</div>\n" + t.repeat(3)],
  ["fenced-tilde", "~~~py\nx = 1\n~~~"],
  ["table", "| Left | Center | Right |\n| :--- | :----: | ----: |\n| a & b | **c** | " + t + "d" + t + " |"],
  ["table-plain", "| A | B |\n| --- | --- |\n| 1 | 2 |\n| 3 | 4 |"],
  ["escape-text", "A < B & C > D"],
  ["escape-backslash", "Literal \*stars\* and \_underscores\_."],
  ["hard-break", "line one  \nline two"],
  ["mixed-deck", "# Slide\n\nIntro *text* with " + t + "code" + t + ".\n\n- one\n- two\n\n" + t.repeat(3) + "js\nlet a = 1;\n" + t.repeat(3)],
];
const norm = (html: string) => html.replace(/>\s+</g, "><").trim();
let bad = 0;
for (const [name, md] of cases) {
  const own = await compileOwnedMarkdownToHtml(md);
  const ref = String(await proc.process(md));
  const ok = norm(own) === norm(ref);
  if (!ok) bad += 1;
  console.log((ok ? "OK   " : "DIFF ") + name);
  if (!ok) {
    console.log("  own: " + JSON.stringify(own));
    console.log("  ref: " + JSON.stringify(ref));
  }
}
console.log("diffs: " + bad + "/" + cases.length);
