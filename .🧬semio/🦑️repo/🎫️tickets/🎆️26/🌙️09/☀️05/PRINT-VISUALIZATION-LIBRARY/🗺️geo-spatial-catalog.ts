#!/usr/bin/env bun
// 🗺️ Rewrites the GEO-SPATIAL catalogue entries (taxonomy sections 10, 26 tessellations, 27, 28, 35 maps,
// 🗺️ and the geometric layouts of 75/78) in 🖼️assets/🔣️viz-catalog.json, in place and atomically.
// 🗺️ Every entry names a family that semio-viz-geo-* actually registers and an option set unique inside its family.

import { readFileSync, writeFileSync, renameSync } from "node:fs";
import { join } from "node:path";

const CATALOG = join(
  "C:/git/semio/🧰️framework/🛍️products/📓️print/🖼️assets",
  "🔣️viz-catalog.json",
);

type Entry = {
  id: string;
  slug: string;
  title: { en: string; de: string };
  kind: string;
  namespace: string;
  family: string;
  options: Record<string, string | number | boolean>;
  data: string;
  covers: string[];
};

type Spec = [family: string, namespace: string, data: string, options: Record<string, unknown>];

//#region 🔖️Section10
const S10: Record<string, Spec> = {
  // Base maps
  "political-map": ["geo-basemap", "geo/map", "demo-geo-regions", { palette: "neutral", labels: true }],
  "physical-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "terrain" }],
  "topographic-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, grid: true, palette: "terrain", steps: 7 }],
  "street-map": ["geo-basemap", "geo/map", "demo-geo-regions", { hydrology: true, settlements: true, palette: "neutral", opacity: 0.5 }],
  "terrain-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, hydrology: true, palette: "terrain", steps: 5 }],
  "relief-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "terrain", opacity: 0.85, steps: 9 }],
  "bathymetric-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "bathymetric", steps: 6 }],
  "geological-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "diverging", labels: true, steps: 5 }],
  // Statistical maps
  "choropleth-map": ["geo-choropleth", "geo/choropleth", "demo-geo-regions", { scale: "quantize", steps: 5 }],
  "dasymetric-map": ["geo-choropleth", "geo/choropleth", "demo-geo-regions", { scale: "quantize", dasymetric: true, weight: 0.65, steps: 4 }],
  "graduated-symbol-map": ["geo-symbol", "geo/symbols", "demo-geo-cities", { sizing: "graduated", classes: 4, shape: "circle" }],
  "proportional-symbol-map": ["geo-symbol", "geo/symbols", "demo-geo-cities", { sizing: "area", shape: "circle", maxSize: 5 }],
  "bubble-map": ["geo-symbol", "geo/symbols", "demo-geo-cities", { sizing: "area", shape: "circle", maxSize: 7, opacity: 0.6 }],
  "dot-density-map": ["geo-dotdensity", "geo/symbols", "demo-geo-regions", { unit: 6, dotSize: 0.5, spread: 6 }],
  "dot-distribution-map": ["geo-dotdensity", "geo/symbols", "demo-geo-regions", { unit: 3, dotSize: 0.4, spread: 8 }],
  "heat-map": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "heat", bandwidth: 8, levels: 5 }],
  "spatial-density-map": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "density", bandwidth: 6, levels: 6 }],
  "hexbin-map": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "hex", radius: 5 }],
  "bivariate-choropleth": ["geo-choropleth", "geo/choropleth", "demo-geo-regions", { scale: "quantile", bivariate: true, palette: "diverging", steps: 3 }],
  "multivariate-choropleth": ["geo-choropleth", "geo/choropleth", "demo-geo-regions", { scale: "threshold", breaks: "15,30,45", bivariate: true, steps: 4 }],
  // Distortion maps
  "cartogram": ["geo-cartogram", "geo/choropleth", "demo-geo-regions", { kind: "dorling", maxSize: 6 }],
  "contiguous-cartogram": ["geo-cartogram", "geo/choropleth", "demo-geo-regions", { kind: "contiguous", iterations: 4 }],
  "non-contiguous-cartogram": ["geo-cartogram", "geo/choropleth", "demo-geo-regions", { kind: "non-contiguous" }],
  "dorling-cartogram": ["geo-cartogram", "geo/choropleth", "demo-geo-regions", { kind: "dorling", maxSize: 8, palette: "diverging" }],
  "demers-cartogram": ["geo-cartogram", "geo/choropleth", "demo-geo-regions", { kind: "demers", maxSize: 6 }],
  // Grid / tile maps
  "tile-grid-map": ["geo-tilegrid", "geo/choropleth", "demo-geo-regions", { tile: "square", columns: 2, cell: 9 }],
  "hex-tile-map": ["geo-tilegrid", "geo/choropleth", "demo-geo-regions", { tile: "hex", columns: 2, cell: 11 }],
  "square-tile-map": ["geo-tilegrid", "geo/choropleth", "demo-geo-regions", { tile: "square", columns: 4, cell: 8, labels: false }],
  "equal-area-grid-map": ["geo-tilegrid", "geo/choropleth", "demo-geo-regions", { tile: "equal-area", columns: 2, cell: 10 }],
  // Routes and movement
  "route-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "direct", stations: true }],
  "transit-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "octilinear", stations: true, routeWidth: 1.8 }],
  "metro-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "octilinear", stations: true, basemap: false, routeWidth: 2.2 }],
  "subway-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "octilinear", stations: true, basemap: false, routeWidth: 1.6, palette: "diverging" }],
  "railway-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "direct", stations: true, routeWidth: 1, palette: "neutral" }],
  "road-network-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "direct", routeWidth: 0.8, palette: "neutral" }],
  "flight-route-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "great-circle", samples: 24, projection: "orthographic" }],
  "shipping-route-map": ["geo-route", "geo/routes", "demo-geo-routes", { mode: "great-circle", samples: 16, palette: "bathymetric" }],
  "origin-destination-map": ["geo-flow", "geo/routes", "demo-geo-routes", { flow: "arrow", curvature: 0.2 }],
  "desire-line-map": ["geo-flow", "geo/routes", "demo-geo-routes", { flow: "desire", origins: true }],
  "migration-map": ["geo-flow", "geo/routes", "demo-geo-routes", { flow: "migration", curvature: 0.35 }],
  "flow-map": ["geo-flow", "geo/routes", "demo-geo-routes", { flow: "band", curvature: 0.25 }],
  // Continuous spatial fields
  "contour-map": ["geo-field", "geo/contours", "demo-grid", { levels: 5 }],
  "isoline-map": ["geo-field", "geo/contours", "demo-grid", { levels: 7, labels: true }],
  "isopleth-map": ["geo-field", "geo/contours", "demo-grid", { levels: 5, filled: true }],
  "isochrone-map": ["geo-field", "geo/contours", "demo-grid", { levels: 4, filled: true, palette: "diverging" }],
  "isodistance-map": ["geo-field", "geo/contours", "demo-grid", { levels: 6, palette: "neutral" }],
  "elevation-contours": ["geo-field", "geo/contours", "demo-grid", { levels: 8, labels: true, palette: "terrain" }],
  "weather-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 4, fronts: true }],
  "pressure-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 6, labels: true }],
  "temperature-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 5, raster: true, palette: "diverging" }],
  "precipitation-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 5, filled: true, palette: "bathymetric" }],
  "wind-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 3, barbs: true, barbScale: 4 }],
  // Terrain representations
  "hillshade": ["geo-terrain", "geo/contours", "demo-grid", { mode: "hillshade", azimuth: 315, altitude: 45 }],
  "hypsometric-tint": ["geo-terrain", "geo/contours", "demo-grid", { mode: "hypsometric", levels: 6 }],
  "shaded-relief": ["geo-terrain", "geo/contours", "demo-grid", { mode: "relief", azimuth: 300, altitude: 35 }],
  "elevation-raster": ["geo-terrain", "geo/contours", "demo-grid", { mode: "raster" }],
  "dem-visualization": ["geo-terrain", "geo/contours", "demo-grid", { mode: "raster", palette: "diverging", cell: 10 }],
  "terrain-profile": ["geo-terrain", "geo/contours", "demo-grid", { mode: "profile", row: 2 }],
  "cross-sectional-terrain-profile": ["geo-terrain", "geo/contours", "demo-grid", { mode: "cross-section", row: 3, zScale: 2 }],
  // Coordinate / cartographic structures
  "graticule": ["geo-graticule", "geo/projection", "demo-geo-regions", { step: "10,10" }],
  "latitude-longitude-grid": ["geo-graticule", "geo/projection", "demo-geo-regions", { step: "15,15", labels: true }],
  "map-projection-diagram": ["geo-graticule", "geo/projection", "demo-geo-regions", { projection: "orthographic", sphere: true, step: "20,20", labels: true }],
  "great-circle-paths": ["geo-geodesic", "geo/projection", "demo-geo-routes", { samples: 32 }],
  "geodesic-paths": ["geo-geodesic", "geo/projection", "demo-geo-routes", { samples: 24, tissot: true, radius: 5 }],
  "voronoi-map": ["spatial-tessellation", "geo/symbols", "demo-points", { kind: "voronoi", sites: true }],
  "delaunay-map": ["spatial-tessellation", "geo/symbols", "demo-points", { kind: "delaunay", sites: true }],
  // Specialised maps
  "electoral-map": ["geo-basemap", "geo/map", "demo-geo-regions", { palette: "diverging", labels: true, steps: 3 }],
  "constituency-map": ["geo-basemap", "geo/map", "demo-geo-regions", { palette: "neutral", labels: true, grid: true }],
  "campus-map": ["geo-basemap", "geo/map", "demo-geo-regions", { settlements: true, labels: true, palette: "neutral", width: 60 }],
  "floor-map": ["geo-basemap", "geo/map", "demo-geo-regions", { palette: "neutral", opacity: 0.55, labels: true, height: 38 }],
  "site-map": ["geo-basemap", "geo/map", "demo-geo-regions", { settlements: true, palette: "neutral", opacity: 0.7 }],
  "archaeological-map": ["geo-basemap", "geo/map", "demo-geo-regions", { settlements: true, hydrology: true, palette: "terrain", opacity: 0.6 }],
  "historical-map": ["geo-basemap", "geo/map", "demo-geo-regions", { hydrology: true, palette: "terrain", opacity: 0.45, labels: true }],
  "linguistic-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "diverging", steps: 4, opacity: 0.7 }],
  "epidemiological-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "sequential", steps: 6, settlements: true }],
  "ecological-range-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "terrain", steps: 3, opacity: 0.5 }],
};
//#endregion 🔖️Section10

