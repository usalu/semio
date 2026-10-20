/** 🏗️ The fixture builders of the transaction-v2 suite: run roots, plans, snapshots and fixture variants. */
import { constants, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../../🧹️normalization/🧠️planning/🟦️.ts";
import { type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { ownedFilePaths, ownedPathByteSort } from "../../🔍️filesystem/🟦️.ts";
import { FIXTURE_RUN_ID, FIXTURE_SCHEMA, type Fixture, SCHEMA_REL, type Snapshot, fixturePlans, fixtureTemplates } from "../📜️boundary-contracts/🟦️.ts";

/** 🔐️ Validates the exact propagated run identity before any fixture access. */
function assertFixtureRunRoot(artifactRoot: string, runId: string, runRoot: string): string {
  const identity = /^[1-9][0-9]*-([0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12})$/u.exec(runId);
  if (!identity || !isAbsolute(artifactRoot) || resolve(artifactRoot) !== artifactRoot || !isAbsolute(runRoot) || resolve(runRoot) !== runRoot) throw new Error("Invalid transaction fixture run identity");
  const expected = join(artifactRoot, "🧾️runs", "🔖️" + identity[1]);
  if (runRoot !== expected) throw new Error("Transaction fixture belongs to a different run");
  let path = parse(runRoot).root;
  for (const part of relative(path, runRoot).split(sep)) {
    path = join(path, part);
    const stat = lstatSync(path);
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction fixture run ancestor must be a no-follow directory: " + path);
  }
  return runRoot;
}

export function fixtureRunRoot(): string {
  return assertFixtureRunRoot(process.env.SEMIO_TEST_ARTIFACT_DIR ?? "", FIXTURE_RUN_ID, process.env.SEMIO_TRANSACTION_V2_RUN_ROOT ?? "");
}

export function retainFixture(root: string): void {
  if (dirname(root) !== fixtureRunRoot() || !basename(root).startsWith("🧪️")) throw new Error("Unexpected transaction fixture root");
  const stat = lstatSync(root);
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction fixture must be a no-follow directory");
}

export function git(root: string, args: readonly string[]): string {
  const result = Bun.spawnSync(["git", ...args], { cwd: root, env: { ...process.env, GIT_AUTHOR_DATE: "2000-01-01T00:00:00Z", GIT_COMMITTER_DATE: "2000-01-01T00:00:00Z" }, stdout: "pipe", stderr: "pipe" });
  const stdout = result.stdout.toString(), stderr = result.stderr.toString();
  if (result.exitCode !== 0) throw new Error(`fixture git ${args.join(" ")} failed: ${stderr || stdout}`);
  return stdout.trim();
}

function writeFiles(root: string, files: Readonly<Record<string, string>>): void {
  for (const [path, bytes] of Object.entries(files)) {
    const target = join(root, path);
    mkdirSync(dirname(target), { recursive: true });
    writeFileSync(target, bytes);
  }
}

function fixture(name: string, files: Readonly<Record<string, string>>, configure?: (row: Fixture) => void): Fixture {
  const owner = fixtureRunRoot();
  const templateKey = `${canonicalJson(files)}\0${configure?.toString() ?? ""}`;
  let template = fixtureTemplates.get(templateKey);
  if (!template) {
    const shared = process.env.SEMIO_TRANSACTION_V2_RUN_ID !== undefined;
    const digest = new Bun.CryptoHasher("sha256").update(templateKey).digest("hex");
    const root = shared ? join(owner, `🧪️template-${digest}`) : mkdtempSync(join(owner, "🧪️template-"));
    let buildRoot = root, ownsBuild = !shared;
    if (shared && !existsSync(root)) {
      buildRoot = `${root}-preparing`;
      try { mkdirSync(buildRoot); ownsBuild = true; }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
      if (!ownsBuild) {
        const deadline = Date.now() + 10_000;
        while (!existsSync(root)) {
          if (Date.now() >= deadline) throw new Error(`Timed out waiting for shared fixture template ${digest}`);
          Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2);
        }
      }
    }
    if (ownsBuild) {
      const ticketDir = join(buildRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), schemaPath = join(buildRoot, SCHEMA_REL), scope = relative(buildRoot, workspace).replaceAll("\\", "/");
      mkdirSync(workspace, { recursive: true });
      mkdirSync(dirname(schemaPath), { recursive: true });
      const taxonomy = JSON.parse(readFileSync(FIXTURE_SCHEMA, "utf8"));
      delete taxonomy.generatorContracts["plugin-registry"].inputDiscovery;
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
      writeFiles(workspace, files);
      git(buildRoot, ["init", "--quiet", "--object-format=sha1"]);
      git(buildRoot, ["config", "user.name", "Semio Transaction Fixture"]);
      git(buildRoot, ["config", "user.email", "transaction-fixture@invalid.example"]);
      git(buildRoot, ["config", "commit.gpgsign", "false"]);
      configure?.({ baselineCommit: "", options: { repoRoot: buildRoot, scope, ticketDir, workers: 1 }, repoRoot: buildRoot, root: buildRoot, scope, ticketDir, workspace });
      git(buildRoot, ["add", "--all", "--", ".", `:(exclude,literal)${SCHEMA_REL}`]);
      git(buildRoot, ["commit", "--quiet", "-m", "transaction fixture"]);
      if (shared) renameSync(buildRoot, root);
    }
    retainFixture(root);
    const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
    template = { baselineCommit: head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head, processOwned: !shared, root };
    fixtureTemplates.set(templateKey, template);
  }
  retainFixture(template.root);
  const root = mkdtempSync(join(owner, `🧪️${name}-`));
  cpSync(template.root, root, { recursive: true, mode: constants.COPYFILE_FICLONE });
  const rebaseSymlinks = (path: string): void => {
    for (const name of readdirSync(path)) {
      const child = join(path, name), stat = lstatSync(child);
      if (stat.isDirectory() && !stat.isSymbolicLink()) rebaseSymlinks(child);
      else if (stat.isSymbolicLink()) {
        const target = readlinkSync(child);
        if (target.startsWith(template!.root)) { rmSync(child); symlinkSync(`${root}${target.slice(template!.root.length)}`, child, "file"); }
      }
    }
  };
  rebaseSymlinks(root);
  const ticketDir = join(root, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(root, workspace).replaceAll("\\", "/");
  return { baselineCommit: template.baselineCommit, options: { repoRoot: root, scope, ticketDir, workers: 1 }, planCacheKey: configure?.toString().includes("symlinkSync") ? undefined : templateKey, repoRoot: root, root, scope, ticketDir, workspace };
}

export function plan(row: Fixture): TaxonomyPlan {
  const cached = row.planCacheKey ? fixturePlans.get(row.planCacheKey) : undefined;
  if (cached) return cached;
  const value = planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
  if (row.planCacheKey) fixturePlans.set(row.planCacheKey, value);
  return value;
}

export function snapshot(root: string): Snapshot {
  if (!existsSync(root)) return {};
  const rows: [string, string][] = [];
  const visit = (path: string): void => {
    const absolute = path ? join(root, path) : root, stat = lstatSync(absolute), mode = (stat.mode & 0o7777).toString(8);
    if (stat.isSymbolicLink()) rows.push([path || ".", `symlink|${mode}|${Buffer.from(readlinkSync(absolute)).toString("base64")}`]);
    else if (stat.isFile()) rows.push([path || ".", `file|${mode}|${readFileSync(absolute).toString("base64")}`]);
    else if (stat.isDirectory()) {
      rows.push([path || ".", `directory|${mode}`]);
      for (const name of readdirSync(absolute).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) visit(path ? `${path}/${name}` : name);
    }
  };
  visit("");
  return Object.fromEntries(rows);
}

export function transactionRoot(row: Fixture): string {
  return join(row.ticketDir, "🧾️taxonomy-transaction");
}

export function journalPaths(row: Fixture): string[] {
  return ownedFilePaths(row.ticketDir)
    .filter((path) => path.startsWith("🧾️taxonomy-transaction/") && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
    .map((path) => join(row.ticketDir, path))
    .sort(ownedPathByteSort);
}

export function writePlan(row: Fixture, value: TaxonomyPlan): string {
  const path = join(row.ticketDir, "📊️taxonomy-plan", "🔣️.json");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${canonicalJson(value)}\n`);
  return path;
}

export function referenceFixture(name: string): Fixture {
  return fixture(name, {
    "🧪️subject/🟦️component.ts": "export const value = 1;\n",
    "🧪️consumer/🟦️component.ts": "export { value } from \"../🧪️subject/🟦️component.ts\";\n",
  });
}

export function cloneFixture(source: Fixture, name: string): Fixture {
  const owner = fixtureRunRoot(), root = mkdtempSync(join(owner, `🧪️${name}-`));
  retainFixture(source.root);
  cpSync(source.root, root, { recursive: true, mode: constants.COPYFILE_FICLONE });
  const ticketDir = join(root, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(root, workspace).replaceAll("\\", "/");
  return { baselineCommit: source.baselineCommit, options: { ...source.options, repoRoot: root, scope, ticketDir }, planCacheKey: source.planCacheKey, repoRoot: root, root, scope, ticketDir, workspace };
}

export function symlinkFixture(name: string): Fixture {
  return fixture(name, { "🧪️target/🟦️component.ts": "export const target = true;\n" }, (row) => {
    const link = join(row.workspace, "🧪️link", "🟦️component.ts");
    mkdirSync(dirname(link), { recursive: true });
    symlinkSync("../🧪️target/🟦️component.ts", link);
  });
}

export function embeddedFixture(name: string): Fixture {
  const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL";
  const files: Record<string, string> = {};
  for (const owner of ["pkg-a", "pkg-b"]) {
    files[`${owner}/${ticket}/🧪️target-os-errors/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
    files[`${owner}/${ticket}/🧪️unique-${owner.slice(-1)}/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
  }
  return fixture(name, files, (row) => {
    const canonicalManifest = join(row.repoRoot, ticket, "🎫️ticket.json");
    mkdirSync(dirname(canonicalManifest), { recursive: true });
    writeFileSync(canonicalManifest, '{"id":"26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"}\n');
    const schemaPath = join(row.repoRoot, SCHEMA_REL), taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
    const scope = relative(row.repoRoot, row.workspace).replaceAll("\\", "/");
    taxonomy.fixedFilenameContracts = Object.fromEntries(Object.entries({ ...(taxonomy.fixedFilenameContracts as Record<string, unknown>), "fixture-cache-tag": { pathPattern: `${scope}/**/CACHEDIR.TAG`, authority: "Transaction disposition golden", reason: "Exact cache marker fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
    taxonomy.fixedDirectoryContracts = Object.fromEntries(Object.entries({ ...(taxonomy.fixedDirectoryContracts as Record<string, unknown>), "fixture-package-prefix": { pathPattern: `${scope}/pkg-*`, authority: "Transaction disposition golden", reason: "Exact embedded-root owner fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
    writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
  });
}

export function generatorFixture(name: string): Fixture {
  return fixture(name, { "🧪️generator/🟦️.ts": "export const input = true;\n", "🧪️generator/🤖️generated/old.txt": "stale\n" }, (row) => {
    const owner = relative(row.repoRoot, join(row.workspace, "🧪️generator")).replaceAll("\\", "/"), outputRoot = `${owner}/🤖️generated`, schemaPath = join(row.repoRoot, SCHEMA_REL);
    const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
    taxonomy.generatorContracts = Object.fromEntries(Object.entries({ ...(taxonomy.generatorContracts as Record<string, unknown>), "fixture-generator": { ownership: "owned", ownerPath: owner, target: "@fixture/generator:generate", previewTarget: "@fixture/generator:preview-generated", inputPatterns: [`${owner}/⚙️engine/🟦️.ts`, `${owner}/🟦️.ts`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Transaction mixed-output fixture" } }).sort(([left], [right]) => left.localeCompare(right)));
    writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
    writeFileSync(join(row.repoRoot, ".gitignore"), `${outputRoot}\nnode_modules\n`);
    symlinkSync(join(getWorkspaceRoot(), "node_modules"), join(row.repoRoot, "node_modules"), "junction");
    writeFileSync(join(row.repoRoot, "nx.json"), "{\"defaultBase\":\"main\"}\n");
    writeFileSync(join(row.repoRoot, "package.json"), "{\"name\":\"transaction-generator-fixture\",\"private\":true}\n");
    const project = `${JSON.stringify({ name: "@fixture/generator", root: owner, targets: { generate: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts generate" } }, "preview-generated": { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts preview-generated" } }, check: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts check" } } } }, null, 2)}\n`;
    writeFileSync(join(row.repoRoot, "project.json"), project);
    writeFiles(join(row.workspace, "🧪️generator"), {
      "📋️project.json": project,
      "📜️script.ts": [
        "import { BundleScript, ScriptRouter, runScriptMain } from \"@semio-tech/repo-lib/routing\";",
        "import { runEvidenceGenerator } from \"./⚙️engine/🟦️.ts\";",
        "class PreviewScript extends BundleScript {",
        "  run(): void { runEvidenceGenerator(this.root,\"preview-generated\"); }",
        "}",
        "class GenerateScript extends BundleScript {",
        "  run(): void { runEvidenceGenerator(this.root,\"generate\"); }",
        "}",
        "class CheckScript extends BundleScript {",
        "  run(): void { runEvidenceGenerator(this.root,\"check\"); }",
        "}",
        "await runScriptMain(new ScriptRouter(import.meta.dir).register(\"preview-generated\",PreviewScript).register(\"generate\",GenerateScript).register(\"check\",CheckScript),import.meta.url);",
        "",
      ].join("\n"),
      "⚙️engine/🟦️.ts": [
        "import { existsSync, mkdirSync, readdirSync, rmSync, writeFileSync, readFileSync } from \"node:fs\";",
        "import { join } from \"node:path\";",
        "/** ⚙️ Implements the declared portable transaction generator fixture. */",
        "export function runEvidenceGenerator(root: string, command: string): void {",
        "  const outputRoot=join(root,\"🤖️generated\"), outputRelative=\"🧪️tests/🧪️fixture/🧪️generator/🤖️generated\";",
        "  const outputFile=join(outputRoot,\"🔤️.txt\"), bytes=Buffer.from(\"generated\\n\");",
        "  const nodes=[{bytesBase64:\"\",mode:0o755,nodeKind:\"directory\",path:outputRelative},{bytesBase64:bytes.toString(\"base64\"),mode:0o644,nodeKind:\"file\",path:`${outputRelative}/🔤️.txt`}];",
        "  const staleRemovals=(existsSync(outputRoot)?readdirSync(outputRoot):[]).filter((name)=>name!==\"🔤️.txt\").map((name)=>`${outputRelative}/${name.normalize(\"NFC\")}`).sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)));",
        "  if(command===\"preview-generated\") process.stdout.write(`${JSON.stringify({contractId:\"fixture-generator\",nodes,schemaVersion:1,staleRemovals})}\\n`);",
        "  else if(command===\"generate\"){rmSync(outputRoot,{recursive:true,force:true});mkdirSync(outputRoot,{recursive:true,mode:0o755});writeFileSync(outputFile,bytes,{mode:0o644});if(process.env.MIXED_GENERATOR_MARKER){writeFileSync(join(outputRoot,\"unexpected.txt\"),\"mixed\\n\");writeFileSync(process.env.MIXED_GENERATOR_MARKER,`${JSON.stringify({pid:process.pid})}\\n`);Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,30000);}}",
        "  else if(command===\"check\"){if(!existsSync(outputFile)||!readFileSync(outputFile).equals(bytes)||readdirSync(outputRoot).join(\"\\0\")!==\"🔤️.txt\")throw new Error(\"generated output is stale\");}",
        "  else throw new Error(`unknown command ${command}`);",
        "}",
        "",
      ].join("\n"),
    });
  });
}
