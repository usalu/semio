import { clauses, lawFile } from "./ct.ts";
import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const plugin = readSourceFamilySource(P, { tests: process.argv[2] === "tests" });
clauses(lawFile("🔒️lifecycle"), "toolJobRuntimeProofQualified", [plugin]);
