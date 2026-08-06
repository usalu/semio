import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const PLUGINS = join(ROOT, "✏️s/🔌️plugins");
const LOG = [];

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

function findImplRange(src, marker = "impl DocumentApp for") {
  const start = src.indexOf(marker);
  if (start < 0) return null;
  let depth = 0;
  let i = src.indexOf("{", start);
  if (i < 0) return null;
  const bodyStart = i + 1;
  depth = 1;
  i++;
  while (i < src.length && depth > 0) {
    const c = src[i];
    if (c === "{") depth++;
    else if (c === "}") depth--;
    i++;
  }
  return { start, end: i, body: src.slice(bodyStart, i - 1), full: src.slice(start, i) };
}

function extractReturnIdent(block) {
  const m = block.match(/->\s*&?str\s*\{\s*\n\s*([A-Z_][A-Z0-9_]*)\s*\n/s);
  if (m) return m[1];
  const m2 = block.match(/->\s*&?str\s*\{\s*\n\s*crate::[^;]+\n/s);
  if (m2) return null;
  const m3 = block.match(/->\s*&?str\s*\{\s*\n\s*"([^"]+)"/s);
  if (m3) return `"${m3[1]}"`;
  return null;
}

function ensureImports(src) {
  let s = src;
  const needsDraft = !s.includes("NoDraft") && s.includes("impl DocumentApp for");
  const needsEngine = !s.includes("EngineHandles") && s.includes("impl DocumentApp for");
  if (!needsDraft && !needsEngine) return s;

  const rePluginUse = /use semio_framework_plugin::\{([^}]+)\};/;
  const m = s.match(rePluginUse);
  if (m) {
    let inner = m[1];
    if (needsDraft) {
      if (!inner.includes("DraftView")) inner = `DraftView, ${inner}`;
      if (!inner.includes("NoDraft")) inner = `NoDraft, NoDraftOperation, ${inner}`;
    }
    s = s.replace(rePluginUse, `use semio_framework_plugin::{${inner}};`);
  }
  if (needsEngine && !s.includes("use store::EngineHandles")) {
    const anchor = s.indexOf("extern crate semio_framework_os_kernel as store;");
    if (anchor >= 0) {
      s = s.replace(
        "extern crate semio_framework_os_kernel as store;",
        "extern crate semio_framework_os_kernel as store;\nuse store::EngineHandles;",
      );
    } else if (s.includes("use semio_framework_plugin::")) {
      s = s.replace(/use semio_framework_plugin::\{[^}]+\};/, (line) => `${line}\nuse store::EngineHandles;`);
    }
  }
  return s;
}

function stripSelfInImplBody(body) {
  let b = body;
  b = b.replace(/\bfn ([a-z_][a-z0-9_]*)\(&self,\s*/g, "fn $1(");
  b = b.replace(/\bfn ([a-z_][a-z0-9_]*)\(&self\)/g, "fn $1()");
  b = b.replace(/\bfn ([a-z_][a-z0-9_]*)\(&self,/g, "fn $1(");
  b = b.replace(/-> &str \{/g, "-> &'static str {");
  b = b.replace(/fn command_id\([^)]*\) -> &str/g, "fn command_id(command: &Self::Command) -> &'static str");
  return b;
}

function addDraftTypes(body) {
  if (body.includes("type Draft")) return body;
  const anchor = body.match(/type ConfigOperation = [^;]+;\n/);
  if (!anchor) return body;
  return body.replace(
    anchor[0],
    `${anchor[0]}    type Draft = NoDraft;\n    type DraftOperation = NoDraftOperation;\n\n`,
  );
}

function migrateAppIdDocumentSchema(body) {
  let b = body;
  const appIdBlock = b.match(/\n\s*fn app_id\(&self\) -> &str \{[^}]+\}\n/s);
  if (appIdBlock) {
    const id = extractReturnIdent(appIdBlock[0]);
    if (id) {
      if (!b.includes("const APP_ID:")) {
        b = b.replace(/type Command = [^;]+;\n/, (m) => `${m}\n    const APP_ID: &'static str = ${id};\n`);
      }
      b = b.replace(appIdBlock[0], "\n");
    }
  }
  const docBlock = b.match(/\n\s*fn document_schema\(&self\) -> &str \{[^}]+\}\n/s);
  if (docBlock) {
    const id = extractReturnIdent(docBlock[0]);
    if (id) {
      if (!b.includes("const DOCUMENT_SCHEMA:")) {
        b = b.replace(/const APP_ID:[^;]+;\n/, (m) => `${m}    const DOCUMENT_SCHEMA: &'static str = ${id};\n`);
      }
      b = b.replace(docBlock[0], "\n");
    }
  }
  return b;
}

function migrateHandle(body) {
  let b = body;
  const old = /fn handle\(\s*command: &Self::Command,\s*doc: &DocumentView<'_, Self::Projection>,\s*cfg: &ConfigView<'_, Self::Config>\)\s*->\s*Result<Emit<([^>]+)>,\s*Fault>/;
  if (old.test(b)) {
    b = b.replace(
      old,
      "fn handle(command: &Self::Command, doc: &DocumentView<'_, Self::Projection>, cfg: &ConfigView<'_, Self::Config>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<$1, Self::DraftOperation>, Fault>",
    );
  }
  b = b.replace(/Emit<([^,>]+),\s*([^,>]+)>/g, (full, a, c) => {
    if (full.includes("DraftOperation")) return full;
    return `Emit<${a}, ${c}, Self::DraftOperation>`;
  });
  return b;
}

