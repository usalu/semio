import { existsSync, readFileSync } from "node:fs";

const LEDGER = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/cargo-fix-x/backup";
let redirect: Record<string, string> | null = null;

/** ⏪ Reads a file; with RS_SWEEP_BEFORE=1 files this tool fixed are read from their first-touch backup. */
export function readText(path: string): string {
  if (process.env.RS_SWEEP_BEFORE === "1") {
    if (redirect === null) redirect = existsSync(`${LEDGER}/ledger.json`) ? JSON.parse(readFileSync(`${LEDGER}/ledger.json`, "utf8")) : {};
    const h = redirect![path];
    if (h) return readFileSync(`${LEDGER}/${h}`, "utf8");
  }
  return readFileSync(path, "utf8");
}
