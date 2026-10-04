/** 🧶️ Ticket tool of work package F1: what the gear spec `🐕️pet-walk` does on the real home overview, played on the stage — the `surveyed` payload `f1_survey.mjs` measured at 1440 × 900 and its cast, lively; after `--settle` seconds the hand takes whoever stands on the tab `--tab` and every grappler off their perches and lets them go on the footer (at `--spots`), and the stage then runs for up to `--seconds` seconds: it prints, per seed, when the first ladder and the first rope were seen (in seconds after the hand), and the poofs.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_rope_story.ts" [--seeds 8] [--settle 20] [--seconds 480] [--tab s20] [--spots 420,1000,1380,140]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie, Stage, Surveyed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
const dump = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "f1", "survey", "home.json"), "utf8")) as { sample?: { surveyed: Surveyed; pets: { id: string }[] }; samples?: { surveyed: Surveyed; pets: { id: string }[] }[] };
const sample = dump.sample ?? dump.samples![0]!;
const cast = sample.pets.map((pet) => pet.id);
const grapplers = cast.filter((id) => menagerie.species.find((kind) => kind.id === id)!.gear.includes("grapple"));
const tab = option("--tab", "s20");
const spots = option("--spots", "420,1000,1380,140").split(",").map(Number);
const settle = Number(option("--settle", "20")) * 64;
const seconds = Number(option("--seconds", "480"));
const watch = option("--watch", "");
const cycle = Number(option("--cycle", "0"));
const lines: string[] = [`cast ${cast.join(", ")}; grapplers ${grapplers.join(", ")}`];
for (let seed = 1; seed <= Number(option("--seeds", "8")); seed++) {
  let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, sample.surveyed, { kind: "summoned", species: cast }, { kind: "permitted", play: true, mischief: false }]);
  stage = advance(menagerie, stage, [{ kind: "ticked", ticks: settle }]);
  const before = stage.actors.map((actor) => `${actor.species}@${actor.perch ?? actor.footing}`).join(" ");
  const moved: string[] = [];
  let spot = 0;
  const floor = Math.max(...stage.perches.map((perch) => perch.y));
  const misplaced = (): boolean => stage.actors.some((actor) => (actor.perch === tab && actor.footing === "perch") || (grapplers.includes(actor.species) && actor.footing === "perch" && actor.y !== floor));
  const arrange = (): void => {
    for (const actor of stage.actors) {
      if (actor.footing !== "perch" || actor.activity !== "idle" || actor.opacity < 1) continue;
      if (!(actor.perch === tab || (grapplers.includes(actor.species) && actor.y !== floor))) continue;
      const x = spots[spot++ % spots.length]!;
      moved.push(actor.species);
      stage = advance(menagerie, stage, [{ kind: "pressed", x: actor.x, y: actor.y - 20, pointer: "mouse" }, { kind: "dragged", x: actor.x, y: actor.y - 40 }, { kind: "ticked", ticks: 2 }, { kind: "dragged", x, y: 780 }, { kind: "ticked", ticks: 12 }, { kind: "dragged", x, y: 780 }, { kind: "ticked", ticks: 12 }, { kind: "released", x, y: 780 }, { kind: "ticked", ticks: 1 }]);
    }
  };
  arrange();
  let pending = misplaced();
  const start = stage.tick;
  const poofs = stage.poofs;
  let ladder = -1;
  let rope = -1;
  let roper = "";
  const trail: string[] = [];
  let lastTab = "";
  let lastLine = "";
  for (let tick = 0; tick < seconds * 64 && (ladder < 0 || rope < 0); tick++) {
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    if (cycle > 0 && tick > 0 && tick % (cycle * 64) === 0 && rope < 0) pending = true;
    if (pending && tick % 8 === 0) {
      arrange();
      pending = misplaced();
    }
    const onTab = `[${tab}:${stage.actors.filter((actor) => actor.perch === tab).map((actor) => actor.species).join("+") || "nobody"}]`;
    if (watch !== "" && onTab !== lastTab) trail.push(`${((stage.tick - start) / 64).toFixed(1)} ${onTab}`);
    lastTab = onTab;
    for (const actor of stage.actors) {
      if (actor.species === watch) {
        const line = `${actor.activity}/${actor.footing}/${actor.perch ?? "-"}`;
        if (line !== lastLine) trail.push(`${((stage.tick - start) / 64).toFixed(1)} ${line} ${actor.x.toFixed(0)}`);
        lastLine = line;
      }
      if (ladder < 0 && actor.footing === "ladder") ladder = stage.tick - start;
      if (rope < 0 && actor.footing === "rope") {
        rope = stage.tick - start;
        roper = actor.species;
      }
    }
  }
  if (watch !== "") lines.push(`  ${watch}: ${trail.join(" | ")}`);
  lines.push(`seed ${seed}: before ${before}; moved ${moved.join(", ")}; ladder ${ladder < 0 ? "never" : `${(ladder / 64).toFixed(1)} s`}, rope ${rope < 0 ? "never" : `${(rope / 64).toFixed(1)} s by ${roper}`}, poofs ${stage.poofs - poofs}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
