import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { EMOJI_FONTCONFIG, PERSISTED_HOME_DIRECTORIES, WORKSPACE_EXTENSION, devcontainerStart, editorCliOrder, gitKrakenWorkspaceState, installWorkspaceExtension, missingSafeDirectories, normalizeClaudeAuth, releaseAssetUrl, toolArchitectures, withSshAgentShellBlock, type LifecycleAnswer, type LifecycleContext, type LifecycleHost } from "../../🔁️lifecycle/🟦️.ts";

/** 🎙️ A host that records every program call and answers from `answer`, never running anything. */
function recordingHost(answer: (command: string, args: readonly string[]) => Partial<LifecycleAnswer> = () => ({})): LifecycleHost & { calls: string[][]; inputs: Map<string, string>; lines: string[] } {
  const calls: string[][] = [], inputs = new Map<string, string>(), lines: string[] = [];
  return {
    calls,
    inputs,
    lines,
    run: (command, args, options) => {
      calls.push([command, ...args]);
      if (options?.input !== undefined) inputs.set([command, ...args].join(" "), options.input);
      return { status: 0, stdout: "", stderr: "", ...answer(command, args) };
    },
    spawnDetached: (command, args) => void calls.push(["detached", command, ...args]),
    download: async () => undefined,
    fetchJson: async () => ({}),
    sleep: async () => undefined,
    log: (line) => void lines.push(line),
  };
}

/** 🔁️ The devcontainer start/attach lifecycle against its language-agnostic fixture: the fontconfig document (fast-xml-parser
 * oracle), architecture and release-asset selection (lodash oracle), editor CLI order, GitKraken workspace states,
 * duplicate-free `safe.directory`, Claude auth storage on a real temporary home, the idempotent shell hook, a start run
 * that never issues a destructive command, and every extension-install outcome through a recording host. */
