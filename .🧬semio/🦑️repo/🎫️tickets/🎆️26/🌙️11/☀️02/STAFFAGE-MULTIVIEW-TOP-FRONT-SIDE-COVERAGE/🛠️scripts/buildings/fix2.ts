import { load, save, clone } from "./lib.ts";
{
  const doc = load("tilt-window");
  for (const sc of ["50", "100", "200"]) {
    const el = doc.asset.representations[sc].geometry.elevation;
    if (!el.some((s: any) => s.id === "jamb-profile"))
      el.unshift({ id: "jamb-profile", bone: "sill", role: "body", z: 0.5, filled: true, tone: 0.75, region: "frame-timber", material: "timber", kind: "rect", x: 0, y: 0, w: 0.3, h: 1.2 });
  }
  save("tilt-window", doc);
}
{
  const doc = load("external-venetian-blind");
  for (const sc of ["100", "200"]) {
    const g = doc.asset.representations[sc].geometry;
    for (const view of ["front", "plan"])
      g[view] = g[view].map((s: any) => {
        if (!/^slat-\d+$/.test(s.id) || s.kind !== "rect") return s;
        const { x, y, w, h, kind, ...rest } = s;
        return { ...rest, kind: "poly", filled: false, closed: false, points: [[x, y + h / 2], [x + w, y + h / 2]] };
      });
  }
  save("external-venetian-blind", doc);
}
{
  const doc = load("circulation-pump");
  for (const view of ["elevation", "plan"])
    for (const s of doc.asset.representations["50"].geometry[view]) if (s.id === "gland" && s.w < 0.011) { s.x = -0.0465; s.w = 0.0165; }
  save("circulation-pump", doc);
}
