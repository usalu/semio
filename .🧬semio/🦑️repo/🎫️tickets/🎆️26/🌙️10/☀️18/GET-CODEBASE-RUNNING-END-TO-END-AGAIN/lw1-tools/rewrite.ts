import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { scan } from "./scan.ts";

const ROOT = "/Users/ueli/Documents/semio";
const SDK = join(ROOT, "🧰️framework/📦️packages/🟦️typescript");
const LIB = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library") + "/";
type Dest = { file: string; key: string };
const D = {
  EXEC: { file: "🏃️process/🎛️execution/🟦️.ts", key: "process/execution" },
  VIT: { file: "🏃️process/🧪️testing/🧪️vitest/🟦️.ts", key: "process/vitest" },
  TEXEC: { file: "🏃️process/🧪️testing/🎛️execution/🟦️.ts", key: "process/test-execution" },
  CARGO: { file: "🏃️process/🧪️testing/🦀️cargo/🟦️.ts", key: "process/cargo-test" },
  TSC: { file: "🏃️process/🏗️build/🟦️typescript/🟦️.ts", key: "process/typescript-check" },
  NATIVE: { file: "🏃️process/📦️artifacts/🏗️native-build/🟦️.ts", key: "process/native-build" },
  ROUTING: { file: "🏃️process/🧭️routing/🟦️.ts", key: "process/routing" },
  MUT: { file: "🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts", key: "test/mutation/inventory/execution" },
  MODTREE: { file: "📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts", key: "compiler/syntax/module-tree" },
  ADAPTER: { file: "🧪️test/🔌️adapter/🟦️.ts", key: "test/adapter" },
  CAPACITY: { file: "🏃️process/🏗️build/🎟️capacity/🟦️.ts", key: "process/compiler-capacity" },
  WASM: { file: "🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts", key: "process/wasm-build" },
  RBT: { file: "🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts", key: "ui/targets/react/build-tooling" },
} satisfies Record<string, Dest>;
type Rule = { all?: Dest; names?: Record<string, { dest?: Dest; as?: string; drop?: boolean }> };
const rules: Record<string, Rule> = {
  "🏃️process/🟦️.ts": { all: D.EXEC },
  "🏃️process/🦀️cargo/🏃️run/🟦️.ts": { all: D.EXEC },
  "🏃️process/🍞️bun/🟦️.ts": { all: D.EXEC },
  "🏃️process/🔍️probe/🟦️.ts": { all: D.EXEC },
  "🏃️process/🌿️environment/🟦️.ts": { all: D.EXEC },
  "⚡️caching/🟦️.ts": { all: D.EXEC },
  "⚡️caching/🦀️cargo/🟦️.ts": { all: D.EXEC },
  "🗂️workspaces/🧬️metadata/🟦️.ts": { all: D.EXEC },
  "🏃️process/🌿️environment/🧪️test-output/🟦️.ts": { all: D.EXEC },
  "🏃️process/🎛️owned-execution/🟦️.ts": { all: D.EXEC },
  "🏃️process/🧭️routing/🧩️contributions/🟦️.ts": { all: D.EXEC },
  "🏃️process/🦀️cargo/🏭️inventory/🟦️.ts": { all: D.MUT },
  "🧪️testing/🧪️vitest/🟦️.ts": { all: D.VIT },
  "🧪️testing/🎛️repository-command/🟦️.ts": { all: D.TEXEC },
  "⚡️caching/📦️artifacts/🦀️rust/🟦️.ts": { all: D.CARGO },
  "⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts": { all: D.TSC },
  "🏃️process/🦀️cargo/🎟️admission/🟦️.ts": { names: { runRepositoryCargoTests: { dest: D.CARGO, as: "runAdmittedCargoPackageTestsV1" }, runRepositoryCargoCommandV1: { dest: D.CARGO, as: "runAdmittedCargoCommandV1" } } },
  "🏃️process/🦀️cargo/🎯️exact/🟦️.ts": { names: { runRepositoryExactCargoLaws: { dest: D.CARGO, as: "runAdmittedExactCargoLawsV1" }, configuredRepositoryCargoPreparationPolicyV1: { drop: true } } },
  "🏃️process/🕸️wasm/🔣️policy/🟦️.ts": { names: { buildRepositoryWasmWebV1: { dest: D.WASM, as: "buildAdmittedWasmWebV1" } } },
  "🗂️workspaces/🟦️.ts": { names: { getWorkspaceRoot: { dest: D.ROUTING, as: "findWorkspaceRoot" } } },
  "🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts": { all: D.NATIVE },
  "🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts": { all: D.NATIVE },
  "🗂️workspaces/🦀️cargo/🧩️capabilities/🧪️tests/🟦️.ts": { all: D.NATIVE },
  "🗂️workspaces/🦀️cargo/🟦️.ts": { names: { cargoRepositoryPackages: { dest: D.NATIVE }, cargoWorkspaceForManifest: { dest: D.NATIVE }, prepareCargoWorkspaceInvocation: { dest: D.NATIVE } } },
  "⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts": { names: { repositoryCargoPreparationStorageV1: { drop: true } } },
  "PROC:routing": { all: D.ROUTING },
  "PROC:capacity": { all: D.CAPACITY },
  "COMPILER:module-tree": { all: D.MODTREE },
  "UI:build-tooling": { all: D.RBT },
  "TEST:adapter-helpers": { names: { committedArtifact: { dest: D.ADAPTER }, currentPlatform: { dest: D.ADAPTER } } },
};

