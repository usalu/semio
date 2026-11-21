/** 🔬 Records actual archive status while using the original isolated Note acceptance runner. */
import { dirname } from "node:path";

Bun.plugin({
  name: "note-archive-status-witness",
  setup(build) {
    build.onLoad({ filter: /note-acceptance\.mjs$/ }, async ({ path }) => {
      const original = await Bun.file(path).text();
      const marker = "const page = await context.newPage();";
      if (!original.includes(marker)) throw Error("The acceptance browser context is absent");
      const route = `
await context.route((address) => decodeURIComponent(address.pathname).includes("archive-transfer/") && decodeURIComponent(address.pathname).endsWith("🟦️.ts"), async route => {
  const response = await route.fetch();
  const contents = await response.text();
  const marker = "if (answer.status) {";
  if (!contents.includes(marker)) throw Error("The archive status hook is absent");
  const trace = \`const statusKey = JSON.stringify(answer.status); if (globalThis.__noteArchiveStatusKey !== statusKey) { globalThis.__noteArchiveStatusKey = statusKey; console.info("[DEBUG] archive case progress", seq, statusKey); }\`;
  await route.fulfill({ response, body: contents.replace(marker, marker + trace) });
});
`;
      return { contents: original.replace(marker, route + marker), loader: "js", resolveDir: dirname(path) };
    });
  },
});
