import { completionRejectionOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts";
import { fileURLToPath } from "node:url";
const root = fileURLToPath(new URL("/Users/ueli/Documents/semio/", import.meta.url)).replace(/\/$/, "");
try { console.log("completionRejectionOracle ok rows=", completionRejectionOracle(root)); } catch (error) { console.log("completionRejectionOracle FAILED:", String(error).slice(0, 400)); }
