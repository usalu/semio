"""🔎️ Compare frozen source identities without rebasing the held checkpoint."""
from pathlib import Path
import hashlib,json
t=Path(__file__).parent
source=json.loads((t/'🗑️generated/boolean-workspace-kernel-source.json').read_text(encoding='utf8'))
value={}
for role in ['owned','references']:
 value[role]=[{**row,'actualSha256':hashlib.sha256(Path(row['path']).read_bytes()).hexdigest()}for row in source[role]]
value['changed']=[row['path']for role in ['owned','references']for row in value[role]if row['sha256']!=row['actualSha256']]
value['nativeExecuted']=False
(t/'🗑️generated/boolean-workspace-kernel-source-after-tests.json').write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG]',json.dumps({'files':sum(len(value[role])for role in ['owned','references']),'changed':value['changed']},ensure_ascii=False))
