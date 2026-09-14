/** 🕹️ wgpu WORLD3D INTERACTION — hover, selection and camera on the 3-D preview surface, hop by hop.
 *
 * `📓️wgpu-input-hit-runtime-2026-09-13.md` §11 proved the pointer reaches the World3d surface and the
 * intent is admitted, then stopped: `lanes.selection` and `camera` stayed byte-identical. This probe
 * reads the two traces that separate the remaining hops, both added by the `wgpu-world3d-interaction`
 * lane:
 *
 *   `[DEBUG] world3d interaction surface=… enter|leave g=N step=… <interaction_census()>`
 *       — one pair per INTENT: its phase/button/down/point, the active transition (`Pick[Hover hit=…]`,
 *         `Plan[0/1 Some(Camera)]`, `MarqueePick`, …), the gestures, the pick inputs (`draws`,
 *         `objects`, `bounds`, `pick`) and the surface's own `hover=`/`selected=`.
 *   `[DEBUG] frame input action controller=… action=… args=…`
 *       — every action the frame actually took OUT of the bounded input queue, i.e. what the world
 *         authority published (`setCamera`, `interactionHover`, `interactionSelect`).
 *   `[DEBUG] frame deferred action failed: …`
 *       — the guest's refusal, which used to be swallowed into `shell.error`.
 *
 * Every point is DERIVED from the shell's own `[DEBUG] wgpu-shell dock plan` body, never guessed.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-world3d/run-1 bun 🐍️wgpu-world3d-interaction-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "";
const role = process.env.SEMIO_PROBE_ROLE ?? "";
const baseUrl = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}${role ? `&role=${role}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 90);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 15);
const outRoot = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-world3d/run");
mkdirSync(outRoot, { recursive: true });

/** 📚️ Every example this run gestures on. One page load each, one folder each, all nine hops each —
 * so the hover/select/camera coverage the ticket asked to widen from `hexagonal-mushroom-column` to
 * all eight is one lane rather than eight lanes, and every hop is scored against THAT example's own
 * published mesh ids and bounding box instead of one hardcoded expectation
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
 *
 * `SEMIO_PROBE_URL` still carries the lane's axes (`mode`/`role`); the example is swapped on it. */
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? decodeURIComponent(/[?&]example=([^&]+)/.exec(baseUrl)?.[1] ?? "")).split(",").map((entry) => entry.trim()).filter(Boolean);
if (examples.length === 0) examples.push("");
const urlFor = (example) => {
  const stripped = baseUrl.replace(/([?&])example=[^&]*/, "$1").replace(/[?&]$/, "");
  if (!example) return stripped;
  return `${stripped}${stripped.includes("?") ? "&" : "?"}example=${encodeURIComponent(example)}`;
};
const EXAMPLES_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "✏️s", "🔌️plugins", "🌀️procedural", "🗿️artifacts", "🧊️generation3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "📚️examples");
const fixtureFor = (example) => {
  try {
    const dir = readdirSync(EXAMPLES_ROOT).find((entry) => entry.endsWith(example));
    return dir ? JSON.parse(readFileSync(join(EXAMPLES_ROOT, dir, "🧫️fixtures", "🧩️example", "🔣️.json"), "utf8")) : null;
  } catch {
    return null;
  }
};

const runs = [];
const allLines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });

