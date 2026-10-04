/** 🧪️ Ticket tool of work package C1b: a quiz-like list whose rows are fixtures (`data-pet-prop`), a pet layer over it and the lifting of the product driven by hand along the core's own lift path (`liftAt` of `🪄️mischief`), with a pet of the architecture menagerie drawn where the pusher would stand.
 *
 * Query parameters: `theme` (`light`, `dark`), `motion` (`reduced`: the host shortens every transition to 0.01 ms, as the quiz does), `quiet`.
 * Everything a probe needs is on `window.c1b`: the fixture ids of the rows, `rest(row)` (the lift at rest, wholly
 * opaque, pushed out), `at(row, age)` (the lift `age` ticks after it began), `play(row)` (the whole lift at 64 ticks a
 * second), `clear()` (a frame without lifts), `gear()` / `ungear()` (the layer's scenery with a tilted pet on a rope
 * with gun and hook, a standing ladder and particles; it loads the layer, so it needs the whole core), `heard` (what
 * lifting told the stage), `transitions` (every transition
 * that ran on a row), `violations` (every report of the Content-Security-Policy) and `clicks` (clicks the rows received).
 */
import { LIFT_TICKS, liftAt, restPose, solveRig, type ActorFrame, type LiftFrame, type Species } from "@semio-tech/pets";
import "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css";
import { depict, paint } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts";
import { liftFixtures } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🪞️lifting/🟦️.ts";
import type { Scenery } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx";
import { fixtureId } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts";
import housy from "../../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json";

const query = new URLSearchParams(location.search);
if (query.get("theme") === "dark") document.documentElement.setAttribute("data-theme", "dark");
if (query.get("motion") === "reduced") document.querySelector(".app")!.classList.add("reduced");
const quiet = query.has("quiet");

const layer = document.querySelector<HTMLElement>(".pet-layer")!;
layer.style.setProperty("pointer-events", "none");
layer.style.setProperty("z-index", "35");
const rows = [...document.querySelectorAll<HTMLElement>("[data-pet-prop]")];
rows[1]!.style.setProperty("letter-spacing", "0.2px");
const ids = rows.map(fixtureId);
const heard: string[] = [];
const transitions: string[] = [];
const violations: string[] = [];
const clicks: number[] = rows.map(() => 0);
rows.forEach((row, index) => row.addEventListener("click", () => (clicks[index]! += 1)));
document.addEventListener("transitionrun", (event) => transitions.push(`${(event.target as Element).getAttribute("data-pet-prop") ?? (event.target as Element).localName}:${(event as TransitionEvent).propertyName}`), true);
document.addEventListener("securitypolicyviolation", (event) => violations.push(`${event.violatedDirective} ${event.sample}`));

const lifting = liftFixtures(layer, { quiet: () => quiet, reclaimed: (fixture) => void heard.push(fixture) });
const species = housy as unknown as Species;
const pet = depict(species, document);
let scenery: Scenery | null = null;
const staged = async (): Promise<Scenery> => {
  const { stageScenery } = await import("../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx");
  scenery ??= stageScenery(layer, new Map([[species.id, species]]), lifting.copies);
  return scenery;
};
const ROOM = species.size.width;
let playing = 0;

const actor = (change: Partial<ActorFrame>): ActorFrame => ({ species: species.id, x: 0, y: 0, facing: 1, activity: "push", opacity: 1, bones: solveRig(species, restPose(species)), eyes: species.face.eyes.map(() => ({ x: 0.6, y: 0, lid: 0 })), footing: "wall", state: species.states[0]!.id, mood: "playful", intensity: 0.6, spirits: 0.5, tilt: 0, pivot: { x: 0, y: -species.grip }, tools: [], body: { x: 0, y: 0, width: species.size.width, height: species.size.height }, ...change });

const gear = async (): Promise<void> => {
  const scenery = await staged();
  show(0, null);
  const card = document.querySelector(".card")!.getBoundingClientRect();
  const hook = { x: card.left + 40, y: card.top };
  const frame = actor({ x: card.left - 70, y: card.top + 120, activity: "reel", footing: "rope", state: species.states[3]?.id ?? species.states[0]!.id, tilt: 0.04, tools: [{ kind: "gun", aim: -0.2 }, { kind: "rope", x: hook.x, y: hook.y, slack: 4 }, { kind: "hook", x: hook.x, y: hook.y }] });
  const emitter = species.emitters.find((each) => each.motion === "rise") ?? species.emitters[0]!;
  const particles = Array.from({ length: Math.min(3, Math.floor(emitter.count)) }, (_, index) => ({ species: species.id, emitter: emitter.id, x: frame.x + 6 * index - 4, y: frame.y - species.size.height - 8 - 10 * index, scale: 1 - 0.15 * index, rotation: 0, opacity: 1 - 0.25 * index }));
  const ladders = [{ x0: card.left - 24, y0: card.bottom + 160, x1: card.left - 2, y1: card.top + 4, rungs: 14, opacity: 1 }];
  scenery.stage({ ladders, particles }, 1);
  paint(pet, frame, 1);
  scenery.actor(pet, frame);
  if (!pet.element.isConnected) layer.insertBefore(pet.element, scenery.behind()?.nextElementSibling ?? layer.firstElementChild);
};

const ungear = (): void => {
  scenery?.stage({ ladders: [], particles: [] }, 1);
  pet.element.remove();
};

const show = (index: number, lift: Omit<LiftFrame, "fixture"> | null): void => {
  lifting.realise(lift === null ? [] : [{ fixture: ids[index]!, ...lift }], { x: 0, y: 0 }, 1);
  if (lift === null) {
    pet.element.remove();
    return;
  }
  const box = rows[index]!.getBoundingClientRect();
  const frame = actor({ x: box.left + lift.dx - species.size.width / 2 - 2, y: box.bottom + lift.dy });
  paint(pet, frame, 1);
  scenery?.actor(pet, frame);
  if (!pet.element.isConnected) layer.append(pet.element);
};

const at = (index: number, age: number): LiftFrame => ({ fixture: ids[index]!, ...liftAt(0, age, 1, ROOM, rows[index]!.getBoundingClientRect().width, 1) });

Object.assign(window, {
  c1b: {
    ids,
    room: ROOM,
    heard,
    transitions,
    violations,
    clicks,
    copies: () => layer.querySelectorAll(":scope > :not(svg)").length,
    rest: (index: number) => show(index, at(index, 300)),
    at: (index: number, age: number) => show(index, at(index, age)),
    clear: () => show(0, null),
    gear,
    ungear,
    play: (index: number) =>
      new Promise<void>((done) => {
        cancelAnimationFrame(playing);
        const start = performance.now();
        const tick = (now: number): void => {
          const age = Math.floor(((now - start) / 1000) * 64);
          if (age >= LIFT_TICKS) {
            show(index, null);
            done();
            return;
          }
          show(index, at(index, age));
          playing = requestAnimationFrame(tick);
        };
        playing = requestAnimationFrame(tick);
      }),
  },
});
