import {pinnedRustToolchainV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🟦️.ts";
const r = pinnedRustToolchainV1(undefined, { ...process.env } as never);
console.log(JSON.stringify(r.environment));
