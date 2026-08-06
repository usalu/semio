import { readdirSync, readFileSync, writeFileSync, statSync } from "fs";
import { join, relative, dirname } from "path";

const paths = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const el = paths[1];
const barrel = paths[2];
const symbols = {
  Button: join(el, "Button"),
  ButtonCycle: join(el, "Button"),
  ButtonProps: join(el, "Button"),
  ButtonCycleProps: join(el, "Button"),
  ButtonGroup: join(el, "ButtonGroup"),
  ButtonGroupItem: join(el, "ButtonGroup"),
  buttonGroupItemVariants: join(el, "ButtonGroup"),
  ButtonGroupProps: join(el, "ButtonGroup"),
};

function compFile(dir) {
  return join(dir, readdirSync(dir).find((n) => n.endsWith("component.tsx")));
}
const homeFile = {};
for (const [sym, dir] of Object.entries(symbols)) homeFile[sym] = compFile(dir);

function walk(d, out = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (n.endsWith(".tsx") || n.endsWith(".ts")) out.push(p);
  }
  return out;
}

const interimMarker = "W3-interim";
let rewired = 0;
const report = [];
for (const file of walk(el)) {
  // skip the homes themselves for symbols they define
  let text = readFileSync(file, "utf8");
  if (!text.includes(interimMarker)) continue;
  if (!text.includes("from ") || !/Button(Group|Cycle|Props)?|buttonGroupItemVariants/.test(text)) continue;

  // Parse import lines from barrel with W3-interim
  const lines = text.split("\n");
  let changed = false;
  const newLines = [];
  const pendingDirect = new Map(); // file -> {values:[], types:[]}

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    // multi-line import starting
    if (/import\s*\{/.test(line) && !line.includes("} from") && i + 1 < lines.length) {
      // collect until from
      let block = [line];
      let j = i + 1;
      while (j < lines.length && !/\} from /.test(lines[j])) {
        block.push(lines[j]);
        j++;
      }
      if (j < lines.length) block.push(lines[j]);
      const blockText = block.join("\n");
      if (blockText.includes("index.tsx") && /Button|buttonGroup/.test(blockText)) {
        // check prior lines for interim marker
        const prev = lines.slice(Math.max(0, i - 3), i).join("\n");
        if (prev.includes(interimMarker) || lines[i - 1]?.includes(interimMarker) || true) {
          // rewrite this import block
          const fromMatch = blockText.match(/from\s+"([^"]+)"/);
          if (fromMatch && fromMatch[1].includes("index.tsx")) {
            const inner = blockText.replace(/^[\s\S]*\{/, "").replace(/\}[\s\S]*$/, "");
            const parts = inner.split(",").map((s) => s.trim()).filter(Boolean);
            const stay = [];
            for (const part of parts) {
              const m = part.match(/^(type\s+)?(\w+)/);
              if (!m) { stay.push(part); continue; }
              const isType = !!m[1];
              const name = m[2];
              if (homeFile[name] && !file.startsWith(dirname(homeFile[name]))) {
                // don't import from self
                if (file === homeFile[name]) { stay.push(part); continue; }
                const target = homeFile[name];
                if (dirname(file) === dirname(target) && file !== target) {
                  // same dir odd
                }
                let r = relative(dirname(file), target).replaceAll("\\", "/");
                if (!r.startsWith(".")) r = "./" + r;
                if (!pendingDirect.has(r)) pendingDirect.set(r, { values: [], types: [] });
                const bucket = pendingDirect.get(r);
                if (isType || part.startsWith("type ")) bucket.types.push(name);
                else bucket.values.push(name);
                changed = true;
              } else {
                stay.push(part);
              }
            }
            if (stay.length) {
              // keep interim import with remaining
              const indent = "  ";
              if (block.length > 2) {
                newLines.push(`import {`);
                for (const s of stay) newLines.push(`${indent}${s},`.replace(/,,/, ","));
                // fix trailing commas
                const last = newLines.length - 1;
                newLines[last] = newLines[last].replace(/,$/, "");
                newLines.push(`} from "${fromMatch[1]}";`);
              } else {
                newLines.push(`import { ${stay.join(", ")} } from "${fromMatch[1]}";`);
              }
            } else {
              // drop entire import; also drop preceding interim comment if it only served this import
              if (newLines.length && newLines[newLines.length - 1].includes("W3-interim")) {
                newLines.pop();
              }
            }
            i = j;
            continue;
          }
        }
      }
      newLines.push(...block);
      i = j;
      continue;
    }

    // single-line import
    const single = line.match(/^import\s*\{([^}]+)\}\s*from\s*"([^"]+index\.tsx)";/);
    if (single && /Button|buttonGroup/.test(single[1])) {
      const parts = single[1].split(",").map((s) => s.trim()).filter(Boolean);
      const stay = [];
      for (const part of parts) {
        const m = part.match(/^(type\s+)?(\w+)/);
        if (!m) { stay.push(part); continue; }
        const isType = !!m[1] || part.startsWith("type ");
        const name = m[2];
        if (homeFile[name] && file !== homeFile[name]) {
          let r = relative(dirname(file), homeFile[name]).replaceAll("\\", "/");
          if (!r.startsWith(".")) r = "./" + r;
          if (!pendingDirect.has(r)) pendingDirect.set(r, { values: [], types: [] });
          const bucket = pendingDirect.get(r);
          if (isType) bucket.types.push(name);
          else bucket.values.push(name);
          changed = true;
        } else stay.push(part);
      }
      if (stay.length) {
        newLines.push(`import { ${stay.join(", ")} } from "${single[2]}";`);
      } else if (newLines.length && newLines[newLines.length - 1].includes("W3-interim")) {
        newLines.pop();
      }
      continue;
    }
    newLines.push(line);
  }

  if (!changed) continue;

  // Insert direct imports after adapters region start / after last core import
  const directLines = [];
  for (const [r, bucket] of pendingDirect) {
    const specs = [
      ...[...new Set(bucket.values)],
      ...[...new Set(bucket.types)].map((t) => `type ${t}`),
    ];
    if (!specs.length) continue;
    directLines.push(`import { ${specs.join(", ")} } from "${r}";`);
  }
  // find insertion point: after last import in adapters, before endregion Adapters or first non-import
  let insertAt = -1;
  for (let i = 0; i < newLines.length; i++) {
    if (/^import /.test(newLines[i]) || (/^import \{/.test(newLines[i]))) insertAt = i + 1;
    // handle multi-line already flattened
    if (newLines[i].includes("#endregion") && newLines[i].includes("Adapters") && insertAt < 0) {
      insertAt = i;
      break;
    }
  }
  // Better: after adapters close's preceding imports — insert before #endregion Adapters
  const adaptersEnd = newLines.findIndex((l) => l.includes("#endregion") && l.includes("Adapters"));
  if (adaptersEnd >= 0) {
    // walk back to last import-related line
    let pos = adaptersEnd;
    // find last line that is part of imports before adaptersEnd
    let lastImp = -1;
    for (let i = 0; i < adaptersEnd; i++) {
      if (/^import /.test(newLines[i]) || /^\} from /.test(newLines[i]) || /^\s+(type )?\w+/.test(newLines[i]) && i > 0 && /import/.test(newLines.slice(Math.max(0,i-5), i).join("\n"))) {
        if (/^import /.test(newLines[i]) || /^\} from /.test(newLines[i])) lastImp = i + 1;
      }
      if (/^import /.test(newLines[i])) lastImp = i + 1;
      if (/^\} from /.test(newLines[i])) lastImp = i + 1;
    }
    if (lastImp < 0) lastImp = adaptersEnd;
    newLines.splice(lastImp, 0, ...directLines);
  } else {
    newLines.unshift(...directLines);
  }

  writeFileSync(file, newLines.join("\n"));
  rewired++;
  report.push(`${file}: +${directLines.join(" | ")}`);
}
console.log("rewired files", rewired);
for (const r of report) console.log(r);
