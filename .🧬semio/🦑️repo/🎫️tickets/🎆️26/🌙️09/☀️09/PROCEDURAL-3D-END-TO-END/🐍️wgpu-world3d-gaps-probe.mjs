/** 🕳️ wgpu WORLD3D GAPS — the three reds `📓️wgpu-end-to-end-verification-2026-09-14.md` §5 left open,
 * each isolated to the ONE reading that decides it, so none of them needs a 16-minute battery lane.
 *
 *   §5.1 the wheel does not zoom → `[DEBUG] wheel gate …` names the POINT the frame applies the
 *        coalesced wheel at, next to the world3d surface bounds, so "the gate refused" is told apart
 *        from "the gate passed at the wrong point".
 *   §5.2 the VIEWER mints `translateSelection` → the gumball drag is replayed in the role under test
 *        and the published verbs are read next to `frame deferred action failed`.
 *   §5.3 `world3d scene mesh-wire bridge faulted` → `SEMIO_PROBE_MODE=generate` adds the Form lane:
 *        `Add Generation`, then N world3d gestures interleaved with M slider DRAGS, each of which
 *        republishes the whole mesh wire under the gestures. `[DEBUG] world3d bridge fault …` carries
 *        the surface census (`fault=…`, `state-meshes`, `state-draws`) at the instant it quarantines.
 *
 * Every rect comes from the two published sources only — a node's own `dumpStructure` rect plus its
 * window origin in the shell's `dock plan` trace. Never a guessed pixel.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_ROLE=viewer bun 🐍️wgpu-world3d-gaps-probe.mjs
 *        cd <ticket> && SEMIO_PROBE_MODE=generate SEMIO_PROBE_GESTURES=20 SEMIO_PROBE_EDITS=3 bun 🐍️wgpu-world3d-gaps-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const role = process.env.SEMIO_PROBE_ROLE ?? "";
const mode = process.env.SEMIO_PROBE_MODE ?? "";
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const url = process.env.SEMIO_PROBE_URL ?? `${origin}/?plugin=generation3d&example=${example}${role ? `&role=${role}` : ""}${mode ? `&mode=${mode}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 150);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const gestures = Number(process.env.SEMIO_PROBE_GESTURES ?? 0);
const edits = Number(process.env.SEMIO_PROBE_EDITS ?? 0);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-world3d-gaps/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const nudge = (n) => page.mouse.move(3 + (n % 5), 3 + (n % 5)).catch(() => {});
const pause = (ms) => page.waitForTimeout(ms);
const settleTicks = async (count) => {
  for (let tick = 0; tick < count; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
};
const pump = async (seconds) => {
  for (let tick = 0; tick < seconds * 5; tick += 1) {
    await pause(200);
    await page.mouse.move(3 + (tick % 2), 3).catch(() => {});
  }
};

const cameraTrace = () => /camera="(\{.*?\})"/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;
const publishedActions = () => has("frame input action").map((line) => /controller=(\S+) action=(\S+) args=(\S+)/.exec(line)).filter(Boolean).map((m) => m[2]);
const dispatchFailures = () => has("frame deferred action failed").map((line) => line.split("frame deferred action failed: ").at(-1).slice(0, 240));
const wheelGates = () => has("wheel gate").map((line) => line.split("[DEBUG] ").at(-1));
/** 🖱️ The `DispatchEvent::Scroll` the Worker actually saw, with the point it carried. */
const scrollEvents = () => has("dispatch_normalized_event Scroll").map((line) => line.split("dispatch_normalized_event ").at(-1));
/** 🎥️ The LOCAL orbit rig, as the world authority's own census reports it — moves before the guest's. */
const localCamera = () => /camera=(\[[^\]]*\]->\[[^\]]*\]\/[\d.]+deg)/.exec(has("world3d interaction surface=").at(-1) ?? "")?.[1] ?? null;
const bridgeFaults = () => has("world3d bridge fault").map((line) => line.split("[DEBUG] ").at(-1));
const frameFaults = () => has("frame fault recorded").map((line) => line.split("frame fault recorded: ").at(-1));
const quarantines = () => has("quarantined=true").length + has("surface quarantined").length;

const dockPlan = () => {
  const line = has("wgpu-shell dock plan").at(-1);
  const plan = {};
  if (!line) return plan;
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

const dumpAll = () =>
  page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const root = JSON.parse(await beacon.dumpStructure());
      const per = {};
      for (const id of root.windowIds ?? []) {
        try {
          per[id] = JSON.parse(await beacon.dumpStructure(id));
        } catch {
          per[id] = null;
        }
      }
      return { windowIds: root.windowIds ?? [], per };
    })
    .catch(() => null);

