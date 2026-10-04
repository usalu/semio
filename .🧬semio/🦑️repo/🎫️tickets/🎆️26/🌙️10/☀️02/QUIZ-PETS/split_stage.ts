/** ✂️ Ticket tool of work package A1 (second round): splits the stage of the pets core — `🎪️stage/🟦️.ts` and its Rust twin — into the modules of design-v2 §15.2 without touching a single body, and proves afterwards that nothing but the place of every definition changed.
 *
 * `split --from <pets> --to <pets>` reads the two stage files below `<from>/🔨️modules/🎪️stage`, cuts them into their
 * top-level definitions (docstring, attributes and body, verbatim), deals these to the parts named in `PARTS` in the
 * order they had, works out from the code alone what each part must import (from the other parts and from the
 * modules the stage imported) and what it must therefore share (`export` in TypeScript, `pub(crate)` in Rust), and
 * writes one `🟦️.ts` and one `🦀️.rs` per part below `<to>/🔨️modules`. Run it on a scratch copy; the repository files
 * are then written by hand and compared with `cmp`.
 *
 * `check --old <pets> --new <pets>` parses the old stage files and every part of the new tree and compares the
 * definitions one by one, with the sharing keywords taken off: every definition must exist exactly once and carry
 * the same text, and no part may import a name it does not use or use a name it neither defines nor imports.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/split_stage.ts split --from <before>/🐾️pets --to <after>/🐾️pets
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/split_stage.ts check --old <before>/🐾️pets --new 🧰️framework/🛍️products/🐾️pets
 *
 * @see ./📓️explore2-core-as-built.md — §5, the functions per module and the dependency direction
 * @see ./📓️design-v2.md — §15.2, the modules of the core
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const V = "️";

/** 🏷️ A path segment the way the taxonomy writes it: one emoji, the variation selector, the slug. */
function named(emoji: string, slug: string): string {
  return `${emoji.replaceAll(V, "")}${V}${slug}`;
}

