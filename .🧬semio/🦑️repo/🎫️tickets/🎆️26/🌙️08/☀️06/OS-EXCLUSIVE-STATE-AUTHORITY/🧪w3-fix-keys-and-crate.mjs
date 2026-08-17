import fs from "fs";
import path from "path";

function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (["node_modules", "target", "dist", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    try {
      if (e.isSymbolicLink()) continue;
    } catch {
      continue;
    }
    if (e.isDirectory()) walk(p, out);
    else if (/\.(ts|tsx)$/.test(e.name)) out.push(p);
  }
  return out;
}

let keyFixes = 0;
for (const file of walk(".")) {
  let text = fs.readFileSync(file, "utf8");
  if (!text.includes("ephemeral")) continue;
  const next = text.replace(/(ephemeral(?:Box|Map|Set|WeakMap)<[^;]*?\(\s*")([^"]+?)\.current(")/g, (_, a, key, c) => {
    keyFixes++;
    return `${a}${key}${c}`;
  }).replace(/(ephemeral(?:Box|Map|Set|WeakMap)\(\s*")([^"]+?)\.current(")/g, (_, a, key, c) => {
    keyFixes++;
    return `${a}${key}${c}`;
  });
  // also keys where name.current appears mid-key after the var name only at end before quote
  const next2 = next.replace(/("[\w.-]+)\.current("\s*,)/g, (m, a, b) => {
    if (m.includes("ephemeral") || true) {
      keyFixes++;
      return `${a}${b}`;
    }
    return m;
  });
  // careful - the above is too broad. Only inside ephemeral calls:
  // revert next2 broad replace - use more precise
}
// redo carefully
keyFixes = 0;
for (const file of walk(".")) {
  let text = fs.readFileSync(file, "utf8");
  if (!/ephemeral(?:Box|Map|Set|WeakMap)/.test(text)) continue;
  const next = text.replace(/ephemeral(Box|Map|Set|WeakMap)(<[^>]*>)?\(\s*"([^"]+)\.current"/g, (m, kind, gen, key) => {
    keyFixes++;
    return `ephemeral${kind}${gen || ""}("${key}"`;
  });
  if (next !== text) {
    fs.writeFileSync(file, next);
    console.log("keys", file);
  }
}
console.log("keyFixes", keyFixes);

// fix cachedCrateIndex block specifically
for (const file of walk(".")) {
  let text = fs.readFileSync(file, "utf8");
  if (!text.includes("cachedCrateIndex = ephemeralBox")) continue;
  const broken = `const cachedCrateIndex = ephemeralBox<{
  exactPkgNames: Set<string>>("framework.products.repo.modules.lib.packages.typescript.index.ts.cachedCrateIndex.current", undefined);
  libNameToCrates: Map<string, CrateIndexRecord[]>;
  aliasToCrates: Map<string, CrateIndexRecord[]>;
} | null = null;`;
  const broken2 = broken.replace(".current", ""); // after key fix may differ
  const good = `const cachedCrateIndex = ephemeralBox<{
  exactPkgNames: Set<string>;
  libNameToCrates: Map<string, CrateIndexRecord[]>;
  aliasToCrates: Map<string, CrateIndexRecord[]>;
} | null>("framework.products.repo.modules.lib.packages.typescript.index.ts.cachedCrateIndex", null);`;
  if (text.includes("exactPkgNames: Set<string>>(") || text.includes("exactPkgNames: Set<string>>(\"")) {
    text = text.replace(/const cachedCrateIndex = ephemeralBox<\{[\s\S]*?\} \| null = null;/, good);
    // try alternate if key already fixed
    if (text.includes("exactPkgNames: Set<string>>(")) {
      text = text.replace(/const cachedCrateIndex = ephemeralBox<\{[\s\S]*?\} \| null>;/, good);
    }
    fs.writeFileSync(file, text);
    console.log("crate fixed", file);
    console.log(text.slice(text.indexOf("const cachedCrateIndex"), text.indexOf("const cachedCrateIndex") + 320));
  }
}
