/** 🎞️ Exercise checked registered PDF Save in the existing independent rotation and reopen law. */
import assert from "node:assert/strict";
import {resolve} from "node:path";
import {readFileSync,writeFileSync} from "node:fs";
const path = process.argv[2];
assert.ok(path?.endsWith("/✏️editor/🧪️tests/🔬️unit/🦀️.rs"));
const source = readFileSync(path, "utf8");
const old = `    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("registered PDF editor saves").expect("natural PDF media is present");
    assert_eq!(artifact.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let bytes = artifact.data;
`;
assert.equal(source.split(old).length, 2);
const body = source.replace(old, await Bun.file(resolve(import.meta.dir,"📜️registered-pdf-save-law.rs")).text());
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const pkg = resolve(path,"../../../../🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value/📦️packages/🦀️rust");
for (const os of ["linux","macos","windows","wasi"]) {
    const cfg = [`target_os="${os}"`,os === "wasi" ? 'target_arch="wasm32"' : os === "macos" ? 'target_arch="aarch64"' : 'target_arch="x86_64"',os === "wasi" ? 'target_family="wasm"' : os === "windows" ? "windows" : "unix"];
    const row = await measureRustCrate(process.cwd(),resolve(pkg,"🦀️.rs"),true,new Map([[resolve(path),body]]),pkg,cfg);
    assert.ok(row.paths.some((file:string)=>resolve(file)===resolve(path)),`Native Save test source is outside ${os} closure`);
    assert.ok(row.files <= 32 && row.lines <= 4000 && row.bytes <= 262144,`Source limit refused ${os} ${row.files} ${row.lines} ${row.bytes}`);
    console.log("[DEBUG]",os,row.files,row.lines,row.bytes);
}
assert.equal(readFileSync(path,"utf8"),source);
writeFileSync(path,body);
