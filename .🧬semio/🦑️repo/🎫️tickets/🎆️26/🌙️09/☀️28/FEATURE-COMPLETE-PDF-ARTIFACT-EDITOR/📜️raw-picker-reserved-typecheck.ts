/** 🔬 Checks the real reserved job declaration and PDF worker against governed WASI metadata. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const ticket = import.meta.dir;
const out = join(ticket, "🗑️generated", "reserved-types");
const overlay = JSON.parse(readFileSync(join(ticket, "🗑️generated", "raw-picker-mounted-overlay.json"), "utf8")).candidate as Record<string, string>;
const cache = resolve(process.argv[2]);
const prototype = resolve(cache, "..", "..", "..", "prototype", "build");
mkdirSync(out, { recursive: true });

function directories(root: string): string[] {
    return readdirSync(root).flatMap(name => {
        const packageDir = join(root, name);
        return readdirSync(packageDir, { withFileTypes: true }).flatMap(unit => {
            const dir = join(packageDir, unit.name, "out");
            return unit.isDirectory() && existsSync(dir) ? [dir] : [];
        });
    });
}
const applicationDirectories = directories(cache);
const search = [...applicationDirectories, ...directories(prototype), out,
    resolve(ticket, "🗑️generated", "picker-carrier"), resolve(ticket, "🗑️generated", "raw-png-edit"),
    resolve(ticket, "🗑️generated", "raw-png", "type-" + basename(resolve(cache, "..", "..", "..")))];
const wasm = new Map<string, string[]>();
const hostMacros = new Map<string, string[]>();
for (const dir of search.filter(path => path !== out && existsSync(path))) {
    for (const file of readdirSync(dir)) {
        const parsed = /^lib([a-z0-9_]+)-[a-f0-9]+\.(rmeta|dylib|so|dll)$/.exec(file);
        if (!parsed) continue;
        const table = parsed[2] === "rmeta" ? wasm : hostMacros;
        if (table === wasm && !dir.startsWith(cache + "/")) continue;
        const matches = table.get(parsed[1]) ?? [];
        matches.push(join(dir, file));
        table.set(parsed[1], matches);
    }
}
function exact(name: string): string {
    const lib = name === "semio_framework_pack" ? "pack" : name;
    const paths = wasm.get(lib) ?? hostMacros.get(lib) ?? [];
    assert.equal(paths.length, 1, `${name} ${paths.join(", ")}`);
    return paths[0]!;
}
function write(name: string, text: string): string {
    const file = join(out, name);
    writeFileSync(file, text);
    return file;
}
const physical = new Map(Object.entries(overlay).map(([path, body]) => [resolve(path), body]));
const materialized = new Map<string, string>();
function materialize(file: string, entry = false): string {
    if (materialized.has(file)) return materialized.get(file)!;
    if (!entry && !physical.has(file)) {
        const dir = dirname(file) + "/";
        if (![...physical.keys()].some(path => path.startsWith(dir))) return file;
    }
    const target = join(out, `stage-${materialized.size}.rs`);
    materialized.set(file, target);
    let code = physical.get(file) ?? readFileSync(file, "utf8");
    code = code.replace(/(#\[path\s*=\s*)"([^"]+)"/g, (_m, prefix, suffix) => {
        const path = resolve(dirname(file), suffix);
        return prefix + JSON.stringify(materialize(path));
    });
    code = code.replace(/(include(?:_str|_bytes)?!\(\s*)"([^"]+)"/g, (_m, prefix, suffix) => {
        const path = resolve(dirname(file), suffix);
        assert.ok(existsSync(path) || physical.has(path), path);
        return prefix + JSON.stringify(prefix.startsWith("include!") ? materialize(path, true) : path);
    });
    writeFileSync(target, code);
    return target;
}

function compile(name: string, entry: string, dependencies: Record<string, string>, manifest?: string): string {
    const target = join(out, `lib${name}.rmeta`);
    const args = ["--target=wasm32-wasip2", "--edition=2024", "--crate-name", name, "--crate-type=lib",
        "--emit=metadata", "-Cmetadata=original-picker-mount", entry, "-o", target];
    for (const dir of search) args.push("-L", "dependency=" + dir);
    for (const [alias, file] of Object.entries(dependencies)) args.push("--extern", alias + "=" + file);
    const response = write(name + ".rsp", args.join("\n") + "\n");
    const env = manifest ? { ...process.env, CARGO_MANIFEST_DIR: dirname(manifest), CARGO_PKG_NAME: name.replaceAll("_", "-"), CARGO_PKG_VERSION: "0.1.0" } : process.env;
    const result = Bun.spawnSync(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
        "rustc", "@" + response], { env, stdout: "inherit", stderr: "inherit" });
    assert.equal(result.exitCode, 0, name);
    console.log("[DEBUG] Checked", name);
    return target;
}

if (process.argv.includes("--live-kernel")) {
    const rawWire = resolve(ticket, "🗑️generated", "rm-wire", "libsemio_framework_replication_command_ingress.rmeta");
    assert.ok(existsSync(rawWire));
    search.push(dirname(rawWire));
    const replication = compile("picker_replication", write("replication.rs", `
pub use released_replication::*;
pub mod wire {
    pub use released_replication::wire::*;
    pub mod command_ingress { pub use raw_wire::*; }
}`), { released_replication: exact("protocol"), raw_wire: rawWire });
    const kernelManifest = resolve("🧰️framework/🔨️modules/🎠️kernel/📦️packages/🦀️rust/Cargo.toml");
    const kernel = Bun.TOML.parse(readFileSync(kernelManifest, "utf8")) as {
        lib: { path: string };
        dependencies: Record<string, string | { package?: string; optional?: boolean; path?: string }>;
    };
    const dependencies = Object.fromEntries(Object.entries(kernel.dependencies).filter(([, spec]) =>
        typeof spec === "string" || !spec.optional).map(([alias, spec]) => {
        const lib = typeof spec === "object" && spec.path
            ? Bun.TOML.parse(readFileSync(resolve(dirname(kernelManifest), spec.path, "Cargo.toml"), "utf8")) as { lib?: { name?: string } }
            : undefined;
        const name = (lib?.lib?.name ?? (typeof spec === "string" ? alias : spec.package ?? alias)).replaceAll("-", "_");
        return [name === "semio_framework_pack" ? "pack" : alias.replaceAll("-", "_"), alias === "replication" ? replication : exact(name)];
    }));
    compile("semio_framework_kernel", resolve(dirname(kernelManifest), kernel.lib.path), dependencies, kernelManifest);
    process.exit(0);
}

const carrier = resolve(ticket, "🗑️generated", "picker-carrier", "libpicker_carrier.rmeta");
assert.ok(existsSync(carrier));
const media = compile("picker_media", write("media.rs", `
pub use released_media::*;
pub mod original_media {
    pub use picker_carrier::original_media::*;
}`), { released_media: exact("semio_plugin_decl_media"), picker_carrier: carrier });
if (process.argv.includes("--live-picker-owners")) {
    const reactorManifest = resolve(plugin, "⚛️reactor/🔌️app/📦️packages/🦀️rust/Cargo.toml");
    const reactor = Bun.TOML.parse(readFileSync(reactorManifest, "utf8")) as {
        lib: { path: string };
        package: { edition?: string };
        dependencies: Record<string, string | { package?: string; optional?: boolean; path?: string }>;
    };
    const dependencies = Object.fromEntries(Object.entries(reactor.dependencies).filter(([, spec]) =>
        typeof spec === "string" || !spec.optional).map(([alias, spec]) => {
        const lib = typeof spec === "object" && spec.path
            ? Bun.TOML.parse(readFileSync(resolve(dirname(reactorManifest), spec.path, "Cargo.toml"), "utf8")) as { lib?: { name?: string } }
            : undefined;
        const name = (lib?.lib?.name ?? (typeof spec === "string" ? alias : spec.package ?? alias)).replaceAll("-", "_");
        return [name === "semio_framework_pack" ? "pack" : alias.replaceAll("-", "_"), name === "semio_plugin_decl_media" ? media : exact(name)];
    }));
    compile("semio_plugin_reactor_app", resolve(dirname(reactorManifest), reactor.lib.path), dependencies, reactorManifest);
    compile("picker_mounted_close", resolve(plugin, "🧵️instance/📄️docs/📥️staging/🧱️components/🎟️picker-publication.rs"), {
        semio_framework_job: exact("semio_framework_job"), semio_framework_value: exact("semio_framework_value"),
        semio_plugin_decl_emission: exact("semio_plugin_decl_emission"), semio_plugin_decl_media: media,
        semio_plugin_ownership: exact("semio_plugin_ownership"),
    });
    process.exit(0);
}

const appManifest = resolve(plugin, "📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml");
const manifest = Bun.TOML.parse(readFileSync(appManifest, "utf8")) as {
    lib: { path: string };
    dependencies: Record<string, string | { package?: string; optional?: boolean; path?: string }>;
};
const externs = Object.fromEntries(Object.entries(manifest.dependencies).filter(([_alias, declaration]) =>
    typeof declaration === "string" || !declaration.optional).map(([alias, declaration]) => {
    const config = typeof declaration === "object" && declaration.path
        ? Bun.TOML.parse(readFileSync(resolve(dirname(appManifest), declaration.path, "Cargo.toml"), "utf8")) as { lib?: { name?: string } }
        : undefined;
    const name = (config?.lib?.name ?? (typeof declaration === "string" ? alias : declaration.package ?? alias)).replaceAll("-", "_");
    return [name === "semio_framework_pack" ? "pack" : alias.replaceAll("-", "_"), name === "semio_plugin_decl_media" ? media : exact(name)];
}));
const actualApp = compile("semio_plugin_decl_app", materialize(resolve(dirname(appManifest), manifest.lib.path), true), externs, appManifest);
if (process.argv.includes("--declaration-only")) process.exit(0);

const facade = compile("picker_plugin", write("facade.rs", `
pub use actual_app::ArtifactApp;
pub use actual_app::app_emission::EphemeralEmit;
pub use emission::Emit;
pub use views::{NoConfigMutation, NoDraftMutation};
pub use job as __job;
pub use value as __value;
pub use media::original_media;
pub mod app {
    pub use actual_app::tool_contract::{ArtifactReservedToolInput, ArtifactReservedToolJobRequest, ArtifactReservedWork};
    pub use emission::ArtifactMutationOutcome;
    pub use ownership::bounded_ownership::original_lifecycle_delta;
}`), { actual_app: actualApp, emission: exact("semio_plugin_decl_emission"), views: exact("semio_plugin_decl_views"),
        ownership: exact("semio_plugin_ownership"), job: exact("semio_framework_job"), value: exact("semio_framework_value"), media });

const worker = resolve(base, "✏️editor/📄️document/🖼️image-authoring/🧵️reserved-picker.rs");
const linkedFormat = resolve(ticket, "🗑️generated", "raw-png-edit", "libpdf_raw_png_edit.rmeta");
compile("pdf_picker_reserved_format", write("verify.rs", `
pub use mutations::standards::v1_7::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
pub use format::picker_testing::PngFormatWork;
#[path = "${materialize(worker)}"] pub mod worker;
`), { snapshot: exact("semio_s_artifact_stdio_pdf_snapshot_2"), mutations: exact("semio_s_artifact_stdio_pdf_mut_family_8"),
        semio_plugin: facade, format: linkedFormat });
