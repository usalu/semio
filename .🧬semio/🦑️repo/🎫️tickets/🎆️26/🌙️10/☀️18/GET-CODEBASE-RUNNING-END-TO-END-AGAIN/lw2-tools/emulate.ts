import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const OS = "🧰️framework/🛍️products/💻️os/";
const os = rows.filter(r => r.to?.startsWith(OS) && !r.sdk);
const pairs = new Map<string, any>(); for (const r of os) pairs.set(r.from + "\0" + r.to + "\0" + r.spec, r);
let sdk = 0, state = 0; for (const r of pairs.values()) { sdk++; if (/\/(🏪️store|🖥️host)\//u.test(r.to)) state++; }
const all = rows.filter(r => r.to && !r.sdk);
console.log(JSON.stringify({ plugin_to_os_edges: pairs.size, files: new Set(os.map(r => r.from)).size, "plugins-framework-sdk-only(os targets)": sdk, "no-state-outside-os": state, plugin_to_any_framework_nonSDK_edges: new Set(all.map(r => r.from + r.to + r.spec)).size }));