//#region 🔖️Section26
const S26: Record<string, Spec> = {
  "voronoi-diagram": ["spatial-tessellation", "spatial", "demo-points", { kind: "voronoi", sites: true, extent: "0,0,72,48" }],
  "delaunay-triangulation": ["spatial-tessellation", "spatial", "demo-points-dense", { kind: "delaunay", sites: true }],
  // 26/convex-hull and 26/concave-hull are covered by the section 0 mark entries owned by SHAPES; see the status file.
  // 26/triangulation and 26/mesh are SCIENTIFIC's geometry meshes (family sci-mesh), not spatial tessellations.
};
//#endregion 🔖️Section26

//#region 🔖️Section27
const S27: Record<string, Spec> = {
  "quiver-plot": ["spatial-vector-field", "spatial", "demo-points", { render: "quiver", field: "shear" }],
  "vector-field": ["spatial-vector-field", "spatial", "demo-points", { render: "quiver", field: "source", arrow: 3 }],
  "direction-field": ["spatial-vector-field", "spatial", "demo-points", { render: "direction", field: "saddle" }],
  "streamline-plot": ["spatial-vector-field", "spatial", "demo-points", { render: "streamline", field: "vortex", seeds: 8 }],
  "streamplot": ["spatial-vector-field", "spatial", "demo-points", { render: "streamline", field: "shear", seeds: 12, steps: 32 }],
  "particle-flow-plot": ["spatial-vector-field", "spatial", "demo-points", { render: "particle", field: "vortex", seeds: 6 }],
  "pathline": ["spatial-vector-field", "spatial", "demo-points", { render: "streamline", field: "dipole", seeds: 4, dt: 0.2 }],
  "streakline": ["spatial-vector-field", "spatial", "demo-points", { render: "streakline", field: "vortex", seeds: 7 }],
  "lic-style-field-depiction": ["spatial-vector-field", "spatial", "demo-points", { render: "lic", field: "shear", seeds: 24, steps: 8 }],
  "gradient-field": ["spatial-vector-field", "spatial", "demo-points", { render: "gradient", field: "source", arrow: 5 }],
  "curl-field": ["spatial-vector-field", "spatial", "demo-points", { render: "curl", field: "vortex", arrow: 4 }],
  "divergence-field": ["spatial-vector-field", "spatial", "demo-points", { render: "divergence", field: "source", arrow: 4.5 }],
  "electric-field-diagram": ["spatial-vector-field", "spatial", "demo-points", { render: "streamline", field: "dipole", seeds: 10, dt: 0.15 }],
  "magnetic-field-diagram": ["spatial-vector-field", "spatial", "demo-points", { render: "streamline", field: "vortex", seeds: 10, dt: 0.5 }],
  "force-field-diagram": ["spatial-vector-field", "spatial", "demo-points", { render: "quiver", field: "dipole", arrow: 2.5 }],
  "velocity-field-diagram": ["spatial-vector-field", "spatial", "demo-points", { render: "quiver", field: "vortex", arrow: 3.5, columns: 11, rows: 7 }],
};
//#endregion 🔖️Section27

