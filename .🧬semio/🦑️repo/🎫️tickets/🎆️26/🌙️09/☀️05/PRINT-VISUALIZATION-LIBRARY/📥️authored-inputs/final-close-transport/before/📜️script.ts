import { readFileSync, writeFileSync, mkdirSync, renameSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { createHash } from "node:crypto";
import { spawn } from "node:child_process";

const root = "C:/git/semio";
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY");
const cli = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli";
const tickets = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets";
const hash = (bytes: string) => createHash("sha256").update(bytes).digest("hex");

/** 🧯️ Preserves authored before inputs and atomically writes only a guarded current candidate. */
function save(path: string, before: string, after: string): void {
  const absolute = join(root, path), input = join(ticket, "📥️authored-inputs/ticket-retention-before", process.argv[2]!, path);
  mkdirSync(dirname(input), { recursive: true });
  writeFileSync(input, before);
  if (readFileSync(absolute, "utf8") !== before) throw new Error(`Concurrent source change: ${path}`);
  const temporary = absolute + ".retention-tmp";
  writeFileSync(temporary, after);
  renameSync(temporary, absolute);
  console.log(`[DEBUG] ${path}: ${hash(before)} -> ${hash(after)}`);
}

const mode = process.argv[2];
if (mode === "neutral") {
  const fixturePath = `${tickets}/🧫️fixtures/🔓️open-close-reopen-lifecycle/🔓️lifecycle.json`;
  const fixtureBefore = readFileSync(join(root, fixturePath), "utf8"), fixture = JSON.parse(fixtureBefore);
  const mib = 1024 * 1024;
  fixture.purge.retention = [
    { path: "📥️authored-inputs/large.json", bytes: 6 * mib, retained: true },
    { path: "📥️authored-inputs/snapshots/source.txt", bytes: 6 * mib, retained: true },
    { path: "📥️authored-inputs/🗑️generated/authored.tex", bytes: 11 * mib, retained: true },
    { path: "📓️large-research.md", bytes: 6 * mib, retained: true },
    { path: "📜️script.ts", bytes: 6 * mib, retained: true },
    { path: "config.json", bytes: 6 * mib, retained: true },
    { path: "large.bin", bytes: 6 * mib, retained: true },
    { path: ".cache.bin", bytes: 6 * mib, retained: true },
    { path: "🎫️ticket.json", bytes: 6 * mib, retained: true },
    { path: "🗑️generated/large.log", bytes: 5 * mib + 1, retained: false },
    { path: "🗑️generated/exact.bin", bytes: 5 * mib, retained: true },
    { path: "🗑️generated/huge/blob.bin", bytes: 10 * mib + 1, retained: false },
    { path: "🗑️generated/huge/small.log", bytes: 10, retained: false },
    { path: "🗑️generated/small/file.bin", bytes: mib, retained: true },
  ];
  save(fixturePath, fixtureBefore, JSON.stringify(fixture, null, 2) + "\n");
  const schemaPath = `${tickets}/🧬️schema/🔣️.json`, schemaBefore = readFileSync(join(root, schemaPath), "utf8");
  const oldPurge = '"purge": { "type": "object", "properties": { "oversizedFile": { "type": "string" }, "smallFile": { "type": "string" } }, "required": ["oversizedFile", "smallFile"] }';
  const newPurge = '"purge": { "type": "object", "properties": { "oversizedFile": { "type": "string" }, "smallFile": { "type": "string" }, "retention": { "type": "array", "items": { "type": "object", "additionalProperties": false, "properties": { "path": { "type": "string", "minLength": 1 }, "bytes": { "type": "integer", "minimum": 0 }, "retained": { "type": "boolean" } }, "required": ["path", "bytes", "retained"] } } }, "required": ["oversizedFile", "smallFile", "retention"] }';
  if (!schemaBefore.includes(oldPurge)) throw new Error("Neutral schema shape changed");
  save(schemaPath, schemaBefore, schemaBefore.replace(oldPurge, newPurge).replace("A file above 5 MiB and a subfolder whose files total above 10 MiB go; 🎫️ticket.json never does.", "Only direct ticket 🗑️generated output is eligible: files above 5 MiB and subfolders above 10 MiB go. Authored inputs, scripts, configs and reports outside that output root are retained regardless of size."));
  const testPath = `${cli}/🧪️tests/🔬️component/🐹️.go`, testBefore = readFileSync(join(root, testPath), "utf8");
  const test = `
// 🛟️ TestTicketGeneratedOutputRetention checks the language-neutral generated-only lifecycle boundary.
func TestTicketGeneratedOutputRetention(t *testing.T) {
	data, err := os.ReadFile(filepath.Join(GetRootDir(), "${fixturePath}"))
	if err != nil { t.Fatal(err) }
	var fixture struct { Purge struct { Retention []struct { Path string; Bytes int64; Retained bool } } }
	if err := json.Unmarshal(data, &fixture); err != nil { t.Fatal(err) }
	if len(fixture.Purge.Retention) != 14 { t.Fatalf("retention vectors = %d", len(fixture.Purge.Retention)) }
	root := t.TempDir()
	for _, row := range fixture.Purge.Retention { writeSparseTicketArtifact(t, filepath.Join(root, filepath.FromSlash(row.Path)), row.Bytes) }
	if err := purgeOversizedTicketArtifacts(root); err != nil { t.Fatal(err) }
	for _, row := range fixture.Purge.Retention {
		info, err := os.Stat(filepath.Join(root, filepath.FromSlash(row.Path)))
		if row.Retained && (err != nil || info.Size() != row.Bytes) { t.Errorf("required retained artifact %s: %v", row.Path, err) }
		if !row.Retained && !os.IsNotExist(err) { t.Errorf("generated artifact still exists: %s", row.Path) }
	}
}
`;
  const marker = "func TestFinishTicketPurgesOversizedArtifacts(t *testing.T) {";
  if (testBefore.includes("func TestTicketGeneratedOutputRetention")) throw new Error("Neutral already saved");
  save(testPath, testBefore, testBefore.replace(marker, test + "\n" + marker));
  const oraclePath = `${cli}/🧪️tests/🃏️glob/🟦️.ts`, oracleBefore = readFileSync(join(root, oraclePath), "utf8");
  const oracle = `
/** 🪟️ Independently enumerates the neutral lifecycle filesystem with the existing third-party glob oracle. */
export function verifyTicketRetentionOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🎫️tickets/🧫️fixtures/🔓️open-close-reopen-lifecycle/🔓️lifecycle.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../../../🎫️tickets/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  if (!new Ajv({ strict: false }).validate({ ...schema, $ref: "#/$defs/LifecycleVectorFile" }, fixture)) throw new Error("Invalid neutral retention fixture");
  const owner = process.env.SEMIO_TICKET_DIR;
  if (!owner) throw new Error("Ticket-owned retention oracle output is required");
  const root = mkdtempSync(join(owner, "🗑️generated", "ticket-retention-oracle-"));
  try {
    for (const row of fixture.purge.retention) {
      const path = join(root, row.path); mkdirSync(dirname(path), { recursive: true });
      const fd = openSync(path, "w"); try { ftruncateSync(fd, row.bytes); } finally { closeSync(fd); }
    }
    const paths = globSync("**/*", { cwd: root, dot: true, nodir: true }).sort();
    const eligible = globSync("🗑️generated/**/*", { cwd: root, dot: true, nodir: true });
    const totals = new Map<string, number>();
    for (const path of eligible) for (let folder = dirname(path); folder !== "🗑️generated"; folder = dirname(folder)) totals.set(folder, (totals.get(folder) ?? 0) + statSync(join(root, path)).size);
    const removed = paths.filter(path => eligible.includes(path) && (statSync(join(root, path)).size > 5 * 1024 * 1024 || [...totals].some(([folder, bytes]) => bytes > 10 * 1024 * 1024 && path.startsWith(folder + "/"))));
    const expected = fixture.purge.retention.filter((row: { retained: boolean }) => !row.retained).map((row: { path: string }) => row.path).sort();
    if (JSON.stringify(removed) !== JSON.stringify(expected)) throw new Error("Independent generated-only filesystem oracle differs from neutral fixture");
    console.log("Ticket retention filesystem reference: 14 vectors verified with glob and AJV");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
`;
  const imports = 'import { readFileSync, mkdtempSync, mkdirSync, openSync, ftruncateSync, closeSync, statSync, rmSync } from "node:fs";\nimport { dirname, join } from "node:path";\nimport { globSync } from "glob";';
  save(oraclePath, oracleBefore, oracleBefore.replace('import { readFileSync } from "node:fs";', imports) + oracle);
  const routerPath = `${cli}/📦️packages/🟦️typescript/📜️script.ts`, routerBefore = readFileSync(join(root, routerPath), "utf8");
  save(routerPath, routerBefore, routerBefore.replace('(await import("../../🧪️tests/🃏️glob/🟦️.ts")).verifyFixtureGlobOracle();', 'const oracle = await import("../../🧪️tests/🃏️glob/🟦️.ts");\n    oracle.verifyFixtureGlobOracle();\n    if (process.env.SEMIO_TICKET_DIR) oracle.verifyTicketRetentionOracle();'));
} else if (mode === "owner") {
  const ownerPath = `${cli}/🧩️component/🐹️.go`, ownerBefore = readFileSync(join(root, ownerPath), "utf8");
  const start = ownerBefore.indexOf("func purgeOversizedTicketArtifacts(ticketDir string) error {");
  const end = ownerBefore.indexOf("// 🧹️collectTicketFolderRoots", start);
  if (start < 0 || end < 0) throw new Error("Purge owner bounds changed");
  const section = ownerBefore.slice(start, end), marker = "\n\ttype fileEntry struct {";
  if (!section.includes(marker)) throw new Error("Purge declaration changed");
  const boundary = `
	absDir = filepath.Join(absDir, "🗑️generated")
	info, err = os.Lstat(absDir)
	if err != nil {
		if os.IsNotExist(err) { return nil }
		return err
	}
	if info.Mode()&fs.ModeSymlink != 0 { return nil }
	if !info.IsDir() { return fmt.Errorf("generated ticket output is not a directory: %s", absDir) }
`;
  save(ownerPath, ownerBefore, ownerBefore.slice(0, start).replace("// 🧹️purgeOversizedTicketArtifacts deletes files above 5 MiB and subfolders above 10 MiB inside a closed ticket folder.", "// 🧺️purgeOversizedTicketArtifacts removes only oversized direct generated ticket output and preserves authored inputs.") + section.replace(marker, boundary + marker) + ownerBefore.slice(end));
  const fixturePath = `${tickets}/🧫️fixtures/🔓️open-close-reopen-lifecycle/🔓️lifecycle.json`, fixtureBefore = readFileSync(join(root, fixturePath), "utf8"), fixture = JSON.parse(fixtureBefore);
  fixture.purge.retention.push({ path: "🗑️generated/exact-directory/a.bin", bytes: 5 * 1024 * 1024, retained: true }, { path: "🗑️generated/exact-directory/b.bin", bytes: 5 * 1024 * 1024, retained: true });
  save(fixturePath, fixtureBefore, JSON.stringify(fixture, null, 2) + "\n");
  const testPath = `${cli}/🧪️tests/🔬️component/🐹️.go`, testBefore = readFileSync(join(root, testPath), "utf8");
  const legacyStart = testBefore.indexOf("func TestFinishTicketPurgesOversizedArtifacts(t *testing.T) {");
  const legacyEnd = testBefore.indexOf("\nfunc ", testBefore.indexOf("func TestPurgeAllOversizedTicketArtifacts(t *testing.T) {", legacyStart) + 5);
  if (legacyStart < 0 || legacyEnd < 0) throw new Error("Existing cleanup tests changed");
  let legacy = testBefore.slice(legacyStart, legacyEnd);
  for (const path of ["large.bin", "exact-5mb.bin", ".cache.bin", "huge-dir", "small-dir"]) legacy = legacy.replaceAll(`filepath.Join(ticketDir, "${path}"`, `filepath.Join(ticketDir, "🗑️generated", "${path}"`);
  const roots = `
// 🧭️ TestTicketGeneratedOutputRoots preserves authored data with missing, non-directory and symbolic generated roots.
func TestTicketGeneratedOutputRoots(t *testing.T) {
	for _, mode := range []string{"absent", "file", "symlink"} {
		t.Run(mode, func(t *testing.T) {
			root, external := t.TempDir(), t.TempDir()
			authored := filepath.Join(root, "📥️authored-inputs", "source.json")
			outside := filepath.Join(external, "blob.bin")
			writeSparseTicketArtifact(t, authored, 11<<20)
			writeSparseTicketArtifact(t, outside, 11<<20)
			output := filepath.Join(root, "🗑️generated")
			if mode == "file" { writeSparseTicketArtifact(t, output, 11<<20) }
			if mode == "symlink" { if err := os.Symlink(external, output); err != nil { t.Skipf("platform does not permit symbolic directory: %v", err) } }
			err := purgeOversizedTicketArtifacts(root)
			if (mode == "file") != (err != nil) { t.Fatalf("generated root %s: %v", mode, err) }
			for _, path := range []string{authored, outside} { info, err := os.Stat(path); if err != nil || info.Size() != 11<<20 { t.Fatalf("retained source %s: %v", path, err) } }
		})
	}
}
`;
  save(testPath, testBefore, (testBefore.slice(0, legacyStart) + roots + "\n" + legacy + testBefore.slice(legacyEnd)).replace("len(fixture.Purge.Retention) != 14", "len(fixture.Purge.Retention) != 16"));
  const oraclePath = `${cli}/🧪️tests/🃏️glob/🟦️.ts`, oracleBefore = readFileSync(join(root, oraclePath), "utf8");
  save(oraclePath, oracleBefore, oracleBefore.replace('import { dirname, join } from "node:path";', 'import { dirname, join } from "node:path";\nimport { tmpdir } from "node:os";').replace('  if (!owner) throw new Error("Ticket-owned retention oracle output is required");\n  const root = mkdtempSync(join(owner, "🗑️generated", "ticket-retention-oracle-"));', '  const root = mkdtempSync(join(owner ? join(owner, "🗑️generated") : tmpdir(), "ticket-retention-oracle-"));').replace('console.log("Ticket retention filesystem reference: 14 vectors verified with glob and AJV");', 'console.log(`Ticket retention filesystem reference: ${fixture.purge.retention.length} vectors verified with glob and AJV`);'));
  const routerPath = `${cli}/📦️packages/🟦️typescript/📜️script.ts`, routerBefore = readFileSync(join(root, routerPath), "utf8");
  save(routerPath, routerBefore, routerBefore.replace('if (process.env.SEMIO_TICKET_DIR) oracle.verifyTicketRetentionOracle();', 'oracle.verifyTicketRetentionOracle();'));
} else if (mode === "prepare-build") {
  const { canonicalGoPlan } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts"));
  const mcp = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp");
  const plan = canonicalGoPlan(mcp), sources = [...new Set(Object.values(plan.sourceReplacements))] as string[];
  const controls = ["go.work", "go.work.sum", "bun.lock", "bunfig.toml", `${cli}/go.mod`, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/go.mod", "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️script.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts"];
  const inputs = [...new Set([...sources, ...controls.map(path => join(root, path))])].sort().map(path => ({ path: relative(root, path).replaceAll("\\", "/"), bytes: readFileSync(path).length, sha256: createHash("sha256").update(readFileSync(path)).digest("hex") }));
  const target = join(ticket, "📥️authored-inputs/ticket-retention-mcp-build/source-capsule.json");
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, JSON.stringify({ capturedAt: new Date().toISOString(), purpose: "Prospective canonical Go MCP source projection and explicit control inputs before actual registered build; separate current ticket-retention owner contract; no print source edits.", module: mcp, binary: join(ticket, "🗑️generated/tools/repo-ticket-retention.exe"), packages: plan.packages, sourceReplacements: plan.sourceReplacements, inputs }, null, 2));
  console.log(`[DEBUG] MCP prospective source capsule: ${inputs.length} inputs, ${createHash("sha256").update(readFileSync(target)).digest("hex")}`);
} else if (mode === "transport") {
  const binary = join(ticket, "🗑️generated/tools/repo-ticket-retention.exe"), binaryBefore = createHash("sha256").update(readFileSync(binary)).digest("hex");
  const child = spawn(process.execPath, ["./📜️script.ts", "dev", "mcp", "stdio", "codex"], { cwd: root, env: { ...process.env, REPO_MCP_BIN: binary }, stdio: ["pipe", "pipe", "pipe"] });
  const pending = new Map<number, { resolve: (value: any) => void; reject: (reason: Error) => void }>();
  let text = "", stderr = "", id = 0;
  child.stdout.setEncoding("utf8"); child.stderr.setEncoding("utf8");
  child.stderr.on("data", chunk => { stderr += chunk; });
  child.stdout.on("data", chunk => {
    text += chunk;
    for (let newline = text.indexOf("\n"); newline >= 0; newline = text.indexOf("\n")) {
      const line = text.slice(0, newline).trim(); text = text.slice(newline + 1);
      try { const response = JSON.parse(line), waiter = pending.get(response.id); if (waiter) { pending.delete(response.id); response.error ? waiter.reject(new Error(JSON.stringify(response.error))) : waiter.resolve(response.result); } } catch {}
    }
  });
  const exited = new Promise<number | null>((resolve, reject) => { child.once("exit", resolve); child.once("error", reject); });
  const request = (method: string, params: unknown) => new Promise<any>((resolve, reject) => {
    const current = ++id;
    const timeout = setTimeout(() => { pending.delete(current); reject(new Error(`Timed out: ${method}`)); }, 30000);
    pending.set(current, { resolve: value => { clearTimeout(timeout); resolve(value); }, reject: reason => { clearTimeout(timeout); reject(reason); } });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id: current, method, params }) + "\n");
  });
  try {
    const initialized = await request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "ticket-retention-read-only", version: "1" } });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
    const tools = await request("tools/list", {}), goals = await request("resources/read", { uri: "repo://goals" });
    const close = tools.tools.find((tool: any) => tool.name === "ticket_close");
    if (!close || !goals.contents?.length) throw new Error("Read-only MCP admission failed");
    child.stdin.end();
    const exit = await exited;
    if (exit !== 0) throw new Error(`MCP read-only process exited ${exit}: ${stderr}`);
    const binaryAfter = createHash("sha256").update(readFileSync(binary)).digest("hex");
    const result = { capturedAt: new Date().toISOString(), selector: "REPO_MCP_BIN", binary, binaryBefore, binaryAfter, stable: binaryBefore === binaryAfter, rootRoute: "bun ./📜️script.ts dev mcp stdio codex", processPid: child.pid, exit, initialized, toolCount: tools.tools.length, closeSchema: close.inputSchema, goalsContents: goals.contents.length, goalsUtf8Bytes: Buffer.byteLength(JSON.stringify(goals.contents)), goalsSha256: hash(JSON.stringify(goals.contents)), mutations: 0, stderr };
    writeFileSync(join(ticket, "🗑️generated/ticket-retention-mcp-read-only.json"), JSON.stringify(result, null, 2));
    console.log(`[DEBUG] Actual source-bound MCP read-only transport: exit${exit}, ${tools.tools.length} tools, ${goals.contents.length} goals content; binary ${binaryAfter}`);
  } finally { child.stdin.end(); }
} else throw new Error(`Unknown retention mode: ${mode}`);
