/** 🗺️ The 24 people of HW-10 … HW-18 and the worker folder that holds the generator and the repair ops of each. */
export const WORKER: Readonly<Record<string, string>> = {
  "picnicker-sitting-on-ground": "HW-10", "person-eating-standing": "HW-10", "person-drinking-coffee": "HW-10",
  "waving-person": "HW-11", "photographer-with-camera": "HW-11", "person-taking-selfie": "HW-11",
  "standing-violinist": "HW-12", "painter-at-easel": "HW-12", "dancer": "HW-12",
  "person-sweeping": "HW-13", "person-hanging-laundry": "HW-13", "person-carrying-shopping-bags": "HW-13",
  "postal-carrier": "HW-14",
  "person-with-guide-dog": "HW-15", "person-signing": "HW-15", "person-with-arm-sling": "HW-15",
  "person-in-rain-poncho": "HW-16", "person-in-winter-coat": "HW-16", "sunbather-on-towel": "HW-16",
  "person-queuing": "HW-17", "person-checking-watch": "HW-17", "parent-with-child-on-shoulders": "HW-17",
  "worker-with-pallet-jack": "HW-18", "person-carrying-ladder": "HW-18",
};
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const K = `${R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE`;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const opsFile = (slug: string): string => `${K}/🧰️scripts/${WORKER[slug]}/repair/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
