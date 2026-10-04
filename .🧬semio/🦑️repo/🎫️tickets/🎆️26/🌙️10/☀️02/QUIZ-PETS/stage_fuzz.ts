/** 🌪️ Ticket tool of work packages B1 and B4: drives the REAL stage (`advance` of `🎪️stage`, not the reference world of `🚧️clearance`) through fuzzed sessions and holds every tick to the invariant of design-v2 §18 — the bodies of no two actors overlap — and to the box of the stage: no body of an actor that does not leave lies outside it.
 *
 * A session is one menagerie, one seed and `--ticks` ticks on a 1280 × 720 stage with a floor and four to seven cards.
 * With `--walls` every card is a box 40 to 240 px deep: its top is the surface, its box a keep-out and its two sides
 * walls, so pets climb, rest on walls, raise ladders and fire ropes between the cards. Everything that can happen to
 * a stage happens at random, from a counter-based generator (mulberry32 of the seed): surveys (a card shrinks, grows,
 * moves, jumps, vanishes or comes back), summons (another company), mode changes (calm, lively, still and back), times
 * of concentration, the pointer roaming, and the learner's hand — presses on a pet, drags of 20 to 150 ticks at 2 to
 * 18 px per tick towards another pet (over it, into it) or anywhere, then a throw (the release), a drop from rest or
 * a cancellation —, clicks, and tosses by deed. After every tick the bodies of the stage are tallied (`tallied` of
 * `🚧️clearance`: overlap ticks, near misses within 2 px), and so are the boxes the pets are drawn in as the browser
 * spec `🐕️pet-walk` measures them (the size box turned by the frame's tilt about the feet; two that reach more than
 * 0.5 px into each other both ways make a drawn-overlap tick), and the tool counts poofs, waits (actor-ticks in the air
 * without a course), parachutes that open, landings on heads, hard landings and those of pets that own a parachute,
 * whether the order of the grounded actors on every perch was kept, actor-ticks outside the stage box (the upright box
 * of the species at the feet of an actor that does not leave reaching beyond an edge; a tilt is the drawing's), bounces off its
 * edges, exits through its bottom edge, and the gear: trips set out on, actor-ticks on a wall, a ladder or a rope,
 * ladders raised and toppled, climbers thrown off a wall, and hooks that missed.
 *
 * With `--mischief` (work package B5; it implies `--walls`) the rows of every card are marked for the pets to play
 * with — each with a key a ground of the menagerie covers —, the learner permits mischief and leaves the page alone
 * for long stretches (15 to 50 s without a pointer, a press, a click or a toss; then 5 to 25 s of the busy learner
 * above), and takes a lifted row back now and then while its copy is out. The tool then also counts the pranks,
 * the ticks a copy is out, the reclaims (and those of a copy whose pusher no longer pushed: it is not thrown), the
 * reclaims that threw a pusher off, the pushers that came down sheepish, the pranks that
 * ended quietly (their row gone or moved, mischief withdrawn, the stage still), the copies that slid home without
 * their pusher, and every copy out on its way without a pusher that pushes (there must be none).
 *
 * With `--pan` (work package F1; it implies `--walls`) the stage is the home overview at 1440 × 900: three columns of
 * three cards, each with a title tab on its top, the footer line 26 px above the floor, the bottom row's walls ending
 * just above the footer so climbers take hold of them, the lively mode — and the panorama pans as the pointer sweeps
 * over it, in sweeps of 1 to 5 s with 2 to 15 s of rest between them (so pets get onto walls, ladders and ropes, and
 * the next sweep finds them there): during a sweep every 2 to 6 ticks every card, its tab, its walls and its
 * keep-outs move by a few pixels (now and then they jump), and the card under the pointer lifts by 2 px or comes down
 * again; the footer and the floor stay where they are.
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stage_fuzz.ts" [--seeds N] [--ticks T] [--menagerie sample|architecture|both] [--name NAME] [--walls] [--mischief] [--pan] [--out DIR]
 * Writes `🗑️generated/<DIR, b1 by default>/fuzz-<name>.json` and prints the metrics tables (Markdown) to standard
 * output; exits with code 1 when any tick overlapped or any body left the stage.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Actor, Fixture, Menagerie, PetMode, Species, Stage, StageEvent, Surface, Wall } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { TALLY, orderKept, orderOf, overlaps, perMillion, tallied, type Tally } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { cosTurns, sinTurns } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📐️trigonometry/🟦️.ts";
import { LIFT_BRACE, LIFT_SHOVE, LIFT_TICKS, LIFT_WOBBLE } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🪄️mischief/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OUT = join(import.meta.dir, "🗑️generated", option("--out", "b1"));
const MISCHIEF = process.argv.includes("--mischief");
const PAN = process.argv.includes("--pan");
const WALLS = MISCHIEF || PAN || process.argv.includes("--walls");
const WATCH = option("--watch", "");
const WIDTH = PAN ? 1440 : 1280;
const HEIGHT = PAN ? 900 : 720;
const NEAR = 2;
const DRAWN_SLACK = 0.5;
const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };
const FOOTER: Surface = { id: "footer", x0: 0, x1: WIDTH, y: HEIGHT - 26 };
const CAP = 22;

/** 🃏️ A card: its top a surface; with `--walls` also a box `depth` deep, a keep-out with a wall on either side; with `--mischief` the keys of its rows; with `--pan` a title tab `cap` wide standing on its top at its left end (0: none). */
type Card = Surface & { readonly depth: number; readonly keys: readonly string[]; readonly cap: number };

