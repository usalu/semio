import { load, save } from "./lib.ts";
for (const slug of ["kneeling-gardener", "parent-carrying-toddler", "skateboarder"]) {
  const doc = load(slug);
  for (const [k, r] of Object.entries<any>(doc.asset.representations)) {
    if (r.status !== "unavailable" || r.cause) continue;
    const { status, denominator, reason, limitingFeature, limitingSizeM, alternatives } = r;
    doc.asset.representations[k] = { status, denominator, cause: "below-printable-feature", reason, limitingFeature, limitingSizeM, alternatives };
    console.log(slug, k, "cause set");
  }
  save(slug, doc);
}