for (const example of examples) {
const url = urlFor(example);
const fixture = fixtureFor(example);
const outDir = join(outRoot, example || "(none)");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)} :: ${String(e?.stack ?? "").slice(0, 3000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const nudge = (n) => page.mouse.move(3 + (n % 5), 3 + (n % 5)).catch(() => {});
const pause = (ms) => page.waitForTimeout(ms);

const cameraTrace = () => /camera="(\{.*?\})"/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;
const lanes = () => {
  const raw = /lanes=\[(.*?)\]/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? "";
  return Object.fromEntries(raw.split(",").filter(Boolean).map((entry) => entry.split(":")).map(([k, v]) => [k, Number(v)]));
};
const settledCommands = () => has("wgpu-shell command").map((line) => /wgpu-shell command (\S+) settled(.*)$/.exec(line)).filter(Boolean).map((m) => `${m[1]}${m[2]}`);
/** 🎬️ Every action the frame took out of the bounded input queue, newest last. */
const publishedActions = () => has("frame input action").map((line) => /controller=(\S+) action=(\S+) args=(\S+)/.exec(line)).filter(Boolean).map((m) => `${m[1]}/${m[2]}/args=${m[3]}`);
const dispatchFailures = () => has("frame deferred action failed").map((line) => line.split("frame deferred action failed: ").at(-1).slice(0, 240));
/** 🕹️ The world authority's own census lines, trimmed to what a reader needs. */
const interactionTrace = () => has("world3d interaction surface=").map((line) => line.split("[DEBUG] ").at(-1));
/** 👆️ The surface's own hover/selection, as the authority reports it. */
const authorityState = () => {
  const line = interactionTrace().at(-1) ?? "";
  return { hover: /hover=(Some\(".*?"\)|None)/.exec(line)?.[1] ?? null, selected: Number(/selected=(\d+)/.exec(line)?.[1] ?? -1), draws: Number(/draws=(\d+)/.exec(line)?.[1] ?? -1), objects: Number(/objects=(\d+)/.exec(line)?.[1] ?? -1), camera: /camera=(\S+)/.exec(line)?.[1] ?? null };
};

const dockPlan = () => {
  const line = has("wgpu-shell dock plan").at(-1);
  if (!line) return {};
  const plan = {};
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

/** 🧊️ The renderer's own published-mesh census, per World3d surface — ids, roles, bounds, instances
 * and the guest's own `selection_json`. This is what turns "an interactionHover action fired" into
 * "the ray hit THIS mesh": the expected target is derived from the example's own publication rather
 * than named in this file. */
const meshStats = async () =>
  page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpMeshStats !== "function") return null;
      try {
        const raw = await beacon.dumpMeshStats();
        return raw ? JSON.parse(raw) : null;
      } catch {
        return null;
      }
    })
    .catch(() => null);

const previewSurfaceOf = (published) => {
  const surfaces = (published?.surfaces ?? []).filter((surface) => surface.surfaceId.includes("preview"));
  if (surfaces.length === 0) return null;
  return surfaces.reduce((best, surface) => ((surface.meshes ?? []).length > (best.meshes ?? []).length ? surface : best), surfaces[0]);
};

/** 🎯️ Which topology target a ray through the surface CENTRE must report. Only a mesh with triangles
 * can be hit by a pick ray, so the expected set is the interaction id of every instance whose mesh
 * carries indices; a wire-only example has none, and "no hover" is then the correct answer rather
 * than a miss. */
const pickableTargets = (surface) => {
  const solid = new Set((surface?.meshes ?? []).filter((mesh) => mesh.indices > 0).map((mesh) => mesh.id));
  return [...new Set((surface?.instances ?? []).filter((instance) => solid.has(instance.meshId)).map((instance) => instance.interactionId))];
};

/** 📦️ Does the published camera FRAME the committed box — the assertion "the camera string changed"
 * never made. The target must sit inside the box (inflated by the LOD chord tolerance) and the eye
 * must stand far enough back that the box's bounding sphere fits the vertical field of view. */
