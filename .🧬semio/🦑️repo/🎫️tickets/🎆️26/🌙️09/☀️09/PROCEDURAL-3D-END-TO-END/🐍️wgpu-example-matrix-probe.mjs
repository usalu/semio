/** 🧮️ wgpu EXAMPLE-MATRIX probe — every bundled example of the open dialect, in every boot lane, each
 * one booted DIRECTLY by url rather than through a pointer gesture.
 *
 * The lanes, one page load each (`?plugin=generation3d&example=<id>` plus the lane's own axes):
 *   • `edit`   — edit mode, editor role   (`?mode=edit`)
 *   • `viewer` — the viewer surface       (`?role=viewer`)
 *   • `generate` — generate mode's preview (`?mode=generate`)
 *
 * `?example=` is the axis this lane added (`🧑‍💻dev/🔗️boot-query/🟦️.ts` `resolveBootQueryExampleId` →
 * wgpu `bootDescriptor` → `semioWgpuSetBootExample` → `ShellState::sync_session_chrome` →
 * `apply_boot_example`'s `setActiveExample` dispatch), so no example needs the picker to be clickable —
 * pointer input on 6118 is a different lane's and is deliberately NOT relied on here.
 *
 * CONVERGENCE is the pair `📓️wgpu-resident-budget-settle-2026-09-12.md` §6.1 proved the hexagonal
 * column with, and nothing weaker — all four must hold, stable across two samples:
 *   1. the frame ledger reports `scenePasses > 0` for some window;
 *   2. the renderer's own per-surface World3d census carries `state-meshes > 0` AND a drawable —
 *      `instances > 0` for a solid body, `lines > 0` for a wire one;
 *   3. at least one published mesh carries REAL geometry: a `positions` or `edgePositions` array that
 *      actually starts with a number. generation3d's preview always ships a `…@wire#0` companion whose
 *      `positions` are empty, so "a mesh list that is not `[]`" is not evidence of a rendered body and
 *      was rejected as a predicate; symmetrically `rectangle-wire-preview` is a WIRE example whose only
 *      geometry is `edgePositions` with `instances=0`, so demanding `positions`/`sceneInstances` would
 *      have failed a body that is on screen;
 *   4. no `[role=alert]` is up.
 * The guest's own `facesDone/facesTotal` and `ratio` are recorded beside the verdict. TIME-TO-MESH is
 * measured from `page.goto` to the first sample that answers, so it is comparable with the React journey
 * probe's own per-example seconds.
 *
 * The wgpu tick is INPUT-DRIVEN, so every wait nudges the pointer 1 px rather than sleeping — a bare
 * `waitForTimeout` measures a renderer that was never asked to run.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-examples/matrix bun 🐍️wgpu-example-matrix-probe.mjs
 *   SEMIO_PROBE_LANES=edit,viewer,generate   SEMIO_PROBE_EXAMPLES=<id>,<id>   SEMIO_PROBE_BUDGET=<seconds>
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outRoot = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-examples/matrix");
const budgetSeconds = Number(process.env.SEMIO_PROBE_BUDGET ?? 150);
/** ⏲️ A FLOOR on observation. Convergence answers the first sample that carries a drawable, and the
 * generation3d preview publishes its port-preview WIRES seconds before the guest finishes tessellating
 * the solid — so a run that stops at first convergence reports a census the user has not finished
 * seeing. `timeToMeshSeconds` is still the FIRST convergence (comparable with React's own per-example
 * seconds); the census fields are the LAST sample, so a floor makes them the settled ones. */
const minSeconds = Number(process.env.SEMIO_PROBE_MIN_SECONDS ?? 0);
const viewport = { width: 1440, height: 900 };

/** 📚️ The eight bundled examples of `s.procedural.generation3d@1/*`, by their own `pub const ID`.
 * `(none)` is the ninth lane React's journey probe calls `No example`: the plugin booted with no
 * `?example=` at all, whose preview must carry NOTHING. */
const ALL_EXAMPLES = ["hexagonal-mushroom-column", "rectangle-extrude-volume", "rectangle-wire-preview", "box-shell-preview", "box-fillet-preview", "sphere-cut-with-torus", "sphere-box-fuse", "face-sweep-extrude"];
const NO_EXAMPLE = "(none)";

/** 🛣️ One lane = one url shape. `generate` opens the three-pane generate layout, whose preview window
 * is the surface `addGeneration` publishes into. */
const axis = (example) => (example === NO_EXAMPLE ? "" : `&example=${encodeURIComponent(example)}`);
const LANES = {
  edit: (example) => `${origin}/?plugin=generation3d&mode=edit${axis(example)}`,
  viewer: (example) => `${origin}/?plugin=generation3d&role=viewer${axis(example)}`,
  generate: (example) => `${origin}/?plugin=generation3d&mode=generate${axis(example)}`,
};

