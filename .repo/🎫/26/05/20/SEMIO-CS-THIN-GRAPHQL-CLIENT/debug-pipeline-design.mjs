import { Session } from "../../../../../semio/client/lib/js/index.ts";

const session = await Session.openInMemory({ timeoutMs: 120_000 });
try {
  const kit = await (await session.stores())[0].wip().theKit().kit();
  for (const label of ["tag", "concept", "quality", "design", "type"]) {
    const r =
      label === "tag"
        ? await kit.createTag(`alpha-${label}`)
        : label === "concept"
          ? await kit.createConcept(`beta-${label}`)
          : label === "quality"
            ? await kit.createQuality(`k-${label}`, "v1")
            : label === "design"
              ? await kit.createDesign(`layout-${label}`)
              : await kit.createType(`kind-${label}`);
    const list =
      label === "tag"
        ? await kit.tags()
        : label === "concept"
          ? await kit.concepts()
          : label === "quality"
            ? await kit.qualities()
            : label === "design"
              ? await kit.designs()
              : await kit.types();
    console.log("[DEBUG]", label, "mutate", r, "listLen", list.length);
    if (list.length) console.log("[DEBUG]", label, "firstId", list[0].id);
  }
} finally {
  await session.dispose();
}
