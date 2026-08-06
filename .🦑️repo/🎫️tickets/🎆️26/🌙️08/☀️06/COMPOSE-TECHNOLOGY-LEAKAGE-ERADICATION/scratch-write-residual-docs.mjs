import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join, relative } from "path";

const ticket = process.argv[2];
const fw = readdirSync(".").find((n) => n.includes("framework") && !n.includes("\uFFFD"));
const scanRoots = ["✏️s", fw, ".storybook"];
const patterns = [
  [/compose\/client/, "compose/client"],
  [/compose\/fixture/, "compose/fixture"],
  [/compose\/dev/, "compose/dev"],
  [/compose\/asset/, "compose/asset"],
  [/compose\/example/, "compose/example"],
  [/@semio-tech\/compose/, "@semio-tech/compose"],
  [/@compose\//, "@compose/"],
  [/🏘️compose/, "🏘️compose"],
  [/ensureComposeWasm/, "ensureComposeWasm"],
  [/compose-sketchpad/, "compose-sketchpad"],
  [/compose\.sketchpad/, "compose.sketchpad"],
  [/compose\//, "compose/"],
];
const excludeDirNames = new Set([
  "node_modules",
  ".git",
  "target",
  "dist",
  "storybook-static",
  ".turbo",
  "coverage",
  "pkg",
]);
const puzzleMarker = "🌉️compose";

function walk(dir, out = []) {
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return out;
  }
  for (const ent of entries) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) {
      if (excludeDirNames.has(ent.name)) continue;
      if (ent.name.includes(puzzleMarker) || p.includes(puzzleMarker)) continue;
      walk(p, out);
    } else if (ent.isFile()) {
      if (ent.name === "AGENTS.md") continue;
      const skipBin =
        /(^|\/)(client|client_bin)$/.test(p) ||
        /\.(o|a|so|dylib|exe|wasm|png|jpg|jpeg|gif|svg|ico|woff2?|ttf|otf|bin)$/i.test(p);
      if (skipBin) continue;
      out.push(p);
    }
  }
  return out;
}

const hits = [];
for (const sr of scanRoots) {
  for (const fp of walk(sr)) {
    let text;
    try {
      text = readFileSync(fp, "utf8");
    } catch {
      continue;
    }
    const lines = text.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      if (patterns.some(([re]) => re.test(lines[i]))) {
        hits.push({ file: relative(".", fp), line: i + 1, text: lines[i].trimEnd().slice(0, 300) });
      }
    }
  }
}

function classify(h) {
  if (h.text.includes("🌉️compose")) return "keep-puzzle";
  if (/compose\/decompose/.test(h.text)) return "keep-verb";
  if (h.file.includes("🛍️products/🦑️repo")) {
    if (h.file.includes("⌨️cli/") && (h.file.endsWith("component.go") || h.file.includes("component_test.go"))) {
      return "repo-cli";
    }
    return "repo-product";
  }
  return "other";
}

const by = {};
for (const h of hits) (by[classify(h)] ??= []).push(h);

const remainingPathHits = hits.length;
const scrub = [];
scrub.push("Compose residual scrub — framework + ✏️s + .storybook");
scrub.push("Date: 2026-08-06");
scrub.push("");
scrub.push("Patterns: compose/{client,fixture,dev,asset,example}, @semio-tech/compose, @compose/, 🏘️compose,");
scrub.push("ensureComposeWasm, compose-sketchpad, compose.sketchpad, compose/");
scrub.push("Excludes: node_modules, .git, AGENTS.md, puzzle 🌉️compose tree, client/client_bin binaries");
scrub.push("");
scrub.push("## Fixed this pass");
scrub.push("- ui-react README: removed compose/js/sketchpad path mention");
scrub.push("- .storybook PresentationDeck: compose/algorithm → puzzle/algorithm");
scrub.push("- repo coordinator .env.example: /srv/compose/blob → /srv/semio/blob");
scrub.push("- vscode extension.ts: removed dead @semio-tech/compose-js/compose import comment");
scrub.push("- repo bootstrap script.sh: ~/.local/share/compose/neo4j-desktop → .../semio/neo4j-desktop");
scrub.push("");
scrub.push("## Already clean / kept (prior or intentional)");
scrub.push("- assets package index: no compose-fixture comments (AGENTS.md still mentions it; not edited per rule)");
scrub.push("- workspaces: compose is in WORKSPACE_SCAN_SKIP_DIR_NAMES as isolated legacy (comment already correct)");
scrub.push("- Band story / UiDriver: already retargeted off compose.sketchpad (demo.* / ui.panelToggle.*)");
scrub.push("- Label breadcrumb compose link: already gone");
scrub.push("- OS product: 0 path-pattern hits");
scrub.push("- puzzle glue.rs 🌉️compose path attribute: KEEP (domain composition engine)");
scrub.push("- draw compose/decompose docstring: KEEP (verb)");
scrub.push("");
scrub.push("## Remaining path-level hits after pass: " + remainingPathHits);
scrub.push("- keep-puzzle: " + (by["keep-puzzle"]?.length ?? 0));
scrub.push("- keep-verb: " + (by["keep-verb"]?.length ?? 0));
scrub.push("- repo-product (non-CLI): " + (by["repo-product"]?.length ?? 0));
scrub.push("- repo-cli (Go CLI + tests; deferred): " + (by["repo-cli"]?.length ?? 0));
scrub.push("- other: " + (by["other"]?.length ?? 0));
scrub.push("");
scrub.push("See 🧾repo-product-residual.md for repo-product follow-up inventory.");
scrub.push("");
scrub.push("## Non-repo remaining detail");
for (const h of [...(by["other"] ?? []), ...(by["keep-puzzle"] ?? []), ...(by["keep-verb"] ?? [])]) {
  scrub.push(h.file + ":" + h.line + ": " + h.text);
}

