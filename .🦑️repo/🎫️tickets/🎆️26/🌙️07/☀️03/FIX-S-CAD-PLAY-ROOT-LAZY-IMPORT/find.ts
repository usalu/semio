import { readFileSync, readdirSync, statSync } from "fs";
import { resolve } from "path";

function walk(dir: string, results: string[] = []) {
  const list = readdirSync(dir);
  for (const file of list) {
    if (file === "node_modules" || file === ".git" || file === ".nx" || file === "dist" || file === "target") continue;
    const path = resolve(dir, file);
    const stat = statSync(path);
    if (stat && stat.isDirectory()) {
      walk(path, results);
    } else {
      if (path.endsWith(".ts") || path.endsWith(".tsx") || path.endsWith(".js") || path.endsWith(".jsx")) {
        try {
          const content = readFileSync(path, "utf8");
          if (content.includes("installation.mdx")) {
            results.push(path);
          }
        } catch {}
      }
    }
  }
  return results;
}

console.log(walk("."));
