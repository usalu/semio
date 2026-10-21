"""Builds plan.json for the repo-launcher set from the live tree: new files come from staged/, edits are anchored replacements, rewrites carry the content hash they replace, deletes carry the hash they remove. Usage: make-plan.py [--root DIR]"""
import hashlib, json, os, re, sys
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.getcwd()
here = os.path.dirname(os.path.abspath(__file__))
sha = lambda data: hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
L0 = M + "/🚀️launcher"
CLI = M + "/⌨️cli"
GO = CLI + "/📦️packages/🐹️go"
plan = {"version": 1, "groups": {}}
def group(name): return plan["groups"].setdefault(name, {"creates": [], "edits": [], "rewrites": [], "deletes": []})
def live(path): return open(os.path.join(root, path), encoding="utf-8", newline="").read()
def edit(g, path, old, new, count=1):
    found = live(path).count(old)
    if found != count: sys.exit(f"anchor count {found} != {count} in {path}: {old[:80]!r}")
    group(g)["edits"].append({"path": path, "old": old, "new": new, "count": count})
def span(g, path, start, end, new):
    text = live(path); a = text.index(start); b = text.index(end, a) + len(end)
    edit(g, path, text[a:b], new)
def sweep(g, path, pattern, repl, count):
    found = len(re.findall(pattern, live(path)))
    if found != count: sys.exit(f"sweep count {found} != {count} in {path}: {pattern}")
    group(g)["edits"].append({"path": path, "pattern": pattern, "repl": repl, "count": count})
def rewrite(g, path, content):
    group(g)["rewrites"].append({"path": path, "sha256": sha(live(path)), "afterSha256": sha(content), "content": content})
def delete(g, path):
    group(g)["deletes"].append({"path": path, "sha256": sha(live(path))})
# --- creates: everything in the authored tree, packed into digest-named blobs (pack.py)
import subprocess
subprocess.run([sys.executable, os.path.join(here, "pack.py")], check=True, stdout=subprocess.DEVNULL)
for row in json.load(open(os.path.join(here, "manifest.json"), encoding="utf-8"))["files"]:
    rel = row["path"]; data = open(os.path.join(here, "blobs", row["sha256"]), "rb").read()
    if os.path.exists(os.path.join(root, rel)):
        if open(os.path.join(root, rel), "rb").read() == data: continue
        sys.exit(f"staged file collides with an existing different file: {rel}")
    group("bootstrap" if rel.startswith(".github/") else "launcher")["creates"].append({"path": rel, "sha256": row["sha256"], "mode": row["mode"]})
