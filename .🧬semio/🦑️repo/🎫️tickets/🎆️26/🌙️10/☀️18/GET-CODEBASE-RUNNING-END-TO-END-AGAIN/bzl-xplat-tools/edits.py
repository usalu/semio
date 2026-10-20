import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import schema_defs

STAGED = os.path.join(HERE, "staged")
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BZ = LIB + "/🏗️build/🔧️tools/🏗️bazel"
BS = LIB + "/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel"
TOOLS = LIB + "/⚡️caching/🚀️bootstrap/🛠️tools"
PROJECTION = TOOLS + "/📦️sdk/📥️source/📐️projection"
README = "🧰️framework/🛍️products/🦑️repo/README.md"

NEW_FILES = {
    BZ + "/🌐️platform/🟦️.ts": "platform.ts",
    BZ + "/🧪️tests/📥️platform/🟦️.ts": "platform-test.ts",
    BZ + "/🧪️tests/📥️platform/🔣️.json": "platform-fixture.json",
    BZ + "/🧪️tests/📥️platform/🧬️schema.json": "platform-fixture-schema.json",
    BS + "/🧪️tests/📥️pins/🟦️.ts": "pins-test.ts",
    BS + "/🧪️tests/📥️pins/🔣️.json": "pins-fixture.json",
}

ADAPTER_START = '''outputRoot=join(storage.paths.outputs,hash(JSON.stringify({version:BAZEL_VERSION,platform:process.platform,arch:process.arch,idle:policy.serverIdleSeconds,jvm:"-Xmx2048m"})).slice(0,16)),cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache}),startup=[`--output_user_root=${outputRoot}`,`--max_idle_secs='''
ADAPTER_START_NEW = '''cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache}),profile=requireBazelPlatformProfileV1(cache),startup=[`--max_idle_secs='''

PROFILE_HELPER = '''/** 🛂️ Renders the platform profile of the selected output base, prints its warnings and refuses before Bazel starts when this host cannot run it. */
function requireBazelPlatformProfileV1(cache:{readonly startup:readonly string[]}){
 const profile=bazelPlatformProfileV1({platform:process.platform,arch:process.arch,container:process.platform==="linux"&&process.env.DEVCONTAINER==="true",outputBase:cache.startup.find(flag=>flag.startsWith("--output_base="))!.slice("--output_base=".length),bun:process.execPath,longPaths:process.platform==="win32"?(readNativeWindowsLongPathsEnabledV1()?"enabled":"disabled"):"unprobed"});
 for(const warning of profile.warnings)console.warn(`[bazel] warning: ${warning}`);
 if(profile.refusals.length)throw new Error(`Bazel platform profile refused:\\n${profile.refusals.join("\\n")}`);
 return profile;
}

'''

