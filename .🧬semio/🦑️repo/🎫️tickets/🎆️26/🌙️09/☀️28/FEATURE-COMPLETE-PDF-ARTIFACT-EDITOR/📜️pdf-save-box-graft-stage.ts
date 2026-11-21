/** 📐 Stages bounded original PDF page box, rotation and UserUnit grafts. */
import assert from "node:assert/strict";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
const path = join(process.argv[2],"✏️editor","💾️native-save","🪡️graft.rs");
const before = await Bun.file(path).text();
const geometry = await Bun.file(join(import.meta.dir,"📜️pdf-save-box-graft-impl.rs")).text();
function exact(source:string,anchor:string,insert:string) {
    assert.equal(source.split(anchor).length,2,anchor);
    return source.replace(anchor,insert);
}
let source=exact(before,"//! 🪡️ Builds only the paid COS replacements for an existing page rotation and document Info.","//! 🪡️ Builds only the paid COS replacements for an existing page's geometry and document Info.");
source=exact(source,"PdfInfo, PdfObject, PdfSnapshot","PdfInfo, PdfObject, PdfPage, PdfSnapshot");
source=exact(source,"Catalog, Pages, Rotation, Info","Catalog, Pages, Rotation, Geometry, Info");
const start = '            let mut allocated = 0;';
const end = '            self.seen[index] = true;';
const code = `            let changes = patch_page(page, original, pages, old)?;
            let mut allocated = 0;
            if changes.iter().any(Option::is_some) {
                if self.rows.page.is_some() { return Err(GraftRefusal::Geometry); }
                if original.len() > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
                let mut count = original.len();
                let mut new_bytes = 0;
                for (key, value) in changes.iter().flatten() {
                    let mut entries = original.iter().filter(|entry| entry.key == *key);
                    let present = entries.next().is_some();
                    if entries.next().is_some() { return Err(GraftRefusal::Geometry); }
                    match value {
                        Some(value) => {
                            count += usize::from(!present);
                            new_bytes += key.len() + value.allocated();
                        }
                        None => count -= usize::from(present),
                    }
                }
                if count > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
                let mut nodes = 0;
                let extent = count * size_of::<PdfDictEntry>() + new_bytes + child_extent(original, &mut nodes, 0)?;
                if let Some(refusal) = missing_grant(extent, grant, maximum_copy) { return Ok(refusal); }
                let entries = replacement_page(original, &changes, count);
                allocated = dict_capacity(&entries, entries.capacity());
                self.rows.page = Some(PdfIndirectObject { id: owner, value: PdfObject::Dict(entries) });
            }
`;
const region=source.slice(source.indexOf(start),source.indexOf(end));
source=exact(source,region,code);
source=exact(source,"fn work(allocated: usize, ready: bool)",geometry+"\nfn work(allocated: usize, ready: bool)");
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const pkg=resolve(path,"..","📦️packages","🦀️rust");
for (const os of ["linux","macos","windows","wasi"]) {
    const cfg = [`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?"wasm":os==="windows"?"windows":"unix"];
    const row=await measureRustCrate(process.cwd(),resolve(pkg,"🦀️.rs"),false,new Map([[resolve(path),source]]),pkg,cfg);
    assert.ok(row.files<=32&&row.lines<=4000&&row.bytes<=262144);
    console.log("[DEBUG]",os,row.files,row.lines,row.bytes);
}
assert.equal(await Bun.file(path).text(),before);
await Bun.write(join(import.meta.dir,"🗑️generated","pdf-save-box-graft-overlay.json"),JSON.stringify([[path,createHash("sha256").update(before).digest("hex"),source]],null,2)+"\n");
