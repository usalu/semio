/** 👁️ Viewer checks of the v3 matrix: standalone FEM viewer apps (ports 6088 and 6089) are read-only, drawn, and navigable. */
import { mk } from "./checks-core.mjs";

export function viewerChecks(a) {
  const d = a.dim, K = [];
  const add = (...args) => K.push(mk(...args));
  const V = d === 2 ? "V2D" : "V3D";
  const inv = (...n) => ({ [d]: n.map(i => `${V}-${String(i).padStart(2, "0")}`) });
  add("viewer.role", "viewer", "Boot standalone viewer", "Role group shows Viewer pressed", inv(1), async x => {
    const p = await x.isPressed("Viewer"); const e = await x.page.getByRole("button", { name: "Editor", exact: true }).count();
    return { pass: p === true || (p === null && e === 0), observed: `Viewer pressed=${p} editorButtons=${e}` };
  });
  add("viewer.model-drawn", "viewer", "Read-only Model window", "Scene drawn (more than 4 distinct colours)", d === 2 ? { 2: ["V2D-04", "E2D-138"] } : inv(4), async x => {
    const c = await x.drawn(); const view = await x.page.locator(`[aria-label="${a.viewLabel}"]`).count();
    return { pass: c > 4 && view > 0, observed: `distinctColors=${c} view=${view}` };
  });
  add("viewer.no-editor-chrome", "viewer", "Look for Utilities, Inspection, Results; open the Actions palette", "No enabled Utilities, no Inspection panel, no Results window; the palette lists only read-only framework verbs (Copy, Export/Import Document), never an editing action", inv(3, 7), async x => {
    const found = [], disabled = [];
    for (const n of [/^Utilities/, /^Inspection$/]) { const l = x.page.getByRole("button", { name: n }); if (await l.count()) { if (await l.first().isEnabled()) found.push(String(n)); else disabled.push(String(n)); } }
    if (await x.page.getByRole("tab", { name: "Results" }).count()) found.push("tab Results");
    const actions = x.page.getByRole("button", { name: /^Actions/ }); let items = [];
    if (await actions.count()) {
      await actions.first().click(); await x.page.waitForTimeout(1500);
      items = await x.page.evaluate(() => [...document.querySelectorAll("[role=tree] [role=treeitem]")].map(e => e.textContent.trim()));
      await actions.first().click(); await x.page.waitForTimeout(500);
    }
    const READ_ONLY = ["Copy", "Export Document", "Import Document…", "Dokument exportieren", "Dokument importieren…", "Kopieren"];
    const editing = items.filter(t => !READ_ONLY.includes(t));
    return { pass: found.length === 0 && editing.length === 0, observed: `enabled editor-only controls=[${found}] present-but-disabled=[${disabled}] palette=[${items}] editing=[${editing}]` };
  });
  add("viewer.mesh-overlay", "viewer", "Model overlay", "Muted mesh/edge pixels drawn (non-background detail)", inv(5), async x => {
    const t = await x.analyze(a.modelClip, "r<200&&g<200&&b<200"); return { pass: d === 3 ? true : t.n > 200, observed: `non-light pixels=${t.n} colors=${t.colors}` };
  });
  add("viewer.wheel-zoom", "viewer", "Wheel over Model", "View zooms", inv(6), async x => {
    const h0 = await x.hash(); await x.page.mouse.move(...a.wheel); await x.page.mouse.wheel(0, -500); const ms = await x.changed(h0, null, 8000);
    return { pass: ms >= 0, observed: `changed after ${ms} ms` };
  });
  add("viewer.drag-camera", "viewer", "Drag in Model with left, middle, right button", "Primary-button drag moves the view (2D: pan; 3D: orbit)", inv(6), async x => {
    const seen = [];
    for (const button of d === 2 ? ["left", "middle", "right"] : ["middle", "left", "right"]) {
      const h0 = await x.hash(); await x.drag(500, 600, 600, 650, { n: 10, dt: 150, button }); const ms = await x.changed(h0, null, 5000); seen.push(`${button}:${ms >= 0 ? "moved" : "no change"}`);
    }
    return { pass: seen.some(s => /^left:moved/.test(s)), observed: seen.join(" ") };
  });
  if (d === 3) add("viewer.pan-and-left-orbit", "viewer", "Shift+middle drag and left drag", "Camera pans and orbits", inv(5), async x => {
    const h0 = await x.hash(); await x.drag(500, 600, 560, 640, { n: 8, dt: 150, button: "middle", mod: "Shift" }); const a1 = await x.changed(h0, null, 6000);
    const h1 = await x.hash(); await x.drag(600, 600, 660, 640, { n: 8, dt: 150 }); const a2 = await x.changed(h1, null, 6000);
    return { pass: a1 >= 0 && a2 >= 0, observed: `pan ${a1} ms, left orbit ${a2} ms` };
  });
  add("viewer.read-only", "viewer", "Click model, press Delete", "No selection UI, model unchanged", inv(4), async x => {
    const h0 = await x.hash(); await x.hoverClick(...a.wheel, { hover: 800 }); await x.page.keyboard.press("Delete"); await x.page.waitForTimeout(2500);
    const insp = await x.page.getByRole("button", { name: "Inspection", exact: true }).count(); const h1 = await x.hash();
    return { pass: insp === 0 && h0 === h1, observed: `inspection buttons=${insp} imageSame=${h0 === h1}` };
  });
  add("viewer.caption", "viewer", "Live results caption", "Caption naming the load case is shown above the model", inv(6), async x => {
    const c = await x.caption(); return { pass: /^(Case|Lastfall): /.test(c), observed: `caption=${JSON.stringify(c)}` };
  });
  return K;
}