//#region 🔖️Section28
const S28: Record<string, Spec> = {
  "contour-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "contour", levels: 5 }],
  "filled-contour-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "filled", levels: 5 }],
  "isoline-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "contour", levels: 8, labels: true }],
  "level-set-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "levelset" }],
  "scalar-heatmap": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "heatmap" }],
  "density-map": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "density", bandwidth: 5, levels: 5 }],
  "surface-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "shaded", tilt: 30, turn: 30 }],
  "mesh-surface": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "mesh", tilt: 26, turn: 34 }],
  "wireframe": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "wireframe", tilt: 30, turn: 30 }],
  "height-map": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "heightmap", zScale: 2.4, palette: "terrain" }],
  "shaded-surface": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "shaded", tilt: 40, turn: 20, palette: "terrain" }],
  "waterfall-surface": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "waterfall", tilt: 22, turn: 28 }],
  "curtain-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "curtain", tilt: 22, turn: 28 }],
  "slice-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "slice", row: 2 }],
  "isosurface": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "levelset", palette: "diverging", cell: 14 }],
  "volume-rendering": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "filled", levels: 7, opacity: 0.5 }],
  "voxel-plot": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "heatmap", cell: 9, steps: 4 }],
};
//#endregion 🔖️Section28

//#region 🔖️Section35
const S35: Record<string, Spec> = {
  "weather-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 4, fronts: true, barbs: true }],
  "synoptic-chart": ["geo-weather", "geo/contours", "demo-grid", { levels: 6, fronts: true, labels: true }],
  "weather-fronts": ["geo-weather", "geo/contours", "demo-grid", { levels: 1, fronts: true }],
  "pressure-contours": ["geo-weather", "geo/contours", "demo-grid", { levels: 8, labels: true, palette: "neutral" }],
  "isobar-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 7, labels: true }],
  "isotherm-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 6, raster: true, palette: "diverging" }],
  "wind-barb-map": ["geo-weather", "geo/contours", "demo-grid", { levels: 2, barbs: true, barbScale: 5 }],
  "tectonic-map": ["geo-basemap", "geo/map", "demo-geo-regions", { relief: true, palette: "diverging", steps: 5, labels: true, grid: true }],
};
//#endregion 🔖️Section35

