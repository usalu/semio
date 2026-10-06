#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../..");
const path = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx");
let text = readFileSync(path, "utf8");
const needle = "  ShellBrandLogo,\n  shellChromeTitleClassName,";
const replacement = "  ShellBrandLogo,\n  ResponsiveLabel,\n  shellChromeTitleClassName,";
if (!text.includes("  ResponsiveLabel,\n  shellChromeTitleClassName,")) {
  if (!text.includes(needle)) throw new Error("import anchor missing");
  text = text.replace(needle, replacement);
  writeFileSync(path, text);
  console.log("import patched");
} else {
  console.log("import ok");
}
