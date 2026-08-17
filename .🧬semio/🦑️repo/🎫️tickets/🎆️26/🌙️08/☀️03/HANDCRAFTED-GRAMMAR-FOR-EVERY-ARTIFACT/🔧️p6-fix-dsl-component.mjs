#!/usr/bin/env bun
/** [DEBUG] P6: remove bogus dsl injections; rewrite test DocumentDsl/Op* impls to crate::os_store paths. */
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
function findDsl() {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl", "🦀️component.rs");
    try { if (statSync(p).isFile()) return p; } catch {}
  }
  throw new Error("dsl component missing");
}
const file = findDsl();
let t = readFileSync(file, "utf8");

function stripRegionForType(content, regionLabel, typeName) {
  // Remove //#region regionLabel ... //#endregion regionLabel blocks that mention `for TypeName`
  const re = new RegExp(`\\n//#region ${regionLabel}\\n[\\s\\S]*?impl [\\w:]+ for ${typeName} \\{[\\s\\S]*?//#endregion ${regionLabel}\\n`, "g");
  return content.replace(re, "\n");
}

// Strip bogus CompletionItem and `this` codec regions
const before = t.length;
t = stripRegionForType(t, "🔖️DocumentCodec", "CompletionItem");
t = stripRegionForType(t, "🔖️OpCodec", "this");

// Also handle if region markers differ
t = t.replace(/\n\/\/#region 🔖️OpCodec\n\/\/\/ 🎞️ Handcrafted OpText \(P6\)\.\nimpl OpText for this \{[\s\S]*?\/\/#endregion 🔖️OpCodec\n/, "\n");
t = t.replace(/\n\/\/#region 🔖️DocumentCodec\n\/\/\/ 📜️ Handcrafted DocumentDsl \(P6\)\.\nimpl DocumentDsl for CompletionItem \{[\s\S]*?\/\/#endregion 🔖️DocumentCodec\n/, "\n");

// Rewrite remaining bare DocumentDsl/DocumentPack/OpText/OpBinary in this file to crate::os_* paths
// Only inside the injected regions in tests — replace common wrong paths.
const replacements = [
  [/impl DocumentDsl for /g, "impl crate::os_store::DocumentDsl for "],
  [/impl DocumentPack for /g, "impl crate::os_store::DocumentPack for "],
  [/impl OpText for /g, "impl crate::os_spr::OpText for "],
  [/impl OpBinary for /g, "impl crate::os_spr::OpBinary for "],
  [/Result<Self, TextError>/g, "Result<Self, crate::os_store::TextError>"],
  [/semio_format::/g, "crate::os_store::semio_format::"],
  [/<Self as DocumentDsl>/g, "<Self as crate::os_store::DocumentDsl>"],
  [/&PackEncodeOptions/g, "&crate::os_store::PackEncodeOptions"],
  [/&PackDecodeOptions/g, "&crate::os_store::PackDecodeOptions"],
  [/Result<Vec<u8>, PackError>/g, "Result<Vec<u8>, crate::os_store::PackError>"],
  [/Result<Self, PackError>/g, "Result<Self, crate::os_store::PackError>"],
  [/PackError::Schema/g, "crate::os_store::PackError::Schema"],
  [/pack_rt::/g, "crate::os_store::pack_rt::"],
  [/text_error_to_pack_error/g, "crate::os_store::text_error_to_pack_error"],
  [/&PackEncodeOptions::default\(\)/g, "&crate::os_store::PackEncodeOptions::default()"],
  [/&PackDecodeOptions::default\(\)/g, "&crate::os_store::PackDecodeOptions::default()"],
];

// Only apply path fixes inside DocumentCodec/OpCodec regions to avoid breaking unrelated TextError uses
function fixRegions(content) {
  return content.replace(/\/\/#region 🔖️(?:DocumentCodec|OpCodec)\n[\s\S]*?\/\/#endregion 🔖️(?:DocumentCodec|OpCodec)\n/g, (block) => {
    let b = block;
    for (const [re, rep] of replacements) b = b.replace(re, rep);
    // undo over-replacement of crate::os_store::crate::os_store
    b = b.replace(/crate::os_store::crate::os_store::/g, "crate::os_store::");
    b = b.replace(/crate::os_spr::crate::os_spr::/g, "crate::os_spr::");
    return b;
  });
}
t = fixRegions(t);

// Verify no bogus leftovers
if (/\bfor this\b/.test(t) && /impl OpText for this/.test(t)) throw new Error("this impl still present");
if (/impl DocumentDsl for CompletionItem/.test(t) || /impl crate::os_store::DocumentDsl for CompletionItem/.test(t)) throw new Error("CompletionItem DocumentDsl still present");

writeFileSync(file, t);
console.log(JSON.stringify({ file, before, after: t.length, delta: t.length - before }, null, 2));

// List remaining codec impls
const impls = [...t.matchAll(/impl (?:crate::os_(?:store|spr)::)?(?:DocumentDsl|DocumentPack|OpText|OpBinary) for ([A-Za-z0-9_]+)/g)].map(m => m[0]);
console.log("impls", impls);
