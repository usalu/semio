// #region 🧲Header
/** @emoji 📝 Markdown-to-HTML compiler behind a swappable interface for presentation renderers. */
// #endregion 🧲Header

// #region 🔌Adapters
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import { unified } from "unified";
// #endregion 🔌Adapters

//#region 🔖Compiler
/** @emoji 📝 Compiles markdown source into an HTML fragment. */
export interface MarkdownHtmlCompiler {
	compile(markdown: string): Promise<string>;
}

const defaultMarkdownHtmlCompiler: MarkdownHtmlCompiler = {
	async compile(markdown) {
		const file = await unified()
			.use(remarkParse)
			.use(remarkGfm)
			.use(remarkRehype)
			.use(rehypeStringify)
			.process(markdown);
		return String(file);
	},
};

let markdownHtmlCompiler: MarkdownHtmlCompiler = defaultMarkdownHtmlCompiler;

/** @emoji 🔌 Replaces the markdown HTML compiler (tests or alternate renderers). */
export function setMarkdownHtmlCompiler(compiler: MarkdownHtmlCompiler): void {
	markdownHtmlCompiler = compiler;
}

/** @emoji 📝 Compiles markdown through the active {@link MarkdownHtmlCompiler}. */
export function compileMarkdownToHtml(markdown: string): Promise<string> {
	return markdownHtmlCompiler.compile(markdown);
}
//#endregion 🔖Compiler

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("compileMarkdownToHtml", () => {
		it("renders GFM tables as HTML", async () => {
			const html = await compileMarkdownToHtml(
				"| A | B |\n| - | - |\n| `x` | y |",
			);
			expect(html).toContain("<table");
			expect(html).toContain("<code>x</code>");
		});
	});
}
//#endregion 🧪Tests
