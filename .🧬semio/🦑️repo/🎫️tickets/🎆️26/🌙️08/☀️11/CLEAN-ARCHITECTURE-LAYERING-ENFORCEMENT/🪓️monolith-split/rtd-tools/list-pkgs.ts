// 📇️ Prints "package-name<TAB>manifest-dir<TAB>lib-path" for each manifest listed in a file.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
for (const m of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  try {
    const t = Bun.TOML.parse(readFileSync(`${ROOT}/${m}`, "utf8")) as any;
    console.log([t.package?.name ?? "?", m.replace("/Cargo.toml", ""), t.lib?.path ?? "src/lib.rs"].join("\t"));
  } catch (e) { console.log(["ERR", m, String(e)].join("\t")); }
}
