import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";

const ticket = path.dirname(fileURLToPath(import.meta.url));
const scriptPath = "/Users/ueli/Documents/semio/📜️script.ts";
let script = fs.readFileSync(scriptPath, "utf8");

function replaceSet(name, entries) {
  const re = new RegExp(
    `(const ${name}[^=]*=\\s*new Set(?:<[^>]*>)?\\(\\s*\\[)([\\s\\S]*?)(\\]\\s*\\))`,
  );
  if (!re.test(script)) throw new Error("set not found: " + name);
  const body =
    entries.length === 0
      ? ""
      : "\n" + entries.map((e) => `  ${JSON.stringify(e)},`).join("\n") + "\n";
  script = script.replace(re, `$1${body}$3`);
  console.log(name, "->", entries.length);
}

// Empty empty-example exemptions — binaries were padded
replaceSet("POLICY_EMPTY_EXAMPLE_EXEMPTIONS", []);

// Keep derive exemptions until derive emission deleted (P6 agent); don't empty yet
// Empty distinctness/generic/declared-use if not already
replaceSet("POLICY_SPEC_DISTINCTNESS_EXEMPTIONS", []);
replaceSet("POLICY_GENERIC_SPEC_EXEMPTIONS", []);
replaceSet("POLICY_DECLARED_USE_EXEMPTIONS", []);

fs.writeFileSync(scriptPath, script);

// Probe policy by importing isn't easy; instead run a dry excerpt
fs.appendFileSync(
  path.join(ticket, "progress-v2.md"),
  `\n\n## P6 shrink exemptions\n- Emptied EMPTY_EXAMPLE, DISTINCTNESS, GENERIC_SPEC, DECLARED_USE exemption sets\n- Wiring + derive exemptions still pending full P6 codec migration\n`,
);
console.log("script updated");
