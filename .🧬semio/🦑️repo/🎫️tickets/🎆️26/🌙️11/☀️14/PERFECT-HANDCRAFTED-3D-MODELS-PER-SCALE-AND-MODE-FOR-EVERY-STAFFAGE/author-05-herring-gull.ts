import { animal } from "./author-05-animal.ts";

animal("herring-gull", "herring gull", "Silbermöwe", [{ match: /^mantle$/, skip: true }], () => {}, "The mantle plate of the fine drawing is merged into the body loft.", "Die Mantelplatte der feinen Zeichnung ist im Körperloft aufgegangen.", (em) => (em.kind === "breath" ? "head-plumage" : "body-plumage"));
