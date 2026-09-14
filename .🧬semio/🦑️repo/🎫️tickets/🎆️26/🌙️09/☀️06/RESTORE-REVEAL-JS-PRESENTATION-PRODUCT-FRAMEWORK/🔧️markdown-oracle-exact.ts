import { readdirSync, readFileSync } from "node:fs";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import { compileOwnedMarkdownToHtml } from "../../../../../../../🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts";
const proc = unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeStringify);
const dir = "🧰️framework/🛍️products/🎤️presentation/🧪️tests/📝️markdown-html-compilation/🧫️fixtures";
let exact = 0, trimmed = 0, bad = 0;
for (const name of readdirSync(dir).sort()) {
  const md = readFileSync(`${dir}/${name}`, "utf8");
  const own = await compileOwnedMarkdownToHtml(md);
  const ref = String(await proc.process(md));
  if (own === ref) { exact += 1; continue; }
  if (own.trim() === ref.trim()) { trimmed += 1; console.log("TRIM-ONLY " + name + " own=" + JSON.stringify(own.slice(-20)) + " ref=" + JSON.stringify(ref.slice(-20))); continue; }
  bad += 1;
  console.log("DIFF " + name + "\n  own: " + JSON.stringify(own) + "\n  ref: " + JSON.stringify(ref));
}
console.log(`exact=${exact} trim-only=${trimmed} diff=${bad}`);
