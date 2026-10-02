/** 🧟️ Work package B mutation check: copies the kinematics modules, their unit suites and their vectors into `🗑️generated/wp-b/mutation`, breaks one expression at a time and reports whether the unit suites notice.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/mutate_kinematics.ts`
 * Nothing outside the sandbox is written. Exit code 1 when the untouched copy fails or a mutant survives.
 */

import { copyFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PETS = join(ROOT, "🧰️framework", "🛍️products", "🐾️pets");
const SANDBOX = join(import.meta.dir, "🗑️generated", "wp-b", "mutation");
const UNIT = join("🧪️tests", "🔬️unit", "🟦️.ts");
const MODULES = { trigonometry: "📐️trigonometry", randomness: "🎲️randomness", rig: "🦴️rig" } as const;
const FIXTURES = ["📐️turn-trigonometry", "🎲️counter-randomness", "🦴️rig-solving", "👀️gaze-tracking"];

type Mutant = { readonly module: keyof typeof MODULES; readonly name: string; readonly find: string; readonly put: string };

const MUTANTS: readonly Mutant[] = [
  { module: "trigonometry", name: "sine coefficient truncated", find: "const SINE_3 = -1.98412698298579493134e-4;", put: "const SINE_3 = -1.98412698e-4;" },
  { module: "trigonometry", name: "cosine coefficient truncated", find: "const COSINE_2 = -1.38888888888741095749e-3;", put: "const COSINE_2 = -1.38888888e-3;" },
  { module: "trigonometry", name: "two pi truncated", find: "const TAU = 6.283185307179586;", put: "const TAU = 6.283185307179;" },
  { module: "trigonometry", name: "second quarter keeps its sign", find: "quarter === 2 ? 0 - sineKernel(angle)", put: "quarter === 2 ? sineKernel(angle)" },
  { module: "trigonometry", name: "nearest quarter rounds down", find: "(Math.floor(phase * 8) + 1) * 0.5", put: "(Math.floor(phase * 8) + 0) * 0.5" },
  { module: "trigonometry", name: "sine is no longer odd", find: "return turns < 0 ? 0 - value : value;", put: "return value;" },
  { module: "trigonometry", name: "cosine kernel adds its tail", find: "0.5 * square - square * (square * tail)", put: "0.5 * square + square * (square * tail)" },
  { module: "trigonometry", name: "cosine kernel reassociated (last bits only)", find: "square * (square * tail)", put: "square * square * tail" },
  { module: "trigonometry", name: "clamp does not raise", find: "value < low ? low : value", put: "value < low ? value : low" },
  { module: "trigonometry", name: "lerp adds the ends", find: "(to - from) * amount", put: "(to + from) * amount" },
  { module: "trigonometry", name: "smoothstep loses a factor", find: "(3 - 2 * held)", put: "(3 - held)" },
  { module: "randomness", name: "output multiplier off by one", find: "const MULT_B = 0x58f38ded;", put: "const MULT_B = 0x58f38dee;" },
  { module: "randomness", name: "pool multiplier off by one", find: "const MULT_A = 0x931e8875;", put: "const MULT_A = 0x931e8877;" },
  { module: "randomness", name: "fold shifts by 15", find: "product >>> 16", put: "product >>> 15" },
  { module: "randomness", name: "mix adds", find: "Math.imul(MIX_MULT_L, into) - Math.imul(MIX_MULT_R, from)", put: "Math.imul(MIX_MULT_L, into) + Math.imul(MIX_MULT_R, from)" },
  { module: "randomness", name: "pool words mix into themselves", find: "if (source === target) continue;", put: "" },
  { module: "randomness", name: "words beyond the pool are dropped", find: "for (let source = POOL_SIZE; source < key.length; source++)", put: "for (let source = POOL_SIZE; source < 0; source++)" },
  { module: "randomness", name: "output reads one pool word", find: "pool[index % POOL_SIZE]!", put: "pool[0]!" },
  { module: "randomness", name: "unit takes the second word", find: "randomWords(key, 1)[0]! / TWO_POW_32", put: "randomWords(key, 2)[1]! / TWO_POW_32" },
  { module: "randomness", name: "range forgets its low bound", find: "low + (high - low) * randomUnit(key)", put: "low + high * randomUnit(key)" },
  { module: "randomness", name: "pick counts negative weights", find: "if (weight > 0) total = total + weight;", put: "total = total + weight;" },
  { module: "randomness", name: "pick skips the mark test", find: "if (mark < running) return index;", put: "if (mark < running - weight * 0.5) return index;" },
  { module: "rig", name: "compose mixes a column", find: "parent[0] * local[2] + parent[2] * local[3],", put: "parent[0] * local[2] + parent[2] * local[1]," },
  { module: "rig", name: "inverse translation flips", find: "(matrix[2] * matrix[5] - matrix[3] * matrix[4]) / determinant", put: "(matrix[3] * matrix[4] - matrix[2] * matrix[5]) / determinant" },
  { module: "rig", name: "singular matrices are divided", find: "if (determinant === 0) return IDENTITY;", put: "" },
  { module: "rig", name: "transform reads the wrong translation", find: "matrix[1] * x + matrix[3] * y + matrix[5]", put: "matrix[1] * x + matrix[3] * y + matrix[4]" },
  { module: "rig", name: "rotation turns the other way", find: "const c = (0 - sine) * posed.scaleY;", put: "const c = sine * posed.scaleY;" },
  { module: "rig", name: "degrees are halved turns", find: ") / 360;", put: ") / 180;" },
  { module: "rig", name: "scale axes swap", find: "const a = cosine * posed.scaleX;", put: "const a = cosine * posed.scaleY;" },
  { module: "rig", name: "children lose the parent's translation", find: "pa * e + pc * f + pe,", put: "pa * e + pc * f," },
  { module: "rig", name: "inverse multiplies by the reciprocal (last bits only)", find: "matrix[0] / determinant,", put: "matrix[0] * (1 / determinant)," },
  { module: "rig", name: "world translation summed in another order (last bits only)", find: "pb * e + pd * f + pf);", put: "pf + pb * e + pd * f);" },
  { module: "rig", name: "every parent is the first bone", find: "return candidate;", put: "return 0;" },
  { module: "rig", name: "short poses are read past their end", find: "index < pose.length ? pose[index]! : REST", put: "pose[index]!" },
  { module: "rig", name: "rest pose is empty", find: "species.bones.map(() => REST)", put: "[]" },
  { module: "rig", name: "negative reach is taken literally", find: "distance + (reach > 0 ? reach : 0)", put: "distance + reach" },
  { module: "rig", name: "coinciding points divide", find: "if (distance === 0) return { x: 0, y: 0 };", put: "" },
  { module: "rig", name: "look offset keeps the distance", find: "{ x: dx / span, y: dy / span }", put: "{ x: dx / span, y: dy / distance }" },
];

/** 📋️ Copies one file, creating the directories above its target. */
function copy(from: string, to: string): void {
  mkdirSync(dirname(to), { recursive: true });
  copyFileSync(from, to);
}

/** 🧹️ Lays the untouched modules, suites, schema twin and vectors out in the sandbox. */
function restore(): void {
  for (const directory of Object.values(MODULES)) {
    copy(join(PETS, "🔨️modules", directory, "🟦️.ts"), join(SANDBOX, "🔨️modules", directory, "🟦️.ts"));
    copy(join(PETS, "🔨️modules", directory, UNIT), join(SANDBOX, "🔨️modules", directory, UNIT));
  }
  copy(join(PETS, "🧬️schema", "🟦️.ts"), join(SANDBOX, "🧬️schema", "🟦️.ts"));
  for (const directory of FIXTURES) for (const file of readdirSync(join(PETS, "🧫️fixtures", directory))) copy(join(PETS, "🧫️fixtures", directory, file), join(SANDBOX, "🧫️fixtures", directory, file));
  writeFileSync(join(SANDBOX, "vitest.config.ts"), 'export default { test: { include: ["🔨️modules/*/🧪️tests/🔬️unit/🟦️.ts"], environment: "node", passWithNoTests: false } };\n');
}

/** 🧪️ Whether the unit suites of the sandbox pass. */
function passes(): boolean {
  const run = Bun.spawnSync([process.execPath, join(ROOT, "node_modules", "vitest", "vitest.mjs"), "run", "--root", SANDBOX, "--config", join(SANDBOX, "vitest.config.ts")], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
  return run.exitCode === 0;
}

restore();
if (!passes()) {
  process.stdout.write("the untouched copy fails its unit suites\n");
  process.exit(1);
}
process.stdout.write("untouched copy: unit suites pass\n");
let survivors = 0;
for (const mutant of MUTANTS) {
  restore();
  const path = join(SANDBOX, "🔨️modules", MODULES[mutant.module], "🟦️.ts");
  const source = readFileSync(path, "utf8");
  if (!source.includes(mutant.find)) throw new Error(`mutant "${mutant.name}" finds nothing to change in ${mutant.module}`);
  writeFileSync(path, source.replace(mutant.find, mutant.put));
  const survived = passes();
  if (survived) survivors += 1;
  process.stdout.write(`${survived ? "SURVIVED" : "killed  "}  ${mutant.module}: ${mutant.name}\n`);
}
restore();
process.stdout.write(`${MUTANTS.length - survivors} of ${MUTANTS.length} mutants killed\n`);
process.exit(survivors === 0 ? 0 : 1);
