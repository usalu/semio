import json,subprocess,sys

def uleb(v):
    out=bytearray()
    while True:
        b=v&127;v>>=7
        if v: out.append(b|128)
        else: out.append(b);return bytes(out)

def sleb(v):
    out=bytearray()
    while True:
        b=v&127;v>>=7
        if (v==0 and not b&64) or (v==-1 and b&64): out.append(b);return bytes(out)
        out.append(b|128)

I32C=lambda v:b"\x41"+sleb(v)
I64C=lambda v:b"\x42"+sleb(v)
OP={"i32.add":b"\x6a","i32.mul":b"\x6c","nop":b"\x01","drop":b"\x1a","end":b"\x0b","return":b"\x0f","i32.wrap_i64":b"\xa7","select":b"\x1b"}
def op(name):return OP[name]
def local(kind,i):return {"get":b"\x20","set":b"\x21","tee":b"\x22"}[kind]+uleb(i)
def glob(kind,i):return {"get":b"\x23","set":b"\x24"}[kind]+uleb(i)
def mem(name,align,off=0):
    code={"i32.load":0x28,"i64.load":0x29,"i32.load8_u":0x2d,"i32.store":0x36,"i64.store":0x37,"i32.store8":0x3a}[name]
    return bytes([code])+uleb(align)+uleb(off)

def section(i,data):return bytes([i])+uleb(len(data))+data
def module(locals_,body):
    code=(bytes([1,0x01,0x7f]) if locals_ else b"\x00")+body
    code=uleb(len(code))+code
    return b"\0asm\x01\0\0\0"+section(1,b"\x01\x60\x00\x01\x7f")+section(3,b"\x01\x00")+section(5,b"\x01\x01\x01\x02")+section(6,b"\x01\x7f\x01\x41\x00\x0b")+section(7,b"\x01\x03run\x00\x00")+section(12,b"\x02")+section(10,b"\x01"+code)+section(11,b"\x02\x00\x41\x00\x0b\x06abcdef\x01\x04ghij")

def prog(*items):
    body=b"";static=[];kinds=[]
    for item in items:
        kind,raw=item[0],item[1]
        body+=raw;static.append(kind)
        if len(item)<3 or item[2]:kinds.append(kind)
    return body,static,kinds

FILL=b"\xfc\x0b\x00";COPY=b"\xfc\x0a\x00\x00";INIT=lambda seg:b"\xfc\x08"+uleb(seg)+b"\x00";DROPD=lambda seg:b"\xfc\x09"+uleb(seg)
SIZE=b"\x3f\x00";GROW=b"\x40\x00"
IF=b"\x04\x40";BR=lambda d:b"\x0c"+uleb(d);BRIF=lambda d:b"\x0d"+uleb(d);BRT=lambda labels,default:b"\x0e"+uleb(len(labels))+b"".join(uleb(l) for l in labels)+uleb(default)

EFFECT={"i32.const":1,"i64.const":1,"i32.add":-1,"i32.mul":-1,"local.tee":0,"local.get":1,"local.set":-1,"global.set":-1,"global.get":1,"nop":0,"drop":-1,"select":-2,"i32.store":-2,"i64.store":-2,"i32.store8":-2,"i32.load":0,"i64.load":0,"i32.load8_u":0,"i32.wrap_i64":0,"if":-1,"br_if":-1,"br":0,"br_table":-1,"return":0,"end":0,"memory.fill":-3,"memory.copy":-3,"memory.init":-3,"data.drop":0,"memory.size":1,"memory.grow":0}
def stack_peak(kinds):
    depth=peak=0
    for kind in kinds:
        depth+=EFFECT[kind];peak=max(peak,depth)
    return peak


