const fs = require("fs");
const R = "/Users/ueli/Documents/semio/";
const L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const G = __dirname + "/../🗑️generated/";
const imp = (n) => fs.readFileSync(G + `taf-importers-${n}.txt`, "utf8").split("\n").filter(Boolean).map((f) => R + f);
const D = L + "⚖️laws/🔬️interactivity-audit/";
const specs = {
  database: {
    file: D + "🗄️database/🟦️.ts", from: 16, to: 27, childDir: "🌱️database-lifecycle",
    doc: "/** 🌱️ Interactivity audit: database catalog bootstrap, document creation, compaction and sync-hello fixtures, artifact history, DB CLI and MCP HTTP transport sources.\n * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts\n */",
    importers: imp("🗄️database"),
  },
  mounted: {
    file: D + "🖥️mounted/🟦️.ts", from: 7, to: 9, childDir: "🖼️mounted-surface",
    doc: "/** 🖼️ Interactivity audit: mounted engine surface lifetime, prepared render and surface lane sources.\n * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts\n */",
    importers: imp("🖥️mounted"),
  },
};
for (const [k, v] of Object.entries(specs)) fs.writeFileSync(__dirname + `/spec-${k}.json`, JSON.stringify(v, null, 1));
