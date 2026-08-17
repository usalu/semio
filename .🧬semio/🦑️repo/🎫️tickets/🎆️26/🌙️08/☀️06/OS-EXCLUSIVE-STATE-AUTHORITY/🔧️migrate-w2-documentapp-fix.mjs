import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const PLUGINS = join("/Users/ueli/Documents/semio", "✏️s/🔌️plugins");

function walk(d, pred, acc = []) {
  for (const n of readdirSync(d)) {
    if (["node_modules", "target", ".git"].includes(n)) continue;
    const p = join(d, n);
    try {
      const s = statSync(p);
      if (s.isDirectory()) walk(p, pred, acc);
      else if (pred(p)) acc.push(p);
    } catch {}
  }
  return acc;
}

function fixFile(path) {
  let s = readFileSync(path, "utf8");
  if (!s.includes("impl DocumentApp for")) return false;
  let changed = false;

  const modAppId = s.match(/pub const APP_ID: &str = ("[^"]+");/);
  if (s.includes("const APP_ID: &'static str = APP_ID;") && modAppId) {
    s = s.replace("const APP_ID: &'static str = APP_ID;", `const APP_ID: &'static str = ${modAppId[1]};`);
    changed = true;
  }
  const modDoc = s.match(/pub const DOCUMENT_SCHEMA: &str = ("[^"]+");/);
  if (s.includes("const DOCUMENT_SCHEMA: &'static str = DOCUMENT_SCHEMA;") && modDoc) {
    s = s.replace(
      "const DOCUMENT_SCHEMA: &'static str = DOCUMENT_SCHEMA;",
      `const DOCUMENT_SCHEMA: &'static str = ${modDoc[1]};`,
    );
    changed = true;
  }

  const handleFix =
    /fn handle\(([^)]+)\) -> Result<Emit</g;
  if (handleFix.test(s) && !s.includes("_draft: &DraftView")) {
    s = s.replace(
      /fn handle\(([^)]+)\) -> Result<Emit</g,
      "fn handle($1, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<",
    );
    changed = true;
  }

  if (s.includes("eval_session_lock(self)")) {
    s = s.replace(/eval_session_lock\(self\)/g, "eval_session_lock()");
    changed = true;
  }

  if (s.includes("\n    }\n}\n\nimpl DocumentApp for LayoutPlayApp")) {
    s = s.replace("\n    }\n}\n\nimpl DocumentApp for LayoutPlayApp", "\n\nimpl DocumentApp for LayoutPlayApp");
    changed = true;
  }

  s = s.replace(
    /fn command_id\(command: &[^{]+\) -> &'static str \{\s*\n\s*command\.command_id\(\)\s*\n\s*\}/g,
    (block) => block.replace("-> &'static str", "-> &str"),
  );

  if (changed) writeFileSync(path, s);
  return changed;
}

function hoistMutex(path) {
  let s = readFileSync(path, "utf8");
  const m = s.match(/pub struct (\w+) \{\s*(\w+): Mutex<([^>]+)>,?\s*\}/);
  if (!m) return false;
  const [, structName, fname, ty] = m;
  const staticName = `${structName.toUpperCase()}_${fname.toUpperCase()}`;
  const init = ty.includes("LayoutEngine") ? "LayoutEngine::new()" : `<${ty}>::default()`;
  const replacement = `#[derive(Default)]
pub struct ${structName};

static ${staticName}: std::sync::LazyLock<std::sync::Mutex<${ty}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(${init}));

fn ${fname}_lock() -> std::sync::MutexGuard<'static, ${ty}> {
    ${staticName}.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}`;
  if (s.includes(staticName)) return false;
  s = s.replace(/#\[derive\(Default\)\]\s*\npub struct \w+ \{\s*\w+: Mutex<[^>]+>,?\s*\}/, replacement);
  s = s.replace(new RegExp(`fn ${fname}_lock\\([^)]+\\)`, "g"), `fn ${fname}_lock()`);
  s = s.replace(new RegExp(`${fname}_lock\\(self\\)`, "g"), `${fname}_lock()`);
  s = s.replace(new RegExp(`${fname}_lock\\(app\\)`, "g"), `${fname}_lock()`);
  if (!s.includes("LazyLock")) {
    s = s.replace("use std::sync::Mutex;", "use std::sync::{LazyLock, Mutex};");
  }
  writeFileSync(path, s);
  return true;
}

const files = walk(PLUGINS, (p) => p.endsWith("🦀️component.rs"));
let n = 0;
for (const f of files) {
  if (fixFile(f)) n++;
  if (readFileSync(f, "utf8").includes("Mutex<") && readFileSync(f, "utf8").includes("impl DocumentApp for")) {
    hoistMutex(f);
  }
}
console.log("fixed", n);
