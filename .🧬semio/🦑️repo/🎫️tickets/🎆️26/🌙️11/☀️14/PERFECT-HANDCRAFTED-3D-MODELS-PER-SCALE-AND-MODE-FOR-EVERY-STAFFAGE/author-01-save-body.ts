const slug = process.argv[2]!;
const mod: any = await import(`./author-01-${slug}.ts`);
const { save, Author } = await import("./author-01-kit.ts");
const as: any = {};
for (const s of [50, 100, 200, 500]) { try { as[s] = (mod.build ?? mod.body)(s); } catch (e) { console.log("scale", s, (e as Error).message); } }
const asset: any = new Author(slug, 50).asset;
for (const s of [50, 100, 200, 500]) if (as[s] === undefined && asset.representations[String(s)].status !== "unavailable") { console.log("missing", s); process.exit(1); }
if (process.argv.includes("--dry")) process.exit(0);
save(slug, as);
