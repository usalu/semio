import {test,expect} from "bun:test";
import {startUnitCacheCollectorV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {measureVolumeV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts";
const root="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u";
test("start the watching collector the way a compile launch does",()=>{expect(startUnitCacheCollectorV1({root,capacityBytes:measureVolumeV1(root,1).volumeBytes,environment:process.env,watch:true})).toEqual({state:"started"});});
