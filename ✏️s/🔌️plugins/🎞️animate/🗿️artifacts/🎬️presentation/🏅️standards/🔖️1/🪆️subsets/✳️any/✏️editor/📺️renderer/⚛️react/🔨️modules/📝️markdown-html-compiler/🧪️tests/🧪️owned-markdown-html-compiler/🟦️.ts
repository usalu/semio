type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { compileOwnedMarkdownToHtml } = dependencies;

  const { describe, expect, it } = vitest;
  const tick = "\x60";
  const fixtures = [
    {
      name: "prose",
      markdown: "# Heading\n\nA *small* and **strong** paragraph with " + tick + "x < y" + tick + " and [docs](https://example.com/a?q=1&b=2).",
      html: '<h1>Heading</h1>\n<p>A <em>small</em> and <strong>strong</strong> paragraph with <code>x &#x3C; y</code> and <a href="https://example.com/a?q=1&#x26;b=2">docs</a>.</p>',
    },
    { name: "fenced code", markdown: tick.repeat(3) + "ts\nconst x = " + tick + "<tag>" + tick + ";\n" + tick.repeat(3), html: '<pre><code class="language-ts">const x = ' + tick + "&#x3C;tag>" + tick + ";\n</code></pre>" },
    {
      name: "lists",
      markdown: "- alpha\n- beta\n  - nested\n\n3. third\n4. fourth",
      html: '<ul>\n<li>alpha</li>\n<li>beta\n<ul>\n<li>nested</li>\n</ul>\n</li>\n</ul>\n<ol start="3">\n<li>third</li>\n<li>fourth</li>\n</ol>',
    },
    {
      name: "table",
      markdown: "| Left | Center | Right |\n| :--- | :----: | ----: |\n| a & b | **c** | " + tick + "d" + tick + " |",
      html: '<table>\n<thead>\n<tr>\n<th align="left">Left</th>\n<th align="center">Center</th>\n<th align="right">Right</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td align="left">a &#x26; b</td>\n<td align="center"><strong>c</strong></td>\n<td align="right"><code>d</code></td>\n</tr>\n</tbody>\n</table>',
    },
  ] as const;
  describe("owned markdown html compiler", () => {
    for (const fixture of fixtures) {
      it("matches the installed compiler for " + fixture.name, async () => {
        expect(await compileOwnedMarkdownToHtml(fixture.markdown)).toBe(fixture.html);
      });
    }
    it("escapes text and drops raw HTML blocks", async () => {
      expect(await compileOwnedMarkdownToHtml("<script>alert(1)</script>\n\nA < B & C > D")).toBe("<p>A &#x3C; B &#x26; C > D</p>");
    });
    it("handles malformed input deterministically", async () => {
      const markdown = "**open *nested\n\n[missing](https://example.com\n\n" + tick.repeat(3) + "js\nunterminated";
      expect(await compileOwnedMarkdownToHtml(markdown)).toBe('<p>**open *nested</p>\n<p>[missing](<a href="https://example.com">https://example.com</a></p>\n<pre><code class="language-js">unterminated\n</code></pre>');
    });
    it("rejects executable and opaque URL schemes", async () => {
      expect(await compileOwnedMarkdownToHtml("[js](javascript:alert(1)) [data](data:text/html,x) [mail](mailto:a@example.com) [relative](/deck?q=1&x=2)")).toBe(
        '<p>js data <a href="mailto:a@example.com">mail</a> <a href="/deck?q=1&#x26;x=2">relative</a></p>',
      );
    });
  });

}
