import { readFileSync, readdirSync } from "node:fs";
const f = readdirSync("G:/claude-scratch/snap").find(n => n.endsWith("-listings.json"))!;
const rows = JSON.parse(readFileSync("G:/claude-scratch/snap/" + f, "utf8"));
console.error("[DEBUG] rows", rows.length);
