/** 🧟️ Work package A9 mutation check: copies the React target (barrel, stylesheet, modules) into `🗑️generated/a9/mutation`, breaks one expression of the gear or the effects at a time and reports whether the suites `🧰️gear-depiction` and `🎆️effect-painting` notice.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_a9_mutants.ts [name fragment…]`
 * Nothing outside the sandbox is written. Exit code 1 when the untouched copy fails or a mutant survives.
 */

import { copyFileSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const TARGET = join(ROOT, "🧰️framework", "🛍️products", "🐾️pets", "🎯️targets", "⚛️react");
const PACKAGE = join(TARGET, "📦️packages", "🟦️typescript");
const SANDBOX = join(import.meta.dir, "🗑️generated", "a9", "mutation");
const CONFIG = join(import.meta.dir, "wp_a9_vitest.config.ts");
const GEAR = join("🔨️modules", "🧰️gear", "🟦️.ts");
const EFFECTS = join("🔨️modules", "✨️effects", "🟦️.ts");
const DEPICTION = join("🔨️modules", "🖌️depiction", "🟦️.ts");

type Mutant = { readonly file: string; readonly name: string; readonly find: string; readonly put: string };

const MUTANTS: readonly Mutant[] = [
  { file: DEPICTION, name: "a mirrored drawing tilts the same way inside its box", find: "rounded((facing < 0 ? -tilt : tilt) * 360, PLACE)", put: "rounded(tilt * 360, PLACE)" },
  { file: DEPICTION, name: "tilt about the feet instead of the pivot", find: "const px = rounded(pivot.x, PLACE);\n  const py = rounded(pivot.y, PLACE);", put: "const px = 0;\n  const py = 0;" },
  { file: GEAR, name: "the tools keep the mirror", find: "${rounded(cos * facing, AXIS)} ${rounded(-sin * facing, AXIS)}", put: "${rounded(cos, AXIS)} ${rounded(-sin, AXIS)}" },
  { file: GEAR, name: "the tools turn with the tilt instead of against it", find: "${rounded(-sin * facing, AXIS)} ${rounded(sin, AXIS)}", put: "${rounded(sin * facing, AXIS)} ${rounded(-sin, AXIS)}" },
  { file: GEAR, name: "the tools' frame slips off the pivot", find: "rounded(px - (cos * px + sin * py))", put: "rounded(px - (cos * px - sin * py))" },
  { file: GEAR, name: "attachment points ignore the tilt", find: "sin * (x - px) + cos * (y - py) + py]", put: "y]" },
  { file: GEAR, name: "attachment points ignore the mirror", find: "(cos * (x - px) - sin * (y - py) + px) * facing", put: "(cos * (x - px) - sin * (y - py) + px)" },
  { file: GEAR, name: "the cords miss the rim of a canopy that opens", find: "rounded((left.x + (right.x - left.x) * cord) * wide, PLACE)", put: "rounded(left.x + (right.x - left.x) * cord, PLACE)" },
  { file: GEAR, name: "the cords ignore how high the rim lies", find: "rounded(rise + (left.y + (right.y - left.y) * cord) * open, PLACE)", put: "rise" },
  { file: GEAR, name: "the rim of an ellipse lies at the origin's height", find: "{ x: shape.cx - shape.rx, y: shape.cy },", put: "{ x: shape.cx - shape.rx, y: 0 }," },
  { file: GEAR, name: "the rim of a rectangle is its top", find: "{ x: shape.x, y: shape.y + shape.height },", put: "{ x: shape.x, y: shape.y }," },
  { file: GEAR, name: "of two points as far out the upper one is the rim", find: "(point.x === left.x && point.y > left.y)", put: "(point.x === left.x && point.y < left.y)" },
  { file: GEAR, name: "relative path commands are read as absolute", find: "const relative = command !== kind;", put: "const relative = false;" },
  { file: GEAR, name: "control points count as points of the outline", find: "x = (relative ? x : 0) + values[arity - 2]!;", put: "x = (relative ? x : 0) + values[0]!;" },
  { file: GEAR, name: "closing an outline does not return to its start", find: "        x = startX;\n        y = startY;\n", put: "" },
  { file: GEAR, name: "an outline that cannot be read gives a rim of nothing", find: "if (!Number.isFinite(point.x) || !Number.isFinite(point.y)) return plain;", put: "" },
  { file: GEAR, name: "the parachute hangs from the top of the box", find: "carried(0, -species.grip)", put: "carried(0, -height)" },
  { file: GEAR, name: "an overshooting canopy is not squashed", find: "open > 1 ? 1 - CHUTE_SQUASH * (open - 1)", put: "open > 1 ? 1" },
  { file: GEAR, name: "the canopy does not stream out while it opens", find: "(CHUTE_STREAM + (1 - CHUTE_STREAM) * Math.min(open, 1))", put: "1" },
  { file: GEAR, name: "a packed parachute shows", find: "chute !== undefined && rounded(chute.open) > 0)", put: "chute !== undefined)" },
  { file: GEAR, name: "the rope hangs half its slack", find: "(ny + fy) / 2 + 2 * sag", put: "(ny + fy) / 2 + sag" },
  { file: GEAR, name: "the rope leaves an unmirrored pivot", find: "gun === undefined ? px * facing :", put: "gun === undefined ? px :" },
  { file: GEAR, name: "the rope leaves the hand, not the muzzle", find: "hx + cosTurns(gun.aim) * GUN_MUZZLE", put: "hx" },
  { file: GEAR, name: "the rope ends at a point of the drawing, not of the stage", find: "const fx = rope.x - bearer.x;", put: "const fx = rope.x;" },
  { file: GEAR, name: "the hook ignores how a slack rope arrives", find: "endX = fx - (sag === 0 ? nx : mx);", put: "endX = fx - nx;" },
  { file: GEAR, name: "a hook without a rope points right", find: "const uy = reach > 0 ? endY / reach : -1;\n    if (moved(painted, HOOK", put: "const uy = reach > 0 ? endY / reach : 0;\n    if (moved(painted, HOOK" },
  { file: GEAR, name: "the gun never turns its handle down", find: "cosTurns(gun.aim) < 0 ? -1 : 1", put: "1" },
  { file: GEAR, name: "the gun points in turns, not degrees", find: "rounded(gun.aim * 360, PLACE), upside", put: "rounded(gun.aim, PLACE), upside" },
  { file: GEAR, name: "the rungs start a whole spacing from the foot", find: "((rung + 0.5) * length) / rungs", put: "((rung + 1) * length) / rungs" },
  { file: GEAR, name: "the rungs are skewed to the rails", find: "${-painted[slot + 1]!} ${painted[slot]} ${painted[slot + 2]}", put: "${painted[slot + 1]!} ${painted[slot]} ${painted[slot + 2]}" },
  { file: GEAR, name: "a raised ladder sinks below the feet", find: "const sunk = Math.max(0, holdY + Math.abs(ay));", put: "const sunk = 0;" },
  { file: GEAR, name: "a carried ladder has one rung too many", find: "Math.max(1, Math.floor(ladder.length / LADDER_RUNG))", put: "Math.max(1, Math.ceil(ladder.length / LADDER_RUNG))" },
  { file: GEAR, name: "a carried ladder leans the other way", find: "const ax = (sinTurns(ladder.lean) * ladder.length) / 2;", put: "const ax = (-sinTurns(ladder.lean) * ladder.length) / 2;" },
  { file: GEAR, name: "every number counts as changed", find: "if (values[index] === painted[slot + index]) continue;", put: "" },
  { file: GEAR, name: "visibility is written with every frame", find: "if (painted[slot] !== flag) {", put: "if (flag >= 0) {" },
  { file: GEAR, name: "the tools are built showing", find: "[VISIBILITY, \"hidden\"],\n  ];\n  const back", put: "[VISIBILITY, \"visible\"],\n  ];\n  const back" },
  { file: GEAR, name: "standing ladders ignore the size pets are drawn at", find: "if (moved(painted, 0, [rounded(scale)])) element.style.transform = `scale(${painted[0]})`;", put: "" },
  { file: GEAR, name: "a ladder without rungs or opacity stays", find: "ladder !== undefined && ladder.rungs >= 1 && rounded(ladder.opacity) > 0)", put: "ladder !== undefined)" },
  { file: GEAR, name: "a standing ladder forgets its opacity", find: "if (moved(painted, slot + 6, [rounded(opacity)])) parts.group.setAttribute(\"opacity\", String(painted[slot + 6]));", put: "" },
  { file: GEAR, name: "a ladder is rebuilt for every frame", find: "for (let index = rack.ladders.length; index < ladders.length; index++) {", put: "for (let index = 0; index < ladders.length; index++) {" },
  { file: EFFECTS, name: "an element that comes back into use stays hidden", find: "if (index >= pool.live) element.setAttribute(VISIBILITY, \"visible\");", put: "" },
  { file: EFFECTS, name: "elements that fell out of use stay visible", find: "for (let index = pool.used; index < pool.live; index++) pool.elements[index]!.setAttribute(VISIBILITY, \"hidden\");", put: "" },
  { file: EFFECTS, name: "particles turn in turns, not degrees", find: "rounded(particle.rotation * 360, PLACE)", put: "rounded(particle.rotation, PLACE)" },
  { file: EFFECTS, name: "particles ignore their size", find: "scale(${grown})`", put: "scale(1)`" },
  { file: EFFECTS, name: "every particle is rewritten with every frame", find: "if (x !== painted[slot] || y !== painted[slot + 1] || turn !== painted[slot + 2] || grown !== painted[slot + 3]) {", put: "if (x === x) {" },
  { file: EFFECTS, name: "opacity is rewritten with every frame", find: "if (opacity !== painted[slot + 4]) {", put: "if (opacity === opacity) {" },
  { file: EFFECTS, name: "a frame without particles still writes", find: "if (particles.length === 0 && effects.live === 0) return;", put: "" },
  { file: EFFECTS, name: "the pool holds one element too few", find: "const size = Math.max(0, Math.floor(emitter.count));", put: "const size = Math.max(0, Math.floor(emitter.count) - 1);" },
  { file: EFFECTS, name: "pools are shared between the emitters of a species", find: "troupe.pools.set(emitter.id, pool);", put: "troupe.pools.set(\"any\", pool);\n  if (troupe.pools.size > 0) return troupe.pools.get(\"any\")!;" },
  { file: EFFECTS, name: "the root ignores the size pets are drawn at", find: "effects.element.style.transform = `scale(${size})`;", put: "" },
  { file: EFFECTS, name: "a state without a tint keeps the last tint", find: "const colour = tint?.[tone] ?? palette[tone];", put: "const colour = tint?.[tone] ?? (element.style.getPropertyValue(name) || palette[tone]);" },
  { file: EFFECTS, name: "the palette is written with every call", find: "if (element.style.getPropertyValue(name) !== colour) element.style.setProperty(name, colour);", put: "element.style.setProperty(name, colour);" },
  { file: EFFECTS, name: "a tint given before the first particle is forgotten", find: "tintPalette(group, species.palette, effects.tints.get(species.id));", put: "tintPalette(group, species.palette);" },
  { file: EFFECTS, name: "a struck root stays in its layer", find: "  effects.element.remove();\n", put: "" },
  { file: EFFECTS, name: "a retired species still counts as alive", find: "for (const pool of troupe.pools.values()) effects.live -= pool.live;", put: "" },
  { file: EFFECTS, name: "a retired species keeps its tint", find: "effects.tints.delete(id);", put: "" },
];

function copyTree(from: string, to: string): void {
  mkdirSync(to, { recursive: true });
  for (const entry of readdirSync(from)) {
    const source = join(from, entry);
    if (statSync(source).isDirectory()) copyTree(source, join(to, entry));
    else copyFileSync(source, join(to, entry));
  }
}

function runSuites(barrel: string): boolean {
  const run = Bun.spawnSync([process.execPath, join(ROOT, "node_modules", "vitest", "vitest.mjs"), "run", "--config", CONFIG], { cwd: PACKAGE, env: { ...process.env, WP_A9_BARREL: barrel }, stdout: "pipe", stderr: "pipe" });
  return run.exitCode === 0;
}

const wanted = process.argv.slice(2);
copyFileSync(join(TARGET, "🟦️.tsx"), (mkdirSync(SANDBOX, { recursive: true }), join(SANDBOX, "🟦️.tsx")));
copyFileSync(join(TARGET, "🎨️.css"), join(SANDBOX, "🎨️.css"));
copyTree(join(TARGET, "🔨️modules"), join(SANDBOX, "🔨️modules"));
const barrel = join(SANDBOX, "🟦️.tsx");
if (!runSuites(barrel)) {
  process.stdout.write("the untouched copy fails\n");
  process.exit(1);
}
process.stdout.write("the untouched copy passes\n");
let survivors = 0;
for (const mutant of MUTANTS) {
  if (wanted.length > 0 && !wanted.some((fragment) => mutant.name.includes(fragment))) continue;
  const path = join(SANDBOX, mutant.file);
  const original = readFileSync(join(TARGET, mutant.file), "utf8");
  if (original.split(mutant.find).length !== 2) {
    process.stdout.write(`NOT APPLICABLE  ${mutant.name}: the expression occurs ${original.split(mutant.find).length - 1} times\n`);
    survivors++;
    continue;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, original.replace(mutant.find, () => mutant.put));
  const noticed = !runSuites(barrel);
  writeFileSync(path, original);
  if (!noticed) survivors++;
  process.stdout.write(`${noticed ? "killed  " : "SURVIVED"}  ${mutant.name}\n`);
}
process.stdout.write(`${MUTANTS.length} mutants, ${survivors} survived or did not apply\n`);
process.exit(survivors === 0 ? 0 : 1);
