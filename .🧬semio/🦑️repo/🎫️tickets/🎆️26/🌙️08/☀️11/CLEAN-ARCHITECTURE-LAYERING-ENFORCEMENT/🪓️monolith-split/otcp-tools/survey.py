import csv,sys,re,os,collections
crate=sys.argv[1]; src='🗑️generated/otcp-baseline-files.tsv'
R='/Users/ueli/Documents/semio/'
rows=[r for r in csv.DictReader(open(src,encoding='utf8'),delimiter='\t') if r['label']=='native' and r['unit']=='lib-test' and r['crate']=='semio-framework-'+crate and r['notInLibNormal']=='true']
for r in rows:
    if r['kind']!='source': print("EMB",r['bytes'],r['file'][-110:]); continue
    t=open(R+r['file'],encoding='utf8').read()
    c=collections.Counter()
    c['glob_super']=len(re.findall(r'use\s+super::\*',t)); c['super_super']=len(re.findall(r'super::super::',t)); c['crate::']=len(re.findall(r'\bcrate::',t)); c['super::']=len(re.findall(r'(?<!super::)\bsuper::(?!super|\*)',t))
    c['tests']=len(re.findall(r'#\[(?:tokio::)?(?:test|async_test|\w+::test)\b[^\]]*\]',t))
    print(f"{r['bytes']:>7} {r['lines']:>5} {dict(c)} {r['file'].split('/')[-3]}/{r['file'].split('/')[-2]}")
