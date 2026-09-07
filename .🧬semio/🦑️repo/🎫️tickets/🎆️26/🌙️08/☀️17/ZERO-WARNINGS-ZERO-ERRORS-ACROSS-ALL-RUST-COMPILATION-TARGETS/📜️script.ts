import { readFileSync, writeFileSync, appendFileSync, existsSync, readdirSync } from "node:fs";
import { join, resolve, relative, dirname, basename } from "node:path";

const [command, name, filter] = process.argv.slice(2);
if (command === "scope-test") {
  const { rustWarningTargetScope } = await import(resolve(process.cwd(), "📜️script.ts"));
  const fixture = await Bun.file("🧪️tests/🦀️rust-warnings/🔣️.json").json();
  for (const row of fixture.cases) {
    const scope = rustWarningTargetScope(process.cwd(), row.target);
    for (const pkg of row.requiredPackages) if (!scope.packages.includes(pkg)) throw new Error(row.target + " misses " + pkg);
  }
  console.log("Warning scope fixtures passed");
  process.exit(0);
}

if (["paths", "apply-paths"].includes(command)) {
  const files = Bun.spawnSync(["rg", "--files", "✏️s/🔌️plugins", "-g", "*.rs"]).stdout.toString().trim().split("\n").filter(file => file.endsWith("📦️packages/🦀️rust/🦀️.rs"));
  const changed: string[] = [];
  for (const file of files) {
    const source = readFileSync(file, "utf8");
    let next = source;
    for (const match of source.matchAll(/#\[path = "([^"]+)"\]/g)) {
      const oldPath = match[1];
      if (!oldPath.includes("🧪️tests/") || existsSync(resolve(dirname(file), oldPath))) continue;
      const missing = resolve(dirname(file), oldPath);
      const testDirectory = dirname(missing);
      const parent = dirname(testDirectory);
      if (!existsSync(parent)) { console.log(`UNRESOLVED ${missing}`); continue; }
      const stem = basename(testDirectory);
      const candidates = readdirSync(parent).filter(candidate => {
        const target = join(parent, candidate, basename(missing));
        return existsSync(target) && readFileSync(target, "utf8").slice(0, 1_000).includes(`\`${stem}\``);
      });
      if (candidates.length !== 1) { console.log(`UNRESOLVED ${missing} candidates=${candidates.join(",")}`); continue; }
      const target = join(parent, candidates[0], basename(missing));
      const newPath = relative(dirname(file), target);
      console.log(`${file}: ${oldPath} → ${newPath}`);
      next = next.replaceAll(`#[path = "${oldPath}"]`, `#[path = "${newPath}"]`);
    }
    if (command === "apply-paths" && next !== source) { writeFileSync(file, next); changed.push(file); }
  }
  if (changed.length) appendFileSync(join(import.meta.dir, "📓️2026-09-07-verification.md"), `\n### Test Module Paths Repaired\n\n${changed.map(file => `- \`${file}\``).join("\n")}\n`);
  process.exit(0);
}
if (!["diagnostics", "unique", "suggestions", "apply"].includes(command) || !name || name.includes("/") || name.includes("\\")) throw new Error("Expected diagnostics|unique|suggestions|apply <log-name> [filter]");
const messages = readFileSync(join(import.meta.dir, "🗑️generated", `${name}.jsonl`), "utf8").split("\n").flatMap(line => {
  try {
    const row = JSON.parse(line);
    return row.reason === "compiler-message" && ["warning", "error"].includes(row.message.level) ? [row] : [];
  } catch { return []; }
});
if (["suggestions", "apply"].includes(command)) {
  const byFile = new Map<string, Map<string, { start: number; end: number; replacement: string; original: string }>>();
  const allowed = new Set(["unused_qualifications", "non_shorthand_field_patterns", "deprecated", "unused_imports", "unused_braces", "unused_mut"]);
  for (const row of messages) {
    if (!allowed.has(row.message.code?.code) || (filter && !JSON.stringify(row).includes(filter))) continue;
    for (const child of row.message.children) {
      if (!child.spans.length || child.spans.some(span => span.suggestion_applicability !== "MachineApplicable" || span.suggested_replacement === null)) continue;
      for (const span of child.spans) {
        if (row.message.code.code === "deprecated" && span.suggested_replacement !== "try_update") continue;
        const file = resolve(span.file_name);
        if (!file.startsWith(process.cwd() + "/") || !file.endsWith(".rs")) continue;
        const content = readFileSync(file);
        if ((file.includes("generated") || /@generated|GENERATED-BY/.test(content.toString().slice(0, 1_000))) && !content.toString().slice(0, 100).includes("Hand-written")) continue;
        const lines = content.toString().split("\n");
        if (span.text.some((line, index) => lines[span.line_start - 1 + index] !== line.text)) continue;
        if (row.message.code.code === "unused_imports") {
          if (span.text.some(line => line.text.includes("pub use"))) continue;
          const names = [...row.message.message.matchAll(/`([^`]+)`/g)].map(match => match[1].split("::").at(-1));
          const rest = lines.filter((_, index) => index < span.line_start - 1 || index >= span.line_end).join("\n");
          const unconditionalPluginLeaf = file.includes("✏️s/🔌️plugins/") && !/#\!?\[cfg|\bmod\s+[a-zA-Z_]\w*\s*[;{]/.test(content.toString());
          if (!unconditionalPluginLeaf && (!names.length || names.some(name => !/^[A-Za-z_][A-Za-z_0-9]*$/.test(name) || new RegExp(`\\b${name}\\b`).test(rest)))) continue;
        }
        const original = span.text.map(line => Array.from(line.text).slice(line.highlight_start - 1, line.highlight_end - 1).join("")).join("\n");
        if (content.subarray(span.byte_start, span.byte_end).toString() !== original) continue;
        const changes = byFile.get(file) ?? new Map();
        changes.set(`${span.byte_start}:${span.byte_end}`, { start: span.byte_start, end: span.byte_end, replacement: span.suggested_replacement, original });
        byFile.set(file, changes);
      }
    }
  }
  const changed: string[] = [];
  for (const [file, changes] of byFile) {
    let content = readFileSync(file);
    let boundary = content.length;
    for (const edit of [...changes.values()].sort((a, b) => b.start - a.start)) {
      if (edit.end > boundary) continue;
      console.log(`${relative(process.cwd(), file)}: ${edit.original} → ${edit.replacement}`);
      content = Buffer.concat([content.subarray(0, edit.start), Buffer.from(edit.replacement), content.subarray(edit.end)]);
      boundary = edit.start;
    }
    if (command === "apply") { writeFileSync(file, content); changed.push(relative(process.cwd(), file)); }
  }
  if (changed.length) appendFileSync(join(import.meta.dir, "📓️2026-09-07-verification.md"), `\n### Compiler Suggestions Applied from ${name}\n\n${changed.map(file => `- \`${file}\``).join("\n")}\n`);
  process.exit(0);
}
if (command === "unique") {
  const seen = new Set<string>();
  for (const row of messages) {
    if (filter && !JSON.stringify(row).includes(filter)) continue;
    const span = row.message.spans.find(span => span.is_primary);
    const key = `${row.message.code?.code}: ${row.message.message} ${span?.file_name}:${span?.line_start}`;
    if (!seen.has(key)) console.log(key);
    seen.add(key);
  }
  process.exit(0);
}
const counts = new Map<string, number>();
for (const row of messages) {
  const key = `${row.message.level} ${row.package_id.split("#").at(-1)} ${row.message.code?.code ?? ""}`;
  counts.set(key, (counts.get(key) ?? 0) + 1);
}
console.log([...counts].sort((a, b) => b[1] - a[1]).map(([key, count]) => `${count} ${key}`).join("\n"));
if (filter) for (const row of messages.filter(row => JSON.stringify(row).includes(filter))) console.log(row.message.rendered);
console.log(`[DEBUG] ${messages.filter(row => row.message.level === "warning").length} warnings; ${messages.filter(row => row.message.level === "error").length} errors`);