/** 📚️ Where the committed per-example oracle lives — the SAME `🔣️.json` the native `example-geometry`
 * lane asserts against, read here so a runtime census and a native one are two readings of one
 * statement instead of two independently invented numbers. Directory names carry an emoji prefix, so
 * the slug is matched against the tail of each entry. */
const EXAMPLES_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "✏️s", "🔌️plugins", "🌀️procedural", "🗿️artifacts", "🧊️generation3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "📚️examples");

const fixtureFor = (example) => {
  try {
    const dir = readdirSync(EXAMPLES_ROOT).find((entry) => entry.endsWith(example));
    if (!dir) return null;
    return JSON.parse(readFileSync(join(EXAMPLES_ROOT, dir, "🧫️fixtures", "🧩️example", "🔣️.json"), "utf8"));
  } catch {
    return null;
  }
};

/** 📐️ How far a LIVE preview's bounding box may sit from the committed one. The fixture's own
 * `boundingBoxTolerance` is measured at its `tessellationTolerance`; the playground asks the kernel
 * for the LOD deflection instead (`preview_eval::preview_tolerance`, 0.05 for the unset lod mode),
 * and a tessellated hull is inscribed in the true surface by at most that chord deviation. So the
 * runtime band is the looser of the two — stated, not fudged. */
const PREVIEW_LOD_TOLERANCE = { coarse: 0.15, fine: 0.02 };
const bboxTolerance = (fixture) => Math.max(fixture.expect.boundingBoxTolerance, PREVIEW_LOD_TOLERANCE[fixture.delivery.lodMode] ?? 0.05);

/** 🌍️ The surface a preview lane publishes into — the one `dumpMeshStats` walked that carries a
 * `preview` in its id. A lane with several World3d surfaces (the generate layout) still has exactly
 * one the example's geometry lands in, so the richest publication wins the tie. */
const previewSurface = (published) => {
  const surfaces = (published?.surfaces ?? []).filter((surface) => surface.surfaceId.includes("preview"));
  if (surfaces.length === 0) return null;
  return surfaces.reduce((best, surface) => ((surface.meshes ?? []).length > (best.meshes ?? []).length ? surface : best), surfaces[0]);
};

/** ✅️ Holds ONE lane's live publication to the committed fixture, role by role and axis by axis.
 *
 * ⚖️ Three separate readings used to be conflated. `state-meshes` in the surface census counts the
 * RENDERER's mesh store (face overlay and placeholder included) — which is why every non-hex example
 * read 2-3 against an oracle of 1 and nobody could say whether that was a companion wire or a
 * duplication. `dumpMeshStats` answers the PRODUCER's own `meshes_json`, one row per published mesh
 * with the role the payload stamps, which is exactly what `delivery.meshes`/`delivery.meshRoles`
 * state (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
 *
 * 📐️ The box is taken over the meshes whose role IS the fixture's own `preview.kind`, never over the
 * union: a preview publishes the wire it extruded and the vector that drove it beside its solid, and
 * the committed box is the preview target's.
 */
const publicationVerdict = (example, published) => {
  const fixture = fixtureFor(example);
  if (!fixture) return null;
  const surface = previewSurface(published);
  const failures = [];
  /** 🩺️ Two different absences a reader must be able to tell apart: a renderer whose wasm predates
   * `dumpMeshStats` answers nothing at all, while a live renderer with a blank preview answers a
   * surface with no meshes. */
  if (published === null) failures.push("the served renderer exposes no dumpMeshStats export — rebuild @semio-tech/framework-renderer-wgpu:wasm and reload");
  else if (!surface) failures.push("no world3d preview surface published a mesh payload");
  const meshes = surface?.meshes ?? [];
  const roles = surface?.roles ?? {};
  const expectedRoles = fixture.delivery.meshRoles;
  if (surface) {
    if (meshes.length !== fixture.delivery.meshes) failures.push(`published ${meshes.length} meshes, the fixture delivers exactly ${fixture.delivery.meshes}`);
    if (roles["(unstamped)"]) failures.push(`${roles["(unstamped)"]} published meshes carry no role stamp — the staged guest predates preview_mesh_role`);
    for (const role of new Set([...Object.keys(expectedRoles), ...Object.keys(roles)])) {
      if ((roles[role] ?? 0) !== (expectedRoles[role] ?? 0)) failures.push(`role ${role}: published ${roles[role] ?? 0}, the fixture delivers ${expectedRoles[role] ?? 0}`);
    }
  }
  const targets = meshes.filter((mesh) => mesh.role === fixture.preview.kind && mesh.bboxMin);
  let box = null;
  if (surface && targets.length === 0) failures.push(`no published mesh carries the fixture's own preview role ${fixture.preview.kind} with bounds`);
  if (targets.length > 0) {
    box = {
      min: [0, 1, 2].map((axis) => Math.min(...targets.map((mesh) => mesh.bboxMin[axis]))),
      max: [0, 1, 2].map((axis) => Math.max(...targets.map((mesh) => mesh.bboxMax[axis]))),
    };
    const tolerance = bboxTolerance(fixture);
    for (let axis = 0; axis < 3; axis += 1) {
      if (Math.abs(box.min[axis] - fixture.expect.boundingBoxMin[axis]) > tolerance) failures.push(`bboxMin[${axis}] ${box.min[axis]} vs committed ${fixture.expect.boundingBoxMin[axis]} (tolerance ${tolerance})`);
      if (Math.abs(box.max[axis] - fixture.expect.boundingBoxMax[axis]) > tolerance) failures.push(`bboxMax[${axis}] ${box.max[axis]} vs committed ${fixture.expect.boundingBoxMax[axis]} (tolerance ${tolerance})`);
    }
  }
  return {
    ok: failures.length === 0,
    failures,
    expected: { meshes: fixture.delivery.meshes, meshRoles: expectedRoles, boundingBoxMin: fixture.expect.boundingBoxMin, boundingBoxMax: fixture.expect.boundingBoxMax, tolerance: bboxTolerance(fixture), previewKind: fixture.preview.kind },
    observed: { surfaceId: surface?.surfaceId ?? null, rect: surface?.rect ?? null, meshes: meshes.length, roles, box, ids: meshes.map((mesh) => mesh.id), instances: (surface?.instances ?? []).map((instance) => instance.interactionId), selected: surface?.selected ?? [], hovered: surface?.hovered ?? null },
  };
};

