import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "fs";
import { join, dirname, resolve, relative } from "path";

const ui = process.env.UI;
const ticket = process.env.TICKET;
const out = [];
const log = (...a) => { const s = a.join(" "); out.push(s); console.log(s); };

function walk(dir, pred, acc = []) {
  if (!existsSync(dir)) return acc;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) {
      if (["node_modules", "target", ".git", "storybook-static"].includes(e.name)) continue;
      walk(p, pred, acc);
    } else if (pred(e.name, p)) acc.push(p);
  }
  return acc;
}

log("=== Cargo.toml under targets (must be none) ===");
const cargoTomls = walk(join(ui, "📦️packages/🦀️rust/🎯️targets"), (n) => n === "Cargo.toml");
log(cargoTomls.length ? cargoTomls.join("\n") : "NONE");

log("\n=== Rust element files ===");
const elDir = join(ui, "트로elements");
// find emoji elements dir
const elActual = readdirSync(ui).find((n) => n.includes("elements") || /elements$/.test(n) || n.endsWith("elements"));
const elementsRoot = join(ui, elActual || "트로elements");
log("elementsRoot", elementsRoot);
const rsEls = walk(elementsRoot, (n) => n.endsWith(".rs"));
for (const p of rsEls.sort()) log(" ", relative(ui, p));

log("\n=== Broken #[path] element/asset refs ===");
const rustRoot = join(ui, "📦️packages/🦀️rust");
const re = /#\[path\s*=\s*"([^"]+)"\]/g;
let miss = 0;
for (const file of walk(rustRoot, (n) => n.endsWith(".rs"))) {
  const text = readFileSync(file, "utf8");
  let m;
  while ((m = re.exec(text))) {
    const rel = m[1];
    if (!rel.includes("elements") && !rel.includes("assets")) continue;
    const resolved = resolve(dirname(file), rel);
    if (!existsSync(resolved)) {
      miss++;
      log("MISSING", relative(ui, file), "->", rel, "=>", resolved);
    }
  }
}
log("missing_count", miss);

log("\n=== Stale non-emoji element path strings in rust ===");
const staleRe = /elements\/([A-Z][A-Za-z0-9]+)\//;
for (const file of walk(rustRoot, (n) => n.endsWith(".rs"))) {
  const text = readFileSync(file, "utf8");
  const lines = text.split("\n");
  lines.forEach((line, i) => {
    const m = line.match(staleRe);
    if (m && !/[\u{1F300}-\u{1FAFF}\u{2600}-\u{27BF}]/u.test(m[0])) {
      // check if the capture has no leading non-ascii
      const before = m[0].slice("elements/".length, m[0].length - 1);
      if (/^[A-Za-z]/.test(before)) log(relative(ui, file) + ":" + (i + 1), line.trim());
    }
  });
}

log("\n=== TS imports to elements without emoji dirs ===");
const tsRoot = join(ui, "📦️packages");
const elNames = readdirSync(elementsRoot).filter((n) => statSync(join(elementsRoot, n)).isDirectory());
const bySuffix = {};
for (const n of elNames) {
  const m = n.match(/([A-Za-z][A-Za-z0-9]*)$/);
  if (m) bySuffix[m[1]] = n;
}
let brokenImports = 0;
const importRe = /from\s+["']([^"']*elements\/[^"']+)["']/g;
for (const file of walk(join(ui, "📦️packages/🟦️typescript"), (n) => /\.(tsx?|jsx?)$/.test(n)).concat(
  walk(elementsRoot, (n) => /\.(tsx?|jsx?)$/.test(n))
)) {
  const text = readFileSync(file, "utf8");
  let m;
  while ((m = importRe.exec(text))) {
    const spec = m[1];
    // resolve relative
    let abs;
    if (spec.startsWith(".")) abs = resolve(dirname(file), spec);
    else continue;
    // try with extensions
    const candidates = [abs, abs + ".ts", abs + ".tsx", join(abs, "index.ts"), join(abs, "index.tsx"),
      abs.replace(/\/$/, "") + "/🟦️component.tsx", abs.replace(/\/$/, "") + "/🟦️component.ts"];
    // Also if path uses bare Element name, check
    const elMatch = spec.match(/elements\/([^/]+)\//);
    if (elMatch) {
      const name = elMatch[1];
      const dir = join(elementsRoot, name);
      if (!existsSync(dir) && bySuffix[name] && bySuffix[name] !== name) {
        brokenImports++;
        log("STALE_EMOJI", relative(ui, file), "uses", name, "should be", bySuffix[name]);
      } else if (!existsSync(dir) && !bySuffix[name]) {
        // might be core subpath
        if (!name.includes("core") && !existsSync(join(elementsRoot, name))) {
          brokenImports++;
          log("UNKNOWN_EL", relative(ui, file), spec);
        }
      }
    }
  }
}
log("stale_emoji_import_count", brokenImports);

log("\n=== W3-interim count under ui ===");
let w3 = 0;
for (const file of walk(ui, () => true)) {
  if (/\.(tsx?|rs|md|json)$/.test(file) && readFileSync(file, "utf8").includes("W3-interim")) {
    w3++;
    log("W3", relative(ui, file));
  }
}
log("w3_files", w3);

writeFileSync(join(ticket, "scratch-finish-audit-out.txt"), out.join("\n"));
