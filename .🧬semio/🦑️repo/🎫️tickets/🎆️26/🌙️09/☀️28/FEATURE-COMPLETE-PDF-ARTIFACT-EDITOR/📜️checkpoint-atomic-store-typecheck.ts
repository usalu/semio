/** 🔬 Type-check the trusted replay implementation using the new native Store engine and exact grouped ABI. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const generated = join(import.meta.dir, "🗑️generated");
const linked = process.argv.includes("--linked");
const out = join(generated, linked ? "checkpoint-codec-linked" : "checkpoint-codec");
const cache = resolve(process.argv[2]);
mkdirSync(out, { recursive: true });

const directories = readdirSync(cache).flatMap(name => {
    const pkg = join(cache, name);
    return readdirSync(pkg, { withFileTypes: true }).flatMap(unit => {
        const dir = join(pkg, unit.name, "out");
        return unit.isDirectory() && existsSync(dir) ? [dir] : [];
    });
});
const artifacts = new Map<string, string[]>();
for (const dir of directories) {
    for (const file of readdirSync(dir)) {
        const match = /^lib(\w+)-[a-f0-9]+\.(rmeta|so|dll|dylib)$/.exec(file);
        if (!match) continue;
        const items = artifacts.get(match[1]!) ?? [];
        items.push(join(dir, file));
        artifacts.set(match[1]!, items);
    }
}
function exact(name: string, matches: Set<string> = new Set()): string {
    const paths = artifacts.get(name) ?? [];
    const selected = paths.length <= 1 ? paths : paths.filter(path => matches.has(path.slice(path.lastIndexOf("/lib") + 4).split(".")[0]!));
    assert.equal(selected.length, 1, `${name} ${selected.join(", ")}`);
    return selected[0]!;
}
function compile(crate: string, input: string, deps: Record<string, string>, env = process.env): string {
    const result = join(out, `lib${crate}.${linked ? "rlib" : "rmeta"}`);
    const flags = ["--edition=2021", "--crate-name", crate, "--crate-type=lib", linked ? "--emit=link" : "--emit=metadata",
        "-Cmetadata=checkpoint-trusted-replay", "-o", result, input];
    for (const dir of [out, ...directories]) flags.push("-L", "dependency=" + dir);
    for (const [name, file] of Object.entries(deps)) {
        flags.push("--extern", name + "=" + file);
        const archive = file.endsWith(".rmeta") ? file.slice(0, -5) + "rlib" : "";
        if (linked && archive && existsSync(archive)) flags.push("--extern", name + "=" + archive);
    }
    const response = join(out, `${crate}.rsp`);
    writeFileSync(response, flags.join("\n") + "\n");
    const command = Bun.spawnSync(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
        "rustc", "@" + response], { env, stdout: "inherit", stderr: "inherit" });
    assert.equal(command.exitCode, 0, crate);
    return result;
}
function create(name: string, source: string): string {
    const file = join(out, name);
    writeFileSync(file, source);
    return file;
}
const engine = compile("checkpoint_engine", create("engine.rs", `
pub use released_engine::*;
pub use released_bundle::AtomicStoreReceive;
`), { released_engine: exact("semio_os_store_engine"), released_bundle: exact("semio_os_store_engine_bundle") });

const wire = readFileSync(join(import.meta.dir, "raw-hub-atomic-group.rs"), "utf8");
const start = wire.indexOf("pub enum CheckpointReplayGroup");
const end = wire.indexOf("\n}", start) + 2;
const signature = wire.match(/pub fn decode_checkpoint_replay\(bytes: &\[u8\], document: &str\) -> Result<Option<Vec<CheckpointReplayGroup>>, &'static str>/)?.[0];
assert.ok(start >= 0 && end > start && signature);
const spr = linked ? exact("semio_os_spr")
: compile("checkpoint_spr", create("spr.rs", `
pub use released_spr::*;
pub mod atomic_hub {
    use super::{MutationEnvelope, VerifiedAtomicEdit};
    ${wire.slice(start, end)}
    ${signature} { let _ = (bytes, document); unreachable!("grouped framing is exercised by the independent linked transport law") }
}
`), { released_spr: exact("semio_os_spr") });

const raw = JSON.parse(readFileSync(join(generated, "checkpoint-atomic-store-overlay.json"), "utf8")).candidate as Record<string, string>;
const source = new Map(Object.entries(raw).map(([path, body]) => [resolve(path), body]));
const memo = new Map<string, string>();
function materialize(path: string): string {
    if (memo.has(path)) return memo.get(path)!;
    const output = join(out, `stage-${memo.size}.rs`);
    memo.set(path, output);
    let body = source.get(path) ?? readFileSync(path, "utf8");
    body = body.replace(/(#\[path\s*=\s*)"([^"]+)"/g, (_, stem, name) => {
        const target = resolve(dirname(path), name);
        return stem + JSON.stringify(source.has(target) || [...source.keys()].some(x => x.startsWith(dirname(target) + "/"))
            ? materialize(target) : target);
    });
    body = body.replace(/(include(?:_str|_bytes)?!\(\s*)"([^"]+)"/g, (_, stem, name) =>
        stem + JSON.stringify(resolve(dirname(path), name)));
    writeFileSync(output, body);
    return output;
}
const file = resolve(store, "📦️codec/🏭️factory/📦️packages/🦀️rust/Cargo.toml");
const config = Bun.TOML.parse(readFileSync(file, "utf8")) as {
    lib: { path: string }; dependencies: Record<string, string | { path?: string; package?: string; optional?: boolean }>;
};
const identity = JSON.parse(readFileSync(exact("semio_os_store_codec_factory") + ".semio-deps", "utf8")) as {
    dependencies: [string, string][];
};
const expected = new Set(identity.dependencies.map(([name]) => name));
const external = Object.fromEntries(Object.entries(config.dependencies).filter(([, spec]) => typeof spec === "string" || !spec.optional)
    .map(([alias, spec]) => {
        const lib = typeof spec === "object" && spec.path
            ? Bun.TOML.parse(readFileSync(resolve(dirname(file), spec.path, "Cargo.toml"), "utf8")) as { lib?: { name?: string } }
            : undefined;
        let crate = (lib?.lib?.name ?? (typeof spec === "string" ? alias : spec.package ?? alias)).replaceAll("-", "_");
        if (crate === "semio_framework_pack") crate = "pack";
        const picked = crate === "semio_os_spr" ? spr : crate === "semio_os_store_engine" ? engine : exact(crate, expected);
        return [alias === "semio-framework-pack" ? "pack" : alias.replaceAll("-", "_"), picked];
    }));
const env = { ...process.env, CARGO_MANIFEST_DIR: dirname(file), CARGO_PKG_NAME: "semio-os-store-codec-factory", CARGO_PKG_VERSION: "0.1.0" };
compile("semio_os_store_codec_factory", materialize(resolve(dirname(file), config.lib.path)), external, env);
console.log("[DEBUG] Trusted codec", linked ? "compiled and linked" : "type-checked", "against the native Store and grouped framing ABI");
