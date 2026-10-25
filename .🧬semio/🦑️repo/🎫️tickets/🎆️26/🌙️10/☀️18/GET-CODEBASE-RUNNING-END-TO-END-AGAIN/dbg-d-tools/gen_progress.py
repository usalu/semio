import sys;sys.path.insert(0,sys.argv[1]+"/dbg-d-tools")
from edit import edit
edit("🏭️generator/👁️preview/📈️progress/🟦️.ts",[
('export interface GeneratorPreviewProgressPolicyV1','const PROGRESS_PREFIX = "[progress] ";\n\nexport interface GeneratorPreviewProgressPolicyV1'),
('const line = `[DEBUG] ${JSON.stringify(row)}\\n`;','const line = `${PROGRESS_PREFIX}${JSON.stringify(row)}\\n`;'),
('!line.startsWith("[DEBUG] ")','!line.startsWith(PROGRESS_PREFIX)'),
('JSON.parse(line.slice(8))','JSON.parse(line.slice(PROGRESS_PREFIX.length))')])
