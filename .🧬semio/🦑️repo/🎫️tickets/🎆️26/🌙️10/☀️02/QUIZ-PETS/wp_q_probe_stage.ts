/** 🎪️ Probe of work package Q: what the statistical sessions of the stage unit suite count for shorter and longer sessions and other seeds, so the amounts per test level can be chosen with their thresholds known to hold. `bun wp_q_probe_stage.ts`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ACTIVITIES, TICKS_PER_SECOND, type Actor, type Menagerie, type PetMode, type Rect, type Slug, type Species, type Stage, type StageEvent, type Surface } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { MODE_LIMITS } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧠️behavior/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const FIXTURES = resolve(HERE, "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures");
const SECOND = TICKS_PER_SECOND;
const WIDTH = 1280;
const HEIGHT = 720;
const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };
const CARD: Surface = { id: "card", x0: 300, x1: 800, y: 400 };
const FAR = 100000000;

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(FIXTURES, name, "🔣️.json"), "utf8")) as T;
}

const COMPANIES = [
  { name: "sample", menagerie: fixture<{ menagerie: Menagerie }>("🧬️schema-conformance").menagerie, walker: "blobby", hopper: "hoppy" },
  { name: "troupe", menagerie: fixture<{ menagerie: Menagerie }>("🎪️stage-trace").menagerie, walker: "mossy", hopper: "sparky" },
];

/** 📣️ One line of findings. */
function say(line: string): void {
  process.stdout.write(`${line}\n`);
}

/** 🧬️ A species of a menagerie. */
function kindOf(menagerie: Menagerie, id: Slug): Species {
  return menagerie.species.find((species) => species.id === id)!;
}

/** 🧸️ The actor of a species on a stage. */
function actorOf(stage: Stage, species: Slug): Actor {
  return stage.actors.find((candidate) => candidate.species === species)!;
}

/** 🎬️ A stage on which species were summoned onto surveyed surfaces. */
function staged(menagerie: Menagerie, species: readonly Slug[], options: { seed?: number; mode?: PetMode; surfaces?: readonly Surface[]; keepouts?: readonly Rect[] } = {}): Stage {
  return advance(menagerie, openStage(options.seed ?? 1), [{ kind: "tuned", mode: options.mode ?? "calm" }, { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces: options.surfaces ?? [CARD, FLOOR], keepouts: options.keepouts ?? [] }, { kind: "summoned", species }]);
}

/** 🏃️ A stage after `ticks` single ticks; `watch` sees every stage with the one before it. */
function run(menagerie: Menagerie, stage: Stage, ticks: number, watch?: (after: Stage, before: Stage) => void): Stage {
  let current = stage;
  const tick: StageEvent[] = [{ kind: "ticked", ticks: 1 }];
  for (let done = 0; done < ticks; done++) {
    const next = advance(menagerie, current, tick);
    if (watch !== undefined) watch(next, current);
    current = next;
  }
  return current;
}

/** ✍️ A stage in which the actor of a species is written anew. */
function craft(stage: Stage, species: Slug, changes: Partial<Actor>): Stage {
  return { ...stage, actors: stage.actors.map((actor) => (actor.species === species ? { ...actor, ...changes } : actor)) };
}

