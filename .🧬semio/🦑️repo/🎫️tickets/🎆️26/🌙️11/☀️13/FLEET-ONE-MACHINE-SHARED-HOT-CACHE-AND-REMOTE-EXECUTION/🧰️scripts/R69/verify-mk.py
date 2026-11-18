import sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R69/"
kind=sys.argv[1]
DP=R+"🚀️bootstrap/📦️dependencies/💾️storage/"
PUBD=R+"💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/"
for t,name in (('t1','field-diagnosis'),('t2','stale-generation')):
    s=open(SC+'stage/%s.new'%t,encoding='utf8').read()
    s=s.replace('new URL("../../../../🟦️.ts",import.meta.url)','new URL("file://'+SC+'stage/s3.'+kind+'")').replace('new URL("../../🟦️.ts",import.meta.url)','new URL("file://'+PUBD+'⚙️setup/♻️retirement/🟦️.ts")')
    s=s.replace('"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/','"'+R+'🚀️bootstrap/📦️dependencies/💾️storage/').replace('"../../../../../../../../../📖️texts/','"'+R+'📖️texts/').replace('"../../🧬️schema/🔣️.json"','"'+PUBD+'⚙️setup/♻️retirement/🧬️schema/🔣️.json"')
    s=s.replace('"../../🟦️.ts"','"'+DP+'🟦️.ts"').replace('"../../../../../💾️storage/','"'+R+'💾️storage/')
    open(SC+'tt/%s.%s.test.ts'%(name,kind),'w',encoding='utf8').write(s)