# --- group launcher: workspace + cli integration
edit("launcher", "go.work", f"\t./{M}/🚚️move/📦️packages/🐹️go\n", f"\t./{M}/🚚️move/📦️packages/🐹️go\n\t./{L0}/📦️packages/🐹️go\n\t./{L0}/📦️packages/🐹️go/📦️main\n")
edit("launcher", GO + "/go.mod", "\tgithub.com/usalu/semio/repo/languages v0.0.0\n", "\tgithub.com/usalu/semio/repo/languages v0.0.0\n\tgithub.com/usalu/semio/repo/launcher v0.0.0\n")
edit("launcher", GO + "/go.mod", "replace github.com/usalu/semio/repo/languages => ../../../🗣️languages/📦️packages/🐹️go\n", "replace github.com/usalu/semio/repo/languages => ../../../🗣️languages/📦️packages/🐹️go\n\nreplace github.com/usalu/semio/repo/launcher => ../../../🚀️launcher/📦️packages/🐹️go\n")
cli = GO + "/🐹️.go"
span("launcher", cli, "// 🚪️NativeEntrypointPlan selects", "return tool.Run(workspace.FindRepoRoot(cwd), arguments)\n}\n", '''// 🚪️NativeEntrypointPlan selects the owned process port without parsing its arguments.
type NativeEntrypointPlan = launcher.Plan

// 🧭️PlanNativeEntrypoint preserves opaque arguments at the native process boundary; the launcher module owns the one planner.
func PlanNativeEntrypoint(argv []string) NativeEntrypointPlan { return launcher.PlanRoute(argv) }

// 🛠️NativeScriptTool is the tool-independent owned build process port.
type NativeScriptTool interface { Run(root string, arguments []string) error }

type nativeLauncherTool struct{ entry string }

func (tool nativeLauncherTool) Run(root string, arguments []string) error {
 ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
 code := launcher.DefaultHost().Execute(ctx, root, tool.entry, arguments)
 stop()
 if code != 0 { return workspace.ExitError{Code: code} }
 return nil
}

// 🏗️RunNativeScript selects the issued build invocation without rewriting its environment.
func RunNativeScript(arguments []string, tool NativeScriptTool) error {
 cwd, err := os.Getwd(); if err != nil { return err }
 return tool.Run(workspace.FindRepoRoot(cwd), arguments)
}
''')
edit("launcher", M + "/💻️client/🔌️mcp/go.mod", "\tgithub.com/usalu/semio/repo/cli v0.0.0\n", "\tgithub.com/usalu/semio/repo/cli v0.0.0\n\tgithub.com/usalu/semio/repo/launcher v0.0.0\n")
edit("launcher", M + "/💻️client/🔌️mcp/go.mod", "replace github.com/usalu/semio/repo/cli => ../../⌨️cli/📦️packages/🐹️go\n", "replace github.com/usalu/semio/repo/cli => ../../⌨️cli/📦️packages/🐹️go\n\nreplace github.com/usalu/semio/repo/launcher => ../../🚀️launcher/📦️packages/🐹️go\n")
edit("launcher", cli, ' if plan.Kind == "build" { return RunNativeScript(plan.Arguments, nativeBunBuildTool{}) }\n if plan.Kind == "workspace" { return RunNativeScript(plan.Arguments, nativeRootScriptTool{}) }\n',
 ' if len(os.Args) > 1 && (os.Args[1] == "--version" || os.Args[1] == "-V") { if code := launcher.DefaultHost().Version(os.Args[1:]); code != 0 { return workspace.ExitError{Code: code} }; return nil }\n if plan.Kind == "build" || plan.Kind == "workspace" || plan.Kind == "script" { return RunNativeScript(plan.Arguments, nativeLauncherTool{plan.Kind}) }\n')
