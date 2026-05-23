#!/usr/bin/env bun
const rendererPath = "c:/git/semio/spatial/js/renderer-r3f/index.tsx";
let r = await Bun.file(rendererPath).text();
r = r.replace(/\tExtensionViewService,\n/g, "");
r = r.replace(/\tlistExtensionViews,\n/g, "");
r = r.replace(
  `function createViewObjectSpatialPickTargets(
\tviews: ExtensionViewService,
\tmodel: Model,
\tactiveViewId: string,
): readonly SpatialPickTarget[] {
\tconst targets: SpatialPickTarget[] = [];
\tfor (const object of views.computeObjects(model, activeViewId)) {
\t\tconst points = viewObjectPickPoints(model, object);
\t\tconst point = geometryPointCentroid(points);
\t\tif (!point) continue;
\t\ttargets.push({
\t\t\tkind: "object",
\t\t\tid: String(object.id),
\t\t\tpoint,
\t\t\tpoints: points.length ? points : undefined,
\t\t});
\t}
\treturn targets;
}`,
  `function createModelObjectSpatialPickTargets(model: Model): readonly SpatialPickTarget[] {
\tconst targets: SpatialPickTarget[] = [];
\tfor (const row of Object.values(model.objects)) {
\t\tconst points = viewObjectPickPoints(model, {
\t\t\tid: row.id,
\t\t\ttypologyId: row.typologyId,
\t\t\tlabel: row.typologyId,
\t\t\tsourceObjectIds: [row.id],
\t\t});
\t\tconst point = geometryPointCentroid(points);
\t\tif (!point) continue;
\t\ttargets.push({
\t\t\tkind: "object",
\t\t\tid: String(row.id),
\t\t\tpoint,
\t\t\tpoints: points.length ? points : undefined,
\t\t});
\t}
\treturn targets;
}`,
);
r = r.replace(
  `export function createSpatialPickTargets(
\tgeometry: SpatialPickGeometry | null | undefined,
\tviews?: ExtensionViewService | null,
\tactiveViewId?: string | null,
): readonly SpatialPickTarget[] {`,
  `export function createSpatialPickTargets(
\tgeometry: SpatialPickGeometry | null | undefined,
\t_model?: Model | null,
\tactiveViewId?: string | null,
): readonly SpatialPickTarget[] {`,
);
r = r.replace(
  `\t} else if (views) {
\t\ttargets.push(...createViewObjectSpatialPickTargets(views, model, activeViewId));
\t}`,
  `\t} else if (activeViewId) {
\t\ttargets.push(...createModelObjectSpatialPickTargets(model));
\t}`,
);
r = r.replaceAll("readonly views?: ExtensionViewService | null", "readonly views?: null");
r = r.replace(
  `\tconst viewService = useMemo(() => viewsProp ?? ExtensionViewService.forKernel(kernel), [viewsProp, kernel]);
\tconst shippedViews = useMemo(() => listExtensionViews(), []);`,
  `\tconst viewService = viewsProp ?? null;`,
);
r = r.replaceAll("ExtensionViewService.forKernel", "(() => null) as unknown as typeof null");
await Bun.write(rendererPath, r);

const playPath = "c:/git/semio/spatial/js/renderer-r3f/play/main.tsx";
let p = await Bun.file(playPath).text();
p = p.replace(/\tExtensionViewService,\n/g, "");
p = p.replace(/readonly views: ExtensionViewService;\n/, "");
p = p.replace(/const views = useMemo\(\(\) => ExtensionViewService\.forKernel\(kernel as unknown as import\("@spatial\/js-core"\)\.SpatialKernel\), \[kernel\]\);/, "const views = null;");
await Bun.write(playPath, p);

for (const file of ["c:/git/semio/spatial/js/kernel-brepjs/index.ts", "c:/git/semio/spatial/js/query/index.ts", "c:/git/semio/spatial/js/machine-stately/index.ts"]) {
  let s = await Bun.file(file).text();
  s = s.replace(/\tExtensionViewService,\n/g, "");
  s = s.replace(/\ttype ExtensionViewService,\n/g, "");
  s = s.replaceAll("ExtensionViewService", "null");
  s = s.replaceAll("listExtensionViews", "listModelDefinitionManifests");
  await Bun.write(file, s);
}
console.log("patched siblings");