for (const company of COMPANIES) {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((species) => species.id);
  const walker = company.walker;
  const kind = kindOf(menagerie, walker);
  say(`\n=== ${company.name} (${everyone.length} species) ===`);

  say("liveliness: per mode and seed, at 60/90/120/150/200/300/600 s: rest share, fidget starts, walk+hop starts, most movers, moving share");
  for (const mode of ["calm", "lively"] as const) {
    for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
      const starts: Record<string, number> = Object.fromEntries(ACTIVITIES.map((activity) => [activity, 0]));
      let resting = 0;
      let all = 0;
      let movers = 0;
      let moving = 0;
      const lines: string[] = [];
      let ticks = 0;
      run(menagerie, staged(menagerie, everyone, { seed, mode }), 600 * SECOND, (after, before) => {
        let now = 0;
        for (const actor of after.actors) {
          all++;
          if (actor.activity === "idle" || actor.activity === "sleep") resting++;
          if (actor.activity === "walk" || actor.activity === "hop") now++;
          const earlier = before.actors.find((candidate) => candidate.species === actor.species);
          if (earlier !== undefined && (earlier.activity !== actor.activity || earlier.since !== actor.since) && !(earlier.activity === "idle" && actor.activity === "idle")) starts[actor.activity]!++;
        }
        movers = Math.max(movers, now);
        if (now > 0) moving++;
        ticks++;
        if ([60, 90, 120, 150, 200, 300, 600].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s rest ${(resting / all).toFixed(3)} fid ${starts.fidget} walk ${starts.walk! + starts.hop!} (walk only ${starts.walk}) mov ${movers} moving ${(moving / ticks).toFixed(3)}`);
      });
      say(`  ${mode} seed ${seed}: ${lines.join(" | ")}`);
    }
  }

  say("goals: walks begun by a lonely walker on the floor, per mode and seed at 60/120/200/300 s");
  for (const mode of ["calm", "lively"] as const) {
    for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
      let walks = 0;
      let ticks = 0;
      const lines: string[] = [];
      run(menagerie, staged(menagerie, [walker], { seed, mode, surfaces: [FLOOR] }), 300 * SECOND, (after, before) => {
        if (actorOf(after, walker).activity === "walk" && actorOf(before, walker).activity !== "walk") walks++;
        ticks++;
        if ([60, 120, 200, 300].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${walks}`);
      });
      say(`  ${mode} seed ${seed}: ${lines.join(" | ")}`);
    }
  }

  say("encounters by walking: per mode and seed, met/approaches/partners/movers at 120/180/240/300/480 s");
  const acts = (actor: Actor): boolean => actor.activity === "greet" || actor.activity === "cuddle" || actor.activity === "squabble";
  for (const mode of ["calm", "lively"] as const) {
    for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
      let met = 0;
      let approaches = 0;
      let partners = 0;
      let movers = 0;
      let ticks = 0;
      const lines: string[] = [];
      run(menagerie, staged(menagerie, everyone, { seed, mode, surfaces: [{ ...CARD, x0: 200, x1: 700 }] }), 480 * SECOND, (after, before) => {
        let partnered = 0;
        let walking = 0;
        let acting = 0;
        for (const actor of after.actors) {
          if (actor.partner !== null) partnered++;
          if ((actor.activity === "walk" || actor.activity === "hop") && !actor.leaving) walking++;
          if (actor.partner !== null && acts(actor)) acting++;
        }
        partners = Math.max(partners, partnered);
        movers = Math.max(movers, walking);
        if (partnered === 2 && before.actors.every((actor) => actor.partner === null)) approaches++;
        if (acting === 2 && !before.actors.some(acts)) met++;
        ticks++;
        if ([120, 180, 240, 300, 480].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s met ${met} appr ${approaches} part ${partners} mov ${movers}`);
      });
      say(`  ${mode} seed ${seed} (gap ${MODE_LIMITS[mode].encounterGap / SECOND}s): ${lines.join(" | ")}`);
    }
  }

  say("the gap between encounters (seed 5 and others): seconds at which an approach begins within 600 s");
  for (const mode of ["calm", "lively"] as const) {
    for (const seed of [5, 1, 2, 3]) {
      const begun: number[] = [];
      run(menagerie, staged(menagerie, everyone, { seed, mode, surfaces: [{ ...CARD, x0: 200, x1: 700 }] }), 600 * SECOND, (after, before) => {
        if (before.actors.every((actor) => actor.partner === null) && after.actors.some((actor) => actor.partner !== null)) begun.push(after.tick);
      });
      say(`  ${mode} seed ${seed}: ${begun.map((tick) => (tick / SECOND).toFixed(0)).join(", ")}`);
    }
  }

  say("distance: walks begun on the narrow card per seed at 60/120/300 s (lively)");
  const widthOf = (id: Slug): number => kindOf(menagerie, id).size.width;
  const widest = Math.max(...everyone.map(widthOf));
  const need = everyone.reduce((sum, id) => sum + widthOf(id), 0) + 8 * Math.max(everyone.length - 1, 0);
  const narrow: Surface = { ...CARD, x0: 200, x1: 200 + need + 3 * widest };
  for (const seed of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]) {
    let walks = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [narrow] }), 300 * SECOND, (after, before) => {
      for (const actor of after.actors) {
        const earlier = before.actors.find((candidate) => candidate.species === actor.species);
        if (earlier !== undefined && actor.activity === "walk" && earlier.activity !== "walk") walks++;
      }
      ticks++;
      if ([60, 120, 300].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${walks}`);
    });
    say(`  seed ${seed}: ${lines.join(" | ")}`);
  }

  say("crowd: relocated (ticks on the card of somebody who was not on stage the tick before) and hops per seed at 60/120/240 s");
  for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
    let stage = run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [FLOOR] }), 2 * SECOND);
    stage = advance(menagerie, stage, [{ kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces: [CARD, FLOOR], keepouts: [] }]);
    let relocated = 0;
    let hops = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, stage, 240 * SECOND, (after, before) => {
      for (const actor of after.actors) {
        if (actor.activity === "hop") hops++;
        if (actor.perch === CARD.id && !before.actors.some((earlier) => earlier.species === actor.species)) relocated++;
      }
      ticks++;
      if ([60, 120, 240].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s relocated ${relocated} hops ${hops}`);
    });
    say(`  seed ${seed}: ${lines.join(" | ")}`);
  }

  say("hopping between two shelves: hops begun per seed at 60/120/200/300 s, without a keep-out, under a far one, under a bar");
  const low: Surface = { id: "low", x0: 300, x1: 500, y: 400 };
  const high: Surface = { id: "high", x0: 520, x1: 800, y: 380 };
  const variants: [string, Rect[]][] = [["free", []], ["far", [{ x: 0, y: 0, width: WIDTH, height: 40 }]], ["bar", [{ x: 0, y: 0, width: WIDTH, height: high.y - kind.size.height - 2 }]]];
  for (const [name, keepouts] of variants) {
    for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
      const arrived = staged(menagerie, [walker], { seed, mode: "lively", surfaces: [low, high], keepouts });
      let count = 0;
      let ticks = 0;
      const lines: string[] = [];
      run(menagerie, craft(arrived, walker, { perch: low.id, x: low.x1 - kind.size.width, y: low.y, goal: low.x1 - kind.size.width }), 300 * SECOND, (after, before) => {
        const actor = after.actors.find((candidate) => candidate.species === walker);
        if (actor !== undefined && actor.activity === "hop" && before.actors.find((candidate) => candidate.species === walker)?.activity !== "hop") count++;
        ticks++;
        if ([60, 120, 200, 300].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${count}`);
      });
      say(`  ${name} seed ${seed} (arrived on ${actorOf(arrived, walker).perch}): ${lines.join(" | ")}`);
    }
  }

  say("hopping gait: walks begun/ended by the lonely hopper per seed at 60/120/200/300 s (lively)");
  for (const seed of [1, 2, 3, 4, 5, 6, 7, 8]) {
    let walks = 0;
    let ended = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, staged(menagerie, [company.hopper], { seed, mode: "lively", surfaces: [FLOOR] }), 300 * SECOND, (after, before) => {
      const actor = actorOf(after, company.hopper);
      const earlier = actorOf(before, company.hopper);
      if (actor.activity === "walk" && earlier.activity !== "walk") walks++;
      if (earlier.activity === "walk" && actor.activity !== "walk") ended++;
      ticks++;
      if ([60, 120, 200, 300].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${walks}/${ended}`);
    });
    say(`  seed ${seed}: ${lines.join(" | ")}`);
  }

  say("concentration: ticks of walk or fidget after the hush is lifted (seed 2, hushed for 60/120/300 s), counted for 30/60/120 s");
  for (const hush of [60, 120, 300]) {
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 2, mode: "lively" }), 45 * SECOND);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "hushed", quiet: true }]), hush * SECOND);
    stage = advance(menagerie, stage, [{ kind: "hushed", quiet: false }]);
    let lively = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, stage, 120 * SECOND, (after) => {
      for (const actor of after.actors) if (actor.activity === "walk" || actor.activity === "fidget") lively++;
      ticks++;
      if ([30, 60, 120].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${lively}`);
    });
    say(`  hushed ${hush}s: ${lines.join(" | ")}`);
  }

  say("the tired asleep (seed 3): actor-ticks asleep quiet/not quiet at 40/60/120 s");
  const asleep = (quiet: boolean): string => {
    let stage = staged(menagerie, everyone, { seed: 3 });
    for (const species of everyone) stage = craft(stage, species, { needs: { energy: 0.3, sociability: 0.5, curiosity: 0.5 } });
    stage = advance(menagerie, stage, [{ kind: "hushed", quiet }]);
    let sleeping = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, stage, 120 * SECOND, (after) => {
      sleeping += after.actors.filter((actor) => actor.activity === "sleep").length;
      ticks++;
      if ([40, 60, 120].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${sleeping}`);
    });
    return lines.join(" | ");
  };
  say(`  quiet: ${asleep(true)}`);
  say(`  awake: ${asleep(false)}`);

  say("back to life from still: activity changes at 30/60/120 s");
  {
    const stage = advance(menagerie, staged(menagerie, everyone, { mode: "still" }), [{ kind: "ticked", ticks: 5000 }, { kind: "tuned", mode: "calm" }]);
    let changes = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, stage, 120 * SECOND, (after, before) => {
      for (const actor of after.actors) if (before.actors.find((earlier) => earlier.species === actor.species)?.activity !== actor.activity) changes++;
      ticks++;
      if ([30, 60, 120].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${changes}`);
    });
    say(`  ${lines.join(" | ")}`);
  }

  say("blinking of the seated walker: blinks begun and open share at 30/45/60/120/240 s");
  {
    const arrived = staged(menagerie, [walker]);
    const seated = craft(arrived, walker, { perch: CARD.id, x: 550, y: CARD.y, vx: 0, vy: 0, goal: 550, activity: "idle", since: arrived.tick, until: arrived.tick + FAR, blink: 200, partner: null, opacity: 1, leaving: false, gaze: { x: 0, y: 0, vx: 0, vy: 0 } });
    let begun = 0;
    let open = 0;
    let lid = 0;
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, { ...seated, tick: 0 }, 240 * SECOND, (after) => {
      const eyes = frameOf(menagerie, after).actors[0]!.eyes;
      if (eyes[0]!.lid > 0 && lid === 0) begun++;
      lid = eyes[0]!.lid;
      if (lid === 0) open++;
      ticks++;
      if ([30, 45, 60, 120, 240].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s begun ${begun} (${Math.floor(ticks / SECOND / 6.2)}…${ticks / SECOND / 2 + 20}) open ${(open / ticks).toFixed(3)}`);
    });
    say(`  ${lines.join(" | ")}`);
  }

  say("rhythms: blinks per actor on the floor at 30/45/60 s");
  {
    const rhythms: Record<string, number[]> = Object.fromEntries(everyone.map((id) => [id, []]));
    let ticks = 0;
    const lines: string[] = [];
    run(menagerie, staged(menagerie, everyone, { surfaces: [FLOOR] }), 60 * SECOND, (after, before) => {
      for (const actor of after.actors) if (actor.blink !== actorOf(before, actor.species).blink) rhythms[actor.species]!.push(actor.blink);
      ticks++;
      if ([30, 45, 60].includes(ticks / SECOND)) lines.push(`${ticks / SECOND}s ${everyone.map((id) => rhythms[id]!.length).join("/")} distinct ${new Set(everyone.map((id) => rhythms[id]!.join(" "))).size}`);
    });
    say(`  ${lines.join(" | ")}`);
  }
}
