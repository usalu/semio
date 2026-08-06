import { readFileSync, readdirSync, existsSync, writeFileSync } from "fs";
import { join, relative, dirname } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = join(
  ROOT,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE",
);
const lines = [];
const log = (...a) => {
  const s = a.map(String).join(" ");
  lines.push(s);
  console.log(s);
};

function walkFiles(d, acc = []) {
  for (const e of readdirSync(d, { withFileTypes: true })) {
    const p = join(d, e.name);
    if (e.isDirectory()) {
      if (["node_modules", "target", ".git", "storybook-static"].includes(e.name)) continue;
      walkFiles(p, acc);
    } else {
      acc.push(p);
    }
  }
  return acc;
}

log("=== package markers ===");
const pkgs = [
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/