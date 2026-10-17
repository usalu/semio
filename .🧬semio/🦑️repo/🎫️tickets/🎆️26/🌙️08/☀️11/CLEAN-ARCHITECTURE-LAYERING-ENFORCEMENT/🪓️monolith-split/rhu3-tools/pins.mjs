import fs from "node:fs";
import { spawnSync } from "node:child_process";
const R = "/Users/ueli/Documents/semio/";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean);
for (const m of spec.moves) {
  const from = m.from.normalize("NFC");
  const forms = [from, from.startsWith(OS) ? from.slice(OS.length) : null, from.split("/").slice(-3).join("/")].filter(Boolean);
  console.log("## " + from.replace(OS, "OS/"));
  const seen = new Set();
  for (const f of forms) {
    for (const line of git(["grep", "-n", "-F", "-e", f, "--", ":!*.ts", ":!*.tsx", ":!*.lock", ":!.🧬semio", ":!.cursor", ":!*.md"])) {
      const key = line.slice(0, 160); if (seen.has(key)) continue; seen.add(key);
      console.log("   " + line.slice(0, 230).replace(OS, "OS/"));
    }
  }
}
