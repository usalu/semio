/** 🧟️ Work package A8 mutation check: copies the effects and mischief modules, what they import, their unit suites and their vectors into `🗑️generated/a8/mutation`, breaks one expression at a time and reports whether the unit suites notice.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/mutate_effects.ts`
 * Nothing outside the sandbox is written. Exit code 1 when the untouched copy fails or a mutant survives.
 */

import { copyFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PETS = join(ROOT, "🧰️framework", "🛍️products", "🐾️pets");
const SANDBOX = join(import.meta.dir, "🗑️generated", "a8", "mutation");
const UNIT = join("🧪️tests", "🔬️unit", "🟦️.ts");
const MODULES = { effects: "✨️effects", mischief: "🪄️mischief" } as const;
const IMPORTED = ["📐️trigonometry", "🎲️randomness", "🏞️terrain", "🧗️climbing", "🪢️swing"];
const FIXTURES = ["✨️particle-motion", "🪄️mischief-choice"];

type Mutant = { readonly module: keyof typeof MODULES; readonly name: string; readonly find: string; readonly put: string };

const PICK = "return candidates[place >= count ? count - 1 : place > 0 ? place : 0]!;";

const MUTANTS: readonly Mutant[] = [
  { module: "effects", name: "hash multiplier off by two", find: "0x7feb352d", put: "0x7feb352f" },
  { module: "effects", name: "hash shifts by 16 in the middle", find: "first ^ (first >>> 15)", put: "first ^ (first >>> 16)" },
  { module: "effects", name: "mix forgets to add one", find: "Math.imul((b + 1) | 0, 0x85ebca6b)", put: "Math.imul(b | 0, 0x85ebca6b)" },
  { module: "effects", name: "mix xors another constant", find: "a ^ 0x9e3779b9", put: "a ^ 0x9e3779b1" },
  { module: "effects", name: "a life is rounded down", find: "emitter.life * TICKS_PER_SECOND + 0.5", put: "emitter.life * TICKS_PER_SECOND" },
  { module: "effects", name: "the swarm may exceed the cap", find: "Math.min(Math.max(Math.floor(emitter.count), 0), EMITTER_CAP)", put: "Math.min(Math.max(Math.floor(emitter.count), 0), EMITTER_CAP + 8)" },
  { module: "effects", name: "the period is rounded down", find: "Math.floor((lifeTicks(emitter) + swarm - 1) / swarm)", put: "Math.floor(lifeTicks(emitter) / swarm)" },
  { module: "effects", name: "bornAt reads another lane", find: "scattered(key, index, LANE_BIRTH) * period", put: "scattered(key, index, LANE_HEADING) * period" },
  { module: "effects", name: "births go on after the stop", find: "    if (until !== null && born >= until) continue;\n", put: "" },
  { module: "effects", name: "a particle lives a tick too long", find: "if (age < 0 || age >= life) continue;", put: "if (age < 0 || age > life) continue;" },
  { module: "effects", name: "the window of indices starts too late", find: "Math.max(0, Math.floor((elapsed - life) / period))", put: "Math.max(0, Math.floor((elapsed - life) / period) + 2)" },
  { module: "effects", name: "an emitter's cap drops the youngest", find: "particles.slice(particles.length - swarm)", put: "particles.slice(0, swarm)" },
  { module: "effects", name: "an emitter has no cap", find: "return particles.length > swarm ? particles.slice(particles.length - swarm) : particles;", put: "return particles;" },
  { module: "effects", name: "fall heads upwards", find: "const heading = DOWN + (scattered(key, index, LANE_HEADING) - 0.5) * emitter.spread;", put: "const heading = UP + (scattered(key, index, LANE_HEADING) - 0.5) * emitter.spread;" },
  { module: "effects", name: "fall sways at any speed", find: "fastNegExp(pace / FALL_SWAY_SPEED)", put: "1" },
  { module: "effects", name: "fall never fades out", find: " * (1 - smoothstep((share - 0.75) / 0.25))", put: "" },
  { module: "effects", name: "rise wanders evenly", find: "sinTurns(swing) * (0.3 + share)", put: "sinTurns(swing) * 0.3" },
  { module: "effects", name: "rise pops in at once", find: "scale: 0.5 + 0.5 * smoothstep(age / 8),", put: "scale: 1," },
  { module: "effects", name: "burst is not stratified", find: "(index + scattered(key, index, LANE_HEADING)) / swarm", put: "scattered(key, index, LANE_HEADING)" },
  { module: "effects", name: "burst does not sink", find: " + 0.5 * BURST_GRAVITY * seconds * seconds", put: "" },
  { module: "effects", name: "burst is not braked", find: "BURST_DRAG * (1 - fastNegExp(seconds / BURST_DRAG))", put: "seconds" },
  { module: "effects", name: "burst fades linearly", find: "opacity: 1 - share * share,", put: "opacity: 1 - share," },
  { module: "effects", name: "a mirrored burst points the same way", find: "rotation: facing > 0 ? heading : 0.5 - heading,", put: "rotation: heading," },
  { module: "effects", name: "a mirrored orbit swaps front and back", find: "facing > 0 ? turned : 0.5 - turned", put: "facing * turned" },
  { module: "effects", name: "orbit radius uses 6 for two pi", find: "/ TURN_RADIANS;", put: "/ 6;" },
  { module: "effects", name: "orbit is a circle", find: "y: origin.y + radius * ORBIT_SQUASH * depth,", put: "y: origin.y + radius * depth," },
  { module: "effects", name: "orbit does not fade out", find: "(until === null ? 1 : 1 - smoothstep((tick - until) / ORBIT_FADE))", put: "1" },
  { module: "effects", name: "orbit vanishes at the stop", find: "tick >= until + ORBIT_FADE", put: "tick >= until" },
  { module: "effects", name: "drift meanders along its path", find: "- meander * across", put: "+ meander * across" },
  { module: "effects", name: "drift heads downwards", find: "const heading = AHEAD + ", put: "const heading = DOWN + " },
  { module: "effects", name: "the cap keeps the eldest", find: "if (particle.age < eldest) kept.push(particle);", put: "if (particle.age > eldest) kept.push(particle);" },
  { module: "effects", name: "the cap keeps every particle of the last age", find: "else if (particle.age === eldest && peers > 0) {", put: "else if (particle.age === eldest) {" },
  { module: "effects", name: "the cap ignores fractions the other way", find: "const room = Math.floor(cap);", put: "const room = Math.ceil(cap);" },
  { module: "effects", name: "a burst ends a tick early", find: 'if (emitter.motion === "burst") return since + life;', put: 'if (emitter.motion === "burst") return since + life - 1;' },
  { module: "effects", name: "a continuous emitter ends a tick early", find: "until + life - 1;", put: "until + life - 2;" },
  { module: "effects", name: "an orbit may end before it began", find: "return Math.max(since, until + ORBIT_FADE);", put: "return until + ORBIT_FADE;" },
  { module: "effects", name: "an endless burst has no end", find: '  if (emitter.motion === "burst") return since + life;\n  if (until === null) return null;', put: '  if (until === null) return null;\n  if (emitter.motion === "burst") return since + life;' },
  { module: "mischief", name: "a ground covers whatever it begins", find: 'return key.length === ground.length || key[ground.length] === "/";', put: "return true;" },
  { module: "mischief", name: "an empty ground covers keys with a leading slash", find: "if (ground.length === 0 || !key.startsWith(ground)) return false;", put: "if (!key.startsWith(ground)) return false;" },
  { module: "mischief", name: "a ground covers only itself", find: 'return key.length === ground.length || key[ground.length] === "/";', put: "return key.length === ground.length;" },
  { module: "mischief", name: "a fixture is listed once per fitting ground", find: "      fitting.push(fixture);\n      break;", put: "      fitting.push(fixture);" },
  { module: "mischief", name: "the pick rounds up", find: "Math.floor(unit * count)", put: "Math.ceil(unit * count)" },
  { module: "mischief", name: "the pick prefers a correct item (leak)", find: PICK, put: "return (candidates.find((candidate) => (candidate as { correct?: boolean }).correct === true) ?? candidates[Math.min(count - 1, Math.max(0, Math.floor(unit * count)))])!;" },
  { module: "mischief", name: "the pick prefers an answered item (leak)", find: PICK, put: "return (candidates.find((candidate) => (candidate as { answered?: unknown }).answered != null) ?? candidates[Math.min(count - 1, Math.max(0, Math.floor(unit * count)))])!;" },
  { module: "mischief", name: "the candidates are ordered by value (leak)", find: "  return fitting;", put: "  return fitting.sort((left, right) => String((left as { value?: unknown }).value).localeCompare(String((right as { value?: unknown }).value)));" },
  { module: "mischief", name: "patience ignores a time of concentration", find: "return quiet ? MISCHIEF_PATIENCE_QUIET : MISCHIEF_PATIENCE;", put: "return MISCHIEF_PATIENCE;" },
  { module: "mischief", name: "the width gate is strict", find: "!(circumstances.width >= MISCHIEF_WIDTH)", put: "!(circumstances.width > MISCHIEF_WIDTH)" },
  { module: "mischief", name: "a lift in progress is ignored", find: " || circumstances.lifting", put: "" },
  { module: "mischief", name: "a coarse pointer is ignored", find: " || !circumstances.fine", put: "" },
  { module: "mischief", name: "consent is ignored", find: " || !circumstances.permitted", put: "" },
  { module: "mischief", name: "the earlier of the two waits counts", find: "return Math.max(circumstances.stirred", put: "return Math.min(circumstances.stirred" },
  { module: "mischief", name: "allowed a tick late", find: "circumstances.tick >= from", put: "circumstances.tick > from" },
  { module: "mischief", name: "lively waits as long as calm", find: 'mode === "lively" ? MISCHIEF_COOLDOWN_LIVELY', put: 'mode === "lively" ? MISCHIEF_COOLDOWN_CALM' },
  { module: "mischief", name: "a station's room is on the near side", find: 'footing: "wall", wall: pitch.wall, surface: pitch.surface, x: pitch.x, y, side: 1, room: width - right', put: 'footing: "wall", wall: pitch.wall, surface: pitch.surface, x: pitch.x, y, side: 1, room: left' },
  { module: "mischief", name: "the later station wins a tie", find: "next.room > best.room", put: "next.room >= best.room" },
  { module: "mischief", name: "a station needs no room", find: "next.room >= LIFT_ROOM && ", put: "" },
  { module: "mischief", name: "a wall counts from either side", find: "pitch.side === -1 && ", put: "" },
  { module: "mischief", name: "the feet may leave the stretch", find: "const y = clamp(bottom, pitch.y0, pitch.y1);", put: "const y = bottom;" },
  { module: "mischief", name: "a perch must not lie below the lower edge", find: "perch.y <= bottom + STATION_STEP", put: "perch.y <= bottom" },
  { module: "mischief", name: "a perch may lie above the fixture", find: "perch.y > top && ", put: "" },
  { module: "mischief", name: "a perch may be far away", find: "perch.x0 < left && left - Math.min(perch.x1, left) <= STATION_GAP", put: "perch.x0 < left" },
  { module: "mischief", name: "a shove ignores the draw", find: "(LIFT_LEAST + (1 - LIFT_LEAST) * unit)", put: "1" },
  { module: "mischief", name: "a negative room pulls", find: "Math.max(0, room) *", put: "room *" },
  { module: "mischief", name: "a wide copy tilts as much as a narrow one", find: "span > 0 ? Math.min(LIFT_TILT, LIFT_RISE / (HALF_TURN_RADIANS * span)) : LIFT_TILT", put: "LIFT_TILT" },
  { module: "mischief", name: "the copy gives while it appears", find: "  if (age < LIFT_FADE) return { dx: 0, dy: 0, tilt: 0, opacity: smoothstep(age / LIFT_FADE) };\n", put: "" },
  { module: "mischief", name: "the wobble never calms", find: "const calm = 1 - smoothstep(phase);", put: "const calm = 1;" },
  { module: "mischief", name: "the copy tips the same way on its way home", find: "tilt: toward(side, 0 - lean * hump), opacity: 1 };", put: "tilt: toward(side, lean * hump), opacity: 1 };" },
  { module: "mischief", name: "the copy does not rise on its way out", find: "dx: toward(side, travel * smoothstep(phase)), dy: 0 - LIFT_RISE * hump,", put: "dx: toward(side, travel * smoothstep(phase)), dy: 0," },
  { module: "mischief", name: "every shove goes right", find: "return side > 0 ? value : 0 - value;", put: "return value;" },
  { module: "mischief", name: "the copy stays visible after the lift", find: "opacity: 1 - smoothstep((age - LIFT_RETURNS - LIFT_RETURN) / LIFT_FADE) };", put: "opacity: 1 };" },
  { module: "mischief", name: "the stage sleeps through putting back", find: "? since + LIFT_RETURNS : tick + 1", put: "? since + LIFT_TICKS : tick + 1" },
  { module: "mischief", name: "the stage is woken during the rest", find: "return age >= LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE && age < LIFT_RETURNS ? since + LIFT_RETURNS : tick + 1;", put: "return tick + 1;" },
  { module: "mischief", name: "a pusher is thrown towards the fixture", find: "? 0 - speed : speed", put: "? speed : 0 - speed" },
  { module: "mischief", name: "a throw ignores the draw", find: "THROW_SPEED + THROW_SPREAD * unit", put: "THROW_SPEED" },
  { module: "mischief", name: "a pusher is thrown downwards", find: "vy: 0 - THROW_LIFT", put: "vy: THROW_LIFT" },
];

/** 📋️ Copies one file, creating the directories above its target. */
function copy(from: string, to: string): void {
  mkdirSync(dirname(to), { recursive: true });
  copyFileSync(from, to);
}

/** 🧹️ Lays the untouched modules, what they import, their suites, the schema twin, the level reader and the vectors out in the sandbox. */
function restore(): void {
  for (const directory of Object.values(MODULES)) {
    copy(join(PETS, "🔨️modules", directory, "🟦️.ts"), join(SANDBOX, "🔨️modules", directory, "🟦️.ts"));
    copy(join(PETS, "🔨️modules", directory, UNIT), join(SANDBOX, "🔨️modules", directory, UNIT));
  }
  for (const directory of IMPORTED) copy(join(PETS, "🔨️modules", directory, "🟦️.ts"), join(SANDBOX, "🔨️modules", directory, "🟦️.ts"));
  copy(join(PETS, "🧬️schema", "🟦️.ts"), join(SANDBOX, "🧬️schema", "🟦️.ts"));
  copy(join(PETS, "🧪️tests", "🎚️config", "🟦️.ts"), join(SANDBOX, "🧪️tests", "🎚️config", "🟦️.ts"));
  for (const directory of FIXTURES) for (const file of readdirSync(join(PETS, "🧫️fixtures", directory))) copy(join(PETS, "🧫️fixtures", directory, file), join(SANDBOX, "🧫️fixtures", directory, file));
  writeFileSync(join(SANDBOX, "vitest.config.ts"), `export default { test: { include: [${Object.values(MODULES).map((directory) => `"🔨️modules/${directory}/🧪️tests/🔬️unit/🟦️.ts"`).join(", ")}], environment: "node", passWithNoTests: false } };\n`);
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
  const source = readFileSync(path, "utf8").replaceAll("\r\n", "\n");
  if (!source.includes(mutant.find)) throw new Error(`mutant "${mutant.name}" finds nothing to change in ${mutant.module}`);
  writeFileSync(path, source.replace(mutant.find, mutant.put));
  const survived = passes();
  if (survived) survivors += 1;
  process.stdout.write(`${survived ? "SURVIVED" : "killed  "}  ${mutant.module}: ${mutant.name}\n`);
}
restore();
process.stdout.write(`${MUTANTS.length - survivors} of ${MUTANTS.length} mutants killed\n`);
process.exit(survivors === 0 ? 0 : 1);
