import json,sys
d=json.load(open(sys.argv[1]))
print(d['numTotalTests'],d['numPassedTests'],d['numFailedTests'])
for f in d['testResults']:
    for t in f['assertionResults']:
        if t['status']!='passed':
            print(t['status'],t['title']); print('   ',(t['failureMessages'] or [''])[0][:600].replace('\n',' | '))
