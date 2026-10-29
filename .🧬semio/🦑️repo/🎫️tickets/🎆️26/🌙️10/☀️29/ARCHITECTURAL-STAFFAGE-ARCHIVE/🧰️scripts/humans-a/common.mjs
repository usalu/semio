// [DEBUG] shared authoring vocabulary for humans-a sources (texts, references, shape helpers, anatomical constants).
export const tx = (en, de) => ({ en, de });
export const REFS = {
  "de-leva-1996": { id: "de-leva-1996", citation: "de Leva, P. (1996). Adjustments to Zatsiorsky-Seluyanov's segment inertia parameters. Journal of Biomechanics 29(9), 1223-1230. Tables 4 (male) and 5 (female), segment mass fractions and centre-of-mass positions." },
  "winter-2009": { id: "winter-2009", citation: "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley. Table 4.1, anthropometric segment data." },
  "drillis-contini-1966": { id: "drillis-contini-1966", citation: "Drillis, R., Contini, R. and Bluestein, M. (1964/1966). Body segment parameters: a survey of measurement techniques. Artificial Limbs 8(1), 44-66; segment lengths as fractions of stature." },
  "dubois-1916": { id: "dubois-1916", citation: "DuBois, D. and DuBois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871." },
  "iso-9920-2007": { id: "iso-9920-2007", citation: "ISO 9920:2007 Ergonomics of the thermal environment, estimation of thermal insulation and water vapour resistance of a clothing ensemble." },
  "iso-8996-2021": { id: "iso-8996-2021", citation: "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate." },
  "ashrae-2021": { id: "ashrae-2021", citation: "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort; Chapter 18 Nonresidential cooling and heating load calculations for emissivities and metabolic data." },
  "gupta-2010": { id: "gupta-2010", citation: "Gupta, J. K., Lin, C.-H. and Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39." },
  "perry-burnfield-2010": { id: "perry-burnfield-2010", citation: "Perry, J. and Burnfield, J. M. (2010). Gait Analysis: Normal and Pathological Function, 2nd ed., SLACK. Gait cycle phases and joint angle ranges." },
  "iso-7730-2005": { id: "iso-7730-2005", citation: "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort; Annex B metabolic rates." },
};
export const refs = (...ids) => ids.map((id) => REFS[id]);
export const rev = (minDeg, maxDeg) => ({ type: "revolute", minDeg, maxDeg });
export const FIXED = { type: "fixed" };
export const deg = (r) => (r * 180) / Math.PI;
export const rad = (d) => (d * Math.PI) / 180;
export const r4 = (v) => Math.round(v * 1e4) / 1e4;
export const smooth = (w) => { const u = Math.min(1, Math.max(0, w)); return u * u * (3 - 2 * u); };
export const sh = (id, bone, role, lod, z, tone, region, material, d, extra = {}) => ({ id, bone, role, lod, z, tone, region, material, d, ...extra });
export const cap = (id, bone, role, lod, z, tone, region, material, p0, p1, r0, r1) => ({ id, bone, role, lod, z, tone, region, material, kind: "capsule", p0, p1, r0, r1: r1 ?? r0 });
export const ell = (id, bone, role, lod, z, tone, region, material, c, rx, ry, rotationDeg = 0, extra = {}) => ({ id, bone, role, lod, z, tone, region, material, kind: "ellipse", c, rx, ry, rotationDeg, ...extra });
export const line = (id, bone, role, lod, z, tone, region, material, points) => ({ id, bone, role, lod, z, tone, region, material, kind: "poly", points, closed: false, filled: false });
export const mass = (kg, com, reference, extra = {}) => ({ kg, com, source: "literature", reference, distribution: "point", ...extra });
export const STD_EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"];
export const ENV = (airTempC, meanRadiantTempC, relativeHumidity, windMS, solarWM2) => ({ airTempC, meanRadiantTempC, relativeHumidity, windMS, windDirectionDeg: 180, solarWM2, pressurePa: 101325 });
export const mat = (id, en, de, emissivity, solarAbsorptivity, source, reference) => ({ id, title: tx(en, de), emissivity, solarAbsorptivity, source, ...(reference ? { reference } : {}) });
export const region = (id, en, de, kind, material, model, areaM2, characteristicLengthM, metabolicShare, extra, reference, source = "literature") => ({ id, title: tx(en, de), kind, material, model, areaM2, characteristicLengthM, ...extra, ...(metabolicShare === undefined ? {} : { metabolicShare }), source, reference });
export const sm5 = (w) => { const u = Math.min(1, Math.max(0, w)); return u * u * u * (u * (6 * u - 15) + 10); };
export const R = (a, p) => { const c = Math.cos(rad(a)), s = Math.sin(rad(a)); return [p[0] * c - p[1] * s, p[0] * s + p[1] * c]; };
export const xf = (d, k, rot, [ox, oy], [dx, dy]) => {
  const tok = d.match(/[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?/g); const out = []; let i = 0;
  while (i < tok.length) { const c = tok[i++]; if (c === "Z") { out.push("Z"); continue; } const n = c === "C" ? 3 : c === "Q" ? 2 : 1; const parts = [c];
    for (let q = 0; q < n; q++) { const p = R(rot, [(Number(tok[i++]) - ox) * k, (Number(tok[i++]) - oy) * k]); parts.push(`${r4(p[0] + dx)} ${r4(p[1] + dy)}`); }
    out.push(parts.join(" ")); }
  return out.join(" ");
};
export const xp = (pts, k, rot, o, dst) => pts.map((p) => { const q = R(rot, [(p[0] - o[0]) * k, (p[1] - o[1]) * k]); return [r4(q[0] + dst[0]), r4(q[1] + dst[1])]; });
export const xpt = (p, k, rot, o, dst) => xp([p], k, rot, o, dst)[0];
export const ik2 = (hip, ankle, L1, L2, phi, tol = 0.006) => {
  const dx = ankle[0] - hip[0], dy = ankle[1] - hip[1], dist = Math.hypot(dx, dy);
  if (dist > L1 + L2 + tol) throw new Error(`leg out of reach ${dist.toFixed(4)} > ${(L1 + L2).toFixed(4)}`);
  const d = Math.min(dist, L1 + L2 - 1e-4), beta = Math.acos((L1 * L1 + d * d - L2 * L2) / (2 * L1 * d));
  const th = Math.atan2(dy, dx) + beta, kx = hip[0] + L1 * Math.cos(th), ky = hip[1] + L1 * Math.sin(th), sk = Math.atan2(ankle[1] - ky, ankle[0] - kx);
  return { thigh: deg(th) + 90, shank: deg(sk - th), foot: phi - deg(sk) - 90 };
};
// de Leva (1996) segment mass fractions of body mass: [head, trunk(mid+upper), lower trunk, upper arm, forearm, hand, thigh, shank, foot]
export const FRAC_M = { head: 0.0694, trunk: 0.3229, pelvis: 0.1117, uarm: 0.0271, farm: 0.0162, hand: 0.0061, thigh: 0.1416, shank: 0.0433, foot: 0.0137 };
export const FRAC_F = { head: 0.0668, trunk: 0.3010, pelvis: 0.1247, uarm: 0.0255, farm: 0.0138, hand: 0.0056, thigh: 0.1478, shank: 0.0481, foot: 0.0129 };
export const armIK = (sh, tgt, L1, L2) => {
  const dx = tgt[0] - sh[0], dy = tgt[1] - sh[1], d = Math.min(Math.hypot(dx, dy), L1 + L2 - 1e-4);
  const beta = Math.acos((L1 * L1 + d * d - L2 * L2) / (2 * L1 * d));
  const th = Math.atan2(dy, dx) - beta, ex = sh[0] + L1 * Math.cos(th), ey = sh[1] + L1 * Math.sin(th), sk = Math.atan2(tgt[1] - ey, tgt[0] - ex);
  return { arm: deg(th) + 90, forearm: deg(sk - th), elbow: [ex, ey], dist: Math.hypot(dx, dy) };
};
export const pt = (p, rotDeg, around) => { const q = R(rotDeg, [p[0] - around[0], p[1] - around[1]]); return [q[0] + around[0], q[1] + around[1]]; };
// Generic two-link-IK gait: left foot path over tau in [0,1) (IC at 0, flat 0..flatTo, heel-off from heelOff, toe-off at to, swing to the next IC); right foot half a cycle later.
// P: { k, S, T, L1, L2, XH, hipBase, hipAmp, hipMinAt, hipPow, flatTo, heelOff, to, phi0, phiTO, bump, clearK }
export const makeGait = (P) => {
  const a = 0.075 * P.k, bx = 0.17 * P.k, hx = 0.045 * P.k, ball = [P.XH + hx + bx, 0], heel = [P.XH, 0];
  const BALLO = [bx, -a];
  const left = (tau) => {
    let phi, ankle;
    if (tau <= P.flatTo) { phi = P.phi0 * (1 - sm5(tau / P.flatTo)); const o = R(phi, [hx, a]); ankle = [heel[0] + o[0], o[1]]; }
    else if (tau <= P.heelOff) { phi = 0; ankle = [P.XH + hx, a]; }
    else if (tau <= P.to) { phi = P.phiTO * sm5((tau - P.heelOff) / (P.to - P.heelOff)); const o = R(phi, BALLO); ankle = [ball[0] - o[0], -o[1]]; }
    else {
      const o0 = R(P.phiTO, BALLO), a0 = [ball[0] - o0[0], -o0[1]], o1 = R(P.phi0, [hx, a]), a1 = [P.XH + P.S + o1[0], o1[1]];
      const w = (tau - P.to) / (1 - P.to), e = sm5(Math.pow(w, P.swingPow ?? 1));
      phi = P.phiTO * (1 - e) + P.phi0 * e;
      ankle = [a0[0] + (a1[0] - a0[0]) * e, a0[1] + (a1[1] - a0[1]) * e + P.bump * Math.sin(Math.PI * w) ** 4];
    }
    const toe = tau < P.to ? Math.max(0, -phi) : Math.max(0, -phi) * (1 - sm5((tau - P.to) / (1 - P.to)));
    return { ankle, phi, toe };
  };
  const foot = (u, right) => { let tau = right ? u - 0.5 : u, shift = right ? P.S / 2 : 0; if (tau < 0) { tau += 1; shift -= P.S; } const f = left(tau); return { ankle: [f.ankle[0] + shift, f.ankle[1]], phi: f.phi, toe: f.toe }; };
  const hipAt = (u) => [P.S * u, P.hipBase - P.hipAmp * (1 - Math.cos(4 * Math.PI * (u - P.hipMinAt))) / 2];
  const legs = (u) => {
    const hip = hipAt(u), out = {};
    for (const [s, right] of [["l", false], ["r", true]]) { const f = foot(u, right), q = ik2(hip, f.ankle, P.L1, P.L2, f.phi, 0.006); out[`thigh-${s}`] = { angleDeg: r4(q.thigh) }; out[`shank-${s}`] = { angleDeg: r4(q.shank) }; out[`foot-${s}`] = { angleDeg: r4(q.foot) }; out[`toe-${s}`] = { angleDeg: r4(f.toe) }; }
    return out;
  };
  return { legs, hipAt, foot, left };
};
export const circ = (cx, cy, r, ccw = true) => { const k = 0.5523 * r, f = (v) => r4(v); const pts = ccw
  ? [[cx + r, cy], [cx + r, cy + k, cx + k, cy + r, cx, cy + r], [cx - k, cy + r, cx - r, cy + k, cx - r, cy], [cx - r, cy - k, cx - k, cy - r, cx, cy - r], [cx + k, cy - r, cx + r, cy - k, cx + r, cy]]
  : [[cx + r, cy], [cx + r, cy - k, cx + k, cy - r, cx, cy - r], [cx - k, cy - r, cx - r, cy - k, cx - r, cy], [cx - r, cy + k, cx - k, cy + r, cx, cy + r], [cx + k, cy + r, cx + r, cy + k, cx + r, cy]];
  return `M ${f(pts[0][0])} ${f(pts[0][1])} ` + pts.slice(1).map((p) => `C ${p.map(f).join(" ")}`).join(" ") + " Z"; };
export const ring = (cx, cy, ro, ri) => `${circ(cx, cy, ro, true)} ${circ(cx, cy, ri, false)}`;
export const arcBand = (cx, cy, ro, ri, a0, a1) => {
  const seg = (r, from, to) => { const n = Math.max(1, Math.ceil(Math.abs(to - from) / 45)), out = []; for (let i = 0; i < n; i++) { const t0 = rad(from + ((to - from) * i) / n), t1 = rad(from + ((to - from) * (i + 1)) / n), h = (4 / 3) * Math.tan((t1 - t0) / 4); out.push([cx + r * (Math.cos(t0) - h * Math.sin(t0)), cy + r * (Math.sin(t0) + h * Math.cos(t0)), cx + r * (Math.cos(t1) + h * Math.sin(t1)), cy + r * (Math.sin(t1) - h * Math.cos(t1)), cx + r * Math.cos(t1), cy + r * Math.sin(t1)]); } return out; };
  const f = (v) => r4(v), P = (r, a) => `${f(cx + r * Math.cos(rad(a)))} ${f(cy + r * Math.sin(rad(a)))}`;
  return `M ${P(ro, a0)} ` + seg(ro, a0, a1).map((s) => `C ${s.map(f).join(" ")}`).join(" ") + ` L ${P(ri, a1)} ` + seg(ri, a1, a0).map((s) => `C ${s.map(f).join(" ")}`).join(" ") + " Z";
};