FILE_EDITS = {
    BZ + "/📜️script.ts": [
        ('import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";\n',
         'import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";\nimport {bazelPlatformProfileV1,bazelPlatformRulesV1} from "./🌐️platform/🟦️.ts";\nimport {readNativeWindowsLongPathsEnabledV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️host/🟦️.ts";\n', 1),
        ('env:{...environment,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})}});', 'env:{...environment,...bazelPlatformRulesV1(process.platform).environment}});', 1),
        ('await symlink(root,link,process.platform==="win32"?"junction":"dir");\n}\n', 'await symlink(root,link,bazelPlatformRulesV1(process.platform).sourceLink);\n}\n\n' + PROFILE_HELPER.rstrip("\n") + "\n", 1),
        ('extension:process.platform==="win32"?".exe":""}));await writeBazelRuntimeV1', 'extension:bazelPlatformRulesV1(process.platform).launcherExtension}));await writeBazelRuntimeV1', 1),
        ('extension=process.platform==="win32"?".exe":"";births.set(', 'extension=bazelPlatformRulesV1(process.platform).launcherExtension;births.set(', 1),
        (ADAPTER_START, ADAPTER_START_NEW, 1),
        ('...cache.startup],options=[', '...cache.startup,...profile.startup],options=[', 1),
        ('options[0]="--lockfile_mode=error";options.push("--incompatible_autoload_externally=",', 'options[0]="--lockfile_mode=error";options.push(...profile.command,"--incompatible_autoload_externally=",', 1),
        ('SEMIO_BAZEL_SELECTION_CHAIN:[...chain,namespace].join(":"),...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};await writeBazelRuntimeV1', 'SEMIO_BAZEL_SELECTION_CHAIN:[...chain,namespace].join(":"),...profile.environment};await writeBazelRuntimeV1', 1),
    ],
    BZ + "/🧪️tests/📥️actions/🟦️.ts": [
        ('toMatch(/"runner": "(darwin|linux)-sandbox"/u)', 'toMatch(/"runner": "(darwin|linux|processwrapper)-sandbox"/u)', 1),
    ],
    BZ + "/💾️storage/🟦️.ts": [
        ('(hostPaths.platforms as Readonly<Record<string,{pattern:string}>>)[platform]', '(hostPaths.platforms as Readonly<Record<string,{pattern:string;guarded?:readonly string[]}>>)[platform]', 1),
        ('for(const key of hostPaths.guarded){const value=layout[key]', 'for(const key of rule.guarded??hostPaths.guarded){const value=layout[key]', 1),
    ],
    BZ + "/💾️storage/🧪️tests/🟦️.ts": [
        ('test("Bazel host paths must be ASCII on macOS only, against an independent Python regex"', 'test("Bazel host paths must be ASCII on macOS (install, outputs, server) and on Linux (install), against an independent Python regex"', 1),
        ('expect(Object.keys(policy.platforms)).toEqual(["darwin"]);', 'expect(Object.keys(policy.platforms)).toEqual(["darwin","linux"]);', 1),
        ('const darwin=fixture.hostPaths.filter(row=>row.platform==="darwin"),result=Bun.spawnSync([python,"-I","-c","import json,re,sys\\npattern,rows=json.loads(sys.stdin.read())\\nprint(json.dumps([all(re.fullmatch(pattern,path) is not None for path in paths) for paths in rows]))"],{stdin:new TextEncoder().encode(JSON.stringify([policy.platforms.darwin.pattern,darwin.map(row=>policy.guarded.map(key=>(row.layout as Record<string,string>)[key]))])),stdout:"pipe",stderr:"pipe"});\n  expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual(darwin.map(row=>row.accepted));',
         'const rules=policy.platforms as Record<string,{pattern:string;guarded?:string[]}>,rows=fixture.hostPaths.filter(row=>row.platform in rules),result=Bun.spawnSync([python,"-I","-c","import json,re,sys\\nrules,rows=json.loads(sys.stdin.read())\\nprint(json.dumps([all(re.fullmatch(rules[name],path) is not None for path in paths) for name,paths in rows]))"],{stdin:new TextEncoder().encode(JSON.stringify([Object.fromEntries(Object.entries(rules).map(([name,rule])=>[name,rule.pattern])),rows.map(row=>[row.platform,(rules[row.platform]!.guarded??policy.guarded).map(key=>(row.layout as Record<string,string>)[key])])])),stdout:"pipe",stderr:"pipe"});\n  expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual(rows.map(row=>row.accepted));', 1),
        ('if(process.platform==="darwin")expect(()=>bazelCacheLayoutV1(emoji)).toThrow("ASCII Bazel host path required");else expect(bazelCacheLayoutV1(emoji).install)', 'if(process.platform==="darwin"||process.platform==="linux")expect(()=>bazelCacheLayoutV1(emoji)).toThrow("ASCII Bazel host path required");else expect(bazelCacheLayoutV1(emoji).install)', 1),
    ],
    BZ + "/💾️storage/🧪️tests/🧫️fixtures/🔣️.json": [
        ('    {"name": "emoji install and output base on Linux", "platform": "linux", "layout": {"install": "/home/dev/🧰️repo/.cache/b/t/bazel/install", "outputs": "/home/dev/🧰️repo/.cache/b/p/bazel/outputs", "server": "/home/dev/🧰️repo/.cache/b/p/bazel/outputs/server", "workspace": "/Users/dev/repo/.cache/b/p/m/bazel/workspaces"}, "accepted": true},\n',
         '    {"name": "ascii checkout on Linux", "platform": "linux", "layout": {"install": "/home/dev/repo/.cache/b/t/m/bazel/install", "outputs": "/home/dev/repo/.cache/b/p/m/bazel/outputs", "server": "/home/dev/repo/.cache/b/p/m/bazel/outputs/server", "workspace": "/home/dev/repo/.cache/b/p/m/bazel/workspaces"}, "accepted": true},\n    {"name": "emoji install base on Linux", "platform": "linux", "layout": {"install": "/home/dev/🧰️repo/.cache/b/t/m/bazel/install", "outputs": "/home/dev/repo/.cache/b/p/m/bazel/outputs", "server": "/home/dev/repo/.cache/b/p/m/bazel/outputs/server", "workspace": "/home/dev/repo/.cache/b/p/m/bazel/workspaces"}, "accepted": false},\n    {"name": "accent install base on Linux", "platform": "linux", "layout": {"install": "/home/Jürgen/repo/.cache/b/t/m/bazel/install", "outputs": "/home/dev/repo/.cache/b/p/m/bazel/outputs", "server": "/home/dev/repo/.cache/b/p/m/bazel/outputs/server", "workspace": "/home/dev/repo/.cache/b/p/m/bazel/workspaces"}, "accepted": false},\n    {"name": "emoji output and server base on Linux are fine", "platform": "linux", "layout": {"install": "/home/dev/repo/.cache/b/t/m/bazel/install", "outputs": "/home/dev/🧰️repo/.cache/b/p/m/bazel/outputs", "server": "/home/dev/🧰️repo/.cache/b/p/m/bazel/outputs/server", "workspace": "/home/dev/🧰️repo/.cache/b/p/m/bazel/workspaces"}, "accepted": true},\n', 1),
    ],
    PROJECTION + "/🧬️schema/🔣️.json": [
        ('          "additionalProperties": {\n            "type": "string",\n            "pattern": "^[0-9]+\\\\.[0-9]+\\\\.[0-9]+$"\n          }',
         '          "additionalProperties": {\n            "type": "string",\n            "pattern": "^(?:[0-9]+\\\\.[0-9]+\\\\.[0-9]+|workspace:\\\\*)$"\n          }', 1),
    ],
    PROJECTION + "/🟦️.ts": [
        ('/** 📐️ Captures exact dependency installation inputs separately from first-party SDK code. */\n',
         '/** 🔗️ Keeps the external pins of a recipe: a `workspace:*` link names first-party code the captured sources resolve, never a package the installer fetches, and the lock holds no row for it. */\nexport function nxExternalPinsV1(dependencies:Readonly<Record<string,string>>):Record<string,string>{\n return Object.fromEntries(Object.entries(dependencies).filter(([,version])=>version!=="workspace:*"));\n}\n\n/** 📐️ Captures exact dependency installation inputs separately from first-party SDK code. */\n', 1),
        ('dependencies:value.dependencies})+"\\n")]', 'dependencies:nxExternalPinsV1(value.dependencies)})+"\\n")]', 1),
    ],
    TOOLS + "/🟦️.ts": [
        ('import{nxDependencyInstallFilesV1}from"./📦️sdk/📥️source/📐️projection/🟦️.ts";', 'import{nxDependencyInstallFilesV1,nxExternalPinsV1}from"./📦️sdk/📥️source/📐️projection/🟦️.ts";', 1),
        ('packageManager:recipe.packageManager,dependencies:recipe.dependencies,patchedDependencies:recipe.semio.toolPatches}', 'packageManager:recipe.packageManager,dependencies:nxExternalPinsV1(recipe.dependencies),patchedDependencies:recipe.semio.toolPatches}', 1),
        ('for (const [name, version] of Object.entries(manifest.dependencies)) {', 'for (const [name, version] of Object.entries(nxExternalPinsV1(manifest.dependencies))) {', 1),
    ],
}


