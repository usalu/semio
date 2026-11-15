import sys,os,json
def root():
    return sys.argv[1]
def read(path):
    return open(os.path.join(root(),*path.split('/')),encoding='utf8').read()
def write(path,text):
    open(os.path.join(root(),*path.split('/')),'w',encoding='utf8').write(text)
def sub(path,old,new,count=1):
    s=read(path)
    n=s.count(old)
    assert n==count,f"{path}: anchor found {n}x, expected {count}: {old[:90]!r}"
    write(path,s.replace(old,new))
def append(path,text):
    write(path,read(path)+text)
def files(name):
    return open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'files',name),encoding='utf8').read()
def json_edit(path,fn,indent=1,newline=''):
    s=json.loads(read(path))
    fn(s)
    write(path,json.dumps(s,indent=indent,ensure_ascii=False)+newline)