cases=[]
def add(name,locals_,items,retained,moved=()):
    body,static,kinds=prog(*items,("end",b"\x0b",not items[-1][0] in("br","return","br_table")))
    cases.append({"name":name,"moduleHex":module(locals_,body).hex(),"static":static,"kinds":kinds,"instructions":len(kinds),"stackPeak":stack_peak(kinds),"labelVectors":1,"retained":[{"kind":k,"bytes":b} for k,b in retained],"moved":[{"kind":k,"bytes":b} for k,b in moved],"workUnits":sum(max(1,-(-b//64)) for _,b in moved),"items":len(retained),"copyBytes":sum(b for _,b in retained)})

add("scalar instructions post work only",True,[("i32.const",I32C(7)),("i32.const",I32C(5)),("i32.add",op("i32.add")),("local.tee",local("tee",0)),("i32.const",I32C(3)),("i32.mul",op("i32.mul")),("global.set",glob("set",0)),("global.get",glob("get",0)),("local.get",local("get",0)),("i32.add",op("i32.add")),("nop",op("nop")),("i32.const",I32C(99)),("drop",op("drop"))],[])
add("select is a scalar instruction",False,[("i32.const",I32C(11)),("i32.const",I32C(22)),("i32.const",I32C(0)),("select",op("select"))],[])
add("loads and stores into born linear memory post work only",False,[("i32.const",I32C(8)),("i32.const",I32C(0x01020304)),("i32.store",mem("i32.store",2)),("i32.const",I32C(8)),("i32.load",mem("i32.load",2)),("i32.const",I32C(9)),("i32.load8_u",mem("i32.load8_u",0)),("i32.add",op("i32.add")),("i32.const",I32C(16)),("i64.const",I64C(0x1122334455667788)),("i64.store",mem("i64.store",3)),("i32.const",I32C(16)),("i64.load",mem("i64.load",3)),("i32.wrap_i64",op("i32.wrap_i64")),("i32.add",op("i32.add")),("i32.const",I32C(40)),("i32.const",I32C(0xAB)),("i32.store8",mem("i32.store8",0)),("i32.const",I32C(40)),("i32.load8_u",mem("i32.load8_u",0)),("i32.add",op("i32.add"))],[])
add("untaken if and untaken br_if move nothing",False,[("i32.const",I32C(0)),("if",IF),("i32.const",I32C(1),False),("drop",op("drop"),False),("end",b"\x0b",False),("i32.const",I32C(5)),("i32.const",I32C(0)),("br_if",BRIF(0))],[])
add("taken branch to the function label with its result in place",False,[("i32.const",I32C(5)),("br",BR(0))],[])
add("return with its result in place",False,[("i32.const",I32C(7)),("return",op("return"))],[])
add("br_table to the function label with its result in place",False,[("i32.const",I32C(6)),("i32.const",I32C(1)),("br_table",BRT([0],0))],[])
add("bulk memory pays the actual length and data.drop pays its release",False,[("i32.const",I32C(0)),("i32.const",I32C(97)),("i32.const",I32C(100)),("memory.fill",FILL),("i32.const",I32C(200)),("i32.const",I32C(0)),("i32.const",I32C(64)),("memory.copy",COPY),("i32.const",I32C(400)),("i32.const",I32C(0)),("i32.const",I32C(4)),("memory.init",INIT(1)),("data.drop",DROPD(1)),("i32.const",I32C(205)),("i32.load8_u",mem("i32.load8_u",0)),("i32.const",I32C(400)),("i32.load8_u",mem("i32.load8_u",0)),("i32.add",op("i32.add"))],[("data.drop",0)],[("memory.fill",100),("memory.copy",64),("memory.init",4)])
add("a large fill is bounded by work, not by the copy ledger",False,[("i32.const",I32C(0)),("i32.const",I32C(7)),("i32.const",I32C(60000)),("memory.fill",FILL),("i32.const",I32C(100)),("i32.load8_u",mem("i32.load8_u",0))],[],[("memory.fill",60000)])
add("zero length bulk, memory.size and trivial memory.grow post work only",False,[("i32.const",I32C(0)),("i32.const",I32C(0)),("i32.const",I32C(0)),("memory.fill",FILL),("memory.size",SIZE),("i32.const",I32C(0)),("memory.grow",GROW),("i32.add",op("i32.add")),("i32.const",I32C(9)),("memory.grow",GROW),("i32.add",op("i32.add"))],[])


def calls_module(count):
    f=b"\x00"+b"\x20\x00"+I32C(1)+b"\x6a"+b"\x0b"
    run=b"\x00"+I32C(0)+b"\x10\x01"*count+b"\x0b"
    code=b"\x02"+uleb(len(run))+run+uleb(len(f))+f
    return b"\0asm\x01\0\0\0"+section(1,b"\x02\x60\x00\x01\x7f\x60\x01\x7f\x01\x7f")+section(3,b"\x02\x00\x01")+section(7,b"\x01\x03run\x00\x00")+section(10,code)

calls=[{"name":"%d sequential calls of one function"%n,"count":n,"moduleHex":calls_module(n).hex()} for n in (1,50)]

js="""
const fs=require('fs');const cases=JSON.parse(fs.readFileSync(0,'utf8'));
for(const c of cases){const bytes=Uint8Array.from(Buffer.from(c.moduleHex,'hex'));const i=new WebAssembly.Instance(new WebAssembly.Module(bytes));c.expected=i.exports.run();}
process.stdout.write(JSON.stringify(cases));
"""
out=subprocess.run(["node","-e",js],input=json.dumps(cases+calls),capture_output=True,text=True,check=True).stdout
both=json.loads(out)
cases=both[:len(cases)];calls=both[len(cases):]
json.dump({"version":1,"workBytesPerUnit":64,"cases":cases,"calls":calls},open(sys.argv[1],"w"),indent=2)
print(sys.argv[1],[(c["name"][:30],c["expected"],c["instructions"],len(c["static"])) for c in cases])
