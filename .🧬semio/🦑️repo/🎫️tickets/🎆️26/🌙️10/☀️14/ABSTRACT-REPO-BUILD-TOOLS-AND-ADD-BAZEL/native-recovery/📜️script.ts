import {test,expect} from "bun:test";
import {existsSync} from "node:fs";
import {withBuildTestInvocationV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {recoverNxGraphFixtureV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️store/🟦️.ts";
test("exact interrupted private fixture managed recovery",async()=>withBuildTestInvocationV1(async original=>{const workspace=process.cwd()+"/.cache/b/p/m/gf-tJbxJX";if(existsSync(workspace))await recoverNxGraphFixtureV1(original,workspace);expect(existsSync(workspace)).toBe(false);console.log("[DEBUG] Exact interrupted private fixture images and allocations retired through genuine owners");}),100000);
