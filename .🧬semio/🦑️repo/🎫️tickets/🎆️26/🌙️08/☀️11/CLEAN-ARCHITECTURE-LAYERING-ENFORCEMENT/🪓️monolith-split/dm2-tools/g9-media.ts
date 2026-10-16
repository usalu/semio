// 🎞️ The compile-ingress source reader accepts include_str! only with string literals, so the intrinsic-media template takes its three fixtures as expression arguments: the literal paths live at the two invocation sites.
import {readFileSync} from "node:fs";
import {put, replaceOnce, read, log} from "./lib.ts";
const FW = "🧰️framework";
const LAWS = `${FW}/🔨️modules/🎒️pack/🌱️value/🧪️tests/🎞️intrinsic-media`;
const PRE = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/dm2pre/";
const original = readFileSync(PRE + `${LAWS}/🦀️.rs`, "utf8");
const split = original.indexOf("\n");
const head = original.slice(0, split + 1);
let body = original.slice(split + 1);
const map: [string, string][] = [["🧮️wire-materialization/🔣️.json", "$wire"], ["🎞️intrinsic-media/🌲️depth/🔣️.json", "$depth"], ["🎞️intrinsic-media/🔣️.json", "$media"]];
let count = 0;
body = body.replace(/include_str!\("\.\.\/\.\.\/🧪️tests\/🧫️fixtures\/([^"]*)"\)/gu, (_m, tail: string) => {
  const hit = map.find(([suffix]) => tail === suffix);
  if (!hit) throw new Error(`unmapped fixture ${tail}`);
  count++;
  return hit[1];
});
if (count !== 6) throw new Error(`fixture reads ${count}`);
put(`${LAWS}/🦀️.rs`, `${head}\n#[macro_export]\nmacro_rules! intrinsic_media_wire_laws {\n    ($wire:expr, $media:expr, $depth:expr) => {\n${body.replace(/\n+$/u, "")}\n    };\n}\n`);
const call = (base: string) => `semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!(\n        include_str!("${base}/🧫️fixtures/🧮️wire-materialization/🔣️.json"),\n        include_str!("${base}/🧫️fixtures/🎞️intrinsic-media/🔣️.json"),\n        include_str!("${base}/🧫️fixtures/🎞️intrinsic-media/🌲️depth/🔣️.json"));`;
replaceOnce(`${FW}/🔨️modules/🎒️pack/🧪️tests/🔬️record-laws/🦀️.rs`, `    semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!("../../🌱️value/🧪️tests");`, `    ${call("../../🌱️value/🧪️tests")}`);
replaceOnce(`${FW}/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs`, `    semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!("../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests");`, `    ${call("../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests")}`);
