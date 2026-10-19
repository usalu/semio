import { readFileSync } from "node:fs";
import { deepStrictEqual as equal } from "node:assert";
import { resolve, win32 } from "node:path";
import { createRequire } from "node:module";
import { Database } from "bun:sqlite";
const require = createRequire(resolve("package.json")), ts = require("typescript") as typeof import("typescript"), Ajv = require("ajv"), root = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/";
const schema = JSON.parse(readFileSync(root + "🗂️system/🧬️schema.json", "utf8")), corpus = JSON.parse(readFileSync(root + "🧪️tests/🌱️birth/🛂️host.json", "utf8")), text = readFileSync(root + "🗂️system/🟦️.ts", "utf8"), tree = ts.createSourceFile("system.ts", text, ts.ScriptTarget.Latest, true), names = ["readNativeFixedWindowsHostPrivilegesV1", "requireNativeFixedWindowsHostV1"];
const nodes = names.map(name => tree.statements.find(row => ts.isFunctionDeclaration(row) && row.name?.text === name));
if (nodes.some(row => !row)) throw Error("Actual authored host functions required");
const source = ts.transpileModule(nodes.map(row => row!.getText(tree).replace("export ", "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.None } }).outputText, db = new Database(":memory:"), ajv = new Ajv({ strict: true }), policy = schema.definitions.HostPrivilegeProtocol.const;
let cases = 0;
try {
  for (const row of corpus.cases) {
    equal(ajv.validate(schema.definitions.HostPrivilegeCase, row), true);
    const bytes = Buffer.alloc(4 + row.count * 12 - Number(row.truncated));
    bytes.writeUInt32LE(row.count);
    for (let index = 0; index < row.count; index++) { if (4 + index * 12 + 12 > bytes.length) break; bytes.writeBigUInt64LE(7n, 4 + index * 12); bytes.writeUInt32LE(row.attributes, 12 + index * 12); }
    const reader = new Function("contract", source + ";return readNativeFixedWindowsHostPrivilegesV1;")(schema);
    let actual = "refused";
    try { actual = reader(bytes, Buffer.from([7, 0, 0, 0, 0, 0, 0, 0])); } catch {}
    const oracle = db.query("SELECT CASE WHEN ?1>128 OR ?2 OR ?1>1 THEN 'refused' WHEN ?1=0 OR (?3 & 4)!=0 THEN 'absent' WHEN (?3 & 2)!=0 THEN 'enabled' ELSE 'available' END result").get(row.count, Number(row.truncated), row.attributes) as { result: string };
    equal(actual, oracle.result); equal(actual, row.expected); cases++;
  }
  for (const row of corpus.checks) {
    equal(ajv.validate(schema.definitions.HostPrivilegeCheckCase, row), true);
    let enabled = row.state === "enabled", adjustments = 0, tokenCloses = 0, libraries = 0;
    const symbols = { OpenProcessToken: (_process: bigint, access: number, token: Buffer) => { equal(access, policy.tokenAccess); token.writeBigInt64LE(5n); return 1; }, LookupPrivilegeValueW: (_system: unknown, _name: Buffer, luid: Buffer) => { luid.writeBigUInt64LE(7n); return 1; }, GetTokenInformation: (_token: bigint, kind: number, bytes: Buffer, _capacity: number, length: Buffer) => { equal(kind, policy.informationClass); if (row.state === "query-error") return 0; const count = Number(row.state !== "absent"); bytes.writeUInt32LE(count); if (count) { bytes.writeBigUInt64LE(7n, 4); bytes.writeUInt32LE(row.state === "removed" ? 4 : enabled ? 2 : 0, 12); } length.writeUInt32LE(4 + 12 * count); return 1; }, AdjustTokenPrivileges: () => { adjustments++; if (row.state === "adjust-refused") return 0; enabled = true; return 1; }, GetLastError: () => 0, SetLastError: () => {}, CloseHandle: (token: bigint) => { equal(token, 5n); tokenCloses++; return 1; } };
    const bindings = { contract: schema, win32, readIssuedResourceControlCapabilitiesV1: () => {}, requireNativeWindowsPublicationObserverRuntimeV1: () => {}, nativeFixedWindowsSystemV1: () => ({ system: "C:\\Windows\\System32" }), systemLibrary: () => ({ FFIType: {}, dlopen: () => ({ symbols, close: () => { libraries++; } }) }) }, owner = new Function(...Object.keys(bindings), source + ";return requireNativeFixedWindowsHostV1;")(...Object.values(bindings));
    let accepted = true;
    try { owner({}); } catch (error) { accepted = false; if (row.accepted) console.log("[DEBUG] Neutral modeled case refusal", row.state, String(error)); }
    const oracle = db.query("SELECT ? IN ('enabled','disabled') accepted").get(row.state) as { accepted: number };
    equal(accepted, Boolean(oracle.accepted)); equal(accepted, row.accepted); equal(adjustments, row.adjustments); equal(tokenCloses, 1); equal(libraries, 2); cases++;
  }
  equal(policy.elevation, false);
  console.log("[DEBUG] Current source Windows privilege neutral fixtures matched Ajv/SQLite:", cases, "; modeled FFI only, no native privilege or cache effects");
} finally { db.close(); }
