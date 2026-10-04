/** 🔬️ Probes miss tests at typed-decimal ×1000 and /1000 boundaries: the quotient form `max / min > reach`, the product form `max > reach × min` and the old `log10` difference, over truths typed with 0 to 3 decimals. */
const typed = (digits: string, power: number): number => Number(`${digits}e${power}`);
const forms = {
  quotient: (value: number, truth: number, reach: number) => (value > truth ? value / truth : truth / value) > reach,
  product: (value: number, truth: number, reach: number) => (value > truth ? value > reach * truth : truth > reach * value),
  log10: (value: number, truth: number, reach: number) => Math.abs(Math.log10(value) - Math.log10(truth)) > Math.log10(reach),
};
for (const [name, misses] of Object.entries(forms)) {
  const counts = { up: 0, down: 0, total: 0, examples: [] as string[] };
  for (let integer = 1; integer <= 2000; integer++) {
    for (const decimals of [0, 1, 2, 3]) {
      const digits = String(integer);
      const truth = typed(digits, -decimals);
      counts.total++;
      if (misses(typed(digits, 3 - decimals), truth, 1000)) {
        counts.up++;
        if (counts.examples.length < 6) counts.examples.push(`${truth}×1000`);
      }
      if (misses(typed(digits, -3 - decimals), truth, 1000)) {
        counts.down++;
        if (counts.examples.length < 6) counts.examples.push(`${truth}/1000`);
      }
    }
  }
  console.log("[DEBUG]", name, JSON.stringify(counts));
}
console.log("[DEBUG] 18/0.018", 18 / 0.018, "1000*0.018", 1000 * 0.018, "1000*18", 1000 * 18);
console.log("[DEBUG] sqrt(72/18)", Math.sqrt(72 / 18), "36.00000000000001/18", 36.00000000000001 / 18, "next 18000", 18000.000000000004 / 18);
const below = 0.018 - 2 ** -58;
console.log("[DEBUG] one ulp below 0.018", below, 18 / below, 1000 * below);
