/** 🌐️ CDN-like static server for the staged site artifact (GitHub Pages behaviour: files as they are, `index.html` for a
 * directory, `404.html` with status 404 for anything else). `bun site_infra_static.ts [directory] [port]`. */
import { existsSync, statSync } from "node:fs";
import { join, normalize } from "node:path";

const root = process.argv[2] ?? join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/dist/pages/quizzes");
const port = Number(process.argv[3] ?? "6062");
Bun.serve({
  hostname: "127.0.0.1",
  port,
  fetch(request) {
    const path = normalize(decodeURIComponent(new URL(request.url).pathname)).replace(/^[\\/]+/u, "");
    const file = join(root, path);
    const target = existsSync(file) && statSync(file).isDirectory() ? join(file, "index.html") : file;
    if (!path.startsWith("..") && existsSync(target) && statSync(target).isFile()) return new Response(Bun.file(target));
    return new Response(Bun.file(join(root, "404.html")), { status: 404, headers: { "content-type": "text/html; charset=utf-8" } });
  },
});
console.log(`[DEBUG] serving ${root} on http://127.0.0.1:${port}`);
