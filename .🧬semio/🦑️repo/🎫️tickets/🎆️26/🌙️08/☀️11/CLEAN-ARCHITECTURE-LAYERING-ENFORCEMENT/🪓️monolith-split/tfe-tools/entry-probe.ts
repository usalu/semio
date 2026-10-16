/** 🔬 Load one fixture entry in this fresh process and exercise every export. */
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

const path = process.argv[2]!;
const module = await import(path);
let loaders = 0, urls = 0, eager = 0;
const bad: string[] = [];
for (const [name, value] of Object.entries(module)) {
  if (typeof value === "function") {
    loaders++;
    try {
      const loaded = await (value as () => Promise<unknown>)();
      if (loaded === undefined || loaded === null) bad.push(`${name}: empty`);
    } catch (error) {
      bad.push(`${name}: ${(error as Error).message.slice(0, 120)}`);
    }
  } else if (value instanceof URL) {
    urls++;
    if (!existsSync(fileURLToPath(value))) bad.push(`${name}: missing ${value.href.slice(-80)}`);
  } else if (value === undefined || value === null) {
    bad.push(`${name}: empty eager`);
  } else eager++;
}
console.log(JSON.stringify({ eager, loaders, urls, bad }));
