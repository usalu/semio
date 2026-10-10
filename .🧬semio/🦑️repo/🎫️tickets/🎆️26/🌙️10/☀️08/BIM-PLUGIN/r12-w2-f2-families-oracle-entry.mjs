#!/usr/bin/env bun
/** 🔮️ Registers the third-party oracle of the family inference (`bim-1-numpy-expression-families`) in the oracle catalog of `s.bim.model@1`. Idempotent. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix)));
const plugins = sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))), "plugins");
const model = sub(sub(sub(plugins, "bim"), "artifacts"), "model");
const subsets = sub(sub(sub(model, "standards"), "1"), "subsets");
const S = join(subsets, readdirSync(subsets).find((n) => n.endsWith("any")));
const path = join(sub(S, "oracles"), readdirSync(sub(S, "oracles")).find((name) => name.endsWith(".json")));

const text = readFileSync(path, "utf8");
const crlf = text.includes("\r\n");
const doc = JSON.parse(text);
if (!doc.oracles.some((oracle) => oracle.id === "bim-1-numpy-expression-families")) {
  doc.oracles.push({
    id: "bim-1-numpy-expression-families",
    kind: "third-party-library",
    ecosystem: "python",
    package: "numpy",
    version: "2.5.0",
    packages: [
      { package: "shapely", version: "2.1.2", license: "BSD-3-Clause", role: "GEOS area and perimeter of the section polygons of the family solids and their validity", homepage: "https://shapely.readthedocs.io" },
      { package: "networkx", version: "3.6.1", license: "BSD-3-Clause", role: "strongly connected components and topological generations of the parameter dependency graph, inside the independent Python interpreter of the expression framework module", homepage: "https://networkx.org" },
    ],
    capabilities: ["bim-1-infer"],
    comparisonProfiles: ["floating-point-v1"],
    license: "BSD-3-Clause",
    testOnly: true,
    homepage: "https://numpy.org",
    rationale:
      "numpy rebuilds every solid of a parametric family as explicit facets (divergence-theorem volume, cross-product area, mitered sweeps, revolutions in rings) and shapely measures the section polygons, while the independent Python interpreter of the expression framework module (ast dimension checking, networkx dependency order, Decimal rounding), reached through a Python recursive-descent translator of the formulas, resolves every parameter. None of them has seen this repository's Rust. The oracle audits the closed forms (a prism is its section area times its height, a pipe is pi r^2 h, a revolution obeys Pappus' theorem) and the parametric laws (the order of the parameters changes nothing, widening a table by 0.1 m widens its top by 0.1 m x depth x thickness) before it answers.",
    engine: { family: "numpy-networkx-shapely", implementation: "numpy ndarray facets, networkx dependency graphs and shapely (GEOS) polygons over the CPython interpreter of the expression oracle", version: "2.5.0" },
    productionReachable: false,
    networkDuringExecution: false,
  });
  const out = JSON.stringify(doc, null, 2) + "\n";
  writeFileSync(path, crlf ? out.replaceAll("\n", "\r\n") : out);
  console.log("registered bim-1-numpy-expression-families");
} else {
  console.log("already registered");
}
