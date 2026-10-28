/** 🧰️ [td-observe] runs one test file of a mirror beneath a genuine birth with the immutable Nx SDK identity of the mirror's own `.nx/installation` clone supplied as the `setup-sdk` birth would. usage: bun sdkdrive.ts <mirror root> <test file> [birth] [pattern] */
import {realpathSync} from "node:fs";
import {basename,join} from "node:path";
const [root,file,birth="setup",pattern]=process.argv.slice(2) as [string,string,string?,string?];
const {runGenuineBunTestV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🟦️.ts"));
const toolkit=realpathSync(join(root,".nx/installation"));
await runGenuineBunTestV1({root,file,birth,pattern:pattern||undefined,environment:{SEMIO_NX_TOOLING_IDENTITY:basename(toolkit),SEMIO_NX_TOOLING_ROOT:toolkit}});
process.exit(0);
