import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = readdirSync(dir);
  } catch {
    return acc;
  }
  for (const name of entries) {
    if (["node_modules", "target", "dist", ".git"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.isDirectory()) walk(p, pred, acc);
    else if (pred(p)) acc.push(p);
  }
  return acc;
}

const root = "/Users/ueli/Documents/semio";
const cores = walk(root, (p) => p.includes("🧩core") && p.endsWith("component.ts") && p.includes("modules"));
if (cores.length !== 1) {
  console.log(cores);
  throw new Error(`expected 1 core component, got ${cores.length}`);
}
const corePath = cores[0];
let text = readFileSync(corePath, "utf8");
const old = `export function ephemeralBox<T>(key: string, init: T | (() => T)): EphemeralBox<T> {
  let box = ephemeralBoxes.get(key) as EphemeralBox<T> | undefined;
  if (!box) {
    box = { current: typeof init === "function" ? (init as () => T)() : init };
    ephemeralBoxes.set(key, box as EphemeralBox<unknown>);
  }
  return box;
}`;
const neu = `export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T> {
  let box = ephemeralBoxes.get(key) as EphemeralBox<T> | undefined;
  if (!box) {
    box = { current: init };
    ephemeralBoxes.set(key, box as EphemeralBox<unknown>);
  }
  return box;
}`;
if (!text.includes(old)) {
  // try flexible whitespace
  const re =
    /export function ephemeralBox<T>\(key: string, init: T \| \(\(\) => T\)\): EphemeralBox<T> \{\n  let box = ephemeralBoxes\.get\(key\) as EphemeralBox<T> \| undefined;\n  if \(!box\) \{\n    box = \{ current: typeof init === "function" \? \(init as \(\) => T\)\(\) : init \};\n    ephemeralBoxes\.set\(key, box as EphemeralBox<unknown>\);\n  \}\n  return box;\n\}/;
  if (!re.test(text)) {
    const idx = text.indexOf("export function ephemeralBox");
    console.log(text.slice(idx, idx + 400));
    throw new Error("ephemeralBox block not found");
  }
  text = text.replace(re, neu);
} else {
  text = text.replace(old, neu);
}
writeFileSync(corePath, text);
console.log("fixed", corePath);
