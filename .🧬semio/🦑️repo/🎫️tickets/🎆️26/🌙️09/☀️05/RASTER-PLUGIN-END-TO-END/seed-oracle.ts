import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
for (const [actor, clockActor, physicalMs, logical] of [["alice","1","42","0"],["💧/Ä","18446744073709551615","9223372036854775807","65535"]]) {
  const bytes = `${actor}\u001f${clockActor}:${physicalMs}:${logical}`;
  console.log(JSON.stringify({actor,clockActor,physicalMs,logical,expected:`authoring-seed-${createHash("sha256").update(bytes).digest("hex").slice(0,16)}`}));
}
const law=JSON.parse(readFileSync("🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/📨️wire/🧪️tests/🔣️.json","utf8")).originalReturn;
if(JSON.stringify(law.input)!==law.expectedUtf8)throw new Error("independent original-return wire oracle differs");
console.log("original-return canonical wire oracle matched");
