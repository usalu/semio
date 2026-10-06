import { readFileSync, writeFileSync } from 'node:fs';
const ticket = process.env.SEMIO_TICKET_DIR!, root = ticket + '/🗑️generated/api-title-current-view';
const files = JSON.parse(readFileSync(root + '/pdf-pages.json', 'utf8'));
const visual = JSON.parse(readFileSync('C:/git/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json', 'utf8')).printed.visual;
const results = files.map((file: any) => {
  let contents = 0, captions = 0, minimumGapPt = Infinity;
  for (const page of file.textPages) {
    if (/(?:Inhaltsverzeichnis|Table\s+of\s+contents)/i.test(page.text)) {
      for (const token of visual.forbiddenContents) if (page.text.replace(/\s+/g, '').includes(token)) throw Error(file.name + ': raw contents token ' + token);
      contents++;
    }
    for (const badge of page.items.filter((item: any) => /^(?:Table|Tabelle):/.test(item.text))) {
      const row = page.items.filter((item: any) => Math.abs(item.y - badge.y) < 2 && item.x < badge.x);
      if (!visual.titles.some((title: string[]) => title.slice(1).some(value => row.map((item: any) => item.text).join(' ').includes(value)))) continue;
      const gap = badge.x - Math.max(...row.map((item: any) => item.x + item.width));
      if (gap < visual.minimumCaptionGapPt) throw Error(file.name + ':' + page.number + ': caption gap ' + gap);
      minimumGapPt = Math.min(minimumGapPt, gap);
      captions++;
    }
  }
  if (!contents || !captions) throw Error('Vacuous current API proof');
  return { name: file.name, sha256: file.sha256, pages: file.pages, contents, captions, minimumGapPt };
});
writeFileSync(import.meta.dir + '/result.json', JSON.stringify(results, null, 2));
console.log('[DEBUG] ' + JSON.stringify(results));
