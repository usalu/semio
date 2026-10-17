// 🧲️ For crates whose lib closure holds a use of the shared fixture reader: is every use inside a cfg(test) item (dev-dependency), behind any(test, feature) (optional dependency) or ungated (normal dependency)?
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rustTokenPairs, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const F = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const G = join(F, "🗑️generated");
const owners = JSON.parse(readFileSync(join(G, "frd-owners.json"), "utf8")) as Record<string, { manifest: string; crate: string; cls: string }[]>;
const MACROS = new Set(["fixture_text", "fixture_bytes", "fixture_path", "manifest_dir", "repo_root", "repo_text", "repo_bytes"]);

type Gate = "test" | "testkit" | "ungated";
function gateOfSites(text: string): { gates: Gate[]; sites: number } {
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const open: number[] = [];
  const enclosing: number[][] = new Array(tokens.length);
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text === "}") open.pop();
    enclosing[i] = [...open];
    if (tokens[i]!.text === "{") open.push(i);
  }
  const attrsBefore = (bodyOpen: number): string[] => {
    let head = bodyOpen - 1;
    while (head >= 0 && !(["mod", "fn", "impl", "const", "static", "trait", "struct", "enum"].includes(tokens[head]!.text) && tokens[head - 1]?.text !== "'")) { head -= 1; if (bodyOpen - head > 200) return []; }
    let start = head;
    while (start > 0 && ["pub", "unsafe", "async", "extern", "const"].includes(tokens[start - 1]!.text)) start -= 1;
    while (tokens[start - 1]?.text === ")" ) { const o = pairs.get(start - 1); if (o === undefined) break; start = o; if (tokens[start - 1]?.text === "pub") start -= 1; }
    const attrs: string[] = [];
    let k = start - 1;
    while (k >= 0 && tokens[k]!.text === "]") {
      const o = pairs.get(k);
      if (o === undefined || tokens[o - 1]?.text !== "#") break;
      attrs.push(tokens.slice(o + 1, k).map(t => t.text).join(" "));
      k = o - 2;
    }
    return attrs;
  };
  const gates: Gate[] = [];
  let sites = 0;
  for (let i = 0; i + 2 < tokens.length; i += 1) {
    if (!(tokens[i]!.kind === "identifier" && MACROS.has(tokens[i]!.text) && tokens[i + 1]!.text === "!" && tokens[i + 2]!.text === "(")) continue;
    sites += 1;
    let gate: Gate = "ungated";
    for (const brace of [...enclosing[i]!].reverse()) {
      const attrs = attrsBefore(brace).filter(a => a.startsWith("cfg"));
      if (attrs.some(a => /^cfg \( test \)$/u.test(a) || /^cfg \( all \( test\b/u.test(a))) { gate = "test"; break; }
      if (attrs.some(a => /^cfg \( any \( test , feature = "testkit" \) \)$/u.test(a) || /^cfg \( any \( feature = "testkit" , test \) \)$/u.test(a))) { gate = "testkit"; break; }
    }
    gates.push(gate);
  }
  return { gates, sites };
}

const rowsOut: { manifest: string; role: string; file: string; cls: string; gate: string }[] = [];
const roles = new Map<string, string>();
for (const [file, rows] of Object.entries(owners)) {
  const abs = join(ROOT, file);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  if (!text.includes("semio_framework_test_fixtures")) continue;
  const { gates } = gateOfSites(text);
  const gate = gates.every(g => g === "test") ? "test" : gates.every(g => g !== "ungated") ? "testkit" : "ungated";
  for (const row of rows) {
    if (!roles.has(row.manifest)) { try { roles.set(row.manifest, ((Bun.TOML.parse(readFileSync(join(ROOT, row.manifest), "utf8")) as any).package?.metadata?.semio?.role) ?? ""); } catch { roles.set(row.manifest, "?"); } }
    rowsOut.push({ manifest: row.manifest, role: roles.get(row.manifest)!, file, cls: row.cls, gate });
  }
}
writeFileSync(join(G, "frd-gating.json"), JSON.stringify(rowsOut, null, 1));
const agg = new Map<string, Set<string>>();
for (const r of rowsOut) { const k = r.manifest; const set = agg.get(k) ?? new Set<string>(); set.add(`${r.cls}:${r.gate}`); agg.set(k, set); }
const kinds: Record<string, number> = {};
for (const set of agg.values()) { const key = [...set].sort().join("+"); kinds[key] = (kinds[key] ?? 0) + 1; }
console.log(JSON.stringify(kinds));
