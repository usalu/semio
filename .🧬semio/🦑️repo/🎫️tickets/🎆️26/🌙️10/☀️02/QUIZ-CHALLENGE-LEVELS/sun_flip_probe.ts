/** 🌞️ Searches values beside `truth / (1000 × (1 + 1e-9))` whose miss verdict against the Sun (`3.828e+26`) flips when both numbers are read as serde_json reads them without `float_roundtrip` (the decimal significand as a double, times or divided by a power of ten from its table) instead of correctly rounded. */
const POW10 = Array.from({ length: 309 }, (_, power) => Number(`1e${power}`));

/** 📖️ serde_json's default reading of a decimal text. */
function naive(text: string): number {
  const match = /^(\d+)(?:\.(\d+))?(?:e([+-]?\d+))?$/u.exec(text.toLowerCase())!;
  const digits = `${match[1]}${match[2] ?? ""}`.replace(/^0+(?=\d)/u, "");
  const exponent = Number(match[3] ?? 0) - (match[2]?.length ?? 0);
  if (BigInt(digits) > 18446744073709551615n) return NaN;
  const significand = Number(BigInt(digits));
  return exponent >= 0 ? significand * POW10[exponent]! : significand / POW10[-exponent]!;
}

/** ⏭️ The next double above `value`. */
function next(value: number, step: number): number {
  const view = new DataView(new ArrayBuffer(8));
  view.setFloat64(0, value);
  view.setBigUint64(0, view.getBigUint64(0) + BigInt(step));
  return view.getFloat64(0);
}

const bound = 1000 * (1 + 1e-9);
const miss = (value: number, truth: number): boolean => (value > truth ? value / truth : truth / value) > bound;
const truthText = "3.828e+26";
const [truth, misread] = [Number(truthText), naive(truthText)];
console.log("[DEBUG] truth", truth, "misread", misread, "bound", bound);
const centre = truth / bound;
for (let step = -40; step <= 40; step++) {
  const value = next(centre, step);
  const text = String(value);
  const read = naive(text);
  const [right, wrong] = [miss(value, truth), miss(read, misread)];
  if (right !== wrong) console.log("[DEBUG] flip", text, "read as", read, "correct miss", right, "serde miss", wrong, "ratio", truth / value, misread / read);
}
for (const text of ["3.828e+26", "1.74e+17", "1.87e+13", "6e+10", "1.41e+9", "3.828e26"]) console.log("[DEBUG] read", text, Number(text), naive(text), Number(text) === naive(text));
