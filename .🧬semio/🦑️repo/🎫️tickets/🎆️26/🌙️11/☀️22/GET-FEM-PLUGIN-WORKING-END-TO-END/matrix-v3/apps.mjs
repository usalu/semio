/** 🧭 App table of the v3 matrix: urls, ids, canvas coordinates and Demo entity names per FEM app (1600x1000 viewport). */
export const VIEWPORT = { width: 1600, height: 1000 };

export const APPS = {
  "2d": {
    key: "2d", dim: 2, kind: "editor", prefix: "A2D", inv: "E2D", url: "http://127.0.0.1:6086/", pfx: "fem2dModel",
    viewLabel: "Structure view", resultsLabel: "Results view", bootCaption: /Converged|Residual/, pending: /Pending/,
    groups: ["Nodes", "Elements", "Regions", "Supports", "Load Cases", "Combinations", "Materials", "Sections"],
    nodeA: { re: /^ridge\b/, at: [520, 402] }, nodeB: { re: /^p0_l2\b/, at: [380, 472] }, nodeC: { re: /^p8_l2\b/, at: [660, 472] },
    marquee: [380, 370, 660, 660], lasso: [[410, 370], [640, 370], [640, 520], [410, 520], [410, 370]], empty: [300, 300],
    modelClip: { x: 320, y: 60, width: 470, height: 860 }, resultsClip: { x: 800, y: 60, width: 800, height: 880 },
    resultsHover: [1200, 300], wheel: [520, 402],
    rows: { node: /^ridge\b/, node2: /^n2\b/, element: /^e5\b/, element2: /^e11\b/, region: /^r1\b/, support: /^s1\b/, loadCase: /^Dead Load\b/, loadNodal: /^Ty .* @ p8_l1/, loadArea: /^800 Pa @ r1/, loadAreaLive: /^1500 Pa @ r1/, combination: /^ULS\b/, material: / Material$/, section: / Section$/, analysis: /^Analysis$/ },
    actions: ["clearSelection", "selectAll", "setSelectionMode", "setGranularity", "addNode", "addBar", "addBeam", "addMaterial", "addSection", "addSupport", "addNodalLoad", "addMemberUdl", "addAreaLoad", "addRegion", "addLoadCase", "addCombination", "setSelfWeight", "setAnalysisSettings", "removeSelection", "setActiveExample", "setResultDisplay", "translateSelection", "rotateSelection", "scaleSelection", "copy", "cut", "paste", "exportDocument", "importDocument", "undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"],
    examples: ["Demo"],
    german: /Aktionen|Hilfsmittel|Inspektion|Einstellungen|Verlauf|Ergebnisse|Befehl/,
  },
  "3d": {
    key: "3d", dim: 3, kind: "editor", prefix: "A3D", inv: "E3D", url: "http://127.0.0.1:6087/", pfx: "fem3dModel",
    viewLabel: "Model view", resultsLabel: "Results view", bootCaption: /^Case: [^·]+$/m, pending: /Preparing|Pending/,
    groups: ["Nodes", "Elements", "Solids", "Supports", "Load Cases", "Combinations", "Materials", "Sections"],
    nodeA: { re: /^col_20_u\b/, at: [391, 540] }, nodeB: { re: /^col_20_l\b/, at: [391, 640] }, nodeC: { re: /^col_20_u\b/, at: [391, 540] },
    marquee: [330, 480, 520, 700], lasso: [[330, 480], [520, 480], [520, 700], [330, 700], [330, 480]], empty: [700, 120],
    modelClip: { x: 320, y: 60, width: 470, height: 860 }, resultsClip: { x: 830, y: 120, width: 740, height: 780 },
    resultsHover: [1200, 400], wheel: [560, 500],
    rows: { node: /^sc3\b/, node2: /^sc0\b/, element: /^col_20_l\b/, element2: /^col_20_u\b/, solid: /^sol1\b/, support: /^ss_0\b/, loadCase: /^Dead Load\b/, loadNodal: /@ n20_l1/, loadArea: /@ sol1/, combination: /^ULS\b/, material: / Material$/, section: / Section$/, analysis: /^Analysis$/ },
    actions: ["clearSelection", "selectAll", "setSelectionMode", "setGranularity", "addNode", "addBar", "addFrame", "addMaterial", "addSection", "addSupport", "addNodalLoad", "addMemberUdl", "addAreaLoad", "addSolid", "addLoadCase", "addCombination", "setSelfWeight", "setAnalysisSettings", "removeSelection", "setActiveExample", "setResultDisplay", "translateSelection", "rotateSelection", "scaleSelection", "copy", "cut", "paste", "exportDocument", "importDocument", "undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"],
    examples: ["Demo", "Concrete Forest", "House"],
    german: /Aktionen|Hilfsmittel|Inspektion|Einstellungen|Verlauf|Ergebnisse|Befehl|Lastfall/,
  },
  "viewer2d": {
    key: "viewer2d", dim: 2, kind: "viewer", prefix: "V2D", inv: "V2D", url: "http://127.0.0.1:6088/", viewLabel: "Structure view",
    modelClip: { x: 0, y: 60, width: 1600, height: 880 }, wheel: [800, 500], german: /Einstellungen|Befehl|Anzeige/,
  },
  "viewer3d": {
    key: "viewer3d", dim: 3, kind: "viewer", prefix: "V3D", inv: "V3D", url: "http://127.0.0.1:6089/", viewLabel: "Model view",
    modelClip: { x: 0, y: 60, width: 1600, height: 880 }, wheel: [800, 500], german: /Einstellungen|Befehl|Anzeige/,
  },
};
