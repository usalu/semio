// 🌳️ c4p49 reader probe: the module-tree text of the split actions and arg-schema roots versus their pre-split snapshots, for the two source-text readers re-pointed by the slice. usage: bun probe-tree.ts
import {readFileSync} from "node:fs";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import {toolJobRustBlock} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";

const R = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const snap = (tag: string) => readFileSync(`${import.meta.dir}/orig/${tag}.rs`, "utf8");

const forKind = (manifest: string) => {
  const start = manifest.indexOf("pub fn for_kind(kind: ActionKind)");
  const open = start < 0 ? -1 : manifest.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(manifest, open);
  return {start: start >= 0, body: block?.body.length ?? -1, migrated: block?.body.includes("InteractiveJobClassification::Migrated") ?? null};
};
const bounded = (manifest: string) => /pub fn bounded_catalog\([\s\S]{0,500}?InteractiveJobClassification::Migrated/.test(manifest);

const actTree = readModuleTreeSource(`${R}/🛂️manifest/🕹️actions/🦀️.rs`);
const actOld = snap("act");
console.log("act tree chars", actTree.length, "snapshot chars", actOld.length);
console.log("for_kind snapshot", JSON.stringify(forKind(actOld)), "tree", JSON.stringify(forKind(actTree)));
console.log("bounded_catalog snapshot", bounded(actOld), "tree", bounded(actTree));
const same = JSON.stringify(forKind(actOld)) === JSON.stringify(forKind(actTree)) && bounded(actOld) === bounded(actTree);
console.log("ACT READER", same ? "EQUAL" : "DIFFERENT");

const argTree = readModuleTreeSource(`${R}/🛂️manifest/🧮️arg-schema/🦀️.rs`);
const argOld = snap("arg");
const owners = ["ActionArgOption", "ActionArgControl", "ActionArgDef", "ActionArgNumberFacets", "ArgSchema", "ArgFormat", "ArgPresentation", "ReferenceIdType", "AppRole", "SnapSource", "OptionSource", "NumberScale"];
let ok = true;
for (const name of owners) {
  const kindOf = (src: string) => {
    for (const k of ["struct", "enum"]) {
      const marker = src.indexOf(`pub ${k} ${name}`);
      if (marker > 0) {
        const derive = src.lastIndexOf("#[derive(", marker);
        return `${k}:${src.slice(derive, src.indexOf(")]", derive)).includes("semio_framework_value::RetireOwned")}`;
      }
    }
    return "missing";
  };
  const a = kindOf(argOld), b = kindOf(argTree);
  if (a !== b) ok = false;
  console.log(name, a, b);
}
console.log("ARG READER", ok ? "EQUAL" : "DIFFERENT", "tree files text chars", argTree.length, "snapshot", argOld.length);
