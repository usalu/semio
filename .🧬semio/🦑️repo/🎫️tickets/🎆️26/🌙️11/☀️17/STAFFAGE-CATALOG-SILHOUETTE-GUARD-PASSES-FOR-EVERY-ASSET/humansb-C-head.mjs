export default (a) => {
  const g = a.representations["50"].geometry.front;
  const f = (id) => g.find((s) => s.id === id);
  const o = 0.0103;
  const up = (s, d) => { if (s.points) s.points.forEach((p) => (p[1] = +(p[1] + d).toFixed(5))); if (s.cy !== undefined) s.cy = +(s.cy + d).toFixed(5); };
  for (const id of ["eye-l", "eye-r", "brow-l", "brow-r", "nose", "mouth"]) up(f(id), -0.018);
  const hull = [[-0.0762, 1.7115], [-0.0755, 1.6889], [-0.0716, 1.6662], [-0.0725, 1.646], [-0.0725, 1.628], [-0.047, 1.6167], [-0.0305, 1.591], [-0.0089, 1.576], [0.0091, 1.5761], [0.0311, 1.5911], [0.048, 1.6169], [0.0725, 1.628], [0.0725, 1.646], [0.073, 1.6667], [0.0769, 1.6893], [0.0776, 1.712], [0.0752, 1.7344], [0.0694, 1.7569], [0.0628, 1.7726], [0.0543, 1.7869], [0.0433, 1.8018], [-0.0423, 1.8018], [-0.0532, 1.7871], [-0.0615, 1.7729], [-0.0681, 1.7573], [-0.0739, 1.7349]];
  const h = f("head"); delete h.cx; delete h.cy; delete h.rx; delete h.ry; h.kind = "poly"; h.points = hull.map(([x, y]) => [x, +(y + o).toFixed(5)]); h.closed = true;
  const half = [[0.09, 1.745], [0.09, 1.757], [0.088, 1.7728], [0.0843, 1.7832], [0.0789, 1.7945], [0.0751, 1.802], [0.0582, 1.8195], [0.0331, 1.8198], [0, 1.8198]];
  f("cap").points = [...half.slice().reverse().map(([x, y]) => [-x, +(y + o).toFixed(5)]), ...half.slice(0, -1).map(([x, y]) => [x, +(y + o).toFixed(5)])];
  f("cap-button").cy = +(1.8198 + o).toFixed(5);
  for (const id of ["ear-l", "ear-r"]) { const s = f(id); s.cy = +(1.6755 + o).toFixed(5); s.ry = 0.0125; }
  const b = f("cap-brim"); b.cy = +(1.7145 + o).toFixed(5); b.ry = 0.0285; b.rx = 0.067;
};