//#region 🔖️Layouts
const S75: Record<string, Spec> = {
  "voronoi": ["spatial-tessellation", "spatial", "demo-points", { kind: "voronoi", sites: false }],
  "delaunay": ["spatial-tessellation", "spatial", "demo-points", { kind: "delaunay", sites: false }],
  // 75/convex-hull is covered by 0/convex-hull (SHAPES); see the status file.
  "contour-generation": ["spatial-scalar-field", "spatial", "demo-grid", { surface: "contour", levels: 6 }],
  "density-estimation": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "density", bandwidth: 7, levels: 4 }],
  "geographic-projection": ["geo-graticule", "geo/projection", "demo-geo-regions", { projection: "natural-earth1", step: "30,30" }],
  "hexagonal-binning": ["geo-hexbin", "geo/symbols", "demo-points-dense", { render: "hex", radius: 6 }],
};

const S78: Record<string, Spec> = {
  // 78/projection, 78/voronoi, 78/delaunay and 78/contours are covered by 76/projection, 75/voronoi,
  // 75/delaunay and 76/contours; see the status file.
  "hull": ["spatial-tessellation", "spatial", "demo-points-dense", { kind: "hull", sites: true, palette: "neutral" }],
  "clip": ["spatial-tessellation", "spatial", "demo-points", { kind: "voronoi", sites: false, extent: "10,8,60,40" }],
  "interpolate": ["geo-geodesic", "geo/projection", "demo-geo-routes", { samples: 40, projection: "orthographic" }],
};
//#endregion 🔖️Layouts