function hoistMutexAppStruct(src, structName) {
  const reStruct = new RegExp(
    `pub struct ${structName} \\{([\\s\\S]*?)\\}\\s*\\n\\s*impl Default for ${structName}`,
  );
  const m = src.match(reStruct);
  if (!m) return { src, hoisted: false };
  const fields = m[1].trim();
  if (!fields.includes("Mutex<")) return { src, hoisted: false };

  const fieldLines = fields
    .split("\n")
    .map((l) => l.trim())
    .filter(Boolean);
  const statics = [];
  const locks = [];
  for (const line of fieldLines) {
    const fm = line.match(/^(\w+):\s*Mutex<([^>]+)>/);
    if (!fm) continue;
    const [, fname, ty] = fm;
    const staticName = `${structName.toUpperCase()}_${fname.toUpperCase()}`;
    statics.push(
      `static ${staticName}: std::sync::LazyLock<std::sync::Mutex<${ty}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(<${ty}>::default()));`,
    );
    locks.push({
      fname,
      staticName,
      lockFn: `fn ${fname}_lock() -> std::sync::MutexGuard<'static, ${ty}> {
    ${staticName}.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}`,
    });
  }
  if (!statics.length) return { src, hoisted: false };

  let s = src.replace(
    reStruct,
    `#[derive(Default)]
pub struct ${structName};

${statics.join("\n\n")}

${locks.map((l) => l.lockFn).join("\n\n")}

impl Default for ${structName}`,
  );
  s = s.replace(
    new RegExp(`impl Default for ${structName} \\{[\\s\\S]*?\\}\\s*\\n`, "m"),
    `impl Default for ${structName} {
    fn default() -> Self {
        Self
    }
}

`,
  );
  for (const l of locks) {
    s = s.replace(new RegExp(`\\b${l.fname}_lock\\(self\\)`, "g"), `${l.fname}_lock()`);
    s = s.replace(new RegExp(`\\b${l.fname}_lock\\(app\\)`, "g"), `${l.fname}_lock()`);
  }
  if (!s.includes("LazyLock")) {
    s = s.replace(
      /use std::sync::Mutex;/,
      "use std::sync::{LazyLock, Mutex};",
    );
    if (!s.includes("use std::sync::Mutex")) {
      s = s.replace(/use std::sync::\{([^}]+)\};/, (line) => {
        if (line.includes("LazyLock")) return line;
        return line.replace("use std::sync::{", "use std::sync::{LazyLock, ");
      });
    }
  }
  return { src: s, hoisted: true };
}

function migrateFile(path) {
  let src = readFileSync(path, "utf8");
  if (!src.includes("impl DocumentApp for")) return { path, status: "skip" };
  if (src.includes("const APP_ID:") && src.includes("type Draft =") && !src.includes("fn app_id(&self)")) {
    return { path, status: "already" };
  }

  const structM = src.match(/impl DocumentApp for (\w+)/);
  const structName = structM?.[1];
  if (structName) {
    const h = hoistMutexAppStruct(src, structName);
    src = h.src;
  }

  src = ensureImports(src);
  const range = findImplRange(src);
  if (!range) return { path, status: "no-impl" };

  let body = range.body;
  body = addDraftTypes(body);
  body = migrateAppIdDocumentSchema(body);
  body = stripSelfInImplBody(body);
  body = migrateHandle(body);

  const newFull = `impl DocumentApp for ${structName} {${body}}`;
  src = src.slice(0, range.start) + newFull + src.slice(range.end);

  writeFileSync(path, src);
  return { path, status: "migrated", structName };
}

function migrateRegisterCalls(path) {
  let src = readFileSync(path, "utf8");
  let changed = false;
  const patterns = [
    [/\.register_document_app\(([^,]+),\s*\|\|\s*([A-Za-z0-9_:]+)\s*\)/g, ".register_document_app::<$2>($1)"],
    [/\.register_document_app\(([^,]+),\s*([A-Za-z0-9_:]+)::default\s*\)/g, ".register_document_app::<$2>($1)"],
    [/bundle\.register_document_app\(([^,]+),\s*\|\|\s*([A-Za-z0-9_:]+)\s*\)/g, "bundle.register_document_app::<$2>($1)"],
    [/bundle\.register_document_app\(([^,]+),\s*([A-Za-z0-9_:]+)::default\s*\)/g, "bundle.register_document_app::<$2>($1)"],
  ];
  for (const [re, rep] of patterns) {
    const next = src.replace(re, rep);
    if (next !== src) {
      src = next;
      changed = true;
    }
  }
  if (changed) writeFileSync(path, src);
  return changed;
}

const files = walk(PLUGINS, (p) => p.endsWith("🦀️component.rs") && readFileSync(p, "utf8").includes("impl DocumentApp for"));
const results = [];
for (const f of files.sort()) {
  try {
    results.push(migrateFile(f));
  } catch (e) {
    results.push({ path: f, status: "error", error: String(e) });
  }
}

const regFiles = walk(PLUGINS, (p) => p.endsWith(".rs") && readFileSync(p, "utf8").includes("register_document_app("));
for (const f of regFiles) migrateRegisterCalls(f);

writeFileSync(
  join(ROOT, ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/🧪w2-documentapp-migrate-log.json"),
  JSON.stringify(results, null, 2),
);
console.log(
  "migrated",
  results.filter((r) => r.status === "migrated").length,
  "already",
  results.filter((r) => r.status === "already").length,
  "errors",
  results.filter((r) => r.status === "error").length,
);