const cameraFit = (surface, box, tolerance) => {
  const camera = surface?.camera ?? null;
  if (!camera || !box) return { ok: false, reason: "no camera or no published box", camera, box };
  const position = camera.position ?? null;
  const target = camera.target ?? [0, 0, 0];
  if (!position) return { ok: false, reason: "the published camera carries no position", camera };
  const centre = [0, 1, 2].map((axis) => (box.min[axis] + box.max[axis]) / 2);
  const radius = Math.hypot(...[0, 1, 2].map((axis) => (box.max[axis] - box.min[axis]) / 2));
  const distance = Math.hypot(...[0, 1, 2].map((axis) => position[axis] - centre[axis]));
  const targetInside = [0, 1, 2].every((axis) => target[axis] >= box.min[axis] - tolerance && target[axis] <= box.max[axis] + tolerance);
  const halfFov = ((camera.fov ?? 45) * Math.PI) / 360;
  const covered = distance * Math.tan(halfFov);
  return { ok: targetInside && distance > radius && covered >= radius, targetInside, distance, radius, covered, position, target, fov: camera.fov ?? 45 };
};

/** 🔍️ What the shell's own retained tree SHOWS of a selection: every node the retained document
 * marks `selected`, and every node whose text names the target. A selection the guest accepted but
 * no panel renders is a selection the user cannot see, and the two readings separate "the document
 * changed" from "a label happens to contain the word". */
const shellSelectionWitness = async (needle) =>
  page
    .evaluate(async (needle) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return { selected: [], needleHits: 0, sample: [] };
      const ids = JSON.parse((await beacon.dumpStructure()) || "{}").windowIds ?? [];
      const selected = [];
      const named = [];
      for (const id of ids) {
        const dump = JSON.parse((await beacon.dumpStructure(id)) || "{}");
        for (const node of dump.nodes ?? []) {
          if (node.state?.selected) selected.push(`${id}:${node.path}=${String(node.text ?? "").slice(0, 60)}`);
          if (needle && typeof node.text === "string" && node.text.toLowerCase().includes(needle.toLowerCase())) named.push(`${id}:${node.path}=${node.text.slice(0, 60)}`);
        }
      }
      return { selected, needleHits: named.length, sample: named.slice(0, 6) };
    }, needle)
    .catch(() => ({ selected: [], needleHits: 0, sample: [] }));

/** 📐️ The committed box this example's camera must frame, and the band it is held to — filled from
 * the live publication once the preview has settled, because the fixture states the box in WORLD
 * units and the publication is the only place the live surface reports them. */
let oracleBox = null;
let oracleTolerance = Math.max(fixture?.expect?.boundingBoxTolerance ?? 0, { coarse: 0.15, fine: 0.02 }[fixture?.delivery?.lodMode ?? ""] ?? 0.05);