function keyOf(spec: string, target: string): string | undefined {
  if (spec === "@semio-tech/framework-compiler-ts/syntax/module-tree" || target.endsWith("📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts")) return "COMPILER:module-tree";
  if (spec === "@semio-tech/process/compiler-capacity" || target.endsWith("🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts")) return "PROC:capacity";
  if (spec === "@semio-tech/process/routing" || target.endsWith("🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")) return "PROC:routing";
  if (target.endsWith("🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts")) return "UI:build-tooling";
  if (target.endsWith("🧪️test/📦️packages/🟦️typescript/🟦️.ts")) return "TEST:adapter-helpers";
  const abs = resolve(ROOT, target);
  return abs.startsWith(LIB) ? abs.slice(LIB.length) : undefined;
}

export type Report = { file: string; unmapped: string[]; changed: boolean };
export function rewriteFile(file: string, edgesOfFile: { spec: string; target: string }[], write: boolean): Report {
  const abs = join(ROOT, file);
  const text = readFileSync(abs, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  const bySpec = new Map(edgesOfFile.map(e => [e.spec, e.target]));
  const script = basename(file) === "📜️script.ts" || /🧪️tests\/🎚️config\/🟦️\.ts$/u.test(file);
  const specifierFor = (dest: Dest): string => {
    if (!script) return `@semio-tech/framework/${dest.key}`;
    const rel = relative(dirname(abs), join(SDK, dest.file)).split("\\").join("/");
    return rel.startsWith(".") ? rel : "./" + rel;
  };
  const existingNames = new Set<string>();
  for (const st of sf.statements) if (ts.isImportDeclaration(st) && st.importClause) {
    const c = st.importClause;
    if (c.name) existingNames.add(c.name.text);
    if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) existingNames.add(c.namedBindings.name.text); else for (const el of c.namedBindings.elements) existingNames.add(el.name.text); }
  }
  const edits: { start: number; end: number; text: string }[] = [];
  const renames = new Map<string, string>();
  const unmapped: string[] = [];
  const dropArgs: string[] = [];
  const emitted = new Set<string>();
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier))) continue;
    const spec = st.moduleSpecifier.text;
    const target = bySpec.get(spec);
    if (target === undefined) continue;
    const key = keyOf(spec, target);
    const rule = key ? rules[key] : undefined;
    if (!rule) { unmapped.push(`${spec} (${key ?? "no key"})`); continue; }
    const clause = st.importClause;
    if (!clause || clause.name || !clause.namedBindings || ts.isNamespaceImport(clause.namedBindings)) { unmapped.push(`${spec} (non-named import)`); continue; }
    const groups = new Map<string, { dest: Dest; items: string[] }>();
    let bad = false;
    for (const el of clause.namedBindings.elements) {
      const imported = (el.propertyName ?? el.name).text, local = el.name.text, typeOnly = clause.isTypeOnly || el.isTypeOnly;
      const m = rule.names?.[imported];
      if (m?.drop) { dropArgs.push(imported); continue; }
      const dest = m?.dest ?? rule.all;
      if (!dest) { unmapped.push(`${spec}#${imported}`); bad = true; continue; }
      const newImported = m?.as ?? imported;
      const alias = el.propertyName ? local : newImported === imported ? undefined : undefined;
      if (!el.propertyName && newImported !== imported) renames.set(imported, newImported);
      const localName = el.propertyName ? local : newImported;
      if (existingNames.has(localName) && !el.propertyName && newImported !== imported && !emitted.has(localName)) {
        if (existingNames.has(localName)) { continue; }
      }
      const item = `${typeOnly && !clause.isTypeOnly ? "type " : ""}${newImported}${el.propertyName && local !== newImported ? ` as ${local}` : ""}`;
      void alias;
      if (!groups.has(dest.file)) groups.set(dest.file, { dest, items: [] });
      groups.get(dest.file)!.items.push(item);
      emitted.add(localName);
    }
    if (bad) continue;
    const typeAll = clause.isTypeOnly;
    const lines = [...groups.values()].map(g => `import ${typeAll ? "type " : ""}{ ${g.items.join(", ")} } from "${specifierFor(g.dest)}";`);
    edits.push({ start: st.getStart(sf), end: st.getEnd(), text: lines.join("\n") });
  }
  const visit = (n: ts.Node) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) {
      const arg = n.arguments[0], spec = arg.text, target = bySpec.get(spec);
      if (target !== undefined) {
        const key = keyOf(spec, target), rule = key ? rules[key] : undefined;
        const decl = ts.isAwaitExpression(n.parent) ? n.parent.parent : undefined;
        const names: string[] = [];
        let pattern: ts.ObjectBindingPattern | undefined;
        if (decl && ts.isVariableDeclaration(decl) && ts.isObjectBindingPattern(decl.name)) { pattern = decl.name; for (const el of decl.name.elements) names.push((el.propertyName as ts.Identifier | undefined)?.text ?? (el.name as ts.Identifier).text); }
        if (!rule || !pattern || !names.length) unmapped.push(`dynamic ${spec} (${key ?? "no key"}) ${names.join(",")}`);
        else {
          const dests = new Set<string>(); let ok = true;
          for (const name of names) { const m = rule.names?.[name]; const dest = m?.dest ?? rule.all; if (!dest || m?.drop) { ok = false; unmapped.push(`dynamic ${spec}#${name}`); } else { dests.add(dest.file); if (m?.as) renames.set(name, m.as); } }
          if (ok && dests.size === 1) {
            const dest = [...dests][0]!; const d = Object.values(D).find(x => x.file === dest)!;
            edits.push({ start: arg.getStart(sf), end: arg.getEnd(), text: JSON.stringify(specifierFor(d)) });
          } else if (ok) unmapped.push(`dynamic ${spec} splits destinations`);
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  if (!edits.length && !unmapped.length) return { file, unmapped, changed: false };
  let out = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  for (const [from, to] of renames) out = out.replace(new RegExp(`(?<![\\w$])${from}(?![\\w$])`, "gu"), to);
  if (dropArgs.includes("configuredRepositoryCargoPreparationPolicyV1")) out = out.replace(/\bpreparation\s*:\s*configuredRepositoryCargoPreparationPolicyV1\(\)\s*,\s*/gu, "");
  const changed = out !== text;
  if (write && changed) writeFileSync(abs, out);
  return { file, unmapped, changed };
}

if (import.meta.main) {
  const write = process.argv[2] === "write";
  const edges = scan(join(ROOT, "✏️s/🔌️plugins")).filter(e => !e.ok && !e.target.includes("💻️os"));
  const byFile = new Map<string, { spec: string; target: string }[]>();
  for (const e of edges) { if (!byFile.has(e.from)) byFile.set(e.from, []); byFile.get(e.from)!.push({ spec: e.spec, target: e.target }); }
  let changed = 0; const un: Report[] = [];
  for (const [file, es] of byFile) {
    const r = rewriteFile(file, es, write);
    if (r.changed) changed++;
    if (r.unmapped.length) un.push(r);
  }
  console.log(JSON.stringify({ files: byFile.size, changed, unmappedFiles: un.length }));
  for (const r of un) console.log(r.file.replace("✏️s/🔌️plugins/", ""), "=>", r.unmapped.join(" | "));
}
