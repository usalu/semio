import fs from "fs";
import path from "path";
const ticket = process.argv[2];
const log = JSON.parse(fs.readFileSync(path.join(ticket, "🧪w3-ephemeral-migrate.log"), "utf8"));
for (const r of log) {
  const abs = path.join(process.cwd(), r.file);
  const text = fs.readFileSync(abs, "utf8");
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (!l.includes("ephemeralBox")) continue;
    if (l.includes("ephemeralBox<()>") || l.includes('", >') || /ephemeralBox<[^>]*=/.test(l) || /=\s*null\);/.test(l) && l.includes("ephemeralBox") && l.includes("> void")) {
      console.log(r.file + ":" + (i + 1), l.slice(0, 200));
    }
    // generic heuristic: unbalanced
    const open = (l.match(/</g) || []).length;
    const close = (l.match(/>/g) || []).length;
    if (l.includes("ephemeralBox<") && open !== close) {
      console.log("UNBAL", r.file + ":" + (i + 1), l.slice(0, 200));
    }
  }
}
