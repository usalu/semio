import json,re,sqlite3,sys
base=sys.argv[1]
sql=open(base+'/🗄️.sql').read()
db=sqlite3.connect(':memory:');db.executescript(sql)
widths={t:len(db.execute(f'PRAGMA table_info({t})').fetchall()) for (t,) in db.execute("select name from sqlite_master where type='table'")}
sem=json.load(open(base+'/🧪️tests/🧫️fixtures/🎛️semantic.json'))['tableWidths']
assert widths==sem,(widths,sem)
print('tables',len(widths),'max width',max(widths.values()))
adm=open(base+'/🛂️admission/🦀️.rs').read()
def consts(n): return re.search(r'const '+n+r':&\[&str\]=&\[(.*?)\];',adm).group(1).replace('"','').split(',')
schema=json.load(open(base+'/../../../🧬️schema/📸️snapshot/🔣️.json'))['$defs']
def checks(t,c,n):
    ddl=db.execute("select sql from sqlite_master where name=?",(t,)).fetchone()[0]
    m=re.search(c+r" TEXT NOT NULL CHECK ?\(\w+ IN \((.*?)\)\)",ddl)
    return m.group(1).replace("'",'').split(',') if m else None
defs={'CLASSES':'StrengthClass','KINDS':'ActionKind','CATEGORIES':'ImposedCategory','DURATIONS':'LoadDurationClass','FASTENERS':'FastenerType'}
for n,t,c in[('CLASSES','en1995_member','strength_class'),('CLASSES','en1995_connection','strength_class'),('KINDS','en1995_member_action','kind'),('CATEGORIES','en1995_member_action','category'),('DURATIONS','en1995_member_action','load_duration'),('FASTENERS','en1995_connection','fastener_type'),('KINDS','en1995_connection_action','kind'),('DURATIONS','en1995_connection_action','load_duration')]:
    assert consts(n)==schema[defs[n]]['enum'],(n,'schema')
    k=checks(t,c,n)
    assert k is None or k==consts(n),(n,t,c)
print('admission enum lists match the JSON schema enums and every SQL CHECK domain')
