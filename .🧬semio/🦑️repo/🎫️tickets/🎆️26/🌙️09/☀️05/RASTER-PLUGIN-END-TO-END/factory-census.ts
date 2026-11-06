import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
function walk(path: string) {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const file = join(path, entry.name);
    if (entry.isDirectory()) walk(file);
    else if (entry.name.endsWith('.rs')) {
      const text = readFileSync(file, 'utf8');
      const pattern = /impl[^\n{]*ArtifactOwnedValueRetirementFactory[^\n{]*\{/g;
      for (const match of text.matchAll(pattern)) {
        let depth = 1, end = match.index! + match[0].length;
        while (depth && end < text.length) { const c = text[end++]; if (c === '{') depth++; if (c === '}') depth--; }
        const block = text.slice(match.index!, end);
        if (process.argv.includes('--unquoted') && block.includes('retire_owned_work_demand')) continue;
        if (process.argv.includes('--nested-result') && !(block.includes('-> Result<') && block.includes('.retire_owned('))) continue;
        if (process.argv.includes('--brief')) console.log(`${file} ${match[0]} ${block.includes('retire_owned_work_demand') ? 'QUOTED' : 'UNQUOTED'} ${block.includes('RetainedWorkOutcome') ? 'OUTCOME' : 'RESULT'}`);
        else console.log(`${file}\n${block}\n`);
      }
    }
  }
}
walk('🧰️framework');
