import { ellipse, path, poly, rect, tx } from "./lib.mjs";

const r5 = (n) => Math.round(n * 1e5) / 1e5;

const translate = (d, ox, oy) => {
  const tokens = d.trim().split(/[\s,]+/);
  const out = [];
  let k = 0;
  for (const tok of tokens) {
    if (/^[A-Za-z]$/.test(tok)) {
      out.push(tok);
      k = 0;
    } else {
      const v = Number(tok);
      out.push(String(r5(v - (k % 2 === 0 ? ox : oy))));
      k += 1;
    }
  }
  return out.join(" ");
};

/**
 * Absolute-coordinate drawing space: bones are declared at absolute positions
 * (rest pose, angle 0) and every shape helper converts to the bone frame.
 */
export const space = (defs) => {
  const abs = Object.fromEntries(defs.map(([id, , x, y]) => [id, [x, y]]));
  const bones = defs.map(([id, parent, x, y, extra]) => {
    const [px, py] = parent ? abs[parent] : [0, 0];
    return { id, parent, x: r5(x - px), y: r5(y - py), angleDeg: 0, ...extra };
  });
  const o = (b) => abs[b];
  return {
    bones,
    abs,
    rect: (id, b, role, z, filled, tone, x, y, w, h, opt) => rect(id, b, role, z, filled, tone, x - o(b)[0], y - o(b)[1], w, h, opt),
    ellipse: (id, b, role, z, filled, tone, cx, cy, rx, ry, opt) => ellipse(id, b, role, z, filled, tone, cx - o(b)[0], cy - o(b)[1], rx, ry, opt),
    poly: (id, b, role, z, filled, tone, pts, closed, opt) => poly(id, b, role, z, filled, tone, pts.map(([x, y]) => [x - o(b)[0], y - o(b)[1]]), closed, opt),
    path: (id, b, role, z, filled, tone, d, opt) => path(id, b, role, z, filled, tone, translate(d, o(b)[0], o(b)[1]), opt),
    local: (b, x, y) => [r5(x - o(b)[0]), r5(y - o(b)[1])],
  };
};

export const sum = (xs) => xs.reduce((a, b) => a + b, 0);

export const materialOf = (id, en, de, emissivity, solar, density, conductivity, specificHeat, source, reference) => ({
  id,
  title: tx(en, de),
  emissivity,
  solarAbsorptivity: solar,
  densityKgM3: density,
  conductivityWmK: conductivity,
  specificHeatJKgK: specificHeat,
  source,
  ...(reference ? { reference } : {}),
});
