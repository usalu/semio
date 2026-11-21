/** 🗂️ Relocate profile-window laws out of the schema-4 library test closure. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";

type Row = [string, string | null, string];
const base = process.argv[2];
assert.ok(base?.endsWith("/🧱️base"));
const staging = join(import.meta.dir, "🗑️generated", "pdf-registered-feature-overlay.json");
const sources = new Map((await Bun.file(staging).json() as Row[]).map(row => [row[0], row]));
const change = async (path: string, transform: (text: string) => string) => {
    const live = await Bun.file(path).text();
    const before = createHash("sha256").update(live).digest("hex");
    const staged = sources.get(path);
    if (staged) assert.equal(staged[1], before, path);
    const candidate = transform(staged?.[2] ?? live);
    assert.notEqual(candidate, staged?.[2] ?? live, path);
    sources.set(path, [path, before, candidate]);
};

const schema = join(base, "🧬️schema");
const manifest = join(schema, "📦️packages/🦀️rust/Cargo.toml");
const mirror = join(dirname(manifest), "🪞️mirror/🦀️.rs");
const root = join(schema, "🧪️tests/🔬️profile-windows/🦀️.rs");
assert.ok(!await Bun.file(root).exists());
const mounts: [string,string][] = [];
const profiles = new Map([["🧱️base","base"],["🗄️a","a"],["📐️e","e"],["⚕️h","h"],["♿️ua","ua"],["🧾️vt","vt"],["🖨️x","x"]]);
const re = /^( +)#\[cfg\(test\)\]\n\1#\[path = "([^"]*🪟️main\/🧪️tests\/🔬️unit\/🦀️\.rs)"\]\n\1pub mod tests;\n/gmu;

await change(mirror, body => body.replace(re, (_whole, _indent, raw:string) => {
    const filename = relative(process.cwd(), resolve(dirname(mirror), raw));
    const profile = [...profiles].find(([folder]) => filename.includes(`/${folder}/`))?.[1];
    assert.ok(profile, raw);
    const mode = filename.includes("/✏️editor/") ? "editor" : "viewer";
    mounts.push([`${profile}_${mode}`, filename]);
    return "";
}));
assert.equal(mounts.length, 14);
assert.equal(new Set(mounts.map(([name]) => name)).size, 14);
for (const [name, file] of mounts) {
    await change(file, body => {
        const render = "    let node = render(&document).expect(";
        assert.equal(body.split(render).length, 2, file);
        const original = body.replace(render, "    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();\n" + render);
        if (name.endsWith("_viewer")) {
            const assertion = "    assert_eq!(node.children.len(), 1);\n";
            const info = "    let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect(\"project the rendered page window\");";
            assert.ok(original.includes(assertion + info), file);
            const projection = `    let observed_children = node.children.len();
${info}
    assert_eq!(observed_children, 1);`;
            return original.replace(assertion + info, projection);
        }
        const start = original.indexOf("    let surface = first_surface(&node)");
        const end = original.indexOf("\n}",start);
        assert.ok(start >= 0 && end > start, file);
        const observed = original.slice(start,end).replace("first_surface(&node)", "first_surface(tree)")
            .split("\n").map(line => "    " + line).join("\n");
        return original.slice(0,start) + `    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node), |tree| {
${observed}
    });` + original.slice(end);
    });
}

await change(manifest, body => {
    const next = relative(dirname(manifest),root).replaceAll("\\", "/");
    assert.ok(!body.includes('name = "profile_windows"'));
    return body.trimEnd() + `\n\n[[test]]\nname = "profile_windows"\npath = "${next}"\n`;
});
const harness = '//! 🪟️ Exercises the authored PDF 1.7 editor and viewer windows under a separate Cargo test target.\n\n'
    + 'pub use semio_s_artifact_stdio_pdf_schema_4::*;\n\n'
    + mounts.map(([name, file]) => `#[path = "${relative(dirname(root), file).replaceAll("\\","/")}"]\nmod ${name};\n`).join("\n");
sources.set(root, [root, null, harness]);
const ordered = [...sources.values()].sort(([first], [second]) => first.localeCompare(second, "en"));
for (const [path] of ordered) assert.ok(resolve(path).normalize("NFD").length <= 256, path);
await Bun.write(staging, JSON.stringify(ordered, null, 2) + "\n");
console.log(`[DEBUG] Relocated ${mounts.length} window law modules into profile_windows; combined roster has ${ordered.length} paths.`);
