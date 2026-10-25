import json,sys,datetime
C='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/'
T=C+'emoji-d-tools/'
files=set()
cnt={}
for n in ['applied.json','applied2.json','applied3.json']:
    try: a=json.load(open(T+n))['applied']
    except Exception: a=[]
    cnt[n]=len(a); files|=set(a)
manual=[
 "🧰️framework/🔨️modules/🖱️ui/⌨️tui/📝️text/🔤️tables/🦀️.rs",
 "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧬️element-state/🟦️.ts",
 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts",
 "🧰️framework/🔨️modules/⏯️tool-run/📐️definition/🦀️.rs",
 "🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs",
]
files|=set(manual)
extra=sys.argv[1] if len(sys.argv)>1 else ''
body=open(T+'result-body.md').read().replace('@@FILES@@',str(len(files))).replace('@@R1@@',str(cnt['applied.json'])).replace('@@R2@@',str(cnt['applied2.json'])).replace('@@R3@@',str(cnt['applied3.json']))
out=body+'\n## Changed files ('+str(len(files))+')\n\nDocstring first-emoji replacements only (no code), plus the five manual fixes marked `[manual]`.\n\n'
for f in sorted(files):
    out+='- '+f+(' [manual]' if f in manual else '')+'\n'
open(C+'result-emoji-d.md','w').write(out)
print(len(files),cnt)