const SECTIONS: Array<[string, Record<string, Spec>]> = [
  ["10", S10], ["26", S26], ["27", S27], ["28", S28], ["35", S35], ["75", S75], ["78", S78],
];

// 🔗 Families that consume a plane point set or a value grid read it from their own option, not from the catalogue
// 🔗 `data` field, so the source name is folded into the options; that also keeps two kinds on the same family
// 🔗 distinguishable when only their source differs.
const POINT_FAMILIES = new Set(["spatial-tessellation", "geo-hexbin", "spatial-vector-field"]);
const GRID_FAMILIES = new Set(["geo-field", "geo-terrain", "geo-weather", "spatial-scalar-field"]);
const GEOMETRY_FAMILIES = new Set(["geo-basemap", "geo-choropleth", "geo-cartogram", "geo-tilegrid", "geo-symbol", "geo-dotdensity", "geo-graticule", "geo-geodesic", "geo-route", "geo-flow"]);

function withSource(family: string, data: string, options: Record<string, unknown>) {
  if (POINT_FAMILIES.has(family)) return { points: data, ...options };
  if (GRID_FAMILIES.has(family)) return { grid: data, ...options };
  if (family === "geo-route" || family === "geo-flow") return { routes: data, ...options };
  if (family === "geo-symbol") return { symbols: data, ...options };
  if (family === "geo-geodesic") return { arcs: data, ...options };
  if (GEOMETRY_FAMILIES.has(family) && data.startsWith("demo-geo")) return { geometry: data, ...options };
  return { ...options };
}

//#region 🔖️Apply
const doc = JSON.parse(readFileSync(CATALOG, "utf8")) as { schemaVersion: number; kinds: Entry[] };
let touched = 0;
const missing: string[] = [];

for (const [section, table] of SECTIONS) {
  for (const [slug, [family, namespace, data, options]] of Object.entries(table)) {
    const id = `${section}/${slug}`;
    const entry = doc.kinds.find((k) => k.id === id);
    if (!entry) { missing.push(id); continue; }
    entry.family = family;
    entry.namespace = namespace;
    entry.data = data;
    entry.options = withSource(family, data, options) as Entry["options"];
    touched += 1;
  }
}

// 🔍 A family must not carry two entries with the same option string: identical options render identically.
const seen = new Map<string, string>();
const clashes: string[] = [];
for (const [section, table] of SECTIONS) {
  for (const [slug, [family, , , options]] of Object.entries(table)) {
    const key = `${family}|${JSON.stringify(Object.entries(withSource(family, table[slug][2], options)).sort())}`;
    if (seen.has(key)) clashes.push(`${section}/${slug} == ${seen.get(key)}`);
    else seen.set(key, `${section}/${slug}`);
  }
}

doc.kinds.sort((a, b) => {
  const sa = Number(a.id.split("/")[0]);
  const sb = Number(b.id.split("/")[0]);
  return sa - sb || a.slug.localeCompare(b.slug);
});

const tmp = `${CATALOG}.geo-spatial.tmp`;
writeFileSync(tmp, `${JSON.stringify(doc, null, 2)}\n`, "utf8");
renameSync(tmp, CATALOG);

console.log(`touched ${touched} entries`);
if (missing.length) console.log(`missing ids (not in catalogue): ${missing.join(", ")}`);
if (clashes.length) console.log(`OPTION CLASHES: ${clashes.join("; ")}`);
else console.log("no option clashes inside any family");
//#endregion 🔖️Apply
