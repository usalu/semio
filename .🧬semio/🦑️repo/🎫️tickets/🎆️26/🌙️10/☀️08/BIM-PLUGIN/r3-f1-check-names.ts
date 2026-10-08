#!/usr/bin/env bun
/** 🔎️ Every file and folder of the bim plugin carries exactly one emoji, written as codepoint + U+FE0F, unique among its siblings; only the files a tool locates by exact name (AGENTS.md, README.md, Cargo.toml, Cargo.lock, package.json) are exempt. */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { plugin } from "./r3-f1-paths.ts";

const TOOL_FILES = new Set(["AGENTS.md", "README.md", "Cargo.toml", "Cargo.lock", "package.json"]);
const pictographic = /\p{Extended_Pictographic}/gu;
let bad = 0;
const walk = (dir: string) => {
  const seen = new Map<string, string>();
  for (const name of readdirSync(dir)) {
    if (TOOL_FILES.has(name)) continue;
    const emojis = [...name.matchAll(pictographic)];
    const first = name.codePointAt(0)!;
    const ok = emojis.length === 1 && name.startsWith(String.fromCodePoint(first)) && name[String.fromCodePoint(first).length] === "️";
    const key = String.fromCodePoint(first);
    if (!ok) {
      bad += 1;
      console.log(`[BAD NAME] ${join(dir, name)}`);
    } else if (seen.has(key)) {
      bad += 1;
      console.log(`[DUPLICATE EMOJI] ${join(dir, name)} vs ${seen.get(key)}`);
    }
    seen.set(key, name);
    const full = join(dir, name);
    if (statSync(full).isDirectory()) walk(full);
  }
};
walk(plugin);
console.log(bad === 0 ? "names ok" : `${bad} problem(s)`);
