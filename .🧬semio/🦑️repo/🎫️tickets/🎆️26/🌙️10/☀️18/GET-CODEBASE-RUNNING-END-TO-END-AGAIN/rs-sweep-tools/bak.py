import sys,os,json,hashlib
root='/Users/ueli/Documents/semio'
B=root+'/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rs-sweep-x/backup'
def backup(path):
    path=os.path.abspath(path)
    led=json.load(open(B+'/ledger.json')) if os.path.exists(B+'/ledger.json') else {}
    if path in led: return
    h=hashlib.sha1(path.encode('utf-8')).hexdigest()[:16]
    open(B+'/'+h,'wb').write(open(path,'rb').read())
    led[path]=h
    json.dump(led,open(B+'/ledger.json','w'),indent=1)
if __name__=='__main__':
    for p in sys.argv[1:]: backup(p)
