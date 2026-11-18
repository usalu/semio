import { plugin } from "bun";
let shown = 0;
plugin({ name: "r129-trace-ts", setup(b) { b.onResolve({ filter: /^typescript(\/.*)?$/ }, args => { if (shown++ < 3) console.log("[DEBUG] typescript requested by", args.importer, "\n" + String(new Error().stack).split("\n").slice(1, 14).join("\n")); return undefined as never; }); } });
