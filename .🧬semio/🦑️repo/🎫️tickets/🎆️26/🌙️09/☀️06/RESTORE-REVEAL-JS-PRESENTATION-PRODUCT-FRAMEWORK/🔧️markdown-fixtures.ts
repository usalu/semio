import { mkdirSync, writeFileSync } from "node:fs";
const dir = "🧰️framework/🛍️products/🎤️presentation/🧪️tests/📝️markdown-html-compilation/🧫️fixtures";
mkdirSync(dir, { recursive: true });
const t = "`";
const files: Record<string, string> = {
  "paragraph.md": "A plain paragraph of prose.\n",
  "paragraph-soft-break.md": "First line of the paragraph\nsecond line of the same paragraph.\n",
  "headings.md": "# One\n\n## Two\n\n### Three\n\n#### Four\n\n##### Five\n\n###### Six\n",
  "heading-inline.md": "## A *slide* **title**\n",
  "emphasis-asterisk.md": "A *small* and **strong** phrase.\n",
  "emphasis-underscore.md": "A _small_ and __strong__ phrase.\n",
  "inline-code.md": "Use " + t + "x < y" + t + " here.\n",
  "inline-code-double-tick.md": "Use " + t.repeat(2) + "a " + t + " b" + t.repeat(2) + " here.\n",
  "escapes.md": "A < B & C > D\n\nLiteral \*stars\* and \_underscores\_.\n",
  "hard-break.md": "line one  \nline two\n",
  "list-unordered.md": "- alpha\n- beta\n- gamma\n",
  "list-unordered-plus.md": "+ alpha\n+ beta\n",
  "list-unordered-star.md": "* alpha\n* beta\n",
  "list-ordered.md": "1. first\n2. second\n",
  "list-ordered-start.md": "3. third\n4. fourth\n",
  "list-ordered-paren.md": "1) first\n2) second\n",
  "list-nested.md": "- alpha\n- beta\n  - nested\n",
  "list-inline.md": "- an *emphasised* item\n- a **strong** item\n",
  "link.md": "See [docs](https://example.com/a?q=1&b=2).\n",
  "link-title.md": "See [docs](https://example.com \"The docs\").\n",
  "link-inline.md": "See [**bold** docs](https://example.com).\n",
  "link-mailto.md": "Write [us](mailto:team@example.com).\n",
  "autolink.md": "Visit <https://example.com/x> now.\n",
  "code-fence-language.md": t.repeat(3) + "ts\nconst x = 1;\n" + t.repeat(3) + "\n",
  "code-fence-plain.md": t.repeat(3) + "\nplain text\n" + t.repeat(3) + "\n",
  "code-fence-escaping.md": t.repeat(3) + "html\n<div>a & b</div>\n" + t.repeat(3) + "\n",
  "code-fence-tilde.md": "~~~py\nx = 1\n~~~\n",
  "table-aligned.md": "| Left | Center | Right |\n| :--- | :----: | ----: |\n| a & b | **c** | " + t + "d" + t + " |\n",
  "table-plain.md": "| A | B |\n| --- | --- |\n| 1 | 2 |\n| 3 | 4 |\n",
  "slide.md": "# Slide\n\nIntro *text* with " + t + "code" + t + ".\n\n- one\n- two\n\n" + t.repeat(3) + "js\nlet a = 1;\n" + t.repeat(3) + "\n",
};
for (const [name, body] of Object.entries(files)) writeFileSync(`${dir}/${name}`, body, "utf8");
console.log(Object.keys(files).length + " fixtures written");