/** 🧭️ Where the panorama of `--pan` has carried every card (`dx`, `dy`) and the card that is lifted by 2 px as the pointer reveals it (`lifted`, or `null`). */
type Pan = { readonly dx: number; readonly dy: number; readonly lifted: string | null };

type Counts = { pans: number; drawnOverlaps: number; firstDrawn: string; poofKinds: Record<string, number>; ticks: number; actorTicks: number; overlaps: number; nears: number; poofs: number; waits: number; surveys: number; summons: number; tunes: number; presses: number; clicks: number; lifts: number; throws: number; drops: number; cancels: number; tosses: number; chutes: number; heads: number; landings: number; hard: number; hardWithChute: number; disorders: number; firstOverlap: string; hardCases: string[]; outside: number; firstOutside: string; bounces: number; exits: number; trips: number; wallTicks: number; ladderTicks: number; ropeTicks: number; raised: number; topples: number; letGo: number; misses: number; firstDisorder: string; pranks: number; copyTicks: number; reclaims: number; unpushed: number; thrownOff: number; sheepish: number; quietEnds: number; slidHome: number; strays: number; firstStray: string };

/** 🎲️ mulberry32: a deterministic stream of units in [0, 1) for a seed. */
function stream(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let mixed = Math.imul(state ^ (state >>> 15), 1 | state);
    mixed = (mixed + Math.imul(mixed ^ (mixed >>> 7), 61 | mixed)) ^ mixed;
    return ((mixed ^ (mixed >>> 14)) >>> 0) / 4294967296;
  };
}

/** 🗝️ The key of a marked row with `--mischief`: a ground of a species of the menagerie drawn at random, one level deeper (an item under it) one time in three. */
function keyOf(menagerie: Menagerie, next: () => number): string {
  const grounds = menagerie.species.flatMap((kind) => kind.grounds);
  const ground = grounds.length === 0 ? "none" : grounds[Math.floor(next() * grounds.length)]!;
  return next() < 1 / 3 ? `${ground}/item` : ground;
}

/** 🗺️ The cards of a session: four to seven boxes at random places on levels between 160 and 600; with `--walls` 40 to 240 px deep; with `--mischief` a key for each of their rows. */
function cardsOf(next: () => number, menagerie: Menagerie): Card[] {
  const cards: Card[] = [];
  const count = 4 + Math.floor(next() * 4);
  for (let index = 0; index < count; index++) {
    const x0 = Math.floor(next() * 1000);
    const x1 = Math.min(x0 + 120 + Math.floor(next() * 360), WIDTH);
    const y = 160 + Math.floor(next() * 11) * 40;
    const depth = WALLS ? 40 + Math.floor(next() * 201) : 0;
    const keys: string[] = [];
    if (MISCHIEF) for (let row = 0; 6 + 32 * row + 28 <= depth - 4; row++) keys.push(keyOf(menagerie, next));
    cards.push({ id: `card-${index}`, x0, x1, y, depth, keys, cap: 0 });
  }
  return cards;
}

/** 🏠️ The cards of a `--pan` session, laid out like the home overview at 1440 × 900 (measured by work package F1): three columns of three cards, each with a title tab on its top; the bottom row's walls end up to 30 px above the footer line, so climbers on the footer take hold of them; every card a little apart from where the overview puts it. */
function homeOf(next: () => number): Card[] {
  const cards: Card[] = [];
  const columns = [120, 616, 1183];
  const rows = [[60, 120], [290, 300], [632, 200]] as const;
  for (const [column, x] of columns.entries()) {
    for (const [row, [top, depth]] of rows.entries()) {
      const x0 = x + Math.floor((next() - 0.5) * 40);
      const width = 90 + Math.floor(next() * 130);
      const y = top + Math.floor(next() * 40);
      const deep = row === 2 ? HEIGHT - 26 - y - Math.floor(next() * 30) : depth + Math.floor((next() - 0.5) * 60);
      cards.push({ id: `card-${column}-${row}`, x0, x1: Math.min(x0 + width, WIDTH - 20), y, depth: deep, keys: [], cap: 50 + Math.floor(next() * 60) });
    }
  }
  return cards;
}

