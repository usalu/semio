import { appendFileSync, writeFileSync } from "fs";
const [,, mode, path, ...rest] = process.argv;
const data = rest.join(" ") === "-" ? await Bun.stdin.text() : Buffer.from(rest.join(" "), "base64").toString("utf8");
if (mode === "write") writeFileSync(path, data);
else if (mode === "append") appendFileSync(path, data);
else throw new Error(mode);
console.log(mode, path, data.length);
