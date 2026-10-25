import { Scanner } from "@tailwindcss/oxide";
import { resolve } from "node:path";
const root = process.cwd();
const dirs = process.argv.slice(2);
const pattern = process.env.TW_PATTERN ?? "**/*";
const t = Date.now();
const scanner = new Scanner({ sources: dirs.map((d) => ({ base: resolve(root, d), pattern, negated: false })) });
const candidates = scanner.scan();
console.log(`[tw] ${dirs.length} sources pattern=${pattern}: ${candidates.length} candidates, ${scanner.files.length} files, ${Date.now() - t} ms, rss ${Math.round(process.memoryUsage().rss / 1e6)} MB`);