/** 🌄️ A survey of the home overview of `--pan` as the panorama has carried it: every card moved by the pan (the lifted one 2 px higher), its tab and its body each a surface with walls on both sides and a keep-out grown 4 px to either side (as the layer's survey grows the surfaces' own elements), the footer line and the floor where they always are. */
function panSurveyOf(cards: readonly Card[], pan: Pan): StageEvent {
  const placed = cards.map((card) => {
    const lift = card.id === pan.lifted ? -2 : 0;
    return { ...card, x0: card.x0 + pan.dx, x1: card.x1 + pan.dx, y: card.y + pan.dy + lift };
  });
  const surfaces: Surface[] = placed.flatMap((card) => [
    { id: `${card.id}-cap`, x0: card.x0, x1: Math.min(card.x0 + card.cap, card.x1), y: card.y - CAP },
    { id: card.id, x0: card.x0, x1: card.x1, y: card.y },
  ]);
  const walls: Wall[] = placed.flatMap((card) => [
    { id: `${card.id}-cap-left`, surface: `${card.id}-cap`, side: -1 as const, x: card.x0, y0: card.y - CAP, y1: card.y },
    { id: `${card.id}-cap-right`, surface: `${card.id}-cap`, side: 1 as const, x: Math.min(card.x0 + card.cap, card.x1), y0: card.y - CAP, y1: card.y },
    { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: card.x0, y0: card.y, y1: card.y + card.depth },
    { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: card.x1, y0: card.y, y1: card.y + card.depth },
  ]);
  const keepouts = placed.flatMap((card) => [
    { x: card.x0 - 4, y: card.y - CAP, width: Math.min(card.cap, card.x1 - card.x0) + 8, height: CAP },
    { x: card.x0 - 4, y: card.y, width: card.x1 - card.x0 + 8, height: card.depth },
  ]);
  return { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces: [...surfaces, FOOTER, FLOOR], keepouts: [...keepouts, { x: -4, y: HEIGHT - 26, width: WIDTH + 8, height: 26 }], walls, fixtures: [] };
}

/** 🎥️ The next pan of the overview: mostly a drift of a few pixels across and less up or down (the pointer glides over the panorama), now and then a jump (it crossed a gap and the strip moved a card's width), held within 240 px of home; the card under the pointer is lifted or let down one time in five. */
function panned(pan: Pan, cards: readonly Card[], next: () => number): Pan {
  const jump = next() < 0.04;
  const dx = Math.max(-240, Math.min(240, pan.dx + (jump ? (next() - 0.5) * 360 : (next() - 0.5) * 9)));
  const dy = Math.max(-60, Math.min(60, pan.dy + (jump ? (next() - 0.5) * 80 : (next() - 0.5) * 5)));
  const lifted = next() < 0.2 ? (next() < 0.5 || cards.length === 0 ? null : cards[Math.floor(next() * cards.length)]!.id) : pan.lifted;
  return { dx, dy, lifted };
}

/** 📡️ A survey of the cards that stand and the floor; with `--walls` the boxes of the cards are keep-outs and their sides walls; with `--mischief` their rows (inset 8 px from the sides, 28 px high, 32 px apart) are marked. */
function surveyOf(cards: readonly Card[]): StageEvent {
  const surfaces: Surface[] = cards.map((card) => ({ id: card.id, x0: card.x0, x1: card.x1, y: card.y }));
  const walls: Wall[] = WALLS
    ? cards.flatMap((card) => [
        { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: card.x0, y0: card.y, y1: card.y + card.depth },
        { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: card.x1, y0: card.y, y1: card.y + card.depth },
      ])
    : [];
  const fixtures: Fixture[] = cards.flatMap((card) => card.keys.map((key, row) => ({ id: `${card.id}-row-${row}`, key, x: card.x0 + 8, y: card.y + 6 + 32 * row, width: card.x1 - card.x0 - 16, height: 28 })));
  return { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces: [...surfaces, FLOOR], keepouts: WALLS ? cards.map((card) => ({ x: card.x0, y: card.y, width: card.x1 - card.x0, height: card.depth })) : [], walls, fixtures };
}

/** 🖼️ The box every visible pet is drawn in, as the browser spec `🐕️pet-walk` measures it: the size box of its species, its feet at the actor's feet, turned by the frame's tilt about the feet (where the frame's turn about the pivot carries the rig's feet), the axis-aligned box of its four corners. */
function drawnOf(menagerie: Menagerie, stage: Stage, kinds: ReadonlyMap<string, Species>): { readonly label: string; readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number }[] {
  const boxes: { label: string; x0: number; y0: number; x1: number; y1: number }[] = [];
  for (const shown of frameOf(menagerie, stage).actors) {
    const actor = stage.actors.find((entry) => entry.species === shown.species);
    if (actor === undefined || !(shown.opacity > 0)) continue;
    const size = kinds.get(actor.species)!.size;
    const sine = sinTurns(shown.tilt);
    const cosine = cosTurns(shown.tilt);
    const corners = [[-size.width / 2, -size.height], [size.width / 2, -size.height], [size.width / 2, 0], [-size.width / 2, 0]] as const;
    const xs = corners.map(([x, y]) => actor.x + x * cosine - y * sine);
    const ys = corners.map(([x, y]) => actor.y + x * sine + y * cosine);
    boxes.push({ label: `${actor.species} ${actor.footing}/${actor.activity} tilt ${shown.tilt.toFixed(4)}`, x0: Math.min(...xs), y0: Math.min(...ys), x1: Math.max(...xs), y1: Math.max(...ys) });
  }
  return boxes;
}

