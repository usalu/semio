/** 🗺️ The 25 people of the batch SH-2 and where the repair ops of each live: the people of HW-10 … HW-18 keep them in the repair file of their generator chain, `tennis-player` (HW-07) as the stage 80 of its chain `REP-A/ops`, every other person in `ops/<slug>.json` of this folder. */
export const WORKER: Readonly<Record<string, string>> = {
  "person-sweeping": "HW-13", "person-taking-selfie": "HW-11", "person-with-guide-dog": "HW-15", "photographer-with-camera": "HW-11",
  "standing-violinist": "HW-12", "sunbather-on-towel": "HW-16", "worker-with-pallet-jack": "HW-18",
};
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const K = `${R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE`;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const STAGED: Readonly<Record<string, string>> = { "tennis-player": "tennis-player.80-silhouette.json" };
export const opsFile = (slug: string): string => (STAGED[slug] !== undefined ? `${K}/🧰️scripts/REP-A/ops/${STAGED[slug]}` : WORKER[slug] === undefined ? `${import.meta.dir}/ops/${slug}.json` : `${K}/🧰️scripts/${WORKER[slug]}/repair/${slug}.json`);
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
