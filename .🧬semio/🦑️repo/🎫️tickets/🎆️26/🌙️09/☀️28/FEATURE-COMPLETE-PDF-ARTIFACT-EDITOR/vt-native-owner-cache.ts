/** 🗄️ Resolves the real cached PDF and plugin owner metadata for the focused VT hook proof. */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync } from "node:fs";
import { resolve } from "node:path";

const out = resolve(import.meta.dirname, "🗑️generated", "vt-load");
const deps = resolve(out, "deps");
const receipts = resolve(".cache/b/r/m/u/e");
const { casRootV1, casObjectPathV1 } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts"));
const events = (await Bun.file(".cache/b/r/m/u/x/pe73647.jsonl").text()).trim().split("\n").map(line => JSON.parse(line));
const eligible = [...new Set(events.filter(row => ["hit", "publish"].includes(row.kind)).map(row => row.key))];
const known = new Map<string, { name: string; sha256: string; bytes: number }>();
const externs = new Map<string, string>();
const requires = new Set<string>();
const requiredRoots = ["semio_plugin", "semio_s_artifact_stdio_pdf_mut_family_8", "semio_s_artifact_stdio_pdf_snapshot_2"];
for (const key of eligible) {
    if (typeof key !== "string" || !/^[a-f0-9]{64}$/.test(key)) continue;
    const prefix = resolve(receipts, key.slice(0, 2), key);
    if (!existsSync(prefix)) continue;
    for (const name of readdirSync(prefix).filter(item => item.endsWith(".json"))) {
        const row = await Bun.file(resolve(prefix, name)).json();
        for (const item of row.outputs ?? []) known.set(item.name, item);
        if (requiredRoots.includes(row.crate)) {
            const root = row.outputs?.find(item => item.name.endsWith(".rmeta"));
            if (root) externs.set(row.crate, root.name);
        }
    }
}

const source = (item: {sha256: string}) => Bun.file(casObjectPathV1(casRootV1(), item.sha256));
for (const name of requiredRoots) {
    const file = externs.get(name);
    if (!file) throw Error(`Missing native receipt for ${name}`);
    const record = known.get(file + ".semio-deps");
    if (!record || !(await source(record).exists())) throw Error(`Missing exact native dependency report for ${file}`);
    const body = await source(record).json();
    for (const [dependency] of body.dependencies) requires.add(dependency);
    console.log(`[DEBUG] owner metadata ${name} ${file} ${body.svh}`);
}

const sysroot = Bun.spawnSync({
    cmd: ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--print", "target-libdir"],
});
if (sysroot.exitCode !== 0) throw Error(sysroot.stderr.toString());
const system = new Set(readdirSync(sysroot.stdout.toString().trim()));
const selected = new Map<string, { name: string; sha256: string; bytes: number }>();
let absent: string[] = [];
for (const identity of requires) {
    const possible = [".rmeta", ".dylib", ".so", ".dll"].map(ext => "lib" + identity + ext);
    if (possible.some(name => system.has(name)) || system.has("lib" + identity + ".rlib")) continue;
    const item = possible.map(name => known.get(name)).find(Boolean);
    if (item && await source(item).exists()) selected.set(item.name, item);
    else absent.push(identity);
}
for (const file of externs.values()) {
    const item = known.get(file);
    if (!item || !(await source(item).exists())) absent.push(file);
    else selected.set(file, item);
}
if (absent.length) {
    const missing = new Map(absent.flatMap(identity =>
        [".rmeta", ".dylib", ".so", ".dll"].map(ext => ["lib" + identity + ext, identity] as const)));
    for (const prefix of readdirSync(receipts)) {
        const shard = resolve(receipts, prefix);
        for (const digest of readdirSync(shard).filter(name => /^[a-f0-9]{64}$/.test(name))) {
            const group = resolve(shard, digest);
            for (const file of readdirSync(group).filter(name => name.endsWith(".json"))) {
                const row = await Bun.file(resolve(group, file)).json();
                for (const item of row.outputs ?? []) {
                    const identity = missing.get(item.name);
                    if (!identity || !(await source(item).exists())) continue;
                    selected.set(item.name, item);
                    absent = absent.filter(value => value !== identity);
                    for (const [name, value] of missing) if (value === identity) missing.delete(name);
                }
            }
        }
    }
}
console.log(`[DEBUG] ${selected.size} exact metadata names available, ${absent.length} missing; bytes ${[...selected.values()].reduce((n, item) => n + item.bytes, 0)}`);
if (absent.length) {
    console.log(`[DEBUG] missing native metadata ${absent.sort().join(", ")}`);
    if (!process.argv.includes("--available-only")) process.exit(2);
}
if (!process.argv.includes("--restore")) process.exit(0);
mkdirSync(deps, { recursive: true });
for (const item of selected.values()) {
    const bytes = await source(item).arrayBuffer();
    if (createHash("sha256").update(new Uint8Array(bytes)).digest("hex") !== item.sha256) throw Error(item.name);
    await Bun.write(resolve(deps, item.name), bytes);
}
await Bun.write(resolve(out, "externs.json"), JSON.stringify(Object.fromEntries(externs)) + "\n");
