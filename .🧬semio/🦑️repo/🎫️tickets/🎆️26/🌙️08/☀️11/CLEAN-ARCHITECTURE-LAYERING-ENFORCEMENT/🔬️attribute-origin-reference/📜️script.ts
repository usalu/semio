import { readFile, mkdir, writeFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { createHash } from "node:crypto";

const output = resolve(process.argv[2] ?? "");
if (!process.argv[2]) throw new Error("Attribute reference requires a ticket-owned output directory");
await mkdir(output, { recursive: true });
const provider = join(output, process.platform === "win32" ? "attribute_provider.dll" : process.platform === "darwin" ? "libattribute_provider.dylib" : "libattribute_provider.so");
const cases = [
  { id: "builtin", imports: "", attribute: "#[test]" },
  { id: "explicit-test", imports: "use attribute_provider::test;", attribute: "#[test]" },
  { id: "renamed-test", imports: "use attribute_provider::rewrite as test;", attribute: "#[test]" },
  { id: "wildcard-test", imports: "use attribute_provider::*;", attribute: "#[test]" },
  { id: "literal-doc-with-import", imports: "use attribute_provider::doc;", attribute: '#[doc = "Portable scope."]\n#[test]' },
];
const operations: { args: string[]; status: number; stdout: string; stderr: string }[] = [];
let child: ReturnType<typeof Bun.spawn> | undefined;
const stop = () => child?.kill();
process.once("SIGINT", stop);
process.once("SIGTERM", stop);
async function run(args: string[]): Promise<(typeof operations)[number]> {
  console.log("[DEBUG] attribute reference start; " + args.join(" "));
  child = Bun.spawn(args, { cwd: output, stdout: "pipe", stderr: "pipe" });
  const timer = setInterval(() => console.log("[DEBUG] attribute reference live"), 10000);
  try {
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    const result = { args, status, stdout, stderr };
    operations.push(result);
    return result;
  } finally { clearInterval(timer); child = undefined; }
}
try {
  const providerSource = await readFile(join(import.meta.dir, "🏷️provider", "🦀️.rs"), "utf8");
  await writeFile(join(output, "provider.rs"), providerSource);
  const built = await run(["rustc", "--edition=2021", "--crate-name", "attribute_provider", "--crate-type", "proc-macro", "provider.rs", "-o", provider]);
  if (built.status !== 0) throw new Error("Owned attribute reference provider did not compile");
  const receipts: Record<string, unknown>[] = [];
  for (const row of cases) {
    const source = row.imports + "\n" + row.attribute + '\nfn original() { let _ = include_str!("input.txt"); }\n';
    const path = join(output, row.id + ".rs"), binary = join(output, row.id + (process.platform === "win32" ? ".exe" : ""));
    await writeFile(path, source);
    await writeFile(join(output, "input.txt"), "sealed-input");
    const built = await run(["rustc", "--edition=2021", "--crate-name", row.id.replaceAll("-", "_"), "--test", "--extern", "attribute_provider=" + provider, "--emit=dep-info=" + join(output, row.id + ".d") + ",link", path, "-o", binary]);
    const runtime = built.status === 0 ? await run([binary, "--nocapture"]) : null;
    const dependencies = built.status === 0 ? await readFile(join(output, row.id + ".d"), "utf8") : null;
    receipts.push({ id: row.id, sourceSha256: createHash("sha256").update(source).digest("hex"), buildStatus: built.status, runtimeStatus: runtime?.status ?? null, runtimeStdout: runtime?.stdout ?? null, stderr: built.stderr, dependencies });
  }
  await writeFile(join(output, "attribute-origin-reference.json"), JSON.stringify({ providerSha256: createHash("sha256").update(providerSource).digest("hex"), platform: process.platform, cases: receipts, operations }, null, 2) + "\n");
} finally { process.off("SIGINT", stop); process.off("SIGTERM", stop); }
