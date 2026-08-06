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

for (const f of walk(PLUGINS, (p) => p.endsWith(".rs"))) {
  let s = readFileSync(f, "utf8");
  const orig = s;
  s = s.replace(/(\w+PlayApp)::default\(\)\.config_spec\(\)/g, "$1::config_spec()");
  s = s.replace(/(\w+PlayApp)\.config_spec\(\)/g, "$1::config_spec()");
  s = s.replace(/(\w+App)\.config_spec\(\)/g, "$1::config_spec()");
  s = s.replace(/(\w+PlayApp)\.io\(\)/g, "$1::io()");
  s = s.replace(/(\w+App)\.io\(\)/g, "$1::io()");
  s = s.replace(/\bself\.io\(\)/g, "Self::io()");
  s = s.replace(/\bself\.document_schema\(\)/g, "Self::DOCUMENT_SCHEMA");
  s = s.replace(
    /fn session_lock\(\) -> std::sync::MutexGuard<'static, DrawSession> \{[\s\S]*?\}\n\nfn session_lock\(\) -> std::sync::MutexGuard<'_, DrawSession> \{[\s\S]*?\}\n/,
    (m) => m.split("\n\n")[0] + "\n",
  );
  s = s.replace(
    /fn eval_session_lock\(\) -> std::sync::MutexGuard<'static, FlowEvalSession> \{[\s\S]*?\}\n\nfn eval_session_lock\(\) -> std::sync::MutexGuard<'_, FlowEvalSession> \{[\s\S]*?\}\n/,
    (m) => m.split("\n\n")[0] + "\n",
  );
  if (s !== orig) writeFileSync(f, s);
}
console.log("receiverless call-site fix done");
