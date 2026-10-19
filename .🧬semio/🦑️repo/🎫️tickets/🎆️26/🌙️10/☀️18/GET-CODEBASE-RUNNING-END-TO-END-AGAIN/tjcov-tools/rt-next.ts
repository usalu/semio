import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fam = (...d: string[]) => d.map((x) => readSourceFamilySource(`${P}/${x}`)).join("");
export const instance = fam("🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job");
if (import.meta.main) {
  const [needle, last] = process.argv.slice(2);
  const at = last === "last" ? instance.lastIndexOf(needle!) : instance.indexOf(needle!);
  const re = /\n    (?:pub(?:\([a-z]+\))? )?(?:async )?fn [a-z_0-9]+/g; re.lastIndex = at + needle!.length;
  const m = re.exec(instance);
  console.log("start", at, "next fn at", m?.index, m?.[0].trim(), "len", m ? m.index - at : -1);
  const nm = /\n    (?:pub(?:\([a-z]+\))? )?(?:async )?fn [a-z_0-9]+/g; nm.lastIndex = at + needle!.length + 1;
  const after = instance.slice(at, m?.index ?? at + 1);
  console.log("contains dispatch_wire_retained_with_spec:", after.includes("dispatch_wire_retained_with_spec"), "retained_wire.take():", after.includes("retained_wire.take()"));
}
