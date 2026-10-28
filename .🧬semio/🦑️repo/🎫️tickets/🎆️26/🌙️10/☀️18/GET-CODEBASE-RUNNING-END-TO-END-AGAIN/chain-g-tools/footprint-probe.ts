import "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts";
export async function runCapturedBuildSourceV1(args:readonly string[],proof:{recheck:()=>void}){
 proof.recheck();
 const before=process.memoryUsage().rss;Bun.gc(true);const after=process.memoryUsage();
 console.log("FOOTPRINT "+JSON.stringify({peakMB:Math.round(process.resourceUsage().maxRSS/1048576),rssMB:Math.round(before/1048576),afterGcRssMB:Math.round(after.rss/1048576),heapUsedMB:Math.round(after.heapUsed/1048576),externalMB:Math.round(after.external/1048576),arrayBuffersMB:Math.round(after.arrayBuffers/1048576),modules:Object.keys(require.cache??{}).length}));
}
