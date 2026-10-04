/** 🔬️ Probe of work package A4: prints what the climbing module answers for a small stage, to choose the numbers of its suites and vectors. `bun a4_probe_climbing.ts` from the repository root; the output goes to the terminal of whoever runs it. */
import * as climbing from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";
import { wallsOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🟦️.ts";

const size = { width: 40, height: 48 };
const floor = { surface: "floor", x0: 0, x1: 640, y: 480 };
const card = { surface: "card", x0: 306, x1: 494, y: 300 };
const solid = { x: 296, y: 300, width: 208, height: 100 };
const pitches = wallsOf([{ id: "card-left", surface: "card", side: -1, x: 300, y0: 300, y1: 400 }, { id: "card-right", surface: "card", side: 1, x: 500, y0: 300, y1: 400 }], [solid], 640, 480, 48, 72);
const lines: string[] = [];
const say = (label: string, value: unknown) => lines.push(`${label}: ${JSON.stringify(value)}`);
say("pitches", pitches);
const left = pitches[0]!;
say("cling", climbing.clingOf(left, size));
say("ledge", climbing.ledgeOf(left, size));
say("rim", climbing.rimOf(left, size));
say("foot", climbing.footOf(left, size));
say("grip floor", climbing.gripFor(floor, left, size));
say("grip card", climbing.gripFor(card, left, size));
say("climb ticks 432.8 → rim", climbing.climbTicks(432.8, climbing.rimOf(left, size)));
say("climb ticks down", climbing.climbTicks(climbing.rimOf(left, size), 432.8));
say("mantle", [0, 0.25, 0.5, 0.75, 1].map((phase) => climbing.mantlePath(left, size, phase)));
const ladder = climbing.ladderFor(floor, card, left, [solid], size);
say("ladder", ladder);
if (ladder !== null) {
  say("ladder length", climbing.ladderLength(ladder));
  say("ladder rungs", climbing.ladderRungs(ladder));
  say("ladder lean", climbing.ladderLean(ladder));
  say("ladder exit", climbing.ladderExit(ladder, size));
  say("ladder landing", climbing.ladderLanding(ladder, size));
  say("ladder holds", climbing.ladderHolds(ladder, [floor, card], pitches, [solid], size));
}
for (const x of [200, 280, 320, 400, 560]) {
  const shot = climbing.shotFor({ x, y: 480 }, [floor, card], [solid], size);
  say(`shot from ${x}`, shot);
  if (shot !== null) {
    say("  hook ticks", climbing.hookTicks(shot.muzzle, shot.hook, climbing.HOOK_SPEED));
    say("  zip ticks", climbing.zipTicks(shot, size));
    say("  landing", climbing.landingFor(shot, card, size));
  }
}
for (const gear of [["climb"], ["ladder"], ["grapple"], ["climb", "ladder", "grapple"], ["parachute"]] as const) say(`route up ${gear.join("+")}`, climbing.routeOf(200, floor, card, gear, size, climbing.GRIP_BUDGET, pitches, [], [solid]));
say("route down climb", climbing.routeOf(400, card, floor, ["climb"], size, climbing.GRIP_BUDGET, pitches, [], [solid]));
process.stdout.write(`${lines.join("\n")}\n`);
