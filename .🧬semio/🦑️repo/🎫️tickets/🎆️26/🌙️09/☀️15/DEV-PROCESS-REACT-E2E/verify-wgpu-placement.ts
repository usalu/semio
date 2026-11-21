import { mkdir, writeFile } from "node:fs/promises";
import { PerspectiveCamera, Vector3 } from "three";
import { chromium, expect, observeProcessPluginDelivery, processPluginIds } from "./browser-tooling";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import fixture from "./browser-import-fixture.json";

type Rect = [number, number, number, number];
type Hit = { controlId: string; kind: string; windowId?: string; rect: Rect };
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
  const cutToggle = async () => (await chrome()).hits.find(hit => hit.windowId === windowId && hit.controlId === `framework.utility.toggle.${fixture.demoPlacement.tool}`);
  if (!(await cutToggle())) {
    const unfold = (await chrome()).hits.find(hit => hit.windowId === windowId && hit.controlId.endsWith(".utilityBar.unfold"));
    expect(unfold).toBeDefined();
    await click(unfold!);
    await expect.poll(cutToggle).toBeDefined();
  }
  await click((await cutToggle())!);
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
  await capture("armed");
  const hoverSamples: [string, number[]][] = [
    ["hover-1", [originalTarget[0]! - 0.15, ...originalTarget.slice(1)]],
    ["hover-2", originalTarget],
  ];
  for (const [phase, position] of hoverSamples) {
    const beforeHover = lastSequence(await chrome());
    await page.mouse.move(...screenPoint(position));
    await expectPointerAction("worldPointerMove", beforeHover, position);
    expect((await world())?.meshes).toEqual(current.meshes);
    await capture(phase);
  }
  const beforeClick = lastSequence(await chrome());
  await page.mouse.click(...screenPoint(originalTarget));
  await expectPointerAction("worldPointerDown", beforeClick, originalTarget);
  const baseline = JSON.stringify(current.meshes);
  await expect.poll(async () => {
    const applied = await world();
    return applied !== undefined && JSON.stringify(applied.meshes) !== baseline;
  }).toBe(true);
  await capture("committed");
  console.log("[DEBUG] Process editor WGPU camera and pointer placement verified");
} finally {
  await browser.close();
}
