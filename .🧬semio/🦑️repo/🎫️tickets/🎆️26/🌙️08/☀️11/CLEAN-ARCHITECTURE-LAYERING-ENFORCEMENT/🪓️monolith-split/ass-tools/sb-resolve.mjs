/** 📖️ Real Vite resolve of subpath specifiers under the Storybook alias record, old prefix entries versus `buildExactAliasEntries`: `bun sb-resolve.mjs`. `.storybook/main.ts` itself cannot be imported (a foreign import of a removed export), so its baseline record is rebuilt from the same constants. */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { existsSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { STORY_SCOPES, buildScopeAliases, buildExactAliasEntries } from "/Users/ueli/Documents/semio/.storybook/📖️stories/🧭️coordination/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const uiReactDir = resolve(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript");
const uiStylingDir = resolve(repo, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript");
const assetsDir = resolve(repo, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript");
const baseline = {
  "@semio-tech/ui-react/test": join(uiReactDir, "🟦️.ts"),
  "@semio-tech/ui-react": join(uiReactDir, "🟦️.tsx"),
  "@semio-tech/ui-styling": uiStylingDir,
  "@semio-tech/assets": assetsDir,
};
const scoped = buildScopeAliases(STORY_SCOPES, {});
const record = { ...baseline };
for (const [k, v] of Object.entries(scoped)) record[k] = v.startsWith("/") ? v : resolve(repo, v);
Object.assign(record, {
  "vite/internal": resolve(repo, "node_modules/vite/dist/node/index.js"),
  "@semio-tech/framework-platform-core": resolve(repo, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"),
});
const specs = ["@semio-tech/ui-react", "@semio-tech/ui-react/i18n", "@semio-tech/ui-react/appearance", "@semio-tech/ui-react/source", "@semio-tech/ui-styling", "@semio-tech/ui-styling/mixing", "@semio-tech/ui-styling/theme/model", "@semio-tech/assets", "@semio-tech/assets/mesh", "@semio-tech/assets/resolver/delivery", "@semio-tech/assets/cursor/default/light", "@semio-tech/assets/fonts/anta/symbols/regular/outline"];
const importer = join(repo, ".storybook/main.ts");
const oldEntries = Object.entries(record).map(([find, replacement]) => ({ find, replacement }));
const newEntries = buildExactAliasEntries(record, [{ find: /^storybook$/u, replacement: "/sb" }]);
const out = { entries: Object.keys(record).length, semioKeys: Object.keys(record).filter((k) => k.startsWith("@semio-tech/")).length, rows: [] };
for (const [label, alias] of [["old", oldEntries], ["new", newEntries]]) {
  const server = await createServer({ root: repo, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [repo] }, watch: null }, resolve: { alias }, optimizeDeps: { noDiscovery: true, include: [] } });
  for (const spec of specs) {
    const r = await server.pluginContainer.resolveId(spec, importer, { ssr: false });
    const id = r?.id ? String(r.id).split("?")[0] : null;
    out.rows.push({ phase: label, spec, status: id && existsSync(id) ? "ok" : "UNRESOLVED", id: id ? id.replace(repo + "/", "") : null });
  }
  await server.close();
}
writeFileSync(process.env.G + "/ass-sb-resolve.json", JSON.stringify(out, null, 1));
for (const label of ["old", "new"]) {
  const rows = out.rows.filter((r) => r.phase === label);
  console.log(label, "specs", rows.length, "unresolved", rows.filter((r) => r.status !== "ok").length);
}
console.log("record keys", out.entries, "@semio-tech keys", out.semioKeys);
for (const r of out.rows) if (r.phase === "old" && r.status !== "ok") console.log("  old UNRESOLVED", r.spec);