/** 🎯️ One published node's page rect, from its own rect plus its window's dock origin. */
const locate = (snapshot, plan, needle) => {
  for (const id of snapshot?.windowIds ?? []) {
    const body = plan[id];
    if (!body) continue;
    for (const node of snapshot.per[id]?.nodes ?? []) {
      const path = String(node.path ?? "");
      if (!path.includes(needle)) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      return { windowId: id, path, kind: node.kind ?? null, text: node.text ?? null, rect: [body.x + rx, body.y + ry, rw, rh], page: [body.x + rx + rw / 2, body.y + ry + rh / 2] };
    }
  }
  return null;
};

/** 🎯️ Jiggles at one point until the shell's own `pointer hit` trace resolves `needle` there — the
 * retained hit registry is drained one entry per frame-build boundary step, so a blind press lands
 * on the empty half about half the time (`📓️wgpu-deferred-action-commit-2026-09-13.md` §5). */
const armHit = async (x, y, needle, tries = 60) => {
  for (let attempt = 0; attempt < tries; attempt += 1) {
    await page.mouse.move(x + (attempt % 2 === 0 ? -0.25 : 0.25), y);
    await pause(110);
    const last = has("os_host pointer hit").at(-1) ?? "";
    if (last.includes("hit=Some(") && last.includes(needle)) return true;
  }
  return false;
};

/** 🎚️ The Nth live slider of the generate Form, by its own published path — the Form carries no
 * controls at all until a generation exists, so this is resolved AFTER `Add Generation`. */
const formSlider = (snapshot, plan, ordinal) => {
  const windowId = (snapshot?.windowIds ?? []).find((id) => id.endsWith("generate-form")) ?? null;
  const body = windowId ? plan[windowId] : null;
  if (!body) return null;
  const sliders = (snapshot.per[windowId]?.nodes ?? []).filter((node) => String(node.path ?? "").includes("slider[") && (node.rect?.[2] ?? 0) > 0 && (node.rect?.[3] ?? 0) > 0);
  const node = sliders[ordinal % Math.max(1, sliders.length)];
  if (!node) return null;
  const [rx, ry, rw, rh] = node.rect;
  return { windowId, path: String(node.path), kind: node.kind ?? null, rect: [body.x + rx, body.y + ry, rw, rh], page: [body.x + rx + rw / 2, body.y + ry + rh / 2], count: sliders.length };
};

const press = async (needle, settleSeconds = 8) => {
  await pump(2);
  const target = locate(await dumpAll(), dockPlan(), needle);
  if (!target) return null;
  await page.mouse.move(target.page[0], target.page[1]);
  await pause(300);
  await page.mouse.click(target.page[0], target.page[1]);
  await pump(settleSeconds);
  return target;
};

