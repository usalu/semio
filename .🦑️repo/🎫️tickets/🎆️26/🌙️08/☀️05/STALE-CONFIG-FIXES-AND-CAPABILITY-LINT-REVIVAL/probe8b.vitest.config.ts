import { defineConfig } from "vite";
import { globSync } from "node:fs";

console.error("[probe8b] cwd:", process.cwd());
const matches = globSync("**/🧪️vitest.config.ts", { cwd: process.cwd() });
console.error("[probe8b] raw matches (no exclude):", matches.length);
console.error(matches.slice(0,5));

export default defineConfig({});