/** 🐍️ The Rust name of a TypeScript name: functions turn to snake_case, types and constants stay. */
function snake(name: string): string {
  return /^[a-z]/.test(name) ? name.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`) : name;
}

type Lang = "ts" | "rs";

/** 📑️ A run of definitions under one region marker. */
type Region = { readonly name: string; readonly items: readonly string[] };

/** 🧩️ One module of the split stage: its directory, the emoji its header docstring starts with, what the header says, what it points to, its regions and whether its Rust twin mounts a unit test file. */
type Part = { readonly slug: string; readonly emoji: string; readonly lead: string; readonly about: readonly string[]; readonly regions: readonly Region[]; readonly tests: "none" | "own" | "shared" };

/** 📄️ One top-level definition of a source file: its name, where its docstring and attributes begin, where its declaration stands and where it ends. */
type Item = { readonly name: string; readonly kind: "const" | "type" | "struct" | "fn"; readonly start: number; readonly decl: number; readonly end: number };

/** 📚️ A parsed source file. */
type Source = { readonly lines: readonly string[]; readonly items: readonly Item[] };

const MODULES = named("🔨", "modules");
const FILES: Record<Lang, string> = { ts: named("🟦", ".ts"), rs: named("🦀", ".rs") };
const ONLY: Record<string, Lang> = { Body: "ts", NO_CLIPS: "ts", NO_RAPPORTS: "ts", RATE: "rs", CENTRED: "rs", haul: "rs", behind: "rs" };
const SHARED_FOR_TESTS = ["BLINK_LOW", "BLINK_HIGH", "CENTRED", "TURN_TICKS", "SHY_STEP"];
const FACADE = "stage";

const PARTS: readonly Part[] = [
  {
    slug: "draft",
    emoji: "📝",
    lead: "✏",
    about: [
      "The draft of the stage: the working copy that `advance` folds events into — the actors in `menagerie.species` order, each beside its species and its random stream — with what every part of the stage needs of it: the lookups of surfaces, clips and actors, the keys of the counter-based draws, the stretches of a perch, and the smallest changes of an actor (taking up an activity, coming to rest, letting go of its partner, leaving the draft).",
      "What the fold must know beside the stage itself belongs to `Draft`; what a new part of the stage needs of every other part belongs here, below all of them.",
    ],
    regions: [
      { name: "Constants", items: ["RATE", "BLINK_LOW", "BLINK_HIGH", "ORIGIN", "NO_CLIPS"] },
      { name: "Draft", items: ["Body", "Draft", "Room", "Launch", "draftOf", "sealed"] },
      { name: "Lookups", items: ["hoverOf", "indexOf", "surfaceOf", "clipsOf", "clipAt", "clipOf", "breathOf", "facingTo", "blinkAt", "actorKey", "stageKey", "shoulders", "beatOf"] },
      { name: "Activities", items: ["shift", "settle", "release", "remove"] },
      { name: "Stretches", items: ["carve", "crowdOn"] },
    ],
    tests: "own",
  },
  {
    slug: "spacing",
    emoji: "📏",
    lead: "📏",
    about: [
      "The spacing of grounded actors: actors of one surface never stand in each other. Where an actor can stand a comfortable gap away from everybody else (`vacancy`, `roomsFor`, `quarters`), how far it can walk (`clearway`), whether somebody is in the way of its next strides (`hindered`) and whether the arc of a hop keeps clear of what must stay free (`soars`) — with the gaps the whole stage keeps: `COMFORT_GAP`, `MEET_GAP` and `CONTACT`.",
      "Everything here measures along one surface: feet on the same perch, distances in x.",
    ],
    regions: [
      { name: "Constants", items: ["MEET_GAP", "COMFORT_GAP", "CONTACT"] },
      { name: "Spacing", items: ["vacancy", "soars", "clearway", "hindered", "roomsFor", "quarters"] },
    ],
    tests: "none",
  },
  {
    slug: "schedule",
    emoji: "🗓",
    lead: "🗓",
    about: [
      "The schedule of the stage: the changes that are due at a known tick and depend on more than one actor — the next whole second on which a pair may be drawn (`pairingTick`) and the next on which somebody who waits off stage may arrive (`arrivalTick`).",
      "The clock jumps to these ticks (`lull`) and the frame reports them (`wake`). A new time horizon of the stage is a function here and a line in both of them.",
    ],
    regions: [
      { name: "Constants", items: ["WARMUP"] },
      { name: "Schedule", items: ["sociable", "pairingTick", "arrivalTick"] },
    ],
    tests: "own",
  },
  {
    slug: "attention",
    emoji: "👀",
    lead: "👁",
    about: [
      "Attention: what a pet does with its eyes, its head and the way it faces — how visible it wants to be under the pointer, the way it ought to face and the turn towards it, the gaze spring, the blink schedule, the mood, perking up at a pointer that has come to rest, and the lean and the squeeze a frame draws.",
      "Whatever the pointer does to one pet belongs here; its horizons (`POINTER_TICKS`, `TURN_REST`, `PERK_LINGER`) are read by the clock and by the frame.",
    ],
    regions: [
      { name: "Constants", items: ["POINTER_TICKS", "GAZE_REACH", "GAZE_REST", "GAZE_CALM", "GAZE_AHEAD", "GAZE_SULK", "GAZE_FALL", "EYE_HEIGHT", "BLINK_AGAIN", "MOOD_EASE", "MOOD_REST", "WAKE_REACH", "SHY_OPACITY", "SHY_REACH", "TURN_TICKS", "TURN_REST", "TURN_CLEAR", "LEAN_TURN", "LEAN_REACH", "LEAN_NOD", "PERK_LINGER", "PERK_URGE", "PERK_COST"] },
      { name: "Presence", items: ["watched", "presenceOf"] },
      { name: "Turning", items: ["headingOf", "turn"] },
      { name: "Gaze", items: ["gazeGoal", "gazeRests", "look"] },
      { name: "Manners", items: ["wink", "cheer", "swivel", "hail", "perk"] },
      { name: "Drawing", items: ["leant", "squeezeOf"] },
    ],
    tests: "own",
  },
  {
    slug: "locomotion",
    emoji: "🚶",
    lead: "🏃",
    about: [
      "Locomotion: how a pet gets around and how it loses its ground — setting out for a goal on its perch, striding, hopping and gliding between perches, falling, touching down, being crowded out, waiting for a partner and leaving.",
      "A new footing or a new way to travel belongs here: what starts it, its tick of motion and where it ends.",
    ],
    regions: [
      { name: "Constants", items: ["PATIENCE", "LEAVE_REACH", "GLIDE_TICKS"] },
      { name: "Footing", items: ["vanish", "crowdOut", "drop", "touch"] },
      { name: "Departure", items: ["paced", "stroll", "attend", "hopsOf"] },
      { name: "Motion", items: ["stride", "aim", "plunge", "fly"] },
      { name: "Leaving", items: ["leave"] },
    ],
    tests: "own",
  },
  {
    slug: "sociability",
    emoji: "💞",
    lead: "💞",
    about: [
      "Sociability: what pets do with each other and with whoever taps them — the pairing on whole seconds, coming together, the encounter, the sulk after a squabble and its mending, the rapport that moves and fades, and the poke.",
      "Whatever happens between two actors, or between an actor and the hand that reaches for it, belongs here.",
    ],
    regions: [
      { name: "Constants", items: ["POKE_REACH", "POKE_CHEER", "ENCOUNTER_REACH", "ENCOUNTER_RISE", "PAIR_WEIGHT", "NO_RAPPORTS"] },
      { name: "Rapport", items: ["sulk", "recall", "bond", "reconcile"] },
      { name: "Encounters", items: ["pair", "parted", "reachable", "approach", "meet"] },
      { name: "Poke", items: ["poke"] },
    ],
    tests: "own",
  },
  {
    slug: "choice",
    emoji: "🎯",
    lead: "🎯",
    about: [
      "Choice: what an actor does next when its time is up — an idle one picks an activity by its weights, with its dwell, its clip and its goal or target (`decide`); everything else ends the way its activity ends (`conclude`).",
      "A new activity an actor may choose by itself gets its branch in `decide`, and its end in `conclude`.",
    ],
    regions: [
      { name: "Constants", items: ["STROLL_LEAST"] },
      { name: "Decision", items: ["decide", "conclude"] },
    ],
    tests: "none",
  },
  {
    slug: "population",
    emoji: "👥",
    lead: "👥",
    about: [
      "Population: who is on stage and where the ground is — the survey (perches cut anew, grounded actors riding their surfaces and seated apart again), arrivals and spreading out over new ground, the summons, and the liveliness (freezing and thawing).",
      "What a survey brings and what it does to the company belongs here.",
    ],
    regions: [
      { name: "Constants", items: ["CENTRED"] },
      { name: "Riding", items: ["measure", "haul", "carry", "seat", "widened", "ride"] },
      { name: "Arrival", items: ["arrive", "spawn", "spread"] },
      { name: "Events", items: ["survey", "summon", "freeze", "tune"] },
    ],
    tests: "own",
  },
  {
    slug: "clock",
    emoji: "🕰",
    lead: "🕰",
    about: [
      "The clock of the stage: one tick of one actor and one tick of the stage in the normative order (`act`, `step`), the lull in which nothing changes but the tick (`lull`), and time passing over both (`pass`).",
      "Whatever moves, fades, turns or is scheduled must be known in three places, or it is drawn at the wrong rate or jumped over: `lull` here, and `paceOf` and the `wake` of `frameOf` in the projection.",
    ],
    regions: [
      { name: "Constants", items: ["FADE_STEP", "SHY_STEP"] },
      { name: "Time", items: ["act", "lull", "step", "pass"] },
    ],
    tests: "own",
  },
  {
    slug: "projection",
    emoji: "🎥",
    lead: "📽",
    about: [
      "The projection of a stage into a frame: the pose of every actor (the idle loop underneath, the clip of its activity on top of it or in its place), the rate its motion needs (`paceOf`), and `frameOf` — the actors back to front with their matrices, eyes and mood, the rate of the frame and the tick it must wake at.",
      "Whatever a render target is to draw is projected here, from the stage alone.",
    ],
    regions: [
      { name: "Constants", items: ["BLEND_TICKS", "BREATH_STAGGER"] },
      { name: "Frame", items: ["replaces", "weightOf", "layer", "poseOf", "paceOf", "behind", "frameOf"] },
    ],
    tests: "own",
  },
  {
    slug: FACADE,
    emoji: "🎪",
    lead: "🎪",
    about: [],
    regions: [{ name: "Stage", items: ["openStage", "apply", "advance"] }],
    tests: "shared",
  },
];

const BRIEFS: Record<string, string> = {
  draft: "the working copy of a stage, its lookups and the smallest changes of an actor",
  spacing: "the gaps between grounded actors",
  schedule: "the whole seconds on which a pair may be drawn or somebody may arrive",
  attention: "eyes, head and facing: presence, turning round, gaze, blinks, mood, perking up",
  locomotion: "walking, hopping, gliding, falling, landing, leaving",
  sociability: "pairing, encounters, sulks, rapport, pokes",
  choice: "what an actor does next when its time is up",
  population: "surveys, arrivals, spreading out, summons, liveliness",
  clock: "one tick in the normative order, the lulls, time passing",
  projection: "poses, the rate and `frameOf`",
};

/** 📂️ The directory of a part. */
function directoryOf(part: Part): string {
  return named(part.emoji, part.slug);
}

/** 🔤️ The name an item of the table carries in a language. */
function nameIn(lang: Lang, name: string): string {
  return lang === "rs" ? snake(name) : name;
}

/** 🪓️ Cuts a source file into its top-level definitions; everything else must be the header, an import, a region marker, a blank line or the mount of the unit tests. */
function parse(lang: Lang, text: string, label: string): Source {
  const lines = text.split("\n");
  if (lines[lines.length - 1] === "") lines.pop();
  const items: Item[] = [];
  let index = 0;
  if (lang === "ts") {
    if (!lines[0]!.startsWith("/**")) throw new Error(`${label}: no header docstring`);
    while (!lines[index]!.endsWith("*/")) index++;
    index++;
  } else {
    if (!lines[0]!.startsWith("//!")) throw new Error(`${label}: no header docstring`);
    while (lines[index]!.startsWith("//!")) index++;
  }
  const outside = lang === "ts" ? [/^import /, /^export \{ .* \} from /] : [/^use /, /^pub use /];
  while (index < lines.length) {
    const line = lines[index]!;
    if (line === "" || line.startsWith("//#region ") || line.startsWith("//#endregion ") || outside.some((pattern) => pattern.test(line))) {
      index++;
      continue;
    }
    if (lang === "rs" && line === "#[cfg(test)]") {
      if (!lines[index + 1]!.startsWith("#[path = ") || !/^(pub\(crate\) )?mod tests;$/.test(lines[index + 2]!)) throw new Error(`${label}:${index + 1}: an unknown test mount`);
      index += 3;
      continue;
    }
    const start = index;
    let decl = index;
    if (lang === "ts" && line.startsWith("/**")) {
      while (!lines[decl]!.endsWith("*/")) decl++;
      decl++;
    }
    if (lang === "rs") while (lines[decl]!.startsWith("///") || lines[decl]!.startsWith("#[")) decl++;
    const head = lines[decl]!;
    const constant = lang === "ts" ? /^(?:export )?const (\w+)\b/.exec(head) : /^(?:pub(?:\(crate\))? )?const (\w+): /.exec(head);
    const alias = lang === "ts" ? /^(?:export )?type (\w+) = /.exec(head) : null;
    const record = lang === "rs" ? /^(?:pub(?:\(crate\))? )?struct (\w+)/.exec(head) : null;
    const routine = lang === "ts" ? /^(?:export )?function (\w+)[(<]/.exec(head) : /^(?:pub(?:\(crate\))? )?fn (\w+)[(<]/.exec(head);
    if (constant !== null || alias !== null) {
      if (!head.endsWith(";")) throw new Error(`${label}:${decl + 1}: a definition over several lines`);
      items.push({ name: (constant ?? alias)![1]!, kind: constant !== null ? "const" : "type", start, decl, end: decl });
      index = decl + 1;
      continue;
    }
    if (record === null && routine === null) throw new Error(`${label}:${decl + 1}: not a definition: ${head}`);
    let end = decl;
    while (lines[end] !== "}") {
      end++;
      if (end >= lines.length) throw new Error(`${label}:${decl + 1}: a definition without an end`);
    }
    items.push({ name: (record ?? routine)![1]!, kind: record !== null ? "struct" : "fn", start, decl, end });
    index = end + 1;
  }
  return { lines, items };
}

/** 🧼️ The text of a definition with the sharing keywords taken off: what must not change in a move. */
function bare(lang: Lang, source: Source, item: Item): string {
  const lines = source.lines.slice(item.start, item.end + 1);
  const at = item.decl - item.start;
  lines[at] = lang === "ts" ? lines[at]!.replace(/^export /, "") : lines[at]!.replace(/^pub\(crate\) /, "");
  if (item.kind === "struct") for (let line = at + 1; line < lines.length; line++) lines[line] = lines[line]!.replace(/^ {4}pub\(crate\) /, "    ");
  return lines.join("\n");
}

/** 🔬️ The identifiers the code of a definition uses: strings are emptied, and a name behind a dot or a path separator (a member, a method, a variant) does not count. */
function tokensOf(source: Source, item: Item): Set<string> {
  const code = source.lines
    .slice(item.decl, item.end + 1)
    .join("\n")
    .replace(/"(?:[^"\\]|\\.)*"/g, '""');
  const tokens = new Set<string>();
  for (const match of code.matchAll(/(?<![.\w:])[A-Za-z_]\w*/g)) tokens.add(match[0]);
  return tokens;
}

/** 📥️ What a source file imports: every name with the module it comes from and, in TypeScript, whether it is a type. */
function importsOf(lang: Lang, source: Source): Map<string, { from: string; type: boolean }> {
  const imports = new Map<string, { from: string; type: boolean }>();
  for (const line of source.lines) {
    if (lang === "ts") {
      const match = /^import \{ (.*) \} from "(.*)";$/.exec(line);
      if (match === null) continue;
      for (const entry of match[1]!.split(", ")) imports.set(entry.replace(/^type /, ""), { from: match[2]!, type: entry.startsWith("type ") });
    } else {
      const many = /^use ([\w:]+)::\{(.*)\};$/.exec(line);
      const one = /^use ([\w:]+)::(\w+);$/.exec(line);
      if (many !== null) for (const entry of many[2]!.split(", ")) imports.set(entry, { from: many[1]!, type: false });
      else if (one !== null) imports.set(one[2]!, { from: one[1]!, type: false });
    }
  }
  return imports;
}

/** 🔠️ The order rustfmt keeps inside braces: snake_case, then CamelCase, then SCREAMING_CASE, each by code unit. */
function rustRank(name: string): number {
  return /^[a-z]/.test(name) ? 0 : /[a-z]/.test(name) ? 1 : 2;
}

/** 🧾️ The import lines of a part. */
function importLines(lang: Lang, wanted: Map<string, { from: string; type: boolean }>): string[] {
  const groups = new Map<string, { name: string; type: boolean }[]>();
  for (const [name, origin] of wanted) groups.set(origin.from, [...(groups.get(origin.from) ?? []), { name, type: origin.type }]);
  const order = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);
  return [...groups.keys()].sort(order).map((from) => {
    const names = groups.get(from)!;
    if (lang === "ts") {
      const values = names.filter((entry) => !entry.type).map((entry) => entry.name);
      const types = names.filter((entry) => entry.type).map((entry) => `type ${entry.name}`);
      return `import { ${[...values.sort(order), ...types.sort(order)].join(", ")} } from "${from}";`;
    }
    const sorted = names.map((entry) => entry.name).sort((left, right) => rustRank(left) - rustRank(right) || order(left, right));
    return sorted.length === 1 ? `use ${from}::${sorted[0]};` : `use ${from}::{${sorted.join(", ")}};`;
  });
}

/** 🗣️ A sentence of a header in a language: in Rust the names in backticks are the twin's. */
function voiced(lang: Lang, sentence: string): string {
  return lang === "ts" ? sentence : sentence.replace(/`([a-z][A-Za-z]*)`/g, (_, name: string) => `\`${snake(name)}\``);
}

