// [DEBUG] WS scratch: the main policyFacetMirrorDriftBreaches run suppresses breaches for
// facets still in POLICY_FACET_MIRROR_DRIFT_ALLOWLIST, so "0 breaches shown for json/png/zip/gif/
// bmp" in ws-scratch-check-shape.ts is AMBIGUOUS — it could mean genuinely clean, or still-drifting
// but allowlist-silenced. This script reimplements just the harvester math (copied verbatim from
// 📜️script.ts's PolicyFacetMirrorDriftReverse region as of this wave) directly against those 5
// artifacts' facet dirs, bypassing the allowlist entirely, to get ground truth. Keep — reusable.
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const STDIO_ARTIFACTS_REL = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";
const FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"];
const SIBLINGS = ["🟦️component.ts", "🔗️component.graphql", "🔣️component.json", "🛰️component.proto"];
const SPOT_ARTIFACTS = ["🔣️json", "📷️png", "🗜️zip", "🎞️gif", "🖼️bmp"];

function snakeToCamel(name: string): string {
  return name.replace(/_([a-z0-9])/g, (_, c: string) => c.toUpperCase());
}
function stripRustCommentsAndStrings(content: string): string {
  // simplified: strip // and /* */ only (sufficient for field-name harvesting purposes here;
  // string contents inside Rust field types are rare/irrelevant to this check)
  return content.replace(/\/\/.*$/gm, "").replace(/\/\*[\s\S]*?\*\//g, "");
}
const RUST_FIELD_RE = /(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&\[<('"]/gm;
const RUST_KEYWORDS = new Set(["self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"]);
function rustFieldNames(content: string): string[] {
  const stripped = stripRustCommentsAndStrings(content);
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  RUST_FIELD_RE.lastIndex = 0;
  while ((m = RUST_FIELD_RE.exec(stripped))) {
    const n = m[1]!;
    if (RUST_KEYWORDS.has(n) || n.startsWith("r#")) continue;
    names.add(n);
  }
  return [...names].map(snakeToCamel).filter(Boolean);
}
const SERDE_TAG_RE = /#\[serde\([^\]]*\btag\s*=\s*"([a-zA-Z_][a-zA-Z0-9_]*)"/g;
function rustTagFieldNames(content: string): string[] {
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  SERDE_TAG_RE.lastIndex = 0;
  while ((m = SERDE_TAG_RE.exec(content))) names.add(m[1]!);
  return [...names];
}
const VARIANT_RE = /^\s*([A-Z][A-Za-z0-9]*)\s*[{(]/gm;
function rustVariantFieldNames(content: string): string[] {
  const stripped = stripRustCommentsAndStrings(content);
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  VARIANT_RE.lastIndex = 0;
  while ((m = VARIANT_RE.exec(stripped))) {
    const n = m[1]!;
    names.add(n.charAt(0).toLowerCase() + n.slice(1));
  }
  return [...names];
}

const TS_RE = /(?:^|[\s{;(])([A-Za-z_][A-Za-z0-9_]*)\??\s*:\s*[^;\n=]+[;\n]/gm;
const TS_KEYWORDS = new Set([
  "interface", "type", "export", "import", "from", "extends", "implements", "class", "function", "const", "let", "var",
  "namespace", "module", "declare", "readonly", "public", "private", "protected", "static", "abstract", "new", "this",
  "super", "typeof", "keyof", "case", "default", "switch", "try", "catch", "finally", "throw", "async", "await", "yield",
  "get", "set", "constructor", "void", "never", "unknown", "any", "undefined", "null", "true", "false", "satisfies",
  "asserts", "is", "infer", "of", "in", "as", "if", "else", "for", "while", "return",
]);
function stripTsCommentsAndStrings(content: string): string {
  const out: string[] = [];
  let i = 0;
  while (i < content.length) {
    const ch = content[i]!;
    const next = content[i + 1];
    if (ch === "/" && next === "/") {
      while (i < content.length && content[i] !== "\n") { out.push(" "); i++; }
      continue;
    }
    if (ch === "/" && next === "*") {
      i += 2;
      while (i < content.length && !(content[i] === "*" && content[i + 1] === "/")) { out.push(" "); i++; }
      if (i < content.length) i += 2;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      const quote = ch;
      out.push(" "); i++;
      while (i < content.length) {
        if (content[i] === "\\") { out.push(" "); out.push(" "); i += 2; continue; }
        if (content[i] === quote) { out.push(" "); i++; break; }
        out.push(" "); i++;
      }
      continue;
    }
    out.push(ch); i++;
  }
  return out.join("");
}
function tsFieldNames(content: string): string[] {
  const stripped = stripTsCommentsAndStrings(content);
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  TS_RE.lastIndex = 0;
  while ((m = TS_RE.exec(stripped))) {
    const n = m[1]!;
    if (TS_KEYWORDS.has(n)) continue;
    names.add(n);
  }
  return [...names].map(snakeToCamel).filter(Boolean);
}

const GRAPHQL_RE = /^\s*([a-z][A-Za-z0-9_]*)\s*:/gm;
function graphqlFieldNames(content: string): string[] {
  const noComments = content.replace(/#.*$/gm, "");
  const noArgs = noComments.replace(/\([^)]*\)/g, "");
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  GRAPHQL_RE.lastIndex = 0;
  while ((m = GRAPHQL_RE.exec(noArgs))) names.add(m[1]!);
  return [...names].map(snakeToCamel).filter(Boolean);
}

function jsonFieldNames(content: string): string[] {
  const root: unknown = JSON.parse(content);
  const names = new Set<string>();
  const visit = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    if (Array.isArray(node)) { for (const item of node) visit(item); return; }
    const obj = node as Record<string, unknown>;
    if (obj.properties && typeof obj.properties === "object" && !Array.isArray(obj.properties)) {
      for (const key of Object.keys(obj.properties as Record<string, unknown>)) names.add(key);
    }
    for (const val of Object.values(obj)) visit(val);
  };
  visit(root);
  return [...names].map(snakeToCamel).filter(Boolean);
}

const PROTO_RE = /^\s*(?:optional\s+|repeated\s+)?(?:map\s*<[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/gm;
const PROTO_SKIP_RE = /^\s*(enum|oneof|message|package|syntax|import)\b/;
function protoFieldNames(content: string): string[] {
  const bodyLines = content.split(/\r?\n/).filter((line) => !PROTO_SKIP_RE.test(line));
  const body = bodyLines.join("\n");
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  PROTO_RE.lastIndex = 0;
  while ((m = PROTO_RE.exec(body))) names.add(m[1]!);
  return [...names].map(snakeToCamel).filter(Boolean);
}

function siblingFieldNames(sib: string, content: string): string[] {
  if (sib.endsWith(".ts")) return tsFieldNames(content);
  if (sib.endsWith(".graphql")) return graphqlFieldNames(content);
  if (sib.endsWith(".json")) return jsonFieldNames(content);
  if (sib.endsWith(".proto")) return protoFieldNames(content);
  return [];
}

const SUBSTRING_MIN_LEN = 4;

// find each spot artifact's standard/subset dirs housing a snapshot schema dir
function findSchemaOwningSubsetDirs(artifactDir: string): string[] {
  const out: string[] = [];
  const standardsDir = join(artifactDir, "🏅️standards");
  if (!existsSync(standardsDir)) return out;
  for (const std of readdirSync(standardsDir, { withFileTypes: true })) {
    if (!std.isDirectory()) continue;
    const subsetsDir = join(standardsDir, std.name, "🪆️subsets");
    if (!existsSync(subsetsDir)) continue;
    for (const sub of readdirSync(subsetsDir, { withFileTypes: true })) {
      if (!sub.isDirectory()) continue;
      const subsetDir = join(subsetsDir, sub.name);
      if (existsSync(join(subsetDir, "🧬️schema", "📸️snapshot"))) out.push(subsetDir);
    }
  }
  return out;
}

let totalChecked = 0;
let totalClean = 0;
for (const artifact of SPOT_ARTIFACTS) {
  const artifactDir = join(repoRoot, STDIO_ARTIFACTS_REL, artifact);
  const subsetDirs = findSchemaOwningSubsetDirs(artifactDir);
  console.log(`\n=== ${artifact} (${subsetDirs.length} schema-owning subset dir(s)) ===`);
  for (const subsetDir of subsetDirs) {
    for (const facet of FACETS) {
      const facetDir = join(subsetDir, "🧬️schema", facet);
      const rustPath = join(facetDir, "🦀️component.rs");
      if (!existsSync(rustPath)) continue;
      totalChecked++;
      const rustContent = readFileSync(rustPath, "utf8");
      const camelFields = rustFieldNames(rustContent);
      const compareFields = [...camelFields, ...rustTagFieldNames(rustContent), ...rustVariantFieldNames(rustContent)];
      const lines: string[] = [];
      for (const sib of SIBLINGS) {
        const sibPath = join(facetDir, sib);
        if (!existsSync(sibPath)) { lines.push(`${sib}:MISSING_FILE`); continue; }
        const sibContent = readFileSync(sibPath, "utf8");
        const missingFields = camelFields.filter((f) => !sibContent.includes(f));
        if (missingFields.length > 0) lines.push(`${sib}:missing:${missingFields.length} [${missingFields.join(",")}]`);
        let fields: string[];
        try {
          fields = siblingFieldNames(sib, sibContent);
        } catch (e) {
          lines.push(`${sib}:PARSE_ERROR:${(e as Error).message}`);
          continue;
        }
        const extraFields = fields.filter((f) => {
          if (f === "schema" || compareFields.includes(f)) return false;
          const fLower = f.toLowerCase();
          return !compareFields.some((cf) => cf.length >= SUBSTRING_MIN_LEN && fLower.includes(cf.toLowerCase()));
        });
        if (extraFields.length > 0) lines.push(`${sib}:extra:${extraFields.length} [${extraFields.join(",")}]`);
      }
      const relFacet = facetDir.slice(repoRoot.length + 1);
      if (lines.length === 0) {
        totalClean++;
        console.log(`  CLEAN  ${relFacet}`);
      } else {
        console.log(`  DRIFT  ${relFacet}`);
        for (const l of lines) console.log(`           ${l}`);
      }
    }
  }
}
console.log(`\nTOTAL facets checked: ${totalChecked}, clean (both directions): ${totalClean}`);
