import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import { compileOwnedMarkdownToHtml } from "../../../../../../../🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts";
const proc = unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeStringify);
const t = "`";
const cases: [string, string][] = [
  ["blockquote", "> quoted line"],
  ["thematic-break", "a\n\n---\n\nb"],
  ["image", "![alt](https://example.com/a.png)"],
  ["setext-heading", "Title\n====="],
  ["strikethrough", "~~gone~~"],
  ["task-list", "- [ ] todo\n- [x] done"],
  ["reference-link", "[docs][d]\n\n[d]: https://example.com"],
  ["raw-html", "<div>kept?</div>"],
  ["indented-code", "    const x = 1;"],
  ["loose-list", "- alpha\n\n- beta"],
];
const norm = (h: string) => h.replace(/>\s+</g, "><").trim();
for (const [name, md] of cases) {
  const own = await compileOwnedMarkdownToHtml(md);
  const ref = String(await proc.process(md));
  console.log((norm(own) === norm(ref) ? "OK   " : "DIFF ") + name + "\n  own: " + JSON.stringify(own) + "\n  ref: " + JSON.stringify(ref));
}
