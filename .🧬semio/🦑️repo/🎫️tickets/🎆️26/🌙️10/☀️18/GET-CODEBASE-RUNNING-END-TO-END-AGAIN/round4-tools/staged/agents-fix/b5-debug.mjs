import fs from "node:fs";
import { load, set, edit } from "./lib.mjs";
import { stripDebugTs } from "./strip-debug-ts.mjs";
import { stripDebugRs } from "./strip-debug-rs.mjs";

export function build() {
  const files = JSON.parse(fs.readFileSync(new URL("./debug-appendix.json", import.meta.url), "utf8"));
  const notes = [];
  const owner = files.find(([, p]) => p.includes("owner-cmd-policy"))[1];
  edit(owner, 'setTimeout(()=>console.log("[DEBUG] interactive-ready")', 'setTimeout(()=>console.log("interactive-ready")');
  edit(owner, 'line.startsWith("[DEBUG] interactive-ready")', 'line.startsWith("interactive-ready")');
  edit(owner, `code = 'console.log("[DEBUG] foreign acquisition occurred")'`, `code = 'console.log("foreign acquisition occurred")'`);
  edit(owner, `'console.log("[DEBUG] original command")'`, `'console.log("original command")'`);
  for (const [, p] of files) {
    if (p === owner) continue;
    const before = load(p);
    const local = [];
    const after = (p.endsWith(".rs") ? stripDebugRs : stripDebugTs)(before, p, local);
    if (after.includes("[DEBUG]")) throw Error("[DEBUG] remains in " + p);
    set(p, after);
    if (local.length) notes.push(p + ": " + local.join("; "));
  }
  const prep = files.find(([, p]) => p.includes("📸️preparation/🧪️tests/🚪️public/🦀️.rs"))[1];
  edit(prep, "let closure = artifact_app_laws::close_registered_fixture_app(&mut app, policy);", "let _ = artifact_app_laws::close_registered_fixture_app(&mut app, policy);");
  return { notes, count: files.length };
}
