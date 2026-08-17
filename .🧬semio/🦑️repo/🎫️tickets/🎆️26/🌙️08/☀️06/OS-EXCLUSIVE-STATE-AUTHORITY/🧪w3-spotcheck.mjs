import fs from "fs";
import path from "path";

const ticket = process.argv[2];
const log = JSON.parse(fs.readFileSync(path.join(ticket, "🧪w3-ephemeral-migrate.log"), "utf8"));
for (const r of log.slice(0, 4)) {
  const abs = path.join(process.cwd(), r.file);
  const lines = fs.readFileSync(abs, "utf8").split("\n");
  console.log("\n====", r.file, "lets", r.letNames);
  let n = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes("ephemeralBox") || lines[i].includes("ephemeralMap") || lines[i].includes("ephemeralSet")) {
      console.log(String(i + 1).padStart(4), lines[i].slice(0, 160));
      if (++n > 12) break;
    }
  }
}
