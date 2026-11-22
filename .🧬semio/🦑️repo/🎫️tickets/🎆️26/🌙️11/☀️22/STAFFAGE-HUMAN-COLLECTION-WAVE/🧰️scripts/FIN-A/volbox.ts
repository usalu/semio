/** 📦️ Prints `{scale: [widthM, heightM, depthM]}` of the body volume of every authored scale of an asset file in its default pose, as the catalog-silhouette guard measures it: bun volbox.ts <asset.json> */
const S = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { worldSolids, bodyDimensions } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const asset = (await Bun.file(process.argv[2]!).json()).asset;
const out: Record<string, number[]> = {};
for (const scale of [50, 100, 200, 500]) {
  if (asset.volumes[String(scale)]?.status !== "authored") continue;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  out[String(scale)] = (bodyDimensions(worldSolids(asset, state.bones, scale)) as number[]).map((v) => +v.toFixed(4));
}
console.log(JSON.stringify(out));