/** 🎩️ The header docstring of a part. */
function headerOf(lang: Lang, part: Part, original: Source): string[] {
  const twin = lang === "ts" ? `@see ./${FILES.rs} — the Rust twin` : `@see ../${directoryOf(part)}/${FILES.ts} — the TypeScript twin`;
  const facade = PARTS.find((candidate) => candidate.slug === FACADE)!;
  const parts = PARTS.filter((candidate) => candidate.slug !== FACADE).map((candidate) => `@see ../${directoryOf(candidate)}/${FILES[lang]} — ${voiced(lang, BRIEFS[candidate.slug]!)}`);
  if (part.slug === FACADE) {
    const end = lang === "ts" ? original.lines.findIndex((line) => line.endsWith("*/")) : original.lines.findIndex((line) => !line.startsWith("//!"));
    const head = original.lines.slice(0, end);
    const see = head.findIndex((line) => line.includes("@see "));
    const mark = lang === "ts" ? " * " : "//! ";
    const story =
      lang === "ts"
        ? ["PARTS. This file is the façade of the stage — `openStage`, `advance` and `frameOf` are all the package exports of", "it — over ten modules that share the working copy of a stage (the draft): the gaps between actors, the schedule of", "the stage, attention, locomotion, sociability, choice, population, the clock and the projection. The order above is", "the order of the clock."]
        : ["PARTS. This file is the façade of the stage — `open_stage`, `advance` and `frame_of` are all the crate exports of", "it — over ten modules that share the working copy of a stage (the draft): the gaps between actors, the schedule of", "the stage, attention, locomotion, sociability, choice, population, the clock and the projection. The order above is", "the order of the clock."];
    const lines = [...head.slice(0, see), ...story.map((line) => `${mark}${line}`), mark.trimEnd(), ...parts.map((line) => `${mark}${line}`), ...head.slice(see)];
    return lang === "ts" ? [...lines, " */"] : lines;
  }
  const door = lang === "ts" ? "A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it." : "A part of the stage, not of the crate: everything is `pub(crate)` at most.";
  const text = [...part.about.map((sentence) => voiced(lang, sentence)), door];
  const see = [`@see ../${directoryOf(facade)}/${FILES[lang]} — the façade of the stage and the normative order of a tick`, twin];
  if (lang === "ts") return [`/** ${part.lead}${V} ${text[0]}`, " *", ...text.slice(1).map((line) => ` * ${line}`), " *", ...see.map((line) => ` * ${line}`), " */"];
  return [`//! ${part.lead}${V} ${text[0]}`, "//!", ...text.slice(1).map((line) => `//! ${line}`), "//!", ...see.map((line) => `//! ${line}`)];
}

