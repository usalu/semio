const range = (a, b) => [[a, b]];
const cfg = {
  extraExports: {},
  modules: [
    { name: "records", folder: "🔷️brep-records", emoji: "🔷️", title: "Kernel brep records", doc: ["The kernel geometry record and reference aliases of `kernelGeometry`, the `geom` accessor that resolves the brep buckets of a `Model`, and the namespace re-export the preview consumers import."], lines: range(0, 52) },
    { name: "vec3", folder: "📏️vec3", emoji: "📏️", title: "Vec3 arithmetic", doc: ["Component-wise `Vec3` arithmetic, normalisation and move-point constraining."], lines: range(53, 104) },
    { name: "curves", folder: "🌀️curve-sampling", emoji: "🌀️", title: "Curve sampling", doc: ["Arc, circle, ellipse and NURBS frames, display samples, lengths and edge sampling."], lines: range(105, 295) },
    { name: "faces", folder: "🔺️face-geometry", emoji: "🔺️", title: "Face and solid geometry", doc: ["Points, centroids, normals and contact fusion of kernel faces and solids, plus the scalar-axis helpers."], names: ["derivedFacePoints", "derivedPointCentroid", "derivedSolidPoints"], lines: range(1003, 1200) },
    { name: "aabb", folder: "📦️aabb", emoji: "📦️", title: "Axis-aligned boxes", doc: ["Boxes of solids and points, their corners, intersection, volume, union and difference."], names: ["solidPrimitiveAabb", "modelObjectAabb", "Aabb", "aabbCornerPoints", "aabbIntersect", "aabbVolume", "aabbUnionVolume", "aabbOverlapUnionVolume", "aabbSubtractSingle", "aabbDifferencePieces", "aabbFromPoints"] },
    { name: "anchors", folder: "⚓️anchor-placement", emoji: "⚓️", title: "Anchor placement", doc: ["Closest-point and surface placement queries, anchor position evaluation and placement from picked entities."], lines: range(296, 559) },
    { name: "params", folder: "🎚️preview-params", emoji: "🎚️", title: "Preview parameters", doc: ["Box preview layout and the point transform of a preview kind, with the raw vector readers they share."], names: ["readVec3", "readVec3Array", "computeBoxPreviewLayout", "transformPointsForPreviewKind"] },
    { name: "kernel", folder: "", isParent: true, names: ["meshFaceModelDiff", "boxModelDiff", "PreciseSpatialKernelMath", "preciseSpatialKernelMath"] },
  ],
};
export default cfg;
