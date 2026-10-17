// 🧰 bd10: lists the `runVitest` call sites the ownership gate rejects (same traversal as the gate test, printed instead of asserted).
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import ts from "typescript";
import { policyRootScriptModulePaths } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
const libraryRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const repoRoot = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as { owners: { ownerPath: string }[] };
const UNWALKED_DIRECTORIES: ReadonlySet<string> = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated", "🤖️jco", ".git", ".🧬semio", "⚡️cache", "temp"]);

let repositoryTreeCache: string[] | undefined;

/** 🗂️ Every file in the repository, absolute. Walked once — three separate walks of this tree were the
 * largest single cost in this gate, and each one is the same traversal. */
function repositoryTree(): string[] {
  if (repositoryTreeCache !== undefined) return repositoryTreeCache;
  const files: string[] = [];
  const stack = [repoRoot];
  while (stack.length > 0) {
    const directory = stack.pop()!;
    let entries;
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!UNWALKED_DIRECTORIES.has(entry.name)) stack.push(path);
      } else files.push(path);
    }
  }
  repositoryTreeCache = files;
  return files;
}

const PRODUCT_TREE_ROOTS = ["♻️mit-bestand", "✏️s", "🌎️hub", "🧰️framework"].map((path) => `${resolve(repoRoot, path)}/`);

function walkProductFiles(visit: (path: string) => void): void {
  for (const path of repositoryTree()) if (PRODUCT_TREE_ROOTS.some((root) => path.startsWith(root))) visit(path);
}

/** 🧭️ The `this.root` a `BundleScript` in `path` runs with. A router lives in a `📜️script.ts`, but its
 * `*Script` classes are routinely extracted into sibling semantic owners (`…/🏃️execution/🟦️.ts`), which
 * have no `📜️script.ts` above them — the bundle that imports them is the `📦️packages/🟦️typescript` of the
 * nearest semantic ancestor. */
function routerRoot(path: string): string {
  let candidate = dirname(path);
  while (candidate !== repoRoot) {
    if (existsSync(join(candidate, "📜️script.ts"))) return candidate;
    const bundle = join(candidate, "📦️packages", "🟦️typescript");
    if (existsSync(join(bundle, "📜️script.ts"))) return bundle;
    candidate = dirname(candidate);
  }
  return repoRoot;
}

const missing: string[] = [];
    const wrong: string[] = [];
    const ownerPaths = new Set(fixture.owners.map(({ ownerPath }) => ownerPath));
    for (const path of policyRootScriptModulePaths(repoRoot)) visitSource(resolve(repoRoot, path));
    function visitSource(path: string): void {
      if (!path.endsWith(".ts")) return;
      const content = readFileSync(path, "utf8");
      if (!content.includes("runVitest")) return;
      const source = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const roots = new Map<string, string>();
      const visit = (node: ts.Node) => {
        if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isCallExpression(node.initializer) && node.initializer.expression.getText(source) === "join" && node.initializer.arguments[0]?.getText(source) === "this.root" && ts.isStringLiteral(node.initializer.arguments[1])) roots.set(node.name.text, resolve(routerRoot(path), node.initializer.arguments[1].text));
        if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "runVitest") {
          const argument = node.arguments[2];
          if (!argument) missing.push(relative(repoRoot, path));
          else if (!ts.isStringLiteral(argument)) wrong.push(`${relative(repoRoot, path)}: ${argument.getText(source)}`);
          else {
            const bundle = node.arguments[0];
            let bundleRoot: string | undefined;
            if (bundle?.getText(source) === "this.root") bundleRoot = routerRoot(path);
            else if (bundle && ts.isCallExpression(bundle) && bundle.expression.getText(source) === "join" && bundle.arguments[0]?.getText(source) === "this.root" && ts.isStringLiteral(bundle.arguments[1])) bundleRoot = resolve(routerRoot(path), bundle.arguments[1].text);
            else if (bundle && ts.isIdentifier(bundle)) bundleRoot = roots.get(bundle.text);
            const selected = bundleRoot && relative(repoRoot, resolve(bundleRoot, argument.text)).replaceAll("\\", "/");
            if (!selected || !ownerPaths.has(selected)) wrong.push(`${relative(repoRoot, path)}: ${argument.text} -> ${selected ?? "unresolved"}`);
          }
        }
        ts.forEachChild(node, visit);
      };
      visit(source);
    }
    walkProductFiles(visitSource);

console.log(JSON.stringify({missing, wrong}, null, 1));
