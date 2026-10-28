from pathlib import Path
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
for r in ['🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base','📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base']:
 p=A/r/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/binary.test.ts';s=p.read_text(encoding='utf-8');s=s.replace('content=>{kind:"binary",value:content}','content=>({kind:"binary",value:content})').replace('content=>{binary:content}','content=>({binary:content})').replace('timeStamp','timestamp');p.write_text(s,encoding='utf-8',newline='\n')