writeFileSync(join(ticket, "🧪residual-scrub.txt"), scrub.join("\n") + "\n");

const repo = by["repo-product"] ?? [];
const cli = by["repo-cli"] ?? [];
const md = [];
md.push("# Repo Product Compose Residuals (Follow-Up)");
md.push("");
md.push("This pass did **not** rewrite the huge Go CLI (`⌨️cli/🐹️component.go`) or its bulk tests.");
md.push(
  "Easy integration-path cleanups already applied: `.env.example` blob root, vscode dead import comment, bootstrap neo4j share path.",
);
md.push("");
md.push("## Policy for follow-up");
md.push("- Keep skip/exempt isolation of the `compose/` root (workspaces scan skip, discovery skip, INTERNAL_PREFIXES awareness).");
md.push(
  "- Tests that use `compose` / `🏘️compose` only as an **example technology name** for monorepo awareness may stay, or retarget to `framework/...` / `✏️s/...` when easy.",
);
md.push("- Remove or retarget path imports / fixtures that **integrate** into `./compose`.");
md.push("");
md.push("## Non-CLI repo-product hits (" + repo.length + ")");
md.push("");

function bucket(name, pred) {
  const list = repo.filter(pred);
  if (!list.length) return;
  md.push("### " + name + " (" + list.length + ")");
  for (const h of list) {
    md.push("- `" + h.file + ":" + h.line + "` — `" + h.text.replaceAll("`", "'") + "`");
  }
  md.push("");
}

bucket(
  "vscode extension.ts (@compose scope stripping)",
  (h) => h.file.includes("extension.ts") && !h.file.includes(".test."),
);
bucket("vscode extension.test.ts (fixtures / URI examples)", (h) => h.file.includes("extension.test.ts"));
bucket("repo-lib index.ts (INTERNAL_PREFIXES / commit msg examples)", (h) => h.file.endsWith("📦️index.ts"));
bucket("repo-lib index.test.ts (example tech paths)", (h) => h.file.includes("index.test.ts"));
bucket(
  "other",
  (h) => !h.file.includes("extension") && !h.file.endsWith("📦️index.ts") && !h.file.includes("index.test.ts"),
);

md.push("## Go CLI deferred (" + cli.length + " hits)");
md.push("");
md.push("Files:");
const cliFiles = [...new Set(cli.map((h) => h.file))];
for (const f of cliFiles) {
  md.push("- `" + f + "` (" + cli.filter((h) => h.file === f).length + " hits)");
}
md.push("");
md.push("Representative themes: `compose/` bundle naming helpers, `@compose/` internal prefix checks,");
md.push("`BreachCompose*` statute ids (`compose/import/...`), tree/analyze/policy tests using `compose/js` as live monorepo sample paths,");
md.push("URI/ID builders with `compose/js/...` fixtures, hook transcript paths under `workspaces-compose`.");
md.push("");
md.push("Binaries `client` / `client_bin` were excluded from the scrub count (rebuild will refresh strings).");

writeFileSync(join(ticket, "🧾repo-product-residual.md"), md.join("\n") + "\n");

console.log("WROTE", join(ticket, "🧪residual-scrub.txt"));
console.log("WROTE", join(ticket, "🧾repo-product-residual.md"));
console.log("REMAINING_PATH_HITS", remainingPathHits);
console.log(JSON.stringify(Object.fromEntries(Object.entries(by).map(([k, v]) => [k, v.length]))));
