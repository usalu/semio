import { globSync } from "glob";
import { resolve } from "node:path";
const [pattern, ...opts] = process.argv.slice(2);
const t = Date.now();
const files = globSync(pattern, { cwd: resolve(".storybook"), ignore: ["**/node_modules/**"], ...(opts.includes("nofollow") ? { follow: false } : {}) });
console.log(`[glob] ${files.length} files ${Date.now() - t} ms rss ${Math.round(process.memoryUsage().rss / 1e6)} MB`);