const results = { url, example, steps: {} };
const record = (name, value) => {
  results.steps[name] = value;
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(value).slice(0, 4000)}`);
};

/** 📸️ One gesture's full before/after, so a hop that moves nothing is told apart from one that moves
 * the shell but not the guest. */
const snapshot = () => ({ camera: cameraTrace(), lanes: lanes(), authority: authorityState(), actions: publishedActions().length, commands: settledCommands().length, failures: dispatchFailures().length, intents: interactionTrace().length });
const snapshotWithPublication = async () => ({ ...snapshot(), published: previewSurfaceOf(await meshStats()), shell: await shellSelectionWitness("") });
const deltaWithPublication = async (before) => {
  const surface = previewSurfaceOf(await meshStats());
  return {
    ...delta(before),
    publishedBefore: { selected: before.published?.selected ?? [], hovered: before.published?.hovered ?? null },
    publishedAfter: { selected: surface?.selected ?? [], hovered: surface?.hovered ?? null },
    cameraFit: cameraFit(surface, oracleBox, oracleTolerance),
  };
};
const delta = (before) => ({
  guestCameraChanged: before.camera !== null && cameraTrace() !== null && before.camera !== cameraTrace(),
  localCameraChanged: before.authority.camera !== null && authorityState().camera !== null && before.authority.camera !== authorityState().camera,
  guestCameraAfter: cameraTrace(),
  selectionLaneBefore: before.lanes.selection,
  selectionLaneAfter: lanes().selection,
  authorityBefore: before.authority,
  authorityAfter: authorityState(),
  newActions: publishedActions().slice(before.actions),
  newCommands: settledCommands().slice(before.commands),
  newFailures: dispatchFailures().slice(before.failures),
  newIntents: interactionTrace().slice(before.intents).slice(-24),
});

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let booted = null;
for (let tick = 0; tick < bootBudget * 2; tick += 1) {
  await nudge(tick);
  await pause(500);
  if (has("boot_shell leave").length) {
    booted = at();
    break;
  }
}
record("boot", { bootShellLeaveMs: booted, dockPlan: dockPlan() });
for (let tick = 0; tick < settle; tick += 1) {
  await nudge(tick);
  await pause(1000);
}

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? null;
const body = previewId ? plan[previewId] : null;
if (!body) {
  record("fatal", { reason: "no preview window in the dock plan", plan });
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
  runs.push(results);
  await page.close();
  continue;
}
const centre = [body.x + body.w / 2, body.y + body.h / 2];
const corner = [body.x + 12, body.y + body.h - 12];
record("preview", { previewId, body, centre, corner, lanes: lanes(), camera: cameraTrace() });

/** 🎯️ The example's OWN oracle, read off the settled publication: which targets a centre ray may
 * report, what the committed box is, and whether the boot camera already frames it. */
{
  const surface = previewSurfaceOf(await meshStats());
  const targets = (surface?.meshes ?? []).filter((mesh) => mesh.role === (fixture?.preview?.kind ?? "solid") && mesh.bboxMin);
  oracleBox = fixture ? { min: fixture.expect.boundingBoxMin, max: fixture.expect.boundingBoxMax } : targets.length > 0 ? { min: [0, 1, 2].map((axis) => Math.min(...targets.map((mesh) => mesh.bboxMin[axis]))), max: [0, 1, 2].map((axis) => Math.max(...targets.map((mesh) => mesh.bboxMax[axis]))) } : null;
  record("oracle", {
    example,
    fixture: fixture === null ? null : { meshes: fixture.delivery.meshes, meshRoles: fixture.delivery.meshRoles, previewKind: fixture.preview.kind, boundingBoxMin: fixture.expect.boundingBoxMin, boundingBoxMax: fixture.expect.boundingBoxMax },
    tolerance: oracleTolerance,
    surfaceId: surface?.surfaceId ?? null,
    rect: surface?.rect ?? null,
    publishedMeshes: (surface?.meshes ?? []).map((mesh) => ({ id: mesh.id, role: mesh.role, indices: mesh.indices, bboxMin: mesh.bboxMin, bboxMax: mesh.bboxMax })),
    publishedRoles: surface?.roles ?? {},
    expectedTargets: pickableTargets(surface),
    bootCameraFit: cameraFit(surface, oracleBox, oracleTolerance),
  });
}
await page.screenshot({ path: join(outDir, "shot-booted.png"), type: "png" }).catch(() => {});

const settleTicks = async (count) => {
  for (let tick = 0; tick < count; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
};

// ── 1 — hover the mesh ────────────────────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.mouse.move(centre[0], centre[1]);
  await settleTicks(5);
  record("h1_hover_centre", await deltaWithPublication(before));
}

// ── 2 — hover off the mesh (the surface's own empty corner) ────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.mouse.move(corner[0], corner[1]);
  await settleTicks(4);
  record("h2_hover_empty", await deltaWithPublication(before));
}

// ── 3 — click the mesh ────────────────────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.mouse.move(centre[0], centre[1]);
  await pause(400);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await settleTicks(6);
  const needle = (pickableTargets(previewSurfaceOf(await meshStats()))[0] ?? "").split("@")[0];
  record("h3_click_select", { ...(await deltaWithPublication(before)), inspectorBefore: before.shell, inspector: await shellSelectionWitness(needle), needle });
}

// ── 4 — shift-click a second point on the mesh ────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.keyboard.down("Shift");
  await page.mouse.move(centre[0] + 18, centre[1] + 18);
  await pause(300);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await page.keyboard.up("Shift");
  await settleTicks(5);
  record("h4_shift_add", await deltaWithPublication(before));
}

// ── 5 — empty click clears ────────────────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.mouse.move(corner[0], corner[1]);
  await pause(300);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await settleTicks(5);
  record("h5_empty_clear", await deltaWithPublication(before));
}

// ── 6 — marquee (left drag enclosing the mesh), BEFORE the camera gestures so the band is measured
//        against the boot camera the fixture's own path assumes ──────────────────────────────────
{
  const before = await snapshotWithPublication();
  // 🔲️ A WINDOW marquee (left → right) only takes what it fully encloses, so the band spans nearly
  // the whole preview body — the same shape as the fixture's `marquee-release-replaces` path over its
  // own 800x600 viewport.
  // 🔲️ RIGHT → LEFT is the CROSSING band (`WorldMarqueePickCursor::new`'s `crossing`), which takes
  // whatever it touches rather than only what it fully encloses — the wider of the two rules, so a
  // pick that reports nothing here is a real hole and not a strict-enclosure answer.
  const from = [body.x + body.w - 8, body.y + 8];
  const to = [body.x + 8, body.y + body.h - 8];
  await page.mouse.move(from[0], from[1]);
  await page.mouse.down();
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
    await pause(120);
  }
  await page.mouse.up();
  await settleTicks(6);
  record("h6_marquee", await deltaWithPublication(before));
}

// ── 7 — wheel zoom ────────────────────────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.mouse.move(centre[0], centre[1]);
  await pause(500);
  for (let tick = 0; tick < 5; tick += 1) {
    await page.mouse.wheel(0, 200);
    await pause(700);
    await nudge(tick);
  }
  await settleTicks(4);
  record("h7_wheel_zoom", await deltaWithPublication(before));
}

// ── 8 — right-drag with alt → orbit ───────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.keyboard.down("Alt");
  await page.mouse.move(centre[0] - 80, centre[1]);
  await page.mouse.down({ button: "right" });
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(centre[0] - 80 + step * 20, centre[1] + step * 6);
    await pause(140);
  }
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Alt");
  await settleTicks(5);
  record("h8_right_drag_orbit", await deltaWithPublication(before));
}

// ── 9 — right-drag with shift → pan ───────────────────────────────────────────────────────────
{
  const before = await snapshotWithPublication();
  await page.keyboard.down("Shift");
  await page.mouse.move(centre[0], centre[1] - 60);
  await page.mouse.down({ button: "right" });
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(centre[0] + step * 14, centre[1] - 60 + step * 10);
    await pause(140);
  }
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Shift");
  await settleTicks(5);
  record("h9_right_drag_pan", await deltaWithPublication(before));
}

await page.screenshot({ path: join(outDir, "shot-final.png"), type: "png" }).catch(() => {});
record("counts", Object.fromEntries(["os_host pointer hit", "world3d interaction surface=", "frame input action", "frame deferred action failed", "world3d surface=", "render begin", "pointer failed", "world3d retained interaction authority faulted", "panicked", "wgpu-worker panicked", "unmapped effect"].map((needle) => [needle, has(needle).length])));

writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
runs.push(results);
allLines.push(...lines.map((line) => `${example || "(none)"} ${line}`));
console.log(`[DEBUG] world3d ${example || "(none)"} hops=${Object.keys(results.steps).length} targets=${JSON.stringify(results.steps.oracle?.expectedTargets ?? [])} roles=${JSON.stringify(results.steps.oracle?.publishedRoles ?? {})}`);
await page.close();
}

writeFileSync(join(outRoot, "results.json"), JSON.stringify({ baseUrl, examples, runs }, null, 2), "utf8");
writeFileSync(join(outRoot, "console.txt"), allLines.join("\n"), "utf8");
console.log(`DONE ${JSON.stringify({ runs: runs.length, out: outRoot })}`);
await browser.close();
