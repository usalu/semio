/** 🎯 Runtime proof that the preview FRAMES the example it just delivered, without the user pressing
 * anything — the defect `📓️coordinator-walk-2026-09-14.md` rows 20 and 21 recorded: every example
 * converged on the seed pose `{position:[4,-4,3], target:[0,0,0], fov:45}`, the box-fillet solid cut
 * off at the top-right edge, and even `Frame visible` left the camera where it was.
 *
 * Every verdict is a PROJECTION of the example's committed delivery box through the pose the pane
 * published (`gradeCameraFrames`, `🐍️example-oracle.mjs`): all eight corners inside the viewport, in
 * front of the eye, and filling enough of it to be worth looking at. Nothing here reproduces the
 * host's fit arithmetic, so a change to that arithmetic cannot make this probe agree with it.
 *
 * Per example, in EDIT and VIEWER role:
 *   `boot-frame`   — opened straight at `?example=<slug>` in a fresh page, the converged camera frames
 *                    the committed `delivery.boundingBox*`. No click, no keystroke.
 *   `switch-frame` — the same, reached by picking the example from the navbar picker instead.
 *   `user-camera`  — after the user orbits, a re-evaluation of the SAME example (a slider nudge on the
 *                    graph) leaves that camera exactly where the user put it.
 *   `frame-button` — `Frame visible` still frames, and still exists, beside the automatic framing.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6028/?plugin=generation3d SEMIO_PROBE_OUT=boot-frame bun 🐍️boot-camera-frame-probe.mjs
 * Env: SEMIO_PROBE_EXAMPLES (comma separated slugs/labels), SEMIO_PROBE_MESH_WAIT (seconds per example),
 *      SEMIO_PROBE_ROLES (`edit,view`), SEMIO_PROBE_SHOTS=0 to skip screenshots.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ORACLES, gradeCameraFrames, gradeMeshes, meshStatsScript } from "./🐍️example-oracle.mjs";

const base = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6028/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "boot-frame");
const meshWait = Number(process.env.SEMIO_PROBE_MESH_WAIT ?? 90);
const shots = process.env.SEMIO_PROBE_SHOTS !== "0";
const roles = (process.env.SEMIO_PROBE_ROLES ?? "edit,view").split(",").map((role) => role.trim()).filter(Boolean);
const wanted = (process.env.SEMIO_PROBE_EXAMPLES ?? "").split(",").map((s) => s.trim()).filter(Boolean);
const oracles = Object.values(ORACLES).filter((oracle) => wanted.length === 0 || wanted.includes(oracle.label) || wanted.includes(oracle.slug));
mkdirSync(outDir, { recursive: true });

const lines = [];
const faults = [];
const results = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });

const record = (example, role, step, ok, detail) => {
  results.push({ example, role, step, ok, t: Date.now() - t0, detail });
  console.log(`[DEBUG] ${role}:${example} ${step}: ok=${ok} ${JSON.stringify(detail).slice(0, 460)}`);
};

/** 📦️ The box a boot framing owes the user: the example's committed DELIVERY extent, which
 * `assert_delivery_bounds` holds the guest's own payload to natively. */
const deliveryBox = (oracle) => ({ min: oracle.deliveryBoundingBoxMin, max: oracle.deliveryBoundingBoxMax });

