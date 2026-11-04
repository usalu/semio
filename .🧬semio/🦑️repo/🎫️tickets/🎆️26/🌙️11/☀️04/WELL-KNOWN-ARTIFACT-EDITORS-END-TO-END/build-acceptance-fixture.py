import json,glob,os
root='/home/deck/semio'
arts=root+'/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
plug={}
for f in glob.glob(root+'/🌎️hub/🧩️compositions/**/🧩️component/🔣️.json',recursive=True):
    try: d=json.load(open(f))
    except Exception: continue
    m=d.get('manifest',{})
    for a in m.get('apps',[]):
        plug.setdefault(a['dialect']['artifactKind'],m.get('pluginId'))
batch={}
for b,l in dict(A='txt md json xml html',B='csv tsv xlsx docx pptx idml',C='png jpg bmp gif tiff',D='svg dxf dwg pdf',E='obj stl ply gltf las step',F='ifc bcf idf epw inventory',G='mp3 wav mp4 avi',H='zip deflate binary semio').items():
    for k in l.split(): batch[k]=b
oracle=dict(txt='text',md='text',json='json',xml='xml',html='html',csv='csv',tsv='tsv',xlsx='ooxml',docx='ooxml',pptx='ooxml',idml='zip',png='png',jpg='jpeg',bmp='bmp',gif='gif',tiff='tiff',svg='svg',dxf='dxf',dwg='dwg',pdf='pdf',obj='obj',stl='stl',ply='ply',gltf='gltf',las='las',step='step',ifc='ifc',bcf='bcf',idf='text',epw='epw',mp3='mp3',wav='wav',mp4='mp4',avi='avi',zip='zip',deflate='zlib',binary='binary',semio='semio',inventory='none')
text={'txt','md','csv','tsv','html','idf','epw'}
edits={'txt':dict(mode='verb',verb='textEdit',args={'text':'Matrix text'}),'md':dict(mode='verb',verb='textEdit',args={'text':'# Matrix'}),'html':dict(mode='verb',verb='textEdit',args={'text':'<p>Matrix</p>'}),
'json':dict(mode='rendered',verb='set-node',control='[data-ui-node-key="edit"][aria-label^="name:"]',value='"Matrix"'),'xml':dict(mode='rendered',verb='set-node',control='[data-ui-node-key="edit"][aria-label^="text:"]',value='Matrix'),
'csv':dict(mode='rendered',verb='set-cell',control='[id$="/row-0/cell-0"]',value='Matrix Cell'),'tsv':dict(mode='rendered',verb='set-cell',control='[id$="/row-0/cell-0"]',value='Matrix Cell')}
inv={'json':'7b','xml':'3c'}
rows=[]
nonart=[]
for dn in sorted(os.listdir(arts)):
    p=os.path.join(arts,dn)
    if not os.path.isdir(p): nonart.append(dn); continue
    k=dn.split('️')[-1]
    dj=os.path.join(p,'📜️artifact-definition.json')
    if not os.path.exists(dj): nonart.append(dn); k=None
    if k is None: continue
    d=json.load(open(dj)); a=d['artifact']
    ext=d['representations'][0]['extensions'][0]
    kind=d['id']
    row=dict(batch=batch[a],artifact=a,directory=dn,kind=kind,plugin=plug.get(kind),extension=ext,oracle=oracle[a],surface='shipped' if kind in plug else 'missing',
      edit=edits.get(a,dict(mode='scan')) if kind in plug else dict(mode='none'),export=dict(format=oracle[a]),
      invalid=dict(bytes=inv.get(a,'ffc328' if a in text else 'deadbeef'),expect='refusal-notice'))
    rows.append(row)
# inventory is a library
nonart=[x for x in nonart]
rows.sort(key=lambda r:(r['batch'],r['artifact']))
out=dict(schema='semio.stdio.artifact-acceptance/v1',nonArtifacts=nonart,artifacts=rows)
json.dump(out,open(root+'/✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/🔣️.json','w'),ensure_ascii=False,indent=1)
print(len(rows),nonart,[r['artifact'] for r in rows if r['surface']=='missing'])
