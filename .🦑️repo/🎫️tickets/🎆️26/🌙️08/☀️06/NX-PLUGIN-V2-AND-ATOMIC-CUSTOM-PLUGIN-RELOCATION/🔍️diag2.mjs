import { readdirSync } from "node:fs";
for (const base of [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/⚡️implementations/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db",
]) {
  console.log("=== " + base);
  for (const b of readdirSync(base, { encoding: "buffer" })) {
    const hex = b.toString("hex");
    const str = b.toString("utf8");
    const valid = Buffer.from(str, "utf8").equals(b);
    console.log(" ", valid ? "VALID  " : "INVALID", hex, JSON.stringify(str));
  }
}
