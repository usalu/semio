import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const ticket = path.dirname(fileURLToPath(import.meta.url));
const generated = path.join(ticket, '🗑️generated');
const metrics = JSON.parse(fs.readFileSync(path.join(generated, 'iv', 'measurements.json'), 'utf8'));
const ids = ['physical_fundamentals', 'ventilation', 'heating', 'cooling', 'energy_balance'];
const names = ['Physikalisches Verständnis', 'Lüftung', 'Heizen', 'Kühlen', 'Energiebilanz'];
const videos = ids.map(id => metrics.videos.find(video => video.id === id && video.pass === true));
if (videos.some(video => !video) || metrics.profile.width !== 480 || metrics.profile.height !== 270 || metrics.profile.fps !== 4) throw Error('Five verified energy movies required');
const digest = file => crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
const sourceRoot = fs.realpathSync(path.join(generated, 'i'));
const stage = path.join(generated, 'inspection-energy');
const output = path.join(ticket, 'inspection-energy');
if (fs.existsSync(stage) || fs.existsSync(output)) throw Error('Early inspection delivery already exists');
fs.mkdirSync(stage);
for (const video of videos.slice(1)) {
  const source = fs.realpathSync(video.path);
  const relative = path.relative(sourceRoot, source);
  if (relative.startsWith('..') || path.isAbsolute(relative) || digest(source) !== video.sha256) throw Error('Verified source changed');
  const copy = path.join(stage, video.id + '.mp4');
  fs.copyFileSync(source, copy);
  if (digest(copy) !== video.sha256 || digest(source) !== video.sha256) throw Error('Early copy mismatch');
}
if (digest(path.join(ticket, 'inspection-physical.mp4')) !== videos[0].sha256) throw Error('Physical preview changed');
fs.renameSync(stage, output);
let index = '# Energy Inspection Videos\n\nAll five complete German spectator energy videos passed full playback, frame, audio, boundary and visual checks. Quick profile:480 ×270,4fps. The full17-video collection is still in progress.\n\n';
for (let i = 0; i < videos.length; i++) {
  const video = videos[i];
  const file = path.join(i ? output : ticket, i ? video.id + '.mp4' : 'inspection-physical.mp4').replaceAll('\\', '/');
  index += '## ' + names[i] + '\n\n[Play video](' + file + ')\n\n![' + names[i] + '](' + file + ')\n\n';
}
fs.writeFileSync(path.join(ticket, 'inspection-energy.md'), index);
process.stdout.write(JSON.stringify({delivered:5,index:path.join(ticket,'inspection-energy.md')})+'\n');