README_PROFILE = (
    "- **Platform profile.** Every platform-dependent decision of the adapter is one pure function of observed host facts, `bazelPlatformProfileV1` in `🏗️build/🔧️tools/🏗️bazel/🌐️platform` (rules and limits in `PlatformRules` of the Bazel schema; fixture `🧪️tests/📥️platform/🔣️.json`, 19 rows for darwin, linux, the devcontainer and win32 on x64 and arm64, including the refusals). Per platform it renders: the extra startup and command flags (`--nobuild_runfile_links` everywhere, because nothing runs `bazel run` and a runfiles symlink forest would need a symlink privilege on Windows; `--noenable_runfiles` and the startup option `--nowindows_enable_symlinks` on Windows, so the source link `s` is a junction and no developer mode or elevation is needed), the locale environment (`LANG`/`LC_ALL` `en_US.UTF-8` on darwin only), the launcher extension (`.exe` on Windows), the kind of the source link (`junction` on Windows) and the path budget `len(output base) + len(\\execroot\\_main\\s\\) + 256` in UTF-16 units against 259 (MAX_PATH) or, with `LongPathsEnabled`, 32766; a budget that does not fit, a Bun path that is not absolute (and `.exe` on Windows) and a devcontainer flag on a non-Linux host are refusals, thrown before Bazel starts. `--output_user_root` is gone: `--output_base` and `--install_base` are always explicit, so it was dead weight and (Windows) a second long path. Every action runs the Bun executable directly (`ctx.actions.run`, `executable = ctx.attr.bun`), never a shell, `genrule` or `run_shell`; the BUILD quotes the Bun path with `JSON.stringify`, which a real Bazel parses for a Windows path with backslashes. Evidence of 2026-10-20 (Bazel 8.4.2): macOS arm64 and Linux arm64 (container built from `.devcontainer/Dockerfile`, user `vscode`, no capability, `.cache` on a Docker ext4 volume): `SEMIO_TEST_BAZEL_BINARY` run of the new suite passed on both (Bazel's own `help` lists every rendered flag name, a BUILD with a Windows Bun path parses, a build of an emoji source through the `s` link with the rendered flags is served from the disk cache after `clean`: `disk cache hit`); the pinned linux/arm64 binary was downloaded and matched its pin; `bun repo --help`, `bun repo preflight` and `bun repo tools setup bazel` (47 s: binary, 156 module blobs, readiness line) ran unprivileged on Linux. Linux findings by experiment: emoji source labels, outputs, tree artifacts and a non-ASCII output base work under every locale tried (unset, POSIX, C, C.UTF-8, en_US.UTF-8), a non-ASCII `--install_base` does not (see Sandbox and host paths), `linux-sandbox` is not registered in an unprivileged container (default seccomp; also not with `seccomp=unconfined`) and Bazel's default `sandboxed` strategy falls back to `processwrapper-sandbox`, which ran a sandboxed action there, while a `--privileged` container registers `linux-sandbox` and ran one; so the adapter never names a sandbox strategy. Not run: Windows (the profile, the refusals and the Starlark escaping are proven by fixture and by the Linux and macOS binaries, not by a Windows binary), the emoji labels on Windows (`unverified` in the profile, one warning per run), whether `bun.exe` is long-path aware, and the CI matrix of `.github/workflows/build-system.yml`.\n"
)


