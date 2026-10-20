// 🔬 Runs validateTaxonomy from the simulated library copy over its own taxonomy document. usage: [TAX6_SIM=<scratch>] bun validate-sim.ts
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const L = process.env.TAX6_SIM ? `${C}/🗑️generated/${process.env.TAX6_SIM}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` : "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const {validateTaxonomy} = await import(`/Users/ueli/Documents/semio/${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const problems = validateTaxonomy();
console.log(JSON.stringify({problems: problems.length}));
for (const p of problems) console.log(p);
