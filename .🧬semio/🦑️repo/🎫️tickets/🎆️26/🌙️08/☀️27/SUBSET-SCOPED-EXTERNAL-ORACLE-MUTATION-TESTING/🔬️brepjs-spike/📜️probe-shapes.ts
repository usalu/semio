// 🔬️ Shape-signature probe: what do brepjs's transform, boolean and exportSTEP actually return?
const b = (await import("brepjs")) as any;
await b.init();
const unwrap = (v: any) => (v && typeof v === "object" && "ok" in v ? (v.ok === false ? (() => { throw new Error(JSON.stringify(v.error)); })() : v.value) : v);
const box = unwrap(b.box(20, 20, 20));
const cyl = unwrap(b.cylinder(5, 40));
console.log("cyl bounds", JSON.stringify(unwrap(b.getBounds(cyl))));
const moved = unwrap(b.translate(cyl, [0, 0, -10]));
console.log("translate arity", b.translate.length, "moved bounds", JSON.stringify(unwrap(b.getBounds(moved))), "volume", unwrap(b.measureVolume(moved)));
const res = unwrap(b.cut(box, moved));
console.log("cut volume", unwrap(b.measureVolume(res)), "expected", 8000 - Math.PI * 25 * 20);
console.log("cut arity", b.cut.length);
const step = b.exportSTEP(res);
console.log("exportSTEP:", Object.prototype.toString.call(step), step && typeof step === "object" ? Object.keys(step) : typeof step);
const inner = unwrap(step);
console.log("unwrapped:", Object.prototype.toString.call(inner), inner && typeof inner === "object" && !ArrayBuffer.isView(inner) ? Object.keys(inner).slice(0, 12) : String(inner).slice(0, 100));
