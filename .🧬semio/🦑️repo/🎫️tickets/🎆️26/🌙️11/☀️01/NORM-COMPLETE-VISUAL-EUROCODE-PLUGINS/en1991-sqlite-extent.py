import sys,re,json,sqlite3
def camel(s):
    p=s.split('_'); return p[0]+''.join(x.title() if not x.isdigit() else x for x in p[1:])
def calc(sqlp, jsonp, empty=False):
    sql=open(sqlp,encoding='utf8').read(); fx=json.load(open(jsonp,encoding='utf8'))
    c=sqlite3.connect(':memory:'); c.executescript(sql)
    cols={}
    for (t,) in c.execute("select name from sqlite_schema where type='table'").fetchall():
        cols[t]=[r[1] for r in c.execute(f'pragma table_info({t})')]
    snap=dict(fx)
    if empty:
        for k in ['floors','selfWeightElements','roofs','windFaces','accidentalCases']: snap[k]=[]
    def size(v): return 8 if isinstance(v,(int,float,bool)) else len(v.encode())
    def row(t, rec, fixed):
        n=0; 
        for col in cols[t]:
            if col in fixed: n+=fixed[col]; continue
            if col.endswith('_ieee754_bits'): n+=8; continue
            if col.endswith('_ieee754_class'): n+=len('finite'); continue
            if col=='logical_id': n+=len(rec['id'].encode()); continue
            v=rec[camel(col)]
            n+= len(('En' if v in('en','En') else 'De').encode()) if col=='annex' else size(v)
        return n
    total=0; rows=0
    total+=row('en1991_document',snap,{'id':8}); rows+=1
    for t,k in [('en1991_floor','floors'),('en1991_self_weight','selfWeightElements'),('en1991_roof','roofs'),('en1991_wind_face','windFaces')]:
        for r in snap[k]: total+=row(t,r,{'id':8,'document_id':8,'ordinal':8}); rows+=1
    for r in snap['accidentalCases']:
        total+=row('en1991_accidental_case',r,{'id':8,'document_id':8,'ordinal':8}); rows+=1
        for t,k in [('en1991_impact','impact'),('en1991_explosion','explosion')]:
            for s in r.get(k,[]): total+=row(t,s,{'id':8,'case_id':8,'ordinal':8}); rows+=1
    return rows,total
if __name__=='__main__':
    for e in (False,True): print(calc(sys.argv[1],sys.argv[2],e))