/** 🏗️ Splits one language of the stage and answers the text of every part. */
function splitOne(lang: Lang, text: string): Map<string, string> {
  const source = parse(lang, text, `stage ${lang}`);
  const byName = new Map(source.items.map((item) => [item.name, item]));
  if (byName.size !== source.items.length) throw new Error(`stage ${lang}: a name is defined twice`);
  const home = new Map<string, string>();
  for (const part of PARTS) {
    for (const region of part.regions) {
      for (const entry of region.items) {
        const name = nameIn(lang, entry);
        if (!byName.has(name)) {
          if (ONLY[entry] !== undefined && ONLY[entry] !== lang) continue;
          throw new Error(`stage ${lang}: ${name} is not defined`);
        }
        if (home.has(name)) throw new Error(`stage ${lang}: ${name} is dealt twice`);
        home.set(name, part.slug);
      }
    }
  }
  for (const item of source.items) if (!home.has(item.name)) throw new Error(`stage ${lang}: ${item.name} is dealt to no part`);
  const outer = importsOf(lang, source);
  const tokens = new Map(source.items.map((item) => [item.name, tokensOf(source, item)]));
  const shared = new Set<string>(lang === "rs" ? SHARED_FOR_TESTS : []);
  for (const item of source.items) for (const token of tokens.get(item.name)!) if (home.has(token) && home.get(token) !== home.get(item.name)) shared.add(token);
  const mark = source.lines.find((line) => line.startsWith("//#region "))!.slice("//#region ".length).replace("Constants", "");
  const mount = lang === "rs" ? source.lines.find((line) => line.startsWith("#[path = "))! : "";
  const files = new Map<string, string>();
  for (const part of PARTS) {
    const wanted = new Map<string, { from: string; type: boolean }>();
    const body: string[] = [];
    let last = -1;
    for (const region of part.regions) {
      const items = region.items
        .map((entry) => byName.get(nameIn(lang, entry)))
        .filter((item): item is Item => item !== undefined && home.get(item.name) === part.slug)
        .sort((left, right) => left.start - right.start);
      if (items.length === 0) continue;
      body.push("", `//#region ${mark}${region.name}`);
      items.forEach((item, at) => {
        if (item.start < last) throw new Error(`${part.slug} ${lang}: ${item.name} would change its place among its neighbours`);
        last = item.start;
        const lines = source.lines.slice(item.start, item.end + 1);
        const decl = item.decl - item.start;
        const open = lang === "ts" ? /^export /.test(lines[decl]!) : /^pub /.test(lines[decl]!);
        if (shared.has(item.name) && !open) {
          lines[decl] = lang === "ts" ? `export ${lines[decl]}` : `pub(crate) ${lines[decl]}`;
          if (item.kind === "struct") for (let line = decl + 1; line < lines.length - 1; line++) lines[line] = lines[line]!.replace(/^ {4}(\w+): /, "    pub(crate) $1: ");
        }
        if (at > 0 && region.name !== "Constants") body.push("");
        body.push(...lines);
        for (const token of tokens.get(item.name)!) {
          if (home.has(token) && home.get(token) !== part.slug) {
            const owner = PARTS.find((candidate) => candidate.slug === home.get(token))!;
            const kind = byName.get(token)!.kind;
            wanted.set(token, { from: lang === "ts" ? `../${directoryOf(owner)}/${FILES.ts}` : `crate::${owner.slug}`, type: lang === "ts" && kind === "type" });
          } else if (!home.has(token) && outer.has(token)) wanted.set(token, outer.get(token)!);
        }
      });
      body.push(`//#endregion ${mark}${region.name}`);
    }
    const projection = PARTS.find((candidate) => candidate.slug === "projection")!;
    const again = part.slug !== FACADE ? [] : lang === "ts" ? ["", `export { frameOf } from "../${directoryOf(projection)}/${FILES.ts}";`] : ["", "pub use crate::projection::frame_of;"];
    const tests = lang === "rs" && part.tests !== "none" ? ["", "#[cfg(test)]", mount, part.tests === "shared" ? "pub(crate) mod tests;" : "mod tests;"] : [];
    files.set(part.slug, `${[...headerOf(lang, part, source), "", ...importLines(lang, wanted), ...again, ...body, ...tests].join("\n")}\n`);
  }
  return files;
}