edit("launcher", cli, '\t"os"\n "path/filepath"\n\t"time"\n', '\t"os"\n\t"os/signal"\n\t"syscall"\n\t"time"\n')
edit("launcher", cli, '\tgraphqlpkg "github.com/usalu/semio/repo/graphql"\n', '\tgraphqlpkg "github.com/usalu/semio/repo/graphql"\n\tlauncher "github.com/usalu/semio/repo/launcher"\n')
edit("launcher", CLI + "/🧪️tests/🛡️hermetic-root-guard/🐹️.go", 'os.Exit(sandbox.Run("cli", nil, m.Run))', 'os.Exit(sandbox.Run("cli", nil, func() int { launcher.BindRoot(sandbox.Origin()); return m.Run() }))')
edit("launcher", CLI + "/🧪️tests/🛡️hermetic-root-guard/🐹️.go", '\tsandbox "github.com/usalu/semio/repo/workspace/sandbox"\n', '\tlauncher "github.com/usalu/semio/repo/launcher"\n\tsandbox "github.com/usalu/semio/repo/workspace/sandbox"\n')
MCPG = M + "/💻️client/🔌️mcp/🧪️tests/🛡️hermetic-root-guard/🐹️.go"
edit("launcher", MCPG, 'os.Exit(sandbox.Run("mcp", nil, m.Run))', 'os.Exit(sandbox.Run("mcp", nil, func() int { launcher.BindRoot(sandbox.Origin()); return m.Run() }))')
edit("launcher", MCPG, '\tsandbox "github.com/usalu/semio/repo/workspace/sandbox"\n', '\tlauncher "github.com/usalu/semio/repo/launcher"\n\tsandbox "github.com/usalu/semio/repo/workspace/sandbox"\n')
UNIT = CLI + "/🧪️tests/🔬️unit/🐹️.go"
span("launcher", UNIT, " root := workspace.FindRepoRoot(", '🔣️.json")\n', ' raw, err := os.ReadFile(sandbox.OriginPath("..", "..", "🧫️fixtures", "🚪️entrypoint", "🔣️.json"))\n')
edit("launcher", UNIT, '\tworkspace "github.com/usalu/semio/repo/workspace"\n', '\tworkspace "github.com/usalu/semio/repo/workspace"\n\tsandbox "github.com/usalu/semio/repo/workspace/sandbox"\n')
delete("launcher", GO + "/🚪️entrypoint_unix.go")
delete("launcher", GO + "/🚪️entrypoint_windows.go")
edit("launcher", CLI + "/README.md", "build verbs enter the owned build implementation through its tool-independent process port.", "build verbs enter the owned build implementation through its tool-independent process port, which is the `🚀️launcher` module: it provisions the pinned JavaScript runtime from its registry, so no runtime has to be installed, and `repo --version` names the launcher, the runtime and the implementation.")
# entrypoint contract: schema, fixture, oracle test, feature
fx_path = CLI + "/🧫️fixtures/🚪️entrypoint/🔣️.json"; sc_path = CLI + "/🧬️schema/🚪️entrypoint/🔣️.json"
fx = json.loads(live(fx_path)); sc = json.loads(live(sc_path))
roundtrip = lambda path, obj: json.dumps(obj, ensure_ascii=False, indent=2) + "\n" == live(path)
fx["scriptExtensions"] = [".ts", ".mjs", ".js"]
fx["scriptOptions"] = ["watch", "cwd"]
def vec(i, argv, kind="script", arguments=None): return {"id": i, "argv": argv, "kind": kind, "arguments": argv if arguments is None else arguments}
fx["vectors"] += [vec("script-plain", ["./📜️script.ts", "build"]), vec("script-watch", ["--watch", "./x.ts", "a"]), vec("script-cwd-equals", ["--cwd=sub", "x.mjs"]), vec("script-cwd-separate", ["--cwd", "sub dir", "--watch", "x.js", "--", "z"]), vec("script-after-tool-is-build", ["--tool=nx", "x.ts"], "build"), vec("script-argument-is-not-a-script", ["ticket", "x.ts"], "domain"), vec("script-option-without-script", ["--watch", "run"], "domain")]
sc["required"] = sorted(set(sc["required"]) | {"scriptExtensions", "scriptOptions"}, key=sc["required"].index) if False else sc["required"] + ["scriptExtensions", "scriptOptions"]
sc["properties"]["scriptExtensions"] = {"const": [".ts", ".mjs", ".js"]}
sc["properties"]["scriptOptions"] = {"const": ["watch", "cwd"]}
sc["properties"]["vectors"]["items"]["properties"]["kind"]["enum"] = ["domain", "build", "mcp", "workspace", "script"]
if not roundtrip(fx_path, json.loads(live(fx_path))) or not roundtrip(sc_path, json.loads(live(sc_path))): sys.exit("entrypoint JSON does not round-trip; switch to anchored edits")
rewrite("launcher", fx_path, json.dumps(fx, ensure_ascii=False, indent=2) + "\n")
rewrite("launcher", sc_path, json.dumps(sc, ensure_ascii=False, indent=2) + "\n")
oracle = CLI + "/🧪️tests/🚪️entrypoint/🟦️.ts"
rewrite("launcher", oracle, '''import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🚪️entrypoint/🔣️.json",import.meta.url),"utf8")),fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚪️entrypoint/🔣️.json",import.meta.url),"utf8"));
const scriptArgument=(argv:string[]):boolean=>{let index=0;while(index<argv.length){if(argv[index]==="--watch"||argv[index]!.startsWith("--cwd="))index++;else if(argv[index]==="--cwd")index+=2;else break;}return index<argv.length&&(fixture.scriptExtensions as string[]).some(extension=>argv[index]!.endsWith(extension));};
test("native entrypoint neutral routes agree with the independent SQLite oracle",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const db=new Database(":memory:");
 try{db.exec("CREATE TABLE verbs(name TEXT PRIMARY KEY)");for(const name of fixture.buildVerbs)db.query("INSERT INTO verbs VALUES (?)").run(name);
 for(const row of fixture.vectors){const selected=db.query(`WITH a AS (SELECT key i,value FROM json_each(?)), head AS (SELECT i,value FROM a WHERE value NOT LIKE '--tool=%' AND value<>'--tool' AND NOT EXISTS (SELECT 1 FROM a prev WHERE prev.i=a.i-1 AND prev.value='--tool') ORDER BY i LIMIT 1) SELECT CASE WHEN json_extract(?1,'$[0]')='dev' AND json_extract(?1,'$[1]')='mcp' AND json_extract(?1,'$[2]')='stdio' AND json_extract(?1,'$[3]')='os' THEN 'workspace' WHEN json_extract(?1,'$[0]')='--tool' OR json_extract(?1,'$[0]') LIKE '--tool=%' THEN 'build' WHEN (SELECT value FROM head)='mcp' AND (SELECT i FROM head)=0 THEN 'mcp' WHEN (SELECT value FROM head)='dashboard' AND (SELECT i FROM head)=0 THEN 'build' WHEN EXISTS (SELECT 1 FROM verbs WHERE name=(SELECT value FROM head)) THEN 'build' ELSE 'domain' END kind`).get(JSON.stringify(row.argv)) as {kind:string};
 const kind=row.argv[0]==="dev"||row.argv[0]==="mcp"||row.argv[0]==="dashboard"||!scriptArgument(row.argv)?selected.kind:"script";expect(kind,row.id).toBe(row.kind);const args=row.argv[0]==="mcp"?row.argv.slice(1):row.argv[0]==="dashboard"?[...fixture.dashboardArguments,...row.argv.slice(1)]:row.argv;expect(args,row.id).toEqual(row.arguments);}
 }finally{db.close();}
});
''')
feat = CLI + "/🧪️tests/🚪️entrypoint/🥒️.feature"
rewrite("launcher", feat, live(feat).rstrip("\n") + '''

  Scenario: A script runs through the provisioned runtime
    Given an argument vector that names a script file, optionally led by the neutral watch and cwd options
    When the launcher plans the route
    Then the script route keeps every argument unchanged
    And the neutral options map to the flags of the selected runtime
    And no runtime has to be installed because the launcher provisions the pinned one
''')
# --- group taxonomy: the Go member names of the launcher module
names = ["🗂️runtime-registry", "🧭️workspace-root", "🗣️language", "📥️runtime-provision", "🏗️entry", "🐚️unix-host", "🪟️windows-host", "🛎️command"]
edit("taxonomy", M + "/📚️library/🔣️taxonomy.json", '        "🌱️fixture-repository"\n      ],', '        "🌱️fixture-repository",\n' + ",\n".join(f'        "{n}"' for n in names) + "\n      ],")
# --- group cargo: the rustc wrapper goes through the launcher
ing = M + "/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress"
edit("cargo", ".cargo/config.toml", 'rustc-wrapper = "bun"', 'rustc-wrapper = "repo"')
edit("cargo", ing + "/🔣️policy.json", '"wrapper":"bun"', '"wrapper":"repo"')
edit("cargo", ing + "/🧬️schema/🔣️.json", '"wrapper":{"const":"bun"}', '"wrapper":{"const":"repo"}')
# --- group rust: one launcher, the Go one
rs = CLI + "/🧭️repo-cli/🚪️entrypoint/🦀️.rs"
text = live(rs)
a = text.index("/// 🛠️ The tool-independent build process boundary."); b = text.index("#[cfg(test)]")
text = text[:a] + "/// 🛠️ Build, workspace and script routes are started by the one canonical launcher, the Go `repo` command (module `🚀️launcher`); this implementation plans the routes and refuses to start them.\npub const LAUNCHER_REFUSAL: &str = \"Build commands are started by the canonical Go repo launcher: repo run repo-mcp:install\\nBuild-Befehle startet der kanonische Go-repo-Launcher: repo run repo-mcp:install\";\n\n" + text[b:]
text = text.replace(' let mut index = 0;\n while index < argv.len() { if argv[index] == "--tool"', ' if script_index(argv).is_some() { return NativeEntrypointPlan { kind: "script", arguments: argv.to_vec() }; }\n let mut index = 0;\n while index < argv.len() { if argv[index] == "--tool"')
text = re.sub(r' let build = index > 0 \|\| argv\.get\(index\)\.is_some_and\(\|value\| matches!\(value\.as_str\(\), [^;]*\);', ' let build = index > 0 || argv.get(index).is_some_and(|value| verbs.iter().any(|verb| verb == value));', text)
text = text.replace("pub fn plan_native_entrypoint(argv: &[String]) -> NativeEntrypointPlan {", "pub fn plan_native_entrypoint(argv: &[String]) -> NativeEntrypointPlan { plan_native_entrypoint_with(&build_verbs(), argv) }\n\n/// 🎯️ Plans with explicit build verbs.\npub fn plan_native_entrypoint_with(verbs: &[String], argv: &[String]) -> NativeEntrypointPlan {")
RUST_VERBS = """/// 🏗️ The build verbs are the operation names of the neutral operations catalogue that the launcher registry names; none outside a repository.
const LAUNCHER_REGISTRY: &str = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json";

fn read_build_verbs(root: &std::path::Path, registry: &std::path::Path) -> Option<Vec<String>> {
 let document: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(registry).ok()?).ok()?;
 let catalogue: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join(document["routing"]["catalogue"].as_str()?)).ok()?).ok()?;
 Some(catalogue["operations"].as_array()?.iter().filter_map(|operation| operation["name"].as_str().map(String::from)).collect())
}

/// 🏗️ Reads the build verbs of the repository around the working directory.
pub fn build_verbs() -> Vec<String> {
 let mut directory = std::env::current_dir().ok();
 while let Some(path) = directory {
  let registry = path.join(LAUNCHER_REGISTRY);
  if registry.is_file() { return read_build_verbs(&path, &registry).unwrap_or_default(); }
  directory = path.parent().map(std::path::Path::to_path_buf);
 }
 Vec::new()
}

/// 📜️ The script file extensions"""
text = text.replace("/// 📜️ The script file extensions", RUST_VERBS, 1)
assert "BunScriptTool" not in text and "script_index(argv)" in text and "verbs.iter()" in text
rewrite("rust", rs, text)
mm = CLI + "/📦️packages/🦀️rust/📦️mcp-main.rs"
edit("rust", mm, '  "workspace" => semio_framework_repo_cli::repo_cli::entrypoint::run_native_workspace(&plan.arguments),\n  "build" => semio_framework_repo_cli::repo_cli::entrypoint::run_native_build(&plan.arguments),\n', '  "workspace" | "build" | "script" => {\n   eprintln!("{}", semio_framework_repo_cli::repo_cli::entrypoint::LAUNCHER_REFUSAL);\n   2\n  },\n')
# --- group bootstrap: shell and PowerShell bootstrap, devcontainer, CI, docs
B = M + "/🔩️native/🥾️bootstrap"
sh = B + "/🐚️.sh"
edit("bootstrap", sh, "# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), Bun, the Node.js\n", "# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), the Node.js\n")
edit("bootstrap", sh, "# then `workspace:setup` (which installs packages). Every downloaded installer is pinned and sha256-verified before it runs:\n# Bun from its tools schema, the rest from `🧬️schema/🔣️.json` next to this script.\n", "# then the `repo` launcher built from the checkout (it provisions the pinned JavaScript runtime itself, so no Bun is installed here)\n# and `workspace:setup` (which installs packages). Every downloaded installer is pinned and sha256-verified before it runs,\n# from `🧬️schema/🔣️.json` next to this script; the runtime pins live in the launcher registry.\n")
edit("bootstrap", sh, "# bootstrap); `bun ./📜️script.ts setup native` routes here once Bun exists.\n", "# bootstrap); `repo setup native` routes here once the launcher exists.\n")
edit("bootstrap", sh, 'BUN_PINS_FILE="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json"\n', 'LAUNCHER_INSTALL="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh"\nLAUNCHER_REGISTRY="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json"\nLAUNCHER_BIN="${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin"\n')
tt = live(sh)
a = tt.index("bun_release_row() {"); b = tt.index("#endregion 🔖️PinnedDownloads")
group("bootstrap")["edits"].append({"path": sh, "old": tt[a:b], "new": "", "count": 1})
edit("bootstrap", sh, "  version=\"$(cd \"$REPO_ROOT\" && bun -e 'process.stdout.write(require(\"./package.json\").engines.node)')\" || return 1\n", "  version=\"$(awk -F'\"' '/\"engines\"/ { engines = 1 } engines && /\"node\"[[:space:]]*:/ { print $4; exit }' \"$REPO_ROOT/package.json\")\"\n  [ -n \"$version\" ] || return 1\n")
tt = live(sh)
a = tt.index("#region 🔖️EnsureBun"); b = tt.index("#endregion 🔖️EnsureBun") + len("#endregion 🔖️EnsureBun")
launcher_region = '''#region 🔖️EnsureLauncher
ensure_launcher() {
  export PATH="$LOCAL_BIN:$PATH"
  log "Building the repo launcher (it provisions the pinned JavaScript runtime itself)…"
  bash "$LAUNCHER_INSTALL" "$LAUNCHER_BIN" || return 1
  export PATH="$LAUNCHER_BIN:$PATH"
  hash -r
  [ -x "$LAUNCHER_BIN/repo" ] || {
    log "The repo launcher was not installed into $LAUNCHER_BIN."
    return 1
  }
}
#endregion 🔖️EnsureLauncher'''
group("bootstrap")["edits"].append({"path": sh, "old": tt[a:b], "new": launcher_region, "count": 1})
edit("bootstrap", sh, '  log "Installing the native repository command…"\n  bun repo run repo-mcp:install || return 1\n', '  log "Installing the native repository command…"\n  repo run repo-mcp:install || return 1\n')
edit("bootstrap", sh, '[ -f "$PINS_FILE" ] && [ -f "$BUN_PINS_FILE" ] || {\n  log "Pinned toolchain manifests are missing; expected $PINS_FILE and $BUN_PINS_FILE."\n', '[ -f "$PINS_FILE" ] && [ -f "$LAUNCHER_REGISTRY" ] && [ -f "$LAUNCHER_INSTALL" ] || {\n  log "Pinned toolchain manifests are missing; expected $PINS_FILE, $LAUNCHER_REGISTRY and $LAUNCHER_INSTALL."\n')
edit("bootstrap", sh, "ensure_native_toolchain\nensure_bun\nensure_node ||", "ensure_native_toolchain\nensure_go || log \"Go provisioning failed; install the Go version pinned in $PINS_FILE from https://go.dev/dl and rerun setup.\"\nensure_launcher || exit 1\nensure_node ||")
edit("bootstrap", sh, "ensure_go || log \"Go provisioning failed; install the Go version pinned in $PINS_FILE from https://go.dev/dl and rerun setup.\"\nensure_dotnet", "ensure_dotnet")
ps = B + "/🔵️.ps1"
edit("bootstrap", ps, "installs Bun, Node.js, Go, uv and rustup from checksum-pinned archives", "installs Node.js, Go, uv and rustup from checksum-pinned archives and builds the `repo` launcher (which provisions the pinned JavaScript runtime itself)")
edit("bootstrap", ps, "# `bun ./📜️script.ts setup native` routes here once Bun exists.", "# `repo setup native` routes here once the launcher exists.")
tt = live(ps)
a = tt.index("function Get-PinnedBunAsset {"); b = tt.index("function Get-InstalledVersion {")
group("bootstrap")["edits"].append({"path": ps, "old": tt[a:b], "new": "", "count": 1})
a = tt.index("    $bun = Get-PinnedBunAsset\n"); b = tt.index("    $node = Get-PinnedAsset -Tool \"node\"")
group("bootstrap")["edits"].append({"path": ps, "old": tt[a:b], "new": "", "count": 1})
edit("bootstrap", ps, '$script:BunPinsFile = Join-Path $repoRoot "🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\⚡️caching\\🚀️bootstrap\\🛠️tools\\🟦️bun\\🧬️schema\\🔣️.json"\n', '$script:LauncherInstall = Join-Path $repoRoot "🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\🚀️launcher\\📥️install\\🔵️.ps1"\n$script:LauncherBin = Join-Path $env:LOCALAPPDATA "semio\\launcher\\bin"\n')
edit("bootstrap", ps, '    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("add", "--global", "@google/gemini-cli", "typescript-language-server", "typescript", "pyright") -WorkingDirectory $repoRoot\n', '')
edit("bootstrap", ps, '    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")\n    $rustupPath', '    $rustupPath')
edit("bootstrap", ps, '    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")\n\n    Stop-RepoPythonProcesses -RepoRoot $repoRoot -Consent:$StopRepoPython\n    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("repo", "run", "repo-mcp:install") -WorkingDirectory $repoRoot\n', '\n    Stop-RepoPythonProcesses -RepoRoot $repoRoot -Consent:$StopRepoPython\n    Invoke-RepoCommand -FilePath (Get-Command "powershell.exe").Source -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $script:LauncherInstall, "-Destination", $script:LauncherBin) -WorkingDirectory $repoRoot\n    Invoke-RepoCommand -FilePath (Join-Path $script:LauncherBin "repo.exe") -ArgumentList @("run", "repo-mcp:install") -WorkingDirectory $repoRoot\n')
edit("bootstrap", ps, '    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")\n    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("x", "playwright", "install", "chromium") -WorkingDirectory $repoRoot\n', '    $playwrightCli = Join-Path $repoRoot "node_modules\\playwright\\cli.js"\n    if (Test-Path -LiteralPath $playwrightCli -PathType Leaf) {\n        $nodePath = Get-CommandPathOrThrow -Label "node" -Candidates @("node.exe")\n        Invoke-RepoCommand -FilePath $nodePath -ArgumentList @($playwrightCli, "install", "chromium") -WorkingDirectory $repoRoot\n    } else {\n        Write-Step "Playwright is not installed; skipping the Chromium browser."\n    }\n')
# devcontainer
D = ".devcontainer"
edit("bootstrap", D + "/Dockerfile", "ARG BUN_VERSION=1.4.2\n", "")
edit("bootstrap", D + "/Dockerfile", 'bun_archive="x64-baseline"; bun_sha="c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f"; ', "")
edit("bootstrap", D + "/Dockerfile", 'bun_archive="aarch64"; bun_sha="54328bbc2d9c8e0c9f892c544d66c57a83b84139e34909e5ee81758f1ac8fda7"; ', "")
dt = live(D + "/Dockerfile"); a = dt.index('    curl --fail --location --retry 3 --max-time 300 "https://github.com/oven-sh/bun/'); b = dt.index('    curl --fail --location --retry 3 --max-time 300 "https://nodejs.org/')
group("bootstrap")["edits"].append({"path": D + "/Dockerfile", "old": dt[a:b], "new": "", "count": 1})
edit("bootstrap", D + "/Dockerfile", "ENV BUN_INSTALL=/home/vscode/.bun\nENV PATH=/home/vscode/.local/bin:/home/vscode/.bun/bin:", "ENV PATH=/home/vscode/.local/bin:/home/vscode/.local/share/semio/launcher/bin:")
edit("bootstrap", D + "/README.md", "plus Bun 1.4.2 and Node 24.15.0", "plus Node 24.15.0 (the JavaScript runtime is provisioned by the `repo` launcher from its pinned registry, not installed in the image)")
edit("bootstrap", D + "/docker-compose.yml", "(`bun repo preflight` refuses", "(`repo preflight` refuses")
# CI
W = ".github/workflows"
edit("bootstrap", W + "/architecture-course.yml", "      - uses: oven-sh/setup-bun@0c5077e51419868618aeaa5fe8019c62421857d6 # v2.2.0\n        with:\n          bun-version-file: package.json\n", "      - uses: ./.github/actions/repo-launcher\n", 3)
sweep("bootstrap", W + "/architecture-course.yml", r"\bbun repo\b", "repo", 3)
bs = live(W + "/build-system.yml")
edit("bootstrap", W + "/build-system.yml", "  BUN_VERSION: 1.4.2\n", "")
edit("bootstrap", W + "/build-system.yml", "      - uses: ./.github/actions/bun-repo\n        with:\n          bun-version: ${{ env.BUN_VERSION }}\n          node-version: ${{ env.NODE_VERSION }}\n", "      - uses: ./.github/actions/repo-launcher\n        with:\n          node-version: ${{ env.NODE_VERSION }}\n", 3)
sweep("bootstrap", W + "/build-system.yml", r'\bbun "\$CI_REPORT"', 'repo "$CI_REPORT"', len(re.findall(r'\bbun "\$CI_REPORT"', bs)))
sweep("bootstrap", W + "/build-system.yml", r"\bbun repo\b", "repo", len(re.findall(r"\bbun repo\b", bs)))
delete("bootstrap", ".github/actions/bun-repo/action.yml")
# docs
R = M[: M.rindex("/")] + "/README.md"
rd = live(R)
sweep("docs", R, r"\bbun repo\b", "repo", len(re.findall(r"\bbun repo\b", rd)))
sweep("docs", R, r"\bbun (\./📜️script\.ts)", r"repo \1", len(re.findall(r"\bbun (\./📜️script\.ts)", rd)))
sweep("docs", R, r"install Bun, Node, Go, rustup and uv", "install Node, Go, rustup and uv, build the `repo` launcher (which provisions the pinned JavaScript runtime itself)", len(re.findall(r"install Bun, Node, Go, rustup and uv", rd)))
sweep("docs", R, r"Bun, Node, Go, rustup und uv", "Node, Go, rustup und uv, bauen den `repo`-Launcher (er stellt die gepinnte JavaScript-Laufzeit selbst bereit)", len(re.findall(r"Bun, Node, Go, rustup und uv", rd)))
for g in plan["groups"].values():
    for row in g["edits"]:
        if "old" in row: row["sha256Before"] = sha(live(row["path"]))
        else: row["sha256Before"] = sha(live(row["path"]))
json.dump(plan, open(os.path.join(here, "plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print({name: {k: len(v) for k, v in g.items()} for name, g in plan["groups"].items()})
