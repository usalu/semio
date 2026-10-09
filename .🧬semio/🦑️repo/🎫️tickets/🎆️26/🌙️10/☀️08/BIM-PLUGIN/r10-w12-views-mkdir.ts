#!/usr/bin/env bun
/**
 * 🖼️ Creates a directory below the BIM subset. Arguments are path segments: a plain argument finds an existing child by name suffix, `emoji:<hex>:<rest>` names a (new) child as the code point plus U+FE0F plus `<rest>`.
 * Prints the created path so it can be handed to the Write tool.
 */
import { mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { subset } from "./r3-f1-paths.ts";

const em = (hex: string) => String.fromCodePoint(parseInt(hex, 16)) + "️";
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
let at = process.env.W12_ROOT ? process.env.W12_ROOT : subset;
for (const part of process.argv.slice(2)) {
  const match = /^emoji:([0-9a-f]+):(.*)$/.exec(part);
  at = match ? join(at, em(match[1]) + match[2]) : find(at, part);
}
mkdirSync(at, { recursive: true });
console.log(at);
