/** 🧪 obsrows2-g: runs one test file of a mirror under the `setup` (SDK-free) genuine birth with the Nx tooling identity and both Nx cache variables supplied by hand. usage: bun mdrive.ts <mirror root> <test file> [birth] [pattern] */
import {readlinkSync} from "node:fs";
import {basename,join} from "node:path";
const [root,file,birth="setup",pattern]=process.argv.slice(2);
const {runGenuineBunTestV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🟦️.ts"));
const toolkit=readlinkSync(join(root,".nx/installation")),identity=basename(toolkit);
await runGenuineBunTestV1({root,file,birth,pattern:pattern||undefined,environment:{SEMIO_NX_TOOLING_IDENTITY:identity,SEMIO_NX_TOOLING_ROOT:toolkit}});
process.exit(0);
