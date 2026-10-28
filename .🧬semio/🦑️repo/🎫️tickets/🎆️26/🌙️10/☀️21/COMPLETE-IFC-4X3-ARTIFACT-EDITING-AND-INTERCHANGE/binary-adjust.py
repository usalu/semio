from pathlib import Path
S=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot')
s=(S/'🟦️.ts').read_text(encoding='utf-8');sql=(S/'🗄️.sql').read_text(encoding='utf-8');a=s.split('export const IFC_SQLITE_SCHEMA=`',1)
print(repr(s[:450]));print(repr(sql[:100]));print(len(sql));print(sql[-30:])
if len(a)>1:
 code=a[1].split('`;',1)[0];print(len(code),code==sql,repr(code[:40]),repr(code[-40:]))
for p in [S/'🧪️tests/🧫️fixtures/🎛️semantic.json',S/'🧪️tests/🟦️.ts',S/'🧪️tests/🦀️.rs']:
 t=p.read_text(encoding='utf-8');p.write_text(t.replace('3250','3271'),encoding='utf-8')
P=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🔢️values/🦀️.rs');t=P.read_text(encoding='utf-8');P.write_text(t.replace("Some(&unused @ b'0'..=b'3')", "Some(&(unused @ b'0'..=b'3'))"),encoding='utf-8')