const openPage = async () => {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
  page.on("pageerror", (e) => {
    faults.push({ t: Date.now() - t0, message: String(e?.message ?? e).slice(0, 300) });
    lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`);
  });
  return page;
};

const paneSnap = async (page, previewMeshId) => {
  const stats = await page.evaluate(meshStatsScript, previewMeshId);
  const dom = await page.evaluate(() => {
    const parse = (raw) => { try { return JSON.parse(raw ?? "null"); } catch { return null; } };
    const panes = [...document.querySelectorAll("[data-meshes-json], [data-status-json]")].map((el) => {
      const status = parse(el.getAttribute("data-status-json"));
      return {
        camera: parse(el.getAttribute("data-viewport-camera-json")),
        sceneCamera: parse(el.getAttribute("data-camera-json")),
        phase: status?.phase ?? null,
        ratio: status?.progress?.ratio ?? null,
        fault: status?.fault?.code ?? null,
      };
    });
    const combo = document.querySelector('[role="combobox"]');
    return { panes, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
  });
  return { ...dom, panes: dom.panes.map((pane, index) => ({ ...(stats[index] ?? {}), ...pane })) };
};

/** 🪟️ The preview surface of the role under test. Both are mounted at once after a role switch, so a
 * bare `endsWith("-preview")` reads whichever the previous mode left behind — the same trap
 * `📓️react-oracle-hardening-2026-09-14.md` §2 fixed in the journey probe with `expectSurface`. */
const previewPane = (snap, role) => {
  const want = role === "view" ? "procedural-view-preview" : "procedural-preview";
  return snap.panes.find((pane) => pane.surfaceId && pane.surfaceId.endsWith(want)) ?? null;
};
const settled = (pane) => Boolean(pane) && pane.phase === "idle" && pane.ratio === 1 && !pane.fault;

const canvasBox = async (page) => {
  const canvas = page.locator('[data-meshes-json] canvas, .semio-world-3d-host canvas').last();
  if (!(await canvas.count())) return null;
  return canvas.boundingBox();
};

const waitConverged = async (page, oracle, role) => {
  let last = null;
  for (let i = 0; i < meshWait; i += 1) {
    await page.waitForTimeout(1000);
    last = await paneSnap(page, oracle.previewMeshId);
    const pane = previewPane(last, role);
    if (settled(pane) && gradeMeshes(oracle, pane).ok) return last;
  }
  return last;
};

/** 👁️ The viewer role is the `Meta+Alt+V` chord, the way `🐍️journey-probe.mjs` reaches it — the navbar
 * control was measured dead on 2026-09-14 (`📓️coordinator-walk-2026-09-14.md` row 19), which is another
 * lane's defect a boot-framing law must not be hostage to. The viewer preview's OWN first delivery is
 * what this grades, so the law is about the viewer's camera, not the editor's leftover. */
const enterRole = async (page, role) => {
  if (role !== "view") return;
  await page.keyboard.press("Meta+Alt+V");
  await page.waitForTimeout(4000);
};

for (const role of roles) {
  for (const oracle of oracles) {
    //#region 🚀️BootFrame
    const page = await openPage();
    await page.goto(`${base}&example=${oracle.slug}`, { waitUntil: "domcontentloaded" });
    if (role === "view") {
      await waitConverged(page, oracle, "edit");
      await enterRole(page, role);
    }
    const booted = await waitConverged(page, oracle, role);
    const bootPane = previewPane(booted, role);
    const box = await canvasBox(page);
    const aspect = box ? box.width / box.height : 1;
    const bootGrade = gradeCameraFrames(bootPane?.camera ?? null, deliveryBox(oracle), aspect);
    record(oracle.label, role, "boot-frame", Boolean(bootPane) && settled(bootPane) && bootGrade.ok, {
      converged: settled(bootPane), meshes: bootPane?.meshCount ?? 0, camera: bootPane?.camera ?? null,
      box: deliveryBox(oracle), worstX: bootGrade.worstX, worstY: bootGrade.worstY, behind: bootGrade.behind, aspect, reasons: bootGrade.reasons,
    });
    if (shots) await page.screenshot({ path: join(outDir, `${role}-${oracle.slug}-boot.png`) });
    //#endregion 🚀️BootFrame

    //#region 🔒️UserCameraLatch
    // 🖱️ Alt + right drag is the product's orbit binding. After it, a re-evaluation of the SAME example
    // (nudging a slider on the flow graph via the keyboard) must leave the pose the user chose.
    let latch = { skipped: !box };
    if (box) {
      const beforeOrbit = bootPane?.camera ?? null;
      const distance = beforeOrbit ? Math.hypot(...beforeOrbit.position.map((v, axis) => v - beforeOrbit.target[axis])) : 1;
      const polar = beforeOrbit ? Math.acos(Math.min(1, Math.max(-1, (beforeOrbit.position[2] - beforeOrbit.target[2]) / (distance || 1)))) : Math.PI / 2;
      await page.keyboard.down("Alt");
      await page.mouse.move(box.x + box.width * 0.6, box.y + box.height * 0.45);
      await page.mouse.down({ button: "right" });
      await page.mouse.move(box.x + box.width * 0.6 + 120, box.y + box.height * 0.45 + (polar < Math.PI / 4 ? -45 : 45), { steps: 14 });
      await page.mouse.up({ button: "right" });
      await page.keyboard.up("Alt");
      await page.waitForTimeout(3000);
      const moved = previewPane(await paneSnap(page, oracle.previewMeshId), role)?.camera ?? null;
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.wheel(0, -180);
      await page.waitForTimeout(12000);
      const after = previewPane(await paneSnap(page, oracle.previewMeshId), role)?.camera ?? null;
      const gestureMoved = Boolean(beforeOrbit && moved) && JSON.stringify(beforeOrbit.position) !== JSON.stringify(moved.position);
      const kept = Boolean(moved && after) && Math.hypot(...after.target.map((v, axis) => v - moved.target[axis])) < 1e-3;
      latch = { gestureMoved, kept, before: beforeOrbit, moved, after };
      record(oracle.label, role, "user-camera", gestureMoved && kept, latch);
    } else {
      record(oracle.label, role, "user-camera", false, { error: "no preview canvas" });
    }
    //#endregion 🔒️UserCameraLatch

    //#region 🎯️FrameButton
    const fitButton = page.locator('[data-slot="world-frame-instances"]').last();
    const fitPresent = (await fitButton.count()) > 0;
    let fitClickError = null;
    if (fitPresent) await fitButton.click({ timeout: 6000 }).catch((error) => { fitClickError = String(error).split("\n")[0].slice(0, 160); });
    await page.waitForTimeout(3000);
    const refit = previewPane(await paneSnap(page, oracle.previewMeshId), role);
    const refitGrade = gradeCameraFrames(refit?.camera ?? null, deliveryBox(oracle), aspect);
    record(oracle.label, role, "frame-button", fitPresent && !fitClickError && refitGrade.ok, {
      fitPresent, fitClickError, camera: refit?.camera ?? null, cameraBeforeClick: latch.after ?? null,
      worstX: refitGrade.worstX, worstY: refitGrade.worstY, reasons: refitGrade.reasons,
    });
    if (shots) await page.screenshot({ path: join(outDir, `${role}-${oracle.slug}-after-frame-button.png`) });
    //#endregion 🎯️FrameButton
    await page.close();
  }
}

//#region 🔀️SwitchFrame
// 🔀️ The other way in: one page, the picker walked across every example. A framing that only works on
// the `?example=` boot is not the framing a user who switches examples gets.
if (process.env.SEMIO_PROBE_SWITCH !== "0") {
  const page = await openPage();
  await page.goto(base, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(20_000);
  for (const oracle of oracles) {
    try {
      const combo = page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 6000 });
      await page.waitForTimeout(400);
      await page.locator('[role="option"]').filter({ hasText: oracle.label }).first().click({ timeout: 6000 });
    } catch (error) {
      record(oracle.label, "edit", "switch-frame", false, { error: String(error).slice(0, 200) });
      continue;
    }
    const snap = await waitConverged(page, oracle, "edit");
    const pane = previewPane(snap, "edit");
    const box = await canvasBox(page);
    const aspect = box ? box.width / box.height : 1;
    const grade = gradeCameraFrames(pane?.camera ?? null, deliveryBox(oracle), aspect);
    record(oracle.label, "edit", "switch-frame", Boolean(pane) && settled(pane) && snap.example === oracle.label && grade.ok, {
      example: snap.example, converged: settled(pane), camera: pane?.camera ?? null, box: deliveryBox(oracle),
      worstX: grade.worstX, worstY: grade.worstY, behind: grade.behind, aspect, reasons: grade.reasons,
    });
    if (shots) await page.screenshot({ path: join(outDir, `switch-${oracle.slug}.png`) });
  }
  await page.close();
}
//#endregion 🔀️SwitchFrame

writeFileSync(join(outDir, "results.json"), JSON.stringify({ base, roles, examples: oracles.map((oracle) => oracle.slug), results, faults }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
const red = results.filter((row) => !row.ok);
console.log(`[DEBUG] DONE rows=${results.length} green=${results.length - red.length} red=${red.length} pageerrors=${faults.length}`);
for (const row of red) console.log(`[DEBUG] RED ${row.role}:${row.example} ${row.step} ${JSON.stringify(row.detail.reasons ?? row.detail).slice(0, 300)}`);
await browser.close();
if (red.length > 0) process.exit(1);
