/** 🦔️ Author 41: 2D root repair for the hedgehog (the 1:50 elevation skirt outlines are drawn unfilled).
 * Usage: bun author-41-hedgehog2d.ts > 🗑️generated/author-41/hedgehog2d-patch.json */
console.log(JSON.stringify(["skirt-front-near", "skirt-rear-near"].map((id) => ({ scale: "50", view: "elevation", id, set: { filled: true } }))));