/** 🎭️ A company of up to eight species of the menagerie. */
function companyOf(menagerie: Menagerie, next: () => number): string[] {
  const all = menagerie.species.map((kind) => kind.id);
  const size = Math.min(all.length, 3 + Math.floor(next() * 6));
  const chosen: string[] = [];
  while (chosen.length < size) {
    const pick = all[Math.floor(next() * all.length)]!;
    if (!chosen.includes(pick)) chosen.push(pick);
  }
  return chosen;
}

/** 🎬️ One fuzzed session; the counts grow. */
function session(menagerie: Menagerie, seed: number, ticks: number, counts: Counts): void {
  const next = stream(seed * 2654435761);
  const kinds = new Map<string, Species>(menagerie.species.map((kind) => [kind.id, kind]));
  let cards = PAN ? homeOf(next) : cardsOf(next, menagerie);
  const spare: Card[] = [];
  let pan: Pan = { dx: 0, dy: 0, lifted: null };
  let panAt = 0;
  let sweepUntil = 0;
  let restFor = 0;
  const measured = (): StageEvent => (PAN ? panSurveyOf(cards, pan) : surveyOf(cards));
  let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: PAN ? "lively" : next() < 0.5 ? "calm" : "lively" }, measured(), { kind: "summoned", species: companyOf(menagerie, next) }, { kind: "permitted", play: true, mischief: MISCHIEF }]);
  let tally: Tally = TALLY;
  let hand: { left: number; pace: number; tx: number; ty: number; mode: "throw" | "drop" | "cancel" } | null = null;
  let stillUntil = -1;
  let orders = new Map<string, string[]>();
  let busyUntil = 0;
  let idleUntil = -1;
  for (let tick = 0; tick < ticks; tick++) {
    const events: StageEvent[] = [];
    if (PAN && tick >= panAt) {
      if (tick >= sweepUntil + restFor) {
        sweepUntil = tick + 64 + Math.floor(next() * 256);
        restFor = 128 + Math.floor(next() * 832);
      }
      if (tick < sweepUntil) {
        pan = panned(pan, cards, next);
        counts.pans++;
        events.push(measured());
      }
      panAt = tick + 2 + Math.floor(next() * 5);
    }
    if (MISCHIEF && idleUntil < 0 && tick >= busyUntil) idleUntil = tick + 960 + Math.floor(next() * 2240);
    if (MISCHIEF && idleUntil >= 0 && tick >= idleUntil) {
      idleUntil = -1;
      busyUntil = tick + 320 + Math.floor(next() * 1280);
    }
    const idle = idleUntil >= 0;
    const lift = stage.lift;
    if (MISCHIEF && lift !== null && stage.tick >= lift.since && stage.tick - lift.since < LIFT_TICKS && next() < 1 / 150) {
      counts.reclaims++;
      if (stage.actors.find((actor) => actor.species === lift.pusher)?.activity !== "push") counts.unpushed++;
      events.push({ kind: "reclaimed", fixture: lift.fixture });
    }
    const roll = next();
    if (roll < 1 / 300) {
      counts.surveys++;
      const kind = next();
      if ((kind < 0.15 && spare.length > 0) || cards.length === 0) {
        if (spare.length > 0) cards = [...cards, spare.pop()!];
      } else {
        const at = Math.floor(next() * cards.length);
        const old = cards[at]!;
        let fresh: Card | null;
        if (kind < 0.35) fresh = null;
        else if (kind < 0.55) {
          const width = (0.3 + 0.6 * next()) * (old.x1 - old.x0);
          const x0 = old.x0 + next() * (old.x1 - old.x0 - width);
          fresh = { ...old, x0, x1: x0 + width };
        } else if (kind < 0.7) fresh = { ...old, x0: Math.max(old.x0 - 80 * next(), 0), x1: Math.min(old.x1 + 80 * next(), WIDTH) };
        else if (kind < 0.9) fresh = { ...old, x0: old.x0 + (next() - 0.5) * 40, x1: old.x1 + (next() - 0.5) * 40, y: old.y + (next() - 0.5) * 24 };
        else {
          const dx = (next() - 0.5) * 240;
          fresh = { ...old, x0: old.x0 + dx, x1: old.x1 + dx, y: Math.min(Math.max(old.y + (next() - 0.5) * 240, 120), 640) };
        }
        if (fresh === null) spare.push(old);
        cards = cards.flatMap((card, index) => (index === at ? (fresh === null ? [] : [fresh]) : [card]));
      }
      events.push(measured());
    } else if (roll < 1 / 300 + 1 / 3000) {
      counts.summons++;
      events.push({ kind: "summoned", species: companyOf(menagerie, next) });
    } else if (roll < 1 / 300 + 1 / 3000 + 1 / 4000 && stillUntil < 0) {
      counts.tunes++;
      const mode: PetMode = next() < 0.2 ? "still" : next() < 0.5 ? "calm" : "lively";
      events.push({ kind: "tuned", mode });
      if (mode === "still") stillUntil = tick + 200 + Math.floor(next() * 600);
    } else if (roll < 1 / 300 + 1 / 3000 + 1 / 4000 + 1 / 5000) events.push({ kind: "hushed", quiet: !stage.quiet });
    if (stillUntil >= 0 && tick >= stillUntil) {
      stillUntil = -1;
      events.push({ kind: "tuned", mode: next() < 0.5 ? "calm" : "lively" });
    }
    if (!idle && next() < 1 / 600 && stage.actors.length > 0) {
      counts.tosses++;
      events.push({ kind: "played", species: stage.actors[Math.floor(next() * stage.actors.length)]!.species, deed: "toss" });
    }
    if (hand === null && !idle && stage.press.phase === "idle" && next() < 1 / 100 && stage.actors.length > 0) {
      const target = stage.actors[Math.floor(next() * stage.actors.length)]!;
      const point = { x: target.x, y: target.y - kinds.get(target.species)!.size.height / 2 };
      events.push({ kind: "pressed", x: point.x, y: point.y, pointer: next() < 0.2 ? "touch" : "mouse" });
      counts.presses++;
      if (next() < 0.25) {
        events.push({ kind: "released", x: point.x, y: point.y });
        counts.clicks++;
      } else {
        const choice = next();
        hand = { left: 20 + Math.floor(next() * 130), pace: 2 + next() * 16, tx: point.x, ty: point.y, mode: choice < 0.6 ? "throw" : choice < 0.9 ? "drop" : "cancel" };
      }
    } else if (hand !== null) {
      const pointer = stage.pointer ?? { x: hand.tx, y: hand.ty };
      hand.left--;
      if (hand.left <= 0) {
        if (stage.press.phase === "lifted") counts.lifts++;
        if (hand.mode === "cancel") {
          events.push({ kind: "cancelled" });
          counts.cancels++;
        } else {
          events.push({ kind: "released", x: pointer.x, y: pointer.y });
          if (hand.mode === "throw") counts.throws++;
          else counts.drops++;
        }
        hand = null;
      } else {
        if (Math.abs(hand.tx - pointer.x) + Math.abs(hand.ty - pointer.y) < hand.pace || next() < 0.02) {
          const other = stage.actors[Math.floor(next() * stage.actors.length)];
          if (other !== undefined && next() < 0.6) {
            hand.tx = other.x;
            hand.ty = other.y - 10;
          } else {
            hand.tx = 20 + next() * (WIDTH - 40);
            hand.ty = 20 + next() * (HEIGHT - 40);
          }
        }
        const pace = hand.mode === "drop" && hand.left < 8 ? 0 : hand.pace;
        const dx = hand.tx - pointer.x;
        const dy = hand.ty - pointer.y;
        const far = Math.max(Math.abs(dx), Math.abs(dy), 1e-9);
        const step = Math.min(pace, far);
        events.push({ kind: "dragged", x: pointer.x + (dx * step) / far, y: pointer.y + (dy * step) / far });
      }
    } else if (!idle && next() < 1 / 40) events.push({ kind: "pointed", over: "free", x: next() * WIDTH, y: next() * HEIGHT });
    const before = stage;
    stage = advance(menagerie, stage, [...events, { kind: "ticked", ticks: 1 }]);
    if (MISCHIEF) {
      const was = before.lift;
      const now = stage.lift;
      const pusherOf = (inside: Stage, species: string): Actor | undefined => inside.actors.find((actor) => actor.species === species);
      if (was === null && now !== null) counts.pranks++;
      if (now !== null && stage.tick >= now.since && stage.tick - now.since < LIFT_TICKS) counts.copyTicks++;
      if (was !== null && pusherOf(before, was.pusher)?.activity === "push" && pusherOf(stage, was.pusher)?.activity === "tumble") counts.thrownOff++;
      if (was !== null && now === null && before.tick - was.since > LIFT_TICKS && pusherOf(stage, was.pusher) !== undefined) counts.sheepish++;
      if (was !== null && now === null && stage.tick - was.since < LIFT_TICKS) counts.quietEnds++;
      if (was !== null && now !== null && now.since < was.since && pusherOf(stage, now.pusher)?.activity !== "tumble" && stage.tick - now.since <= LIFT_TICKS) counts.slidHome++;
      if (now !== null && stage.tick >= now.since && stage.tick - now.since < LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE && pusherOf(stage, now.pusher)?.activity !== "push") {
        counts.strays++;
        if (counts.firstStray === "") counts.firstStray = `${menagerie.id} seed ${seed} tick ${stage.tick}: ${JSON.stringify(now)} pusher ${pusherOf(stage, now.pusher)?.activity ?? "gone"}`;
      }
    }
    const frozen = events.some((event) => event.kind === "tuned" && event.mode === "still");
    const actorKinds = stage.actors.map((actor) => kinds.get(actor.species)!);
    const bodies = bodiesOf(stage.actors, actorKinds);
    tally = tallied(tally, bodies, NEAR, stage.poofs - before.poofs, stage.actors.filter((actor) => (actor.footing === "air" || actor.footing === "chute") && !stage.courses.some((course) => course.owner === actor.species)).length);
    if (overlaps(bodies).length > 0 && counts.firstOverlap === "") counts.firstOverlap = `${menagerie.id} seed ${seed} tick ${stage.tick}: ${JSON.stringify(overlaps(bodies))}`;
    const drawn = drawnOf(menagerie, stage, kinds);
    let clashed = false;
    for (let one = 0; one < drawn.length; one++) {
      for (let other = one + 1; other < drawn.length; other++) {
        const [a, b] = [drawn[one]!, drawn[other]!];
        if (Math.min(a.x1, b.x1) - Math.max(a.x0, b.x0) > DRAWN_SLACK && Math.min(a.y1, b.y1) - Math.max(a.y0, b.y0) > DRAWN_SLACK) {
          clashed = true;
          if (counts.firstDrawn === "") counts.firstDrawn = `${menagerie.id} seed ${seed} tick ${stage.tick}: ${a.label} [${a.x0.toFixed(1)} ${a.y0.toFixed(1)} ${a.x1.toFixed(1)} ${a.y1.toFixed(1)}] × ${b.label} [${b.x0.toFixed(1)} ${b.y0.toFixed(1)} ${b.x1.toFixed(1)} ${b.y1.toFixed(1)}] after ${JSON.stringify(events.map((event) => event.kind))}`;
        }
      }
    }
    if (clashed) counts.drawnOverlaps++;
    if (stage.poofs > before.poofs) {
      const surveyed = events.some((event) => event.kind === "surveyed");
      for (const puff of stage.puffs.filter((entry) => !before.puffs.includes(entry)).slice(0, stage.poofs - before.poofs)) {
        let nearest: Actor | null = null;
        let least = Infinity;
        for (const earlier of before.actors) {
          const size = kinds.get(earlier.species)!.size;
          const far = Math.abs(puff.x - earlier.x) + Math.abs(puff.y - (earlier.y - size.height / 2)) + (size.width === puff.width && size.height === puff.height ? 0 : 1e6);
          if (far < least) {
            least = far;
            nearest = earlier;
          }
        }
        const key = nearest === null ? "unknown" : `${nearest.footing}/${nearest.activity}${surveyed ? " (survey)" : ""}`;
        counts.poofKinds[key] = (counts.poofKinds[key] ?? 0) + 1;
      }
    }
    for (const trip of stage.trips) if (!before.trips.some((entry) => entry.owner === trip.owner && entry.from === trip.from)) counts.trips++;
    for (const ladder of stage.ladders) if (!before.ladders.some((entry) => entry.owner === ladder.owner && entry.since === ladder.since)) counts.raised++;
    for (const earlier of before.actors) {
      if (stage.actors.some((actor) => actor.species === earlier.species) || (earlier.footing !== "air" && earlier.footing !== "chute")) continue;
      if (before.courses.some((course) => course.owner === earlier.species && course.ending === "away")) counts.exits++;
    }
    for (const actor of stage.actors) {
      const size = kinds.get(actor.species)!.size;
      if (WATCH !== "" && WATCH.split(",").includes(actor.species) && stage.tick >= Number(option("--from", "0")) && stage.tick <= Number(option("--to", "0"))) process.stdout.write(`watch seed ${seed} tick ${stage.tick} ${actor.footing}/${actor.activity} at ${actor.x.toFixed(2)},${actor.y.toFixed(2)} v ${actor.vx.toFixed(1)},${actor.vy.toFixed(1)} trip ${JSON.stringify(stage.trips.find((trip) => trip.owner === actor.species)?.ending ?? null)} course ${JSON.stringify(stage.courses.find((course) => course.owner === actor.species)?.ending ?? null)} events ${JSON.stringify(events.map((event) => event.kind))}\n`);
      if (actor.footing === "wall") counts.wallTicks++;
      if (actor.footing === "ladder") counts.ladderTicks++;
      if (actor.footing === "rope") counts.ropeTicks++;
      if (!actor.leaving && (actor.x - size.width / 2 < -1e-9 || actor.x + size.width / 2 > stage.width + 1e-9 || actor.y - size.height < -1e-9 || actor.y > stage.height + 1e-9)) {
        counts.outside++;
        if (counts.firstOutside === "") counts.firstOutside = `${menagerie.id} seed ${seed} tick ${stage.tick} ${actor.species} ${actor.footing}/${actor.activity} at ${actor.x},${actor.y}`;
      }
      const earlier = before.actors.find((candidate: Actor) => candidate.species === actor.species);
      if (earlier === undefined) continue;
      const flying = (earlier.footing === "air" || earlier.footing === "chute") && (actor.footing === "air" || actor.footing === "chute");
      if (flying && ((actor.vx * earlier.vx < 0 && (actor.x === size.width / 2 || actor.x === stage.width - size.width / 2)) || (earlier.vy < 0 && actor.vy > 0 && actor.y === size.height))) counts.bounces++;
      if (earlier.footing === "ladder" && actor.footing !== "ladder" && before.ladders.some((ladder) => ladder.rider === actor.species) && !stage.ladders.some((ladder) => ladder.owner === before.ladders.find((entry) => entry.rider === actor.species)!.owner)) counts.topples++;
      if (earlier.footing === "wall" && (actor.footing === "air" || actor.footing === "chute") && !before.trips.some((trip) => trip.owner === actor.species && trip.ending === "air")) counts.letGo++;
      if (earlier.activity !== "shrug" && actor.activity === "shrug" && before.trips.some((trip) => trip.owner === actor.species)) counts.misses++;
      if (earlier.footing !== "chute" && actor.footing === "chute") counts.chutes++;
      if (earlier.footing !== "head" && actor.footing === "head") counts.heads++;
      if (!frozen && (earlier.footing === "air" || earlier.footing === "chute") && actor.footing === "perch" && earlier.activity !== "hop" && actor.opacity > 0) {
        counts.landings++;
        const touch = (actor.y - earlier.y) * 64;
        if (touch > 600) {
          counts.hard++;
          if (kinds.get(actor.species)!.gear.includes("parachute")) {
            counts.hardWithChute++;
            const course = before.courses.find((entry) => entry.owner === actor.species);
            const start = course === undefined ? null : course.steps[0]!;
            if (counts.hardCases.length < 12) counts.hardCases.push(`${menagerie.id} seed ${seed} tick ${stage.tick} ${actor.species} ${earlier.activity}: touch ${touch.toFixed(1)}, course from ${course?.from} start vy ${start?.vy.toFixed(1)} at y ${start?.y.toFixed(1)}, landing y ${actor.y}, steps ${course?.steps.length}, canopy ${course?.steps.some((step) => step.canopy !== null)}`);
          }
        }
      }
    }
    const fresh = new Map<string, string[]>();
    for (const perch of stage.perches) {
      const grounded = stage.actors.map((actor, index) => ({ actor, body: bodies[index]! })).filter(({ actor }) => actor.footing === "perch" && actor.perch === perch.surface && before.actors.some((earlier) => earlier.species === actor.species && earlier.footing === "perch" && earlier.perch === perch.surface));
      const order = orderOf(grounded.map(({ body }) => body));
      const previous = orders.get(perch.surface);
      if (previous !== undefined && !orderKept(previous, order)) {
        counts.disorders++;
        if (counts.firstDisorder === "") counts.firstDisorder = `${menagerie.id} seed ${seed} tick ${stage.tick} on ${perch.surface}: ${JSON.stringify(previous)} → ${JSON.stringify(order)} after ${JSON.stringify(events.map((event) => event.kind))}`;
      }
      fresh.set(perch.surface, orderOf(stage.actors.map((actor, index) => ({ actor, body: bodies[index]! })).filter(({ actor }) => actor.footing === "perch" && actor.perch === perch.surface).map(({ body }) => body)));
    }
    orders = fresh;
  }
  counts.ticks += tally.ticks;
  counts.actorTicks += tally.actors;
  counts.overlaps += tally.overlaps;
  counts.nears += tally.nears;
  counts.poofs += tally.poofs;
  counts.waits += tally.waits;
}

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const seeds = Number(option("--seeds", "6"));
const ticks = Number(option("--ticks", "6000"));
const which = option("--menagerie", "both");
const name = option("--name", "run");
const menageries: Menagerie[] = [];
if (which === "sample" || which === "both") menageries.push((JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie);
if (which === "architecture" || which === "both") {
  const module = (await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie };
  menageries.push(module.ARCHITECTURE_MENAGERIE);
}
const rows: { menagerie: string; counts: Counts; seconds: number }[] = [];
for (const menagerie of menageries) {
  const counts: Counts = { pans: 0, drawnOverlaps: 0, firstDrawn: "", poofKinds: {}, ticks: 0, actorTicks: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0, surveys: 0, summons: 0, tunes: 0, presses: 0, clicks: 0, lifts: 0, throws: 0, drops: 0, cancels: 0, tosses: 0, chutes: 0, heads: 0, landings: 0, hard: 0, hardWithChute: 0, disorders: 0, firstOverlap: "", hardCases: [], outside: 0, firstOutside: "", bounces: 0, exits: 0, trips: 0, wallTicks: 0, ladderTicks: 0, ropeTicks: 0, raised: 0, topples: 0, letGo: 0, misses: 0, firstDisorder: "", pranks: 0, copyTicks: 0, reclaims: 0, unpushed: 0, thrownOff: 0, sheepish: 0, quietEnds: 0, slidHome: 0, strays: 0, firstStray: "" };
  const started = performance.now();
  for (let seed = 1; seed <= seeds; seed++) session(menagerie, seed, ticks, counts);
  rows.push({ menagerie: menagerie.id, counts, seconds: (performance.now() - started) / 1000 });
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, `fuzz-${name}.json`), `${JSON.stringify({ seeds, ticks, rows }, null, 2)}\n`);
const lines = [
  `| menagerie | seeds × ticks | actor-ticks | overlap ticks | drawn-overlap ticks | near-miss ticks (2 px) | poofs | poofs per million actor-ticks | waits (actor-ticks in the air without a course) | order breaks | presses / clicks / lifts / throws / drops / cancels / tosses | surveys / summons / tunes | parachutes / heads / landings / hard / hard with a parachute | seconds |`,
  `|---|---|---|---|---|---|---|---|---|---|---|---|---|---|`,
  ...rows.map(({ menagerie, counts, seconds }) => `| ${menagerie} | ${seeds} × ${ticks} | ${counts.actorTicks} | **${counts.overlaps}** | **${counts.drawnOverlaps}** | ${counts.nears} | ${counts.poofs} | ${perMillion(counts.poofs, counts.actorTicks).toFixed(1)} | ${counts.waits} | ${counts.disorders} | ${counts.presses} / ${counts.clicks} / ${counts.lifts} / ${counts.throws} / ${counts.drops} / ${counts.cancels} / ${counts.tosses} | ${counts.surveys} / ${counts.summons} / ${counts.tunes} | ${counts.chutes} / ${counts.heads} / ${counts.landings} / ${counts.hard} / ${counts.hardWithChute} | ${seconds.toFixed(1)} |`),
  ``,
  `| menagerie | walls | panning surveys | actor-ticks outside the stage box | bounces off its edges | exits through its bottom edge | trips set out on | actor-ticks on a wall / a ladder / a rope | ladders raised / toppled | let go of a wall (thrown off or out of grip) | hooks that missed |`,
  `|---|---|---|---|---|---|---|---|---|---|---|`,
  ...rows.map(({ menagerie, counts }) => `| ${menagerie} | ${WALLS ? "yes" : "no"} | ${counts.pans} | **${counts.outside}** | ${counts.bounces} | ${counts.exits} | ${counts.trips} | ${counts.wallTicks} / ${counts.ladderTicks} / ${counts.ropeTicks} | ${counts.raised} / ${counts.topples} | ${counts.letGo} | ${counts.misses} |`),
  ...(MISCHIEF
    ? [
        ``,
        `| menagerie | pranks | ticks a copy is out | reclaims sent / of a copy whose pusher no longer pushed / pushers thrown off | came down sheepish | ended early (errand off, row gone or moved, mischief withdrawn, still, copy gone on its way out) | slid home without its pusher | copies on their way out without a pushing pusher |`,
        `|---|---|---|---|---|---|---|---|`,
        ...rows.map(({ menagerie, counts }) => `| ${menagerie} | ${counts.pranks} | ${counts.copyTicks} | ${counts.reclaims} / ${counts.unpushed} / ${counts.thrownOff} | ${counts.sheepish} | ${counts.quietEnds} | ${counts.slidHome} | **${counts.strays}** |`),
      ]
    : []),
];
process.stdout.write(`${lines.join("\n")}\n`);
for (const { counts } of rows) {
  if (counts.firstOverlap !== "") process.stdout.write(`first overlap: ${counts.firstOverlap}\n`);
  if (counts.firstDrawn !== "") process.stdout.write(`first drawn overlap: ${counts.firstDrawn}\n`);
  process.stdout.write(`poofs by what the pet did the tick before: ${JSON.stringify(Object.entries(counts.poofKinds).sort((one, other) => other[1] - one[1]))}\n`);
  if (counts.firstOutside !== "") process.stdout.write(`first body outside the stage: ${counts.firstOutside}\n`);
  if (counts.firstDisorder !== "") process.stdout.write(`first order break: ${counts.firstDisorder}\n`);
  for (const line of counts.hardCases) process.stdout.write(`hard landing of a pet with a parachute: ${line}\n`);
  if (counts.firstStray !== "") process.stdout.write(`first copy on its way out without a pushing pusher: ${counts.firstStray}\n`);
}
if (rows.some(({ counts }) => counts.overlaps > 0 || counts.drawnOverlaps > 0 || counts.outside > 0 || counts.strays > 0)) process.exit(1);
