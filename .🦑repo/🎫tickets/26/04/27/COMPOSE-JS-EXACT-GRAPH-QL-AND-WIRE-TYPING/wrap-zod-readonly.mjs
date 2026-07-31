import fs from "node:fs";
import path from "node:path";

const p = path.resolve("compose/js/index.ts");
let s = fs.readFileSync(p, "utf8");
// Insert ReadonlyDto after JsonObject if missing
if (!s.includes("export type ReadonlyDto<")) {
  s = s.replace(
    "export type JsonObject = { readonly [key: string]: JsonValue };",
    `export type JsonObject = { readonly [key: string]: JsonValue };

/** @emoji 🔒 Recursive readonly view for wire/DTO value kinds (Zod inferences, GraphQL DTOs). */
export type ReadonlyDto<T> = T extends ReadonlyArray<infer U>
  ? ReadonlyArray<ReadonlyDto<U>>
  : T extends object
    ? { readonly [K in keyof T]: ReadonlyDto<T[K]> }
    : T;`,
  );
}
s = s.replace(/^export type ([A-Za-z0-9_]+) = z\.infer<typeof ([A-Za-z0-9_]+Schema)>;$/gm, "export type $1 = ReadonlyDto<z.infer<typeof $2>>;");
fs.writeFileSync(p, s);
console.log("ok");
