/** 🏗️ Model checks of the v3 matrix: inspection edits per entity type, artifact tree, Actions palette (every action with valid args). */
import { mk } from "./checks-core.mjs";
import { sleep } from "./lib.mjs";

const SKIP = why => ({ pass: null, observed: `SKIP: ${why}` });

export function modelChecks(a) {
  const d = a.dim, K = [];
  const add = (...args) => K.push(mk(...args));
  const ids = (group, n = 0) => x => x.refId(group, n);
  const caseId = n => x => x.refId("Load Cases", n);
  const edit = (id, inv, row, group, label, keys, value, what) => add(`edit.${id}`, "inspection", `${what ?? row} > ${label}`, `Inspection "${label}" applies (readback equals the new value), Results re-solve in the background`, inv, async x => {
    if (value === undefined && label !== "Self Weight") {
      const s = await x.selectRow(a.rows[row] ?? row, { group }); const f = s.ok ? await x.field(label, keys) : null;
      if (!f) return { pass: false, observed: `field ${label} missing` };
      if (f.role === "combobox") { const o = await x.options(f.loc); await x.page.keyboard.press("Escape"); if (o.length < 2) return SKIP(`only ${o.length} option(s) for ${label}`); }
    }
    return x.editField(a.rows[row] ?? row, label, { keys, value, group });
  }, { timeout: 400000 });

  edit("node-x", { 2: [56], 3: [56] }, "node", "Nodes", "X", ["node.x"], 5.5);
  edit("node-y", { 2: [57], 3: [56] }, "node", "Nodes", "Y", ["node.y"], 2.5);
  if (d === 3) edit("node-z", { 3: [56] }, "node", "Nodes", "Z", ["node.z"], 0.5);
  if (d === 3) edit("element-roll", { 3: [58] }, "element", "Elements", "Roll", ["element.roll"], 0.5);
  edit("element-kind", { 2: [58], 3: [57] }, "element", "Elements", "Kind", ["element.kind"]);
  edit("element-start", { 2: [59], 3: [59] }, "element", "Elements", "Start", ["element.start"]);
  edit("element-end", { 2: [59], 3: [59] }, "element", "Elements", "End", ["element.end"]);
  edit("element-material", { 2: [60], 3: [60] }, "element", "Elements", "Material", ["element.material"]);
  edit("element-section", { 2: [61], 3: [61] }, "element", "Elements", "Section", ["element.section"]);
  edit("material-name", { 2: [62], 3: [62] }, "material", "Materials", "Name", ["material.name"], "Steel S355 v3");
  edit("material-e", { 2: [63], 3: [63] }, "material", "Materials", "Young's Modulus", ["material.e"], 210000000000);
  if (d === 3) edit("material-g", { 3: [64] }, "material", "Materials", "Shear Modulus", ["material.g"], 80000000000);
  edit("material-nu", { 2: [64], 3: [65] }, "material", "Materials", "Poisson Ratio", ["material.nu"], 0.25);
  edit("material-rho", { 2: [65], 3: [66] }, "material", "Materials", "Density", ["material.rho"], 7800);
  edit("section-name", { 2: [66], 3: [67] }, "section", "Sections", "Name", ["section.name"], "HEA 220 v3");
  edit("section-area", { 2: [67], 3: [68] }, "section", "Sections", "Area", ["section.area"], 0.007);
  if (d === 2) edit("section-i", { 2: [68] }, "section", "Sections", "Second Moment of Area", ["section.i", "section.iy"], 0.00002);
  if (d === 3) { edit("section-iy", { 3: [69] }, "section", "Sections", "Second Moment Iy", ["section.iy"], 0.00003); edit("section-iz", { 3: [70] }, "section", "Sections", "Second Moment Iz", ["section.iz"], 0.00004); edit("section-j", { 3: [71] }, "section", "Sections", "Torsion Constant", ["section.j"], 0.00005); }
  edit("support-node", { 2: [69], 3: [72] }, "support", "Supports", "Node", ["support.node", "support.node-id"]);
  for (const dof of d === 2 ? ["Tx", "Ty", "Rz"] : ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]) edit(`support-${dof.toLowerCase()}`, { 2: [dof === "Tx" ? 70 : 71], 3: [73] }, "support", "Supports", dof, [`support.${dof.toLowerCase()}`], undefined);
  if (d === 2) {
    edit("region-name", { 2: [72] }, "region", "Regions", "Name", ["region.name"], "Slab v3");
    edit("region-thickness", { 2: [73] }, "region", "Regions", "Thickness", ["region.thickness"], 0.22);
    edit("region-mesh", { 2: [74] }, "region", "Regions", "Mesh Size", ["region.mesh-size"], 1.5);
    edit("region-material", { 2: [75] }, "region", "Regions", "Material", ["region.material"]);
  } else {
    edit("solid-name", { 3: [74] }, "solid", "Solids", "Name", ["solid.name"], "Slab v3");
    edit("solid-axis", { 3: [75] }, "solid", "Solids", "Axis", ["solid.axis"]);
    edit("solid-base", { 3: [76] }, "solid", "Solids", "Base", ["solid.base-z"], 0.1);
    edit("solid-height", { 3: [77] }, "solid", "Solids", "Height", ["solid.height"], 0.4);
    edit("solid-layers", { 3: [78] }, "solid", "Solids", "Layers", ["solid.layers"], 2);
    edit("solid-mesh", { 3: [79] }, "solid", "Solids", "Mesh Size", ["solid.mesh-size"], 0.8);
    edit("solid-material", { 3: [80] }, "solid", "Solids", "Material", ["solid.material"]);
    add("inspect.solid-readonly", "inspection", "Solid Outline and Holes rows", "Read-only values listed", { 3: [81] }, async x => {
      await x.selectRow(a.rows.solid, { group: "Solids" }); const t = await x.insp();
      return { pass: /Outline/i.test(t) && /Holes/i.test(t), observed: t.slice(0, 200) };
    }, { fresh: true });
  }
  edit("load-nodal-value", { 2: [76], 3: [82] }, "loadNodal", "Load Cases", "Value", ["load.value"], 7000);
  if (d === 3) { edit("load-nodal-dof", { 3: [83] }, "loadNodal", "Load Cases", "Degree of Freedom", ["load.dof"]); edit("load-nodal-node", { 3: [83] }, "loadNodal", "Load Cases", "Node", ["load.node-id"]); }
  edit("load-area-pressure", { 2: [78], 3: [86] }, "loadArea", "Load Cases", "Pressure", ["load.pressure", "load.value"], 1800);
  const MEMBER_ROW = d === 2 ? /N\/m|UDL/i : /N\/m @/;
  for (const w of d === 2 ? ["Value"] : ["Wx", "Wy", "Wz"]) add(`edit.load-member-${w.toLowerCase()}`, "inspection", `Member load > ${w}`, `Member load ${w} applies after adding a member UDL by action`, { 2: [77], 3: [w === "Wz" ? 84 : 85] }, async x => {
    await x.workbench(); await x.expandGroup("Load Cases");
    if (!(await x.findRow(MEMBER_ROW)) && !(await x.openCases() && await x.findRow(MEMBER_ROW))) {
      const r = await x.runAction("addMemberUdl", d === 2 ? { "elementId|element|member": a.memberTarget, "w|value|q": -1500, "caseId|case": a.memberCase } : { elementId: a.memberTarget, wx: 100, wy: 200, wz: -1500, caseId: a.memberCase });
      if (!r.pass) return { pass: false, observed: `precondition addMemberUdl failed: ${r.observed}` };
      const ms = await x.until(async () => !!(await x.findRow(MEMBER_ROW)) || (await x.openCases() && !!(await x.findRow(MEMBER_ROW))), 30000, 800);
      if (ms < 0) return { pass: false, observed: `precondition: member load row never appeared after addMemberUdl (${r.observed})` };
    }
    return x.editField(MEMBER_ROW, w, { keys: ["load.udl", "load.w", `load.${w.toLowerCase()}`, "load.value"], value: -2200, group: "Load Cases" });
  }, { timeout: 200000 });
  edit("loadcase-name", { 2: [79], 3: [87] }, "loadCase", "Load Cases", "Name", ["load-case.name"], "Dead Load v3");
  edit("loadcase-selfweight", { 2: [80], 3: [88] }, "loadCase", "Load Cases", "Self Weight", ["load-case.self-weight"], undefined);
  edit("combination-name", { 2: [81], 3: [90] }, "combination", "Combinations", "Name", ["combination.name"], "ULS v3");
  edit("combination-term", { 2: [82], 3: [91] }, "combination", "Combinations", /dead|live/i, ["combination.term.dead", "combination.term.live"], 1.4);
  const terms = x => x.inspTree().getByRole("spinbutton").count();
  const pickTerm = async (x, label) => { const f = await x.field(label, []); if (!f) return { ok: false, observed: `no "${label}" control` }; const r = await x.chooseOption(f.loc, undefined, { other: "" }); return r.ok ? { ok: true, picked: r.picked } : { ok: false, observed: `"${label}" has no options (${r.options.length})` }; };
  if (d === 3) add("edit.combination-delete-term", "inspection", "Combination > Delete Term", "A term row is removed", { 3: [93] }, async x => {
    await x.selectRow(a.rows.combination, { group: "Combinations" }); const n0 = await terms(x);
    const r = await pickTerm(x, "Delete Term"); if (!r.ok) return { pass: false, observed: r.observed };
    const ms = await x.until(async () => (await terms(x)) < n0, 25000, 500);
    return { pass: ms >= 0, observed: `Delete Term ${r.picked}: term inputs ${n0} -> ${await terms(x)} (${ms} ms)` };
  }, { timeout: 90000, fresh: true });
  add("edit.combination-add-term", "inspection", "Combination > Add Term", "A term row is added (the select lists the cases the combination does not yet superpose)", { 2: [83], 3: [92] }, async x => {
    await x.selectRow(a.rows.combination, { group: "Combinations" }); let n0 = await terms(x); let pre = "";
    const probe = await x.field(d === 2 ? /add term/i : "Add Term", []); if (!probe) return { pass: false, observed: "no Add Term control" };
    const opts = await x.options(probe.loc); await x.page.keyboard.press("Escape");
    if (!opts.length) {
      let r; if (d === 3) r = await pickTerm(x, "Delete Term"); else { const f = await x.field(/dead|live/i, []); if (f) { await x.setField(f, 0); r = { ok: true, picked: "factor 0" }; } else r = { ok: false, observed: "no term factor field" }; }
      if (!r.ok) return { pass: false, observed: `precondition: ${r.observed}` };
      await x.until(async () => (await terms(x)) < n0, 25000, 500); n0 = await terms(x); pre = `after dropping ${r.picked}: `;
    }
    const r = await pickTerm(x, d === 2 ? /add term/i : "Add Term"); if (!r.ok) return { pass: false, observed: r.observed };
    const ms = await x.until(async () => (await terms(x)) > n0, 25000, 500);
    return { pass: ms >= 0, observed: `${pre}Add Term ${r.picked}: term inputs ${n0} -> ${await terms(x)} (${ms} ms)` };
  }, { timeout: 90000 });

  if (d === 3) add("inspect.loadcase-pick-load", "inspection", "Load Case inspection: click an entry of the Loads list", "Selection switches to that load (load form shown)", { 3: [89] }, async x => {
    await x.selectRow(a.rows.loadCase, { group: "Load Cases" }); const before = await x.insp();
    const b = x.inspTree().getByRole("button", { name: /^l\d+ · / }).first(); if (!(await b.count())) return { pass: false, observed: `no load pick buttons; inspection="${before.slice(0, 120)}"` };
    await b.click(); const ms = await x.until(async () => /Value|Pressure|Wx|Wy|Wz/.test(await x.insp()) && (await x.insp()) !== before, 15000, 400);
    return { pass: ms >= 0, observed: `inspection now "${(await x.insp()).slice(0, 120)}" (${ms} ms)` };
  });
  add("tree.groups-expand", "tree", "Expand Materials, Sections, Analysis groups", "Child rows shown", { 2: [86], 3: [125] }, async x => {
    await x.panel("Artifact", true); const res = [];
    for (const [g, re] of [["Materials", / Material$/], ["Sections", / Section$/]]) { await x.expandGroup(g); await sleep(900); res.push(`${g}=${(await x.rowLoc(re).count())}`); }
    return { pass: res.every(s => !/=0$/.test(s)), observed: res.join(" ") };
}, { fresh: true });
  add("tree.loadcase-children", "tree", "Load Cases children", "Loads listed under cases within 8 s", { 2: [85], 3: [126] }, async x => {
    await x.panel("Artifact", true); await x.expandGroup("Load Cases");
    const ms = await x.until(async () => (await x.rowLoc(/ Load$/).count()) > 0, 8000, 500);
    return { pass: ms >= 0, observed: `loads rows=${await x.rowLoc(/ Load$/).count()} (${ms} ms)` };
  });
  const SEL = [["node", "Nodes", /\bX\b/], ["element", "Elements", /Kind/], [d === 2 ? "region" : "solid", d === 2 ? "Regions" : "Solids", d === 2 ? /Thickness/ : /Axis/], ["support", "Supports", /Tx/], ["loadCase", "Load Cases", /Self Weight/], [d === 2 ? "loadNodal" : "loadNodal", "Load Cases", /Value|Pressure|wz/], ["combination", "Combinations", /Name/], ["material", "Materials", /Young/], ["section", "Sections", /Area/]];
  for (const [k, g, re] of SEL) add(`tree.select-${k}`, "tree", `Click ${k} row`, "Inspection shows that entity's form", { 2: [44], 3: [127] }, async x => {
    const r = await x.selectRow(a.rows[k], { group: g }); const t = await x.insp();
    return { pass: r.ok && re.test(t), observed: `ok=${r.ok} inspection="${t.slice(0, 150)}"` };
  });
  add("tree.analysis-row", "tree", "Click Analysis row", d === 2 ? "Inspection switches to analysis" : "Inspection unchanged (read-only)", { 2: [45], 3: [128] }, async x => {
    await x.selectRow(a.rows.node, { group: "Nodes" }); const before = await x.insp(); await x.panel("Artifact", true);
    const row = await x.findRow(a.rows.analysis); if (!row) return { pass: false, observed: "no Analysis row" };
    await row.click(); await sleep(2500); const after = await x.insp();
    return { pass: d === 2 ? after !== before : after === before, observed: `before="${before.slice(0, 60)}" after="${after.slice(0, 60)}"` };
  });
  add("tree.section-label", "tree", "View Section rows", "Row label free of icon text overlap (no literal 'ruler' or 'ile:')", { 2: [46], 3: [129] }, async x => {
    await x.expandGroup("Sections"); const t = await x.rowLoc(/ Section$/).first().innerText().catch(() => "");
    return { pass: !!t && !/ruler|ile:/.test(t), observed: JSON.stringify(t) };
  });
  add("tree.poisson-visible", "inspection", "Material Poisson Ratio value", "Value fully visible (not clipped)", { 2: [], 3: [130] }, async x => {
    await x.selectRow(a.rows.material, { group: "Materials" }); const f = await x.field("Poisson Ratio", ["material.nu"]); if (!f) return { pass: false, observed: "field missing" };
    const c = await f.loc.evaluate(e => ({ sw: e.scrollWidth, cw: e.clientWidth, v: e.value ?? e.textContent }));
    return { pass: c.sw <= c.cw + 1, observed: JSON.stringify(c) };
  });

  const cnt = (x, g) => ({ pre: async () => x.groupCount(g), post: async p => (await x.groupCount(g)) === p + 1 });
  const loadKeys = async x => {
    await x.expandGroup("Load Cases"); await x.openCases(); const keys = new Set();
    for (const dy of [-9000, 700, 700, 700, 700, 700, 700]) { await x.scrollTree(dy); const ls = (await x.artTree().innerText().catch(() => "")).split(String.fromCharCode(10)).map(l => l.trim()); ls.forEach((l, i) => { if (/ Load$/.test(l) && !/Case$/.test(l)) keys.add(`${ls[i - 1] ?? ""}|${l}`); }); }
    return keys;
  };
  const newLoad = async x => { const pre = await loadKeys(x); return { pre: async () => pre, post: async p => [...(await loadKeys(x))].some(k => !p.has(k)) }; };
  const uniq = () => 1000 + Math.floor(Math.random() * 8000);
  const act = (id, label, args, inv, expected, verifyFn, o = {}) => add(`act.${id}`, "actions", `Palette ${label}`, expected, inv, async x => {
    await x.panel("Artifact", true); const v = await verifyFn(x); const r = await x.runAction(id, args, { verify: v, timeout: o.vt ?? 70000 }); return r;
  }, { timeout: 200000, ...o });
  const n = (dim2, dim3) => ({ 2: dim2 === null ? [] : [dim2], 3: dim3 === null ? [] : [dim3] });

  act("addNode", "Add Node", { x: 2, y: 2, z: 0 }, n(87, 94), "Nodes +1", async x => cnt(x, "Nodes"));
  act(d === 2 ? "addBar" : "addBar", "Add Bar", { "start|from|node1|startNode": ids("Nodes", 0), "end|to|node2|endNode": ids("Nodes", 1), "materialId|material": ids("Materials"), "sectionId|section": ids("Sections") }, n(88, 95), "Elements +1", async x => cnt(x, "Elements"));
  act(d === 2 ? "addBeam" : "addFrame", d === 2 ? "Add Beam" : "Add Frame", { "start|from|node1|startNode": ids("Nodes", 0), "end|to|node2|endNode": ids("Nodes", 2), "materialId|material": ids("Materials"), "sectionId|section": ids("Sections"), roll: 0.2 }, n(89, 96), "Elements +1", async x => cnt(x, "Elements"));
  act("addMaterial", "Add Material", { name: "Mat v3", e: 210000000000, "g|shear": 80000000000, nu: 0.3, rho: 7800 }, n(90, 97), "Materials +1", async x => cnt(x, "Materials"), { vt: 120000 });
  act("addSection", "Add Section", { name: "Sec v3", area: 0.01, "iy|i|inertia": 0.00001, iz: 0.00001, j: 0.00001 }, n(91, 98), "Sections +1", async x => cnt(x, "Sections"), { vt: 120000 });
  act("addSupport", "Add Support", { "nodeId|node": ids("Nodes", 3), "fixed|dofs|restraints": d === 2 ? "tx,ty" : "tx,ty,tz" }, n(92, 99), "Supports +1", async x => cnt(x, "Supports"));
  act("addNodalLoad", "Add Nodal Load", { "nodeId|node": ids("Nodes", 1), "dof|direction": d === 2 ? "Ty" : "Tx", "value|force|magnitude": uniq() }, n(93, 100), "A new nodal load row appears (all cases expanded)", newLoad);
  act("addMemberUdl", "Add Member UDL", { "elementId|member|element": ids("Elements"), "wz|w|wy|udl|value|q": -uniq() }, n(94, 101), "A new member load row appears (all cases expanded)", newLoad);
  act("addAreaLoad", "Add Area Load", { "solidId|regionId|solid|region": ids(d === 2 ? "Regions" : "Solids"), "pressure|value": uniq() }, n(95, 102), "A new area load row appears (all cases expanded)", newLoad);
  if (d === 2) act("addRegion", "Add Region", { name: "Region v3", "materialId|material": ids("Materials"), thickness: 0.2, "meshSize|mesh": 1, "outline|points|polygon|vertices": "0,0 4,0 4,4 0,4" }, n(96, null), "Regions +1", async x => cnt(x, "Regions"), { vt: 120000 });
  else {
    act("addSolid", "Add Solid", { x: 20, y: 0, "width|dx|sizeX|lx": 3, "depth|dy|sizeY|ly": 3, "height|thickness|dz": 0.3, "materialId|material": ids("Materials") }, n(null, 103), "Solids +1", async x => cnt(x, "Solids"), { vt: 120000 });
    add("act.addSolid-unknown-material", "actions", "Palette Add Solid with unknown material c30", "A localized notice names 'c30' and no solid is added", n(null, 104), async x => {
      await x.panel("Artifact", true); const g0 = await x.groupCount("Solids");
      const r = await x.runAction("addSolid", { "materialId|material": "c30", x: 30, y: 0, "width|dx": 3, "depth|dy": 3, "height|dz": 0.3 }, { verify: { pre: async () => 0, post: async () => (await x.alerts()).some(t => /c30/.test(t)) }, timeout: 20000 });
      return { pass: r.pass && (await x.groupCount("Solids")) === g0, observed: `${r.observed}; alerts=${JSON.stringify(await x.alerts())}` };
    }, { timeout: 400000 });
  }
  add("act.unknown-reference-refusals", "actions", "Palette actions naming an id the document lacks", "Each refusal shows a localized notice quoting the id and nothing is added", n(null, null), async x => {
    await x.panel("Artifact", true);
    const first = ids("Nodes", 0), material = ids("Materials"), section = ids("Sections");
    const rows = [
      ["addSupport", { "nodeId|node": "ghost" }],
      ["addNodalLoad", { "nodeId|node": "ghost", "value|force|magnitude": 5 }],
      ["addMemberUdl", { "elementId|member|element": "ghost", "wz|w|wy|udl|value|q": 1 }],
      ["addAreaLoad", { "solidId|regionId|solid|region": "ghost", "pressure|value": 3 }],
      ["addBar", { "start|from|node1|startNode": first, "end|to|node2|endNode": "ghost", "materialId|material": material, "sectionId|section": section }],
      ["addBar", { "start|from|node1|startNode": first, "end|to|node2|endNode": first, "materialId|material": material, "sectionId|section": section }],
    ];
    const counts = async () => JSON.stringify([await x.groupCount("Supports"), await x.groupCount("Elements"), (await loadKeys(x)).size]);
    const before = await counts(), seen = [];
    for (const [id, args] of rows) {
      await x.until(async () => (await x.alerts()).length === 0, 20000, 300);
      const ours = t => /ghost|different nodes/i.test(t);
      const r = await x.runAction(id, args, { verify: { pre: async () => 0, post: async () => (await x.alerts()).some(ours) }, timeout: 20000 });
      const alert = (await x.alerts()).find(ours) ?? null; seen.push(`${id}: ${alert ? alert.slice(0, 90) : r.observed}`);
      await x.until(async () => (await x.alerts()).length === 0, 8000, 300);
    }
    const after = await counts();
    const refused = seen.every(t => /ghost|different nodes|keine?n? ?|two/i.test(t) && !/no effect/.test(t));
    return { pass: refused && before === after, observed: `counts ${before} -> ${after}; ${seen.join(" || ")}` };
  }, { timeout: 240000 });
  act("addLoadCase", "Add Load Case", { name: "Wind v3", "selfWeight|self": false }, n(97, 105), "Load Cases +1", async x => cnt(x, "Load Cases"));
  act("addCombination", "Add Combination", { name: "SLS v3", "terms|term|factors": "dead:1,live:1" }, n(98, 106), "Combinations +1", async x => cnt(x, "Combinations"));
  add("act.setSelfWeight", "actions", "Palette Set Self Weight", "The second load case's Self Weight tag flips in the tree, from whatever state it has", n(99, 107), async x => {
    await x.panel("Artifact", true); const id = await x.refId("Load Cases", 1); await x.panel("Artifact", true); await x.expandGroup("Load Cases");
    const tag = async () => /Self Weight/.test(((await x.rowLoc(/ Load Case$/).allInnerTexts()).map(t => t.trim()).filter(Boolean))[1] ?? "");
    const was = await tag(); const wants = !was;
    const r = await x.runAction("setSelfWeight", { "caseId|case|loadCase": id, "enabled|selfWeight|value|on": wants }, { verify: { pre: async () => 0, post: async () => (await tag()) === wants }, timeout: 70000 });
    return { pass: r.pass, observed: `case ${id} tag ${was} -> ${await tag()} (wanted ${wants}); ${r.observed}` };
  }, { timeout: 200000 });
  act("setAnalysisSettings", "Set Analysis Settings", { "modalCount|modal": 4, "bucklingCount|buckling": 2, "deformationScale|deformation": 120 }, n(100, 108), "Results panel Modal Count = 4", async x => ({ pre: async () => { await x.panel("Results", true); return 0; }, post: async () => { const f = x.page.getByRole("spinbutton", { name: "Modal Count", exact: true }).first(); return (await f.count()) && (await f.inputValue()) === "4"; } }));
  const selected = async (x, ok, ms = 40000) => { const t = await x.until(async () => ok(await x.selection()), ms, 500); return { ms: t, s: await x.selection() }; };
  add("act.clearSelection", "actions", "Palette Clear Selection", "A selected node is deselected: Inspection falls back to the Summary", { 2: [101], 3: [27] }, async x => {
    await x.selectRow(a.rows.node, { group: "Nodes" }); const s0 = await selected(x, s => !s.summary && s.n >= 1);
    const r = await x.runAction("clearSelection", {}); const s1 = await selected(x, s => s.summary);
    return { pass: s0.ms >= 0 && r.pass && s1.ms >= 0, observed: `${r.observed}; before=${JSON.stringify(s0.s.id ?? s0.s.text)}; cleared after ${s1.ms} ms selection=${JSON.stringify(s1.s)}` };
  }, { timeout: 200000 });
  add("act.selectAll", "actions", "Palette Select All", "Every element is selected: N SELECTED equals the Elements count of the Summary", { 2: [102], 3: [26, 122] }, async x => {
    const c = await x.counts(); const r = await x.runAction("selectAll", {}); const s = await selected(x, s => s.n > 1);
    return { pass: r.pass && s.ms >= 0 && s.s.n === c?.Elements, observed: `${r.observed}; elements=${c?.Elements}; selected after ${s.ms} ms ${JSON.stringify(s.s)}` };
  }, { timeout: 200000 });
  add("act.setSelectionMode", "actions", "Palette Set Selection Mode (Single, then Multiple)", "Single: Shift+click still leaves exactly 1 selected; Multiple: Shift+click grows the selection to 2 or more", { 2: [103], 3: [31] }, async x => {
    await x.toSummary(); const dom = d === 2 ? "fem2d" : "fem3d";
    const rs = await x.runAction("setSelectionMode", { domainId: dom, mode: "Single" });
    const s1 = await x.pickAt(a.nodeA); const s2 = await x.pickAt(a.nodeB, { mod: "Shift", differs: true }); const one = await selected(x, s => s.n === 1, 20000);
    const rm = await x.runAction("setSelectionMode", { domainId: dom, mode: "Multiple" }); await x.toSummary();
    const m1 = await x.pickAt(a.nodeA); const m2 = await x.pickAt(a.nodeB, { mod: "Shift", differs: true }); const many = await selected(x, s => s.n >= 2, 20000);
    return { pass: rs.pass && rm.pass && s1.ok && s2.ok && one.ms >= 0 && m1.ok && m2.ok && many.ms >= 0, observed: `single: ${rs.observed}; picks ${s1.ok}/${s2.ok} -> ${JSON.stringify(one.s)} | multiple: ${rm.observed}; picks ${m1.ok}/${m2.ok} -> ${JSON.stringify(many.s)}` };
  }, { timeout: 300000 });
  add("act.setGranularity", "actions", "Palette Set Granularity", "Executes; next canvas pick still selects an entity", { 2: [104], 3: [32] }, async x => {
    await x.toSummary(); const r = await x.runAction("setGranularity", { "domain": d === 2 ? "fem2d" : "fem3d", "granularity": "" }); const p = await x.pickAt(a.nodeA);
    return { pass: r.pass && p.ok, observed: `${r.observed}; pick=${p.ok}` };
  });
  const xform = (id, label, args, inv) => add(`act.${id}`, "actions", `Palette ${label}`, "Selected entity changes: Model image changes and History gains a command", inv, async x => {
    await x.selectRow(a.rows.element, { group: "Elements" }); await x.panel("History", false); const h0 = await x.hash(); const n0 = (await x.commands()).length; await x.panel("History", false);
    const r = await x.runAction(id, args); const ms = await x.until(async () => (await x.commands()).length > n0, 20000, 600); await x.panel("History", false); const h1 = await x.hash();
    return { pass: r.pass && ms >= 0 && h0 !== h1, observed: `${r.observed}; commands ${n0}->${(await x.commands()).length} imageChanged=${h0 !== h1}` };
  }, { timeout: 200000 });
  xform("translateSelection", "Translate Selection", { dx: 0.5, dy: 0, dz: 0 }, n(106, 110));
  xform("rotateSelection", "Rotate Selection", { axis: "Z", angle: 3 }, n(106, 111));
  xform("scaleSelection", "Scale Selection", { "sx|factor|scale": 1.1, sy: 1.1, sz: 1.1 }, n(106, 112));
  add("act.setResultDisplay", "actions", "Results window palette: Set Result Display (second load case)", "The action is declared by the Results window, not Model; Source shows the second load case and the Results image changes", { 2: [108], 3: [113] }, async x => {
    const id = await x.refId("Load Cases", 1); await x.panel("Results", true); const source = async () => (await x.page.getByRole("combobox", { name: "Source", exact: true }).first().textContent().catch(() => "?"))?.trim();
    const name = await source(); await x.openPalette("model"); const inModel = await x.modelPanel().getByRole("button", { name: x.actionLabel("setResultDisplay") }).count();
    const h0 = await x.hash(a.resultsClip);
    const r = await x.runAction("setResultDisplay", { "sourceId|source": id }, { win: "results", verify: { pre: async () => 0, post: async () => (await source()) !== name }, timeout: 60000 });
    const src = await source(); const ms = await x.changed(h0, null, 40000, a.resultsClip);
    return { pass: inModel === 0 && r.pass && src !== name && ms >= 0, observed: `modelPaletteHasIt=${inModel}; ${r.observed}; Source "${name}" -> "${src}" (id ${id}); imageChanged after ${ms} ms caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 200000 });
  const other = d === 2 ? "Demo" : "House", home = "Demo";
  add("act.setActiveExample", "actions", `Palette Set Active Example (${other}, then ${home})`, `Only declared examples are chosen (${d === 2 ? "No example, Demo" : "No example, Demo, Concrete Forest, House"}, the same rows as the Artifact panel combobox): the model is replaced, then Demo is restored`, { 2: [109], 3: [114] }, async x => {
    const c0 = await x.counts(); const same = async () => JSON.stringify(await x.counts()) === JSON.stringify(c0);
    const r0 = await x.runAction("setActiveExample", { "example|exampleId|id": other }, { verify: { pre: async () => 0, post: async () => { await sleep(5000); return d === 2 || !(await same()); } }, timeout: 400000 });
    const c1 = await x.counts();
    const r1 = await x.runAction("setActiveExample", { "example|exampleId|id": home }, { verify: { pre: async () => 0, post: async () => { await sleep(5000); return same(); } }, timeout: 1000000 });
    return { pass: r0.pass && r1.pass && (await x.solved(600000)) >= 0, observed: `${other}: ${r0.observed} counts ${JSON.stringify(c0)} -> ${JSON.stringify(c1)} | ${home}: ${r1.observed}` };
  }, { timeout: 1800000, slow: d === 3 });
  return K;
}