export async function testDevcontainerLifecycle(workspace: string, generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔁️lifecycle/🔣️.json"), "utf8"));
  const { XMLParser } = require("fast-xml-parser"), lodash = require("lodash");
  const document = new XMLParser({ ignoreAttributes: false }).parse(EMOJI_FONTCONFIG).fontconfig;
  const aliases = Object.fromEntries(document.alias.map((alias: { family: string; prefer: { family: string[] } }) => [alias.family, alias.prefer.family]));
  assert.deepEqual(aliases, fixture.fontconfig.aliases);
  assert.ok(document.match.every((match: { edit: unknown }) => JSON.stringify(match.edit).includes(fixture.fontconfig.emojiFamily) || JSON.stringify(match).includes(fixture.fontconfig.emojiFamily)));
  for (const row of fixture.architectures) assert.deepEqual(toolArchitectures(row.arch), { debian: row.debian, f3d: row.f3d, gk: row.gk }, row.arch);
  for (const row of fixture.releaseCases) {
    assert.equal(releaseAssetUrl(fixture.release, row.suffix), row.url, row.suffix);
    assert.equal(lodash.find(fixture.release.assets, (asset: { name: string }) => lodash.endsWith(asset.name, row.suffix))?.browser_download_url ?? null, row.url);
  }
  for (const row of fixture.editorOrder) assert.deepEqual(editorCliOrder(row.env), row.order, JSON.stringify(row.env));
  for (const row of fixture.gitKrakenInfo) assert.equal(gitKrakenWorkspaceState(row.info), row.state, row.info);
  assert.deepEqual(missingSafeDirectories(fixture.safeDirectories.listed, fixture.safeDirectories.required), fixture.safeDirectories.missing);
  const once = withSshAgentShellBlock("export PATH=$PATH\n");
  assert.equal(withSshAgentShellBlock(once), once, "the signing-agent shell hook is appended once");

  for (const row of fixture.claudeAuth) {
    const home = mkdtempSync(join(generated, `lifecycle-claude-${row.id}-`)), volume = join(home, ".claude");
    mkdirSync(volume, { recursive: true });
    for (const [name, content] of Object.entries(row.home)) writeFileSync(join(home, name), String(content));
    for (const [name, content] of Object.entries(row.volume)) writeFileSync(join(volume, name), String(content));
    normalizeClaudeAuth(home);
    normalizeClaudeAuth(home);
    for (const [name, content] of Object.entries(row.expected)) {
      assert.equal(readFileSync(join(volume, name), "utf8"), content, `${row.id}: ${name} lives in the volume`);
      assert.ok(lstatSync(join(home, name)).isSymbolicLink() && readlinkSync(join(home, name)) === join(volume, name), `${row.id}: ${name} is linked back`);
    }
    if (!Object.keys(row.expected).length) assert.equal(existsSync(join(home, ".claude.json")), false, row.id);
  }

  const home = mkdtempSync(join(generated, "lifecycle-start-home-")), checkout = mkdtempSync(join(generated, "lifecycle-start-workspace-"));
  writeFileSync(join(checkout, ".gitmodules"), "");
  const start = recordingHost((command, args) => (command === "git" && args.includes("--get-regexp") ? { stdout: "submodule.recherche.path ♻️mit-bestand/🔎️recherche\n" } : command === "git" && args.includes("--get-all") ? { stdout: `${checkout}\n` } : {}));
  const context: LifecycleContext = { workspace: checkout, home, user: "vscode", env: {}, arch: "x64" };
  devcontainerStart(start, context);
  const flat = start.calls.map((call) => call.join(" "));
  for (const directory of PERSISTED_HOME_DIRECTORIES) assert.ok(flat.includes(`sudo chown -R vscode:vscode ${join(home, directory)}`), directory);
  assert.ok(flat.includes(`sudo chown -R vscode:vscode ${join(checkout, "♻️mit-bestand/🔎️recherche")}`));
  assert.equal(start.inputs.get("sudo tee /etc/fonts/local.conf"), EMOJI_FONTCONFIG);
  assert.deepEqual(flat.filter((call) => call.startsWith("git config --global --add safe.directory")), [`git config --global --add safe.directory ${join(checkout, "♻️mit-bestand/🔎️recherche")}`]);
  assert.deepEqual(flat.filter((call) => fixture.forbiddenCommands.some((forbidden: string) => call.includes(forbidden))), [], "a container start never discards source-control or database state");

  for (const row of fixture.extension.cases) {
    const root = mkdtempSync(join(generated, `lifecycle-extension-${row.id}-`)), packageRoot = join(root, WORKSPACE_EXTENSION.packageRoot);
    mkdirSync(packageRoot, { recursive: true });
    writeFileSync(join(packageRoot, "package.json"), JSON.stringify({ publisher: fixture.extension.publisher, name: fixture.extension.name }));
    if (row.vsix) writeFileSync(join(packageRoot, WORKSPACE_EXTENSION.vsix), "vsix");
    const host = recordingHost((command, args) => {
      if (command === "bun") return { status: row.build };
      if (row.unreachable.includes(command)) return { stderr: "Command is only available in WSL or inside a Visual Studio Code terminal." };
      if (args[0] === "--list-extensions") return { stdout: row.listed[command] ?? "" };
      return {};
    });
    const installed = await installWorkspaceExtension(host, { ...context, workspace: root }, row.clis, join(root, "install.lock"));
    assert.equal(installed, row.installedVia, row.id);
    assert.equal(host.calls.filter((call) => call[1] === "--install-extension").length, row.installCalls, row.id);
    assert.equal(host.calls.filter((call) => call[1] === "--list-extensions").length, row.listCalls, `${row.id}: an unreachable editor is not asked for its extensions`);
    assert.deepEqual(host.calls.filter((call) => call[0] === "bun"), [["bun", "nx", "run", WORKSPACE_EXTENSION.target]], `${row.id}: packaging goes through Nx exactly once`);
    assert.equal(existsSync(join(root, "install.lock")), false, `${row.id}: the install lock is released`);
  }
  assert.ok(existsSync(join(workspace, WORKSPACE_EXTENSION.packageRoot, "package.json")), "the workspace extension package exists where the lifecycle packages it");
  console.log("✅️ Devcontainer lifecycle: fontconfig (fast-xml-parser), tool selection (lodash), editor order, GitKraken states, safe.directory, Claude auth storage, start without destructive commands, extension install outcomes PASS");
}
