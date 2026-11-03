import { fileOf, patchAnchors } from "./patch.ts";
patchAnchors(fileOf("🪑️objects", "tealight"), (l) => l.map((a) => (a.id === "flame-top" ? { ...a, y: 0.0135 } : a)));
patchAnchors(fileOf("🪑️objects", "street-lamp"), (l) => l.map((a) => (a.id === "lamp-plume" ? { ...a, y: -0.08 } : a)));
