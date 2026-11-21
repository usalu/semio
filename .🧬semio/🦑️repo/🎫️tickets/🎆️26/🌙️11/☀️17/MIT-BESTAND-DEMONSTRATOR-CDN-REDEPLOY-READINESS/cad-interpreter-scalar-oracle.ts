const text = await Bun.file(process.argv[2]!).text();
const type = { i32: 127, i64: 126, f32: 125, f64: 124 };
function value(kind: string, input: string) {
  return kind === "i64" ? BigInt(input) : Number(input);
}
let passed = 0;
for (const row of text.trim().split("\n").slice(1)) {
  const [opcode, parameter, result, literals, expected] = row.split("\t") as [string, keyof typeof type, keyof typeof type, string, string];
  const inputs = literals.split(",").map(x => value(parameter, x));
  const signature = [1, 96, inputs.length, ...inputs.map(() => type[parameter]), 1, type[result]];
  const body = [0, ...inputs.flatMap((_, slot) => [32, slot]), parseInt(opcode, 16), 11];
  const bytes = Uint8Array.of(0, 97, 115, 109, 1, 0, 0, 0,
    1, signature.length, ...signature, 3, 2, 1, 0, 7, 5, 1, 1, 120, 0, 0, 10, body.length + 2, 1, body.length, ...body);
  const execute = new WebAssembly.Instance(new WebAssembly.Module(bytes)).exports.x as Function;
  let actual: unknown;
  try {
    actual = execute(...inputs);
  } catch (error) {
    if (!(error instanceof WebAssembly.RuntimeError)) throw error;
    actual = "trap";
  }
  if (!Object.is(actual, expected === "trap" ? "trap" : value(result, expected))) throw Error(row + " observed " + actual);
  passed++;
}
console.log(passed + " scalar cases match JavaScriptCore WebAssembly");