/** 🧮️ What a part file defines, imports and uses: the definitions with their bare text, and the names that are imported without use or used without being known. */
function audit(lang: Lang, text: string, label: string, known: Set<string>): { defined: Map<string, string>; idle: string[]; stray: string[] } {
  const source = parse(lang, text, label);
  const defined = new Map(source.items.map((item) => [item.name, bare(lang, source, item)]));
  const imports = importsOf(lang, source);
  const used = new Set<string>();
  for (const item of source.items) for (const token of tokensOf(source, item)) used.add(token);
  const idle = [...imports.keys()].filter((name) => !used.has(name));
  const stray = [...used].filter((name) => known.has(name) && !defined.has(name) && !imports.has(name));
  return { defined, idle, stray };
}

/** 📁️ The arguments of a command by name. */
function option(name: string): string {
  const at = process.argv.indexOf(`--${name}`);
  if (at < 0 || process.argv[at + 1] === undefined) throw new Error(`--${name} <path> is missing`);
  return resolve(process.argv[at + 1]!);
}

/** ✂️ `split`: writes every part of both languages below `--to`. */
function split(): void {
  const from = join(option("from"), MODULES, named("🎪", FACADE));
  const to = join(option("to"), MODULES);
  if (!existsSync(to)) throw new Error(`no modules directory at ${to}`);
  const report: string[] = [];
  for (const lang of ["ts", "rs"] as const) {
    const files = splitOne(lang, readFileSync(join(from, FILES[lang]), "utf8"));
    for (const part of PARTS) {
      const directory = join(to, directoryOf(part));
      mkdirSync(directory, { recursive: true });
      writeFileSync(join(directory, FILES[lang]), files.get(part.slug)!);
      report.push(`${directoryOf(part)}/${FILES[lang]}: ${files.get(part.slug)!.split("\n").length - 1} lines`);
    }
  }
  process.stdout.write(`${report.join("\n")}\n`);
}

