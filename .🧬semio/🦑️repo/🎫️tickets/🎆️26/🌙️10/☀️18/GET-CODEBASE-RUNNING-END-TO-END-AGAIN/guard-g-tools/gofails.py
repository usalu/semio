import sys,json
for l in open(sys.argv[1],encoding='utf8',errors='replace'):
    if not l.startswith('{'): print(l.rstrip()[:400]); continue
    e=json.loads(l)
    o=e.get('Output','')
    if e.get('Action')=='fail' and e.get('Test'): print('FAILED',e['Test'])
    if e.get('Action')=='output' and e.get('Test') and ('_test.go' in o or 'FAIL:' in o or 'panic' in o): print(' ',e['Test'],o.rstrip()[:500])
    if e.get('Action')=='output' and not e.get('Test') and ('FAIL' in o or 'touched' in o or 'resolved' in o): print(o.rstrip()[:600])
