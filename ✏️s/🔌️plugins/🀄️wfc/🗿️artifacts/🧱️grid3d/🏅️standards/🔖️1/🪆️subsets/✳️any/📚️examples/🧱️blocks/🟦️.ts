/** 🧱️ Example `blocks` — the TypeScript metadata twin of `🦀️.rs`, same id, label, icon and seed. */
export const id = "blocks";
export const label = { en: "Building Blocks", de: "Bauklötze" } as const;
export const icon = "building";
export const seed = 7;
export const dslPath = new URL("./🖼️assets/🧱️blocks/🗣️.dsl.semio", import.meta.url);
