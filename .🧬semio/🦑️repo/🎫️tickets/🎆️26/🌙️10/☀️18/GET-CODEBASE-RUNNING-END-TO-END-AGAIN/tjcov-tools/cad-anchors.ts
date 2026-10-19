import { readFileSync } from "node:fs";
import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const F = `${R}/🧰️framework`;
const S = `${F}/🛍️products/💻️os/🔨️modules/🏪️store`;
const P = `${S}/👥️presence`;
const rd = (p: string) => readFileSync(p, "utf8");
const srcs: Record<string, string> = {
  read: rd(`${F}/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs`),
  presence: rd(`${P}/🦀️.rs`),
  eph: rd(`${S}/🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs`),
  one: rd(`${S}/🫧️ephemeral/📤️ephemeral-one-item-publication/🦀️.rs`),
  ret: rd(`${P}/♻️retirement/🦀️.rs`),
  rej: rd(`${P}/🚫️rejection/🦀️.rs`),
  io: rd(`${P}/🚪️io/📬️publication/🦀️.rs`),
  retTests: rd(`${P}/♻️retirement/🧪️tests/🔬️unit/🦀️.rs`),
  cad: rd(`${R}/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs`),
  storeLaws: rd(`${S}/🧪️tests/🌌️bin-04/🔬️unit/🛤️history-lane-interaction-store-and-immutable-root-laws/🦀️.rs`),
};
const anchors: [string, string][] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const [where, needle] of anchors) {
  const t = where === "any" ? Object.values(srcs).join("\n") : srcs[where] ?? "";
  console.log(t.includes(needle) ? "ok  " : (Object.entries(srcs).filter(([, v]) => v.includes(needle)).map(([k]) => k).join(",") ? "ELSE" : "MISS"), where, JSON.stringify(needle).slice(0, 120), t.includes(needle) ? "" : "=> " + Object.entries(srcs).filter(([, v]) => v.includes(needle)).map(([k]) => k).join(","));
}
