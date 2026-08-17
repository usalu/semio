import { readdirSync, readFileSync, existsSync, statSync } from "fs";
import { join } from "path";

const uiPkg = [...Bun.Glob("**/package.json").scanSync({ cwd: "/Users/ueli/Documents/semio/𝒯framework" })];
