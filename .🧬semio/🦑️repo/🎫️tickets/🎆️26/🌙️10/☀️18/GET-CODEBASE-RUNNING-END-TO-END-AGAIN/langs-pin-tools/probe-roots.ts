import {repositoryCachePartitionRootV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
const root = process.cwd();
for (const name of ["tools", "dependencies", "rust"] as const) console.log(name, repositoryCachePartitionRootV1(root, name as never));