/** ⚖️ `check`: every definition of the old stage exists exactly once in the parts of the new tree, with the same text, and the imports of every part are exactly what it uses. */
function check(): void {
  const old = join(option("old"), MODULES, named("🎪", FACADE));
  const fresh = join(option("new"), MODULES);
  let problems = 0;
  for (const lang of ["ts", "rs"] as const) {
    const before = parse(lang, readFileSync(join(old, FILES[lang]), "utf8"), `old stage ${lang}`);
    const expected = new Map(before.items.map((item) => [item.name, bare(lang, before, item)]));
    const known = new Set([...expected.keys(), ...importsOf(lang, before).keys()]);
    const found = new Map<string, string>();
    let lines = 0;
    for (const part of PARTS) {
      const label = `${directoryOf(part)}/${FILES[lang]}`;
      const text = readFileSync(join(fresh, directoryOf(part), FILES[lang]), "utf8");
      lines += text.split("\n").length - 1;
      const { defined, idle, stray } = audit(lang, text, label, known);
      for (const name of idle) process.stdout.write(`PROBLEM ${label}: imports ${name} without using it\n`);
      for (const name of stray) process.stdout.write(`PROBLEM ${label}: uses ${name} without importing it\n`);
      problems += idle.length + stray.length;
      for (const [name, body] of defined) {
        if (found.has(name)) {
          process.stdout.write(`PROBLEM ${label}: ${name} is defined a second time\n`);
          problems++;
        }
        found.set(name, body);
      }
    }
    let same = 0;
    for (const [name, body] of expected) {
      if (!found.has(name)) process.stdout.write(`PROBLEM ${lang}: ${name} is gone\n`);
      else if (found.get(name) !== body) process.stdout.write(`PROBLEM ${lang}: ${name} changed\n`);
      else same++;
    }
    const added = [...found.keys()].filter((name) => !expected.has(name));
    for (const name of added) process.stdout.write(`PROBLEM ${lang}: ${name} is new\n`);
    problems += expected.size - same + added.length;
    process.stdout.write(`${lang}: ${expected.size} definitions in the old stage (${before.lines.length} lines), ${found.size} in ${PARTS.length} parts (${lines} lines), ${same} identical\n`);
  }
  process.stdout.write(`${problems} problem(s)\n`);
  process.exitCode = problems === 0 ? 0 : 1;
}

const command = process.argv[2];
if (command === "split") split();
else if (command === "check") check();
else throw new Error("usage: split_stage.ts split --from <pets> --to <pets> | check --old <pets> --new <pets>");
