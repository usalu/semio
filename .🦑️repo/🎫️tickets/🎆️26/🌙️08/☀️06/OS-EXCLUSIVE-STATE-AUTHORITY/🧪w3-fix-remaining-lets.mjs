import fs from "fs";
import path from "path";

const ticket = process.argv[2];
const hits = JSON.parse(fs.readFileSync(path.join(ticket, "🧪w3-eslint-os-hits2.json"), "utf8"));

function ensureImport(text) {
  if (!/\bephemeralBox\s*[<(]/.test(text) && !/\bephemeralMap\s*[<(]/.test(text) && !/\bephemeralSet\s*[<(]/.test(text)) return text;
  const need = [];
  if (/\bephemeralBox\s*[<(]/.test(text)) need.push("ephemeralBox");
  if (/\bephemeralMap\s*[<(]/.test(text)) need.push("ephemeralMap");
  if (/\bephemeralSet\s*[<(]/.test(text)) need.push("ephemeralSet");
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

const byFile = new Map();
for (const h of hits) {
  if (!h.msg.includes("module-level `let`") && !h.msg.includes("empty new Map") && !h.msg.includes("new Map()/Set()")) continue;
  if (!byFile.has(h.file)) byFile.set(h.file, []);
  byFile.get(h.file).push(h);
}

for (const [file, fileHits] of byFile) {
  let text = fs.readFileSync(file, "utf8");
  const lines = text.split("\n");
  const letNames = [];
  const kb = keyBase(file);

  for (const h of [...fileHits].sort((a, b) => b.line - a.line)) {
    const i = h.line - 1;
    let line = lines[i];
    if (!line) continue;

    // empty map/set
    let m = line.match(/^(\s*)const\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?::\s*[^=]+)?\s*=\s*new\s+(Map|Set)\s*(<[^>]*>)?\s*\(\s*\)\s*;?\s*(.*)$/);
    if (m) {
      const [, indent, name, kind, gen, rest] = m;
      const fn = kind === "Map" ? "ephemeralMap" : "ephemeralSet";
      lines[i] = `${indent}const ${name} = ${fn}${gen || ""}(${JSON.stringify(kb + "." + name)});${rest ? " " + rest : ""}`;
      continue;
    }

    // let name: Type;  or let name: Type = init;
    m = line.match(/^(\s*)let\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+)$/);
    if (m) {
      const [, indent, name, rest] = m;
      // rest may be `Type;` or `Type = init;` or start of multiline type
      if (rest.includes("{") && !rest.includes("}")) {
        // multiline type — collect until semicolon line
        let j = i;
        let block = line;
        while (j + 1 < lines.length && !/;\s*$/.test(lines[j])) {
          j++;
          block += "\n" + lines[j];
        }
        const bm = block.match(/^(\s*)let\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([\s\S]+?)\s*(?:=\s*([\s\S]+?))?\s*;\s*$/);
        if (!bm) continue;
        const type = bm[3].trim();
        const init = bm[4] !== undefined ? bm[4].trim() : "undefined";
        letNames.push(name);
        const repl = `${indent}const ${name} = ephemeralBox<${type}>(${JSON.stringify(kb + "." + name)}, ${init});`;
        lines.splice(i, j - i + 1, repl);
        continue;
      }
      const eq = rest.indexOf("=");
      let type, init;
      if (eq >= 0) {
        type = rest.slice(0, eq).replace(/;?\s*$/, "").trim();
        init = rest.slice(eq + 1).replace(/;?\s*$/, "").trim();
      } else {
        type = rest.replace(/;?\s*$/, "").trim();
        init = "undefined";
      }
      // skip function types with => that still have = for default — already handled if simple
      letNames.push(name);
      lines[i] = `${indent}const ${name} = ephemeralBox<${type}>(${JSON.stringify(kb + "." + name)}, ${init});`;
    }
  }

  text = lines.join("\n");
  for (const name of letNames) {
    const declRe = new RegExp(`const ${name} = ephemeralBox`);
    text = text
      .split("\n")
      .map((l) => {
        if (declRe.test(l) || /^\s*import\b/.test(l)) return l;
        return l.replace(new RegExp(`\\b${name}\\b`, "g"), `${name}.current`).replaceAll(`${name}.current.current`, `${name}.current`);
      })
      .join("\n");
  }
  text = ensureImport(text);
  fs.writeFileSync(file, text);
  console.log("updated", file.replace(/.*semio\//, ""), "lets", letNames);
}
