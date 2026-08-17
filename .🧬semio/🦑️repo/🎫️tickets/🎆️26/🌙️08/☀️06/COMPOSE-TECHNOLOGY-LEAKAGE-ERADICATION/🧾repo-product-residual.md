# Repo Product Compose Residuals (Follow-Up)

This pass did **not** rewrite the huge Go CLI (`⌨️cli/🐹️component.go`) or its bulk tests.
Easy integration-path cleanups already applied: `.env.example` blob root, vscode dead import comment, bootstrap neo4j share path.

## Policy for follow-up
- Keep skip/exempt isolation of the `compose/` root (workspaces scan skip, discovery skip, INTERNAL_PREFIXES awareness).
- Tests that use `compose` / `🏘️compose` only as an **example technology name** for monorepo awareness may stay, or retarget to `framework/...` / `✏️s/...` when easy.
- Remove or retarget path imports / fixtures that **integrate** into `./compose`.

## Non-CLI repo-product hits (66)

### vscode extension.test.ts (fixtures / URI examples) (42)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:295` — `  await openFixture("compose/metabolism/wip/initialKit/kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:392` — `    const document = await openFixture("compose/metabolism/wip/initialKit/kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:398` — `    const document = await openFixture("compose/invalid.kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:417` — `    const document = await openFixture("compose/invalid.kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:431` — `    const document = await openFixture("compose/invalid.kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:445` — `    const document = await openFixture("compose/invalid.kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:547` — `    const document = await openFixture("compose/invalid.kit.compose.json");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1079` — `    const node: TreeNodeData = { Kind: "file", ID: "💻️compose/go/compose.go", Label: "compose.go", URI: "" };`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1260` — `    assert.strictEqual(slugify("compose/js/compose.ts"), "COMPOSE-JS-COMPOSE-TS");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1328` — `    const result = parseUri("repo://folders/compose/js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1331` — `    assert.strictEqual(result!.path, "compose/js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1335` — `    const result = parseUri("repo://folder/compose/js/sketchpad/page/getting-started");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1338` — `    assert.strictEqual(result!.path, "compose/js/sketchpad/page/getting-started");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1342` — `    const result = parseUri("repo://files/compose/js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1345` — `    assert.strictEqual(result!.path, "compose/js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1349` — `    const result = parseUri("repo://file/compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1352` — `    assert.strictEqual(result!.path, "compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1356` — `    const result = parseUri("repo://sections/compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1359` — `    assert.strictEqual(result!.path, "compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1363` — `    const result = parseUri("repo://section/compose/js/sketchpad/design.tsx/state-management/design-store");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1366` — `    assert.strictEqual(result!.path, "compose/js/sketchpad/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1370` — `    const result = parseUri("repo://definitions/compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1373` — `    assert.strictEqual(result!.path, "compose/js/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1377` — `    const result = parseUri("repo://definition/compose/js/compose.ts/validate-kit");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1380` — `    assert.strictEqual(result!.path, "compose/js/compose.ts/VALIDATE-KIT");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1553` — `    await vscode.commands.executeCommand("compose.navigate", "repo://folder/compose/js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1613` — `    await vscode.commands.executeCommand("compose.navigate", "repo://section/compose/js/compose.ts/header");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1618` — `    await vscode.commands.executeCommand("compose.navigate", "repo://definition/compose/js/compose.ts/validate-kit");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1764` — `    const text = "Main technology: 🏘️compose📚️js";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1767` — `    assert.strictEqual(matches[0][3], "🏘️compose📚️js");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1772` — `    const text = "[🏘️compose📚️js💻️composets](repo://p/u/compose/b/l/js/f/compose.ts)";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1775` — `    assert.strictEqual(matches[0][1], "🏘️compose📚️js💻️composets");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1776` — `    assert.strictEqual(matches[0][2], "repo://p/u/compose/b/l/js/f/compose.ts");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1820` — `    const text = "Full: 🏘️compose📚️js🗃️sketchpad💻️designtsx🔖️statemanagement🛠️createstore";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1823` — `    assert.strictEqual(matches[0][3], "🏘️compose📚️js🗃️sketchpad💻️designtsx🔖️statemanagement🛠️createstore");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1828` — `    const text = "Compare 🧰️repo⌨️client with 🏘️compose📚️js and 🎯️goalname";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1966` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.json"), true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1967` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism/wip/initialKit/kit.compose.json"), true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1968` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/kit-metabolism.json"), true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1969` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.embedded.compose.json"), true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1970` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/compose/jsonschema/kit.json"), false);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.test.ts:1971` — `    assert.strictEqual(isLikelyKitJsonFilePath("/workspace/asset/compose/metabolism.kit.diff.compose.json"), false);`

