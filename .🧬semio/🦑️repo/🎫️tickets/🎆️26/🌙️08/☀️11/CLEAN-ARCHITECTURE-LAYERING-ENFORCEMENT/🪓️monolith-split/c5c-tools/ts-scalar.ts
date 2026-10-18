// 🧪 c5c: runs the scalar config cohort self-tests of the retained-command tool-job law. usage: bun ts-scalar.ts
import {toolJobScalarConfigCohortSelfTests} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts";
try { console.log(JSON.stringify(toolJobScalarConfigCohortSelfTests())); } catch (e) { console.log("FAIL", String((e as Error).message).slice(0, 400)); }
