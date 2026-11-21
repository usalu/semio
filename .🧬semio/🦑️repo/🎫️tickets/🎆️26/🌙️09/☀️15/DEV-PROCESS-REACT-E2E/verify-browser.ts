import { chromium, expect, observeProcessPluginDelivery, processPluginIds } from "./browser-tooling";
import { fileURLToPath } from "node:url";
import { Box3, PerspectiveCamera, Vector3 } from "three";
import fixture from "./browser-import-fixture.json";
import { objVolume } from "./obj-volume";

const target = new URL(process.argv[2] ?? "http://127.0.0.1:6022/");
target.searchParams.set("example", "demo");
target.searchParams.set("role", "editor");
const browser = await chromium.launch({ downloadsPath: fileURLToPath(new URL("./🗑️generated", import.meta.url)) });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  page.setDefaultTimeout(20000);
  const treeRow = (key: string) => page.locator(`[id$="/${key}"]`);
  const faults: string[] = [];
  const delivered = observeProcessPluginDelivery(page.context(), target);
  page.on("pageerror", error => faults.push(error.message));
  page.on("console", message => {
    if (message.type() === "error" && /surface-render|faults=|react boot failed/i.test(message.text())) faults.push(message.text());
  });
  await page.goto(target.href);
  const drill = page.getByText("Drill Dowel Hole", { exact: true });
  await expect(drill.first()).toBeVisible({ timeout: 90000 });
  await drill.first().click();
  const radius = page.getByRole("spinbutton", { name: /^Radius$/i });
  if (!(await radius.isVisible())) {
    const inspection = page.getByRole("tab", { name: /^(Inspection|Inspector)$/i });
    if (await inspection.isVisible()) await inspection.click();
  }
  await expect(radius).toBeVisible();
  const original = await radius.inputValue();
  const mesh = page.locator("[data-meshes-json]").first();
  const readProcessMesh = async (host = mesh) => {
    const payload = await host.getAttribute("data-meshes-json");
    const row = JSON.parse(payload ?? "[]")[0];
    if (row?.id !== "processed" || !row.data?.positions?.length || !row.data?.indices?.length) return null;
    return JSON.stringify({ positions: row.data.positions, indices: row.data.indices });
  };
  const expectChangedMesh = async (baseline: string | null) => {
    await expect.poll(async () => {
      const rendered = await readProcessMesh();
      return rendered !== null && rendered !== baseline;
    }).toBe(true);
  };
  await expect.poll(readProcessMesh).not.toBeNull();
  const canvas = mesh.locator("canvas").first();
  await expect(canvas).toBeVisible();
  const previous = await readProcessMesh();
  const readGuestCamera = () => mesh.getAttribute("data-camera-json");
  const cameraPosition = (camera: string | null) => camera === null ? null : JSON.stringify(JSON.parse(camera).position);
  const initialGuestPosition = cameraPosition(await readGuestCamera());
  const initialViewportPosition = cameraPosition(await mesh.getAttribute("data-viewport-camera-json"));
  expect(initialGuestPosition).not.toBeNull();
  expect(initialViewportPosition).not.toBeNull();
  const canvasBounds = (await canvas.boundingBox())!;
  await page.mouse.move(canvasBounds.x + canvasBounds.width * 0.7, canvasBounds.y + canvasBounds.height * 0.45);
  await page.mouse.wheel(0, -240);
  await expect.poll(async () => {
    const viewport = cameraPosition(await mesh.getAttribute("data-viewport-camera-json"));
    const guest = cameraPosition(await readGuestCamera());
    return guest !== null && viewport === guest && guest !== initialGuestPosition && viewport !== initialViewportPosition;
  }).toBe(true);
  const movedCamera = await readGuestCamera();
  await expect.poll(readProcessMesh).toBe(previous);
  const readSun = async () => JSON.parse((await mesh.getAttribute("data-sun-json")) ?? "null");
  const priorSun = await readSun();
  expect(typeof priorSun?.enabled).toBe("boolean");
  const windowOptions = page.locator('[data-slot="window-measures-overlay"]').first();
  const sunGroup = windowOptions.getByRole("button", { name: "Sun", exact: true });
  const sunEnabled = windowOptions.getByRole("checkbox", { name: "Enabled", exact: true });
  const optionsButton = windowOptions.locator('button[data-slot="window-pane-chrome-toggle"][title="Window Options"]');
  const openedOptions = !(await sunGroup.isVisible());
  if (openedOptions) await optionsButton.click();
  const openedSun = !(await sunEnabled.isVisible());
  if (openedSun) await sunGroup.click();
  await expect(sunEnabled).toBeVisible();
  await expect(windowOptions).toBeVisible();
  await sunEnabled.click();
  await expect.poll(async () => (await readSun())?.enabled).toBe(!priorSun.enabled);
  await expect.poll(readProcessMesh).toBe(previous);
  await sunEnabled.click();
  await expect.poll(readSun).toEqual(priorSun);
  await expect.poll(readGuestCamera).toBe(movedCamera);
  if (openedSun) await sunGroup.click();
  if (openedOptions) await optionsButton.click();
  expect(Number(original)).toBeGreaterThan(0);
  await radius.fill(String(Number(original) * 1.25));
  await radius.press("Tab");
  await expectChangedMesh(previous);
  await radius.fill(original);
  await radius.press("Tab");
  await expect.poll(readProcessMesh).toBe(previous);
  const drillEnabled = treeRow("dowel-drill").getByRole("button", { name: "Enabled", exact: true });
  await drillEnabled.click();
  await expectChangedMesh(previous);
  await expect(drill.first()).toBeVisible();
  await drillEnabled.click();
  await expect.poll(readProcessMesh).toBe(previous);
  const cursor = page.locator("#process3d-cursor");
  const stepper = page.locator('[data-slot="stepper-group"]').filter({ has: cursor });
  const lastStep = Number(await cursor.inputValue());
  expect(lastStep).toBeGreaterThan(0);
  await stepper.locator('[data-slot="stepper-minus"]').click();
  await expect(cursor).toHaveValue(String(lastStep - 1));
  await expectChangedMesh(previous);
  await stepper.locator('[data-slot="stepper-plus"]').click();
  await expect(cursor).toHaveValue(String(lastStep));
  await expect.poll(readProcessMesh).toBe(previous);
  const attachment = fixture.demoAttachment;
  const dowel = treeRow(attachment.step);
  await dowel.getByText(attachment.label, { exact: true }).click();
  const dowelHeight = page.getByRole("spinbutton", { name: "Height", exact: true });
  await expect(dowelHeight).toHaveValue(String(attachment.originalHeightMetres));
  await dowelHeight.fill(String(attachment.editedHeightMetres));
  await dowelHeight.press("Tab");
  await expectChangedMesh(previous);
  await dowelHeight.fill(String(attachment.originalHeightMetres));
  await dowelHeight.press("Tab");
  await expect.poll(readProcessMesh).toBe(previous);
  const documentTree = page.locator('[data-ui-node-key="process3d-play-document"]');
  const documentLabels = documentTree.locator('[role="treeitem"] [data-slot="tree-label"]');
  const artifactTab = page.getByRole("tab", { name: "Artifact", exact: true });
  const timelineRows = documentTree.locator('[role="treeitem"][id*="/step-"]');
  const status = page.locator('[data-slot="engagement-status-item"]').filter({ hasText: "m³" });
  const drillLabel = "Drill Dowel Hole";
  const visibleDowelSteps = async () => (await documentLabels.allTextContents()).filter(label => label === attachment.label || label === drillLabel);
  await page.getByText("Move Earlier", { exact: true }).click();
  await expect.poll(visibleDowelSteps).toEqual([attachment.label, drillLabel]);
  await expectChangedMesh(previous);
  await page.getByText("Move Later", { exact: true }).click();
  await expect.poll(visibleDowelSteps).toEqual([drillLabel, attachment.label]);
  await expect.poll(readProcessMesh).toBe(previous);
  await cursor.fill("0");
  await cursor.press("Enter");
  await expectChangedMesh(previous);
  const rawStockMesh = await readProcessMesh();
  await page.getByText("Timber Beam", { exact: true }).first().click();
  await expect.poll(async () => {
    const selection = await mesh.getAttribute("data-selection-json");
    return JSON.parse(selection ?? "{}").selectedIds;
  }).toContain("beam");
  const width = page.getByRole("spinbutton", { name: /^Width$/i });
  await expect(width).toBeVisible();
  const originalWidth = await width.inputValue();
  await width.fill(String(Number(originalWidth) * 1.1));
  await width.press("Tab");
  await expectChangedMesh(rawStockMesh);
  await width.fill(originalWidth);
  await width.press("Tab");
  await expect.poll(readProcessMesh).toBe(rawStockMesh);
  await cursor.fill(String(lastStep));
  await cursor.press("Enter");
  await expect.poll(readProcessMesh).toBe(previous);
  await drill.first().click();
  const moveEarlier = page.getByText("Move Earlier", { exact: true });
  await expect(moveEarlier).toBeVisible();
  await moveEarlier.click();
  const lap = page.getByText("Cut Lap Joint", { exact: true });
  await expect.poll(async () => {
    const drillElement = await drill.first().elementHandle();
    try {
      return await lap.first().evaluate((lapElement, drillElement) => Boolean(drillElement.compareDocumentPosition(lapElement) & Node.DOCUMENT_POSITION_FOLLOWING), drillElement);
    } finally {
      await drillElement.dispose();
    }
  }).toBe(true);
  const installedChopSaw = treeRow("machine:chopSaw");
  const addMachine = async ({ catalog, catalogLabel, machine }: (typeof fixture.workshop)[number]) => {
    const section = treeRow(`process3d-play-workshop.catalog.${catalog}`);
    if (!(await section.isVisible())) {
      await page.getByRole("tab", { name: /^Workshop$/i }).click();
    }
    const installed = treeRow(`machine:${machine}`);
    if (await installed.isVisible()) return;
    const action = treeRow(`process3d-workshop.catalog.${catalog}.${machine}`).locator('[data-slot="tree-label"]');
    if (!(await action.isVisible())) {
      await section.getByText(catalogLabel, { exact: true }).click();
    }
    await action.click();
    await expect(installed).toBeVisible();
  };
  await addMachine(fixture.workshop[0]!);
  await installedChopSaw.getByText("Chop Saw", { exact: true }).click();
  const machineLabel = page.getByRole("textbox", { name: /^Label$/i });
  await expect(machineLabel).toHaveValue("Chop Saw");
  await machineLabel.fill("Shop Chop Saw");
  await machineLabel.press("Tab");
  await expect(installedChopSaw.getByText("Shop Chop Saw", { exact: true })).toBeVisible();
  await expect.poll(delivered).toEqual(processPluginIds);

  const placement = fixture.demoPlacement;
  await artifactTab.click();
  await expect(timelineRows).toHaveCount(0);
  await cursor.fill("0");
  await cursor.press("Enter");
  await expect(cursor).toHaveValue("0");
  await expect.poll(readProcessMesh).toBe(rawStockMesh);
  const workpieceWindow = page.locator('[data-slot="window"]').filter({ has: mesh });
  const utilityBar = workpieceWindow.locator('[data-slot="utility-bar-overlay"]');
  const utilityBody = utilityBar.locator('[data-slot="utility-bar-body"]');
  if (!(await utilityBody.isVisible())) {
    await utilityBar.locator('button[data-slot="window-pane-chrome-toggle"]').click();
  }
  const cutTool = utilityBody.locator(`[data-toggle-value="${placement.tool}"]`);
  await cutTool.click();
  const activeUtility = async () => JSON.parse((await mesh.getAttribute("data-selection-json")) ?? "{}").activeUtility;
  await expect.poll(activeUtility).toBe(placement.tool);
  const hostBounds = (await mesh.boundingBox())!;
  const liveCanvasBounds = (await canvas.boundingBox())!;
  const liveCamera = JSON.parse((await mesh.getAttribute("data-viewport-camera-json"))!);
  expect(liveCamera.projection).toBe("perspective");
  const camera = new PerspectiveCamera(liveCamera.fov, hostBounds.width / hostBounds.height);
  camera.position.fromArray(liveCamera.position);
  camera.up.fromArray(liveCamera.up ?? [0, 0, 1]);
  camera.zoom = liveCamera.zoom;
  camera.lookAt(new Vector3().fromArray(liveCamera.target));
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld();
  const clientPoint = (position: readonly number[]) => {
    const point = new Vector3().fromArray(position).project(camera);
    return { x: hostBounds.x + (point.x + 1) * hostBounds.width / 2, y: hostBounds.y + (1 - point.y) * hostBounds.height / 2 };
  };
  const readPlacementPreview = async (): Promise<{ kind: string; role: string; from: number[]; to: number[] }[]> =>
    JSON.parse((await mesh.getAttribute("data-engagement-preview-json")) ?? "[]");
  const previewBounds = async () => {
    const segments = (await readPlacementPreview()).filter(item => item.kind === "segment" && item.role === placement.tool);
    return new Box3().setFromPoints(segments.flatMap(({ from, to }) => [new Vector3().fromArray(from), new Vector3().fromArray(to)]));
  };
  const start = [...placement.positionMetres];
  start[0] = start[0]! - 0.3;
  const startPoint = clientPoint(start);
  await page.mouse.move(startPoint.x, startPoint.y);
  await expect.poll(async () => {
    const bounds = await previewBounds();
    return bounds.isEmpty() ? Number.NaN : bounds.getCenter(new Vector3()).x;
  }).toBeCloseTo(start[0], 1);
  const point = clientPoint(placement.positionMetres);
  await page.mouse.move(point.x, point.y);
  await expect.poll(async () => (await previewBounds()).getCenter(new Vector3()).x).toBeCloseTo(placement.positionMetres[0]!, 1);
  const previewSize = (await previewBounds()).getSize(new Vector3()).toArray();
  for (const [axis, size] of previewSize.entries()) expect(size).toBeCloseTo(placement.toolSizeMetres[axis]!, 4);
  await expect(cursor).toHaveValue("0");
  await expect(timelineRows).toHaveCount(0);
  await expect.poll(readProcessMesh).toBe(rawStockMesh);
  await canvas.click({ position: {
    x: point.x - liveCanvasBounds.x,
    y: point.y - liveCanvasBounds.y,
  } });
  await expect(cursor).toHaveValue("1");
  await expect(timelineRows).toHaveCount(1);
  await expect(status).toContainText(placement.remainingVolumeMetresCubed.toFixed(4) + " m³");
  await expectChangedMesh(rawStockMesh);
  await expect.poll(activeUtility).toBe("select");
  await expect.poll(readPlacementPreview).toEqual([]);
  await timelineRows.first().getByText(placement.label, { exact: true }).click();
  const placedX = page.getByRole("spinbutton", { name: "X", exact: true });
  if (!(await placedX.isVisible())) {
    await page.getByRole("tab", { name: /^(Inspection|Inspector)$/i }).click();
  }
  await expect.poll(async () => Number(await placedX.inputValue())).toBeCloseTo(placement.positionMetres[0]!, 1);

  const concrete = new URL(target);
  concrete.searchParams.set("example", "concrete-forest");
  await page.goto(concrete.href);
  await expect(page.getByText("Wall Saw Slab Cut", { exact: true }).first()).toBeVisible({ timeout: 90000 });
  await expect(cursor).toHaveValue("7");
  await expect.poll(readProcessMesh).not.toBeNull();
  const processedConcrete = await readProcessMesh();
  await cursor.fill("0");
  await cursor.press("Enter");
  await expect(cursor).toHaveValue("0");
  await expectChangedMesh(processedConcrete);
  await expect.poll(async () => {
    const payload = await mesh.getAttribute("data-instances-json");
    return JSON.parse(payload ?? "[]")[0]?.id;
  }).toBe("hexagonal-cut-concrete-forest-left");

  const openStockPicker = async () => {
    const stockSection = treeRow("process3d-play-catalogue.stock");
    if (!(await stockSection.isVisible())) await page.getByRole("tab", { name: /^Catalog(?:ue)?$/i }).click();
    const importStock = treeRow("process3d-catalogue.stock-import").locator('[data-slot="tree-label"]');
    if (!(await importStock.isVisible())) await stockSection.getByText("Stock", { exact: true }).click();
    const [chooser] = await Promise.all([page.waitForEvent("filechooser"), importStock.click()]);
    return chooser;
  };
  const picker = await openStockPicker();
  await picker.setFiles(fileURLToPath(new URL("./browser-stock.obj", import.meta.url)));
  await expect(page.getByText("Imported OBJ", { exact: true }).first()).toBeVisible();
  await expect(cursor).toHaveValue("0");
  await expect(page.getByText(/0\/0 .+ · 8\.0000 m³$/)).toBeVisible();
  await expect.poll(readProcessMesh).not.toBeNull();
  const importedStockMesh = await readProcessMesh();
  const oversized = await openStockPicker();
  await oversized.setFiles({ name: "over-limit.obj", mimeType: "text/plain", buffer: Buffer.alloc(fixture.maximumRawFileBytes + 1) });
  await expect(page.getByText("“over-limit.obj” exceeds the size limit for this import.", { exact: true })).toBeVisible();
  await expect(page.getByText("Imported OBJ", { exact: true }).first()).toBeVisible();
  await expect.poll(readProcessMesh).toBe(importedStockMesh);

  const stepFile = await openStockPicker();
  const stepBytes = await Bun.file(fixture.step.source).bytes();
  expect(stepBytes.length).toBe(fixture.step.rawFileBytes);
  const extension = ".STEP";
  const filename = "株".repeat(fixture.maximumFilenameUtf16Units - extension.length) + extension;
  const paddedStep = Buffer.concat([stepBytes, Buffer.alloc(fixture.maximumRawFileBytes - stepBytes.length, 0x20)]);
  await stepFile.setFiles({ name: filename, mimeType: "application/octet-stream", buffer: paddedStep });
  await expect(page.getByText("Imported STEP", { exact: true }).first()).toBeVisible();
  await expect(page.getByText(/0\/0 .+ · 8\.0000e-6 m³$/)).toBeVisible();
  await expectChangedMesh(importedStockMesh);
  const stepMesh = await readProcessMesh();
  const positions: number[] = JSON.parse(stepMesh!).positions;
  expect(positions.length).toBeGreaterThan(0);
  for (const [axis, expected] of fixture.step.extentMetres.entries()) {
    const values = positions.filter((_, index) => index % 3 === axis);
    expect(Math.max(...values) - Math.min(...values)).toBeCloseTo(expected, 5);
  }
  for (const machine of fixture.workshop) await addMachine(machine);
  const catalogueTab = page.getByRole("tab", { name: /^Catalog(?:ue)?$/i });
  for (const [index, { catalog, catalogLabel, machine, capability, stepLabel, remainingVolumeMetresCubed, edit }] of fixture.workshop.entries()) {
    await catalogueTab.click();
    const action = treeRow(`process3d-catalogue.${machine}.${capability}`).locator('[data-slot="tree-label"]');
    if (!(await action.isVisible())) {
      await treeRow(`process3d-play-catalogue.${catalog}`).getByText(catalogLabel, { exact: true }).click();
    }
    await action.click();
    const count = index + 1;
    await expect(cursor).toHaveValue(String(count));
    await expect(status).toContainText(`${count}/${count}`);
    await expect(status).toContainText(`${remainingVolumeMetresCubed.toExponential(4)} m³`);
    await expect.poll(readProcessMesh).not.toBeNull();
    const composedMesh = await readProcessMesh();
    await artifactTab.click();
    await expect(timelineRows).toHaveCount(count);
    await timelineRows.nth(index).getByText(stepLabel, { exact: true }).click();
    const size = page.getByRole("spinbutton", { name: edit.field, exact: true });
    if (!(await size.isVisible())) {
      await page.getByRole("tab", { name: /^(Inspection|Inspector)$/i }).click();
    }
    const originalSize = await size.inputValue();
    await size.fill(String(edit.sizeMetres));
    await size.press("Tab");
    await expect(status).toContainText(`${edit.remainingVolumeMetresCubed.toExponential(4)} m³`);
    await expectChangedMesh(composedMesh);
    await size.fill(originalSize);
    await size.press("Tab");
    await expect(status).toContainText(`${remainingVolumeMetresCubed.toExponential(4)} m³`);
    await expect.poll(readProcessMesh).toBe(composedMesh);
  }
  await expectChangedMesh(stepMesh);
  const composedMesh = await readProcessMesh();
  const exportVolume = fixture.workshop.at(-1)!.remainingVolumeMetresCubed;
  await canvas.click();
  await page.keyboard.press("ControlOrMeta+p");
  await page.locator('[id="ui.search.input"]').fill("Export Model");
  await page.getByRole("option", { name: /^Export Model\b/ }).click();
  await page.getByRole("combobox", { name: "Format", exact: true }).click();
  await page.getByRole("option", { name: "OBJ", exact: true }).click();
  const [workpieceDownload] = await Promise.all([
    page.waitForEvent("download"),
    page.getByRole("button", { name: "Execute", exact: true }).click(),
  ]);
  expect(workpieceDownload.suggestedFilename()).toMatch(/\.obj$/i);
  const exported = await Bun.file(await workpieceDownload.path()).text();
  expect(objVolume(exported)).toBeCloseTo(exportVolume, 10);
  await expect(cursor).toHaveValue(String(fixture.workshop.length));
  await expect.poll(readProcessMesh).toBe(composedMesh);

  const finalStep = fixture.workshop.at(-1)!;
  const stepCount = String(fixture.workshop.length);
  const edit = finalStep.edit;
  const editedSize = String(edit.sizeMetres);
  const savedSize = page.getByRole("spinbutton", { name: edit.field, exact: true }).filter({ visible: true });
  const openStepInspector = async () => {
    await artifactTab.click();
    await timelineRows.filter({ visible: true }).last().getByText(finalStep.stepLabel, { exact: true }).click();
    if (!(await savedSize.isVisible())) await page.getByRole("tab", { name: /^(Inspection|Inspector)$/i }).filter({ visible: true }).click();
  };
  await openStepInspector();
  await expect(savedSize).not.toHaveValue(editedSize);
  await savedSize.fill(editedSize);
  await savedSize.press("Tab");
  await expect(status).toContainText(`${edit.remainingVolumeMetresCubed.toExponential(4)} m³`);
  await expectChangedMesh(composedMesh);

  const workpieceSurface = page.locator('[data-surface-window-kind-id="process-workpiece"]:visible');
  const instanceAttribute = "data-surface-session-instance-id";
  const originalInstance = await workpieceSurface.getAttribute(instanceAttribute);
  expect(originalInstance).toMatch(/^\d+$/);
  const exportDocument = page.locator('[id="action.exportArtifactDocument"]:visible');
  const importDocument = page.locator('[id="action.importArtifactDocument"]:visible');
  const openDocumentActions = async () => {
    const workpiecePane = page.locator('[data-slot="window"]').filter({ has: workpieceSurface });
    await workpiecePane.locator('[id$=".engagement.toggle"]:visible').click();
  };
  if (!(await exportDocument.isVisible())) await openDocumentActions();
  const [archive] = await Promise.all([page.waitForEvent("download"), exportDocument.click()]);
  const archiveName = archive.suggestedFilename();
  expect(archiveName).toMatch(/\.semio-archive$/i);
  const archiveBytes = await Bun.file(await archive.path()).bytes();
  expect(archiveBytes.length).toBeGreaterThan(0);
  if (!(await importDocument.isVisible())) await openDocumentActions();
  const [archiveChooser] = await Promise.all([page.waitForEvent("filechooser"), importDocument.click()]);
  await archiveChooser.setFiles({ name: archiveName, mimeType: "application/octet-stream", buffer: Buffer.from(archiveBytes) });
  const reopenedSurface = workpieceSurface.and(page.locator(`[${instanceAttribute}]:not([${instanceAttribute}="${originalInstance}"])`));
  await expect(reopenedSurface).toBeVisible({ timeout: 90000 });

  await expect(artifactTab).toBeVisible();
  await artifactTab.click();
  const reopenedLabels = timelineRows.filter({ visible: true }).locator('[data-slot="tree-label"]');
  await expect(reopenedLabels).toHaveText(fixture.workshop.map(row => row.stepLabel));
  await openStepInspector();
  await expect(savedSize).toHaveValue(editedSize);

  const reopenedCursor = cursor.filter({ visible: true });
  const reopenedStatus = status.filter({ visible: true });
  const reopenedMesh = reopenedSurface.locator("[data-meshes-json]");
  const replayedVolume = async () => {
    const contents = await readProcessMesh(reopenedMesh);
    if (contents === null) return Number.NaN;
    const { positions, indices } = JSON.parse(contents) as { positions: number[]; indices: number[] };
    const a = new Vector3(), b = new Vector3(), c = new Vector3();
    let volumeTimesSix = 0;
    for (let i = 0; i + 2 < indices.length; i += 3) {
      a.fromArray(positions, 3 * indices[i]!);
      b.fromArray(positions, 3 * indices[i + 1]!);
      c.fromArray(positions, 3 * indices[i + 2]!);
      volumeTimesSix += a.dot(b.cross(c));
    }
    return Math.abs(volumeTimesSix / 6);
  };
  const verifyReopenedPrefix = async (prefix: string, volume: number) => {
    await reopenedCursor.fill(prefix);
    await reopenedCursor.press("Enter");
    await expect(reopenedCursor).toHaveValue(prefix);
    await expect(reopenedStatus).toContainText(`${prefix}/${stepCount}`);
    await expect(reopenedStatus).toContainText(`${volume.toExponential(4)} m³`);
    await expect.poll(replayedVolume).toBeCloseTo(volume, 10);
  };
  await verifyReopenedPrefix("0", fixture.step.extentMetres.reduce((volume, extent) => volume * extent, 1));
  await verifyReopenedPrefix(stepCount, edit.remainingVolumeMetresCubed);

  expect(faults).toEqual([]);
  console.log("[DEBUG] Process editor browser verification passed");
} finally {
  await browser.close();
}
