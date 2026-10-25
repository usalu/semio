import json,sys
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
T=[('🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json','test-canonical-stdio',('live','drives-running-process')),
('🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📋️project.json','test-catalog-reservation-source',True),
('🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📋️project.json','test-inline-symbol-source',True),
('🧰️framework/🔨️modules/🏃️process/📋️project.json','test-windows-conditional-retirement',True),
('🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json','test-retained-wire-field',True),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json','test-ci-source',True),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json','test-language',True),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📋️project.json','check-types',True),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📋️project.json','generate-types',('side-effecting','mutates-workspace')),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📋️project.json','preview-types',True),
('🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📋️project.json','test-types-source',True),
('🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json','test-exclusive-retirement-source',True),
('🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json','test-paged-reservation-source',True),
(L+'🏗️build/📥️inputs/💾️storage/📋️project.json','test',True),
('🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json','test-producer-source',True)]
by={}
for f,t,a in T: by.setdefault(f,[]).append((t,a))
for f,rows in by.items():
  raw=open(f,encoding='utf8').read(); d=json.loads(raw)
  assert json.dumps(d,indent=2,ensure_ascii=False)+"\n"==raw,f
  for t,a in rows:
    tg=d['targets'][t]; assert tg.get('cache') is False,(f,t)
    if a is True: tg['cache']=True
    else:
      tg.setdefault('metadata',{}).setdefault('semio',{})['cache']={'version':1,'class':a[0],'reason':a[1]}
  if '--write' in sys.argv: open(f,'w',encoding='utf8').write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
  print(f[-60:],[t for t,_ in rows])
