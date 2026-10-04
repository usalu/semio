/** 🖼️ Reads the exact retained Raster and Shooting domain owner classification. */
const ticket = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const roster = await Bun.file(`${ticket}/📥️inputs/global-child-current-production-domain-owner-readback.json`).json();
const capsule = await Bun.file(`${ticket}/📥️inputs/global-child-current-immutable-frame-after-plugin-checked-position-joins-held-pairs.json`).json();
const rows = (Array.isArray(roster) ? roster : roster.rows ?? roster.paths ?? roster.entries).filter((row: {family:string}) => ["Raster","Shooting"].includes(row.family));
console.log(JSON.stringify({rows,pairs:capsule.pairs.filter((pair:{path:string})=>rows.some((row:{path:string})=>row.path===pair.path))},null,2));
