/** 🧪️ WP4b acceptance checks: job-budget consolidation, render hoist, ui-host declaration contract. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const Ajv = createRequire(import.meta.url)("ajv");
const load = (path) => JSON.parse(readFileSync(path, "utf8"));
const bind = (module, name) => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(module);
  return ajv.getSchema(`${module.$id}#/$defs/${name}`);
};
const report = (label, validate, data) => console.log(label, validate(data) ? "PASS" : `FAIL ${JSON.stringify(validate.errors)}`);

const budget = "🧰️framework/🔨️modules/🧵️job/⏱️budget";
const budgetModule = load(`${budget}/🧬️schema/🔣️.json`);
for (const [file, name] of [["🧫️fixture/🔣️.json", "Budget"], ["🕰️clock.json", "Clock"], ["🪢️binding.json", "Binding"]]) {
  report(`job.budget/${name}`, bind(budgetModule, name), load(`${budget}/${file}`));
}
const budgetFixture = load(`${budget}/🧫️fixture/🔣️.json`);
const validateBudget = bind(budgetModule, "Budget");
console.log("job.budget/Budget rejects forged unit:", !validateBudget({ ...budgetFixture, unit: "milliseconds" }), "rejects extra field:", !validateBudget({ ...budgetFixture, extra: true }));

const renderModule = load("🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema/🔣️.json");
report("ui.render/MetalObjectiveCAbiFixture", bind(renderModule, "MetalObjectiveCAbiFixture"), load("🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧫️fixtures/🔣️.json"));
console.log("ui.render/WebgpuSurfacePort compiled:", typeof bind(renderModule, "WebgpuSurfacePort") === "function");

const hostModule = load("🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json");
report("ui.host/BrowserHostDeclaration", bind(hostModule, "BrowserHostDeclaration"), load("🧰️framework/🔨️modules/🖱️ui/🖥️host/🤝️contract.json"));

// negative probes for the hand-authored ui.host declaration contract
const hostValidate = bind(hostModule, "BrowserHostDeclaration");
const contract = load("🧰️framework/🔨️modules/🖱️ui/🖥️host/🤝️contract.json");
const clone = () => JSON.parse(JSON.stringify(contract));
const probe = (label, mutate) => { const data = clone(); mutate(data); console.log(`ui.host rejects ${label}:`, !hostValidate(data)); };
probe("eventBytes above 1024", (d) => { d.limits.eventBytes = 1025; });
probe("encodedEventBytes above 1051", (d) => { d.limits.encodedEventBytes = 1052; });
probe("pageBytes above 1033", (d) => { d.limits.pageBytes = 1034; });
probe("listeners above 64", (d) => { d.limits.listeners = 65; });
probe("criticalEvents above 32", (d) => { d.limits.criticalEvents = 33; });
probe("retainedPollItems above 1", (d) => { d.limits.retainedPollItems = 2; });
probe("semanticUnitsPerGrant above 1", (d) => { d.limits.semanticUnitsPerGrant = 2; });
probe("canvas identity minimum 0", (d) => { d.identities.CanvasId.minimum = 0; });
probe("canvas identity above u32", (d) => { d.identities.CanvasId.minimum = 4294967296; });
probe("operation code outside 1793..1798", (d) => { d.operations.attach = 1799; });
probe("event code outside 1801..1811", (d) => { d.events.metrics = 1812; });
probe("foreign transport", (d) => { d.transport = "semio.framework.wire.v1"; });
probe("empty title", (d) => { d.title = ""; });
probe("malformed event prefix field", (d) => { d.eventPrefix[1] = "canvas u32le"; });
probe("duplicate lifecycle stage", (d) => { d.lifecycle[7] = "close"; });
probe("unknown lifecycle stage", (d) => { d.lifecycle[7] = "reopen"; });
probe("unknown coalescing policy", (d) => { d.coalescing.metrics = "first-wins"; });
probe("unknown poll short-capacity policy", (d) => { d.poll.shortCapacity = "truncate"; });
probe("unknown accessibility label policy", (d) => { d.accessibility.localizedLabel = "optional"; });
probe("extra top-level key", (d) => { d.extra = 1; });

// every module $id must be derivable from its path (contract §A: no override table)
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const MODULES = "🧰️framework/🔨️modules";
const walk = (dir, out = []) => {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (entry === "🤖️generated" || entry === "node_modules" || entry === "target" || entry === "dist" || entry === "pkg") continue;
    if (statSync(path).isDirectory()) walk(path, out);
    else if (entry === "🔣️.json" && dir.endsWith("🧬️schema")) out.push(path);
  }
  return out;
};
const ascii = (segment) => segment.replace(/[\p{Extended_Pictographic}\u{FE0F}\u{200D}]/gu, "");
const undeliverable = [];
let derivable = 0;
for (const path of walk(MODULES)) {
  if (path.includes("🧬️mutations") || path.startsWith(`${MODULES}/🧬️schema/`)) continue;
  const document = JSON.parse(readFileSync(path, "utf8"));
  const segments = path.split("/").slice(2, -2).map(ascii);
  const expected = `https://semio.tech/schema/framework/${segments.join("/")}/schema.json`;
  if (document.$id === expected) derivable++;
  else undeliverable.push([path, document.$id, expected]);
}
console.log(`path-derivable module $ids: ${derivable} ok, ${undeliverable.length} not derivable`);
for (const [path, actual, expected] of undeliverable) console.log(`  ${path}\n    actual   ${actual}\n    expected ${expected}`);
