import {doctorPortsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts";
import {writeFileSync} from "node:fs";
const ports=doctorPortsV1(process.cwd(),{signal:new AbortController().signal});const all=await ports.processes();const selected=all.filter(row=>JSON.stringify(row).includes("original-source-return-place"));const result={at:Date.now(),total:all.length,selected};writeFileSync(`${import.meta.dirname}/🗑️generated/return-place-census.json`,JSON.stringify(result));console.log(JSON.stringify(result));
