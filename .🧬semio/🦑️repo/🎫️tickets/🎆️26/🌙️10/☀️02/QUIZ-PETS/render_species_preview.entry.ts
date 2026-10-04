/** 🖼️ Ticket tool, browser side of `render_species_preview.mjs`: lays out the contact sheet of the species documents the script hands over in `PREVIEW`, drawn by the product's own modules — the core samples the clips, solves the rigs and moves the particles; the React target depicts the pets, their gear, their particles and their tints. Bundled by the script with `bun build`; never part of the product. */
import { clipTicks, pupilReach, restPose, sampleClip, solveRig, speciesIssues, type Clip, type Emitter, type Species, type Tint } from "@semio-tech/pets";
import { ORBIT_FADE, emitterKey, lifeTicks, particlesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✨️effects/🟦️.ts";
import { depict, paint, tiltedPlacement, tintPalette } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts";
import { paintEffects, stageEffects, tintEffects } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/✨️effects/🟦️.ts";
import { CHUTE_DOME, CHUTE_RISE, CHUTE_SPAN, canopyRim, equip, paintLadders, paintTools, rackLadders, type GearLadder, type GearTool } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🧰️gear/🟦️.ts";

declare const PREVIEW: { readonly species: readonly Species[]; readonly scale: number; readonly sections: readonly string[]; readonly clips: readonly string[] | null; readonly themes: readonly string[] };

const SVG = "http://www.w3.org/2000/svg";
const PHASES = [0, 0.25, 0.5, 0.75];
const MARGIN = 14;
const NEW_ACTIVITIES = ["hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "push", "dizzy", "shrug", "scoot"] as const;

type Look = { readonly x: number; readonly y: number; readonly lid: number; readonly mood: number };
type Room = { readonly left?: number; readonly right?: number; readonly up?: number };
type Scene = {
  readonly label: string;
  readonly clip?: Clip;
  readonly phase?: number;
  readonly look?: Look;
  readonly facing?: 1 | -1;
  readonly tint?: Tint;
  readonly tilt?: number;
  readonly pivot?: { readonly x: number; readonly y: number };
  readonly tools?: readonly GearTool[];
  readonly ladders?: readonly GearLadder[];
  readonly emitter?: Emitter;
  readonly lift?: number;
  readonly room?: Room;
  readonly wall?: "side" | "corner";
  readonly block?: boolean;
  readonly dot?: boolean;
};

const REST: Look = { x: 0, y: 0, lid: 0, mood: 0.4 };

const element = <Tag extends keyof SVGElementTagNameMap>(tag: Tag, attributes: Record<string, string | number>): SVGElementTagNameMap[Tag] => {
  const made = document.createElementNS(SVG, tag);
  for (const [name, value] of Object.entries(attributes)) made.setAttribute(name, String(value));
  return made;
};

/** 🎇️ The tick at which an emitter is shown beside a clip at `phase`: a burst along its life, a ring one lap on, a steady stream once it is full. */
const sampleTick = (emitter: Emitter, phase: number): number => {
  const life = lifeTicks(emitter);
  if (emitter.motion === "burst") return Math.round((0.1 + 0.8 * phase) * life);
  if (emitter.motion === "orbit") return ORBIT_FADE + Math.round(phase * life);
  return life + Math.round(phase * life);
};

/** 🧩️ One cell of the sheet: the pet of `scene` on its ground, with its gear, its particles, its tint and what it leans on. */
function cell(species: Species, theme: string, scene: Scene): HTMLElement {
  const scale = PREVIEW.scale;
  const { width, height } = species.size;
  const hover = species.locomotion.hover ?? 0;
  const left = Math.max(width / 2 + MARGIN, scene.room?.left ?? 0);
  const right = Math.max(width / 2 + MARGIN, scene.room?.right ?? 0);
  const up = Math.max(height + hover + MARGIN, (scene.room?.up ?? 0) + (scene.lift ?? 0));
  const down = 14;
  const feet = { x: left, y: up - hover - (scene.lift ?? 0) };
  const ground = up;
  const figure = document.createElement("figure");
  figure.className = "cell";
  figure.style.width = `${(left + right) * scale}px`;
  figure.style.height = `${(up + down) * scale}px`;

  const decor = element("svg", { class: "decor", width: (left + right) * scale, height: (up + down) * scale, viewBox: `0 0 ${left + right} ${up + down}` });
  decor.append(element("rect", { class: "panel", x: 0, y: ground, width: left + right, height: down }));
  decor.append(element("rect", { class: "box", x: feet.x - width / 2, y: feet.y - height, width, height }));
  const wallX = feet.x + width / 2 + 1;
  if (scene.wall === "side") decor.append(element("path", { class: "wall", d: `M ${wallX} 0 V ${ground}` }));
  if (scene.wall === "corner") decor.append(element("path", { class: "wall", d: `M ${wallX} ${ground} V ${feet.y - height * 0.55} H ${left + right}` }));
  if (scene.block) decor.append(element("rect", { class: "block", x: wallX + 1, y: feet.y - 16, width: 20, height: 16, rx: 3 }));

  const clip = scene.clip;
  const pose = clip === undefined ? restPose(species) : sampleClip(species, clip, Math.round((scene.phase ?? 0) * clipTicks(clip)));
  const bones = solveRig(species, pose);
  const look = scene.look ?? REST;
  const facing = scene.facing ?? 1;
  const tilt = scene.tilt ?? 0;
  const pivot = scene.pivot ?? { x: 0, y: -species.grip };
  const depiction = depict(species, document);
  paint(depiction, { species: species.id, x: feet.x * scale, y: feet.y * scale, facing, activity: "idle", opacity: 1, bones, eyes: species.face.eyes.map((eye) => ({ x: look.x * pupilReach(eye), y: look.y * pupilReach(eye), lid: look.lid })), mood: look.mood, spirits: look.mood } as never, scale);
  depiction.element.style.transform = tiltedPlacement(feet.x * scale, feet.y * scale, facing * scale, scale, tilt, pivot);
  tintPalette(depiction.element, species.palette, scene.tint);
  const equipment = equip(species, document);
  depiction.element.prepend(equipment.back);
  depiction.element.append(equipment.front);
  const staged = (tool: GearTool): GearTool => (tool.kind === "rope" || tool.kind === "hook" ? { ...tool, x: tool.x + feet.x, y: tool.y + feet.y } : tool);
  paintTools(equipment, { x: feet.x, y: feet.y, facing, tilt, pivot, tools: (scene.tools ?? []).map(staged) });
  const rack = rackLadders(document);
  paintLadders(rack, (scene.ladders ?? []).map((ladder) => ({ ...ladder, x0: ladder.x0 + feet.x, y0: ladder.y0 + feet.y, x1: ladder.x1 + feet.x, y1: ladder.y1 + feet.y })), scale);
  const effects = stageEffects(document);
  const emitter = scene.emitter;
  if (emitter !== undefined) {
    const bone = Math.max(0, species.bones.findIndex((candidate) => candidate.id === emitter.bone));
    const [a, b, c, d, e, f] = bones.slice(bone * 6, bone * 6 + 6) as [number, number, number, number, number, number];
    const origin = { x: feet.x + facing * (a * emitter.x + c * emitter.y + e), y: feet.y + b * emitter.x + d * emitter.y + f };
    const key = emitterKey(1, 0, species.emitters.indexOf(emitter), 0);
    const particles = particlesOf(emitter, origin, facing, 0, null, sampleTick(emitter, scene.phase ?? 0.25), key);
    tintEffects(effects, species, scene.tint);
    paintEffects(effects, new Map([[species.id, species]]), particles.map((particle) => ({ ...particle, species: species.id, emitter: emitter.id })), scale);
  }
  const caption = document.createElement("figcaption");
  caption.textContent = scene.label;
  figure.append(decor, rack.element, depiction.element, effects.element, caption);
  if (scene.dot) {
    const dot = document.createElement("i");
    dot.style.left = `${(feet.x + facing * pivot.x) * scale}px`;
    dot.style.top = `${(feet.y + pivot.y) * scale}px`;
    figure.append(dot);
  }
  figure.dataset.theme = theme;
  return figure;
}

/** 🎞️ One band of the sheet: its title, then the scenes once per theme. A sheet is only ever cut between bands. */
function band(species: Species, themes: readonly string[], scenes: readonly Scene[], title?: string): HTMLElement {
  const made = document.createElement("div");
  made.className = "band";
  if (title !== undefined) {
    const line = document.createElement("h4");
    line.textContent = title;
    made.append(line);
  }
  for (const theme of themes) {
    const strip = document.createElement("div");
    strip.className = `row ${theme}`;
    for (const scene of scenes) strip.append(cell(species, theme, scene));
    made.append(strip);
  }
  return made;
}

const caption = (clip: Clip, phase: number): string => `${clip.id} @${phase} (${clip.seconds}s${clip.loop ? ", loop" : ""})`;

/** 🧗️ The scenes of one new activity for one of its clips: what the pet holds, hangs from or leans on while it plays. */
function activityScenes(species: Species, activity: (typeof NEW_ACTIVITIES)[number], clip: Clip): Scene[] {
  const { width, height } = species.size;
  const grip = species.grip;
  const named = (phase: number, extra = ""): string => `@${phase}${extra}`;
  const each = (scene: (phase: number, index: number) => Omit<Scene, "label" | "clip" | "phase"> & { readonly note?: string }): Scene[] =>
    PHASES.map((phase, index) => {
      const { note, ...rest } = scene(phase, index);
      return { ...rest, clip, phase, label: named(phase, note ?? "") };
    });
  const [left, right] = canopyRim(species);
  const canopyUp = grip + CHUTE_RISE * height + (species.canopy === undefined ? CHUTE_DOME * height * 1.1 : height * 0.8) + 6;
  const canopySide = Math.max(Math.abs(left.x), Math.abs(right.x), species.canopy === undefined ? CHUTE_SPAN * width : 0) + 10;
  switch (activity) {
    case "hang":
      return each((_, index) => ({ tilt: [20, 0, -20, 0][index]! / 360, lift: 16, dot: true, room: { up: height + 24, left: width * 0.75, right: width * 0.75 }, note: ` tilt ${[20, 0, -20, 0][index]}°` }));
    case "tumble":
      return each((_, index) => ({ tilt: [0.06, 0.2, -0.15, -0.04][index]!, pivot: { x: 0, y: -height / 2 }, lift: 22, dot: true, room: { up: height + 30, left: width * 0.8, right: width * 0.8 }, note: ` tilt ${[0.06, 0.2, -0.15, -0.04][index]}` }));
    case "glide":
      return [
        ...each((_, index) => ({ tools: [{ kind: "chute", open: 1, sway: [0.02, 0, -0.02, 0][index]! }], tilt: [-6, 0, 6, 0][index]! / 360, lift: 24, room: { up: canopyUp, left: canopySide, right: canopySide }, note: ` sway ${[0.02, 0, -0.02, 0][index]}` })),
        { clip, phase: 0, tools: [{ kind: "chute", open: 0.45, sway: 0 }], lift: 24, room: { up: canopyUp, left: canopySide, right: canopySide }, label: "opening (0.45)" },
        { clip, phase: 0, tools: [{ kind: "chute", open: 1.25, sway: 0 }], lift: 24, room: { up: canopyUp + 8, left: canopySide, right: canopySide }, label: "overshooting (1.25)" },
      ];
    case "aim":
      return each((_, index) => ({ pivot: { x: 0, y: 0 }, tools: [{ kind: "gun", aim: [-0.1, -0.14, -0.18, -0.14][index]! }], room: { right: width / 2 + 22 }, note: ` aim ${[-0.1, -0.14, -0.18, -0.14][index]}` }));
    case "reel": {
      const anchor = { x: width * 0.7, y: -(grip + height * 0.9) };
      return each((_, index) => ({ tilt: [8, 3, -4, 2][index]! / 360, lift: 20, tools: [{ kind: "rope", ...anchor, slack: 0 }, { kind: "hook", ...anchor }], room: { up: grip + height * 0.9 + 12, right: width * 0.7 + 14 }, note: ` tilt ${[8, 3, -4, 2][index]}°` }));
    }
    case "climb":
      return [
        ...each(() => ({ wall: "side" as const, lift: 22, room: { up: height + 34 }, note: "" })),
        ...(species.gear.includes("ladder") ? [{ clip, phase: 0.25, lift: 22, ladders: [{ x0: -height * 0.3, y0: 22, x1: height * 0.15, y1: 22 - height * 1.8, rungs: Math.floor((height * 1.86) / 10.5), opacity: 1 }], room: { up: height * 1.8 + 6, left: width / 2 + 14 }, label: "@0.25 on a standing ladder" } satisfies Scene] : []),
      ];
    case "mantle":
      return each(() => ({ wall: "corner" as const, lift: 0, room: { up: height + 20, right: width / 2 + 30 }, note: "" }));
    case "slide":
      return each(() => ({ wall: "side" as const, lift: 22, room: { up: height + 34 }, note: "" }));
    case "carry": {
      const length = Math.round(height * 2);
      return [
        ...each((_, index) => ({ pivot: { x: 0, y: 0 }, tools: [{ kind: "ladder", lean: [0.22, 0.225, 0.22, 0.215][index]!, length }], room: { left: length / 2 + 8, right: length / 2 + 8 }, note: ` lean ${[0.22, 0.225, 0.22, 0.215][index]}` })),
        { clip, phase: 0, pivot: { x: 0, y: 0 }, tools: [{ kind: "ladder", lean: 0.07, length }], room: { up: length + 8, right: length / 2 }, label: "raised (lean 0.07)" },
      ];
    }
    case "push":
      return each(() => ({ block: true, room: { right: width / 2 + 28 }, note: "" }));
    default:
      return each(() => ({}));
  }
}

const WHAT: Readonly<Record<(typeof NEW_ACTIVITIES)[number], string>> = {
  hang: "held at the scruff: the whole drawing tilts about the grip (red dot)",
  tumble: "thrown: the whole drawing tilts about its middle (red dot)",
  glide: "under the open parachute, hanging from the grip",
  aim: "with the grappling gun at its side",
  reel: "on a rope that holds it at the grip, the hook at the far end",
  climb: "at a wall on the side it faces",
  mantle: "over the corner of a wall onto its top",
  slide: "down a wall on the side it faces",
  carry: "with a ladder on its back",
  push: "against a block",
  dizzy: "after a long fall or a shake",
  shrug: "after a miss, or when it has had enough",
  scoot: "making room for a neighbour",
};

function heading(text: string, level: "h2" | "h3" = "h3"): HTMLElement {
  const made = document.createElement(level);
  made.textContent = text;
  return made;
}

for (const species of PREVIEW.species) {
  const sections = PREVIEW.sections;
  const clipOf = (id: string | undefined): Clip | undefined => species.clips.find((clip) => clip.id === id);
  const emitterOf = (id: string | undefined): Emitter | undefined => species.emitters.find((emitter) => emitter.id === id);
  const issues = speciesIssues(species);
  const block = document.createElement("section");
  block.className = "species";
  block.append(heading(`${species.id} · ${species.name?.en ?? ""} · ${species.name?.de ?? ""} · ${species.size.width}×${species.size.height} · ${species.locomotion.gait} · grip ${species.grip} · gear ${species.gear.join(", ") || "none"} · ${species.states.length} states, ${species.tricks.length} tricks, ${species.emitters.length} emitters${species.canopy === undefined ? "" : ", own canopy"} · ${issues.length === 0 ? "valid" : `${issues.length} issues: ${issues.slice(0, 4).map((issue) => `${issue.code} ${issue.path}`).join("; ")}`}`, "h2"));

  if (sections.includes("rest")) {
    block.append(
      band(species, PREVIEW.themes, [
        { label: "rest" },
        { label: "look left, sad", look: { x: -1, y: 0.2, lid: 0, mood: -0.8 } },
        { label: "look right, happy", look: { x: 1, y: -0.2, lid: 0, mood: 1 } },
        { label: "blink", look: { x: 0, y: 0, lid: 1, mood: 0.4 } },
        { label: "facing left", facing: -1, look: { x: 0.6, y: 0, lid: 0, mood: 0.4 } },
      ]),
    );
  }

  if (sections.includes("clips")) {
    for (const clip of species.clips) {
      if (PREVIEW.clips !== null && !PREVIEW.clips.includes(clip.id)) continue;
      block.append(band(species, PREVIEW.themes.slice(0, 1), PHASES.map((phase) => ({ label: caption(clip, phase), clip, phase }))));
    }
  }

  if (sections.includes("states") && species.states.length > 0) {
    block.append(heading(`states: ${species.states.map((state) => `${state.id}${state.lasts === undefined ? "" : ` (${state.lasts}s → ${state.then ?? "?"})`}`).join(" · ")}`));
    {
      const scenes: Scene[] = [];
      for (const state of species.states) {
        const overlay = clipOf(state.clip);
        const emitter = emitterOf(state.emitter);
        const tinted = state.tint === undefined ? "no tint" : `tint ${(["body", "accent", "detail"] as const).filter((tone) => state.tint?.[tone] !== undefined).join("+")}`;
        scenes.push({ label: `state ${state.id}: rest, ${tinted}`, tint: state.tint, room: { up: species.size.height + 26 } });
        if (overlay !== undefined || emitter !== undefined) scenes.push({ label: `state ${state.id}: ${overlay === undefined ? "rest" : caption(overlay, 0.25)}${emitter === undefined ? "" : ` + ${emitter.id} (${emitter.motion})`}`, tint: state.tint, clip: overlay, phase: 0.25, emitter, room: { up: species.size.height + 26, left: species.size.width / 2 + 24, right: species.size.width / 2 + 24 } });
        if (state.clip !== undefined && overlay === undefined) scenes.push({ label: `state ${state.id}: clip "${state.clip}" is missing` });
        if (state.emitter !== undefined && emitter === undefined) scenes.push({ label: `state ${state.id}: emitter "${state.emitter}" is missing` });
      }
      block.append(band(species, PREVIEW.themes, scenes));
    }
  }

  if (sections.includes("tricks")) {
    const shows: { readonly title: string; readonly clip: string; readonly emitter?: string; readonly tint?: Tint }[] = [
      ...species.tricks.map((trick) => ({ title: `trick ${trick.id} [${trick.cues.join(", ")}]${trick.from === undefined ? "" : ` from ${trick.from.join("/")}`}${trick.to === undefined ? "" : ` → ${trick.to}`}`, clip: trick.clip, emitter: trick.emitter, tint: species.states.find((state) => state.id === trick.from?.[0])?.tint })),
      ...(species.purr === undefined ? [] : [{ title: "purr", clip: species.purr.clip, emitter: species.purr.emitter }]),
    ];
    if (shows.length > 0) block.append(heading(`tricks and purr (${species.tricks.length} tricks)`));
    for (const show of shows) {
      const clip = clipOf(show.clip);
      const emitter = emitterOf(show.emitter);
      const title = `${show.title}: ${clip === undefined ? `clip "${show.clip}" is missing` : `${clip.id} (${clip.seconds}s${clip.loop ? ", loop" : ""})`}${emitter === undefined ? (show.emitter === undefined ? "" : ` — emitter "${show.emitter}" is missing`) : ` + ${emitter.id} (${emitter.motion} ×${emitter.count}, ${emitter.life}s)`}`;
      block.append(band(species, PREVIEW.themes, clip === undefined ? [{ label: "rest" }] : PHASES.map((phase) => ({ label: `@${phase}`, clip, phase, emitter, tint: show.tint, room: { up: species.size.height + 30, left: species.size.width / 2 + 28, right: species.size.width / 2 + 28 } })), title));
    }
  }

  if (sections.includes("gear")) {
    const missing = NEW_ACTIVITIES.filter((activity) => (species.repertoire[activity as keyof typeof species.repertoire] ?? []).length === 0);
    block.append(heading(`getting around and being handled${missing.length === 0 ? "" : ` — no clip in the repertoire for: ${missing.join(", ")}`}`));
    for (const activity of NEW_ACTIVITIES) {
      for (const id of species.repertoire[activity as keyof typeof species.repertoire] ?? []) {
        const clip = clipOf(id);
        block.append(band(species, PREVIEW.themes, clip === undefined ? [{ label: "rest" }] : activityScenes(species, activity, clip), `${activity}: ${clip === undefined ? `clip "${id}" is missing` : `${clip.id} (${clip.seconds}s${clip.loop ? ", loop" : ""})`} — ${WHAT[activity]}`));
      }
    }
  }
  document.body.append(block);
}
document.body.dataset.ready = "true";
