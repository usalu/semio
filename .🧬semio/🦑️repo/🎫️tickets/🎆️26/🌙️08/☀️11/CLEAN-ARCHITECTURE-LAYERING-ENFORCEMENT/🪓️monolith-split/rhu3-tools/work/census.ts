import { loadChannelVersionSourceV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🟦️.ts";
import { channelVersionCensus } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🔍️census/🟦️.ts";
const source = loadChannelVersionSourceV1("/Users/ueli/Documents/semio");
const result = channelVersionCensus(source);
console.log("pin", result.pin, "consumers", source.consumers.length, "candidates", source.candidates.length, "findings", result.findings.length);
for (const f of result.findings.slice(0, 20)) console.log(f.problem, f.path.slice(-90), f.detail.slice(0, 120));