const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? ALL_EXAMPLES.join(",")).split(",").filter(Boolean);
const lanes = (process.env.SEMIO_PROBE_LANES ?? "edit,viewer,generate").split(",").filter(Boolean);
mkdirSync(outRoot, { recursive: true });

const results = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });

for (const lane of lanes) {
  for (const example of examples) {
    const outDir = join(outRoot, example, lane);
    mkdirSync(outDir, { recursive: true });
    const lines = [];
    const t0 = Date.now();
    const at = () => Date.now() - t0;
    const page = await browser.newPage({ viewport });
    page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
    page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1200)}`));

    const has = (needle) => lines.filter((line) => line.includes(needle));

    /** 🌍️ The renderer's own per-surface World3d census (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`): how many
     * instances and meshes the scene actually holds, and the guest's own published status. */
    const world3dTraces = () => {
      const byId = {};
      for (const line of has("world3d surface=")) {
        const surface = /world3d surface=(\S+)/.exec(line)?.[1];
        if (!surface) continue;
        byId[surface] = {
          instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0),
          draws: Number(/ draws=(\d+)/.exec(line)?.[1] ?? 0),
          lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0),
          stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0),
          meshesHead: /meshesHead="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
          status: /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1] ?? null,
        };
      }
      return byId;
    };

    /** 📐️ A mesh with an EMPTY `positions` array is a published entry, not a rendered body — the
     * generation3d preview always carries a `@wire` companion whose arrays are empty. Convergence
     * therefore demands at least one mesh whose positions array actually starts with a number. */
    const geometryEvidence = () => {
      for (const line of has("world3d surface=")) {
        const solid = /\\"positions\\":\[-?\d/.test(line);
        const wire = /\\"edgePositions\\":\[-?\d/.test(line);
        if (!solid && !wire) continue;
        const head = line.indexOf("meshesHead");
        return { body: solid ? "solid" : "wire", head: head < 0 ? null : line.slice(head, head + 180) };
      }
      return null;
    };

    /** 🧮️ The guest's OWN convergence, as it publishes it: `facesDone/facesTotal` and `ratio`. */
    const guestProgress = () => {
      const lines2 = has("facesDone");
      const last = lines2.at(-1);
      if (!last) return null;
      const done = Number(/facesDone\\":(\d+)/.exec(last)?.[1] ?? -1);
      const total = Number(/facesTotal\\":(\d+)/.exec(last)?.[1] ?? -1);
      const ratio = Number(/ratio\\":([\d.]+)/.exec(last)?.[1] ?? -1);
      return { facesDone: done, facesTotal: total, ratio };
    };

    const dump = (windowId) =>
      page
        .evaluate(async (id) => {
          const beacon = globalThis.semioWgpuIntrospection;
          if (typeof beacon?.dumpStructure !== "function") return null;
          const parse = async (call) => {
            try {
              const raw = await call();
              return raw ? JSON.parse(raw) : null;
            } catch {
              return null;
            }
          };
          return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)), meshStats: typeof beacon.dumpMeshStats === "function" ? await parse(() => beacon.dumpMeshStats(id)) : null };
        }, windowId)
        .catch(() => null);

    const domSignals = () =>
      page
        .evaluate(() => ({
          status: document.querySelector('[role="status"]')?.textContent?.trim() ?? null,
          alert: document.querySelector('[role="alert"]')?.textContent?.trim()?.slice(0, 300) ?? null,
        }))
        .catch(() => ({ status: null, alert: null }));

    const measure = async () => {
      const ids = (await dump(undefined))?.structure?.windowIds ?? [];
      let sceneInstances = 0;
      let sceneDraws = 0;
      let scenePasses = 0;
      let quadCount = 0;
      for (const id of ids) {
        const stats = (await dump(id))?.stats ?? null;
        sceneInstances = Math.max(sceneInstances, stats?.sceneInstances ?? 0);
        sceneDraws = Math.max(sceneDraws, stats?.sceneDraws ?? 0);
        scenePasses = Math.max(scenePasses, stats?.scenePasses ?? 0);
        quadCount = Math.max(quadCount, stats?.quadCount ?? 0);
      }
      const dom = await domSignals();
      const published = (await dump(undefined))?.meshStats ?? null;
      const traces = world3dTraces();
      const geometry = geometryEvidence();
      const progress = guestProgress();
      const meshy = Object.entries(traces).filter(([, trace]) => trace.stateMeshes > 0 && (trace.instances > 0 || trace.lines > 0));
      return {
        windowIds: ids,
        sceneInstances,
        sceneDraws,
        scenePasses,
        quadCount,
        dom,
        progress,
        geometry,
        meshSurfaces: meshy.map(([id, trace]) => ({ surface: id, instances: trace.instances, lines: trace.lines, meshes: trace.stateMeshes, draws: trace.draws })),
        published,
        converged: scenePasses > 0 && meshy.length > 0 && geometry !== null && !dom.alert,
      };
    };

    const url = LANES[lane](example);
    await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 300)}`));

    let sample = { windowIds: [], sceneInstances: 0, sceneDraws: 0, scenePasses: 0, quadCount: 0, dom: { status: null, alert: null }, progress: null, geometry: null, meshSurfaces: [], published: null, converged: false };
    let stable = 0;
    let firstConvergedMs = null;
    let flip = 0;
    for (let elapsed = 0; elapsed < budgetSeconds && (stable < 2 || elapsed < minSeconds); elapsed += 1) {
      for (let nudge = 0; nudge < 5; nudge += 1) {
        await page.waitForTimeout(200);
        flip = 1 - flip;
        await page.mouse.move(3 + flip, 3).catch(() => {});
      }
      sample = await measure();
      if (sample.converged) {
        firstConvergedMs ??= at();
        stable += 1;
      } else stable = 0;
    }

    const bootExample = has("wgpu-shell boot example").at(-1) ?? null;
    const oracle = publicationVerdict(example, sample.published);
    const row = {
      oracle,
      example,
      lane,
      url,
      verdict: sample.converged ? "pass" : "fail",
      timeToMeshSeconds: firstConvergedMs === null ? null : Math.round(firstConvergedMs / 10) / 100,
      elapsedSeconds: Math.round(at() / 1000),
      windowIds: sample.windowIds,
      sceneInstances: sample.sceneInstances,
      sceneDraws: sample.sceneDraws,
      scenePasses: sample.scenePasses,
      quadCount: sample.quadCount,
      guestProgress: sample.progress,
      geometry: sample.geometry,
      meshSurfaces: sample.meshSurfaces,
      status: sample.dom.status,
      alert: sample.dom.alert,
      bootExampleTrace: bootExample ? bootExample.slice(bootExample.indexOf("[DEBUG]")) : null,
      renderBegin: has("wgpu-shell render begin").length,
      renderLeave: has("wgpu-shell render leave").length,
      capacity: has("Capacity").length,
      faults: has("invokeExtension faulted").length,
    };
    results.push(row);
    console.log(`[DEBUG] ${lane}/${example}: ${row.verdict} oracle=${oracle === null ? "n/a" : oracle.ok} ${oracle === null ? "" : JSON.stringify(oracle.failures)} published=${JSON.stringify(oracle?.observed ?? null)} t=${row.timeToMeshSeconds}s meshes=${JSON.stringify(row.meshSurfaces)} progress=${JSON.stringify(row.guestProgress)} quads=${row.quadCount} status=${JSON.stringify(row.status)} alert=${JSON.stringify(row.alert)} bootExample=${JSON.stringify(row.bootExampleTrace)}`);
    await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
    writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
    writeFileSync(join(outDir, "result.json"), JSON.stringify(row, null, 2));
    await page.close();
  }
}

writeFileSync(join(outRoot, "results.json"), JSON.stringify({ origin, lanes, examples, results }, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ pass: results.filter((row) => row.verdict === "pass").length, total: results.length, out: outRoot }, null, 2));