### repo-lib index.ts (INTERNAL_PREFIXES / commit msg examples) (1)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:4248` — `    return "commit: bundle scope needs an area name after emojis (e.g. 🏘️compose✍️sketchpad, 🥅️framework, 🖱️ui⚛️react)";`

### repo-lib index.test.ts (example tech paths) (23)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:151` — `    expect(isAdapterBoundaryFile("compose/client/lib/js/index.ts", "//#region 🌐️RsWasmTransport\nexport async function x() {}")).toBe(true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:152` — `    expect(isAdapterBoundaryFile("compose/client/lib/js/kit-store.🟦️worker.ts", "export async function x() {}")).toBe(true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:153` — `    expect(isAdapterBoundaryFile("compose/client/bin/assistant/mcp-app.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:164` — `    const file = "compose/client/lib/js/boundary-probe.ts";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:396` — `    expect(shouldSkipPathForUloc(root, "compose/client/ui/LICENSE.md")).toBe(true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:665` — `    const result = spawnSync("rg", ["-l", "@compose/ui|@ui/react|@elements/", "--glob", "*.{ts,tsx}", "--glob", "!**/.🦑️repo/**", "--glob", "!**/🧪️index.test.ts"], {`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:698` — `    const bundles = parseCommitBundleBody("🏘️compose✍️sketchpad\n🎆️26🌙️06☀️04\n🗺️Map work\n🎆️26🌙️06☀️03\n🧪️Playground\n\n🖱️ui⚛️react\n🎆️26🌙️06☀️02\n🖥️Shell");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:700` — `    expect(bundles[0]?.label).toBe("🏘️compose✍️sketchpad");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:707` — `    expect(() => parseCommitBundleBody("compose/foo|🏘️compose\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:708` — `    expect(() => parseCommitBundleBody("🏘️compose🔀️📊️uloc\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:714` — `    expect(normalizeBundleScopeLabel("🏘️compose🔀️📊️uloc➕️1")).toBe("🏘️compose");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:748` — `      { label: "🏘️compose✍️sketchpad", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["✍️x"] }] },`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:749` — `      { label: "🏘️compose🗃️fixtures", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗃️y"] }] },`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:751` — `    const prefixSets = [[], ["compose/fixture"]];`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:752` — `    expect(pathMatchesBundleIndex("compose/fixture/a.json", 0, prefixSets, bundles)).toBe(false);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:753` — `    expect(pathMatchesBundleIndex("compose/fixture/a.json", 1, prefixSets, bundles)).toBe(true);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:765` — `    expect(commitBundleBodyError("🏘️compose\n🎆️26🌙️06☀️04📊️uloc➕️1\n🗺️Work")).toMatch(/per-day/);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:912` — `      writeFileSync(join(root, "compose/a.ts"), "a\n", "utf8");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:918` — `      writeFileSync(join(root, "compose/a.ts"), '${"a\n".repeat(11)}', "utf8");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:924` — `      const bundles = [mk("🏘️compose"), mk("🖱️ui"), mk("🥅️framework")];`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:925` — `      const paths = [["compose/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:927` — `      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️compose", "🥅️framework"]);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:966` — `    const commitMessage = "🐙️ueli🎆️26🌙️06☀️04🔀️\n\n🏘️compose✍️sketchpad📊️uloc\n🎆️26🌙️06☀️04\n🗺️Work\n\n📊️uloc➕️1🟰️1\n\nSigned-off-by: U <u@e.com>\n";`

## Go CLI deferred (151 hits)

Files:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go` (141 hits)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go` (10 hits)

Representative themes: `compose/` bundle naming helpers, `@compose/` internal prefix checks,
`BreachCompose*` statute ids (`compose/import/...`), tree/analyze/policy tests using `compose/js` as live monorepo sample paths,
URI/ID builders with `compose/js/...` fixtures, hook transcript paths under `workspaces-compose`.

Binaries `client` / `client_bin` were excluded from the scrub count (rebuild will refresh strings).
