import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function findFile(root, pred, acc = []) {
  for (const name of readdirSync(root)) {
    if (name === "node_modules" || name === ".git" || name === "compose" || name === "target" || name === ".🦑️repo") continue;
    const p = join(root, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) findFile(p, pred, acc);
    else if (pred(p)) acc.push(p);
  }
  return acc;
}

const stories = findFile(".", (p) => p.includes("🍞Breadcrumb") && p.includes("story"));
const story = stories[0];
if (!story) throw new Error("breadcrumb story not found");
let text = readFileSync(story, "utf8");
const old = 'next={{ path: "tutorials/hello-compose", title: "Hello Compose", section: "Tutorials" }}';
const neu = 'next={{ path: "tutorials", title: "Tutorials", section: "Basics" }}';
if (!text.includes(old)) throw new Error("breadcrumb pattern missing");
writeFileSync(story, text.replace(old, neu));
console.log("fixed", story);

let preview = readFileSync(".storybook/preview.tsx", "utf8");
const before = preview;
// Remove compose loader entry
preview = preview.replace(/\n\s*compose: \(\) => import\("\.\/compose\/algorithm\/kit-store\/index\.tsx"\)\.then\(\(m\) => m\.ensureComposeWasm\(\)\),?\n/, "\n");
// Update comments mentioning compose UI / algorithms and kit-store path
preview = preview.replace(" (compose UI / algorithms).", ".");
preview = preview.replace(/\n \* \(\.storybook\/compose\/algorithm\/kit-store\/index\.tsx\)\. Dynamic imports code-split per loader, so a/, "\n * Dynamic imports code-split per loader, so a");
if (preview === before) console.log("preview: no structural change? checking residuals...");
else writeFileSync(".storybook/preview.tsx", preview);
const residuals = preview.split("\n").map((l,i)=>[i+1,l]).filter(([,l])=>/compose/i.test(l) && !/composes decorators/.test(l));
console.log("preview residuals:", residuals);
