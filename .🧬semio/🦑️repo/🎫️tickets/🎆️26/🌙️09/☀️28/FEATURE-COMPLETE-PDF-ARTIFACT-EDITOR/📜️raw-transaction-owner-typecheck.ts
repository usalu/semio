/** 🧪 Type-checks the staged causal-before-transaction merge against the governed WASI Store. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const cache = resolve(process.argv[2]);
const host = resolve(cache, "../../..", "prototype/build");
const generated = join(import.meta.dir, "🗑️generated");
const out = join(generated, "transaction-types");
mkdirSync(out, { recursive: true });
const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine";
const bySource = new Map<string, string>();
for (const name of ["raw-reactor-inbound-overlay.json", "checkpoint-atomic-store-overlay.json"]) {
    const input = JSON.parse(readFileSync(join(generated, name), "utf8")) as { candidate: Record<string, string> };
    for (const [path, text] of Object.entries(input.candidate)) {
        if (text.includes("prepare_verified_atomic_edit") || path.endsWith("/⚙️engine/🦀️.rs"))
            bySource.set(resolve(path), text);
    }
}
const materialized = new Map<string, string>();
function materialize(file: string): string {
    if (materialized.has(file)) return materialized.get(file)!;
    const result = join(out, "source-" + materialized.size + ".rs");
    materialized.set(file, result);
    const text = bySource.get(file) ?? readFileSync(file, "utf8");
    const body = text.replace(/(#\[path\s*=\s*)"([^"]+)"/g, (_, prefix, relative) => {
        const child = resolve(dirname(file), relative);
        const includesStage = bySource.has(child) || [...bySource.keys()].some(path => path.startsWith(dirname(child) + "/"));
        return prefix + JSON.stringify(includesStage ? materialize(child) : child);
    }).replace(/(include(?:_str|_bytes)?!\(\s*)"([^"]+)"/g, (_, prefix, relative) =>
        prefix + JSON.stringify(resolve(dirname(file), relative)));
    writeFileSync(result, body);
    return result;
}

function directories(root: string): string[] {
    return readdirSync(root).flatMap(pkg => readdirSync(join(root, pkg)).map(key => join(root, pkg, key, "out")).filter(dir => existsSync(dir)));
}
const folders = [...directories(cache), ...directories(host)];
const artifacts = new Map<string, string[]>();
for (const dir of folders) {
    for (const filename of readdirSync(dir)) {
        const found = /^lib([\w_]+)-[a-f0-9]+\.(rmeta|so|dylib|dll)$/.exec(filename);
        if (!found || (found[2] === "rmeta" && !dir.startsWith(cache + "/"))) continue;
        const paths = artifacts.get(found[1]!) ?? [];
        paths.push(join(dir, filename));
        artifacts.set(found[1]!, paths);
    }
}
function released(name: string, exact?: ReadonlySet<string>): string {
    const candidates = artifacts.get(name) ?? [];
    const matched = exact ? candidates.filter(entry => exact.has(basename(entry).replace(/^lib/, "").replace(/\.[^.]+$/, ""))) : candidates;
    return matched.sort((left, right) => statSync(right).mtimeMs - statSync(left).mtimeMs)[0] ??
        assert.fail("No exact governed metadata for " + name);
}
type Spec = string | { package?: string; path?: string; optional?: boolean };
type Manifest = { package: { name: string; version?: string; edition?: string }; lib: { path: string; name?: string }; dependencies: Record<string, Spec> };
function readManifest(file: string): Manifest {
    return Bun.TOML.parse(readFileSync(file, "utf8")) as Manifest;
}
function compile(owner: string, overrides: Record<string, string>): string {
    const manifest = resolve(store, owner, "📦️packages/🦀️rust/Cargo.toml");
    const folder = dirname(manifest);
    const pkg = readManifest(manifest);
    const name = pkg.lib.name ?? pkg.package.name.replaceAll("-", "_");
    const known = JSON.parse(readFileSync(released(name) + ".semio-deps", "utf8"));
    const deps = new Set<string>(known.dependencies.map(([key]: [string, string]) => key));
    const output = join(out, "lib" + name + ".rmeta");
    const args = ["--target=wasm32-wasip2", "--edition=" + (pkg.package.edition ?? "2021"), "--emit=metadata",
        "--crate-type=lib", "--crate-name=" + name, "-Cmetadata=raw-transaction-verified", "-o", output,
        materialize(resolve(folder, pkg.lib.path))];
    for (const dir of [out, ...folders]) args.push("-L", "dependency=" + dir);
    for (const [key, spec] of Object.entries(pkg.dependencies)) {
        if (typeof spec !== "string" && spec.optional) continue;
        const declared = typeof spec !== "string" && spec.path ? readManifest(resolve(folder, spec.path, "Cargo.toml")) : undefined;
        const packageName = declared?.package.name ?? (typeof spec === "string" ? key : spec.package ?? key);
        let library = declared?.lib?.name ?? packageName.replaceAll("-", "_");
        if (library === "semio_framework_pack") library = "pack";
        if (library === "semio_framework_replication") library = "protocol";
        const alias = key === packageName ? library : key.replaceAll("-", "_");
        const dependency = overrides[library] ?? artifacts.get(library)?.find(entry =>
            deps.has(basename(entry).replace(/^lib/, "").replace(/\.[^.]+$/, "")));
        if (dependency) args.push("--extern", alias + "=" + dependency);
    }
    const response = join(out, name + ".rsp");
    writeFileSync(response, args.join("\n") + "\n");
    const result = Bun.spawnSync(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + response],
        { env: { ...process.env, CARGO_MANIFEST_DIR: folder, CARGO_PKG_NAME: pkg.package.name, CARGO_PKG_VERSION: pkg.package.version ?? "0.1.0" },
          stderr: "inherit", stdout: "inherit" });
    assert.equal(result.exitCode, 0, name);
    console.log("[DEBUG] Checked current WASI", pkg.package.name);
    return output;
}
const bundle = compile("🧳️bundle", {});
compile("", { semio_os_store_engine_bundle: bundle });
