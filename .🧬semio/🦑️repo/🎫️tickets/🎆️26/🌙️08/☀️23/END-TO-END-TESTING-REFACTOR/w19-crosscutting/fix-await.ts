import { readFileSync, writeFileSync } from "node:fs";
import { execSync } from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";
const files = execSync(`find ${JSON.stringify(ROOT)} -name '*component.rs'`, { encoding: "utf8", maxBuffer: 64 << 20 }).split("\n").filter(Boolean);

let touched = 0;
for (const path of files) {
  const before = readFileSync(path, "utf8");
  let s = before;

  // 1. `Xxx::sniff(..)` compared directly — the async sweep made `sniff` a future but left the
  //    #[cfg(test)] call sites (never compiled by `cargo check --lib`) un-awaited.
  s = s.replace(/(Analyz\w*::sniff\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\))(?!\s*\.await)/g, "$1.await");

  // 2. `.await` landed on the BINDING's use sites instead of the producing call.
  if (/\bdiagnostics\.await\b/.test(s)) {
    s = s.replace(/(let diagnostics = [^;\n]*Validator::validate\([^;\n]*?\))(\.await)?;/g, "$1.await;");
    s = s.replace(/\bdiagnostics\.await\b/g, "diagnostics");
  }

  if (s !== before) {
    writeFileSync(path, s);
    touched++;
  }
}
console.log(`touched ${touched} files`);
