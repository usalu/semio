import hashlib,json,sys
value=json.load(sys.stdin)
def accepted(p):
 return p['launched'] is True and p['joined'] is True and p['known'] is True and p['failed'] is False and p['status']==0 and p['signal'] is None
json.dump({'policySha256':hashlib.sha256(value['policy'].encode()).hexdigest(),'settlements':[accepted(row['proof']) for row in value['settlements']],'prefixes':[all(stage==value['stages'][i] for i,stage in enumerate(row['executed'])) for row in value['observations']]},sys.stdout,separators=(',',':'))
