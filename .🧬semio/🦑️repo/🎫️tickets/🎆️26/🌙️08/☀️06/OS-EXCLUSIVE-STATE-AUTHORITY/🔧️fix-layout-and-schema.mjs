import { readFileSync, writeFileSync } from "fs";
import { execSync } from "child_process";

const layout = "✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs";
let s = readFileSync(layout, "utf8");
s = s.replace(
  /static LAYOUTPLAYAPP_LAYOUT_ENGINE:[\s\S]*?fn layout_engine_lock\(\)[\s\S]*?\n\}\n\nimpl Default for LayoutPlayApp \{\n    fn default\(\) -> Self \{\n        Self\n    \}\n\}\n\n\n/,
  "",
);
s = s.replace(
  `    const APP_ID: &'static str = LAYOUT_PLAY_APP_ID;

    fn document_schema() -> &'static str {
        crate::artifacts::layout::LAYOUT_FIXTURE_SCHEMA
    }
`,
  `    const APP_ID: &'static str = LAYOUT_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::layout::LAYOUT_FIXTURE_SCHEMA;
`,
);
s = s.replace(`        let mut engine = self.layout_engine.lock().expect("layout engine");`, `        let mut engine = LayoutEngine::new();`);
writeFileSync(layout, s);
console.log("layout ok");

const files = execSync('rg -l "fn document_schema\\(\\)" --glob "*.rs" "✏️s/🔌️plugins"', { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter(Boolean);
console.log("document_schema methods", files.length);
for (const f of files) {
  let text = readFileSync(f, "utf8");
  const re = /fn document_schema\(\) -> &'static str \{\n\s*([^\n]+)\n\s*\}/;
  const m = text.match(re);
  if (!m) continue;
  const expr = m[1].trim().replace(/;$/, "");
  if (text.includes("const DOCUMENT_SCHEMA")) {
    text = text.replace(m[0], "");
  } else {
    text = text.replace(m[0], `const DOCUMENT_SCHEMA: &'static str = ${expr};`);
  }
  writeFileSync(f, text);
  console.log("patched", f);
}
