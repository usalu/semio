import { readFileSync } from "node:fs";
import { join } from "node:path";

const source = join(process.cwd(), "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🗣️both-languages/🟦️.ts");
new Bun.Transpiler({ loader: "ts" }).transformSync(readFileSync(source, "utf8"));
console.log("Bilingual browser PDF acceptance syntax passed; runtime checks remain pending");
