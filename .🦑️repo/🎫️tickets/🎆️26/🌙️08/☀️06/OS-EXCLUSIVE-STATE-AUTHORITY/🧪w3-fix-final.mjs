import fs from "fs";
import path from "path";

function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", "dist", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isSymbolicLink()) continue;
    if (e.isDirectory()) walk(p, out);
    else if (/\.(ts|tsx)$/.test(e.name)) out.push(p);
  }
  return out;
}

function ensureImport(text, names) {
  const need = names.filter((n) => text.includes(n + "<") || text.includes(n + "("));
  if (!need.length) return text;
  const re = /import\s*\{([^}]*)\}\s*from\s*["']@semio-tech\/framework-core["'];?/;
  const m = text.match(re);
  if (m) {
    const existing = m[1].split(",").map((s) => s.trim()).filter(Boolean);
    const next = [...new Set([...existing, ...need])];
    return text.replace(re, `import { ${next.join(", ")} } from "@semio-tech/framework-core";`);
  }
  const stmt = `import { ${need.join(", ")} } from "@semio-tech/framework-core";\n`;
  const firstImport = text.search(/^import\s/m);
  return firstImport >= 0 ? text.slice(0, firstImport) + stmt + text.slice(firstImport) : stmt + text;
}

function keyBase(file) {
  return file.replace(/\\/g, "/").replace(/^.*\/(✏️s|🧰️framework)\//, "$1/").replace(/[^a-zA-Z0-9]+/g, ".").replace(/^\.|\.$/g, "");
}

// 1) fix mangled function-type boxes
for (const file of walk(".")) {
  let text = fs.readFileSync(file, "utf8");
  if (!text.includes('ephemeralBox<()>("') && !text.includes("ephemeralBox<(()>(")) continue;
  const lines = text.split("\n");
  let changed = false;
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (l.includes("surfaceChromeCleanup = ephemeralBox<(()>(")) {
      lines[i] = `const surfaceChromeCleanup = ephemeralBox<(() => void) | null>("s.plugins.animate.apps.present.renderer.react.component.tsx.surfaceChromeCleanup", null);`;
      changed = true;
    } else if (l.includes("_uiDriverProvider = ephemeralBox<(()>(")) {
      lines[i] = `const _uiDriverProvider = ephemeralBox<(() => UiDriver) | null>("framework.modules.ui.elements.core.UiDriver.component.tsx._uiDriverProvider", null);`;
      changed = true;
    }
  }
  if (changed) {
    fs.writeFileSync(file, lines.join("\n"));
    console.log("fixed mangled", file);
  }
}

// 2) convert remaining empty Map/Set/WeakMap at module level from hits3
const hits = JSON.parse(fs.readFileSync(path.join(process.argv[2], "🧪w3-eslint-os-hits3.json"), "utf8"));
const byFile = new Map();
for (const h of hits) {
  if (!byFile.has(h.file)) byFile.set(h.file, []);
  byFile.get(h.file).push(h);
}

for (const [rel, fileHits] of byFile) {
  const file = path.join(process.cwd(), rel);
  let text = fs.readFileSync(file, "utf8");
  const lines = text.split("\n");
  const kb = keyBase(file);
  for (const h of [...fileHits].sort((a, b) => b.line - a.line)) {
    const i = h.line - 1;
    const line = lines[i];
    if (!line) continue;
    const m = line.match(/^(\s*)const\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*new\s+(WeakMap|Map|Set)(<.*>)?\s*\(\s*\)\s*;?\s*(.*)$/);
    if (!m) {
      console.log("NOMATCH", rel + ":" + h.line, line.trim().slice(0, 120));
      continue;
    }
    const [, indent, name, kind, gen, rest] = m;
    const fn = kind === "Set" ? "ephemeralSet" : kind === "WeakMap" ? "ephemeralWeakMap" : "ephemeralMap";
    lines[i] = `${indent}const ${name} = ${fn}${gen || ""}(${JSON.stringify(kb + "." + name)});${rest ? " " + rest : ""}`;
  }
  text = ensureImport(lines.join("\n"), ["ephemeralMap", "ephemeralSet", "ephemeralWeakMap"]);
  fs.writeFileSync(file, text);
  console.log("maps fixed", rel);
}
