import { catalogChildId } from "./siphash.ts";
console.log("door   ", catalogChildId([{ id: "door", name: "door", category: "vortex-kind" }]), "expect catalog-9dc5de0f33c9568d");
const hex = ["b-l","b-l-m","b-s","b-s-m","c-b","c-t"];
console.log("hex6   ", catalogChildId(hex.map((id) => ({ id, name: id, category: "vortex-kind" }))), "expect catalog-3b18d1b44d9af6de");
console.log("empty  ", catalogChildId([]));
