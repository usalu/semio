import sys,re
used={l.split('\t')[0].replace('️','') for l in open('/Users/ueli/.cache/c6flow/folder-emojis.txt',encoding='utf-8')}
for e in sys.argv[1:]:
    print(e, 'USED' if e.replace('️','') in used else 'NOT-USED-IN-REPO')
