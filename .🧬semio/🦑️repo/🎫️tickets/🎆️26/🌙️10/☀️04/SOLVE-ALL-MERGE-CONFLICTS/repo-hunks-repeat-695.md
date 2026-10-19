# Repo and Server Incoming Hunks

### 0 🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/package.json
HEAD 27 BASE 26 STASH 27 CURRENT H
--- 

+++ 

@@ -13,4 +13,5 @@

   },
   "devDependencies": {
+    "ajv": "8.20.0",
     "typescript": "^5.9.3",
     "vitest": "^4.0.17",

### 1 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs
HEAD 73 BASE 1512 STASH 1526 CURRENT H
--- 

+++ 

@@ -1375,4 +1375,18 @@

 
 #[tokio::test]
+async fn oversized_bodies_discard_only_a_bounded_transport_tail(){
+    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/📥️body/🔣️.json")).unwrap();
+    for row in fixture["cases"].as_array().unwrap(){
+        let read=Arc::new(AtomicU64::new(0));let counter=Arc::clone(&read);
+        let chunks=row["chunks"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().as_bytes().to_vec()).collect::<Vec<_>>();
+        let mut original=request(Method::POST,"/commands",remote(1),&[JSON],Vec::new());
+        original.headers_mut().insert(header::CONTENT_LENGTH,row["declared"].as_u64().unwrap().to_string().parse().unwrap());
+        *original.body_mut()=axum::body::Body::from_stream(futures::stream::iter(chunks).map(move|chunk|{counter.fetch_add(chunk.len()as u64,Ordering::Relaxed);Ok::<_,std::convert::Infallible>(Bytes::from(chunk))}));
+        assert!(matches!(body_taken(original,row["limit"].as_u64().unwrap()as usize,Duration::from_secs(1)).await,Err(ServerError::PayloadTooLarge)));
+        assert_eq!(read.load(Ordering::Relaxed),row["read"].as_u64().unwrap());
+    }
+}
+
+#[tokio::test]
 async fn the_throttle_refuses_before_a_handler_runs_and_keys_on_the_address_the_proxy_names() {
     let rate = Rate::per_second(1, 2);

### 2 🧰️framework/🛍️products/🖥️server/🟦️.ts
HEAD 46 BASE 1226 STASH 1219 CURRENT H
--- 

+++ 

@@ -1218,9 +1218,2 @@

 }
 //#endregion 🔖️Client
-
-//#region 🔖️Tests
-if (import.meta.vitest) {
-  const { registerServerWireTests } = await import("./🧪️tests/🔬️wire/🟦️.ts");
-  await registerServerWireTests(import.meta.vitest, { PRESENCE_PROTOCOL, SERVER_ERRORS, SERVER_ROUTES, ServerCallError, ServerClient, WireError, decodeErrorBody, encodeErrorBody, serverCallError, decodeActorKey, decodeCommandEnvelope, decodeCommandOutcome, decodeCommandReceipt, decodeDocumentFrame, decodeEphemeralFrame, decodeEventRecord, decodeEventStreamFrame, decodeFrontierSummary, decodeHybridLogicalClock, decodePresenceFrame, decodePrincipal, decodeQueryConsistency, decodeQueryEnvelope, decodeQueryResult, decodeRejection, decodeServerInstanceDefinition, decodeTraceContext, documentLane, encodeActorKey, encodeCommandEnvelope, encodeCommandOutcome, encodeCommandReceipt, encodeEphemeralFrame, encodeEventRecord, encodeFrontierSummary, encodeHybridLogicalClock, encodePresenceFrame, encodePrincipal, encodeQueryConsistency, encodeQueryEnvelope, encodeQueryResult, encodeRejection, encodeServerInstanceDefinition, encodeTraceContext, ephemeralLane, fetchTransport, presenceLane, presenceSocketUrl, socketRoot, streamLane }, { directory: import.meta.dir, url: import.meta.url });
-}
-//#endregion 🔖️Tests

### 3 🧰️framework/🛍️products/🖥️server/🧪️tests/🎚️config/🟦️.ts
HEAD 26 BASE 29 STASH 29 CURRENT H
--- 

+++ 

@@ -9,5 +9,5 @@

 const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
 
-/** 🧪️ Vitest for `@semio-tech/framework-server` (inline `import.meta.vitest`). */
+/** 🧪️ Vitest for `@semio-tech/framework-server` (owned standalone wire tests). */
 export default defineConfig({
   root: testRoot,
@@ -20,9 +20,9 @@

     root: testRoot,
     name: "@semio-tech/framework-server",
+    cache: false,
     mode: "test",
     environment: "node",
-    include: [],
+    include: ["../../🧪️tests/🔬️wire/🟦️.ts"],
     coverage: { include: ["../../🟦️.ts"] },
-    includeSource: ["../../🟦️.ts"],
     passWithNoTests: false,
   },

### 4 🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️wire/🟦️.ts
HEAD 213 BASE 213 STASH 226 CURRENT H
--- 

+++ 

@@ -1,2 +1,4 @@

+import * as vitest from "vitest";
+import * as serverWire from "../../🟦️.ts";
 import { readFileSync } from "node:fs";
 import { dirname, resolve } from "node:path";
@@ -60,4 +62,13 @@

     });
 
+    it("validates the shared course root query with the owned codec and Ajv", async () => {
+      const { default: Ajv } = await import("ajv");
+      const vector = fixture.vectors.find((entry) => entry.name === "course-root-query-envelope")!;
+      const validate = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["queryId", "kind", "version", "scope", "principal", "arguments", "consistency", "cursor"], properties: { queryId: { type: "string" }, kind: { const: "course.course" }, version: { const: 8 }, scope: { type: "string" }, principal: { type: "object", required: ["kind"], properties: { kind: { const: "anonymous" } }, additionalProperties: false }, arguments: { type: "array", items: { type: "integer", minimum: 0, maximum: 255 } }, consistency: { type: "object", required: ["kind"], properties: { kind: { const: "authority" } }, additionalProperties: false }, cursor: { type: "null" } } });
+      expect(validate(vector.json)).toBe(true);
+      expect(encodeQueryEnvelope(decodeQueryEnvelope(vector.json))).toEqual(vector.json);
+      expect(JSON.parse(new TextDecoder().decode(decodeQueryEnvelope(vector.json).arguments))).toEqual({ type: "course" });
+    });
+
     it("refuses a payload byte outside 0..=255", () => {
       expect(() => decodeEventRecord({ stream: { tenant: "t", kind: "k", id: "i" }, seq: 1, hlc: { millis: 0, counter: 0 }, kind: "e", payload: [256] })).toThrow(WireError);
@@ -212,2 +223,4 @@

   });
 }
+
+await registerServerWireTests(vitest,serverWire,{directory:resolve(dirname(fileURLToPath(import.meta.url)),"../.."),url:new URL("../../🟦️.ts",import.meta.url).href});

### 5 🧰️framework/🛍️products/🦑️repo/README.md
HEAD 211 BASE 136 STASH 137 CURRENT H
--- 

+++ 

@@ -17,20 +17,8 @@

     - 🦾️ Droid
     - …
-- 🧑️‍💻️ IDE integrations
-  - ✅️ Currently
-    - 💻️ VSCode
-    - 🌊️ Windsurf
-    - 🖱️ Cursor
-    - Antigravity
-    - ⚙️ Codex
-    - ❄️ Claude Code
-    - 🦾️ Droid
-  - 📅️ Possible
-    - InteliJ
-    - PyCharm
-    - WebStorm
-    - Rider
-    - Android Studio
-    - …
+- 🎛️ Dashboard developer operations
+  - Setup, build, test, check, serve and debugging through registered commands
+  - Shared task overview, output, progress, restart and cancellation
+  - Passive source navigation and editing through an installed editor
 - 🗣️ Language integrations
   - ✅️ Currently
@@ -112,4 +100,17 @@

 ### 📊️ Meaningful stats
 
+## Developer Operations
+
+Start `bun run dashboard` at the monorepo root. Choose **New Task** (`Ctrl+B n`),
+search by action and owner, complete the command form and choose **Start**. The
+[dashboard guide](🔨️modules/🎛️dashboard/README.md#-daily-work) gives concrete
+setup, testing, debugging, source navigation and repository workflows. Every
+command uses the workspace daemon, so its status, output and cancellation remain
+available after closing a view or restarting an editor.
+
+Editor integrations provide source navigation and language editing. Executable
+commands are maintained in owner dashboard metadata or Nx targets and selected
+in the dashboard.
+
 ## Configuration
 
@@ -129,5 +130,5 @@

 - [`💻️client/⌨️cli`](🔨️modules/💻️client/⌨️cli) – Command line tool for monorepo interactions
 - [`💻️client/🔌️mcp`](🔨️modules/💻️client/🔌️mcp) – MCP server the agent clients speak to
-- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode/README.md) – Visual Studio Code extension, owner of the technology catalog
+- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode) – Passive source navigation and language editing; the dashboard exclusively controls developer operations
 - [`💻️client/🪶️sqlite`](🔨️modules/💻️client/🪶️sqlite/README.md) – SQLite schema of the client's local-only entity store
 - [`🖥️server/🎛️coordinator`](🔨️modules/🖥️server/🎛️coordinator/README.md) – Coordinator REST API and event log

### 6 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go
HEAD 281 BASE 6892 STASH 6889 CURRENT H
--- 

+++ 

@@ -453,4 +453,5 @@

 	root.AddCommand(technologyCommand(factory, &config))
 	root.AddCommand(bundleCommand(factory, &config))
+	analyzeCmd.Flags().StringSlice("policy", nil, "Analyze only the selected registered policies")
 	root.AddCommand(analyzeCmd)
 	root.AddCommand(entityEmojisCommand(&config))
@@ -4386,42 +4387,12 @@

 	Args:  MaximumNArgs(1),
 	RunE: func(cmd *Command, args []string) error {
-		var scope *string
-		if len(args) > 0 {
-			scope = &args[0]
-		}
-		if scope == nil {
-			ctx := codebasepkg.NewCodebaseContext()
-			ctx.LoadBundles()
-			if err := ctx.LoadFiles(); err != nil {
-				return err
-			}
-			if err := ctx.LoadBreachs(); err != nil {
-				return err
-			}
-			if err := ctx.LoadTickets(); err != nil {
-				return err
-			}
-			ctx.LoadPolicies()
-		}
-		variables := map[string]interface{}{}
-		if scope != nil {
-			variables["scope"] = *scope
-		}
-		return graphqlpkg.PrintGQL(`
-			query Analyze($scope: String) {
-				analyze(scope: $scope) {
-					breachs {
-						id
-						summary
-						scope
-						line
-						column
-						excerpt
-						kind { id priority autofixable reason solution }
-					}
-					metrics { total autofixable byPriority { high medium low } }
-				}
-			}
-		`, variables)
+		scope := "all"
+		if len(args) > 0 { scope = args[0] }
+		policies, err := cmd.Flags().GetStringSlice("policy")
+		if err != nil { return err }
+		result := ToolAnalyze(scope, policies)
+		if result.Error != "" { return fmt.Errorf("%s", result.Error) }
+		for _, line := range result.Output.Lines { fmt.Fprintln(cmd.OutOrStdout(), line.Text) }
+		return nil
 	},
 }
@@ -4623,8 +4594,25 @@

 // 🔬️ToolAnalyze MUST complete the operation successfully.
 func ToolAnalyze(scopeRaw string, policyIDs []string) workspace.ToolResult {
+	for _, id := range policyIDs {
+		if _, found := statutes.FindPolicy(id); !found { return workspace.ToolResult{Error: fmt.Sprintf("unknown analysis policy %q", id)} }
+	}
 	eventspkg.Emit(eventspkg.EventAnalyzeStarting, "repo-cli", eventspkg.FolderPayload{Path: scopeRaw})
 	scope := workspace.ParseScope(scopeRaw)
 	bundles := codebasepkg.GetTechnologies()
-	breachs, err := statutes.CheckPolicies(scope, bundles, policyIDs)
+	extensions, language := analysisScopeExtensions(scopeRaw)
+	var breachs []model.Breach
+	var err error
+	if language {
+		scope = workspace.ParseScope("compose")
+		wanted := []string{}
+		for extension := range extensions { wanted = append(wanted, strings.TrimPrefix(extension, ".")) }
+		files, listErr := workspace.GlobByExtension(workspace.RootDir, "**/*", wanted, []string{"**/node_modules/**", "**/.venv/**"}, true)
+		if listErr != nil { return workspace.ToolResult{Error: listErr.Error()} }
+		selected := make([]string, 0)
+		for _, file := range files { if extensions[strings.ToLower(filepath.Ext(file))] { selected = append(selected, file) } }
+		breachs, err = statutes.CheckPoliciesWithContext(statutes.NewPolicyContextWithFiles(scope, bundles, selected), policyIDs)
+	} else {
+		breachs, err = statutes.CheckPolicies(scope, bundles, policyIDs)
+	}
 	if err != nil {
 		return workspace.ToolResult{Error: err.Error()}
@@ -4651,4 +4639,13 @@

 	output.Plain(string(bytes))
 	return workspace.ToolResult{Output: *output, Data: report}
+}
+
+// 🔬️ Selects the explicit dashboard language scopes before policies read their files.
+func analysisScopeExtensions(scope string) (map[string]bool, bool) {
+	values := map[string][]string{"all": {".rs", ".ts", ".tsx", ".go", ".py", ".cs", ".md", ".json"}, "rust": {".rs"}, "typescript": {".ts", ".tsx"}, "go": {".go"}, "python": {".py"}, "markdown": {".md"}}
+	exts, found := values[scope]
+	selected := map[string]bool{}
+	for _, extension := range exts { selected[extension] = true }
+	return selected, found
 }
 

### 7 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/📜️script.ts
HEAD 40 BASE 39 STASH 50 CURRENT H
--- 

+++ 

@@ -1,7 +1,10 @@

 #!/usr/bin/env bun
 import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
-/** 🧭️ `repo-cli-go` router: `bun ./📜️script.ts test`. */
+import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
+/** 🧭️ Builds and tests the Go repository implementation used by dashboard tasks. */
+import { mkdirSync } from "node:fs";
+import { dirname, join } from "node:path";
 import type { FileLinter } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
-import { defineLint, goCoverageArgs, goLevelTestArgs, runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
+import { defineLint, goCoverageArgs, goLevelTestArgs, runCanonicalGoBuild, runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
 import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { runRepoScriptMain } from "../../../📚️library/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
@@ -27,13 +30,21 @@

 });
 
+class BuildScript extends BundleScript {
+  run(): void {
+    const binary = join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/🗃️bin", process.platform === "win32" ? "semio-repo.exe" : "semio-repo");
+    mkdirSync(dirname(binary), { recursive: true });
+    runCanonicalGoBuild(import.meta.dir, ["-trimpath", "-o", binary, "./🚀️bin"], { env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") }, budgetMs: buildBudgetMs() });
+  }
+}
+
 class TestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
     const { level, rest } = resolveTestLevel(segments);
     const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
-    await runRepositoryTestCommand("go", ["test", "...", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
+    await runRepositoryTestCommand("go", ["test", ".", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
   }
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
+const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);
 
 await runRepoScriptMain(router, import.meta.url);

### 8 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔬️_test.go
HEAD 0 BASE 4013 STASH 3992 CURRENT H
--- 

+++ 

@@ -177,15 +177,4 @@

 		},
 		{
-			name: "cursor uses the cross-platform repo bootstrap",
-			path: filepath.Join(repoRoot, ".cursor", "mcp.json"),
-			requiredFragments: []string{
-				`"command": "bun"`,
-				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "cursor"]`,
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-			},
-		},
-		{
 			name: "generic clients use the cross-platform repo bootstrap",
 			path: filepath.Join(repoRoot, ".mcp.json"),
@@ -199,84 +188,20 @@

 		},
 		{
-			name: "copilot uses the cross-platform repo bootstrap",
-			path: filepath.Join(repoRoot, ".vscode", "mcp.json"),
-			requiredFragments: []string{
-				`"command": "bun"`,
-				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "copilot"]`,
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-			},
+			name:               "devcontainer builds the repo client from its canonical source",
+			path:               filepath.Join(repoRoot, ".devcontainer", "devcontainer.json"),
+			requiredFragments:  []string{`"postCreateCommand": ["bun", "repo", "run", "workspace:setup"]`},
+			forbiddenFragments: []string{"customizations", "editor-overlay.json"},
 		},
 		{
-			name: "kiro uses the cross-platform repo bootstrap",
-			path: filepath.Join(repoRoot, ".kiro", "settings", "mcp.json"),
-			requiredFragments: []string{
-				`"command": "bun"`,
-				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "kiro"]`,
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-			},
+			name:               "native shell bootstrap builds the repo client from its canonical source",
+			path:               filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🐚️.sh"),
+			requiredFragments:  []string{"bun repo run workspace:setup", "bun repo run @semio-tech/repo-client:build"},
+			forbiddenFragments: []string{"repo/client/client", "cargo build --release -p semio-framework-repo-cli"},
 		},
 		{
-			name: "windsurf uses the cross-platform repo bootstrap",
-			path: filepath.Join(repoRoot, ".windsurf", "mcp.json"),
-			requiredFragments: []string{
-				`"command": "bun"`,
-				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"]`,
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-			},
-		},
-		{
-			name: "devcontainer builds the repo client from its canonical source",
-			path: filepath.Join(repoRoot, ".devcontainer", "post-create.sh"),
-			requiredFragments: []string{
-				"cargo build --release -p semio-framework-repo-cli",
-				`.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
-				"bun repo run workspace:setup",
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-				"./repo/client/mcp/go",
-				"💻️client",
-			},
-		},
-		{
-			name: "native shell bootstrap builds the repo client from its canonical source",
-			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🐚️.sh"),
-			requiredFragments: []string{
-				"cargo build --release -p semio-framework-repo-cli",
-				`.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
-				"bun repo run workspace:setup",
-			},
-			forbiddenFragments: []string{
-				"repo/client/client",
-				"./repo/client/mcp/go",
-				"💻️client",
-			},
-		},
-		{
-			name: "native windows bootstrap builds the repo client from its canonical source",
-			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🔵️.ps1"),
-			requiredFragments: []string{
-				`@("build", "--release", "-p", "semio-framework-repo-cli")`,
-				`semio-repo.exe`,
-				`.🧬semio/🦑️repo/⚡️cache/🗃️bin`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
-				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
-				`@("repo", "run", "workspace:setup")`,
-			},
-			forbiddenFragments: []string{
-				"repo/client/client.exe",
-				"./repo/client/mcp/go",
-				"💻️client",
-			},
+			name:               "native windows bootstrap builds the repo client from its canonical source",
+			path:               filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🔵️.ps1"),
+			requiredFragments:  []string{`@("repo", "run", "workspace:setup")`, `@("repo", "run", "@semio-tech/repo-client:build")`},
+			forbiddenFragments: []string{"repo/client/client.exe", `@("build", "--release", "-p", "semio-framework-repo-cli")`},
 		},
 	}
@@ -491,4 +416,58 @@

 		t.Errorf("ToolAnalyze returned error: %s", result.Error)
 	}
+}
+
+func TestDashboardAnalysisScopesAndPolicies(t *testing.T) {
+	fixturePath := filepath.Join("..", "..", "..", "🎛️dashboard", "🧫️fixtures", "🏛️repo-action", "🔣️.json")
+	data, err := os.ReadFile(fixturePath)
+	if err != nil { t.Fatal(err) }
+	var fixture struct { Valid []struct { Action map[string]json.RawMessage `json:"action"`; Argv []string `json:"argv"` } `json:"valid"` }
+	if err := json.Unmarshal(data, &fixture); err != nil { t.Fatal(err) }
+	for _, vector := range fixture.Valid {
+		if action, present := vector.Action["Analyze"]; present {
+			var selected struct { Scope string `json:"scope"`; Policy *string `json:"policy"` }
+			if err := json.Unmarshal(action, &selected); err != nil { t.Fatal(err) }
+			extensions, found := analysisScopeExtensions(selected.Scope)
+			if !found || len(extensions) == 0 { t.Fatalf("missing scope %s", selected.Scope) }
+			if selected.Scope == "rust" && !reflect.DeepEqual(extensions, map[string]bool{".rs": true}) { t.Fatal(extensions) }
+			if selected.Scope == "typescript" && !reflect.DeepEqual(extensions, map[string]bool{".ts": true, ".tsx": true}) { t.Fatal(extensions) }
+		}
+	}
+	if result := ToolAnalyze("rust", []string{"NOT-A-POLICY"}); result.Error == "" { t.Fatal("unknown policy produced false success") }
+	root := NewRoot(nil)
+	root.SetArgs([]string{"analyze", "rust", "--policy", "NOT-A-POLICY"})
+	root.SetOut(&bytes.Buffer{})
+	root.SetErr(&bytes.Buffer{})
+	if err := root.Execute(); err == nil || !strings.Contains(err.Error(), "unknown analysis policy") { t.Fatalf("policy CLI dispatch: %v", err) }
+}
+
+func TestDashboardTicketInteractionDurability(t *testing.T) {
+	data, err := os.ReadFile(filepath.Join("..", "..", "..", "🎫️tickets", "🧫️fixtures", "📄️ticket-document-codec", "📄️documents.json"))
+	if err != nil { t.Fatal(err) }
+	var vectors struct { Documents []struct { Text string `json:"text"` } `json:"documents"` }
+	if err := json.Unmarshal(data, &vectors); err != nil { t.Fatal(err) }
+	ticket, err := ticketspkg.DecodeTicketDocument(vectors.Documents[0].Text)
+	if err != nil { t.Fatal(err) }
+	encoded := ticketspkg.EncodeTicketDocument(ticket)
+	oracle, err := json.MarshalIndent(ticket, "", "  ")
+	if err != nil { t.Fatal(err) }
+	if encoded != string(oracle) { t.Fatalf("codec disagrees with encoding/json:\n%s\n%s", encoded, oracle) }
+	roundtrip, err := ticketspkg.DecodeTicketDocument(encoded)
+	if err != nil || !reflect.DeepEqual(roundtrip.Interactions, ticket.Interactions) { t.Fatalf("interaction roundtrip: %+v %v", roundtrip, err) }
+	root := t.TempDir()
+	tracker := ticketspkg.NewRecordedIssueTracker(ticketspkg.IssueTrackerScript{})
+	events := ticketspkg.NewRecordingEventSink()
+	service := ticketspkg.NewTicketService(ticketspkg.NewTicketLayout(filepath.Join(root, ".🧬semio", "🦑️repo")), ticketspkg.NewFileTicketStore(), tracker, ticketspkg.NewFixedTicketClock(26, 9, 23, "2026-09-23 12:00:00"), events)
+	opened, err := service.Open(ticketspkg.TicketOpenRequest{Emoji: "🦑", Title: "Human Reopen", Prompt: "Initial", Client: "claude-code", Goal: "TEST", NoManagement: true})
+	if err != nil { t.Fatal(err) }
+	if _, err := service.Close(ticketspkg.TicketCloseRequest{ID: opened.ID, NoManagement: true, Bulk: true}); err != nil { t.Fatal(err) }
+	prompt := "Continue the exact human request 🦑"
+	if _, err := service.Reopen(ticketspkg.TicketReopenRequest{ID: opened.ID, Prompt: prompt, Client: "claude-code", NoManagement: true}); err != nil { t.Fatal(err) }
+	id, err := ticketspkg.ParseTicketID(opened.ID)
+	if err != nil { t.Fatal(err) }
+	saved, err := service.Read(id)
+	if err != nil { t.Fatal(err) }
+	if len(saved.Interactions) != 3 || saved.Interactions[2].Prompt != prompt || len(tracker.Calls()) != 0 { t.Fatalf("durable human prompt: %+v tracker=%v", saved.Interactions, tracker.Calls()) }
+	if len(events.Recorded()) != 3 { t.Fatal(events.Recorded()) }
 }
 

### 9 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md
HEAD 329 BASE 326 STASH 385 CURRENT H
--- 

+++ 

@@ -3,5 +3,9 @@

 The native Rust developer control plane discovers workspace commands and manages their terminal
 sessions through one workspace daemon. It owns startup, preferences, command selection, output,
-process lifecycle and reconnect. It works without an editor.
+process lifecycle and reconnect. It works without an editor. A ready task offers **Open ready address**
+in its overview row and output context menu; `Ctrl+B u` opens the verified URL and `Ctrl+B U` cancels
+opening while the task keeps running. Both bindings are customizable. The opener runs outside the
+view loop with a bounded, cancellable helper. `SEMIO_DASHBOARD_URL_OPENER` optionally selects a JSON
+argument vector to which the exact ready URL is appended.
 
 ## 🎛️ Developer Control Plane
@@ -13,5 +17,8 @@

 
 Start with `bun run dashboard` (the root script is the build runner bootstrap; it runs the installed native
-executable `semio`). The first start builds and installs the executable once. Installation builds and
+executable `semio`). Commands, typed parameters, arguments, environment, output, progress and
+cancellation are controlled through the dashboard. No IDE workspace, solution, task, launch or IDE MCP configuration is generated or required.
+Headless agent client files are derived from registered dashboard declarations by
+`workspace:agents-write`. The first start builds and installs the executable once. Installation builds and
 publishes an immutable executable with visible progress and cancellation (`Ctrl+C`), and records beside it
 a digest of the sources it was built from: the names, sizes and modification times of the files below the
@@ -32,4 +39,58 @@

 A selection made while the workspace daemon connects remains pending and can be cancelled with
 `Ctrl+B c`; at most 128 starts can wait. The known catalog and full discovery run after the first frame.
+
+### 🧑‍💻 Daily Work
+
+Choose **New Task** (`Ctrl+B n`), search the words below, select the command and
+complete its form. Choose **Start** after checking the preview. The same form
+accepts extra arguments and environment values; recurring operations belong in
+an owner target or typed dashboard declaration.
+
+| Work | Search or command | Next action |
+| --- | --- | --- |
+| Prepare the checkout | `workspace:setup` | Start the setup task; read progress in its output. For native toolchain repair choose `workspace:setup-native`. |
+| Test a project | `test` plus the owner name | Select `<project>:test`; choose the test level in the form. `workspace:test` runs the workspace suite. |
+| Test one Bun file | `tool:workspace/bun-test` | Fill in the workspace-relative file and owning project. |
+| Check, lint and format | `workspace:verify`, `workspace:lint`, `workspace:format` | Choose the check when offered, then inspect the exit code and output. Search `check` or `typecheck` plus the owner for scoped checks. |
+| Build and publish | `workspace:build`, `workspace:publish` or `build` plus the owner | Choose the build mode and scope offered by the form; output identifies the artifacts. |
+| Serve and preview | `playground:s`, `os-hub:dev`, `workspace:dev-storybook` or `dev` plus the owner | Choose renderer/example/user slot when offered; wait for readiness, then choose **Open ready address** (`Ctrl+B u`). |
+| Inspect a failure | **Task overview** (`Ctrl+B h`) | Select the task to see its status, exit code and retained output. Restore a hidden output view with `Ctrl+B s`. |
+| Retry or cancel | Selected task menu | **Restart** (`Ctrl+B r`), **Stop** (`Ctrl+B c`) or **Kill** (`Ctrl+B k`) controls the task and descendants. Closing its view keeps it running. |
+| Debug interactively | `@semio-tech/repo-developer-tools:terminal` | Select working directory and system shell. In that supervised task invoke an installed interactive debugger, provide its program arguments and set breakpoints. Toggle task input with `Ctrl+B t`; stop or restart with the task menu. |
+| Open code at a line | `@semio-tech/repo-developer-tools:open-source` | Fill in file, optional line and installed editor; edit the source, then return to the task controls. |
+| Inspect repository changes | `@semio-tech/repo-developer-tools:git-status`, `git-diff`, `git-history` | Choose path, change area or revision in the form; staged diffs take one base revision. |
+| Inspect a revision or line history | `@semio-tech/repo-developer-tools:git-show`, `@semio-tech/repo-developer-tools:git-blame` | For Show choose one revision, path and summary/patch view. For Blame choose file, optional revision, starting line and up to 500 lines. |
+| Manage tickets and goals | `repo` plus `ticket`, `goal`, `analyze`, `tree` or `statute` | Select the domain action, fill in its typed parameters and confirm any state-changing action. |
+| Start developer tools | `tool` plus `mcp`, `storybook`, or the tool name | Select the registered tool; readiness and process controls appear in its task. |
+| Run several targets | `tool:workspace/run-targets` | Select target names, project names or patterns, concurrency and cache. |
+| Run a scoped authored operation | `tool:workspace/run-script` | Select the workspace-relative `📜️script.ts`, Nx owner and working directory; put its command words in **Arguments**. |
+
+The task overview and context menus offer these controls without memorizing keys.
+Settings (`Ctrl+B p`) changes language, appearance and bindings. English and
+German labels are available throughout. A detached view reconnects to the same
+running tasks on the next dashboard start. For automation, `semio tasks`,
+`semio logs <task> --follow`, `semio restart <task>` and `semio stop <task>` use
+the same handles and output as those controls.
+
+### 🪟️ Window System
+
+Drag a tab to the left or right edge for side-by-side windows, or to the top or bottom edge for stacked
+windows. Drop in the center or on another group's tab strip to join its tabs. A tinted outline previews
+the final rectangle, including space reclaimed from the old group; release commits the arrangement.
+Drag tabs along their strip to reorder them and drag dividers to resize adjacent windows. Splits can nest
+in either direction, and each split retains its own tab group. Escape, focus loss, terminal resize and
+release outside a window cancel a pending docking gesture. Rearranging and closing views preserve running
+tasks and terminal output; layouts belong to each local view.
+
+The prefix defaults to `Ctrl+B`. After it, `Ctrl+Left/Right/Up/Down` moves the current tab beside the next
+window, `m` joins that window's tabs, and `Shift+Left/Right` changes tab order. Existing `|` / `v` and `-`
+create splits, `+` / `_` resize, `z` zooms or restores, `Tab` / `Shift+Tab` cycles focus and `Ctrl+W` closes a
+view. Every action is customizable and listed in English and German keyboard help.
+
+The docking command is schema-defined in `🧬️schema/🪟️windows/🔣️.json`. The shared TUI layout owns atomic
+docking, topology normalization, weights and geometry; its engine owns pointer capture, hit testing and
+preview painting. The dashboard routes the same commands for pointer and keyboard input. Run the
+**test windows** registry target for contract and native interaction tests, and **test journeys** for the
+independent pseudo-terminal screen checks.
 
 ### 🎮️ The Command Registry
@@ -133,8 +194,7 @@

 verbs and forwards nothing to another script: every task of the monorepo is a registry command. The routes of
 the root script are targets of the root project, so `semio run workspace:test`, `semio run workspace:verify
---param check=taxonomy`, `semio run workspace:dev -- storybook`, `semio run workspace:os -- run <bundle>.studio`,
+--param check=taxonomy`, `semio run workspace:dev-storybook`, `semio run workspace:os -- run <bundle>.studio`,
 `semio run workspace:semio -- inspect <path>` and `semio run workspace:examples -- list` start them as daemon
-tasks (`semio commands workspace` lists them). Git workflow routes (`commit`, `micro-commit`) are the
-implementation layer of the agent skills and are run by them as `bun ./📜️script.ts …`. Any other verb is an
+tasks (`semio commands workspace` lists them). Agent skills use registered dashboard commands for authored operations. Any other verb is an
 error that prints the usage and exits with status 2.
 
@@ -142,10 +202,9 @@

 address and attach browser previews to it; find ids with `bun run dashboard commands <words> --json`.
 
-### 🧩 VSCode Launch Projection
-
-Dashboard ticket declarations also own their VSCode controls. Run the normal build target
-`bun repo run @semio-tech/repo-dashboard-rs:launch -- generate YY/MM/DD/TICKET`, or use `check` to refuse a stale projection and `test` to run the independent projection oracle. Generation reads the ticket's current `🎮️commands.json`, emits English and German controls grouped by verb, and replaces only that ticket's projected entries. Other debug configurations and inputs stay in place.
-
-Each control runs its safe registry identity through the same dashboard run route. Text and choice inputs enter dedicated environment variables and then native argument arrays; their contents never enter a shell command. Native registry admission still controls parameter choices, readiness, dependencies and cancellation. The projection does not read the retired launch seed and has no artifact-codec receipt dependency.
+The bootstrap preserves the original process envelope through native preparation and cancels its build
+subprocess tree on interruption or deadline expiry. Cancelling a view ends that view and retains the
+workspace daemon and its tasks. A separately supervised task gets a new invocation
+for its actual command: the view's script envelope, capabilities and policy are discarded from inherited
+environment, while explicit task environment remains authoritative.
 
 ### 🚀 The Launcher
@@ -303,9 +362,9 @@

 and checks the XXH3 vectors of the graph-reuse decision against Bun's own hash:
 `bun repo run @semio-tech/repo-dashboard-rs:test` builds the debug binary and runs it. The scenarios of
-`./🧪️tests/🧫️fixtures/🎮️registry/🥒️.feature` are mapped to Rust tests of `🎮️registry`.
+`🧫️fixtures/🎮️registry/🥒️.feature` are mapped to Rust tests of `🎮️registry`.
 
 `🌀️control-plane` defines language-neutral lifecycle scenarios and protocol vectors. The owned
 Rust codec and the existing Ajv library validate the same vectors. The quick suite runs actual
-Bun PTYs, build-runner server/build/test targets, output replay, cancellation, restart, process-tree
+Bun PTYs, Nx server/build/test targets, output replay, cancellation, restart, process-tree
 termination, singleton enforcement and reattachment: `bun repo run @semio-tech/repo-dashboard-rs:test -- quick`.
 
@@ -313,5 +372,5 @@

 `run` and `install`, the immutable installation that keeps a running executable's bytes while
 a new build is published, and the source digest that decides whether an installation is stale (the fixture names the
-files that must and must not change it): `bun ./📜️script.ts test execution` in `📦️packages/🦀️rust`.
+files that must and must not change it): choose `@semio-tech/repo-dashboard-rs:test-execution` in New Task.
 
 After installing, set `SEMIO_TEST_CLI` to the printed native executable and run the dashboard

### 10 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs
HEAD 842 BASE 840 STASH 840 CURRENT H
--- 

+++ 

@@ -39,5 +39,5 @@

 const EXIT_LIMIT: Duration = Duration::from_secs(8);
 const SIGKILL_EXIT: i32 = 137;
-const DROPPED_ENVIRONMENT: [&str; 1] = ["SEMIO_BUILD_INVOCATION_PID"];
+const DROPPED_ENVIRONMENT: [&str; 5] = ["SEMIO_BUILD_INVOCATION_PID", "NX_INVOCATION_ROOT_PID", "SEMIO_SCRIPT_PROCESS_INVOCATION", "SEMIO_SCRIPT_CAPABILITIES", "SEMIO_SCRIPT_POLICY"];
 
 /// 🚨 A request the daemon refuses, with the kind of failure a view can act on.
@@ -127,5 +127,5 @@

 pub fn child_environment(client: &[(String, String)], declared: &[(String, String)]) -> Vec<(String, String)> {
     let mut env = Vec::new();
-    for (key, value) in client.iter().filter(|(key, _)| !DROPPED_ENVIRONMENT.contains(&key.as_str())) { set_environment(&mut env, key, value); }
+    for (key, value) in client.iter().filter(|(key, _)| !DROPPED_ENVIRONMENT.iter().any(|name| if cfg!(windows) { key.eq_ignore_ascii_case(name) } else { key.as_str() == *name })) { set_environment(&mut env, key, value); }
     for (key, value) in declared { set_environment(&mut env, key, value); }
     for (key, value) in [("TERM", "xterm-256color"), ("COLORTERM", "truecolor")] {

### 11 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
HEAD 76 BASE 1385 STASH 1448 CURRENT H
--- 

+++ 

@@ -31,5 +31,5 @@

 pub const FALLBACK_VERB: &str = "task";
 /// 🦑️ The branches the repo domain contributes, listed after every verb.
-pub const REPO_VERBS: &[&str] = &["tickets", "goals", "analyze", "tree", "statutes"];
+pub const REPO_VERBS: &[&str] = &["tickets", "draft", "goals", "analyze", "tree", "statutes"];
 const RESERVED_KINDS: &[&str] = &["playground", "tool", "compound", "group", "ticket"];
 
@@ -37,5 +37,5 @@

 fn is_action_key(text: &str) -> bool {
     let scope = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_lowercase());
-    matches!(text, "goals.list" | "goals.tree" | "tree.monorepo" | "tree.goal" | "tree.statute" | "tree.territory" | "statutes.catalog")
+    matches!(text, "goals.list" | "goals.tree" | "tree.monorepo" | "tree.goal" | "tree.statute" | "tree.territory" | "statutes.catalog" | "draft.start" | "draft.discard")
         || text.strip_prefix("analyze:").is_some_and(scope)
         || ["ticket.show:", "ticket.files:", "ticket.close:", "ticket.reopen:"].iter().any(|head| text.strip_prefix(head).is_some_and(|id| !id.is_empty()))
@@ -218,4 +218,6 @@

 struct ParameterDeclaration {
     id: String,
+    #[serde(default, deserialize_with = "localized_declaration")] title: Option<LocalizedText>,
+    #[serde(default, deserialize_with = "localized_declaration")] description: Option<LocalizedText>,
     kind: ParameterKind,
     applies_to: Option<AppliesTo>,
@@ -237,5 +239,5 @@

 #[derive(Debug, Clone, Default, PartialEq, Deserialize)]
 #[serde(rename_all = "camelCase", deny_unknown_fields)]
-struct TargetDeclaration { verb: Option<String>, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<MemberDeclaration>, #[serde(default)] parameters: Vec<ParameterDeclaration> }
+struct TargetDeclaration { #[serde(default, deserialize_with = "localized_declaration")] title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] description: Option<LocalizedText>, verb: Option<String>, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<MemberDeclaration>, #[serde(default)] parameters: Vec<ParameterDeclaration> }
 
 /// 🔌️ `#/$defs/McpExposure`: the tool is an MCP stdio server the listed agent clients start.
@@ -249,10 +251,10 @@

 pub struct McpClient { #[serde(default, skip_serializing_if = "BTreeMap::is_empty")] pub parameters: BTreeMap<String, String> }
 
-/// 🧑‍💻️ The agent and editor clients whose MCP configuration the workspace derives (`#/$defs/AgentClientId`).
-pub const AGENT_CLIENTS: [&str; 4] = ["claude-code", "vscode", "cursor", "codex"];
+/// 🧑‍💻️ The agent clients whose MCP configuration the workspace derives (`#/$defs/AgentClientId`).
+pub const AGENT_CLIENTS: [&str; 2] = ["claude-code", "codex"];
 
 #[derive(Debug, Clone, PartialEq, Deserialize)]
 #[serde(rename_all = "camelCase", deny_unknown_fields)]
-struct ToolDeclaration { id: String, verb: Option<String>, command: Vec<String>, #[serde(default)] cwd: String, #[serde(default)] env: BTreeMap<String, String>, #[serde(default)] continuous: bool, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<MemberDeclaration>, #[serde(default)] parameters: Vec<ParameterDeclaration>, mcp: Option<Mcp> }
+struct ToolDeclaration { #[serde(default, deserialize_with = "localized_declaration")] title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] description: Option<LocalizedText>, id: String, verb: Option<String>, command: Vec<String>, #[serde(default)] cwd: String, #[serde(default)] env: BTreeMap<String, String>, #[serde(default)] continuous: bool, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<MemberDeclaration>, #[serde(default)] parameters: Vec<ParameterDeclaration>, mcp: Option<Mcp> }
 
 #[derive(Debug, Clone, PartialEq, Deserialize)]
@@ -276,9 +278,9 @@

 #[derive(Debug, Clone, PartialEq, Deserialize)]
 #[serde(rename_all = "camelCase", deny_unknown_fields)]
-struct CompoundDeclaration { id: String, verb: Option<String>, stop: Option<Stop>, members: Vec<MemberDeclaration> }
+struct CompoundDeclaration { #[serde(default, deserialize_with = "localized_declaration")] title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] description: Option<LocalizedText>, id: String, verb: Option<String>, stop: Option<Stop>, members: Vec<MemberDeclaration> }
 
 #[derive(Debug, Clone, PartialEq, Deserialize)]
 #[serde(rename_all = "camelCase", deny_unknown_fields)]
-struct GroupDeclaration { id: String, targets: Vec<String>, projects: Vec<String> }
+struct GroupDeclaration { #[serde(default, deserialize_with = "localized_declaration")] title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] description: Option<LocalizedText>, id: String, targets: Vec<String>, projects: Vec<String> }
 
 #[derive(Debug, Clone, Default, PartialEq, Deserialize)]
@@ -396,5 +398,5 @@

 #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
 #[serde(rename_all = "lowercase")]
-pub enum Origin { Axis, Target, Tool, Playground, Configuration, Member }
+pub enum Origin { Axis, Target, Tool, Playground, Configuration, Member, Repo }
 
 /// 💥️ What a chosen value does to the launch: `env` joins the environment, `build_flags` go before
@@ -407,8 +409,24 @@

 pub enum Destination { #[default] Token, Env(String), Flag(String), Positional }
 
+/// 🌐️ A complete single-line text in each supported dashboard language.
+#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
+#[serde(deny_unknown_fields)]
+pub struct LocalizedText { pub en: String, pub de: String }
+
+fn localized_declaration<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<LocalizedText>, D::Error> { LocalizedText::deserialize(deserializer).map(Some) }
+
+impl LocalizedText {
+    /// 🗣️ Selects the explicitly chosen dashboard language.
+    pub fn get(&self, locale: Locale) -> &str { match locale { Locale::En => &self.en, Locale::De => &self.de } }
+
+    fn valid(&self) -> bool { [&self.en, &self.de].iter().all(|text| !text.trim().is_empty() && text.chars().count() <= 1024 && !text.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}')) }
+}
+
 /// 🎛️ One parameter a command accepts.
 #[derive(Debug, Clone, PartialEq, Eq)]
 pub struct Parameter {
     pub id: String,
+    pub title: Option<LocalizedText>,
+    pub description: Option<LocalizedText>,
     pub kind: ParameterKind,
     pub origin: Origin,
@@ -424,5 +442,8 @@

 
 impl Parameter {
-    fn choice(id: &str, origin: Origin, default: Option<&str>, values: Vec<(String, Effect)>) -> Self { Self { id: id.into(), kind: ParameterKind::Choice, origin, default: default.map(str::to_string), required: false, values, effect: Effect::default(), destination: Destination::Token, verbs: None, playground: false, scoped: false } }
+    /// 🏷️ A human title in the selected language, or the parameter identity.
+    pub fn display_title(&self, locale: Locale) -> &str { self.title.as_ref().map_or(self.id.as_str(), |text| text.get(locale)) }
+
+    fn choice(id: &str, origin: Origin, default: Option<&str>, values: Vec<(String, Effect)>) -> Self { Self { id: id.into(), title: None, description: None, kind: ParameterKind::Choice, origin, default: default.map(str::to_string), required: false, values, effect: Effect::default(), destination: Destination::Token, verbs: None, playground: false, scoped: false } }
 
     /// 🔬️ Whether a global axis is offered on a command the build runner runs.
@@ -443,4 +464,15 @@

 #[derive(Debug, Clone, PartialEq, Eq)]
 struct Value { text: String, stated: bool }
+
+fn repo_parameters(action: &RepoAction) -> Vec<Parameter> {
+    let text = |id: &str, en: &str, de: &str, help_en: &str, help_de: &str| { let mut parameter = Parameter::choice(id, Origin::Repo, None, Vec::new()); parameter.kind = ParameterKind::Text; parameter.required = true; parameter.title = Some(LocalizedText { en: en.into(), de: de.into() }); parameter.description = Some(LocalizedText { en: help_en.into(), de: help_de.into() }); parameter };
+    match action {
+        RepoAction::DraftStart { .. } => vec![text("title", "Draft Title", "Entwurfstitel", "Give the draft a human title; its identifier is derived from it.", "Gib dem Entwurf einen verständlichen Titel; daraus entsteht seine Kennung.")],
+        RepoAction::DraftDiscard { .. } => vec![text("slug", "Draft Identifier", "Entwurfskennung", "Enter the exact uppercase draft identifier, such as HUMAN-DRAFT.", "Gib die genaue Entwurfskennung in Großbuchstaben ein, etwa HUMAN-DRAFT.")],
+        RepoAction::TicketReopen { .. } => vec![text("prompt", "Reason to Reopen", "Grund Zum Wiederöffnen", "Describe the work to continue; the reason is saved with the ticket.", "Beschreibe die fortzusetzende Arbeit; der Grund wird im Ticket gespeichert.")],
+        RepoAction::Analyze { .. } => { let mut parameter = Parameter::choice("policy", Origin::Repo, Some("all"), ["all", "code"].into_iter().map(|value| (value.into(), Effect::default())).collect()); parameter.title = Some(LocalizedText { en: "Analysis Policy".into(), de: "Analyserichtlinie".into() }); parameter.description = Some(LocalizedText { en: "All runs the implementation's supported policies; Rust currently implements code.".into(), de: "All führt die unterstützten Richtlinien aus; Rust implementiert derzeit code.".into() }); vec![parameter] },
+        _ => Vec::new(),
+    }
+}
 
 #[derive(Debug, Clone, PartialEq, Eq)]
@@ -467,4 +499,6 @@

 pub struct Entry {
     pub id: String,
+    pub title: Option<LocalizedText>,
+    pub description: Option<LocalizedText>,
     pub kind: Kind,
     pub verb: String,
@@ -664,4 +698,15 @@

         Some(ReadySpec { port: ready.port.and_then(|port| u16::try_from(port).ok()).filter(|port| *port > 0), port_env: ready.port_env.clone(), path: ready.path.clone().unwrap_or_default(), printed: ready.printed.unwrap_or(false) })
     }
+    fn localized(&mut self, name: &str, value: Option<&LocalizedText>) -> Option<LocalizedText> {
+        let value = value?;
+        if value.valid() { Some(value.clone()) } else { self.within(name).report("both en and de need non-whitespace single-line text of at most 1024 characters without terminal controls"); None }
+    }
+
+    fn presentation(&mut self, entry: &mut Entry, title: Option<&LocalizedText>, description: Option<&LocalizedText>) {
+        entry.title = self.localized("title", title);
+        entry.description = self.localized("description", description);
+        entry.relabel();
+    }
+
     fn parameters(&mut self, declarations: &[ParameterDeclaration], origin: Origin) -> Vec<Parameter> {
         let mut parameters: Vec<Parameter> = Vec::new();
@@ -680,5 +725,5 @@

             let forbid = |scope: &mut Scope<'_>, kind: &str, members: &[(&str, bool)]| for (name, present) in members { if *present { scope.report(format!("a {kind} has no `{name}`")); } };
             let (effects, values, destinations) = ([("env", declaration.env.is_some()), ("buildFlags", declaration.build_flags.is_some()), ("args", declaration.args.is_some())], [("values", declaration.values.is_some())], [("valueEnv", declaration.value_env.is_some()), ("valueFlag", declaration.value_flag.is_some()), ("valuePositional", declaration.value_positional.is_some())]);
-            let mut parameter = Parameter { id: declaration.id.clone(), kind: declaration.kind, origin, default: None, required: declaration.required.unwrap_or(false), values: Vec::new(), effect: Effect::default(), destination: Destination::Token, verbs: applies.verbs.clone(), playground: applies.playground.unwrap_or(false), scoped: declaration.applies_to.is_some() };
+            let mut parameter = Parameter { id: declaration.id.clone(), title: scope.localized("title", declaration.title.as_ref()), description: scope.localized("description", declaration.description.as_ref()), kind: declaration.kind, origin, default: None, required: declaration.required.unwrap_or(false), values: Vec::new(), effect: Effect::default(), destination: Destination::Token, verbs: applies.verbs.clone(), playground: applies.playground.unwrap_or(false), scoped: declaration.applies_to.is_some() };
             match declaration.kind {
                 ParameterKind::Choice => {
@@ -789,4 +834,5 @@

                 let qualifier = target.name.strip_prefix(verb.as_str()).map_or(target.name.as_str(), |rest| rest.trim_start_matches(['-', ':', '_'])).to_string();
                 entries.push(Entry::new(Draft { id: format!("{}:{}", project.name, target.name), kind: Kind::Target, verb, path, owner: keys.clone(), subject: short_name(&project.name).into(), qualifier, long_running: target.continuous.unwrap_or(false) || ready.is_some(), source: &file, own, action: Action::Target { project: project.name.clone(), target: target.name.clone(), configurations: target.configurations.clone(), ready, requires } }));
+                if let Some(entry) = entries.last_mut() { scope.presentation(entry, declaration.title.as_ref(), declaration.description.as_ref()); }
             }
             let Some(declaration) = project.dashboard.as_ref().and_then(|value| decode::<ProjectDeclaration>(&file, "metadata.semio.dashboard", value, &mut problems)) else { continue };
@@ -812,4 +858,5 @@

                 let qualifier = if group.targets.len() > 1 { group.targets.join("+") } else { first.strip_prefix(verb.as_str()).map_or(first, |rest| rest.trim_start_matches(['-', ':', '_'])).to_string() };
                 entries.push(Entry::new(Draft { id: format!("group:{}/{}", project.name, group.id), kind: Kind::Group, verb, path, owner: keys.clone(), subject: group.id.clone(), qualifier, long_running: false, source: &file, own: Vec::new(), action: Action::Group { targets: group.targets.clone(), projects: group.projects.clone() } }));
+                if let Some(entry) = entries.last_mut() { scope.presentation(entry, group.title.as_ref(), group.description.as_ref()); }
             }
         }
@@ -823,8 +870,9 @@

         for ticket in &facts.tickets {
             let head = segment(ticket.id.as_str(), format!("{} [{}] {}", ticket.id, if ticket.open { "open" } else { "closed" }, elide_end(&ticket.title, LABEL_CELLS)));
-            let actions = [("show", RepoAction::TicketShow { id: ticket.id.clone() }, false), ("files", RepoAction::TicketFiles { id: ticket.id.clone() }, false), if ticket.open { ("close", RepoAction::TicketClose { id: ticket.id.clone() }, true) } else { ("reopen", RepoAction::TicketReopen { id: ticket.id.clone() }, true) }];
+            let actions = [("show", RepoAction::TicketShow { id: ticket.id.clone() }, false), ("files", RepoAction::TicketFiles { id: ticket.id.clone() }, false), if ticket.open { ("close", RepoAction::TicketClose { id: ticket.id.clone() }, true) } else { ("reopen", RepoAction::TicketReopen { id: ticket.id.clone(), prompt: String::new() }, true) }];
             for (key, action, mutating) in actions {
                 let mut entry = Entry::new(Draft { id: format!("repo:{}", dashboard_repo_domain::action_key(&action)), kind: Kind::Repo, verb: "tickets".into(), path: vec![head.clone(), segment(key, key)], owner: Vec::new(), subject: ticket.id.clone(), qualifier: key.into(), long_running: false, source: "", own: Vec::new(), action: Action::Repo(action) });
-                (entry.listed, entry.mutating) = (ticket.open, mutating);
+                (entry.listed, entry.mutating) = (ticket.open || key == "reopen", mutating);
+                if key == "reopen" { entry.title = Some(LocalizedText { en: format!("Reopen Ticket {}", ticket.id), de: format!("Ticket {} Wiederöffnen", ticket.id) }); entry.description = Some(LocalizedText { en: "Continue a closed ticket with a saved human reason.".into(), de: "Setze ein geschlossenes Ticket mit einem gespeicherten Grund fort.".into() }); }
                 entries.push(entry);
             }
@@ -837,10 +885,16 @@

             Self::compounds(&mut scope, &mut entries, &declaration.compounds, &owner, &keys, &|id| format!("ticket:{}/{id}", ticket.id), Kind::Ticket);
         }
-        let fixed = [("goals", "list", RepoAction::GoalsList), ("goals", "tree", RepoAction::GoalsTree), ("tree", "monorepo", RepoAction::TreeMonorepo), ("tree", "goal", RepoAction::TreeGoal), ("tree", "statute", RepoAction::TreeStatute), ("tree", "territory", RepoAction::TreeTerritory), ("statutes", "catalog", RepoAction::StatutesCatalog)];
-        let scopes = dashboard_repo_domain::ANALYZE_SCOPES.iter().map(|(scope, _)| ("analyze", *scope, RepoAction::Analyze { scope: (*scope).to_string() }));
+        let fixed = [("goals", "list", RepoAction::GoalsList), ("goals", "tree", RepoAction::GoalsTree), ("tree", "monorepo", RepoAction::TreeMonorepo), ("tree", "goal", RepoAction::TreeGoal), ("tree", "statute", RepoAction::TreeStatute), ("tree", "territory", RepoAction::TreeTerritory), ("statutes", "catalog", RepoAction::StatutesCatalog), ("draft", "start", RepoAction::DraftStart { title: String::new() }), ("draft", "discard", RepoAction::DraftDiscard { slug: String::new() })];
+        let scopes = dashboard_repo_domain::ANALYZE_SCOPES.iter().map(|(scope, _)| ("analyze", *scope, RepoAction::Analyze { scope: (*scope).to_string(), policy: None }));
         for (verb, key, action) in fixed.into_iter().chain(scopes) {
-            entries.push(Entry::new(Draft { id: format!("repo:{}", dashboard_repo_domain::action_key(&action)), kind: Kind::Repo, verb: verb.into(), path: vec![segment(key, key)], owner: Vec::new(), subject: key.into(), qualifier: String::new(), long_running: false, source: "", own: Vec::new(), action: Action::Repo(action) }));
+            let mutating = action.mutates();
+            let mut entry = Entry::new(Draft { id: format!("repo:{}", dashboard_repo_domain::action_key(&action)), kind: Kind::Repo, verb: verb.into(), path: vec![segment(key, key)], owner: Vec::new(), subject: key.into(), qualifier: String::new(), long_running: false, source: "", own: Vec::new(), action: Action::Repo(action) });
+            entry.mutating = mutating;
+            match key { "start" => { entry.title = Some(LocalizedText { en: "Start Draft".into(), de: "Entwurf Beginnen".into() }); }, "discard" => { entry.title = Some(LocalizedText { en: "Discard Draft".into(), de: "Entwurf Verwerfen".into() }); }, _ => {} }
+            match key { "start" => entry.description = Some(LocalizedText { en: "Create an empty local draft for notes and unfinished work.".into(), de: "Erstelle einen leeren lokalen Entwurf für Notizen und laufende Arbeit.".into() }), "discard" => entry.description = Some(LocalizedText { en: "Remove the selected draft and its contents after confirmation.".into(), de: "Entferne den ausgewählten Entwurf samt Inhalt nach Bestätigung.".into() }), _ => {} }
+            entries.push(entry);
         }
         entries.sort_by(|left, right| verb_rank(&left.verb).cmp(&verb_rank(&right.verb)).then_with(|| left.verb.cmp(&right.verb)).then_with(|| left.label.cmp(&right.label)).then_with(|| left.id.cmp(&right.id)));
+        for entry in &mut entries { if matches!(entry.action, Action::Repo(_)) { for text in [&entry.title, &entry.description].into_iter().flatten() { entry.haystack.push_str(&format!(" {} {}", text.en.to_lowercase(), text.de.to_lowercase())); } } }
         let mut index = HashMap::with_capacity(entries.len());
         entries.retain(|entry| {
@@ -873,5 +927,5 @@

             path.push(segment(format!("tool:{}", tool.id), tool.id.as_str()));
             entries.push(Entry::new(Draft { id: id_of(&tool.id), kind, verb: verb_of(tool.verb.as_deref().filter(|verb| is_verb(verb)), &tool.id), path, owner: keys.to_vec(), subject: tool.id.clone(), qualifier: String::new(), long_running: tool.continuous || ready.is_some(), source: scope.file, own, action: Action::Tool { command: tool.command.clone(), cwd: tool.cwd.clone(), env, ready, requires } }));
-            if let Some(entry) = entries.last_mut() { entry.mcp.clone_from(&tool.mcp); }
+            if let Some(entry) = entries.last_mut() { entry.mcp.clone_from(&tool.mcp); scope.presentation(entry, tool.title.as_ref(), tool.description.as_ref()); }
         }
     }
@@ -889,4 +943,5 @@

             let verb = compound.verb.clone().filter(|verb| is_verb(verb)).unwrap_or_else(|| match verb_of(None, &compound.id) { verb if verb == FALLBACK_VERB => String::new(), verb => verb });
             entries.push(Entry::new(Draft { id: id_of(&compound.id), kind, verb, path, owner: keys.to_vec(), subject: compound.id.clone(), qualifier: String::new(), long_running: false, source: scope.file, own: Vec::new(), action: Action::Compound { members, stop: compound.stop.unwrap_or_default() } }));
+                if let Some(entry) = entries.last_mut() { scope.presentation(entry, compound.title.as_ref(), compound.description.as_ref()); }
         }
     }
@@ -1023,5 +1078,5 @@

                 if slots > 0 { parameters.push(Parameter::choice("user-slot", Origin::Playground, None, (1..=slots).map(|slot| (slot.to_string(), Effect::default())).collect())); }
                 parameters.push(Parameter::choice("app-role", Origin::Playground, None, vec![("viewer".into(), Effect { env: vec![("SEMIO_APP_ROLE".into(), "viewer".into())], ..Default::default() })]));
-                let flag = |id: &str, default: bool, effect: Effect| Parameter { id: id.into(), kind: ParameterKind::Flag, origin: Origin::Playground, default: default.then(|| "true".to_string()), required: false, values: Vec::new(), effect, destination: Destination::Token, verbs: None, playground: false, scoped: false };
+                let flag = |id: &str, default: bool, effect: Effect| Parameter { id: id.into(), title: None, description: None, kind: ParameterKind::Flag, origin: Origin::Playground, default: default.then(|| "true".to_string()), required: false, values: Vec::new(), effect, destination: Destination::Token, verbs: None, playground: false, scoped: false };
                 if let Some(hub) = &playground.hub { parameters.push(flag("hub", false, Effect { env: vec![(HUB_ENV.into(), hub.clone())], ..Default::default() })); }
                 if playground.data_dir.is_some() || playground.user_data_dir.is_some() { parameters.push(flag("data", true, Effect::default())); }
@@ -1046,5 +1101,5 @@

                 parameters
             }
-            Action::Repo(_) => Vec::new(),
+            Action::Repo(action) => repo_parameters(action),
         }
     }
@@ -1258,7 +1313,9 @@

             Action::Repo(action) => {
                 if !extra.is_empty() { return Err(format!("{} takes no extra arguments", entry.id)); }
+                let action = action.selected(&|id| value_of(id).map(str::to_string))?;
+                action.admit_root(&self.root)?;
                 match policy.repo {
                     RepoImplementation::Go => (dashboard_repo_domain::go_binary_path(&self.root).display().to_string(), action.go_argv(), self.root.clone(), false),
-                    RepoImplementation::Rust => (std::env::current_exe().map_err(|error| format!("{}: {error}", entry.id))?.display().to_string(), vec!["repo-view".into(), "--action".into(), serde_json::to_string(action).map_err(|error| error.to_string())?], self.root.clone(), false),
+                    RepoImplementation::Rust => (std::env::current_exe().map_err(|error| format!("{}: {error}", entry.id))?.display().to_string(), vec!["repo-view".into(), "--action".into(), serde_json::to_string(&action).map_err(|error| error.to_string())?], self.root.clone(), false),
                 }
             }
@@ -1330,8 +1387,10 @@

         let mut value = serde_json::json!({
             "id": entry.id, "kind": entry.kind, "verb": entry.verb, "label": entry.label, "owner": entry.owner, "longRunning": entry.long_running, "listed": entry.listed, "mutating": entry.mutating, "source": entry.source,
-            "parameters": self.parameters(entry).iter().map(|parameter| { let mut row = serde_json::json!({ "id": parameter.id, "kind": parameter.kind, "values": parameter.values.iter().map(|(value, _)| value).collect::<Vec<_>>(), "required": parameter.required, "origin": parameter.origin }); if let Some(default) = &parameter.default { row["default"] = default.clone().into(); } row }).collect::<Vec<_>>(),
+            "parameters": self.parameters(entry).iter().map(|parameter| { let mut row = serde_json::json!({ "id": parameter.id, "kind": parameter.kind, "values": parameter.values.iter().map(|(value, _)| value).collect::<Vec<_>>(), "required": parameter.required, "origin": parameter.origin }); if let Some(default) = &parameter.default { row["default"] = default.clone().into(); } if let Some(title) = &parameter.title { row["title"] = serde_json::json!(title); } if let Some(description) = &parameter.description { row["description"] = serde_json::json!(description); } row }).collect::<Vec<_>>(),
         });
         if let Some(ready) = ready { value["ready"] = serde_json::json!({ "port": ready.port, "path": ready.path }); if ready.printed { value["ready"]["printed"] = true.into(); } }
         if let Some(mcp) = &entry.mcp { value["mcp"] = serde_json::json!(mcp); }
+        if let Some(title) = &entry.title { value["title"] = serde_json::json!(title); }
+        if let Some(description) = &entry.description { value["description"] = serde_json::json!(description); }
         value
     }
@@ -1342,7 +1401,10 @@

 
 impl Entry {
+    /// 🏷️ A human command title in the selected language, or its source label.
+    pub fn display_title(&self, locale: Locale) -> &str { self.title.as_ref().map_or(self.label.as_str(), |text| text.get(locale)) }
+
     fn new(draft: Draft<'_>) -> Self {
         let Draft { id, kind, verb, path, owner, subject, qualifier, long_running, source, own, action } = draft;
-        let mut entry = Self { id, kind, verb, path, label: String::new(), owner, subject, qualifier, long_running, listed: true, mutating: false, source: source.to_string(), mcp: None, own, action, haystack: String::new() };
+        let mut entry = Self { id, title: None, description: None, kind, verb, path, label: String::new(), owner, subject, qualifier, long_running, listed: true, mutating: false, source: source.to_string(), mcp: None, own, action, haystack: String::new() };
         entry.relabel();
         entry
@@ -1352,4 +1414,5 @@

         self.label = std::iter::once(self.verb.as_str()).chain(self.path.iter().map(|part| part.label.as_str())).collect::<Vec<_>>().join(" / ");
         self.haystack = format!("{} {}", self.label, self.id).to_lowercase();
+        for text in self.title.iter().chain(self.description.iter()).chain(self.own.iter().flat_map(|parameter| parameter.title.iter().chain(parameter.description.iter()))) { self.haystack.push_str(&format!(" {} {}", text.en, text.de).to_lowercase()); }
     }
 

### 12 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🏛️repo-domain/📦️packages/🦀️rust/Cargo.toml
HEAD 38 BASE 38 STASH 41 CURRENT H
--- 

+++ 

@@ -35,4 +35,7 @@

 semio-framework-repo-tree = { path = "../../../../🌳️tree/📦️packages/🦀️rust" }
 
+semio-framework-repo-todos = { path = "../../../../📝️todos/📦️packages/🦀️rust" }
+semio-framework-repo-identity = { path = "../../../../🪪️identity/📦️packages/🦀️rust" }
+
 [dev-dependencies]
 dashboard_core = { path = "../../../🧱️core/📦️packages/🦀️rust", features = ["testkit"], package = "semio-framework-repo-dashboard-core" }

### 13 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️installation/🟦️.ts
HEAD 185 BASE 185 STASH 186 CURRENT H
--- 

+++ 

@@ -15,4 +15,5 @@

 import { dirname, basename, join, relative, resolve, isAbsolute } from "node:path";
 import { spawn } from "node:child_process";
+import { terminateOwnedChildTree } from "../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
 
 const cache=(root:string)=>join(root,".🧬semio/🦑️repo/⚡️cache");

### 14 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts
HEAD 156 BASE 153 STASH 167 CURRENT H
--- 

+++ 

@@ -70,4 +70,8 @@

 
 class TestScript extends BundleScript {
+  private async native(selection: { manifestPath: string; packages: string[]; extraArgs: string[] }, rest: string[]): Promise<void> {
+    await runRepositoryCargoTests(selection.packages, this.repoRoot, this.invocation, [...selection.extraArgs, "--test-threads", "1", "--success-output", "immediate", ...rest]);
+  }
+
   async run(segments: string[]): Promise<void> {
     if (segments[0] === "unit") {
@@ -75,5 +79,15 @@

       await runRepositoryCargoTests(crates, this.repoRoot, this.invocation, rest); return;
     }
-    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧭️authority/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
+    if (segments[0] === "windows") {
+      const { rest } = resolveTestLevel(segments.slice(1), "quick");
+      runCmd("bun", ["test", join(this.root, "../../🧪️tests/🪟️windows/🟦️.ts"), join(this.root, "../../🧪️tests/⌨️controls/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🖱️pointer-routing/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
+      const selections = [
+        { manifestPath: join(this.root, "Cargo.toml"), packages: [crate], extraArgs: ["--lib", "-E", "test(docking_) | test(real_pointer_drags_) | test(escape_focus_loss_) | test(keyboard_docking_) | test(every_window_docking_) | test(splitter_drag_) | test(nested_splits_)"] },
+        { manifestPath: join(this.repoRoot, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml"), packages: ["semio-framework-ui"], extraArgs: ["--lib", "--features", "tui-terminal", "-E", "test(tui::)"] },
+      ];
+      for (const selection of selections) await this.native(selection, rest);
+      return;
+    }
+    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧭️authority/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_DASHBOARD_BIN: await dashboardExecutable(this.root, this.repoRoot) }) });
     if (segments[0] === "authority") return;
     if (segments[0] === "execution") {

### 15 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs
HEAD 537 BASE 536 STASH 587 CURRENT H
--- 

+++ 

@@ -11,5 +11,5 @@

 use ui_tui::tui::engine::Tui;
 use ui_tui::tui::event::{mods, Event, Key, KeyEvent, KeypadKey, MouseButton, MouseEvent, MouseKind};
-use ui_tui::tui::layout::{reorder_stack_tab, resize_split, resize_window, zoom_window};
+use ui_tui::tui::layout::{dock_window, reorder_stack_tab, resize_split, resize_window, zoom_window, DockPlacement};
 use ui_tui::tui::scene::NodeId;
 use ui_tui::tui::widget::{WidgetSignal, WidgetState};
@@ -21,5 +21,5 @@

 /// 🎬️ Every action a keymap binding can name; the ids are data, this is the one place that knows what they do.
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
-pub(super) enum Action { NewTask, ShowTasks, ShowSettings, ShowHelp, ShowHiddenTasks, SearchOutput, RefreshCommands, CancelDiscovery, RestartTask, StopTask, KillTask, CopySelection, ToggleInput, ToggleAppearance, ToggleLanguage, NextWindow, PreviousWindow, SplitDown, SplitRight, GrowWindow, ShrinkWindow, Zoom, CloseWindow, Detach, Shutdown, CancelPrefix, SendPrefix }
+pub(super) enum Action { NewTask, ShowTasks, ShowSettings, ShowHelp, ShowHiddenTasks, SearchOutput, RefreshCommands, CancelDiscovery, RestartTask, StopTask, KillTask, OpenReady, CancelOpenReady, CopySelection, ToggleInput, ToggleAppearance, ToggleLanguage, NextWindow, PreviousWindow, SplitDown, SplitRight, DockLeft, DockRight, DockTop, DockBottom, JoinWindow, TabTopLeft, TabTopRight, TabBottomLeft, TabBottomRight, TabLeft, TabRight, GrowWindow, ShrinkWindow, Zoom, CloseWindow, Detach, Shutdown, CancelPrefix, SendPrefix }
 
 impl Action {
@@ -29,8 +29,11 @@

             Self::NewTask => "new-task", Self::ShowTasks => "show-tasks", Self::ShowSettings => "show-settings", Self::ShowHelp => "show-help", Self::ShowHiddenTasks => "show-hidden-tasks", Self::SearchOutput => "search-output",
             Self::RefreshCommands => "refresh-commands", Self::CancelDiscovery => "cancel-discovery", Self::RestartTask => "restart-task", Self::StopTask => "stop-task", Self::KillTask => "kill-task",
+            Self::OpenReady => "open-ready", Self::CancelOpenReady => "cancel-open-ready",
             Self::CopySelection => "copy-selection", Self::ToggleInput => "toggle-input", Self::ToggleAppearance => "toggle-appearance", Self::ToggleLanguage => "toggle-language",
             Self::NextWindow => "next-window", Self::PreviousWindow => "previous-window", Self::SplitDown => "split-down", Self::SplitRight => "split-right", Self::GrowWindow => "grow-window",
             Self::ShrinkWindow => "shrink-window", Self::Zoom => "zoom", Self::CloseWindow => "close-window", Self::Detach => "detach", Self::Shutdown => "shutdown", Self::CancelPrefix => "cancel-prefix",
+            Self::TabTopLeft => "tab-top-left", Self::TabTopRight => "tab-top-right", Self::TabBottomLeft => "tab-bottom-left", Self::TabBottomRight => "tab-bottom-right",
             Self::SendPrefix => "send-prefix",
+            Self::DockLeft => "dock-left", Self::DockRight => "dock-right", Self::DockTop => "dock-top", Self::DockBottom => "dock-bottom", Self::JoinWindow => "join-window", Self::TabLeft => "tab-left", Self::TabRight => "tab-right",
         }
     }
@@ -40,8 +43,11 @@

             "new-task" => Self::NewTask, "show-tasks" => Self::ShowTasks, "show-settings" => Self::ShowSettings, "show-help" => Self::ShowHelp, "show-hidden-tasks" => Self::ShowHiddenTasks, "search-output" => Self::SearchOutput,
             "refresh-commands" => Self::RefreshCommands, "cancel-discovery" => Self::CancelDiscovery, "restart-task" => Self::RestartTask, "stop-task" => Self::StopTask, "kill-task" => Self::KillTask,
+            "open-ready" => Self::OpenReady, "cancel-open-ready" => Self::CancelOpenReady,
             "copy-selection" => Self::CopySelection, "toggle-input" => Self::ToggleInput, "toggle-appearance" => Self::ToggleAppearance, "toggle-language" => Self::ToggleLanguage,
             "next-window" => Self::NextWindow, "previous-window" => Self::PreviousWindow, "split-down" => Self::SplitDown, "split-right" => Self::SplitRight, "grow-window" => Self::GrowWindow,
             "shrink-window" => Self::ShrinkWindow, "zoom" => Self::Zoom, "close-window" => Self::CloseWindow, "detach" => Self::Detach, "shutdown" => Self::Shutdown, "cancel-prefix" => Self::CancelPrefix,
+            "tab-top-left" => Self::TabTopLeft, "tab-top-right" => Self::TabTopRight, "tab-bottom-left" => Self::TabBottomLeft, "tab-bottom-right" => Self::TabBottomRight,
             "send-prefix" => Self::SendPrefix,
+            "dock-left" => Self::DockLeft, "dock-right" => Self::DockRight, "dock-top" => Self::DockTop, "dock-bottom" => Self::DockBottom, "join-window" => Self::JoinWindow, "tab-left" => Self::TabLeft, "tab-right" => Self::TabRight,
             _ => return None,
         })
@@ -124,4 +130,5 @@

     /// 🎮️ Routes one event; the only entry point of input, with or without a real terminal.
     pub fn handle(&mut self, tui: &mut Tui, event: &Event) -> Flow {
+        if matches!(event, Event::Key(KeyEvent { key: Key::Esc, .. })) && tui.dragging_window() && tui.cancel_pointer() { return Flow::Continue; }
         match event {
             Event::Key(key) => self.on_key(tui, key),
@@ -297,4 +304,6 @@

             Outcome::Start(request) => { if let Err(error) = self.start(tui, index, &request) { self.notice = Some(self.text().form_problem.fill(&[("problem", &error)]).into_string()); } }
         }
+        self.paint_launcher(tui, index);
+        self.follow_stage(tui, self.focused_index(tui));
     }
 
@@ -321,5 +330,5 @@

                 (WidgetSignal::Dismissed, _) if self.menu.as_ref().is_some_and(|menu| menu.node == node) => self.menu = None,
                 (WidgetSignal::WindowClose(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { if self.close_window(tui, target) { self.quit_requested = true; } } }
-                (WidgetSignal::WindowMaximize(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { self.toggle_zoom(tui, target); } }
+                (WidgetSignal::WindowMaximize(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { self.focus_window(tui, target); self.toggle_zoom(tui, target); } }
                 (WidgetSignal::WindowNewTab(tab), Some(index)) => {
                     let host = self.tab_window(index, tab).map_or_else(|| self.windows[index].id.clone(), |target| self.windows[target].id.clone());
@@ -330,8 +339,13 @@

                 (WidgetSignal::Activated(item), Some(index)) if self.windows[index].tree == Some(node) => {
                     if let Body::Launcher(launcher) = &mut self.windows[index].body { launcher.activated(item); }
+                    self.paint_launcher(tui, index);
                     tui.set_focus(Some(self.windows[index].focus()));
                 }
                 (WidgetSignal::WindowFocus, Some(index)) => self.focus_window(tui, index),
-                (WidgetSignal::TabMoved { from, to }, Some(index)) => { let id = self.windows[index].id.clone(); if reorder_stack_tab(&mut self.layout, &id, from, to) { self.remount(tui); } }
+                (WidgetSignal::WindowDocked { source, target, placement, tab_index }, _) => {
+                    if dock_window(&mut self.layout, &source, &target, placement, tab_index) {
+                        if let Some(index) = self.window_index(&source) { self.focus_window(tui, index); }
+                    }
+                }
                 (WidgetSignal::SplitterDragged { path, delta }, _) => { let area = tui.scene.rect(self.shell.canvas); if resize_split(&mut self.layout, area, &path, delta) { self.remount(tui); } }
                 _ => {}
@@ -347,5 +361,9 @@

         let output = self.windows[target].is_output();
         let rows: Vec<Option<Action>> = if output {
-            vec![Some(Action::CopySelection), Some(Action::ToggleInput), None, Some(Action::RestartTask), Some(Action::StopTask), Some(Action::KillTask), None, Some(Action::SplitRight), Some(Action::SplitDown), Some(Action::Zoom), Some(Action::CloseWindow)]
+            let mut rows = vec![Some(Action::CopySelection), Some(Action::ToggleInput)];
+            if self.ready_address(target).is_some() { rows.push(Some(Action::OpenReady)); }
+            if self.opening_url.is_some() { rows.push(Some(Action::CancelOpenReady)); }
+            rows.extend([None, Some(Action::RestartTask), Some(Action::StopTask), Some(Action::KillTask), None, Some(Action::SplitRight), Some(Action::SplitDown), Some(Action::Zoom), Some(Action::CloseWindow)]);
+            rows
         } else {
             vec![Some(Action::NewTask), None, Some(Action::SplitRight), Some(Action::SplitDown), Some(Action::Zoom), Some(Action::CloseWindow)]
@@ -369,4 +387,5 @@

 
     fn toggle_zoom(&mut self, tui: &mut Tui, index: usize) {
+        if self.windows.len() < 2 && self.layout.zoomed.is_none() { return; }
         if self.layout.zoomed.is_some() { zoom_window(&mut self.layout, None); } else { zoom_window(&mut self.layout, Some(&self.windows[index].id.clone())); }
         self.remount(tui);
@@ -415,4 +434,8 @@

             Action::StopTask => self.control_session(tui, index, "stop-task"),
             Action::KillTask => self.control_session(tui, index, "kill-task"),
+            Action::OpenReady => {
+                if let Some(id) = self.selected_session_id(index).map(str::to_string) { self.request_open_ready(&id); } else { self.notice = Some(self.text().open_unavailable.as_str().into()); }
+            }
+            Action::CancelOpenReady => self.cancel_open_ready(),
             Action::CopySelection => {
                 let signal = self.terminal_node(index).and_then(|node| match tui.scene.node_mut(node).widget() { Some(WidgetState::Terminal(terminal)) => terminal.copy_selection(), _ => None });
@@ -425,4 +448,28 @@

             Action::PreviousWindow => self.cycle_window(tui, -1),
             Action::SplitDown | Action::SplitRight => { let created = self.create_window(tui, Kind::Launcher, Place::Split(if action == Action::SplitDown { "column" } else { "row" })); self.focus_window(tui, created); }
+            Action::DockLeft | Action::DockRight | Action::DockTop | Action::DockBottom | Action::JoinWindow => {
+                let source = self.windows[index].id.clone();
+                let order = self.window_order();
+                if order.len() > 1 {
+                    let from = order.iter().position(|id| id == &source).unwrap_or(0);
+                    let target = order[(from + 1) % order.len()].clone();
+                    let placement = match action { Action::DockLeft => DockPlacement::Left, Action::DockRight => DockPlacement::Right, Action::DockTop => DockPlacement::Top, Action::DockBottom => DockPlacement::Bottom, _ => DockPlacement::Tabs };
+                    if dock_window(&mut self.layout, &source, &target, placement, None) { self.focus_window(tui, index); }
+                }
+            }
+            Action::TabTopLeft | Action::TabTopRight | Action::TabBottomLeft | Action::TabBottomRight => {
+                use ui_tui::tui::layout::WindowStackCorner::*;
+                let source = self.windows[index].id.clone();
+                let corner = match action { Action::TabTopLeft => TopLeft, Action::TabTopRight => TopRight, Action::TabBottomLeft => BottomLeft, _ => BottomRight };
+                if dock_window(&mut self.layout, &source, &source, DockPlacement::Corner(corner), None) { self.focus_window(tui, index); }
+            }
+            Action::TabLeft | Action::TabRight => {
+                let source = self.windows[index].id.clone();
+                let tabs = super::windows::stack_of(&self.layout, &source);
+                if let Some(from) = tabs.iter().position(|id| id == &source) {
+                    let to = if action == Action::TabLeft { from.saturating_sub(1) } else { (from + 1).min(tabs.len() - 1) };
+                    if reorder_stack_tab(&mut self.layout, &source, from, to) { self.remount(tui); }
+                }
+            }
             Action::GrowWindow | Action::ShrinkWindow => { let window = self.windows[index].id.clone(); resize_window(&mut self.layout, &window, if action == Action::GrowWindow { GROW_STEP } else { -GROW_STEP }); self.remount(tui); }
             Action::Zoom => self.toggle_zoom(tui, index),
@@ -509,5 +556,5 @@

         } else {
             let controls = KeyHint { key: self.keymap.prefix().label(), label: text.hint_controls.as_str().into() };
-            if self.terminal_has_keyboard(tui) { vec![controls] } else {
+            if self.terminal_has_keyboard(tui) { std::iter::once(controls).chain(hint(Scope::Prefix, if self.opening_url.is_some() { "cancel-open-ready" } else { "open-ready" }).filter(|_| self.opening_url.is_some() || self.ready_address(self.focused_index(tui)).is_some())).collect() } else {
                 std::iter::once(controls).chain([(Scope::View, "activate"), (Scope::View, "back"), (Scope::View, "up"), (Scope::View, "down"), (Scope::Window, "next-window")].into_iter().filter_map(|(scope, action)| hint(scope, action).map(|found| if action == "activate" { KeyHint { label: text.hint_activate.as_str().to_string(), ..found } } else { found }))).collect()
             }
@@ -518,4 +565,8 @@

     fn footer_status(&self, tui: &Tui) -> String {
         let text = self.text();
+        if let Some((placement, _)) = tui.docking_preview() {
+            let action = match placement { DockPlacement::Tabs => text.dock_tabs, DockPlacement::Corner(corner) => match corner { ui_tui::tui::layout::WindowStackCorner::TopLeft => text.dock_top_left, ui_tui::tui::layout::WindowStackCorner::TopRight => text.dock_top_right, ui_tui::tui::layout::WindowStackCorner::BottomLeft => text.dock_bottom_left, ui_tui::tui::layout::WindowStackCorner::BottomRight => text.dock_bottom_right }, DockPlacement::Left => text.dock_left, DockPlacement::Right => text.dock_right, DockPlacement::Top => text.dock_top, DockPlacement::Bottom => text.dock_bottom };
+            return text.dock_preview.fill(&[("action", action.as_str())]).into_string();
+        }
         let link = match &self.state {
             LinkState::Connecting => text.state_connecting.as_str().to_string(),

### 16 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️labels/🦀️.rs
HEAD 290 BASE 290 STASH 323 CURRENT H
--- 

+++ 

@@ -33,4 +33,9 @@

     row_no_tasks: "No tasks yet — start one with New task", "Noch keine Aufgaben — mit Neue Aufgabe starten";
     row_back: "Back to tasks", "Zurück zu Aufgaben";
+    open_starting: "Opening ready address; {cancel} cancels", "Bereite Adresse öffnen; {cancel} bricht ab";
+    open_finished: "Ready address opened", "Bereite Adresse geöffnet";
+    open_unavailable: "The selected task has no active ready address", "Die ausgewählte Aufgabe hat keine aktive bereite Adresse";
+    open_failed: "Cannot open ready address: {problem}", "Bereite Adresse lässt sich nicht öffnen: {problem}";
+    open_cancelled: "Opening cancelled; task keeps running", "Öffnen abgebrochen; Aufgabe läuft weiter";
     status_pending: "waiting", "wartet";
     status_running: "running", "läuft";
@@ -104,4 +109,5 @@

     help_customize: "Customize with: semio preferences set --prefix KEY --bindings JSON", "Anpassen mit: semio preferences set --prefix TASTE --bindings JSON";
     chrome_close: "Close tab", "Tab schließen";
+    chrome_drag: "Drag window", "Fenster ziehen";
     chrome_maximize: "Maximize window", "Fenster maximieren";
     chrome_restore: "Restore window", "Fenster wiederherstellen";
@@ -156,4 +162,6 @@

     action_restart_task: "restart", "neu starten";
     action_stop_task: "stop", "anhalten";
+    action_open_ready: "Open ready address", "Bereite Adresse öffnen";
+    action_cancel_open_ready: "Cancel opening", "Öffnen abbrechen";
     action_kill_task: "kill", "beenden";
     action_copy_selection: "copy selection", "Auswahl kopieren";
@@ -166,4 +174,25 @@

     action_split_down: "split down", "unten teilen";
     action_split_right: "split right", "rechts teilen";
+    action_tab_top_left: "place tab at top left", "Tab oben links platzieren";
+    action_tab_top_right: "place tab at top right", "Tab oben rechts platzieren";
+    action_tab_bottom_left: "place tab at bottom left", "Tab unten links platzieren";
+    action_tab_bottom_right: "place tab at bottom right", "Tab unten rechts platzieren";
+    action_dock_left: "move tab left of next window", "Tab links vom nächsten Fenster";
+    action_dock_right: "move tab right of next window", "Tab rechts vom nächsten Fenster";
+    action_dock_top: "move tab above next window", "Tab über nächstes Fenster";
+    action_dock_bottom: "move tab below next window", "Tab unter nächstes Fenster";
+    action_join_window: "join next window as a tab", "als Tab in nächstes Fenster";
+    action_tab_left: "move tab left", "Tab nach links";
+    action_tab_right: "move tab right", "Tab nach rechts";
+    dock_top_left: "tab at top left", "Tab oben links";
+    dock_top_right: "tab at top right", "Tab oben rechts";
+    dock_bottom_left: "tab at bottom left", "Tab unten links";
+    dock_bottom_right: "tab at bottom right", "Tab unten rechts";
+    dock_tabs: "join tabs", "Tabs zusammenführen";
+    dock_left: "split left", "links teilen";
+    dock_right: "split right", "rechts teilen";
+    dock_top: "split above", "oben teilen";
+    dock_bottom: "split below", "unten teilen";
+    dock_preview: "{action} · Esc cancels · release to dock", "{action} · Esc bricht ab · zum Andocken loslassen";
     action_grow_window: "grow window", "Fenster vergrößern";
     action_shrink_window: "shrink window", "Fenster verkleinern";
@@ -217,6 +246,10 @@

             "show-hidden-tasks" => self.action_show_hidden_tasks, "refresh-commands" => self.action_refresh_commands, "cancel-discovery" => self.action_cancel_discovery,
             "search-output" => self.action_search_output, "restart-task" => self.action_restart_task, "stop-task" => self.action_stop_task, "kill-task" => self.action_kill_task, "copy-selection" => self.action_copy_selection,
+            "open-ready" => self.action_open_ready, "cancel-open-ready" => self.action_cancel_open_ready,
             "toggle-input" => self.action_toggle_input, "toggle-appearance" => self.action_toggle_appearance, "toggle-language" => self.action_toggle_language,
             "next-window" => self.action_next_window, "previous-window" => self.action_previous_window, "split-down" => self.action_split_down, "split-right" => self.action_split_right,
+            "dock-left" => self.action_dock_left, "dock-right" => self.action_dock_right, "dock-top" => self.action_dock_top, "dock-bottom" => self.action_dock_bottom,
+            "tab-top-left" => self.action_tab_top_left, "tab-top-right" => self.action_tab_top_right, "tab-bottom-left" => self.action_tab_bottom_left, "tab-bottom-right" => self.action_tab_bottom_right,
+            "join-window" => self.action_join_window, "tab-left" => self.action_tab_left, "tab-right" => self.action_tab_right,
             "grow-window" => self.action_grow_window, "shrink-window" => self.action_shrink_window, "zoom" => self.action_zoom, "close-window" => self.action_close_window,
             "detach" => self.action_detach, "shutdown" => self.action_shutdown, "cancel-prefix" => self.action_cancel_prefix, "send-prefix" => self.action_send_prefix,

### 17 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📋️panes/🦀️.rs
HEAD 179 BASE 179 STASH 185 CURRENT H
--- 

+++ 

@@ -13,5 +13,5 @@

 /// 🎬️ What activating a row does.
 #[derive(Clone, Debug, PartialEq, Eq)]
-pub(super) enum RowAction { None, NewTask, Settings, Help, Refresh, CancelDiscovery, Session(String), Language, Appearance, Terminology, Renderer, Layout, SaveScope, Back }
+pub(super) enum RowAction { None, NewTask, Settings, Help, Refresh, CancelDiscovery, Session(String), OpenReady(String), CancelOpenReady, Language, Appearance, Terminology, Renderer, Layout, SaveScope, Back }
 
 /// 🪧️ The rows of one list window and where the selection and the visible slice are.
@@ -40,5 +40,9 @@

                 let mut rows = vec![(text.row_new_task.as_str().into(), RowAction::NewTask), (text.row_settings.as_str().into(), RowAction::Settings), (text.row_keyboard.as_str().into(), RowAction::Help), (text.row_refresh.as_str().into(), RowAction::Refresh), (text.row_cancel_discovery.as_str().into(), RowAction::CancelDiscovery)];
                 if self.sessions.is_empty() { rows.push((text.row_no_tasks.as_str().into(), RowAction::None)); }
-                rows.extend(self.sessions.values().map(|session| (self.session_row(session), RowAction::Session(session.session_id.clone()))));
+                if self.opening_url.is_some() { rows.push((text.action_cancel_open_ready.as_str().into(), RowAction::CancelOpenReady)); }
+                for session in self.sessions.values() {
+                    rows.push((self.session_row(session), RowAction::Session(session.session_id.clone())));
+                    if let Some(url) = dashboard_cli::browser::ready_url(session.status, session.ready_url.as_deref()) { rows.push((format!("  {}: {url}", text.action_open_ready.as_str()), RowAction::OpenReady(session.session_id.clone()))); }
+                }
                 rows
             }
@@ -137,4 +141,6 @@

             RowAction::CancelDiscovery => self.cancel_inventory(),
             RowAction::Session(id) => self.reveal_session(tui, &id),
+            RowAction::OpenReady(id) => self.request_open_ready(&id),
+            RowAction::CancelOpenReady => self.cancel_open_ready(),
             RowAction::Language => self.queue_preference(Change { language: Some(if self.preferences.language == "en" { "de" } else { "en" }.into()), ..Default::default() }),
             RowAction::Appearance => self.queue_preference(Change { appearance: Some(if self.light { "dark" } else { "light" }.into()), ..Default::default() }),

### 18 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📡️sessions/🦀️.rs
HEAD 381 BASE 381 STASH 428 CURRENT H
--- 

+++ 

@@ -162,4 +162,45 @@

         let message = match action { "restart-task" => ClientMsg::Restart { session_id }, "stop-task" => ClientMsg::Stop { session_id }, _ => ClientMsg::Kill { session_id } };
         if let Err(error) = self.send(&message) { self.notice = Some(error.to_string()); }
+    }
+
+    /// 🎯️ The task selected by an output window or an overview row.
+    pub fn selected_session_id(&self, index: usize) -> Option<&str> {
+        let window = self.windows.get(index)?;
+        window.session().map(|session| session.session_id.as_str()).or_else(|| match window.pane.current() { Some(super::panes::RowAction::Session(id) | super::panes::RowAction::OpenReady(id)) => Some(id.as_str()), _ => None })
+    }
+
+    pub fn ready_address(&self, index: usize) -> Option<&str> {
+        let session = self.sessions.get(self.selected_session_id(index)?)?;
+        dashboard_cli::browser::ready_url(session.status, session.ready_url.as_deref())
+    }
+
+    fn opening_notice(&self) -> String {
+        let cancel = self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "cancel-open-ready").unwrap_or_else(|| self.text().action_cancel_open_ready.as_str().into());
+        self.text().open_starting.fill(&[("cancel", &cancel)]).into_string()
+    }
+
+    /// 🌐️ Queues a verified address once without starting a process inside event dispatch.
+    pub fn request_open_ready(&mut self, session_id: &str) {
+        if self.opening_url.is_some() || self.effects.iter().any(|effect| matches!(effect, super::windows::Effect::OpenReady(_))) { self.notice = Some(self.opening_notice()); return; }
+        let address = self.sessions.get(session_id).and_then(|session| dashboard_cli::browser::ready_url(session.status, session.ready_url.as_deref())).map(str::to_string);
+        match address {
+            Some(url) => { self.effects.push(super::windows::Effect::OpenReady(url)); self.notice = Some(self.opening_notice()); }
+            None => self.notice = Some(self.text().open_unavailable.as_str().into()),
+        }
+    }
+
+    /// ⏳️ Starts the opener outside the UI loop and reports its completion through the wake queue.
+    pub fn begin_open_ready(&mut self, url: &str) {
+        match dashboard_cli::browser::Job::start(url, self.waker.clone()) {
+            Ok(job) => { self.opening_url = Some(job); self.notice = Some(self.opening_notice()); }
+            Err(error) => self.notice = Some(self.text().open_failed.fill(&[("problem", &error.to_string())]).into_string()),
+        }
+    }
+
+    /// 🛑️ Cancels a queued opening or its owned helper without stopping the task.
+    pub fn cancel_open_ready(&mut self) {
+        self.effects.retain(|effect| !matches!(effect, super::windows::Effect::OpenReady(_)));
+        self.opening_url = None;
+        self.notice = Some(self.text().open_cancelled.as_str().into());
     }
 
@@ -357,4 +398,10 @@

     pub fn poll_background(&mut self, tui: &mut Tui) -> bool {
         let mut changed = false;
+        if let Some(result) = self.opening_url.as_ref().and_then(|job| job.receiver.try_recv().ok()) {
+            self.opening_url = None;
+            self.notice = Some(match result { Ok(()) => self.text().open_finished.as_str().into(), Err(error) => self.text().open_failed.fill(&[("problem", &error.to_string())]).into_string() });
+            self.refresh_panes(tui);
+            changed = true;
+        }
         if let Some((change, result)) = self.saving_preferences.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
             self.saving_preferences = None;

### 19 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust/Cargo.toml
HEAD 44 BASE 39 STASH 40 CURRENT H
--- 

+++ 

@@ -23,4 +23,5 @@

 
 [dependencies]
+dashboard_cli = { path = "../../../🧭️cli/📦️packages/🦀️rust", package = "semio-framework-repo-dashboard-cli" }
 dashboard_core = { path = "../../../🧱️core/📦️packages/🦀️rust", package = "semio-framework-repo-dashboard-core" }
 dashboard_daemon = { path = "../../../🌀️daemon/📦️packages/🦀️rust", package = "semio-framework-repo-dashboard-daemon" }

### 20 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🚀️launcher/🦀️.rs
HEAD 506 BASE 505 STASH 511 CURRENT H
--- 

+++ 

@@ -104,4 +104,5 @@

         let mut groups: Vec<(String, String)> = vec![(format!("verb:{}", entry.verb), verb_text(&entry.verb, locale).to_string())];
         let (leaf, parents) = entry.path.split_last().map_or((entry.verb.clone(), &[][..]), |(last, rest)| (last.label.clone(), rest));
+        let leaf = entry.title.as_ref().map_or(leaf, |title| title.get(locale).to_string());
         groups.extend(parents.iter().map(|segment| (segment.key.clone(), segment.label.clone())));
         let key_of = |depth: usize| groups[..=depth].iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>().join("/");
@@ -210,8 +211,9 @@

         let entry = registry.entries().get(self.entry_of(tree.selected_item()?)?)?;
         if entry.repo_action().is_some() { return Some(entry.id.clone()); }
-        Some(match registry.resolve(&entry.id, &Request::default()) {
+        let command = match registry.resolve(&entry.id, &Request::default()) {
             Ok(launch) => launch.processes.first().map_or_else(|| entry.id.clone(), |process| format!("{} {}", process.cmd, process.args.join(" "))),
             Err(_) => entry.id.clone(),
-        })
+        };
+        Some(entry.description.as_ref().map_or(command.clone(), |description| format!("{} · {command}", description.get(self.locale))))
     }
 
@@ -226,5 +228,6 @@

 
     fn reveal(&mut self) {
-        let line = match (self.stage, self.form.as_ref()) { (Stage::Configure, Some(form)) => Self::slot_lines(form, labels(self.locale), self.locale)[form.focus], _ => 0 };
+        let (line, total) = match (self.stage, self.form.as_ref()) { (Stage::Configure, Some(form)) => { let (lines, focus) = self.form_lines(form, labels(self.locale)); (focus, lines.len()) }, _ => (0, 0) };
+        self.offset = self.offset.min(total.saturating_sub(self.page));
         if line < self.offset { self.offset = line; } else if line >= self.offset + self.page { self.offset = line + 1 - self.page; }
     }
@@ -249,5 +252,5 @@

     pub fn click(&mut self, widget: &mut WidgetState, item: usize) -> Outcome {
         if self.stage == Stage::Confirm {
-            let start = self.form.as_ref().map(|form| Self::slot_lines(form, labels(self.locale), self.locale)[form.start_slot()] + 1);
+            let start = self.form.as_ref().map(|form| self.form_layout(form, labels(self.locale)).1[form.start_slot()] + 1);
             return if start == Some(self.offset + item) { self.apply(widget, Input::Activate) } else { Outcome::Idle };
         }
@@ -255,5 +258,5 @@

         let line = self.offset + item;
         let Some(form) = self.form.as_ref() else { return Outcome::Idle };
-        let slot = Self::slot_lines(form, labels(self.locale), self.locale).iter().position(|candidate| *candidate == line);
+        let slot = self.form_layout(form, labels(self.locale)).1.iter().position(|candidate| *candidate == line);
         let start = form.start_slot();
         match (slot, self.form.as_mut()) {
@@ -394,9 +397,10 @@

                     None => "",
                 };
-                self.window(lines, focus, input, placeholder, caption.to_string())
+                let caption = form.fields.get(form.focus).and_then(|field| field.parameter.description.as_ref()).map_or_else(|| caption.to_string(), |description| format!("{} · {caption}", description.get(self.locale)));
+                self.window(lines, focus, input, placeholder, caption)
             }
             (Stage::Confirm, Some(form)) => {
                 let (mut lines, _) = self.form_lines(form, text);
-                let command = self.preview(form).map_or_else(|_| form.id.clone(), |launch| dashboard_labels::window_title(&launch.label, self.locale));
+                let command = self.registry.as_ref().and_then(|registry| registry.entries().get(form.entry)).and_then(|entry| entry.title.as_ref()).map(|title| title.get(self.locale).to_string()).unwrap_or_else(|| self.preview(form).map_or_else(|_| form.id.clone(), |launch| dashboard_labels::window_title(&launch.label, self.locale)));
                 lines.insert(0, text.confirm_prompt.fill(&[("command", &command)]).into_string());
                 self.window(lines, 0, String::new(), String::new(), text.form_mutating.as_str().to_string())
@@ -433,11 +437,12 @@

             slot if slot == args => (form.extra_args.clone(), text.form_extra_args_prompt.as_str().to_string()),
             slot if slot == env => (form.extra_env.clone(), text.form_extra_env_prompt.as_str().to_string()),
-            slot if slot < form.fields.len() && form.fields[slot].parameter.kind == ParameterKind::Text => (form.fields[slot].value.clone().unwrap_or_default(), form.fields[slot].parameter.id.clone()),
+            slot if slot < form.fields.len() && form.fields[slot].parameter.kind == ParameterKind::Text => (Form::effective(&form.fields[slot]).unwrap_or_default().to_string(), form.fields[slot].parameter.display_title(self.locale).to_string()),
             _ => (String::new(), String::new()),
         }
     }
 
-    fn field_text(field: &Field, text: &DashboardLabels, focused: bool) -> String {
+    fn field_text(field: &Field, text: &DashboardLabels, locale: Locale, focused: bool) -> String {
         let parameter = &field.parameter;
+        let title = parameter.display_title(locale);
         let marker = if focused { "▸ " } else { "  " };
         let required = if parameter.required { format!(" ({})", text.form_required.as_str()) } else { String::new() };
@@ -445,19 +450,19 @@

             ParameterKind::Flag => {
                 let on = Form::effective(field) == Some("true");
-                format!("{marker}[{}] {}{required}  {}", if on { "x" } else { " " }, parameter.id, if on { text.form_on.as_str() } else { text.form_off.as_str() })
-            }
-            ParameterKind::Text => format!("{marker}{}: {}{required}", parameter.id, field.value.clone().unwrap_or_else(|| format!("({})", text.form_unset.as_str()))),
+                format!("{marker}[{}] {}{required}  {}", if on { "x" } else { " " }, title, if on { text.form_on.as_str() } else { text.form_off.as_str() })
+            }
+            ParameterKind::Text => format!("{marker}{}: {}{required}", title, Form::effective(field).map_or_else(|| format!("({})", text.form_unset.as_str()), str::to_string)),
             ParameterKind::Choice => {
                 let shown = Form::effective(field).map_or_else(|| format!("({})", text.form_unset.as_str()), |value| format!("‹{value}›{}", if parameter.default.as_deref() == Some(value) { format!(" ({})", text.form_default.as_str()) } else { String::new() }));
                 let options = parameter.values.iter().take(6).map(|(value, _)| value.as_str()).collect::<Vec<_>>().join(" | ");
-                format!("{marker}{}: {shown}{required}  [{options}{}]", parameter.id, if parameter.values.len() > 6 { " | …" } else { "" })
-            }
-        }
-    }
-
-    fn slot_lines(form: &Form, text: &DashboardLabels, locale: Locale) -> Vec<usize> { Self::layout(form, text, locale, None).1 }
+                format!("{marker}{}: {shown}{required}  [{options}{}]", title, if parameter.values.len() > 6 { " | …" } else { "" })
+            }
+        }
+    }
+
+    fn form_layout(&self, form: &Form, text: &DashboardLabels) -> (Vec<String>, Vec<usize>) { Self::layout(form, text, self.locale, self.registry.as_deref().map(|registry| (registry, self.preview(form)))) }
 
     fn form_lines(&self, form: &Form, text: &DashboardLabels) -> (Vec<String>, usize) {
-        let (lines, slots) = Self::layout(form, text, self.locale, self.registry.as_deref().map(|registry| (registry, self.preview(form))));
+        let (lines, slots) = self.form_layout(form, text);
         (lines, slots.get(form.focus).copied().unwrap_or(0))
     }
@@ -468,9 +473,10 @@

         let marker = |slot: usize| if form.focus == slot { "▸ " } else { "  " };
         let entry: Option<&Entry> = preview.as_ref().and_then(|(registry, _)| registry.entries().get(form.entry));
-        lines.push(entry.map_or_else(|| form.id.clone(), |entry| entry.label.clone()));
+        lines.push(entry.map_or_else(|| form.id.clone(), |entry| entry.display_title(locale).to_string()));
+        if let Some(description) = entry.and_then(|entry| entry.description.as_ref()) { lines.push(description.get(locale).to_string()); }
         lines.push(form.id.clone());
         if form.mutating { lines.push(format!("⚠ {}", text.form_mutating.as_str())); }
         lines.push(if form.fields.is_empty() { text.form_no_parameters.as_str() } else { text.form_parameters.as_str() }.to_string());
-        for (slot, field) in form.fields.iter().enumerate() { slots[slot] = lines.len(); lines.push(Self::field_text(field, text, slot == form.focus)); }
+        for (slot, field) in form.fields.iter().enumerate() { slots[slot] = lines.len(); lines.push(Self::field_text(field, text, locale, slot == form.focus)); }
         slots[form.args_slot()] = lines.len();
         lines.push(format!("{}{}: {}", marker(form.args_slot()), text.form_extra_args.as_str(), form.extra_args));

### 21 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🚀️launcher/🧪️tests/🔬️unit/🦀️.rs
HEAD 119 BASE 119 STASH 189 CURRENT H
--- 

+++ 

@@ -118,2 +118,72 @@

     ]);
 }
+
+#[test]
+fn localized_metadata_is_visible_in_command_search_and_parameter_forms() {
+    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
+    let facts: Facts = serde_json::from_value(fixture["workspace"].clone()).unwrap();
+    let registry = Arc::new(Registry::build(std::path::Path::new("workspace"), &facts));
+    for row in fixture["display"].as_array().unwrap() {
+        let locale = if row["locale"] == "en" { Locale::En } else { Locale::De };
+        let mut state = LauncherState::new(Preferences::default(), locale);
+        state.set_registry(registry.clone());
+        state.set_page(40);
+        let mut harness = Harness { widget: WidgetState::Tree(state.rebuild_tree(None)), state };
+        steps(&mut harness, &[serde_json::json!(format!("type:{}", row["title"].as_str().unwrap()))]);
+        assert_eq!(harness.ids(), ["tool:workspace/source-open"]);
+        assert!(harness.state.rows(harness.tree()).join(" ").contains(row["title"].as_str().unwrap()));
+        assert!(harness.state.caption(harness.tree()).contains(row["description"].as_str().unwrap()));
+        steps(&mut harness, &[serde_json::json!("enter")]);
+        let screen = harness.state.screen();
+        assert!(screen.items.iter().any(|item| item == row["title"].as_str().unwrap()));
+        assert!(screen.items.iter().any(|item| item == row["description"].as_str().unwrap()));
+        assert!(screen.items.iter().any(|item| item.contains(row["parameterTitle"].as_str().unwrap())));
+        assert!(screen.caption.contains(row["parameterDescription"].as_str().unwrap()));
+        assert_eq!(screen.placeholder, row["parameterTitle"].as_str().unwrap());
+        steps(&mut harness, &[serde_json::json!("type:a b.rs")]);
+        let start = harness.state.form.as_ref().unwrap().start().unwrap();
+        assert_eq!(start.id, "tool:workspace/source-open");
+        assert_eq!(start.chosen, [("path".to_string(), "a b.rs".to_string())]);
+    }
+}
+
+#[test]
+fn launcher_viewport_and_default_text_follow_the_shared_vectors() {
+    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
+    for row in fixture["viewportCases"].as_array().unwrap() {
+        let mut harness = launcher();
+        harness.state.set_page(row["initialPage"].as_u64().unwrap() as usize);
+        steps(&mut harness, &[serde_json::json!("type:serve docs"), serde_json::json!("enter")]);
+        harness.state.set_page(row["page"].as_u64().unwrap() as usize);
+        assert_eq!(harness.state.offset, row["expectedTop"].as_u64().unwrap() as usize);
+        assert!(harness.state.screen().items.iter().any(|item| item == "tool:workspace/docs"));
+    }
+    for row in fixture["defaultTextCases"].as_array().unwrap() {
+        let mut harness = launcher();
+        steps(&mut harness, &[serde_json::json!("type:serve docs"), serde_json::json!("enter")]);
+        let form = harness.state.form.as_mut().unwrap();
+        form.fields[0].parameter.default = row["default"].as_str().map(str::to_string);
+        let screen = harness.state.screen();
+        assert_eq!(screen.input, row["expected"].as_str().unwrap());
+        if row["default"].is_string() { assert!(screen.items.iter().any(|item| item.contains(&format!("dir: {}", row["expected"].as_str().unwrap())))); }
+    }
+}
+
+#[test]
+fn launcher_pointer_rows_follow_localized_description_layout() {
+    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
+    let metadata: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
+    let facts: Facts = serde_json::from_value(metadata["workspace"].clone()).unwrap();
+    for row in vectors["pointerCases"].as_array().unwrap() {
+        let locale = if row["locale"] == "en" { Locale::En } else { Locale::De };
+        let mut state = LauncherState::new(Preferences::default(), locale);
+        state.set_registry(Arc::new(Registry::build(std::path::Path::new("workspace"), &facts)));
+        state.set_page(40);
+        let mut harness = Harness { widget:WidgetState::Tree(state.rebuild_tree(None)), state };
+        steps(&mut harness, &[serde_json::json!("type:source-open"), serde_json::json!("enter"), serde_json::json!("down")]);
+        let screen = harness.state.screen();
+        let clicked = screen.items.iter().position(|line| line.contains(row["expectedPlaceholder"].as_str().unwrap())).unwrap();
+        harness.state.click(&mut harness.widget, clicked);
+        assert_eq!(harness.state.screen().placeholder, row["expectedPlaceholder"].as_str().unwrap());
+    }
+}

### 22 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs
HEAD 106 BASE 106 STASH 107 CURRENT H
--- 

+++ 

@@ -69,4 +69,5 @@

     let size = term.size().unwrap_or(Size { width: 100, height: 32 });
     let mut tui = Tui::new(size, Theme::new(if preferences.appearance == "light" { AppearanceName::Light } else { AppearanceName::Dark }));
+    tui.set_width_mode(term.width_mode());
     let mut dash = Dashboard::new(&mut tui, root.clone(), preferences, dashboard_preferences::local_path(&root, parsed), parsed.has_flag("workspace"));
     dash.waker = Some(term.waker());
@@ -89,6 +90,6 @@

         let mut quit = polled.quit;
         for event in &events { if dash.handle(&mut tui, event) == Flow::Quit { quit = true; break; } }
-        for effect in std::mem::take(&mut dash.effects) { match effect { Effect::Copy(text) => { let _ = term.copy(&text); } } }
         if quit || dash.quit_requested { break; }
+        for effect in std::mem::take(&mut dash.effects) { match effect { Effect::Copy(text) => { let _ = term.copy(&text); }, Effect::OpenReady(url) => dash.begin_open_ready(&url) } }
         let expired = dash.expire_prefix(std::time::Instant::now());
         let ticked = tui.tick(now());

### 23 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs
HEAD 910 BASE 910 STASH 1495 CURRENT H
--- 

+++ 

@@ -46,5 +46,532 @@

 fn view() -> (Dashboard, Tui) { dashboard_with(Preferences::default()) }
 
+#[test]
+fn open_ready_routes_keyboard_overview_and_context_menu_using_shared_vectors() {
+    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌐️open/🔣️.json")).unwrap();
+    for row in vectors["tasks"].as_array().unwrap() {
+        let (mut dashboard, mut tui) = view();
+        let status = serde_json::from_value(row["status"].clone()).unwrap();
+        let mut task = session("ready-task", label("dev", "quiz", ""), status, None);
+        task.ready_url = row["url"].as_str().map(str::to_string);
+        dashboard.update_session(&mut tui, task);
+        dashboard.focus_window(&mut tui, 1);
+        press(&mut dashboard, &mut tui, &[ctrl('b'), key('u')]);
+        let expected = if row["accepted"].as_bool().unwrap() { vec![Effect::OpenReady(row["url"].as_str().unwrap().into())] } else { vec![] };
+        assert_eq!(dashboard.effects, expected, "keyboard: {row}");
+        dashboard.effects.clear();
+        dashboard.focus_window(&mut tui, 0);
+        dashboard.refresh_panes(&mut tui);
+        let ready_row = dashboard.windows[0].pane.rows.iter().position(|(_, action)| matches!(action, super::panes::RowAction::OpenReady(_)));
+        assert_eq!(ready_row.is_some(), row["accepted"].as_bool().unwrap());
+        if let Some(selected) = ready_row { dashboard.windows[0].pane.selected = selected; dashboard.activate_row(&mut tui, 0); assert_eq!(dashboard.effects, expected, "overview: {row}"); }
+        dashboard.effects.clear();
+        dashboard.focus_window(&mut tui, 1);
+        dashboard.frame(&mut tui);
+        let rect = tui.scene.rect(dashboard.windows[1].list);
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Right), Pos { x: rect.x + 2, y: rect.y + 2 })]);
+        assert_eq!(dashboard.menu.as_ref().unwrap().actions.contains(&Some(Action::OpenReady)), row["accepted"].as_bool().unwrap());
+        if !expected.is_empty() { press(&mut dashboard, &mut tui, &[named(Key::Down), named(Key::Down), named(Key::Enter)]); assert_eq!(dashboard.effects, expected, "context menu: {row}"); }
+    }
+}
+
+#[test]
+fn open_ready_customization_is_localized_deduplicated_and_cancellable() {
+    let mut preferences = Preferences { language: "de".into(), ..Default::default() };
+    preferences.apply(&Change::parse(r#"{"prefix":"ctrl+a","bindings":{"prefix.open-ready":["g"],"prefix.cancel-open-ready":["G"]}}"#).unwrap());
+    let (mut dashboard, mut tui) = dashboard_with(preferences);
+    let mut task = session("ready-task", label("dev", "quiz", ""), SessionStatus::Running, None);
+    task.ready_url = Some("http://127.0.0.1:6061/".into());
+    dashboard.update_session(&mut tui, task);
+    dashboard.focus_window(&mut tui, 1);
+    assert_eq!(dashboard.text().action("open-ready"), "Bereite Adresse öffnen");
+    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "Ctrl+A g" && hint.label == "Bereite Adresse öffnen"));
+    press(&mut dashboard, &mut tui, &[ctrl('a'), key('g'), ctrl('a'), key('g')]);
+    assert_eq!(dashboard.effects, [Effect::OpenReady("http://127.0.0.1:6061/".into())]);
+    assert!(dashboard.notice.as_ref().unwrap().contains("Ctrl+A G"));
+    press(&mut dashboard, &mut tui, &[ctrl('a'), key('G')]);
+    assert!(dashboard.effects.is_empty());
+    assert_eq!(dashboard.sessions["ready-task"].status, SessionStatus::Running);
+    assert_eq!(dashboard.notice.as_deref(), Some("Öffnen abgebrochen; Aufgabe läuft weiter"));
+}
+
 fn key(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: 0 }) }
+
+fn docking_vectors() -> serde_json::Value { serde_json::from_str(include_str!("../../../🧫️fixtures/🪟️windows/🔣️.json")).unwrap() }
+
+fn docking_placement(value: &str) -> ui_tui::tui::layout::DockPlacement {
+    use ui_tui::tui::layout::DockPlacement::*;
+    use ui_tui::tui::layout::WindowStackCorner::*;
+    match value { "tabs-top-left" => Corner(TopLeft), "tabs-top-right" => Corner(TopRight), "tabs-bottom-left" => Corner(BottomLeft), "tabs-bottom-right" => Corner(BottomRight), "left" => Left, "right" => Right, "top" => Top, "bottom" => Bottom, "tabs" => Tabs, _ => panic!("unknown docking placement") }
+}
+
+fn pointer(kind: MouseKind, pos: Pos) -> Event { Event::Mouse(MouseEvent { kind, pos, mods: 0, clicks: 1 }) }
+
+fn tab_position(dashboard: &Dashboard, tui: &Tui, id: &str) -> Pos {
+    for window in &dashboard.windows {
+        let node = tui.scene.node(window.chrome);
+        let ui_tui::tui::scene::NodeContent::Chrome(chrome) = &node.content else { continue };
+        let ChromeState::Window(state) = chrome else { continue };
+        let Some(index) = state.tab_ids.iter().position(|tab| tab == id) else { continue };
+        let rect = tui.scene.rect(window.chrome);
+        for y in rect.y..rect.y + rect.height {
+            for x in rect.x..rect.x + rect.width {
+                if chrome.window_target(rect, Pos { x, y }) == Some(ui_tui::tui::chrome::WindowHit::Tab(index)) { return Pos { x, y }; }
+            }
+        }
+    }
+    panic!("no visible tab for {id}")
+}
+
+fn docking_ghost(tui: &mut Tui, placement: ui_tui::tui::layout::DockPlacement) -> (ui_tui::tui::geometry::Rect, ui_tui::tui::cell::CellBuffer) {
+    let (actual, rect) = tui.docking_preview().expect("destination tab preview");
+    assert_eq!(actual, placement);
+    assert!(rect.width > 0 && rect.height > 0);
+    tui.render();
+    (rect, tui.frame().clone())
+}
+
+fn assert_committed_ghost(dashboard: &Dashboard, tui: &mut Tui, source: &str, ghost: &(ui_tui::tui::geometry::Rect, ui_tui::tui::cell::CellBuffer)) {
+    use ui_tui::tui::chrome::WindowHit;
+    use ui_tui::tui::geometry::Rect;
+    tui.render();
+    let chrome = dashboard.windows.iter().find(|window| window.id == source).unwrap().chrome;
+    let rect = tui.scene.rect(chrome);
+    let ui_tui::tui::scene::NodeContent::Chrome(state @ ChromeState::Window(window)) = &tui.scene.node(chrome).content else { panic!("window chrome") };
+    let index = window.tab_ids.iter().position(|id| id == source).unwrap();
+    let cells: Vec<_> = (rect.y..rect.y + rect.height).flat_map(|y| (rect.x..rect.x + rect.width).map(move |x| Pos { x, y })).filter(|pos| matches!(state.window_target(rect, *pos), Some(WindowHit::Tab(tab) | WindowHit::Close(tab) | WindowHit::Maximize(tab)) if tab == index)).collect();
+    let left = cells.iter().map(|pos| pos.x).min().unwrap() - 1;
+    let right = cells.iter().map(|pos| pos.x).max().unwrap() + 1;
+    let text_y = cells[0].y;
+    let compact = text_y == rect.y;
+    let expected = Rect::new(left, if compact { text_y } else { text_y - 1 }, right - left + 1, if compact { 1 } else { 3 });
+    assert_eq!(ghost.0, expected, "ghost bounds agree with committed tab hit geometry");
+    for y in expected.y..expected.y + expected.height { for x in expected.x..expected.x + expected.width {
+        assert_eq!(ghost.1.get(x, y).unwrap().ch, tui.frame().get(x, y).unwrap().ch, "committed ghost {source} at {x}:{y}");
+    } }
+}
+
+#[test]
+fn docking_window_handle_tooltips_follow_dashboard_language() {
+    for language in ["en", "de"] {
+        let (dashboard, mut tui) = small_view(language);
+        tui.render();
+        let source = &dashboard.windows[0].id;
+        let grip = tab_position(&dashboard, &tui, source);
+        let node = dashboard.windows[0].chrome;
+        let ui_tui::tui::scene::NodeContent::Chrome(chrome) = &tui.scene.node(node).content else { panic!("chrome") };
+        assert_eq!(dashboard.text().chrome_drag.as_str(), if language == "en" { "Drag window" } else { "Fenster ziehen" });
+        assert_eq!(tui.frame().get(grip.x, grip.y).unwrap().ch, '⠿');
+        assert_eq!(chrome.tooltip(tui.scene.rect(node), grip).as_deref(), Some(dashboard.text().chrome_drag.as_str()));
+        assert_eq!(chrome.window_control_at(tui.scene.rect(node), grip), None);
+    }
+}
+
+#[test]
+fn docking_tabs_reach_all_four_corners_with_preview_and_identity_owned_controls() {
+    use ui_tui::tui::layout::{DockPlacement, WindowStackCorner, stack_hosting};
+    use ui_tui::tui::chrome::WindowHit;
+    let corners = [WindowStackCorner::TopLeft, WindowStackCorner::TopRight, WindowStackCorner::BottomLeft, WindowStackCorner::BottomRight];
+    for corner in corners {
+        let (mut dashboard, mut tui) = view();
+        dashboard.perform(&mut tui, Action::NewTask);
+        dashboard.frame(&mut tui);
+        let source = dashboard.windows[1].id.clone();
+        let root = dashboard.windows[1].root;
+        if corner == WindowStackCorner::TopLeft { ui_tui::tui::layout::dock_window(&mut dashboard.layout, &source, &source, DockPlacement::Corner(WindowStackCorner::BottomRight), None); dashboard.remount(&mut tui); dashboard.frame(&mut tui); }
+        let start = tab_position(&dashboard, &tui, &source);
+        let area = tui.scene.rect(dashboard.windows[1].chrome);
+        let drop = Pos { x: if corner.is_left() { area.x + 1 } else { area.x + area.width - 2 }, y: if corner.is_top() { area.y + 1 } else { area.y + area.height - 2 } };
+        let before = dashboard.layout.clone();
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(corner));
+        let preview = ghost.0;
+        assert_eq!(dashboard.layout, before);
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
+        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
+        let stack = stack_hosting(&dashboard.layout, &source).unwrap();
+        let index = stack.children.iter().position(|node| node.window_kind_id == source).unwrap();
+        assert_eq!(stack.children[index].corner, Some(corner));
+        assert_eq!(dashboard.windows[1].root, root);
+        let node = dashboard.windows[1].chrome;
+        let rect = tui.scene.rect(node);
+        let ui_tui::tui::scene::NodeContent::Chrome(chrome) = &tui.scene.node(node).content else { panic!("chrome") };
+        let mut controls = Vec::new();
+        for y in rect.y..rect.y + rect.height { for x in rect.x..rect.x + rect.width {
+            if matches!(chrome.window_target(rect, Pos { x, y }), Some(WindowHit::Maximize(tab) | WindowHit::Close(tab)) if tab == index) { controls.push(Pos { x, y }); }
+        }}
+        assert_eq!(controls.len(), 2);
+        println!("[DEBUG] corner={corner:?} source={source} preview={preview:?} tab-controls={controls:?}");
+    }
+}
+
+#[test]
+fn docking_interleaved_layout_events_keep_the_held_tab_identity() {
+    use ui_tui::tui::layout::stack_hosting;
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::NewTask);
+    dashboard.perform(&mut tui, Action::NewTask);
+    dashboard.frame(&mut tui);
+    let source = dashboard.windows[1].id.clone();
+    let target = dashboard.windows[0].id.clone();
+    let other = dashboard.windows[2].id.clone();
+    let body = dashboard.windows[1].root;
+    let start = tab_position(&dashboard, &tui, &source);
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
+    dashboard.perform(&mut tui, Action::TabRight);
+    dashboard.frame(&mut tui);
+    let drop = tab_position(&dashboard, &tui, &target);
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), drop), pointer(MouseKind::Up(MouseButton::Left), drop)]);
+    let group = stack_hosting(&dashboard.layout, &source).unwrap();
+    assert_eq!(group.children.iter().map(|node| node.window_kind_id.as_str()).collect::<Vec<_>>(), vec![source.as_str(), target.as_str(), other.as_str()]);
+    assert_eq!(dashboard.windows[1].root, body);
+    println!("[DEBUG] a layout event during pointer capture retained held identity={source} and its body");
+}
+
+#[test]
+fn docking_corner_pointer_moves_between_groups_and_orders_within_the_chosen_corner() {
+    use ui_tui::tui::layout::{DockPlacement, WindowStackCorner::*, dock_window, stack_hosting};
+    for corner in [TopLeft, TopRight, BottomLeft, BottomRight] {
+        let (mut dashboard, mut tui) = view();
+        dashboard.perform(&mut tui, Action::SplitRight);
+        dashboard.frame(&mut tui);
+        let source = dashboard.windows[1].id.clone();
+        let target = dashboard.windows[0].id.clone();
+        let body = dashboard.windows[1].root;
+        let start = tab_position(&dashboard, &tui, &source);
+        let area = tui.scene.rect(dashboard.windows[0].chrome);
+        let drop = Pos { x: if corner.is_left() { area.x + 1 } else { area.x + area.width - 2 }, y: if corner.is_top() { area.y + 1 } else { area.y + area.height - 2 } };
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(corner));
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
+        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
+        let group = stack_hosting(&dashboard.layout, &target).unwrap();
+        assert_eq!(group.children.len(), 2);
+        assert_eq!(group.children.iter().find(|node| node.window_kind_id == source).unwrap().corner, Some(corner));
+        assert_eq!(dashboard.windows[1].root, body);
+        assert_eq!(dashboard.focused_index(&tui), 1);
+        dashboard.perform(&mut tui, Action::NewTask);
+        let other = dashboard.windows[2].id.clone();
+        assert!(dock_window(&mut dashboard.layout, &other, &target, DockPlacement::Corner(corner), Some(0)));
+        assert!(dock_window(&mut dashboard.layout, &source, &target, DockPlacement::Corner(corner), Some(0)));
+        let group = stack_hosting(&dashboard.layout, &target).unwrap();
+        let order: Vec<_> = group.children.iter().filter(|node| node.corner.unwrap_or_default() == corner).map(|node| node.window_kind_id.as_str()).collect();
+        assert_eq!(&order[..2], &[source.as_str(), other.as_str()]);
+        println!("[DEBUG] cross-group corner={corner:?} retained body and focus; corner-local order={order:?}");
+    }
+}
+
+#[test]
+fn docking_corner_contract_covers_cross_group_and_sole_tab_moves() {
+    use ui_tui::tui::layout::{create_default_layout, dock_window, stack_hosting, DockPlacement};
+    use ui_tui::tui::geometry::Rect;
+    let vectors = docking_vectors();
+    let area = Rect::new(10, 4, 80, 40);
+    for vector in vectors["corners"].as_array().unwrap() {
+        let placement = docking_placement(vector["placement"].as_str().unwrap());
+        let preview = placement.preview(area);
+        assert_eq!(serde_json::json!([preview.x, preview.y, preview.width, preview.height]), vector["rect"]);
+        assert_eq!(DockPlacement::at(area, Pos { x: vector["point"][0].as_u64().unwrap() as u16, y: vector["point"][1].as_u64().unwrap() as u16 }), Some(placement));
+        let DockPlacement::Corner(corner) = placement else { panic!("corner") };
+        for ids in [vec!["a".into()], vec!["a".into(), "b".into()]] {
+            let mut layout = create_default_layout(&ids, "row", None, None);
+            let target = ids.last().unwrap();
+            assert!(dock_window(&mut layout, "a", target, placement, Some(0)));
+            let stack = stack_hosting(&layout, "a").unwrap();
+            assert_eq!(stack.children.iter().find(|node| node.window_kind_id == "a").unwrap().corner, Some(corner));
+            assert_eq!(stack.children.len(), ids.len());
+        }
+    }
+}
+
+#[test]
+fn keyboard_docking_places_tabs_in_all_corners_and_localizes_every_action() {
+    use ui_tui::tui::layout::{stack_hosting, WindowStackCorner::*};
+    for (action, corner) in [(Action::TabTopLeft, TopLeft), (Action::TabTopRight, TopRight), (Action::TabBottomLeft, BottomLeft), (Action::TabBottomRight, BottomRight)] {
+        for language in ["en", "de"] {
+            let (mut dashboard, mut tui) = small_view(language);
+            let id = dashboard.windows[0].id.clone();
+            assert_eq!(Action::parse(action.id()), Some(action));
+            assert!(!dashboard.text().action(action.id()).is_empty());
+            assert!(dashboard.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, action.id()).is_some());
+            let digit = match action { Action::TabTopLeft => '1', Action::TabTopRight => '2', Action::TabBottomLeft => '3', _ => '4' };
+            press(&mut dashboard, &mut tui, &[ctrl('b'), Event::Key(KeyEvent { key: Key::Char(digit), mods: mods::ALT })]);
+            assert_eq!(stack_hosting(&dashboard.layout, &id).unwrap().children[0].corner, Some(corner));
+            assert_eq!(dashboard.focused_index(&tui), 0);
+        }
+    }
+}
+
+#[test]
+fn docking_fullscreen_and_close_on_an_inactive_tab_target_its_identity() {
+    use ui_tui::tui::chrome::WindowHit;
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::NewTask);
+    dashboard.focus_window(&mut tui, 0);
+    dashboard.frame(&mut tui);
+    let source = dashboard.windows[1].id.clone();
+    let find = |dashboard: &Dashboard, tui: &Tui, wanted: WindowHit| {
+        let chrome = dashboard.windows[dashboard.focused_index(tui)].chrome;
+        let rect = tui.scene.rect(chrome);
+        let ui_tui::tui::scene::NodeContent::Chrome(state) = &tui.scene.node(chrome).content else { panic!("chrome") };
+        (rect.y..rect.y + rect.height).flat_map(|y| (rect.x..rect.x + rect.width).map(move |x| Pos { x, y })).find(|pos| state.window_target(rect, *pos) == Some(wanted)).unwrap()
+    };
+    let maximize = find(&dashboard, &tui, WindowHit::Maximize(1));
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), maximize)]);
+    assert_eq!(dashboard.layout.zoomed.as_deref(), Some(source.as_str()));
+    assert_eq!(dashboard.focused_index(&tui), 1);
+    for index in [0, 1] {
+        let tab = find(&dashboard, &tui, WindowHit::Tab(index));
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), tab)]);
+        assert_eq!(dashboard.layout.zoomed.as_deref(), Some(dashboard.windows[index].id.as_str()));
+        assert_eq!(dashboard.focused_index(&tui), index);
+    }
+    let restore = find(&dashboard, &tui, WindowHit::Maximize(0));
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), restore)]);
+    assert!(dashboard.layout.zoomed.is_none());
+    assert_eq!(dashboard.focused_index(&tui), 0);
+    dashboard.focus_window(&mut tui, 0);
+    dashboard.frame(&mut tui);
+    let close = find(&dashboard, &tui, WindowHit::Close(1));
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), close)]);
+    assert!(!dashboard.windows.iter().any(|window| window.id == source));
+    assert_eq!(dashboard.focused_index(&tui), 0);
+    dashboard.perform(&mut tui, Action::NewTask);
+    dashboard.perform(&mut tui, Action::NewTask);
+    let retained = dashboard.windows[2].id.clone();
+    dashboard.perform(&mut tui, Action::Zoom);
+    dashboard.frame(&mut tui);
+    for remaining in [2, 1] {
+        let close = find(&dashboard, &tui, WindowHit::Close(0));
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), close)]);
+        assert_eq!(dashboard.windows.len(), remaining);
+        assert_eq!(dashboard.windows[dashboard.focused_index(&tui)].id, retained);
+        assert_eq!(dashboard.layout.zoomed.as_deref(), (remaining > 1).then_some(retained.as_str()));
+    }
+    dashboard.perform(&mut tui, Action::Zoom);
+    assert!(dashboard.layout.zoomed.is_none(), "a lone window has no fullscreen action");
+    println!("[DEBUG] inactive tab fullscreen activated its identity; inactive close retained active focus and fullscreen until only one tab remained");
+}
+
+#[test]
+fn docking_preview_matches_the_committed_split_geometry_in_every_zone() {
+    use ui_tui::tui::geometry::Rect;
+    use ui_tui::tui::layout::{create_default_layout, dock_window, push_window_to_stack, solve_window_layout, DockPlacement, WindowLayoutWindowNode};
+    let vectors = docking_vectors();
+    let area = Rect::new(10, 4, 80, 40);
+    for vector in vectors["previews"].as_array().unwrap() {
+        let placement = docking_placement(vector["placement"].as_str().unwrap());
+        if let Some(point) = vector.get("point") {
+            assert_eq!(DockPlacement::at(area, Pos { x: point[0].as_u64().unwrap() as u16, y: point[1].as_u64().unwrap() as u16 }), Some(placement));
+        }
+        let preview = placement.preview(area);
+        assert_eq!(serde_json::json!([preview.x, preview.y, preview.width, preview.height]), vector["rect"]);
+        let mut layout = create_default_layout(&["b".into()], "row", None, None);
+        push_window_to_stack(&mut layout, "b", WindowLayoutWindowNode { window_kind_id: "a".into(), title: Some("Held task".into()), corner: None });
+        assert!(dock_window(&mut layout, "a", "b", placement, None));
+        let measures = solve_window_layout(&layout, area);
+        let held = measures.iter().find(|measure| measure.window_kind_id == "a").unwrap();
+        assert_eq!(held.rect, preview);
+        assert_eq!(held.title.as_deref(), Some("Held task"));
+    }
+    for vector in vectors["bodyZones"].as_array().unwrap() {
+        assert_eq!(DockPlacement::at(area, Pos { x: vector["point"][0].as_u64().unwrap() as u16, y: vector["point"][1].as_u64().unwrap() as u16 }), Some(docking_placement(vector["placement"].as_str().unwrap())));
+    }
+    assert_eq!(DockPlacement::at(area, Pos { x: 0, y: 0 }), None);
+}
+
+#[test]
+fn docking_commands_preserve_windows_prune_empty_groups_and_reject_invalid_targets() {
+    use ui_tui::tui::layout::{create_default_layout, dock_window, push_window_to_stack, stack_hosting, WindowLayoutRoot, WindowLayoutWindowNode};
+    let vectors = docking_vectors();
+    let mut layout = create_default_layout(&["a".into()], "row", None, None);
+    for id in ["b", "c"] { push_window_to_stack(&mut layout, "a", WindowLayoutWindowNode { window_kind_id: id.into(), title: Some(id.into()), corner: None }); }
+    for command in vectors["commands"].as_array().unwrap() {
+        assert!(dock_window(&mut layout, command["source"].as_str().unwrap(), command["target"].as_str().unwrap(), docking_placement(command["placement"].as_str().unwrap()), command["tabIndex"].as_u64().map(|value| value as usize)));
+    }
+    assert!(matches!(layout.root, WindowLayoutRoot::Stack(_)));
+    let group = stack_hosting(&layout, "b").unwrap();
+    assert_eq!(serde_json::json!(group.children.iter().map(|node| node.window_kind_id.clone()).collect::<Vec<_>>()), vectors["finalTabs"]);
+    for command in vectors["invalid"].as_array().unwrap() {
+        let before = layout.clone();
+        assert!(!dock_window(&mut layout, command["source"].as_str().unwrap(), command["target"].as_str().unwrap(), docking_placement(command["placement"].as_str().unwrap()), None));
+        assert_eq!(layout, before);
+    }
+}
+
+#[test]
+fn real_pointer_drags_preview_every_split_then_join_tabs_without_recreating_their_bodies() {
+    use ui_tui::tui::layout::{solve_window_layout, DockPlacement};
+    for placement in [DockPlacement::Left, DockPlacement::Right, DockPlacement::Top, DockPlacement::Bottom] {
+        let (mut dashboard, mut tui) = view();
+        dashboard.perform(&mut tui, Action::NewTask);
+        dashboard.frame(&mut tui);
+        let source = dashboard.windows[1].id.clone();
+        let root = dashboard.windows[1].root;
+        let before = dashboard.layout.clone();
+        let start = tab_position(&dashboard, &tui, &source);
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
+        let area = tui.scene.rect(dashboard.windows[1].chrome);
+        let center = Pos { x: area.x + area.width / 2, y: area.y + area.height / 2 };
+        let drop = match placement { DockPlacement::Left => Pos { x: area.x + 1, ..center }, DockPlacement::Right => Pos { x: area.x + area.width - 2, ..center }, DockPlacement::Top => Pos { y: area.y + 4, ..center }, DockPlacement::Bottom => Pos { y: area.y + area.height - 2, ..center }, _ => center };
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+        let ghost = docking_ghost(&mut tui, placement);
+        assert_eq!(dashboard.layout, before, "preview must not change the layout");
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
+        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
+        assert!(tui.docking_preview().is_none());
+        assert_eq!(dashboard.windows[1].root, root);
+        assert_eq!(dashboard.focused_index(&tui), 1);
+        let measure = solve_window_layout(&dashboard.layout, area).into_iter().find(|window| window.window_kind_id == source).unwrap();
+        assert_eq!(measure.rect, placement.preview(area));
+        let start = tab_position(&dashboard, &tui, &source);
+        let target = tui.scene.rect(dashboard.windows[0].chrome);
+        let center = Pos { x: target.x + 2, y: target.y + 1 };
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), center)]);
+        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(ui_tui::tui::layout::WindowStackCorner::TopLeft));
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), center)]);
+        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
+        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 2);
+        assert_eq!(tui.scene.rect(dashboard.windows[1].chrome), area);
+        assert_eq!(dashboard.windows[1].root, root);
+    }
+}
+
+#[test]
+fn escape_focus_loss_resize_and_an_outside_release_cancel_docking() {
+    for cancel in [named(Key::Esc), Event::FocusLost, Event::Resize(Size { width: 120, height: 40 }), pointer(MouseKind::Up(MouseButton::Left), Pos { x: 0, y: 0 })] {
+        let (mut dashboard, mut tui) = view();
+        dashboard.perform(&mut tui, Action::NewTask);
+        dashboard.frame(&mut tui);
+        let before = dashboard.layout.clone();
+        let source = dashboard.windows[1].id.clone();
+        let start = tab_position(&dashboard, &tui, &source);
+        let area = tui.scene.rect(dashboard.windows[1].chrome);
+        let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+        assert!(tui.docking_preview().is_some());
+        press(&mut dashboard, &mut tui, &[cancel, pointer(MouseKind::Up(MouseButton::Left), drop)]);
+        assert!(tui.docking_preview().is_none());
+        assert_eq!(dashboard.layout, before);
+    }
+}
+
+#[test]
+fn keyboard_docking_and_tab_reordering_have_localized_customizable_actions() {
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::NewTask);
+    let source = dashboard.windows[1].id.clone();
+    for action in [Action::DockLeft, Action::DockRight, Action::DockTop, Action::DockBottom] {
+        dashboard.perform(&mut tui, action);
+        dashboard.frame(&mut tui);
+        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 1);
+        dashboard.perform(&mut tui, Action::JoinWindow);
+        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 2);
+    }
+    dashboard.perform(&mut tui, Action::TabLeft);
+    assert_eq!(super::windows::stack_of(&dashboard.layout, &source)[0], source);
+    dashboard.perform(&mut tui, Action::TabRight);
+    assert_eq!(super::windows::stack_of(&dashboard.layout, &source)[1], source);
+    for action in [Action::DockLeft, Action::DockRight, Action::DockTop, Action::DockBottom, Action::JoinWindow, Action::TabLeft, Action::TabRight] {
+        assert!(super::labels::labels(ui_locale::Locale::parse("de").unwrap()).try_action(action.id()).is_some());
+    }
+}
+
+#[test]
+fn every_window_docking_scenario_has_a_native_test() { every_scenario_of_the_window_view_features_is_proved_by_a_test(); }
+
+#[test]
+fn docking_a_live_terminal_preserves_its_session_output_and_reports_the_new_pty_size() {
+    let (mut dashboard, mut tui) = view();
+    dashboard.update_session(&mut tui, session("held-task", label("test", "quiz", ""), SessionStatus::Running, None));
+    dashboard.focus_window(&mut tui, 1);
+    dashboard.feed(&mut tui, 1, "HELD-OUTPUT");
+    dashboard.frame(&mut tui);
+    let terminal = dashboard.windows[1].list;
+    let source = dashboard.windows[1].id.clone();
+    let start = tab_position(&dashboard, &tui, &source);
+    let area = tui.scene.rect(dashboard.windows[1].chrome);
+    let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
+    let before = dashboard.link.outbox.len();
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+    assert_eq!(dashboard.link.outbox.len(), before, "preview cannot resize the child");
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
+    assert_eq!(dashboard.windows[1].list, terminal);
+    assert_eq!(dashboard.windows[1].session().unwrap().session_id, "held-task");
+    let rect = tui.scene.rect(terminal);
+    assert!(dashboard.link.outbox[before..].iter().any(|message| matches!(message, ClientMsg::Resize { session_id, cols, rows } if session_id == "held-task" && (*cols, *rows) == (rect.width, rect.height))));
+    assert!(!dashboard.link.outbox[before..].iter().any(|message| matches!(message, ClientMsg::SpawnGroup { .. } | ClientMsg::Stop { .. } | ClientMsg::Kill { .. })));
+    if let Some(ui_tui::tui::widget::WidgetState::Terminal(state)) = tui.scene.node_mut(terminal).widget() {
+        let line: String = (0..11).filter_map(|x| state.screen.cell_at(x, 0).map(|cell| cell.ch)).collect();
+        assert_eq!(line, "HELD-OUTPUT");
+    } else { panic!("the same terminal survives docking"); }
+}
+
+#[test]
+fn docking_survives_zoom_and_removing_the_held_window_cancels_its_preview() {
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::NewTask);
+    dashboard.perform(&mut tui, Action::Zoom);
+    dashboard.frame(&mut tui);
+    let source = dashboard.windows[1].id.clone();
+    let start = tab_position(&dashboard, &tui, &source);
+    let area = tui.scene.rect(dashboard.windows[1].chrome);
+    let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop), pointer(MouseKind::Up(MouseButton::Left), drop)]);
+    assert!(dashboard.layout.zoomed.is_none());
+    dashboard.perform(&mut tui, Action::JoinWindow);
+    dashboard.frame(&mut tui);
+    let start = tab_position(&dashboard, &tui, &source);
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
+    assert!(tui.docking_preview().is_some());
+    assert!(!dashboard.close_window(&mut tui, 1));
+    dashboard.frame(&mut tui);
+    assert!(tui.docking_preview().is_none());
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
+    assert_eq!(dashboard.windows.len(), 1);
+}
+
+#[test]
+fn nested_splits_preserve_the_weight_of_their_existing_sibling_slot() {
+    use ui_tui::tui::geometry::Rect;
+    use ui_tui::tui::layout::{create_default_layout, solve_window_layout, split_window};
+    let mut layout = create_default_layout(&["a".into(), "b".into()], "row", Some(&[4.0, 1.0]), None);
+    let area = Rect::new(0, 0, 160, 48);
+    let before = solve_window_layout(&layout, area).into_iter().find(|window| window.window_kind_id == "b").unwrap().rect;
+    assert!(split_window(&mut layout, "a", "column", "c", None));
+    let after = solve_window_layout(&layout, area).into_iter().find(|window| window.window_kind_id == "b").unwrap().rect;
+    assert_eq!(before, after);
+}
+
+#[test]
+fn docking_changes_keyboard_focus_order_to_match_the_visible_layout() {
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::NewTask);
+    let first = dashboard.windows[0].id.clone();
+    let second = dashboard.windows[1].id.clone();
+    dashboard.perform(&mut tui, Action::DockLeft);
+    assert_eq!(dashboard.window_order(), [second.clone(), first.clone()]);
+    dashboard.cycle_window(&mut tui, 1);
+    assert_eq!(dashboard.focused(&tui).unwrap().id, first);
+    dashboard.cycle_window(&mut tui, 1);
+    assert_eq!(dashboard.focused(&tui).unwrap().id, second);
+}
+
+#[test]
+fn splitter_drag_keeps_its_capture_across_each_layout_remount() {
+    let (mut dashboard, mut tui) = view();
+    dashboard.perform(&mut tui, Action::SplitRight);
+    dashboard.frame(&mut tui);
+    let left = tui.scene.rect(dashboard.windows[0].chrome);
+    let start = Pos { x: left.x + left.width, y: left.y + left.height / 2 };
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
+    for delta in [3, 7, 5] {
+        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), Pos { x: start.x + delta, ..start })]);
+        assert_eq!(tui.scene.rect(dashboard.windows[0].chrome).width, left.width + delta, "capture must survive remount at delta {delta}");
+    }
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), Pos { x: start.x + 5, ..start })]);
+    let width = tui.scene.rect(dashboard.windows[0].chrome).width;
+    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Move, Pos { x: start.x + 20, ..start })]);
+    assert_eq!(tui.scene.rect(dashboard.windows[0].chrome).width, width);
+}
 fn ctrl(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: mods::CTRL }) }
 fn named(key: Key) -> Event { Event::Key(KeyEvent { key, mods: 0 }) }
@@ -797,4 +1324,13 @@

         ("A compound opens one tab per member", &["a_compound_opens_one_tab_per_member_each_with_its_own_task_label"]),
         ("An unchanged screen is neither dirty nor repainted", &["p1_7_an_unchanged_screen_is_neither_dirty_nor_repainted_and_nothing_renders_in_full", "repainting_is_incremental_and_follows_typing"]),
+        ("Every docking zone previews the committed geometry", &["docking_preview_matches_the_committed_split_geometry_in_every_zone"]),
+        ("Docking preserves identities and collapses empty groups", &["docking_commands_preserve_windows_prune_empty_groups_and_reject_invalid_targets", "nested_splits_preserve_the_weight_of_their_existing_sibling_slot"]),
+        ("Pointer docking preserves window bodies and focus", &["real_pointer_drags_preview_every_split_then_join_tabs_without_recreating_their_bodies", "docking_a_live_terminal_preserves_its_session_output_and_reports_the_new_pty_size"]),
+        ("Interrupted docking leaves the layout unchanged", &["escape_focus_loss_resize_and_an_outside_release_cancel_docking", "docking_survives_zoom_and_removing_the_held_window_cancels_its_preview"]),
+        ("Keyboard users can dock, join and reorder tabs", &["keyboard_docking_and_tab_reordering_have_localized_customizable_actions", "docking_changes_keyboard_focus_order_to_match_the_visible_layout"]),
+        ("Tabs dock into every corner with stable identities", &["docking_tabs_reach_all_four_corners_with_preview_and_identity_owned_controls", "docking_corner_contract_covers_cross_group_and_sole_tab_moves", "docking_interleaved_layout_events_keep_the_held_tab_identity"]),
+        ("Every tab owns its fullscreen and close controls", &["docking_fullscreen_and_close_on_an_inactive_tab_target_its_identity"]),
+        ("Keyboard users can place tabs in every corner", &["keyboard_docking_places_tabs_in_all_corners_and_localizes_every_action"]),
+        ("Divider dragging continues across layout frames", &["splitter_drag_keeps_its_capture_across_each_layout_remount"]),
     ]);
     dashboard_core::testkit::assert_proved(include_str!("../../../🧪️tests/📡️sessions/🥒️.feature"), &[source], &[
@@ -909,2 +1445,51 @@

     assert!(dashboard.terminal_has_keyboard(&tui));
 }
+
+#[test]
+fn launcher_batched_navigation_edits_the_new_field_before_any_frame() {
+    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
+    for row in fixture["cases"].as_array().unwrap() {
+        let (mut dashboard, mut tui) = view();
+        press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
+        press(&mut dashboard, &mut tui, &type_text("serve docs"));
+        let index = dashboard.focused_index(&tui);
+        for event in row["events"].as_array().unwrap() {
+            let text = event.as_str().unwrap();
+            let event = match text { "enter" => named(Key::Enter), "down" => named(Key::Down), _ => if let Some(value) = text.strip_prefix("char:") { key(value.chars().next().unwrap()) } else { Event::Paste(text.strip_prefix("paste:").unwrap().into()) } };
+            dashboard.handle(&mut tui, &event);
+        }
+        let input = dashboard.windows[index].input.unwrap();
+        let ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::Input(state)) = &tui.scene.node(input).content else { panic!("editor expected") };
+        assert_eq!(state.value, row["expected"].as_str().unwrap(), "{}", row["name"]);
+        assert_eq!(tui.focus(), Some(input));
+    }
+}
+
+#[test]
+fn launcher_pointer_focus_is_synchronized_before_the_next_character() {
+    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
+    let metadata: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
+    let facts: Facts = serde_json::from_value(metadata["workspace"].clone()).unwrap();
+    for row in vectors["pointerCases"].as_array().unwrap() {
+        let (mut dashboard, mut tui) = dashboard_with(Preferences { language:row["locale"].as_str().unwrap().into(), ..Default::default() });
+        dashboard.set_registry(&mut tui, Arc::new(Registry::build(std::path::Path::new("workspace"), &facts)));
+        press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
+        press(&mut dashboard, &mut tui, &type_text("source-open"));
+        press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
+        press(&mut dashboard, &mut tui, &type_text(row["initialParameter"].as_str().unwrap()));
+        press(&mut dashboard, &mut tui, &[named(Key::Down)]);
+        press(&mut dashboard, &mut tui, &type_text(row["initialArgs"].as_str().unwrap()));
+        tui.render();
+        let index = dashboard.focused_index(&tui);
+        let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
+        let clicked = launcher.screen().items.iter().position(|line| line.contains(row["expectedPlaceholder"].as_str().unwrap())).unwrap();
+        let rectangle = tui.scene.rect(dashboard.windows[index].list);
+        let click = pointer(MouseKind::Down(MouseButton::Left), Pos { x:rectangle.x+2, y:rectangle.y+clicked as u16 });
+        dashboard.handle(&mut tui, &click);
+        for event in type_text(row["typed"].as_str().unwrap()) { dashboard.handle(&mut tui, &event); }
+        let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
+        let screen = launcher.screen();
+        assert_eq!(screen.placeholder, row["expectedPlaceholder"].as_str().unwrap());
+        assert_eq!(screen.input, row["expectedText"].as_str().unwrap());
+    }
+}

### 24 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs
HEAD 450 BASE 449 STASH 457 CURRENT H
--- 

+++ 

@@ -86,5 +86,5 @@

 /// 🌬️ Something the view asks of the terminal it runs in, collected so tests need no terminal.
 #[derive(Clone, Debug, PartialEq, Eq)]
-pub(super) enum Effect { Copy(String) }
+pub(super) enum Effect { Copy(String), OpenReady(String) }
 // #endregion 🔖️Window
 
@@ -101,4 +101,5 @@

     pub shared_preferences: bool,
     pub saving_preferences: Option<std::sync::mpsc::Receiver<(Change, std::io::Result<u64>)>>,
+    pub opening_url: Option<dashboard_cli::browser::Job>,
     pub keymap: Keymap,
     pub locale: Locale,
@@ -140,5 +141,5 @@

         let light = preferences.appearance == "light";
         let mut dashboard = Self {
-            root, registry: None, inventory: None, phase: None, last_progress: 0, preferences, preference_path, shared_preferences, saving_preferences: None, keymap, locale, light, layout, shell, windows: Vec::new(), next_serial: 2, armed: false, armed_at: None, input_mode: false, restore_focus: false,
+            root, registry: None, inventory: None, phase: None, last_progress: 0, preferences, preference_path, shared_preferences, saving_preferences: None, opening_url: None, keymap, locale, light, layout, shell, windows: Vec::new(), next_serial: 2, armed: false, armed_at: None, input_mode: false, restore_focus: false,
             link: Link::default(), state: LinkState::Connecting, notice: None, sessions: BTreeMap::new(), hidden: HashSet::new(), shutdown_since: None, effects: Vec::new(), quit_requested: false, last_focus: std::cell::Cell::new(0), menu: None, glyphs: GlyphSet::Unicode, waker: None,
         };
@@ -276,11 +277,13 @@

     pub fn close_window(&mut self, tui: &mut Tui, index: usize) -> bool {
         if index >= self.windows.len() { return false; }
+        let focused = self.windows.get(self.focused_index(tui)).map(|window| window.id.clone());
+        tui.cancel_pointer();
         let window = self.windows.remove(index);
         if let Some(session) = window.session() { self.hidden.insert(session.session_id.clone()); }
         tui.scene.remove(window.chrome);
         remove_window(&mut self.layout, &window.id);
-        if self.layout.zoomed.as_deref() == Some(window.id.as_str()) { self.layout.zoomed = None; }
+        if self.windows.len() < 2 || self.layout.zoomed.as_deref() == Some(window.id.as_str()) { self.layout.zoomed = None; }
         if self.windows.is_empty() { return true; }
-        let next = index.saturating_sub(1).min(self.windows.len() - 1);
+        let next = focused.filter(|id| id != &window.id).and_then(|id| self.windows.iter().position(|window| window.id == id)).unwrap_or_else(|| index.saturating_sub(1).min(self.windows.len() - 1));
         self.focus_window(tui, next);
         false
@@ -301,8 +304,13 @@

     pub fn cycle_window(&mut self, tui: &mut Tui, delta: isize) {
         if self.windows.len() < 2 { return; }
-        let count = self.windows.len() as isize;
-        let next = (self.focused_index(tui) as isize + delta).rem_euclid(count) as usize;
-        self.focus_window(tui, next);
-    }
+        let current = self.windows[self.focused_index(tui)].id.clone();
+        let order = self.window_order();
+        let Some(from) = order.iter().position(|id| id == &current) else { return };
+        let next = (from as isize + delta).rem_euclid(order.len() as isize) as usize;
+        if let Some(index) = self.window_index(&order[next]) { self.focus_window(tui, index); }
+    }
+
+    /// 🧭️ The current visual order of groups and their tabs, independent of creation order.
+    pub fn window_order(&mut self) -> Vec<String> { layout_windows(&mut self.layout).into_iter().map(|node| node.window_kind_id.clone()).collect() }
 }
 // #endregion 🔖️Bodies
@@ -435,5 +443,5 @@

     pub fn sync_chrome(&mut self, tui: &mut Tui) {
         let text = self.text();
-        let labels = ChromeLabels { close: text.chrome_close.as_str().into(), maximize: text.chrome_maximize.as_str().into(), restore: text.chrome_restore.as_str().into(), new_tab: text.chrome_new_tab.as_str().into(), previous_tabs: text.chrome_previous_tabs.as_str().into(), next_tabs: text.chrome_next_tabs.as_str().into() };
+        let labels = ChromeLabels { drag: text.chrome_drag.as_str().into(), close: text.chrome_close.as_str().into(), maximize: text.chrome_maximize.as_str().into(), restore: text.chrome_restore.as_str().into(), new_tab: text.chrome_new_tab.as_str().into(), previous_tabs: text.chrome_previous_tabs.as_str().into(), next_tabs: text.chrome_next_tabs.as_str().into() };
         for window in &self.windows {
             let current = match &tui.scene.node(window.chrome).content { NodeContent::Chrome(ChromeState::Window(chrome)) => Some(chrome.labels == labels && chrome.new_tab), _ => None };

### 25 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts
HEAD 131 BASE 130 STASH 137 CURRENT H
--- 

+++ 

@@ -109,4 +109,11 @@

 });
 
+test("Node environment projection independently isolates task process authority", () => {
+  const boundary = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🌀️control-plane/🔣️.json", import.meta.url), "utf8")).environmentBoundary;
+  const client = Object.fromEntries(boundary.client.filter(([key]:string[]) => !boundary.discarded.includes(key)));
+  const projected = { ...client, ...Object.fromEntries(boundary.declared), TERM:"xterm-256color", COLORTERM:"truecolor" };
+  expect(Object.entries(projected)).toEqual(boundary.expected);
+});
+
 test("every independent oracle of this file proves a named scenario of its feature", () => {
   const own = readFileSync(new URL(import.meta.url), "utf8");

### 26 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts
HEAD 483 BASE 484 STASH 486 CURRENT H
--- 

+++ 

@@ -86,4 +86,6 @@

   const project = (dashboardBlock: Json) => ({ metadata: { semio: { dashboard: dashboardBlock } } });
   const refused: Json[] = [
+    project({ tools: [{ id: "server", command: ["bun", "--version"], mcp: { server: "server", clients: { vscode: {} } } }] }),
+    project({ tools: [{ id: "server", command: ["bun", "--version"], mcp: { server: "server", clients: { cursor: {} } } }] }),
     project({ groups: [{ id: "g", target: "build", projects: ["p"] }] }),
     project({ groups: [{ id: "g", targets: [], projects: ["p"] }] }),
@@ -117,5 +119,5 @@

 
 // #region 🔖️Listing
-const FIXED_REPO = ["goals.list", "goals.tree", "tree.monorepo", "tree.goal", "tree.statute", "tree.territory", "statutes.catalog", ...["all", "rust", "typescript", "go", "python", "markdown"].map((scope) => `analyze:${scope}`)];
+const FIXED_REPO = ["goals.list", "goals.tree", "tree.monorepo", "tree.goal", "tree.statute", "tree.territory", "statutes.catalog", "draft.start", "draft.discard", ...["all", "rust", "typescript", "go", "python", "markdown"].map((scope) => `analyze:${scope}`)];
 
 /** 🧭️ The command ids the fixture workspace must offer, derived without the registry. */
@@ -166,8 +168,8 @@

 });
 
-test("a search narrows by every word and the default listing leaves closed tickets out", () => {
+test("a search narrows by every word and the default listing offers closed tickets for reopening", () => {
   const search = (...words: string[]) => (JSON.parse(semio(["commands", "--json", ...words, "--root", root]).out) as Json[]).map((row) => row.id as string);
   expect(search("dev", "shell")).toContain("playground:shell");
-  expect(search("closed-probe")).toEqual([]);
+  expect(search("closed-probe")).toEqual(["repo:ticket.reopen:26/08/01/CLOSED-PROBE"]);
   expect(search("closed-probe", "--all")).toContain("repo:ticket.reopen:26/08/01/CLOSED-PROBE");
   expect(search("--all", "nonsense-word-xyz")).toEqual([]);
@@ -369,5 +371,5 @@

 const CASES: Case[] = [
   { id: "workspace:test", parameters: { cache: "skip-local", "test-level": "quick", dependencies: true, "cargo-jobs": "2" }, args: ["--filter", "x"], env: { EXTRA: "1" } },
-  { id: "workspace:build", parameters: { "build-mode": "ship" } },
+  { id: "workspace:build", parameters: { "build-mode": "release" } },
   { id: "workspace:dev" },
   { id: "workspace:verify", parameters: { rule: "literal-external" } },
@@ -382,5 +384,5 @@

   { id: "@fixture/quiz:test-e2e", parameters: { "test-level": "long" } },
   { id: "tool:workspace/inspector" },
-  { id: "tool:workspace/bun-test", parameters: { file: "a.test.ts", bail: true, "build-mode": "ship", "test-level": "quick" } },
+  { id: "tool:workspace/bun-test", parameters: { file: "a.test.ts", bail: true, "build-mode": "release", "test-level": "quick" } },
   { id: "tool:workspace/printer" },
   { id: "tool:workspace/viewer" },

### 27 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🧭️authority/🔣️.json
HEAD 211 BASE 18 STASH 1126 CURRENT H
--- 

+++ 

@@ -1,4 +1,449 @@

 {
   "version": 1,
+  "devcontainerRuntime": {
+    "postCreateCommand": [
+      "bun",
+      "repo",
+      "run",
+      "workspace:setup"
+    ],
+    "postStartCommand": [
+      "bun",
+      "repo",
+      "run",
+      "workspace:setup-devcontainer-start"
+    ],
+    "postAttachCommand": [
+      "bun",
+      "repo",
+      "run",
+      "workspace:setup-devcontainer-attach"
+    ],
+    "mounts": [
+      "source=semio-claude,target=/home/vscode/.claude,type=volume",
+      "source=semio-codex,target=/home/vscode/.codex,type=volume"
+    ]
+  },
+  "devcontainerCases": [
+    {
+      "name": "dashboard lifecycle and retained headless agents",
+      "add": {},
+      "accepted": true
+    },
+    {
+      "name": "direct lifecycle script bypasses Nx owner",
+      "add": {
+        "postStartCommand": [
+          "bun",
+          "./📜️script.ts",
+          "setup",
+          "devcontainer",
+          "start"
+        ]
+      },
+      "accepted": false
+    },
+    {
+      "name": "IDE extensions customize the runtime container",
+      "add": {
+        "customizations": {
+          "vscode": {
+            "extensions": [
+              "ms-vscode.cpptools"
+            ]
+          }
+        }
+      },
+      "accepted": false
+    },
+    {
+      "name": "IDE settings customize the runtime container",
+      "add": {
+        "settings": {
+          "editor.formatOnSave": true
+        }
+      },
+      "accepted": false
+    },
+    {
+      "name": "Codeium IDE configuration persists in a volume",
+      "add": {
+        "mounts": [
+          "source=semio-codeium,target=/home/vscode/.codeium,type=volume"
+        ]
+      },
+      "accepted": false
+    },
+    {
+      "name": "Kiro IDE configuration persists in a volume",
+      "add": {
+        "mounts": [
+          "source=semio-kiro,target=/home/vscode/.kiro,type=volume"
+        ]
+      },
+      "accepted": false
+    },
+    {
+      "name": "compiler toolchains and base image Linux account remain valid",
+      "add": {
+        "remoteUser": "vscode",
+        "features": {
+          "toolchain": {}
+        },
+        "onCreateCommand": [
+          "sudo",
+          "chown",
+          "vscode:vscode",
+          "/workspaces/semio/node_modules"
+        ]
+      },
+      "accepted": true
+    }
+  ],
+  "ownerCommandCases": [
+    {
+      "command": "bun ./📜️script.ts test authority",
+      "accepted": true
+    },
+    {
+      "command": "bun \"domain/📜️script.ts\" build --release",
+      "accepted": true
+    },
+    {
+      "command": "bun domain/📜️script.ts test",
+      "accepted": true
+    },
+    {
+      "command": "bun ../../📜️script.ts sync",
+      "accepted": true
+    },
+    {
+      "command": "bun 📜️script.ts generate",
+      "accepted": true
+    },
+    {
+      "command": "bun domain\\📜️script.ts test",
+      "accepted": true
+    },
+    {
+      "command": "bun --watch ./📜️script.ts generate-logo",
+      "accepted": true
+    },
+    {
+      "command": "bun ./📜️script.ts test --filter=\"test(a) | test(b)\"",
+      "accepted": true
+    },
+    {
+      "command": "bun ./📜️script.ts test 'quoted;argument'",
+      "accepted": true
+    },
+    {
+      "command": "bun ./📜️script.ts",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test ",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test && cargo test",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test;cargo test",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test | tee log",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test \"literal | value\" | tee log",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test $(whoami)",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test \"$(whoami)\"",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test `whoami`",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test \"`whoami`\"",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test $TASK_ARGS",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test ${TASK_ARGS}",
+      "accepted": false
+    },
+    {
+      "command": "bun ./📜️script.ts test\ncargo test",
+      "accepted": false
+    },
+    {
+      "command": "bun .🧬semio/🦑️repo/🎫️tickets/probe.mjs",
+      "accepted": false
+    },
+    {
+      "command": "cargo test",
+      "accepted": false
+    },
+    {
+      "command": "npm run build",
+      "accepted": false
+    },
+    {
+      "command": "bun nx run owner:build",
+      "accepted": false
+    },
+    {
+      "command": "bun ./script.ts test",
+      "accepted": false
+    },
+    {
+      "command": "bun \"./📜️script.ts build",
+      "accepted": false
+    }
+  ],
+  "ownerTargetCases": [
+    {
+      "target": {
+        "executor": "repo:script",
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        }
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        }
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "command": "bun ./📜️script.ts test"
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "executor": "repo:noop",
+        "dependsOn": [
+          "build"
+        ]
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "options": {
+          "commands": [
+            "bun ./📜️script.ts build",
+            {
+              "command": "bun ./📜️script.ts test",
+              "forwardAllArgs": true
+            }
+          ]
+        }
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        },
+        "configurations": {
+          "release": {
+            "command": "bun ./📜️script.ts build --release"
+          }
+        }
+      },
+      "accepted": true
+    },
+    {
+      "target": {
+        "executor": "nx:run-commands"
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "options": {
+          "commands": []
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "executor": "@nx/js:node",
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        },
+        "configurations": {
+          "release": {
+            "command": "cargo build"
+          }
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "options": {
+          "commands": [
+            "bun ./📜️script.ts build",
+            {
+              "command": "cargo test"
+            }
+          ]
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "executor": "repo:noop",
+        "options": {
+          "command": "cargo test"
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "executor": "nx:run-commands",
+        "options": {
+          "command": "bun ./📜️script.ts build"
+        }
+      },
+      "accepted": false
+    },
+    {
+      "target": {
+        "executor": "nx:noop",
+        "dependsOn": [
+          "build"
+        ]
+      },
+      "accepted": false
+    }
+  ],
+  "developerDocuments": [
+    "README.md",
+    ".devcontainer/README.md",
+    "AGENTS.md",
+    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md",
+    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md",
+    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📖️readme.md"
+  ],
+  "instructionCases": [
+    {
+      "text": "Start bun run dashboard and choose workspace:setup.",
+      "accepted": true
+    },
+    {
+      "text": "An editor may navigate source files while dashboard owns all development operations.",
+      "accepted": true
+    },
+    {
+      "text": "MCP clients use .mcp.json and .codex/config.toml.",
+      "accepted": true
+    },
+    {
+      "text": "Configure .vscode/launch.json and press Run.",
+      "accepted": false
+    },
+    {
+      "text": "The generator writes .vscode/mcp.json.",
+      "accepted": false
+    },
+    {
+      "text": "Configure .cursor/hooks.json.",
+      "accepted": false
+    },
+    {
+      "text": "Use .windsurf/mcp.json or .kiro/settings/mcp.json.",
+      "accepted": false
+    },
+    {
+      "text": "IDE startup runs the root script.",
+      "accepted": false
+    },
+    {
+      "text": "Select Dev Containers: Reopen in Container.",
+      "accepted": false
+    },
+    {
+      "text": "Install VS Code with the Dev Containers extension.",
+      "accepted": false
+    },
+    {
+      "text": "Configure domain/.idea/workspace.xml to build.",
+      "accepted": false
+    },
+    {
+      "text": "Launch from .zed/tasks.json or .fleet/run.json.",
+      "accepted": false
+    },
+    {
+      "text": "Set .kiro/settings/mcp.json.",
+      "accepted": false
+    },
+    {
+      "text": "Edit .cursor/settings.json and .cursor/rules/build.mdc.",
+      "accepted": false
+    },
+    {
+      "text": "Configure .cursor/config.toml.",
+      "accepted": false
+    },
+    {
+      "text": "Run from .cursor/launch.json.",
+      "accepted": false
+    },
+    {
+      "text": "Edit .vscode\\tasks.json.",
+      "accepted": false
+    },
+    {
+      "text": "Use app.sublime-build or app.sublime-project.",
+      "accepted": false
+    },
+    {
+      "text": "Configure CMakeSettings.json.",
+      "accepted": false
+    },
+    {
+      "text": "Use compiler-owned CMakePresets.json through workspace:build.",
+      "accepted": true
+    },
+    {
+      "text": "Consult .cursor/plans/archive.md for historical planning evidence.",
+      "accepted": true
+    }
+  ],
   "scripts": {
     "repo": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo",
@@ -8,11 +453,674 @@

   },
   "cases": [
-    {"name": "canonical bootstrap", "add": {}, "remove": [], "accepted": true},
-    {"name": "competing developer alias", "add": {"dev": "bun repo run workspace:dev"}, "remove": [], "accepted": false},
-    {"name": "missing dashboard", "add": {}, "remove": ["dashboard"], "accepted": false},
-    {"name": "bypass task owner", "add": {"setup": "bun ./📜️script.ts setup"}, "remove": [], "accepted": false},
-    {"name": "build alias without rebuild", "add": {"dashboard:install": "bun repo run @semio-tech/repo-dashboard-rs:install"}, "remove": [], "accepted": false}
-  ],
-  "editorDirectories": [".vscode", ".claude", ".cursor", ".codex", ".windsurf", ".zed", ".idea", ".fleet", ".run", ".gemini"],
-  "excludedDirectories": [".git", ".🧬semio", "node_modules", "target", "dist", "build", "generated", "cache", "fixtures", ".nx"]
+    {
+      "name": "canonical bootstrap",
+      "add": {},
+      "remove": [],
+      "accepted": true
+    },
+    {
+      "name": "competing developer alias",
+      "add": {
+        "dev": "bun repo run workspace:dev"
+      },
+      "remove": [],
+      "accepted": false
+    },
+    {
+      "name": "missing dashboard",
+      "add": {},
+      "remove": [
+        "dashboard"
+      ],
+      "accepted": false
+    },
+    {
+      "name": "bypass task owner",
+      "add": {
+        "setup": "bun ./📜️script.ts setup"
+      },
+      "remove": [],
+      "accepted": false
+    },
+    {
+      "name": "build alias without rebuild",
+      "add": {
+        "dashboard:install": "bun repo run @semio-tech/repo-dashboard-rs:install"
+      },
+      "remove": [],
+      "accepted": false
+    }
+  ],
+  "forbiddenDirectories": [
+    ".vscode",
+    ".vscode-insiders",
+    ".idea",
+    ".fleet",
+    ".zed",
+    ".vs",
+    ".run",
+    ".nova",
+    ".settings",
+    ".metadata",
+    ".windsurf",
+    ".kiro",
+    ".atom",
+    ".brackets"
+  ],
+  "forbiddenExtensions": [
+    ".sln",
+    ".slnx",
+    ".code-workspace",
+    ".iml",
+    ".ipr",
+    ".iws",
+    ".suo",
+    ".user",
+    ".userosscache",
+    ".sln.docstates",
+    ".sublime-project",
+    ".sublime-workspace",
+    ".sublime-build"
+  ],
+  "forbiddenFiles": [
+    ".project",
+    ".classpath",
+    ".vsconfig",
+    ".cursorrules",
+    ".windsurfrules",
+    "cmakesettings.json"
+  ],
+  "forbiddenPaths": [
+    ".cursor/mcp.json",
+    ".cursor/hooks.json",
+    ".devcontainer/editor-overlay.json",
+    ".cursor/settings.json",
+    ".cursor/rules"
+  ],
+  "excludedDirectories": [
+    ".git",
+    "node_modules",
+    "target",
+    "dist",
+    "build",
+    "generated",
+    "🗑️generated",
+    "🤖️generated",
+    "cache",
+    "⚡️cache",
+    ".nx",
+    ".venv",
+    ".repo",
+    ".pytest_cache",
+    ".ruff_cache",
+    "temp",
+    "test-results",
+    "storybook-static",
+    "coverage",
+    "__pycache__"
+  ],
+  "pathCases": [
+    {
+      "path": ".cursor",
+      "accepted": true
+    },
+    {
+      "path": ".cursor/launch.json",
+      "accepted": false
+    },
+    {
+      "path": ".cursor/extensions.json",
+      "accepted": false
+    },
+    {
+      "path": ".cursor/config.toml",
+      "accepted": false
+    },
+    {
+      "path": "domain/.cursor/plans-notes/settings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.vscode-insiders/tasks.json",
+      "accepted": false
+    },
+    {
+      "path": ".kiro/settings/mcp.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.kiro/steering/development.md",
+      "accepted": false
+    },
+    {
+      "path": ".atom/config.cson",
+      "accepted": false
+    },
+    {
+      "path": "domain/.brackets/settings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.cursorrules",
+      "accepted": false
+    },
+    {
+      "path": ".windsurfrules",
+      "accepted": false
+    },
+    {
+      "path": "domain/app.sublime-project",
+      "accepted": false
+    },
+    {
+      "path": "domain/app.sublime-workspace",
+      "accepted": false
+    },
+    {
+      "path": "domain/app.sublime-build",
+      "accepted": false
+    },
+    {
+      "path": ".vsconfig",
+      "accepted": false
+    },
+    {
+      "path": "domain/.vsconfig",
+      "accepted": false
+    },
+    {
+      "path": "domain/CMakeSettings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/CMakePresets.json",
+      "accepted": true
+    },
+    {
+      "path": ".vscode",
+      "accepted": false
+    },
+    {
+      "path": "domain/.vscode/tasks.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.idea/workspace.xml",
+      "accepted": false
+    },
+    {
+      "path": "domain/.fleet/settings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.zed/settings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.vs/cache",
+      "accepted": false
+    },
+    {
+      "path": "domain/.run/test.run.xml",
+      "accepted": false
+    },
+    {
+      "path": "domain/application.sln",
+      "accepted": false
+    },
+    {
+      "path": "domain/application.slnx",
+      "accepted": false
+    },
+    {
+      "path": "domain/application.code-workspace",
+      "accepted": false
+    },
+    {
+      "path": "domain/application.iml",
+      "accepted": false
+    },
+    {
+      "path": "domain/application.csproj.user",
+      "accepted": false
+    },
+    {
+      "path": "domain/.project",
+      "accepted": false
+    },
+    {
+      "path": ".cursor/mcp.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.cursor/hooks.json",
+      "accepted": false
+    },
+    {
+      "path": ".devcontainer/editor-overlay.json",
+      "accepted": false
+    },
+    {
+      "path": ".devcontainer/devcontainer.json",
+      "accepted": true
+    },
+    {
+      "path": ".cursor/plans/audit.md",
+      "accepted": true
+    },
+    {
+      "path": ".cursor/plans/mcp.json",
+      "accepted": true
+    },
+    {
+      "path": ".codex/config.toml",
+      "accepted": true
+    },
+    {
+      "path": ".mcp.json",
+      "accepted": true
+    },
+    {
+      "path": "domain/application.csproj",
+      "accepted": true
+    },
+    {
+      "path": "domain/application.fsproj",
+      "accepted": true
+    },
+    {
+      "path": "domain/application.vcxproj",
+      "accepted": true
+    },
+    {
+      "path": "domain/Cargo.toml",
+      "accepted": true
+    },
+    {
+      "path": "domain/CMakeLists.txt",
+      "accepted": true
+    },
+    {
+      "path": ".windsurf/mcp.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.windsurf/hooks.json",
+      "accepted": false
+    },
+    {
+      "path": ".cursor/settings.json",
+      "accepted": false
+    },
+    {
+      "path": "domain/.cursor/rules",
+      "accepted": false
+    },
+    {
+      "path": "domain/.cursor/rules/development.mdc",
+      "accepted": false
+    }
+  ],
+  "liveControls": [
+    {
+      "id": "@semio-tech/framework-os-dev:s-host-foreign-kind-s",
+      "parameters": {},
+      "args": [
+        "http://127.0.0.1:6070/",
+        "--tag",
+        "authority",
+        "dag"
+      ],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-os-dev:s-host-foreign-kind-s"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-os-dev:s-host-foreign-kind-s",
+        "--",
+        "http://127.0.0.1:6070/",
+        "--tag",
+        "authority",
+        "dag"
+      ]
+    },
+    {
+      "id": "@semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s",
+      "parameters": {},
+      "args": [
+        "http://127.0.0.1:6070/",
+        "--tag",
+        "authority"
+      ],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s",
+        "--",
+        "http://127.0.0.1:6070/",
+        "--tag",
+        "authority"
+      ]
+    },
+    {
+      "id": "@semio-tech/stdio-plugin:parent-removal-check",
+      "parameters": {
+        "snapshot": "{workspace}/witness"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/stdio-plugin:parent-removal-check"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/stdio-plugin:parent-removal-check",
+        "--",
+        "{workspace}/witness"
+      ]
+    },
+    {
+      "id": "@semio-tech/framework-renderer-wgpu:browser-dock-acceptance",
+      "parameters": {
+        "mode": "wgpu",
+        "serve": "http://127.0.0.1:7301/",
+        "output": "{workspace}/witness-output"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-renderer-wgpu:browser-dock-acceptance"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-renderer-wgpu:browser-dock-acceptance",
+        "--",
+        "run",
+        "wgpu",
+        "http://127.0.0.1:7301/",
+        "--output",
+        "{workspace}/witness-output"
+      ]
+    },
+    {
+      "id": "@semio-tech/framework-renderer-wgpu:browser-media-acceptance",
+      "parameters": {
+        "serve": "http://127.0.0.1:7303/",
+        "locale": "de",
+        "output": "{workspace}/witness-output"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-renderer-wgpu:browser-media-acceptance"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-renderer-wgpu:browser-media-acceptance",
+        "--",
+        "--serve",
+        "http://127.0.0.1:7303/",
+        "--locale",
+        "de",
+        "--output",
+        "{workspace}/witness-output"
+      ]
+    },
+    {
+      "id": "@semio-tech/framework-renderer-wgpu:browser-embedded-acceptance",
+      "parameters": {
+        "configuration": "{workspace}/witness.json",
+        "output": "{workspace}/witness-output"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-renderer-wgpu:browser-embedded-acceptance"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-renderer-wgpu:browser-embedded-acceptance",
+        "--",
+        "--configuration",
+        "{workspace}/witness.json",
+        "--output",
+        "{workspace}/witness-output"
+      ]
+    },
+    {
+      "id": "@semio-tech/framework-os-dev:parity-journey",
+      "parameters": {
+        "renderer": "paired",
+        "react-serve": "http://127.0.0.1:7300/",
+        "wgpu-serve": "http://127.0.0.1:7301/",
+        "locale": "en",
+        "output": "{workspace}/witness-output"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/framework-os-dev:parity-journey"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/framework-os-dev:parity-journey",
+        "--",
+        "--renderer",
+        "paired",
+        "--react-serve",
+        "http://127.0.0.1:7300/",
+        "--wgpu-serve",
+        "http://127.0.0.1:7301/",
+        "--locale",
+        "en",
+        "--output",
+        "{workspace}/witness-output"
+      ]
+    },
+    {
+      "id": "@semio-tech/bim-model-rs:sqlite-oracle-words",
+      "parameters": {
+        "word": "7ff8000000000042",
+        "file": "witness.sqlite"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/bim-model-rs:sqlite-oracle-words"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/bim-model-rs:sqlite-oracle-words",
+        "--",
+        "7ff8000000000042",
+        "--file",
+        "witness.sqlite"
+      ]
+    },
+    {
+      "id": "@semio-tech/bim-model-rs:sqlite-oracle-edit",
+      "parameters": {
+        "file": "witness.sqlite"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/bim-model-rs:sqlite-oracle-edit"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/bim-model-rs:sqlite-oracle-edit",
+        "--",
+        "--file",
+        "witness.sqlite"
+      ]
+    },
+    {
+      "id": "tool:workspace/run-targets",
+      "parameters": {
+        "targets": "test-snapshot-sqlite-source",
+        "projects": "@semio-tech/stdio-step-rs,@semio-tech/stdio-ifc-rs",
+        "parallel": "1",
+        "cache": "skip-all"
+      },
+      "args": [
+        "--outputStyle=stream"
+      ],
+      "env": {
+        "SEMIO_CONTROL_PROBE": "explicit"
+      },
+      "commandIds": [
+        "tool:workspace/run-targets"
+      ],
+      "words": [
+        "nx",
+        "run-many",
+        "--targets=test-snapshot-sqlite-source",
+        "--projects=@semio-tech/stdio-step-rs,@semio-tech/stdio-ifc-rs",
+        "--parallel=1",
+        "--skip-nx-cache",
+        "--skip-remote-cache",
+        "--outputStyle=stream"
+      ]
+    },
+    {
+      "id": "tool:workspace/run-script",
+      "parameters": {
+        "script": "📜️script.ts",
+        "project": "workspace",
+        "directory": "{workspace}"
+      },
+      "args": [
+        "--help"
+      ],
+      "env": {
+        "SEMIO_CONTROL_PROBE": "explicit"
+      },
+      "commandIds": [
+        "tool:workspace/run-script"
+      ],
+      "words": [
+        "run",
+        "nx",
+        "--",
+        "exec",
+        "--projects=workspace",
+        "--",
+        "bun",
+        "{workspace}/📜️script.ts",
+        "--help"
+      ]
+    },
+    {
+      "id": "tool:workspace/bun-test",
+      "parameters": {
+        "file": "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts"
+      },
+      "args": [
+        "--timeout",
+        "300000"
+      ],
+      "env": {
+        "SEMIO_CONTROL_PROBE": "explicit"
+      },
+      "commandIds": [
+        "tool:workspace/bun-test"
+      ],
+      "words": [
+        "run",
+        "nx",
+        "--",
+        "exec",
+        "--projects=workspace",
+        "--",
+        "bun",
+        "test",
+        "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts",
+        "--timeout",
+        "300000"
+      ]
+    },
+    {
+      "id": "compound:@teaching/architecture-course/course-with-proctor",
+      "parameters": {},
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@teaching/proctor:dev",
+        "@teaching/architecture-course:dev"
+      ],
+      "stop": "together",
+      "ports": [
+        8791,
+        6061
+      ]
+    },
+    {
+      "id": "@semio-tech/repo-dashboard-rs:test-authority",
+      "parameters": {},
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/repo-dashboard-rs:test-authority"
+      ]
+    },
+    {
+      "id": "@semio-tech/repo-dashboard-rs:test-execution",
+      "parameters": {},
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/repo-dashboard-rs:test-execution"
+      ]
+    },
+    {
+      "id": "@semio-tech/repo-dashboard-rs:daemon",
+      "parameters": {
+        "action": "status"
+      },
+      "args": [],
+      "env": {},
+      "commandIds": [
+        "@semio-tech/repo-dashboard-rs:daemon"
+      ],
+      "words": [
+        "nx",
+        "run",
+        "@semio-tech/repo-dashboard-rs:daemon",
+        "--",
+        "status"
+      ]
+    }
+  ],
+  "agentCases": [
+    {
+      "client": "claude-code",
+      "accepted": true
+    },
+    {
+      "client": "codex",
+      "accepted": true
+    },
+    {
+      "client": "vscode",
+      "accepted": false
+    },
+    {
+      "client": "cursor",
+      "accepted": false
+    }
+  ]
 }

### 28 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts
HEAD 85 BASE 75 STASH 419 CURRENT H
--- 

+++ 

@@ -1,6 +1,11 @@

 import { expect, test } from "bun:test";
 import Ajv2020 from "ajv/dist/2020";
-import { readdirSync, readFileSync } from "node:fs";
-import { join, relative, resolve } from "node:path";
+import { parse as parseJsonc } from "jsonc-parser";
+import { JSDOM } from "jsdom";
+import clamp from "lodash/clamp";
+import stringWidth from "string-width";
+import { existsSync, readdirSync, readFileSync } from "node:fs";
+import { dirname, join, relative, resolve } from "node:path";
+import { installedDashboard } from "../../📦️installation/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../../.."), dashboard = resolve(import.meta.dir, "../..");
@@ -9,4 +14,242 @@

 const validate = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")));
 const accepts = (scripts: Record<string, unknown>) => Object.keys(scripts).length === Object.keys(fixture.scripts).length && Object.entries(fixture.scripts).every(([name, command]) => scripts[name] === command);
+const allowedPath = (path: string) => {
+  const parts = path.replaceAll("\\", "/").toLowerCase().split("/");
+  const file = parts.at(-1)!;
+  return !parts.some(part => fixture.forbiddenDirectories.includes(part)) && !fixture.forbiddenExtensions.some((extension: string) => file.endsWith(extension)) && !fixture.forbiddenFiles.includes(file) && !fixture.forbiddenPaths.includes(parts.slice(-2).join("/")) && !parts.some((part, index) => part === ".cursor" && parts[index + 1] !== undefined && parts[index + 1] !== "plans");
+};
+
+const object = (value: unknown): value is Record<string, any> => typeof value === "object" && value !== null && !Array.isArray(value);
+const ownerCommand = /^bun (?:--watch )?(?:"((?:[^"\r\n]*[/\\])?📜️script\.ts)"|((?:[^\s"\r\n]*[/\\])?📜️script\.ts)) [a-z][a-z0-9-]*(?: (?:[^\s"';&|<>\r\n]+|"[^"\r\n]*"|'[^'\r\n]*')+)*$/;
+const acceptsOwnerCommand = (command: unknown) => typeof command === "string" && ownerCommand.test(command) && !/[$`]/.test(command);
+const acceptsOwnerOptions = (options: unknown) => object(options) && (!Object.hasOwn(options, "command") || acceptsOwnerCommand(options.command)) && (!Object.hasOwn(options, "commands") || Array.isArray(options.commands) && options.commands.length > 0 && options.commands.every((command: unknown) => acceptsOwnerCommand(object(command) ? command.command : command)));
+const acceptsOwnerTarget = (target: unknown) => object(target) && (!Object.hasOwn(target, "executor") || ["repo:script", "repo:noop"].includes(target.executor)) && (!Object.hasOwn(target, "command") || acceptsOwnerCommand(target.command)) && (!Object.hasOwn(target, "options") || acceptsOwnerOptions(target.options)) && (!Object.hasOwn(target, "configurations") || object(target.configurations) && Object.values(target.configurations).every(acceptsOwnerOptions)) && (target.executor === "repo:noop" || Object.hasOwn(target, "command") || object(target.options) && (Object.hasOwn(target.options, "command") || Object.hasOwn(target.options, "commands")));
+let repositoryPaths: { path: string; directory: boolean }[] | undefined;
+const activeRepositoryPaths = () => {
+  if (repositoryPaths) return repositoryPaths;
+  repositoryPaths = [];
+  const key = (name: string) => name.replaceAll("\uFE0F", "").toLowerCase();
+  const excluded = new Set<string>(fixture.excludedDirectories.map(key)), pending = [root];
+  let visited = 0;
+  while (pending.length) {
+    const directory = pending.pop()!;
+    if (++visited % 10000 === 0) console.log(`[DEBUG] authority gate inspected ${visited} source directories`);
+    for (const entry of readdirSync(directory, { withFileTypes: true })) {
+      const path = relative(root, join(directory, entry.name)).replaceAll("\\", "/");
+      if (entry.isSymbolicLink()) { repositoryPaths.push({ path, directory: false }); continue; }
+      if (entry.isDirectory() && (excluded.has(key(entry.name)) || path === ".🧬semio/🦑️repo/🎫️tickets")) continue;
+      repositoryPaths.push({ path, directory: entry.isDirectory() });
+      if (entry.isDirectory() && allowedPath(path)) pending.push(join(root, path));
+    }
+  }
+  return repositoryPaths;
+};
+
+test("every owner command and target agrees with the independent schema validator", () => {
+  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
+  const validateCommand = oracle.getSchema(`${schema.$id}#/$defs/OwnerCommand`)!, validateTarget = oracle.getSchema(`${schema.$id}#/$defs/OwnerTarget`)!;
+  for (const row of fixture.ownerCommandCases) {
+    expect(acceptsOwnerCommand(row.command), row.command).toBe(row.accepted);
+    expect(validateCommand(row.command), row.command).toBe(row.accepted);
+  }
+  for (const row of fixture.ownerTargetCases) {
+    expect(acceptsOwnerTarget(row.target), JSON.stringify(row.target)).toBe(row.accepted);
+    expect(validateTarget(row.target), JSON.stringify(row.target)).toBe(row.accepted);
+  }
+  console.log(`[DEBUG] authority checked ${fixture.ownerCommandCases.length} command vectors and ${fixture.ownerTargetCases.length} owner target vectors`);
+});
+
+test("runtime devcontainer lifecycle uses build owners and provisions no IDE customization", () => {
+  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
+  const validateRuntime = new Ajv2020({ strict: false, allErrors: true }).compile(schema.$defs.DevcontainerRuntime);
+  const acceptsRuntime = (value: Record<string, any>) => ["postCreateCommand", "postStartCommand", "postAttachCommand"].every(key => JSON.stringify(value[key]) === JSON.stringify(fixture.devcontainerRuntime[key])) && !["customizations", "settings", "extensions"].some(key => Object.hasOwn(value, key)) && (!Object.hasOwn(value, "mounts") || Array.isArray(value.mounts) && value.mounts.every((mount: unknown) => typeof mount === "string" && !/target=[^,]*\/\.(?:codeium|kiro|vscode|vscode-insiders|cursor|windsurf|idea|fleet|zed|atom|brackets)(?:\/|,|$)/.test(mount)));
+  for (const row of fixture.devcontainerCases) {
+    const value = { ...fixture.devcontainerRuntime, ...row.add };
+    expect(acceptsRuntime(value), row.name).toBe(row.accepted);
+    expect(validateRuntime(value), row.name).toBe(row.accepted);
+  }
+  const runtime = parseJsonc(readFileSync(join(root, ".devcontainer/devcontainer.json"), "utf8"));
+  expect(acceptsRuntime(runtime)).toBe(true);
+  expect(validateRuntime(runtime), JSON.stringify(validateRuntime.errors)).toBe(true);
+  const owner = read(join(root, "📋️project.json"));
+  for (const phase of ["start", "attach"]) expect(owner.targets[`setup-devcontainer-${phase}`]?.options.command).toBe(`bun ./📜️script.ts setup devcontainer ${phase}`);
+  console.log(`[DEBUG] authority checked ${fixture.devcontainerCases.length} runtime container vectors and all three build lifecycle owners`);
+});
+test("exclusive dashboard paths agree with the independent schema validator", () => {
+  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
+  const validatePath = new Ajv2020({ strict: false }).compile(schema.$defs.ActiveRepositoryPath);
+  for (const row of fixture.pathCases) {
+    expect(allowedPath(row.path), row.path).toBe(row.accepted);
+    expect(validatePath(row.path.toLowerCase()), row.path).toBe(row.accepted);
+    expect(allowedPath(row.path.toUpperCase().replaceAll("/", "\\")), row.path).toBe(row.accepted);
+  }
+});
+
+test("developer instructions agree with the independent schema validator and retire IDE setup", () => {
+  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
+  const validateInstructions = new Ajv2020({ strict: false }).compile(schema.$defs.DeveloperInstructions);
+  const acceptsInstructions = (text: string) => !/(?:\.(?:vscode(?:-insiders)?|idea|fleet|zed|vs|run|nova|settings|metadata|windsurf|kiro|atom|brackets)[/\\]|\.cursor[/\\](?!plans(?:[/\\]|$))|\.(?:cursorrules|windsurfrules)\b|\.sublime-(?:project|workspace|build)\b|cmakesettings\.json|ide startup|reopen in vs code|all required vs code extensions|dev containers: reopen in container|vs code.*with.*dev containers extension)/.test(text.toLowerCase());
+  for (const row of fixture.instructionCases) {
+    expect(acceptsInstructions(row.text), row.text).toBe(row.accepted);
+    expect(validateInstructions(row.text.toLowerCase()), row.text).toBe(row.accepted);
+  }
+  const documents = new Set<string>(fixture.developerDocuments);
+  for (const { path, directory } of activeRepositoryPaths()) if (!directory && /(?:^|\/)(?:README|CONTRIBUTING|DEVELOPMENT|AGENTS)\.md$/i.test(path) && !/(?:🧫️fixtures|🧪️tests|🧬️schema|\.cursor\/plans)(?:\/|$)/.test(path)) documents.add(path);
+  for (const path of documents) {
+    const text = readFileSync(join(root, path), "utf8");
+    expect(acceptsInstructions(text), path).toBe(true);
+    expect(validateInstructions(text.toLowerCase()), path).toBe(true);
+  }
+  console.log(`[DEBUG] authority checked ${documents.size} developer documents and ${fixture.instructionCases.length} shared instruction vectors`);
+}, 300_000);
+
+test("localized command metadata follows shared language-neutral vectors and independent schema validation", () => {
+  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
+  const vectors = read(join(dashboard, "🧫️fixtures/🌐️metadata/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
+  const validText = (value: unknown) => typeof value === "object" && value !== null && !Array.isArray(value) && Object.keys(value).sort().join(",") === "de,en" && Object.values(value).every(text => typeof text === "string" && text.trim().length > 0 && [...text].length <= 1024 && !/[\u0000-\u001f\u007f]/.test(text));
+  for (const row of vectors.cases) {
+    expect(Object.values(row.metadata).every(validText), row.name).toBe(row.accepted);
+    for (const [definition, base] of [
+      ["Tool", { id: "source-open", command: ["bun", "--version"] }],
+      ["TargetDeclaration", {}],
+      ["Parameter", { id: "path", kind: "text" }],
+      ["Compound", { id: "source-open", members: ["workspace:test"] }],
+      ["Group", { id: "source-open", targets: ["test"], projects: ["workspace"] }],
+      ["ParameterInfo", { id: "path", kind: "text", values: [], required: true, origin: "tool" }],
+      ["RegistryEntry", { id: "tool:workspace/source-open", kind: "tool", verb: "task", label: "source-open", owner: ["workspace"], longRunning: false, listed: true, mutating: false, parameters: [], source: "📋️project.json" }]
+    ] as const) {
+      const validate = oracle.getSchema(`${schema.$id}#/$defs/${definition}`)!;
+      expect(validate({ ...base, ...row.metadata }), `${row.name}: ${definition}`).toBe(row.accepted);
+    }
+  }
+  console.log(`[DEBUG] authority checked ${vectors.cases.length} localized metadata vectors across seven declarations and outputs`);
+});
+
+test("launcher input batches and viewport bounds match independent DOM and clamp oracles", () => {
+  const vectors = read(join(dashboard, "🧫️fixtures/⌨️batch/🔣️.json"));
+  const schema = read(join(dashboard, "🧬️schema/⌨️batch/🔣️.json"));
+  const validate = new Ajv2020({ strict: false }).compile(schema);
+  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
+  const fields = ["parameter", "args", "env"];
+  for (const row of vectors.cases) {
+    let focus = -1; const model = ["", "", ""];
+    const dom = new JSDOM("<form><input id='parameter'><input id='args'><input id='env'></form>");
+    const inputs = fields.map(field => dom.window.document.getElementById(field) as HTMLInputElement);
+    for (const event of row.events) {
+      if (event === "enter") { focus = 0; inputs[0]!.focus(); }
+      else if (event === "down") { focus = Math.min(2, focus + 1); inputs[focus]!.focus(); }
+      else {
+        const value = event.slice(event.indexOf(":") + 1);
+        model[focus] += value;
+        const input = dom.window.document.activeElement as HTMLInputElement;
+        input.setSelectionRange(input.value.length, input.value.length);
+        input.setRangeText(value, input.selectionStart!, input.selectionEnd!, "end");
+      }
+    }
+    expect(fields[focus], row.name).toBe(row.field);
+    expect(model[focus], row.name).toBe(row.expected);
+    expect(inputs[fields.indexOf(row.field)]!.value, row.name).toBe(row.expected);
+    dom.window.close();
+  }
+  for (const row of vectors.defaultTextCases) {
+    const dom = new JSDOM("<input>"); const input = dom.window.document.querySelector("input")!;
+    input.defaultValue = row.default ?? "";
+    expect(row.default ?? "").toBe(row.expected);
+    expect(input.value).toBe(row.expected);
+    dom.window.close();
+  }
+  for (const row of vectors.viewportCases) {
+    const oldTop = Math.max(0, 3 - row.initialPage + 1), maximum = Math.max(0, 12 - row.page);
+    expect(Math.max(0, Math.min(oldTop, maximum))).toBe(row.expectedTop);
+    expect(clamp(oldTop, 0, maximum)).toBe(row.expectedTop);
+  }
+  const metadata = read(join(dashboard, "🧫️fixtures/🌐️metadata/🔣️.json")).workspace.projects[0].dashboard.tools[0];
+  for (const row of vectors.pointerCases) {
+    const dom = new JSDOM("<form><h1></h1><p></p><label for='parameter'></label><input id='parameter'><label for='args'>Arguments</label><input id='args'></form>");
+    const document = dom.window.document, input = document.querySelector<HTMLInputElement>("#parameter")!, args = document.querySelector<HTMLInputElement>("#args")!, label = document.querySelector("label")!;
+    document.querySelector("h1")!.textContent = metadata.title[row.locale];
+    document.querySelector("p")!.textContent = metadata.description[row.locale];
+    input.placeholder = metadata.parameters[0].title[row.locale]; label.textContent = input.placeholder;
+    input.value = row.initialParameter; args.value = row.initialArgs; args.focus();
+    label.control!.focus(); input.setSelectionRange(input.value.length, input.value.length); input.setRangeText(row.typed, input.selectionStart!, input.selectionEnd!, "end");
+    expect(document.activeElement).toBe(input);
+    expect(input.placeholder).toBe(row.expectedPlaceholder);
+    expect(row.initialParameter + row.typed).toBe(row.expectedText);
+    expect(input.value).toBe(row.expectedText);
+    expect(args.value).toBe(row.initialArgs);
+    dom.window.close();
+  }
+  console.log(`[DEBUG] launcher batch oracle checked ${vectors.cases.length} input sequences, ${vectors.defaultTextCases.length} defaults, ${vectors.viewportCases.length} growing viewports and ${vectors.pointerCases.length} described pointer fields`);
+});
+
+test("typed repo actions agree with shared vectors and independent schema validation", () => {
+  const vectors = read(join(dashboard, "🧫️fixtures/🏛️repo-action/🔣️.json"));
+  const validateAction = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(dashboard, "🧬️schema/🏛️repo-action/🔣️.json")));
+  const registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json")), tree = read(join(dashboard, "🧬️schema/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(registry); oracle.addSchema(tree);
+  const text = (value: unknown) => typeof value === "string" && value.trim().length > 0 && [...value].length <= 1024 && !/[\u0000-\u001f\u007f]/.test(value);
+  const ticket = (value: unknown) => typeof value === "string" && /^\d{2}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*(?:\/[A-Z0-9]+(?:-[A-Z0-9]+)*)*$/.test(value);
+  const object = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
+  const keys = (value: Record<string, unknown>, expected: string[]) => Object.keys(value).sort().join(",") === expected.sort().join(",");
+  const accepts = (action: unknown) => {
+    if (typeof action === "string") return ["GoalsList", "GoalsTree", "TreeMonorepo", "TreeGoal", "TreeStatute", "TreeTerritory", "StatutesCatalog"].includes(action);
+    if (!object(action) || Object.keys(action).length !== 1) return false;
+    const [kind, value] = Object.entries(action)[0]!; if (!object(value)) return false;
+    if (["TicketShow", "TicketFiles", "TicketClose"].includes(kind)) return keys(value, ["id"]) && ticket(value.id);
+    if (kind === "TicketReopen") return keys(value, ["id", "prompt"]) && ticket(value.id) && text(value.prompt);
+    if (kind === "DraftStart") return keys(value, ["title"]) && text(value.title);
+    if (kind === "DraftDiscard") return keys(value, ["slug"]) && typeof value.slug === "string" && value.slug.length <= 1024 && /^[A-Z0-9]+(?:-[A-Z0-9]+)*$/.test(value.slug);
+    return kind === "Analyze" && keys(value, ["scope", "policy"]) && ["all", "rust", "typescript", "go", "python", "markdown"].includes(value.scope as string) && (value.policy === null || value.policy === "code");
+  };
+  for (const row of vectors.valid) {
+    expect(accepts(row.action), row.key).toBe(true);
+    expect(validateAction(row.action), `${row.key}: ${JSON.stringify(validateAction.errors)}`).toBe(true);
+    expect(oracle.getSchema(`${registry.$id}#/$defs/CommandId`)!(`repo:${row.key}`), row.key).toBe(true);
+    expect(oracle.getSchema(`${registry.$id}#/$defs/Reference`)!(`repo:${row.key}`), row.key).toBe(false);
+    expect(oracle.getSchema(`${tree.$id}#/$defs/RepoLeaf`)!({ kind: "repo", action: row.key, goArgv: row.argv }), row.key).toBe(true);
+  }
+  for (const action of vectors.invalid) {
+    expect(accepts(action), JSON.stringify(action)).toBe(false);
+    expect(validateAction(action), JSON.stringify(action)).toBe(false);
+  }
+  expect(oracle.getSchema(`${registry.$id}#/$defs/ParameterInfo`)!({ id: "title", kind: "text", values: [], required: true, origin: "repo" })).toBe(true);
+  console.log(`[DEBUG] repo action schema oracle checked ${vectors.valid.length} accepted and ${vectors.invalid.length} refused shared actions`);
+});
+
+test("native console width profiles match the shared schema and independent cursor model", () => {
+  const ui = join(root, "🧰️framework/🔨️modules/🖱️ui/⌨️tui");
+  const vectors = read(join(ui, "🧫️fixtures/📏️text-width/🪟️console/🔣️.json"));
+  const validate = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(ui, "🧬️schema/📏️text-width/🪟️console/🔣️.json")));
+  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
+  const windowsCells = (text: string) => [...text].reduce((cells, char) => cells + (char.codePointAt(0)! > 0xffff ? 2 : Math.max(1, stringWidth(char))), 0);
+  const sampleIds = new Set<string>();
+  for (const row of vectors.cases) {
+    expect(sampleIds.has(row.id), row.id).toBe(false); sampleIds.add(row.id);
+    expect(row.text.replace(/[\ufe0f\u200d]/gu, ""), row.id).toBe(row.scalarText);
+    expect(windowsCells(row.scalarText), `${row.id}: scalar console cells`).toBe(row.windowsScalarCells);
+    expect(windowsCells(row.text), `${row.id}: unprojected console cells`).toBe(row.windowsClusterCells);
+    if (!["variation-path", "canonical-argument"].includes(row.id)) expect(stringWidth(row.text), `${row.id}: formal cluster cells`).toBe(row.clusterCells);
+  }
+  expect(vectors.cases.find((row: { id: string }) => row.id === "canonical-argument")).toMatchObject({ scalarCells: 107, clusterCells: 110, windowsScalarCells: 110, windowsClusterCells: 119 });
+  console.log(`[DEBUG] independent string-width/UTF16 console model checked ${vectors.cases.length} measured native cursor vectors; formal VS16 cases use the native unicode-width 0.2.2 oracle`);
+});
+
+test("the Go repository implementation has a dashboard-owned source build", () => {
+  const cli = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli");
+  const vector = read(join(cli, "🧫️fixtures/🏗️build/🔣️.json"));
+  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
+  const validate = oracle.compile(read(join(cli, "🧬️schema/🏗️build/🔣️.json")));
+  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
+  const owner = read(join(cli, "📦️packages/🐹️go/📋️project.json"));
+  expect(owner.name).toBe(vector.project);
+  expect(owner.targets.build?.options.command).toBe(vector.command);
+  expect(owner.targets.build?.metadata.semio.dashboard).toEqual(vector.declaration);
+  expect(owner.targets.build?.outputs).toEqual(Object.values(vector.artifacts).map(path => `{workspaceRoot}/${path}`));
+  expect(oracle.getSchema(`${schema.$id}#/$defs/TargetDeclaration`)!(owner.targets.build?.metadata.semio.dashboard)).toBe(true);
+  console.log("[DEBUG] canonical Go repository build target agrees with its shared build/registry schemas");
+});
 
 test("the bootstrap decision matches the independent schema validator for every shared vector", () => {
@@ -25,20 +268,39 @@

 });
 
-test("every active source directory is free of editor and agent launch configuration", () => {
-  const editors = new Set<string>(fixture.editorDirectories), excluded = new Set<string>(fixture.excludedDirectories);
-  const failures: string[] = [], pending = [{ directory: root, editor: false }];
-  let visited = 0;
-  const key = (name: string) => name.replaceAll("\uFE0F", "").replace(/^[^\p{L}\p{N}.]+/u, "");
-  while (pending.length) {
-    const { directory, editor } = pending.pop()!;
-    if (++visited % 1000 === 0) console.log(`[DEBUG] authority gate inspected ${visited} source directories`);
-    for (const entry of readdirSync(directory, { withFileTypes: true })) {
-      const path = join(directory, entry.name);
-      if (entry.isSymbolicLink()) continue;
-      if (entry.isDirectory() && !excluded.has(key(entry.name)) && (!entry.name.startsWith(".") || editors.has(entry.name))) pending.push({ directory: path, editor: editor || editors.has(entry.name) });
-      if (editor && entry.isFile() && /(?:^|[.️])launch(?:[.][^/]*)?[.]jsonc?$/u.test(entry.name)) failures.push(relative(root, path).replaceAll("\\", "/"));
+test("every active source directory is free of IDE configuration and workspace files", () => {
+  const failures = activeRepositoryPaths().filter(({ path }) => !allowedPath(path)).map(({ path }) => path);
+  expect(failures.sort(), "active IDE configurations compete with the dashboard registry").toEqual([]);
+}, 300_000);
+
+test("all active owner manifests retain executable canonical scripts and valid dashboard declarations", () => {
+  const authority = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")), registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(authority); oracle.addSchema(registry);
+  const validateTarget = oracle.getSchema(`${authority.$id}#/$defs/OwnerTarget`)!, validateManifest = oracle.getSchema(`${registry.$id}#/$defs/ProjectManifest`)!;
+  const failures: string[] = [], owners = new Set<string>();
+  let targets = 0, commands = 0;
+  for (const { path, directory } of activeRepositoryPaths()) {
+    if (directory || !/(?:^|\/)📋️project\.json$/.test(path) || /(?:^|\/)(?:🧫️fixtures|fixtures)(?:\/|$)/.test(path)) continue;
+    const manifest = read(join(root, path)), projectRoot = dirname(join(root, path));
+    if (owners.has(manifest.name)) failures.push(`${path}: duplicate owner ${manifest.name}`);
+    owners.add(manifest.name);
+    if (!validateManifest(manifest)) failures.push(`${path}: ${oracle.errorsText(validateManifest.errors)}`);
+    for (const [name, target] of Object.entries(manifest.targets ?? {}) as [string, Record<string, any>][]) {
+      targets++;
+      const modeled = acceptsOwnerTarget(target), validated = validateTarget(target);
+      if (!modeled || !validated) failures.push(`${path}#${name}: model=${modeled}, schema=${validated}, ${oracle.errorsText(validateTarget.errors)}`);
+      for (const options of [target.options ?? {}, ...Object.values(target.configurations ?? {})] as Record<string, any>[]) {
+        const cwd = (options.cwd ?? target.options?.cwd ?? "{projectRoot}").replaceAll("{workspaceRoot}", root).replaceAll("{projectRoot}", projectRoot);
+        for (const raw of [target.command, options.command, ...(options.commands ?? [])].filter(value => value !== undefined)) {
+          commands++;
+          const command = object(raw) ? raw.command : raw, match = typeof command === "string" ? ownerCommand.exec(command) : null;
+          if (!match || !existsSync(resolve(root, cwd, match[1] ?? match[2]!))) failures.push(`${path}#${name}: missing canonical script for ${JSON.stringify(command)}`);
+        }
+      }
     }
   }
-  expect(failures.sort(), "active launch configurations compete with the dashboard registry").toEqual([]);
+  expect(owners.size).toBeGreaterThan(100);
+  expect(targets).toBeGreaterThan(3000);
+  expect(failures.sort(), "every owner must resolve commands through its permanent scripts and dashboard declarations").toEqual([]);
+  console.log(`[DEBUG] authority checked ${owners.size} owners, ${targets} targets and ${commands} existing canonical script invocations`);
 }, 300_000);
 
@@ -53,4 +315,27 @@

 });
 
+test("the dashboard has no editor projection route or implementation", () => {
+  const manifest = read(join(dashboard, "📦️packages/🦀️rust/📋️project.json"));
+  expect(manifest.targets.launch).toBeUndefined();
+  const script = readFileSync(join(dashboard, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
+  for (const name of ["LaunchScript", "projectDashboardLaunch", "launchParameterArguments", 'register("launch"']) expect(script).not.toContain(name);
+  expect(existsSync(join(dashboard, "🌳️command-tree/🚀️launch/🟦️.ts"))).toBe(false);
+  expect(readFileSync(join(dashboard, "README.md"), "utf8")).not.toContain("VSCode Launch Projection");
+});
+
+test("the editor extension exposes source navigation without developer operations", () => {
+  const extension = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode");
+  const contract = read(join(extension, "🧫️fixtures/🎛️dashboard-only/🔣️.json"));
+  const manifest = read(join(extension, "📦️packages/🟦️typescript/package.json"));
+  const source = readFileSync(join(extension, "🟦️.ts"), "utf8");
+  const contributions = JSON.stringify(manifest.contributes);
+  for (const command of contract.forbiddenCommands) {
+    expect(contributions, command).not.toContain(JSON.stringify(command));
+    expect(source, command).not.toContain(JSON.stringify(command));
+    expect(source, command).not.toContain(`'${command}'`);
+  }
+  for (const symbol of contract.forbiddenSymbols) expect(source, symbol).not.toContain(symbol);
+});
+
 test("authored scripts use one declarative build entry", () => {
   const manifest = read(join(root, "📋️project.json"));
@@ -59,5 +344,5 @@

   expect(tool.command).toEqual(["bun", "repo", "exec", "--projects={project}", "--", "bun", "{workspace}/{script}"]);
   expect(tool.cwd).toBe("{directory}");
-  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "script")).toEqual({ id: "script", kind: "text", required: true });
+  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "script")).toMatchObject({ id: "script", kind: "text", required: true });
   expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "project").default).toBe("workspace");
   const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
@@ -66,4 +351,44 @@

   expect(declaration(manifest), oracle.errorsText(declaration.errors)).toBe(true);
 });
+
+test("only the two retained agent clients can own derived MCP configuration", () => {
+  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
+  const validateClient = new Ajv2020({ strict: false }).compile(schema.$defs.AgentClientId);
+  const native = readFileSync(join(dashboard, "🎮️registry/🦀️.rs"), "utf8");
+  const clients = JSON.parse(native.match(/pub const AGENT_CLIENTS: \[&str; \d+\] = (\[[^;]+\]);/)![1]!);
+  for (const row of fixture.agentCases) {
+    expect(validateClient(row.client), row.client).toBe(row.accepted);
+    expect(clients.includes(row.client), row.client).toBe(row.accepted);
+  }
+});
+
+test("course and proctor compound belongs to the course project and retains readiness and cancellation", () => {
+  const course = read(join(root, "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"));
+  const proctor = read(join(root, "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"));
+  expect(course.metadata.semio.dashboard.compounds.find((compound: { id: string }) => compound.id === "course-with-proctor")).toEqual({ id: "course-with-proctor", verb: "dev", stop: "together", members: [{ run: "@teaching/proctor:dev" }, { run: "@teaching/architecture-course:dev" }] });
+  expect(proctor.targets.dev.metadata.semio.dashboard.ready).toEqual({ port: 8791, portEnv: "PROCTOR_PORT" });
+  expect(course.targets.dev.metadata.semio.dashboard.ready.port).toBe(6061);
+});
+
+test("the native dashboard resolves retained live controls with typed inputs and explicit argument vectors", () => {
+  const binary = process.env.SEMIO_DASHBOARD_BIN ?? installedDashboard(root, "prototype");
+  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
+  const validateLaunch = oracle.getSchema(`${schema.$id}#/$defs/Launch`)!;
+  for (const row of fixture.liveControls) {
+    const expand = (value: string) => value.replaceAll("{workspace}", root);
+    const args = ["run", row.id, "--root", root, "--dry-run", ...Object.entries(row.parameters).flatMap(([key, value]) => ["--param", `${key}=${expand(String(value))}`]), ...Object.entries(row.env).flatMap(([key, value]) => ["--env", `${key}=${value}`]), ...(row.args.length ? ["--", ...row.args] : [])];
+    const run = Bun.spawnSync([binary, ...args], { cwd: root, env: { ...process.env, SEMIO_DASHBOARD_GRAPH: "fresh" }, stdout: "pipe", stderr: "pipe" });
+    expect(run.exitCode, `${row.id}: ${run.stderr}`).toBe(0);
+    const launch = JSON.parse(run.stdout.toString());
+    expect(validateLaunch(launch), `${row.id}: ${oracle.errorsText(validateLaunch.errors)}`).toBe(true);
+    expect(launch.processes.map((process: { commandId: string }) => process.commandId), row.id).toEqual(row.commandIds);
+    if (row.words) expect(launch.processes[0].args, row.id).toEqual(row.words.map(expand));
+    if (row.stop) expect(launch.stop, row.id).toBe(row.stop);
+    if (row.ports) expect(launch.processes.map((process: { ready: { port: number } }) => process.ready.port), row.id).toEqual(row.ports);
+    for (const process of launch.processes) for (const [key, value] of Object.entries(row.env)) expect(Object.fromEntries(process.env)[key], `${row.id}: ${key}`).toBe(value);
+    console.log(`[DEBUG] native dashboard resolved ${row.id}: ${launch.processes.map((process: { commandId: string }) => process.commandId).join(", ")}`);
+  }
+}, 120_000);
 
 test("workspace startup never waits for Nx analytics consent", () => {
@@ -74,2 +399,21 @@

   expect(validateAnalytics(config.analytics)).toBe(true);
 });
+
+
+test("ready-address opening has a customizable binding and independent schema vectors", () => {
+  const vectors = read(join(dashboard, "🧫️fixtures/🌐️open/🔣️.json"));
+  const schema = read(join(dashboard, "🧬️schema/🌐️open/🔣️.json"));
+  const oracle = new Ajv2020({ strict: false });
+  const validateOpener = oracle.compile(schema.$defs.Opener), validateTask = oracle.compile(schema.$defs.ReadyTask);
+  const clean = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !/[\u0000\r\n]/.test(value);
+  for (const row of vectors.openers) {
+    expect(Array.isArray(row.value) && row.value.length > 0 && row.value.every(clean)).toBe(row.accepted);
+    expect(validateOpener(row.value)).toBe(row.accepted);
+  }
+  for (const row of vectors.tasks) {
+    expect(row.status === "running" && clean(row.url) && /^https?:\/\/.+/.test(row.url)).toBe(row.accepted);
+    expect(validateTask(row)).toBe(row.accepted);
+  }
+  const keymap = read(join(dashboard, "⚙️preferences/⌨️keymap/🔣️.json"));
+  expect(keymap.bindings.find((binding: { action: string }) => binding.action === vectors.action)).toEqual({ scope: "prefix", action: vectors.action, keys: [vectors.key] });
+});

### 29 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🥒️.feature
HEAD 33 BASE 31 STASH 92 CURRENT H
--- 

+++ 

@@ -3,13 +3,17 @@

   the same daemon through the dashboard. Package scripts only bootstrap the workspace.
 
-  Scenario: Keep only the four root bootstrap scripts
+  Scenario: Keep only the three root bootstrap scripts
     Given the repository root package manifest
-    Then its scripts are exactly nx, setup, dashboard and dashboard:install
+    Then its scripts are exactly nx, dashboard and dashboard:install
     And each script uses the canonical bootstrap or an owning Nx target
     And an independent JSON Schema validator reaches the same decision
 
-  Scenario: Refuse competing editor and agent launch configuration
+  Scenario: Refuse competing IDE configuration
     Given every active source directory of the monorepo
-    Then no editor or agent launch JSON or JSONC file exists
+    Then no IDE directory, workspace, solution or project-user file exists
+    And no Cursor MCP, hooks, settings or rules configuration exists
+    And Kiro, Atom, Brackets, Sublime and VS Code Insiders have no active configuration
+    And native compiler projects and historical Cursor plans remain valid
+    And the path decision agrees with an independent JSON Schema validator
     And historical ticket inputs, dependencies and generated output are outside the active configuration scope
 
@@ -18,4 +22,25 @@

     Then root commands are discoverable from targets or declarations
     And no root package script is required to discover them
+
+  Scenario: Every active owner resolves executable commands through canonical scripts
+    Given every active owner Nx manifest and configuration override
+    Then each runnable target invokes Bun with an existing permanent canonical script and explicit command
+    And dependency-only Nx noop targets remain valid
+    And shell pipelines, substitutions, direct compiler commands and direct temporary ticket probes are refused
+    And each dashboard declaration validates against the canonical registry schema
+    And independent JSON Schema validation agrees with the shared owner command and target vectors
+
+  Scenario: Active developer documents retain dashboard-only instructions
+    Given every active README, contributing guide, development guide and agent instruction file
+    Then no document prescribes retired IDE configuration or startup
+    And shared instruction vectors allow compiler configuration and historical planning evidence
+    And independent JSON Schema validation agrees with every instruction decision
+
+  Scenario: Container lifecycle belongs to dashboard Nx owners
+    Given the development runtime container
+    Then create, start and attach invoke their workspace Nx owners
+    And no IDE extension, settings customization or IDE configuration volume is provisioned
+    And compiler toolchains and retained headless agent volumes remain valid
+    And independent JSON Schema validation agrees with every shared container vector
 
   Scenario: Run authored scripts through one declarative Nx entry
@@ -30,2 +55,38 @@

     Then analytics consent is already explicitly configured
     And no Nx consent prompt blocks the selected task
+
+  Scenario: Editors use the dashboard registry
+    Given runnable commands are declared in owning Nx manifests or open ticket declarations
+    When a developer selects a command in the dashboard
+    Then its parameters and dependencies are resolved by the native registry
+    And one workspace daemon retains its identity and output
+    And no active editor or agent launch catalog exists
+    And no dashboard route generates an editor launch projection
+
+  Scenario: Derive configuration only for retained agent clients
+    Given MCP servers are declared by their owning dashboard tools
+    When the workspace derives agent configuration
+    Then only Claude Code and Codex are accepted clients
+    And native validation and an independent JSON Schema validator agree
+    And no IDE tasks, launches or MCP files are generated
+
+  Scenario: Own the course and proctor compound beside the course project
+    Given the architecture course owner declares course-with-proctor
+    When the developer starts the compound
+    Then the proctor becomes ready before the course starts
+    And the course uses its declared HTTP readiness endpoint
+    And cancelling either member stops both
+
+  Scenario: A task owns its original process invocation
+    Given a requesting view has its own process policy, capabilities and deadline
+    When it starts a separately supervised dashboard task
+    Then the view's process authority is not inherited by that task
+    And explicitly selected task environment is retained
+    And the task bootstrap creates authority for its actual selected command
+
+  Scenario: Original receiving and compiler controls remain executable
+    Given the original receiving native test is declared by the OS owner
+    And Cargo synchronization is declared by its workspace owner
+    When editor launch catalogs are removed
+    Then both commands remain discoverable as Nx target identities
+    And their current execution controls are provided by the Nx caller

### 30 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏗️workspace/🦀️.rs
HEAD 114 BASE 113 STASH 189 CURRENT H
--- 

+++ 

@@ -1,3 +1,3 @@

-//! 🏗️ The journey workspace: a copy of the fixture workspace in a temporary directory, with free ports,
+//! 🏗️ The journey workspace: isolated fixture metadata with free ports and immutable fixture code,
 //! its own daemon (the daemon of a workspace is named after the workspace path) and the real `semio`.
 //!
@@ -9,4 +9,17 @@

 use std::process::{Command, Output, Stdio};
 use std::time::Duration;
+use std::sync::atomic::{AtomicUsize, Ordering};
+
+static WORKSPACES: AtomicUsize = AtomicUsize::new(0);
+
+/// 🧾️ The portable human workflow vectors consumed by the terminal journeys.
+pub fn journey_fixture() -> serde_json::Value {
+    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🧭️journeys/🔣️.json"))).unwrap()
+}
+
+/// 📜️ Immutable fixture behavior, executed with the isolated workspace as its working directory.
+pub fn fixture_script() -> PathBuf {
+    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧫️fixtures/🧭️journeys/🏗️workspace/📜️script.ts").canonicalize().unwrap()
+}
 
 /// 🔢️ The sentinel ports of the fixture manifest that every copy replaces by free ones.
@@ -45,21 +58,71 @@

 
 impl Workspace {
-    /// 📂 Copies the fixture into a fresh directory outside the repository (a `.git` folder marks its root).
+    /// 📂 Generates short-lived metadata under the test artifact root without copying source code.
     pub fn new(name: &str) -> Self {
         let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧫️fixtures/🧭️journeys/🏗️workspace");
-        let base = std::env::temp_dir().join(format!("semio-journeys-{}-{}", std::process::id(), name));
-        let _ = std::fs::remove_dir_all(&base);
-        let root = base.join("workspace");
+        let artifacts = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
+        let artifacts = if artifacts.is_absolute() { artifacts } else { std::env::current_dir().unwrap().join(artifacts) };
+        let base = artifacts.join(format!("j{:x}.{:x}", std::process::id(), WORKSPACES.fetch_add(1, Ordering::Relaxed)));
+        let root = base.join("w");
+        assert!(root.to_string_lossy().encode_utf16().count() <= 152, "journey roots must leave room for native logs within 256 UTF-16 units");
         std::fs::create_dir_all(root.join(".git")).expect("create the workspace");
         let (port_a, port_b) = (free_port(), free_port());
-        for entry in std::fs::read_dir(&source).expect("read the fixture workspace") {
-            let entry = entry.unwrap();
-            if !entry.file_type().unwrap().is_file() || entry.file_name().to_string_lossy().ends_with(".rs") { continue; }
-            let text = std::fs::read_to_string(entry.path()).unwrap().replace(&PORT_A.to_string(), &port_a.to_string()).replace(&PORT_B.to_string(), &port_b.to_string());
-            std::fs::write(root.join(entry.file_name()), text).unwrap();
+        let text = std::fs::read_to_string(source.join("📋️project.json")).unwrap().replace(&PORT_A.to_string(), &port_a.to_string()).replace(&PORT_B.to_string(), &port_b.to_string());
+        let mut manifest: serde_json::Value = serde_json::from_str(&text).unwrap();
+        for tool in manifest["metadata"]["semio"]["dashboard"]["tools"].as_array_mut().unwrap() {
+            for argument in tool["command"].as_array_mut().unwrap() {
+                if argument.as_str() == Some("{workspace}/📜️script.ts") { *argument = fixture_script().display().to_string().into(); }
+            }
         }
-        let runtime = base.join("runtime");
+        std::fs::write(root.join("📋️project.json"), serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
+        let runtime = base.join("r");
         std::fs::create_dir_all(&runtime).unwrap();
+        println!("[DEBUG] isolated journey {name} root={}", root.display());
         Self { root: std::fs::canonicalize(&root).unwrap(), port_a, port_b, runtime }
+    }
+
+    /// 🎚️ Adds the portable choice/flag/text fixture only to the journey that exercises its form.
+    pub fn with_form_tool(name: &str) -> Self {
+        Self::with_fixture_tool(name, "localizedForm")
+    }
+
+    /// 🐚️ Uses the immutable production developer operation with a test-owned original invocation.
+    pub fn with_terminal_tool(name: &str) -> Self {
+        Self::with_fixture_tool(name, "workspaceTerminal")
+    }
+
+    fn with_fixture_tool(name: &str, key: &str) -> Self {
+        let workspace = Self::new(name);
+        let path = workspace.root.join("📋️project.json");
+        let mut manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
+        let fixture = journey_fixture();
+        let mut tool = fixture[key]["tool"].clone();
+        if key == "workspaceTerminal" { tool["parameters"][1]["default"] = fixture[key][if cfg!(windows) { "windowsShell" } else { "unixShell" }].clone(); }
+        tool["command"][1] = fixture_script().display().to_string().into();
+        manifest["metadata"]["semio"]["dashboard"]["tools"].as_array_mut().unwrap().push(tool);
+        std::fs::write(path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
+        workspace
+    }
+
+    /// 💾️ The bounded local preference journal of this isolated test workspace.
+    pub fn preference_path(&self) -> PathBuf { self.runtime.join("p.jsonl") }
+
+    /// 🌐️ Where the independent opener records the exact address the UI handed it.
+    pub fn opened_url_path(&self) -> PathBuf { self.runtime.join("opened.json") }
+
+    /// 🧾️ A durable independent receipt unaffected by native terminal hard wrapping.
+    pub fn terminal_receipt_path(&self) -> PathBuf { self.runtime.join("t.json") }
+
+    /// 📐️ Physical console cursor facts of the immutable fixture width probe.
+    pub fn width_probe_path(&self) -> PathBuf { self.runtime.join("width.json") }
+
+    /// 📏️ Verifies the actual native files remain within the repository's cross-platform path budget.
+    pub fn assert_paths(&self) {
+        fn maximum(path: &Path) -> usize {
+            let length = path.to_string_lossy().encode_utf16().count();
+            assert!(length <= 256, "journey path exceeds 256 UTF-16 units: {}", path.display());
+            if path.is_dir() { std::fs::read_dir(path).unwrap().map(|entry| maximum(&entry.unwrap().path())).max().unwrap_or(length).max(length) } else { length }
+        }
+        println!("[DEBUG] native journey file paths maxUtf16={}", maximum(self.root.parent().unwrap()));
     }
 
@@ -75,5 +138,18 @@

     /// 🌍️ The environment every process of this workspace gets: its own socket directory and no inherited nesting.
     pub fn env(&self) -> Vec<(String, String)> {
-        vec![("SEMIO_DASHBOARD_RUNTIME_DIR".into(), self.runtime.display().to_string()), ("SEMIO_LOCALE".into(), "en".into())]
+        vec![
+            ("SEMIO_DASHBOARD_RUNTIME_DIR".into(), self.runtime.display().to_string()),
+            ("SEMIO_DASHBOARD_INSTANCE".into(), String::new()),
+            ("SEMIO_DASHBOARD_CONFIG".into(), self.preference_path().display().to_string()),
+            ("JOURNEY_TERMINAL_RECORD".into(), self.terminal_receipt_path().display().to_string()),
+            ("JOURNEY_WIDTH_RECORD".into(), self.width_probe_path().display().to_string()),
+            ("SEMIO_LOCALE".into(), "en".into()),
+            ("SEMIO_APPEARANCE".into(), String::new()),
+            ("SEMIO_LOCKED_TERMINOLOGY".into(), String::new()),
+            ("SEMIO_DASHBOARD_RENDERER".into(), String::new()),
+            ("SEMIO_DASHBOARD_LAYOUT".into(), String::new()),
+            ("SEMIO_DASHBOARD_PREFIX".into(), String::new()),
+            ("SEMIO_REPO_IMPLEMENTATION".into(), "rust".into()),
+        ]
     }
 

### 31 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/💨️smoke/🟦️.ts
HEAD 85 BASE 88 STASH 88 CURRENT H
--- 

+++ 

@@ -69,5 +69,5 @@

     const busy = await get("http://127.0.0.1:6061/").then(() => true, () => false);
     if (busy) { console.log("[smoke] port 6061 is in use (a developer's quiz is running): scenario skipped"); return; }
-    const started = await run(["run", "@teaching/architecture-quiz:dev", "--detach", "--wait-ready", "--timeout", "900"], 1_000_000);
+    const started = await run(["run", "@teaching/architecture-course:dev", "--detach", "--wait-ready", "--timeout", "900"], 1_000_000);
     expect(started.code, `${started.stderr}\n${started.stdout}`).toBe(0);
     const address = /http:\/\/(?:127\.0\.0\.1|localhost):6061\S*/.exec(started.stdout)?.[0];
@@ -75,5 +75,5 @@

     const early = await get(address!).then(() => false, () => true);
     expect((await get(address!, 240)).status).toBe(200);
-    const mine = async () => (JSON.parse((await run(["tasks", "--json"])).stdout) as { session: string; commandId: string; status: string; readyUrl: string | null; startedMs: number }[]).filter((task) => task.commandId === "@teaching/architecture-quiz:dev").sort((a, b) => b.startedMs - a.startedMs)[0]!;
+    const mine = async () => (JSON.parse((await run(["tasks", "--json"])).stdout) as { session: string; commandId: string; status: string; readyUrl: string | null; startedMs: number }[]).filter((task) => task.commandId === "@teaching/architecture-course:dev").sort((a, b) => b.startedMs - a.startedMs)[0]!;
     const task = await mine();
     expect(task.status).toBe("running");

### 32 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🟦️.ts
HEAD 230 BASE 231 STASH 245 CURRENT H
--- 

+++ 

@@ -9,4 +9,5 @@

 import { join } from "node:path";
 import { Workspace, get, plain, scenarios, semio, sleep, slug, type Task } from "./🧰️support/🟦️.ts";
+import Ajv from "ajv/dist/2020.js";
 
 const feature = join(import.meta.dir, "🥒️.feature");
@@ -36,4 +37,17 @@

 
 describe("cli journeys", () => {
+  test("The portable human workflows satisfy their schema", () => {
+    const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧭️journeys/🔣️.json"), "utf8"));
+    const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧭️journeys/🔣️.json"), "utf8"));
+    const validate = new Ajv().compile(schema);
+    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
+    expect(fixture.localizedForm.tool.parameters.map((parameter: { kind: string }) => parameter.kind)).toEqual(["text", "choice", "flag"]);
+    expect(fixture.localizedForm.expected.args).toEqual(["--label", "wide value"]);
+    for (const locale of ["en", "de"]) {
+      expect(fixture.localizedForm.tool.title[locale]).toBe(fixture.localizedForm.labels[locale].title);
+      expect(fixture.localizedForm.tool.description[locale]).toBe(fixture.localizedForm.labels[locale].description);
+      for (const parameter of fixture.localizedForm.tool.parameters) expect(parameter.title[locale]).toBe(fixture.localizedForm.labels[locale][parameter.id]);
+    }
+  });
   test("A listening server waits for a successful HTTP response", async () => {
     const ws = await workspace("http-readiness");

### 33 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🦀️.rs
HEAD 300 BASE 305 STASH 845 CURRENT H
--- 

+++ 

@@ -14,9 +14,542 @@

 use workspace::Workspace;
 
+fn wait_json(path: &std::path::Path, what: &str) -> serde_json::Value {
+    let deadline = std::time::Instant::now() + SLOW;
+    loop {
+        if let Ok(bytes) = std::fs::read(path) { if let Ok(value) = serde_json::from_slice(&bytes) { return value; } }
+        assert!(std::time::Instant::now() < deadline, "no {what} at {}", path.display());
+        std::thread::sleep(Duration::from_millis(40));
+    }
+}
+
+fn process_alive(pid: u64) -> bool {
+    let pid = pid.to_string();
+    if cfg!(windows) {
+        std::process::Command::new("tasklist").args(["/FI", &format!("PID eq {pid}"), "/NH"]).output().is_ok_and(|answer| String::from_utf8_lossy(&answer.stdout).split_whitespace().any(|word| word == pid))
+    } else { std::process::Command::new("kill").args(["-0", &pid]).output().is_ok_and(|answer| answer.status.success()) }
+}
+
+fn tab_chip(text: &str, label: &str, right: bool, bottom: bool) -> Option<(usize, usize, Vec<String>)> {
+    let rows: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
+    for (y, row) in rows.iter().enumerate() {
+        let line: String = row.iter().collect();
+        for (byte, _) in line.match_indices(label) {
+            let first = line[..byte].chars().count();
+            if (first > 60) != right || (y > 17) != bottom || y == 0 || y + 1 >= rows.len() { continue; }
+            let x = row[..first].iter().rposition(|character| matches!(character, '┃' | '│' | '║' | '|'))?;
+            let end = first + label.chars().count();
+            let last = end + row[end..].iter().position(|character| matches!(character, '┃' | '│' | '║' | '|'))?;
+            if rows[y - 1..=y + 1].iter().any(|row| row.len() <= last) { continue; }
+            return Some((x, y - 1, rows[y - 1..=y + 1].iter().map(|row| row[x..=last].iter().collect()).collect()));
+        }
+    }
+    None
+}
+
+#[test]
+fn a_draft_starts_and_is_discarded_after_visible_confirmation() {
+    let fixture = workspace::journey_fixture();
+    let flow = &fixture["domainLifecycle"];
+    let draft = &flow["draft"];
+    let ws = Workspace::new("draft-human-flow");
+    let directory = ws.root.join(draft["directory"].as_str().unwrap());
+    let mut term = open(&ws);
+    domain_form(&mut term, draft["startId"].as_str().unwrap(), "Draft Title");
+    term.type_text(draft["title"].as_str().unwrap());
+    term.send(b"\x1b[B\x1b[B\x1b[B\r");
+    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
+    assert!(!directory.exists());
+    assert!(tasks(&ws).is_empty());
+    term.checkpoint("draft-confirmation");
+    term.send(terminal::ESCAPE);
+    term.wait_until("returning to the draft form", SLOW, |text| text.contains("Draft Title") && !text.contains(flow["confirmation"].as_str().unwrap())).unwrap();
+    assert!(!directory.exists());
+    term.send(terminal::ENTER);
+    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
+    term.send(terminal::ENTER);
+    term.wait_for(draft["startLine"].as_str().unwrap(), SLOW).unwrap();
+    let started = wait_task(&ws, "draft created", |task| command_id(task) == draft["startId"].as_str().unwrap() && task["status"] == "exited");
+    assert_eq!(started["code"], 0);
+    assert!(directory.is_dir());
+    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 0);
+    term.checkpoint("draft-created");
+    domain_form(&mut term, draft["discardId"].as_str().unwrap(), "Draft Identifier");
+    term.type_text(draft["slug"].as_str().unwrap());
+    term.send(b"\x1b[B\x1b[B\x1b[B\r");
+    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
+    assert!(directory.is_dir());
+    term.send(terminal::ENTER);
+    term.wait_for(draft["discardLine"].as_str().unwrap(), SLOW).unwrap();
+    let discarded = wait_task(&ws, "draft discarded", |task| command_id(task) == draft["discardId"].as_str().unwrap() && task["status"] == "exited");
+    assert_eq!(discarded["code"], 0);
+    assert!(!directory.exists());
+    assert_eq!(tasks(&ws).len(), 2);
+    term.checkpoint("draft-discarded");
+    ws.assert_paths();
+    println!("[DEBUG] visible draft confirmations created and discarded the exact isolated directory");
+}
+
+fn domain_form(term: &mut terminal::Terminal, command: &str, label: &str) {
+    term.leader("n");
+    term.wait_for("commands", SLOW).unwrap();
+    term.type_text(command);
+    term.wait_for("1 of", SLOW).unwrap();
+    term.send(terminal::ENTER);
+    term.wait_for(label, SLOW).unwrap();
+    assert!(term.text().contains(command));
+}
+
+#[test]
+fn a_closed_ticket_reopens_with_the_required_durable_human_prompt() {
+    let fixture = workspace::journey_fixture();
+    let flow = &fixture["domainLifecycle"];
+    let reopen = &flow["reopen"];
+    let ws = Workspace::new("ticket-reopen-human-flow");
+    let document = ws.root.join(reopen["documentPath"].as_str().unwrap());
+    let important = ws.root.join(reopen["importantPath"].as_str().unwrap());
+    std::fs::create_dir_all(document.parent().unwrap()).unwrap();
+    let original = serde_json::to_vec_pretty(&reopen["document"]).unwrap();
+    std::fs::write(&document, &original).unwrap();
+    let mut term = open(&ws);
+    domain_form(&mut term, reopen["commandId"].as_str().unwrap(), "Reason to Reopen");
+    term.send(b"\x1b[B\x1b[B\x1b[B\r");
+    term.wait_for("required", SLOW).unwrap();
+    assert!(tasks(&ws).is_empty());
+    assert_eq!(std::fs::read(&document).unwrap(), original);
+    assert!(!important.exists());
+    term.send(b"\x1b[A\x1b[A\x1b[A");
+    term.type_text(reopen["prompt"].as_str().unwrap());
+    term.send(b"\x1b[B\x1b[B\x1b[B\r");
+    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
+    assert_eq!(std::fs::read(&document).unwrap(), original);
+    assert!(!important.exists());
+    term.checkpoint("ticket-reopen-confirmation");
+    term.send(terminal::ENTER);
+    term.wait_for(reopen["expectedLine"].as_str().unwrap(), SLOW).unwrap();
+    let task = wait_task(&ws, "ticket reopened", |task| command_id(task) == reopen["commandId"].as_str().unwrap() && task["status"] == "exited");
+    assert_eq!(task["code"], 0);
+    let actual: serde_json::Value = serde_json::from_slice(&std::fs::read(&document).unwrap()).unwrap();
+    assert_eq!(actual["status"], "open");
+    let history = actual["interactions"].as_array().expect("durable ticket interaction history");
+    let previous = reopen["document"]["interactions"].as_array().unwrap();
+    assert_eq!(history.len(), previous.len() + 1);
+    for (actual, expected) in history.iter().zip(previous) { for (key, value) in expected.as_object().unwrap() { assert_eq!(&actual[key], value); } }
+    assert_eq!(history.last().unwrap()["kind"], "ticket.reopen");
+    assert_eq!(history.last().unwrap()["prompt"], reopen["prompt"]);
+    assert_eq!(history.last().unwrap()["client"], "claude-code");
+    assert!(history.last().unwrap()["date"].as_str().is_some_and(|date| !date.is_empty()));
+    assert_eq!(std::fs::read(&important).unwrap(), b"");
+    assert_eq!(tasks(&ws).len(), 1);
+    term.checkpoint("ticket-reopened");
+    ws.assert_paths();
+    println!("[DEBUG] visible ticket reopen retained history and persisted the exact required human prompt without management");
+}
+
+#[test]
+fn a_workspace_terminal_streams_shell_input_and_exits_inside_the_dashboard() {
+    let fixture = workspace::journey_fixture();
+    let flow = &fixture["workspaceTerminal"];
+    let input = &flow["input"][if cfg!(windows) { "windows" } else { "unix" }];
+    let ws = Workspace::with_terminal_tool("supervised workspace terminal");
+    let mut term = open(&ws);
+    term.leader("n");
+    term.wait_for("commands", SLOW).unwrap();
+    term.type_text(flow["commandId"].as_str().unwrap());
+    term.wait_for("1 of", SLOW).unwrap();
+    term.send(terminal::ENTER);
+    term.wait_for("[ Start ]", SLOW).unwrap();
+    term.checkpoint("workspace-terminal-form");
+    let form = term.wait_for("Directory: .", SLOW).unwrap();
+    assert!(form.contains(&format!("Shell: ‹{}›", flow[if cfg!(windows) { "windowsShell" } else { "unixShell" }].as_str().unwrap())));
+    for _ in 0..4 { term.send(b"\x1b[B"); }
+    term.send(terminal::ENTER);
+    term.wait_for("Running developer command: terminal", SLOW).unwrap();
+    std::thread::sleep(Duration::from_millis(500));
+    term.type_text(input[0].as_str().unwrap());
+    term.send(terminal::ENTER);
+    term.wait_until("the exact shell output row", SLOW, |screen| screen.lines().any(|line| line.trim_matches(|character| matches!(character, '┃' | '│' | '║' | '|' | ' ')) == flow["expectedLine"].as_str().unwrap())).unwrap();
+    term.type_text(input[1].as_str().unwrap());
+    term.send(terminal::ENTER);
+    let task = wait_task(&ws, "workspace terminal exited", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
+    assert_eq!(task["code"], 0);
+    term.wait_until("the terminal exit on screen", SLOW, shows_end).unwrap();
+    let log = ws.ok(&["logs", task["session"].as_str().unwrap()]);
+    let received = wait_json(&ws.terminal_receipt_path(), "the production terminal receiver's exact arguments and root");
+    assert_eq!(received["args"], flow["expectedArguments"][if cfg!(windows) { "windows" } else { "unix" }]);
+    assert_eq!(std::path::Path::new(received["root"].as_str().unwrap()).canonicalize().unwrap(), ws.root);
+    let raw = ws.ok(&["logs", task["session"].as_str().unwrap(), "--raw"]);
+    let after = raw.split_once(&format!("\n{}", flow["expectedLine"].as_str().unwrap())).expect("the actual shell emitted the probe at the start of a raw terminal line").1;
+    assert!(after.starts_with('\r') || after.starts_with('\n') || after.starts_with("\x1b["), "the shell receipt has unexpected trailing text: {log}");
+    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == flow["commandId"].as_str().unwrap()).count(), 1);
+    term.checkpoint("workspace-terminal");
+    ws.assert_paths();
+    println!("[DEBUG] production workspace shell received echo and exit through PTY, logged the exact probe line and exited 0");
+}
+
+#[test]
+fn a_german_form_submits_choices_flags_quoted_arguments_and_environment() {
+    let fixture = workspace::journey_fixture();
+    let form = &fixture["localizedForm"];
+    let labels = &form["labels"]["de"];
+    let ws = Workspace::with_form_tool("German configured form");
+    let mut term = terminal::Terminal::spawn(&workspace::binary(), &["--language", "de"], &ws.root, &ws.env(), steps::ROWS, steps::COLS);
+    term.wait_for(labels["newTask"].as_str().unwrap(), SLOW).unwrap();
+    term.leader("n");
+    term.wait_for(labels["commands"].as_str().unwrap(), SLOW).unwrap();
+    term.type_text(form["commandId"].as_str().unwrap());
+    term.wait_for(labels["matched"].as_str().unwrap(), SLOW).unwrap();
+    term.send(terminal::ENTER);
+    term.wait_for(labels["required"].as_str().unwrap(), SLOW).unwrap();
+    term.type_text(form["input"]["message"].as_str().unwrap());
+    term.send(b"\x1b[B\x1b[C");
+    term.wait_for(&format!("{}: ‹beta›", labels["flavor"].as_str().unwrap()), SLOW).unwrap();
+    term.send(b"\x1b[B ");
+    term.wait_for(&format!("[x] {}", labels["enabled"].as_str().unwrap()), SLOW).unwrap();
+    term.send(b"\x1b[B");
+    term.type_text(form["input"]["extraArguments"].as_str().unwrap());
+    term.wait_for(form["input"]["extraArguments"].as_str().unwrap(), SLOW).unwrap();
+    term.send(b"\x1b[B");
+    term.type_text(form["input"]["extraEnvironment"].as_str().unwrap());
+    term.wait_for(form["input"]["extraEnvironment"].as_str().unwrap(), SLOW).unwrap();
+    term.send(b"\x1b[B");
+    let screen = term.wait_for(labels["start"].as_str().unwrap(), SLOW).unwrap();
+    assert!(screen.contains(labels["extraArguments"].as_str().unwrap()) && screen.contains(labels["extraEnvironment"].as_str().unwrap()));
+    for key in ["title", "description", "message", "flavor", "enabled"] { assert!(screen.contains(labels[key].as_str().unwrap()), "missing localized {key}: {screen}"); }
+    assert!(!screen.contains("required") && !screen.contains("Extra arguments"));
+    for key in ["title", "description", "message", "flavor", "enabled"] { assert!(!screen.contains(form["labels"]["en"][key].as_str().unwrap()), "English {key} leaked into German form: {screen}"); }
+    term.checkpoint("form-before-start");
+    term.send(terminal::ENTER);
+    term.wait_for("CONFIGURED", SLOW).unwrap();
+    let task = wait_task(&ws, "configured form exited", |task| command_id(task) == form["commandId"].as_str().unwrap() && task["status"] == "exited");
+    let logs = ws.ok(&["logs", task["session"].as_str().unwrap()]);
+    let actual: serde_json::Value = serde_json::from_str(logs.lines().find_map(|line| line.strip_prefix("CONFIGURED ")).expect("configured receipt").trim()).unwrap();
+    assert_eq!(actual, form["expected"]);
+    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == form["commandId"].as_str().unwrap()).count(), 1);
+    term.checkpoint("form-received");
+    ws.assert_paths();
+    println!("[DEBUG] German PTY form received exact independent JSON receipt {actual}");
+}
+
+#[test]
+fn settings_changes_persist_when_the_view_is_reopened() {
+    let ws = Workspace::new("persisted settings");
+    let fixture = workspace::journey_fixture();
+    let mut term = open(&ws);
+    term.leader("p");
+    term.wait_for("Language: en", SLOW).unwrap();
+    term.send(terminal::ENTER);
+    term.wait_for("Sprache: de", SLOW).unwrap();
+    term.wait_for("Einstellungen gespeichert", SLOW).unwrap();
+    term.send(b"\x1b[B");
+    term.send(terminal::ENTER);
+    term.wait_for("Darstellung: light", SLOW).unwrap();
+    let journal = std::fs::read_to_string(ws.preference_path()).unwrap();
+    let events: Vec<serde_json::Value> = journal.lines().map(|line| serde_json::from_str(line).unwrap()).collect();
+    let changes: Vec<serde_json::Value> = events.iter().map(|event| event["change"].clone()).collect();
+    assert_eq!(serde_json::Value::Array(changes), fixture["preferences"]["events"]);
+    assert!(events.iter().enumerate().all(|(index, event)| event["version"] == 1 && event["revision"] == index + 1));
+    term.checkpoint("preferences-saved");
+    term.leader("d");
+    assert_eq!(term.exit_code(SLOW), Some(0));
+    let mut env = ws.env();
+    env.iter_mut().find(|(key, _)| key == "SEMIO_LOCALE").unwrap().1.clear();
+    let mut reopened = terminal::Terminal::spawn(&workspace::binary(), &[], &ws.root, &env, steps::ROWS, steps::COLS);
+    reopened.wait_for("Neue Aufgabe", SLOW).unwrap();
+    reopened.leader("p");
+    reopened.wait_for("Sprache: de", SLOW).unwrap();
+    reopened.wait_for("Darstellung: light", SLOW).unwrap();
+    assert_eq!(std::fs::read_to_string(ws.preference_path()).unwrap(), journal);
+    reopened.checkpoint("preferences-reopened");
+    ws.assert_paths();
+    println!("[DEBUG] visible settings wrote consecutive events and reopened German light preferences without environment overrides");
+}
+
+#[test]
+fn a_ready_server_opens_its_address_restarts_and_stops_from_the_view() {
+    let fixture = workspace::journey_fixture();
+    let flow = &fixture["readyLifecycle"];
+    let ws = Workspace::new("ready lifecycle");
+    let mut env = ws.env();
+    env.push(("SEMIO_DASHBOARD_URL_OPENER".into(), serde_json::to_string(&["bun".to_string(), workspace::fixture_script().display().to_string(), "open-url".to_string()]).unwrap()));
+    env.push(("JOURNEY_OPEN_RECORD".into(), ws.opened_url_path().display().to_string()));
+    let mut term = terminal::Terminal::spawn(&workspace::binary(), &[], &ws.root, &env, steps::ROWS, steps::COLS);
+    term.wait_for("New task", SLOW).unwrap();
+    start(&mut term, "serve-a", "journey server listening");
+    let before = wait_task(&ws, "ready server", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["readyUrl"].is_string());
+    let url = format!("http://127.0.0.1:{}{}", ws.port_a, flow["path"].as_str().unwrap());
+    assert_eq!(before["readyUrl"], url);
+    term.leader("h");
+    term.wait_for(&url, SLOW).unwrap();
+    term.send(b"\x1b[F");
+    term.leader(flow["openPrefix"].as_str().unwrap());
+    assert_eq!(wait_json(&ws.opened_url_path(), "opened URL"), serde_json::json!({ "url": url }));
+    term.wait_for(flow["openComplete"].as_str().unwrap(), SLOW).unwrap();
+    term.checkpoint("ready-opened");
+    term.send(b"\x1b[A");
+    term.send(terminal::ENTER);
+    term.wait_for("journey server listening", SLOW).unwrap();
+    term.leader("d");
+    assert_eq!(term.exit_code(SLOW), Some(0));
+    std::fs::remove_file(ws.opened_url_path()).unwrap();
+    env.push(("JOURNEY_OPEN_DELAY".into(), "10000".into()));
+    let bindings = serde_json::json!({ "prefix.open-ready": [flow["remappedOpenPrefix"].as_str().unwrap()], "prefix.cancel-open-ready": [flow["remappedCancelPrefix"].as_str().unwrap()] }).to_string();
+    term = terminal::Terminal::spawn(&workspace::binary(), &["--bindings", &bindings], &ws.root, &env, steps::ROWS, steps::COLS);
+    term.wait_for("New task", SLOW).unwrap();
+    restore(&mut term, "journey server listening");
+    term.leader(flow["remappedOpenPrefix"].as_str().unwrap());
+    let opener = wait_json(&ws.opened_url_path(), "slow opener process");
+    assert_eq!(opener["url"], url);
+    let pid = opener["pid"].as_u64().unwrap();
+    assert!(process_alive(pid));
+    term.leader(flow["remappedCancelPrefix"].as_str().unwrap());
+    term.wait_for(flow["openCancelled"].as_str().unwrap(), SLOW).unwrap();
+    let cancel_deadline = std::time::Instant::now() + SLOW;
+    while process_alive(pid) {
+        assert!(std::time::Instant::now() < cancel_deadline, "cancelled opener {pid} still exists");
+        std::thread::sleep(Duration::from_millis(50));
+    }
+    let retained = wait_task(&ws, "server survived opener cancellation", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "running");
+    assert_eq!(retained["pid"], before["pid"]);
+    term.checkpoint("ready-cancelled");
+    term.leader(flow["restartPrefix"].as_str().unwrap());
+    let after = wait_task(&ws, "restarted ready server", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["pid"].is_number() && task["pid"] != before["pid"] && task["readyUrl"] == url);
+    assert_eq!(before["session"], after["session"]);
+    assert_ne!(before["pid"], after["pid"]);
+    assert_eq!(Workspace::get(ws.port_a, flow["path"].as_str().unwrap()).unwrap(), (200, flow["response"].as_str().unwrap().to_string()));
+    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == flow["commandId"].as_str().unwrap()).count(), 1);
+    term.leader(flow["stopPrefix"].as_str().unwrap());
+    wait_task(&ws, "server stopped by UI", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
+    term.wait_until("the stopped server on screen", SLOW, shows_end).unwrap();
+    let deadline = std::time::Instant::now() + SLOW;
+    while Workspace::get(ws.port_a, flow["path"].as_str().unwrap()).is_ok() {
+        assert!(std::time::Instant::now() < deadline, "stopped server still accepts connections");
+        std::thread::sleep(Duration::from_millis(50));
+    }
+    term.checkpoint("ready-stopped");
+    ws.assert_paths();
+    println!("[DEBUG] UI ready opener received {url}; restart changed pid {} -> {} and UI stop released its endpoint", before["pid"], after["pid"]);
+}
+
+#[test]
+fn a_read_only_repository_action_runs_from_the_launcher() {
+    let fixture = workspace::journey_fixture();
+    let flow = &fixture["repository"];
+    let ws = Workspace::new("read only repository action");
+    let path = ws.root.join(".🧬semio/🦑️repo/🎯️goals").join(flow["goalId"].as_str().unwrap()).join("🎯️goal.json");
+    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
+    let bytes = serde_json::to_vec_pretty(&flow["document"]).unwrap();
+    std::fs::write(&path, &bytes).unwrap();
+    let mut term = open(&ws);
+    term.leader("n");
+    term.wait_for("commands", SLOW).unwrap();
+    term.type_text(flow["commandId"].as_str().unwrap());
+    term.wait_for("1 of", SLOW).unwrap();
+    term.send(terminal::ENTER);
+    let form = term.wait_for("[ Start ]", SLOW).unwrap();
+    term.checkpoint("repository-form");
+    assert!(form.contains("repo:goals.list"));
+    term.send(terminal::ENTER);
+    term.wait_for(flow["expectedLine"].as_str().unwrap(), SLOW).unwrap();
+    let task = wait_task(&ws, "repository action completed", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
+    assert_eq!(task["code"], 0);
+    assert_eq!(std::fs::read(&path).unwrap(), bytes);
+    let log = ws.ok(&["logs", task["session"].as_str().unwrap()]);
+    assert!(log.lines().any(|line| line.trim_end() == flow["expectedLine"].as_str().unwrap()));
+    term.checkpoint("repository-action");
+    ws.assert_paths();
+    println!("[DEBUG] read-only repo action showed {:?}, exited 0 and preserved the goal bytes", flow["expectedLine"]);
+}
+
+#[test]
+fn tabs_occupy_every_corner_and_own_fullscreen_and_close_controls() {
+    let fixture = workspace::journey_fixture();
+    let preview = &fixture["tabPreview"];
+    let ws = Workspace::new("corner-tab-controls");
+    let mut term = open(&ws);
+    let command_body = |text: &str| text.lines().any(|line| {
+        let mut words = line.trim_start_matches(|c| matches!(c, '┃' | '│' | '║' | '|' | ' ')).split_whitespace();
+        words.next().is_some_and(|word| word.parse::<usize>().is_ok()) && words.next() == Some("commands")
+    });
+    term.leader("n");
+    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
+    let locate = |term: &terminal::Terminal, label: &str, glyph: Option<char>| -> (usize, usize) {
+        let rows = term.rows();
+        let row = rows.iter().position(|line| line.contains(label)).unwrap();
+        let chars: Vec<char> = rows[row].chars().collect();
+        let title = rows[row].split_once(label).unwrap().0.chars().count();
+        let column = glyph.map_or(title + 1, |glyph| title + chars[title..].iter().position(|c| *c == glyph).unwrap());
+        (column + 1, row + 1)
+    };
+    for drop in preview["corners"].as_array().unwrap() {
+        let (x, y, right, bottom, label) = (drop["x"].as_u64().unwrap(), drop["y"].as_u64().unwrap(), drop["right"].as_bool().unwrap(), drop["bottom"].as_bool().unwrap(), drop["label"].as_str().unwrap());
+        let (column, row) = locate(&term, "Commands", None);
+        term.send(format!("\x1b[<0;{column};{row}M\x1b[<32;{x};{y}M").as_bytes());
+        let proposed = term.wait_until("the exact proposed destination tab", SLOW, |text| text.contains(&format!("tab at {label}")) && tab_chip(text, preview["label"].as_str().unwrap(), right, bottom).is_some()).unwrap();
+        let chip = tab_chip(&proposed, preview["label"].as_str().unwrap(), right, bottom).unwrap();
+        assert_eq!(chip.2.len(), preview["rows"].as_u64().unwrap() as usize);
+        for control in preview["controls"].as_array().unwrap() { assert!(chip.2[1].contains(control.as_str().unwrap()), "missing preview control: {proposed}"); }
+        term.send(format!("\x1b[<0;{x};{y}m").as_bytes());
+        term.wait_until("the committed corner tab agrees with its preview", SLOW, |text| !text.contains("Esc cancels") && tab_chip(text, preview["label"].as_str().unwrap(), right, bottom).as_ref() == Some(&chip)).unwrap();
+        println!("[DEBUG] independent VT100 committed {label} tab after its visible slot preview");
+    }
+    let (x, y) = locate(&term, "Tasks", None);
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    let (x, y) = locate(&term, "Commands", Some('⤢'));
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    term.wait_for("⤡", SLOW).unwrap();
+    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
+    let (x, y) = locate(&term, "Tasks", None);
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    term.wait_until("overview active inside the fullscreen group", SLOW, |text| text.contains("⤡") && !command_body(text)).unwrap();
+    let (x, y) = locate(&term, "Commands", None);
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
+    assert!(term.text().contains("⤡"));
+    let (x, y) = locate(&term, "Commands", Some('⤡'));
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    term.wait_for("⤢", SLOW).unwrap();
+    let (x, y) = locate(&term, "Tasks", None);
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    let (x, y) = locate(&term, "Commands", Some('✕'));
+    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
+    term.wait_until("only the neighbouring overview remains", SLOW, |text| text.contains("Tasks") && !text.contains("Commands") && !command_body(text)).unwrap();
+    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
+    println!("[DEBUG] per-tab fullscreen focused the inactive command body; restore retained it and close removed only its identity");
+}
+
+#[test]
+fn tabs_dock_with_a_preview_and_escape_cancels_the_drag() {
+    let fixture = workspace::journey_fixture();
+    let preview = &fixture["tabPreview"];
+    let ws = Workspace::new("window-docking");
+    let mut term = open(&ws);
+    term.leader("n");
+    term.wait_for("commands", SLOW).unwrap();
+    std::thread::sleep(Duration::from_millis(250));
+    let rows = term.rows();
+    let row = rows.iter().position(|line| line.contains("Tasks") && line.contains("Commands")).expect("both tab chips are visible");
+    let column = rows[row].split_once("Commands").unwrap().0.chars().count() + 2;
+    let (drop_x, drop_y) = (preview["split"]["x"].as_u64().unwrap(), preview["split"]["y"].as_u64().unwrap());
+    let held = format!("\x1b[<0;{column};{}M\x1b[<32;{drop_x};{drop_y}M", row + 1);
+    term.send(held.as_bytes());
+    let proposed = term.wait_until("the proposed right split tab", SLOW, |text| text.contains(preview["split"]["label"].as_str().unwrap()) && tab_chip(text, preview["label"].as_str().unwrap(), true, false).is_some()).unwrap();
+    let chip = tab_chip(&proposed, preview["label"].as_str().unwrap(), true, false).unwrap();
+    for control in preview["controls"].as_array().unwrap() { assert!(chip.2[1].contains(control.as_str().unwrap()), "missing split preview control: {proposed}"); }
+    term.send(b"\x1b");
+    term.send(format!("\x1b[<0;{drop_x};{drop_y}m").as_bytes());
+    std::thread::sleep(Duration::from_millis(250));
+    assert!(term.rows().iter().any(|line| line.contains("Tasks") && line.contains("Commands")), "Escape committed a split: {}", term.text());
+    assert!(tab_chip(&term.text(), preview["label"].as_str().unwrap(), true, false).is_none(), "Escape retained the proposed tab: {}", term.text());
+    term.send(held.as_bytes());
+    term.wait_until("the right split tab preview", SLOW, |text| tab_chip(text, preview["label"].as_str().unwrap(), true, false).as_ref() == Some(&chip)).unwrap();
+    term.send(format!("\x1b[<0;{drop_x};{drop_y}m").as_bytes());
+    let borders = |text: &str| -> Vec<usize> { text.lines().nth(16).unwrap_or_default().chars().enumerate().filter_map(|(x, c)| matches!(c, '┃' | '│' | '║' | '|').then_some(x)).collect() };
+    let split = term.wait_until("two independently tiled windows", SLOW, |text| borders(text).len() == 4).unwrap();
+    assert_eq!(tab_chip(&split, preview["label"].as_str().unwrap(), true, false), Some(chip));
+    let divider = borders(&split)[2];
+    term.send(format!("\x1b[<0;{divider};17M").as_bytes());
+    for delta in [3, 7, 5] {
+        term.send(format!("\x1b[<32;{};17M", divider + delta).as_bytes());
+        term.wait_until("the divider following its held pointer", SLOW, |text| borders(text).get(2) == Some(&(divider + delta))).unwrap();
+    }
+    term.send(format!("\x1b[<0;{};17m", divider + 5).as_bytes());
+    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
+    println!("[DEBUG] independent VT100 screen confirmed preview, cancellation, committed side-by-side windows and continuous divider resizing");
+}
+
+#[test]
+fn a_live_server_moves_between_split_axes_and_tabs_without_restarting() {
+    let ws = Workspace::new("live-window-docking");
+    let mut term = open(&ws);
+    start(&mut term, "serve-a", "journey server listening");
+    let before = wait_task(&ws, "the live server", |task| command_id(task) == "tool:journey-fixture/serve-a");
+    assert!(before["pid"].is_number());
+    let hold = |term: &mut terminal::Terminal, x: usize, y: usize| {
+        let rows = term.rows();
+        let row = rows.iter().position(|line| line.contains("serve-a")).unwrap();
+        let column = rows[row].split_once("serve-a").unwrap().0.chars().count() + 2;
+        term.send(format!("\x1b[<0;{column};{}M\x1b[<32;{x};{y}M", row + 1).as_bytes());
+    };
+    hold(&mut term, 118, 17);
+    term.wait_for("split right", SLOW).unwrap();
+    term.send(b"\x1b[<0;118;17m");
+    term.wait_until("the server beside the task overview", SLOW, |text| text.lines().any(|line| line.contains("Tasks") && line.contains("serve-a")) && text.lines().nth(16).unwrap_or_default().chars().filter(|c| matches!(*c, '┃' | '│' | '║' | '|')).count() == 4).unwrap();
+    hold(&mut term, 30, 5);
+    term.wait_for("split above", SLOW).unwrap();
+    term.send(b"\x1b[<0;30;5m");
+    term.wait_until("the server above the task overview", SLOW, |text| {
+        let lines: Vec<_> = text.lines().collect();
+        match (lines.iter().position(|line| line.contains("serve-a")), lines.iter().position(|line| line.contains("Tasks"))) { (Some(server), Some(overview)) => server < overview, _ => false }
+    }).unwrap();
+    let rows = term.rows();
+    let target_row = rows.iter().position(|line| line.contains("Tasks")).unwrap();
+    let target_x = rows[target_row].split_once("Tasks").unwrap().0.chars().count() + 2;
+    let target_y = target_row + 1;
+    hold(&mut term, target_x, target_y);
+    term.wait_for("tab at top left", SLOW).unwrap();
+    term.send(format!("\x1b[<0;{target_x};{target_y}m").as_bytes());
+    term.wait_until("the server rejoined as a tab", SLOW, |text| text.lines().any(|line| line.contains("Tasks") && line.contains("serve-a"))).unwrap();
+    let after = wait_task(&ws, "the same live server", |task| command_id(task) == "tool:journey-fixture/serve-a");
+    assert_eq!(before["session"], after["session"]);
+    assert_eq!(before["pid"], after["pid"]);
+    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == "tool:journey-fixture/serve-a").count(), 1);
+    assert!(ws.ok(&["logs", after["session"].as_str().unwrap()]).contains("journey server listening"));
+    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
+    println!("[DEBUG] independent VT100, daemon task list and logs confirmed a live process survived right/top/tab docking");
+}
+
+#[test]
+#[cfg(windows)]
+fn the_terminal_driver_reports_physical_emoji_widths() {
+    let ws = Workspace::new("physical-width-probe");
+    let receipt = ws.root.join("width.json");
+    let mut environment = ws.env();
+    environment.push(("JOURNEY_WIDTH_RECORD".into(), receipt.display().to_string()));
+    let script = workspace::fixture_script();
+    let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &[script.to_str().unwrap(), "width-probe"], &ws.root, &environment, 20, 240);
+    let fixture = workspace::journey_fixture();
+    let mut measurements = Vec::new();
+    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
+    let widths: serde_json::Value = serde_json::from_slice(&std::fs::read(repository.join(fixture["widthProbe"]["fixture"].as_str().unwrap())).unwrap()).unwrap();
+    for sample in widths["cases"].as_array().unwrap() {
+        for model in ["scalar", "cluster"] {
+            let text = sample[if model == "scalar" { "scalarText" } else { "text" }].as_str().unwrap();
+            let deadline = std::time::Instant::now() + SLOW;
+            let native = loop {
+                if let Ok(bytes) = std::fs::read(&receipt) {
+                    if let Ok(records) = serde_json::from_slice::<Vec<serde_json::Value>>(&bytes) {
+                        if records.len() == measurements.len() + 1 { break records.last().unwrap().clone(); }
+                    }
+                }
+                assert!(std::time::Instant::now() < deadline, "physical cursor receipt missing: {}", String::from_utf8_lossy(&term.raw_bytes()));
+                std::thread::sleep(Duration::from_millis(10));
+            };
+            term.wait_for(text, SLOW).unwrap();
+            let (_, (row, column)) = term.cursor();
+            assert_eq!(native["id"], sample["id"]);
+            assert_eq!(native["model"], model);
+            assert_eq!(native["columns"], 240);
+            assert_eq!(native["y"], 0);
+            assert_eq!(row, 0);
+            assert_eq!(native["x"], sample[if model == "scalar" { "windowsScalarCells" } else { "windowsClusterCells" }]);
+            assert_eq!(u64::from(column), native["x"].as_u64().unwrap());
+            let mut measured = native;
+            measured["vt100X"] = column.into();
+            println!("[DEBUG] actual ConPTY cursor {measured}");
+            measurements.push(measured);
+            term.send(terminal::ENTER);
+        }
+    }
+    term.wait_for("WIDTH-PROBE-DONE", SLOW).unwrap();
+    if let Some(directory) = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR") { std::fs::write(std::path::Path::new(&directory).join("physical-cursor-measurements.json"), serde_json::to_vec_pretty(&measurements).unwrap()).unwrap(); }
+    term.checkpoint("physical-cursor");
+    ws.assert_paths();
+}
+
 #[test]
 fn the_terminal_driver_preserves_committed_unicode_text() {
     let ws = Workspace::new("unicode-probe");
     let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &["-e", "process.stdin.setRawMode(true);process.stdin.on('data',data=>process.stdout.write(data));console.log('ready');"], &ws.root, &ws.env(), 20, 120);
-    term.wait_for("ready", Duration::from_secs(3)).unwrap();
+    term.wait_for("ready", SLOW).unwrap();
     let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
     let vectors: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(repository.join("🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/⌨️input-decoding/🔣️.json")).unwrap()).unwrap();
@@ -46,19 +579,25 @@

     let row = &term.rows()[usize::from(end.0)];
     let (prefix, _) = row.split_once("A界e\u{301}").expect("the visible input row contains the typed text");
-    assert_eq!(end.1, prefix.chars().count() as u16 + 4, "the hardware caret follows all four visible text columns");
+    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
+    let fixture = workspace::journey_fixture();
+    let widths: serde_json::Value = serde_json::from_slice(&std::fs::read(repository.join(fixture["widthProbe"]["fixture"].as_str().unwrap())).unwrap()).unwrap();
+    let combining = widths["cases"].as_array().unwrap().iter().find(|sample| sample["id"] == "combining").unwrap();
+    let combining_cells = combining[if cfg!(windows) { "windowsScalarCells" } else { "scalarCells" }].as_u64().unwrap() as u16;
+    let start_column = prefix.chars().count() as u16;
+    assert_eq!(end.1, start_column + 3 + combining_cells, "the hardware caret follows the independently measured text columns");
     term.send(b"\x1b[D");
-    term.wait_until("the caret crossing one combining cluster", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, end.1 - 1))).unwrap();
+    term.wait_until("the caret crossing one combining cluster", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, start_column + 3))).unwrap();
     let (shown, left) = term.cursor();
     assert!(shown);
-    assert_eq!(left, (end.0, end.1 - 1), "left crosses one combining cluster");
+    assert_eq!(left, (end.0, start_column + 3), "left crosses one complete combining cluster");
     term.type_text("x");
     term.wait_for("A界xe\u{301}", SLOW).unwrap();
     term.send(b"\x7f\x1b[3~\x1b[H\x1b[200~Z\nQ\x1b[201~");
     term.wait_for("Z QA界", SLOW).unwrap();
-    term.wait_until("the pasted caret", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, end.1 - 1))).unwrap();
+    term.wait_until("the pasted caret", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, start_column + 3))).unwrap();
     let (shown, pasted) = term.cursor();
     assert!(shown);
-    assert_eq!(pasted, (end.0, end.1 - 1), "the three pasted columns precede the remaining A界");
-    let (column, row) = (end.1 - 4 + 1, end.0 + 1);
+    assert_eq!(pasted, (end.0, start_column + 3), "the three pasted columns precede the remaining A界");
+    let (column, row) = (start_column + 1, end.0 + 1);
     term.send(format!("\x1b[<0;{column};{row}M\x1b[<0;{column};{row}m").as_bytes());
     std::thread::sleep(Duration::from_millis(250));
@@ -208,5 +747,6 @@

     let ws = Workspace::new("fixture-resize");
     let bun = workspace::which("bun");
-    let mut term = terminal::Terminal::spawn(&bun, &["📜️script.ts", "resize"], &ws.root, &ws.env(), 20, 70);
+    let script = workspace::fixture_script().display().to_string();
+    let mut term = terminal::Terminal::spawn(&bun, &[&script, "resize"], &ws.root, &ws.env(), 20, 70);
     term.wait_for("SIZE 70x20", SLOW).unwrap_or_else(|screen| panic!("{screen}"));
     term.resize(25, 90);

### 34 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🧰️support/🟦️.ts
HEAD 141 BASE 141 STASH 143 CURRENT H
--- 

+++ 

@@ -5,5 +5,5 @@

  * @see ../../../🧫️fixtures/🧭️journeys/🏗️workspace/📋️project.json
  */
-import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
+import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
 import { createServer } from "node:net";
 import { tmpdir } from "node:os";
@@ -68,18 +68,20 @@

 const PORT_B = "47102";
 
-/** 🏗️ A copy of the fixture workspace with free ports and its own daemon; `close` stops that daemon and removes the copy. */
+/** 🏗️ Isolated fixture metadata with free ports, its own daemon and immutable source behavior. */
 export class Workspace {
   private constructor(readonly root: string, readonly portA: number, readonly portB: number, private readonly runtime: string) {}
 
   static async open(name: string, options: { bulkTools?: number } = {}): Promise<Workspace> {
-    const base = mkdtempSync(join(tmpdir(), `semio-battle-${name}-`));
-    const root = join(base, "workspace");
+    const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir());
+    mkdirSync(artifacts, { recursive: true });
+    const base = mkdtempSync(join(artifacts, "j-"));
+    const root = join(base, "w");
+    if (root.length > 152) throw new Error("journey roots must leave room for native logs within 256 UTF-16 units");
     mkdirSync(join(root, ".git"), { recursive: true });
     const [portA, portB] = [await freePort(), await freePort()];
-    const source = join(dashboard, "🧫️fixtures/🧭️journeys/🏗️workspace");
-    for (const entry of readdirSync(source)) {
-      const text = readFileSync(join(source, entry), "utf8").replaceAll(PORT_A, String(portA)).replaceAll(PORT_B, String(portB));
-      writeFileSync(join(root, entry), text);
-    }
+    const source = join(dashboard, "🧪️tests/🧫️fixtures/🧭️journeys/🏗️workspace");
+    const manifest = JSON.parse(readFileSync(join(source, "📋️project.json"), "utf8").replaceAll(PORT_A, String(portA)).replaceAll(PORT_B, String(portB)));
+    for (const tool of manifest.metadata.semio.dashboard.tools) tool.command = tool.command.map((argument: string) => argument === "{workspace}/📜️script.ts" ? join(source, "📜️script.ts") : argument);
+    writeFileSync(join(root, "📋️project.json"), JSON.stringify(manifest));
     if (options.bulkTools) {
       mkdirSync(join(root, "bulk"));
@@ -87,10 +89,10 @@

       writeFileSync(join(root, "bulk/📋️project.json"), JSON.stringify({ name: "bulk", metadata: { semio: { dashboard: { tools } } } }));
     }
-    const runtime = join(base, "runtime");
+    const runtime = join(base, "r");
     mkdirSync(runtime);
     return new Workspace(root, portA, portB, runtime);
   }
 
-  get env(): Record<string, string> { return { SEMIO_DASHBOARD_RUNTIME_DIR: this.runtime, SEMIO_LOCALE: "en" }; }
+  get env(): Record<string, string> { return { SEMIO_DASHBOARD_RUNTIME_DIR: this.runtime, SEMIO_DASHBOARD_INSTANCE: "", SEMIO_DASHBOARD_CONFIG: join(this.runtime, "p.jsonl"), SEMIO_LOCALE: "en", SEMIO_APPEARANCE: "", SEMIO_LOCKED_TERMINOLOGY: "", SEMIO_DASHBOARD_RENDERER: "", SEMIO_DASHBOARD_LAYOUT: "", SEMIO_DASHBOARD_PREFIX: "", SEMIO_REPO_IMPLEMENTATION: "rust" }; }
 
   semio(args: string[], options: { env?: Record<string, string>; timeoutMs?: number } = {}): Promise<Run> {

### 35 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json
HEAD 46 BASE 20 STASH 252 CURRENT H
--- 

+++ 

@@ -3,16 +3,248 @@

   "$id": "https://json.schemas.assets.semio-tech.coms/repo/dashboard/authority.json",
   "title": "🧭️ Developer control plane authority",
+  "$defs": {
+    "DevcontainerRuntime": {
+      "type": "object",
+      "description": "🐳️ The runtime container provisions toolchains and retained headless agents while lifecycle commands remain owned by the build registry.",
+      "required": [
+        "postCreateCommand",
+        "postStartCommand",
+        "postAttachCommand"
+      ],
+      "not": {
+        "anyOf": [
+          {
+            "required": [
+              "customizations"
+            ]
+          },
+          {
+            "required": [
+              "extensions"
+            ]
+          },
+          {
+            "required": [
+              "settings"
+            ]
+          }
+        ]
+      },
+      "properties": {
+        "postCreateCommand": {
+          "const": [
+            "bun",
+            "repo",
+            "run",
+            "workspace:setup"
+          ]
+        },
+        "postStartCommand": {
+          "const": [
+            "bun",
+            "repo",
+            "run",
+            "workspace:setup-devcontainer-start"
+          ]
+        },
+        "postAttachCommand": {
+          "const": [
+            "bun",
+            "repo",
+            "run",
+            "workspace:setup-devcontainer-attach"
+          ]
+        },
+        "mounts": {
+          "type": "array",
+          "items": {
+            "type": "string",
+            "not": {
+              "pattern": "target=[^,]*/\\.(codeium|kiro|vscode|vscode-insiders|cursor|windsurf|idea|fleet|zed|atom|brackets)(/|,|$)"
+            }
+          }
+        }
+      }
+    },
+    "OwnerCommand": {
+      "type": "string",
+      "description": "📜️ One owner invokes a permanent canonical script through Bun with an explicit command; shell pipelines, substitutions and alternate task executors are forbidden.",
+      "not": {
+        "pattern": "[$`]"
+      },
+      "pattern": "^bun (?:--watch )?(?:\"(?:[^\"\\r\\n]*[/\\\\])?📜️script\\.ts\"|(?:[^\\s\"\\r\\n]*[/\\\\])?📜️script\\.ts) [a-z][a-z0-9-]*(?: (?:[^\\s\"';&|<>\\r\\n]+|\"[^\"\\r\\n]*\"|'[^'\\r\\n]*')+)*$"
+    },
+    "OwnerTarget": {
+      "type": "object",
+      "description": "🎯️ Runnable build targets invoke canonical scripts; dependency-only aggregators use the repository noop contract.",
+      "properties": {
+        "executor": {
+          "enum": [
+            "repo:script",
+            "repo:noop"
+          ]
+        },
+        "command": {
+          "$ref": "#/$defs/OwnerCommand"
+        },
+        "options": {
+          "$ref": "#/$defs/OwnerOptions"
+        },
+        "configurations": {
+          "type": "object",
+          "additionalProperties": {
+            "$ref": "#/$defs/OwnerOptions"
+          }
+        }
+      },
+      "anyOf": [
+        {
+          "required": [
+            "executor"
+          ],
+          "properties": {
+            "executor": {
+              "const": "repo:noop"
+            }
+          }
+        },
+        {
+          "required": [
+            "command"
+          ]
+        },
+        {
+          "required": [
+            "options"
+          ],
+          "properties": {
+            "options": {
+              "anyOf": [
+                {
+                  "required": [
+                    "command"
+                  ]
+                },
+                {
+                  "required": [
+                    "commands"
+                  ]
+                }
+              ]
+            }
+          }
+        }
+      ]
+    },
+    "OwnerOptions": {
+      "type": "object",
+      "properties": {
+        "command": {
+          "$ref": "#/$defs/OwnerCommand"
+        },
+        "commands": {
+          "type": "array",
+          "minItems": 1,
+          "items": {
+            "anyOf": [
+              {
+                "$ref": "#/$defs/OwnerCommand"
+              },
+              {
+                "type": "object",
+                "required": [
+                  "command"
+                ],
+                "properties": {
+                  "command": {
+                    "$ref": "#/$defs/OwnerCommand"
+                  }
+                }
+              }
+            ]
+          }
+        }
+      }
+    },
+    "DeveloperInstructions": {
+      "type": "string",
+      "description": "📖️ Lowercase developer instructions use the dashboard and never prescribe retired IDE configuration or editor startup.",
+      "not": {
+        "anyOf": [
+          {
+            "pattern": "\\.(vscode(?:-insiders)?|idea|fleet|zed|vs|run|nova|settings|metadata|windsurf|kiro|atom|brackets)(/|\\\\)"
+          },
+          {
+            "pattern": "\\.cursor[/\\\\](?:mcp|hooks|settings)\\.json|\\.cursor[/\\\\]rules(?:[/\\\\]|$)|\\.(cursorrules|windsurfrules)\\b|\\.sublime-(project|workspace|build)\\b|cmakesettings\\.json"
+          },
+          {
+            "pattern": "\\.cursor[/\\\\](?!plans(?:[/\\\\]|$))"
+          },
+          {
+            "pattern": "ide startup|reopen in vs code|all required vs code extensions|dev containers: reopen in container|vs code.*with.*dev containers extension"
+          }
+        ]
+      }
+    },
+    "ActiveRepositoryPath": {
+      "type": "string",
+      "description": "🚫️ A normalized lowercase active repository path. IDE directories, workspace/solution files and Cursor MCP/hooks/settings/rules have no authority; compiler project files and historical Cursor plans remain valid.",
+      "not": {
+        "anyOf": [
+          {
+            "pattern": "(^|/)\\.(vscode|vscode-insiders|idea|fleet|zed|vs|run|nova|settings|metadata|windsurf|kiro|atom|brackets)(/|$)"
+          },
+          {
+            "pattern": "\\.(sln|slnx|code-workspace|iml|ipr|iws|suo|user|userosscache|sln.docstates|sublime-project|sublime-workspace|sublime-build)$"
+          },
+          {
+            "pattern": "(^|/)\\.(project|classpath|vsconfig|cursorrules|windsurfrules)$"
+          },
+          {
+            "pattern": "(^|/)cmakesettings\\.json$"
+          },
+          {
+            "pattern": "(^|/)\\.cursor/(mcp|hooks|settings)\\.json$"
+          },
+          {
+            "pattern": "(^|/)\\.cursor/(?!plans(?:/|$))"
+          },
+          {
+            "pattern": "(^|/)\\.devcontainer/editor-overlay\\.json$"
+          },
+          {
+            "pattern": "(^|/)\\.cursor/rules(/|$)"
+          }
+        ]
+      }
+    }
+  },
   "type": "object",
-  "required": ["scripts"],
+  "required": [
+    "scripts"
+  ],
   "properties": {
     "scripts": {
       "type": "object",
-      "required": ["repo", "setup", "dashboard", "dashboard:install"],
+      "required": [
+        "repo",
+        "setup",
+        "dashboard",
+        "dashboard:install"
+      ],
       "additionalProperties": false,
       "properties": {
-        "repo": {"const": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo"},
-        "setup": {"const": "bun repo run workspace:setup"},
-        "dashboard": {"const": "bun repo run @semio-tech/repo-dashboard-rs:run --output=stream"},
-        "dashboard:install": {"const": "bun repo run @semio-tech/repo-dashboard-rs:build --exclude-dependencies --skip-cache --output=stream"}
+        "repo": {
+          "const": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo"
+        },
+        "setup": {
+          "const": "bun repo run workspace:setup"
+        },
+        "dashboard": {
+          "const": "bun repo run @semio-tech/repo-dashboard-rs:run --output=stream"
+        },
+        "dashboard:install": {
+          "const": "bun repo run @semio-tech/repo-dashboard-rs:build --exclude-dependencies --skip-cache --output=stream"
+        }
       }
     }

### 36 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/📦️packages/🦀️rust/Cargo.toml
HEAD 34 BASE 34 STASH 35 CURRENT H
--- 

+++ 

@@ -23,4 +23,5 @@

 
 [dependencies]
+ui_tui = { path = "../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust", features = ["tui-terminal"], package = "semio-framework-ui" }
 dashboard_daemon = { path = "../../../🌀️daemon/📦️packages/🦀️rust", package = "semio-framework-repo-dashboard-daemon" }
 dashboard_inventory = { path = "../../../📚️inventory/📦️packages/🦀️rust", package = "semio-framework-repo-dashboard-inventory" }

### 37 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs
HEAD 459 BASE 458 STASH 452 CURRENT H
--- 

+++ 

@@ -19,4 +19,7 @@

 use std::sync::atomic::AtomicBool;
 use std::time::{Duration, Instant};
+
+#[path = "🌐️open/🦀️.rs"]
+pub mod browser;
 
 // #region 🔖️Arguments
@@ -311,8 +314,4 @@

     let launch = match registry.resolve(id, &request) { Ok(launch) => launch, Err(error) => return refuse(&error) };
     if arguments.has("dry-run") { println!("{}", registry.launch_plan_json(&launch)); return 0; }
-    if let Some(action) = registry.find(id).and_then(|entry| entry.repo_action()).filter(|_| RunPolicy::current().repo == RepoImplementation::Rust) {
-        println!("{}", action.execute(&root));
-        return 0;
-    }
     let result = connect_or_start(&root).and_then(|mut connection| start(&mut connection, SpawnGroup::from_launch(&launch, &[]), &arguments));
     result.unwrap_or_else(|error| refuse(&error))
@@ -440,13 +439,8 @@

     let sessions = sessions_of(&root_of(root, &arguments));
     let session = match pick(&sessions, needle).and_then(|picked| { let ready: Vec<&SessionInfo> = picked.sessions.iter().copied().filter(|session| session.ready_url.is_some()).collect(); only(&Picked { sessions: if ready.len() == 1 { ready } else { picked.sessions }, group: None }, needle) }) { Ok(session) => session, Err(error) => return refuse(&error) };
-    let Some(url) = session.ready_url.as_deref().filter(|_| session.status != SessionStatus::Interrupted) else { return refuse(&format!("{} has no ready address", label_text(session))) };
+    let Some(url) = browser::ready_url(session.status, session.ready_url.as_deref()) else { return refuse(&format!("{} has no ready address", label_text(session))) };
     println!("{url}");
     if arguments.has("print") { return 0; }
-    match browser(url) { Ok(()) => 0, Err(error) => refuse(&format!("cannot open a browser: {error}")) }
-}
-
-fn browser(url: &str) -> std::io::Result<()> {
-    let mut command = if cfg!(windows) { let mut command = std::process::Command::new("cmd"); command.args(["/c", "start", "", url]); command } else if cfg!(target_os = "macos") { let mut command = std::process::Command::new("open"); command.arg(url); command } else { let mut command = std::process::Command::new("xdg-open"); command.arg(url); command };
-    command.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().map(|_| ())
+    match browser::open(url) { Ok(()) => 0, Err(error) => refuse(&format!("cannot open a browser: {error}")) }
 }
 // #endregion 🔖️Tasks

### 38 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/🐹️.go
HEAD 404 BASE 6245 STASH 6274 CURRENT H
--- 

+++ 

@@ -3797,4 +3797,5 @@

 	nested  []goMember
 	strings []string
+	objects [][]goMember
 	kind    int
 }
@@ -3814,4 +3815,10 @@

 		case 1:
 			lines = append(lines, indent+GoJSONString(member.name)+": "+renderGoMembers(member.nested, depth+1))
+		case 3:
+			if len(member.objects) == 0 { lines = append(lines, indent+GoJSONString(member.name)+": []"); continue }
+			elementIndent := strings.Repeat("  ", depth+2)
+			elements := make([]string, 0, len(member.objects))
+			for _, object := range member.objects { elements = append(elements, elementIndent+renderGoMembers(object, depth+2)) }
+			lines = append(lines, indent+GoJSONString(member.name)+": [\n"+strings.Join(elements, ",\n")+"\n"+indent+"]")
 		default:
 			if len(member.strings) == 0 {
@@ -4038,5 +4045,27 @@

 		members = append(members, goMember{name: "sessions", strings: ticket.Sessions, kind: 2})
 	}
+	if len(ticket.Interactions) > 0 {
+		objects := make([][]goMember, 0, len(ticket.Interactions))
+		for _, interaction := range ticket.Interactions { objects = append(objects, interactionMembers(interaction)) }
+		members = append(members, goMember{name: "interactions", objects: objects, kind: 3})
+	}
 	return renderGoMembers(members, 0)
+}
+
+// 🧾️ Encodes durable interaction records in the canonical model field order.
+func interactionMembers(interaction model.Interaction) []goMember {
+	members := []goMember{{name: "kind", text: interaction.Kind}, {name: "date", text: interaction.Date}, {name: "author", text: interaction.Author}, {name: "system", text: interaction.System}, {name: "client", text: interaction.Client}, {name: "checkpoint", text: interaction.Checkpoint}}
+	for _, field := range []goMember{{name: "prompt", text: interaction.Prompt}, {name: "summary", text: interaction.Summary}, {name: "llm", text: interaction.LLM}, {name: "effort", text: interaction.Effort}} { if field.text != "" { members = append(members, field) } }
+	if len(interaction.Files) > 0 {
+		objects := make([][]goMember, 0, len(interaction.Files))
+		for _, file := range interaction.Files {
+			fields := []goMember{{name: "path", text: file.Path}}
+			if file.ID != "" { fields = append(fields, goMember{name: "id", text: file.ID}) }
+			if file.URI != "" { fields = append(fields, goMember{name: "uri", text: file.URI}) }
+			objects = append(objects, fields)
+		}
+		members = append(members, goMember{name: "files", objects: objects, kind: 3})
+	}
+	return members
 }
 

### 39 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🐹️.go
HEAD 274 BASE 1851 STASH 1957 CURRENT H
--- 

+++ 

@@ -963,4 +963,109 @@

 }
 
+// 🛡️ IsIdeConfigurationPath identifies editor-owned configuration from dashboard authority.
+func IsIdeConfigurationPath(path string) bool {
+	parts := strings.Split(strings.ToLower(NormalizePath(path)), "/")
+	for index, part := range parts {
+		if part == ".cursor" && index+1 < len(parts) && parts[index+1] == "rules" {
+			return true
+		}
+		switch part {
+		case ".vscode", ".idea", ".fleet", ".zed", ".vs", ".run", ".nova", ".settings", ".metadata", ".windsurf":
+			return true
+		}
+	}
+	name := parts[len(parts)-1]
+	if name == ".project" || name == ".classpath" || name == ".vsconfig" || name == "cmakesettings.json" {
+		return true
+	}
+	for _, suffix := range []string{".sln", ".slnx", ".code-workspace", ".iml", ".ipr", ".iws", ".suo", ".user", ".userosscache", ".sln.docstates"} {
+		if strings.HasSuffix(name, suffix) {
+			return true
+		}
+	}
+	if len(parts) > 1 {
+		parent := parts[len(parts)-2]
+		return parent == ".cursor" && (name == "mcp.json" || name == "hooks.json" || name == "settings.json") || parent == ".devcontainer" && name == "editor-overlay.json"
+	}
+	return false
+}
+
+// 🧹️ IdeConfigurationPaths visits active physical owners without entering generated or historical files.
+func IdeConfigurationPaths(root string, scope string) ([]string, error) {
+	paths := []string{}
+	base := filepath.Join(root, filepath.FromSlash(scope))
+	err := filepath.WalkDir(base, func(path string, entry fs.DirEntry, err error) error {
+		if err != nil {
+			return err
+		}
+		if entry.Type()&os.ModeSymlink != 0 {
+			return nil
+		}
+		relative, err := filepath.Rel(root, path)
+		if err != nil {
+			return err
+		}
+		relative = filepath.ToSlash(relative)
+		if entry.IsDir() {
+			name := strings.TrimLeftFunc(strings.ReplaceAll(entry.Name(), "\uFE0F", ""), func(value rune) bool {
+				return value != '.' && (value < 'a' || value > 'z') && (value < 'A' || value > 'Z') && (value < '0' || value > '9')
+			})
+			switch strings.ToLower(name) {
+			case ".git", "node_modules", "target", "dist", "build", "generated", "cache", ".nx":
+				return filepath.SkipDir
+			}
+			if strings.Contains(relative, ".🧬semio/🦑️repo/🎫️tickets") {
+				return filepath.SkipDir
+			}
+		}
+		if IsIdeConfigurationPath(relative) {
+			paths = append(paths, relative)
+			if entry.IsDir() {
+				return filepath.SkipDir
+			}
+		}
+		return nil
+	})
+	return paths, err
+}
+
+// 🏗️ DotnetFormatterProjects finds the nearest project owner within the repository.
+func DotnetFormatterProjects(root string, relPath string) []string {
+	projects := []string{}
+	absolute, err := filepath.Abs(root)
+	if err != nil {
+		return projects
+	}
+	source := filepath.Join(absolute, filepath.FromSlash(relPath))
+	if filepath.IsAbs(relPath) {
+		source = filepath.Clean(relPath)
+	}
+	relative, err := filepath.Rel(absolute, source)
+	if err != nil || relative == ".." || strings.HasPrefix(relative, ".."+string(filepath.Separator)) {
+		return projects
+	}
+	for directory := filepath.Dir(source); ; directory = filepath.Dir(directory) {
+		identity, err := os.Lstat(directory)
+		if err != nil || !identity.IsDir() || identity.Mode()&os.ModeSymlink != 0 {
+			return projects
+		}
+		entries, err := os.ReadDir(directory)
+		if err != nil {
+			return projects
+		}
+		for _, entry := range entries {
+			if entry.Type().IsRegular() && strings.EqualFold(filepath.Ext(entry.Name()), ".csproj") {
+				project, err := filepath.Rel(absolute, filepath.Join(directory, entry.Name()))
+				if err == nil {
+					projects = append(projects, filepath.ToSlash(project))
+				}
+			}
+		}
+		if len(projects) > 0 || directory == absolute {
+			return projects
+		}
+	}
+}
+
 // 🗣️formatterPlansForLanguage holds the data fields for a formatterPlansForLanguage record.
 func FormatterPlansForLanguage(languageName string, relPath string) []formatterPlan {
@@ -985,8 +1090,9 @@

 		}
 	case "csharp":
-		return []formatterPlan{
-			{Binary: "dotnet", Args: []string{"format", "Monorepo.sln", "--include", relPath, "--verbosity", "minimal"}, requirement: []string{"Monorepo.sln"}},
-			prettierPlan,
-		}
+		plans := []formatterPlan{}
+		for _, project := range DotnetFormatterProjects(RootDir, relPath) {
+			plans = append(plans, formatterPlan{Binary: "dotnet", Args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
+		}
+		return append(plans, prettierPlan)
 	case "rust":
 		return []formatterPlan{

### 40 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/📜️script.ts
HEAD 22 BASE 22 STASH 22 CURRENT H
--- 

+++ 

@@ -14,5 +14,5 @@

   async run(segments: string[]): Promise<void> {
     const { level, rest } = resolveTestLevel(segments);
-    await runRepositoryTestCommand("go", ["test", "...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], { cwd: moduleRoot, env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") } });
+    await runRepositoryTestCommand("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], { cwd: moduleRoot, env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") } });
   }
 }

### 41 🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🔬️_test.go
HEAD 0 BASE 371 STASH 449 CURRENT H
--- 

+++ 

@@ -6,8 +6,10 @@

 	bufio "bufio"
 	context "context"
+	"encoding/json"
 	io "io"
 	os "os"
 	exec "os/exec"
 	filepath "path/filepath"
+	"reflect"
 	strings "strings"
 	testing "testing"
@@ -370,2 +372,78 @@

 	}
 }
+
+// 🧭️ Portable owner vectors select the nearest real project without a workspace solution.
+func TestDotnetFormatterProjects(t *testing.T) {
+	var vectors struct {
+		Cases []struct {
+			Name     string
+			Source   string
+			Files    []string
+			Projects []string
+		}
+	}
+	data, err := os.ReadFile(filepath.Join("..", "..", "🧫️fixtures", "📡️dotnet-format-owners.json"))
+	if err != nil {
+		t.Fatal(err)
+	}
+	if err := json.Unmarshal(data, &vectors); err != nil {
+		t.Fatal(err)
+	}
+	for _, row := range vectors.Cases {
+		t.Run(row.Name, func(t *testing.T) {
+			root := t.TempDir()
+			for _, file := range row.Files {
+				path := filepath.Join(root, filepath.FromSlash(file))
+				if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
+					t.Fatal(err)
+				}
+				if err := os.WriteFile(path, []byte(""), 0644); err != nil {
+					t.Fatal(err)
+				}
+			}
+			got := DotnetFormatterProjects(root, row.Source)
+			if !reflect.DeepEqual(got, row.Projects) {
+				t.Fatalf("projects = %v, want %v", got, row.Projects)
+			}
+		})
+	}
+}
+
+// 🚫️ The Go authority agrees with the dashboard's language-neutral path vectors.
+func TestIdeConfigurationPaths(t *testing.T) {
+	data, err := os.ReadFile("../../../🎛️dashboard/🧫️fixtures/🧭️authority/🔣️.json")
+	if err != nil {
+		t.Fatal(err)
+	}
+	var fixture struct {
+		PathCases []struct {
+			Path     string
+			Accepted bool
+		}
+	}
+	if err := json.Unmarshal(data, &fixture); err != nil {
+		t.Fatal(err)
+	}
+	for _, row := range fixture.PathCases {
+		if got := IsIdeConfigurationPath(row.Path); got == row.Accepted {
+			t.Errorf("%s: forbidden = %v", row.Path, got)
+		}
+	}
+	root := t.TempDir()
+	for _, path := range []string{"app/.idea/workspace.xml", ".codex/config.toml", "node_modules/example/.vscode/settings.json", "🗑️generated/example/Old.sln", ".🧬semio/🦑️repo/🎫️tickets/old/.vscode/launch.json"} {
+		absolute := filepath.Join(root, filepath.FromSlash(path))
+		if err := os.MkdirAll(filepath.Dir(absolute), 0755); err != nil {
+			t.Fatal(err)
+		}
+		if err := os.WriteFile(absolute, []byte(""), 0644); err != nil {
+			t.Fatal(err)
+		}
+	}
+	paths, err := IdeConfigurationPaths(root, "")
+	if err != nil {
+		t.Fatal(err)
+	}
+	if !reflect.DeepEqual(paths, []string{"app/.idea"}) {
+		t.Fatalf("active IDE paths = %v", paths)
+	}
+}

### 42 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go
HEAD 176 BASE 46930 STASH 46676 CURRENT H
--- 

+++ 

@@ -31,8 +31,9 @@

 	mcp "github.com/usalu/semio/repo/client/internal/mcp"
 	server "github.com/usalu/semio/repo/client/internal/mcpserver"
+	templatefunc "github.com/usalu/semio/repo/client/internal/templatefunc"
+	repopkg "github.com/usalu/semio/repo/go"
 	search "github.com/usalu/semio/repo/search"
-	templatefunc "github.com/usalu/semio/repo/client/internal/templatefunc"
+	workspace "github.com/usalu/semio/repo/workspace"
 	yaml "github.com/usalu/semio/repo/yaml"
-	repopkg "github.com/usalu/semio/repo/go"
 	"io"
 	"io/fs"
@@ -1743,115 +1744,9 @@

 		return runExternalCommand(absRoot, "cargo", args, cmd)
 	case "csharp":
-		args := []string{"test"}
-		if testFilter != "" {
-			args = append(args, "--filter", testFilter)
-		}
-		return runExternalCommand(absRoot, "dotnet", args, cmd)
-	case "typescript", "javascript":
-		runner, runArgs := detectJSTestRunner(absRoot, testFilter)
-		return runExternalCommand(absRoot, runner, runArgs, cmd)
-	case "python":
-		args := []string{"run", "pytest"}
-		if fileFilter != "" {
-			args = append(args, fileFilter)
-		}
-		if testFilter != "" {
-			args = append(args, "-k", testFilter)
-		}
-		if uvExists() {
-			return runExternalCommand(absRoot, "uv", args, cmd)
-		}
-		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
-	}
-	return fmt.Errorf("unknown language %q for bundle %s", lang, bundleRoot)
-}
-
-func uvExists() bool {
-	_, err := exec.LookPath("uv")
-	return err == nil
-}
-
-func detectJSTestRunner(absRoot string, testFilter string) (string, []string) {
-
-	pkgPath := filepath.Join(absRoot, "package.json")
-	if FileExists(pkgPath) {
-		content, err := ReadTextFile(pkgPath)
-		if err == nil {
-			if strings.Contains(content, "vitest") {
-				args := []string{"vitest", "run"}
-				if testFilter != "" {
-					args = append(args, "-t", testFilter)
-				}
-				return "npx", args
-			}
-			if strings.Contains(content, "jest") {
-				args := []string{"jest"}
-				if testFilter != "" {
-					args = append(args, "-t", testFilter)
-				}
-				return "npx", args
-			}
-
-			if strings.Contains(content, `"test"`) {
-				args := []string{"test"}
-				return "npm", args
-			}
-		}
-	}
-
-	args := []string{"vitest", "run"}
-	if testFilter != "" {
-		args = append(args, "-t", testFilter)
-	}
-	return "npx", args
-}
-
-// 🔺️runFileTests holds the data fields for a runFileTests record.
-func runFileTests(filePath string, lang string, testFilter string, cmd *command.Command) error {
-	if lang == "" {
-		l := GetLanguage(filePath)
-		if l != nil {
-			lang = l.Name()
-		}
-	}
-	absFilePath := filePath
-	if !filepath.IsAbs(absFilePath) {
-		absFilePath = filepath.Join(rootDir, filePath)
-	}
-	bundle := GetBundleByPath(filePath)
-	if bundle == nil {
-		return fmt.Errorf("no bundle found for file %s", filePath)
-	}
-	absRoot := filepath.Join(rootDir, bundle.Root)
-	relFile, _ := filepath.Rel(absRoot, absFilePath)
-	switch lang {
-	case "go":
-		args := []string{"-v", "-run"}
-		if testFilter != "" {
-			args = append(args, testFilter)
-		} else {
-			args = append(args, ".")
-		}
-
-		return runCanonicalGoTestDispatch(absRoot, absFilePath, args, cmd)
-	case "python":
-		args := []string{"run", "pytest", relFile}
-		if testFilter != "" {
-			args = append(args, "-k", testFilter)
-		}
-		if uvExists() {
-			return runExternalCommand(absRoot, "uv", args, cmd)
-		}
-		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
-	case "typescript", "javascript":
-		runner, runArgs := detectJSTestRunner(absRoot, testFilter)
-		runArgs = append(runArgs, relFile)
-		return runExternalCommand(absRoot, runner, runArgs, cmd)
-	case "csharp":
-		args := []string{"test"}
-		if testFilter != "" {
-			args = append(args, "--filter", testFilter)
-		}
-		return runExternalCommand(absRoot, "dotnet", args, cmd)
+		plans := []formatterPlan{}
+		for _, project := range workspace.DotnetFormatterProjects(rootDir, relPath) {
+			plans = append(plans, formatterPlan{binary: "dotnet", args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
+		}
+		return append(plans, prettierPlan)
 	case "rust":
 		args := []string{"test"}
@@ -1976,6 +1871,9 @@

 		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
 	case "csharp":
-		args := []string{"test", "--filter", "FullyQualifiedName~" + originalName}
-		return runExternalCommand(absRoot, "dotnet", args, cmd)
+		plans := []formatterPlan{}
+		for _, project := range workspace.DotnetFormatterProjects(rootDir, relPath) {
+			plans = append(plans, formatterPlan{binary: "dotnet", args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
+		}
+		return append(plans, prettierPlan)
 	case "rust":
 		args := []string{"test", testName}
@@ -15879,6 +15777,5 @@

 	BreachRepoMissingCommand                           Statute = "repo/missing-command"
 	BreachRepoMissingTicketTracking                    Statute = "repo/missing-ticket-tracking"
-	BreachSystemDevcontainerVscodeSettingsDrift      Statute = "system/devcontainer/vscode/settings-drift"
-	BreachSystemDevcontainerVscodeExtensionsDrift    Statute = "system/devcontainer/vscode/extensions-drift"
+	BreachSystemIdeConfiguration                       Statute = "system/ide-configuration"
 	BreachFolderIllegalEmpty                           Statute = "folder/illegal/empty"
 	BreachFolderNameMissingEmoji                       Statute = "folder/name/missing-emoji"
@@ -16272,16 +16169,9 @@

 		Autofixable: false,
 	},
-	BreachSystemDevcontainerVscodeSettingsDrift: {
-		Kind:        BreachSystemDevcontainerVscodeSettingsDrift,
+	BreachSystemIdeConfiguration: {
+		Kind:        BreachSystemIdeConfiguration,
 		Priority:    BreachPriorityHigh,
-		Reason:      "The devcontainer editor settings are derived from .vscode/settings.json and .devcontainer/editor-overlay.json; a hand-edited copy drifts from the host editor",
-		Solution:    "Run `bun repo run workspace:generate-config` to rewrite the devcontainer editor block; edit .vscode/settings.json, .vscode/extensions.json or the overlay instead of the copy",
-		Autofixable: false,
-	},
-	BreachSystemDevcontainerVscodeExtensionsDrift: {
-		Kind:        BreachSystemDevcontainerVscodeExtensionsDrift,
-		Priority:    BreachPriorityHigh,
-		Reason:      "The devcontainer extensions are derived from .vscode/extensions.json and .devcontainer/editor-overlay.json; a hand-edited copy drifts from the host editor",
-		Solution:    "Run `bun repo run workspace:generate-config` to rewrite the devcontainer editor block; edit .vscode/settings.json, .vscode/extensions.json or the overlay instead of the copy",
+		Reason:      "Developer commands and configuration belong to the repository dashboard",
+		Solution:    "Remove editor-owned configuration and register the command in the repository dashboard",
 		Autofixable: false,
 	},
@@ -17190,8 +17080,9 @@

 		}
 	case "csharp":
-		return []formatterPlan{
-			{binary: "dotnet", args: []string{"format", "Monorepo.sln", "--include", relPath, "--verbosity", "minimal"}, requirement: []string{"Monorepo.sln"}},
-			prettierPlan,
-		}
+		plans := []formatterPlan{}
+		for _, project := range workspace.DotnetFormatterProjects(rootDir, relPath) {
+			plans = append(plans, formatterPlan{binary: "dotnet", args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
+		}
+		return append(plans, prettierPlan)
 	case "rust":
 		return []formatterPlan{
@@ -18712,24 +18603,9 @@

 		ID:          "system",
 		Name:        "System",
-		Description: "Validates system configuration files like devcontainer and editor settings",
-		Scopes:      []string{".vscode/settings.json", ".vscode/extensions.json", ".devcontainer/devcontainer.json", ".devcontainer/editor-overlay.json"},
+		Description: "Enforces repository dashboard authority over developer configuration",
+		Scopes:      []string{"**/*"},
 		Priority:    BreachPriorityHigh,
-		Groups: []Territory{
-			{
-				Name:        "Devcontainer",
-				Description: "Devcontainer configuration breachs",
-				Groups: []Territory{
-					{
-						Name:        "VSCode",
-						Description: "The devcontainer editor block must equal its derivation from the host editor files",
-						Kinds: []Statute{
-							BreachSystemDevcontainerVscodeSettingsDrift,
-							BreachSystemDevcontainerVscodeExtensionsDrift,
-						},
-					},
-				},
-			},
-		},
-		Run: systemPolicy,
+		Groups:      []Territory{{Name: "Developer Configuration", Description: "Editor-owned configuration is forbidden", Kinds: []Statute{BreachSystemIdeConfiguration}}},
+		Run:         systemPolicy,
 	},
 	{
@@ -19649,5 +19525,9 @@

 		return strings.HasPrefix(trimmed, "def ") || strings.HasPrefix(trimmed, "class ") || strings.HasPrefix(trimmed, "async def ")
 	case "csharp":
-		return !strings.Contains(trimmed, " interface ") && !strings.Contains(trimmed, " enum ")
+		plans := []formatterPlan{}
+		for _, project := range workspace.DotnetFormatterProjects(rootDir, relPath) {
+			plans = append(plans, formatterPlan{binary: "dotnet", args: []string{"format", project, "--include", relPath, "--verbosity", "minimal"}, requirement: []string{project}})
+		}
+		return append(plans, prettierPlan)
 	case "rust":
 		noPrefix := strings.TrimPrefix(trimmed, "pub ")
@@ -20647,159 +20527,19 @@

 }
 
-// 📝️EditorSurface is the editor state of one host: workspace settings and recommended extensions.
-type EditorSurface struct {
-	Settings   map[string]interface{}
-	Extensions []string
-}
-
-// 🧩️EditorOverlay declares what the devcontainer does not inherit from the host editor files and what only it adds (`.devcontainer/editor-overlay.json`).
-type EditorOverlay struct {
-	Settings struct {
-		Omit []string               `json:"omit"`
-		Set  map[string]interface{} `json:"set"`
-	} `json:"settings"`
-	Extensions struct {
-		Omit []string `json:"omit"`
-		Add  []string `json:"add"`
-	} `json:"extensions"`
-}
-
-// 🧮️DeriveContainerEditor derives the devcontainer editor surface from the host canonical surface and the declared overlay:
-// the host entries minus the omitted ones, `${workspaceFolder}` rewritten to `${containerWorkspaceFolder}`, then the overlay's own settings and extensions.
-func DeriveContainerEditor(canonical EditorSurface, overlay EditorOverlay) EditorSurface {
-	settings := map[string]interface{}{}
-	encoded, err := json.Marshal(canonical.Settings)
-	if err == nil {
-		_ = json.Unmarshal([]byte(strings.ReplaceAll(string(encoded), "${workspaceFolder}", "${containerWorkspaceFolder}")), &settings)
-	}
-	for _, key := range overlay.Settings.Omit {
-		delete(settings, key)
-	}
-	for key, value := range overlay.Settings.Set {
-		settings[key] = value
-	}
-	var extensions []string
-	seen := map[string]struct{}{}
-	for _, id := range append(slices.DeleteFunc(slices.Clone(canonical.Extensions), func(id string) bool { return slices.Contains(overlay.Extensions.Omit, id) }), overlay.Extensions.Add...) {
-		if _, ok := seen[id]; !ok {
-			seen[id] = struct{}{}
-			extensions = append(extensions, id)
-		}
-	}
-	return EditorSurface{Settings: settings, Extensions: extensions}
-}
-
-// 📦️stripJSONComments removes // and /* */ comments outside strings so JSONC files parse as JSON.
-func stripJSONComments(data []byte) []byte {
-	out := make([]byte, 0, len(data))
-	for index := 0; index < len(data); index++ {
-		switch {
-		case data[index] == '"':
-			start := index
-			for index++; index < len(data) && data[index] != '"'; index++ {
-				if data[index] == '\\' {
-					index++
-				}
-			}
-			out = append(out, data[start:min(index+1, len(data))]...)
-		case data[index] == '/' && index+1 < len(data) && data[index+1] == '/':
-			for index < len(data) && data[index] != '\n' {
-				index++
-			}
-			out = append(out, '\n')
-		case data[index] == '/' && index+1 < len(data) && data[index+1] == '*':
-			end := strings.Index(string(data[index+2:]), "*/")
-			if end < 0 {
-				return out
-			}
-			index += end + 3
-		default:
-			out = append(out, data[index])
-		}
-	}
-	return out
-}
-
-// 📦️readJSONC parses a JSONC file into `out`.
-func readJSONC(path string, out interface{}) bool {
-	data, err := os.ReadFile(path)
-	if err != nil {
-		return false
-	}
-	return json.Unmarshal(stripJSONComments(data), out) == nil
-}
-
-// 📖️ReadHostEditor reads the host canonical editor surface (`.vscode/settings.json`, `.vscode/extensions.json`).
-func ReadHostEditor(rootDir string) (EditorSurface, bool) {
-	var surface EditorSurface
-	var extensions struct {
-		Recommendations []interface{} `json:"recommendations"`
-	}
-	if !readJSONC(filepath.Join(rootDir, ".vscode", "settings.json"), &surface.Settings) || !readJSONC(filepath.Join(rootDir, ".vscode", "extensions.json"), &extensions) {
-		return surface, false
-	}
-	surface.Extensions = extensionIDsFromJSONList(extensions.Recommendations)
-	return surface, true
-}
-
-// 📖️ReadContainerEditor reads the editor surface checked into `.devcontainer/devcontainer.json`.
-func ReadContainerEditor(rootDir string) (EditorSurface, bool) {
-	var devcontainer struct {
-		Customizations struct {
-			Vscode struct {
-				Settings   map[string]interface{} `json:"settings"`
-				Extensions []interface{}          `json:"extensions"`
-			} `json:"vscode"`
-		} `json:"customizations"`
-	}
-	if !readJSONC(filepath.Join(rootDir, ".devcontainer", "devcontainer.json"), &devcontainer) {
-		return EditorSurface{}, false
-	}
-	return EditorSurface{Settings: devcontainer.Customizations.Vscode.Settings, Extensions: extensionIDsFromJSONList(devcontainer.Customizations.Vscode.Extensions)}, true
-}
-
-// 📖️ReadEditorOverlay reads `.devcontainer/editor-overlay.json`; a missing file declares no overlay.
-func ReadEditorOverlay(rootDir string) EditorOverlay {
-	var overlay EditorOverlay
-	readJSONC(filepath.Join(rootDir, ".devcontainer", "editor-overlay.json"), &overlay)
-	return overlay
-}
-
-// 📦️extensionIDsFromJSONList normalizes a JSON string list of extension ids.
-func extensionIDsFromJSONList(values []interface{}) []string {
-	if len(values) == 0 {
-		return nil
-	}
-	ids := make([]string, 0, len(values))
-	for _, value := range values {
-		id, ok := value.(string)
-		if !ok || id == "" {
-			continue
-		}
-		ids = append(ids, id)
-	}
-	return ids
-}
-
-// ⚖️systemPolicy requires the devcontainer editor block to equal its derivation from the host editor files and the declared overlay.
+// ⚖️ SystemPolicy rejects editor-owned configuration in the active repository.
 func systemPolicy(ctx *PolicyContext) []Breach {
+	paths := ctx.filesOverride
+	if paths == nil {
+		var err error
+		paths, err = workspace.IdeConfigurationPaths(ctx.RootDir, ctx.Scope.FilePath)
+		if err != nil {
+			return []Breach{ctx.CreateBreach(err.Error(), BreachSystemIdeConfiguration, ctx.Scope.FilePath, 1, 0, "")}
+		}
+	}
 	var breachs []Breach
-	canonical, hasCanonical := ReadHostEditor(ctx.RootDir)
-	actual, hasContainer := ReadContainerEditor(ctx.RootDir)
-	if !hasCanonical || !hasContainer {
-		return ctx.FilterIgnored(breachs)
-	}
-	expected := DeriveContainerEditor(canonical, ReadEditorOverlay(ctx.RootDir))
-	if !reflect.DeepEqual(expected.Settings, actual.Settings) {
-		breachs = append(breachs, ctx.CreateBreach(
-			".devcontainer/devcontainer.json customizations.vscode.settings must equal .vscode/settings.json plus .devcontainer/editor-overlay.json",
-			BreachSystemDevcontainerVscodeSettingsDrift,
-			".devcontainer/devcontainer.json", 1, 0, ""))
-	}
-	if !slices.Equal(expected.Extensions, actual.Extensions) {
-		breachs = append(breachs, ctx.CreateBreach(
-			".devcontainer/devcontainer.json customizations.vscode.extensions must equal .vscode/extensions.json plus .devcontainer/editor-overlay.json",
-			BreachSystemDevcontainerVscodeExtensionsDrift,
-			".devcontainer/devcontainer.json", 1, 0, ""))
+	for _, path := range paths {
+		if workspace.IsIdeConfigurationPath(path) {
+			breachs = append(breachs, ctx.CreateBreach("Developer commands and configuration belong to the repository dashboard", BreachSystemIdeConfiguration, path, 1, 0, ""))
+		}
 	}
 	return ctx.FilterIgnored(breachs)
@@ -25759,9 +25499,15 @@

 	info, err = os.Lstat(absDir)
 	if err != nil {
-		if os.IsNotExist(err) { return nil }
+		if os.IsNotExist(err) {
+			return nil
+		}
 		return err
 	}
-	if info.Mode()&fs.ModeSymlink != 0 { return nil }
-	if !info.IsDir() { return fmt.Errorf("generated ticket output is not a directory: %s", absDir) }
+	if info.Mode()&fs.ModeSymlink != 0 {
+		return nil
+	}
+	if !info.IsDir() {
+		return fmt.Errorf("generated ticket output is not a directory: %s", absDir)
+	}
 
 	type fileEntry struct {

### 43 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go
HEAD 0 BASE 23637 STASH 23577 CURRENT H
--- 

+++ 

@@ -5333,115 +5333,36 @@

 
 func TestSystemPolicy(t *testing.T) {
-	const settings = `{"editor.fontSize": 14, "python.testing.cwd": "${workspaceFolder}", "terminal.integrated.env.windows": {"A": "b"}}`
-	const extensions = `{"recommendations": ["ms-python.python", "ms-vscode-remote.remote-containers"]}`
-	const overlay = `{"settings": {"omit": ["terminal.integrated.env.windows"], "set": {"python.defaultInterpreterPath": "${containerWorkspaceFolder}/.venv/bin/python"}}, "extensions": {"omit": ["ms-vscode-remote.remote-containers"], "add": ["anthropic.claude-code"]}}`
-	const devcontainer = "// SPDX-License-Identifier: AGPL-3.0-only\n{\n  /* derived */\n  \"name\": \"https://example.test/a\",\n  \"customizations\": {\"vscode\": {\"settings\": {\"editor.fontSize\": 14, \"python.testing.cwd\": \"${containerWorkspaceFolder}\", \"python.defaultInterpreterPath\": \"${containerWorkspaceFolder}/.venv/bin/python\"}, \"extensions\": [\"ms-python.python\", \"anthropic.claude-code\"]}}\n}\n"
-	arrange := func(t *testing.T, files map[string]string) {
-		tmpDir := t.TempDir()
-		oldRoot := rootDir
-		rootDir = tmpDir
-		t.Cleanup(func() { rootDir = oldRoot })
-		for name, content := range files {
-			os.MkdirAll(filepath.Dir(filepath.Join(tmpDir, name)), 0o755)
-			WriteTextFile(filepath.Join(tmpDir, name), content)
-		}
-	}
-	kinds := func() map[Statute]bool {
-		found := map[Statute]bool{}
-		for _, v := range systemPolicy(NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})) {
-			found[v.Kind] = true
-		}
-		return found
-	}
-	files := func(overrides map[string]string) map[string]string {
-		base := map[string]string{".vscode/settings.json": settings, ".vscode/extensions.json": extensions, ".devcontainer/editor-overlay.json": overlay, ".devcontainer/devcontainer.json": devcontainer}
-		for name, content := range overrides {
-			base[name] = content
-		}
-		return base
-	}
-	t.Run("no breach when the devcontainer block equals its derivation", func(t *testing.T) {
-		arrange(t, files(nil))
-		if found := kinds(); len(found) != 0 {
-			t.Errorf("expected no breach, got %v", found)
-		}
-	})
-	t.Run("detects drifted settings", func(t *testing.T) {
-		arrange(t, files(map[string]string{".vscode/settings.json": `{"editor.fontSize": 15}`}))
-		found := kinds()
-		if !found[BreachSystemDevcontainerVscodeSettingsDrift] || found[BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected only the settings drift breach, got %v", found)
-		}
-	})
-	t.Run("detects drifted extensions", func(t *testing.T) {
-		arrange(t, files(map[string]string{".vscode/extensions.json": `{"recommendations": ["ms-python.python", "golang.go"]}`}))
-		found := kinds()
-		if found[BreachSystemDevcontainerVscodeSettingsDrift] || !found[BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected only the extensions drift breach, got %v", found)
-		}
-	})
-	t.Run("a missing overlay declares no overlay", func(t *testing.T) {
-		arrange(t, files(map[string]string{".devcontainer/editor-overlay.json": ""}))
-		found := kinds()
-		if !found[BreachSystemDevcontainerVscodeSettingsDrift] || !found[BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected both drift breaches without overlay, got %v", found)
-		}
-	})
-	t.Run("no breachs when the editor files are absent", func(t *testing.T) {
-		arrange(t, nil)
-		if found := kinds(); len(found) != 0 {
-			t.Errorf("expected 0 breachs, got %v", found)
-		}
-	})
-	t.Run("derivation equals the language-agnostic fixture", func(t *testing.T) {
-		data, err := os.ReadFile("../../../../📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/🧫️fixtures/📝️derived-config/🔣️.json")
-		if err != nil {
-			t.Fatal(err)
-		}
-		var fixture struct {
-			Editor []struct {
-				Name      string
-				Canonical EditorSurface
-				Overlay   EditorOverlay
-				Expected  EditorSurface
-			}
-		}
-		if err := json.Unmarshal(data, &fixture); err != nil {
-			t.Fatal(err)
-		}
-		if len(fixture.Editor) == 0 {
-			t.Fatal("fixture holds no editor cases")
-		}
-		for _, row := range fixture.Editor {
-			derived := DeriveContainerEditor(row.Canonical, row.Overlay)
-			if !reflect.DeepEqual(derived.Settings, row.Expected.Settings) || !reflect.DeepEqual(derived.Extensions, row.Expected.Extensions) {
-				t.Errorf("%s: derived %v, expected %v", row.Name, derived, row.Expected)
-			}
-		}
-	})
-	t.Run("policy registered with correct id", func(t *testing.T) {
-		p, found := FindPolicy("system")
-		if !found {
-			t.Fatal("expected system policy to be registered")
-		}
-		if p.Name != "System" {
-			t.Errorf("expected name System, got %s", p.Name)
-		}
-		kinds := p.AllKinds()
-		if len(kinds) != 2 || kinds[0] != BreachSystemDevcontainerVscodeSettingsDrift || kinds[1] != BreachSystemDevcontainerVscodeExtensionsDrift {
-			t.Fatalf("expected the settings and extensions drift statutes, got %v", kinds)
-		}
-	})
-	t.Run("statute meta is correct", func(t *testing.T) {
-		for _, kind := range []Statute{BreachSystemDevcontainerVscodeSettingsDrift, BreachSystemDevcontainerVscodeExtensionsDrift} {
-			meta := kind.Info()
-			if meta.Autofixable {
-				t.Errorf("%s: a derived copy is rewritten by generate-config, never autofixed", kind)
-			}
-			if meta.Priority != BreachPriorityHigh {
-				t.Errorf("%s: expected high priority", kind)
-			}
-		}
-	})
+	data, err := os.ReadFile("../../../../🎛️dashboard/🧫️fixtures/🧭️authority/🔣️.json")
+	if err != nil {
+		t.Fatal(err)
+	}
+	var fixture struct {
+		PathCases []struct {
+			Path     string
+			Accepted bool
+		}
+	}
+	if err := json.Unmarshal(data, &fixture); err != nil {
+		t.Fatal(err)
+	}
+	for _, row := range fixture.PathCases {
+		t.Run(row.Path, func(t *testing.T) {
+			ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeRepo}, []Bundle{}, []string{row.Path})
+			breachs := systemPolicy(ctx)
+			if (len(breachs) == 0) != row.Accepted {
+				t.Fatalf("accepted = %v, breachs = %v", row.Accepted, breachs)
+			}
+			if len(breachs) > 0 && breachs[0].Kind != BreachSystemIdeConfiguration {
+				t.Fatalf("unexpected statute %v", breachs[0].Kind)
+			}
+		})
+	}
+	policy, found := FindPolicy("system")
+	if !found || len(policy.AllKinds()) != 1 || policy.AllKinds()[0] != BreachSystemIdeConfiguration {
+		t.Fatalf("unexpected system policy %v", policy)
+	}
+	if BreachSystemIdeConfiguration.Info().Autofixable || BreachSystemIdeConfiguration.Info().Priority != BreachPriorityHigh {
+		t.Fatal("IDE configuration removal must be explicit and high priority")
+	}
 }
 
@@ -17738,5 +17659,4 @@

 }
 
-
 // 🛟️ TestTicketGeneratedOutputRetention checks the language-neutral generated-only lifecycle boundary.
 func TestTicketGeneratedOutputRetention(t *testing.T) {
@@ -17747,13 +17667,20 @@

 	if len(fixture.Purge.Retention) != 16 { t.Fatalf("retention vectors = %d", len(fixture.Purge.Retention)) }
 	root := t.TempDir()
-	for _, row := range fixture.Purge.Retention { writeSparseTicketArtifact(t, filepath.Join(root, filepath.FromSlash(row.Path)), row.Bytes) }
-	if err := purgeOversizedTicketArtifacts(root); err != nil { t.Fatal(err) }
+	for _, row := range fixture.Purge.Retention {
+		writeSparseTicketArtifact(t, filepath.Join(root, filepath.FromSlash(row.Path)), row.Bytes)
+	}
+	if err := purgeOversizedTicketArtifacts(root); err != nil {
+		t.Fatal(err)
+	}
 	for _, row := range fixture.Purge.Retention {
 		info, err := os.Stat(filepath.Join(root, filepath.FromSlash(row.Path)))
-		if row.Retained && (err != nil || info.Size() != row.Bytes) { t.Errorf("required retained artifact %s: %v", row.Path, err) }
-		if !row.Retained && !os.IsNotExist(err) { t.Errorf("generated artifact still exists: %s", row.Path) }
-	}
-}
-
+		if row.Retained && (err != nil || info.Size() != row.Bytes) {
+			t.Errorf("required retained artifact %s: %v", row.Path, err)
+		}
+		if !row.Retained && !os.IsNotExist(err) {
+			t.Errorf("generated artifact still exists: %s", row.Path)
+		}
+	}
+}
 
 // 🧭️ TestTicketGeneratedOutputRoots preserves authored data with missing, non-directory and symbolic generated roots.
@@ -17767,9 +17694,22 @@

 			writeSparseTicketArtifact(t, outside, 11<<20)
 			output := filepath.Join(root, "🗑️generated")
-			if mode == "file" { writeSparseTicketArtifact(t, output, 11<<20) }
-			if mode == "symlink" { if err := os.Symlink(external, output); err != nil { t.Skipf("platform does not permit symbolic directory: %v", err) } }
+			if mode == "file" {
+				writeSparseTicketArtifact(t, output, 11<<20)
+			}
+			if mode == "symlink" {
+				if err := os.Symlink(external, output); err != nil {
+					t.Skipf("platform does not permit symbolic directory: %v", err)
+				}
+			}
 			err := purgeOversizedTicketArtifacts(root)
-			if (mode == "file") != (err != nil) { t.Fatalf("generated root %s: %v", mode, err) }
-			for _, path := range []string{authored, outside} { info, err := os.Stat(path); if err != nil || info.Size() != 11<<20 { t.Fatalf("retained source %s: %v", path, err) } }
+			if (mode == "file") != (err != nil) {
+				t.Fatalf("generated root %s: %v", mode, err)
+			}
+			for _, path := range []string{authored, outside} {
+				info, err := os.Stat(path)
+				if err != nil || info.Size() != 11<<20 {
+					t.Fatalf("retained source %s: %v", path, err)
+				}
+			}
 		})
 	}

### 44 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧩️component/🐹️.go
HEAD 76 BASE 72 STASH 72 CURRENT H
--- 

+++ 

@@ -20,5 +20,5 @@

 	"os"
 
-	"github.com/usalu/semio/repo/client"
+	"github.com/usalu/semio/repo/providers"
 )
 
@@ -30,10 +30,10 @@

 const MCPProfileEnvironment = "SEMIO_REPO_MCP_CLIENT"
 
-func resolveMCPProfile(arguments []string, lookupEnvironment func(string) (string, bool)) (client.McpClientKind, error) {
+func resolveMCPProfile(arguments []string, lookupEnvironment func(string) (string, bool)) (providers.McpClientKind, error) {
 	if len(arguments) != 0 {
 		return "", errors.New("repo MCP accepts no command arguments")
 	}
 	raw, _ := lookupEnvironment(MCPProfileEnvironment)
-	return client.ParseMcpClientKind(raw)
+	return providers.ParseMcpClientKind(raw)
 }
 
@@ -51,8 +51,8 @@

 
 func runMCP(ctx context.Context, transport Transport, repository RepositoryHandlers) (*Server, error) {
-	return runMCPForProfile(ctx, transport, repository, client.McpClientGeneric)
+	return runMCPForProfile(ctx, transport, repository, providers.McpClientGeneric)
 }
 
-func runMCPForProfile(ctx context.Context, transport Transport, repository RepositoryHandlers, profile client.McpClientKind) (*Server, error) {
+func runMCPForProfile(ctx context.Context, transport Transport, repository RepositoryHandlers, profile providers.McpClientKind) (*Server, error) {
 	server, err := NewRepositoryServerFor(repository, profile)
 	if err != nil {

### 45 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go
HEAD 437 BASE 1385 STASH 1438 CURRENT H
--- 

+++ 

@@ -18,4 +18,5 @@

 	"os"
 	"path/filepath"
+	"reflect"
 	"slices"
 	"strings"
@@ -25,5 +26,6 @@

 	"time"
 
-	"github.com/usalu/semio/repo/client"
+	"github.com/usalu/semio/repo/providers"
+	"github.com/usalu/semio/repo/workspace"
 )
 
@@ -45,4 +47,9 @@

 	ProfileEnvironment string   `json:"profileEnvironment"`
 	Arguments          []string `json:"arguments"`
+	ResourceContents []json.RawMessage `json:"resourceContents"`
+	ReopenPrompt struct {
+		Invalid []json.RawMessage `json:"invalid"`
+		Valid []string `json:"valid"`
+	} `json:"reopenPrompt"`
 	ProtocolVersions   []string `json:"protocolVersions"`
 	Profiles           []struct {
@@ -90,6 +97,6 @@

 				t.Fatal(err)
 			}
-			if string(kind) != profile.Kind || client.McpServerName(kind) != profile.ServerName {
-				t.Fatalf("profile %q resolved kind=%q server=%q", profile.Slug, kind, client.McpServerName(kind))
+			if string(kind) != profile.Kind || providers.McpServerName(kind) != profile.ServerName {
+				t.Fatalf("profile %q resolved kind=%q server=%q", profile.Slug, kind, providers.McpServerName(kind))
 			}
 			server, err := NewRepositoryServerFor(&testRepository{}, kind)
@@ -152,4 +159,50 @@

 }
 
+func TestRepositoryReopenPromptContract(t *testing.T) {
+	data, err := os.ReadFile(filepath.Join("..", "..", "🔌️mcp", "🧫️fixtures", "🚪️entrypoint-contract.json"))
+	if err != nil { t.Fatal(err) }
+	var fixture entrypointContractFixture
+	if err := decodeExact(data, &fixture); err != nil { t.Fatal(err) }
+	root, err := os.MkdirTemp(os.TempDir(), "rp")
+	if err != nil { t.Fatal(err) }
+	t.Cleanup(func() { os.RemoveAll(root) })
+	if err := os.Mkdir(filepath.Join(root, ".git"), 0700); err != nil { t.Fatal(err) }
+	previous := workspace.GetRootDir()
+	workspace.SetRootDir(root)
+	t.Cleanup(func() { workspace.SetRootDir(previous) })
+	if workspace.GetRootDir() != root { t.Fatal("private repository root refused") }
+	for _, prompt := range fixture.ReopenPrompt.Invalid {
+		arguments := json.RawMessage(fmt.Sprintf(`{"path":"26/09/23/PROMPT-CONTRACT","prompt":%s,"no_management":true}`, prompt))
+		_, err := NewClientRepository(providers.McpClientGeneric).Call(context.Background(), "ticket_reopen", arguments)
+		var failure *HandlerError
+		if !errors.As(err, &failure) || failure.Code != -32010 || failure.Message != "invalid ticket_reopen arguments" {
+			t.Errorf("prompt=%s: expected domain argument refusal, got %v", prompt, err)
+		}
+	}
+	_, err = NewClientRepository(providers.McpClientGeneric).Call(context.Background(), "ticket_reopen", json.RawMessage(`{"path":"26/09/23/PROMPT-CONTRACT","no_management":true}`))
+	var failure *HandlerError
+	if !errors.As(err, &failure) || failure.Code != -32010 || failure.Message != "invalid ticket_reopen arguments" { t.Errorf("missing prompt: got %v", err) }
+	entries, err := os.ReadDir(root)
+	if err != nil { t.Fatal(err) }
+	if len(entries) != 1 || entries[0].Name() != ".git" { t.Fatal("refused prompt changed repository metadata") }
+}
+
+func TestResourceContentContract(t *testing.T) {
+	data, err := os.ReadFile(filepath.Join("..", "..", "🔌️mcp", "🧫️fixtures", "🚪️entrypoint-contract.json"))
+	if err != nil { t.Fatal(err) }
+	var fixture entrypointContractFixture
+	if err := decodeExact(data, &fixture); err != nil { t.Fatal(err) }
+	for _, expected := range fixture.ResourceContents {
+		var content ResourceContent
+		if err := decodeExact(expected, &content); err != nil { t.Fatal(err) }
+		actual, err := json.Marshal(content)
+		if err != nil { t.Fatal(err) }
+		var actualValue, expectedValue any
+		if err := json.Unmarshal(actual, &actualValue); err != nil { t.Fatal(err) }
+		if err := json.Unmarshal(expected, &expectedValue); err != nil { t.Fatal(err) }
+		if !reflect.DeepEqual(actualValue, expectedValue) { t.Errorf("resource content got=%s want=%s", actual, expected) }
+	}
+}
+
 // 🤝️The handshake fixture is owned by the `🔌️mcp` module (both twins read the same file), so this
 // implementation never restates the contract it must satisfy.

### 46 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json
HEAD 1022 BASE 1015 STASH 671 CURRENT H
--- 

+++ 

@@ -4,5 +4,5 @@

   "name": "repo-vscode",
   "displayName": "repo",
-  "description": "compose devtools for VS Code 🧑️‍💻️",
+  "description": "Read-only monorepo source navigation and language editing for VS Code",
   "version": "0.0.1",
   "publisher": "usalu",
@@ -16,5 +16,5 @@

   },
   "categories": [
-    "Linters"
+    "Programming Languages"
   ],
   "activationEvents": [
@@ -55,185 +55,10 @@

     "commands": [
       {
-        "command": "compose.analyze",
-        "title": "repo: Analyze Codebase"
-      },
-      {
-        "command": "compose.analyzeFile",
-        "title": "repo: Analyze Current File"
-      },
-      {
-        "command": "compose.autofix",
-        "title": "repo: Autofix Codebase Problems"
-      },
-      {
-        "command": "compose.autofixFile",
-        "title": "repo: Autofix Current File Problems"
-      },
-      {
-        "command": "compose.policyList",
-        "title": "repo: List Policies"
-      },
-      {
-        "command": "compose.policyTree",
-        "title": "repo: Show Policy Tree"
-      },
-      {
-        "command": "compose.policyCheck",
-        "title": "repo: Check Policy"
-      },
-      {
         "command": "compose.ticketOpen",
         "title": "repo: Create Ticket"
       },
       {
-        "command": "compose.ticketList",
-        "title": "repo: List Tickets"
-      },
-      {
-        "command": "compose.ticketClose",
-        "title": "repo: Finish Ticket",
-        "icon": "$(issue-closed)"
-      },
-      {
-        "command": "compose.ticketReopen",
-        "title": "repo: Reopen Ticket",
-        "icon": "$(issue-reopened)"
-      },
-      {
-        "command": "compose.draftCreate",
-        "title": "repo: Create Draft",
-        "icon": "$(new-file)"
-      },
-      {
-        "command": "compose.draftDelete",
-        "title": "repo: Delete Draft",
-        "icon": "$(trash)"
-      },
-      {
-        "command": "compose.ticketTree",
-        "title": "repo: Show Ticket Tree"
-      },
-      {
-        "command": "compose.technologyList",
-        "title": "repo: List Bundles"
-      },
-      {
-        "command": "compose.technologyTree",
-        "title": "repo: Show Bundle Tree"
-      },
-      {
-        "command": "compose.contributorAdd",
-        "title": "repo: Add Contributor"
-      },
-      {
-        "command": "compose.contributorList",
-        "title": "repo: List Contributors"
-      },
-      {
-        "command": "compose.contributorRemove",
-        "title": "repo: Remove Contributor"
-      },
-      {
-        "command": "compose.folderTree",
-        "title": "repo: Show Folder Tree"
-      },
-      {
-        "command": "compose.folderCreate",
-        "title": "repo: Create Folder"
-      },
-      {
-        "command": "compose.folderMove",
-        "title": "repo: Move Folder"
-      },
-      {
-        "command": "compose.folderDelete",
-        "title": "repo: Delete Folder"
-      },
-      {
-        "command": "compose.folderList",
-        "title": "repo: List Folders"
-      },
-      {
-        "command": "compose.fileCreate",
-        "title": "repo: Create File"
-      },
-      {
-        "command": "compose.fileMove",
-        "title": "repo: Move File"
-      },
-      {
-        "command": "compose.fileDelete",
-        "title": "repo: Delete File"
-      },
-      {
-        "command": "compose.fileList",
-        "title": "repo: List Files"
-      },
-      {
-        "command": "compose.fileTree",
-        "title": "repo: Show File Tree"
-      },
-      {
-        "command": "compose.sectionTree",
-        "title": "repo: Show Section Tree"
-      },
-      {
-        "command": "compose.sectionCreate",
-        "title": "repo: Create Section"
-      },
-      {
-        "command": "compose.sectionMove",
-        "title": "repo: Move Section"
-      },
-      {
-        "command": "compose.sectionDelete",
-        "title": "repo: Delete Section"
-      },
-      {
-        "command": "compose.sectionList",
-        "title": "repo: List Sections"
-      },
-      {
-        "command": "compose.sectionOpen",
-        "title": "repo: Open Section"
-      },
-      {
-        "command": "compose.sectionRename",
-        "title": "repo: Rename Section",
-        "icon": "$(edit)"
-      },
-      {
-        "command": "compose.sectionCreateChild",
-        "title": "repo: Create Section Child",
-        "icon": "$(add)"
-      },
-      {
-        "command": "compose.sectionRemove",
-        "title": "repo: Delete Section",
-        "icon": "$(trash)"
-      },
-      {
-        "command": "compose.sectionIntegrate",
-        "title": "repo: Integrate Source Into Section"
-      },
-      {
-        "command": "compose.definitionList",
-        "title": "repo: List Definitions"
-      },
-      {
-        "command": "compose.definitionTree",
-        "title": "repo: Show Definition Tree"
-      },
-      {
-        "command": "compose.refreshDiagnostics",
-        "title": "repo: Refresh Diagnostics"
-      },
-      {
         "command": "compose.open",
         "title": "repo: Open Entity"
-      },
-      {
-        "command": "compose.navigateToRepo",
-        "title": "repo: Navigate to Repo"
       },
       {
@@ -276,112 +101,4 @@

       },
       {
-        "command": "compose.refreshTickets",
-        "title": "repo: Refresh Tickets",
-        "icon": "$(refresh)"
-      },
-      {
-        "command": "compose.refreshPolicies",
-        "title": "repo: Refresh Policies",
-        "icon": "$(refresh)"
-      },
-      {
-        "command": "compose.refreshContributors",
-        "title": "repo: Refresh Contributors",
-        "icon": "$(refresh)"
-      },
-      {
-        "command": "compose.toggleFilter",
-        "title": "repo: Toggle File Kind Filter"
-      },
-      {
-        "command": "compose.toggleBundleFilter",
-        "title": "repo: Toggle Bundle Kind Filter"
-      },
-      {
-        "command": "compose.toggleFolderFilter",
-        "title": "repo: Toggle Folder Kind Filter"
-      },
-      {
-        "command": "compose.toggleDefinitionFilter",
-        "title": "repo: Toggle Definition Kind Filter"
-      },
-      {
-        "command": "compose.toggleYearFilter",
-        "title": "repo: Toggle Year Filter"
-      },
-      {
-        "command": "compose.toggleMonthFilter",
-        "title": "repo: Toggle Month Filter"
-      },
-      {
-        "command": "compose.toggleDayFilter",
-        "title": "repo: Toggle Day Filter"
-      },
-      {
-        "command": "compose.toggleContributorFilter",
-        "title": "repo: Toggle Contributor Filter"
-      },
-      {
-        "command": "compose.togglePolicyFilter",
-        "title": "repo: Toggle Policy Filter"
-      },
-      {
-        "command": "compose.toggleBreachFilter",
-        "title": "repo: Toggle Breach Filter"
-      },
-      {
-        "command": "compose.filterAction",
-        "title": "repo: Filter Action"
-      },
-      {
-        "command": "compose.toggleTicketFilter",
-        "title": "repo: Toggle Ticket Filter",
-        "icon": "$(filter)"
-      },
-      {
-        "command": "compose.openTicket",
-        "title": "repo: Open Ticket"
-      },
-      {
-        "command": "compose.openTicketPlan",
-        "title": "repo: Open Plan",
-        "icon": "$(book)"
-      },
-      {
-        "command": "compose.checkPolicy",
-        "title": "repo: Check Policy"
-      },
-      {
-        "command": "compose.runCommand",
-        "title": "repo: Run Command"
-      },
-      {
-        "command": "compose.createTicket",
-        "title": "repo: Create Ticket",
-        "icon": "$(add)"
-      },
-      {
-        "command": "compose.createPolicy",
-        "title": "repo: Create Policy",
-        "icon": "$(add)"
-      },
-      {
-        "command": "compose.createContributor",
-        "title": "repo: Add Contributor",
-        "icon": "$(add)"
-      },
-      {
-        "command": "compose.openPolicy",
-        "title": "repo: Open Policy"
-      },
-      {
-        "command": "compose.openContributor",
-        "title": "repo: Open Contributor"
-      },
-      {
-        "command": "compose.openTechnology",
-        "title": "repo: Open Bundle"
-      },
-      {
         "command": "compose.copyId",
         "title": "repo: Copy ID",
@@ -389,14 +106,4 @@

       },
       {
-        "command": "compose.copyCommitSha",
-        "title": "repo: Copy Commit SHA",
-        "icon": "$(copy)"
-      },
-      {
-        "command": "compose.openCommitInGitHub",
-        "title": "repo: Open Commit in GitHub",
-        "icon": "$(github)"
-      },
-      {
         "command": "compose.mailto",
         "title": "repo: Send Email"
@@ -405,12 +112,4 @@

         "command": "compose.openLink",
         "title": "repo: Open Link"
-      },
-      {
-        "command": "compose.goalOpen",
-        "title": "repo: Open/Create Goal"
-      },
-      {
-        "command": "compose.goalList",
-        "title": "repo: List Goals"
       },
       {
@@ -724,13 +423,15 @@

         "toggled": "compose.filter.commit.all",
         "title": "✅️"
+      },
+      {
+        "command": "compose.copyCheckpointSha",
+        "title": "repo: Copy Checkpoint SHA"
+      },
+      {
+        "command": "compose.openCheckpointInGitHub",
+        "title": "repo: Open Checkpoint in GitHub"
       }
     ],
-    "keybindings": [
-      {
-        "command": "compose.sectionRename",
-        "key": "f2",
-        "when": "view == compose.sections && viewItem == section"
-      }
-    ],
+    "keybindings": [],
     "menus": {
       "view/title": [
@@ -758,34 +459,4 @@

         },
         {
-          "command": "compose.ticketClose",
-          "when": "view == compose.monorepo && viewItem == ticket",
-          "group": "inline@1"
-        },
-        {
-          "command": "compose.ticketReopen",
-          "when": "view == compose.monorepo && viewItem == ticket",
-          "group": "inline@2"
-        },
-        {
-          "command": "compose.draftDelete",
-          "when": "view == compose.monorepo && viewItem == draft",
-          "group": "inline@1"
-        },
-        {
-          "command": "compose.copyCommitSha",
-          "when": "view == compose.monorepo && viewItem == commit",
-          "group": "inline@1"
-        },
-        {
-          "command": "compose.openCommitInGitHub",
-          "when": "view == compose.monorepo && viewItem == commit",
-          "group": "inline@2"
-        },
-        {
-          "command": "compose.checkPolicy",
-          "when": "view == compose.monorepo && viewItem == policy",
-          "group": "inline@1"
-        },
-        {
           "command": "compose.filter.toggle.technology.user",
           "toggled": "compose.filter.technology.user",
@@ -942,19 +613,4 @@

           "when": "view == compose.filter && viewItem == filter_ticket",
           "group": "inline@2"
-        },
-        {
-          "command": "compose.sectionRename",
-          "when": "view == compose.sections && viewItem == section",
-          "group": "inline"
-        },
-        {
-          "command": "compose.sectionCreateChild",
-          "when": "view == compose.sections && viewItem == section",
-          "group": "inline"
-        },
-        {
-          "command": "compose.sectionRemove",
-          "when": "view == compose.sections && viewItem == section",
-          "group": "inline"
         }
       ]

### 47 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json
HEAD 244 BASE 239 STASH 251 CURRENT H
--- 

+++ 

@@ -89,4 +89,16 @@

       ]
     },
+    "test-dashboard-only": {
+      "executor": "repo:script",
+      "cache": true,
+      "inputs": [
+        "default",
+        "^default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript",
+        "command": "bun ./📜️script.ts test-dashboard-only"
+      }
+    },
     "compile-extension": {
       "executor": "repo:script",

### 48 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts
HEAD 70 BASE 154 STASH 164 CURRENT H
--- 

+++ 

@@ -76,8 +76,8 @@

 }
 
-/** ⏱️The extension-host Mocha case (`🧪️tests/🧩️extension/🟦️.ts`) is this package's only test and can
- * only run inside the VSCode test harness, so it is added at `long` and above. */
+/** ⏱️Checks dashboard ownership at every level and runs extension-host navigation cases at `long` and above. */
 class TestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
+    await new DashboardOnlyTestScript(this.root, this.repoRoot, this.invocation).run();
     const { level } = resolveTestLevel(segments);
     await new TestGraphqlTypesScript(this.root, this.repoRoot, this.invocation).run([]);
@@ -94,4 +94,13 @@

     const control = scriptCompilerAdmissionControlV1(this.invocation, "vscode:compile-extension", 600_000), lease = await acquireCompilerCapacityLeaseV1({ directory: join(this.repoRoot, ".cache/framework/leases"), control, environment: process.env });
     try { await buildExtension(this.root, false, resolve(args[0])); } finally { lease.release(); }
+  }
+}
+
+
+
+/** 🎛️Proves the extension retains source navigation while repository actions belong to the dashboard. */
+class DashboardOnlyTestScript extends BundleScript {
+  async run(): Promise<void> {
+    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🎛️dashboard-only/🟦️.ts")], this.root, "repo-vscode:dashboard-only", 30_000, { signal: this.invocation.control.signal });
   }
 }
@@ -142,4 +151,5 @@

   .register("dev", DevScript)
   .register("test", TestScript)
+  .register("test-dashboard-only", DashboardOnlyTestScript)
   .register("build", BuildScript)
   .register("compile-extension", CompileExtensionScript)

### 49 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts
HEAD 130 BASE 1969 STASH 1835 CURRENT H
--- 

+++ 

@@ -12,4 +12,5 @@

 import * as path from "path";
 import * as vscode from "vscode";
+import dashboardOnly from "../../🧫️fixtures/🎛️dashboard-only/🔣️.json";
 import { buildCliTreeArgs, FilterTreeDataProvider, FilterTreeItem, MonorepoTreeDataProvider, MonorepoTreeItem, treeNodeToItem } from "../../🌳️tree-providers/🟦️.ts";
 import { buildEntityEmojiPattern, buildEntityIdRegex, ENTITY_EMOJIS, ENTITY_ID_REGEX } from "../../🌪️entity-emoji-registry/🟦️.ts";
@@ -30,40 +31,4 @@

 // 🔒️#region 🎞️Constants
 const EXPECTED_COMMANDS = [
-  "compose.analyze",
-  "compose.analyzeFile",
-  "compose.autofix",
-  "compose.autofixFile",
-  "compose.policyList",
-  "compose.ticketOpen",
-  "compose.ticketList",
-  "compose.ticketClose",
-  "compose.ticketRead",
-  "compose.ticketOpen",
-  "compose.technologyList",
-  "compose.contributorAdd",
-  "compose.contributorList",
-  "compose.contributorRemove",
-  "compose.sectionTree",
-  "compose.definitionList",
-  "compose.folderTree",
-  "compose.folderCreate",
-  "compose.folderMove",
-  "compose.folderDelete",
-  "compose.folderList",
-  "compose.fileCreate",
-  "compose.fileMove",
-  "compose.fileDelete",
-  "compose.fileList",
-  "compose.fileTree",
-  "compose.sectionCreate",
-  "compose.sectionMove",
-  "compose.sectionDelete",
-  "compose.sectionIntegrate",
-  "compose.sectionList",
-  "compose.definitionTree",
-  "compose.technologyTree",
-  "compose.policyCheck",
-  "compose.refreshDiagnostics",
-  "compose.autofixBreach",
   "compose.copyId",
   "compose.mailto",
@@ -71,10 +36,21 @@

   "compose.refreshMonorepo",
   "compose.refreshCodebase",
+  "compose.refreshItem",
+  "compose.filter.search",
+  "compose.filter.toggle",
+  "compose.filter.toggleYear",
+  "compose.filter.toggleMonth",
+  "compose.filter.toggleDay",
+  "compose.navigateToBundle",
+  "compose.navigateToFolder",
+  "compose.navigateToFile",
+  "compose.navigateToSection",
+  "compose.navigateToDefinition",
+  "compose.navigate",
+  "compose.navigateTo",
+  "compose.ticketOpen",
   "compose.copyCheckpointSha",
   "compose.openCheckpointInGitHub",
-  "compose.ticketReopen",
-  "compose.refreshItem",
-  "compose.navigate",
-  "compose.navigateTo",
+  "compose.open"
 ];
 const EXPECTED_CONSTRAINTS = [
@@ -259,14 +235,4 @@

 }
 
-async function getAnalyzeLensIds(document: vscode.TextDocument): Promise<string[]> {
-  const lenses = await getCodeLenses(document);
-  const ids = lenses
-    .filter((lens) => lens.command?.command === "compose.analyze")
-    .map((lens) => String(lens.command?.arguments?.[0] ?? ""))
-    .filter((id) => id.length > 0);
-
-  return ids;
-}
-
 async function getCodeLenses(document: vscode.TextDocument): Promise<vscode.CodeLens[]> {
   await vscode.window.showTextDocument(document, { preview: true, preserveFocus: false });
@@ -530,16 +496,4 @@

 suite("Refresh Diagnostics Test Suite", function () {
   this.timeout(15000);
-
-  test("compose.refreshDiagnostics updates all open documents", async function () {
-    const document = await openFixture("compose/invalid.kit.compose.json");
-    await vscode.commands.executeCommand("compose.refreshDiagnostics");
-    await new Promise((resolve) => setTimeout(resolve, 3000));
-    const diagnostics = vscode.languages.getDiagnostics(document.uri).filter((d) => d.source === "compose");
-    if (diagnostics.length === 0) {
-      console.log("Skipping: validation may be disabled due to bundling issues");
-      return;
-    }
-    assert.ok(diagnostics.length > 0, "Diagnostics should be present after refresh");
-  });
 });
 
@@ -613,9 +567,4 @@

     const commands = await vscode.commands.getCommands(true);
     assert.ok(commands.includes("compose.openCheckpointInGitHub"), "openCheckpointInGitHub command should be registered");
-  });
-
-  test("Ticket reopen command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.ticketReopen"), "ticketReopen command should be registered");
   });
 
@@ -673,44 +622,4 @@

   test("Sections view can be focused", async function () {
     await vscode.commands.executeCommand("compose.sections.focus");
-  });
-
-  test("sectionTree command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionTree"), "sectionTree command should be registered");
-  });
-
-  test("sectionList command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionList"), "sectionList command should be registered");
-  });
-
-  test("sectionCreate command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionCreate"), "sectionCreate command should be registered");
-  });
-
-  test("sectionMove command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionMove"), "sectionMove command should be registered");
-  });
-
-  test("sectionDelete command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionDelete"), "sectionDelete command should be registered");
-  });
-
-  test("sectionOpen command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionOpen"), "sectionOpen command should be registered");
-  });
-
-  test("sectionRename command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionRename"), "sectionRename command should be registered");
-  });
-
-  test("sectionIntegrate command is available", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.sectionIntegrate"), "sectionIntegrate command should be registered");
   });
 
@@ -1902,68 +1811,25 @@

   });
 
-  test("compose.analyze does not throw on unknown ID", async function () {
-    await vscode.commands.executeCommand("compose.analyze", "🧰️unknownentity");
-    assert.ok(true, "should not throw on unknown entity");
-  });
-
-  test("compose.analyze command is registered", async function () {
-    const commands = await vscode.commands.getCommands(true);
-    assert.ok(commands.includes("compose.analyze"), "analyze command should be registered");
-  });
-
-  test("Analyze CodeLens covers every definition entity in TypeScript and Go files", async function () {
-    const cases = [
-      {
-        label: "TypeScript",
-        paths: ["repo/vscode/🟦️.ts", "vscode/🟦️.ts", "🟦️.ts"],
-      },
-      {
-        label: "Go",
-        paths: ["repo/go/main.go", "go/main.go", "main.go"],
-      },
-    ];
-
-    for (const testCase of cases) {
-      const document = await openWorkspaceDocument(...testCase.paths);
-      const expectedIds = collectDefinitionEntityIds(document.getText());
-      assert.ok(expectedIds.length > 0, `expected definition IDs in ${testCase.label} source: ${document.uri.fsPath}`);
-
-      const analyzeLensIds = new Set(await getAnalyzeLensIds(document));
-      const missingIds = expectedIds.filter((id) => !analyzeLensIds.has(id));
-
-      assert.deepStrictEqual(missingIds, [], `missing Analyze CodeLens IDs in ${testCase.label} source ${document.uri.fsPath}: ${missingIds.join(", ")}`);
-    }
-  });
-
   test("Entity CodeLenses do not expose summarize commands", async function () {
     const document = await openWorkspaceDocument("repo/vscode/🟦️.ts");
     const summarizeLenses = (await getCodeLenses(document)).filter((lens) => lens.command?.command === "compose.summarize");
-    assert.deepStrictEqual(summarizeLenses, [], "summarize CodeLenses should be fully replaced by analyze");
-  });
-
-  test("Analyze CodeLens covers native definition scopes in TypeScript and Go files", async function () {
-    const cases = [
-      {
-        label: "TypeScript",
-        paths: ["repo/vscode/🟦️.ts", "vscode/🟦️.ts", "🟦️.ts"],
-      },
-      {
-        label: "Go",
-        paths: ["repo/go/main.go", "go/main.go", "main.go"],
-      },
-    ];
-
-    for (const testCase of cases) {
-      const document = await openWorkspaceDocument(...testCase.paths);
-      const expectedScopes = await collectNativeDefinitionScopes(document);
-      assert.ok(expectedScopes.length > 0, `expected native definition scopes in ${testCase.label} source: ${document.uri.fsPath}`);
-
-      const analyzeLensIds = new Set(await getAnalyzeLensIds(document));
-      const missingScopes = expectedScopes.filter((scope) => !analyzeLensIds.has(scope));
-
-      assert.deepStrictEqual(missingScopes, [], `missing native Analyze CodeLens scopes in ${testCase.label} source ${document.uri.fsPath}: ${missingScopes.join(", ")}`);
-    }
+    assert.deepStrictEqual(summarizeLenses, [], "summarize CodeLenses remain retired");
   });
 });
 
 // #endregion 🔋️CodeLens Behavior Tests
+
+/** 🎛️ Keeps repository operations exclusively in the dashboard while retaining navigation. */
+suite("Exclusive Dashboard Operation Ownership", () => {
+  test("does not register repository operations", async () => {
+    const commands = await vscode.commands.getCommands(true);
+    for (const command of dashboardOnly.forbiddenCommands) assert.ok(!commands.includes(command), command);
+    for (const command of dashboardOnly.retainedCommands) assert.ok(commands.includes(command), command);
+  });
+  test("provides navigation lenses without repository action lenses", async () => {
+    const document = await openWorkspaceDocument("🟦️.ts");
+    const lenses = await getCodeLenses(document);
+    assert.ok(lenses.some(lens => lens.command?.command === "compose.navigate" || lens.command?.command === "compose.navigateToDefinition"));
+    assert.deepStrictEqual(lenses.filter(lens => dashboardOnly.forbiddenCommands.includes(lens.command?.command ?? "")), []);
+  });
+});

### 50 🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🐹️go/🐹️.go
HEAD 209 BASE 7010 STASH 7001 CURRENT H
--- 

+++ 

@@ -1588,5 +1588,5 @@

 	Plan          *TicketPlan           `json:"plan,omitempty" yaml:"plan,omitempty"`
 	Sessions      []string              `json:"sessions,omitempty" yaml:"sessions,omitempty"`
-	Interactions  []Interaction         `json:"-" yaml:"-"`
+	Interactions  []Interaction         `json:"interactions,omitempty" yaml:"interactions,omitempty"`
 	Agents        []TicketAgent         `json:"-" yaml:"-"`
 	FolderPath    string                `json:"-" yaml:"-"`
@@ -2768,7 +2768,5 @@

 const BreachRepoMissingTicketTracking Statute = "repo/missing-ticket-tracking"
 
-const BreachSystemDevcontainerVscodeSettingsDrift Statute = "system/devcontainer/vscode/settings-drift"
-
-const BreachSystemDevcontainerVscodeExtensionsDrift Statute = "system/devcontainer/vscode/extensions-drift"
+const BreachSystemIdeConfiguration Statute = "system/ide-configuration"
 
 const BreachFolderIllegalEmpty Statute = "folder/illegal/empty"
@@ -3182,16 +3180,9 @@

 		Autofixable: false,
 	},
-	BreachSystemDevcontainerVscodeSettingsDrift: {
-		Kind:        BreachSystemDevcontainerVscodeSettingsDrift,
+	BreachSystemIdeConfiguration: {
+		Kind:        BreachSystemIdeConfiguration,
 		Priority:    BreachPriorityHigh,
-		Reason:      "The devcontainer editor settings are derived from .vscode/settings.json and .devcontainer/editor-overlay.json; a hand-edited copy drifts from the host editor",
-		Solution:    "Run `bun repo run workspace:generate-config` to rewrite the devcontainer editor block; edit .vscode/settings.json, .vscode/extensions.json or the overlay instead of the copy",
-		Autofixable: false,
-	},
-	BreachSystemDevcontainerVscodeExtensionsDrift: {
-		Kind:        BreachSystemDevcontainerVscodeExtensionsDrift,
-		Priority:    BreachPriorityHigh,
-		Reason:      "The devcontainer extensions are derived from .vscode/extensions.json and .devcontainer/editor-overlay.json; a hand-edited copy drifts from the host editor",
-		Solution:    "Run `bun repo run workspace:generate-config` to rewrite the devcontainer editor block; edit .vscode/settings.json, .vscode/extensions.json or the overlay instead of the copy",
+		Reason:      "Developer commands and configuration belong to the repository dashboard",
+		Solution:    "Remove editor-owned configuration and register the command in the repository dashboard",
 		Autofixable: false,
 	},

### 51 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
HEAD 457 BASE 442 STASH 417 CURRENT H
--- 

+++ 

@@ -8,5 +8,5 @@

 import { policyAllRustFiles } from "../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
 import { policyReadMemberStoreProducerSources, policyReadOsStoreSource, policyReadRustPolicySource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
-import { toolJobArtifactEnvelopeOwnedCodecExact, toolJobArtifactStoreStructuralOwnersExact, toolJobChildContentRootExact, toolJobChildRetirementInventoryExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobImmutableOperationRootsExact, toolJobJackEnvelopeCallerRetainedExact, toolJobPagedIngressExact, toolJobPeerInteractionRootsExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact } from "../🗄️artifact-store/🟦️.ts";
+import { toolJobArtifactEnvelopeOwnedCodecExact, toolJobArtifactStoreStructuralOwnersExact, toolJobChildContentRootExact, toolJobChildRetirementInventoryExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobImmutableOperationRootsExact, toolJobJackEnvelopeCallerRetainedExact, toolJobPagedIngressExact, toolJobPeerInteractionRootsExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobRasterEnvelopeSourceEvidence, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact } from "../🗄️artifact-store/🟦️.ts";
 import { toolJobFrameworkReservedRoutesExact, toolJobMicrosecondWorkerExact, toolJobPluginProductionSources, toolJobProofCatalogFailures } from "../🏭️factory/🟦️.ts";
 import { type FixedOperationFixtureOutput, toolJobDrawingGestureOperationOwnerExact, toolJobFixedOperationFixtureRun, toolJobFixedOperationRegistryExact, toolJobFixedOperationRustFixtureSource, toolJobSharedFrameworkActionFixtureRun } from "../⚙️fixed-operation/🟦️.ts";
@@ -191,31 +191,6 @@

   const gisMapEditor = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
   const gisMapWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
-  const rasterRetainedSource = (file: string): string => {
-    const source = policyReadFileSafe(root, file);
-    if (source.trim() === "") throw new Error(`[verify interactivity tool-jobs] retained Raster envelope source ${JSON.stringify(file)} is missing or empty; the Raster envelope law cannot be evaluated against a path that does not resolve.`);
-    return source;
-  };
-  const rasterMountedOutputSerializers = [
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs",
-  ]
-    .map(rasterRetainedSource);
-  const rasterEnvelopeCodec = [
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
-    "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs",
-  ]
-    .map(rasterRetainedSource)
-    .concat(rasterMountedOutputSerializers)
-    .join("\n");
-  const rasterEditor = rasterRetainedSource("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
-  const drawingEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs");
+  const rasterEvidence = toolJobRasterEnvelopeSourceEvidence(root);
+  const drawingEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs");
   const drawingEditorSources = [
     "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
@@ -223,5 +198,5 @@

     "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs",
   ].map((file) => policyReadFileSafe(root, file));
-  const drawingEditorTestLaws = drawingEditorSources[0] ?? "";
+  const drawingEditorTestLaws = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs");
   const drawingEditor = drawingEditorSources
     .map(toolJobProductionSource)
@@ -304,6 +279,6 @@

   if (!toolJobTrinityRewriteEnvelopeCallerRetainedExact(store, jackEnvelopeCodec, jackEditor, trinityRewrite, plugin)) failures.push("Trinity Rewrite envelope caller lacks the Jack-owned fixed-page operation store, generation handle, bounded progress/cancel/close, exact rejected-page handback, or completion acknowledgement");
   if (!toolJobGisMapEnvelopeCallerRetainedExact(store, gisMapEnvelopeCodec, gisMapEditor, gisMapWasm, plugin)) failures.push("GIS Map `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, exact drawing/image/value child retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement");
-  if (!toolJobRasterEnvelopeCallerRetainedExact(store, rasterEnvelopeCodec, rasterEditor, plugin)) failures.push("Raster `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, recursive domain retirement, the editor-hosted credit-before-page fixed-page ingress cohort (submit/pump/exact-ack plus cancel), initializer recovery, the guarded owned-map `dsl::ToValue`/`dsl::FromValue` output authority, or its eight mounted export serializers are not the six real format encoders plus two typed `RASTER_*_EXPORT_UNSUPPORTED` declines");
-  if (!toolJobDrawingEnvelopeCallerRetainedExact(store, drawingEnvelopeCodec, drawingEditor, plugin, drawingEditorTestLaws)) failures.push("Drawing `.spr`/`.ops` envelope caller lacks recursive layer/style/asset preflight, one-item clone/retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement");
+  if (!toolJobRasterEnvelopeCallerRetainedExact(rasterEvidence.store, rasterEvidence.raster, rasterEvidence.editor, rasterEvidence.plugin, rasterEvidence.laws)) failures.push("Raster `.spr`/`.ops` envelope caller lacks its owned field catalog, framework replay initializer, funded central mutation preparation or mounted grant/cancellation laws");
+  if (!toolJobDrawingEnvelopeCallerRetainedExact(store, drawingEnvelopeCodec, drawingEditor, plugin, drawingEditorTestLaws)) failures.push("Drawing `.spr`/`.ops` envelope caller lacks the owned field catalog, the read-only native layer utilities, the framework-default replay initializer through Mutation::diff and protocol::apply_diff (a host or editor owning a second apply lane or initializer is refused), or its live replay/cancellation/acknowledgement laws");
   if (!toolJobChildRetirementInventoryExact(root, allRustFiles)) failures.push("child snapshot retirement domain cohorts or callsites do not match the exact machine-readable owner inventory");
   if (!toolJobPeerInteractionRootsExact(plugin, store, channel, presenceRetirement)) failures.push("peer ingress, app-typed presence, or interaction roots lack reserve-before-decode retained per-entry publication, atomic validated commit, or O(1) immutable capture");

### 52 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts
HEAD 485 BASE 359 STASH 360 CURRENT H
--- 

+++ 

@@ -18,5 +18,5 @@

 
 /** 🧭️ Resolves one crate module through its package's explicit taxonomy declarations. */
-function toolJobFactoryModuleFile(files: ReadonlyMap<string, string>, ownerFile: string, modulePath: string): string | undefined {
+export function toolJobFactoryModuleFile(files: ReadonlyMap<string, string>, ownerFile: string, modulePath: string): string | undefined {
   const candidates = new Set<string>();
   const visit = (source: string, directory: string, prefix: string): void => {
@@ -29,4 +29,5 @@

         const exportedComponent = prefix === modulePath && match[2] === "component" && /pub\s+use\s+component::\*\s*;/.test(source);
         if ((name === modulePath || exportedComponent) && files.has(target)) candidates.add(target);
+        else if (modulePath.startsWith(`${name}::`) && files.has(target)) visit(files.get(target)!, posix.dirname(target), name);
       } else {
         const block = toolJobRustBlock(source, source.indexOf("{", match.index));

### 53 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts
HEAD 676 BASE 537 STASH 629 CURRENT H
--- 

+++ 

@@ -1,2 +1,6 @@

+import { WORKSPACE_ROOT } from "../../../🗂️workspaces/📍️checkout-root/🟦️.ts";
+import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
+import { toolJobFactoryModuleFile } from "../🏭️factory/🟦️.ts";
+import { rustTokens, rustTokenPairs, rustTokenSegments, rustAttributes, rustVisibility } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 /** 📋️ Tool-job coverage inventory: reserved ids, static rows, publication contracts, app-owned rows, dispositions and proofs.
  * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
@@ -140,5 +144,5 @@

 export type ToolJobStaticRow = { file: string; id: string; source: "literal" | "macro" };
 
-type ToolJobPublicationLane = "HostOnly" | "Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child";
+type ToolJobPublicationLane = "HostOnly" | "Artifact" | "Config" | "WindowConfig" | "WindowTransient" | "Draft" | "Presence" | "Transient" | "Child";
 type ToolJobAppOwnedRow = {
   ownerFile: string;
@@ -189,31 +193,43 @@

 }
 
+/** 🧹️ Masks only test-only Rust items, preserving neighboring fields and production cfg alternatives. */
 export function toolJobProductionProtocolSource(raw: string): string {
-  let source = toolJobProductionSource(raw);
-  const marker = /#\[cfg\((?:all\([^\]]*\btest\b[^\]]*\)|test)\)\]/g;
-  for (;;) {
-    const match = marker.exec(source);
-    if (!match) return source;
-    const item = match.index + match[0].length;
-    let parentheses = 0;
-    let brackets = 0;
-    let end = source.length;
-    for (let index = item; index < source.length; index += 1) {
-      const char = source[index]!;
-      if (char === "(") parentheses += 1;
-      else if (char === ")") parentheses -= 1;
-      else if (char === "[") brackets += 1;
-      else if (char === "]") brackets -= 1;
-      else if (char === ";" && parentheses === 0 && brackets === 0) {
-        end = index + 1;
-        break;
-      } else if (char === "{" && parentheses === 0 && brackets === 0) {
-        end = toolJobRustBlock(source, index)?.end ?? source.length;
-        break;
-      }
-    }
-    source = `${source.slice(0, match.index)}${" ".repeat(end - match.index)}${source.slice(end)}`;
-    marker.lastIndex = 0;
-  }
+  const source = raw, tokens = rustTokens(source), pairs = rustTokenPairs(tokens), spans: [number, number][] = [];
+  const production = (start: number, end: number): number => {
+    if (end === start + 1 && tokens[start]?.text === "test") return 0;
+    const name = tokens[start]?.text, open = start + 1, close = pairs.get(open);
+    if (tokens[open]?.text !== "(" || close !== end - 1 || !["all", "any", "not"].includes(name ?? "")) return 2;
+    const values = rustTokenSegments(tokens, pairs, open + 1, close, ",").map(([a, b]) => production(a, b));
+    if (name === "not") return values.length === 1 && values[0] !== 2 ? 1 - values[0]! : 2;
+    if (name === "all") return values.includes(0) ? 0 : values.every(value => value === 1) ? 1 : 2;
+    return values.includes(1) ? 1 : values.every(value => value === 0) ? 0 : 2;
+  };
+  for (let index = 0; index < tokens.length; index++) {
+    if (tokens[index]?.text !== "#" || tokens[index + 1]?.text !== "[") continue;
+    const attributes = rustAttributes(tokens, pairs, index, tokens.length);
+    const testOnly = attributes.ranges.some(([a, b]) => tokens[a]?.text === "cfg" && tokens[a + 1]?.text === "(" && pairs.get(a + 1) === b - 1 && production(a + 2, b - 1) === 0);
+    if (!testOnly) { index = attributes.next - 1; continue; }
+    const definition = rustVisibility(tokens, pairs, attributes.next).next, field = tokens[definition]?.kind === "identifier" && tokens[definition + 1]?.text === ":";
+    let angle = 0, last = -1, expression = false;
+    for (let cursor = attributes.next; cursor < tokens.length; cursor++) {
+      const text = tokens[cursor]!.text, paired = pairs.get(cursor);
+      if (text === "=") expression = true;
+      if (text === "<" && (!expression || tokens[cursor - 1]?.text === "::")) { angle++; continue; }
+      if (/^>+$/.test(text)) { angle = Math.max(0, angle - text.length); continue; }
+      if (paired !== undefined && paired > cursor) {
+        if (text === "{" && !angle && !field && tokens[cursor - 1]?.text !== "!") { last = paired + (tokens[paired + 1]?.text === "," ? 1 : 0); break; }
+        cursor = paired;
+        continue;
+      }
+      if (!angle && [",", ";"].includes(text)) { last = cursor; break; }
+      if (!angle && [")", "}"].includes(text)) { last = cursor - 1; break; }
+    }
+    if (last < attributes.next) throw new Error("[production Rust source] test item boundary is not proved");
+    spans.push([tokens[index]!.start, tokens[last]!.end]);
+    index = last;
+  }
+  let result = source;
+  for (const [start, end] of spans.reverse()) result = result.slice(0, start) + source.slice(start, end).replace(/[^\r\n]/g, " ") + result.slice(end);
+  return result;
 }
 
@@ -314,5 +330,6 @@

   strings: ReadonlyMap<string, string>,
 ): Map<string, ToolJobPublicationLane[]> | undefined {
-  const expression = factoryBody.match(/const\s+PUBLICATION_CONTRACTS\s*:\s*&'static\s*\[\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\]\s*=\s*([^;]+);/s)?.[1]?.trim();
+  const assignment = factoryBody.match(/const\s+PUBLICATION_CONTRACTS\s*:\s*&'static\s*\[\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\]\s*=\s*(&\[|(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*)/s);
+  const expression = assignment?.[1];
   if (!expression) return undefined;
   const constant = expression.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)$/)?.[1];
@@ -320,5 +337,6 @@

   const open = declaration < 0 ? -1 : source.indexOf("[", source.indexOf("=", declaration));
   const array = open < 0 ? undefined : toolJobRustArray(source, open);
-  const contracts = constant ? array?.body : expression;
+  const inlineOpen = expression === "&[" && assignment ? factoryBody.indexOf("[", assignment.index! + assignment[0].length - 2) : -1;
+  const contracts = constant ? array?.body : inlineOpen >= 0 ? toolJobRustArray(factoryBody, inlineOpen)?.body : undefined;
   if (!contracts) return undefined;
   const result = new Map<string, ToolJobPublicationLane[]>();
@@ -327,12 +345,14 @@

   while ((match = row.exec(contracts))) {
     const toolId = match[1] ?? strings.get(match[2] ?? "");
-    const lanes = [...match[3]!.matchAll(/(?:semio_framework_plugin::)?ArtifactToolPublicationLane::(HostOnly|Artifact|Config|Draft|Presence|Transient|Child)/g)].map((lane) => lane[1] as ToolJobPublicationLane);
+    const names = match[3]!.split(",").map((lane) => lane.trim()).filter(Boolean);
+    const lanes = names.map((lane) => lane.match(/^(?:semio_framework_plugin::)?ArtifactToolPublicationLane::(HostOnly|Artifact|Config|WindowConfig|WindowTransient|Draft|Presence|Transient|Child)$/)?.[1] as ToolJobPublicationLane | undefined);
+    if (lanes.some((lane) => lane === undefined)) return undefined;
     if (!toolId || result.has(toolId) || lanes.length === 0 || new Set(lanes).size !== lanes.length || (lanes.includes("HostOnly") && lanes.length !== 1)) return undefined;
-    result.set(toolId, lanes);
+    result.set(toolId, lanes as ToolJobPublicationLane[]);
   }
   return result.size > 0 ? result : undefined;
 }
 
-export function toolJobPublicationAuthorityReady(ownerBody: string, lanes: readonly ToolJobPublicationLane[]): boolean {
+export function toolJobPublicationAuthorityReady(ownerBody: string, lanes: readonly ToolJobPublicationLane[], files: ReadonlyMap<string, string> = new Map(), ownerFile: string = ""): boolean {
   const factory = (name: string) => {
     const start = ownerBody.search(new RegExp(`fn\\s+${name}\\s*\\(`));
@@ -344,4 +364,31 @@

     if (lane === "HostOnly") return true;
     if (lane === "Child") return false;
+    if (lane === "WindowConfig" || lane === "WindowTransient") {
+      const name = lane === "WindowConfig" ? "register_window_config_owners" : "register_window_transient_owners";
+      const start = ownerBody.search(new RegExp(`fn\\s+${name}\\s*\\(`));
+      const body = start < 0 ? undefined : toolJobRustBlock(ownerBody, ownerBody.indexOf("{", start))?.body;
+      if (!body) return false;
+      if (/registry\.register::<[A-Za-z_][A-Za-z0-9_:]*>\(\)/.test(body)) return true;
+      const source = files.get(ownerFile) ?? "";
+      const calls = [...body.matchAll(/([A-Za-z_][A-Za-z0-9_:]*)::register\(registry\)/g)];
+      return calls.length > 0 && calls.every((call) => {
+        const parts = call[1]!.split("::");
+        const imports = [...source.matchAll(/use\s+(crate(?:::[A-Za-z_][A-Za-z0-9_]*)+)::\{([^}]+)\}\s*;/g)].flatMap((group) => group[2]!.split(",").map((item) => {
+          const entry = item.trim().match(/^([A-Za-z_][A-Za-z0-9_]*)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?$/);
+          return entry && (entry[2] ?? entry[1]) === parts[0] ? `${group[1]}::${entry[1]}::${parts.slice(1).join("::")}` : undefined;
+        })).filter((item): item is string => !!item);
+        const path = call[1]!.startsWith("crate::") ? call[1]! : imports.length === 1 ? imports[0]! : "";
+        const file = path ? toolJobFactoryModuleFile(files, ownerFile, path) : undefined;
+        const registration = file ? files.get(file) ?? "" : "";
+        const start = registration.search(/(?:pub(?:\([^)]*\))?\s+)?fn\s+register\s*\(/);
+        const registered = start < 0 ? undefined : toolJobRustBlock(registration, registration.indexOf("{", start))?.body;
+        if (registered) return /registry\.register::<[A-Za-z_][A-Za-z0-9_:]*>\(\)/.test(registered);
+        if (lane !== "WindowTransient") return false;
+        const macros = [...registration.matchAll(/semio_framework_plugin::window_transient_owners!\s*\{/g)];
+        if (macros.length !== 1) return false;
+        const body = toolJobRustBlock(registration, registration.indexOf("{", macros[0]!.index))?.body;
+        return !!body && /^\s*state:\s*[A-Za-z_][A-Za-z0-9_]*,\s*mutation:\s*[A-Za-z_][A-Za-z0-9_]*,\s*windows:\s*\{(?:\s*[A-Za-z_][A-Za-z0-9_]*\s*=>\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*\s*,)+\s*\},?\s*$/.test(body);
+      });
+    }
     if (lane === "Artifact") return factory("build_artifact_store_one_item_preparation_factory");
     if (lane === "Config") return factory("build_config_store_one_item_preparation_factory");
@@ -426,5 +473,5 @@

           toolId,
           publicationLanes,
-          publicationReady: exactPublication && toolJobPublicationAuthorityReady(owner.body, publicationLanes),
+          publicationReady: exactPublication && toolJobPublicationAuthorityReady(owner.body, publicationLanes, files, owner.file),
         });
       }
@@ -467,8 +514,51 @@

 };
 
+/** 🧮️ Resolves static contracts through exact constants and side-effect-free zero-argument helpers. */
+export function toolJobContractSource(source: string, expression: string): { execution: "bounded" | "resumable"; values: number[] } | undefined {
+  const external = new Map([
+    ["semio_framework::kernel::COMMAND_MAXIMUM_BYTES", ["🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs", "COMMAND_MAXIMUM_BYTES"]],
+    ["semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES", ["🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs", "GUEST_HOST_ANSWER_CEILING_BYTES"]],
+  ]);
+  const number = (text: string, owner: string, seen: Set<string>): number | undefined => {
+    const value = text.trim().replace(/\s+as\s+(?:u64|usize)$/, "");
+    if (/^[0-9][0-9_]*$/.test(value)) { const result = Number(value.replaceAll("_", "")); return Number.isSafeInteger(result) ? result : undefined; }
+    if (seen.has(value) || seen.size > 32) return undefined;
+    const next = new Set(seen).add(value);
+    const imported = external.get(value);
+    if (imported) return number(imported[1]!, policyReadFileSafe(WORKSPACE_ROOT, imported[0]!), next);
+    const work = value.match(/^([A-Z][A-Z0-9_]*)\.work_items\(\)$/);
+    if (work) {
+      const rows = [...owner.matchAll(new RegExp(`const\\s+${work[1]}\\s*:\\s*ArtifactRetainedWorkCapacity\\s*=\\s*ArtifactRetainedWorkCapacity::for_invertible_items\\(([^()]*)\\)\\s*;`, "g"))];
+      const retained = policyReadFileSafe(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs").replace(/\s+/g, "");
+      if (rows.length !== 1 || !retained.includes("pubconstfnwork_items(self)->usize{self.rows(self.invertible_items)}") || !retained.includes("items.saturating_mul(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)")) return undefined;
+      const items = number(rows[0]![1]!, owner, next);
+      const itemSource = policyReadFileSafe(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🦀️.rs");
+      const multiplier = number("ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS", itemSource, next);
+      const result = items === undefined || multiplier === undefined ? undefined : items * multiplier;
+      return result !== undefined && Number.isSafeInteger(result) ? result : undefined;
+    }
+    if (!/^[A-Z][A-Z0-9_]*$/.test(value)) return undefined;
+    const constants = [...owner.matchAll(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?const\\s+${value}\\s*:\\s*(?:usize|u64|u32)\\s*=\\s*([^;]+);`, "g"))];
+    return constants.length === 1 ? number(constants[0]![1]!, owner, next) : undefined;
+  };
+  const resolve = (value: string, seen: Set<string>): { execution: "bounded" | "resumable"; values: number[] } | undefined => {
+    const constructor = value.trim().match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^()]*)\)$/);
+    if (constructor) {
+      const values = constructor[2]!.split(",").filter((part) => part.trim()).map((part) => number(part, source, new Set()));
+      return values.every((part) => part !== undefined) ? { execution: constructor[1] === "resumable" ? "resumable" : "bounded", values: values as number[] } : undefined;
+    }
+    const call = value.trim().match(/^([a-z_][a-z0-9_]*)\(\)$/);
+    if (!call || seen.has(call[1]!) || seen.size > 32) return undefined;
+    const declarations = [...source.matchAll(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?(?:const\\s+)?fn\\s+${call[1]}\\(\\)\\s*->\\s*ToolExecutionContract\\s*\\{`, "g"))];
+    if (declarations.length !== 1) return undefined;
+    const body = toolJobRustBlock(source, source.indexOf("{", declarations[0]!.index))?.body.trim().replace(/^return\s+/, "").replace(/;$/, "");
+    return body ? resolve(body, new Set(seen).add(call[1]!)) : undefined;
+  };
+  return resolve(expression, new Set());
+}
+
 /** 📐️ Reads only owner-local typed bounded-first-step proof catalogs. */
 export function toolJobProofs(files: ReadonlyMap<string, string>): ToolJobProof[] {
   const proofs: ToolJobProof[] = [];
-  const valuesOf = (source: string): number[] => source.split(",").map((value) => Number(value.trim().replaceAll("_", "")));
   const validValues = (values: readonly number[]) => (values.length === 5 || values.length === 7) && values.every((value) => Number.isFinite(value) && value > 0) && values[4]! < 8_000;
   for (const [sourceFile, source] of files) {
@@ -484,13 +574,15 @@

         const mappedRows = [...block.body.matchAll(/"([^"]+)"\s*=>\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^)]*)\)/g)];
         for (const row of mappedRows) {
-          const values = valuesOf(row[3]!);
-          if (validValues(values)) proofs.push({ ...common, toolId: row[1]!, execution: row[2] === "resumable" ? "resumable" : "bounded", values });
+          const contract = toolJobContractSource(source, `ToolExecutionContract::${row[2]}(${row[3]})`);
+          const values = contract?.values ?? [];
+          if (validValues(values)) proofs.push({ ...common, toolId: row[1]!, execution: contract!.execution, values });
         }
         if (mappedRows.length === 0) {
-          const sharedContract = block.body.match(/contract:\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^)]*)\)/);
+          const sharedExpression = block.body.match(/contract:\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(?:bounded_first_step|resumable)\([^)]*\)|[a-z_][a-z0-9_]*\(\))\s*,/);
+          const sharedContract = sharedExpression ? toolJobContractSource(source, sharedExpression[1]!) : undefined;
           const sharedTools = block.body.match(/tools:\s*\[([^\]]*)\]/s);
-          const values = valuesOf(sharedContract?.[2] ?? "");
+          const values = sharedContract?.values ?? [];
           if (sharedTools && validValues(values)) {
-            for (const tool of sharedTools[1]!.matchAll(/"([^"]+)"/g)) proofs.push({ ...common, toolId: tool[1]!, execution: sharedContract?.[1] === "resumable" ? "resumable" : "bounded", values });
+            for (const tool of sharedTools[1]!.matchAll(/"([^"]+)"/g)) proofs.push({ ...common, toolId: tool[1]!, execution: sharedContract?.execution === "resumable" ? "resumable" : "bounded", values });
           }
         }

### 54 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts
HEAD 452 BASE 374 STASH 345 CURRENT H
--- 

+++ 

@@ -48,10 +48,19 @@

   if (count(source, workerSignature) !== 1) return false;
   const direct = "self.start_typed_command_operation(command, admission, meta, operation_id, None).await";
-  const stages = ["dispatch_wire_retained_with_spec", "MountedWorkerJobSession::try_new", "self.tool_operations.insert_admitted(", "active.drive_worker_step(self.mounted_policy.maintenance)"];
+  const stages = ["dispatch_wire_retained_with_spec", "crate::reserved_job_session::admit_mounted", "self.tool_operations.insert_admitted(", "active.drive_worker_step(self.mounted_policy.maintenance)"];
   const offsets = stages.map((token) => setup.indexOf(token));
-  return count(inner.body, direct) === 1 && stages.every((token) => count(setup, token) === 1)
+  const admissionStart = source.indexOf("pub(crate) fn admit_mounted<");
+  const admission = admissionStart < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", admissionStart));
+  const macroStart = source.indexOf("macro_rules! admit_session");
+  const owner = macroStart < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", macroStart));
+  const custody = ["let mut job = Some($job)", "let mut params = Some($params)", "(original.operation, original.generation, original.now_us)", "let mut recipient = RetainedCloneProgress::default()", "WorkerJobAdmissionContext::new(operation, generation, StepBudget::new(1, u64::MAX, $grant), now_us, &mut recipient)", "$session::try_admit_owned(&mut job, &mut params, &mut control)", "Ok(None) => Err(refused(job, params, None))", "Err(error) => Err(refused(job, params, Some(error)))"];
+  return count(source, "pub(crate) fn admit_mounted<") === 1 && count(source, "macro_rules! admit_session") === 1
+    && admission?.body.replace(/\s+/gu, "") === "admit_session!(MountedWorkerJobSession,job,params,grant)"
+    && !!owner && custody.every(token => owner.body.includes(token))
+    && count(setup, "crate::reserved_job_session::admit_mounted(dispatch.job, params, self.mounted_policy.preparation)") === 1
+    && count(inner.body, direct) === 1 && stages.every((token) => count(setup, token) === 1)
     && offsets.every((offset, index) => offset >= 0 && (index === 0 || offset > offsets[index - 1]!))
-    && !["self.tool_jobs.dispatch", "MountedWorkerJobSession::try_new", "drive_worker_step("].some((token) => inner.body.includes(token))
-    && !["run_on_worker_async", "run_to_completion", "thread_local!", "A::handle(", "A::ephemeral("].some((token) => inner.body.includes(token) || setup.includes(token));
+    && !["self.tool_jobs.dispatch", "crate::reserved_job_session::admit_mounted", "MountedWorkerJobSession::try_new", "drive_worker_step("].some((token) => inner.body.includes(token))
+    && !setup.includes("MountedWorkerJobSession::try_new") && !["run_on_worker_async", "run_to_completion", "thread_local!", "A::handle(", "A::ephemeral("].some((token) => inner.body.includes(token) || setup.includes(token));
 }
 
@@ -172,75 +181,47 @@

  * ONE gesture stages N mutations into ONE `Edit` and ONE ledger slot, one fold per turn. */
 export function toolJobStoreBatchPublicationBounded(storeSource: string, pluginSource: string): boolean {
-  const blockOf = (source: string, needle: string, fromEnd = false) => {
-    const start = fromEnd ? source.lastIndexOf(needle) : source.indexOf(needle);
-    const open = start < 0 ? -1 : source.indexOf("{", start);
-    return open < 0 ? undefined : toolJobRustBlock(source, open);
-  };
-  const begin = blockOf(storeSource, "fn begin_apply_batch");
-  const ownedBegin = blockOf(storeSource, "fn begin_apply_batch_owned");
-  const preparation = begin?.body.includes("self.begin_apply_batch_owned(") && ownedBegin?.body.includes("source.footprint(lane)") ? ownedBegin : begin;
-  const advance = blockOf(storeSource, "fn advance_apply_batch");
-  const cancel = blockOf(storeSource, "fn cancel_apply_batch");
-  const fold = blockOf(storeSource, "fn fold_batch_item");
-  const base = blockOf(storeSource, "fn batch_item_base");
-  const publicationState = blockOf(storeSource, "pub struct ArtifactStoreBatchPublication<P, Mutation>");
-  const publication = blockOf(storeSource, "impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation>");
-  const close = publication ? blockOf(publication.body, "fn close_step") : undefined;
-  const terminal = publication ? blockOf(publication.body, "fn terminal_is_empty") : undefined;
-  const publisher = blockOf(pluginSource, "fn publish_mounted_typed_operation_unit", true);
-  const monolithic = ["replay_mutations(", "apply_command(", "flush_outbound(", "mutation.diff(", "diff.apply(", "try_reserve", "with_capacity(", ".collect::<Vec", ".to_vec("];
-  return (
-    storeSource.includes("pub struct ArtifactStoreBatchPublication") &&
-    storeSource.includes("pub enum ArtifactStoreOneItemPublicationPhase") &&
-    storeSource.includes("pub trait ArtifactStoreOneItemPreparationFactory") &&
-    storeSource.includes("pub trait ArtifactStoreOneItemPreparation") &&
-    storeSource.includes("pub struct ArtifactStoreOneItemPrepared") &&
-    storeSource.includes("trait ArtifactStoreBatchSource") &&
-    !!begin &&
-    !!advance &&
-    !!cancel &&
-    !!fold &&
-    !!base &&
-    !!publicationState &&
-    !!publication &&
-    !!close &&
-    !!terminal &&
-    !!preparation && preparation.body.includes("ArtifactStoreBatchPublication") &&
-    storeSource.includes("fn advance_apply_batch") &&
-    storeSource.includes("fn cancel_apply_batch") &&
-    publication.body.includes("fn begin_close") &&
-    close.body.includes("min(1)") &&
-    close.body.includes("maximum_bytes") &&
-    !close.body.includes(".clear()") &&
-    terminal.body.includes("ArtifactStoreOneItemPublicationPhase::Complete") &&
-    (terminal.body.includes(".is_empty()") || terminal.body.includes(".is_none()")) &&
-    publication.body.includes("maximum_items") &&
-    publication.body.includes("maximum_bytes") &&
-    /\b(?:expected_)?generation\b/.test(publicationState.body) &&
-    storeSource.includes("impl<P, Mutation> Drop for ArtifactStoreBatchPublication") &&
-    monolithic.every((needle) => !begin.body.includes(needle) && !preparation.body.includes(needle) && !advance.body.includes(needle)) &&
-    fold.body.includes("authority.validate_prepared(candidate)") &&
-    fold.body.includes("inverse.reverse();") &&
-    !fold.body.includes("replay_mutations(") &&
-    !fold.body.includes("apply_command(") &&
-    !advance.body.includes("while ") &&
-    !fold.body.includes("while ") &&
-    pluginSource.includes("pending_artifact_publication") &&
-    pluginSource.includes("artifact_one_item_factory: Option<") &&
-    pluginSource.includes("config_one_item_factory: Option<") &&
-    pluginSource.includes("draft_one_item_factory: Option<") &&
-    pluginSource.includes("unsupported_publication_contracts") &&
-    !!publisher &&
-    publisher.body.includes("begin_apply_batch") &&
-    publisher.body.includes("advance_apply_batch") &&
-    ["artifact_mutations", "config_mutations", "draft_mutations"].every((lane) => publisher.body.includes(`std::mem::take(&mut emit.${lane})`) && !publisher.body.includes(`${lane}.last().cloned()`)) &&
-    publisher.body.includes("self.artifact_one_item_factory.as_ref()") &&
-    publisher.body.includes("self.config_one_item_factory.as_ref()") &&
-    publisher.body.includes("self.draft_one_item_factory.as_ref()") &&
-    !publisher.body.includes("A::build_artifact_store_one_item_preparation_factory") &&
-    !publisher.body.includes("A::build_config_store_one_item_preparation_factory") &&
-    !publisher.body.includes("A::build_draft_store_one_item_preparation_factory") &&
-    !publisher.body.includes(".apply_one(")
-  );
+  const blockOf = (source: string, needle: string, last = false) => {
+    const start = last ? source.lastIndexOf(needle) : source.indexOf(needle);
+    return start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start));
+  };
+  const admission = blockOf(storeSource, "fn admit_apply_batch(");
+  const advance = blockOf(storeSource, "fn advance_apply_batch(");
+  const cancel = blockOf(storeSource, "fn cancel_apply_batch(");
+  const fold = blockOf(storeSource, "fn fold_batch_item(");
+  const base = blockOf(storeSource, "fn batch_item_base(");
+  const state = blockOf(storeSource, "pub struct ArtifactStoreBatchPublication<P, Mutation>");
+  const owner = blockOf(storeSource, "impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation>");
+  const close = owner && blockOf(owner.body, "fn close_step(");
+  const terminal = owner && blockOf(owner.body, "fn terminal_is_empty(");
+  const publisher = blockOf(pluginSource, "fn publish_mounted_typed_operation_unit(", true);
+  if (!admission || !advance || !cancel || !fold || !base || !state || !owner || !close || !terminal || !publisher) return false;
+  const compact = (text: string) => text.replace(/\s+/gu, "");
+  const admitted = compact(admission.body), published = compact(publisher.body);
+  const quoted = admitted.indexOf("ifgrant.maximum_items==0");
+  const original = admitted.indexOf("std::mem::take(mutations)");
+  const lanes = { store: "artifact_mutations", config_store: "config_mutations", draft_store: "draft_mutations" } as const;
+  return ["pub enum ArtifactStoreOneItemPublicationPhase", "pub trait ArtifactStoreOneItemPreparationFactory", "pub trait ArtifactStoreOneItemPreparation", "pub struct ArtifactStoreOneItemPrepared", "trait ArtifactStoreBatchSource", "impl<P, Mutation> Drop for ArtifactStoreBatchPublication"].every(token => storeSource.includes(token))
+    && quoted >= 0 && original > quoted
+    && ["grant.maximum_copy_bytes<copied_bytes", "grant.maximum_capacity_bytes<retained_capacity_bytes", "grant.maximum_depth==0", "transaction:transaction.take()", "RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0}"].every(token => admitted.includes(token))
+    && !admitted.includes("mutations.clone()") && !admitted.includes("replay_mutations(")
+    && state.body.includes("expected_generation: u64") && state.body.includes("source_census_finished: bool")
+    && ["grant.maximum_items == 0", "self.retirement_demands(grant.maximum_copy_bytes)?", "demand.copy_bytes > grant.maximum_copy_bytes", "demand.capacity_bytes > grant.maximum_capacity_bytes", "demand.release_bytes > grant.maximum_release_bytes", "grant.maximum_depth < demand.depth", "maximum_items: 1", "maximum_depth: grant.maximum_depth - 1", "admit_retained_clone_close(child"].every(token => close.body.includes(token))
+    && !close.body.includes(".clear()")
+    && terminal.body.includes("ArtifactStoreOneItemPublicationPhase::Complete")
+    && ["self.preparation.is_none()", "self.source.is_none()", "self.stage.is_none()", "self.authority.is_none()", "self.refused_base.is_none()", "self.refused_authority.is_none()", "self.refused_issuers.iter().all(Option::is_none)"].every(token => terminal.body.includes(token))
+    && cancel.body.includes("publication.begin_close()")
+    && ["replay_mutations(", "apply_command(", "mutation.diff(", "diff.apply(", ".collect::<Vec", ".to_vec("].every(token => !admission.body.includes(token) && !advance.body.includes(token) && !fold.body.includes(token))
+    && !advance.body.includes("try_reserve") && !advance.body.includes("with_capacity(")
+    && fold.body.includes("grant.maximum_capacity_bytes<capacity") && fold.body.includes("grant.maximum_copy_bytes<control")
+    && fold.body.includes("authority.validate_prepared(candidate)")
+    && fold.body.includes("let item=original.inverse.pop().unwrap();target.inverse.push_reserved(item)")
+    && fold.body.includes('cursor.post.take().expect("original prepared post root")')
+    && fold.body.includes("stage.post.replace(incoming)")
+    && !fold.body.includes("while ") && !advance.body.includes("while ")
+    && !publisher.body.includes(".await")
+    && Object.entries(lanes).every(([store, lane]) => published.includes(`self.${store}.admit_apply_batch(`) && published.includes(`&mutemit.${lane},HistoryLane::Document`) && !published.includes(`emit.${lane}.clone()`) && !published.includes(`emit.${lane}.last().cloned()`) && !published.includes(`emit.${lane}.clear()`))
+    && ["artifact", "config", "draft"].every(lane => publisher.body.includes(`self.${lane}_one_item_factory.as_ref()`) && !publisher.body.includes(`A::build_${lane}_store_one_item_preparation_factory`))
+    && publisher.body.includes("advance_apply_batch") && published.includes("mounted.publication_ownership_progress=Some((retained,ownership))")
+    && !publisher.body.includes(".apply_one(");
 }
 
@@ -259,7 +240,7 @@

     }
   };
-  const advances = blocks(storeSource, "pub fn advance_publish_one");
-  const begins = blocks(storeSource, "pub fn begin_publish_one");
-  const cancels = blocks(storeSource, "pub fn cancel_publish_one");
+  const advances = blocks(storeSource, "pub fn advance_publish_one(");
+  const begins = blocks(storeSource, "pub fn begin_publish_one(");
+  const cancels = blocks(storeSource, "pub fn cancel_publish_one(");
   const publication = storeSource.indexOf("impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation>");
   const publicationOpen = publication < 0 ? -1 : storeSource.indexOf("{", publication);
@@ -324,51 +305,41 @@

 export function toolJobTypedPersistentFoundation(source: string): boolean {
   const blockOf = (needle: string) => {
+    if (source.split(needle).length !== 2) return undefined;
     const start = source.indexOf(needle);
-    const open = start < 0 ? -1 : source.indexOf("{", start);
-    return open < 0 ? undefined : toolJobRustBlock(source, open);
-  };
-  const dispatchStart = source.lastIndexOf("async fn dispatch_typed_command_inner");
-  const dispatchOpen = dispatchStart < 0 ? -1 : source.indexOf("{", dispatchStart);
-  const dispatch = dispatchOpen < 0 ? undefined : toolJobRustBlock(source, dispatchOpen);
+    return toolJobRustBlock(source, source.indexOf("{", start))?.body;
+  };
+  const dispatch = blockOf("async fn dispatch_typed_command_inner(");
   const active = blockOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>");
-  const maintenanceStart = source.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp");
-  const maintenanceName = source.indexOf("fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize)", maintenanceStart);
-  const maintenanceOpen = maintenanceName < 0 ? -1 : source.indexOf("{", maintenanceName);
-  const maintenance = maintenanceOpen < 0 ? undefined : toolJobRustBlock(source, maintenanceOpen);
+  const hosted = blockOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp");
+  const maintenanceMarker = "fn maintenance_step(&mut self, grant: RetainedCloneGrant)";
+  const maintenanceStart = hosted?.indexOf(maintenanceMarker) ?? -1;
+  const maintenance = hosted && hosted.split(maintenanceMarker).length === 2 && maintenanceStart >= 0 ? toolJobRustBlock(hosted, hosted.indexOf("{", maintenanceStart))?.body : undefined;
+  const ladder = blockOf("fn maintenance_ladder_step(&mut self, grant: RetainedCloneGrant)");
+  const unit = blockOf("fn maintenance_ladder_unit_step(&mut self, grant: RetainedCloneGrant)");
+  const selection = blockOf("fn maintenance_tool_selection(&self)");
+  const stage = blockOf("fn maintenance_stage_step(&mut self, stage: u8, grant: RetainedCloneGrant)");
+  const retirement = blockOf("fn retire_typed_operation_unit(");
   const setup = toolJobRetainedDispatchSetup(source);
-  if (!dispatch || !active || !maintenance || !setup) return false;
-  const guard = dispatch.body.indexOf("require_complete_tool_operation_pipeline(&admission)?");
-  const preparation = setup.indexOf("refresh_cache().await");
-  const preAdmission = dispatch.body.indexOf("!self.can_admit_typed_operation(operation_id.0)");
-  const session = setup.indexOf("MountedWorkerJobSession::try_new");
-  const retained = setup.indexOf("self.tool_operations.insert_admitted(");
-  return (
-    guard >= 0 &&
-    preparation >= 0 &&
-    preAdmission >= 0 &&
-    preAdmission > guard &&
-    session > preparation &&
-    retained > session &&
-    setup.includes("active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?") &&
-    dispatch.body.includes('DslValue::String(operation_id.0.to_string())') &&
-    !setup.includes("session.step(&pool") &&
-    !setup.includes("let outcome = loop") &&
-    source.includes("tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>") &&
-    active.body.includes("session.pump_one(pool, semio_framework_async::Lane::Interactive)") &&
-    active.body.includes("session.take_checked_out_outcome()") &&
-    active.body.includes("MountedTypedCommandFullOperationStage::AwaitingAck") &&
-    active.body.includes('Ok(_) => Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits its next outcome" })') &&
-    active.body.includes("WorkerJobSubmitFault::Contention(_)") &&
-    active.body.includes("WorkerSubmitErrorKind::Saturated") &&
-    active.body.includes("WorkerJobTakeFault::Pending") &&
-    active.body.includes('Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits transient scheduler authority" })') &&
-    maintenance.body.includes("for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS") &&
-    maintenance.body.includes("self.tool_operations.entry(index)") &&
-    maintenance.body.includes("every_operation_awaits_presented_ack") &&
-    maintenance.body.includes("PluginCloseStep::AwaitingInput") &&
-    maintenance.body.includes(".drive_worker_step(&pool, maximum_bytes)") &&
-    maintenance.body.includes("operation.retirement_step(maximum_items.min(1), maximum_bytes)?") &&
-    active.body.includes("fn terminal_is_empty") &&
-    source.includes("retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document")
-  );
-}
+  if (!dispatch || !active || !maintenance || !ladder || !unit || !selection || !stage || !retirement || !setup) return false;
+  const compact = (body: string) => body.replace(/\s+/gu, "");
+  const ordered = (body: string, tokens: readonly string[]) => {
+    let previous = -1;
+    return tokens.every(token => { const index = body.indexOf(token); const valid = index > previous; previous = index; return valid; });
+  };
+  return toolJobMountedDispatchOneTurnExact(source)
+    && ordered(dispatch, ["require_complete_tool_operation_pipeline(&admission)?", "self.admit_typed_operation_slot()"])
+    && ordered(setup, ["self.capture_typed_command_roots(command.as_ref(), meta).await?", "crate::reserved_job_session::admit_mounted(dispatch.job, params, self.mounted_policy.preparation)", "self.tool_operations.insert_admitted(", "active.drive_worker_step(self.mounted_policy.maintenance)"])
+    && dispatch.includes('DslValue::String(operation_id.0.to_string())')
+    && !setup.includes("session.step(") && !setup.includes("let outcome = loop")
+    && source.includes("tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>")
+    && compact(maintenance) === "self.maintenance_ladder_step(grant)"
+    && ["session.pump_one_for_interactive_turn(grant)", "session.checked_out_outcome()", "session.acknowledge_checked_out_outcome(grant)", "session.take_checked_out_retained_step_receipt()", "progress.fits(issued)", "progress.fits(grant)", "issued.maximum_items>grant.maximum_items", "issued.maximum_copy_bytes>grant.maximum_copy_bytes", "issued.maximum_capacity_bytes>grant.maximum_capacity_bytes", "issued.maximum_release_bytes>grant.maximum_release_bytes", "issued.maximum_depth>grant.maximum_depth", "fn terminal_is_empty"].every(token => active.includes(token))
+    && active.includes('Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits its next outcome"})')
+    && ["WorkerJobSubmitFault::Contention(_)", "WorkerSubmitErrorKind::Contended|semio_framework_async::WorkerSubmitErrorKind::Saturated", "WorkerJobTakeFault::Pending", 'PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits transient scheduler authority"}'].every(token => active.includes(token))
+    && ["for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS", "self.tool_operations.entry(index)", "every_operation_awaits_presented_ack", "operation.result_page_presented", "(self.maintenance_tool_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS"].every(token => selection.includes(token))
+    && ["self.maintenance_tool_selection()", "self.maintenance_tool_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS", "operation.drive_worker_step(grant)", "MaintenanceTool::Awaiting => Ok(PluginLifecycleStep::AwaitingInput"].every(token => stage.includes(token))
+    && ["remaining_plugin_turn_grant(grant, spent)", "spent.checked_add(unit)", "INTERACTIVE_TURN_WORKER_WALL_US"].every(token => ladder.includes(token))
+    && ordered(unit, ["for position in 0..MAINTENANCE_POSITIONS", "self.maintenance_unit_demand(unit, grant.maximum_copy_bytes)", "plugin_turn_admitted(demand, grant)?", "self.maintenance_unit_step(unit, grant)?"])
+    && unit.includes("awaiting.or(blocked)")
+    && ordered(retirement, ["operation.retirement_step(grant)?", "MountedTypedCommandFullOperation::terminal_is_empty", "self.tool_operations.remove(operation_id)"]);
+}

### 55 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts
HEAD 644 BASE 645 STASH 645 CURRENT H
--- 

+++ 

@@ -271,5 +271,5 @@

   const activation = activationOpen < 0 ? "" : toolJobRustBlock(source, activationOpen)?.body ?? "";
   const registered = activation.indexOf("A::register_tool_job_factories(&mut app_tool_registry)");
-  const joined = activation.indexOf(".tool_job_registration::<A>");
+  const joined = activation.indexOf("crate::reserved_jobs::tool_job_registration::<A>");
   return (
     source.includes("pub struct ArtifactBoundedFirstStepProof") &&
@@ -293,5 +293,5 @@

     source.includes("proof.with_factory_type::<$owner, $factory_type>()") &&
     source.includes("seen != expected") &&
-    /tool_job_registration\s*::<A>\s*\(\s*&app_id,\s*A::DOCUMENT_SCHEMA,/.test(source) &&
+    /tool_job_registration\s*::<A>\s*\(\s*&registry,\s*&app_id,\s*A::DOCUMENT_SCHEMA,/.test(activation) &&
     source.includes("fn qualified_tool_proof(&self, verb: &str)") &&
     source.includes("owner: ToolOwnerWitness") &&

### 56 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts
HEAD 289 BASE 1500 STASH 1156 CURRENT H
--- 

+++ 

@@ -1,2 +1,6 @@

+import { WORKSPACE_ROOT } from "../../../🗂️workspaces/📍️checkout-root/🟦️.ts";
+import { inspectRustModuleGraph, inspectRustModuleGraphFacts } from "../../../🔍️discovery/🟦️.ts";
+import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
+import { policyAllRustFiles } from "../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
 /** 🗄️ Tool-job coverage: artifact store, envelope and retained-ownership laws per app.
  * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
@@ -5,5 +9,5 @@

 import { join } from "node:path";
 import { OS_STORE_ROOT_SOURCES, policyRustProductionSource, type PolicyRustSourceEvidence, policyRustTestSource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
-import { TOOL_JOB_CHILD_RETIREMENT_INVENTORY, toolJobProductionSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";
+import { TOOL_JOB_CHILD_RETIREMENT_INVENTORY, toolJobProductionProtocolSource, toolJobProductionSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";
 import { toolJobMountedDispatchOneTurnExact, toolJobRetainedDispatchSetup, toolJobTypedCaptureBody } from "../📤️publication/🟦️.ts";
 
@@ -620,353 +624,149 @@

 }
 
-export function toolJobRasterEnvelopeCallerRetainedExact(store: string, raster: string, editor: string, plugin: string): boolean {
-  const variants = [
-    "CreateLayer(payload)",
-    "DeleteLayer(payload)",
-    "ReorderLayers(payload)",
-    "RenameLayer(payload)",
-    "ChangeLayerVisible(payload)",
-    "ChangeLayerOpacity(payload)",
-    "ChangeLayerBlendMode(payload)",
-    "MoveLayer(payload)",
-    "ResizeLayer(payload)",
-    "ChangeLayerAdjustmentKind(payload)",
-    "AddLayerAsset(payload)",
-    "RemoveLayerAsset(payload)",
+
+type RasterEnvelopeSourceEvidence = Readonly<{ store: string; raster: string; editor: string; plugin: string; laws: string; productionPaths: readonly string[]; testPaths: readonly string[] }>;
+
+/** 🖨️ Reads the current declared Raster owners and their mounted tests as distinct source evidence. */
+export function toolJobRasterEnvelopeSourceEvidence(root: string = WORKSPACE_ROOT): RasterEnvelopeSourceEvidence {
+  const raster = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster", subset = raster + "/🏅️standards/🔖️1/🪆️subsets/✳️any";
+  const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin", value = "🧰️framework/🔨️modules/🌱️value";
+  const domainPaths = [raster + "/🦀️.rs", raster + "/🔨️modules/🏠️host/🧰️owned/🦀️.rs", subset + "/🧬️schema/📸️snapshot/🦀️.rs", subset + "/🧬️schema/🧬️mutations/🦀️.rs", subset + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs", subset + "/🚪️io/🦀️.rs", ...["🔣️json/🔖️rfc8259/✳️any", "📷️png/🔖️1.2/✳️any", "🪟️bmp/🔖️v3/✳️any", "🖼️tiff/🔖️6.0/✳️any", "📸️jpg/🔖️jfif-1.01/♾️any", "🎞️gif/🔖️87a/✳️any", "🎨️svg/🔖️1.1/✳️any"].map(path => subset + "/🚪️io/📤️export/🧵️serializers/🗿️artifacts/" + path + "/🦀️.rs")];
+  const editorPaths = [subset + "/✏️editor/🦀️.rs", subset + "/✏️editor/📤️export/🦀️.rs"];
+  const testPaths = [raster + "/🔨️modules/🏠️host/🧰️owned/🧪️tests/🦀️.rs", subset + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", subset + "/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", subset + "/✏️editor/📤️export/🧪️tests/🦀️.rs"];
+  const retainedMutation = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained";
+  const sharedPaths = [store + "/📦️codec/🧬️owned-schema/🦀️.rs", store + "/📬️paged-one-item/🦀️.rs", store + "/📬️paged-one-item/🧬️mutation-apply/🦀️.rs", store + "/🏗️initialization/🔁️replay/🦀️.rs", retainedMutation + "/🦀️.rs", retainedMutation + "/🔺️diff/🦀️.rs", value + "/🧬️retained-clone/🧺️std-collections/🦀️.rs", value + "/♻️retirement/🦀️.rs", value + "/♻️retirement/🔗️shared/🦀️.rs", value + "/♻️retirement/🎮️controlled/🦀️.rs"];
+  const pluginPaths = [plugin + "/🦀️.rs", plugin + "/🏛️ownership/🦀️.rs", plugin + "/🧵️instance/🦀️.rs", plugin + "/🌱️initialization/♻️retirement/🦀️.rs", plugin + "/👷️job/🏛️reserved/🦀️.rs", plugin + "/♻️lifecycle/🦀️.rs", plugin + "/🗃️document/🦀️.rs", plugin + "/📜️declaration/🧩️app/🦀️.rs", plugin + "/📜️declaration/🪟️surface/🦀️.rs", plugin + "/👷️job/🚪️session/🦀️.rs"];
+  const files = [...policyAllRustFiles(root).filter(path => path.startsWith(raster + "/")), raster + "/📦️packages/🦀️rust/Cargo.toml"];
+  const graph = inspectRustModuleGraph(files, path => policyReadFileSafe(root, path) || undefined);
+  for (const path of [...domainPaths, ...editorPaths, ...testPaths]) if (!graph.contexts.get(path)?.some(context => context.crateRoot === raster + "/🦀️.rs")) throw new Error("[raster evidence] current owner is not mounted: " + path);
+  const read = (path: string) => { const source = policyReadFileSafe(root, path); if (!source) throw new Error("[raster evidence] source absent: " + path); return source; };
+  const production = (paths: readonly string[]) => paths.map(path => toolJobProductionProtocolSource(read(path))).join("\n");
+  const host = read(domainPaths[1]!);
+  const declaredHostTests = inspectRustModuleGraphFacts(host).modules.some(module => module.conditional && module.pathTarget === "🧪️tests/🦀️.rs");
+  const mounts = declaredHostTests ? "mounted-raster-host-laws" : "";
+  return { store: production(sharedPaths), raster: production(domainPaths), editor: production(editorPaths), plugin: production(pluginPaths), laws: mounts + "\n" + testPaths.map(read).join("\n"), productionPaths: [...domainPaths, ...editorPaths, ...sharedPaths, ...pluginPaths], testPaths };
+}
+
+/** 🧾️ Current Raster obligations follow original typed owners, quoted currencies and the single shared apply/seal path. */
+export function toolJobRasterEnvelopeCallerFailures(store: string, raster: string, editor: string, plugin: string, laws: string): string[] {
+  const failures: string[] = [], require = (id: string, condition: boolean) => { if (!condition) failures.push(id); }, compact = (source: string) => source.replaceAll(/\s+/g, "");
+  const body = (source: string, anchor: string) => { const start = source.indexOf(anchor); return start < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", start))?.body ?? ""; };
+  const preparationStart = editor.search(/fn\s+build_artifact_store_one_item_preparation_factory\s*\(/);
+  const preparation = preparationStart < 0 ? "" : toolJobRustBlock(editor, editor.indexOf("{", preparationStart))?.body ?? "";
+  require("central-factory", compact(preparation) === "Some(store::mutation_apply_preparation_factory::<RasterSnapshot,RasterMutation>())");
+  const forbidden = ["struct RasterStoreInitializationAuthority", "struct RasterMutationCandidateAuthority", "struct RasterOneItemApply", "struct RasterSnapshotCloneAuthority", "struct RasterLayerCloneAuthority", "struct RasterLayerLocator", "struct RasterStorePreparationFactory", "fn build_document_store_initialization_job", "raster_document_store_initialization_job", "fn node_at_mut", "fn container_mut", "snapshot_mut", "operation.diff(", "diff.apply("];
+  require("single-apply-owner", forbidden.every(anchor => !raster.includes(anchor) && !editor.includes(anchor)) && !editor.includes("apply_diff_cold(") && !editor.includes("apply_diff_into("));
+  require("original-field-catalog", ["pub struct RasterEnvelopeOwnedFieldCatalog", "artifact_owned_spr_edit_history_decoder", "RasterSnapshotDecodeAuthority", "RasterMutationDecodeAuthority", "OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>", "pub fn raster_envelope_decode_owner_bundle()", "pub fn raster_document_store_owners()"].every(anchor => raster.includes(anchor)));
+  require("shared-history-decoder", store.includes("pub fn artifact_owned_spr_edit_history_decoder") && store.includes("struct ArtifactOwnedSprMutationArrayAuthority") && !raster.includes("artifact_bounded_history_entry_decoder"));
+  require("owned-retirement-issuers", raster.includes("pub struct RasterSnapshotRetirementFactory") && raster.includes("pub struct RasterMutationRetirementFactory") && (raster.match(/retirement::admit_owned_retirement\(value, grant\)/g) ?? []).length === 2);
+  require("shared-retirement-issuer", raster.includes("retirement::shared::admit_shared_retirement(snapshot, grant, true)"));
+  const ownedBirth = compact(body(store, "pub fn admit_typed_controlled_retirement<")), sharedBirth = compact(body(store, "pub fn admit_shared_retirement<"));
+  require("shared-owned-birth", ownedBirth.includes("grant.maximum_items==0") && ownedBirth.includes("grant.maximum_depth==0") && ownedBirth.includes("layout.size()>grant.maximum_capacity_bytes") && ownedBirth.indexOf("layout.size()>grant.maximum_capacity_bytes") < ownedBirth.indexOf("std::alloc::alloc(layout)") && ownedBirth.includes("retained_capacity_bytes:layout.size()") && ownedBirth.includes("value));"));
+  require("shared-arc-birth", sharedBirth.includes("grant.maximum_items==0") && sharedBirth.includes("grant.maximum_depth==0") && sharedBirth.includes("layout.size()>grant.maximum_capacity_bytes") && sharedBirth.indexOf("layout.size()>grant.maximum_capacity_bytes") < sharedBirth.indexOf("std::alloc::alloc(layout)") && sharedBirth.includes("retained_capacity_bytes:layout.size()") && sharedBirth.includes("source));"));
+  require("shared-physical-retirement", store.includes("controlled retirement abandoned before terminal-empty") && store.includes("shared retirement abandoned actual Arc allocation ownership") && compact(store).includes("Arc::try_unwrap(source)") && compact(store).includes("Arc::weak_count(source)==0") && compact(store).includes("grant.maximum_release_bytes<extent"));
+
+  const quotedBirth = (owner: string, source: string, message: string) => owner.includes("iflayout.size()>grant.maximum_capacity_bytes{returnErr((ValueError::literal(ValueRefusalKind::OwnershipLimit,\"" + compact(message) + "\")," + source + "));}") && owner.includes("copied_items:1") && !owner.includes("expect(") && !owner.includes("unwrap(") && !owner.includes("panic!(");
+  require("exact-controlled-producer-refusal", quotedBirth(ownedBirth, "value", "typed retirement frame exceeds admitted capacity"));
+  require("exact-shared-producer-refusal", quotedBirth(sharedBirth, "source", "shared frame exceeds admitted capacity"));
+  const controlledOwner = body(store, "impl<T: RetireOwned> ControlledRetirement<T>"), controlledDemand = compact(body(controlledOwner, "pub fn next_demand("));
+  const controlled = compact(body(controlledOwner, "fn step_original(&mut self, grant: RetainedCloneGrant)"));
+  const controlledAdmission = ["letdemand=self.next_demand(grant.maximum_copy_bytes)?;", "grant.maximum_depth<demand.depth", "grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes", "self.cursors.release_empty_page(grant.maximum_release_bytes)"].map(anchor => controlled.indexOf(anchor));
+  require("controlled-body-quote", controlledAdmission.every((position, index) => position >= 0 && (index === 0 || position > controlledAdmission[index - 1]!)) && (controlled.match(/self\.next_demand\(grant\.maximum_copy_bytes\)/g) ?? []).length === 1 && controlledDemand.includes("letmutd=cursor.next_demand(body)?;") && controlled.includes("letbody_bytes=grant.maximum_copy_bytes;") && controlled.includes("letminimum_work_bytes=demand.copy_bytes;") && controlled.includes("letbirth=demand.capacity_bytes;"));
+  require("controlled-turn-currencies", controlled.includes("grant.maximum_items==0") && controlled.includes("grant.maximum_depth<demand.depth") && controlled.includes("grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes") && controlled.includes("body_bytes<minimum_work_bytes") && controlled.includes("maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:grant.maximum_depth-1") && controlled.includes("progress.fits(cursor_grant)") && controlled.includes("self.cursors.release_empty_page(grant.maximum_release_bytes)"));
+  require("controlled-frame-physical-return", controlled.includes("cursor.terminal_release_bytes()") && controlled.includes("ifbytes>grant.maximum_release_bytes") && controlled.includes("released_bytes:bytes") && controlledDemand.includes("d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,\"retirementfrontierdepthoverflow\"))?;"));
+  const bytes = compact(body(store, "impl RetirementCursor for Bytes"));
+  require("original-string-byte-capacity", compact(body(store, "impl RetireOwned for String")).includes("bytes(self.into_bytes())") && bytes.includes("letbytes=self.0.capacity()") && bytes.includes("ifbytes>grant.maximum_release_bytes") && bytes.includes("RetirementStep::Bytes(bytes)") && bytes.includes("Some(ifself.0.is_empty(){self.0.capacity()}else{0})") && store.includes("owned bytes retired before terminal-empty"));
+  const shared = compact(body(store, "impl<T:RetireOwned+Sync> SharedControlledRetirement<T>"));
+  require("exact-shared-owner-return", shared.includes("Arc::strong_count(source)!=1") && shared.includes("Arc::weak_count(source)==0") && shared.includes("Err(source)=>{*self.source=Some(source)") && shared.includes("released_bytes:extent") && shared.includes("self.source.is_none()&&self.owned.is_none()"));
+
+  const collection = compact(body(store, "impl<T: RetireOwned> RetirementCursor for Collection<T>"));
+  require("original-vector-backing", collection.includes("self.0.capacity()*size_of::<T>()") && collection.includes("ifbytes>maximum_bytes") && collection.includes("self.0.is_empty()&&(self.0.capacity()==0||size_of::<T>()==0)") && collection.includes("RetirementStep::Child(value.retirement())") && collection.includes("grant.maximum_capacity_bytes<birth") && store.includes("owned collection retired before terminal-empty"));
+  const registry = compact(body(plugin, "impl<T> ArtifactFixedRegistry<T>"));
+  const registryRelease = compact(body(plugin, "pub(crate) fn close_empty_backing_granted("));
+  require("fixed-registry-retains-physical-owner", registry.includes("slots.try_reserve_exact(capacity).is_ok()") && registry.includes("self.allocation_admitted&&self.entry(slot).is_none()") && registry.includes("self.entry(index).is_none_or(|(candidate,_)|*candidate!=id)") && registry.includes("self.occupied&=!(1u64<<index)") && registry.includes("self.slots[index].assume_init_read().1") && registry.includes("size_of_val(self.slots.as_ref())") && registryRelease.includes("grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0") && registryRelease.includes("released_bytes:bytes") && plugin.includes("fixed artifact owner registry reached Drop before every exact owner was transferred to bounded retirement"));
+  require("fixed-registry-saturated-original-refusal", compact(plugin).includes("!self.jobs.can_insert(self.operation.0){returnErr(envelope);") && compact(plugin).indexOf("!self.jobs.can_insert(self.operation.0)") < compact(plugin).indexOf("letjob=A::build_document_store_initialization_job(envelope,self.operation,self.generation,self.actor.clone())?"));
+  const fields = compact(body(raster, "fn accept_token(")), close = compact(body(raster, "fn close_step(&mut self, grant: RetainedCloneGrant)"));
+  require("field-identity", fields.includes("cx.operation()!=self.operation||cx.generation()!=self.generation"));
+  require("field-cancellation", fields.includes("ifcx.is_cancelled()"));
+  require("field-deadline", fields.includes("ifcx.should_yield()"));
+  require("field-close-admission", close.includes("grant.maximum_items==0") && close.includes("grant.maximum_depth<demand.depth") && close.includes("raster_short(grant,demand)") && compact(raster).includes("grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes"));
+  require("field-original-close", close.includes("artifact_retirement_box_close_step(&mutself.retirement,grant)") && close.includes("artifact_retirement_admit_owned(&mutself.value,&mutself.retirement,grant)") && raster.includes("decode reached Drop before publication or bounded retirement"));
+  require("fixed-map-backing", compact(raster).includes("pubconstRASTER_OWNED_MAP_CAPACITY:usize=64;") && compact(raster).includes("pubconstRASTER_OWNED_MAP_PAGE_CAPACITY:usize=8;") && compact(raster).includes("pages:std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>;RASTER_OWNED_MAP_PAGE_COUNT]>"));
+  const mapInsert = compact(body(raster, "pub(crate) fn insert_pre_admitted("));
+  require("map-cap-refusal", mapInsert.includes("self.length>=RASTER_OWNED_MAP_CAPACITY") && mapInsert.includes("RasterOwnedMapRejected{key,value,reason:\"raster-map.item-capacity\"}") && mapInsert.includes("std::mem::replace(entry,(key,value))") && mapInsert.includes("RasterOwnedMapEntry::new(previous_key,previous_value)"));
+  require("map-key-handback", raster.includes("pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>") && !raster.includes("pub fn remove(&mut self, key: &str)"));
+  require("map-terminal-drop", compact(body(raster, "impl<V> Drop for RasterOwnedMap<V>")).includes("self.length==0&&self.pages.iter().all(Option::is_none)"));
+  const mapRetirement = compact(body(raster, "impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for RasterOwnedMapRetirement<V>"));
+  require("map-owner-retirement", mapRetirement.includes("grant.maximum_items==0||grant.maximum_depth==0") && mapRetirement.includes("RetirementStep::Child(entry.retirement())") && mapRetirement.includes("bytes>grant.maximum_release_bytes") && mapRetirement.includes("take_empty_page_backing()") && mapRetirement.includes("page.release();") && mapRetirement.includes("self.map.length==0&&self.map.allocated_page_count()==0"));
+  require("map-page-physical-size", compact(raster).includes("size_of::<RasterOwnedMapPage<V>>()") && compact(raster).includes("Some(size_of::<Self>())") && raster.includes("Raster owned map retirement abandoned before terminal-empty"));
+  const mapQuote = compact(body(raster, "fn insertion_demands(building:"));
+  require("map-clone-admission", raster.includes("RetainedCloneEntriesCursor<Self>") && mapQuote.includes("map.page_required_for_insert(key)") && mapQuote.includes("size_of::<RasterOwnedMapPage<V>>()") && mapQuote.includes("copy_bytes:size_of::<(String,V)>()+RASTER_OWNED_MAP_CAPACITY+capacity") && mapQuote.includes("capacity_bytes:capacity") && compact(raster).includes("self.insert(key,value).map_err(|rejected|(rejected.key,rejected.value))"));
+  const insertAt = store.indexOf("            INSERT => {"), insertEnd = store.indexOf("            DONE =>", insertAt), insert = compact(insertAt < 0 ? "" : store.slice(insertAt, insertEnd));
+  const admission = insert.indexOf("grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth");
+  require("shared-map-original-quote", insert.includes("M::insertion_demands(self.building.as_ref(),self.key.as_ref()") && admission >= 0 && insert.indexOf("self.key.take()") > admission && insert.indexOf("self.value.take()") > admission && insert.includes("retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes") && compact(store).includes("INSERT=>returnOk(M::insertion_demands(self.building.as_ref(),self.key.as_ref()"));
+  require("typed-mutation-retirement", /derive\([^\]]*RetireOwned[^\]]*\)\][\s\S]{0,400}pub enum RasterMutation/.test(raster) && raster.includes("RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset)") && raster.includes("params: RasterOwnedMap<semio_framework_value::DslValue>"));
+  require("typed-clone", /derive\([^\]]*RetainedClone[^\]]*\)\][\s\S]{0,200}pub struct RasterSnapshot/.test(raster) && /derive\([^\]]*RetainedClone[^\]]*\)\][\s\S]{0,200}pub enum RasterLayerNode/.test(raster));
+  require("paged-original-birth", store.includes("if !grant.permits_one() || self.cancelled || self.closing") && store.includes("base.admit_retained_clone_source(clone_grant)") && store.includes("source.try_borrow()?") && store.includes("admit_retained_clone_progress") && store.includes("self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)"));
+  const central = compact(store), commit = compact(body(store, "pub fn commit_retained_diff<")), inverse = compact(body(store, "pub fn inverse_retained_mutation<"));
+  const preflightPhase = compact(body(store, "Stage::Preflight=>")), inversePhase = compact(body(store, "Stage::Inverse=>")), demand = compact(body(store, "pub fn retained_mutation_demand_fits("));
+  require("single-central-mutation", preflightPhase.includes("RetainedDiffPreflightStep::Admitted(_,rows)=>{self.messages[1]=Some(rows);self.stage=Stage::Inverse;}") && inversePhase.includes("inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)") && inversePhase.includes("RetainedMutationInverseStep::Ready(_,rows)=result{*self.inverse=Some(rows);self.stage=Stage::Commit;}") && central.includes("preflight_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,g)") && central.includes("inverse_retained_mutation(self.semantics.as_mut().unwrap(),mutation,&a,g)") && central.includes("commit_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,g)") && ["g.maximum_items>0", "g.maximum_copy_bytes>=d.copy_bytes", "g.maximum_capacity_bytes>=d.capacity_bytes", "g.maximum_release_bytes>=d.release_bytes", "g.maximum_depth>=d.depth"].every(axis => demand.includes(axis)) && inverse.includes("if!retained_mutation_demand_fits(d,grant)") && commit.includes("if!retained_mutation_demand_fits(demand,grant)") && commit.includes("diff.bind(&mutowner.diff,grant)?.is_some()") && commit.includes("original.bind(&mutowner.original,grant)?.is_some()") && commit.includes("super::super::mint_capability()") && commit.includes("admit_retained_clone_progress(grant,receipt"));
+  require("funded-sealer", store.includes("retained.maximum_capacity_bytes < wrapped") && store.includes("retained.maximum_copy_bytes < ArtifactStoreOneItemSealer::<S, M>::constructor_copy_byte_demand()") && store.includes("authority.begin_one_item_seal(edit, Arc::new(post), mutation_retirement, snapshot_retirement, retained)"));
+  require("default-replay-owner", plugin.includes("Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))") && compact(plugin).includes("store::ArtifactStoreReplayInitializer::new(envelope,schema,operation,generation,actor)"));
+  const replay = compact(body(store, "fn turn(&mut self, cx: &mut StepContext<'_>) -> Turn"));
+  require("replay-entry-denial", replay.includes("cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0") && replay.indexOf("cx.should_yield()") < replay.indexOf("self.settle(cx)") && replay.includes("letunclaimed=self.phase==Phase::Complete&&!self.terminal_handoff&&self.candidate.is_some()"));
+  require("replay-candidate-cancellation", compact(store).includes("self.phase!=Phase::Complete||self.terminal_handoff||self.cancel_requested") && store.includes("self.generation.0.checked_add(1)"));
+  require("retained-replacement-session", compact(plugin).includes("session:ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P,Mutation>>>>") && compact(plugin).includes("session_rejected:ManuallyDrop<Option<crate::reserved_job_session::RefusedWorkerSource<ArtifactStoreInitializationJob<P,Mutation>>>>") && compact(plugin).includes("crate::reserved_job_session::admit_mounted(job,params,retained)") && compact(plugin).includes("self.jobs.insert_admitted(self.operation.0,ActiveArtifactStoreReplacement::new(self.operation,self.generation,self.child_content_generation,job,self.retained))") && plugin.includes("artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty") && plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") && plugin.includes("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close"));
+  const credit = laws.indexOf("begin_artifact_envelope_ingress(pages, "), page = laws.indexOf("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]"), construct = laws.indexOf("ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())"), admitted = laws.indexOf("admit_artifact_envelope_ingress_page(handle, page)");
+  require("ingress-order", credit >= 0 && page > credit && construct > page && admitted > construct && laws.includes("chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") && laws.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle"));
+  require("ingress-owned-pump", laws.includes("seal_artifact_envelope_ingress(handle)") && laws.includes("fixture_mounted_policy().maintenance") && laws.includes("advance_artifact_envelope_load(handle)") && laws.includes("assert_eq!(handle.generation, base_generation)") && laws.includes("duplicate Raster load acknowledgement is a no-op") && laws.includes("cancel_artifact_envelope_load(handle)"));
+  require("ingress-no-whole-bypass", !editor.includes("envelope_json: &str") && !editor.includes("reject_whole_buffer_artifact_envelope_ingress") && !editor.includes("ArtifactStore::new"));
+  require("editor-owned-decode", compact(body(editor, "fn build_envelope_decode_owner_bundle()")) === "Some(crate::host::owned::raster_envelope_decode_owner_bundle())");
+  require("bounded-mounted-output", editor.includes("if context.should_yield(){return Ok(None);}") && editor.includes("context.is_cancelled()") && editor.includes("bytes.len()>BASE64_INPUT_BYTES") && editor.includes("credit.credit(chunk.len())?") && editor.includes("self.chunks.as_ref().unwrap().push(chunk)?") && editor.includes("self.work.preparing.take()") && editor.includes("self.work.encoding.take()") && editor.includes("raster export snapshot has no live close witness"));
+  require("output-domain-preflight", raster.includes("if depth>32") && raster.includes("self.nodes>1024") && raster.includes("validate_extent(image.width,image.height)") && raster.includes("frame.rgba8.len()!=count*4") && editor.includes("raster_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1)"));
+  require("serializer-real-output", (raster.match(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\)/g) ?? []).length === 7 && raster.includes("write_json_pretty(&serialize(snapshot)?.value)") && raster.includes("let image = raster_composite_image(snapshot)") && raster.includes("raster_document_json_to_svg(snapshot)?") && !raster.includes("snapshot.require_empty_output_shell()"));
+  const names = ["raster_generic_initializer_retains_genesis_identity_next_generation_and_second_map_page", "raster_empty_generic_initializer_progresses_under_mounted_sixty_four_fuel", "raster_generic_initializer_zero_fuel_items_and_expired_deadline_preserve_phase_and_owners", "raster_generic_initializer_cancel_stale_and_completed_candidate_retire_all_exact_owners", "raster_mounted_preparation_deep_atomic_inverse_and_unclaimed_result_retirement", "raster_fixed_map_clone_prices_exact_page_births_and_denies_unfunded_insertions", "raster_mounted_partial_clone_and_before_prepared_cancellation_retain_source_and_retire_owners", "raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed", "raster_live_envelope_cancel_closes_retained_pages_without_publication", "a_demo_edit_archive_loads_back_through_the_document_archive_door", "raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant", "raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant", "raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner", "raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses", "raster_maximum_combined_layer_and_value_depth_retires_to_terminal", "raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly", "raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact"];
+  require("mounted-laws", laws.includes("mounted-raster-host-laws") && names.every(name => new RegExp("\\b(?:async\\s+)?fn\\s+" + name + "\\s*\\(").test(laws)));
+
+  const mapBody = body(raster, "pub struct RasterOwnedMap<V>");
+  require("private-map-backing", !/\bpub(?:\([^)]*\))?\s+(?:pages|order|length)\s*:/.test(mapBody));
+  const serializerBodies = Array.from(raster.matchAll(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\)/g), match => compact(toolJobRustBlock(raster, raster.indexOf("{", match.index))?.body ?? ""));
+  require("each-mounted-format", serializerBodies.length === 7 && ["encode_png(&target)", "encode_bmp(&target)", "encode_tiff(&target)", "encode_jpg(&target,", "encode_gif(&gif87a::from_89a(&gif89a))", "write_json_pretty(&serialize(snapshot)?.value)", "raster_document_json_to_svg(snapshot)?"].every(call => serializerBodies.some(owner => owner.includes(call))) && serializerBodies.filter(owner => owner.includes("letimage=raster_composite_image(snapshot)")).length === 5 && !serializerBodies.some(owner => owner.includes("Ok(Vec::new())") || owner.includes("print_dsl(")));
+  require("exact-replaced-fixture", /assert_eq!\(previous_key\.as_ptr\(\),\s*old_key_pointer\)/.test(laws) && /assert_eq!\(previous_child\.child_id(?:\(\))?\.as_ptr\(\),\s*old_child_pointer\)/.test(laws) && /assert_eq!\(installed_key\.as_ptr\(\),\s*replacement_key_pointer\)/.test(laws) && /assert_eq!\(installed_child\.child_id(?:\(\))?\.as_ptr\(\),\s*replacement_child_pointer\)/.test(laws));
+  const capture = laws.indexOf("let plus_one_param_value_pointer = plus_one_param_value.as_ptr();"), rejected = laws.indexOf("let rejected_param = params.insert("), observed = laws.indexOf("assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer"), retired = laws.indexOf("retire_raster_value_cold((rejected_param.key, rejected_param.value))");
+  require("rejected-output-value-identity", capture >= 0 && rejected > capture && observed > rejected && retired > observed);
+  const childCapture = laws.search(/let plus_one_asset_child_pointer = plus_one_asset_child\.child_id(?:\(\))?\.as_ptr\(\);/), childRejected = laws.indexOf("let rejected_asset = assets.insert("), childObserved = laws.search(/assert_eq!\(rejected_asset\.value\.child_id(?:\(\))?\.as_ptr\(\),\s*plus_one_asset_child_pointer/), childRetired = laws.indexOf("retire_raster_value_cold((rejected_asset.key, rejected_asset.value))");
+  require("rejected-output-child-identity", childCapture >= 0 && childRejected > childCapture && childObserved > childRejected && childRetired > childObserved);
+  require("exact-physical-fixture", laws.includes("demand.capacity_bytes-1") && laws.includes("output.allocated_page_count(),2") && laws.includes("Arc::try_unwrap(root)") && laws.includes("measured midpoint and final-minus-one cuts remain partial") && laws.includes("serde_json::to_value(&actual)"));
+  return failures;
+}
+
+export function toolJobRasterEnvelopeCallerRetainedExact(store: string, raster: string, editor: string, plugin: string, editorTestLaws: string): boolean {
+  return toolJobRasterEnvelopeCallerFailures(store, raster, editor, plugin, editorTestLaws).length === 0;
+}
+
+export function toolJobDrawingEnvelopeCallerRetainedExact(store: string, drawing: string, editor: string, plugin: string, editorTestLaws: string = editor): boolean {
+  const secondApplyLane = [
+    "struct DrawingStoreInitializationAuthority",
+    "struct DrawingMutationCandidateAuthority",
+    "struct DrawingContainerRebuildAuthority",
+    "struct DrawingDuplicateRewriteAuthority",
+    "struct DrawingMutationArenaPool",
+    "fn node_at_mut",
+    "fn container_mut",
+    "fn apply_selection_transform",
+    "&mut DrawingSnapshot",
+    "snapshot_mut",
+    "operation.diff(",
+    "diff.apply(",
+    "source.assets.remove(",
   ];
-  const credits = editor.indexOf("begin_artifact_envelope_ingress(pages, ");
-  const page = editor.indexOf("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]");
-  const construct = editor.indexOf("store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, ");
-  const admit = editor.indexOf("admit_artifact_envelope_ingress_page(handle, page)");
-  const rasterOwnedMapSerializeBound = /impl(?:\s*<[^>]*>)?\s*(?:serde::)?Serialize\s+for\s+RasterOwnedMap\b/s.test(raster);
-  const rasterOwnedMapWholeSerializeMap = /serialize_map\s*\(\s*Some\s*\(\s*self\.length\s*\)\s*\)/s.test(raster);
-  const rasterOwnedMapWholeSerializeEntry = /\.serialize_entry\s*\(/s.test(raster);
-  const rasterOwnedMapSerdeGuards = (raster.match(/serde\(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"\)/g) ?? []).length;
-  const rasterOwnedMapFieldSites = (raster.match(/^\s*(?:pub )?[a-z_]+: RasterOwnedMap</gm) ?? []).length;
-  const rasterWholeJoinedOutput = /\.collect\s*::\s*<\s*Vec\s*<\s*_\s*>\s*>\s*\(\s*\)\s*\.join\s*\(/s.test(raster);
-  const rasterPerValueJsonOutput = /serde_json::to_vec\s*\(\s*v\s*\)/s.test(raster);
-  const rasterDirectMapOutput = /for\s*\(\s*k\s*,\s*v\s*\)\s*in\s*(?:map|params)\b/s.test(raster);
-  const rasterEmptyMapOutputGuards = (raster.match(/assert!\((?:map|params)\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
-  const rasterEmptyLayerOutputGuards = (raster.match(/assert!\(list\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
-  const rasterSnapshotOutputPreflights = (raster.match(/s\.require_empty_output_shell\(\)\.expect\(RASTER_POPULATED_OUTPUT_ERROR\);/g) ?? []).length;
-  const rasterMountedOutputEntryPoints = (raster.match(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\) -> Result<Vec<u8>, String> \{/g) ?? []).length;
-  const rasterMountedPixelComposites = (raster.match(/raster_composite_image\(snapshot\)/g) ?? []).length;
-  const rasterMountedPixelDialects = (raster.match(/semio_image_to_format\(&image, /g) ?? []).length;
-  const rasterMountedStdioEncoders = (raster.match(/semio_s_artifact_stdio_[a-z0-9_]*::[A-Za-z0-9_:]*io::encode_[a-z0-9]+\(/g) ?? []).length;
-  const rasterMountedVectorComposites = (raster.match(/raster_document_json_to_svg\(snapshot\)/g) ?? []).length;
-  const rasterMountedTypedDeclines = (raster.match(/Err\(RASTER_(?:PDF|DWG)_EXPORT_UNSUPPORTED\.to_string\(\)\)/g) ?? []).length;
-  const rasterMountedOutputGuards = (raster.match(/snapshot\.require_empty_output_shell\(\)\.map_err\(str::to_owned\)\?;/g) ?? []).length;
-  const rasterMountedOutputCallers = (raster.match(/Ok\(<RasterSnapshot as store::os_store::serialization::ArtifactDsl>::print_dsl\(snapshot\)\.into_bytes\(\)\)/g) ?? []).length;
-  const rasterOutputFixtureStart = raster.indexOf("fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()");
-  const rasterOutputFixtureEnd = raster.indexOf("fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()", rasterOutputFixtureStart);
-  const rasterOutputFixture = rasterOutputFixtureStart >= 0 && rasterOutputFixtureEnd > rasterOutputFixtureStart ? raster.slice(rasterOutputFixtureStart, rasterOutputFixtureEnd) : "";
-  const rasterRejectedParamValueCapture = rasterOutputFixture.indexOf("let plus_one_param_value_pointer = plus_one_param_value.as_ptr();");
-  const rasterRejectedParamInsert = rasterOutputFixture.indexOf("params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))");
-  const rasterRejectedParamValueBinding = rasterOutputFixture.indexOf("let rejected_param_value = match &rejected_param.value");
-  const rasterRejectedParamValueAssertion = rasterOutputFixture.indexOf(
-    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
-  );
-  const rasterRejectedParamRetirement = rasterOutputFixture.indexOf(
-    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
-  );
-  const rasterRejectedAssetChildCapture = rasterOutputFixture.indexOf("let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();");
-  const rasterRejectedAssetInsert = rasterOutputFixture.indexOf("assets.insert(plus_one_asset_key, plus_one_asset_child)");
-  const rasterRejectedAssetChildBinding = rasterOutputFixture.indexOf("rejected_asset.value.child_id.as_ptr()");
-  const rasterRejectedAssetChildAssertion = rasterOutputFixture.indexOf(
-    ', plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
-    rasterRejectedAssetChildBinding,
-  );
-  const rasterRejectedAssetRetirement = rasterOutputFixture.indexOf(
-    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
-  );
-  return (
-    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
-    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
-    raster.includes("pub struct RasterEnvelopeOwnedFieldCatalog") &&
-    raster.includes("artifact_owned_spr_edit_history_decoder") &&
-    raster.includes("RasterSnapshotDecodeAuthority") &&
-    raster.includes("RasterMutationDecodeAuthority") &&
-    raster.includes("OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>") &&
-    raster.includes("struct RasterSnapshotBoundsAuthority") &&
-    raster.includes("struct RasterSnapshotCloneAuthority") &&
-    raster.includes("struct RasterMutationCandidateAuthority") &&
-    raster.includes("RASTER_RETIREMENT_STACK_CAPACITY") &&
-    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>") &&
-    raster.includes("pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>") &&
-    raster.includes("struct RasterStoreInitializationAuthority") &&
-    raster.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority") &&
-    raster.includes("RasterStoreInitializationPhase::ValidateEdit { index }") &&
-    raster.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
-    raster.includes("RasterStoreInitializationPhase::SeedHistory") &&
-    raster.includes("RasterStoreInitializationPhase::BuildCandidate") &&
-    (raster.match(/if cx\.should_yield\(\)/g) ?? []).length === 3 &&
-    raster.includes("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool") &&
-    raster.includes("cx.consume_fuel(1)") &&
-    raster.includes("self.cancel_requested || cx.is_cancelled()") &&
-    raster.includes("ArtifactStoreInitializationRuntime::new") &&
-    raster.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
-    raster.includes("self.generation.0.checked_add(1)") &&
-    raster.includes("value.layers.pop()") &&
-    raster.includes("value.assets.take_last_entry()") &&
-    raster.includes("value.assets.take_empty_page_backing()") &&
-    raster.includes("RasterRetirementOwner::AssetMapPage(page)") &&
-    raster.includes("RasterRetirementOwner::ValueMapPage(page)") &&
-    raster.includes("RasterOwnedMap::<V>::conservative_page_credit_bytes()") &&
-    raster.includes("fn observe_candidate_capacity(") &&
-    raster.includes("fn raster_exact_string_from_parts(") &&
-    raster.includes("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES") &&
-    raster.includes("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS") &&
-    raster.includes("const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES") &&
-    raster.includes("source_control_bytes: usize") &&
-    raster.includes("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES") &&
-    raster.includes("raster-store.control-backing-double-reservation") &&
-    raster.includes("static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize") &&
-    raster.includes("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
-    raster.includes("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
-    raster.includes("held_items: usize") &&
-    raster.includes("held_bytes: usize") &&
-    (raster.match(/control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>/g) ?? []).length === 2 &&
-    (raster.match(/let control = RasterStandaloneControlCredit::try_claim\(\)\.ok\(\);/g) ?? []).length === 2 &&
-    raster.includes("fn claim_control_if_available(&mut self) -> Result<bool, String>") &&
-    raster.includes("Err(\"raster-store.standalone-control-capacity\") => Ok(false)") &&
-    raster.includes("control_returned: bool") &&
-    raster.includes("remaining: RASTER_NON_STACK_CONTROL_BACKINGS") &&
-    raster.includes("control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>") &&
-    raster.includes("normal completion returns every non-stack process control credit") &&
-    raster.includes("standalone Box control credit is returned before terminal-empty") &&
-    raster.includes("standalone Arc and inner Box control credits return before terminal-empty") &&
-    raster.includes("fn reserve_page_credit(&mut self, page_index: usize)") &&
-    raster.includes("compare_exchange(current, next") &&
-    raster.includes("self.return_page_credit(page_index)?") &&
-    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>") &&
-    raster.includes("Raster owned map reached Drop before every entry and page backing was explicitly retired") &&
-    raster.includes("Populated Raster owned maps require the retained page clone authority") &&
-    raster.includes("Raster maps require the retained page decoder") &&
-    raster.includes("impl<V> dsl::ToValue for RasterOwnedMap<V>") &&
-    raster.includes("impl<V> dsl::FromValue for RasterOwnedMap<V>") &&
-    raster.includes("Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority") &&
-    rasterOwnedMapFieldSites === 3 &&
-    (rasterOwnedMapSerdeGuards === 0 || raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>")) &&
-    !rasterOwnedMapSerializeBound &&
-    !rasterOwnedMapWholeSerializeMap &&
-    !rasterOwnedMapWholeSerializeEntry &&
-    raster.includes("pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>") &&
-    raster.includes("self.layers.is_empty() && self.assets.is_empty()") &&
-    raster.includes("Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority") &&
-    raster.includes("pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String") &&
-    raster.includes("pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String") &&
-    raster.includes("fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
-    raster.includes("fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
-    raster.includes("fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
-    raster.includes("fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
-    !raster.includes("async fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
-    !raster.includes("async fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
-    !raster.includes("async fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
-    !raster.includes("async fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
-    raster.includes("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);") &&
-    raster.includes("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;") &&
-    rasterEmptyMapOutputGuards === 4 &&
-    rasterEmptyLayerOutputGuards === 2 &&
-    rasterSnapshotOutputPreflights === 2 &&
-    rasterMountedOutputEntryPoints === 8 &&
-    rasterMountedPixelComposites === 5 &&
-    rasterMountedPixelDialects === 5 &&
-    rasterMountedStdioEncoders === 5 &&
-    rasterMountedVectorComposites === 1 &&
-    rasterMountedTypedDeclines === 2 &&
-    raster.includes("pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str =") &&
-    raster.includes("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =") &&
-    rasterMountedOutputGuards === 0 &&
-    rasterMountedOutputCallers === 0 &&
-    rasterRejectedParamValueCapture >= 0 &&
-    rasterRejectedParamInsert > rasterRejectedParamValueCapture &&
-    rasterRejectedParamValueBinding > rasterRejectedParamInsert &&
-    rasterRejectedParamValueAssertion > rasterRejectedParamValueBinding &&
-    rasterRejectedParamRetirement > rasterRejectedParamValueAssertion &&
-    rasterRejectedAssetChildCapture >= 0 &&
-    rasterRejectedAssetInsert > rasterRejectedAssetChildCapture &&
-    rasterRejectedAssetChildBinding > rasterRejectedAssetInsert &&
-    rasterRejectedAssetChildAssertion > rasterRejectedAssetChildBinding &&
-    rasterRejectedAssetRetirement > rasterRejectedAssetChildAssertion &&
-    !rasterWholeJoinedOutput &&
-    !rasterPerValueJsonOutput &&
-    !rasterDirectMapOutput &&
-    raster.includes("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority") &&
-    raster.includes("dsl::FieldValue::Map(Vec::new())") &&
-    raster.includes("pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>") &&
-    !raster.includes("pub fn remove(&mut self, key: &str)") &&
-    !raster.includes("result.insert(key.clone(), value.clone())") &&
-    !raster.includes("while let Some((key, value)) = source.next_entry") &&
-    raster.includes("RasterRetirementOwner::BoxedLayer(Some(layer))") &&
-    raster.includes("const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES") &&
-    raster.includes("const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN") &&
-    raster.includes("dsl::DslValue::Array(values)") &&
-    raster.includes("dsl::DslValue::Object(values)") &&
-    variants.every((variant) => raster.includes(variant)) &&
-    raster.includes("Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
-    raster.includes("raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally") &&
-    raster.includes("raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
-    raster.includes("raster_store_initializer_zero_budget_advances_no_owner_or_phase") &&
-    raster.includes("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant") &&
-    raster.includes("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant") &&
-    raster.includes("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner") &&
-    raster.includes("raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel") &&
-    raster.includes("raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page") &&
-    raster.includes("raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner") &&
-    raster.includes("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic") &&
-    raster.includes("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal") &&
-    raster.includes("raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth") &&
-    raster.includes("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone") &&
-    raster.includes("raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing") &&
-    raster.includes("raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses") &&
-    raster.includes("raster_empty_asset_map_retirement_has_no_hidden_allocation_release") &&
-    raster.includes("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly") &&
-    raster.includes("raster_observed_capacity_and_combined_retirement_depth_are_exact") &&
-    raster.includes("raster_box_and_arc_control_backings_require_and_report_fixed_credit") &&
-    raster.includes("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation") &&
-    raster.includes("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase") &&
-    raster.includes("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
-    raster.includes("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
-    raster.includes("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
-    raster.includes("std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES") &&
-    raster.includes("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES") &&
-    raster.includes("raster_maximum_combined_layer_and_value_depth_retires_to_terminal") &&
-    (raster.match(/RasterOwnedMapInsert::Replaced/g) ?? []).length >= 1 &&
-    raster.includes("replacement returns the exact displaced pair") &&
-    raster.includes("assert_eq!(previous_key.as_ptr(), old_key_pointer)") &&
-    raster.includes("candidate_disposer") &&
-    raster.includes("let released_bytes = value.capacity();") &&
-    raster.includes("let bytes = value.capacity();") &&
-    !raster.includes("serde_json::to_vec(source)") &&
-    !raster.includes("source.clone()") &&
-    !raster.includes("source.assets.clone()") &&
-    !raster.includes("serde_json::to_value(&source.assets)") &&
-    !raster.includes("source.assets.to_value()") &&
-    !raster.includes("entries.push((key.clone(), value.to_value()))") &&
-    !raster.includes("drop(populated_map)") &&
-    !raster.includes("operation.diff(current)") &&
-    !raster.includes("diff.apply(current)") &&
-    !raster.includes("active: Box<RasterOwnedRetirement>") &&
-    !raster.includes("artifact_bounded_history_entry_decoder") &&
-    editor.includes("fn build_envelope_decode_owner_bundle()") &&
-    editor.includes("raster_envelope_decode_owner_bundle()") &&
-    editor.includes("fn build_document_store_initialization_job(") &&
-    editor.includes("raster_document_store_initialization_job(envelope, operation, generation)") &&
-    editor.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle") &&
-    editor.includes("assert_eq!(handle.generation, base_generation)") &&
-    editor.includes("chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
-    credits >= 0 &&
-    page > credits &&
-    construct > page &&
-    admit > construct &&
-    editor.includes("seal_artifact_envelope_ingress(handle)") &&
-    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
-    editor.includes("advance_artifact_envelope_load(handle)") &&
-    editor.includes("acknowledge_artifact_store_replacement(handle)") &&
-    editor.includes("duplicate Raster load acknowledgement is a no-op") &&
-    editor.includes("cancel_artifact_envelope_load(handle)") &&
-    editor.includes("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
-    editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
-    editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication") &&
-    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
-    toolJobStoreInitializerRetainedExact(plugin) &&
-    plugin.includes("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close") &&
-    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
-    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
-    !editor.includes("envelope_json: &str") &&
-    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
-    !editor.includes("ArtifactStore::new") &&
-    !editor.includes("source.to_vec()") &&
-    !editor.includes("while let") &&
-    !editor.includes("loop {")
-  );
-}
-
-export function toolJobDrawingEnvelopeCallerRetainedExact(store: string, drawing: string, editor: string, plugin: string, editorTestLaws: string = editor): boolean {
-  const variants = [
-    "SetLayerVisible(payload)",
-    "SetLayerLocked(payload)",
-    "SetLayerOpacity(payload)",
-    "SetLayerBlendMode(payload)",
-    "RenameLayer(payload)",
-    "UpdateLayerTransform(payload)",
-    "ReplaceLayerFill(payload)",
-    "ReplaceLayerStroke(payload)",
-    "SetLayerBooleanOperation(payload)",
-    "UpdateLayerTraceParams(payload)",
-    "CreateLayer(payload)",
-    "DuplicateLayer(payload)",
-    "DeleteLayer(payload)",
-    "ReorderLayer(payload)",
+  const ownedInitializerHooks = [
+    "fn build_document_store_initialization_job",
+    "drawing_document_store_initialization_job",
+    "request_drawing_mutation_arena_pool",
+    "drawing_mutation_arena_pool_fault",
+    "arena_boot_fault",
   ];
-  const owners = [
-    "DrawingRetirementOwner::Layer(value)",
-    "DrawingRetirementOwner::Fill(fill)",
-    "DrawingRetirementOwner::Stroke(stroke)",
-    "DrawingRetirementOwner::Segments(values)",
-    "DrawingRetirementOwner::Stops(values)",
-    "DrawingRetirementOwner::Points(values)",
-    "DrawingRetirementOwner::AssetEntry",
-  ];
-  const digestFields = [
-    "FillStyle::Solid { color }",
-    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
-    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
-    "stop.offset",
-    "stop.color",
-    "value.color[(self.phase - 1) as usize]",
-    "value.width",
-    "observe_owned_string(digest, 246, &value.cap",
-    "observe_owned_string(digest, 247, &value.join",
-    "value.dash.is_some()",
-    "base.visible",
-    "base.locked",
-    "base.opacity",
-    "observe_owned_string(digest, 106, &base.blend_mode",
-    "base.transform.rotation",
-    "observe_owned_string(digest, 340, &value.shape_kind",
-    "value.rect.is_some()",
-    "value.ellipse.is_some()",
-    "value.circle.is_some()",
-    "value.line.is_some()",
-    "value.polygon.is_some()",
-    "PathSegment::Move",
-    "PathSegment::Line",
-    "PathSegment::Quad",
-    "PathSegment::Cubic",
-    "PathSegment::Arc",
-    "PathSegment::Close",
-    "observe_owned_string(digest, 382, &value.content",
-    "observe_owned_string(digest, 390, &value.image_key",
-    "observe_owned_string(digest, 410, &value.operation",
-    "value.children.len()",
-    "observe_owned_string(digest, 420, &value.source_key",
-    "value.params.threshold",
-    "Self::variant(mutation)",
-    "value.visible",
-    "value.locked",
-    "value.opacity",
-    "observe_owned_string(digest, 3, &value.blend_mode",
-    "observe_owned_string(digest, 3, &value.new_name",
-    "value.transform.rotation",
-    "value.fill.as_ref()",
-    "value.stroke.as_ref()",
-    "observe_owned_string(digest, 3, &value.boolean_operation",
-    "value.params.simplify_epsilon",
-    "value.parent_id.is_some()",
-    "value.index",
-    "value.layer",
-  ];
-  const requestStart = drawing.indexOf("pub fn request_drawing_mutation_arena_pool()");
-  const requestAuthority = requestStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", requestStart)) : undefined;
-  const requestBlock = requestAuthority?.body ?? "";
-  const borrowStart = drawing.indexOf("fn borrow_drawing_mutation_arena()");
-  const borrowAuthority = borrowStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", borrowStart)) : undefined;
-  const borrowBlock = borrowAuthority?.body ?? "";
-  const defaultStart = editor.indexOf("impl Default for DrawingPlayApp");
-  const defaultAuthority = defaultStart >= 0 ? toolJobRustBlock(editor, editor.indexOf("{", defaultStart)) : undefined;
-  const defaultBlock = defaultAuthority?.body ?? "";
-  const governedBootstrapArm = /DrawingStoreInitializationPhase::InitializeArena\s*=>\s*\{\s*match\s+self\.arena_bootstrap_job\.step\(cx\)/s.test(drawing);
-  const exactSourceUndo = /if let Some\(undo\) = self\.source_undo\s*\{[\s\S]*?self\.start_rebuild\(source, undo\.parent, None, Some\(undo\.index\), DrawingContainerRebuildRole::CloseSourceUndo\)\?;/s.test(drawing);
+  const ingress = `${editor}\n${editorTestLaws}`;
   return (
     store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
@@ -976,4 +776,23 @@

     store.includes("id_digest: [u8; 32]") &&
     store.includes('Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?') &&
+    !store.includes("id.clone(), edit_digest") &&
+    ingress.includes("pub struct DrawingEnvelopeLoadHandle") &&
+    ingress.includes("source: &js_sys::Uint8Array") &&
+    ingress.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
+    ingress.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
+    ingress.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
+    ingress.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
+    ingress.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
+    ingress.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
+    ingress.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
+    !ingress.includes("reject_whole_buffer_artifact_envelope_ingress") &&
+    !ingress.includes("envelope_json: Option<String>") &&
+    !ingress.includes("while let") &&
+    !ingress.includes("loop {") &&
+    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
+    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
+    store.includes("self.scalar_entry") &&
+    store.includes("artifact-spr.mutation-array-cancelled") &&
+    store.includes("id_digest: [u8; 32]") &&
     drawing.includes("pub struct DrawingEnvelopeOwnedFieldCatalog") &&
     drawing.includes("artifact_owned_spr_edit_history_decoder") &&
@@ -983,183 +802,20 @@

     drawing.includes("const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096") &&
     drawing.includes("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64") &&
-    drawing.includes("struct DrawingSnapshotBoundsAuthority") &&
-    drawing.includes("struct DrawingFixedOwnerCensus") &&
-    drawing.includes("slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]") &&
-    drawing.includes("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16") &&
-    drawing.includes("const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64") &&
-    drawing.includes("const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4") &&
-    drawing.includes("struct DrawingMutationArenaOwner") &&
-    drawing.includes("struct DrawingMutationArenaPool {") &&
-    drawing.includes("DrawingMutationArenaProcessState::Building") &&
-    drawing.includes("#[cfg(test)]\n    fn try_new()") &&
-    drawing.includes("struct DrawingMutationArenaOwnerBuilder") &&
-    drawing.includes("struct DrawingMutationArenaPoolBootstrap") &&
-    drawing.includes("struct DrawingMutationArenaBootstrapJob") &&
-    drawing.includes("fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep") &&
-    drawing.includes("DrawingMutationArenaProcessState::Inert") &&
-    drawing.includes("DrawingMutationArenaBootstrapAdmission::fixed()") &&
-    drawing.includes("DrawingMutationArenaBootstrapStep::Blocked") &&
-    governedBootstrapArm &&
-    exactSourceUndo &&
-    requestBlock.includes("DrawingMutationArenaProcessState::Inert") &&
-    requestBlock.includes("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true") &&
-    !requestBlock.includes("bootstrap.step(") &&
-    !requestBlock.includes("try_reserve_exact") &&
-    !requestBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
-    borrowBlock.includes("DrawingMutationArenaBorrowError::NotReady") &&
-    borrowBlock.includes("request_drawing_mutation_arena_pool()") &&
-    !borrowBlock.includes("bootstrap.step(") &&
-    !borrowBlock.includes("try_reserve_exact") &&
-    !borrowBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
-    drawing.includes("DrawingStoreInitializationPhase::InitializeArena") &&
-    drawing.includes("DrawingMutationArenaProcessTransition::Retire") &&
-    drawing.includes("drawing_mutation_arena_pool_fault") &&
-    drawing.includes("borrow_drawing_mutation_arena_from") &&
-    drawing.includes("drawing-store.mutation-arena-pool-saturated") &&
-    drawing.includes("drawing-store.mutation-arena-stale-generation") &&
-    drawing.includes("owner.terminal_is_empty()") &&
-    drawing.includes("arena_return_phase: u8") &&
-    drawing.includes("Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })") &&
-    drawing.includes("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)") &&
-    drawing.includes("pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)") &&
-    drawing.includes("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)") &&
-    drawing.includes("total.checked_add(page.capacity())") &&
-    drawing.includes("fn write_overlay_string") &&
-    drawing.includes("target.capacity() < source.len()") &&
-    drawing.includes("struct DrawingMutationOverlayPatch") &&
-    drawing.includes("drawing-store.mutation-overlay-owner-changed") &&
-    drawing.includes("struct DrawingAssetBoundsCursor") &&
-    drawing.includes("assets.range::<str, _>") &&
-    drawing.includes("drawing-store.preflight-depth-capacity") &&
-    drawing.includes("drawing-store.preflight-item-capacity") &&
-    drawing.includes("drawing-store.preflight-byte-capacity") &&
-    drawing.includes("struct DrawingLayerCloneAuthority") &&
-    drawing.includes("self.bounds.step(source, cx)") &&
-    drawing.includes("layer.step(source.layers.get(self.index)") &&
-    drawing.includes("target.children.push(Self::skeleton(child)?)") &&
-    drawing.includes("struct DrawingStoreInitializationAuthority") &&
-    drawing.includes("struct DrawingSemanticDigestCredit") &&
-    drawing.includes("semantic: Option<semio_framework_hash::Sha256>") &&
-    drawing.includes('digest.observe(b"drawing.semantic.sha256")') &&
-    drawing.includes("struct DrawingFillDigestAuthority") &&
-    drawing.includes("struct DrawingStrokeDigestAuthority") &&
-    drawing.includes("struct DrawingPathSegmentDigestAuthority") &&
-    drawing.includes("struct DrawingLayerVariantDigestAuthority") &&
-    drawing.includes("struct DrawingLayerDigestAuthority") &&
-    drawing.includes("struct DrawingMutationDigestAuthority") &&
-    drawing.includes("struct DrawingMutationAggregateReservation") &&
-    drawing.includes("mutation_source_items") &&
-    drawing.includes("mutation_derived_items") &&
-    drawing.includes("duplicate_candidate_items") &&
-    drawing.includes("source_owner_bytes") &&
-    drawing.includes("derived_owner_bytes") &&
-    drawing.includes("DRAWING_MUTATION_AGGREGATE_ITEMS") &&
-    drawing.includes("DRAWING_MUTATION_AGGREGATE_BYTES") &&
-    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_ITEMS") &&
-    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_BYTES") &&
-    drawing.includes("drawing-store.mutation-aggregate-item-capacity") &&
-    drawing.includes("drawing-store.mutation-aggregate-byte-capacity") &&
-    drawing.includes("struct DrawingMutationCandidateAuthority") &&
-    drawing.includes("drawing-store.mutation-candidate-stale-authority") &&
-    drawing.includes("drawing-store.mutation-candidate-cancelled") &&
-    drawing.includes("struct DrawingDuplicateRewriteAuthority") &&
-    drawing.includes("material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]") &&
-    drawing.includes("self.hash_cursor + DRAWING_OWNED_FIELD_BYTES") &&
-    drawing.includes('update(b"semio.drawing.duplicate-id.v1")') &&
-    drawing.includes("update(&(self.id_len as u64).to_be_bytes())") &&
-    drawing.includes("update(&(self.name_len as u64).to_be_bytes())") &&
-    drawing.includes("pending_name: std::mem::ManuallyDrop<Option<String>>") &&
-    drawing.includes("drawing-store.duplicate-destination-capacity") &&
-    drawing.includes("DrawingMutationCandidatePhase::PreflightSource") &&
-    drawing.includes("DrawingMutationCandidatePhase::PreflightMutation") &&
-    drawing.includes("struct DrawingContainerRebuildAuthority") &&
-    drawing.includes("const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY") &&
-    drawing.includes("enum DrawingContainerRebuildMove") &&
-    drawing.includes("rebuild.rollback_step()?") &&
-    drawing.includes("DrawingContainerRebuildRole::CloseSourceUndo") &&
-    drawing.includes("source_undo: Option<DrawingContainerSourceUndo>") &&
-    drawing.includes("source.capacity() < output_capacity") &&
-    drawing.includes("self.source.as_mut().ok_or(\"drawing-store.container-source\")?.push(value)") &&
-    drawing.includes("DrawingStoreInitializationPhase::ValidateEditId") &&
-    drawing.includes("DrawingStoreInitializationPhase::ValidateEditMeta") &&
-    drawing.includes("DrawingStoreInitializationPhase::PrepareApplied") &&
-    drawing.includes("DrawingStoreInitializationPhase::PrepareRedo") &&
-    drawing.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority") &&
-    drawing.includes("DrawingStoreInitializationPhase::ValidateEdit { index }") &&
-    drawing.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
-    drawing.includes("DrawingStoreInitializationPhase::SeedHistory") &&
-    drawing.includes("DrawingStoreInitializationPhase::BuildCandidate") &&
-    drawing.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
-    drawing.includes("ArtifactStoreInitializationOwnerCatalog::try_new()") &&
-    drawing.includes("ArtifactStoreInitializationRuntime::new_with_owner_catalog") &&
-    drawing.includes("DrawingStoreInitializationPhase::MoveInitialOwner") &&
-    drawing.includes("self.generation.0.checked_add(1)") &&
-    owners.every((owner) => drawing.includes(owner)) &&
-    digestFields.every((field) => drawing.includes(field)) &&
-    variants.every((variant) => drawing.includes(variant)) &&
-    drawing.includes("retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners") &&
-    drawing.includes("retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty") &&
-    drawing.includes("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner") &&
-    drawing.includes("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field") &&
-    drawing.includes("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback") &&
-    drawing.includes("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision") &&
-    drawing.includes("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba") &&
-    drawing.includes("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner") &&
-    drawing.includes("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid") &&
-    drawing.includes("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot") &&
-    drawing.includes("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots") &&
-    drawing.includes("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant") &&
-    drawing.includes("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close") &&
-    drawing.includes("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool") &&
-    drawing.includes("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close") &&
-    drawing.includes("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation") &&
-    drawing.includes("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed") &&
-    !drawing.includes("source.clone()") &&
-    !drawing.includes("snapshot.clone()") &&
-    !drawing.includes("operation.encode_op()") &&
-    !drawing.includes("operation.diff(") &&
-    !drawing.includes("diff.apply(") &&
-    !drawing.includes("serde_json::from_value") &&
-    !drawing.includes("serde_json::from_str") &&
-    !drawing.includes("serde_json::to_vec") &&
-    !drawing.includes("mutation_meta.iter().any") &&
-    !drawing.includes("exact_for_test") &&
-    !drawing.includes("iter().nth") &&
-    !drawing.includes("structural_copies") &&
-    !drawing.includes("DRAWING_ASSET_MAP_NODE_POINTERS") &&
-    !drawing.includes("DrawingStoreInitializationPhase::CloneInitial") &&
-    !drawing.includes("base.name.try_reserve_exact") &&
-    !drawing.includes("DrawingMutationArenaOwner::try_new()?") &&
-    !drawing.includes("rposition(Option::is_some)") &&
-    !store.includes("id.clone(), edit_digest") &&
-    !drawing.includes("drop(value.layers)") &&
-    editor.includes("drawing_document_store_initialization_job(envelope, operation, generation)") &&
+    drawing.includes("struct DrawingNativeCloneAuthority") &&
+    drawing.includes("struct DrawingLayerCloneWorkAuthority") &&
+    drawing.includes("struct DrawingLayerLocator") &&
+    secondApplyLane.every((forbidden) => !drawing.includes(forbidden)) &&
+    ownedInitializerHooks.every((forbidden) => !editor.includes(forbidden) && !drawing.includes(forbidden)) &&
     editorTestLaws.includes("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed") &&
     editorTestLaws.includes("drawing_live_envelope_cancel_closes_retained_pages_without_publication") &&
     editorTestLaws.includes("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate") &&
     editorTestLaws.includes("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle") &&
-    editor.includes("arena_boot_fault: Option<&'static str>") &&
-    editor.includes("drawing_mutation_arena_pool_fault") &&
     editorTestLaws.includes('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer') &&
-    editor.includes("pub struct DrawingEnvelopeLoadHandle") &&
-    editor.includes("source: &js_sys::Uint8Array") &&
-    editor.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
-    editor.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
-    editor.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
-    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
-    editor.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
-    editor.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
-    editor.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
-    editor.includes("request_drawing_mutation_arena_pool()") &&
-    defaultBlock.includes("request_drawing_mutation_arena_pool()") &&
-    !defaultBlock.includes("bootstrap.step(") &&
-    !defaultBlock.includes("try_reserve_exact") &&
-    !defaultBlock.includes("initialize_drawing_mutation_arena_pool") &&
+    editorTestLaws.includes("drawing_live_envelope_replay_equals_the_central_apply_fold_of_every_forward") &&
+    editorTestLaws.includes("protocol::apply_diff_cold(outcome.diff(), &expected)") &&
     plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
-    toolJobStoreInitializerRetainedExact(plugin) &&
+    plugin.includes("Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))") &&
     !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
-    !editor.includes("envelope_json: Option<String>") &&
-    !editor.includes("while let") &&
-    !editor.includes("loop {")
+    !editor.includes("envelope_json: Option<String>")
   );
 }

### 57 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts
HEAD 313 BASE 1027 STASH 1444 CURRENT H
--- 

+++ 

@@ -1,2 +1,3 @@

+import { createRequire } from "node:module";
 /** 🎯️ Wave 4 plugin consistency policy: mutation outcome, merge policy and diff-only laws.
  * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
@@ -21,5 +22,5 @@

  * merge-strategy/conflict-rule vocabulary reaches zero, `MergePolicy`'s 3 variants mirror across all 4
  * surfaces that spell them, and the dsl derive macro's package glue mounts its one owner implementation.
- * All sixteen rules (the seven above plus the diff-only rules R8–R16 of ticket `26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`) read one shared git-visible inventory (`policyMutationLawInventory`).
+ * All eighteen rules (the seven above plus the diff-only rules R8–R18 of ticket `26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`) read one shared git-visible inventory (`policyMutationLawInventory`).
  */
 /** 📖️The frozen outcome-code vocabulary's language-agnostic document — the one table this gate, the Rust protocol
@@ -519,5 +520,5 @@

 }
 
-type PolicyDiffOnlyRuleId = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16";
+type PolicyDiffOnlyRuleId = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16" | "R17" | "R18";
 type PolicyDiffOnlyRule = Readonly<{ code: string; slug: string; reason: string; solution: string }>;
 
@@ -533,4 +534,6 @@

   R15: { code: "V4-LAW-UNTESTED", slug: "inverse-sum-law-untested", reason: "no test under the leaf's own `🧪️tests` calls `assert_mutation_inverse_sum_law`", solution: "Add a test under the leaf's `🧪️tests` that calls `assert_mutation_inverse_sum_law(&mutation, &before)` on each applied fixture." },
   R16: { code: "V3-LEAF-APPLY", slug: "outcome-apply-to", reason: "`MutationOutcome::apply_to` is gone: a diff is applied only by the central applier", solution: "Call `protocol::apply_diff(outcome.diff(), &base)` instead." },
+  R17: { code: "V3-LEAF-APPLY", slug: "host-apply", reason: "a host/editor/apply/preparation lane changes a snapshot outside `protocol::apply_diff` (`&mut <X>Snapshot` field writes, `snapshot_mut`, before/after differencing, TypeScript local apply into a snapshot ref)", solution: "Turn the gesture into a mutation whose sparse diff names the changed rows and apply it only with `protocol::apply_diff(&mutation.diff(&base).diff, &base)` (directly or through the artifact's `🌉️apply` wrapper); the shell dispatches the envelope and renders the store's projection." },
+  R18: { code: "V1-SNAPSHOT-DIFF", slug: "row-emitter", reason: "a computational gesture commits an assembled mesh or compares rows with the base to sparsify (`commit_built`, `equals_base`, a kernel `*_job(` builder, `sparse_rows(`, `::produced(`)", solution: "Make the gesture a row emitter: read the base through the write-recording overlay, decide which faces and vertices it replaces, removes or appends, and state exactly those rows through the face editor; the journal is the diff." },
 };
 
@@ -549,4 +552,6 @@

 /** 🔎️Snapshot-differencing and simulation helpers under any spelling: `*between*`, `*_replacing`, `*_state_diff`, `state_after*`, `value_diff*`. */
 const POLICY_DIFF_ONLY_BETWEEN_RE = /(?<!\bfn\s+)(?<!\w)(?:\w*between\w*|\w*_replacing|\w*_state_diff|state_after\w*|value_diff\w*)\s*(?:::\s*<[^>(]*>)?\s*\(/g;
+/** 📐️Pure geometry named `…_between` (the angle, distance or gap between two vectors or points) is not snapshot differencing and never counts for R10. */
+const POLICY_DIFF_ONLY_GEOMETRY_BETWEEN_RE = /^(?:\w+_)?(?:angle|distance|gap)_between\b/;
 /** 🔎️Fns that build diff-like or whole-state values from the forward payload: negative simulators and derived-inverse helpers. */
 const POLICY_DIFF_ONLY_SIMULATOR_RE = /(?<!\bfn\s+)(?<!\w)(?:negative|state_after\w*|inverse_of|value_diff\w*)\s*\(|\.negat(?:e|ion)\w*\s*\(/g;
@@ -565,4 +570,8 @@

 const POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE = /\b\w*[dD]iff\w*\b|\boutcome\b/;
 const POLICY_DIFF_ONLY_RESTORE_RE = /\b(?:SetSnapshot|PatchSnapshot|ReplaceDocument|ReplaceSnapshot|ReplaceConfig|ReplaceState|SetDocument|Restore\w*)\b|\b(?:Self|[A-Z]\w*)::\w*Snapshot\b|(?<!\blet\s)(?<!\bmut\s)\b\w+_(?:state|content|snapshot|document)\s*:(?!:)/g;
+/** 🫧️The ephemeral-root setter kinds: the presence/window-transient mutation TYPES that travel by whole-root transfer. Judged by the mutation's own type name, never by a directory. */
+const POLICY_DIFF_ONLY_EPHEMERAL_MUTATION_RE = /^\w*(?:Transient|Presence)Mutation$/;
+/** 🫧️The one whole-root setter an ephemeral mutation may invert to: `Self::Snapshot` / `Self::<Name>Snapshot` carrying the base root. */
+const POLICY_DIFF_ONLY_EPHEMERAL_SETTER_RE = /^Self::\w*Snapshot$/;
 const POLICY_DIFF_ONLY_APPLY_TO_RE = /(?:\.|::)apply_to\s*\(/g;
 const POLICY_DIFF_ONLY_TYPE_DIFF_RE = /\btype\s+Diff\s*=\s*([^;]+);/;
@@ -800,4 +809,276 @@

 }
 
+const POLICY_HOST_APPLY_LANE_DIRS: ReadonlySet<string> = new Set(["✏️editor", "🌉️apply", "📬️preparation"]);
+/** 🚪️Directories R17 never reads: snapshot decoders, test oracles and fixtures, and the trees R8–R16 already gate leaf by leaf. */
+const POLICY_HOST_APPLY_EXCLUDED_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🔮️oracles", "🧪️tests", "🧫️fixtures", "🧬️schema", "🧬️mutations", "🧬️mutation-support"]);
+/** 🚪️Directories whose code is an exempt CONTEXT for the call sites of a snapshot-writing helper: decoders, oracles, tests. */
+const POLICY_HOST_APPLY_SITE_EXEMPT_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🔮️oracles", "🧪️tests", "🧫️fixtures"]);
+const POLICY_HOST_APPLY_FRAMEWORK_PREFIX = "🧰️framework/🛍️products/💻️os/";
+const POLICY_HOST_APPLY_PLUGIN_ROOT_RE = new RegExp(`^✏️s/🔌️plugins/[^/]+/🗿️artifacts/[^/]+/${POLICY_RS_COMPONENT_LEAF_NAME.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`);
+/** 🔎️A `&mut <X>Snapshot` / `&mut <X>Document` parameter: the only parameter shape R17 follows (framework code has its own `*Document` containers, so there only `Snapshot` counts). */
+const POLICY_HOST_APPLY_MUT_PARAM_RE = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(?:'\w+\s+)?mut\s+(?:\w+::)*\w*(?:Snapshot|Document)\b/;
+const POLICY_HOST_APPLY_MUT_SNAPSHOT_PARAM_RE = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(?:'\w+\s+)?mut\s+(?:\w+::)*\w*Snapshot\b/;
+/** 🔎️A fn that builds a load effect: everything it calls fills a freshly decoded document (genesis), not an apply lane. */
+const POLICY_HOST_APPLY_LOAD_BUILDER_RE = /\breset_document_effect\b|\bLoadDocument\b/;
+const POLICY_HOST_APPLY_PLACE = String.raw`(?:\s*\.\s*\w+|\s*\[[^\]\n]*\])*`;
+const POLICY_HOST_APPLY_WRITE_METHODS = String.raw`push|push_str|push_back|push_front|pop|remove|insert|extend|extend_from_slice|truncate|clear|retain|swap|swap_remove|drain|append|resize|entry|take|replace|sort\w*|dedup\w*|set_\w+|\w+_mut`;
+/** 🔎️A before/after differencing call: `object_patch_diff(&before, &after)`. */
+const POLICY_HOST_APPLY_BEFORE_AFTER_CALL_RE = /\b(\w*(?:diff|delta|patch)\w*)\s*\(\s*&\s*\w*(?:before|old|prev|previous|original)\w*\s*,\s*&\s*\w*(?:after|new|next|updated|edited)\w*\s*[,)]/gi;
+const POLICY_HOST_APPLY_BEFORE_NAME_RE = /^(?:\w*_)?(?:before|old|prev|previous|original)\w*$/i;
+const POLICY_HOST_APPLY_AFTER_NAME_RE = /^(?:\w*_)?(?:after|new|next|updated|edited)\w*$/i;
+const POLICY_HOST_APPLY_TS_LOCAL_APPLY_RE = /\bapply(?:Local|Optimistic)\w*\s*\(|\bapply\w*Mutation\w*Local\w*\s*\(/;
+const POLICY_HOST_APPLY_TS_REF_ASSIGN_RE = /\b(\w*(?:Catalog|Snapshot|Document|Archive)\w*Ref)\.current\s*=\s*([^;\n]+)/g;
+/** 🔎️TypeScript probe: only files that mention an archive put/restore or a snapshot ref are parsed. */
+const POLICY_HOST_APPLY_TS_PROBE_RE = /Ref\.current|localDocumentArchive|encodeDocumentArchiveBytes|restore\w*Archive/;
+/** 🗃️The archive read back from the store/plugin after its central apply: persisting THAT is the persistence lane, not a second apply. */
+const POLICY_HOST_APPLY_TS_READ_BACK_RE = /\bread\w*DocumentArchive\w*\s*\(/;
+/** 🚪️An archive that enters from outside (admission, folder read-back, decode, parse, load): an import/admission path. */
+const POLICY_HOST_APPLY_TS_IMPORT_RE = /[Aa]dmission|ReadBack|\b(?:decode|parse|load|admit|verify)\w*Archive\w*\s*\(|\bloadAppDocumentArchive\b/;
+const POLICY_HOST_APPLY_TS_SEND_RE = /\bpostMessage\s*\(|\.send\s*\(/;
+const POLICY_HOST_APPLY_TS_RESTORE_RE = /^restore\w*Archive\w*$/;
+
+/** 🏠️Which language lane R17 reads `relPath` in: `rust` host/editor/apply/preparation/crate-root/framework-os files, `ts` for non-test shell sources, `null` otherwise. */
+function policyHostApplyScope(relPath: string): "rust" | "ts" | null {
+  const directories = relPath.split("/").slice(0, -1);
+  if (directories.some((segment) => POLICY_HOST_APPLY_EXCLUDED_DIRS.has(segment) || segment === "node_modules")) return null;
+  if (relPath.endsWith(".rs")) {
+    const host = directories.some((segment, index) => segment === "🔨️modules" && directories[index + 1] === "🏠️host");
+    return host || directories.some((segment) => POLICY_HOST_APPLY_LANE_DIRS.has(segment)) || relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX) || POLICY_HOST_APPLY_PLUGIN_ROOT_RE.test(relPath) ? "rust" : null;
+  }
+  if (/\.tsx?$/.test(relPath) && !/\.(?:test|spec)\.tsx?$/.test(relPath) && !relPath.endsWith(".d.ts")) return "ts";
+  return null;
+}
+
+/** 🧭️The files whose text resolves a helper's call sites: a plugin artifact's tree, a framework os module's tree, else the file's own directory. */
+export function policyHostApplyGroupKey(relPath: string): string {
+  const segments = relPath.split("/");
+  if (segments[0] === "✏️s" && segments[1] === "🔌️plugins" && segments[3] === "🗿️artifacts" && segments.length > 5) return segments.slice(0, 5).join("/");
+  const module = segments.indexOf("🔨️modules");
+  if (relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX) && module !== -1 && segments.length > module + 2) return segments.slice(0, module + 2).join("/");
+  return segments.slice(0, -1).join("/");
+}
+
+/** 🔬️One parsed Rust file for R17: comments, literals and `#[cfg(test)]` items blanked, plus its fns, impls and diff machinery. */
+type PolicyHostApplyRustFile = Readonly<{ relPath: string; allCode: string; code: string; fns: readonly PolicyRustFn[]; impls: readonly PolicyRustImpl[]; machinery: ReadonlySet<string>; exemptDirectory: boolean }>;
+
+function policyHostApplyParseRust(relPath: string, content: string): PolicyHostApplyRustFile {
+  const all = policyMaskRustSource(content);
+  const close = policyRustBraceClose(all);
+  const code = policyRustBlankTestItems(all, close);
+  const impls = policyRustImpls(code, close);
+  const fns = policyRustFns(code, close);
+  const machinery = new Set<string>();
+  const reach = (from: number, to: number) => {
+    for (const call of code.slice(from, to).matchAll(/\b([A-Za-z_]\w*)\s*\(/g)) {
+      if (machinery.has(call[1]!)) continue;
+      const callees = fns.filter((fn) => fn.name === call[1]);
+      if (callees.length === 0) continue;
+      machinery.add(call[1]!);
+      for (const callee of callees) reach(callee.bodyStart, callee.end);
+    }
+  };
+  for (const impl of impls) if (impl.kind === "diff") reach(impl.bodyStart, impl.end);
+  return { relPath, allCode: all, code, fns, impls, machinery, exemptDirectory: relPath.split("/").slice(0, -1).some((segment) => POLICY_HOST_APPLY_SITE_EXEMPT_DIRS.has(segment)) };
+}
+
+/**
+ * 🧮️Is a snapshot-writing helper exempt because EVERY call site of it, resolved textually within its group, sits in an exempt context
+ * — an `🚪️io`/`🔮️oracles`/`🧪️tests` file, a fn that builds a `LoadDocument`/`reset_document_effect`, a `MutationDiff` impl or the helpers it calls —
+ * or in a helper that is itself exempt by the same rule? A helper with no resolvable call site is NOT exempt (nothing proves where it runs).
+ */
+function policyHostApplyHelperExempt(name: string, files: ReadonlyMap<string, PolicyHostApplyRustFile>, depth: number, visiting: Set<string>): boolean {
+  const key = name;
+  if (depth > 6 || visiting.has(key)) return false;
+  visiting.add(key);
+  try {
+    const callPattern = new RegExp(String.raw`(?<!\bfn\s+)(?<![\w])${name}\s*(?:::<[^>]*>)?\s*\(`, "g");
+    let sites = 0;
+    for (const file of files.values()) {
+      for (const match of file.allCode.matchAll(callPattern)) {
+        sites++;
+        if (file.exemptDirectory || !/\S/.test(file.code.slice(match.index, match.index + match[0].length))) continue;
+        const enclosing = file.fns.filter((fn) => fn.start <= match.index && match.index <= fn.end).sort((a, b) => b.start - a.start)[0];
+        if (enclosing === undefined) return false;
+        if (enclosing.name === name) continue;
+        if (file.impls.some((impl) => impl.kind === "diff" && impl.bodyStart <= match.index && match.index <= impl.end) || file.machinery.has(enclosing.name)) continue;
+        if (POLICY_HOST_APPLY_LOAD_BUILDER_RE.test(file.code.slice(enclosing.bodyStart, enclosing.end + 1))) continue;
+        if (!policyHostApplyHelperExempt(enclosing.name, files, depth + 1, visiting)) return false;
+      }
+    }
+    return sites > 0;
+  } finally {
+    visiting.delete(key);
+  }
+}
+
+let policyHostApplyTypescript: typeof import("typescript") | undefined;
+
+/** 🔎️TypeScript R17: whole-archive puts and archive restores from handlers that are not import/admission/persistence paths, and snapshot refs written from a local apply. */
+function policyHostApplyTypescriptBreaches(relPath: string, content: string, add: (index: number, token: string, detail: string) => void): void {
+  if (!POLICY_HOST_APPLY_TS_PROBE_RE.test(content)) return;
+  const code = content.replace(/\/\*[\s\S]*?\*\/|\/\/[^\n]*/g, (comment) => comment.replace(/[^\n]/g, " "));
+  const locals = new Set<string>();
+  for (const match of code.matchAll(/\b(?:const|let)\s+(\w+)\s*(?::[^=;\n]+)?=\s*(?:await\s+)?(?:\w+\.)*apply(?:Local|Optimistic)\w*\s*\(/g)) locals.add(match[1]!);
+  for (const match of code.matchAll(POLICY_HOST_APPLY_TS_REF_ASSIGN_RE)) {
+    const rhs = match[2]!.trim();
+    if (POLICY_HOST_APPLY_TS_LOCAL_APPLY_RE.test(rhs) || locals.has(rhs.replace(/\W.*$/s, ""))) add(match.index, `${match[1]}.current = ${rhs.slice(0, 40)}`, "The shell writes the snapshot ref from a local apply instead of replacing it with the store's central-apply projection.");
+  }
+  if (!/localDocumentArchive|encodeDocumentArchiveBytes|restore\w*Archive/.test(code)) return;
+  policyHostApplyTypescript ??= createRequire(import.meta.url)("typescript") as typeof import("typescript");
+  const ts = policyHostApplyTypescript;
+  const source = ts.createSourceFile(relPath, content, ts.ScriptTarget.Latest, true, relPath.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
+  const isFunction = (node: import("typescript").Node): node is import("typescript").FunctionLikeDeclaration => ts.isArrowFunction(node) || ts.isFunctionExpression(node) || ts.isFunctionDeclaration(node) || ts.isMethodDeclaration(node);
+  const lastSegment = (text: string): string => text.replace(/\??\.current$/, "").split(/[.?]+/).at(-1) ?? "";
+  const declaredName = (fn: import("typescript").Node): string | null => {
+    let parent: import("typescript").Node | undefined = fn.parent;
+    while (parent !== undefined && (ts.isParenthesizedExpression(parent) || ts.isAsExpression(parent) || (ts.isCallExpression(parent) && /^use(?:Callback|Memo)$/.test(parent.expression.getText(source))))) parent = parent.parent;
+    if (parent === undefined) return null;
+    if (ts.isFunctionDeclaration(fn) || ts.isMethodDeclaration(fn)) return fn.name?.getText(source) ?? null;
+    if (ts.isVariableDeclaration(parent)) return parent.name.getText(source);
+    if (ts.isPropertyAssignment(parent)) return parent.name.getText(source);
+    if (ts.isBinaryExpression(parent) && parent.operatorToken.kind === ts.SyntaxKind.EqualsToken) return lastSegment(parent.left.getText(source));
+    return null;
+  };
+  const contextOf = (node: import("typescript").Node): import("typescript").Node => {
+    const hookCallback = (at: import("typescript").Node): boolean => ts.isCallExpression(at.parent) && /^use[A-Z]\w*$/.test(at.parent.expression.getText(source));
+    for (let at: import("typescript").Node | undefined = node.parent; at !== undefined; at = at.parent) if (isFunction(at) && (declaredName(at) !== null || hookCallback(at))) return at;
+    return source;
+  };
+  const isBranch = (node: import("typescript").Node): boolean => (ts.isIfStatement(node.parent) && (node.parent.thenStatement === node || node.parent.elseStatement === node)) || ts.isCaseClause(node) || ts.isDefaultClause(node);
+  const branchOf = (site: import("typescript").Node): import("typescript").Node | null => {
+    for (let at: import("typescript").Node | undefined = site.parent; at !== undefined && at !== source; at = at.parent) {
+      if (isFunction(at) && (declaredName(at) !== null)) return null;
+      if (isBranch(at)) return at;
+    }
+    return null;
+  };
+  const callSites = new Map<string, import("typescript").CallExpression[]>();
+  const walkCalls = (node: import("typescript").Node): void => {
+    if (ts.isCallExpression(node)) {
+      const name = lastSegment(node.expression.getText(source));
+      callSites.set(name, [...(callSites.get(name) ?? []), node]);
+    }
+    ts.forEachChild(node, walkCalls);
+  };
+  walkCalls(source);
+  const exempt = (context: import("typescript").Node, depth: number, visiting: Set<import("typescript").Node>): boolean => {
+    if (context === source) return false;
+    const text = context.getText(source);
+    if (POLICY_HOST_APPLY_TS_READ_BACK_RE.test(text) || POLICY_HOST_APPLY_TS_IMPORT_RE.test(text)) return true;
+    const name = isFunction(context) ? declaredName(context) : null;
+    if (name === null || depth > 6 || visiting.has(context)) return false;
+    const sites = callSites.get(name) ?? [];
+    visiting.add(context);
+    const all = sites.length > 0 && sites.every((site) => {
+      const branch = branchOf(site);
+      return branch === null ? exempt(contextOf(site), depth + 1, visiting) : POLICY_HOST_APPLY_TS_READ_BACK_RE.test(branch.getText(source)) || POLICY_HOST_APPLY_TS_IMPORT_RE.test(branch.getText(source));
+    });
+    visiting.delete(context);
+    return all;
+  };
+  const flag = (node: import("typescript").Node, token: string, detail: string): void => add(node.getStart(source), token, detail);
+  const putContexts = new Set<import("typescript").Node>();
+  const encodes: { node: import("typescript").Node; context: import("typescript").Node }[] = [];
+  const walk = (node: import("typescript").Node): void => {
+    if (ts.isPropertyAssignment(node) && node.name.getText(source) === "kind" && ts.isStringLiteralLike(node.initializer) && node.initializer.text === "localDocumentArchive") {
+      const context = contextOf(node);
+      if (!exempt(context, 0, new Set())) {
+        putContexts.add(context);
+        flag(node, 'kind: "localDocumentArchive"', "A whole-archive put from a handler that is not an import, admission or persistence path replaces the document without a mutation envelope, history or inverse; dispatch a mutation instead.");
+      }
+    }
+    if (ts.isCallExpression(node)) {
+      const callee = lastSegment(node.expression.getText(source));
+      const context = contextOf(node);
+      if (callee === "encodeDocumentArchiveBytes" && POLICY_HOST_APPLY_TS_SEND_RE.test(context.getText(source))) encodes.push({ node, context });
+      if (POLICY_HOST_APPLY_TS_RESTORE_RE.test(callee) && !(isFunction(context) && POLICY_HOST_APPLY_TS_RESTORE_RE.test(declaredName(context) ?? "")) && !exempt(context, 0, new Set()))
+        flag(node, `${callee}(…)`, "A revert path restores a captured whole archive over the user's edits; replay the recorded mutations' concrete inverses in reverse instead.");
+    }
+    ts.forEachChild(node, walk);
+  };
+  walk(source);
+  for (const { node, context } of encodes)
+    if (!putContexts.has(context) && !exempt(context, 0, new Set())) {
+      putContexts.add(context);
+      flag(node, "encodeDocumentArchiveBytes(…) sent", "A handler encodes a whole archive and sends it outside any import, admission or persistence path; dispatch a mutation instead.");
+    }
+}
+
+/**
+ * 🏠️Rule R17 over ONE source file, pure (L4/L5 beyond `🧬️schema`/`🧬️mutations`): a host, editor, apply or preparation lane turns a mutation into a
+ * snapshot only through `protocol::apply_diff`. Rust (`policyHostApplyScope`): a fn with a `&mut <X>Snapshot`/`&mut <X>Document` parameter that WRITES
+ * through it (assignment, `push`/`remove`/`insert`/… or `*_mut(` on a path rooted at it, `mem::replace|take|swap(&mut it…)`) — except `*it = …apply_diff(…)`, which is
+ * the central application itself —, any `snapshot_mut`, and a before/after differencing helper (`object_patch_diff(&before, &after)`, or a diff/delta/patch fn
+ * taking a `before` and an `after` reference). A writing helper is exempt only when it is `MutationDiff` machinery or EVERY call site of it, resolved textually within
+ * `group` (the files of its artifact/module tree; the file itself by default), sits in an exempt context — see `policyHostApplyHelperExempt`; there are no name-based exemptions.
+ * TypeScript: `<catalog|snapshot|document|archive>…Ref.current = applyLocal…(…)` (directly or through a local bound to it); a `kind: "localDocumentArchive"` put or an
+ * `encodeDocumentArchiveBytes` send, and a `restore…Archive(…)` revert call, in a declared function that does not read the archive back from the store, does not take it from an
+ * admission/decode/load, and whose every call site is not such a function either. `#[cfg(test)]` items, comments and literals never count; decoders live in `🚪️io` and are out of scope.
+ */
+export function policyHostApplyFileBreaches(relPath: string, content: string, group?: () => ReadonlyMap<string, string>): BreachRecord[] {
+  const scope = policyHostApplyScope(relPath);
+  if (scope === null) return [];
+  const breaches: BreachRecord[] = [];
+  const seen = new Set<number>();
+  const lineOf = policyRustLineLookup(content);
+  const add = (index: number, token: string, detail: string) => {
+    const line = lineOf(index);
+    if (seen.has(line)) return;
+    seen.add(line);
+    const breach = policyDiffOnlyBreach("R17", relPath, line, token.replace(/\s+/g, " ").trim());
+    breaches.push({ ...breach, reason: `${breach.reason} ${detail}` });
+  };
+  if (scope === "ts") {
+    policyHostApplyTypescriptBreaches(relPath, content, add);
+    return breaches;
+  }
+  const self = policyHostApplyParseRust(relPath, content);
+  const { code, fns, machinery } = self;
+  const framework = relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX);
+  const pending: { index: number; token: string; detail: string; fn: string }[] = [];
+  for (const match of code.matchAll(/\bsnapshot_mut\b/g)) add(match.index, "snapshot_mut", "A mutable snapshot escape hatch lets callers write fields outside the central applier.");
+  if (!framework) for (const match of code.matchAll(POLICY_HOST_APPLY_BEFORE_AFTER_CALL_RE)) add(match.index, `${match[1]}(&before, &after)`, "A before/after differencing helper builds the diff from two snapshots (V1-SNAPSHOT-DIFF).");
+  for (const fn of fns) {
+    const open = code.indexOf("(", fn.start);
+    if (open < 0 || open > fn.bodyStart) continue;
+    const params = policyRustSplitTop(code.slice(open + 1, policyRustParenEnd(code, open)));
+    if (/diff|delta|patch/.test(fn.name)) {
+      const refs = params.filter((param) => !framework || /Snapshot|Document/.test(param)).map((param) => /^(?:mut\s+)?(\w+)\s*:\s*&(?!\s*mut\b)/.exec(param)?.[1]).filter((name): name is string => name !== undefined);
+      if (refs.some((name) => POLICY_HOST_APPLY_BEFORE_NAME_RE.test(name)) && refs.some((name) => POLICY_HOST_APPLY_AFTER_NAME_RE.test(name))) add(fn.start, `fn ${fn.name}(before, after)`, "A before/after differencing helper builds the diff from two snapshots (V1-SNAPSHOT-DIFF).");
+    }
+    if (machinery.has(fn.name)) continue;
+    const body = code.slice(fn.bodyStart, fn.end + 1);
+    for (const param of params) {
+      const name = (framework ? POLICY_HOST_APPLY_MUT_SNAPSHOT_PARAM_RE : POLICY_HOST_APPLY_MUT_PARAM_RE).exec(param)?.[1];
+      if (name === undefined) continue;
+      const writes: { index: number; token: string }[] = [];
+      for (const match of body.matchAll(new RegExp(String.raw`(?<![\w.])(\*\s*)?${name}${POLICY_HOST_APPLY_PLACE}\s*(?:[-+*/|&^%]|<<|>>)?=(?![=>])`, "g"))) {
+        if (match[1] !== undefined && match[0].replace(/\s+/g, "") === `*${name}=`) {
+          const end = body.indexOf(";", match.index);
+          const statement = body.slice(match.index + match[0].length, end === -1 ? body.length : end);
+          if (/\bapply_diff\s*\(/.test(statement)) continue;
+          const bound = /^\s*\(?\s*(\w+)\s*\)?\s*$/.exec(statement)?.[1];
+          if (bound !== undefined && new RegExp(String.raw`\b(?:let|Ok\()[^;{}]*\b${bound}\b[^;{}]*?=[^;]*?\bapply_(?:diff|mutation|\w+_mutation\w*)\s*\(`).test(body.slice(0, match.index))) continue;
+        }
+        writes.push({ index: match.index, token: match[0] });
+      }
+      for (const match of body.matchAll(new RegExp(String.raw`(?<![\w.])${name}${POLICY_HOST_APPLY_PLACE}\s*\.\s*(?:${POLICY_HOST_APPLY_WRITE_METHODS})\s*\(`, "g"))) writes.push({ index: match.index, token: match[0] });
+      for (const match of body.matchAll(new RegExp(String.raw`\b(?:std::)?mem::(?:replace|take|swap)\s*\(\s*&\s*mut\s+\*?\s*${name}\b`, "g"))) writes.push({ index: match.index, token: match[0] });
+      for (const write of writes) pending.push({ index: fn.bodyStart + write.index, token: write.token, detail: `\`${fn.name}\` writes fields of its \`&mut\` snapshot parameter \`${name}\` instead of applying a diff through \`protocol::apply_diff\`.`, fn: fn.name });
+    }
+  }
+  if (pending.length > 0) {
+    const files = new Map<string, PolicyHostApplyRustFile>();
+    for (const [path, text] of group?.() ?? []) if (path !== relPath && path.endsWith(".rs")) files.set(path, policyHostApplyParseRust(path, text));
+    files.set(relPath, self);
+    const verdicts = new Map<string, boolean>();
+    for (const finding of pending) {
+      if (!verdicts.has(finding.fn)) verdicts.set(finding.fn, policyHostApplyHelperExempt(finding.fn, files, 0, new Set()));
+      if (!verdicts.get(finding.fn)) add(finding.index, finding.token, finding.detail);
+    }
+  }
+  return breaches;
+}
+
 /**
  * 🔬️Rules R8–R14 and R16 over ONE Rust file, pure. Comments, literals and `#[cfg(test)]` items never count.
@@ -813,9 +1094,11 @@

  * mutation side. Every `DiffAlgebra::inverse` body IS scanned. R9 additionally covers the whole
  * file under any `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` directory. R13's `impl MutationDiff<X> for X` is judged in
- * every non-test Rust file. R13 and R14 apply to every lane and mutation type, including presence and transient roots:
- * diffs remain sparse and inverses restore touched values through concrete mutations. R16 scans every Rust file.
+ * every non-test Rust file. R13 and R14 apply to every lane and mutation type; the single exemption is R14's
+ * `Self::…Snapshot` setter inside an `impl Mutation<…> for *TransientMutation|*PresenceMutation` (the ephemeral
+ * whole-root setter, inverted to itself carrying the base root — design.md "Ephemeral roots"): decided by the mutation's
+ * TYPE name, never by a path, and R13 stays strict there because the diff must remain sparse. R16 scans every Rust file.
  * The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function.
  */
-export function policyDiffOnlyFileBreaches(relPath: string, content: string): BreachRecord[] {
+function policyDiffOnlyLeafFileBreaches(relPath: string, content: string): BreachRecord[] {
   if (!relPath.endsWith(".rs")) return [];
   const directories = relPath.split("/").slice(0, -1);
@@ -880,5 +1163,5 @@

   for (const match of withoutUses.matchAll(POLICY_DIFF_ONLY_APPLY_RE))
     if (!within(appliers, match.index) && (applyEverywhere || within(scannedSpans, match.index) || within(mutationImpls, match.index))) add("R9", match.index, match[0]);
-  for (const match of code.matchAll(POLICY_DIFF_ONLY_BETWEEN_RE)) if (within(scannedSpans, match.index)) add("R10", match.index, match[0]);
+  for (const match of code.matchAll(POLICY_DIFF_ONLY_BETWEEN_RE)) if (within(scannedSpans, match.index) && !POLICY_DIFF_ONLY_GEOMETRY_BETWEEN_RE.test(match[0])) add("R10", match.index, match[0]);
   for (const fn of inverseFns) {
     const body = code.slice(fn.bodyStart, fn.end + 1);
@@ -892,5 +1175,8 @@

       }
     }
-    for (const match of body.matchAll(POLICY_DIFF_ONLY_RESTORE_RE)) add("R14", fn.bodyStart + match.index, match[0]);
+    const owner = innermost(fn.start);
+    const ephemeralSetter = owner?.kind === "mutation" && POLICY_DIFF_ONLY_EPHEMERAL_MUTATION_RE.test(policyRustTypeName(owner.self));
+    for (const match of body.matchAll(POLICY_DIFF_ONLY_RESTORE_RE))
+      if (!(ephemeralSetter && POLICY_DIFF_ONLY_EPHEMERAL_SETTER_RE.test(match[0]))) add("R14", fn.bodyStart + match.index, match[0]);
   }
   for (const fn of scanned) {
@@ -911,16 +1197,111 @@

 }
 
+/** 🔎️R18 tokens: the commit of an assembled mesh, the base-equality filter that sparsifies it, a kernel topology builder (`extrude_faces_job`, `bevel_job`, …) and the whole-arena difference oracles. */
+const POLICY_ROW_EMITTER_BANNED_RE = /\b(?:commit_built|equals_base|sparse_rows)\b|::produced\s*\(|\.(?:extrude_faces|inset_faces|bevel|loop_cut|subdivide_faces|triangulate|mirror|decimate|flip_faces|merge_vertices|dissolve_edges|snap_vertices)_job(?:_owned)?\s*\(/g;
+/** 🔎️Cheap probe: only files naming a banned token are parsed. */
+const POLICY_ROW_EMITTER_PROBE_RE = /commit_built|equals_base|sparse_rows|::produced|_job(?:_owned)?\s*\(/;
+
 /**
- * 🧪️Rule R15, pure: every direct mutation leaf (a directory below a `🧬️mutations` directory that owns a `🔺️diff` and no
- * `🧩️plan`) needs a Rust test under its OWN `🧪️tests` that calls the framework's `assert_mutation_inverse_sum_law`
- * (L3: the inverse diffs sum to the negative diff). `files` is the git-visible inventory, `read` returns a file's content.
+ * 🧱️Rule R18 over ONE source file, pure: a computational mesh gesture is a row emitter. In the non-test code of a `🧬️mutations` tree and
+ * of the mesh overlay (`🥽️mesh/📝️journal`) nothing may commit an assembled mesh, filter rows against the base to sparsify them, call a kernel topology
+ * builder or difference whole arenas: that is a before/after comparison (V1-SNAPSHOT-DIFF) however it is named.
+ */
+export function policyRowEmitterFileBreaches(relPath: string, content: string): BreachRecord[] {
+  if (!relPath.endsWith(".rs") || !POLICY_ROW_EMITTER_PROBE_RE.test(content)) return [];
+  const directories = relPath.split("/").slice(0, -1);
+  if (directories.some((segment) => segment === "🧪️tests" || segment === "🧫️fixtures")) return [];
+  const emitter = directories.includes(POLICY_MUTATIONS_FACET) || (directories.includes("🥽️mesh") && directories.includes("📝️journal"));
+  if (!emitter) return [];
+  const masked = policyMaskRustSource(content);
+  const code = policyRustBlankTestItems(masked, policyRustBraceClose(masked));
+  const lineOf = policyRustLineLookup(content);
+  const seen = new Set<number>();
+  const breaches: BreachRecord[] = [];
+  for (const match of code.matchAll(POLICY_ROW_EMITTER_BANNED_RE)) {
+    const line = lineOf(match.index);
+    if (seen.has(line)) continue;
+    seen.add(line);
+    breaches.push(policyDiffOnlyBreach("R18", relPath, line, match[0].replace(/\s+/g, " ").trim()));
+  }
+  return breaches;
+}
+
+/** 🔬️Rules R8–R14, R16 (leaf trees), R17 (host/editor apply lanes) and R18 (row emitters) over ONE source file, pure. The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function. */
+export function policyDiffOnlyFileBreaches(relPath: string, content: string, group?: () => ReadonlyMap<string, string>): BreachRecord[] {
+  return [...policyDiffOnlyLeafFileBreaches(relPath, content), ...policyHostApplyFileBreaches(relPath, content, group), ...policyRowEmitterFileBreaches(relPath, content)];
+}
+
+const POLICY_INVERSE_SUM_LAW_CALL = "assert_mutation_inverse_sum_law";
+const POLICY_LAW_PAYLOAD_IMPL_RE = /\bMutationKind\s*<[^;{]*?>\s*for\s+(\w+)/g;
+
+/** 🔎️The named fns of a Rust source as `{name, start, end}` spans over the braces of their bodies; comments, strings and chars are blanked first so a brace inside them never unbalances the scan. */
+function policyRustFnSpans(content: string): { name: string; start: number; end: number }[] {
+  const blanked = content.replace(/\/\/[^\n]*|\/\*[\s\S]*?\*\/|r#*"[\s\S]*?"#*|"(?:\\[\s\S]|[^"\\])*"|'(?:\\.|[^'\\])'/g, (hit) => " ".repeat(hit.length));
+  const spans: { name: string; start: number; end: number }[] = [];
+  for (const head of blanked.matchAll(/\bfn\s+(\w+)\b[^{;]*\{/g)) {
+    const start = head.index + head[0].length - 1;
+    let depth = 0;
+    let end = start;
+    for (; end < blanked.length; end++) {
+      const ch = blanked[end];
+      if (ch === "{") depth++;
+      else if (ch === "}" && --depth === 0) break;
+    }
+    spans.push({ name: head[1]!, start, end });
+  }
+  return spans;
+}
+
+/** 🔎️Everything a test source's sum-law fns can reach by name: the fns calling `assert_mutation_inverse_sum_law`, the same-file fns they call and the same-file fns calling them (two hops each way) — the text a family test names its variants in. Empty when the file never calls the law inside a fn. */
+function policyInverseSumLawReach(content: string): string {
+  if (!content.includes(POLICY_INVERSE_SUM_LAW_CALL)) return "";
+  const spans = policyRustFnSpans(content);
+  const bodyOf = (span: { start: number; end: number }): string => content.slice(span.start, span.end + 1);
+  const seeds = new Set(spans.filter((span) => bodyOf(span).includes(POLICY_INVERSE_SUM_LAW_CALL)).map((span) => span.name));
+  if (seeds.size === 0) return "";
+  const reached = new Set(seeds);
+  for (let hop = 0; hop < 2; hop++)
+    for (const span of spans) {
+      const body = bodyOf(span);
+      if (reached.has(span.name)) {
+        for (const call of body.matchAll(/\b(\w+)\s*\(/g)) if (spans.some((other) => other.name === call[1])) reached.add(call[1]!);
+      } else if ([...reached].some((name) => new RegExp(`\\b${name}\\s*\\(`).test(body))) reached.add(span.name);
+    }
+  return spans.filter((span) => reached.has(span.name)).map(bodyOf).join("\n");
+}
+
+/** 🗺️The tree a leaf's family tests live in: its `🪆️subsets/<subset>` directory when it has one (stdio standards), else the artifact root above its `🧬️mutations` directory. */
+function policyInverseSumLawFamilyScope(leaf: string): string {
+  const segments = leaf.split("/");
+  const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
+  const subsets = segments.indexOf("🪆️subsets");
+  if (subsets !== -1 && subsets + 1 < facet) return segments.slice(0, subsets + 2).join("/");
+  return policyArtifactRootOfMutationsDir(segments.slice(0, facet + 1).join("/"));
+}
+
+/**
+ * 🧪️Rule R15, pure: every direct mutation leaf needs the framework's `assert_mutation_inverse_sum_law` (L3: the inverse
+ * diffs sum to the negative diff). A leaf is a directory below a `🧬️mutations` directory that owns a `🔺️diff`, or a
+ * `🦀️.rs` that implements `MutationKind<..>` (stdio's leaf layout carries no `🔺️diff` directory); a leaf with a `🧩️plan`
+ * is composite and exempt. It is tested when a test under its OWN `🧪️tests` calls the law, or when a family test of its
+ * subset/artifact (any `🧪️tests` file) calls the law in a fn that NAMES the leaf — its `MutationKind` payload type, its enum
+ * variant (the Pascal-cased leaf slug) or its quoted kebab slug — directly or through same-file helper fns. A family test
+ * that calls the law without naming the leaf — a shared loop over cases defined elsewhere — does not count.
+ * `files` is the git-visible inventory, `read` returns a file's content.
  */
 export function policyInverseSumLawUntestedLeafBreaches(files: readonly string[], read: (relPath: string) => string): BreachRecord[] {
   const leaves = new Map<string, string>();
+  const payloads = new Map<string, string[]>();
   const composites = new Set<string>();
   const tested = new Set<string>();
+  const familyReach = new Map<string, string>();
   for (const relPath of files) {
     const segments = relPath.split("/");
     const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
+    const inTests = segments.includes("🧪️tests");
+    if (inTests && relPath.endsWith(".rs")) {
+      const reach = policyInverseSumLawReach(read(relPath));
+      if (reach) familyReach.set(relPath, reach);
+    }
     if (facet === -1) continue;
     const planAt = segments.indexOf(POLICY_MUTATION_PLAN_DIR, facet + 1);
@@ -928,5 +1309,5 @@

     const testsAt = segments.indexOf("🧪️tests", facet + 1);
     if (testsAt !== -1) {
-      if (relPath.endsWith(".rs") && read(relPath).includes("assert_mutation_inverse_sum_law")) tested.add(segments.slice(0, testsAt).join("/"));
+      if (relPath.endsWith(".rs") && read(relPath).includes(POLICY_INVERSE_SUM_LAW_CALL)) tested.add(segments.slice(0, testsAt).join("/"));
       continue;
     }
@@ -935,9 +1316,23 @@

       const leaf = segments.slice(0, diffAt).join("/");
       if (!leaves.has(leaf)) leaves.set(leaf, relPath);
-    }
-  }
+    } else if (segments.length === facet + 3 && segments[facet + 2] === POLICY_RS_COMPONENT_LEAF_NAME) {
+      const kinds = [...read(relPath).matchAll(POLICY_LAW_PAYLOAD_IMPL_RE)].map((hit) => hit[1]!);
+      if (kinds.length === 0) continue;
+      const leaf = segments.slice(0, facet + 2).join("/");
+      leaves.set(leaf, relPath);
+      payloads.set(leaf, kinds);
+    }
+  }
+  const named = (leaf: string): boolean => {
+    const slug = leaf.split("/").at(-1)!.replace(/^[^a-z0-9]+/u, "");
+    const variant = slug.split("-").map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("");
+    const names = [...(payloads.get(leaf) ?? []), variant];
+    const scope = `${policyInverseSumLawFamilyScope(leaf)}/`;
+    const texts = [...familyReach].filter(([testRel]) => testRel.startsWith(scope)).map(([, reach]) => reach);
+    return texts.some((text) => text.includes(`"${slug}"`) || names.some((name) => new RegExp(`\\b${name}\\b`).test(text)));
+  };
   return [...leaves]
-    .filter(([leaf]) => !composites.has(leaf) && !tested.has(leaf))
-    .map(([leaf, diffRel]) => policyDiffOnlyBreach("R15", diffRel, 1, `${leaf.split("/").at(-1)}/🧪️tests`));
+    .filter(([leaf]) => !composites.has(leaf) && !tested.has(leaf) && !named(leaf))
+    .map(([leaf, anchorRel]) => policyDiffOnlyBreach("R15", anchorRel, 1, `${leaf.split("/").at(-1)}/🧪️tests`));
 }
 
@@ -949,6 +1344,16 @@

   if (cached) return cached;
   const byKind = new Map<string, BreachRecord[]>();
-  for (const relPath of policyMutationLawRustFiles(repoRoot))
-    for (const breach of policyDiffOnlyFileBreaches(relPath, policyReadFileSafe(repoRoot, relPath))) {
+  const sources = policyMutationLawSourceFiles(repoRoot);
+  const groupFiles = new Map<string, string[]>();
+  for (const relPath of sources) if (relPath.endsWith(".rs")) groupFiles.set(policyHostApplyGroupKey(relPath), [...(groupFiles.get(policyHostApplyGroupKey(relPath)) ?? []), relPath]);
+  const groups = new Map<string, ReadonlyMap<string, string>>();
+  const groupOf = (relPath: string) => () => {
+    const key = policyHostApplyGroupKey(relPath);
+    let group = groups.get(key);
+    if (!group) groups.set(key, (group = new Map((groupFiles.get(key) ?? []).map((path) => [path, policyReadFileSafe(repoRoot, path)] as const))));
+    return group;
+  };
+  for (const relPath of sources)
+    for (const breach of policyDiffOnlyFileBreaches(relPath, policyReadFileSafe(repoRoot, relPath), groupOf(relPath))) {
       const rows = byKind.get(breach.kind);
       if (rows) rows.push(breach);
@@ -999,4 +1404,14 @@

 }
 
+/** 📏️Rule R17: host/editor/apply/preparation lanes and the shell change a snapshot only through `protocol::apply_diff` — no `&mut <X>Snapshot` field writes, `snapshot_mut`, before/after differencing or TypeScript local apply into a snapshot ref (L4/L5 beyond `🧬️schema`/`🧬️mutations`). */
+export function policyHostApplyBreaches(repoRoot: string): BreachRecord[] {
+  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R17.slug}`) ?? [];
+}
+
+/** 📏️Rule R18: computational mesh gestures are row emitters — no assembled-mesh commit, base-equality sparsifier, kernel topology builder or whole-arena difference in the non-test code of a `🧬️mutations` tree or the mesh overlay. */
+export function policyRowEmitterBreaches(repoRoot: string): BreachRecord[] {
+  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R18.slug}`) ?? [];
+}
+
 /** 📏️Rule R16: `MutationOutcome::apply_to` and `.apply_to(` are gone from every Rust file (L4). */
 export function policyOutcomeApplyToBreaches(repoRoot: string): BreachRecord[] {
@@ -1004,5 +1419,5 @@

 }
 
-/** ⚖️Aggregates this ticket's 7 mutation-outcome / merge-policy / no-CRDT / no-validate / derive-glue gates plus the nine diff-only mutation rules (R8–R16) over ONE shared inventory (`policyMutationLawInventory`) — the bundle both `policy` (below) and `VerifyScript.runGate`/`verify mutation-outcome-law` share. */
+/** ⚖️Aggregates this ticket's 7 mutation-outcome / merge-policy / no-CRDT / no-validate / derive-glue gates plus the eleven diff-only mutation rules (R8–R18) over ONE shared inventory (`policyMutationLawInventory`) — the bundle both `policy` (below) and `VerifyScript.runGate`/`verify mutation-outcome-law` share. */
 export function policyMutationOutcomeMergePolicyBreaches(repoRoot: string): BreachRecord[] {
   return [
@@ -1023,4 +1438,6 @@

     ...policyInverseSumLawUntestedBreaches(repoRoot),
     ...policyOutcomeApplyToBreaches(repoRoot),
+    ...policyHostApplyBreaches(repoRoot),
+    ...policyRowEmitterBreaches(repoRoot),
   ];
 }

### 58 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts
HEAD 530 BASE 532 STASH 532 CURRENT H
--- 

+++ 

@@ -512,5 +512,5 @@

     scope: drift.path,
     priority: "high",
-    reason: "Every agent and editor client file is derived from `metadata.semio.dashboard.tools[].mcp` of the root 📋️project.json; a hand edit or a stale copy makes the clients disagree.",
+    reason: "Every headless agent client file is derived from `metadata.semio.dashboard.tools[].mcp` of the root 📋️project.json; a hand edit or a stale copy makes the clients disagree.",
     solution: "Run `bun ./📜️script.ts agents write`.",
   }));

### 59 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts
HEAD 673 BASE 671 STASH 671 CURRENT H
--- 

+++ 

@@ -656,5 +656,5 @@

 
 //#region 🔧️PolicyRuleDerivedConfig
-/** 📝️ Derived-config rule: the devcontainer editor block, the nextest scopes and the dependabot directories equal their derivation.
+/** 📝️ Derived-config rule: nextest scopes and dependency update directories equal their derivation.
  * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts */
 export function policyDerivedConfigBreaches(repoRoot: string): BreachRecord[] {
@@ -666,5 +666,5 @@

     priority: "high" as const,
     reason: "A checked-in file that derives from one canonical source must equal its derivation, otherwise two copies drift and the editor, container and test runners disagree.",
-    solution: "Run `bun repo run workspace:generate-config` to rewrite the derived files; edit the canonical source (.vscode/settings.json, .vscode/extensions.json, .devcontainer/editor-overlay.json, .config/nextest.toml) instead of the copy.",
+    solution: "Run `bun repo run workspace:generate-config` to rewrite the derived files; edit the canonical source (.config/nextest.toml and its declared scope overrides) instead of the copy.",
   }));
 }

### 60 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts
HEAD 74 BASE 72 STASH 73 CURRENT H
--- 

+++ 

@@ -16,4 +16,5 @@

     runtimeChannels:readonly ViteRuntimeChannelV1[];
     config: string;
+    runtime?: "bun" | "node";
     artifactInputs?: readonly RepositoryArtifactRequestV1[];
     portEnv?: string;
@@ -68,5 +69,5 @@

   }
   const {serveViteCommandV1}=await import("../🟦️.ts");
-  await serveViteCommandV1({runtimeChannels,invocation,control,root:bundleRoot,config:resolve(bundleRoot,opts.config),mode:readBuildMode(spawnEnv.SEMIO_BUILD_MODE),host:selectedHost,port:selectedPort,strictPort,environment:spawnEnv,artifactInputs:opts.artifactInputs,signal:invocation.control.signal});
+  await serveViteCommandV1({runtimeChannels,invocation,control,root:bundleRoot,config:resolve(bundleRoot,opts.config),mode:readBuildMode(spawnEnv.SEMIO_BUILD_MODE),host:selectedHost,port:selectedPort,strictPort,environment:spawnEnv,artifactInputs:opts.artifactInputs,runtime:opts.runtime,signal:invocation.control.signal});
 }
 

### 61 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts
HEAD 187 BASE 186 STASH 186 CURRENT H
--- 

+++ 

@@ -89,12 +89,12 @@

 
 /** 🖥️ Describes one isolated compiler process without mutating another live service's environment. */
-export interface ViteCommandServiceV1{readonly runtimeChannels:readonly ViteRuntimeChannelV1[];readonly invocation:ScriptInvocation;readonly control:CompilerAdmissionControl;readonly root:string;readonly config:string;readonly mode:BuildMode;readonly host:string;readonly port:number;readonly signal:AbortSignal;readonly environment:Readonly<Record<string,string|undefined>>;readonly artifactInputs?:readonly RepositoryArtifactRequestV1[];readonly strictPort?:boolean}
+export interface ViteCommandServiceV1{readonly runtimeChannels:readonly ViteRuntimeChannelV1[];readonly invocation:ScriptInvocation;readonly control:CompilerAdmissionControl;readonly root:string;readonly config:string;readonly mode:BuildMode;readonly host:string;readonly port:number;readonly signal:AbortSignal;readonly environment:Readonly<Record<string,string|undefined>>;readonly artifactInputs?:readonly RepositoryArtifactRequestV1[];readonly strictPort?:boolean;readonly runtime?:"bun"|"node"}
 
 /** 🧵️ Holds paid compiler, exact input readers and bounded cache until its isolated Vite child joins. */
 export async function serveViteCommandV1(options:ViteCommandServiceV1):Promise<void>{
- requireIssuedResourceControlInvocationV1(options.control,options.invocation);const invocation=options.invocation,signal=options.signal,phase=createRequestedIssuedResourceControlV1(options.control,0,signal),mode=options.mode,root=options.root,config=options.config,host=options.host,port=options.port,strictPort=options.strictPort!==false,workspace=requireViteBuildIntentV1(invocation,mode),authority=readNativeCallerAuthorityV1(invocation.capabilities);if(!Number.isSafeInteger(port)||port<1||port>65535||!host||host.length>253||host.includes("\0"))throw Error("Explicit Vite command listener required");const owner="vite-command-service:"+root,artifactInputs=readViteArtifactInputsV1(options.artifactInputs??[]),runtimeChannels=readViteRuntimeChannelsV1(options.runtimeChannels),projection=projectViteCommandEnvironmentV1({version:1,mode,platform:process.platform,layers:[{...process.env},{...options.environment}]}),environment=projection.environment;
+ requireIssuedResourceControlInvocationV1(options.control,options.invocation);const invocation=options.invocation,signal=options.signal,phase=createRequestedIssuedResourceControlV1(options.control,0,signal),mode=options.mode,root=options.root,config=options.config,host=options.host,port=options.port,strictPort=options.strictPort!==false,runtime=options.runtime,workspace=requireViteBuildIntentV1(invocation,mode),authority=readNativeCallerAuthorityV1(invocation.capabilities);if(!Number.isSafeInteger(port)||port<1||port>65535||!host||host.length>253||host.includes("\0"))throw Error("Explicit Vite command listener required");const owner="vite-command-service:"+root,artifactInputs=readViteArtifactInputsV1(options.artifactInputs??[]),runtimeChannels=readViteRuntimeChannelsV1(options.runtimeChannels),projection=projectViteCommandEnvironmentV1({version:1,mode,platform:process.platform,layers:[{...process.env},{...options.environment}]}),environment=projection.environment;
  return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,phase,(_dependencies,dependencyPhase)=>withViteStorageV1(workspace,mode,owner,invocation,dependencyPhase,(store,layout,_temporary,storePhase)=>withViteArtifactInputsV1({workspace,invocation,store,control:storePhase,requests:artifactInputs},async control=>{
-  const require=createRequire(join(workspace,"package.json")),cli=join(dirname(require.resolve("vite/package.json")),"bin/vite.js"),state={root:layout.state,owner,compiler:`vite:${require("vite/package.json").version}:bun:${process.versions.bun}`,configuration:JSON.stringify([config,readFileSync(config,"utf8"),projection.digest]),mode};
-  await withCompilerState({...state,environment,control,storage:store.storage,storageControl:store.control,files:["deps/_metadata.json"]},async(directory,compilerEnvironment,control)=>{requireNativeStoragePathV1(store.root,directory);const child=viteCommandChildEnvironmentV1({version:1,mode,platform:process.platform,directory,environment:compilerEnvironment});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await mkdir(child.temporary,{recursive:true});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await runOwnedCommand(process.execPath,[cli,"--mode",mode,"--config",config,"--configLoader","runner","--host",host,"--port",String(port),...(strictPort?["--strictPort"]:[]),"--clearScreen","false"],root,owner,compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,{...child.environment,...viteRuntimeEnvironmentV1(runtimeChannels)}),signal:control.signal,output:authority.transport,onProgress:async()=>{await control.publish({resource:owner,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}});await store.recheck();});
+  const require=createRequire(join(workspace,"package.json")),cli=join(dirname(require.resolve("vite/package.json")),"bin/vite.js"),state={root:layout.state,owner,compiler:`vite:${require("vite/package.json").version}:${runtime??"caller"}:${process.version}`,configuration:JSON.stringify([config,readFileSync(config,"utf8"),projection.digest]),mode};
+  await withCompilerState({...state,environment,control,storage:store.storage,storageControl:store.control,files:["deps/_metadata.json"]},async(directory,compilerEnvironment,control)=>{requireNativeStoragePathV1(store.root,directory);const child=viteCommandChildEnvironmentV1({version:1,mode,platform:process.platform,directory,environment:compilerEnvironment});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await mkdir(child.temporary,{recursive:true});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await runOwnedCommand(runtime==="node"?"node":runtime==="bun"&&!process.versions.bun?"bun":process.execPath,[...(runtime==="node"?["--experimental-transform-types"]:[]),cli,"--mode",mode,"--config",config,"--configLoader",runtime==="node"?"bundle":"runner","--host",host,"--port",String(port),...(strictPort?["--strictPort"]:[]),"--clearScreen","false"],root,owner,compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,{...child.environment,...viteRuntimeEnvironmentV1(runtimeChannels)}),signal:control.signal,output:authority.transport,onProgress:async()=>{await control.publish({resource:owner,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}});await store.recheck();});
  })));
 }

### 62 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json
HEAD 1084 BASE 1083 STASH 1127 CURRENT H
--- 

+++ 

@@ -157,4 +157,24 @@

       "options": {
         "command": "bun ./📜️script.ts cache-prune"
+      }
+    },
+    "cache-prune-stale-incremental": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts cache-prune --stale-incremental-only --profile debug",
+        "cwd": "{projectRoot}"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "cache-prune",
+            "--stale-incremental-only",
+            "--profile",
+            "debug"
+          ]
+        }
       }
     },
@@ -1035,4 +1055,28 @@

         "prototype": {},
         "release": {}
+      }
+    },
+    "test-bootstrap-original-invocation-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts test bootstrap-original-invocation"
+      }
+    },
+    "test-playground-preparation-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts test playground-preparation-source"
+      }
+    },
+    "test-runtime-bootstrap": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts test runtime-bootstrap"
       }
     }

### 63 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts
HEAD 287 BASE 288 STASH 306 CURRENT H
--- 

+++ 

@@ -91,4 +91,16 @@

   }
   private async runTests(args:string[],environment:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl):Promise<void>{
+    if(args[0]==="playground-preparation-source"){
+      if(args.length!==1)throw Error("Expected test playground-preparation-source");
+      const source=join(SCRIPT_ROOT,"🧪️tests/🎮️playground-preparation/🧪️tests/🟦️.ts"),budget=scriptInvocationBudget(this.invocation,120000);
+      await this.runBunTests([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowJs","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",source],budget,environment,phaseControl);
+      await this.runBunTests(["test","--timeout",String(scriptInvocationBudget(this.invocation,budget)),source],scriptInvocationBudget(this.invocation,budget),environment,phaseControl);return;
+    }
+    if(args[0]==="bootstrap-original-invocation"){
+      if(args.length!==1)throw Error("Expected test bootstrap-original-invocation");
+      const source=join(SCRIPT_ROOT,"🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts"),budget=scriptInvocationBudget(this.invocation,120000);
+      await this.runBunTests([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],budget,environment,phaseControl);
+      await this.runBunTests(["test","--timeout",String(scriptInvocationBudget(this.invocation,budget)),source],scriptInvocationBudget(this.invocation,budget),environment,phaseControl);return;
+    }
     if(args[0]==="pruning"){
       if(args.length!==1)throw Error("Expected test pruning");
@@ -184,4 +196,10 @@

       mkdirSync(output, { recursive: true });
       await (await import("./🧪️tests/🔁️graph-revision/🟦️.ts")).testGraphRevision(this.repoRoot, output);
+      return;
+    }
+    if(args[0]==="runtime-bootstrap"){
+      if(args.length!==1)throw Error("Expected test runtime-bootstrap");
+      await (await import("./📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts")).testContainerRuntimeBootstrap(this.repoRoot,this.invocation);
+      console.log("[DEBUG] Current devcontainer runtime pins, official digests, setup prerequisites and framework controls validated");
       return;
     }

### 64 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts
HEAD 48 BASE 47 STASH 46 CURRENT H
--- 

+++ 

@@ -12,5 +12,4 @@

 import { cargoDirectories } from "../../🦀️cargo/🟦️.ts";
 import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
-import { repoCacheDirectory } from "../../🟦️.ts";
 import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
 import { buildCargoArtifacts, type CargoArtifactBuildPolicyV1, type CargoArtifactBuildOptionsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";

### 65 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts
HEAD 88 BASE 93 STASH 93 CURRENT H
--- 

+++ 

@@ -19,5 +19,5 @@

 /** 🧪️ Runs one artifact's suites across every Cargo package it is cut into (a split artifact family), in one invocation. */
 export async function runArtifactRustPackagesTests(cargoNames: readonly string[], repoRoot: string, segments: string[], invocation: ScriptInvocation): Promise<void> {
-  const { runRepositoryCargoTests } = await import("../../../🟦️.ts");
+  const { runRepositoryCargoTests } = await import("../../../🏃️process/🦀️cargo/🎟️admission/🟦️.ts");
   const { rest } = resolveTestLevel(segments);
   validateNativeCargoArguments("test", rest);
@@ -34,5 +34,5 @@

 export type ArtifactTwinSelfTest = { readonly name: string; readonly run: () => number };
 
-/** 📦️ Runs an independently owned Rust artifact package through the shared Nx-native contract. */
+/** 📦️ Runs an independently owned Rust artifact package through the shared native compiler contract. */
 export async function runArtifactRustPackageMain(packageRoot: string, cargoName: string, options: { readonly packages?: readonly string[]; readonly testPackages?: readonly string[]; readonly twins?: readonly ArtifactTwinSelfTest[]; readonly commands?: Readonly<Record<string, ScriptCommand>>; readonly testCommands?: Readonly<Record<string, ScriptCommand>>; readonly snapshotSqliteTests?: readonly string[]; readonly snapshotSqliteTestBudgetMs?: number; readonly snapshotSqliteTestGroups?: readonly (readonly string[])[] } = {}): Promise<void> {
   return receiveScriptProcessInvocation(process.env, async original => {
@@ -79,6 +79,6 @@

       if (mode !== "source") for (const testPackage of options.testPackages ?? [cargoName]) await runArtifactRustTests(testPackage, this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], this.invocation);
       if (mode !== "native") {
-        const { runRepositoryTestCommand } = await import("../../../🟦️.ts");
-        for (const group of snapshotGroups) await runRepositoryTestCommand(process.execPath, ["test", ...group], { cwd: this.repoRoot, budgetMs: scriptInvocationBudget(this.invocation, options.snapshotSqliteTestBudgetMs ?? 0), signal: this.invocation.control.signal });
+        const { runBudgetedTestCommand } = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts");
+        for (const group of snapshotGroups) await runBudgetedTestCommand(process.execPath, ["test", ...group], { cwd: this.repoRoot, budgetMs: scriptInvocationBudget(this.invocation, options.snapshotSqliteTestBudgetMs ?? 0), signal: this.invocation.control.signal });
       }
     }

### 66 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts
HEAD 622 BASE 622 STASH 625 CURRENT H
--- 

+++ 

@@ -190,11 +190,12 @@

   });
   const discoveryFinished=performance.now();
+  if(progress)console.log(`[nx] Validating current owner custody (${visited.length} directories, ${admitted.length} manifests)`);
   await verifyNxAdmissionDirectoriesV1(root,visited,admitted,current,signal);
-  for (const file of admitted) {
+  await boundedWalk(admitted, NX_ADMISSION_CONCURRENCY, async file => {
     signal?.throwIfAborted();
     await real(dirname(join(root, file.path)));
     const now = await lstat(join(root, file.path));
     if (!now.isFile() || now.isSymbolicLink() || now.dev !== file.device || now.ino !== file.inode) throw new Error("Nx current admission requires unchanged real manifest custody");
-  }
+  });
   const custodyFinished=performance.now();
   const begin = "# 🧫️ Canonical Example Collection Admission", end = "# 🧫️ End Example Collection Admission";
@@ -600,23 +601,25 @@

 }
 
+/** 📨️ Retains the inherited process deadline, policy and capabilities before dashboard or Nx dispatch. */
+export function nxBootstrapEnvelopeV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,now:number):ReturnType<typeof readScriptProcessEnvelope>{
+ const encoded=environment.SEMIO_SCRIPT_POLICY,capabilityWire=environment.SEMIO_SCRIPT_CAPABILITIES,inherited=environment[SCRIPT_PROCESS_INVOCATION_ENV];
+ const incoming=inherited===undefined?undefined:readScriptProcessEnvelope(JSON.parse(inherited),now);
+ const policy=encoded===undefined?incoming?.policy??readScriptPolicy({version:1,owner:"repository:nx",maximumElapsedMilliseconds:orchestratorBudgetMs()??0}):readScriptPolicy(JSON.parse(encoded));
+ const capabilities=incoming?capabilityWire===undefined?incoming.capabilities:JSON.parse(capabilityWire):capabilityWire===undefined?createNxBootstrapCapabilitiesV1(root,args,environment):readNxBootstrapAuthoringCapabilitiesV1(JSON.parse(capabilityWire));
+ const envelope=incoming??createScriptProcessEnvelope(policy,capabilities,now);
+ if(envelope.policy.version!==policy.version||envelope.policy.owner!==policy.owner||envelope.policy.maximumElapsedMilliseconds!==policy.maximumElapsedMilliseconds)throw Error("Original Nx process policy changed across handoff");
+ if(JSON.stringify(envelope.capabilities)!==JSON.stringify(capabilities))throw Error("Original Nx process capabilities changed across handoff");
+ return envelope;
+}
+
 if (import.meta.main) {
-  const args=process.argv.slice(2);
-    const encoded = process.env.SEMIO_SCRIPT_POLICY, capabilityWire = process.env.SEMIO_SCRIPT_CAPABILITIES, inherited = process.env[SCRIPT_PROCESS_INVOCATION_ENV];
-    const incoming = inherited === undefined ? undefined : readScriptProcessEnvelope(JSON.parse(inherited), Date.now());
-    const policy = encoded === undefined ? incoming?.policy ?? readScriptPolicy({ version: 1, owner: "repository:nx", maximumElapsedMilliseconds: orchestratorBudgetMs() ?? 0 }) : readScriptPolicy(JSON.parse(encoded));
-    const capabilities = incoming ? capabilityWire===undefined?incoming.capabilities:JSON.parse(capabilityWire) : capabilityWire===undefined?createNxBootstrapCapabilitiesV1(WORKSPACE_ROOT,args,process.env):readNxBootstrapAuthoringCapabilitiesV1(JSON.parse(capabilityWire));
-    const envelope = incoming ?? createScriptProcessEnvelope(policy, capabilities, Date.now());
-    if (envelope.policy.version !== policy.version || envelope.policy.owner !== policy.owner || envelope.policy.maximumElapsedMilliseconds !== policy.maximumElapsedMilliseconds) throw Error("Original Nx process policy changed across handoff");
-    if (JSON.stringify(envelope.capabilities) !== JSON.stringify(capabilities)) throw Error("Original Nx process capabilities changed across handoff");
-    process.env[SCRIPT_PROCESS_INVOCATION_ENV] = scriptProcessEnvironment(envelope, process.env)[SCRIPT_PROCESS_INVOCATION_ENV];
-  const native=await import("../../../🎛️dashboard/📦️installation/🟦️.ts"),selectedMode=resolveNxInvocation(args.slice(1)).env.SEMIO_BUILD_MODE,mode=readBuildMode((envelope.capabilities as {buildMode?:unknown}).buildMode??selectedMode??"prototype");
+ const args=process.argv.slice(2),inherited=process.env[SCRIPT_PROCESS_INVOCATION_ENV],envelope=nxBootstrapEnvelopeV1(WORKSPACE_ROOT,args,process.env,Date.now());
+ process.env[SCRIPT_PROCESS_INVOCATION_ENV]=scriptProcessEnvironment(envelope,process.env)[SCRIPT_PROCESS_INVOCATION_ENV];
+ try{await withScriptProcessEnvelope(envelope,async original=>{
+  const native=await import("../../../🎛️dashboard/📦️installation/🟦️.ts"),selectedMode=resolveNxInvocation(args.slice(1)).env.SEMIO_BUILD_MODE,mode=readBuildMode((original.capabilities as{buildMode?:unknown}).buildMode??selectedMode??"prototype");
   if(selectedMode!==undefined&&mode!==selectedMode)throw Error("Original dashboard build mode changed across handoff");
   const invocation=args[0]==="nx"&&native.dashboardInstalled(WORKSPACE_ROOT,mode)?native.dashboardInvocation(args.slice(1)):undefined;
-  if(invocation){try{await withScriptProcessEnvelope(envelope,async original=>{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation,{invocation:original,workspace:WORKSPACE_ROOT,mode,network:readNativeCallerAuthorityV1(original.capabilities).network,control:scriptCompilerAdmissionControlV1(original,"dashboard:launch",0)});});}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}}
-  else{
-    try { await withScriptProcessEnvelope(envelope, async original => {
-      publishBootstrapSources(WORKSPACE_ROOT);
-      await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(args, original);
-    }); } finally { if (inherited === undefined) delete process.env[SCRIPT_PROCESS_INVOCATION_ENV]; else process.env[SCRIPT_PROCESS_INVOCATION_ENV] = inherited; }
-  }
-}
+  if(invocation){try{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation,{invocation:original,workspace:WORKSPACE_ROOT,mode,network:readNativeCallerAuthorityV1(original.capabilities).network,control:scriptCompilerAdmissionControlV1(original,"dashboard:launch",0)});}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=original.control.signal.aborted?130:1;}}
+  else{publishBootstrapSources(WORKSPACE_ROOT);await new ScriptRouter(WORKSPACE_ROOT).register("nx",NxScript).run(args,original);}
+ });}catch(error){if(process.exitCode!==130)throw error;}finally{if(inherited===undefined)delete process.env[SCRIPT_PROCESS_INVOCATION_ENV];else process.env[SCRIPT_PROCESS_INVOCATION_ENV]=inherited;}
+}

### 67 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts
HEAD 233 BASE 233 STASH 273 CURRENT H
--- 

+++ 

@@ -2,9 +2,11 @@

 /** 📦️ Portable locked-install and owner-requested lock refresh command laws. */
 import { expect, test } from "bun:test";
-import { openSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
-import { resolve, join } from "node:path";
+import { openSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, renameSync, rmSync } from "node:fs";
+import { resolve, join, dirname, isAbsolute } from "node:path";
 import { pathToFileURL } from "node:url";
 import Ajv from "ajv";
-import { build } from "esbuild";
+import { build, transform } from "esbuild";
+import glob from "fast-glob";
+import * as bunOwnership from "../../../../🗂️workspaces/🟦️bun/🟦️.ts";
 import { createSourceFile, isFunctionDeclaration, parseConfigFileTextToJson, ScriptTarget } from "typescript";
 import {prepareJavascriptDependencies,javascriptDependencyArgumentsV1} from "../🟦️.ts";
@@ -19,4 +21,42 @@

 const owner = resolve(import.meta.dir, "..");
 const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { operations: { mode: "sync" | "lock"; arguments: string[] }[]; platforms: string[]; rejections: string[][] };
+
+/** 🧭️ Executes neutral output churn while retaining authored topology, manifest and cancellation fences. */
+async function outputCensus(root:string,input:any,mutation:string,api:typeof bunOwnership):Promise<any>{
+ const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);if(!isAbsolute(root)||dirname(root)!==base||dirname(root+"-retired")!==base)throw Error("Neutral census root escaped");
+ const put=(path:string,value:any):void=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),JSON.stringify(value));};
+ put("package.json",{name:"neutral-census",private:true,workspaces:["source/member"],semio:{workspace:{schemaVersion:1,members:input.members,owners:[]}}});
+ put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});
+ if(mutation!=="generatedBirth")for(const name of input.outputs)put("source/"+name+"/runtime/package.json",{name:"neutral-output-"+input.outputs.indexOf(name),version:"1.0.0"});
+ const controller=new AbortController(),visited:string[]=[];let changed=false;
+ try{const report=await api.publishBunWorkspaceOwnership(root,{signal:controller.signal,advance:async progress=>{
+  visited.push(progress.path);
+  const at=mutation==="cancel"||mutation==="authoredDirectory"?join(root,"source"):join(root,"source/member/package.json");
+  if(!changed&&progress.path===at&&progress.operation===(mutation==="cancel"||mutation==="authoredDirectory"?"list":"readText")){
+   changed=true;
+   if(mutation==="cancel")controller.abort(Error("Neutral census canceled"));
+   else if(mutation==="authoredManifest")put("source/member/package.json",{name:"neutral-changed",version:"1.0.0"});
+   else if(mutation==="authoredDirectory"){renameSync(join(root,"source/member"),join(root,"source/retired"));put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});}
+   else if(mutation==="authoredRoot"){renameSync(root,root+"-retired");put("package.json",{name:"neutral-census",private:true,workspaces:["source/member"],semio:{workspace:{schemaVersion:1,members:input.members,owners:[]}}});put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});}
+   else for(const name of input.outputs){const path=join(root,"source",name);if(dirname(path)!==join(root,"source"))throw Error("Neutral output path escaped");if(mutation==="generatedRemoval"){rmSync(path,{recursive:true,force:true});continue;}put("source/"+name+"/leases/late/package.json",{name:"neutral-late-output",version:"1.0.0"});mkdirSync(join(root,"source",name,"captures"),{recursive:true});}
+  }
+  await new Promise<void>(done=>setImmediate(done));
+ }});return {outcome:"admitted",packages:report.packages,visitedOutputs:input.outputs.filter((name:string)=>visited.some(path=>path===join(root,"source",name)||path.startsWith(join(root,"source",name)+"/"))) };}
+ catch(error){return {outcome:controller.signal.aborted?"canceled":"rejected",error:String(error),changed};}
+}
+
+test("Bun ownership excludes mutable outputs and preserves authored custody against Node and esbuild",async()=>{
+ const censusOwner=resolve(owner,"../../../🗂️workspaces/🟦️bun"),input=JSON.parse(readFileSync(join(censusOwner,"🧫️fixtures/🔣️.json"),"utf8")).outputCensus,schema=JSON.parse(readFileSync(join(censusOwner,"🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema.$defs.OutputCensus)(input)).toBe(true);
+ const source=createSourceFile("census.ts",readFileSync(import.meta.path,"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="outputCensus")!;
+ const compiled=await transform(operation.getText(source),{loader:"ts",format:"esm",target:"es2022"});
+ for(const row of input.cases){
+  const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"q")),actual=await outputCensus(root,input,row.mutation,bunOwnership);expect(actual.outcome,JSON.stringify(actual)).toBe(row.outcome);
+  if(row.outcome==="admitted"){expect(actual.packages).toEqual(["source/member"]);expect(actual.visitedOutputs).toEqual([]);expect(glob.sync("source/**/package.json",{cwd:root,ignore:input.outputs.map((name:string)=>"**/"+name+"/**"),onlyFiles:true}).map(path=>path.replace(/\/package.json$/u,""))).toEqual(actual.packages);}
+  else {expect(actual.changed).toBe(true);expect(actual.error).toContain(row.outcome==="canceled"?"Neutral census canceled":"Bun current");}
+  const oracleRoot=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"q")),entry='import {mkdirSync,writeFileSync,renameSync,rmSync} from "node:fs";import {join,dirname} from "node:path";import {createRequire} from "node:module";const api=createRequire(import.meta.url)('+JSON.stringify(join(censusOwner,"🟨️.cjs"))+');'+compiled.code+'console.log(JSON.stringify(await outputCensus('+JSON.stringify(oracleRoot)+','+JSON.stringify(input)+','+JSON.stringify(row.mutation)+',api)));';
+  const child=Bun.spawn(["node","--input-type=module","-e",entry.replace('import {join,dirname}', 'import {join,dirname,resolve,isAbsolute}')],{stdout:"pipe",stderr:"pipe",signal:new AbortController().signal}),[stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toBe(0);const reference=JSON.parse(stdout);expect(reference.outcome).toBe(row.outcome);if(row.outcome==="admitted")expect(reference).toEqual(actual);else expect(reference.changed).toBe(true);
+ }
+ console.log("[DEBUG] Bun output census and independent esbuild/Node/fast-glob agree: generated birth/churn/removal admitted; authored manifest/directory/root drift and original cancellation refused");
+});
 
 test("source watchers have stable independent graph stores and sockets for each activation binding", () => {
@@ -81,5 +121,5 @@

   expect(new Ajv({ strict: true }).validate(JSON.parse(readFileSync(join(bunOwner, "🧬️schema/🔣️.json"), "utf8")), bunManifest)).toBe(true);
   for (const platform of ["win32", "darwin", "linux"]) for (const arch of ["x64", "arm64"]) for (const musl of platform === "linux" ? [false, true] : [false]) {
-    const distribution = bunDistribution("1.3.14", platform, arch, musl);
+    const distribution = bunDistribution("1.4.2", platform, arch, musl);
     const member = distribution.archive.slice(0, -4) + (platform === "win32" ? "/bun.exe" : "/bun");
     const archive = zipSync({ [member]: new TextEncoder().encode("pinned executable"), "ignored.txt": new Uint8Array([1, 2]) });
@@ -88,6 +128,6 @@

     expect(distribution.sha256).toMatch(/^[a-f0-9]{64}$/);
   }
-  expect(() => bunDistribution("1.4.2", "win32", "x64", false)).toThrow();
-  expect(() => bunDistribution("1.3.14", "freebsd", "x64", false)).toThrow();
+  expect(() => bunDistribution("1.3.14", "win32", "x64", false)).toThrow();
+  expect(() => bunDistribution("1.4.2", "freebsd", "x64", false)).toThrow();
   expect(() => bunArchiveExecutable(Buffer.from("invalid"), "bun")).toThrow();
 });

### 68 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json
HEAD 51 BASE 16 STASH 16 CURRENT H
--- 

+++ 

@@ -2,5 +2,5 @@

   "name": "@semio-tech/nx-bootstrap-tools",
   "private": true,
-  "packageManager": "bun@1.3.14",
+  "packageManager": "bun@1.4.2",
   "dependencies": {
     "nx": "23.2.0",

### 69 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🔣️.json
HEAD 0 BASE 14 STASH 14 CURRENT H
--- 

+++ 

@@ -1,14 +1,14 @@

 {
-  "version": "1.3.14",
-  "release": "https://github.com/oven-sh/bun/releases/download/bun-v1.3.14",
+  "version": "1.4.2",
+  "release": "https://github.com/oven-sh/bun/releases/download/bun-v1.4.2",
   "distributions": [
-    { "platform": "win32", "arch": "x64", "musl": false, "archive": "bun-windows-x64-baseline.zip", "bytes": 38023440, "sha256": "538f9c846355d9e847b2671bc00c47da4229a0befb24df3282b739770f3b475f" },
-    { "platform": "win32", "arch": "arm64", "musl": false, "archive": "bun-windows-aarch64.zip", "bytes": 36659109, "sha256": "89841f5a57f2348b67ec0839b718f4bf4ea7d07c371c9ba4b77b6c790f918953" },
-    { "platform": "darwin", "arch": "x64", "musl": false, "archive": "bun-darwin-x64-baseline.zip", "bytes": 26509145, "sha256": "3e35ad6f53971a9834bf9e6786e2adf72b5f1921cc9a9c5fde073d2972944076" },
-    { "platform": "darwin", "arch": "arm64", "musl": false, "archive": "bun-darwin-aarch64.zip", "bytes": 23586433, "sha256": "d8b96221828ad6f97ac7ac0ab7e95872341af763001e8803e8267652c2652620" },
-    { "platform": "linux", "arch": "x64", "musl": false, "archive": "bun-linux-x64-baseline.zip", "bytes": 35595658, "sha256": "a063908ae08b7852ca10939bbdc6ceed3ddabce8fb9402dce83d65d73b36e6c7" },
-    { "platform": "linux", "arch": "arm64", "musl": false, "archive": "bun-linux-aarch64.zip", "bytes": 35700603, "sha256": "a27ffb63a8310375836e0d6f668ae17fa8d8d18b88c37c821c65331973a19a3b" },
-    { "platform": "linux", "arch": "x64", "musl": true, "archive": "bun-linux-x64-musl-baseline.zip", "bytes": 34384861, "sha256": "56a7d6806cf155536c0178f0ea5fbd098e684fa509ebdb4fc0a7e19fb65382dc" },
-    { "platform": "linux", "arch": "arm64", "musl": true, "archive": "bun-linux-aarch64-musl.zip", "bytes": 34304457, "sha256": "b98e0ad3625c5c00d1d5b5ff55605c7adddbfae151861e68ade57b2d3b8703bb" }
+    { "platform": "win32", "arch": "x64", "musl": false, "archive": "bun-windows-x64-baseline.zip", "bytes": 39807490, "sha256": "78c221c2376f79731ccf4e4af0b3bb46d81fefa3296c5abee09ad8a1b21e68c6" },
+    { "platform": "win32", "arch": "arm64", "musl": false, "archive": "bun-windows-aarch64.zip", "bytes": 36330062, "sha256": "a7a16b876a305fd1029c66dbd27007b4f6112ae896532f675878731a21e50cfd" },
+    { "platform": "darwin", "arch": "x64", "musl": false, "archive": "bun-darwin-x64-baseline.zip", "bytes": 28440543, "sha256": "bad5bbd6cf14d0980d115f5954c9ff904df619d5e994d2da1ffccd3f316300b0" },
+    { "platform": "darwin", "arch": "arm64", "musl": false, "archive": "bun-darwin-aarch64.zip", "bytes": 25377591, "sha256": "90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f" },
+    { "platform": "linux", "arch": "x64", "musl": false, "archive": "bun-linux-x64-baseline.zip", "bytes": 36646949, "sha256": "c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f" },
+    { "platform": "linux", "arch": "arm64", "musl": false, "archive": "bun-linux-aarch64.zip", "bytes": 36602920, "sha256": "54328bbc2d9c8e0c9f892c544d66c57a83b84139e34909e5ee81758f1ac8fda7" },
+    { "platform": "linux", "arch": "x64", "musl": true, "archive": "bun-linux-x64-musl-baseline.zip", "bytes": 34100976, "sha256": "76e1db84e98f22f78de0a87e309bfbbf297732847f9720db36750646c85c8c18" },
+    { "platform": "linux", "arch": "arm64", "musl": true, "archive": "bun-linux-aarch64-musl.zip", "bytes": 33975245, "sha256": "71760b6c8ea30623b81a4907cb815d48e2ea266f2e73e751534a44a0607950df" }
   ]
 }

### 70 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts
HEAD 607 BASE 1054 STASH 1054 CURRENT H
--- 

+++ 

@@ -77,5 +77,5 @@

   await testCiBaselineCommand(workspace, output);
   testDevcontainerContext(workspace);
-  testContainerRuntimeBootstrap(workspace,join(workspace,(await (await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs")).cacheInternals.workspaceCatalogBuildContributionsV1(workspace))[0].publication.outputRoot));
+  await testContainerRuntimeBootstrap(workspace,invocation);
   await testDevcontainerLifecycle(workspace, output);
   await testDerivedConfig(workspace);
@@ -809,5 +809,5 @@

     assert.equal(dotnet!.targets.deps.cache, false);
     assert.ok(workspace.targets["deps-dotnet"].dependsOn.includes(`${bootstrap.dotnetProject}:deps`));
-    assert.ok(readFileSync(join(root, "Monorepo.sln"), "utf8").includes(bootstrap.dotnetPath.replaceAll("/", "\\")));
+    assert.ok(existsSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj")), "The .NET owner builds its explicit compiler project without an IDE solution");
     assert.ok(readFileSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj"), "utf8").includes(`Include="${bootstrap.compile}"`));
     assert.ok(existsSync(join(root, bootstrap.dotnetPath, bootstrap.compile)), "The .NET compile item must resolve to its current source file");
@@ -846,6 +846,6 @@

     const { testNxCoordinator } = await import("../🛑️coordinator/🟦️.ts");
     await testNxCoordinator(root);
-    const invocationNode = nxSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
-    const route = ts.transpileModule(invocationNode.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
+    const invocationFunction = nxSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
+    const route = ts.transpileModule(invocationFunction.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
     const resolveInvocation = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
     const resolvePrint = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
@@ -1046,5 +1046,5 @@

     const osDevProject = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { dependsOn?: readonly string[]; options?: { command?: string } }> };
     assert.equal(osDevProject.targets.dev, undefined, "the framework-os-dev dev alias is dissolved: playgrounds are the generated dev-<variant>-<renderer>-<profile> targets");
-    assert.ok(!invocationNode.getText(nxSource).includes("PlaygroundSelections") && !invocationNode.getText(nxSource).includes("PlaygroundCatalog"), "the bootstrap resolves no playground variant");
+    assert.ok(!invocationFunction.getText(nxSource).includes("PlaygroundSelections") && !invocationFunction.getText(nxSource).includes("PlaygroundCatalog"), "the bootstrap resolves no playground variant");
     assert.equal(root.length > 0, true);
     rmSync(fixture, { recursive: true });

### 71 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts
HEAD 61 BASE 62 STASH 62 CURRENT H
--- 

+++ 

@@ -43,5 +43,5 @@

 `);
     const launch = (suffix: string) => {
-      const control=createRequestedIssuedResourceControlV1(childPhase,60000),env=scriptInvocationEnvironment(invocation,{...process.env,...environment,NX_SKIP_NX_CACHE:"false",NX_DISABLE_NX_CACHE:"false",NX_SKIP_REMOTE_CACHE:"true",NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false",NX_FORCE_REUSE_CACHED_GRAPH:"false"});delete env.SEMIO_BUILD_GRAPH_PATH;delete env.SEMIO_BUILD_GRAPH_DIGEST;delete env.SEMIO_BUILD_REUSE_GRAPH;
+      const control=createRequestedIssuedResourceControlV1(childPhase,60000),env:NodeJS.ProcessEnv=scriptInvocationEnvironment(invocation,{...process.env,...environment,NX_SKIP_NX_CACHE:"false",NX_DISABLE_NX_CACHE:"false",NX_SKIP_REMOTE_CACHE:"true",NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false",NX_FORCE_REUSE_CACHED_GRAPH:"false"});delete env.SEMIO_BUILD_GRAPH_PATH;delete env.SEMIO_BUILD_GRAPH_DIGEST;delete env.SEMIO_BUILD_REUSE_GRAPH;
       let log="",finished=false;const remaining=checkResourceLeaseControl(control),done=(async()=>{let failed=false,failure:unknown;try{await runOwnedCommand("node",[cli,"run","fixture:build","--output-style=stream","--verbose"],root,"test:nx-source-custody",Math.min(60000,Math.floor(remaining??60000)),{env,signal:control.signal,stdout:"ignore",output:{maximumBytes:8388608,maximumLines:65536},onLine:line=>{log+=line+"\n";},onProgress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"test:nx-source-custody",mode:"shared",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}});}catch(error){failed=true;failure=error;}try{writeFileSync(join(output,`source-custody-${suffix}.log`),log);}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;})().finally(()=>{finished=true;});pending.add(done);void done.then(()=>pending.delete(done),()=>pending.delete(done));void done.catch(()=>{});return{done,finished:()=>finished,output:()=>log};
     };

### 72 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts
HEAD 265 BASE 268 STASH 276 CURRENT H
--- 

+++ 

@@ -1,2 +1,3 @@

+import fixture from "../🧫️fixtures/nx-contract/🔣️.json";
 import {testOwnedFixtureWireBindingsV1,testNxCustodyInputsModelV1} from "./📥️contracts/🟦️.ts";
 import {scriptInvocationEnvironment,createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -29,4 +30,11 @@

 import {toolPatchReceipt} from "../../🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts";
 
+/** 🔒️ The actual lock fields compared against independent tooling manifests. */
+type ObservedBunLock={workspaces:Record<string,{name?:string;dependencies?:Record<string,string>}>;patchedDependencies?:Record<string,string>};
+/** 🧷️ Narrows only a declared environment input before reading its selected inventory. */
+const isEnvironmentInput=(input:unknown):input is {env:string}=>input!==null&&typeof input==="object"&&"env"in input&&typeof input.env==="string";
+/** 📁️ Narrows a native fileset while preserving every other input kind separately. */
+const inputFileset=(input:unknown):string|undefined=>typeof input==="string"?input:input!==null&&typeof input==="object"&&"fileset"in input&&typeof input.fileset==="string"?input.fileset:undefined;
+
 /** 🧪️ Portable mode selections agree with the installed Nx task graph and output resolver. */
 export async function testNxBuildModes(workspace: string, output: string|undefined, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
@@ -34,5 +42,5 @@

   output = output === undefined ? undefined : resolve(workspace, output);
   const parse = createRequire(resolve(workspace, ".nx/installation/package.json"))("yargs-parser");
-  const scheduling = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).schedulerSelections;
+  const scheduling = fixture.schedulerSelections;
   const parallelOracle=createRequire(resolve(workspace,".nx/installation/package.json"))("nx/src/command-line/yargs-utils/shared-options").readParallelFromArgsAndEnv;
   for (const row of scheduling) {
@@ -40,7 +48,7 @@

     try{process.env.NX_PARALLEL=environment.NX_PARALLEL;const head=selected.args.slice(0,selected.args.indexOf("--")<0?undefined:selected.args.indexOf("--"));assert.equal(parallelOracle(parse(head)),row.expectedParallel);}finally{if(prior===undefined)delete process.env.NX_PARALLEL;else process.env.NX_PARALLEL=prior;}
   }
-  const tooling=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingInstall;
+  const tooling=fixture.toolingInstall;
   assert.deepEqual(nxToolingInstallManifestV1(tooling.recipe),tooling.expected);
-  const toolingIdentity = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingIdentity, toolingFiles = new Map(Object.entries(toolingIdentity.files).map(([path, bytes]) => [path, Buffer.from(String(bytes))]));
+  const toolingIdentity = fixture.toolingIdentity, toolingFiles = new Map(Object.entries(toolingIdentity.files).map(([path, bytes]) => [path, Buffer.from(String(bytes))]));
   const toolingOracle = new Bun.CryptoHasher("sha256").update(`nx-tooling-install-v3\0${toolingIdentity.environment.platform}\0${toolingIdentity.environment.architecture}\0${toolingIdentity.environment.libc}\0`);
   for (const [path, bytes] of toolingFiles) toolingOracle.update(path + "\0").update(bytes).update("\0");
@@ -50,5 +58,5 @@

   const installed=nxToolingInstallManifestV1(recipe);
   assert.deepEqual(installed.dependencies,testRecordV1(frozen,"workspaces","").dependencies);assert.equal(installed.name,testRecordV1(frozen,"workspaces","").name);assert.deepEqual(installed.patchedDependencies,frozen.patchedDependencies);assert.ok(!("devDependencies" in installed));
-  const cases = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).buildModes;
+  const cases = fixture.buildModes;
   for (const row of cases) {
     const invoke = () => resolveNxBuildMode({ args: row.args, env: {} }, row.environment ?? {});
@@ -57,5 +65,5 @@

   }
   const Ajv = createRequire(resolve(workspace, "package.json"))("ajv/dist/2020").default, ajv = new Ajv({ strict: false });
-  const cacheBudgetSchema=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/💾️budget/🔣️schema.json",import.meta.url),"utf8")),cacheBudgetValid=ajv.compile(cacheBudgetSchema),cacheBudgets=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).cacheBudgets;
+  const cacheBudgetSchema=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/💾️budget/🔣️schema.json",import.meta.url),"utf8")),cacheBudgetValid=ajv.compile(cacheBudgetSchema),cacheBudgets=fixture.cacheBudgets;
   const cacheSource=readFileSync(createRequire(resolve(workspace,".nx/installation/package.json")).resolve("nx/src/tasks-runner/cache.js"),"utf8"),cacheParser=cacheSource.slice(cacheSource.indexOf("function parseMaxCacheSize("),cacheSource.indexOf("function formatCacheSize(")),cacheOracle=new Function(cacheParser+";return parseMaxCacheSize;")();
   for(const row of cacheBudgets){const input={availableBytes:row.availableBytes,...(row.override===undefined?{}:{override:row.override})};assert.ok(cacheBudgetValid(input));const selected=nxCacheBudgetV1(row.availableBytes,row.override);assert.equal(selected,row.expected);assert.equal(cacheOracle(selected),cacheOracle(row.expected));}
@@ -66,13 +74,13 @@

   assert.ok(lockValid({ version: 1, manifest: "owner/Cargo.toml" }));
   for (const path of ["../escape", "/absolute", "owner/../escape", "C:/absolute", "owner\\source", ""]) { assert.equal(sourceValid({ version: 1, files: [path] }), false); assert.equal(lockValid({ version: 1, manifest: path }), false); }
-  const identity = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).inputIdentity;
+  const identity = fixture.inputIdentity;
   const compact = deduplicateProjectInputs(identity.inputs, identity.root);
   assert.deepEqual(compact, identity.expected);
   const minimatch = createRequire(resolve(workspace, "package.json"))("minimatch").minimatch;
-  const sourceInputCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sourceInputCompaction;
+  const sourceInputCase = fixture.sourceInputCompaction;
   const sourceInputs = compactSourceInputs(sourceInputCase.inputs, workspace, sourceInputCase.root);
   assert.ok(!sourceInputs.some(input => typeof input === "string" && sourceInputCase.inputs.includes(input) && !input.startsWith("!")), "Excluded exact paths must leave the native plan together with compacted paths");
   assert.equal(sourceInputs.filter(input => typeof input === "string" && input.startsWith("!")).length, sourceInputCase.expectedNegations, "An exact digest must not retain negations after their entire positive source scope was consumed");
-  const sourceInputEnv = sourceInputs.find((input):input is Record<string,unknown>=>isTestRecordV1(input)&&typeof input.env==="string"&&input.env.startsWith("SEMIO_SOURCE_CLOSURE_"));
+  const sourceInputEnv = sourceInputs.filter(isEnvironmentInput).find(input=>input.env.startsWith("SEMIO_SOURCE_CLOSURE_"));
   assert(sourceInputEnv&&typeof sourceInputEnv.env==="string");
   const sourceInputInventory = JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"), sourceInputEnv.env.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json"), "utf8"));
@@ -82,9 +90,9 @@

   const accepts = (inputs: unknown[], path: string) => inputs.filter((input): input is string => typeof input === "string").some(input => !input.startsWith("!") && minimatch(path, input.replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", ""))) && !inputs.filter((input): input is string => typeof input === "string").some(input => input.startsWith("!") && minimatch(path, input.slice(1).replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", "")));
   for (const path of ["owner/package/src/main.ts", "owner/package/dist/result.js", "other/main.ts"]) assert.equal(accepts(compact, path), accepts(identity.inputs, path), "Canonical input identity must preserve independent minimatch membership");
-  const locked = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).cargoLockClosure;
+  const locked = fixture.cargoLockClosure;
   const projection = cargoLockClosure({ version: 4, package: locked.packages }, locked.roots);
   assert.deepEqual(projection.packages.map(cargoLockIdentity), locked.expected);
   const graphlib = createRequire(resolve(workspace, "package.json"))("graphlib"), dependencyGraph = new graphlib.Graph({ directed: true });
-  const planCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).taskPlanCompaction, compactPlans = compactTaskPlans(planCase.plans), planGraph = new graphlib.Graph({ directed: true });
+  const planCase = fixture.taskPlanCompaction, compactPlans = compactTaskPlans(planCase.plans), planGraph = new graphlib.Graph({ directed: true });
   assert.ok(ajv.compile(JSON.parse(readFileSync(new URL("../../🕸️graph/📤️export/🔣️schema.json", import.meta.url), "utf8")))(compactPlans));
   assert.deepEqual(compactPlans.instructions, planCase.instructions);
@@ -98,12 +106,12 @@

   const independent = graphlib.alg.preorder(dependencyGraph, [cargoLockIdentity(locked.packages[0])]).sort();
   assert.deepEqual(projection.packages.map(cargoLockIdentity), independent, "Cargo lock scope must match independent graphlib reachability");
-  const unrelated = structuredClone(locked.packages); unrelated.at(-1).version = "9.0.0";
+  const unrelated = structuredClone(locked.packages),last=unrelated.at(-1);assert.ok(last);last.version = "9.0.0";
   assert.deepEqual(cargoLockClosure({ version: 4, package: unrelated }, locked.roots), projection);
   const changed = structuredClone(locked.packages); changed[2].checksum = "changed";
   assert.notDeepEqual(cargoLockClosure({ version: 4, package: changed }, locked.roots), projection);
   assert.throws(() => cargoLockClosure({ version: 4, package: [...locked.packages, { name: "local", version: "2.0.0" }] }, locked.roots), /Ambiguous/);
-  const scopes = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).outputInputScope;
+  const scopes = fixture.outputInputScope;
   for (const row of scopes.cases) assert.equal(sourceOutputOverlaps(row.output, scopes.sources, scopes.root), row.overlap);
-  const repeated = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sharedInputGroups, shared = internProjectInputGroups(repeated);
+  const repeated = fixture.sharedInputGroups, shared = internProjectInputGroups(repeated);
   const nxHasher = createRequire(resolve(workspace, ".nx/installation/package.json"))("nx/src/hasher/task-hasher");
   assert.ok(JSON.stringify(shared).length < JSON.stringify(repeated).length + 128);
@@ -115,5 +123,5 @@

   const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph");
   const { getOutputs } = require("nx/src/tasks-runner/utils");
-  const testOwnerCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).testOwnerModeOutputs;
+  const testOwnerCase = fixture.testOwnerModeOutputs;
   const testOwner = testRecordV1(testMethodV1(testOwnerInternals,"target","testing","cases owner",[],true,testOwnerCase.scope));
   for (const mode of ["prototype", "release"]) {
@@ -141,5 +149,5 @@

   assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["native-build-release"], "prototype", {}), /Conflicting build mode/);
   assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["build"], "dev", {}), /prototype or release/);
-  const dependencyCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).commandDependencyOracle;
+  const dependencyCase = fixture.commandDependencyOracle;
   const entry = dependencyCase.entry;
   const commandSources = (await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs")).createScriptInputCache(), commandTarget = { options: { command: `bun ${JSON.stringify(entry)} nx` } };
@@ -151,5 +159,5 @@

   const patches = Object.fromEntries(Object.values(lock.patchedDependencies ?? {}).map(path => [path, readFileSync(resolve(workspace, String(path)))]));
   const model = testRecordV1(testMethodV1(cacheInternals,"bunLockGraph",lock,patches)), expected = new Set<string>(),externalNodes=new Set(Object.keys(testRecordV1(model,"externalNodes")));
-  const oracle = await createRequire(resolve(workspace, "package.json"))("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
+  const oracle:{metafile:import("esbuild").Metafile} = await createRequire(resolve(workspace, "package.json"))("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
   for (const [file, input] of Object.entries(oracle.metafile.inputs)) {const imports=testRecordV1(input).imports;assert(Array.isArray(imports));for (const dependency of imports) {
     if (!dependency.external || dependency.path.startsWith(".") || dependency.path.startsWith("/") || isBuiltin(dependency.path) || dependency.path.startsWith("bun:")) continue;
@@ -162,9 +170,9 @@

   assert.deepEqual(closure.find((input):input is Record<string,unknown>=>isTestRecordV1(input)&&Array.isArray(input.externalDependencies))?.externalDependencies, [...expected].sort(), "Command package scope must match esbuild's independent import oracle");
   assert.ok(!closure.includes("{workspaceRoot}/package.json") && !closure.includes("{workspaceRoot}/bun.lock"));
-  const workerCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).ownerCommandClosure;
+  const workerCase=fixture.ownerCommandClosure;
   const workerOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[workerCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),workerFiles=Object.keys(workerOracle.metafile.inputs);
   assert.ok(workerFiles.length<=workerCase.maximumFiles);for(const file of workerFiles)for(const segment of workerCase.excludedSegments)assert.ok(!file.includes(segment),"Owner command runtime must not import unrelated UI or test implementations");
   const workerInputs=testInputsV1(await testMethodV1(cacheInternals,"genericTargetCommandInputs",{options:{cwd:".",command:`bun ${JSON.stringify(workerCase.entry)} owner-command`}},workspace,[]));for(const file of workerFiles)assert.ok(workerInputs.includes("{workspaceRoot}/"+file),"Own command input closure must retain every actual bundled runtime source");
-  const nativeCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).nativeCommandClosure,nativeOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[nativeCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),nativeFiles=Object.keys(nativeOracle.metafile.inputs);console.log(`[DEBUG] Native command runtime source count=${nativeFiles.length}`);const paths=new Map<string,string[]>([[nativeCase.entry,[nativeCase.entry]]]);for(const file of paths.keys())for(const imported of nativeOracle.metafile.inputs[file]?.imports??[])if(!imported.external&&!paths.has(imported.path))paths.set(imported.path,[...paths.get(file)!,imported.path]);for(const file of nativeFiles)for(const segment of nativeCase.excludedSegments)assert.ok(!file.includes(segment),`Native command runtime contains unrelated source: ${(paths.get(file)??[file]).join(" -> ")}`);const nativeInputs=testInputsV1(await testMethodV1(cacheInternals,"nativeTargetCommandInputs",{options:{cwd:".",command:`bun ${JSON.stringify(nativeCase.entry)} native cargo build`}},workspace));for(const file of nativeFiles)assert.ok(nativeInputs.includes("{workspaceRoot}/"+file),"Native command input closure must retain every actual bundled runtime source");console.log(`[DEBUG] Independent esbuild source owners generic=${workerFiles.length} native=${nativeFiles.length}, unrelated UI/tests absent`);
+  const nativeCase=fixture.nativeCommandClosure,nativeOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[nativeCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),nativeFiles=Object.keys(nativeOracle.metafile.inputs);console.log(`[DEBUG] Native command runtime source count=${nativeFiles.length}`);const paths=new Map<string,string[]>([[nativeCase.entry,[nativeCase.entry]]]);for(const file of paths.keys())for(const imported of nativeOracle.metafile.inputs[file]?.imports??[])if(!imported.external&&!paths.has(imported.path))paths.set(imported.path,[...paths.get(file)!,imported.path]);for(const file of nativeFiles)for(const segment of nativeCase.excludedSegments)assert.ok(!file.includes(segment),`Native command runtime contains unrelated source: ${(paths.get(file)??[file]).join(" -> ")}`);const nativeInputs=testInputsV1(await testMethodV1(cacheInternals,"nativeTargetCommandInputs",{options:{cwd:".",command:`bun ${JSON.stringify(nativeCase.entry)} native cargo build`}},workspace));for(const file of nativeFiles)assert.ok(nativeInputs.includes("{workspaceRoot}/"+file),"Native command input closure must retain every actual bundled runtime source");console.log(`[DEBUG] Independent esbuild source owners generic=${workerFiles.length} native=${nativeFiles.length}, unrelated UI/tests absent`);
   const workerControl=createRequestedIssuedResourceControlV1(phaseControl,workerCase.policy.maximumElapsedMilliseconds),workerEnvironment=scriptInvocationEnvironment(invocation,process.env),wire=workerEnvironment.SEMIO_SCRIPT_PROCESS_INVOCATION;assert(typeof wire==="string");const workerEnvelope=JSON.parse(wire);
   const workerResultCapture=await captureOwnedCommandResultV1(process.execPath,[resolve(workspace,workerCase.entry),"owner-command","--cwd",workspace,"--","node","-e","console.log(JSON.stringify({arguments:process.argv.slice(1),envelope:JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)}))","--",...workerCase.forwardedArguments],workspace,{control:workerControl,env:workerEnvironment,output:{maximumBytes:65536,maximumLines:256}});assert.equal(workerResultCapture.status,0,workerResultCapture.stderr);assert.equal(workerResultCapture.signal,null);const resultLine=workerResultCapture.stdout.slice(workerResultCapture.stdout.indexOf('{"arguments":')).split("\n")[0];assert(resultLine!==undefined);const workerResult=JSON.parse(resultLine);assert.deepEqual(workerResult.arguments,workerCase.forwardedArguments);assert.deepEqual(workerResult.envelope,workerEnvelope);
@@ -173,5 +181,5 @@

   if (output) {
     mkdirSync(output, { recursive: true });
-    const patchCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingPatchReceipt,patchRoot=mkdtempSync(join(output,"p-"));
+    const patchCase=fixture.toolingPatchReceipt,patchRoot=mkdtempSync(join(output,"p-"));
     try{
       const oracle=createRequire(resolve(workspace,".nx/installation/package.json"))("diff").applyPatch(patchCase.before,patchCase.patch);assert.equal(oracle,patchCase.after);
@@ -181,15 +189,15 @@

     }finally{rmSync(patchRoot,{recursive:true,force:true});}
     await testNxSourceCacheCustody(workspace, output, invocation, phaseControl);
-    const manifestInputs = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).manifestInputOwnership;
+    const manifestInputs = fixture.manifestInputOwnership;
     const portable=testRecordV1(JSON.parse(readFileSync(buildGraphPathV1(workspace),"utf8"))),projection=testRecordV1((await import("../../../🟨️.mjs")).projectBuildGraphForNxV1(portable)),portableDependencies=portable.dependencies;assert(Array.isArray(portableDependencies));const admitted={nodes:Object.fromEntries(Object.entries(testRecordV1(projection,"projects")).map(([name,data])=>[name,{name,type:"lib",data:testRecordV1(data)}])),dependencies:Object.fromEntries(Object.keys(testRecordV1(portable,"projects")).map(name=>[name,portableDependencies.filter((edge:unknown)=>testRecordV1(edge).source===name)]))};
-    const goOwnership=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).goSourceOwnership,goOwner=testRecordV1(admitted,"nodes",goOwnership.project,"data"),goDependency=testRecordV1(admitted,"nodes",goOwnership.dependency,"data"),goEdges=admitted.dependencies[goOwnership.project],goRoot=goOwner.root,goDependencyRoot=goDependency.root;assert(goEdges);assert(typeof goRoot==="string"&&typeof goDependencyRoot==="string");
+    const goOwnership=fixture.goSourceOwnership,goOwner=testRecordV1(admitted,"nodes",goOwnership.project,"data"),goDependency=testRecordV1(admitted,"nodes",goOwnership.dependency,"data"),goEdges=admitted.dependencies[goOwnership.project],goRoot=goOwner.root,goDependencyRoot=goDependency.root;assert(goEdges);assert(typeof goRoot==="string"&&typeof goDependencyRoot==="string");
     assert.ok(goEdges.some((edge:unknown)=>testRecordV1(edge).target===goOwnership.dependency),"Selected Go local replacement must have a genuine Nx source owner edge");
     const goOracle=Bun.spawnSync(["go","list","-m","-json",goOwnership.module],{cwd:resolve(workspace,goRoot)});assert.equal(goOracle.exitCode,0);assert.equal(JSON.parse(new TextDecoder().decode(goOracle.stdout)).Dir,resolve(workspace,goDependencyRoot));
-    const goInputs=nxHasher.expandNamedInput("default",testRecordV1(goDependency,"namedInputs")),goInventory=new Set<string>();for(const input of goInputs)if(isTestRecordV1(input)&&typeof input.env==="string"&&input.env.startsWith("SEMIO_SOURCE_CLOSURE_")){const inventory=JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"),input.env.slice("SEMIO_SOURCE_CLOSURE_".length)+".json"),"utf8"));for(const file of inventory.files)goInventory.add(file);}
-    const goPatterns=goInputs.map((input:unknown)=>typeof input==="string"?input:testRecordV1(input).fileset).filter((input:unknown):input is string=>typeof input==="string").map((input:string)=>input.replaceAll("{projectRoot}",goDependencyRoot).replaceAll("{workspaceRoot}/",""));
+    const goInputs=nxHasher.expandNamedInput("default",testRecordV1(goDependency,"namedInputs")),goInventory=new Set<string>();for(const input of goInputs)if(isEnvironmentInput(input)&&input.env.startsWith("SEMIO_SOURCE_CLOSURE_")){const inventory=JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"),input.env.slice("SEMIO_SOURCE_CLOSURE_".length)+".json"),"utf8"));for(const file of inventory.files)goInventory.add(file);}
+    const goPatterns=goInputs.map(inputFileset).filter((input:unknown):input is string=>typeof input==="string").map((input:string)=>input.replaceAll("{projectRoot}",goDependencyRoot).replaceAll("{workspaceRoot}/",""));
     for(const file of goOwnership.files){const path=goDependencyRoot+"/"+file,covered=goInventory.has(path)||goPatterns.some((pattern:string)=>!pattern.startsWith("!")&&minimatch(path,pattern))&&!goPatterns.some((pattern:string)=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1)));assert.ok(covered,"Go dependency source inventory and native glob inputs must retain both platform implementations and module identity");}
     const goBuildInputs=testRecordV1(goOwner,"targets","build").inputs;assert(Array.isArray(goBuildInputs));assert.ok(goBuildInputs.some((input:unknown)=>typeof input==="string"?input.startsWith("^"):testRecordV1(input).projects!==undefined),"Go build must hash genuine dependency source inputs");
     console.log("[DEBUG] Actual Go module resolver agrees with neutral durability Nx dependency and both platform source inventories");
-    const nativeOwnership = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).nativeSourceOwnership;
+    const nativeOwnership = fixture.nativeSourceOwnership;
     const nativeInputs = nxHasher.expandNamedInput("nativeSources", testRecordV1(admitted,"nodes",nativeOwnership.project,"data","namedInputs")), nativeInventory = new Set<string>();
     for (const input of nativeInputs) if (input.env?.startsWith("SEMIO_SOURCE_CLOSURE_")) {
@@ -205,8 +213,8 @@

     }
     const manifestTargetInputs=testRecordV1(admitted,"nodes",manifestInputs.project,"data","targets",manifestInputs.target).inputs;assert(Array.isArray(manifestTargetInputs));assert.ok(manifestTargetInputs.length <= manifestInputs.maximumInputs, "Manifest-only Cargo projects must share the same source/command ownership projection as authored projects");
-    const digestCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sourceDigest;
+    const digestCase = fixture.sourceDigest,digestFiles:Readonly<Record<string,string>>=digestCase.files;
     const digestRoot = mkdtempSync(join(output, "d-"));
     try {
-      const missingLock = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).missingCargoLock;
+      const missingLock = fixture.missingCargoLock;
       const missingManifest = join(digestRoot, "Cargo.toml");
       writeFileSync(missingManifest, missingLock.manifest);
@@ -219,5 +227,5 @@

       for (const path of [...digestCase.paths].sort()) {
         expected.update(path + "\0");
-        const bytes = digestCase.files[path];
+        const bytes = digestFiles[path];
         if (bytes === undefined) expected.update("missing\0");
         else expected.update("file\0" + new Bun.CryptoHasher("sha256").update(bytes).digest("hex") + "\0");
@@ -231,5 +239,5 @@

       assert.notEqual(await sourceClosureDigest(digestRoot,contract,phaseControl), actual, "Missing input admission changes a reused closure hash");
     } finally { rmSync(digestRoot, { recursive: true, force: true }); }
-    const row = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).collectionAdmission;
+    const row = fixture.collectionAdmission;
     const policy = JSON.parse(readFileSync(new URL("../../🔣️policy.json", import.meta.url), "utf8"));
     const collections = row.collections;
@@ -248,5 +256,5 @@

   }
   if(output){
-    const contract=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).collectionCustody;
+    const contract=fixture.collectionCustody;
     const glob=createRequire(resolve(workspace,"package.json"))("fast-glob");
     for(const row of contract.cases){

### 73 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts
HEAD 238 BASE 245 STASH 246 CURRENT H
--- 

+++ 

@@ -112,4 +112,5 @@

       const currentLock=readFileSync(join(root,"Cargo.lock"),"utf8"), parsed=Bun.TOML.parse(currentLock);
       assert.deepEqual(parsed,require("smol-toml").parse(currentLock));
+      assert.ok("package"in parsed&&Array.isArray(parsed.package));
       const packages=testRecordV1(parsed).package;assert(Array.isArray(packages));assert.deepEqual(packages.map(value=>{const name=testRecordV1(value).name;assert.equal(typeof name,"string");return name;}).sort(),fixture.cargo.preparedPackages);
       if(preparedLock!==undefined)assert.equal(currentLock,preparedLock);preparedLock=currentLock;

### 74 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts
HEAD 141 BASE 139 STASH 142 CURRENT H
--- 

+++ 

@@ -1,2 +1,3 @@

+import {preparePlaygroundFixtureTargetsV1} from "../🎮️playground-preparation/🟦️.ts";
 import {preparePlaygroundSourceFixtureV1,type PlaygroundPreparationTargetV1} from "../🧊️preparation/🟦️.ts";
 import {type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -22,4 +23,6 @@

   const { cacheInternals, libraryBootstrap } = await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs");
   await libraryBootstrap;
+  const receivedTargets=await preparePlaygroundFixtureTargetsV1(workspace,root,invocation,phaseControl);
+  assert(receivedTargets[`prepare-${fixture.variant}-wgpu-${fixture.profile}`]);
   const {contracts,catalogContributions,context,check}=preparePlaygroundSourceFixtureV1(workspace,root,invocation,phaseControl);
   const targets:Record<string,PlaygroundPreparationTargetV1> = await cacheInternals.playgroundPreparationTargets(["component/Cargo.toml"], root, owner, contracts, catalogContributions, context);
@@ -34,5 +37,5 @@

   }
   const prepare = `prepare-${fixture.variant}-wgpu-${fixture.profile}`, activate = `activate-${fixture.variant}-wgpu-${fixture.profile}`;
-  assert.equal(targets[prepare].cache, false);
+  assert.equal(targets[prepare].cache, false, "Preparation owns its bounded build driver before activation");
   assert.equal(targets[activate].cache, false, "Live installation and reload publication must execute after every preparation/cache restoration");
   assert.deepEqual(targets[activate].outputs, [`{workspaceRoot}/.cache/b/p/m/browser/${fixture.profile}/runtime/wgpu/${fixture.variant}`], "Uncached activation still owns its complete live output namespace");

### 75 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-renderer-outputs/🟦️.ts
HEAD 97 BASE 97 STASH 97 CURRENT H
--- 

+++ 

@@ -33,5 +33,5 @@

   put(".cargo/config.toml", `[profile.prototype]\ninherits="dev"\n[profile.release]\nopt-level=3\n`);
   put("nx.json", JSON.stringify({ useDaemonProcess: false }));
-  const selectedTarget=targets[mode];assert(selectedTarget);
+  const selectedTarget=targets[mode];assert(selectedTarget!==undefined&&typeof selectedTarget==="object");
   put(`${typescript}/project.json`, JSON.stringify({ name: "native-renderer", targets: { "native-build": { ...selectedTarget,outputs:project.targets["native-build"].outputs.map((path:string)=>path.replaceAll(authoredRust,rust)),configurations:Object.fromEntries(profiles.map((row:any)=>[row.profile,{command:`bun ./📜️script.ts ${row.profile}`,env:{SEMIO_BUILD_MODE:row.profile}}])), defaultConfiguration:mode } } }));
   put(".gitignore", "node_modules\n.nx\ndist\noracle\n.runs\n");

### 76 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts
HEAD 103 BASE 103 STASH 105 CURRENT H
--- 

+++ 

@@ -1,2 +1,3 @@

+import {preparePlaygroundFixtureTargetsV1} from "../🎮️playground-preparation/🟦️.ts";
 import {readNativeRuntimeManifestV1} from "../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📋️contract/🟦️.ts";
 import {captureOwnedCommandResultV1} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
@@ -56,4 +57,5 @@

   await withInstalledNxFixtureV1(workspace,{kind:"products"},invocation,phaseControl,async(root,namespaceEnvironment,productOutput,actualPhase,toolkitRoot)=>{
   const put=(path:string,value:string)=>{if(join(root,path).length>256)throw Error("Bounded Native fixture path required");mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),value);};
+  const sourceRoot=join(root,"preparation-source"),receivedTargets=await preparePlaygroundFixtureTargetsV1(workspace,sourceRoot,invocation,actualPhase);assert(receivedTargets[`prepare-${fixture.variant}-native-${mode}`]);
   await buildComponentClosureV1(workspace,new Set([declaration.pluginId]),{facet:"model",invocation,network:repositoryCargoNetworkV1(invocation,workspace),control:actualPhase});checkResourceLeaseControl(actualPhase);
   put("package.json",'{"name":"native-runtime-fixture","private":true,"type":"module"}');put("nx.json",JSON.stringify({useDaemonProcess:false}));put("source/input","first");put(".gitignore","node_modules\n.nx\nout\n.counts\n*.pid\n");

### 77 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/🟦️.ts
HEAD 16 BASE 16 STASH 16 CURRENT H
--- 

+++ 

@@ -11,6 +11,6 @@

 /** 🔤️ Independent byte-wise lsof escaping of a non-ASCII store root still admits only the exact physical guard. */
 test("lsof escaped non-ASCII store roots preserve exact physical guard custody",()=>{const directory=join(process.cwd(),"🦑️compiler"),path=join(directory,"debug/.cargo-build-lock"),escape=(value:string)=>[...Buffer.from(value,"utf8")].map(byte=>byte<128?String.fromCharCode(byte):"\\x"+byte.toString(16).padStart(2,"0")).join(""),guard={pid:71,paths:[path]},observe=(records:string[])=>({status:0,signal:null,reason:"exit"as const,stdout:records.join("\n")+"\n",stderr:""});expect(escape(directory)).not.toBe(directory);expect(decodeLsofPathV1(escape(directory))).toBe(directory);expect(()=>assertNativeCacheDescriptorCustodyV1(observe(["p71","n"+escape(path)]),directory,guard,"newline-pn")).not.toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1(observe(["p71","n"+escape(path),"p72","n"+escape(join(directory,"debug/output.rlib"))]),directory,guard,"newline-pn")).toThrow("foreign open descriptors");expect(()=>assertNativeCacheDescriptorCustodyV1(observe(["p71"]),directory,guard,"newline-pn")).toThrow("absent");});
-/** 🔎️ Admits the actual policy, defining types, and registered pressure gates in both modes. */
-test("storage pressure policy and defining owner have complete strict and launch registration",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"test:storage-pressure-source",30000),policy=JSON.parse(readFileSync(join(import.meta.dir,"../../../🔣️policy.json"),"utf8")),rootSchema=JSON.parse(readFileSync(join(import.meta.dir,"../../../🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:false}).validate(rootSchema,policy)).toBe(true);expect(rootSchema.properties.storage.properties.pressure).toEqual(schema.definitions.Policy);expect(new Ajv({strict:true}).validate(schema.definitions.Policy,policy.storage.pressure)).toBe(true);await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",import.meta.dir),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",join(import.meta.dir,"../🟦️.ts")],workspace,{control});const{parse}=await import("jsonc-parser");for(const path of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const errors:any[]=[],launch=parse(readFileSync(join(workspace,path),"utf8"),errors,{allowTrailingComma:path.endsWith("jsonc")});expect(errors).toEqual([]);for(const mode of["prototype","release"])expect(launch.configurations.filter((row:any)=>row.name===`⚖️storage-pressure ${mode}`).length).toBe(1);}console.log("[DEBUG] Actual storage pressure policy, defining strict types and both launch inventories qualified");});},60000);
+/** 🔎️ Admits the actual policy, defining types, and registered pressure gates in default/ship modes. */
+test("storage pressure policy and defining owner have complete strict and dashboard registration",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"test:storage-pressure-source",30000),policy=JSON.parse(readFileSync(join(import.meta.dir,"../../../🔣️policy.json"),"utf8")),rootSchema=JSON.parse(readFileSync(join(import.meta.dir,"../../../🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:false}).validate(rootSchema,policy)).toBe(true);expect(rootSchema.properties.storage.properties.pressure).toEqual(schema.definitions.Policy);expect(new Ajv({strict:true}).validate(schema.definitions.Policy,policy.storage.pressure)).toBe(true);await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",import.meta.dir),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",join(import.meta.dir,"../🟦️.ts")],workspace,{control});const dashboard=JSON.parse(readFileSync(join(workspace,"📋️project.json"),"utf8")).metadata.semio.dashboard,source=dashboard.tools.find((tool:any)=>tool.id==="run-script");expect(source.command).toEqual(["bun","repo","exec","--projects={project}","--","bun","{workspace}/{script}"]);expect(dashboard.parameters.find((parameter:any)=>parameter.id==="build-mode").values).toEqual([{id:"ship",env:{SEMIO_BUILD_MODE:"release"},buildFlags:["--skip-cache","--skip-remote-cache"]}]);console.log("[DEBUG] Actual storage pressure policy, defining strict types and dashboard source/ship controls qualified");});},60000);
 /** 📉️ A closed neutral pressure corpus matches independent SQLite oldest-first admission and exclusion. */
 test("cache storage pressure admits only finite inactive cold slices and preserves hot working set",()=>{if(corpus.policy.version!==1)throw Error("Exact corpus pressure version required");const policyData={...corpus.policy,version:1 as const};expect(new Ajv({strict:true}).validate(schema.definitions.Policy,policyData)).toBe(true);const plan=Reflect.get(pruning,"planCachePressurePruneV1");expect(typeof plan).toBe("function");const units=corpus.units.map(row=>{if(row.kind!=="cargo-build")throw Error("Exact corpus cache-unit kind required");return{...row,kind:"cargo-build"as const};});const db=new Database(":memory:");try{db.exec("create table units(path text primary key, bytes integer, recency integer, locked integer)");for(const row of corpus.units)db.query("insert into units values(?,?,?,?)").run(row.path,row.bytes,row.recencyMs,Number(row.lockHeld));for(const row of corpus.cases){const expected:string[]=[],needed=row.availableBytes<policyData.minimumAvailableBytes?policyData.targetAvailableBytes-row.availableBytes:0;let bytes=0;for(const unit of db.query("select path,bytes from units where locked=0 and recency<=? order by recency,path").all(corpus.nowMs-policyData.guardAgeMs)as{path:string;bytes:number}[]){if(bytes>=needed)break;if(bytes+unit.bytes>policyData.maximumDeletionBytes)continue;expected.push(unit.path);bytes+=unit.bytes;}expect(expected).toEqual(row.selected);expect(bytes).toBe(row.bytes);const actual=plan(units,corpus.nowMs,row.availableBytes,policyData,row.availableBytes);expect(actual.units.map((unit:any)=>unit.path)).toEqual(expected);expect(actual.plannedBytes).toBe(bytes);expect(actual.targetUnmetBytes).toBe(row.targetUnmetBytes);expect(actual.units.some((unit:any)=>unit.path==="locked"||unit.path==="hot"||unit.path==="larger")).toBe(false);}expect(plan(units,corpus.nowMs,40,policyData,0).units.map((unit:any)=>unit.path)).toEqual(["old"]);expect(plan(units,corpus.nowMs,60,policyData,0).units).toEqual([]);for(const policy of[{...policyData,guardAgeMs:0},{...policyData,targetAvailableBytes:10},{...policyData,maximumDeletionBytes:Infinity},{...policyData,unknown:true}])expect(()=>plan(units,corpus.nowMs,0,policy,0)).toThrow();expect(()=>plan(units,corpus.nowMs,-1,policyData,0)).toThrow();console.log("[DEBUG] Storage pressure plan agrees with independent SQLite while excluding held/hot/oversized slices and bounding deletion");}finally{db.close();}});

### 78 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts
HEAD 124 BASE 109 STASH 109 CURRENT H
--- 

+++ 

@@ -39,6 +39,6 @@

   }
 
-  /** 📝️ Writes (or with `--check` only verifies) every checked-in file that derives from one canonical source: the devcontainer
-   * editor block, the nextest scopes and the dependabot manifests.
+  /** 📝️ Writes (or with `--check` only verifies) every checked-in file that derives from one canonical source: the nextest
+   * scope configurations and dependency update manifests.
    * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts */
   private async generateConfig(args: string[]): Promise<void> {

### 79 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts
HEAD 132 BASE 131 STASH 131 CURRENT H
--- 

+++ 

@@ -102,5 +102,5 @@

     const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
     if (a === "repo") {
-      runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], {
+      runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector", "--config", ".mcp.json", "--server", "repo"], {
         cwd: this.root,
         env: { ...process.env, HOST: host },

### 80 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts
HEAD 217 BASE 210 STASH 212 CURRENT H
--- 

+++ 

@@ -72,5 +72,5 @@

     }
     if (args.length) throw Error("Workspace build tests accept no arguments");
-    await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "🧪️tests/🟦️.ts"), resolve(import.meta.dir,"🧪️tests/🕸️graph/🟦️.ts"),resolve(import.meta.dir,"👁️observation/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"../⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts")], this.repoRoot, "workspace-build:contract", scriptInvocationBudget(this.invocation, 300_000), { signal: this.invocation.control.signal });
+    await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "🧪️tests/🟦️.ts"), resolve(import.meta.dir,"🧪️tests/🕸️graph/🟦️.ts"),resolve(import.meta.dir,"👁️observation/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"🩺️preflight/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"../⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts")], this.repoRoot, "workspace-build:contract", scriptInvocationBudget(this.invocation, 300_000), { signal: this.invocation.control.signal });
   }
 }
@@ -85,4 +85,6 @@

 export async function runOwnedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);const{readOwnedBuildInvocationV1}=await import("./📥️caller/🟦️.ts");readOwnedBuildInvocationV1(root,parent,args);
+ const{parseBuildInvocationV1}=await import("./🧭️invocation/🟦️.ts"),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:(parent.capabilities as {tool?:string}).tool});
+ if(selected.args[0]==="preflight"){const{runBuildPreflightV1}=await import("./🩺️preflight/🟦️.ts");await runBuildPreflightV1(root,selected.args,parent);return;}
  const {openRepositoryCachePartitionV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),control=phaseControl,leases=await openRepositoryCachePartitionV1(root,"leases",control);let failed=false;try{const {withRepositoryCacheStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");await withRepositoryCacheStoreV1(root,"diagnostics",parent,control,(diagnostics,diagnosticPhase)=>withRepositoryCacheStoreV1(root,"dependencies",parent,diagnosticPhase,async(_dependencies,phaseControl)=>{const previous=process.env.SEMIO_DIAGNOSTIC_DIRECTORY;process.env.SEMIO_DIAGNOSTIC_DIRECTORY=diagnostics.environment.SEMIO_DIAGNOSTIC_DIRECTORY;try{await runOwnedBuildInLeasesV1(root,args,parent,phaseControl);}finally{if(previous===undefined)delete process.env.SEMIO_DIAGNOSTIC_DIRECTORY;else process.env.SEMIO_DIAGNOSTIC_DIRECTORY=previous;}}));}catch(error){failed=true;throw error;}finally{try{await leases.close();}catch(error){if(!failed)throw error;}}
 }
@@ -111,5 +113,5 @@

     const command=resolved.args[0];
     if(!command||resolved.args.slice(0,resolved.args.indexOf("--")<0?undefined:resolved.args.indexOf("--")).includes("--help")){
-      console.log("bun repo [--tool=nx|bazel] run <project:target[:configuration]> | run-many --targets=<targets> [--projects=<projects>] | affected --targets=<targets> | exec --projects=<projects> -- <command> | graph | show projects|project <name>|targets | watch --projects=<projects> -- <command> | reset | tools setup [nx|bazel|all] | dependencies sync|lock --all|--scope <owner> | cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo subcommand> <args>");return;
+      console.log("bun repo [--tool=nx|bazel] run <project:target[:configuration]> | run-many --targets=<targets> [--projects=<projects>] | affected --targets=<targets> | exec --projects=<projects> -- <command> | graph | show projects|project <name>|targets | watch --projects=<projects> -- <command> | reset | tools setup [nx|bazel|all] | preflight --scope <owner> | dependencies sync|lock --all|--scope <owner> | cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo subcommand> <args>");return;
     }
     if(["run","run-many","affected","exec","watch"].includes(command)){

### 81 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts
HEAD 54 BASE 54 STASH 55 CURRENT H
--- 

+++ 

@@ -12,4 +12,5 @@

   const args=arguments_[0]==="repo"?arguments_.slice(1):arguments_,authored=environment.SEMIO_SCRIPT_CAPABILITIES===undefined?undefined:JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES),effective={...environment,...(environment.SEMIO_BUILD_MODE===undefined&&authored?.buildMode?{SEMIO_BUILD_MODE:authored.buildMode}:{})},selected=parseBuildInvocationV1(args,effective),resolved=resolveBuildInvocationV1(root,selected.args,effective);
   const {createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("../🟦️.ts");
+  if(resolved.args[0]==="preflight"){const {readBuildPreflightScopeV1,readBuildPreflightRequirementsV1}=await import("../../🩺️preflight/🟦️.ts");readBuildPreflightRequirementsV1(root,readBuildPreflightScopeV1(resolved.args));}
   const grant=authored===undefined?createNativeBuildCallerCapabilitiesV1(root,selected.args,selected.tool,{...effective,...resolved.env}):readBuildAuthoringCapabilitiesV1(root,selected.args,selected.tool,authored);
   if(effective.SEMIO_BUILD_MODE!==undefined&&effective.SEMIO_BUILD_MODE!==grant.buildMode)throw Error("Original authored build mode changed");
@@ -24,5 +25,5 @@

    const args=pending.shift()!,key=JSON.stringify(args);if(seen.has(key))continue;if(seen.size>=maximumWatchBuildDescendantsV1)throw Error("Bounded watch descendant declarations required");seen.add(key);
    const parsed=parseBuildInvocationV1(args,{...effective,SEMIO_BUILD_TOOL:selected.tool});if(parsed.tool!==selected.tool)throw Error("Watch callback cannot change the original build implementation");
-   const invocation=resolveBuildInvocationV1(root,parsed.args,{...effective,SEMIO_BUILD_MODE:grant.buildMode}),command=invocation.args[0],tools=invocation.args[2]??selected.tool,main=command==="tools"?(tools!=="bazel"?["--version"]:undefined):command==="reset"?(selected.tool==="nx"?["reset"]:undefined):selected.tool==="nx"&&!["dependencies","cargo","watch"].includes(command??"")?nxBuildArgumentsV1(invocation.args):undefined;
+   const invocation=resolveBuildInvocationV1(root,parsed.args,{...effective,SEMIO_BUILD_MODE:grant.buildMode}),command=invocation.args[0],tools=invocation.args[2]??selected.tool,main=command==="tools"?(tools!=="bazel"?["--version"]:undefined):command==="reset"?(selected.tool==="nx"?["reset"]:undefined):selected.tool==="nx"&&!["dependencies","cargo","watch","preflight"].includes(command??"")?nxBuildArgumentsV1(invocation.args):undefined;
    add(["repo",...parsed.args]);
    if(main){const{resolveNxBootstrapLaunchesV1}=await import("../../../⚡️caching/🚀️bootstrap/📥️caller/🟦️.ts"),nx=resolveNxBootstrapLaunchesV1(root,["nx",...main],{...effective,...invocation.env,SEMIO_BUILD_MODE:grant.buildMode});for(const launch of nx.launches)add(["nx",...launch]);}

### 82 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs
HEAD 151 BASE 2114 STASH 2118 CURRENT H
--- 

+++ 

@@ -1263,5 +1263,9 @@

 }
 
-/** 🎮️ Flattens every plugin playground row from Cargo manifests (with authored `distDir`). */
+/** 🎮️ Flattens every plugin playground row from Cargo manifests (with authored `distDir`).
+ * @param {readonly string[]} configFiles
+ * @param {string} workspaceRoot
+ * @param {readonly import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]} [catalogContributions]
+ */
 function collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions=[]) {
   if(!catalogContributions.length)return [];
@@ -1353,5 +1357,5 @@

  * @param {string} workspaceRoot
  * @param {string} projectRoot
- * @returns {Record<string, { cache: boolean, continuous?: boolean, dependsOn: string[], outputs: string[], inputs?: unknown[], options: { command: string, forwardAllArgs?: boolean } }>}
+ * @returns {Record<string, { cache: boolean, continuous?: boolean, dependsOn: string[], outputs: string[], inputs?: unknown[], metadata?: { semio?: { orchestrator?: boolean, execution?: object, runtimeInputAdmission?: object } }, options: { command: string, forwardAllArgs?: boolean } }>}
  */
 async function playgroundPreparationTargets(configFiles, workspaceRoot, projectRoot, contracts = {},catalogContributions=[],context) {

### 83 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts
HEAD 12 BASE 137 STASH 179 CURRENT H
--- 

+++ 

@@ -4,6 +4,7 @@

 import { existsSync, readFileSync, rmSync } from "node:fs";
 import { availableParallelism, homedir } from "node:os";
-import { join, resolve } from "node:path";
-import { spawnSync } from "node:child_process";
+import { join, resolve, sep } from "node:path";
+import { spawn, spawnSync } from "node:child_process";
+import { StringDecoder } from "node:string_decoder";
 import { Script, ScriptRouter, findWorkspaceRoot, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -15,9 +16,10 @@

 import { acquireQueuedResourceLease } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
 import { devToolingEnv } from "../../🏃️process/🌿️environment/🟦️.ts";
+import { terminateOwnedChildTree } from "../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
 
 const runProbe = (command: string, args: string[], options: {env?: NodeJS.ProcessEnv} = {}) => spawnSync(command,args,{encoding:"utf8",...options});
-const WINDOWS_CMAKE_GENERATOR = "Visual Studio 18 2026";
+const CMAKE_GENERATOR = "Ninja";
 
-function queryVisualStudio2026InstallPath(): string | undefined {
+function queryMsvcInstallPath(): string | undefined {
  if (process.platform !== "win32") return undefined;
  const vswhere = join(process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)", "Microsoft Visual Studio", "Installer", "vswhere.exe");
@@ -26,4 +28,48 @@

  return result.status === 0 ? result.stdout.trim() || undefined : undefined;
 }
+
+/** 🧾️ Reads a compiler command-file environment without duplicate Windows variable names. */
+export function compilerEnvironmentFromOutput(output: string, base: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
+  const entries = new Map(Object.entries(base).map(([key, value]) => [key.toLowerCase(), [key, value] as const]));
+  for (const line of output.split(/\r?\n/)) {
+    const item = /^([A-Za-z_][A-Za-z0-9_()]*)=(.*)$/.exec(line);
+    if (item) entries.set(item[1]!.toLowerCase(), [item[1]!, item[2]!]);
+  }
+  return Object.fromEntries(entries.values());
+}
+
+/** 🛠️ Activates standalone MSVC tools for a supervised Ninja task, without an IDE session. */
+export async function windowsCompilerEnvironment(root: string, base: NodeJS.ProcessEnv, signal: AbortSignal): Promise<NodeJS.ProcessEnv> {
+  if (process.platform !== "win32") return base;
+  signal.throwIfAborted();
+  const installation = queryMsvcInstallPath();
+  if (!installation) throw Error("Microsoft C++ Build Tools are missing; select workspace:setup-native in the dashboard.");
+  const file = join(installation, "VC", "Auxiliary", "Build", "vcvarsall.bat");
+  if (!existsSync(file) || /[\r\n"%&|<>^!]/.test(file)) throw Error("Invalid MSVC compiler command file");
+  const architectures: Readonly<Record<string, string>> = { x64: "amd64", arm64: "amd64_arm64", ia32: "x86" }, architecture = architectures[process.arch];
+  if (!architecture) throw Error(`Unsupported MSVC architecture: ${process.arch}`);
+  const control = AbortSignal.any([signal, AbortSignal.timeout(30_000)]);
+  control.throwIfAborted();
+  console.log(`[cpp] Activating Microsoft C++ Build Tools for ${process.arch}`);
+  const output = await new Promise<string>((accept, reject) => {
+    const child = spawn(base.ComSpec ?? "cmd.exe", ["/d", "/u", "/s", "/c", "call", file, architecture, ">nul", "&&", "set"], { cwd: root, env: base, windowsHide: true, stdio: ["ignore", "pipe", "ignore"] });
+    const decoder = new StringDecoder("utf16le");
+    let text = "", bytesRead = 0, exceeded = false;
+    const stop = () => terminateOwnedChildTree(child);
+    control.addEventListener("abort", stop, { once: true });
+    if (control.aborted) stop();
+    child.stdout.on("data", bytes => { bytesRead += bytes.length; if (bytesRead > 262_144) { exceeded = true; stop(); } else text += decoder.write(bytes); });
+    child.once("error", reject);
+    child.once("close", code => {
+      control.removeEventListener("abort", stop);
+      if (control.aborted) reject(control.reason);
+      else if (exceeded || code !== 0) reject(Error("MSVC compiler environment activation failed"));
+      else accept(text + decoder.end());
+    });
+  });
+  const environment = compilerEnvironmentFromOutput(output, base);
+  if (!["include", "lib", "path"].every(name => Object.entries(environment).some(([key, value]) => key.toLowerCase() === name && value))) throw Error("MSVC compiler environment is incomplete");
+  return environment;
+}
 //#region 🔖️CppScript
 export class CppScript extends Script {
@@ -31,4 +77,5 @@

     const [command = "all", ...arguments_] = segments;
     if (arguments_.length) throw Error("CMake uses the host preset and explicit SEMIO_BUILD_MODE");
+    if (command === "test-toolchain") { await runOwnedCommand(process.execPath, ["test", join(import.meta.dir, "🧪️tests/🟦️.ts")], this.root, "cpp:toolchain-contract", scriptInvocationBudget(this.invocation,buildBudgetMs()), { signal: this.invocation.control.signal }); return; }
     if (command === "setup") { await this.runSetup(); return; }
     const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), host = process.platform === "win32" ? "windows" : process.platform === "darwin" ? "macos" : "linux", preset = `${host}-${mode}`;
@@ -42,12 +89,13 @@

   private async runSetup(): Promise<void> {
     await this.ensureTool("cmake", "cmake");
-    if (process.platform !== "win32") await this.ensureTool("ninja", "ninja");
-    if (process.platform === "win32") this.ensureWindowsMsvc();
+    await this.ensureTool("ninja", "ninja");
+    await windowsCompilerEnvironment(this.root, this.cppEnv(), this.invocation.control.signal);
   }
 
   private async withCompilerState(preset: string, action: (environment: NodeJS.ProcessEnv, signal: AbortSignal, control:CompilerAdmissionControl) => Promise<void>): Promise<void> {
+    const environment = await windowsCompilerEnvironment(this.root, this.cppEnv(), this.invocation.control.signal);
     const compiler = process.env.CXX ?? (process.platform === "win32" ? "cl" : "c++");
-    const compilerIdentity = process.platform === "win32" ? queryVisualStudio2026InstallPath() : runProbe(compiler, ["--version"]).stdout;
-    const toolchain = createHash("sha256").update(JSON.stringify([runProbe(this.resolveTool("cmake"), ["--version"]).stdout, compiler, compilerIdentity, process.env.CC, process.env.CMAKE_TOOLCHAIN_FILE, process.env.CMAKE_TOOLCHAIN_FILE ? readFileSync(resolve(this.root, process.env.CMAKE_TOOLCHAIN_FILE)) : null])).digest("hex").slice(0, 12);
+    const compilerProbe = runProbe(compiler, process.platform === "win32" ? [] : ["--version"], { env: environment });
+    const toolchain = createHash("sha256").update(JSON.stringify([CMAKE_GENERATOR, runProbe(this.resolveTool("cmake"), ["--version"]).stdout, compiler, compilerProbe.stdout, compilerProbe.stderr, environment.VCToolsVersion, process.env.CC, process.env.CMAKE_TOOLCHAIN_FILE, process.env.CMAKE_TOOLCHAIN_FILE ? readFileSync(resolve(this.root, process.env.CMAKE_TOOLCHAIN_FILE)) : null])).digest("hex").slice(0, 12);
     const directory = join(readProcessOwnerContextV1(process.env,this.root).cacheRoot, "cmake", preset, `${process.arch}-${toolchain}`);
     const leases = readProcessOwnerContextV1(process.env,this.root).leaseDirectory;
@@ -59,5 +107,5 @@

       try {
         this.purgeStaleCmakeCache(preset, directory);
-        await action({...this.cppEnv(), SEMIO_CMAKE_BINARY_DIR: directory, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(capacity.parallelism), CMAKE_BUILD_PARALLEL_LEVEL: String(capacity.parallelism), CTEST_PARALLEL_LEVEL: String(capacity.parallelism)}, control.signal,control);
+        await action({...environment, SEMIO_CMAKE_BINARY_DIR: directory, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(capacity.parallelism), CMAKE_BUILD_PARALLEL_LEVEL: String(capacity.parallelism), CTEST_PARALLEL_LEVEL: String(capacity.parallelism)}, control.signal,control);
       } finally { lease.release(); }
     } finally { capacity?.release(); }
@@ -113,12 +161,4 @@

   }
 
-  private ensureWindowsMsvc(): void {
-    if (process.platform !== "win32") return;
-    if (queryVisualStudio2026InstallPath()) return;
-    console.error("[cpp] Visual Studio 2026 with the Desktop development with C++ workload is required.");
-    console.error("[cpp] On native Windows run: bun ./📜️script.ts setup native");
-    process.exit(1);
-  }
-
   private purgeStaleCmakeCache(preset: string, cacheDir: string): void {
     const cacheFile = join(cacheDir, "CMakeCache.txt");
@@ -127,7 +167,9 @@

     const generatorMatch = content.match(/^CMAKE_GENERATOR:INTERNAL=(.+)$/m);
     const cachedGenerator = generatorMatch?.[1]?.trim();
-    if (process.platform === "win32" && cachedGenerator && cachedGenerator !== WINDOWS_CMAKE_GENERATOR) {
+    if (cachedGenerator && cachedGenerator !== CMAKE_GENERATOR) {
+      const compilerRoot = resolve(readProcessOwnerContextV1(process.env, this.root).cacheRoot, "cmake"), target = resolve(cacheDir);
+      if (!target.toLowerCase().startsWith(compilerRoot.toLowerCase() + sep)) throw Error("CMake output escapes the owned compiler cache");
       console.log(`[cpp] Removing stale CMake cache for preset "${preset}" (generator ${cachedGenerator}).`);
-      rmSync(cacheDir, { recursive: true, force: true });
+      rmSync(target, { recursive: true, force: true });
     }
   }

### 84 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/🟦️.ts
HEAD 31 BASE 0 STASH 66 CURRENT H
--- 

+++ 

@@ -0,0 +1,66 @@

+import { expect, test } from "bun:test";
+import Ajv2020 from "ajv/dist/2020";
+import { parse } from "dotenv";
+import { readFileSync } from "node:fs";
+import { join, resolve } from "node:path";
+import { compilerEnvironmentFromOutput, windowsCompilerEnvironment } from "../📜️script.ts";
+
+const owner = resolve(import.meta.dir, ".."), root = resolve(owner, "../../../../../../..");
+const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
+const fixture = read(join(owner, "🧫️fixtures/🔣️.json"));
+const validate = new Ajv2020({ strict: false }).compile(read(join(owner, "🧬️schema/🔣️.json")));
+
+test("all native host presets use the compiler-only generator and agree with Ajv", () => {
+  for (const row of fixture.presets) {
+    const native = row.preset.hidden === true || row.preset.generator === "Ninja" && !Object.hasOwn(row.preset, "architecture") && ["Debug", "Release"].includes(row.preset.cacheVariables?.CMAKE_BUILD_TYPE);
+    expect(native, row.name).toBe(row.accepted);
+    expect(validate({ configurePresets: [row.preset] }), row.name).toBe(row.accepted);
+  }
+  const presets = read(join(root, "CMakePresets.json"));
+  expect(validate(presets), JSON.stringify(validate.errors)).toBe(true);
+  expect(presets.configurePresets.filter((preset: { hidden?: boolean }) => !preset.hidden)).toHaveLength(6);
+  const bootstrap = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"), "utf8");
+  expect(bootstrap).toContain('Sync-WingetPackage -Id "Microsoft.VisualStudio.BuildTools"');
+  expect(bootstrap).not.toContain('Sync-WingetPackage -Id "Microsoft.VisualStudio.Community"');
+  console.log("[DEBUG] all six native CMake presets use Ninja; Windows setup installs standalone Build Tools");
+});
+
+test("compiler environment vectors agree with the independent dotenv parser", () => {
+  for (const row of fixture.environments) {
+    const actual = compilerEnvironmentFromOutput(row.output, row.base);
+    expect(actual, row.name).toEqual(row.expected);
+    if (row.oracle) {
+      const normalized = Object.fromEntries(Object.entries(row.base).filter(([key]) => !Object.keys(parse(row.output)).some(name => name.toLowerCase() === key.toLowerCase())));
+      expect({ ...normalized, ...parse(row.output) }, row.name).toEqual(actual);
+    }
+  }
+  console.log(`[DEBUG] verified ${fixture.environments.length} shared compiler environment vectors`);
+});
+
+test("compiler activation observes cancellation before spawning", async () => {
+  const controller = new AbortController(); controller.abort();
+  if (process.platform === "win32") await expect(windowsCompilerEnvironment(root, process.env, controller.signal)).rejects.toThrow();
+  else expect(await windowsCompilerEnvironment(root, { PATH: "/compiler" }, controller.signal)).toEqual({ PATH: "/compiler" });
+});
+
+test("standalone compiler activation completes under Bun with spaced installation paths", async () => {
+  if (process.platform !== "win32") return;
+  const unicode = "C:/工具/📦️", environment = await windowsCompilerEnvironment(root, { ...process.env, SEMIO_CPP_PROBE_UNICODE: unicode }, new AbortController().signal);
+  expect(environment.SEMIO_CPP_PROBE_UNICODE).toBe(unicode);
+  const entries = Object.entries(environment);
+  for (const name of ["path", "include", "lib", "vctoolsversion"]) {
+    const values = entries.filter(([key]) => key.toLowerCase() === name);
+    expect(values).toHaveLength(1);
+    expect(values[0]![1]).toBeTruthy();
+  }
+  console.log("[DEBUG] Bun activated standalone MSVC command-line tools with complete compiler environment");
+}, 35_000);
+
+test("running compiler activation cancels its owned process tree", async () => {
+  if (process.platform !== "win32") return;
+  const controller = new AbortController(), timer = setTimeout(() => controller.abort(Error("compiler cancellation probe")), 20);
+  try {
+    await expect(windowsCompilerEnvironment(root, process.env, controller.signal)).rejects.toThrow("compiler cancellation probe");
+    console.log("[DEBUG] cancelled active compiler environment activation");
+  } finally { clearTimeout(timer); }
+}, 10_000);

### 85 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧬️schema/🔣️.json
HEAD 225 BASE 0 STASH 24 CURRENT H
--- 

+++ 

@@ -0,0 +1,24 @@

+{
+  "$schema": "https://json-schema.org/draft/2020-12/schema",
+  "title": "🛠️ IDE-independent native build presets",
+  "type": "object",
+  "required": ["configurePresets"],
+  "properties": {
+    "configurePresets": {
+      "type": "array",
+      "items": {
+        "type": "object",
+        "if": { "required": ["hidden"], "properties": { "hidden": { "const": true } } },
+        "then": {},
+        "else": {
+          "required": ["generator", "cacheVariables"],
+          "properties": {
+            "generator": { "const": "Ninja" },
+            "cacheVariables": { "type": "object", "required": ["CMAKE_BUILD_TYPE"], "properties": { "CMAKE_BUILD_TYPE": { "enum": ["Debug", "Release"] } } }
+          },
+          "not": { "required": ["architecture"] }
+        }
+      }
+    }
+  }
+}

### 86 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts
HEAD 950 BASE 964 STASH 972 CURRENT H
--- 

+++ 

@@ -417,4 +417,10 @@

 });
 
+test("dashboard target selection preserves the portable initial authority corpus",()=>{
+ const validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema),root=process.cwd(),registry=JSON.parse(readFileSync(resolve(root,"📋️build-owners.json"),"utf8")),dashboard=JSON.parse(readFileSync(resolve(root,"📋️project.json"),"utf8")).metadata.semio.dashboard;
+ expect(dashboard.tools.find((tool:any)=>tool.id==="run-targets").command.slice(0,3)).toEqual(["bun","repo","run-many"]);
+ for(const row of fixture.initialAuthoring.cases){const request=["repo",...row.arguments],declared={workspaceRoot:row.workspaceRoot,tool:row.tool,arguments:request,selection:request,launches:[request],buildMode:row.buildMode,native:row.native};expect(validate(declared),row.name).toBe(true);const [project,target]=row.arguments[1]!.split(":");const owner=registry.owners.find((owner:any)=>owner.name===project);expect(owner,row.name).toBeDefined();expect(JSON.parse(readFileSync(resolve(root,owner.manifest),"utf8")).targets[target],row.name).toBeDefined();}
+});
+
 test("the defining graph fact module exposes its declared writer port under original custody",async()=>{
  const source=new URL("../../📥️inputs/💾️storage/🟨️.mjs",import.meta.url),storage=await import(source.href),parser=require("typescript"),tree=parser.createSourceFile(fileURLToPath(source),readFileSync(source,"utf8"),parser.ScriptTarget.Latest,true),exports=tree.statements.filter((node:any)=>parser.isFunctionDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text);
@@ -681,5 +687,7 @@

   const parser=require("typescript"),workspace=process.cwd(),source=readFileSync(resolve(import.meta.dir,"../../📜️script.ts"),"utf8"),syntax=parser.createSourceFile("build.ts",source,parser.ScriptTarget.Latest,true),definition=syntax.statements.find((node:any)=>parser.isFunctionDeclaration(node)&&node.name?.text==="runOwnedBuildInLeasesV1"),body=definition.getText(syntax);expect(body.indexOf('command==="dependencies"')).toBeLessThan(body.indexOf("await ensureBuildEnvironmentV1"));expect(body).toContain("readBuildDependencyInvocationV1(resolved.args)");
   for(const path of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const launch=parser.parseConfigFileTextToJson(path,readFileSync(join(workspace,path),"utf8"));expect(launch.error).toBeUndefined();for(const expected of fixture.dependencyInvocations.launches){const rows=launch.config.configurations.filter((row:any)=>row.name===expected.name);expect(rows).toHaveLength(1);const row=rows[0];expect(row.command).toBe(expected.command);expect(JSON.parse(row.env.SEMIO_SCRIPT_POLICY)).toEqual(expected.policy);expect(row.env.SEMIO_SCRIPT_CAPABILITIES).toBeUndefined();expect(row.env.CARGO_NET_OFFLINE).toBeUndefined();const args=expected.command.split(" ").slice(2),capabilities=createNativeBuildCallerCapabilitiesV1(workspace,args,"nx",{});expect(capabilities.native.network.offline).toBe(false);expect(capabilities.arguments).toEqual(["repo",...args]);const envelope=createScriptProcessEnvelope(expected.policy,capabilities,Date.now());expect(envelope.deadlineEpochMilliseconds).not.toBeNull();expect(envelope.capabilities).toEqual(capabilities);}}
-  console.log("[DEBUG] Both authored JavaScript GUI rows select direct dependency commands; original finite600000 initial factory remains online before first process envelope");
+  const dashboard=JSON.parse(readFileSync(join(workspace,"📋️project.json"),"utf8")).metadata.semio.dashboard;expect(dashboard.tools.find((tool:any)=>tool.id==="run-script").command.slice(0,3)).toEqual(["bun","repo","exec"]);
+  for(const expected of fixture.dependencyInvocations.launches){const args=expected.command.split(" ").slice(2),capabilities=createNativeBuildCallerCapabilitiesV1(workspace,args,"nx",{});expect(capabilities.native.network.offline).toBe(false);expect(capabilities.arguments).toEqual(["repo",...args]);const envelope=createScriptProcessEnvelope(expected.policy,capabilities,Date.now());expect(envelope.deadlineEpochMilliseconds).not.toBeNull();expect(envelope.capabilities).toEqual(capabilities);}
+  console.log("[DEBUG] Both authored JavaScript dashboard source rows select direct dependency commands; original finite600000 initial factory remains online before first process envelope");
 });
 

### 87 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md
HEAD 3578 BASE 3770 STASH 3770 CURRENT H
--- 

+++ 

@@ -326,6 +326,6 @@

 | `framework.process.testing.cargo.execution` | framework-module | `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧬️schema` | `CargoExecutionArgumentsV1` (`🔣️.json`), `CargoExecutionBackendV1` (`🔣️.json`), `CargoExecutionCommandV1` (`🔣️.json`), `CargoExecutionContinuationV1` (`🔣️.json`), `CargoExecutionDeclarationV1` (`🔣️.json`), `CargoExecutionGroupV1` (`🔣️.json`), `CargoExecutionIdentityV1` (`🔣️.json`), `CargoExecutionRequestV1` (`🔣️.json`), `CargoExecutionSourceV1` (`🔣️.json`), `CargoExecutionTargetV1` (`🔣️.json`), `CargoExecutionTextV1` (`🔣️.json`) | — |
 | `framework.process.testing.vitest` | framework-module | `🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🧬️schema` | — | — |
+| `framework.product.course` | product-root | `🧰️framework/🛍️products/📚️course/🧬️schema` | `Anchor` (`🔣️.json`), `Answer` (`🔣️.json`), `Axis` (`🔣️.json`), `Badge` (`🔣️.json`), `BadgeAward` (`🔣️.json`), `BadgeRule` (`🔣️.json`), `Best` (`🔣️.json`), `Course` (`🔣️.json`), `CourseQuizView` (`🔣️.json`), `CourseView` (`🔣️.json`), `Category` (`🔣️.json`), `CategoryHint` (`🔣️.json`), `Challenge` (`🔣️.json`), `ClassificationAnswer` (`🔣️.json`), `ClassificationItem` (`🔣️.json`), `ClassificationItemResult` (`🔣️.json`), `ClassificationTask` (`🔣️.json`), `Command` (`🔣️.json`), `CompareHint` (`🔣️.json`), `CrowdCount` (`🔣️.json`), `CrowdItem` (`🔣️.json`), `CrowdScores` (`🔣️.json`), `CrowdTask` (`🔣️.json`), `CrowdView` (`🔣️.json`), `Cursor` (`🔣️.json`), `CursorState` (`🔣️.json`), `Dimension` (`🔣️.json`), `DimensionResult` (`🔣️.json`), `Event` (`🔣️.json`), `GroupHint` (`🔣️.json`), `Handle` (`🔣️.json`), `HandleView` (`🔣️.json`), `Hint` (`🔣️.json`), `Icon` (`🔣️.json`), `Id` (`🔣️.json`), `Identity` (`🔣️.json`), `IdentityClaim` (`🔣️.json`), `Introduction` (`🔣️.json`), `Leaderboard` (`🔣️.json`), `LeaderboardPeriod` (`🔣️.json`), `LeaderboardRow` (`🔣️.json`), `LeaderboardWindow` (`🔣️.json`), `LearnerView` (`🔣️.json`), `Limits` (`🔣️.json`), `MatchingAnswer` (`🔣️.json`), `MatchingItem` (`🔣️.json`), `MatchingItemResult` (`🔣️.json`), `MatchingTask` (`🔣️.json`), `Motion` (`🔣️.json`), `Place` (`🔣️.json`), `PresenceState` (`🔣️.json`), `Profile` (`🔣️.json`), `ProfileHint` (`🔣️.json`), `Quantity` (`🔣️.json`), `Query` (`🔣️.json`), `Quiz` (`🔣️.json`), `Rejection` (`🔣️.json`), `RunResult` (`🔣️.json`), `RunStatus` (`🔣️.json`), `RunSummary` (`🔣️.json`), `RunView` (`🔣️.json`), `Scale` (`🔣️.json`), `Score` (`🔣️.json`), `Screen` (`🔣️.json`), `Sheet` (`🔣️.json`), `SheetAxis` (`🔣️.json`), `SheetClassificationTask` (`🔣️.json`), `SheetDimension` (`🔣️.json`), `SheetItem` (`🔣️.json`), `SheetMatchingTask` (`🔣️.json`), `SheetSortingTask` (`🔣️.json`), `SheetTask` (`🔣️.json`), `ShortText` (`🔣️.json`), `Slug` (`🔣️.json`), `SortingAnswer` (`🔣️.json`), `SortingItem` (`🔣️.json`), `SortingItemResult` (`🔣️.json`), `SortingTask` (`🔣️.json`), `Task` (`🔣️.json`), `TaskKind` (`🔣️.json`), `TaskResult` (`🔣️.json`), `Text` (`🔣️.json`), `ThinkingAnswer` (`🔣️.json`), `ThinkingMatchingAnswer` (`🔣️.json`), `ThinkingState` (`🔣️.json`), `Timestamp` (`🔣️.json`), `Verdict` (`🔣️.json`), `WireVersion` (`🔣️.json`) | — |
 | `framework.product.pets` | product-root | `🧰️framework/🛍️products/🐾️pets/🧬️schema` | `Activity` (`🔣️.json`), `Actor` (`🔣️.json`), `ActorFrame` (`🔣️.json`), `Bond` (`🔣️.json`), `Bone` (`🔣️.json`), `Cancelled` (`🔣️.json`), `Canopy` (`🔣️.json`), `Cast` (`🔣️.json`), `Channel` (`🔣️.json`), `ChuteTool` (`🔣️.json`), `Circling` (`🔣️.json`), `Claim` (`🔣️.json`), `Clip` (`🔣️.json`), `Color` (`🔣️.json`), `Cooling` (`🔣️.json`), `Course` (`🔣️.json`), `Cue` (`🔣️.json`), `Deed` (`🔣️.json`), `Degrees` (`🔣️.json`), `Dragged` (`🔣️.json`), `Drift` (`🔣️.json`), `Ease` (`🔣️.json`), `Effect` (`🔣️.json`), `EllipseShape` (`🔣️.json`), `Emitter` (`🔣️.json`), `Ensemble` (`🔣️.json`), `Extent` (`🔣️.json`), `Eye` (`🔣️.json`), `EyeFrame` (`🔣️.json`), `Face` (`🔣️.json`), `Feeling` (`🔣️.json`), `Fixture` (`🔣️.json`), `Foothold` (`🔣️.json`), `Footing` (`🔣️.json`), `Frame` (`🔣️.json`), `Gait` (`🔣️.json`), `Gaze` (`🔣️.json`), `Gear` (`🔣️.json`), `Glanced` (`🔣️.json`), `Grip` (`🔣️.json`), `GunTool` (`🔣️.json`), `Hang` (`🔣️.json`), `HookTool` (`🔣️.json`), `Hover` (`🔣️.json`), `Hushed` (`🔣️.json`), `Key` (`🔣️.json`), `Ladder` (`🔣️.json`), `LadderFrame` (`🔣️.json`), `LadderTool` (`🔣️.json`), `LiftFrame` (`🔣️.json`), `LineShape` (`🔣️.json`), `Locomotion` (`🔣️.json`), `Menagerie` (`🔣️.json`), `Mood` (`🔣️.json`), `Mouth` (`🔣️.json`), `Needs` (`🔣️.json`), `Paint` (`🔣️.json`), `Palette` (`🔣️.json`), `Parachute` (`🔣️.json`), `Part` (`🔣️.json`), `ParticleFrame` (`🔣️.json`), `PathShape` (`🔣️.json`), `Perch` (`🔣️.json`), `Permitted` (`🔣️.json`), `PetMode` (`🔣️.json`), `Pitch` (`🔣️.json`), `Played` (`🔣️.json`), `Pledge` (`🔣️.json`), `Plume` (`🔣️.json`), `Point` (`🔣️.json`), `Pointed` (`🔣️.json`), `Pointer` (`🔣️.json`), `Prank` (`🔣️.json`), `Press` (`🔣️.json`), `PressPhase` (`🔣️.json`), `Pressed` (`🔣️.json`), `Puff` (`🔣️.json`), `PuffFrame` (`🔣️.json`), `Purr` (`🔣️.json`), `Rapport` (`🔣️.json`), `Reaction` (`🔣️.json`), `Reclaimed` (`🔣️.json`), `Rect` (`🔣️.json`), `RectShape` (`🔣️.json`), `Released` (`🔣️.json`), `Repertoire` (`🔣️.json`), `Rope` (`🔣️.json`), `RopeTool` (`🔣️.json`), `Scrolled` (`🔣️.json`), `Shaking` (`🔣️.json`), `Shape` (`🔣️.json`), `Shot` (`🔣️.json`), `Size` (`🔣️.json`), `Slice` (`🔣️.json`), `Slug` (`🔣️.json`), `Species` (`🔣️.json`), `SpeciesState` (`🔣️.json`), `Stage` (`🔣️.json`), `StageEvent` (`🔣️.json`), `Stirred` (`🔣️.json`), `Stroking` (`🔣️.json`), `Summoned` (`🔣️.json`), `Surface` (`🔣️.json`), `Surveyed` (`🔣️.json`), `Temperament` (`🔣️.json`), `Text` (`🔣️.json`), `Ticked` (`🔣️.json`), `Ticks` (`🔣️.json`), `Tier` (`🔣️.json`), `Tint` (`🔣️.json`), `ToolFrame` (`🔣️.json`), `Track` (`🔣️.json`), `Trait` (`🔣️.json`), `Trick` (`🔣️.json`), `Trip` (`🔣️.json`), `Tuned` (`🔣️.json`), `Turns` (`🔣️.json`), `Unpointed` (`🔣️.json`), `Wall` (`🔣️.json`), `Warmth` (`🔣️.json`), `Waypoint` (`🔣️.json`) | — |
-| `framework.product.quiz` | product-root | `🧰️framework/🛍️products/❓️quiz/🧬️schema` | `Anchor` (`🔣️.json`), `Answer` (`🔣️.json`), `Axis` (`🔣️.json`), `Badge` (`🔣️.json`), `BadgeAward` (`🔣️.json`), `BadgeRule` (`🔣️.json`), `Best` (`🔣️.json`), `Catalog` (`🔣️.json`), `CatalogQuizView` (`🔣️.json`), `CatalogView` (`🔣️.json`), `Category` (`🔣️.json`), `CategoryHint` (`🔣️.json`), `Challenge` (`🔣️.json`), `ClassificationAnswer` (`🔣️.json`), `ClassificationItem` (`🔣️.json`), `ClassificationItemResult` (`🔣️.json`), `ClassificationTask` (`🔣️.json`), `Command` (`🔣️.json`), `CompareHint` (`🔣️.json`), `CrowdCount` (`🔣️.json`), `CrowdItem` (`🔣️.json`), `CrowdScores` (`🔣️.json`), `CrowdTask` (`🔣️.json`), `CrowdView` (`🔣️.json`), `Cursor` (`🔣️.json`), `CursorState` (`🔣️.json`), `Dimension` (`🔣️.json`), `DimensionResult` (`🔣️.json`), `Event` (`🔣️.json`), `GroupHint` (`🔣️.json`), `Handle` (`🔣️.json`), `HandleView` (`🔣️.json`), `Hint` (`🔣️.json`), `Icon` (`🔣️.json`), `Id` (`🔣️.json`), `Identity` (`🔣️.json`), `IdentityClaim` (`🔣️.json`), `Introduction` (`🔣️.json`), `Leaderboard` (`🔣️.json`), `LeaderboardPeriod` (`🔣️.json`), `LeaderboardRow` (`🔣️.json`), `LeaderboardWindow` (`🔣️.json`), `LearnerView` (`🔣️.json`), `Limits` (`🔣️.json`), `MatchingAnswer` (`🔣️.json`), `MatchingItem` (`🔣️.json`), `MatchingItemResult` (`🔣️.json`), `MatchingTask` (`🔣️.json`), `Motion` (`🔣️.json`), `Place` (`🔣️.json`), `PresenceState` (`🔣️.json`), `Profile` (`🔣️.json`), `ProfileHint` (`🔣️.json`), `Quantity` (`🔣️.json`), `Query` (`🔣️.json`), `Quiz` (`🔣️.json`), `Rejection` (`🔣️.json`), `RunResult` (`🔣️.json`), `RunStatus` (`🔣️.json`), `RunSummary` (`🔣️.json`), `RunView` (`🔣️.json`), `Scale` (`🔣️.json`), `Score` (`🔣️.json`), `Screen` (`🔣️.json`), `Sheet` (`🔣️.json`), `SheetAxis` (`🔣️.json`), `SheetClassificationTask` (`🔣️.json`), `SheetDimension` (`🔣️.json`), `SheetItem` (`🔣️.json`), `SheetMatchingTask` (`🔣️.json`), `SheetSortingTask` (`🔣️.json`), `SheetTask` (`🔣️.json`), `ShortText` (`🔣️.json`), `Slug` (`🔣️.json`), `SortingAnswer` (`🔣️.json`), `SortingItem` (`🔣️.json`), `SortingItemResult` (`🔣️.json`), `SortingTask` (`🔣️.json`), `Task` (`🔣️.json`), `TaskKind` (`🔣️.json`), `TaskResult` (`🔣️.json`), `Text` (`🔣️.json`), `ThinkingAnswer` (`🔣️.json`), `ThinkingMatchingAnswer` (`🔣️.json`), `ThinkingState` (`🔣️.json`), `Timestamp` (`🔣️.json`), `Verdict` (`🔣️.json`), `WireVersion` (`🔣️.json`) | — |
 | `framework.public-invocation` | framework-module | `🧰️framework/🔨️modules/🛂️manifest/🎛️public-invocation/🧬️schema` | `PublicInvocationEnvelope` (`🔣️.json`) | — |
 | `framework.replication.apply-refusal` | framework-module | `🧰️framework/🔨️modules/📡️replication/🚧️apply-refusal/🧬️schema` | `TransientApplyRefusalCodeV1` (`🔣️.json`), `TransientApplyRefusalMessageV1` (`🔣️.json`) | — |

### 88 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json
HEAD 2495 BASE 2417 STASH 2445 CURRENT H
--- 

+++ 

@@ -708,5 +708,4 @@

       "inputs": [
         "sharedGlobals",
-        "{workspaceRoot}/.vscode/settings.json",
         "{workspaceRoot}/📜️script.ts",
         "{workspaceRoot}/**/📜️script.ts",
@@ -735,5 +734,4 @@

       "inputs": [
         "sharedGlobals",
-        "{workspaceRoot}/.vscode/settings.json",
         "{workspaceRoot}/📜️script.ts",
         "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/{🏗️builder/🌐️vite/🟦️.ts,📦️packages/🟦️typescript/{📋️project.json,📜️script.ts}}",
@@ -1797,11 +1795,25 @@

     },
     "test-build-discovery": {
-      "executor":"repo:script",
-      "cache":false,
-      "dependsOn":[],
-      "inputs":["default","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/**/*","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/**/*"],
-      "outputs":[],
-      "metadata":{"semio":{"execution":{"version":1,"kind":"source"}}},
-      "options":{"cwd":".","command":"bun \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts\" test build-discovery"}
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "default",
+        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/**/*"
+      ],
+      "outputs": [],
+      "metadata": {
+        "semio": {
+          "execution": {
+            "version": 1,
+            "kind": "source"
+          }
+        }
+      },
+      "options": {
+        "cwd": ".",
+        "command": "bun \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts\" test build-discovery"
+      }
     },
     "test-nx-project-inference": {
@@ -2413,4 +2425,20 @@

         "command": "bun ./📜️script.ts test build-surfaces"
       }
+    },
+    "test-agent-clients": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",
+        "command": "bun ./📜️script.ts test agent-clients"
+      }
+    },
+    "test-derived-config": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",
+        "command": "bun ./📜️script.ts test derived-config"
+      }
     }
   }

### 89 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts
HEAD 920 BASE 1046 STASH 1053 CURRENT H
--- 

+++ 

@@ -448,4 +448,11 @@

       const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts");
       await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
+      return;
+    }
+    if (segments.length === 1 && (segments[0] === "agent-clients" || segments[0] === "derived-config")) {
+      const source = segments[0] === "agent-clients"
+        ? "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤖️agent-client-configuration/🟦️.ts"
+        : "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts";
+      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, source)], { cwd: this.repoRoot, budgetMs: 120_000, signal: this.invocation.control.signal });
       return;
     }

### 90 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json
HEAD 100170 BASE 99764 STASH 101663 CURRENT H
--- 

+++ 

@@ -66,38 +66,16 @@

       },
       "exports": {
-        "PresentationPresence": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [
-        "app.animate.presentation.presence.mutation.replace-presence"
-      ],
-      "hashes": {
-        "🔗️.graphql": "e45af323989d912c6604912789ec39eb415bb5b0ed16b0040ae4a1d388579628",
-        "🔣️.json": "17fc3a69bfc53aad3adad010f246a0a6b3e6ddb0c33aeeb3fe0850342ec8e0d0",
-        "🛰️.proto": "c906a9ea6e43cabd8b024d37dcdea0125524da28f690728fabe07a6e7b17b5d4",
-        "🟦️.ts": "c069b5b348e1d0310446f899b58716bdcb2755645de4d0fa70fe5fc4b4e46cce",
-        "🦀️.rs": "819ecb25c89e18c1abedb99c3e732e4e07e8011ebf0884ecee406035e7db2e24",
-        "🧬️mutations/🔣️.json": "a01243fb643634826dac6fd417f5d8fe105e2f2324ec30066d6762b19ee0fa1a",
-        "🧬️mutations/🦀️.rs": "3009ebd50dfeb61fa3eb67824ebf091a9793e6503fcf6aaa953ed292f5b57b10"
-      }
-    },
-    "app.animate.presentation.presence.mutation.replace-presence": {
-      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations/📸️replace-presence/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplacePresence": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "6ebc32cf37c8c639109f90709587d978653e7238e695b19a7b6b56f7be6c533d"
+        "NoPresence": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔗️.graphql": "03205b5c0f4e44d96aa164430d1d5ff6952fee8c4cdd52a0d20ed58b6e2296a6",
+        "🔣️.json": "24a014db712ff045e78b2029141427071dbf58156639858d30039183853d72f8",
+        "🛰️.proto": "cdc70475e1955f488caf7c881199578ad4d534d54dc76fec3f40f8ac08fbfa5c",
+        "🟦️.ts": "d82dc45197abb473e5b53c1ec7f7925aa891660ad87a4040e873b2cf38a5fdd6",
+        "🦀️.rs": "19422a27c82739681dc6b60188f8b48eec18bec51e0949f8875b7dbf4b913ed5"
       }
     },
@@ -258,8 +236,8 @@

       "dependsOn": [],
       "hashes": {
-        "🔗️.graphql": "1bc01431ffe7afb967516d23682d29da30e6a110690b8537a74176e091d31d9f",
-        "🔣️.json": "04e75c7a678511764df55672f3a8c96909055c127992a4c12d4049ab4ab51b1c",
-        "🛰️.proto": "a34130e061a5a8d91d953bc061ac0c03926c167b04d2efbacc07cd1bac50df65",
-        "🟦️.ts": "3fe40eba53b971890d57dc8b88e076c5eedf9f138341de1beb26e989ae11160d"
+        "🔗️.graphql": "1fe2da8397c4a5d71aba6fb85f98e101744717ded09bef43c94793f82a6519ea",
+        "🔣️.json": "1a3b9dd45864cb6f303856a4d817ed6967635f8d06a9a10d62657edebfc8662d",
+        "🛰️.proto": "cb357b624cec3e9952d01e12422d7a949922eb7aaae9305abd9f3d0772c9ebda",
+        "🟦️.ts": "ff2d6a3cd9f02669afd35e4e62a27ced1dbad2a29b88c64799675b7caf35937b"
       }
     },
@@ -518,6 +496,5 @@

       },
       "dependsOn": [
-        "app.dag.dag.config.mutation.change-camera",
-        "app.dag.dag.config.mutation.replace-config"
+        "app.dag.dag.config.mutation.change-camera"
       ],
       "hashes": {
@@ -527,6 +504,6 @@

         "🟦️.ts": "e11ff6d0ee3117056141220193873286c28b71f4db896f2418adfacd40a935f5",
         "🦀️.rs": "25e60e6dd488568bca0331892601b8f8b2df472f854b65eb5f2679a99126df00",
-        "🧬️mutations/🔣️.json": "01733331320aaeabc92935057b7c9ae0d91347c992371a7083f626486e13d160",
-        "🧬️mutations/🦀️.rs": "378fcc6a106a139741874c686b6a1c826c2cf56a9e55d0a877582fd8ca4580a4"
+        "🧬️mutations/🔣️.json": "ecfa0db19ea1c942e772f1cdfd873ab8a6b94a96c77fac30b12f5c5f9b0e263c",
+        "🧬️mutations/🦀️.rs": "2a95c04a52d16fad6ff22bd79b25ef2499f3a3a1a470098acbef969344c6d033"
       }
     },
@@ -547,22 +524,4 @@

       "hashes": {
         "🔣️.json": "32db7cde9805757efda9d3056b614a5d53e3626729f7c6d8d16f497c20f952ee"
-      }
-    },
-    "app.dag.dag.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔄️replace-config/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "af07f5bd1b53cac4ad37f38bc05ef321c2340470557498f2d4a26371b7a84759"
       }
     },
@@ -889,5 +848,4 @@

       },
       "dependsOn": [
-        "app.forms.forms.config.mutation.replace-config",
         "app.forms.forms.config.mutation.set-contributions"
       ],
@@ -898,24 +856,6 @@

         "🟦️.ts": "825c0bbcfa2c55fbf44ff9cc2abea683741e4761330de96d8eae4a41796fe821",
         "🦀️.rs": "d4cda513c737eaef469eccf46a34e8d266d4f8c91f64efc10c55f28665a1deec",
-        "🧬️mutations/🔣️.json": "05e5ad63c4b4fbd58a50a91076447b9ea2e5578400c4953ae4e9149caeed01c0",
-        "🧬️mutations/🦀️.rs": "94622715a8d872fcea07478c1eda61ba783128d0d3aa71f44feff500ed244641"
-      }
-    },
-    "app.forms.forms.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "1020b8e030f9f893d93a3f87209e4147ca1818963f31e6227b2e728cedcc0c63"
+        "🧬️mutations/🔣️.json": "ad7d495067c33326215a6775506142efb59ccc56b5efa3ad4901b656d43f5715",
+        "🧬️mutations/🦀️.rs": "b0c1990307ebbce84f508617c114cafefa8acd462819aaff8e1336b57e2dd894"
       }
     },
@@ -1044,5 +984,4 @@

       },
       "dependsOn": [
-        "app.imperative.imperative.config.mutation.replace-config",
         "app.imperative.imperative.config.mutation.set-contributions",
         "app.imperative.imperative.config.mutation.set-run-output"
@@ -1054,24 +993,6 @@

         "🟦️.ts": "65488ff660ce2604ca98756a732fa3bd99fa16d4e268461955fea01cf8d9bfe2",
         "🦀️.rs": "ff600ecb121f0c4f8584f0e4ac7197d86fd7d4e19f303a19d2e8c0fc9f0d4a1b",
-        "🧬️mutations/🔣️.json": "4cd0ab595803c1b3332195a427771e523a2b774a9ff18e9b78b90f8cbd071002",
-        "🧬️mutations/🦀️.rs": "9de7530b9514010523f9588560a67ca534a8be0f75fd0cc8409f814c0f386653"
-      }
-    },
-    "app.imperative.imperative.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "6c5db7a907a4b60a43c69e7c030c4570833b4ed85b86e43fb351a1ba0435658c"
+        "🧬️mutations/🔣️.json": "a99d9ef257b789265cb094d112c9cef4b3c69ea4d695e1a168b00b741c16a56d",
+        "🧬️mutations/🦀️.rs": "c3afda2a45e7aba3cceb33bc60418cbcd106d19968ee8055deb19f3736d81d59"
       }
     },
@@ -2112,5 +2033,5 @@

         "🔣️.json": "2893040faff0f65e492de3f96c05acee48d5f90ab00ca9e1884e3ef572db0bcc",
         "🔺️diff/🔣️.json": "7be8d369bde1a63996d3e75c3cbad1f74904b91a7462d946175eab873d87ada5",
-        "🔺️diff/🦀️.rs": "ce155db26e37440281a34bb54911008d9c0ae23a2a574adc943d3d2ef060557a",
+        "🔺️diff/🦀️.rs": "5112fc4f1a5f740af276d7abaf99ebe929b3f469543da48f08180fc0c51c2dd8",
         "🛰️.proto": "90a4b361f91f1da180d14c64a726bb3ecfe248d0ea603554ab0d5255d5f95a37",
         "🟦️.ts": "89c468cbc2b0afb6727826fcd965dbf45e77be24a8781d60c855640e76a37824",
@@ -2156,5 +2077,4 @@

       },
       "dependsOn": [
-        "app.playbook.playbook.config.mutation.replace-config",
         "app.playbook.playbook.config.mutation.set-contributions"
       ],
@@ -2165,24 +2085,6 @@

         "🟦️.ts": "789c98da7df123ab890101a8d2906178956d5eacebd8bee71ca1ff35946e23c6",
         "🦀️.rs": "e4a6d06f6d8405eb0ccb08967815d70050cb9c49a48588f82bdedfd1f10e342d",
-        "🧬️mutations/🔣️.json": "342aa4b006d9b810fca709506d5e66c441f4dfc5a3906860a2608923d712afa2",
-        "🧬️mutations/🦀️.rs": "73b3ac081e9699b4b0e74eae1afd27ec893ac9d3cdc48c48a22c2774cf77d0f9"
-      }
-    },
-    "app.playbook.playbook.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "8e09792af335fb90505bc144887eef91025a7b1d6aa551e928db944041b475bf"
+        "🧬️mutations/🔣️.json": "9e7825805d4b8b9daff5fa673edac1b85d6eeaa30f1b6bad25442eb4464d7748",
+        "🧬️mutations/🦀️.rs": "a45b863915a9487c37bf66ab646aacd59734d645dd415edb272d04dc3307b719"
       }
     },
@@ -3105,5 +3007,4 @@

       },
       "dependsOn": [
-        "app.shooting.shooting.config.mutation.replace-config",
         "app.shooting.shooting.config.mutation.set-camera",
         "app.shooting.shooting.config.mutation.set-center-model",
@@ -3117,28 +3018,10 @@

         "🔣️.json": "f6baddf595e91487fa387072baa0fd899a31b69f4d523e02944a73f3adb4912d",
         "🔺️diff/🔣️.json": "d2bb691c2d0c968d6a982cb4031ba991c6092c74e610009fa88d79663ebd3a72",
-        "🔺️diff/🦀️.rs": "b63fef30381fe8c0ea74af76ee83a56bba82a76c97b65d408d215503443eff57",
+        "🔺️diff/🦀️.rs": "4498087f3a88973f9622abb357876dada14e797b9205bddbc17e58ed2a25d136",
         "🛰️.proto": "62712050157bc85dab26330180e30d7a6a66c5b49534df251fa975031eb39297",
         "🟦️.ts": "d38e8f66e52bb492d1f55e42dff328b01e1f7b2b353c95d371c49b2aef1da6d1",
         "🦀️.rs": "30b81a475a7b39a14e3b626e3e048f67a545b9ab6217834a76f4252d4eeecd63",
-        "🧬️mutations/🔣️.json": "b99dfbc69311c896a81079491fdeec299f36c22b99ec6873e183b3613444cccf",
-        "🧬️mutations/🦀️.rs": "5e908e2c2ae572bae5d62671b54249c337a73b5a21547e61def4dc9a823a083d"
-      }
-    },
-    "app.shooting.shooting.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "e9b4b07e698d7ae9b14c4c7c189ced5c2d6d3f2ad3f0ef499d549a0a56caff38"
+        "🧬️mutations/🔣️.json": "d48997ff9168926473321bd33a1da43347b5b85b97f9d94313cbe2930595c007",
+        "🧬️mutations/🦀️.rs": "f5539bf28e2014fab1563f9372656169051a17dcbf88233787292ccfb16636b7"
       }
     },
@@ -3262,5 +3145,5 @@

         "🔣️.json": "2db61b9790bdbd2b152e5edac3be4e647d0cb63d40f36ad875f81363e77a309b",
         "🔺️diff/🔣️.json": "2c58463d75abf828205d3c71db3efae4bb9dbd24d40e8fc15dfce6f80cf1f219",
-        "🔺️diff/🦀️.rs": "5502ad4a2d86294dafb48d25e4e632da1359a89a2c53e769baa1f0a2a0752e13",
+        "🔺️diff/🦀️.rs": "1d87d086688c76b6a453cb5d871661b332caadccd3b521ec787c84602866afa3",
         "🛰️.proto": "70826159a726a63f9e4d02b12628b8a9a97666bf21e131ddfc5c491f9c6792f6",
         "🟦️.ts": "181b0092f23bd63cb09ceefd3c9c812432a49b9c5b2a124ef84fe8a3b91cd350",
@@ -3441,10 +3324,11 @@

       "dependsOn": [
         "app.space.home.transient.mutation.apply-directory-page",
+        "app.space.home.transient.mutation.set-directory",
         "os.directory"
       ],
       "hashes": {
         "🔣️.json": "9a9db1490390c1436dd4ec9718c853d1e9d2d6232da403d7ae2b1e4dceeee4af",
-        "🧬️mutations/🔣️.json": "6f9d0da75af79cc25855a69522e6ab26dd41a8c78d66986f23b5eb9a8a41f980",
-        "🧬️mutations/🦀️.rs": "f11cc8dcb49992b657cd59ba1e55d40ba69f8c9999a8a5b999685b6c1ef1b50a"
+        "🧬️mutations/🔣️.json": "8567b2bc015c223b9a31938d35d0ab0d052804be7bac5f0160b76be1b79aa520",
+        "🧬️mutations/🦀️.rs": "ee1cefb0dd0a84e3b4f92e90d9fcf11ce59037ab5954247684837769ac9af49e"
       }
     },
@@ -3465,4 +3349,22 @@

       "hashes": {
         "🔣️.json": "ef6011b43d4caae08f0b2bb64d5221181eb81762ee9e49448010e4677cd0121a"
+      }
+    },
+    "app.space.home.transient.mutation.set-directory": {
+      "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🗂️set-directory/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SetDirectory": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "1a56c01599795826346d093bd7d1e571ec004df79a0c6f42602f7086ea520ad9"
       }
     },
@@ -5620,6 +5522,5 @@

       "dependsOn": [
         "s.wfc.wfc2d.1.any.config.mutation.change-active-tile",
-        "s.wfc.wfc2d.1.any.config.mutation.change-camera",
-        "s.wfc.wfc2d.1.any.config.mutation.replace-config"
+        "s.wfc.wfc2d.1.any.config.mutation.change-camera"
       ],
       "hashes": {
@@ -5629,6 +5530,6 @@

         "🟦️.ts": "911f0c001fbf891c1fef2820a659c933f7eaa477fadfd0ea0a7da8d56f0be598",
         "🦀️.rs": "b2f6a75acd8b27bf43bda4e515de4c63068a041ee2fc875f392d3d3f0d64dcc5",
-        "🧬️mutations/🔣️.json": "0eb108251c8998148a64097ad3d674a6b9c74480668e84c378d5d61c5f17fa73",
-        "🧬️mutations/🦀️.rs": "878201320f0db8cb966be6869f93c6bb1ac55846f41768a97faf65ccbd82e79d"
+        "🧬️mutations/🔣️.json": "8b53ddd322a5faf67fe77707720f0444604f361910cd73a718d47a52223d3832",
+        "🧬️mutations/🦀️.rs": "c95de07db5d80142f94787be729d62e004920e6c2ca4a100932871469d579889"
       }
     },
@@ -5678,6 +5579,5 @@

       "dependsOn": [
         "s.wfc.wfc3d.1.any.config.mutation.change-active-tile",
-        "s.wfc.wfc3d.1.any.config.mutation.change-camera",
-        "s.wfc.wfc3d.1.any.config.mutation.replace-config"
+        "s.wfc.wfc3d.1.any.config.mutation.change-camera"
       ],
       "hashes": {
@@ -5687,6 +5587,6 @@

         "🟦️.ts": "49284a19c1bee90c8a9f63685a6297182b1b2803ebf99057c504639558612a47",
         "🦀️.rs": "f26f440bbe0ae9eaef480e4ae546dda64b643dc8a159668a3ada1f1a9682c122",
-        "🧬️mutations/🔣️.json": "92c913748728f89a13818dec4d8e70f39746b903d84535b633396b25a6e2f31d",
-        "🧬️mutations/🦀️.rs": "2348177d5a0e5a8751434b5b0dc0c5f59ddf55f67e7bb681a93c0fcf83d12a1a"
+        "🧬️mutations/🔣️.json": "e7f127fbbb98325b7e21db8b57c37cf3fcbaeaf5863da57c28866e6fd4a8504c",
+        "🧬️mutations/🦀️.rs": "ca5aa02790b13f63ef0f4376a5332374dac2697e17818ad14a9096fa388d9304"
       }
     },
@@ -7786,4 +7686,418 @@

       }
     },
+    "framework.product.course": {
+      "path": "🧰️framework/🛍️products/📚️course/🧬️schema",
+      "level": "product-root",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🦀️rust": "🦀️.rs",
+        "🟦️typescript": "🟦️.ts",
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "Anchor": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Answer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Axis": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Badge": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "BadgeAward": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "BadgeRule": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Best": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Category": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CategoryHint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Challenge": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ClassificationAnswer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ClassificationItem": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ClassificationItemResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ClassificationTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Command": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CompareHint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Course": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CourseQuizView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CourseView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CrowdCount": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CrowdItem": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CrowdScores": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CrowdTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CrowdView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Cursor": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "CursorState": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Dimension": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "DimensionResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Event": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "GroupHint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Handle": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "HandleView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Hint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Icon": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Id": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Identity": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "IdentityClaim": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Introduction": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Leaderboard": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "LeaderboardPeriod": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "LeaderboardRow": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "LeaderboardWindow": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "LearnerView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Limits": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "MatchingAnswer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "MatchingItem": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "MatchingItemResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "MatchingTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Motion": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Place": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceBadgeAward": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresencePoint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresencePreferences": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresencePresentation": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceProfile": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceRun": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceRunSummary": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceSort": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceState": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "PresenceSubmission": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Profile": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ProfileHint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Quantity": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Query": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "QuestionPool": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Quiz": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Rejection": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "RunResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "RunStatus": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "RunSummary": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "RunView": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Scale": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Score": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Screen": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Sheet": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetAxis": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetClassificationTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetDimension": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetItem": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetMatchingTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetSortingTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SheetTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ShortText": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Slug": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SortingAnswer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SortingItem": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SortingItemResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "SortingTask": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Task": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "TaskKind": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "TaskResult": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Text": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ThinkingAnswer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ThinkingMatchingAnswer": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "ThinkingState": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Timestamp": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Topic": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "Verdict": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
+        "WireVersion": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "2ca865763b904881665ba137c456355c9b2ef48f039efecc417947c7c9f6cce8",
+        "🟦️.ts": "1e838cca3aa0005f91ab2dee395336c75728c4f83522bd6821304c5df6510176",
+        "🦀️.rs": "289222b4249dfc4c29fc906919d5a735ced4d7cbd5eee97bc2ce5af6f6111833"
+      }
+    },
     "framework.product.pets": {
       "path": "🧰️framework/🛍️products/🐾️pets/🧬️schema",
@@ -8286,374 +8600,4 @@

         "🟦️.ts": "1c1450d24e6f27c022e9a650ff9bd40bca205bd03b5379ebe855d120975b8ffb",
         "🦀️.rs": "dde632eb67e2d0de26c5bf5850a8236bc12aa90aca711f10380b4dd0563d0c2f"
-      }
-    },
-    "framework.product.quiz": {
-      "path": "🧰️framework/🛍️products/❓️quiz/🧬️schema",
-      "level": "product-root",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🦀️rust": "🦀️.rs",
-        "🟦️typescript": "🟦️.ts",
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "Anchor": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Answer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Axis": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Badge": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "BadgeAward": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "BadgeRule": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Best": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Catalog": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CatalogQuizView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CatalogView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Category": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CategoryHint": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Challenge": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ClassificationAnswer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ClassificationItem": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ClassificationItemResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ClassificationTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Command": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CompareHint": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CrowdCount": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CrowdItem": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CrowdScores": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CrowdTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CrowdView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Cursor": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "CursorState": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Dimension": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "DimensionResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Event": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "GroupHint": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Handle": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "HandleView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Hint": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Icon": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Id": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Identity": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "IdentityClaim": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Introduction": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Leaderboard": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "LeaderboardPeriod": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "LeaderboardRow": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "LeaderboardWindow": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "LearnerView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Limits": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "MatchingAnswer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "MatchingItem": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "MatchingItemResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "MatchingTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Motion": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Place": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "PresenceState": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Profile": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ProfileHint": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Quantity": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Query": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Quiz": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Rejection": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "RunResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "RunStatus": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "RunSummary": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "RunView": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Scale": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Score": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Screen": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Sheet": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetAxis": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetClassificationTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetDimension": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetItem": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetMatchingTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetSortingTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SheetTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ShortText": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Slug": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SortingAnswer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SortingItem": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SortingItemResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "SortingTask": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Task": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "TaskKind": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "TaskResult": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Text": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ThinkingAnswer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ThinkingMatchingAnswer": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "ThinkingState": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Timestamp": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "Verdict": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        },
-        "WireVersion": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "c7d1cfb6851fde76ecc5c88357446bc77ede5adb95665cdc20346c807daa0df7",
-        "🟦️.ts": "cff3ceec10f3cfa8dd046eba1cf91d2124ed3497abb9058a3f9c7c619551e52a",
-        "🦀️.rs": "0eb1d649da743603230cc1a8fb1436ffbb6f0d2e4bfe8fef415d70ac7ab383fb"
       }
     },
@@ -8754,8 +8698,8 @@

       ],
       "hashes": {
-        "🔗️.graphql": "bec1420ef8c78fcbab8e47035d64e846ea41a4f34ef9b7d3cb400fe7a1828c8e",
-        "🔣️.json": "770d9fe03a0efec0b08a0bc7eaedad1d0baba051da2ef1249c96b7cf598b6aba",
-        "🛰️.proto": "1b2d18a6adeb98d52b5062bd5a843341246a498e2019002f1db3583f929a763f",
-        "🟦️.ts": "434a46440f7057191927da5e54e6ac0f765aeb14805caecfc52822d860c05511"
+        "🔗️.graphql": "619fd2203bafdad075cfc0fcacd61177a4b1840243f4dd58a7699e3f3a11baf9",
+        "🔣️.json": "0c9eba4921cd6504fa2e29fd20e33d337a962f59413294f320d86dcdb60f4bc0",
+        "🛰️.proto": "572e11b048b7b5f77c7711ee9c5cf6ad3a70d0147c2b1e9c828ecc2a378b3674",
+        "🟦️.ts": "0e18b7757132b67c8abf88009adac6eb35692f6c246524e8d72c9ce06ca5480a"
       }
     },
@@ -12279,4 +12223,34 @@

       "hashes": {
         "🔣️.json": "4f6d3fbc5539cdde794faa7c3341f1f115ae2903999245877f4879dbbea2ffac"
+      }
+    },
+    "os.dev.activation.browser-host": {
+      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🧬️schema",
+      "level": "product-module",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "GeneratedCoreV1": {
+          "file": "🔣️.json",
+          "facet": "component"
+        },
+        "Sha256Digest": {
+          "file": "🔣️.json",
+          "facet": "component"
+        },
+        "TestBrowserGisMaterializationV1": {
+          "file": "🔣️.json",
+          "facet": "component"
+        },
+        "TestBrowserHostStagingV1": {
+          "file": "🔣️.json",
+          "facet": "component"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "da3631a0cc46adeea06a86e853a541df9edb7bacffa4ba86fb1f22a241ee0dd9"
       }
     },
@@ -13334,6 +13308,6 @@

       "hashes": {
         "🔣️.json": "7332ca89a852c627340597f43b12178f696558db6a2e7cb680cb806d1925f380",
-        "🔺️diff/🔣️.json": "f52017dd80c54a1479f93f05a8e7d7c79e66b5cf693b3474fd11160b885a7146",
-        "🔺️diff/🦀️.rs": "2e3bbe4a4b7fd30097252af2ea7ed33b721bb3b38277a5dbbce268e0fb6e8a1a",
+        "🔺️diff/🔣️.json": "02ef079e830d6afc1cf74d58e6a8c90af94b7ace0f2c20bc5c4c0f071d7881db",
+        "🔺️diff/🦀️.rs": "64e7088a0f2fa11a353fcb259ba9cece091d91def1f4f79d5821e068fb25797c",
         "🧬️mutations/🔣️.json": "e6bcdc05f9ce543b8a7abb0758c79ff5a58a05c9b931ca4d38e3c43504670b29",
         "🧬️mutations/🦀️.rs": "2a79678316acf35e0401272a21eeac7bb310af0efa2aa8580abf84a60caf077e"
@@ -13641,5 +13615,5 @@

         "🔣️.json": "cb2c4513654a92206a6b4a529265717da89689c7d661a986643d7460748dccf9",
         "🧬️mutations/🔣️.json": "dd420b03ef8f3a4b1de293b5978f0b60df37294d8342bdb0e508a0e3e137cb27",
-        "🧬️mutations/🦀️.rs": "a57e7ea5bf074f6a44f8c7fe5905f91014361f8a0f05df88a2cc06288f705441"
+        "🧬️mutations/🦀️.rs": "cc63f604eb5f756e5db87b741d3317e99db30e7bcf44e8294ee7753913b58afa"
       }
     },
@@ -13843,5 +13817,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "6defafa61c181a594b0bcef940df98cb3f7e923e561ec76cc7bebd9584758227"
+        "🔣️.json": "3afba978d8bd341a480e7b2e2bea2f1f7cc61df49284c1ad9b8ff93593e62fc5"
       }
     },
@@ -16293,5 +16267,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "85030105c236c6f49c9edb7ba9c1612ad36bf915e7e7a3fa1dcab66232024926"
+        "🔣️.json": "b827d8b7ca8c9b62a59310aad92e63803f087fc50a5516d865d9807c3169ec05"
       }
     },
@@ -16836,5 +16810,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "bdd90e7811f08055e7f510117decbf503c932eb701632dd877df37c77f506f31"
+        "🔣️.json": "47532fd69b7fa03b88a24d40cc0c164813d6b54f68bc8fa7c0ed9382684fe828"
       }
     },
@@ -17265,4 +17239,24 @@

       "hashes": {
         "🔣️.json": "a7f1d0e50fd5b9abfb87a2f2d436730fc3942f5a223dae4eb7513c3a6e4ee287"
+      }
+    },
+    "os.workflow.artifacts.workflow.mutation.set-node-ports": {
+      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔧set-node-ports/🧬️schema",
+      "level": "product-module",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SetNodePorts": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [
+        "os.workflow.artifacts.workflow.mutation.add-node"
+      ],
+      "hashes": {
+        "🔣️.json": "0b7ff69f8dabeca23dd99782a2a0d25c133906f83494fa8667adf27147d1336b"
       }
     },
@@ -17577,5 +17571,5 @@

         "🧬️mutations/🔣️.json": "e45a28c298c67c13abafe6ca5ac4e09b4e6ee30345d2d43018cae3eac4bc3358",
         "🧬️mutations/🟦️.ts": "de2393d41ce275ea21aed1f6b2eff312ea5ba6ed54d3d25a29bfee4b76425565",
-        "🧬️mutations/🦀️.rs": "4460c2e274bad09dee31cda0dfd7590aa5419c7fe83bc275039babc6d0cc6355"
+        "🧬️mutations/🦀️.rs": "b3b9eb359e630729a25289b86c65ddab80b8010329a18c858fb7950919720535"
       }
     },
@@ -19368,5 +19362,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "9652250b48442bccbe3aa3f7c3aa8afff2c3928cf91f7b65f4ffff166baab8cf"
+        "🔣️.json": "92e5e0999402fd4149732016e1dfffad4b191aeed32d384fef2a5a77ea7cb20f"
       }
     },
@@ -19969,4 +19963,8 @@

           "facet": "schema"
         },
+        "Interaction": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        },
         "JournalStep": {
           "file": "🔣️.json",
@@ -20040,5 +20038,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "710a18f9e4ecac7ef7c04f177d1f3a7a1f02b794cf72112e8e5da3ad86ac2a45"
+        "🔣️.json": "7a7da30e5540a2c5ba8dca3e609f40de070a988f77f106652094ac00738e0a3a"
       }
     },
@@ -20194,5 +20192,5 @@

         "🔺️diff/🛰️.proto": "34a96197523260d8a2c2b6d395f4c46d823fe027f3e77df947d7adc9e35bf909",
         "🔺️diff/🟦️.ts": "17df36fcd107dd646512c1303f1483a21c4e74cf26620ca38f70747f5171f5c7",
-        "🔺️diff/🦀️.rs": "8d583381e30b502850e6421069a5aec9b6ed41fa9827fc6da5c49a91d4eb23a8",
+        "🔺️diff/🦀️.rs": "15c92ce65d1f1e463cf2940b640c89ea126853aced9a16d979171af65225c6d2",
         "🛰️.proto": "546919916a45750106c24495e78c9fc8885878df6f4916a5ec7a88ecde48db65",
         "🟦️.ts": "b2d998b053e09a9bee4204bf403afd19083717e7b69f829f396992014caf934e",
@@ -22663,4 +22661,868 @@

       "hashes": {
         "🔣️.json": "313ed5af0a0880466e6bb8c2cc89edf408cb37f455385d3ce5903a128fa3bb43"
+      }
+    },
+    "s.architect.program.mutation.rename-quantity-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameQuantityRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "eb3843a8312ba202d5b5c22222934b7ce681f7972bf5d0b04e7305217d3b65f6"
+      }
+    },
+    "s.architect.program.mutation.rename-regulatory-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameRegulatoryRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "9c10438a6ece0d36f7c9929c428bfa06785cc251ec60c82de22adeb99acd32ea"
+      }
+    },
+    "s.architect.program.mutation.rename-relationship": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameRelationship": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "55146c95ee6558603fb9022e01f21c6e724942f67607ee3e721dd79a83918f2f"
+      }
+    },
+    "s.architect.program.mutation.rename-report-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameReportRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "abba6177baecb780aa06e47b9ef0a0266c7121df2339cc2496e2bd830a979a80"
+      }
+    },
+    "s.architect.program.mutation.rename-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "0eced5139b7e6f1f22052f69d422fa86fca874a4f8bacd33d54caa833ae9739a"
+      }
+    },
+    "s.architect.program.mutation.rename-resilience-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameResilienceRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "4269aab1478aad9740cdad5b0fbde8de3b61a9e204e59cc294ee70ba15bbfebb"
+      }
+    },
+    "s.architect.program.mutation.rename-resource": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameResource": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "fbb14e88ab388696463395041d00aa384b239b396a17894868ab46a1d7277a66"
+      }
+    },
+    "s.architect.program.mutation.rename-risk": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameRisk": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "c5bd575457039b877f747c2209a1ad3912ef922473c7e3f5276bfc06c5d42302"
+      }
+    },
+    "s.architect.program.mutation.rename-safety-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSafetyRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b7378b503249c9c723846494d23100fb6906e272b0aff3c2dd1e5d511ec52a17"
+      }
+    },
+    "s.architect.program.mutation.rename-scenario": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameScenario": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b81efa00dca5efe003b12adbe086d77354c57b5b1d60b13de0a1292c25c6df2c"
+      }
+    },
+    "s.architect.program.mutation.rename-schedule-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameScheduleRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "aa5c22ec5f4019f3b9d71c71f702efb23182d1475d9be5514451319af023492d"
+      }
+    },
+    "s.architect.program.mutation.rename-search-filter": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSearchFilter": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "0002c3e54d0edbc5e695edf78b66fb5146696fb0234db4d06ae13089d45413f1"
+      }
+    },
+    "s.architect.program.mutation.rename-security-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSecurityRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "ee03956f525b3a1b1a197629df9f8708021e005da6186d7a7ddfae5916617105"
+      }
+    },
+    "s.architect.program.mutation.rename-service-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameServiceRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b43c29abff19bece28c8cd6b2f72c0c609ca394983ae37361f5444b44d8f4ee7"
+      }
+    },
+    "s.architect.program.mutation.rename-site-context": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSiteContext": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "4bccd331b144456cfcb2f0102cbf946690896238b892fad6870b84d24a5adfa5"
+      }
+    },
+    "s.architect.program.mutation.rename-stakeholder": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameStakeholder": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "33210b77f6ca7d04fe4522619ad97365d6aa7800d995001da6aed2d18ccd9bb5"
+      }
+    },
+    "s.architect.program.mutation.rename-status-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameStatusRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "db8edfad9d4183a0fa7c83b8aa2e5bf5b61c6991fd3e85435c6e1bec1da23f2a"
+      }
+    },
+    "s.architect.program.mutation.rename-storage-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameStorageRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "a1700334d70c871e487c744676fca45c9dd5d55602db972cbfd662b5822d73b2"
+      }
+    },
+    "s.architect.program.mutation.rename-survey": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSurvey": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "ed85ffa6ac29c6f3a183d8656fd33e9b1fba2d24caa18342725b92769bb96a38"
+      }
+    },
+    "s.architect.program.mutation.rename-sustainability-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameSustainabilityRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "2201631995d9193dcad64c29f9e394a61c6a548659f124d81b06f9e6222ad2c6"
+      }
+    },
+    "s.architect.program.mutation.rename-template-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameTemplateRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6a25384ae5f519576dc1c49bf39dc82478d895be6fee4873ca36633885fd8632"
+      }
+    },
+    "s.architect.program.mutation.rename-user-profile": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameUserProfile": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "1f46b9873ef8e1324b98bee8386e3bc6c64fdf78a910ec784766de0082481f06"
+      }
+    },
+    "s.architect.program.mutation.rename-validation-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameValidationRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6304eef2f7310cfdb663f3f76dc96986eb13755ad9cd407086c0cb78d4f60b01"
+      }
+    },
+    "s.architect.program.mutation.rename-wayfinding-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameWayfindingRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6bc8cc98de3e2cde5385444cab5fe99a57cc586b1f5941652df5715e41e3232e"
+      }
+    },
+    "s.architect.program.mutation.rename-workshop": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RenameWorkshop": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "2dd8b86904950171f7e7898d4b681b0d5ae4db2fb0ced04b116018f3f556966b"
+      }
+    },
+    "s.architect.program.mutation.replace-access-rule": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceAccessRule": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "7fd14a2869a5c3685df7077e90003457dd3e2016c459e039fbf5289539710585"
+      }
+    },
+    "s.architect.program.mutation.replace-accessibility-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceAccessibilityRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "76407c24cb0b2201d4ba7f806bd46d8a98578b8b9a43e3131643f510f8022151"
+      }
+    },
+    "s.architect.program.mutation.replace-activity": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceActivity": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "a8e3fdecb075468e921064681c9ee95f5a52bb5ab5ebd0a2767ae0b7174e39db"
+      }
+    },
+    "s.architect.program.mutation.replace-analysis-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceAnalysisRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "9e9c311b92674d9bb936d2d2ffc9f8b84d96706b68ac55611ae025f67130ef87"
+      }
+    },
+    "s.architect.program.mutation.replace-approval-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceApprovalRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b1773f1c94072679115a265bf023607d0282c233314f9a48694f9a70d937d7ed"
+      }
+    },
+    "s.architect.program.mutation.replace-assumption": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceAssumption": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "8910dbc12b82d3dbe9336f5517c4a7be506f6a9a0898110438ba9213951476cc"
+      }
+    },
+    "s.architect.program.mutation.replace-audit-event": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceAuditEvent": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "1756c6976ebbc9385543b47141c24bcfb6a275db428155674a05985592e6e1fb"
+      }
+    },
+    "s.architect.program.mutation.replace-benchmark-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceBenchmarkRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "a416b08331749f16e8e8a46616caeccfde09986cfbeb8aa1fbfef9693dd9d69b"
+      }
+    },
+    "s.architect.program.mutation.replace-change-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceChangeRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "28c819459de854a8dae6e255ec2197966fb9a393221b6ea06dca64dc1db4d4a2"
+      }
+    },
+    "s.architect.program.mutation.replace-collaboration-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceCollaborationRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "a9d952739ab8b59de37fdc1ca599fff27bede36c8ab4066bb0df8af616506fd5"
+      }
+    },
+    "s.architect.program.mutation.replace-communication-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceCommunicationRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "f6c80645864c88ba13a9fb07c3eb55d9c6088747ebcc8122c1681679a33e38d2"
+      }
+    },
+    "s.architect.program.mutation.replace-compliance-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceComplianceRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "37588b549d0f20682c1620a29f1a8d1a16a2b21cdb95e1d785c597a6bd6b397c"
+      }
+    },
+    "s.architect.program.mutation.replace-conflict": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceConflict": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "fea604df0ac6aa884280086c3481f47bab332ba40351c1be7042a697fbf6e584"
+      }
+    },
+    "s.architect.program.mutation.replace-constraint-record": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceConstraintRecord": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "ba417a3f8ce59bfd7fe292f0d27b5d4baaa23bf0ddd26e140608f81210bd7ddb"
+      }
+    },
+    "s.architect.program.mutation.replace-cost-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceCostRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "e2acede4599b2a71faa57ea1c3cc49f028a2044bac8ca7bc57c73cec3b12a0e0"
+      }
+    },
+    "s.architect.program.mutation.replace-decision": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceDecision": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "31c3416d42852eacde60979d5e2d04e485f3784d971cb6f8500b166122268ba0"
+      }
+    },
+    "s.architect.program.mutation.replace-delivery-constraint": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceDeliveryConstraint": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "065b5766e46369e3b07e1a340cecea0ab07857c0fcf2638a6bce6bf1062b7cd4"
+      }
+    },
+    "s.architect.program.mutation.replace-document": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceDocument": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "08308f7181a22c0ac64975772cc7919036fe21191f30990da2fe8a01d9fb7af3"
+      }
+    },
+    "s.architect.program.mutation.replace-environmental-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceEnvironmentalRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "226e349153c896164a90c8491b9b7525fa7baa49c8f8cafb2df265a558412d44"
+      }
+    },
+    "s.architect.program.mutation.replace-equipment": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceEquipment": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "cf70d9861ddee27320ae1c5eae59534d18c5ef5ed83c3e6ce127446489127757"
+      }
+    },
+    "s.architect.program.mutation.replace-flexibility-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceFlexibilityRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "64eac554df8d245f77dfdf0284458c8333b006b3edc4e83994138296eba11ba9"
+      }
+    },
+    "s.architect.program.mutation.replace-flow-requirement": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceFlowRequirement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "9420cb6aecd9739b0768503c572cab2d07782c6bb8d5bc2a40e9c318272b02e0"
+      }
+    },
+    "s.architect.program.mutation.replace-function": {
+      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ReplaceFunction": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6257fe963bb168d266957eed5d35bb287ac0bf37c72bcbafcd78f05151b2c000"
       }
     },
@@ -28270,5 +29132,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "3408a4ede3d7d4ae4f195d1c104f20d01b55f3f1597dd0586cca0d1fcf880574"
+        "🔣️.json": "ff6a72de43d5e5fd7a072671f736615556c7ccbdb4d797301bcdc01b131d18c6"
       }
     },
@@ -28615,5 +29477,5 @@

         "🔺️diff/🛰️.proto": "26fdd019d5b5acb340a2937119c63cd8be37c845e6a77b02fe1aa3f8a5e07e3f",
         "🔺️diff/🟦️.ts": "1f38fee22b9bd7dce62b57b3766f3b35fad0104a22e19413349bd80ae5f15f71",
-        "🔺️diff/🦀️.rs": "a59ea2478b43b7ad42ff2b21a4c40f2a6807799ba123643079b8bde3a92b09bc",
+        "🔺️diff/🦀️.rs": "f1667a1e68dc6e64b2ae9c4023e7b609094f6b80a1e4950a35aa9bd984863801",
         "🛰️.proto": "75fe2424d37d26449bf1154401af558f4fda281634b56ca869b55a01e7b259f6",
         "🟦️.ts": "7e74aa4722e56a8a1fb50528f59f5484ee5c8b35830a444f10f6033924551478",
@@ -29412,5 +30274,5 @@

         "🔺️diff/🛰️.proto": "5f9b5f7f342a8ad4114d4ed78d2cb43bb7795f79ddabd48eead8901c2caa012e",
         "🔺️diff/🟦️.ts": "5d8c48fbb6957ec07d9383197277e7aa698dd6c4d48e2151181b46bf104d30c4",
-        "🔺️diff/🦀️.rs": "c46aa81f73c127c8f5bf757527f475f1bc4bf368e5596607e2e6af1ad4f0939a",
+        "🔺️diff/🦀️.rs": "857e02241429fadb5122d496f02d7cfb3dcf3a195773c2e7f2f6c10b0ff94856",
         "🛰️.proto": "22bbde10ed286330c93ad36b09d934f53ab27b7b2fa713ceeeaf865869d8f61d",
         "🟦️.ts": "aae83ec7523d720784342c05207057bb5b07a8aa81a9bcec00836c536bec9f38",
@@ -30434,5 +31296,5 @@

         "🔺️diff/🛰️.proto": "b456a173fcc449c5fe2c20f48c261ee96423b2aec3da6d655ede9042438d0ce0",
         "🔺️diff/🟦️.ts": "9e3cb67859bbcc4a2c4381504274ed69717c011b4daa41268bf351afd445f843",
-        "🔺️diff/🦀️.rs": "efa9148ad048c978290109a1fe54d04039df007cb2649f82a0b577da1a4d4d9b",
+        "🔺️diff/🦀️.rs": "7a1b4a878a86ea2ce9183cc55251c72af13ef94d38b0775b347dca23d84e1182",
         "🛰️.proto": "1e5134e2eddda59f648dab666e992c0bf163e21dd10bad6087570aa58014ede5",
         "🟦️.ts": "b92db8d7e3b72dd76ad84e165bde05da668f0a870ad26a960a8bd5c12237eba1",
@@ -31410,5 +32272,5 @@

         "🔺️diff/🛰️.proto": "a6e1aa2568cd89344cc3940dc6a391eeb7314db26f60e143de31799fee9ba87f",
         "🔺️diff/🟦️.ts": "f8228ff73de006fff3ae53bcca5d4aa8c75daba7840f7b3f4931c8fa64520130",
-        "🔺️diff/🦀️.rs": "6a2a1338ef6fdfa4ca60a2292ff9fc449da237d5ff279dae5c01f7c8a754bd22",
+        "🔺️diff/🦀️.rs": "b51757d326d64e1b58c20ba37387e9dff134bb1307d78a65455e61947d49bd01",
         "🛰️.proto": "79cf066a9b629f2162da7f455fd1c6c28241677229627fb7559874a1fe6e945f",
         "🟦️.ts": "3b5fa672e10970483387c099eb20ae0da21ce3c22325ea838d9d9b16075ac518",
@@ -31418,5 +32280,5 @@

         "🧬️mutations/🛰️.proto": "358afb1c0279d96ad3633d66979c01c129a907bc1c276213299a5958fb34186a",
         "🧬️mutations/🟦️.ts": "de44e13d482b96a3931719a265451eeb60f96af50bbf75eae20b6d8cfbe1dfd7",
-        "🧬️mutations/🦀️.rs": "6148094477083abf75afa73e6d66125c1a05043fee91d5a48a3534d03fdaf535"
+        "🧬️mutations/🦀️.rs": "803734dbe467695c12dcc1bb961545a32585cbc1c1d04d89ece9c173383f3de2"
       }
     },
@@ -31850,5 +32712,5 @@

         "🔺️diff/🛰️.proto": "5831febcf602619581a7a9bc1d403b2144a9c2fb8bc7815f7c276fd71ab8c6a1",
         "🔺️diff/🟦️.ts": "594ddb5e4147c1e46e07b11e01cf952642f1e79913ce647b63a9d6f9bf1a9c80",
-        "🔺️diff/🦀️.rs": "f5b66f8072b0d21f38626bb2661c89a78935267c1ce29755e8fb2a64e4686a37",
+        "🔺️diff/🦀️.rs": "7a8db0a3c51a415abd1a06ad64bb3ebc29b48ab16d4a8dc6f1ba3327449061eb",
         "🛰️.proto": "e13b8f3488ebe7d75392e5335063c7e8dac84e346210dc027704df692089b9e8",
         "🟦️.ts": "dfb3967aad645c6453a04f9e70698defd4a6715e8365adbc01bdb42487f70823",
@@ -31914,5 +32776,5 @@

         "🔺️diff/🛰️.proto": "a393a3c1df2daa80b19d4570fa1daab7d5470653b3f53e22b0d8f850972ed771",
         "🔺️diff/🟦️.ts": "d453cf4e5b97944027aca617148b6227347e9e4b223f6eceb7ed4f5dd00a8aa5",
-        "🔺️diff/🦀️.rs": "3df3fdd0b40dbc2b788a8b46159f63cbadb03598d12042bd15e0ca0b40eba83a",
+        "🔺️diff/🦀️.rs": "fceb54dab5550db759702448239ab84cd30b6cd555b131f444e53c4a4c5d0c76",
         "🛰️.proto": "f1ffef8f26e3d8323f5e79a7f3ee7c8084fb599009ce60500359e6e7c2116839",
         "🟦️.ts": "de295dcae804c26def9396c77caea9cff323b50f6c2bc67163daec18d8225f4c",
@@ -32303,5 +33165,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "388b4383837c645964abc70aee20ef744f08e34d25aa0cf545c779770000a218"
+        "🔣️.json": "b19ef98c4db652a933aae7c7a6e9471e863360767a007b06be941c773df7b141"
       }
     },
@@ -32448,5 +33310,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "b4764f5b7184c97bf7221b38b619eba298444aee4f9778a3ce0e27d603bcce73"
+        "🔣️.json": "1b2ff9b4b7065b47d5bd4a1f02b325eaa1c57c728504f903a5019c5e9c6fa81f"
       }
     },
@@ -32665,12 +33527,12 @@

         "🔺️diff/🛰️.proto": "ff059c9371268856bf179aa907bf70e7e96968112ca01c8c79c62c4a643a1c7c",
         "🔺️diff/🟦️.ts": "80752cfbe33aa490a3e0894e208cabb1e68fee81054641a241da11cfef702ece",
-        "🔺️diff/🦀️.rs": "c8015d3a4610c23dca7b001e25d8cc36e384c5f65a1aa3118289462befc26b9f",
+        "🔺️diff/🦀️.rs": "b9ccfdb15aa89129d50345918a7197606b3319cbd53c7c76f1a89aae4de95dfd",
         "🛰️.proto": "24e7b2592892b413a2b1aad0b6ac2d4d1ca112ef6470128d191ec15c9611fb70",
         "🟦️.ts": "a974927ebd2fba5659b39faaecac5fcc2cdb54ad6626728197aefb4b49de00c4",
         "🦀️.rs": "0c7443b32bf6df50b8b503ffbd1c08785cb09860d207b389bce5d7982bcba331",
         "🧬️mutations/🔗️.graphql": "ed296cb480745e6e8a665921385736ea01b352121f559ecaa550e926da5b87ed",
-        "🧬️mutations/🔣️.json": "7a8c66f2a9fc96fb2a23825754798a31bab32e15e07dc26eec0bcf856a33ac0c",
+        "🧬️mutations/🔣️.json": "34a7641346474f2fe78a4bf3c9cf76a896b2e9f650a68b9d255577bbf75f86b2",
         "🧬️mutations/🛰️.proto": "65ac78e019b7c933c6da6375d25a8cf3fd59fc248c3b037e66ec371ed6555057",
-        "🧬️mutations/🟦️.ts": "0e1dc236af6bb7b73a60d349527e2ec6d4060b61ced441d59ada24563e2fd2ce",
+        "🧬️mutations/🟦️.ts": "2897abfefabce7639724eb5f007fa79b0f115fe401362ab314009e049233b2b4",
         "🧬️mutations/🦀️.rs": "2bdff434e960915a13c35648ec3ba21f6e2886d89fab4f14396fe1b6df331db6"
       }
@@ -33573,5 +34435,5 @@

         "🔺️diff/🛰️.proto": "1d79141cad1cbda900c7833e0453ba74f5b9acaeb0b946a48efcdaf19376dfaf",
         "🔺️diff/🟦️.ts": "4604bf37459d82e0c50eda0cbb0fa57eb4607634ff808660bc0f525b63d03d4f",
-        "🔺️diff/🦀️.rs": "265ef067b18848db8e580473735c23028248f1d15c77b3d36d9fe6a66c7c9fd4",
+        "🔺️diff/🦀️.rs": "7df8028db7798c9497a58aad9d91bab3e2ca2a4245632b523ba5b510655e461d",
         "🛰️.proto": "b04dec86b7de8500a608f92b13cd84edef79ea2c8b21fddcd47969d8bdc170bf",
         "🟦️.ts": "81c18ff3ab8df86807f144dd02ebe493d684f3745bb3f50928a41aa5d7e17d6c",
@@ -37577,5 +38439,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "4ff8a3cad07a0732d5e902cb806344c78db86947f803d3ae957225d45dbc8d54"
+        "🔣️.json": "c357c2a96b7bdad1561873855de2eb1ed6a2da4a8b6a01c37e664b7539e3ca95"
       }
     },
@@ -39068,5 +39930,5 @@

         "🔺️diff/🛰️.proto": "0b93c274fa9a15629f21d23fa378733a16b56fc4357f7d082ba81d8166a69c9e",
         "🔺️diff/🟦️.ts": "6c92f381c6e7a964027531e0370f7c26280f47d2fe57465660cb1e73ca4a2c0d",
-        "🔺️diff/🦀️.rs": "e1c7e53a721e2d5d1339740d74c27d7d3156574789f869d9c14617682ad06855",
+        "🔺️diff/🦀️.rs": "d0bdbc6f8e22dd066f6e369ed1e8ec648ea33d3d622d75d693b66f452d1763e9",
         "🛰️.proto": "a9f3532c9f9b9067e46ecfedbc0ae3457927699c970b174357e296b0153ae3d9",
         "🟦️.ts": "4524a473cadb832b35a9e5c9efbe217df6ab9f607f8569e72810abeb684ba6d4",
@@ -39127,4 +39989,22 @@

       }
     },
+    "s.fem.2d.1.boundary.mutation.change-support": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🩹️change-support/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeSupport": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6848498779bb9a3cf8422a0669bc5264d0c36b85e23225840613bdab2f450fe2"
+      }
+    },
     "s.fem.2d.1.boundary.mutation.create-support": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧬️schema",
@@ -39142,5 +40022,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "d35c6e3cd9fa154e946edb46e9f7c57927cd794936bd5445420ad1a118c96d32"
+        "🔣️.json": "2becdfed19ed1f789dd1283cf4c40c743485336c2b564fdeadcc60da496fb47b"
       }
     },
@@ -39178,5 +40058,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "16aaf212d4a4dcc143be0f02ed6dce083fb0dba9da0c2a0ad1eb74cae1d30772"
+        "🔣️.json": "8e94c0a349ef298bf5713566c404c251e15bd5c1094be0f7f8d5e06ff96035f3"
       }
     },
@@ -39214,4 +40094,58 @@

       }
     },
+    "s.fem.2d.1.load.mutation.change-combination-name": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-combination-name/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeCombinationName": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b73406fc015dcd5b06c93f92c44712368699452143ad15a67b4e20f0c6b7a911"
+      }
+    },
+    "s.fem.2d.1.load.mutation.change-combination-term": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-combination-term/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeCombinationTerm": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "27c2ba854be346cca219e78db9255714229d36f6a61fa45e6eed211f4c37711e"
+      }
+    },
+    "s.fem.2d.1.load.mutation.change-load": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-load/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeLoad": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "75c0f8046994b2f844c81774e1007a342b5c4c339cecfd434aab09b85de335d7"
+      }
+    },
     "s.fem.2d.1.load.mutation.change-load-case-name": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🏷️change-load-case-name/🧬️schema",
@@ -39391,4 +40325,22 @@

       }
     },
+    "s.fem.2d.1.material.mutation.change-material": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🩹️change-material/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeMaterial": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "f29ba35b8e60c48a7f3824ac957ee4c781af2a05db45997955411677ca8162ac"
+      }
+    },
     "s.fem.2d.1.material.mutation.create-material": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧬️schema",
@@ -39460,4 +40412,76 @@

       }
     },
+    "s.fem.2d.1.mesh.mutation.change-element": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-element/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeElement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "ba8513292cf152af45eb43f301c721a63c1c90fdcebd2c5e838d893c8353215d"
+      }
+    },
+    "s.fem.2d.1.mesh.mutation.change-node": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-node/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeNode": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "e54bf4e26d000110982c7a9bca838be9595a098f181603e56745f12a57aa0637"
+      }
+    },
+    "s.fem.2d.1.mesh.mutation.change-region": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-region/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeRegion": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "b7b9d5a9e310f6417347f6d6bc3a13bd52d966af46b68c5b8f3332b3ffea69aa"
+      }
+    },
+    "s.fem.2d.1.mesh.mutation.change-section": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-section/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeSection": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "5f66ffac069e7812e33540d1b9ea95a71e45231ac72a07ceb3563c98b9416843"
+      }
+    },
     "s.fem.2d.1.mesh.mutation.create-element": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧬️schema",
@@ -39742,4 +40766,22 @@

       }
     },
+    "s.fem.3d.1.boundary.mutation.change-support": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🩹️change-support/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeSupport": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "8de66cadde1c7c90127a234ac0a9a340e06f4bd36533626705d1ec3f448a37cf"
+      }
+    },
     "s.fem.3d.1.boundary.mutation.create-support": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧬️schema",
@@ -39757,5 +40799,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "37abd27022212d8cf3fed100160e4872f3f1fcfd23d3b2537083e3e3fcb64111"
+        "🔣️.json": "f0a08a1735e741d6d29e2a6b0ceacf65ac915e40c1c5c27abf65db5d74051314"
       }
     },
@@ -39793,5 +40835,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "c9df01b0d2a772257fc06c92e1c62c76363274c710c2fd3e971de70656d3062a"
+        "🔣️.json": "e3ecb7f9b8fd15846635101b1647ff076ac1d736f0c1e363fd217edcc6799a76"
       }
     },
@@ -39829,4 +40871,58 @@

       }
     },
+    "s.fem.3d.1.load.mutation.change-combination-name": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-combination-name/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeCombinationName": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "e449ed26630c17000a977ebbe7cf4b947ba74614428b2ff137ab17f5b7a9a282"
+      }
+    },
+    "s.fem.3d.1.load.mutation.change-combination-term": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-combination-term/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeCombinationTerm": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "d487765f117dc2db864dad378f67782a602b7d54327f3eb6ade8f30a74ba7b8d"
+      }
+    },
+    "s.fem.3d.1.load.mutation.change-load": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🩹️change-load/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeLoad": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "5cb13947479d34d641c29538bef7ea3bbdd7639a59744dc54bd892479c50906d"
+      }
+    },
     "s.fem.3d.1.load.mutation.change-load-case-name": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🏷️change-load-case-name/🧬️schema",
@@ -40006,4 +41102,22 @@

       }
     },
+    "s.fem.3d.1.material.mutation.change-material": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🩹️change-material/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeMaterial": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "6f2585ff6cc8f9075df10593fb019ea4392f455aef997be8a98165f3a1b0a9e0"
+      }
+    },
     "s.fem.3d.1.material.mutation.create-material": {
       "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧬️schema",
@@ -40073,4 +41187,76 @@

       "hashes": {
         "🔣️.json": "247250c770500776c809764b6f789bc113127d548263c26cc2fa8908a4232533"
+      }
+    },
+    "s.fem.3d.1.mesh.mutation.change-element": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-element/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeElement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "76626489af44758e3b92ef338b41dc56c85c6a78fd9b072c01750922155fac7d"
+      }
+    },
+    "s.fem.3d.1.mesh.mutation.change-node": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-node/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeNode": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "e6b4556f0f48381b02e217f940b637b7f8cadca4fec7ced4ca02ed690fe513c7"
+      }
+    },
+    "s.fem.3d.1.mesh.mutation.change-section": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-section/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeSection": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "c8a1fd60a04f927e7da4d39776206271d6d493db778ac8d798249313af99d1d6"
+      }
+    },
+    "s.fem.3d.1.mesh.mutation.change-solid": {
+      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🩹️change-solid/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ChangeSolid": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "3e956c42ed1bbb48da1fd4b1b4d1eb6fd62c1a2889e8b172e11b6de1471b87d5"
       }
     },
@@ -40359,4 +41545,8 @@

           "facet": "diff"
         },
+        "Fem2dCombinationPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dCombinationRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40367,4 +41557,28 @@

           "facet": "diff"
         },
+        "Fem2dCombinationTermInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dCombinationTermModification": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dCombinationTermPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dCombinationTermRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dCombinationTermRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dCombinationTermsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dCombinationsDelta": {
           "file": "🔺️diff/🔣️.json",
@@ -40380,4 +41594,8 @@

         },
         "Fem2dElementInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dElementPatch": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -40431,4 +41649,8 @@

           "facet": "diff"
         },
+        "Fem2dLoadPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dLoadRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40451,4 +41673,8 @@

           "facet": "diff"
         },
+        "Fem2dMaterialPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dMaterialRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40471,4 +41697,8 @@

           "facet": "diff"
         },
+        "Fem2dNodePatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dNodeRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40491,4 +41721,8 @@

           "facet": "diff"
         },
+        "Fem2dRegionPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dRegionRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40508,4 +41742,8 @@

         },
         "Fem2dSectionInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem2dSectionPatch": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -40535,4 +41773,8 @@

           "facet": "diff"
         },
+        "Fem2dSupportPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem2dSupportRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40556,42 +41798,58 @@

         },
         "FemCombination": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemCombinationTerm": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemDof": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemElement": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemLoad": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemLoadCase": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemMaterial": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemNode": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemRegion": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemSection": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemSupport": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         }
       },
       "dependsOn": [
         "s.fem.2d.1.analysis.mutation.update-analysis-settings",
+        "s.fem.2d.1.boundary.mutation.change-support",
         "s.fem.2d.1.boundary.mutation.create-support",
         "s.fem.2d.1.boundary.mutation.delete-support",
         "s.fem.2d.1.boundary.mutation.replace-support",
         "s.fem.2d.1.load.mutation.add-load",
+        "s.fem.2d.1.load.mutation.change-combination-name",
+        "s.fem.2d.1.load.mutation.change-combination-term",
+        "s.fem.2d.1.load.mutation.change-load",
         "s.fem.2d.1.load.mutation.change-load-case-name",
         "s.fem.2d.1.load.mutation.change-load-case-self-weight",
@@ -40603,7 +41861,12 @@

         "s.fem.2d.1.load.mutation.replace-combination",
         "s.fem.2d.1.load.mutation.replace-load",
+        "s.fem.2d.1.material.mutation.change-material",
         "s.fem.2d.1.material.mutation.create-material",
         "s.fem.2d.1.material.mutation.delete-material",
         "s.fem.2d.1.material.mutation.replace-material",
+        "s.fem.2d.1.mesh.mutation.change-element",
+        "s.fem.2d.1.mesh.mutation.change-node",
+        "s.fem.2d.1.mesh.mutation.change-region",
+        "s.fem.2d.1.mesh.mutation.change-section",
         "s.fem.2d.1.mesh.mutation.create-element",
         "s.fem.2d.1.mesh.mutation.create-node",
@@ -40632,10 +41895,10 @@

         "📸️snapshot/🦀️.rs": "7a483ced4adaaeedd80f59a907d6499128ed013e9c8536addf0d981f14695162",
         "🔗️.graphql": "1031a91cf08a64d62438ced32e1cdec34c3c926516990cce92bb197f7d5ff5f7",
-        "🔣️.json": "6d229d684bcfc16ef2638d2e5b87bcf06ffabd25ce40305e81d55b5a747d8045",
-        "🔺️diff/🔗️.graphql": "fb867e314611f6313fd0cc5cc9ec2d4345d6098e9ea4bd33bbb8c65015330cc5",
-        "🔺️diff/🔣️.json": "70addea335db851f50de5aae2c190378f3938c0fccf76920c01979284e0b966e",
-        "🔺️diff/🛰️.proto": "dd555f40e92abaeb3d934f58890f084d09f24f52337889eb262caee9acc4d502",
-        "🔺️diff/🟦️.ts": "922be38d883db4f1a2ae825bc06603968042a8ef7a31e442eb426606de7ebc62",
-        "🔺️diff/🦀️.rs": "057737901001b710f60eddf2e7cd7d993d321325a6877c5a1ca28ad05130fecd",
+        "🔣️.json": "fbb31ba3964a2be6952b4dea317c63ff0593cee5c71d2ad0a8a69bd2275d5baa",
+        "🔺️diff/🔗️.graphql": "f399d161c3289de61cf0a1737931a98dcb20038a4a728b882b366897d950c27b",
+        "🔺️diff/🔣️.json": "c78e089059124a202aba5a6a0ab442adde0572e77ebe11c6f555e07df5c2d871",
+        "🔺️diff/🛰️.proto": "42e4b6c52b30b2cf0d7df640685b522fda005caa01820b2d51e9d6b3772fbdee",
+        "🔺️diff/🟦️.ts": "b1f4ca2c5f6dfb4418139e4f34984faff916fb53fc1890738603d5b0c8487f1c",
+        "🔺️diff/🦀️.rs": "fc2fa93c3982d8a311c5b214d97561cb935ccb069a6641de3250a702cb7fa79f",
         "🛰️.proto": "9003d0cc41c475e37d2fd6e8ae40288407a46eecb8ec33ea0c3e44e2b1181f36",
         "🟦️.ts": "d79a5e4f78604f42673545ac182e539b074d43014f1166b3cc6d0ccb1c56d891",
@@ -40680,4 +41943,8 @@

           "facet": "diff"
         },
+        "Fem3dCombinationPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dCombinationRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40688,4 +41955,8 @@

           "facet": "diff"
         },
+        "Fem3dCombinationTermsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dCombinationsDelta": {
           "file": "🔺️diff/🔣️.json",
@@ -40701,4 +41972,8 @@

         },
         "Fem3dElementInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem3dElementPatch": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -40752,4 +42027,8 @@

           "facet": "diff"
         },
+        "Fem3dLoadPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dLoadRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40772,4 +42051,8 @@

           "facet": "diff"
         },
+        "Fem3dMaterialPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dMaterialRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40792,4 +42075,8 @@

           "facet": "diff"
         },
+        "Fem3dNodePatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dNodeRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40809,4 +42096,8 @@

         },
         "Fem3dSectionInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "Fem3dSectionPatch": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -40836,4 +42127,8 @@

           "facet": "diff"
         },
+        "Fem3dSolidPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dSolidRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40856,4 +42151,8 @@

           "facet": "diff"
         },
+        "Fem3dSupportPatch": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "Fem3dSupportRelocation": {
           "file": "🔺️diff/🔣️.json",
@@ -40873,46 +42172,62 @@

         },
         "FemAnalysisSettings": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemAxis": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemCombination": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemDof": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemElement": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FemLoad": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemLoadCase": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemMaterial": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemNode": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemSection": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemSolid": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         },
         "FemSupport": {
-          "file": "🔣️.json",
-          "facet": "artifact"
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         }
       },
       "dependsOn": [
         "s.fem.3d.1.analysis.mutation.update-analysis-settings",
+        "s.fem.3d.1.boundary.mutation.change-support",
         "s.fem.3d.1.boundary.mutation.create-support",
         "s.fem.3d.1.boundary.mutation.delete-support",
         "s.fem.3d.1.boundary.mutation.replace-support",
         "s.fem.3d.1.load.mutation.add-load",
+        "s.fem.3d.1.load.mutation.change-combination-name",
+        "s.fem.3d.1.load.mutation.change-combination-term",
+        "s.fem.3d.1.load.mutation.change-load",
         "s.fem.3d.1.load.mutation.change-load-case-name",
         "s.fem.3d.1.load.mutation.change-load-case-self-weight",
@@ -40924,7 +42239,12 @@

         "s.fem.3d.1.load.mutation.replace-combination",
         "s.fem.3d.1.load.mutation.replace-load",
+        "s.fem.3d.1.material.mutation.change-material",
         "s.fem.3d.1.material.mutation.create-material",
         "s.fem.3d.1.material.mutation.delete-material",
         "s.fem.3d.1.material.mutation.replace-material",
+        "s.fem.3d.1.mesh.mutation.change-element",
+        "s.fem.3d.1.mesh.mutation.change-node",
+        "s.fem.3d.1.mesh.mutation.change-section",
+        "s.fem.3d.1.mesh.mutation.change-solid",
         "s.fem.3d.1.mesh.mutation.create-element",
         "s.fem.3d.1.mesh.mutation.create-node",
@@ -40953,10 +42273,10 @@

         "📸️snapshot/🦀️.rs": "f8c91a7de6de756050d66df2666d6995173409bad55e193fa7fcd614a7f120a7",
         "🔗️.graphql": "71f15841b8617144edb3bd679a9af4d09ec525d54cde32402c65f28869b915fa",
-        "🔣️.json": "e21a2c2be6cb37435f6a9f3eff558895f5b2bd525073f7dab62902144c2eeada",
-        "🔺️diff/🔗️.graphql": "37ec92bef9d63003780d6bf56c756484563155f1d07b45162e7abfc145992df0",
-        "🔺️diff/🔣️.json": "f931d7011939b9b43b64f6b67172e13858d464ea8325e3d67cb6b240072408f0",
-        "🔺️diff/🛰️.proto": "20256026e82df3d62618065b8c9536466dec1e71fdc1e5359267ef55e59dac5d",
-        "🔺️diff/🟦️.ts": "0932a55047a0895aef95ae5ffd32b5f5d895222cc6db3bc3898afec77eea1428",
-        "🔺️diff/🦀️.rs": "eca81a1f95dffb15aa3781b5f37fab8f3524859c8c9068eb2be0b426ac5ead3a",
+        "🔣️.json": "efadbd509d3b1c916fdfe2d8a2539c4cb3db797f06df4dd72e36107beb88f149",
+        "🔺️diff/🔗️.graphql": "09c615148f54979472b6dc95604e149f2f2ab812a068defcb0282d36062364f8",
+        "🔺️diff/🔣️.json": "34e28fe63dadc4ebc51afc91bfb183d50059a656e1eb48fb0078559c16215270",
+        "🔺️diff/🛰️.proto": "3150e666e942f6f33078fcb9c40c729f7dac3ab88a0d4cc217e370cc859a374b",
+        "🔺️diff/🟦️.ts": "657852502f5a489dd01a6037a0d6fc343a0e76d3213a0c87592a77e49bef9d9d",
+        "🔺️diff/🦀️.rs": "d9af54e02247160545745d02c517dba391ef345c7f4499277fb2ecec060a3f2b",
         "🛰️.proto": "1c1cc9338449a20ba8c0fb46decfae1726f64f199a4274d683629b25db9a7d6c",
         "🟦️.ts": "b77cff83038b5d579f6c5f9dc9b1005ebf14738794c2e04656baaa244b1b4b82",
@@ -41044,5 +42364,5 @@

         "🔺️diff/🛰️.proto": "cf050f1779ba71c6a764861ef426ba8d7c59dcba14bf76830fcb22280a024cb6",
         "🔺️diff/🟦️.ts": "31144f24564f4b7a15a24ceef278c3aa2570394a5bd32e8c566e9b27711570a2",
-        "🔺️diff/🦀️.rs": "af856adf224114438db08193237dc24b184eba9b40074255795b9a0f4ad78f24",
+        "🔺️diff/🦀️.rs": "e475af1801386db1271c9fd3ddc39f6a6de838a5241bb4c15d7a17d57020cb81",
         "🛰️.proto": "6eb758088c8607e9d86bf883712a0fb9ea9a9a5f67fbcd8c7ed9b385ebdcc505",
         "🟦️.ts": "21cfc649dc5fb7dcb7667ed4cc319253c9b4d5be0f8a6300612e0047418dbb2a",
@@ -41192,5 +42512,5 @@

         "🔺️diff/🛰️.proto": "521af390b86acfdd74b139101725ce3b5c75e770f0fb64c2745fc6b6fa67435f",
         "🔺️diff/🟦️.ts": "a330d5b8d0561c5c308e55c13ba46a63e8f6a001ebb4cec7413eb132349b8896",
-        "🔺️diff/🦀️.rs": "bc6e05b3d7ea60c362f393c40007ac9359109998c29aa9fec4bb94e16a5a0ef9",
+        "🔺️diff/🦀️.rs": "7836c74a0252aa984fae32d7e57d3caa1de5ed10d252fbd0dca31bdf57fe6b11",
         "🛰️.proto": "92270379a17b5e7cd465fd855ef65fd29326bbe0044f5d29b57d878a5e1e8d24",
         "🟦️.ts": "a399b2dff3237c5201f9096f1988a748900ae87acd83842e0ddabaa81a8403b0",
@@ -41528,5 +42848,5 @@

         "🔺️diff/🛰️.proto": "8ce8c880c298523a57b1814eb906e7f84de653a1a24aeacd38b29bd56c348915",
         "🔺️diff/🟦️.ts": "fff500e6da20c066863465b71a63be2713db28ff1662928c1d7f26b53e0e01ba",
-        "🔺️diff/🦀️.rs": "9bade2e9cf6b660a8bddba7e77d41d1c7e3a49798f8d1b93b2824765e902aca4",
+        "🔺️diff/🦀️.rs": "680b4c2bcb643b14aa874313275aaca8a08263e4f08d2cfe591cadba3ff45af3",
         "🛰️.proto": "ebb17d69ebaa90e77f0493ac27c96f6ce6c7fb3e867ea5c7eeac9b7e023ee62f",
         "🟦️.ts": "19e7d706bfb55a802fd4cf7303f9c8d29ba8a0fd323100d74417c37483b59730",
@@ -42018,5 +43338,5 @@

         "🔺️diff/🛰️.proto": "f6533942ba4dd8d586d209476f88c728adac393bae9e8812ab47aa6449c22737",
         "🔺️diff/🟦️.ts": "1c07720b43a6091cc233c5896304747f0a0a446b618d854730d36cca74e5a537",
-        "🔺️diff/🦀️.rs": "761c571bb735e2fe1a514560c0b5476c282679869098e9cdf83ae32247e30982",
+        "🔺️diff/🦀️.rs": "56f9e4811bd050ff0b486028d15547b33e6226c049e24f497e56ed9e8307c2a0",
         "🛰️.proto": "0a4c7548c539ca4e6d99ee47bf5e8ba7209f081ae51b730393a206ca4f7f172f",
         "🟦️.ts": "e6524af3567d994f80f4f47bfb0bba3c8fecaf643492c7eb8929ec54daddf260",
@@ -42488,8 +43808,8 @@

         "🟦️.ts": "d973806e94f9dfdeb942d7864417ca80eacd6288fae7512e21b64436a52fd70f",
         "🦀️.rs": "799f3229cbde48219648d8a2ca5699489ab3b1c1283d8f5e4d053a71ff56a9af",
-        "🧬️mutations/🔗️.graphql": "92f3781bff49aa8c4f19abceaf887c0ed8051a1b6d6f343cf0f1d45617646688",
+        "🧬️mutations/🔗️.graphql": "6a141f2b5165f4d9aa5b51af65a9a7600fc9a8267ec2574a9b8158d1cba38b7f",
         "🧬️mutations/🔣️.json": "2f43931554b7dd74065e4238386226e20deb7cad3facb4b4db7770f75e126b25",
         "🧬️mutations/🛰️.proto": "e0991d497ff5173c19bc6d9e7e3394459e1b3b342aa37cc1d2e143e76cc7f263",
-        "🧬️mutations/🟦️.ts": "31bb78606beab668d69c7a9bd270f51e77638993914249ee60c906e08e0d453a",
+        "🧬️mutations/🟦️.ts": "f9df85d8e620cf4190588a9aa25980f7a20b0515b2aea18f4bbb5719141f2bcd",
         "🧬️mutations/🦀️.rs": "7706b84718cff01cba1032fd2264d28f70405bf5d0b8b1799a23a2998a987056"
       }
@@ -42730,5 +44050,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "624a99b32fb464a8d79d431a5ea73b2f1652b2e00d212e0a7bca1db5427ff4ca"
+        "🔣️.json": "3f479b54d0613c33f0195af956cda369931c0ea2c299d1b2e29644792d7efdc5"
       }
     },
@@ -42806,5 +44126,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "dad7434dc213eefb86981adca2fdc0bc3c158c0aee71be7b53039dea173dbbe9"
+        "🔣️.json": "6460f941592b190acd7215e0b1581efe64a2816998045b52c45991d41c71e5f3"
       }
     },
@@ -43429,4 +44749,72 @@

           "facet": "inference"
         },
+        "LowpolyMeshAttributeRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshAttributesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshEdit": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshFaceRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshFaceValue": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshFacesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshHalfedgeRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshHalfedgeValue": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshHalfedgesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshMaterialRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshMaterialsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshRows": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshTextureRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshTexturesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshVertexRow": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshVertexValue": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "LowpolyMeshVerticesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "LowpolyObject": {
           "file": "🔣️.json",
@@ -43509,4 +44897,5 @@

         "os.store.child",
         "s.lowpoly.lowpoly.mutation.apply-paint-stroke",
+        "s.lowpoly.lowpoly.mutation.bevel-edges",
         "s.lowpoly.lowpoly.mutation.change-object-smooth-shading",
         "s.lowpoly.lowpoly.mutation.change-paint-layer-blend-mode",
@@ -43515,8 +44904,18 @@

         "s.lowpoly.lowpoly.mutation.create-mesh",
         "s.lowpoly.lowpoly.mutation.create-object",
+        "s.lowpoly.lowpoly.mutation.decimate-mesh",
         "s.lowpoly.lowpoly.mutation.delete-mesh",
+        "s.lowpoly.lowpoly.mutation.delete-mesh-components",
         "s.lowpoly.lowpoly.mutation.delete-object",
+        "s.lowpoly.lowpoly.mutation.dissolve-edges",
         "s.lowpoly.lowpoly.mutation.edit-paint-layer",
+        "s.lowpoly.lowpoly.mutation.extrude-faces",
+        "s.lowpoly.lowpoly.mutation.flip-faces",
         "s.lowpoly.lowpoly.mutation.insert-paint-layer",
+        "s.lowpoly.lowpoly.mutation.inset-faces",
+        "s.lowpoly.lowpoly.mutation.loop-cut",
+        "s.lowpoly.lowpoly.mutation.mark-uv-seam",
+        "s.lowpoly.lowpoly.mutation.merge-vertices",
+        "s.lowpoly.lowpoly.mutation.mirror-mesh",
         "s.lowpoly.lowpoly.mutation.move-object",
         "s.lowpoly.lowpoly.mutation.move-selection",
@@ -43529,5 +44928,10 @@

         "s.lowpoly.lowpoly.mutation.scale-object",
         "s.lowpoly.lowpoly.mutation.scale-selection",
-        "s.lowpoly.lowpoly.mutation.set-vertex-positions"
+        "s.lowpoly.lowpoly.mutation.set-mesh-rows",
+        "s.lowpoly.lowpoly.mutation.set-vertex-positions",
+        "s.lowpoly.lowpoly.mutation.snap-vertices",
+        "s.lowpoly.lowpoly.mutation.subdivide-faces",
+        "s.lowpoly.lowpoly.mutation.triangulate-mesh",
+        "s.lowpoly.lowpoly.mutation.unwrap-uv"
       ],
       "hashes": {
@@ -43577,6 +44981,24 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.bevel-edges": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️bevel-edges/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "BevelEdges": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "4c1239046970c65263a23db5bdc0cb0fb14ebda08a64fa8578234dd25bcdc9db"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.change-object-smooth-shading": {
-      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth/🧬️schema",
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🧬️schema",
       "level": "mutation-leaf",
       "facetKind": "🧬️data",
@@ -43596,5 +45018,5 @@

     },
     "s.lowpoly.lowpoly.mutation.change-paint-layer-blend-mode": {
-      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer/🧬️schema",
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🧬️schema",
       "level": "mutation-leaf",
       "facetKind": "🧬️data",
@@ -43614,5 +45036,5 @@

     },
     "s.lowpoly.lowpoly.mutation.change-paint-layer-opacity": {
-      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer/🧬️schema",
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🧬️schema",
       "level": "mutation-leaf",
       "facetKind": "🧬️data",
@@ -43632,5 +45054,5 @@

     },
     "s.lowpoly.lowpoly.mutation.change-paint-layer-visible": {
-      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer/🧬️schema",
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🧬️schema",
       "level": "mutation-leaf",
       "facetKind": "🧬️data",
@@ -43685,4 +45107,22 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.decimate-mesh": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️decimate-mesh/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "DecimateMesh": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "170eb92ca0ea2b4253aec63def17335d23de7bfa25bae20b637d94077b522553"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.delete-mesh": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🧬️schema",
@@ -43703,4 +45143,22 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.delete-mesh-components": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-mesh-components/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "DeleteMeshComponents": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "a945c234c4f53354de5ca07647a75b4bc17ff334c227b7c26a58c7df6c5a1cb5"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.delete-object": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🧬️schema",
@@ -43721,4 +45179,22 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.dissolve-edges": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️dissolve-edges/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "DissolveEdges": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "af5472e21a55515648559c5e294530dcaa9d900c5db97628567624c6a58b3b0f"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.edit-paint-layer": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🧬️schema",
@@ -43739,4 +45215,40 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.extrude-faces": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️extrude-faces/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ExtrudeFaces": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "23d44440950391bb7a2e9b6eb3a1f2f89e25474b70a7f283c4c04176c31b795f"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.flip-faces": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️flip-faces/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "FlipFaces": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "80a8ffb8c88b71964c1b411e626e96fc11b11535ef185db238e74eff51bf1507"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.insert-paint-layer": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🧬️schema",
@@ -43757,4 +45269,94 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.inset-faces": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️inset-faces/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "InsetFaces": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "d5d808d8740c09f72b2bd3d96b6315db23bd05371dbf4d8ca31c88c3921082df"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.loop-cut": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️loop-cut/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "LoopCut": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "3144f3dde89de364bb11746fd927f8e4a40165437d9816c75e786dd7efb59a94"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.mark-uv-seam": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️mark-uv-seam/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "MarkUvSeam": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "c6497308585e947002516e770f96f56f7354bb86913fd399c8d24caaeb89b142"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.merge-vertices": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️merge-vertices/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "MergeVertices": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "3eb26b0bf82ff8b043349e97d36070a3a15e2ba2eda024424881f61489113ea6"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.mirror-mesh": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️mirror-mesh/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "MirrorMesh": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "4f6250c0728bb9f90abdd992b67db26fa0441249cda7ffab3f1641bd4e849778"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.move-object": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🧬️schema",
@@ -43937,4 +45539,24 @@

       }
     },
+    "s.lowpoly.lowpoly.mutation.set-mesh-rows": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️set-mesh-rows/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SetMeshRows": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [
+        "s.lowpoly.lowpoly"
+      ],
+      "hashes": {
+        "🔣️.json": "dfd20febf7097802179899181ca2de3cda470303a3d23547dd90c9b99067b904"
+      }
+    },
     "s.lowpoly.lowpoly.mutation.set-vertex-positions": {
       "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-vertex-positions/🧬️schema",
@@ -43953,4 +45575,76 @@

       "hashes": {
         "🔣️.json": "a9b3d8bf54830434e2a09a24cf456298b3416a27e4a233a9db8275bbf0fbd438"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.snap-vertices": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️snap-vertices/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SnapVertices": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "043d4aa413011e79d580422f8c6c1a638d7008baf8cc2fce19965121739a57df"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.subdivide-faces": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️subdivide-faces/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SubdivideFaces": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "e64eec8b21ed9ad0595bd0a5a394338c4cafb62fd3642e79021fdf414bf4d3b8"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.triangulate-mesh": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️triangulate-mesh/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "TriangulateMesh": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "65e948f44308c35085b6d329231b37af8aad1e8f8b42a864ba909309a7fc0fef"
+      }
+    },
+    "s.lowpoly.lowpoly.mutation.unwrap-uv": {
+      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️unwrap-uv/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "UnwrapUv": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "41988aff8f3d24f1b10f1c1e99d65a8f421c7cff48206d9055970976d5ca961e"
       }
     },
@@ -44533,5 +46227,5 @@

         "🔺️diff/🛰️.proto": "0bb8d1080e63c7fd03592e677b58ae4c829ab36f548153529e770424abb6c780",
         "🔺️diff/🟦️.ts": "c7b86bd54c730946689602442aa923f6418791bcbbf05bf0a13cda34921b2d66",
-        "🔺️diff/🦀️.rs": "d1c663251e4161787fdce3f1aaf5c31cc41dc0219240382236e4fd973dd2de8b",
+        "🔺️diff/🦀️.rs": "f4fbfbcaebc4012a725480f42ddd2ba2081021ab46e6ea90c5acc0a537802f7b",
         "🛰️.proto": "7a18c825777cea5fa49020c3133dabb806d4141d0721dd92f9719a245298a325",
         "🟦️.ts": "df169c45f047c4dfd06eee5876eaf878d15a7b33830b553c2981ef5aa61622ea",
@@ -45593,5 +47287,5 @@

         "🔺️diff/🛰️.proto": "e0f14377423af2555ba715492cbc5b2cbed7d1d6e42751a5741933b6e27a68d6",
         "🔺️diff/🟦️.ts": "eae97a70af4694089e31fbe0a5c92d197cd03da29610c7bf3b6144c3f6dc997e",
-        "🔺️diff/🦀️.rs": "d4714ab21a5e7d302e30195a66f8229720812a3fa963ba47557528e2d291db6a",
+        "🔺️diff/🦀️.rs": "7b58004fa5be4ced706d8e909c91637dcf4d84dd86884c1f90d034228dda4670",
         "🛰️.proto": "21ab21792b4dc2dc37e7bb862813453a9426585c8deb60c92b8a726c25318683",
         "🟦️.ts": "b030dc12129e79a03bec81ad3a43760973c96835686cf837fe3f67c3941b4cc3",
@@ -46255,5 +47949,5 @@

         "🔺️diff/🛰️.proto": "bc237ddba2533f03415697ec3c4067695be3fe2225250fafe7561f82bae762a0",
         "🔺️diff/🟦️.ts": "6dfe6680b961121223cb1c2597d6bbafc710e738788bbe72c57c390b843a7f02",
-        "🔺️diff/🦀️.rs": "d1e0c413177da84277f19bfea976aa12b2f29e72a0ba1c45d4b6238920555c07",
+        "🔺️diff/🦀️.rs": "dc13049a7f9d0c78b3e5567dbb8aa0e13d98ae336ae74f0f32e0b19215d61595",
         "🛰️.proto": "0fcedc10a10f13d767df38fcd65ea0e075ea46844468475bfec8c2bca971e87e",
         "🟦️.ts": "8e6e0104948d5570a9618ea6cf130e90ce4dba7ef71a3ac8855ef85bcdb820ca",
@@ -47408,5 +49102,5 @@

         "🔺️diff/🛰️.proto": "59f6fbd07d91ab61433667c44979694223b5b41bd4b1040b30fcf70acd3125c0",
         "🔺️diff/🟦️.ts": "82e5449659b3ecd0258c2f3a46ab5e26f4e6a650ad2199d50fb8fd12ca13277b",
-        "🔺️diff/🦀️.rs": "5f6df7f1979014b7e2997328476657881b826fee01c4256828f199db9f861c6f",
+        "🔺️diff/🦀️.rs": "55c26ca7576825daa0438398c95437434fec1e63c4a9f2b8d56c8f4ef688cecb",
         "🛰️.proto": "f6742bb827f479cd4506bbeb72b887a66254fe6e97b1550eecc09994deb0bf86",
         "🟦️.ts": "e1ae9f9dbfb0fdb11479697563b51c90422a09a37d923d41eca1ad451bc7df90",
@@ -48343,5 +50037,5 @@

         "🔺️diff/🛰️.proto": "4294ea3cb855b9a8566577044f049a416719892a688cd15387d5bc6c5cc5ace4",
         "🔺️diff/🟦️.ts": "97180a0998462d07acdf605ca91b87d6d6e96b2236da02e724359d8122e7481c",
-        "🔺️diff/🦀️.rs": "e0f1746aa903414a86a2050bcadd8d31607620fa4fdf0fd096fa2a4ff154526b",
+        "🔺️diff/🦀️.rs": "beff91b62bea3aa22d915a5ae545ef3b33237a0f4fc9337d2d92cf1979c389ab",
         "🛰️.proto": "8e718c69a8ab18e4a956d9883c59b410b44a3e754412e21a07bbe5f5d7a62897",
         "🟦️.ts": "876935731f3efeee62d44ac1c9b1ffc078ca7a700a58354c1ab197c6b49465d5",
@@ -50288,5 +51982,5 @@

         "🔺️diff/🛰️.proto": "ab5a86eccdf3126c142cfae2a371559edb20c7976d26249d5ba4348b0c94a252",
         "🔺️diff/🟦️.ts": "1264f01afaf14a98eb94dc1614e57c55fc005eaa3640204a5a80557626b67c4c",
-        "🔺️diff/🦀️.rs": "9c2fad0163471e73bd9f67a45d73bd76b993064f604b2a137c057cbf38aaacac",
+        "🔺️diff/🦀️.rs": "a7197d2f566d12ae3b8e6e88688a29b177ecacddea6c9934931a772bef68a3c2",
         "🛰️.proto": "263904742cb2404d59f1e3ff9188aa0b432d7e975afa442ef83990b80a0fb739",
         "🟦️.ts": "6a1ef4ece635c171e0cf9d27b56654be445428c074ac00b26d5a808c9f1007dc",
@@ -51550,5 +53244,5 @@

         "🔺️diff/🛰️.proto": "9466d01556bb78ff7a796fa6a0a0970550e636056a325b75395e9603e52a600d",
         "🔺️diff/🟦️.ts": "a2fdddc469e2e93a1ce458eaf3b403ff8c478352bf08a2906f648207d396e27b",
-        "🔺️diff/🦀️.rs": "51e1d5f1773708b7424c8e7721e9122572f66cd38760e66d140079baab68c5dc",
+        "🔺️diff/🦀️.rs": "5cc34029cecf6ba768c534f0054663dddb474382be539fa25c42bb0024a20e20",
         "🛰️.proto": "583be9eff96cb4be43dcb7c880cd0da65b951608c5916fa6ce78d738046b853e",
         "🟦️.ts": "e1d79984a04d8eb6061363fa99eaa784551b35fbee81f6554d95a0c175bc89c1",
@@ -52780,5 +54474,5 @@

         "🔺️diff/🛰️.proto": "8145cada1ead738b899988251f4f3e9da9a6d8392bafacabd07dabcf9f942d8a",
         "🔺️diff/🟦️.ts": "65f22c17b9bbdd54eeafb46125390a420bda850000711b2704972ea97b573a8c",
-        "🔺️diff/🦀️.rs": "b377733958ad6940f59f5d3655595abcecab1609780baf1cecbca0f8b25a8e2c",
+        "🔺️diff/🦀️.rs": "f630b0b6d91a109b329439f659574ebc4719b6159b03255f2534769790892cc2",
         "🛰️.proto": "9fe54fefd2f95ddcc8a402f52b0729d510dd641f8481957401252b839e6b9e07",
         "🟦️.ts": "05f2aa078a96ca159f001a725d374fa6e83391e5e8bb80764c267fbad40762c4",
@@ -53541,5 +55235,5 @@

         "🔺️diff/🛰️.proto": "4be9856f46d8500f79882b6e5dd135c2966a96dcf0174610a960050b9359642d",
         "🔺️diff/🟦️.ts": "ab81006e70e964ed9c6c483566a4591c70fd5b8d66c2b294be65562f5d3053f1",
-        "🔺️diff/🦀️.rs": "7d5396f1786f7cfe5e70fb576a4b79be5aa5e521f2c214eae81c9a7d85b3d762",
+        "🔺️diff/🦀️.rs": "b96a74f1a0d98540737af2944636d1470b16a41d73df051a92c04974046018c3",
         "🛰️.proto": "931f3feb1a9527ec51c444b4b96a8dec7cd935fed54d462de6ab551f801fac67",
         "🟦️.ts": "efa1f21785178dc72b51941283ad4b17ef5649c4d6e6958f7366d83c14b582e9",
@@ -55140,5 +56834,5 @@

         "🔺️diff/🛰️.proto": "3ef2cd82d123789f2fd2e40cb97b19edcd8b26d7213b38842a04c87d2da4ea42",
         "🔺️diff/🟦️.ts": "34430793b9601dfebfd7830a65d7e52eeefeebb178d48168027d4808403a0357",
-        "🔺️diff/🦀️.rs": "7a3aa6f7bb6d952c88e246c66f6648912b6659b49c77184b0b37a2ead482b658",
+        "🔺️diff/🦀️.rs": "745dfef4e9d10826505c2a144ff6e1bcaa05d7ea92cb8437d2ec43e19e44e071",
         "🛰️.proto": "b93b5c66bf4f8b0ef82e983dbc4e10ba5fa86a0dd96ca39b647d74a9d210474a",
         "🟦️.ts": "30bcef6c234cd01f30f124622c2f984ef034b806748b2824ab2edad55d21eb27",
@@ -56589,5 +58283,5 @@

         "🔺️diff/🛰️.proto": "afc0c9140608a77c35f5171ee1ddcc964b3206161456f6a91cc2d8b918a2dd21",
         "🔺️diff/🟦️.ts": "c7b002f8348fd037e10dff9c212451b17e43479fb048fd15635f9484f1da532f",
-        "🔺️diff/🦀️.rs": "64be0bee8042916c604ea4f23f6a5d2f419beacf4ef80302fc26203d6476d027",
+        "🔺️diff/🦀️.rs": "a9b1ebe302b6371d7bcab653551d683bd520c9ecbd7affb4c31e87352dbdc090",
         "🛰️.proto": "6ee1203d5c36cad424b5037aae3ea64ee82cbe5c1cadb7d8cc4e5ff3bfb6a569",
         "🟦️.ts": "747b12dc2992d08a42ee6c0bf81aca4f664bb39dcb821589d64b2439ced9a0e4",
@@ -57413,5 +59107,5 @@

         "🔺️diff/🛰️.proto": "e07f4d89d71ca0cd9aa5411ed8d84a8427560c585e393c928949b683124e6816",
         "🔺️diff/🟦️.ts": "2f19babe00073ac50124c5e6b3ff0bc3c44fd6413643540105ab992ac047f62e",
-        "🔺️diff/🦀️.rs": "c1ff9e906f56996d18b78cfabee3326dacf91d683e2dc2fb9eeba7007c45b4a3",
+        "🔺️diff/🦀️.rs": "dbf5597377ca50e85e5f914dabd9cbcf5915dcfac77254a4b9f60b77888ae5b6",
         "🛰️.proto": "4af3883ad49f7f6b4a4eb4245cb0aa34ab00497b318d739ffd007b05bffdcb70",
         "🟦️.ts": "47ac08217c120bf076f6fc2b811c29123a1b2b344b588f222fefdf2ab1cd827e",
@@ -58388,5 +60082,5 @@

         "🔺️diff/🛰️.proto": "593fee07fbfd9cfcbc75c1ee0b171eb45671358b75756aeadab9043838c28692",
         "🔺️diff/🟦️.ts": "593b3eb92ac92d3f72224e63026aad356b46e03f35d3fba07a8535496bf50419",
-        "🔺️diff/🦀️.rs": "dde9aea1df97cdd2ba27425176a00d8219d27a35ea29a26f890cfac6e744f5ee",
+        "🔺️diff/🦀️.rs": "9cea880a06c927b308a4d41c50a6f1d838241fd17f453670e93e9e21c44e2c92",
         "🛰️.proto": "25de5708a743f9b61ae8816cc8221a983c9a653b3931455dcca6c339203f15df",
         "🟦️.ts": "decc8074f584983e3c8786d0475d1b434d702f012306f57d943fd95171a99a87",
@@ -59288,5 +60982,5 @@

         "🔺️diff/🛰️.proto": "c553a33386a1eb90c3789a679814ab3390b89ee3d9eadbc84f03ff428cc741ce",
         "🔺️diff/🟦️.ts": "a9234a84ed2558491c1e7e1be545a3411df864736593e12f222e14ce0d7381af",
-        "🔺️diff/🦀️.rs": "3f6d6013b4dcd4089a8875c6010ec2194c64c87699f6abd2a242b7a479f7434c",
+        "🔺️diff/🦀️.rs": "f5be5d10c3d78fa67b2b8106a9ffa9686c49b25b47da2fcbd918f62f75076654",
         "🛰️.proto": "a1b2a7c36fd3b955dc0ab94dd9906b86f7260fcc400d9c86f1a27d8d45929498",
         "🟦️.ts": "064c062a5b845334fb3653afbb6adc16d3389f6c2671c20a4e0bdabd4895b831",
@@ -60182,5 +61876,5 @@

         "🔺️diff/🛰️.proto": "4a8d2eab56429a5ab2fd02c7ab50e2184f6b0d67a473af51e95f7e3bce48dae6",
         "🔺️diff/🟦️.ts": "6820c2d123e17185d93584da679b50fa215d2579a989cc08a9fe09f510b9cf42",
-        "🔺️diff/🦀️.rs": "58831819556beea0918794937b1f94a2cba797f5c86642c96726abea0347e78f",
+        "🔺️diff/🦀️.rs": "bc7ae57a62c83b478a86632403b670235dd3a70f78029b85b2ee0bc8d0d3da68",
         "🛰️.proto": "3c38d3e8b0dc2223284b582abed1e00cabf93da98e98295ab260962615c51889",
         "🟦️.ts": "e3174cb167edb00e21a8e36888b9ee3070b6f72c4996b0897e5c4e27959860da",
@@ -60936,5 +62630,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "91c7f21808908927b5bcbbb7be52b6c731652cdd7521433b3c84b58f82d986fd"
+        "🔣️.json": "488d0e304b762a963131352c74d5f7683cbd3d412602db8ea16e4fea1ff4288e"
       }
     },
@@ -61379,5 +63073,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "6d95df0f9a952a6fb6a45d96579a5d12e9c54dbf58404cf7fdb7598b3d704540"
+        "🔣️.json": "2f1eeac7f0cc47d48cae4e5b8769945fa22e1fdad53b2bc89a10e0fec036d431"
       }
     },
@@ -61397,5 +63091,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "0311b8396298c51031049c141f647d42f97435667ed84138f0e06b84326dd798"
+        "🔣️.json": "39b418c52b8a7e1c0087ff3bb0461fc16ce9569af803d53a13c5536a7d99b71c"
       }
     },
@@ -61523,5 +63217,5 @@

         "🔺️diff/🛰️.proto": "fd8d7ff17dcce8c1afaadc7abaef94cbd3155b147f15aebe1ad5df7d65408c49",
         "🔺️diff/🟦️.ts": "6a176171ff2751eaa79bc340a9a5c43b26fcfe30cd87f957a7638040e7cbeadf",
-        "🔺️diff/🦀️.rs": "f89b28ebe5590811fa60414208c1b5e775fa0a599aa8a04ccf94c8deb6f4a4bf",
+        "🔺️diff/🦀️.rs": "1d984b6f40156a7824a5225df584236fdb0d09adadfb19de437f65ba4ba8eeb9",
         "🛰️.proto": "a5d3110510e4517b4fdaa4f67f3965850e15b66089018b2d24273219f2f8f506",
         "🟦️.ts": "80b7bf6d2eec7b107332e56051e054ce9965c065e5303b4f100f6098215be215",
@@ -61762,4 +63456,5 @@

         "s.procedural.generation2d.mutation.change-schema",
         "s.procedural.generation2d.mutation.change-slider-value",
+        "s.procedural.generation2d.mutation.clear-generation-value",
         "s.procedural.generation2d.mutation.clear-widget-layout",
         "s.procedural.generation2d.mutation.connect-synapse",
@@ -61794,5 +63489,5 @@

         "🔺️diff/🛰️.proto": "b7b30e937ad26aa75829e7d8384147a7a7a50f693e5484f112be629d8eba78a7",
         "🔺️diff/🟦️.ts": "19b9e0317f0535723772e3f018cbc6daaedf5476ed24aeda070c6847b5c9a045",
-        "🔺️diff/🦀️.rs": "cda24d37120f02c17bb37dc1a8dfc2dbff15b0027fa0cc656355cd5baa9f2e18",
+        "🔺️diff/🦀️.rs": "96aa20fd923fb46a019138bbb31e1a6789d38efa0fb87b22b73df5c5d1b759ba",
         "🛰️.proto": "77a83f57515f17afc78d3d62c745d53faee9b6188ecab8b81ccebc5871ca8e47",
         "🟦️.ts": "70a428a257e426f9bc97b3bad0f11e17e7ebf466e0609e97f809b82244dcc812",
@@ -61800,6 +63495,6 @@

         "🧬️mutations/🔣️.json": "8e866bd241dd6e009ee0c7934a95dfff94621cc261652e3eb78690fa40e1ec4a",
         "🧬️mutations/🛰️.proto": "f6faf9a1ddb025723650353444990c381429f642716dc86c5ad49b935553537b",
-        "🧬️mutations/🟦️.ts": "871460250a12081e0adfd65f4bf8fb8e90fce2fef2e1d1c0ad0896fddeca3166",
-        "🧬️mutations/🦀️.rs": "d1ee70ddca1598f9e380527a7faa60e6b39636306f224d132d727b12a3bdd1d5"
+        "🧬️mutations/🟦️.ts": "f57ef5b2ec5f224888eb58881c58d06929598c249cc96399176743812f175596",
+        "🧬️mutations/🦀️.rs": "53b74cd53cab20295774b4f636e03680a28815bbda8d2c69365e53ffbb0b4a91"
       }
     },
@@ -61858,4 +63553,22 @@

       }
     },
+    "s.procedural.generation2d.mutation.clear-generation-value": {
+      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽clear-generation-value/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ClearGenerationValue": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "c5abb849906de865a8b138ae9647af0b5834be5d4a105137c6c06174f94264ba"
+      }
+    },
     "s.procedural.generation2d.mutation.clear-widget-layout": {
       "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🧬️schema",
@@ -61993,5 +63706,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "5bbb96fee586e314f69405a3e6445387730926d9a049d2c19c404c8ca43e9790"
+        "🔣️.json": "d117a29460e915ee942a57582d817c9e1e4f0e4a79abdab4ab72f8da1932a6a0"
       }
     },
@@ -62405,4 +64118,5 @@

         "s.procedural.generation3d.mutation.change-slider-value",
         "s.procedural.generation3d.mutation.change-widget-input",
+        "s.procedural.generation3d.mutation.clear-generation-value",
         "s.procedural.generation3d.mutation.connect-synapse",
         "s.procedural.generation3d.mutation.create-generation",
@@ -62535,4 +64249,22 @@

       }
     },
+    "s.procedural.generation3d.mutation.clear-generation-value": {
+      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽clear-generation-value/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "ClearGenerationValue": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "f2538aac9f0c0ea6629f80faf69ce19d9fd5c6999f6cec33e51698c3f01dd098"
+      }
+    },
     "s.procedural.generation3d.mutation.connect-synapse": {
       "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-synapse/🧬️schema",
@@ -62652,5 +64384,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "632a67eae7140337d59f5e7e187283d692fa860aab4c9469f2d0c97bdf110f3a"
+        "🔣️.json": "35725096869ac754e82453111cb15b8ffe715af9fdfc7a0d28ff9f9cca885160"
       }
     },
@@ -63076,5 +64808,5 @@

         "🔺️diff/🛰️.proto": "77373e28f91091313180ea1e23c31f8c2220e07548dda5ba9b43d81aecbe6581",
         "🔺️diff/🟦️.ts": "d873bce0a396afa48b3ae6654b331a32a27aa89911b82fa9bc86c27ba2a4fa7b",
-        "🔺️diff/🦀️.rs": "962a77d7219a1456a9ba19b45f6860f397e64ccaaa4bbaf465300f2dc1adeaf0",
+        "🔺️diff/🦀️.rs": "6dc3c84f0a738398da66191c8b8a63f36f2de3f951db801e76ec5f45fef86a3e",
         "🛰️.proto": "9e41b2d0817258fb63340726370be21003960a9da17308a82e4cb55d83bd9f00",
         "🟦️.ts": "bec4257a13755dd5c54bc3b91f74915f922fc78e96c173bd719d9b2fe1a9c91c",
@@ -63673,5 +65405,5 @@

         "🔺️diff/🛰️.proto": "ffeec2823824d549102c347ca6690bc9463affcebfcce3c251121628abf83c35",
         "🔺️diff/🟦️.ts": "28d0787c3cc184c645114dd6befc7713892308d0cdd2feec1af0d96f4492e055",
-        "🔺️diff/🦀️.rs": "58261d025107e52bc42ff9295bc3e0440530d46545a44b8c52e62d4169477040",
+        "🔺️diff/🦀️.rs": "03b4865d880095e302132096c0e2b616dca80f527c99b06c6071f788536e6c35",
         "🛰️.proto": "2463c5fd400da2ca1919688340f92ce50e438b8738c58c996a810492931b9a6e",
         "🟦️.ts": "4e1baf64fc56351568257cd9d68977183bfac1749abd37d63eb570f3cd829a57",
@@ -63681,5 +65413,5 @@

         "🧬️mutations/🛰️.proto": "0c9ec41358857521991f5c88b66622314034c480f3151b1e3c6deab1b401ba03",
         "🧬️mutations/🟦️.ts": "cc2e9e82928b326f8c57b3bb113557cac2b30d11741686546c49032174f8f3c1",
-        "🧬️mutations/🦀️.rs": "111b1bbd6d61e6977b6e3ed4c8ca649f8e6720c207ce95b0ee62eefc2876f913"
+        "🧬️mutations/🦀️.rs": "314f126dd51b144c194e0e3037adfb9709c72d90ccb02563ea05c11ca401dfc9"
       }
     },
@@ -65828,5 +67560,5 @@

         "🔺️diff/🛰️.proto": "4cdc05746378a57e820404ec26c39579bc36633311ce69d6b5f9e75284b1e2fc",
         "🔺️diff/🟦️.ts": "aad471d838370b4cb510b1846a88fc28fef4fe8f817f0c070fb91be2c18e3407",
-        "🔺️diff/🦀️.rs": "952f0b1f9b08929390275ade5cc1ca1b6e813930123cb0adb3c1886da0f3c050",
+        "🔺️diff/🦀️.rs": "8bcc5849b7f50aac57103f1232b62dabdda9ea1d6a5329b07afaac1541a829fe",
         "🛰️.proto": "3617441bd45e5e4c558fa0b577b24461be6b9f2890351be8707d4fb33ce297eb",
         "🟦️.ts": "edba2511071609b5f4ff3c0574bdfca3fa1000db05c873d9a4f74dfe10ea95ac",
@@ -66668,19 +68400,19 @@

         "📸️snapshot/🛰️.proto": "76eea1118151abc0c11c2585debe9d188773cda68a7a7708f691e22e14660e64",
         "📸️snapshot/🟦️.ts": "31cc92b6658e8f508706670cc7bf50cebf150e2be016eef5006f79761df03b62",
-        "📸️snapshot/🦀️.rs": "1682ad7b692e9f881e4141c1b306ec5c5d312bc839d63ad02e9dcc4bd244dedb",
+        "📸️snapshot/🦀️.rs": "f6fcf28cde485dd0790024b487f9385d03f3f30c3c6017720db60efaec8aa2f6",
         "🔗️.graphql": "283faf15afd21491d9d5151effa58323ef9252f73554ad2daa6d27b00514b046",
         "🔣️.json": "f1f2a9f6d83ed7a51180af6c3d4d2a1c32738a36c777e53fe418462eacff8800",
-        "🔺️diff/🔗️.graphql": "76ad3f01150fc89de3961796e3075fbd592b01e1cdee2bcd615aef7dc0a24db9",
-        "🔺️diff/🔣️.json": "b5c785cca212141ee213753761f760bd7688a9b3892d10daf44f91c02b3e0a12",
-        "🔺️diff/🛰️.proto": "bf17fa117305fcb774a5f320caaabd759d5faab08f16d562847f5b55e2b87780",
-        "🔺️diff/🟦️.ts": "00ff26fc3c051303646afb004f5ab5874b8fbcaadbd1e55221a71ab55bf80a6d",
-        "🔺️diff/🦀️.rs": "6c0687f1804f2bbc48948ce72246782fd4542afc9b621cfdf584fb9a4a2596ff",
+        "🔺️diff/🔗️.graphql": "a8b652d77773cfc607b4ab9e97c8fab9624e664ddf14c149b51b41d7b5f4511b",
+        "🔺️diff/🔣️.json": "91b0e8cb9d9a55855a70645a0c46593cf3ad696a63554df2adb729415751fb6f",
+        "🔺️diff/🛰️.proto": "f6d4ca8373c1c7c8ca74a030f30b7a02bf8ef738c1f6f774f9378a92f31f7bbf",
+        "🔺️diff/🟦️.ts": "19e0154ef04e455b806aedd6c70ad154b009037ffa7ffea1cde237f5cefb9896",
+        "🔺️diff/🦀️.rs": "451360ab79bb1a1806f2bb17ca3f6854f497e844f431e639bcdc56614ac3a7c8",
         "🛰️.proto": "0344c8d032568c8cddae813af7b439620bfbf4e5b4be776c5284220332523d37",
         "🟦️.ts": "788f6674427ee0380f038fde9b39e21f86db354ba7afa2b6944d74256c73a801",
         "🦀️.rs": "baa40243e51348b09adee4502abf44f11a385ccecf08f591a190c12cb2b4a10c",
-        "🧬️mutations/🔗️.graphql": "b2b67c66980c72eadb6ef8e73b5e5b265fb4adf0c657d8faeadc3aadee99caf0",
+        "🧬️mutations/🔗️.graphql": "75a19449aa284822058a0b4db721258708833671afd7f9264dfcc32adf53894a",
         "🧬️mutations/🔣️.json": "319dc63f643174491cf99d18cbb18608e89bb4a2ea761cb522ba3b2de589ff5b",
-        "🧬️mutations/🛰️.proto": "fcdd21a6cff2cb8200294d4326d3a61071fd0ff790e8f77a4840186014a0b7fd",
-        "🧬️mutations/🟦️.ts": "fa744611de65f7495754fb3cc9864a529548cb4e6b1c7a8047761b7454a1e472",
+        "🧬️mutations/🛰️.proto": "b9909c06a61d2739eef1880d787b34d9140da967cf3f240faf2402866aa5c482",
+        "🧬️mutations/🟦️.ts": "14a40229488e85d23017d10d575a66d0168f7329fd7da49af6842a9e57cf5c8d",
         "🧬️mutations/🦀️.rs": "8d7b8afbd8657ab801afe83ab958ae4c2bc2d1e3cef89151defacbba86a643a4"
       }
@@ -67090,5 +68822,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "40311193336a8a028f3708e6e582383ff651fe962a233ae8c6c880431b1a97c1"
+        "🔣️.json": "be9ab82015a9d9460117d1226f9643bb1f35d9297ed1a2a8de3748f2078c06b9"
       }
     },
@@ -67151,5 +68883,5 @@

         "🔺️diff/🛰️.proto": "fa5102feeede7d2241e01e509cbc7d288630db3621d61ed60552734aee4e74db",
         "🔺️diff/🟦️.ts": "2d46bfe6506aea4aafa9a0474e6e0307f591b15a88d1a2f115a8c3a168f0242a",
-        "🔺️diff/🦀️.rs": "f811eccad772bd6d9d19457728bebda279d9cd503e987b6f4d00837673142cb3",
+        "🔺️diff/🦀️.rs": "b1dcaa27b87704bd3330dbba105dda065d32c9ab3dc8a5529f7d353788e31b68",
         "🛰️.proto": "ff8a734d986ad4c2a21c84dab39f8a3c54a64e41350aed1e5aafdab8c4144f23",
         "🟦️.ts": "8b0055c23d9c97ccdbb7580d9ba56c93b03466e9b36108c794dea0ac74bb5603",
@@ -67664,5 +69396,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "ef172ecc75b84bc8df480005da1bbfda4e3d426658f4d37207938d4264cc7f54"
+        "🔣️.json": "e73081638044e450e6129f8eeb2c3754a7b257cff2122ebc164493cb58e4d34d"
       }
     },
@@ -67762,5 +69494,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "336c86e413fafef45a41baab38d63a0250da96c38f44e6ba5ee1cc8cb9315e14"
+        "🔣️.json": "1b8b1e6140d3d7a35f8567b3fa4a4ea206a9e466afec435e284ded52be0c0ae6"
       }
     },
@@ -68330,5 +70062,5 @@

         "🔺️diff/🛰️.proto": "94a47531c722c5e1e89c7376b2138166cbc84cdcba3aceb6dcf327b061b255f3",
         "🔺️diff/🟦️.ts": "29155a6fe4306766a5291d6344d1262b7160eecfad65a5fb17c7e67edadc7d14",
-        "🔺️diff/🦀️.rs": "043874a54ec6074b82b63a14632261cfe809801f244fd5862775efbf00666db2",
+        "🔺️diff/🦀️.rs": "1b49a22c53791ec8c78d4f6320846c48935f74eeb696679042f38752021daa31",
         "🛰️.proto": "04e749f42ab5d1993b42e83f26c4293e59161abc458b24a184340d41ded63cbc",
         "🟦️.ts": "fbab0e51e73da1dace9de8486709519d795dea2c758d46197afdcd340aa3f8a0",
@@ -68521,5 +70253,5 @@

         "🔺️diff/🛰️.proto": "ea9390cccbbf734eb300b1fe8a81f95420786a5a24470ce52ac0f5f2502abdbf",
         "🔺️diff/🟦️.ts": "20361cea72fd71add8a554391c404011a99ed7bae107406624000bad1c939aa8",
-        "🔺️diff/🦀️.rs": "381a05d15021e7f8bce70fe2e1e670f04d1517b923776ede34af1018cd19e2e1",
+        "🔺️diff/🦀️.rs": "e9c716c49379962b6cc5b3693000fc727c871277501e52c7cde7f235fb260f45",
         "🛰️.proto": "b08b3a51d4b98f7ae94adbb8319902dac5f091728a0a54b474f2e1559c24aee4",
         "🟦️.ts": "444310bce4a193a8165a089602031551033bcd449277e2a9fc774c91c6bd79c7",
@@ -69213,5 +70945,5 @@

         "🔺️diff/🛰️.proto": "8ac17c9474642b33e3e34181b6c8a36d52d9ef8e7e48561be0b726088865c0f2",
         "🔺️diff/🟦️.ts": "3b269d1de6c5d8683616b765281ec05ad17758ea679009e89c019e9a3404dcde",
-        "🔺️diff/🦀️.rs": "7ef008a021f4d045d8d0ec71d2b1b1a6e11dd97e93d2be8c8ee695b66b02284b",
+        "🔺️diff/🦀️.rs": "570f51d0bb3ad88d1d1e7b1ae06860b0bac8510cc73d6ffe3cfcb6a98fd66035",
         "🛰️.proto": "b60c31f7440d362528bcf570e34456191b153a4f30526b69ec7ab13f45b9ec4f",
         "🟦️.ts": "9a1c49f33086388a8f22518b6e1351def0bf63ccf7423bb8a2e06462be944125",
@@ -69340,5 +71072,5 @@

         "🔺️diff/🛰️.proto": "d53deb211c92a22b56876cdba5f40c5f2c00ee0a7f0a6d1e1e1f61a5302bf83d",
         "🔺️diff/🟦️.ts": "736b4a618e1eed57a821dd82b52928322cf92ba458d85a5cc662d0581fa71fc8",
-        "🔺️diff/🦀️.rs": "4727eeead6d97d0aa66a24117cb2f0ae5b6a14977eae2af1822fc9ff0e48d859",
+        "🔺️diff/🦀️.rs": "8f2a73ac326642cf21ef76e3cd9df8b87600b873a03a78a6870a7cc86353b8ca",
         "🛰️.proto": "3ddc3adcf332c68086f9a7069874bac41a19ec503c2d98a16ba119df3bc68de3",
         "🟦️.ts": "1b962639a99189a8a1b9f3565ee8e54e4f64d1eb3b6b2a7a87bfb6a8213867ee",
@@ -69403,5 +71135,5 @@

         "🔣️.json": "c24f0c49e77d9aa5cfa62a4648832de0db38e1debd3db09d92a6fc0925ddcb26",
         "🔺️diff/🟦️.ts": "d693a282185c5bd8ddee59bf7ba50ae2017eab9079085553e75f20a438574547",
-        "🔺️diff/🦀️.rs": "c352f1c9119605ac4c255919e94ea409afe7fd93ff1f0781fd5b1fbb30ba89cd",
+        "🔺️diff/🦀️.rs": "b9423a2da64d7898671df43196c526e0efeba2b0df50802688745f1a2ba0312a",
         "🟦️.ts": "f960502e1182b215f009d9f5d57b67224c992b1136e97592bc7dcde08f074e0f",
         "🦀️.rs": "8feb4da893001ebe23dd7e79f65f7f4c63dc40d4d45d95a272605bb002dc1d24",
@@ -69645,5 +71377,5 @@

         "🔺️diff/🛰️.proto": "1d7ded5168198471501c2e4abfbaaaa0415f91d826363c64cfe64fd25a5b09c8",
         "🔺️diff/🟦️.ts": "39015308c9430b2f4147b5c73858a396e7964355b6e44820bee9f782bf251081",
-        "🔺️diff/🦀️.rs": "e2247e17315688e46a2bbef7fefe276039a72d202e0f7fdb167b5e7cc0276f37",
+        "🔺️diff/🦀️.rs": "33ca08cf6d794b99c6a3569bc68953e07da29bb8c8a4e430cef440c3c1243559",
         "🛰️.proto": "b82abd6a13e8aeefdbd6f5e6f0e90e267689ad31f3e1dbe886de7b296f3be833",
         "🟦️.ts": "8994acdb59fd9ab66a17842ac427824f5453151919b7e11e0a97ec895ee3238f",
@@ -69977,5 +71709,5 @@

         "🔺️diff/🛰️.proto": "24a5e5981744796eae1c15e4386cc5c953c7bf68a546f76ebf43a4956c0f9e00",
         "🔺️diff/🟦️.ts": "7f52a4a06c712e5a7de9b3161337d39c3971ea4cf198c19906796bf0d28cab42",
-        "🔺️diff/🦀️.rs": "4ead45fc8e55eab0e0daf8544d730a8a088a191fb3b70a9af5172ab0fd2a8751",
+        "🔺️diff/🦀️.rs": "7de066fec408c6f0ec5cf278559eed22e05e49dcd6ac55ed48245a6aa6930290",
         "🛰️.proto": "95ed291acb7b2a0e122cab6c06f274dc99102f054c8d21c7cff507abb450fba8",
         "🟦️.ts": "34c80d3b3719749f034a2ebb9683e72046e41132876e8e3ce2f3068ca6a89cd3",
@@ -70357,5 +72089,5 @@

         "🔺️diff/🛰️.proto": "1dd8ad7c43bcf9c230f9775c9a3ddb62e0afe7943c4bc3614ff9ae2d62a7957b",
         "🔺️diff/🟦️.ts": "cd4527c868d614153f4b70543f917dc0c076456b54ca69aecc753516086f861b",
-        "🔺️diff/🦀️.rs": "4ffb284c207365b43d4b976d8328f81412b4f92d8a6f5bfdbc65ce9d2ec5c0a7",
+        "🔺️diff/🦀️.rs": "0d6a2865e32a9025683c0d04d82bfb2b46a407167b003d3e93da09d47ddcb96a",
         "🛰️.proto": "cc733ad29932382092372df81480bc93850e29d3b3225c7dcd3a78f79d4c741c",
         "🟦️.ts": "73fcfd314853b5b0558b000e913a31f53d6c375b175c25af2aceefc0807a211d",
@@ -70471,5 +72203,5 @@

         "📸️snapshot/🛰️.proto": "355e48ee9f55f2ae6b3b4a3311ac4ded57a0e050eb1a08ae0f7f24095cf91b85",
         "📸️snapshot/🟦️.ts": "50eb0773a58cfdbc7918b89dbd296c444bcd5ac19f2ca5c5eabcd502c98f9707",
-        "📸️snapshot/🦀️.rs": "d898a41daa65cf1d58d8a9171c8817bdc186e8d683b4488796258ada7d0b8276",
+        "📸️snapshot/🦀️.rs": "031034d7cd0720a9aa580c24f73c58b4071e7c82d514dc9f1ce89eaeea72ca79",
         "🔗️.graphql": "f585834e9d3c1707a5626fd1899abc3f65a5204cb8ad042a9df43859d47dcea2",
         "🔣️.json": "3c51c4876684f0047ef6f845606deca0eb033278d6a801571d01c83773411561",
@@ -70478,5 +72210,5 @@

         "🔺️diff/🛰️.proto": "0fb671c7c35da84593640ed6edaf822fed150448005c08652211872b9ada17ae",
         "🔺️diff/🟦️.ts": "a8f22e6075b43e61a40aa2c1949f783af9f71bdd4c6755ffe20cab6f51d5d3e5",
-        "🔺️diff/🦀️.rs": "281838e58dd0bb79e36aaa91a30821e4146a28f548e1d7b8c78b9792d2ae4daf",
+        "🔺️diff/🦀️.rs": "9f8a0d9ceabf7a4eb4cb83a61ad954b53c223b4090dc21eea4fdd1e0078f3e70",
         "🛰️.proto": "10b05423a7e28f0e147714d6ad9142174d3e4038d9831f17053b2cac471ec7b3",
         "🟦️.ts": "a481707d03bd799c3255b2fe39236ce26f2ed4e9a9aa297dc167c4b80f19d788",
@@ -70645,5 +72377,5 @@

         "🔺️diff/🛰️.proto": "441c9f3982fdce46d0b750ed060459952d9bfd9d90a1ca023e81972098fa409f",
         "🔺️diff/🟦️.ts": "38275a8652f489fc41223e092663168bc56fa8d13c11b70f606d4316d1dbed89",
-        "🔺️diff/🦀️.rs": "26e9cb44e48b4d69570a6db4ae29c8e17f0be73c44694b0a9295d25bfb1b3cdd",
+        "🔺️diff/🦀️.rs": "6fe396ef246a3267029f9ff9d9530cdab0f2746d5df8f9d2bc4f6ffa198827f7",
         "🛰️.proto": "4cf417a07e12420d2c0a2f68aeafea071d3f7a78281b6cd49ff5e45c5eca3ac2",
         "🟦️.ts": "3312b79565d6d277ad59c75a9d13f55a15cf0d974916e5c13587b0a17952a831",
@@ -70791,5 +72523,5 @@

         "🔺️diff/🛰️.proto": "f0003daaee4622a75dd1b2877294860ca9f1136194fbdb20098b8750535e4ae5",
         "🔺️diff/🟦️.ts": "7610e099fadfe25adaeb728ab955426c9afd74f61a95c552417048c51fc27869",
-        "🔺️diff/🦀️.rs": "99e5c90175f267c2a18940d0f5ecaa33f157118cd611964f056ba230e83f72b5",
+        "🔺️diff/🦀️.rs": "bba45fd9b233fcdc466fbd14f67a1320e8a197896eae88abe498d9a2e442b678",
         "🛰️.proto": "59be9e2628766717dac1cc917284fdfefffcc29ef2f0b13bb8a690b9ad3fa0f7",
         "🟦️.ts": "4cf24658e71831e3cf0d7f7634418a390fd2c033127fb42ce69dd60a929423b7",
@@ -71006,5 +72738,5 @@

         "🔺️diff/🛰️.proto": "88f1c1daf7f2c0e195457078d36fcabcb8fe67cc40c12abc9899d61c4361fd5b",
         "🔺️diff/🟦️.ts": "4c309b500deaf6d363945cd7aed419f21d3d31b822e1fb5fc912e0ea06ab9082",
-        "🔺️diff/🦀️.rs": "ca7a2eaf28598373cabe967907ea3fd7d0ce9c926458e7cc70c162f83d7e9e96",
+        "🔺️diff/🦀️.rs": "f4c11454841067d524f9d7a4e186289b733a7c1722175a9b7e231bb57cfa430a",
         "🛰️.proto": "134a147b26c19058b97d6990f699d9f492caf5b737730f9d8072b36e90eb3e5c",
         "🟦️.ts": "4baed819cd154a0f35cd1360ba6c42fb4194446d092987f9135574869511aeec",
@@ -71014,5 +72746,5 @@

         "🧬️mutations/🛰️.proto": "fd8deef18cf08e2b54f809b63019ebbe19443ad782487d5070c22fa704ff0dea",
         "🧬️mutations/🟦️.ts": "39a42b2b6da6e8a1924349c1b33e60179a0e20f919592d8341cc0f3b0f77de1a",
-        "🧬️mutations/🦀️.rs": "8d260b75d063ba76186335833c7883fde9377c4f451c06897c86c294edd7fdc6"
+        "🧬️mutations/🦀️.rs": "c5fd2f5230d33c11d6f656be2411b33a6fffc7ab1f78caa02216144a5c9e12d2"
       }
     },
@@ -71524,5 +73256,5 @@

         "🦀️.rs": "846d06e4f33fb63bd7fd2b35e1c92c37f8e2255e1edbb5c617effd559c32b79c",
         "🧬️mutations/🔣️.json": "9663c17ded4943cfb22e6f074a24884d126dbfe6a5077d6872467008506f7207",
-        "🧬️mutations/🦀️.rs": "8bfce7c3c3532f2fb3697e86b140e47cbcac1165912ea8d013d827eb7ce396f3"
+        "🧬️mutations/🦀️.rs": "0a1e3bd93927a452333fca276535902227e2aa02db56d9ce0ff67614b40f5c55"
       }
     },
@@ -71636,5 +73368,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "7a1d5bc6f484ba8f438557d1837af16cb53ac79d407fcdde355541951cc8fbe8"
+        "🔣️.json": "4b1f47a60947e2af5bab19f75e0ca1cfac07aaee32476f74f9c2938228e70da4"
       }
     },
@@ -71702,5 +73434,5 @@

         "🦀️.rs": "5e30cf982646f241a60e614d15b50e981a4612ebde87b7a4e653cc80e507233d",
         "🧬️mutations/🔣️.json": "3eb07a622a2e99c2211a9e56e86070434310cc7de0b68f3a9947748cc8109c2e",
-        "🧬️mutations/🦀️.rs": "ff4e85b850601fa936df663073376641949051611fb9b538114ed8674bd85612"
+        "🧬️mutations/🦀️.rs": "c3deb06cbd52121989cec552852fe831451c31662ffc8edae538888c4bcadf88"
       }
     },
@@ -71738,5 +73470,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "89ea950585bbdb7e538fdf596abfa0ef6bcfac8c8192375d2365a402d1497ca7"
+        "🔣️.json": "96d1b1b920df7ffba78aa95e392367b8f0dd5707e7a70f99bb82629f57bcd997"
       }
     },
@@ -71889,4 +73621,8 @@

           "facet": "artifact"
         },
+        "DwgSnapshot": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
         "DwgStructure": {
           "file": "💡️inferences/🔣️.json",
@@ -71918,4 +73654,9 @@

         "💡️inferences/🟦️.ts": "c25e50d18c5cdef43e9c68ddaad71b1b6cdfa4f9dedaa28f975af6def64c0698",
         "💡️inferences/🦀️.rs": "2cc1c1cf8ae20996668d1b87cd765567ddaa93cbec4294e3c3885a904fc5f2ba",
+        "📸️snapshot/🔗️.graphql": "8ed1839568905d7b71b539930b5be9f17330284e45cac2732992c6fd5db0dc70",
+        "📸️snapshot/🔣️.json": "dfae207406a429ec627730b237c625dbc6eb41dccb0ded4ab2b99d8cfe5d7530",
+        "📸️snapshot/🛰️.proto": "06e1772dcd68be5522ac85f67685766d928438a92c2643441304d807feb477f8",
+        "📸️snapshot/🟦️.ts": "b9135f58d7178b5a23c18704a81102a877b62ebbe0df369cff7784e17336f03b",
+        "📸️snapshot/🦀️.rs": "d2910e65bb8e41970e9f2ae59be4a9f59dc0a7d48e21580c1e5de7d7fa59606f",
         "🔗️.graphql": "4146a1527cf742160e9c888815a83781ebbe5c4262957f63c9350dc6f4c913fe",
         "🔣️.json": "5f08de2e82f764a513e470730e3d760f12baa0dce95ed512766fa0172dcafab0",
@@ -72339,5 +74080,5 @@

         "🔺️diff/🛰️.proto": "d4313a9b7b9dad7431dd752c05f23ec7b921b8f8ca9ef7ecb2a6115bfa1355e4",
         "🔺️diff/🟦️.ts": "bd12b3a6b7b262d2672a36e56d2817b620819c1791d8d885a85e5a876e0e5b9b",
-        "🔺️diff/🦀️.rs": "4e3df1143c237ee26076cb956b49fd53a3a5ad70ac724bfb1ef18c434014b0d4",
+        "🔺️diff/🦀️.rs": "1a70f10531cb97e92d4b162643a6d3eb2915e3ad14786b2f507a8c90c94bf0c4",
         "🛰️.proto": "8b2d01a9ac0e585ce2dbc767ba3dc3ee0e816f354daa10abc09c2ca25403f68c",
         "🟦️.ts": "3f18e5d32c1e0fe65f5ab4aaa9ee73dfb6631eafde562ad7cd987713bd67c1eb",
@@ -72347,5 +74088,5 @@

         "🧬️mutations/🛰️.proto": "baaa61873deac214c3db5c283af972ce10658006b0c71e3a35cd4b404b0562f4",
         "🧬️mutations/🟦️.ts": "3f7f61185159db0d6116162ba1fffb58d10b6a0e89b5563dee3dc26c3d71606a",
-        "🧬️mutations/🦀️.rs": "6e5afd18f91e38beb7f04e4674c4fd6618b317886bf7258d19693120eba01767"
+        "🧬️mutations/🦀️.rs": "40bb30e16e1f26de2ce132fcc16077ab2bc62f937fbfb80b39cc096e99ccfea6"
       }
     },
@@ -72889,5 +74630,5 @@

         "🔺️diff/🛰️.proto": "6fb0a7f3e537ffe3913d3516c14ad6ac4b9f884ab342a53d59e64c7ed2b9701a",
         "🔺️diff/🟦️.ts": "c814a88336116db26946068f832eda650e518e9145471a7b0dfe543ffca6d754",
-        "🔺️diff/🦀️.rs": "711a3f413a7e068ec1e1904643c62f2c48ab10bac8e49346fc2f21e4ac9f8303",
+        "🔺️diff/🦀️.rs": "82fb0a4f610df49519ea93fb1df4bc9e8a18908939e627c6a79050a1c675817b",
         "🛰️.proto": "6f5a06a967ccd53d442cef5cddf1e8988435780f343189a6944e56d01075749a",
         "🟦️.ts": "2a067cf2c91416ef8908f50ebf987ffe140d44b16b361eee0d6708e84f0d733f",
@@ -73176,5 +74917,5 @@

         "🔺️diff/🛰️.proto": "a1cfe2477de17481e562e3d6f86b25316455f1d309de42eb21f2a159a26e0a08",
         "🔺️diff/🟦️.ts": "bda6ce2085b98eced8a4eeff9e6a8e8b234b4abac3461ca6d23fde1d8a2dbfaa",
-        "🔺️diff/🦀️.rs": "457e6e321162463331385075498a17a36f9fd2bb5151c80e611554b69610504f",
+        "🔺️diff/🦀️.rs": "6148400dc298430eb890188b587c92826285288784b18b76241f4f8580c9b4bb",
         "🛰️.proto": "25edf4dc7b68e4246940ec7ef532b23d80b3be003741f7812f49935223f9a017",
         "🟦️.ts": "10a704489b37513e1586eb540cadd92d3cab7b44540d4bb2876f2d3341b6d17c",
@@ -73184,5 +74925,5 @@

         "🧬️mutations/🛰️.proto": "335013a5a42a67a57262e41b88f2474996467bdbcf0fa8485c0a955d4d34b5f3",
         "🧬️mutations/🟦️.ts": "1ffb0af311ed986b3ac47101793efdc295db763dcd46c53ca37e341aa5a87bf1",
-        "🧬️mutations/🦀️.rs": "b923af340af7d192dcc8e9e0d540b103d2f6927507399c389ea5158701fc359a"
+        "🧬️mutations/🦀️.rs": "7ab35d19c7066ec612ebd871404d40d52f8cdae761aeae3c5d65b4a3881753f4"
       }
     },
@@ -73458,5 +75199,5 @@

         "🔺️diff/🛰️.proto": "a1cfe2477de17481e562e3d6f86b25316455f1d309de42eb21f2a159a26e0a08",
         "🔺️diff/🟦️.ts": "f082e2afa9e2c729888fb304bca93e82c42e113e83ef43300ca8c7b426abe786",
-        "🔺️diff/🦀️.rs": "e596ca367a2e3465a2a539250d06c79aa7a2c271743e7fd68ad88e19d8fa75f4",
+        "🔺️diff/🦀️.rs": "8b568c471eda05872ab17c56299d8c72c37b29de98cb99f77a18293c1902b873",
         "🛰️.proto": "25edf4dc7b68e4246940ec7ef532b23d80b3be003741f7812f49935223f9a017",
         "🟦️.ts": "47f0a96c3498ab05360d9dbc3ffa4783eccc7e533c8a3e435cd7c59a13412e6d",
@@ -73466,5 +75207,5 @@

         "🧬️mutations/🛰️.proto": "335013a5a42a67a57262e41b88f2474996467bdbcf0fa8485c0a955d4d34b5f3",
         "🧬️mutations/🟦️.ts": "bf5a87809291afe9a8e991af9ebbdd183830160cbe54760710164545c07e678a",
-        "🧬️mutations/🦀️.rs": "edfc176e9859abaae4e15ac477d12350d2d1be68c7697685944af262a3fa19fa"
+        "🧬️mutations/🦀️.rs": "ece2ad0d7c60cbd36ea2357100c7b854ada37dd7c884825619fcd0ca30084d15"
       }
     },
@@ -77474,5 +79215,5 @@

         "🔺️diff/🛰️.proto": "caeb6d658114d29d85001720b9acae2a31bc23e7e07e7fba3643425f6d5bdecd",
         "🔺️diff/🟦️.ts": "8a64f42e9f7336c5f0afc6c0c814899df7c959c1e77f46a560bd3360d12ebabe",
-        "🔺️diff/🦀️.rs": "37e2bf2a024e5ce8fb2c92561d6f81ae9999610ba9cc056581d8f83cad0e0071",
+        "🔺️diff/🦀️.rs": "66764ca6a269a48a295a043524f53f7b79ba0340c1062d38e8f6a00929618c9b",
         "🛰️.proto": "3803e6dba121cb840d0eb5a50ad49dd9b466b105a28c659be47292fbe2c9f150",
         "🟦️.ts": "1322b6f42343b56f89b2edde171d167e03514dc8c3c0513650fc3299ba51732f",
@@ -77480,7 +79221,7 @@

         "🧬️mutations/🔗️.graphql": "55180497be0cd0dd70aaf238b89398d44d15e62ddc5b6836907b05ed63b58ddc",
         "🧬️mutations/🔣️.json": "823c5ed3a8caa2d0cff1b0d86aa273ec136fae42872a9d2a17874d8ea79c4409",
-        "🧬️mutations/🛰️.proto": "8539571381319ec234a13ae1a7a787c3d310b00bd876ea6b3a67aa977b0258b4",
-        "🧬️mutations/🟦️.ts": "5c8999ef1e521486ffe2772f3299ce036b0918c9729c2d1365a9af85255b796e",
-        "🧬️mutations/🦀️.rs": "f62f1ea515d1e48010d6b15dd43b9953f749b4a644b39d617be9b2fb94358a55"
+        "🧬️mutations/🛰️.proto": "29b7d3391409b608d93baaf097b7f1388458cfb36955a88f5d67333a1c314589",
+        "🧬️mutations/🟦️.ts": "d2da8f41e62dcdddde0c8835d5f1d13fcf695944ccd51aa821e538a2cd30ce22",
+        "🧬️mutations/🦀️.rs": "a4af93e017a50b4e1ed5b4562e1c5c01ea3dfc1a9091b3f7d17612ca2337f2b3"
       }
     },
@@ -77548,5 +79289,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "ed1a392258486b72127becfa7e50b268b0fb7044fc1a890bbb94b7936e712fff"
+        "🔣️.json": "04a092183613dc058fdbdab39c057b08669b9a57e2ab2f84c65249239257eccd"
       }
     },
@@ -77724,5 +79465,5 @@

         "🔺️diff/🛰️.proto": "54bce806f7cc0ae30bb58365cf52c9297e991271a60573433b51c7589d4916cb",
         "🔺️diff/🟦️.ts": "54604b66eca31a9f56d1fd290f8ad8641bdea5b0b77dbbfc27d7ac377728c0d2",
-        "🔺️diff/🦀️.rs": "9f03d94eb02f9812f7d5e11c14d6d1c87725f3b7ad624b84dafeb1be27fe5e19",
+        "🔺️diff/🦀️.rs": "bdba2e152ec80228cb59a95a689b810a8b3d390fa3afeb83095c6a2a144a243f",
         "🛰️.proto": "5a8a0a125f707ba501ea0a9534ed551a332e405cea596b41e8230638748f5e33",
         "🟦️.ts": "ef1cb21d4a31e607e4f4bddf22ff81d6de9b2e5d0eb97670d011cc6b7fd6358f",
@@ -77732,5 +79473,5 @@

         "🧬️mutations/🛰️.proto": "dc691327e787171c31981356f1ce6c7cbc11201ca66c4215f4a66b6a36a2b1b1",
         "🧬️mutations/🟦️.ts": "37a3c5be61490ce15e0dc86b88b9d6dab1772bd93e742fa1b857cd3f373036a8",
-        "🧬️mutations/🦀️.rs": "b269ae5fa93df664de1476e99b1c1a09cc804f715af7a7d296f4fdf3e4b44393"
+        "🧬️mutations/🦀️.rs": "c5f4128bd30731ef54a9fb4734b3f0e2ddd3e35c1c5f790f4fa16b287afaccc8"
       }
     },
@@ -77821,5 +79562,5 @@

         "🦀️.rs": "3632d3d7a6408f98d1abdce32b3b96801ce11a66cd59a7395b98495d7f442bad",
         "🧬️mutations/🔣️.json": "a91d27446f3af7ef1277eeb0247a34af234d1c2d61f1efb8c39f26856d636bcf",
-        "🧬️mutations/🦀️.rs": "f3d183b48d13d547ce1bf531c3a4a838e4674d389800e51b251b192f396b8adf"
+        "🧬️mutations/🦀️.rs": "aeaa44261f22986f83967d760fa1831ec6e50c685259f2c4f9d622c5bef5770e"
       }
     },
@@ -77953,5 +79694,5 @@

         "🦀️.rs": "03abb3b43f001cb761aa585b11fa8dacfd0c377a23d774b911db1e2c415bf8c2",
         "🧬️mutations/🔣️.json": "e1420cec36175d4f4938eae9905b01b1b87fbc9e25bb6b35b5a45038d3157af8",
-        "🧬️mutations/🦀️.rs": "4b5794f9e0260ff05f6d6e8b50661c479addce76a72e27fd1ac3f21ab708b11f"
+        "🧬️mutations/🦀️.rs": "3cee0de3c7e55dfb092428b3f8b30a9246925e39c00aa7cbf09b45d43639db8e"
       }
     },
@@ -78061,5 +79802,5 @@

         "🦀️.rs": "7ade3f6febde2b05bbe04413767cd16ff6023b0f04f2a4ac5dc682313b66e827",
         "🧬️mutations/🔣️.json": "05bc660a83f571f32ed66f778447da075343191f3dd459883e67ead85fe4056f",
-        "🧬️mutations/🦀️.rs": "071c83cd98492a5544ea305492f325c4a9a384394503b4e050108b0e5684d998"
+        "🧬️mutations/🦀️.rs": "4aabc6b500498701eb2c8a160d0a2bb9e4d74710d9318b97a85c689dd7384e26"
       }
     },
@@ -78227,5 +79968,5 @@

         "🔺️diff/🛰️.proto": "5a1ac48d87f95811d287a40633550c350f2c6c6791471ce5dacf678b1e101aca",
         "🔺️diff/🟦️.ts": "6abef0b6311ee23eb1a7cc522c9d12636cffa507fab1ec55b360d5a0313d8d04",
-        "🔺️diff/🦀️.rs": "a5235f1e16509cfa04eff26c99cafc6ffb27d0734193e5e796331564f7a66a3f",
+        "🔺️diff/🦀️.rs": "abfab1c7832d44975bce3a71f498a98348b8db3a26be39ed4ae912cd14e1cbe9",
         "🛰️.proto": "89bcb83aadd8c610595e9815c3dcc037b45394f6cf5949685d57293660e35631",
         "🟦️.ts": "fecd67288d8a3681c01d5d5c8f23c21c7ec5b05a1776bce788a3e7a23173372a",
@@ -78235,5 +79976,5 @@

         "🧬️mutations/🛰️.proto": "95e8a7155b93a14b1733a3d48152c00bcb9474c119e6e1b45ff97d8629ff06f2",
         "🧬️mutations/🟦️.ts": "13692ce33c0b46cd34d66a08f903a25087734813ff88733bd706d7ff2bb2eea9",
-        "🧬️mutations/🦀️.rs": "d8c397accd8a90667e88401deae98acfd529771c85fb6f7179f1abb1b5da2ad8"
+        "🧬️mutations/🦀️.rs": "963e09501e39a07ba7f8cfc20054c1239e7703dc20adf81c3c27d5acaf9e5f18"
       }
     },
@@ -78521,5 +80262,5 @@

         "🔺️diff/🛰️.proto": "6ff7afc0b593944818292b33d0049e6da3fdff38f593725cc92b4070620d4db3",
         "🔺️diff/🟦️.ts": "9ad6d5a1a0deb42d543277c06263ab9390e689409370f54363438c1022ac62b3",
-        "🔺️diff/🦀️.rs": "c6dd42e45cef025156d5b05fcd9c002b6f6266cff2b04482a73cef75fa55cc58",
+        "🔺️diff/🦀️.rs": "0c51df82d6e04762a7dae1f8d3e6decd912bb1de6322892cf8a50a753341a7bd",
         "🛰️.proto": "26dcc76dff8f02e451097e11215c488ec523f95e42b98a4bb96e87e38c9627f2",
         "🟦️.ts": "18486ae4bfeb4de82a03eb22d066e2825c753c229d14b86e57554c017dcaab94",
@@ -78713,5 +80454,5 @@

         "🔺️diff/🛰️.proto": "d77d57b4edf54e5fc835ea8033ad29c2d9ddccf14b7cd914278e619ec743371c",
         "🔺️diff/🟦️.ts": "658cf1cbbcce9165298ca01a53a41871b916b4e18f9c159aa502d7861e6d589c",
-        "🔺️diff/🦀️.rs": "e9caf308ea24e6806ec8f570a5f5d17e677f4b52710ed14fc15451df7bf9f36d",
+        "🔺️diff/🦀️.rs": "2343dc38d8ac96b45b8ae5761108dc9dea62ff2127787f02ee31825b4959daa8",
         "🛰️.proto": "32f108a2edcce64cc4ff9365afbd2da83fd835a8f96a0dcfec8bb7e759a67d77",
         "🟦️.ts": "e73ee4cc2a1dcda21dc2c256a3d0b8bd2756094baac00e00ef44c74303dbd9c6",
@@ -79159,5 +80900,5 @@

         "🔺️diff/🛰️.proto": "5b28d6b3ec69041894fbf896ad1fc3a7337fee37142db3dc1c9e068ede072039",
         "🔺️diff/🟦️.ts": "901d2b7997adadd7f2237add017b8b47709a69cc6a0b1a3b52aced1be88ff66d",
-        "🔺️diff/🦀️.rs": "111f8bb7a926f909525105cf67490dcd0c3f72f6fb31bad2e946ac663c43898a",
+        "🔺️diff/🦀️.rs": "8108300ec4a9ac4303091cd848a6001f4dfcf6a87abf4c0b6bd5bac94c38fc93",
         "🛰️.proto": "d11dae206e858356be469d2000a14a8a237267f6086ba4980cc11d1d4b7c0850",
         "🟦️.ts": "2405c0cfb6443ffa697f8fc36d7c6ac0acd4560529482e56d07e72640b7b17ee",
@@ -79525,5 +81266,5 @@

         "🔺️diff/🛰️.proto": "53dc62a0bcc374206c3ed5080aaa60e7454c6cbed03db56392d13a6653bac75f",
         "🔺️diff/🟦️.ts": "ed8d8f5e8a7e9d3b0a1e403b690e3e511b9167015a83212ecd8f984436a76102",
-        "🔺️diff/🦀️.rs": "78084c0166db4670d4ee8d1c3a66c0db187d18e0e4781ba45630752eab60905e",
+        "🔺️diff/🦀️.rs": "45d9f7781c60ec6b63662059fba548ddd1cfbe661471b0fed4e449e7b8efcf78",
         "🛰️.proto": "9d5f8ae121b5d37b24cada1f03adf91a087b4df669fa9a0c969272af812a22a1",
         "🟦️.ts": "91114f37a4535461a90a30ddb77527fe08fb9d16a7a64a531828458e85c98e9c",
@@ -79717,5 +81458,5 @@

         "🔺️diff/🛰️.proto": "036d06caf5ee38de94ab991641ae304c04ecbe3653da3f17debfd62009d4e87c",
         "🔺️diff/🟦️.ts": "942fd2bfcc5fc34b24450992b68b2ef86126fb87b38f17bb96b4d2a0971ddb33",
-        "🔺️diff/🦀️.rs": "f33701c308664ae08849b2b35019472dc03586fd38491105c13d1aa061929fe0",
+        "🔺️diff/🦀️.rs": "8e203fd7a126c3ec224aa66df44a9e25b40f1dfd82d8109bde6c90da22ee2782",
         "🛰️.proto": "12463adb2d3709530d8f72da935607278791276cbc1c1e0241ee50eaba1f74d0",
         "🟦️.ts": "e860ba8c2da407b12727804bc77f28d96475a84ce3a2b01e6a1947c3bbef8288",
@@ -79955,5 +81696,5 @@

         "🔺️diff/🛰️.proto": "19b7c2d06ed5a56cb9a2b901f3f746994cb49449b7b4543f911ad864710e5d14",
         "🔺️diff/🟦️.ts": "80e662720c5f7bce93c07c940fdb0488c3a1d57b0e6badb2d4e0f5fc4f21aa78",
-        "🔺️diff/🦀️.rs": "fb7c5dfa313f50ad1c4c530094cb112f85228e202080c6d2ec4e4cefa8922b09",
+        "🔺️diff/🦀️.rs": "819786b06ce3eed387c888a2aeefe216f4beca3f69c92e99df2cecf1cf4984ff",
         "🛰️.proto": "a2ce57cf547df1505811d596d47ba26991b1188af00d05446b1594b0935d88f3",
         "🟦️.ts": "20a106bbb892723afd9bf7701370d5c16bbfb53e32eed0d9ba8c21c62266240f",
@@ -80988,5 +82729,5 @@

         "🔺️diff/🛰️.proto": "3afbf7388f1eb7d200590778ca09a43cce60e18605bc2a0244b610bb02fea1c6",
         "🔺️diff/🟦️.ts": "98919ba62ed99254d066cc6937fe698bbacc3527aa887f2a4b866bbbb12c896f",
-        "🔺️diff/🦀️.rs": "c582d9511b2dc6a34b8650f6c63bee6c3b5a9ee91d19faa0a0a267ff94a170bd",
+        "🔺️diff/🦀️.rs": "cd464ed2cc42a60f02a13a182d1c57008d90a97ed477c756fad82103f55f3589",
         "🛰️.proto": "49541d6d8e7724e1efd3349a58e3db9314cb670665a0cd7818df29b90c7f94bc",
         "🟦️.ts": "183573a9e8027e5e76407e42ac3bf7cffff88d26ba0dfb377e6567c99f98e9b9",
@@ -81489,4 +83230,20 @@

           "facet": "artifact"
         },
+        "PdfColorSpaceInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfColorSpaceRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfColorSpaceRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfColorSpacesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "PdfDictAdded": {
           "file": "🔺️diff/🔣️.json",
@@ -81505,4 +83262,84 @@

           "facet": "diff"
         },
+        "PdfEmbeddedFileInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfEmbeddedFileRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfEmbeddedFileRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfEmbeddedFilesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfExtGStateInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfExtGStateRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfExtGStateRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfExtGStatesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFontInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFontRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFontRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFontsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFormInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFormRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFormRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfFormsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfImageInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfImageRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfImageRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfImagesDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "PdfIndexedDiffPdfAnnotation": {
           "file": "🔺️diff/🔣️.json",
@@ -81533,36 +83370,8 @@

           "facet": "diff"
         },
-        "PdfIndexedItemPdfEmbeddedFile": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfExtGState": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfFont": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfFormXObject": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfImage": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfNamedColorSpace": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
         "PdfIndexedItemPdfNamedDestination": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
         },
-        "PdfIndexedItemPdfNamedProperties": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
         "PdfIndexedItemPdfOp": {
           "file": "🔺️diff/🔣️.json",
@@ -81581,88 +83390,8 @@

           "facet": "diff"
         },
-        "PdfIndexedItemPdfPattern": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfIndexedItemPdfShading": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
         "PdfInfoDiff": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
         },
-        "PdfKeyedDiffPdfEmbeddedFile": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfExtGState": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfFont": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfFormXObject": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfImage": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfNamedColorSpace": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfNamedProperties": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfPattern": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedDiffPdfShading": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfEmbeddedFile": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfExtGState": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfFont": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfFormXObject": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfImage": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfNamedColorSpace": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfNamedProperties": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfPattern": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
-        "PdfKeyedItemPdfShading": {
-          "file": "🔺️diff/🔣️.json",
-          "facet": "diff"
-        },
         "PdfObjectAdded": {
           "file": "🔺️diff/🔣️.json",
@@ -81701,4 +83430,36 @@

           "facet": "diff"
         },
+        "PdfPatternInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPatternRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPatternRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPatternsDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPropertiesInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPropertiesRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPropertiesRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfPropertiessDelta": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
         "PdfSetPdfAcroForm": {
           "file": "🔺️diff/🔣️.json",
@@ -81754,4 +83515,20 @@

         },
         "PdfSetu32": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfShadingInsertion": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfShadingRelocation": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfShadingRemoval": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "PdfShadingsDelta": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -81834,9 +83611,9 @@

         "🔗️.graphql": "ab63e4aae36992eab4d7a5f92e4d884276f07c93d1f5e8e5856cab18e2002e50",
         "🔣️.json": "ac98598a907826bb42c85bfb690f56a2260ee71424e5153f1444408e07b655b4",
-        "🔺️diff/🔗️.graphql": "c3fa3c6f909c4365cb4c9a80c0a58d3fc9f0bf6ced6b6f6c17ad56b1ab8929b2",
-        "🔺️diff/🔣️.json": "643dfb12003e26ed6d09044ef8b26174f3eda77d9d573955bc711cc6e2fc9431",
-        "🔺️diff/🛰️.proto": "122326e158ba53c33c0759579eeea51818dc1a87a38c1b5cc414f32592820b09",
-        "🔺️diff/🟦️.ts": "91842edc3c3018212874fe6c7c38a3728d404555ce054915605448c7bd3cbb9c",
-        "🔺️diff/🦀️.rs": "62abdcb0978dfa5bcd536d7348eb792fa0160ca05c76abaf8e1086880a958219",
+        "🔺️diff/🔗️.graphql": "671b29a5e0b2cb5103167ff03a48d936928f3f9a3801e85a372bdb010f09ec62",
+        "🔺️diff/🔣️.json": "5152bee57e8a8c98251159ee68903358078adc0f8f15069fd82086d0ab9d53c8",
+        "🔺️diff/🛰️.proto": "364f8402715ff49bcd3ea692388cbe04eb94a9325428de3361513f16c7724adf",
+        "🔺️diff/🟦️.ts": "b906afb060ee1f2f06d4536f4cb93192977370ad8fb47c9c144a3f207745cf9b",
+        "🔺️diff/🦀️.rs": "4ee66d2d3c9791cccedf14d0d3e8e82b96fad17b5d204819b91d106b79ebaf94",
         "🛰️.proto": "b1a80879705ca69d8daf27c5fe062085faae6b28d7aac81bf3448eb8d9441fc3",
         "🟦️.ts": "e1b279bcaab21c276cacaf4fa3f1867c0f577044fcc1e7d05be647fd9648b0aa",
@@ -84480,5 +86257,5 @@

         "🔺️diff/🛰️.proto": "42120af4080494845f0819b9e1052d77c817d189209eafa403b0f790329bc677",
         "🔺️diff/🟦️.ts": "4d5053f65d93d0d7e3dfcd11173c370293e174565b8fcf12c74167cef050d79c",
-        "🔺️diff/🦀️.rs": "a6d0802e0cc750c847cec844133037f6acaea0151a1ef3df617f3827c4174221",
+        "🔺️diff/🦀️.rs": "46df2e416703896d98030efec0a8d3b08261b0a9a9c927393a2a6ccbabb7ed58",
         "🛰️.proto": "73ba9b8df75301f04390aa17a6cff7c8b16239857046ebe155db6bfff8f81dcc",
         "🟦️.ts": "5a5d80bfd6ab61222eba6e390b26d6570542060c35472f073c7da3c9e858b67e",
@@ -84729,5 +86506,5 @@

         "🔺️diff/🛰️.proto": "581bd14c6b580832d785a1649631f93760445f081eeea0e3480ad584c81ae43d",
         "🔺️diff/🟦️.ts": "9b6c9960a17d3153e97a71f260585bab9998e757ab164530f370e2a00ff33271",
-        "🔺️diff/🦀️.rs": "e80543ab904862caed67fb66fd7ada94a461c85496dd70e7f15a468cb05d055d",
+        "🔺️diff/🦀️.rs": "6d554ef78d0e611ebd5ce5d540877520db3d46cbf35e08b5af13e4834aa84593",
         "🛰️.proto": "2944c516db7a51323abfcb631ef4b6a51565966f43709a93459549f9f147eda9",
         "🟦️.ts": "39e5f2823ae88d2ddc741ad0830b1d11bcdcd25965cf9d9d5462930f8424cada",
@@ -85032,5 +86809,5 @@

         "🔺️diff/🛰️.proto": "ece76d6701c6cfac1cdc95d8d03afc5639ca23f7813bf117c72495f8a7ead049",
         "🔺️diff/🟦️.ts": "f011cad97469e350b860c1f1897ed9158b17ff15729bbba5de8be66f608604c6",
-        "🔺️diff/🦀️.rs": "abd90df43672af37e220b2f8aa06593af2481b381c74e7ece7688fb90d415bf6",
+        "🔺️diff/🦀️.rs": "20fed46fb831a4729064bf6ad239f928811475c72373fef220cceeb5c6cc865c",
         "🛰️.proto": "5e2600df1903e9b892c61121fea44ad6aacc1e8213ab2ec9ded0aa7624a4c9d0",
         "🟦️.ts": "077bade9d82be68ae1bdcc54e712cddeabef7ee59c2f5d190aba95eb30dacd4c",
@@ -85040,5 +86817,5 @@

         "🧬️mutations/🛰️.proto": "0b9f0d320ab2840bb54a03c124cb7aedd5abdde18cf5dbf2923645a5ffb071c3",
         "🧬️mutations/🟦️.ts": "965ec64fab50cf76d138c2c4a6a970506ef35c1e8d5fc9be8ef1549365565fcf",
-        "🧬️mutations/🦀️.rs": "212b401db2b2ebbc5e0f51f19534ac99fd207da64d74332a2298879151993fde"
+        "🧬️mutations/🦀️.rs": "3e9816a0f605d3c54b462ddfdacecf1114bdc4bc53b817edde73e6f94c41f6a9"
       }
     },
@@ -85299,5 +87076,5 @@

         "🦀️.rs": "98889e972159a8e5161eabc0a96504b8d60798a00ed99d5e974e5238924e6ec8",
         "🧬️mutations/🔣️.json": "fdd9bf3e495d271e85edd1a86adb5fa9938d2fd88ec98d1fe92dd03a3e0ed086",
-        "🧬️mutations/🦀️.rs": "052c069f76dd3b12bf60f1b00f2b9b9fac78ef0406f0b22b925856d45f4f4cf9"
+        "🧬️mutations/🦀️.rs": "ba73fc706ac74f77d18f6801db980cbb5783fec62a0f3a7b075db0fed38982f6"
       }
     },
@@ -85411,5 +87188,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "7cfb0d03fdd93565694d91276ce820c1052eba17bf8a353a84069a6fe39c5f86"
+        "🔣️.json": "ad02dff317c947c246a97b02e05acc494bad16a4f5fed72f62cb69e03fe9ea71"
       }
     },
@@ -85496,5 +87273,5 @@

         "🦀️.rs": "c041072b65544d125d92eb9b3e4ba2eb749e2a8b8a44b6a98f13c4b1b957b450",
         "🧬️mutations/🔣️.json": "fbdce2bb4d997468e487a459416219388cca8c8e8e3953b626cc4359e717993b",
-        "🧬️mutations/🦀️.rs": "cf6b22262b497e7d1c2784d34875ffcfc3b8da615fd8361f57a8fa7385f4a6c5"
+        "🧬️mutations/🦀️.rs": "6402b53d1216a68e40129206b50530edf82e95b5c91b6c410d62cad90c41eee6"
       }
     },
@@ -85532,5 +87309,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "502c85250af4e2b4dea191cf17fff6165b79c699df8cda7de7250b5787cd7359"
+        "🔣️.json": "b42ca033e1a68fc8cbfa4e4323049638c60e723040e75a535e1054adcca47434"
       }
     },
@@ -90970,5 +92747,5 @@

         "🔺️diff/🛰️.proto": "85ebde0de6b0d513ba0be691f3074142795361616e45ab97f2243b8c19f6de71",
         "🔺️diff/🟦️.ts": "48481553ec7a2cbe1d8ac8bc6db1f70d92c885c4d5bcecc4de0dc5377e650e3d",
-        "🔺️diff/🦀️.rs": "a6b78f7656f6d4615b792be432ecd1b8b954d2c6e3659a33735e3e2a64d9335a",
+        "🔺️diff/🦀️.rs": "c62990727bd04bd6e59da6d46de5928319039e49f7ccebd7e1618b02bc2dcf92",
         "🛰️.proto": "879ccd8cb69901dc6488aedbd858faf5f56d745eacd76ab74e9c555e7fe986e8",
         "🟦️.ts": "29eee0346c74093bbfe544c3c50f2ec953f3cc203264a2766f7bd9136c78f592",
@@ -92579,5 +94356,5 @@

         "🔺️diff/🛰️.proto": "f7690a2fa72f4f20342c0c3f2d50b6e7083ba56a6f93f4f4760923d8f98c0349",
         "🔺️diff/🟦️.ts": "90f5e75984eca8a586fa9ca84de3fdd11e4abd13004d222c47a8b1ae2da950c1",
-        "🔺️diff/🦀️.rs": "6f1cf58643c150c2b97bfa727768bd934e7b55e4abba820c84b51b2d6e652876",
+        "🔺️diff/🦀️.rs": "59dde84ec136412bb419923e3d6b8fcad4e5e2ce2dfe7b00906df53452519520",
         "🛰️.proto": "4ae41d8fe8314f79357bd82defe6c2d770affc870464dcb74ec0e58ee439562a",
         "🟦️.ts": "fd0cf6e6b133cb1a0cada1cc1406d5722f25632fdccfe07850c1ce2f52916687",
@@ -92587,5 +94364,5 @@

         "🧬️mutations/🛰️.proto": "a9942bf9adb8dfa42976197283af31bc005bc38b618c3e8a1226fde0291d2de5",
         "🧬️mutations/🟦️.ts": "7aab1883acb74b8233d52155c6d33e22f34b50d7ead0d80608785eb03708fea6",
-        "🧬️mutations/🦀️.rs": "fffd3c34726ce06ff816aa6d41183251fb977bffba6d30f69b2515426b4bbeee"
+        "🧬️mutations/🦀️.rs": "ec12cbf1c54f66c8c2fab37d6145164c7ecb58e5d70f2fec1aa9ae4799305a22"
       }
     },
@@ -92811,5 +94588,5 @@

         "🦀️.rs": "74bcc7a8411f73aeb59504ad8062937953aa7989f3d52588446e60a43f3ea969",
         "🧬️mutations/🔣️.json": "186dbced5bbd42e74cdd15911837a2c148bf5a3be3ac5be2fca6f7e3c8ecdf8d",
-        "🧬️mutations/🦀️.rs": "104fd2587a85fe52031232e235edd0df948d4829d30017baf8bdc197756b4b28"
+        "🧬️mutations/🦀️.rs": "d17fe47778262d40579019d55bc8888da342243b5ce675135cf48f98a8af510b"
       }
     },
@@ -92934,5 +94711,5 @@

         "🦀️.rs": "fff7bbae786421ce715d03e06fc8d64aba50a4754e5dabc9093c6984aa9cd04a",
         "🧬️mutations/🔣️.json": "1c68311e3403b86a124f1525014abbde1eec4cf38aa5598bcd34635f8a4d2ccb",
-        "🧬️mutations/🦀️.rs": "7ab65e488fec0d1830676164cd340e1885dfda3ca2fc167f4517c70a1ae4890e"
+        "🧬️mutations/🦀️.rs": "2d3965cd980d028013f79cca900c1ecdbb0cdeaebf59faacfd162efb5db8b03e"
       }
     },
@@ -93079,5 +94856,5 @@

         "🦀️.rs": "1a21c99c375819248204c66744afd589aaf57f9311972b4969650e4cfbc5b009",
         "🧬️mutations/🔣️.json": "0768fcfa56288a225ae905077d69e9bd65f5ace649f98e1ae207d72e4791eb78",
-        "🧬️mutations/🦀️.rs": "0ae52402080fb2aa2abd5f5645ecf651c2be3077ac4441340415c9efe4f1109d"
+        "🧬️mutations/🦀️.rs": "7804eca0bd239ee2739d494f433d38079f4dfd06dd4363ad1728e2e3a21125d7"
       }
     },
@@ -93224,5 +95001,5 @@

         "🦀️.rs": "096c3a8d06c6354e88b517b0f51704e5a74f10c5f85058b020fc3f02da89c144",
         "🧬️mutations/🔣️.json": "31f21bc3fefc5245843408e2c137d5cf4b64b6153df90a8c85fe34649608648a",
-        "🧬️mutations/🦀️.rs": "6b1dbefab56261d6c97ecf103f44e8c6ef0bc8e91286917889a9bad21f63d737"
+        "🧬️mutations/🦀️.rs": "16384fdd8d2935c15926d45d4964b32844c7782042515c38b4e7c4d9abae986c"
       }
     },
@@ -93369,5 +95146,5 @@

         "🦀️.rs": "fb00e3806ef8ceb9ad3712726a7348a0588fefc978307b43ab005039a89a6de6",
         "🧬️mutations/🔣️.json": "55cf47d5b488c56db9a4eb9542bf17a84e79d9577ce92cfd424366b7f2882d0f",
-        "🧬️mutations/🦀️.rs": "47457f3e6204349fe7b9ee40dccdc364872a4d5203bfc8484cd37c1bae787c68"
+        "🧬️mutations/🦀️.rs": "697ce66c17ffe7fffcae7fdf94c74b68de87f2d9210a7e51f9c3cbabbd04eb87"
       }
     },
@@ -93513,5 +95290,5 @@

         "🦀️.rs": "b43323016f59dafdd8db6d93d03050457c87916053ff663b2f7284ea7ea38da3",
         "🧬️mutations/🔣️.json": "b0041ce67a0afeef54e617a4722f1a282170985e918108ea280992fbbaebb708",
-        "🧬️mutations/🦀️.rs": "4d01894f7b2517ea2fbab24c47f3d950942462682849d6c184db4f417023b708"
+        "🧬️mutations/🦀️.rs": "477b8a0f8e4ca03b388982ba96d7b3a5b8eee7092bb48374c67781b10652cfbe"
       }
     },
@@ -93935,6 +95712,6 @@

         "🔺️diff/🔣️.json": "54622b9358ede64eb06cd922fbb6d1179d723d84b103472fbd33ca8b3070d998",
         "🔺️diff/🛰️.proto": "7024a1d1cdcdf281fb4217eb42013039625b7b2dd2485a1dc7fa3879a9124858",
-        "🔺️diff/🟦️.ts": "bf4a696278d502d7c0eee3ae31231de2f05fe7fb91fcc8f5648f99169db99f12",
-        "🔺️diff/🦀️.rs": "ab109ee6354880e1103dc57fcbb37f65d5bc930d650b073eef89aba23c0d5ce8",
+        "🔺️diff/🟦️.ts": "8a75e4ac11c39bf7ad406c242b86020ea826059eaffbad8c0378300ebaedd3fc",
+        "🔺️diff/🦀️.rs": "2ef28055e97b9455ae05e97396c9ca0272df3bbb9aef2138cd5cfab1ee0fc13e",
         "🛰️.proto": "afbb973cda771c60167a8a36f1e2f4394f46f1badf8d0815d1d7d54b0c4947b8",
         "🟦️.ts": "e8f66719986b9835ec1298376818f0962c7bfa8e9bd320ad81b305f6fabe9f32",
@@ -93944,5 +95721,5 @@

         "🧬️mutations/🛰️.proto": "57cc245e9cd52a09301b3c0d8a222fb4b2fbef8a9933ce77fab9c3d81e5768cd",
         "🧬️mutations/🟦️.ts": "579bc5428487b66d023c2932afc89d952afadc7badf061373647a65efdef02ab",
-        "🧬️mutations/🦀️.rs": "913001d1229a125eb6218dda2137be4b82a8bb5eafacaa95a3fa0df7ba6bfe98"
+        "🧬️mutations/🦀️.rs": "ad1e5e71eea9e8bea2014ab5193bf73d94a979d751cedd0d9ee304cc9f83b26a"
       }
     },
@@ -94173,5 +95950,5 @@

         "🦀️.rs": "f6b20c49f0ac1ab3a7936cc438fe7c0397fa603a4611d331b7642d5bc07ae08e",
         "🧬️mutations/🔣️.json": "9487d9d1edc63495be9803034183f12e272f05ce59deeba7157d014bffee7bdc",
-        "🧬️mutations/🦀️.rs": "82f88bdb6c60e74737ae8243a7c636708ea55de834776d555f86946d2d47a82a"
+        "🧬️mutations/🦀️.rs": "beabee10555176fa1dda591c3f191c846466c2ba80daefcec42258698e9b5e37"
       }
     },
@@ -94367,7 +96144,8 @@

       "dependsOn": [
         "s.stdio.svg.1.1.base",
+        "s.stdio.svg.1.1.tiny.mutation.insert-excluded-element",
         "s.stdio.svg.1.1.tiny.mutation.insert-tiny-element",
         "s.stdio.svg.1.1.tiny.mutation.remove-element",
-        "s.stdio.svg.1.1.tiny.mutation.restore-non-tiny",
+        "s.stdio.svg.1.1.tiny.mutation.set-excluded-attribute",
         "s.stdio.svg.1.1.tiny.mutation.set-text",
         "s.stdio.svg.1.1.tiny.mutation.set-tiny-attribute",
@@ -94381,17 +96159,17 @@

         "🟦️.ts": "e42a256eeac2c7eeebf882211fc9053abcda9a2bd4784b11b337fa4fa4ec4f44",
         "🦀️.rs": "e468de6b78690514bf220b3e3047b8451ee8822ceb79dd8bf062534a6ef4b942",
-        "🧬️mutations/🔣️.json": "6dbb77f14812e4763d89ce4c1a7837ac7ffd38ab748676a088666a5147d9745b",
-        "🧬️mutations/🦀️.rs": "8db0e62105e50b457934c6a056df784643fa0e5a827475dcf5b860ffa67111c9"
-      }
-    },
-    "s.stdio.svg.1.1.tiny.mutation.insert-tiny-element": {
-      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/➕insert-tiny-element/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "InsertTinyElement": {
+        "🧬️mutations/🔣️.json": "e0e78729485ad6144427cb586ae863ae416ce4c6e1433c3dc7a1b299ddf40700",
+        "🧬️mutations/🦀️.rs": "cf7601e5ada88581bff030aa97f210836947d500d1adc0af557026a7bb2863f4"
+      }
+    },
+    "s.stdio.svg.1.1.tiny.mutation.insert-excluded-element": {
+      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/📥insert-excluded-element/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "InsertExcludedElement": {
           "file": "🔣️.json",
           "facet": "schema"
@@ -94402,34 +96180,16 @@

       ],
       "hashes": {
-        "🔣️.json": "c452a995255a65e175e4b470d96f46cc57d59a3af0add06027e012da3e2ce5fc"
-      }
-    },
-    "s.stdio.svg.1.1.tiny.mutation.remove-element": {
-      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/➖remove-element/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "RemoveElement": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "18a5a4e8303bfeed40ede537f688e8ac2d8f5a3612460b638245e7b31beea248"
-      }
-    },
-    "s.stdio.svg.1.1.tiny.mutation.restore-non-tiny": {
-      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/♻️restore-non-tiny/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "RestoreNonTiny": {
+        "🔣️.json": "194c9ddc3493f1ef39aa1f1058a71bd44d8484a18bd19df12a762821f8eb5b7f"
+      }
+    },
+    "s.stdio.svg.1.1.tiny.mutation.insert-tiny-element": {
+      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/➕insert-tiny-element/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "InsertTinyElement": {
           "file": "🔣️.json",
           "facet": "schema"
@@ -94440,5 +96200,43 @@

       ],
       "hashes": {
-        "🔣️.json": "6214955ff62c5f110998a28045ab77851c9255d3eff1d9a79ecc6dc08b696bbc"
+        "🔣️.json": "c452a995255a65e175e4b470d96f46cc57d59a3af0add06027e012da3e2ce5fc"
+      }
+    },
+    "s.stdio.svg.1.1.tiny.mutation.remove-element": {
+      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/➖remove-element/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "RemoveElement": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [],
+      "hashes": {
+        "🔣️.json": "18a5a4e8303bfeed40ede537f688e8ac2d8f5a3612460b638245e7b31beea248"
+      }
+    },
+    "s.stdio.svg.1.1.tiny.mutation.set-excluded-attribute": {
+      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/🚫set-excluded-attribute/🧬️schema",
+      "level": "mutation-leaf",
+      "facetKind": "🧬️data",
+      "formats": {
+        "🔣️jsonschema": "🔣️.json"
+      },
+      "exports": {
+        "SetExcludedAttribute": {
+          "file": "🔣️.json",
+          "facet": "schema"
+        }
+      },
+      "dependsOn": [
+        "s.stdio.svg.1.1.base"
+      ],
+      "hashes": {
+        "🔣️.json": "bd3a7a64767516afa03ec1313ec247c1470079fd50965e35b6d6cff7263dba7a"
       }
     },
@@ -94635,4 +96433,80 @@

       },
       "exports": {
+        "AsciiSequence": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "AsciiValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "Binary32": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "Byte": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "ByteSequence": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "ByteValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "DoubleValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "FloatValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "Int16": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "Int32": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "Int8": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "LongValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "RationalValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "SByteValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "SLongValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "SRationalValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "SShortValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "ShortValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "SignedRational": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
         "TiffArtifact": {
           "file": "🔣️.json",
@@ -94647,4 +96521,12 @@

           "facet": "inference"
         },
+        "TiffFieldType": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "TiffIfd": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
         "TiffIfdAdded": {
           "file": "🔺️diff/🔣️.json",
@@ -94667,8 +96549,20 @@

           "facet": "inference"
         },
+        "TiffSampleBlock": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
         "TiffSampleRun": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
         },
+        "TiffSnapshot": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "TiffTag": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
         "TiffTagAdded": {
           "file": "🔺️diff/🔣️.json",
@@ -94682,4 +96576,32 @@

           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
+        },
+        "TiffValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "TiffWord64": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "UInt16": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "UInt32": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "UndefinedSequence": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "UndefinedValues": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
+        },
+        "UnsignedRational": {
+          "file": "📸️snapshot/🔣️.json",
+          "facet": "snapshot"
         }
       },
@@ -94698,4 +96620,9 @@

         "💡️inferences/🟦️.ts": "b358473d3fd2d63685e27b1dd8ef68ac0f822dbf9799d9b61627aca611694271",
         "💡️inferences/🦀️.rs": "b7ae7b4205cca8159e748a634a574aa383908ba0cc0ff7fe22b76eb73c567f86",
+        "📸️snapshot/🔗️.graphql": "ab000a8c7a55dc002bb4de500e3e75177d923e8f033852662caffe7a4161319a",
+        "📸️snapshot/🔣️.json": "72119c67c3602b01027d0966d1162449d94f4913b3dc68dfc858f7426a4a871b",
+        "📸️snapshot/🛰️.proto": "c8f7ff1de27035348cecdb705a326fcfa4df139b8fb28a3eb0b0375a2c1bd129",
+        "📸️snapshot/🟦️.ts": "13c8dfe92a6f419adc04e3dddf72101241236142d432e6320964216bad90fe4b",
+        "📸️snapshot/🦀️.rs": "4d14713b43a9e8ca70aaf55eeacc727e6e6ba2ddd7af05dddefbf2d18f3d735f",
         "🔗️.graphql": "3e8da5271d04c445989ad661e2f461191cb9947e5b4dd02dd997aa2fc348d4be",
         "🔣️.json": "92bb2ae5b95b8a77e0e0ad539ac583f765fbffc58a4c06c7e2045a1b8b84e1f0",
@@ -94704,5 +96631,5 @@

         "🔺️diff/🛰️.proto": "a18ad34b84233745d528c0435b3cfb1af932160239cd3d2b444679e2a037059d",
         "🔺️diff/🟦️.ts": "b3b4cff8705fc7e093521da99ea5f54abf29fe0699c235c065fa59de177bf76e",
-        "🔺️diff/🦀️.rs": "5c302e73c684644aa2021ed05f60691a8549d70206f3e47d95976dfc4ace3f78",
+        "🔺️diff/🦀️.rs": "3e303e7e9a06ae79490a4d9673b5af9bf3faac7a1a56b7bd1bdbe2f1f2d5409c",
         "🛰️.proto": "58fa340de659e7cba905bf33a038bb39c5fd505e56ac3b4436a7381828a3689b",
         "🟦️.ts": "f1c9c4ee480200ab1fd8b3c2ffceb7f61e1064242dd0fca58e50847a15cd3330",
@@ -94728,5 +96655,7 @@

         }
       },
-      "dependsOn": [],
+      "dependsOn": [
+        "s.stdio.tiff.6.0.document"
+      ],
       "hashes": {
         "🔣️.json": "131e7e5552df7d9212dfba823e1acd481a74070afc91eaa9323f0c1b6264d853"
@@ -94802,5 +96731,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "9a21ec2faf3d1b951cef5dd23f31a85e81615541a6b6435aa16e64ceb1aa4ffb"
+        "🔣️.json": "bd10127e3ecdfa3fa7ba0d2929ad91131783aff9d7dd343e8504a4426b5cd6f7"
       }
     },
@@ -94818,5 +96747,7 @@

         }
       },
-      "dependsOn": [],
+      "dependsOn": [
+        "s.stdio.tiff.6.0.document"
+      ],
       "hashes": {
         "🔣️.json": "5029f03cd4d08191988e93dc317c9fa94f41e89162b47c0860a93c098f81a475"
@@ -94896,5 +96827,5 @@

         "🔺️diff/🛰️.proto": "c12af135dcf4f65045da1d71c20a6ff359ac27c4f0c137a01da10ac456fc449e",
         "🔺️diff/🟦️.ts": "fb29797d02d8cd65224000f4f66d1d3417d4f97f2b59dbb7b3a5bd48cb4395db",
-        "🔺️diff/🦀️.rs": "fe66a7f97be9ddc01efad43f921beae46cb0e1f30fcf917d09610efb4891ca2b",
+        "🔺️diff/🦀️.rs": "21c6b48d8cda0bf49245c3754893de6231e1259077aa0fe49ce769ea4d6a84e1",
         "🛰️.proto": "6013532c85baf1fe179be792394800d7d7a1bcb6dad2f80db7239140c74b3c57",
         "🟦️.ts": "7ab009c0fe70c4a5b7b0a250eff9f1aff312938f483a52ad7cbb211cd43ac17a",
@@ -95059,5 +96990,5 @@

         "🔺️diff/🛰️.proto": "3ada92cb5c6ad02a9948b4b3aa60cd01baaf6dc4fb0ee19a8d819f3f8601634c",
         "🔺️diff/🟦️.ts": "5fa126716cf97f59005650653f69cff1769030ab7b8099e10f407d008322c0db",
-        "🔺️diff/🦀️.rs": "0c91950fda5ea0dd4254b814abd05fe026e15e1b09d22495d52cb98687c320f7",
+        "🔺️diff/🦀️.rs": "d9508f78066a4a13cb0839e08d175f5a6e7dae888390b0c65028ae52a768da3c",
         "🛰️.proto": "2023860ec00952ea361d144de1d04bc13dbec211f68561a0f13c63181a768a3b",
         "🟦️.ts": "302faeda54f6697a9e52522b4137a470e9947d5c4e3e89d46fd6250ec0f49911",
@@ -95263,5 +97194,5 @@

         "🔺️diff/🛰️.proto": "7552ae75f1fc9b03ada16b40421d37652143d98fb6c11d2156a2d8db5fc752b1",
         "🔺️diff/🟦️.ts": "5888f98f158bdfcf079b5339883778d9f022a4ed3d11482f4e43e49198024e27",
-        "🔺️diff/🦀️.rs": "9fda670bf43fb6405209e0d6fcddfb9690c522e4ab348570ebe1ff43db62ed2d",
+        "🔺️diff/🦀️.rs": "16b9359950f25da949b752a44666c12c2e2d7493ccb456c42e018c0adda565ea",
         "🛰️.proto": "b648fc60b0274b8ecbab4326688c5dfa198909715607892fb999fd6ee7671e3e",
         "🟦️.ts": "6ed0d0c5cfa712dac4e66a63a187853a4af11b972e9a4855f7e7cf663581920d",
@@ -95524,5 +97455,5 @@

         "🔺️diff/🛰️.proto": "2487466e907adb08c3f38fb5777e2e897d983a06975e990004d7e93335e06015",
         "🔺️diff/🟦️.ts": "bfc8d8f711868ef60cf3ac834e356a36fe32976dcf1fd8a76550c8dd620151c9",
-        "🔺️diff/🦀️.rs": "bf651d603f72f41fb66063787d483adbef9e76e265d154dda7068b3f2be89e15",
+        "🔺️diff/🦀️.rs": "06ef033c5337c7ef0c792bf928eec63479cf088f9fc8dc3f656c90ba86940568",
         "🛰️.proto": "6c8676c66738c95216d71b24cf73ad932375ef323a66a97a35119f5d4918df11",
         "🟦️.ts": "fe3df165fa7e5eda5a70a74cffa78323347e99388f4dac8bdec5344bde4645bb",
@@ -95532,5 +97463,5 @@

         "🧬️mutations/🛰️.proto": "c631053cbb61234af2b40fb427eb8ec6d43b68485ab8c78df3851f695adb851f",
         "🧬️mutations/🟦️.ts": "f75131c500e30fd8cfc8d51ee99bb7275a148129f32227ffbb59d03386c92f48",
-        "🧬️mutations/🦀️.rs": "b59a3407bd568f638c05d04f2dcd2378f73852c934885fe912648fb4008312dc"
+        "🧬️mutations/🦀️.rs": "ab3e70adf5585b4d292641b752226bd3be08a6a06ca918372a23d90ca2edf31e"
       }
     },
@@ -95838,5 +97769,5 @@

         "🦀️.rs": "9d21eaa85664d25759a5ea19ed6b72c85b0972c84ba2adaf764dbdc4d7456f77",
         "🧬️mutations/🔣️.json": "e11ad07fac5ef1e35a3187c606eee14f3befec0d4c08af28ea7630d8ec328ae8",
-        "🧬️mutations/🦀️.rs": "e0f89abc0818c3b54a7cfa277ef6de4c720d28093e85d3806f71f115fe5d6f44"
+        "🧬️mutations/🦀️.rs": "50578605cb4f45c1c75bf5455c4571b1e48ef7b8d93b70865cee1d517a4badf9"
       }
     },
@@ -95912,5 +97843,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "80b5e6628cb537f2c3832627711beeeff8bfec9b40349c498beef6922d275cdf"
+        "🔣️.json": "2af65ac9e6ec74250ad2378bbab45583499930d3c2fe3342c36a70d19b310a8f"
       }
     },
@@ -96016,5 +97947,5 @@

         "🦀️.rs": "c7553b3ef78d6e153850ce712a2764e848e14956486cd9c238f08ae2b0c37dfb",
         "🧬️mutations/🔣️.json": "37050d7d05fbc18204a5f1c73c82b7a2d8e79644c1be953d6b800116681dbe7b",
-        "🧬️mutations/🦀️.rs": "144cc31dbc3968607de9616e78b9c441fc13bc05ebdb84e547265491d9e50e56"
+        "🧬️mutations/🦀️.rs": "0db2b31895d9509e5e05f9f3bd0b9b82285ae0f71f3b801e03798d71a9090194"
       }
     },
@@ -96052,5 +97983,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "af6865d84d71d32551a4b86e21841eddf0875e41567efe162d1a17589a0c64e2"
+        "🔣️.json": "cc4d66d2bff44f799b1c0509834421750d96ed0d5adece5471a817b201989808"
       }
     },
@@ -96192,4 +98123,8 @@

         },
         "XmlAttrModified": {
+          "file": "🔺️diff/🔣️.json",
+          "facet": "diff"
+        },
+        "XmlAttrRemoved": {
           "file": "🔺️diff/🔣️.json",
           "facet": "diff"
@@ -96281,9 +98216,9 @@

         "🔗️.graphql": "64d0a5fc9c8a6747f96f3d0e0ccceea56e8673e29ffd7373b2328ecc65d1fe91",
         "🔣️.json": "538956a7b0287a766da7f4b40fae025ff52d8b53ff3670b250f3df04071f19fd",
-        "🔺️diff/🔗️.graphql": "d4de9e66f0c54157b0a089c3641b8cf05b77ba2037e3897d44217ef031f56a74",
-        "🔺️diff/🔣️.json": "f3cab1b62e84ffa64c962016af6e47026120d378ae4aa2f9b61286137c4af652",
-        "🔺️diff/🛰️.proto": "80d770eec4f27f898f359300b1b29f3120fc88d21c432beead672334185d35af",
-        "🔺️diff/🟦️.ts": "758f4a04e18172dfcfd1e06c6c2e48b9457c0e9f9a9db05cdd9c7fc5d8762c0a",
-        "🔺️diff/🦀️.rs": "597cbaabd9124a4880acd3bebada5f4d8b6a5f446a332473e65ad44eb8169b64",
+        "🔺️diff/🔗️.graphql": "e92883e114154d40f55a721e278ffdc15b4394f7fec4b9c30963036fc44d16dc",
+        "🔺️diff/🔣️.json": "c2cab9d55119ee0443a30250e1d3f1eae51fd68371f723cabbcc111bed4d7146",
+        "🔺️diff/🛰️.proto": "5bd9583d11180cb10463e8f845e5de64ad5866f3f231181772814cc353756b33",
+        "🔺️diff/🟦️.ts": "c60ad84c141f634c3e80ab57ea037cf4aa6984f9d8e3058f00630a2294de9330",
+        "🔺️diff/🦀️.rs": "fd0df50e6e1f2f4cc788ca14936f2949e399652c32b40e9ab6592b3318b87082",
         "🛰️.proto": "a8f75e8370fd7bc0b2d503726f61a21002d75c8eba8d7f963bab46ec3e459d9f",
         "🟦️.ts": "19a53d7dda33c5fb0c9e5042f151c4651aff44de2bb79ca5ee14fe065a5d5367",
@@ -96480,5 +98415,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "6268aa8dd7dc4988ddbc7e69f7be7bc0cb414ecd19659bc477b16e193ecc123a"
+        "🔣️.json": "284e53e097b567bfbfe9f967a08eaeb75b954dd097b508f69839d3e407189e86"
       }
     },
@@ -96578,5 +98513,5 @@

       "dependsOn": [],
       "hashes": {
-        "🔣️.json": "039771dcf4523dbea097fbcfb35c5bd7b15ee078a53102a312dbd0ad7169bfe3"
+        "🔣️.json": "7360f65de9307bba7a7a73d844f24604e1d8b4c22117d0ab8ff557a1c7d19949"
       }
     },
@@ -97144,5 +99079,5 @@

         "🔺️diff/🛰️.proto": "ca75234c648dac0cef2479e02abeee41a5d6d9682e729c474f1a126671e02aa1",
         "🔺️diff/🟦️.ts": "8d277181409d392ff5d710c0eb04483c9627e4187c2266798626492fce37b803",
-        "🔺️diff/🦀️.rs": "04a5a6a942b784328a8ea63c75e9b0931f7d4925755d10af8983d0bea060964a",
+        "🔺️diff/🦀️.rs": "e0b719a97884095b960b960d18e50d5d2c01a374e327187f5dbe2b143e298366",
         "🛰️.proto": "7a59f78224e68d902d8e03c6e6196a72bfd6ae7d34c9afaddb0923adfb3c72d8",
         "🟦️.ts": "ccf1549611589556898a31542d862413564793e789d3cf4a48369d58a25af5ce",
@@ -97404,5 +99339,5 @@

         "🔺️diff/🛰️.proto": "c34cdf04b1a9e15a253c5137da3537cf8ce37d8b4fa3d2ef45ae70e836515803",
         "🔺️diff/🟦️.ts": "a862e44c487c698cccaaecafa9ba3ef1e257912e6c6676212c793bcf209b8677",
-        "🔺️diff/🦀️.rs": "00bc2b0f3879dc396e4807098bcdc72f52f589973e3821c42912112aeb604954",
+        "🔺️diff/🦀️.rs": "55513ace34d78186f54512530108b07e473861a432629dc5e20304086dde8f1d",
         "🛰️.proto": "69d9d812eac914a2f8f3b7162bab9a492c8f561637d92b68b4a41a2e111d3b3e",
         "🟦️.ts": "d4e4e2d75e1f6d4c65e1e665c7d7be4d256160f9dc8de2f52a606fe4ea2652dd",
@@ -97582,5 +99517,5 @@

         "🔺️diff/🛰️.proto": "323aa6d74fb808e80bfb2bb95198919b4e2bd1a605259e4415f15c0d598595fd",
         "🔺️diff/🟦️.ts": "7504391b61f6a5f11841dd29ef069fdbbc3bb0e30d8c08995b4d07e81d2cb29d",
-        "🔺️diff/🦀️.rs": "c9f16d17bce7266beac1550a9f4e69fa42b2c2380198a9b6fd3532c0db081f1f",
+        "🔺️diff/🦀️.rs": "94c3725da392a449d5fcc0bf4ee5e2168caf02138d01d9c2be51e21e5678307d",
         "🛰️.proto": "66a455f3fb00aaff8fe027c6707ff3f45f849c4a3912c4d874892440a5aee6d1",
         "🟦️.ts": "278a7d45c0a38e0ae8376d14691266124f4dfff54516c2f00e1bb8082f69e2d9",
@@ -98581,22 +100516,4 @@

       }
     },
-    "s.wfc.wfc2d.1.any.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔄️replace-config/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "cfeb69603841ff1d78fb78b5998393d1b4a68e49e023655249f834a572506d0f"
-      }
-    },
     "s.wfc.wfc2d.1.any.mutation.change-seed": {
       "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧬️schema",
@@ -98988,5 +100905,5 @@

         "🔺️diff/🛰️.proto": "89c86acd2c646b0cded2680c8154d44a860f366f4044f11a3f387b6fa8929947",
         "🔺️diff/🟦️.ts": "a03413e99862925044ef1ee22aee7156d79854d806362a7b64c8324b7383d9df",
-        "🔺️diff/🦀️.rs": "c71cbbdd1374dc5187c849f850d499e1995967722b7386f0ad1a460d1cac95f0",
+        "🔺️diff/🦀️.rs": "dee69110a46e2d3fb10ded21723c0c3a1be4584768c03c8a1c4dc6a813336751",
         "🛰️.proto": "af69dd64540aff88023260625d9ec08f384f1f5b1b3258a9a9b593e1beacac1a",
         "🟦️.ts": "e84da5befab06ff84536a0fac028ee547409ea5373b7489c1efb1e0017acb021",
@@ -99033,22 +100950,4 @@

       "hashes": {
         "🔣️.json": "52981057905bb9479d45de4ba615b75c1bdb66d5537500f84a3542a88f1359da"
-      }
-    },
-    "s.wfc.wfc3d.1.any.config.mutation.replace-config": {
-      "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔄️replace-config/🧬️schema",
-      "level": "mutation-leaf",
-      "facetKind": "🧬️data",
-      "formats": {
-        "🔣️jsonschema": "🔣️.json"
-      },
-      "exports": {
-        "ReplaceConfig": {
-          "file": "🔣️.json",
-          "facet": "schema"
-        }
-      },
-      "dependsOn": [],
-      "hashes": {
-        "🔣️.json": "c83b9d35da6e25dc5fd68ff65819914410a307fe0ae61460f3ec21ca2e5a64d7"
       }
     },
@@ -99435,5 +101334,5 @@

         "🔺️diff/🛰️.proto": "970f9f9a0c7720952858eaadc358c20bc9d783a575b5bb3fcc5cf1ce4f6197e7",
         "🔺️diff/🟦️.ts": "4840b786412c309cbaf4ecfb94ab49096483faa639e2aa3125428820f847b799",
-        "🔺️diff/🦀️.rs": "088d73d120f0c773e302c89680f12bd691c5d7bdc2bcec3683b736693129c284",
+        "🔺️diff/🦀️.rs": "0c3f2efc0098522d4edb6b97abd7550c94ac72d22052f7b1f670c9bb0f17f6ed",
         "🛰️.proto": "34aec05e413f0fc41c6dff1f20af648b0e9cbb8962a008ae5ffa2a7f1d4ac2fc",
         "🟦️.ts": "1ca45d5baa8fc2d3403e55b814595077faa9496011e3d2bb71f06e7a2956a1cf",

### 91 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
HEAD 32732 BASE 31519 STASH 31508 CURRENT H
--- 

+++ 

@@ -24077,15 +24077,4 @@

       "expires": null
     },
-    "root-dotnet-solution": {
-      "pathPattern": "Monorepo.sln",
-      "authority": ".NET SDK",
-      "reason": "Solution discovery",
-      "configurability": "unconfigurable",
-      "scope": {
-        "kind": "repository-root"
-      },
-      "verification": "dotnet restore",
-      "expires": null
-    },
     "root-bun-lock": {
       "pathPattern": "bun.lock",

### 92 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs
HEAD 403 BASE 361 STASH 366 CURRENT H
--- 

+++ 

@@ -7,5 +7,7 @@

 const ownershipPath = join(__dirname, "../🔣️policy.json"), ownershipSource = readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes);
 const {workspaceTestingCollectionRoot: collectionRoot} = require("../🟨️.cjs");
-const opaque = new Set(["🗑️generated", "node_modules", "target", "dist", "build", "storybook-static", "temp", "coverage", "🔌️plugin-modules", "compose"]);
+const opaque = new Set(["🗑️generated", "🤖️generated", "node_modules", "target", "dist", "📤️dist", "build", "storybook-static", "temp", "coverage", "__pycache__", "🔌️plugin-modules", "compose"]);
+const outputName = name => opaque.has(name) || /^target(?:-[a-z0-9]+)*$/u.test(name);
+const outputDirectory = (root, directory) => relative(root, directory).split(/[\\/]/u).some(outputName);
 
 /** 🧬️ Admits authored native patterns independently of package identity or product roles.
@@ -82,4 +84,5 @@

 }
 function* walkPlan(owner, directory) {
+  if (outputDirectory(owner.root, directory)) return { paths: [], collections: [] };
   const collection = collectionRoot(owner.root, directory);
   if (collection) return { paths: [], collections: [collection] };
@@ -88,5 +91,5 @@

   const entries = yield { operation: "list", path: directory }, paths = [], collections = [];
   if (entries.some(entry => entry.name === "package.json")) paths.push(directory);
-  for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
+  for (const entry of entries) if (!entry.name.startsWith(".") && !outputName(entry.name)) {
     if (entry.kind === "symlink") throw Error("Bun source directory is a symlink: " + join(directory, entry.name));
     if (entry.kind === "directory") { const child = yield* walkPlan(owner, join(directory, entry.name)); paths.push(...child.paths); collections.push(...child.collections); }
@@ -98,5 +101,7 @@

   const positive = scope.declaration.members.filter(path => !path.startsWith("!")), negative = scope.declaration.members.filter(path => path.startsWith("!")).map(path => patternRegex(path.slice(1))), found = new Set();
   for (const pattern of positive) {
-    const selector = patternRegex(pattern), start = bounded(owner.root, join(scope.directory, sourcePrefix(pattern))), state = yield { operation: "state", path: start };
+    const selector = patternRegex(pattern), start = bounded(owner.root, join(scope.directory, sourcePrefix(pattern)));
+    if (outputDirectory(owner.root, start)) continue;
+    const state = yield { operation: "state", path: start };
     if (state === "missing") continue;
     if (state !== "directory") throw Error("Bun source owner must be a real directory: " + start);
@@ -144,5 +149,5 @@

     if (pattern !== "*/package.json") throw Error("Unsupported physical owner recipe: " + pattern);
     const entries = yield { operation: "list", path: owner.root };
-    for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name) && !collectionRoot(owner.root, join(owner.root, entry.name))) {
+    for (const entry of entries) if (!entry.name.startsWith(".") && !outputName(entry.name) && !collectionRoot(owner.root, join(owner.root, entry.name))) {
       if (entry.kind === "symlink") throw Error("Bun installation owner is a symlink: " + entry.name);
       if (entry.kind === "directory" && (yield { operation: "state", path: join(owner.root, entry.name, "package.json") }) !== "missing") paths.add(entry.name + "/package.json");
@@ -206,5 +211,5 @@

       if (this.state(path) !== "directory") throw Error("Bun workspace requires a real directory: " + path);
       budget.entries = budget.entries - prior + entries.length; budget.lists.set(path, entries.length); entries.sort((a, b) => Buffer.from(a.name).compare(Buffer.from(b.name)));
-      retain("list", path, observations ? entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !opaque.has(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name)))) : entries); return entries;
+      retain("list", path, observations ? entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !outputName(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name)))) : entries); return entries;
     },
     readText(path) {

### 93 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests/🧫️fixtures/🔣️.json
HEAD 944 BASE 929 STASH 942 CURRENT H
--- 

+++ 

@@ -1,4 +1,17 @@

 {
   "schemaVersion": 1,
+  "outputCensus": {
+    "members": ["source/**", "source/🤖️generated/**"],
+    "outputs": ["🤖️generated", "🗑️generated", "node_modules", "target", "target-prototype", "📤️dist", "dist", "build", "storybook-static", "temp", "coverage", "__pycache__", "🔌️plugin-modules"],
+    "cases": [
+      { "mutation": "generatedChurn", "outcome": "admitted" },
+      { "mutation": "generatedBirth", "outcome": "admitted" },
+      { "mutation": "generatedRemoval", "outcome": "admitted" },
+      { "mutation": "authoredManifest", "outcome": "rejected" },
+      { "mutation": "authoredDirectory", "outcome": "rejected" },
+      { "mutation": "authoredRoot", "outcome": "rejected" },
+      { "mutation": "cancel", "outcome": "canceled" }
+    ]
+  },
   "accepted": [
     {

### 94 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧬️schema/🔣️.json
HEAD 264 BASE 224 STASH 245 CURRENT H
--- 

+++ 

@@ -36,4 +36,25 @@

   "description": "Portable authoritative Source membership recipes use literal path components, * and ? within a component, and ** for zero or more components. Brackets and braces are literal Source names. Native Bun workspaces are the complete current byte-sorted native literal directory projection of independently owned packages, excluding the installation scope itself and all testing collections. Native metacharacters are escaped and an empty brace alternative selects that same exact directory through Bun glob interpretation; ordinary directories remain direct literal entries. Leading ! is prefixed with ./ to preserve literal identity. Source discovery does not depend on published native projection. Only the explicitly selected scopes are refreshed through the original caller control before the first installation; current topology, manifest and policy guards refuse concurrent changes.",
   "$defs": {
+    "OutputCensus": {
+      "type": "object",
+      "additionalProperties": false,
+      "required": ["members", "outputs", "cases"],
+      "properties": {
+        "members": { "type": "array", "items": { "type": "string" } },
+        "outputs": { "type": "array", "uniqueItems": true, "items": { "type": "string", "minLength": 1, "pattern": "^[^/\\\\]+$" } },
+        "cases": {
+          "type": "array",
+          "items": {
+            "type": "object",
+            "additionalProperties": false,
+            "required": ["mutation", "outcome"],
+            "properties": {
+              "mutation": { "enum": ["generatedChurn", "generatedBirth", "generatedRemoval", "authoredManifest", "authoredDirectory", "authoredRoot", "cancel"] },
+              "outcome": { "enum": ["admitted", "rejected", "canceled"] }
+            }
+          }
+        }
+      }
+    },
     "OwnershipProgress": {
       "type": "object",

### 95 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🔁️replan/🧪️tests/🟦️.ts
HEAD 18 BASE 18 STASH 18 CURRENT H
--- 

+++ 

@@ -14,5 +14,5 @@

 const packageManifest=(version:string)=>`[package]\nname="replan-kernel"\nversion="${version}"\nedition="2021"\n[lib]\npath="lib.rs"\n`;
 /** 🔎️ Type-checks the defining preparation owner and verifies both registered mode invocations. */
-test("Cargo manifest replanning defining types and launch inventories are complete",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"test:cargo-replan-source",30000);await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",import.meta.dir),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",join(import.meta.dir,"../../🟦️.ts")],workspace,{control});const{parse}=await import("jsonc-parser");for(const path of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const errors:any[]=[],launch=parse(readFileSync(join(workspace,path),"utf8"),errors,{allowTrailingComma:path.endsWith("jsonc")});expect(errors).toEqual([]);for(const mode of["prototype","release"])expect(launch.configurations.filter((row:any)=>row.name===`⚖️cargo-replan ${mode}`).length).toBe(1);}console.log("[DEBUG] Cargo original manifest replanning defining strict types and both launch inventories qualified");});},60000);
+test("Cargo manifest replanning defining types and dashboard controls are complete",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"test:cargo-replan-source",30000);await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",import.meta.dir),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",join(import.meta.dir,"../../🟦️.ts")],workspace,{control});const{parse}=await import("jsonc-parser");for(const path of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const errors:any[]=[],launch=parse(readFileSync(join(workspace,path),"utf8"),errors,{allowTrailingComma:path.endsWith("jsonc")});expect(errors).toEqual([]);for(const mode of["prototype","release"])expect(launch.configurations.filter((row:any)=>row.name===`⚖️cargo-replan ${mode}`).length).toBe(1);}const dashboard=JSON.parse(readFileSync(join(workspace,"📋️project.json"),"utf8")).metadata.semio.dashboard,source=dashboard.tools.find((tool:any)=>tool.id==="run-script");expect(source.command).toEqual(["bun","repo","exec","--projects={project}","--","bun","{workspace}/{script}"]);expect(dashboard.parameters.find((parameter:any)=>parameter.id==="build-mode").values).toEqual([{id:"ship",env:{SEMIO_BUILD_MODE:"release"},buildFlags:["--skip-cache","--skip-remote-cache"]}]);console.log("[DEBUG] Cargo original manifest replanning defining strict types and dashboard source/ship controls qualified");});},60000);
 /** 🔄️ Real Cargo identity and metadata agree with bounded pre-command semantic replanning. */
 for(const row of corpus.cases)test("Cargo original preparation replan "+row.kind,async()=>{expect(new Ajv({strict:true}).validate(schema.definitions.Policy,corpus.policy)).toBe(true);expect(new Ajv({strict:true}).validate(schema.definitions.Case,row)).toBe(true);await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"rp-")),storage=repositoryCargoPreparationStorageV1(workspace),control=scriptCompilerAdmissionControlV1(original,"test:cargo-replan",row.kind==="expired"?100:30000),progress:string[]=[],admission={...control,publish:async(event:any)=>{progress.push(event.resource);await control.publish(event);}},db=new Database(":memory:");let identities=0,dependencies=0,version="0.1.0",failure:unknown;mkdirSync(join(root,"kernel"));writeFileSync(join(root,"Cargo.toml"),workspaceManifest);writeFileSync(join(root,"kernel/Cargo.toml"),packageManifest(version));writeFileSync(join(root,"kernel/lib.rs"),"pub fn current() {}\n");db.exec("create table observations(kind text, identities integer, dependencies integer, refused integer)");const edit=()=>{version=version==="0.1.0"?"0.2.0":"0.1.0";writeFileSync(join(root,"kernel/Cargo.toml"),packageManifest(version));};try{try{await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",admission,async(args,cwd,signal)=>{expect(signal).toBe(original.control.signal);dependencies++;await captureOwnedCommandV1("cargo",[...args,"--offline"],cwd,{control:admission});if(row.kind==="changed-after-update"&&args[0]==="update")edit();},async(cwd,signal)=>{expect(signal).toBe(original.control.signal);identities++;const identity=await captureOwnedCommandV1("cargo",["--version"],cwd,{control:admission});if(row.kind==="identical")writeFileSync(join(root,"kernel/Cargo.toml"),packageManifest(version));if(row.kind==="always-changing"||row.kind==="changed-once"&&identities===1)edit();if(row.kind==="expired")await Bun.sleep(200);return identity;},{signal:original.control.signal,advance:async()=>{await original.control.yieldContinuation();},childBudget:requested=>Math.min(requested,admission.remainingMilliseconds()!),childOptions:()=>{throw Error("Unowned recipe execution");}});}catch(error){failure=error;}db.query("insert into observations values(?,?,?,?)").run(row.kind,identities,dependencies,failure===undefined?0:1);expect(db.query("select identities,dependencies,refused from observations where kind=?").get(row.kind)).toEqual({identities:row.identityCalls,dependencies:row.dependencyCalls,refused:row.refused?1:0});expect(readFileSync(join(root,"kernel/Cargo.toml"),"utf8")).toBe(packageManifest(version));if(!row.refused){const metadata=JSON.parse(await captureOwnedCommandV1("cargo",["metadata","--manifest-path",join(root,"Cargo.toml"),"--locked","--offline","--no-deps","--format-version=1"],root,{control:admission}));expect(metadata.packages[0].version).toBe(version);expect(existsSync(join(root,"target"))).toBe(false);}else expect(String(failure)).toMatch(row.kind==="expired"?/deadline exhausted/u:/Prepared manifest changed/u);console.log("[DEBUG] Actual original Cargo preparation "+JSON.stringify({kind:row.kind,identities,dependencies,refused:failure!==undefined,progress,version}));}finally{db.close();rmSync(root,{recursive:true,force:true});}});},60000);

### 96 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts
HEAD 89 BASE 71 STASH 92 CURRENT H
--- 

+++ 

@@ -1,2 +1,6 @@

+import {randomBytes} from "node:crypto";
+import {rmSync} from "node:fs";
+import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
+import type {CargoPreparationControlV1} from "../../🟦️.ts";
 import custodySchema from "./🧬️schema/🔣️.json";
 import {createHash} from "node:crypto";
@@ -70,2 +74,19 @@

  const state=capture();if(!state)return;const row={...state.row,resolverInputs:[],resolutions:[],sources:[...state.sources.values()],inputs:[...state.inputs.values()],outputs:[...state.outputs.values()]};assertCargoPreparationObservationCurrentV1(row);writeFileSync(process.env.SEMIO_CARGO_PREPARATION_OBSERVATION!,JSON.stringify(row));active=undefined;
 }
+
+/** 🛰️ Captures an exact resolver response in its own bounded original-control child. */
+async function controlledResolver(control:CargoPreparationControlV1,root:string,directory:string,args:readonly string[]):Promise<unknown> {
+ control.signal.throwIfAborted();const output=resolve(directory,"r-"+randomBytes(8).toString("hex")+".json"),budget=control.childBudget(30_000);if(!Number.isSafeInteger(budget)||budget<1||budget>30_000)throw Error("Finite original resolver child budget required");
+ try{await runOwnedCommand(process.execPath,[fileURLToPath(new URL("../📜️script.ts",import.meta.url)),...args,output],root,"cargo:preparation-custody",budget,{env:{...process.env,NX_WORKSPACE_ROOT:root},signal:control.signal,stdout:"stderr",output:{maximumBytes:134217728,maximumLines:65536},onProgress:async line=>{await control.advance({operation:"program",path:args[1]??root,completedOperations:0});control.signal.throwIfAborted();process.stderr.write(line+"\n");}});control.signal.throwIfAborted();return JSON.parse(readFileSync(output,"utf8"));}catch(error){control.signal.throwIfAborted();throw error;}finally{rmSync(output,{force:true});}
+}
+/** 📡️ Captures the exact current program closure without blocking original cancellation. */
+export async function cargoPreparationProgramSourcesControlledV1(root:string,entry:string,control:CargoPreparationControlV1,directory:string):Promise<ReturnType<typeof cargoPreparationProgramSourcesV1>> {
+ control.signal.throwIfAborted();physical(root,entry);const value=await controlledResolver(control,root,directory,["capture-program-file",root,entry]),row=admitCargoPreparationObservationV1({...value as object,version:1,root,script:entry,command:["capture-program"],outputs:[]});return{sourceRoot:row.sourceRoot,sources:row.sources,resolverInputs:row.resolverInputs,inputs:row.inputs,resolutions:row.resolutions};
+}
+/** 🧭️ Rechecks exact resolver and physical custody while retaining the original signal and budget. */
+export async function assertCargoPreparationObservationCurrentControlledV1(value:unknown,control:CargoPreparationControlV1,directory:string):Promise<void> {
+ control.signal.throwIfAborted();const row=admitCargoPreparationObservationV1(value);
+ for(const edge of row.resolutions){control.signal.throwIfAborted();cargoPreparationProgramRootV1(row.root,row.sourceRoot,edge.source);cargoPreparationProgramRootV1(row.root,row.sourceRoot,edge.selected);}
+ if(row.resolutions.length){const input=resolve(directory,"i-"+randomBytes(8).toString("hex")+".json");try{writeFileSync(input,JSON.stringify(row.resolutions));const selected=await controlledResolver(control,row.root,directory,["resolve-program-file",input]);if(!Array.isArray(selected)||selected.length!==row.resolutions.length)throw Error("Invalid current program resolution");for(const [index,edge] of row.resolutions.entries())if(selected[index]!==edge.selected)throw Error(`Preparation program resolution changed: ${edge.source} ${edge.specifier}`);}finally{rmSync(input,{force:true});}}
+ const outputs=new Set(row.outputs.map(input=>input.path));let completed=0;for(const input of [...row.sources,...row.resolverInputs,...row.inputs.filter(input=>!outputs.has(input.path)),...row.outputs]){control.signal.throwIfAborted();const root=row.sources.includes(input)?cargoPreparationProgramRootV1(row.root,row.sourceRoot,input.path):row.resolverInputs.includes(input)?row.sourceRoot:row.root;if(cargoPreparationInputV1(root,input.path,input.kind).sha256!==input.sha256)throw Error(`Preparation custody changed: ${input.path}`);await control.advance({operation:"observation",path:input.path,completedOperations:++completed});control.signal.throwIfAborted();}
+}

### 97 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts
HEAD 412 BASE 412 STASH 421 CURRENT H
--- 

+++ 

@@ -32,8 +32,8 @@

 }
 
-export type CargoWorkspaceProgressV1=Readonly<{operation:"state"|"presence"|"list"|"text"|"recipe";path:string;completedOperations:number}>;
+export type CargoWorkspaceProgressV1=Readonly<{operation:"state"|"presence"|"list"|"text"|"recipe"|"program"|"observation"|"queue";path:string;completedOperations:number}>;
 export interface CargoWorkspaceControlV1 {readonly signal:AbortSignal;advance(progress:CargoWorkspaceProgressV1):Promise<void>;}
 export interface CargoPreparationControlV1 extends CargoWorkspaceControlV1 {childBudget(requestedMilliseconds:number):number;childOptions(environment:Readonly<Record<string,string|undefined>>):OwnedCommandOptions&{readonly signal:AbortSignal;readonly output:NonNullable<OwnedCommandOptions["output"]>};}
-type CargoPhysicalOperationV1=Readonly<{operation:"state"|"presence"|"list"|"text";path:string;root:string}>|Readonly<{operation:"recipe";path:string;root:string;args:string[];cwd:string;sourceSha256:string;environment:Record<string,string|undefined>}>;
+type CargoPhysicalOperationV1=Readonly<{operation:"state"|"presence"|"list"|"text"|"program";path:string;root:string}>|Readonly<{operation:"observation";path:string;root:string;directory:string;value:CargoPreparationObservationV1}>|Readonly<{operation:"recipe";path:string;root:string;args:string[];cwd:string;sourceSha256:string;environment:Record<string,string|undefined>}>;
 type CargoPhysicalPlanV1<T>=Generator<CargoPhysicalOperationV1,T,any>;
 /** 🛡️ Rechecks current directory custody immediately before one physical operation. */
@@ -45,4 +45,6 @@

 function recipeCurrent(step:Extract<CargoPhysicalOperationV1,{operation:"recipe"}>):void {directory(step.root,step.cwd);if(createHash("sha256").update(readFileSync(physical(step.root,step.path))).digest("hex")!==step.sourceSha256)throw Error("Cargo recipe source changed before acquisition");}
 const operate=(step:CargoPhysicalOperationV1):any=>{
+ if(step.operation==="program")return cargoPreparationProgramSourcesV1(step.root,step.path);
+ if(step.operation==="observation")return assertCargoPreparationObservationCurrentV1(step.value);
  if(step.operation==="recipe"){recipeCurrent(step);const result=Bun.spawnSync([process.execPath,step.path,...step.args],{cwd:step.cwd,env:step.environment,stdout:"pipe",stderr:"inherit",timeout:30_000});if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw Error(`Cargo owner preparation failed: ${step.path} (${result.exitCode})`);return;}
  if(step.operation==="list"){directory(step.root,step.path);return readdirSync(step.path,{withFileTypes:true});}
@@ -167,4 +169,8 @@

 }
 
+/** 🎯️ Resolves the current native owner through the original physical-operation control. */
+export function cargoWorkspaceForManifestControlledV1(root:string,manifest:string,control:CargoWorkspaceControlV1):Promise<CargoWorkspaceScope> {return controlled(selectPlan(root,manifest,path=>readPlan(root,path),new Map()),control);}
+
+
 /** 🏎️ Binds native profiles to their selected workspace or the repository-wide configuration. */
 export function cargoNextestConfiguration(root:string,directory:string):string {
@@ -312,7 +318,7 @@

  const localDependencies=function*(manifest:string,development:boolean):CargoPhysicalPlanV1<string[]>{
  const {owner,pkg,current}=yield* binding(manifest),authority=(yield* document(owner.manifest)).workspace,groups=[current,...Object.values(current.target??{})] as Record<string,any>[],paths:string[]=[];
- for(const group of groups)for(const kind of development?["dependencies","dev-dependencies","build-dependencies"]:["dependencies","build-dependencies"])for(const [alias,value] of Object.entries(group[kind]??{})){
+ for(const group of groups)for(const kind of ["dependencies","dev-dependencies","build-dependencies"])for(const [alias,value] of Object.entries(group[kind]??{})){
  const entry=value as any;if(!entry || typeof entry!=="object")continue;const dependency=entry.workspace?authority.dependencies?.[alias]:entry;
- if(!dependency)throw Error(`Missing inherited Cargo dependency: ${manifest} ${alias}`);if(typeof dependency.path!=="string")continue;
+ if(!dependency)throw Error(`Missing inherited Cargo dependency: ${manifest} ${alias}`);if(kind==="dev-dependencies"&&!development)continue;if(typeof dependency.path!=="string")continue;
  const path=resolve(root,entry.workspace?owner.directory:pkg.directory,dependency.path,"Cargo.toml"),local=slash(relative(root,path));
  if(local.startsWith("../") || isAbsolute(local))throw Error(`Cargo dependency escapes preparation authority: ${alias}`);yield* physicalPlan(root,local);paths.push(local);
@@ -349,5 +355,5 @@

  break;
  }
- for(const recipe of recipes)assertCargoPreparationObservationCurrentV1(recipe);
+ for(const recipe of recipes)yield {operation:"observation",path:recipe.script,root,directory:observationDirectory!,value:recipe};
  diagnostic("finished",selected.manifest,started,pending.size);
  return {manifests:[...new Set([...observedManifests,...scopes.map(scope=>scope.manifest)])],owners:[...observedOwners],recipes};
@@ -375,10 +381,13 @@

 /** 🧭️ Selects the current owned preparation launch without executing or borrowing a completed receipt. */
 export function cargoWorkspacePreparationInvocationV1(storage:CargoPreparationStorageV1,root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string,string|undefined>>):CargoPreparationInvocationV1|undefined {
+  return drain(preparationInvocationPlan(storage,root,args,cwd,environment));
+}
+function* preparationInvocationPlan(storage:CargoPreparationStorageV1,root:string,args:readonly string[],cwd:string,environment:Readonly<Record<string,string|undefined>>):CargoPhysicalPlanV1<CargoPreparationInvocationV1|undefined> {
   parseCargoPreparationStorageV1(storage);if (!cargoCommandRequiresOwnerPreparationV1(args[0] ?? "")) return;
   const path = cargoInvocationManifestV1(args, cwd);
   if (environment.SEMIO_CARGO_PREPARATION_ACTIVE) throw new Error(`Cargo recursion in owner preparation: ${environment.SEMIO_CARGO_PREPARATION_ACTIVE}`);
-  const owner = cargoWorkspaceForManifest(root, slash(relative(root, path))), source = read(root, owner.manifest), names:string[]=[];
+  const owner = yield* selectPlan(root, slash(relative(root, path)),path=>readPlan(root,path),new Map()), source = yield* readPlan(root, owner.manifest), names:string[]=[];
   for(let cursor=1;cursor<args.length && args[cursor]!=="--";cursor++){const arg=args[cursor]!;if(arg==="-p" || arg==="--package"){const name=args[++cursor];if(!name)throw Error("Cargo package selector requires a current name");names.push(name);}else if(arg.startsWith("--package="))names.push(arg.slice(10));else if(arg.startsWith("-p") && arg.length>2)names.push(arg.slice(2));}
-  const selectedPackage=read(root,slash(relative(root,path))).package?.name;
+  const selectedPackage=(yield* readPlan(root,slash(relative(root,path)))).package?.name;
   if(!names.length && !args.includes("--workspace") && typeof selectedPackage==="string")names.push(selectedPackage);
   if (source.workspace?.metadata?.semio?.repository === undefined)return;

### 98 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts
HEAD 362 BASE 360 STASH 382 CURRENT H
--- 

+++ 

@@ -1,2 +1,3 @@

+import {cargoWorkspaceForManifestControlledV1} from "../🟦️.ts";
 import {repositoryCargoPreparationStorageV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
 import {stripVTControlCharacters} from "node:util";
@@ -359,2 +360,23 @@

  for(const selection of fixture.reachablePreparation){let valid=false,diagnostic="";try{await captureCargoFixtureV1(root,["tree","--offline","--manifest-path",join(root,"Cargo.toml"),...selection.names.flatMap((name:string)=>["-p",name])]);valid=true;}catch(error){diagnostic=String(error);if(!/exited with status/u.test(diagnostic))throw error;}expect(valid,diagnostic).toBe(selection.valid);if(selection.valid)expect(()=>prepareCargoOwners(root,selected,selection.names)).not.toThrow();else expect(()=>prepareCargoOwners(root,selected,selection.names)).toThrow();}
 });
+
+
+test("reachable inherited development aliases match Cargo metadata without expanding production preparation",()=>{
+ const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8")).inheritedAliases;
+ for(const row of law.cases){
+  const root=mkdtempSync(join(artifactRoot!,"cargo-alias-"));
+  put(root,"Cargo.toml",workspace(["kernel"]));put(root,"kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"kernel/🦀️.rs","");put(root,"application/Cargo.toml",workspace(["app"]));put(root,"application/app/Cargo.toml",pkg("selected-app",'[dependencies]\nowned-dependency={path="../../dependency/member"}\n'));put(root,"application/app/🦀️.rs","");
+  put(root,"dependency/Cargo.toml",workspace(["member"],row.declared?'[workspace.dependencies]\n'+law.alias+'={path="../observer/member"}\n':""));put(root,"dependency/member/Cargo.toml",pkg("owned-dependency",'['+row.section+']\n'+law.alias+'={workspace=true}\n'));put(root,"dependency/member/🦀️.rs","");
+  put(root,"observer/Cargo.toml",workspace(["member"]));put(root,"observer/member/Cargo.toml",pkg(law.alias));put(root,"observer/member/🦀️.rs","");
+  const manifests=["Cargo.toml","application/Cargo.toml","dependency/Cargo.toml","observer/Cargo.toml"],before=manifests.map(path=>readFileSync(join(root,path),"utf8")),native=cargo(root,"application/Cargo.toml");expect(native.status===0,row.id+"\n"+native.text).toBe(row.valid);
+  if(row.valid){const prepared=prepareCargoOwners(root,cargoWorkspaceForManifest(root,"application/Cargo.toml"),["selected-app"]);expect([...prepared.owners].sort()).toEqual(law.productionOwners);expect(JSON.parse(native.text).workspace_members).toHaveLength(1);}else expect(()=>prepareCargoOwners(root,cargoWorkspaceForManifest(root,"application/Cargo.toml"),["selected-app"])).toThrow("Missing inherited Cargo dependency");
+  expect(manifests.map(path=>readFileSync(join(root,path),"utf8"))).toEqual(before);
+  console.log("[DEBUG] Cargo inherited alias case="+row.id+" valid="+row.valid+" productionOwners="+law.productionOwners.join(","));
+ }
+});
+
+
+ test("controlled native owner resolution preserves original cancellation and independent Cargo ownership",async()=>{
+ for(const row of fixture.controlledSelection){const root=mkdtempSync(join(artifactRoot!,"ctrl-"));put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n");const controller=new AbortController(),reason=Error("neutral original cancellation");let completed=0;if(row.cancelAfter===0)controller.abort(reason);const control={signal:controller.signal,async advance(){completed++;if(row.cancelAfter===completed)controller.abort(reason);}};if(row.expectedOwner===null){await expect(cargoWorkspaceForManifestControlledV1(root,row.manifest,control)).rejects.toBe(reason);expect(completed).toBe(row.cancelAfter);}else{const selected=await cargoWorkspaceForManifestControlledV1(root,row.manifest,control);expect(selected.directory).toBe(row.expectedOwner);const independent=cargo(root,selected.manifest);expect(independent.status,independent.text).toBe(0);expect(JSON.parse(independent.text).workspace_root.replaceAll("\\","/")).toBe(root.replaceAll("\\","/"));expect(completed).toBeGreaterThan(0);}}
+ console.log("[DEBUG] controlled native owner neutral vectors=4 original cancellation preserved");
+});

### 99 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts
HEAD 0 BASE 5182 STASH 5192 CURRENT H
--- 

+++ 

@@ -1697,4 +1697,10 @@

 }
 
+/** 🪁️ Resolves the installed Vite CLI under the explicitly selected development runtime. */
+export function viteDevInvocation(cwd: string, args: string[], runtime: "bun" | "node"): { command: string; args: string[] } {
+  const configured = args.some(arg => arg === "--configLoader" || arg.startsWith("--configLoader=")) ? args : ["--configLoader", runtime === "node" ? "bundle" : "native", ...args];
+  return { command: runtime === "node" ? "node" : process.versions.bun ? process.execPath : "bun", args: [...(runtime === "node" ? ["--experimental-transform-types"] : []), viteCliEntry(cwd), ...configured] };
+}
+
 function bunxCmdArgs(args: readonly string[], cwd: string): string[] {
   const formatted = bunArgsForVite(args);
@@ -1738,6 +1744,10 @@

 /** 🥖️Spawns `bunx` asynchronously; exits with child code. */
 export function spawnBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): Promise<void> {
+  return spawnPackageCli(process.execPath, bunxCmdArgs(args, cwd), cwd, env);
+}
+
+function spawnPackageCli(runtime: string, args: string[], cwd: string, env: NodeJS.ProcessEnv): Promise<void> {
   return new Promise((resolve, reject) => {
-    const child = spawn(process.execPath, bunxCmdArgs(args, cwd), { cwd, env, shell: false, stdio: "inherit" });
+    const child = spawn(runtime, args, { cwd, env, shell: false, stdio: "inherit", windowsHide: true });
     child.on("exit", (code) => {
       if ((code ?? 0) === 0) resolve();

### 100 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts
HEAD 230 BASE 230 STASH 227 CURRENT H
--- 

+++ 

@@ -63,5 +63,5 @@

     expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(12);
     expect(fixture.removedShims).toHaveLength(2);
-    expect(fixture.consumers).toHaveLength(23);
+    expect(fixture.consumers).toHaveLength(22);
     expect(fixture.projectInputs).toHaveLength(8);
   });
@@ -77,12 +77,9 @@

   });
 
-  test("binds every selector, import, source-data record and editor setting to the semantic owners", () => {
+  test("binds every selector, import, source-data record to the semantic owners", () => {
     for (const consumer of fixture.consumers) {
       const source = read(consumer.path);
       for (const token of consumer.tokens) expect(source, `${consumer.path}: ${token}`).toContain(token);
     }
-    const settings = JSON.parse(read(".vscode/settings.json"));
-    expect(settings["eslint.options"].overrideConfigFile).toBe(ownerById.get("root-eslint")!.ownerPath);
-    expect(settings["tailwindCSS.experimental.configFile"]).toBe(ownerById.get("styling-tailwind")!.ownerPath);
   });
 

### 101 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts
HEAD 300 BASE 281 STASH 278 CURRENT H
--- 

+++ 

@@ -13,5 +13,5 @@

   schemaVersion: 1;
   owners: readonly Owner[];
-  selector: Readonly<{ libraryDeclaration: string; relativeOwnerSuffix: string; editorSetting: string; editorPattern: string }>;
+  selector: Readonly<{ libraryDeclaration: string; relativeOwnerSuffix: string }>;
   directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
   registration: Readonly<{ target: string }>;
@@ -246,5 +246,5 @@

   }, 60_000);
 
-  test("registers exact Nx inputs and the real editor selector", () => {
+  test("registers exact Nx inputs for the semantic owners", () => {
     const projectFiles = [resolve(repoRoot, "📋️project.json")];
     walkProductFiles((path) => { if (path.endsWith("/📋️project.json")) projectFiles.push(path); });
@@ -254,7 +254,4 @@

       expect(inputs.some((pattern) => pattern === target || new Bun.Glob(pattern).match(target)), owner.ownerPath).toBe(true);
     }
-    const settings = JSON.parse(readFileSync(resolve(repoRoot, ".vscode/settings.json"), "utf8"));
-    expect(settings[fixture.selector.editorSetting]).toBe(fixture.selector.editorPattern);
-    expect(settings["vitest.configSearchPattern"]).toBeUndefined();
   }, 30_000);
 

### 102 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts
HEAD 186 BASE 158 STASH 134 CURRENT H
--- 

+++ 

@@ -76,6 +76,6 @@

   expect(source.includes("NX_TASK_TARGET_PROJECT")).toBe(false);
   expect(source).toContain('.registerLazy("repo"');
-  for (const path of [".vscode/settings.json", ".devcontainer/devcontainer.json", ".devcontainer/Dockerfile"]) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
-  expect(read(".github/workflows/architecture-quiz.yml").includes(["bun", "nx"].join(" "))).toBe(false);
+  for (const path of [".devcontainer/devcontainer.json", ".devcontainer/Dockerfile"]) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
+  expect(read(".github/workflows/architecture-course.yml").includes(["bun", "nx"].join(" "))).toBe(false);
 });
 
@@ -91,15 +91,5 @@

 test("native executable producers admit portable project declarations", () => nativeHostContractLaws(root));
 
-test("editor entry points expose every build infrastructure command in both languages", () => {
-  for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
-    const document = JSON.parse(read(file));
-    for (const input of fixture.editorInputs) expect(document.inputs.find((row: { id?: string }) => row.id === input.id)).toEqual(input);
-    for (const row of fixture.editorCommands) for (const language of ["en", "de"] as const) {
-      const expected = `${row.labels[language]} [${language.toUpperCase()}]`;
-      const launch = document.configurations.find((value: unknown) => typeof value === "object" && value !== null && (value as { name?: string }).name === expected);
-      expect(launch, expected).toMatchObject({ type: "node-terminal", request: "launch", command: `bun repo ${row.args}`, cwd: "${workspaceFolder}", presentation: { group: row.group } });
-    }
-  }
-});
+test("dashboard entry points retain localized build and source controls",()=>{const dashboard=JSON.parse(read("📋️project.json")).metadata.semio.dashboard;for(const id of ["run-targets","run-script"]){const tool=dashboard.tools.find((tool:any)=>tool.id===id);expect(tool,id).toBeDefined();for(const language of ["en","de"]){expect(tool.title[language].trim().length).toBeGreaterThan(0);expect(tool.description[language].trim().length).toBeGreaterThan(0);}expect(tool.command.slice(0,2)).toEqual(["bun","repo"]);}});
 
 test("application task environments use neutral execution controls", () => {
@@ -107,19 +97,5 @@

 });
 
-test("editor task arguments cross the neutral argument boundary", () => {
-  const commands = (value: unknown): Record<string, unknown>[] => Array.isArray(value) ? value.flatMap(commands) : value && typeof value === "object" ? [...(typeof (value as Record<string, unknown>).command === "string" ? [value as Record<string, unknown>] : []), ...Object.values(value).flatMap(commands)] : [];
-  for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) for (const row of commands(JSON.parse(read(file)))) {
-    if (typeof row !== "object" || row === null || typeof row.command !== "string") continue;
-    expect(row.command, String(row.name)).not.toMatch(/(?:^|\s)nx\s+(?:run|run-many|exec|show|graph|watch|affected)\b/u);
-    if (!row.command.startsWith("bun repo ")) continue;
-    for(const answers of fixture.editorAnswers){
-      const template=createRequire(import.meta.url)("lodash/template"),command=row.command.replace(/\$\{input:(repoBuild[^}]+)\}/gu,(_,id:string)=>{expect(Object.hasOwn(answers,id),id).toBe(true);return answers[id as keyof typeof answers];});
-      expect(command).toBe(template(row.command,{interpolate:/\$\{input:(repoBuild[^}]+)\}/g})(answers));
-      const parsed = parseBuildInvocationV1(parseArgsStringToArgv(command).slice(2), {}).args, boundary = parsed.findIndex(value => /^(?:\d*[<>]|&&|\|\||;)$/u.test(value));
-      const args = boundary < 0 ? parsed : parsed.slice(0, boundary);
-      if (["run", "run-many", "affected", "exec", "watch"].includes(args[0]!)) expect(() => validateBuildTaskArgumentsV1(args), String(row.name)).not.toThrow();
-    }
-  }
-});
+test("dashboard task arguments cross the neutral argument boundary",()=>{const dashboard=JSON.parse(read("📋️project.json")).metadata.semio.dashboard;expect(dashboard.tools.find((tool:any)=>tool.id==="run-targets").command.slice(0,3)).toEqual(["bun","repo","run-many"]);for(const row of fixture.editorCommands)for(const answers of fixture.editorAnswers){const template=createRequire(import.meta.url)("lodash/template"),command=("bun repo "+row.args).replace(/\$\{input:(repoBuild[^}]+)\}/gu,(_,id:string)=>{expect(Object.hasOwn(answers,id),id).toBe(true);return answers[id as keyof typeof answers];});expect(command).toBe(template("bun repo "+row.args,{interpolate:/\$\{input:(repoBuild[^}]+)\}/g})(answers));const parsed=parseBuildInvocationV1(parseArgsStringToArgv(command).slice(2),{}).args;if(["run","run-many","affected","exec","watch"].includes(parsed[0]!))expect(()=>validateBuildTaskArgumentsV1(parsed),row.args).not.toThrow();}});
 
 test("native orchestration and rebuild declarations use neutral build commands", () => {

### 103 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤖️agent-client-configuration/🟦️.ts
HEAD 185 BASE 186 STASH 187 CURRENT H
--- 

+++ 

@@ -4,5 +4,5 @@

 import { describe, expect, test } from "bun:test";
 import { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
-import { tmpdir } from "node:os";
+
 import { dirname, join, resolve } from "node:path";
 import Ajv2020 from "ajv/dist/2020";
@@ -135,5 +135,7 @@

 describe("check and write on a scratch workspace", () => {
   function scratch(): string {
-    const root = mkdtempSync(join(tmpdir(), "agent-clients-"));
+    const temporary = process.env.SEMIO_TEST_OUTPUT_DIR ?? join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/agent-clients");
+    mkdirSync(temporary, { recursive: true });
+    const root = mkdtempSync(join(temporary, "agent-clients-"));
     cpSync(join(repoRoot, AGENT_CLIENT_DECLARATION_MANIFEST), join(root, AGENT_CLIENT_DECLARATION_MANIFEST));
     for (const file of deriveAgentClientFiles(readDeclaredTools(repoRoot))) {
@@ -150,11 +152,10 @@

       writeFileSync(join(root, ".mcp.json"), readFileSync(join(root, ".mcp.json"), "utf8").replace("stdio", "http"));
       writeFileSync(join(root, ".codex/config.toml"), readFileSync(join(root, ".codex/config.toml"), "utf8").replace('"."', '"os"'));
-      rmSync(join(root, ".cursor/mcp.json"));
+      rmSync(join(root, ".codex/config.toml"));
       expect(checkAgentClients(root).map((drift) => [drift.path, drift.reason])).toEqual([
         [".mcp.json", "drift"],
-        [".cursor/mcp.json", "missing"],
-        [".codex/config.toml", "drift"],
+        [".codex/config.toml", "missing"],
       ]);
-      expect([...writeAgentClients(root)].sort()).toEqual([".codex/config.toml", ".cursor/mcp.json", ".mcp.json"]);
+      expect([...writeAgentClients(root)].sort()).toEqual([".codex/config.toml", ".mcp.json"]);
       expect(checkAgentClients(root)).toEqual([]);
       expect(writeAgentClients(root)).toEqual([]);
@@ -169,5 +170,5 @@

       const manifestPath = join(root, AGENT_CLIENT_DECLARATION_MANIFEST);
       writeFileSync(manifestPath, readFileSync(manifestPath, "utf8").replace("conversation.write", "conversation.read"));
-      expect(checkAgentClients(root).map((drift) => drift.path)).toEqual([".mcp.json", ".vscode/mcp.json", ".cursor/mcp.json", ".codex/config.toml"]);
+      expect(checkAgentClients(root).map((drift) => drift.path)).toEqual([".mcp.json", ".codex/config.toml"]);
     } finally {
       rmSync(root, { recursive: true, force: true });

### 104 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts
HEAD 261 BASE 261 STASH 291 CURRENT H
--- 

+++ 

@@ -5,7 +5,8 @@

 import { describe, expect, test } from "bun:test";
 import { spawnSync } from "node:child_process";
+import { createRequire } from "node:module";
 import { existsSync, lstatSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
 import { tmpdir } from "node:os";
-import { extname, join, relative, resolve } from "node:path";
+import { dirname, extname, join, relative, resolve } from "node:path";
 import { pathToFileURL } from "node:url";
 import Ajv from "ajv";
@@ -20,9 +21,9 @@

   cargoLinker: Readonly<{ config: string; linuxTable: string; linuxRustflags: readonly string[]; forbiddenRustflagFragments: readonly string[] }>;
   repoLocalGitConfig: readonly (readonly [string, string])[];
-  unixBootstrap: Readonly<{ path: string; packageManagers: readonly string[]; requiredTools: readonly string[]; forbidden: readonly string[] }>;
+  unixBootstrap: Readonly<{ path: string; packageManagers: readonly string[]; requiredTools: readonly string[]; forbidden: readonly string[]; installerInvocations: number }>;
   windowsBootstrap: Readonly<{ path: string; required: readonly string[]; forbidden: readonly string[] }>;
   nodePin: Readonly<{ manifest: string; dockerfile: string; dockerfileArg: string }>;
   workspaceManifests: Readonly<{ rootManifest: string }>;
-  freshClone: Readonly<{ bootstrapSources: string; bootstrapSourcesSchema: string; entries: readonly string[] }>;
+  freshClone: Readonly<{ bootstrapSources: string; bootstrapSourcesSchema: string; entries: readonly string[]; deferredImports: Readonly<{ source: string; imports: readonly Readonly<{ path: string; kind: string }>[] }> }>;
 }>;
 type BootstrapSources = Readonly<{ sources: readonly Readonly<{ id: string; module: string; export: string; outputs: readonly string[]; requiredBy: string }>[] }>;
@@ -39,9 +40,10 @@

   name: "workspace-packages",
   setup(builder) {
+    builder.onResolve({ filter: /.*/ }, (args) => args.kind === "dynamic-import" ? { path: args.path, external: true } : undefined);
     builder.onResolve({ filter: /^[^./]/ }, (args) => {
       if (args.path.startsWith("node:")) return { path: args.path, external: true };
       try {
         const real = realpathSync(Bun.resolveSync(args.path, args.resolveDir));
-        return real.includes("/node_modules/") ? { path: args.path, external: true } : { path: real };
+        return real.split(/[\\/]/u).includes("node_modules") ? { path: args.path, external: true } : { path: real };
       } catch {
         return { path: args.path, external: true };
@@ -96,4 +98,31 @@

 
 describe("cross-platform bootstrap", () => {
+  test("Nx entrypoints resolve the declared package layout through independent loaders", () => {
+    const contract = JSON.parse(readFileSync(join(libraryRoot, "🧫️fixtures/🥾️nx-entrypoints/🔣️.json"), "utf8"));
+    const schema = JSON.parse(readFileSync(join(libraryRoot, "🧬️schema/🥾️nx-entrypoints/🔣️.json"), "utf8"));
+    expect(new Ajv({ allErrors: true, strict: true }).compile(schema)(contract)).toBe(true);
+    const nodeOracle = spawnSync("node", ["--input-type=module", "-e", 'import {createRequire} from "node:module";import {readFileSync,realpathSync} from "node:fs";import {resolve} from "node:path";const rows=JSON.parse(readFileSync(0,"utf8")),load=createRequire(resolve("package.json"));process.stdout.write(JSON.stringify(rows.map(row=>realpathSync(load.resolve(row.specifier)))));'], { cwd: repoRoot, input: JSON.stringify(contract.cases), encoding: "utf8", windowsHide: true, timeout: 10_000 });
+    expect(nodeOracle.status).toBe(0);
+    const nodeTargets = JSON.parse(nodeOracle.stdout) as string[];
+    const load = createRequire(join(repoRoot, "package.json"));
+    const metadata = load("nx/package.json") as { bin: { nx: string } };
+    for (const row of contract.cases as { id: string; source: string; pattern: string; specifier: string; target: string }[]) {
+      const original = readFileSync(join(libraryRoot, row.source), "utf8").match(new RegExp(row.pattern))?.[1];
+      expect(original).toBe(row.specifier);
+      const independent = realpathSync(load.resolve(row.specifier));
+      expect(realpathSync(Bun.resolveSync(original!, repoRoot))).toBe(independent);
+      expect(nodeTargets[contract.cases.indexOf(row)]).toBe(independent);
+      expect(independent).toBe(realpathSync(join(dirname(load.resolve("nx/package.json")), row.target)));
+      if (row.id === "cli") expect(independent).toBe(realpathSync(join(dirname(load.resolve("nx/package.json")), metadata.bin.nx)));
+    }
+    console.log("[DEBUG] Nx entrypoints: two neutral declarations match Node/Bun loaders and the package bin");
+  });
+
+  test("keeps deferred command imports distinct from initial router imports", async () => {
+    const result = await build({ stdin: { contents: fixture.freshClone.deferredImports.source, resolveDir: repoRoot, loader: "ts" }, bundle: true, write: false, metafile: true, platform: "node", format: "esm", plugins: [workspacePackages] });
+    const imports = Object.values(result.metafile.outputs).flatMap((output) => output.imports.map(({ path, kind }) => ({ path, kind })));
+    expect(imports).toEqual(fixture.freshClone.deferredImports.imports);
+    expect(Object.keys(result.metafile.inputs)).toEqual(["<stdin>"]);
+  });
   test("validates the portable law fixture", () => {
     
@@ -195,5 +224,6 @@

     expect(source.match(/\bapt-get (update|install)\b/gu)?.length ?? 0).toBe(source.match(/run_privileged env DEBIAN_FRONTEND=noninteractive apt-get (update|install)\b/gu)?.length ?? 0);
     const calls = source.replace(/\\\n\s*/gu, " ").split("\n").filter((line) => /^\s*install_linux_packages\s/u.test(line));
-    expect(calls.length).toBeGreaterThan(1);
+    expect(calls.length).toBe(fixture.unixBootstrap.installerInvocations);
+    expect(shellFunction(source, "ensure_native_toolchain")).toContain('done < <(linux_toolchain_packages "$manager")');
     for (const call of calls) expect({ call: call.trim(), guarded: call.includes("|| log ") }).toEqual({ call: call.trim(), guarded: true });
     if (process.platform === "win32") return;

### 105 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️tool-configuration-ownership/🔣️.json
HEAD 367 BASE 368 STASH 361 CURRENT H
--- 

+++ 

@@ -256,11 +256,4 @@

       "tokens": [
         "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"
-      ]
-    },
-    {
-      "path": ".vscode/settings.json",
-      "tokens": [
-        "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/📐️source-policy/🟨️.mjs",
-        "🧰️framework/🔨️modules/🖱️ui/🎨️styling/💨️tailwind/🟦️.ts"
       ]
     },

### 106 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json
HEAD 445 BASE 358 STASH 356 CURRENT H
--- 

+++ 

@@ -327,7 +327,5 @@

   "selector": {
     "libraryDeclaration": "export async function runVitest(bundleRoot: string, segments: string[], config: string): Promise<void>",
-    "relativeOwnerSuffix": "🧪️tests/🎚️config/🟦️.ts",
-    "editorSetting": "vitest.configSearchPatternInclude",
-    "editorPattern": "**/🧪️tests/🎚️config/🟦️.ts"
+    "relativeOwnerSuffix": "🧪️tests/🎚️config/🟦️.ts"
   },
   "directoryContexts": [

### 107 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧱️dependency-direction/🔣️.json
HEAD 5209 BASE 5209 STASH 5209 CURRENT H
--- 

+++ 

@@ -3868,6 +3868,6 @@

     {
       "id": "actual-quiz-react-.-🎨️.css",
-      "owner": "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript",
-      "name": "@semio-tech/quiz-react",
+      "owner": "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript",
+      "name": "@semio-tech/course-react",
       "subpath": "./🎨️.css",
       "target": "./🎨️.css",

### 108 🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🐹️go/🐹️.go
HEAD 557 BASE 3672 STASH 3515 CURRENT H
--- 

+++ 

@@ -23,7 +23,5 @@

 	exec "os/exec"
 	filepath "path/filepath"
-	reflect "reflect"
 	regexp "regexp"
-	slices "slices"
 	sort "sort"
 	strings "strings"
@@ -349,24 +347,9 @@

 		ID:          "system",
 		Name:        "System",
-		Description: "Validates system configuration files like devcontainer and editor settings",
-		Scopes:      []string{".vscode/settings.json", ".vscode/extensions.json", ".devcontainer/devcontainer.json", ".devcontainer/editor-overlay.json"},
+		Description: "Enforces repository dashboard authority over developer configuration",
+		Scopes:      []string{"**/*"},
 		Priority:    model.BreachPriorityHigh,
-		Groups: []model.Territory{
-			{
-				Name:        "Devcontainer",
-				Description: "Devcontainer configuration breachs",
-				Groups: []model.Territory{
-					{
-						Name:        "VSCode",
-						Description: "The devcontainer editor block must equal its derivation from the host editor files",
-						Kinds: []model.Statute{
-							model.BreachSystemDevcontainerVscodeSettingsDrift,
-							model.BreachSystemDevcontainerVscodeExtensionsDrift,
-						},
-					},
-				},
-			},
-		},
-		Run: SystemPolicy,
+		Groups:      []model.Territory{{Name: "Developer Configuration", Description: "Editor-owned configuration is forbidden", Kinds: []model.Statute{model.BreachSystemIdeConfiguration}}},
+		Run:         SystemPolicy,
 	},
 	{
@@ -2307,159 +2290,19 @@

 }
 
-// 📝️EditorSurface is the editor state of one host: workspace settings and recommended extensions.
-type EditorSurface struct {
-	Settings   map[string]interface{}
-	Extensions []string
-}
-
-// 🧩️EditorOverlay declares what the devcontainer does not inherit from the host editor files and what only it adds (`.devcontainer/editor-overlay.json`).
-type EditorOverlay struct {
-	Settings struct {
-		Omit []string               `json:"omit"`
-		Set  map[string]interface{} `json:"set"`
-	} `json:"settings"`
-	Extensions struct {
-		Omit []string `json:"omit"`
-		Add  []string `json:"add"`
-	} `json:"extensions"`
-}
-
-// 🧮️DeriveContainerEditor derives the devcontainer editor surface from the host canonical surface and the declared overlay:
-// the host entries minus the omitted ones, `${workspaceFolder}` rewritten to `${containerWorkspaceFolder}`, then the overlay's own settings and extensions.
-func DeriveContainerEditor(canonical EditorSurface, overlay EditorOverlay) EditorSurface {
-	settings := map[string]interface{}{}
-	encoded, err := json.Marshal(canonical.Settings)
-	if err == nil {
-		_ = json.Unmarshal([]byte(strings.ReplaceAll(string(encoded), "${workspaceFolder}", "${containerWorkspaceFolder}")), &settings)
-	}
-	for _, key := range overlay.Settings.Omit {
-		delete(settings, key)
-	}
-	for key, value := range overlay.Settings.Set {
-		settings[key] = value
-	}
-	var extensions []string
-	seen := map[string]struct{}{}
-	for _, id := range append(slices.DeleteFunc(slices.Clone(canonical.Extensions), func(id string) bool { return slices.Contains(overlay.Extensions.Omit, id) }), overlay.Extensions.Add...) {
-		if _, ok := seen[id]; !ok {
-			seen[id] = struct{}{}
-			extensions = append(extensions, id)
-		}
-	}
-	return EditorSurface{Settings: settings, Extensions: extensions}
-}
-
-// 📦️stripJSONComments removes // and /* */ comments outside strings so JSONC files parse as JSON.
-func stripJSONComments(data []byte) []byte {
-	out := make([]byte, 0, len(data))
-	for index := 0; index < len(data); index++ {
-		switch {
-		case data[index] == '"':
-			start := index
-			for index++; index < len(data) && data[index] != '"'; index++ {
-				if data[index] == '\\' {
-					index++
-				}
-			}
-			out = append(out, data[start:min(index+1, len(data))]...)
-		case data[index] == '/' && index+1 < len(data) && data[index+1] == '/':
-			for index < len(data) && data[index] != '\n' {
-				index++
-			}
-			out = append(out, '\n')
-		case data[index] == '/' && index+1 < len(data) && data[index+1] == '*':
-			end := strings.Index(string(data[index+2:]), "*/")
-			if end < 0 {
-				return out
-			}
-			index += end + 3
-		default:
-			out = append(out, data[index])
-		}
-	}
-	return out
-}
-
-// 📦️readJSONC parses a JSONC file into `out`.
-func readJSONC(path string, out interface{}) bool {
-	data, err := os.ReadFile(path)
-	if err != nil {
-		return false
-	}
-	return json.Unmarshal(stripJSONComments(data), out) == nil
-}
-
-// 📖️ReadHostEditor reads the host canonical editor surface (`.vscode/settings.json`, `.vscode/extensions.json`).
-func ReadHostEditor(rootDir string) (EditorSurface, bool) {
-	var surface EditorSurface
-	var extensions struct {
-		Recommendations []interface{} `json:"recommendations"`
-	}
-	if !readJSONC(filepath.Join(rootDir, ".vscode", "settings.json"), &surface.Settings) || !readJSONC(filepath.Join(rootDir, ".vscode", "extensions.json"), &extensions) {
-		return surface, false
-	}
-	surface.Extensions = extensionIDsFromJSONList(extensions.Recommendations)
-	return surface, true
-}
-
-// 📖️ReadContainerEditor reads the editor surface checked into `.devcontainer/devcontainer.json`.
-func ReadContainerEditor(rootDir string) (EditorSurface, bool) {
-	var devcontainer struct {
-		Customizations struct {
-			Vscode struct {
-				Settings   map[string]interface{} `json:"settings"`
-				Extensions []interface{}          `json:"extensions"`
-			} `json:"vscode"`
-		} `json:"customizations"`
-	}
-	if !readJSONC(filepath.Join(rootDir, ".devcontainer", "devcontainer.json"), &devcontainer) {
-		return EditorSurface{}, false
-	}
-	return EditorSurface{Settings: devcontainer.Customizations.Vscode.Settings, Extensions: extensionIDsFromJSONList(devcontainer.Customizations.Vscode.Extensions)}, true
-}
-
-// 📖️ReadEditorOverlay reads `.devcontainer/editor-overlay.json`; a missing file declares no overlay.
-func ReadEditorOverlay(rootDir string) EditorOverlay {
-	var overlay EditorOverlay
-	readJSONC(filepath.Join(rootDir, ".devcontainer", "editor-overlay.json"), &overlay)
-	return overlay
-}
-
-// 📦️extensionIDsFromJSONList normalizes a JSON string list of extension ids.
-func extensionIDsFromJSONList(values []interface{}) []string {
-	if len(values) == 0 {
-		return nil
-	}
-	ids := make([]string, 0, len(values))
-	for _, value := range values {
-		id, ok := value.(string)
-		if !ok || id == "" {
-			continue
-		}
-		ids = append(ids, id)
-	}
-	return ids
-}
-
-// ⚖️SystemPolicy requires the devcontainer editor block to equal its derivation from the host editor files and the declared overlay.
+// ⚖️ SystemPolicy rejects editor-owned configuration in the active repository.
 func SystemPolicy(ctx *PolicyContext) []model.Breach {
+	paths := ctx.filesOverride
+	if paths == nil {
+		var err error
+		paths, err = workspace.IdeConfigurationPaths(ctx.RootDir, ctx.Scope.FilePath)
+		if err != nil {
+			return []model.Breach{ctx.CreateBreach(err.Error(), model.BreachSystemIdeConfiguration, ctx.Scope.FilePath, 1, 0, "")}
+		}
+	}
 	var breachs []model.Breach
-	canonical, hasCanonical := ReadHostEditor(ctx.RootDir)
-	actual, hasContainer := ReadContainerEditor(ctx.RootDir)
-	if !hasCanonical || !hasContainer {
-		return ctx.FilterIgnored(breachs)
-	}
-	expected := DeriveContainerEditor(canonical, ReadEditorOverlay(ctx.RootDir))
-	if !reflect.DeepEqual(expected.Settings, actual.Settings) {
-		breachs = append(breachs, ctx.CreateBreach(
-			".devcontainer/devcontainer.json customizations.vscode.settings must equal .vscode/settings.json plus .devcontainer/editor-overlay.json",
-			model.BreachSystemDevcontainerVscodeSettingsDrift,
-			".devcontainer/devcontainer.json", 1, 0, ""))
-	}
-	if !slices.Equal(expected.Extensions, actual.Extensions) {
-		breachs = append(breachs, ctx.CreateBreach(
-			".devcontainer/devcontainer.json customizations.vscode.extensions must equal .vscode/extensions.json plus .devcontainer/editor-overlay.json",
-			model.BreachSystemDevcontainerVscodeExtensionsDrift,
-			".devcontainer/devcontainer.json", 1, 0, ""))
+	for _, path := range paths {
+		if workspace.IsIdeConfigurationPath(path) {
+			breachs = append(breachs, ctx.CreateBreach("Developer commands and configuration belong to the repository dashboard", model.BreachSystemIdeConfiguration, path, 1, 0, ""))
+		}
 	}
 	return ctx.FilterIgnored(breachs)

### 109 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/🔬️_test.go
HEAD 0 BASE 2016 STASH 1921 CURRENT H
--- 

+++ 

@@ -9,5 +9,4 @@

 	exec "os/exec"
 	filepath "path/filepath"
-	reflect "reflect"
 	runtime "runtime"
 	strings "strings"
@@ -1109,130 +1108,36 @@

 
 func TestSystemPolicy(t *testing.T) {
-	const settings = `{"editor.fontSize": 14, "python.testing.cwd": "${workspaceFolder}", "terminal.integrated.env.windows": {"A": "b"}}`
-	const extensions = `{"recommendations": ["ms-python.python", "ms-vscode-remote.remote-containers"]}`
-	const overlay = `{"settings": {"omit": ["terminal.integrated.env.windows"], "set": {"python.defaultInterpreterPath": "${containerWorkspaceFolder}/.venv/bin/python"}}, "extensions": {"omit": ["ms-vscode-remote.remote-containers"], "add": ["anthropic.claude-code"]}}`
-	const devcontainer = "// SPDX-License-Identifier: AGPL-3.0-only\n{\n  /* derived */\n  \"name\": \"https://example.test/a\",\n  \"customizations\": {\"vscode\": {\"settings\": {\"editor.fontSize\": 14, \"python.testing.cwd\": \"${containerWorkspaceFolder}\", \"python.defaultInterpreterPath\": \"${containerWorkspaceFolder}/.venv/bin/python\"}, \"extensions\": [\"ms-python.python\", \"anthropic.claude-code\"]}}\n}\n"
-	arrange := func(t *testing.T, files map[string]string) {
-		tmpDir := t.TempDir()
-		oldRoot := workspace.RootDir
-		workspace.RootDir = tmpDir
-		t.Cleanup(func() { workspace.RootDir = oldRoot })
-		for name, content := range files {
-			os.MkdirAll(filepath.Dir(filepath.Join(tmpDir, name)), 0o755)
-			workspace.WriteTextFile(filepath.Join(tmpDir, name), content)
-		}
-	}
-	kinds := func() map[model.Statute]bool {
-		found := map[model.Statute]bool{}
-		for _, v := range statutes.SystemPolicy(statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})) {
-			found[v.Kind] = true
-		}
-		return found
-	}
-	files := func(overrides map[string]string) map[string]string {
-		base := map[string]string{".vscode/settings.json": settings, ".vscode/extensions.json": extensions, ".devcontainer/editor-overlay.json": overlay, ".devcontainer/devcontainer.json": devcontainer}
-		for name, content := range overrides {
-			base[name] = content
-		}
-		return base
-	}
-	t.Run("no breach when the devcontainer block equals its derivation", func(t *testing.T) {
-		arrange(t, files(nil))
-		if found := kinds(); len(found) != 0 {
-			t.Errorf("expected no breach, got %v", found)
-		}
-	})
-	t.Run("detects drifted settings", func(t *testing.T) {
-		arrange(t, files(map[string]string{".vscode/settings.json": `{"editor.fontSize": 15}`}))
-		found := kinds()
-		if !found[model.BreachSystemDevcontainerVscodeSettingsDrift] || found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected only the settings drift breach, got %v", found)
-		}
-	})
-	t.Run("detects drifted extensions", func(t *testing.T) {
-		arrange(t, files(map[string]string{".vscode/extensions.json": `{"recommendations": ["ms-python.python", "golang.go"]}`}))
-		found := kinds()
-		if found[model.BreachSystemDevcontainerVscodeSettingsDrift] || !found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected only the extensions drift breach, got %v", found)
-		}
-	})
-	t.Run("a missing overlay declares no overlay", func(t *testing.T) {
-		arrange(t, files(map[string]string{".devcontainer/editor-overlay.json": ""}))
-		found := kinds()
-		if !found[model.BreachSystemDevcontainerVscodeSettingsDrift] || !found[model.BreachSystemDevcontainerVscodeExtensionsDrift] {
-			t.Errorf("expected both drift breaches without overlay, got %v", found)
-		}
-	})
-	t.Run("no breachs when the editor files are absent", func(t *testing.T) {
-		arrange(t, nil)
-		if found := kinds(); len(found) != 0 {
-			t.Errorf("expected 0 breachs, got %v", found)
-		}
-	})
-	t.Run("derivation equals the language-agnostic fixture", func(t *testing.T) {
-		data, err := os.ReadFile("../../../📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/🧫️fixtures/📝️derived-config/🔣️.json")
-		if err != nil {
-			t.Fatal(err)
-		}
-		var fixture struct {
-			Editor []struct {
-				Name      string
-				Canonical statutes.EditorSurface
-				Overlay   statutes.EditorOverlay
-				Expected  statutes.EditorSurface
+	data, err := os.ReadFile("../../../🎛️dashboard/🧫️fixtures/🧭️authority/🔣️.json")
+	if err != nil {
+		t.Fatal(err)
+	}
+	var fixture struct {
+		PathCases []struct {
+			Path     string
+			Accepted bool
+		}
+	}
+	if err := json.Unmarshal(data, &fixture); err != nil {
+		t.Fatal(err)
+	}
+	for _, row := range fixture.PathCases {
+		t.Run(row.Path, func(t *testing.T) {
+			ctx := statutes.NewPolicyContextWithFiles(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{}, []string{row.Path})
+			breachs := statutes.SystemPolicy(ctx)
+			if (len(breachs) == 0) != row.Accepted {
+				t.Fatalf("accepted = %v, breachs = %v", row.Accepted, breachs)
 			}
-		}
-		if err := json.Unmarshal(data, &fixture); err != nil {
-			t.Fatal(err)
-		}
-		if len(fixture.Editor) == 0 {
-			t.Fatal("fixture holds no editor cases")
-		}
-		for _, row := range fixture.Editor {
-			derived := statutes.DeriveContainerEditor(row.Canonical, row.Overlay)
-			if !reflect.DeepEqual(derived.Settings, row.Expected.Settings) || !reflect.DeepEqual(derived.Extensions, row.Expected.Extensions) {
-				t.Errorf("%s: derived %v, expected %v", row.Name, derived, row.Expected)
+			if len(breachs) > 0 && breachs[0].Kind != model.BreachSystemIdeConfiguration {
+				t.Fatalf("unexpected statute %v", breachs[0].Kind)
 			}
-		}
-	})
-	t.Run("the checked-in devcontainer block equals its derivation", func(t *testing.T) {
-		root, err := filepath.Abs("../../../../../../..")
-		if err != nil {
-			t.Fatal(err)
-		}
-		if _, err := os.Stat(filepath.Join(root, ".devcontainer", "editor-overlay.json")); err != nil {
-			t.Skipf("not running inside the repository checkout: %v", err)
-		}
-		oldRoot := workspace.RootDir
-		workspace.RootDir = root
-		t.Cleanup(func() { workspace.RootDir = oldRoot })
-		if found := kinds(); len(found) != 0 {
-			t.Errorf("run bun repo run workspace:generate-config; the checked-in block drifts: %v", found)
-		}
-	})
-	t.Run("policy registered with correct id", func(t *testing.T) {
-		p, found := statutes.FindPolicy("system")
-		if !found {
-			t.Fatal("expected system policy to be registered")
-		}
-		if p.Name != "System" {
-			t.Errorf("expected name System, got %s", p.Name)
-		}
-		kinds := p.AllKinds()
-		if len(kinds) != 2 || kinds[0] != model.BreachSystemDevcontainerVscodeSettingsDrift || kinds[1] != model.BreachSystemDevcontainerVscodeExtensionsDrift {
-			t.Fatalf("expected the settings and extensions drift statutes, got %v", kinds)
-		}
-	})
-	t.Run("statute meta is correct", func(t *testing.T) {
-		for _, kind := range []model.Statute{model.BreachSystemDevcontainerVscodeSettingsDrift, model.BreachSystemDevcontainerVscodeExtensionsDrift} {
-			meta := kind.Info()
-			if meta.Autofixable {
-				t.Errorf("%s: a derived copy is rewritten by generate-config, never autofixed", kind)
-			}
-			if meta.Priority != model.BreachPriorityHigh {
-				t.Errorf("%s: expected high priority", kind)
-			}
-		}
-	})
+		})
+	}
+	policy, found := statutes.FindPolicy("system")
+	if !found || len(policy.AllKinds()) != 1 || policy.AllKinds()[0] != model.BreachSystemIdeConfiguration {
+		t.Fatalf("unexpected system policy %v", policy)
+	}
+	if model.BreachSystemIdeConfiguration.Info().Autofixable || model.BreachSystemIdeConfiguration.Info().Priority != model.BreachPriorityHigh {
+		t.Fatal("IDE configuration removal must be explicit and high priority")
+	}
 }
 

### 110 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md
HEAD 205 BASE 205 STASH 208 CURRENT H
--- 

+++ 

@@ -37,24 +37,27 @@

 
 ```bash
-bun ./📜️script.ts discover                 # every case Nx will generate a project for
-bun ./📜️script.ts doctor                   # toolchains; a missing one fails setup, never skips
-bun ./📜️script.ts contract                 # everything provable without executing a test
-bun ./📜️script.ts oracle   <level>         # the reference implementation only
-bun ./📜️script.ts subject  <level>         # this repository's implementations only
-bun ./📜️script.ts parity   <level>         # oracle + subjects + semantic comparison
-bun ./📜️script.ts run      <level>         # contract, then parity
-bun ./📜️script.ts report                   # re-render JUnit from the last run
-bun ./📜️script.ts clean [--dry] [--stale]  # marked generated test state, nothing else
-bun ./📜️script.ts dependency               # oracle purity + production reachability
-bun ./📜️script.ts inventory                # runs the owner's production bridge; runtime = manifest = tests
-bun ./📜️script.ts fixture  <subcommand>    # generate | reproduce | verify | audit — provenance, not ad hoc files
-bun ./📜️script.ts probe    [--json]        # lists external measurement tools and their qualification status
-bun ./📜️script.ts matrix   [--enforce]     # the 16-dimension coverage report; --enforce applies the 6 release gates
-bun ./📜️script.ts gc       [--apply]       # mark-and-sweep over the fixture CAS and run directories; dry by default
+bun run dashboard run @semio-tech/repo-test-domain:test-discover                 # every case Nx will generate a project for
+bun run dashboard run @semio-tech/repo-test-domain:test-doctor                   # toolchains; a missing one fails setup, never skips
+bun run dashboard run @semio-tech/repo-test-domain:test-contract                 # everything provable without executing a test
+bun run dashboard run @semio-tech/repo-test-domain:test-oracle -- <level>         # the reference implementation only
+bun run dashboard run @semio-tech/repo-test-domain:test-subject -- <level>         # this repository's implementations only
+bun run dashboard run @semio-tech/repo-test-domain:test-parity -- <level>         # oracle + subjects + semantic comparison
+bun run dashboard run @semio-tech/repo-test-domain:test -- <level>         # contract, then parity
+bun run dashboard run @semio-tech/repo-test-domain:test-report                   # re-render JUnit from the last run
+bun run dashboard run @semio-tech/repo-test-domain:test-clean -- [--dry] [--stale]  # marked generated test state, nothing else
+bun run dashboard run @semio-tech/repo-test-domain:test-dependency               # oracle purity + production reachability
+bun run dashboard run @semio-tech/repo-test-domain:test-inventory                # runs the owner's production bridge; runtime = manifest = tests
+bun run dashboard run @semio-tech/repo-test-domain:test-fixture-generate
+bun run dashboard run @semio-tech/repo-test-domain:test-fixture-reproduce
+bun run dashboard run @semio-tech/repo-test-domain:test-fixture-verify
+bun run dashboard run @semio-tech/repo-test-domain:test-fixture-audit
+bun run dashboard run @semio-tech/repo-test-domain:test-probe -- [--json]        # lists external measurement tools and their qualification status
+bun run dashboard run @semio-tech/repo-test-domain:test-matrix -- [--enforce]     # the 16-dimension coverage report; --enforce applies the 6 release gates
+bun run dashboard run @semio-tech/repo-test-domain:test-gc -- [--apply]       # mark-and-sweep over the fixture CAS and run directories; dry by default
 ```
 
 `--owner <path>`, `--owner-root <path>`, `--case <slug>` and `--project <name>` narrow case-based phases;
 `--implementation <id>` narrows their subject implementations. `discover` lists all cases.
-From the repository root the same phases are reachable as `bun ./📜️script.ts test <phase>`, and every
+Choose **New Task** (`Ctrl+B n`), search `repo test` plus the phase and select its registered target. Extra selectors go in **Arguments**, results appear in the task output and the task menu controls cancellation. Every
 test owner has one generated Nx project: `lint` runs its contract phase and `test` runs its cases
 (`cases <owner> [level] [--facet <phase>] [--case <slug>]`), the level and selection passed as arguments.
@@ -101,7 +104,7 @@

 back to the whole artifact. The owner's `🔮️oracles/🔣️.json` carries a `MutationManifest` naming, per
 mutation, the production dispatch variant it corresponds to and the outcome classes
-(`applied|no-op|empty|disjoint|rejected`) it can reach. `bun ./📜️script.ts inventory` runs the owner's
+(`applied|no-op|empty|disjoint|rejected`) it can reach. `bun run dashboard run @semio-tech/repo-test-domain:test-inventory` runs the owner's
 **production mutation bridge** (`🏭️bridge/📜️script.ts`, discovered by walking up from the owner) and
-`bun ./📜️script.ts contract` then requires runtime dispatch, the manifest and the claimed test catalog
+`bun run dashboard run @semio-tech/repo-test-domain:test-contract` then requires runtime dispatch, the manifest and the claimed test catalog
 to agree **exactly** — a mutation reachable in production and missing from the manifest, or declared in
 the manifest with no dispatch behind it, is a blocking breach, not a coverage footnote.