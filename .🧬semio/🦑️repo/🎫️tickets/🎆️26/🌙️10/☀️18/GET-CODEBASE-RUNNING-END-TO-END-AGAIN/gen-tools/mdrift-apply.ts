import { projectParsers, renderProjectionReport } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const under = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh";
for (const mode of ["report", "apply"] as const) process.stdout.write(`${mode}\n${renderProjectionReport(projectParsers({ root: "/Users/ueli/Documents/semio", mode, under, formats: new Set(["🟦️typescript"]), closing: true, replace: new Set(), restate: new Set(["self-call", "untyped-map", "open", "stale"]) }))}`);
