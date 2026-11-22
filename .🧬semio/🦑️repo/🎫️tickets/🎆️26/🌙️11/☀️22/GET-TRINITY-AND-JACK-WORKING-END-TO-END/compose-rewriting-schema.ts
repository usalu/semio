import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const text = readFileSync(path, "utf8");
const lines = text.split("\n");
const index = lines.findIndex(line => line.startsWith("export const REWRITING_SQLITE_SCHEMA="));
const line = lines[index]!;
const cut = line.indexOf("CREATE TABLE jack_document");
if (cut < 0) { console.log("already composed"); process.exit(0); }
lines[index] = line.slice(0, cut) + '"+JACK_SQLITE_SCHEMA;';
writeFileSync(path, lines.join("\n"));
console.log("composed", lines[index]!.slice(-120));
