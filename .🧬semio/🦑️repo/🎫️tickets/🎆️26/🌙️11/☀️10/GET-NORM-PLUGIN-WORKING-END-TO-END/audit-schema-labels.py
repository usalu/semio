import json,glob,sys,re
ROOT='/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/'
G=json.load(open('/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json'))['labels']
def loc(l): return isinstance(l,dict) and l.get('en') and l.get('de')
def infer(key,many):
    suf=('Ids','_ids') if many else ('Id','_id')
    st=None
    for s in suf:
        if key.endswith(s): st=key[:-len(s)];break
    if st is None: return None
    if st.startswith('new') and st[3:4].isupper(): st=st[3:]
    elif st=='new': return None
    return st or None
def audit(fam):
    out=[]
    sd=glob.glob(ROOT+fam+'/🏅️standards/*/🪆️subsets/*/🧬️schema')[0]
    snap=json.load(open(sd+'/📸️snapshot/🔣️.json'))
    def res(node):
        ui={};nullable=False;ref=node.get('_def')
        for _ in range(8):
            for k,v in (node.get('x-semio-ui') or {}).items(): ui.setdefault(k,v)
            if '$ref' in node:
                r=node['$ref']; ref=r.split('/')[-1]; 
                node=snap
                for part in r.split('#/')[1].split('/'): node=node[part]
                continue
            t=node.get('type')
            if isinstance(t,list):
                nt=[x for x in t if x!='null']
                if len(nt)==1: node=dict(node,type=nt[0]);nullable=True
            for alt in ('oneOf','anyOf'):
                if alt in node:
                    c=[x for x in node[alt] if x.get('type')!='null']
                    if len(c)==1 and len(node[alt])==2: node=c[0];break
            else: break
        return dict(node,_def=ref),ui
    def fields(obj,ptr,mid):
        node,_=res(obj)
        props=dict(node.get('properties',{}))
        for a in node.get('allOf',[]):
            n2,_=res(a); props.update(n2.get('properties',{}))
        for k,p in props.items(): inp(k,p,ptr+'/'+k,mid,node.get('_def'))
    def inp(key,p,ptr,mid,dfn=None):
        node,ui=res(p)
        role=ui.get('role');widget=ui.get('widget')
        if role=='discriminator' or 'const' in node: return
        if not loc(ui.get('label')) and key not in G: out.append((mid,ptr,'LabelMissing',dfn,key))
        t=node.get('type')
        many=t=='array'
        it=None
        if many and 'items' in node: it,_=res(node['items']); 
        idk=it.get('type') if (many and it) else t
        explicit=('ref' in ui) or widget=='reference' or role=='target'
        if explicit and idk not in('string','integer'): out.append((mid,ptr,'WidgetIncompat')); return
        inf=(role is None and widget is None and idk=='string' and 'enum' not in node) and infer(key,many)
        if explicit or inf:
            bad=[k for k in ui if k in('unit','step','precision','softMin','softMax','snaps','snapSource','displayUnit','displayFactor','scale')]
            if bad: out.append((mid,ptr,'UiInvalid '+bad[0]+' on reference'))
            ref=ui.get('ref')
            if not (ref and ref.get('kind')) and not infer(key,many): out.append((mid,ptr,'UiInvalid ref.kind'))
            return
        if 'enum' in node and t=='string':
            for v in node['enum']:
                lab=(ui.get('options') or {}).get(v)
                if not (loc(lab) or (lab is None and v in G)): out.append((mid,ptr,'OptionLabelMissing '+v))
        if t=='object' and 'properties' in node: fields(node,ptr,mid)
        if many and it and it.get('type')=='object': fields(it,ptr+'/-',mid)
    for p in sorted(glob.glob(sd+'/🧬️mutations/*/🧬️schema/🔣️.json')):
        d=json.load(open(p)); mid=p.split('/')[-3]
        fields(d,'',mid)
    return out
if __name__=='__main__':
    for f in sorted(glob.glob(ROOT+'*/')):
        fam=f.rstrip('/').split('/')[-1]
        if len(sys.argv)>1 and sys.argv[1] not in fam: continue
        o=audit(fam); print(fam,len(o))
        if len(sys.argv)>1: 
            for x in o: print('  ',x)
