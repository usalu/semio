/** 🧹️ Requests compiler headroom through the owner that checks active unit leases. */
import { statfsSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const source = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts";
const { reclaimUnitStorageV1 } = await import(pathToFileURL(resolve(root,source)).href);
const image = resolve(root,".cache/b/r/m");
const volume = statfsSync(image), host = statfsSync(resolve(image,".."));
if (volume.blocks === host.blocks && volume.bsize === host.bsize) throw Error("The fixed compiler image is not mounted");
const controller = new AbortController(), abort = () => controller.abort();
const limit = setTimeout(abort,45_000);
process.once("SIGINT",abort);
process.once("SIGTERM",abort);
try {
    const gib = 1024 ** 3, free = volume.bavail * volume.bsize;
    const demand = Math.min(3 * gib,Math.max(0,8 * gib-free));
    const reclaimed = demand ? await reclaimUnitStorageV1(image,demand,{signal:controller.signal,report:{language:"en",write:line=>console.log("[DEBUG]",line)}}) : 0;
    console.log("[DEBUG] Compiler owner reclaimed",reclaimed,"bytes from idle storage");
} finally {
    clearTimeout(limit);
    process.removeListener("SIGINT",abort);
    process.removeListener("SIGTERM",abort);
}