const results = { url, role: role || "editor", mode: mode || "edit", steps: {} };
const record = (name, value) => {
  results.steps[name] = value;
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(value).slice(0, 6000)}`);
};

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
await settleTicks(settle);

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? null;
const body = previewId ? plan[previewId] : null;
if (!body) {
  record("fatal", { reason: "no preview window in the dock plan", plan });
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
  await browser.close();
  process.exit(1);
}
const centre = [body.x + body.w / 2, body.y + body.h / 2];
record("preview", { previewId, body, centre, plan });
{
  const snapshot = await dumpAll();
  record("structure", { windowIds: snapshot?.windowIds ?? null, paths: Object.fromEntries(Object.entries(snapshot?.per ?? {}).map(([id, doc]) => [id, (doc?.nodes ?? []).map((node) => node.path).filter(Boolean).slice(0, 200)])) });
}

// ── §5.1 the wheel ────────────────────────────────────────────────────────────────────────────
{
  // 🎯️ The battery's own `h7_wheel_zoom` shape: the wheel is always scrolled AT the preview centre,
  // and the pointer is nudged to the corner between notches — which is what makes the input-driven
  // tick run, and what a slow (edit) session drains the wheel AFTER.
  const before = { camera: cameraTrace(), local: localCamera(), gates: wheelGates().length, actions: publishedActions().length };
  for (let tick = 0; tick < 5; tick += 1) {
    await page.mouse.move(centre[0], centre[1]);
    await pause(500);
    await page.mouse.wheel(0, 200);
    await pause(700);
    await nudge(tick);
  }
  await settleTicks(8);
  record("wheel", {
    scrolledAt: centre,
    wheelEvents: scrollEvents().slice(-6),
    localCameraBefore: before.local,
    localCameraAfter: localCamera(),
    localCameraChanged: before.local !== null && before.local !== localCamera(),
    cameraBefore: before.camera,
    cameraAfter: cameraTrace(),
    guestCameraChanged: before.camera !== cameraTrace(),
    gates: wheelGates().slice(before.gates).slice(0, 12),
    newActions: publishedActions().slice(before.actions),
  });
}

// ── §5.2 the gumball ──────────────────────────────────────────────────────────────────────────
{
  await page.mouse.move(centre[0], centre[1]);
  await settleTicks(2);
  await page.mouse.click(centre[0], centre[1]);
  await settleTicks(5);
  const before = { actions: publishedActions().length, failures: dispatchFailures().length };
  await page.keyboard.down("Shift");
  await page.mouse.move(centre[0], centre[1]);
  await page.mouse.down();
  for (let step = 1; step <= 6; step += 1) {
    await page.mouse.move(centre[0] + step * 9, centre[1] - step * 3);
    await pause(120);
  }
  await page.mouse.up();
  await page.keyboard.up("Shift");
  await settleTicks(8);
  record("gumball", { newActions: publishedActions().slice(before.actions), newFailures: dispatchFailures().slice(before.failures) });
}

// ── §5.3 N gestures interleaved with M Form slider DRAGS ──────────────────────────────────────
if (gestures > 0 || edits > 0) {
  const added = await press("procedural3d-play-generate.add-generation", 12);
  record("add-generation", { pressed: Boolean(added), path: added?.path ?? null });
  await pump(10);
  const livePlan = dockPlan();
  const liveId = Object.keys(livePlan).find((id) => id.includes("preview")) ?? previewId;
  const liveBody = livePlan[liveId] ?? body;
  const point = [liveBody.x + liveBody.w / 2, liveBody.y + liveBody.h / 2];
  const editEvery = edits > 0 ? Math.max(1, Math.floor(gestures / (edits + 1))) : gestures + 1;
  let done = 0;
  const timeline = [];
  for (let gesture = 1; gesture <= gestures; gesture += 1) {
    const dx = ((gesture % 5) - 2) * 18;
    const dy = ((gesture % 3) - 1) * 22;
    const kind = ["hover", "click", "shift-click", "empty-click", "marquee"][gesture % 5];
    await page.mouse.move(point[0] + dx, point[1] + dy);
    if (kind === "click") await page.mouse.click(point[0] + dx, point[1] + dy);
    if (kind === "shift-click") {
      await page.keyboard.down("Shift");
      await page.mouse.click(point[0] + dx, point[1] + dy);
      await page.keyboard.up("Shift");
    }
    if (kind === "empty-click") await page.mouse.click(liveBody.x + 8, liveBody.y + liveBody.h - 8);
    if (kind === "marquee") {
      await page.mouse.move(point[0] - 60, point[1] - 60);
      await page.mouse.down();
      await page.mouse.move(point[0] + 60, point[1] + 60, { steps: 6 });
      await page.mouse.up();
    }
    await settleTicks(4);
    timeline.push({ gesture, kind, frameFaults: frameFaults().length, bridgeFaults: bridgeFaults().length });
    if (gesture % editEvery === 0 && done < edits) {
      done += 1;
      const slider = formSlider(await dumpAll(), dockPlan(), done - 1);
      let seated = 0;
      const seatedBase = has("retained press").filter((line) => line.includes("kind=Slider") && line.includes("down=true")).length;
      if (slider) {
        const [x, y, w, h] = slider.rect;
        const from = [x + w * (0.2 + done * 0.12), y + h / 2];
        const to = [x + w * (0.45 + done * 0.15), y + h / 2];
        const needle = slider.path.split("#").at(-1);
        for (let attempt = 0; attempt < 4 && seated === 0; attempt += 1) {
          await armHit(from[0], from[1], needle);
          await page.mouse.move(from[0], from[1]);
          await page.mouse.down();
          for (let step = 1; step <= 6; step += 1) {
            await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 6, to[1]);
            await pause(90);
          }
          await page.mouse.up();
          await pump(6);
          seated = has("retained press").filter((line) => line.includes("kind=Slider") && line.includes("down=true")).length - seatedBase;
        }
      }
      await pump(12);
      timeline.push({ edit: done, sliderPath: slider?.path ?? null, sliders: slider?.count ?? 0, seated, frameFaults: frameFaults().length, bridgeFaults: bridgeFaults().length });
      record(`edit-${done}`, { sliderPath: slider?.path ?? null, seated, frameFaults: frameFaults(), bridgeFaults: bridgeFaults().slice(-4) });
    }
  }
  record("gestures", { gestures, edits: done, timeline });
}

record("faults", { frameFaults: frameFaults(), bridgeFaults: bridgeFaults().slice(-12), dispatchFailures: dispatchFailures().slice(-12), pageErrors: has("pageerror").length, quarantines: quarantines() });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
await browser.close();
console.log(JSON.stringify(results.steps, null, 2).slice(0, 14000));
