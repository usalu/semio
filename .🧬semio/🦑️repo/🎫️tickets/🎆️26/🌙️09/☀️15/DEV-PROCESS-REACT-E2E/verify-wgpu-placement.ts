import { mkdir, writeFile } from "node:fs/promises";
import { PNG } from "pngjs";
import { PerspectiveCamera, Vector3 } from "three";
import { chromium, expect, observeProcessPluginDelivery, processPluginIds } from "./browser-tooling";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import fixture from "./browser-import-fixture.json";

type Rect = [number, number, number, number];
type Hit = { controlId: string; kind: string; windowId?: string; rect: Rect; dragAxis?: string };
type Camera = { position: number[]; target: number[]; fov?: number; zoom?: number; up?: number[]; projection?: string };
type World = { rect: Rect; camera: Camera; liveCamera: Camera | null; meshes: { id: string; indices: number; positions: number }[] };
type ChromeDump = { hits: Hit[]; actions: { seq: number; action: string; args?: string }[] };

declare global {
  interface Window {
    semioWgpuIntrospection?: {
      dumpChrome: () => Promise<string>;
      dumpMeshStats: () => Promise<string>;
    };
  }
}

const target = new URL(process.argv[2] ?? "http://127.0.0.1:6122/");
target.searchParams.set("example", "demo");
target.searchParams.set("role", "editor");
const viewport = { width: 1600, height: 1000 };
const screenshots = new URL("./🗑️generated/", import.meta.url);
const browser = await chromium.launch({ args: browserLaunchOptionsV1().args });
try {
  const page = await browser.newPage({ locale: "en-US", viewport, deviceScaleFactor: 1 });
  const delivered = observeProcessPluginDelivery(page.context(), target);
  await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  await page.goto(target.href);
  await expect(page.locator("#semio-wgpu-canvas")).toBeVisible({ timeout: 90_000 });
  await page.waitForFunction(() => typeof window.semioWgpuIntrospection?.dumpChrome === "function");
  const chrome = async (): Promise<ChromeDump> => page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection!.dumpChrome()));
  const world = async (): Promise<World | undefined> => page.evaluate(async () => {
    const { surfaces } = JSON.parse(await window.semioWgpuIntrospection!.dumpMeshStats()) as { surfaces: World[] };
    return surfaces.find(surface => surface.meshes.some(mesh => mesh.id === "processed" && mesh.indices > 0));
  });
  await expect.poll(world, { timeout: 90_000 }).toBeDefined();
  await expect.poll(delivered).toEqual(processPluginIds);
  const workpiece = (await world())!;
  const workpieceHit = (await chrome()).hits.find(hit => hit.kind === "World3d" && hit.rect.every((value, index) => Math.abs(value - workpiece.rect[index]!) <= 1));
  expect(workpieceHit?.windowId).toBeTruthy();
  const windowId = workpieceHit!.windowId;
  const [workpieceX, workpieceY, workpieceWidth, workpieceHeight] = workpiece.rect;
  const originalCamera = new Vector3().fromArray(workpiece.camera.position);
  await page.mouse.move(workpieceX + workpieceWidth * 0.7, workpieceY + workpieceHeight * 0.45);
  await page.mouse.wheel(0, -120);
  await expect.poll(async () => {
    const state = await world();
    if (!state?.liveCamera) return false;
    const live = new Vector3().fromArray(state.liveCamera.position);
    const published = new Vector3().fromArray(state.camera.position);
    return published.distanceTo(originalCamera) > 1e-4 && live.distanceTo(published) < 1e-4;
  }).toBe(true);
  expect((await world())?.meshes).toEqual(workpiece.meshes);

  const click = ({ rect: [x, y, width, height] }: Hit) => page.mouse.click(x + width / 2, y + height / 2);
  const cutId = `framework.utility.toggle.${fixture.demoPlacement.tool}`;
  const cutToggle = async () => (await chrome()).hits.find(hit => hit.windowId === windowId && hit.controlId === cutId);
  const cutState = page.locator(`button[data-node-key="${cutId}"]`);
  if (!(await cutToggle())) {
    const unfold = (await chrome()).hits.find(hit => hit.windowId === windowId && hit.controlId.endsWith(".utilityBar.unfold"));
    expect(unfold).toBeDefined();
    await click(unfold!);
    await expect.poll(cutToggle).toBeDefined();
  }
  await click((await cutToggle())!);
  await expect(cutState).toHaveAttribute("aria-pressed", "true");
  const current = (await world())!;
  const [left, top, width, height] = current.rect;
  const canvas = (await page.locator("#semio-wgpu-canvas").boundingBox())!;
  const x = Math.ceil(Math.max(left + 2, canvas.x, 0));
  const y = Math.ceil(Math.max(top + 2, canvas.y, 0));
  const clip = {
    x, y,
    width: Math.floor(Math.min(left + width - 2, canvas.x + canvas.width, viewport.width)) - x,
    height: Math.floor(Math.min(top + height - 2, canvas.y + canvas.height, viewport.height)) - y,
  };
  expect(clip.width).toBeGreaterThan(0);
  expect(clip.height).toBeGreaterThan(0);
  await mkdir(screenshots, { recursive: true });
  const capture = async (phase: string) => {
    const filename = `wgpu-workpiece-${phase}.png`;
    let previous: Buffer | undefined;
    let stable = false;
    for (let attempt = 0; attempt < 5 && !stable; attempt++) {
      await page.waitForFunction(() => new Promise<boolean>(resolve => {
        requestAnimationFrame(() => requestAnimationFrame(() => resolve(true)));
      }), undefined, { timeout: 5_000 });
      const pixels = await page.screenshot({ clip, scale: "css", timeout: 5_000 });
      stable = previous !== undefined && pixels.equals(previous);
      previous = pixels;
    }
    await writeFile(new URL(filename, screenshots), previous!);
    console.log(`[DEBUG] ${filename} ${JSON.stringify({ windowId, clip, stable })}`);
    return PNG.sync.read(previous!);
  };
  const pose = current.liveCamera ?? current.camera;
  const camera = new PerspectiveCamera(pose.fov ?? current.camera.fov ?? 45, width / height);
  camera.position.fromArray(pose.position);
  camera.up.fromArray(pose.up ?? [0, 0, 1]);
  camera.zoom = pose.zoom ?? 1;
  camera.lookAt(new Vector3().fromArray(pose.target));
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld();
  const originalTarget = fixture.demoPlacement.positionMetres;
  const screenPoint = (position: number[]) => {
    const point = new Vector3().fromArray(position).project(camera);
    return [left + width * (point.x + 1) / 2, top + height * (1 - point.y) / 2] as const;
  };
  const lastSequence = (dump: ChromeDump) => Math.max(0, ...dump.actions.map(action => action.seq));
  const expectPointerAction = async (action: string, after: number, position: number[]) => {
    await expect.poll(async () => {
      const row = (await chrome()).actions.findLast(row => row.seq > after && row.action === action);
      if (!row?.args) return false;
      const args = JSON.parse(row.args);
      return args.windowId === windowId && Array.isArray(args.position) && args.position.length === 3
        && position.every((value, axis) => Math.abs(args.position[axis] - value) < 0.025);
    }).toBe(true);
  };
  const armedFrame = await capture("armed");
  const toolSize = fixture.demoPlacement.toolSizeMetres;
  const previewPixels = (frame: PNG, position: number[], reference = armedFrame, size = toolSize) => {
    expect([frame.width, frame.height]).toEqual([reference.width, reference.height]);
    const corners = Array.from({ length: 8 }, (_, corner) => screenPoint(
      position.map((value, axis) => value + (corner & (1 << axis) ? 0.5 : -0.5) * size[axis]!),
    ));
    const xs = corners.map(([x]) => x - clip.x);
    const ys = corners.map(([, y]) => y - clip.y);
    const minX = Math.max(0, Math.floor(Math.min(...xs)) - 4);
    const minY = Math.max(0, Math.floor(Math.min(...ys)) - 4);
    const maxX = Math.min(frame.width - 1, Math.ceil(Math.max(...xs)) + 4);
    const maxY = Math.min(frame.height - 1, Math.ceil(Math.max(...ys)) + 4);
    let changed = 0;
    for (let row = minY; row <= maxY; row++) {
      for (let column = minX; column <= maxX; column++) {
        const pixel = (row * frame.width + column) * 4;
        const difference = Math.max(
          Math.abs(frame.data[pixel]! - reference.data[pixel]!),
          Math.abs(frame.data[pixel + 1]! - reference.data[pixel + 1]!),
          Math.abs(frame.data[pixel + 2]! - reference.data[pixel + 2]!),
        );
        if (difference >= 16) changed++;
      }
    }
    return changed;
  };
  const hoverAt = async (phase: string, position: number[]) => {
    const beforeHover = lastSequence(await chrome());
    await page.mouse.move(...screenPoint(position));
    await expectPointerAction("worldPointerMove", beforeHover, position);
    expect((await world())?.meshes).toEqual(current.meshes);
    await expect.poll(async () => {
      const frame = PNG.sync.read(await page.screenshot({ clip, scale: "css", timeout: 5_000 }));
      return previewPixels(frame, position);
    }, { message: `The visible Workpiece should paint the cut preview at ${phase}` }).toBeGreaterThan(12);
    expect(previewPixels(await capture(phase), position)).toBeGreaterThan(12);
  };
  await hoverAt("hover-1", [originalTarget[0]! - 0.15, ...originalTarget.slice(1)]);
  const beforeAbort = lastSequence(await chrome());
  await page.keyboard.press("Escape");
  await expect.poll(async () => (await chrome()).actions.some(
    row => row.seq > beforeAbort && row.action === "engagementAbort",
  )).toBe(true);
  await expect(cutState).toHaveAttribute("aria-pressed", "false");
  await expect.poll(async () => (await world())?.meshes).toEqual(current.meshes);
  await click((await cutToggle())!);
  await expect(cutState).toHaveAttribute("aria-pressed", "true");
  await hoverAt("hover-2", originalTarget);
  const beforeClick = lastSequence(await chrome());
  await page.mouse.click(...screenPoint(originalTarget));
  await expectPointerAction("worldPointerDown", beforeClick, originalTarget);
  const baseline = JSON.stringify(current.meshes);
  await expect.poll(async () => {
    const applied = await world();
    return applied !== undefined && JSON.stringify(applied.meshes) !== baseline;
  }).toBe(true);
  await expect(cutState).toHaveAttribute("aria-pressed", "false");
  await capture("committed");

  const drillStep = "dowel-drill";
  const attachStep = fixture.demoAttachment.step;
  const timelineRowHit = (hits: Hit[], id: string) => hits.find(hit => hit.kind === "TreeItem" && hit.controlId === `tree.label.${id}`);
  if (!timelineRowHit((await chrome()).hits, drillStep)) {
    await page.getByRole("tab", { name: "Artifact", exact: true }).press("Enter");
  }
  const timelineOrder = async () => {
    const { hits } = await chrome();
    const drill = timelineRowHit(hits, drillStep);
    const attach = timelineRowHit(hits, attachStep);
    if (!drill?.windowId || !attach || drill.windowId !== attach.windowId) return [];
    return drill.rect[1] < attach.rect[1] ? [drillStep, attachStep] : [attachStep, drillStep];
  };
  const dragAttachment = async (position: "before" | "after") => {
    const { hits } = await chrome();
    const targetRow = timelineRowHit(hits, drillStep)!;
    const sourceRow = timelineRowHit(hits, attachStep)!;
    const handle = hits.find(hit => hit.windowId === sourceRow.windowId && hit.kind === "TreeDragHandle"
      && hit.controlId === `tree.drag.transfer.${attachStep}`);
    expect(handle ?? sourceRow.dragAxis).toBeTruthy();
    const [fromX, fromY, fromWidth, fromHeight] = (handle ?? sourceRow).rect;
    const [toX, toY, toWidth, toHeight] = targetRow.rect;
    const x = toX + toWidth / 2;
    const y = toY + toHeight * (position === "before" ? 0.15 : 0.85);
    const edge = position === "before" ? toY : toY + toHeight - 2;
    const edgeX = Math.ceil(Math.max(toX + 0.2 * toWidth, canvas.x, 0));
    const edgeY = Math.floor(Math.max(edge - 1, canvas.y, 0));
    const edgeClip = {
      x: edgeX, y: edgeY,
      width: Math.floor(Math.min(toX + 0.8 * toWidth, viewport.width)) - edgeX,
      height: Math.min(4, viewport.height - edgeY),
    };
    expect(edgeClip.width).toBeGreaterThan(20);
    expect(edgeClip.height).toBeGreaterThan(0);
    const sampleEdge = async () => PNG.sync.read(await page.screenshot({ clip: edgeClip, scale: "css", timeout: 5_000 }));
    await page.mouse.move(fromX + fromWidth / 2, fromY + fromHeight / 2);
    const unmarked = await sampleEdge();
    await page.mouse.down();
    try {
      await page.mouse.move(x, y, { steps: 12 });
      await expect.poll(async () => {
        const marked = await sampleEdge();
        let longest = 0;
        for (let row = 0; row < marked.height; row++) {
          let changed = 0;
          for (let column = 0; column < marked.width; column++) {
            const pixel = 4 * (row * marked.width + column);
            if ([0, 1, 2].some(channel => Math.abs(marked.data[pixel + channel]! - unmarked.data[pixel + channel]!) >= 16)) changed++;
          }
          longest = Math.max(longest, changed);
        }
        return longest;
      }).toBeGreaterThan(Math.floor(edgeClip.width * 0.45));
    } finally {
      await page.mouse.up();
    }
  };
  await expect.poll(timelineOrder).toEqual([drillStep, attachStep]);
  await dragAttachment("before");
  await expect.poll(timelineOrder).toEqual([attachStep, drillStep]);
  await dragAttachment("after");
  await expect.poll(timelineOrder).toEqual([drillStep, attachStep]);
  console.log("[DEBUG] Process editor WGPU camera, placement, cancellation and timeline dragging verified");

  const chosen = fixture.wgpuChosenPlacement;
  const byKey = (key: string) => page.locator(`[data-node-key=${JSON.stringify(key)}]`);
  await page.getByRole("tab", { name: "Workshop", exact: true }).press("Enter");
  const installed = byKey(`machine:${chosen.machine}`);
  if (!(await installed.count())) {
    const machine = byKey(`process3d-workshop.catalog.${chosen.catalog}.${chosen.machine}`);
    if (!(await machine.count())) await byKey(`process3d-play-workshop.catalog.${chosen.catalog}`).press("Enter");
    await machine.press("Enter");
    await expect(installed).toHaveCount(1);
  }
  await page.getByRole("tab", { name: /^Catalog(?:ue)?$/i }).press("Enter");
  const rowKey = `process3d-catalogue.${chosen.machine}.${chosen.capability}`;
  const place = byKey(`${rowKey}::row-action::0`);
  if (!(await place.count())) await byKey(`process3d-play-catalogue.${chosen.catalog}`).press("Enter");
  await expect(place).toHaveAccessibleName(/^Place capability:/);
  const beforeChosen = lastSequence(await chrome());
  await place.press("Enter");
  await expect.poll(async () => (await chrome()).actions.some(row => {
    if (row.seq <= beforeChosen || row.action !== "placeCapability" || !row.args) return false;
    const args = JSON.parse(row.args);
    return args.machineId === chosen.machine && args.capabilityId === chosen.capability;
  })).toBe(true);
  const chosenMesh = (await world())!.meshes;
  const chosenFrame = await capture("chosen-armed");
  const beforeChosenHover = lastSequence(await chrome());
  await page.mouse.move(...screenPoint(chosen.positionMetres));
  await expectPointerAction("worldPointerMove", beforeChosenHover, chosen.positionMetres);
  await expect.poll(async () => {
    const frame = PNG.sync.read(await page.screenshot({ clip, scale: "css", timeout: 5_000 }));
    return previewPixels(frame, chosen.positionMetres, chosenFrame, chosen.toolSizeMetres);
  }).toBeGreaterThan(8);
  expect((await world())?.meshes).toEqual(chosenMesh);
  await capture("chosen-hover");
  const beforeChosenClick = lastSequence(await chrome());
  await page.mouse.click(...screenPoint(chosen.positionMetres));
  await expectPointerAction("worldPointerDown", beforeChosenClick, chosen.positionMetres);
  await expect.poll(async () => {
    const applied = await world();
    return applied !== undefined && JSON.stringify(applied.meshes) !== JSON.stringify(chosenMesh);
  }).toBe(true);
  await capture("chosen-committed");
  console.log("[DEBUG] Process editor WGPU chosen capability preview and commit verified");

} finally {
  await browser.close();
}
