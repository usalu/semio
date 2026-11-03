const fs = require('fs');
const path = require('path');
const [dir, out, zoomText, ...specs] = process.argv.slice(2);
const zoom = Number(zoomText);
let html = '<!doctype html><meta charset="utf-8"><style>body{background:#001117;color:#f7f3e3;font:12px sans-serif;margin:8px}.c{display:inline-block;margin:6px;padding:4px;border:1px solid #2e3c3d;vertical-align:top}.c svg{display:block}</style>';
for (const spec of specs) {
  const [id, frameText] = spec.split(':');
  const folder = path.join(dir, id);
  const index = JSON.parse(fs.readFileSync(path.join(folder, '🔣️.json'), 'utf8'));
  const frames = frameText === 'all' ? index.frames : [index.frames[Number(frameText || 0)]];
  for (const frame of frames) {
    let svg = fs.readFileSync(path.join(folder, frame), 'utf8').replace(/^<\?xml[^>]*>\s*/u, '');
    svg = svg.replace(/width="([\d.]+)mm" height="([\d.]+)mm"/u, (_, w, h) => `width="${Number(w) * zoom}" height="${Number(h) * zoom}"`);
    svg = svg.replace(/stroke-width="([\d.]+)"/gu, (_, w) => `stroke-width="${Number(w) * 0.3}"`);
    html += `<div class="c">${svg}<div>${id} ${frame}</div></div>`;
  }
}
fs.writeFileSync(out, html);
console.log(out);
