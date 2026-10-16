const queriesExtra = `/** 🕸️ One GraphQL operation source typed by the result and variables the generated types describe for it. */
export type TypedDocument<TResult, TVariables> = string & { readonly __result?: TResult; readonly __variables?: TVariables };

/** 🕸️ Turns one generated GraphQL operation source into the document the repo CLI accepts — its own source text, which is what \`graphql --query\` is given. */
export function graphql<TResult, TVariables>(source: string): TypedDocument<TResult, TVariables> {
  return source as TypedDocument<TResult, TVariables>;
}`;
const cfg = {
  ext: ".ts",
  drop: ["DocumentNode", "Maybe", "InputMaybe", "Exact", "MakeOptional", "MakeMaybe", "MakeEmpty", "Incremental", "graphql", "DocumentType", "FixDocument"],
  modules: [
    { name: "types", regions: ["⚙️Types"], names: ["RepoEvent"] },
    { name: "entity-emoji-registry", regions: ["🌪️Entity Emoji Registry"] },
    { name: "constants", regions: ["🎞️Constants"] },
    { name: "queries", regions: ["⌛️Queries"], extra: queriesExtra },
    { name: "kit-validation", names: ["Problem", "deserializeKit", "validateKit", "isKitDocument", "validateKitDocument"] },
    { name: "globals", regions: ["🎩️Globals"] },
    { name: "utilities", regions: ["🎼️Utilities"], names: ["execAsync", "execFileAsync"] },
    { name: "helpers", regions: ["🎵️Helpers"] },
    { name: "uri-resolution", regions: ["🏬️URI Resolution"] },
    { name: "file-analysis", regions: ["🗺️File Analysis & Diagnostics"] },
    { name: "tree-providers", names: ["FilterTreeItem", "FilterTreeDataProvider", "MonorepoTreeItem", "treeNodeToItem", "MonorepoTreeDataProvider", "SectionTreeItem", "SectionsTreeDataProvider", "buildCliTreeArgs", "filterProvider", "monorepoProvider", "loadAvailableFilterValues"] },
    { name: "code-lens", regions: ["🪵️Providers"] },
    { name: "commands", names: ["registerCommands"] },
    { name: "extension", isParent: true, regions: ["📜️Activation"] },
  ],
};
const meta = {
  "types": ["⚙️", "⚙️data-models", "Extension data models", ["Structured CLI output and the data models the extension reads from the repo CLI."]],
  "entity-emoji-registry": ["🌪️", "🌪️entity-emoji-registry", "Entity emoji registry", ["The entity-emoji registry projected from the framework-owned entity-kind catalog and the id regex built from it."]],
  "constants": ["🎞️", "🎞️constants", "Extension constants", ["Static configuration for diagnostics and UI strings."]],
  "queries": ["⌛️", "⌛️queries", "Repo GraphQL operations", ["The GraphQL operations the extension sends, typed by the types generated from the served schema."]],
  "kit-validation": ["🧾️", "🧾️kit-validation", "Kit validation", ["Deserialization and diagnostics for kit documents."]],
  "globals": ["🎩️", "🎩️globals", "Extension globals", ["Output channel, diagnostic collections, caches and the CLI concurrency gate."]],
  "utilities": ["🎼️", "🎼️utilities", "Extension utilities", ["Logging, workspace and repo-binary resolution, CLI execution and repo-event parsing."]],
  "helpers": ["🎵️", "🎵️path-helpers", "Path helpers", ["Resolution of scope paths and ticket paths and opening of files at a line."]],
  "uri-resolution": ["🏬️", "🏬️uri-resolution", "Repo URI resolution", ["Tree node cache and labels, CLI tree arguments and navigation to repo URIs."]],
  "file-analysis": ["🗺️", "🗺️file-analysis", "File analysis and diagnostics", ["Analysis of the active file, breach diagnostics and autofix."]],
  "tree-providers": ["🌳️", "🌳️tree-providers", "Tree view providers", ["The filter, monorepo and sections tree views with their providers and filter values."]],
  "code-lens": ["🔎️", "🔎️code-lens", "Code lenses and decorations", ["Definition code lenses, entity-id decorations and the gutter icon."]],
  "commands": ["⌨️", "⌨️commands", "Extension commands", ["Registration of every command the extension contributes."]],
};
for (const m of cfg.modules) {
  if (m.isParent) continue;
  const [emoji, folder, title, doc] = meta[m.name];
  const base = emoji.replace(/️/g, ""), kebab = folder.replace(/^[^\p{L}]+/u, "");
  Object.assign(m, { emoji: base + "️", folder: base + "️" + kebab, title, doc });
}
export default cfg;
