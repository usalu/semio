import { describe, expect, it } from "vitest";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { createRequire } from "node:module";
import ts from "typescript";
import { transformSync } from "esbuild";
import fg from "fast-glob";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { STORY_SCOPES, buildScopeAliases, buildScopeStoryGlobs, repoRootFromHere, resolveActiveScopes, storybookRuntimeChunk, STORYBOOK_PREVIEW_DEPENDENCIES } from "../../📖️stories/🧭️coordination/🟦️.ts";

const root = repoRootFromHere();

describe("Storybook catalog integrity", () => {
  it.each(fixture.sourceAliasScopes)("resolves the current canvas renderer in %s", scope => {
    const aliases = buildScopeAliases(resolveActiveScopes(scope), {});
    expect(aliases[fixture.canvasRendererAlias]).toBe(fixture.canvasRendererEntry);
    expect(existsSync(resolve(root, aliases[fixture.canvasRendererAlias]))).toBe(true);
  });

  it("keeps every authored source alias and watch root on disk", () => {
    for (const scope of STORY_SCOPES) {
      for (const [, target] of Object.entries(scope.aliases ?? {}).filter(([name]) => !name.startsWith("/"))) {
        expect(existsSync(resolve(root, target)), `${scope.id}: ${target}`).toBe(true);
      }
      for (const target of scope.sourceRoots) expect(existsSync(resolve(root, target)), `${scope.id}: ${target}`).toBe(true);
    }
  });

  it("includes every feature-owned story exactly once", () => {
    const registered = fg.sync(buildScopeStoryGlobs(STORY_SCOPES), { cwd: resolve(root, ".storybook"), absolute: true }).sort();
    const authored = fg.sync(["🧰️framework/**/🧪️.story.tsx", "✏️s/**/🧪️.story.tsx"], { cwd: root, absolute: true, ignore: ["**/node_modules/**", "**/🗑️generated/**"] }).sort();
    expect(registered).toEqual(authored);
    expect(new Set(registered).size).toBe(registered.length);
  });

  it("resolves authored story relative imports against their current owner", () => {
    const files = fg.sync(buildScopeStoryGlobs(STORY_SCOPES), { cwd: resolve(root, ".storybook"), absolute: true });
    const missing: string[] = [];
    for (const file of files) {
      const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
      for (const statement of source.statements) {
        if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
        const name = statement.moduleSpecifier.text;
        expect(name, file).not.toBe("@testing-library/react");
        if (!name.startsWith(".")) continue;
        const target = resolve(dirname(file), name.split("?")[0]);
        if (![target, target + ".ts", target + ".tsx", target + ".js", resolve(target, "index.ts")].some(existsSync)) missing.push(`${file}: ${name}`);
      }
    }
    expect(missing).toEqual([]);
  });

  it("aligns the Storybook toolchain with the stable release fixture", () => {
    const manifests = ["package.json", "🧰️framework/🔨️modules/🖱️ui/package.json", "🧰️framework/🛍️products/🦑️repo/package.json"];
    for (const file of manifests) {
      const manifest = JSON.parse(readFileSync(resolve(root, file), "utf8"));
      for (const name of fixture.storybookPackages) {
        const version = manifest.devDependencies?.[name];
        if (version !== undefined) expect(version, `${file}: ${name}`).toBe(fixture.storybookVersion);
      }
    }
  });

  it("prepares generated browser catalogs before every canonical compiler entry", () => {
    const project = JSON.parse(readFileSync(resolve(root, "📋️project.json"), "utf8"));
    expect(project.targets["prepare-storybook"].dependsOn).toEqual(["@semio-tech/ui-styling-tokens:generate", "@semio-tech/assets:build"]);
    for (const [name, target] of Object.entries(project.targets)) {
      if (name.startsWith("dev-storybook") || ["build-storybook", "test-storybook"].includes(name)) {
        expect((target as { dependsOn?: string[] }).dependsOn, name).toContain("prepare-storybook");
      }
    }
  });

  it.each(fixture.runtimeChunks)("isolates the runtime boundary for $path", ({ path, chunk }) => {
    expect(storybookRuntimeChunk(path) ?? null).toBe(chunk);
  });

  it("excludes inactive host WASM imports from scoped UI builds", () => {
    const source = readFileSync(resolve(root, ".storybook/preview.tsx"), "utf8");
    const compile = (active: boolean) => transformSync(source, { loader: "tsx", minifySyntax: true, define: { __STORYBOOK_HOST_WASM__: String(active) } }).code;
    const host = "@semio-tech/framework-editor-rs/pkg/framework_editor.js";
    expect(compile(false)).not.toContain(host);
    expect(compile(true)).toContain(host);
  });

  it("prebundles resolvable external ports before a cold workspace preview", () => {
    expect(STORYBOOK_PREVIEW_DEPENDENCIES).toEqual(expect.arrayContaining(fixture.previewDependencies));
    expect(new Set(STORYBOOK_PREVIEW_DEPENDENCIES).size).toBe(STORYBOOK_PREVIEW_DEPENDENCIES.length);
    const require = createRequire(import.meta.url);
    for (const name of STORYBOOK_PREVIEW_DEPENDENCIES) expect(existsSync(require.resolve(name)), name).toBe(true);
    const config = readFileSync(resolve(root, ".storybook/main.ts"), "utf8");
    expect(config).toContain("...STORYBOOK_PREVIEW_DEPENDENCIES");
    expect(config).not.toContain('".ts": "tsx"');
  });

  it("does not force an undeclared golden-layout optimizer dependency", () => {
    expect(readFileSync(resolve(root, ".storybook/main.ts"), "utf8")).not.toContain('"golden-layout"');
  });
});
