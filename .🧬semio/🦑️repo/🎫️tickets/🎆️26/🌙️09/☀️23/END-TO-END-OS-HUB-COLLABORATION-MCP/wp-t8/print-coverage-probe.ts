import { vizCoverageReport } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts";
const report = vizCoverageReport();
console.log(JSON.stringify({ undocumented: report.undocumentedOptions, phantom: report.phantomOptions, unknown: report.unknownOptions, missingVariant: report.missingVariant }, null, 1));