def schema_edit(text):
    return json.dumps(schema_defs.apply(json.loads(text)), ensure_ascii=False, indent=2) + "\n"


JSON_EDITS = {BZ + "/🧬️schema/🔣️.json": schema_edit}


FILE_EDITS[README] = [
    ("Linux (bind mounts instead of a profile) and Windows were not examined, the guard is darwin only; Windows is covered by the readiness refusal for non-ASCII workspace and partition paths.",
     "Linux was examined on 2026-10-20 (Bazel 8.4.2 linux/arm64 in an unprivileged container): a non-ASCII `--install_base` crashes the server there too (\"Could not find or load main class\", exit code 37) under every locale tried (unset, POSIX, C, C.UTF-8, en_US.UTF-8), so `requireBazelHostPathsV1` refuses a non-ASCII `install` path on Linux (\"ASCII Bazel host path required on linux: install\", `HostPaths.platforms.linux.guarded`), whereas a non-ASCII output base builds and caches on Linux and is not refused. Windows is covered by the readiness refusal for non-ASCII workspace and partition paths.", 1),
    ("the adapter sets a UTF-8 locale on darwin (`LANG` and `LC_ALL` `en_US.UTF-8` in `runBazelTasksInStorageV1`).",
     "the adapter sets a UTF-8 locale on darwin only (`LANG` and `LC_ALL` `en_US.UTF-8`, the `environment` of the darwin row of `PlatformRules`). On Linux arm64 the same emoji source, output and tree-artifact names built under every locale tried (unset, POSIX, C, C.UTF-8, en_US.UTF-8), so Linux needs no locale variable; Windows is `unverified` in the profile.", 1),
    ("- **Verified and not yet verified.**", README_PROFILE + "- **Verified and not yet verified.**", 1),
    ("a remote cache, Linux, Windows and the devcontainer.", "a remote cache, Windows and sandboxed task actions (see Platform profile for the Linux and macOS runs of 2026-10-20).", 1),
]

