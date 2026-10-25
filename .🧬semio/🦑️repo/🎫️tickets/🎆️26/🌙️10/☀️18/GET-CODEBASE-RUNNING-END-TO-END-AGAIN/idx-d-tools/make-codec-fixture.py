import json,sys,importlib.util
spec=importlib.util.spec_from_file_location("oracle",sys.argv[1]);o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
H="a"*64;H2="0123456789abcdef"*4
idx=lambda owners=(),rows=(),classes=():{"version":1,"owners":list(owners),"rows":list(rows),"classes":list(classes)}
row=lambda k,p,f:{"kind":k,"path":p,"fact":f}
deep=lambda n,leaf:leaf if n==0 else [deep(n-1,leaf)]
indexes=[
 {"id":"empty","index":idx()},
 {"id":"owners-only","index":idx(["b","a","@env"],[],[{"r":[],"o":[0]},{"r":[],"o":[1,2]}])},
 {"id":"shared-facts-and-paths","index":idx(["x","y"],[row("exists","/w/a",True),row("exists","/w/b",True),row("exists","/w/c",False),row("lstat","/w/a",None)],[{"r":[0,1],"o":[0,1]},{"r":[2,3],"o":[1]}])},
 {"id":"digest-facts","index":idx(["x"],[row("readdir","/w/dir",H),row("readdir","/w/other",H2),row("field","/w/f",{"name":H,"size":0})],[{"r":[0,1,2],"o":[0]}])},
 {"id":"numbers","index":idx(["x"],[row("file","/w/n",{"zero":0,"one":1,"neg":-1,"big":9007199254740991,"negbig":-9007199254740991,"float":1792653807536.8682,"half":0.5,"neg_float":-2.25,"huge":1.5e300,"tiny":1.5e-300,"integral_float":3.0})],[{"r":[0],"o":[0]}])},
 {"id":"unicode-and-emoji","index":idx(["🚀","é","a"],[row("file","/w/📋️project.json",{"🚀":"quote\" slash\\ tab\t nl\n","emoji":"🧰️","bmp":"ﬁ￿","astral":"\U0001f600"}),row("file","/w/￿",1),row("file","/w/\U0001f600",2)],[{"r":[0,1,2],"o":[0,1,2]}])},
 {"id":"proto-key-and-empty-strings","index":idx([""],[row("field","",{"__proto__":{"x":1},"":""})],[{"r":[0],"o":[0]}])},
 {"id":"nested","index":idx(["x"],[row("analysis","/w/a.ts",{"nestedObservations":[],"provider":"p","source":{"ctimeMs":1792653807536.8682,"ino":558728385,"sha256":H,"size":2366},"list":[[1,2],[3,[4,[5,None]]],{"k":[True,False]}]})],[{"r":[0],"o":[0]}])},
 {"id":"unsorted-class-lists","index":idx(["a","b","c"],[row("exists","/1",True),row("exists","/2",False),row("exists","/3",True)],[{"r":[2,0,1],"o":[2,0]},{"r":[1],"o":[1,2,0]}])},
 {"id":"depth-64","index":idx(["x"],[row("field","/d",deep(64,0))],[{"r":[0],"o":[0]}])},
 {"id":"closure-references","index":idx(["x"],[row("file","/w/c",{"SEMIO_SOURCE_CLOSURE_"+"a"*64:{"fact":"SEMIO_CARGO_LOCK_"+"b"*64}}),row("closure","[\"/w\",\""+H+"\",[\"some\",[\"/a/\"]]]","true")],[{"r":[0,1],"o":[0]}])},
]
def enc(i): return o.encode(i)
base=enc(indexes[2]["index"])
def raw(*parts): return b"".join(parts)
v=o.varint
refusals=[
 ("empty-document",b""),
 ("bad-magic",b"SFI2"+base[4:]),
 ("magic-only",o.MAGIC),
 ("truncated",base[:-1]),
 ("trailing-byte",base+b"\x00"),
 ("count-beyond-bytes",raw(o.MAGIC,v(1000),v(0),v(0),v(0),v(0),v(0))),
 ("strings-descending",raw(o.MAGIC,v(2),v(0),v(0),v(0),v(0),v(0),v(0),v(1),b"b",v(0),v(1),b"a")),
 ("strings-duplicate",raw(o.MAGIC,v(2),v(0),v(0),v(0),v(0),v(0),v(0),v(1),b"a",v(1),v(0))),
 ("lcp-beyond-previous",raw(o.MAGIC,v(2),v(0),v(0),v(0),v(0),v(0),v(0),v(1),b"a",v(5),v(1),b"b")),
 ("first-string-nonzero-lcp",raw(o.MAGIC,v(1),v(0),v(0),v(0),v(0),v(0),v(1),v(1),b"a")),
 ("string-invalid-utf8",raw(o.MAGIC,v(1),v(0),v(0),v(0),v(0),v(0),v(0),v(1),b"\xff")),
 ("bad-tag",raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x0a")),
 ("negative-zero",raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x04",v(0))),
 ("overlong-varint",raw(o.MAGIC,b"\x80"*10+b"\x00")),
 ("object-keys-unsorted",raw(o.MAGIC,v(2),v(0),v(0),v(1),v(0),v(0),v(0),v(1),b"a",v(0),v(1),b"b",b"\x08",v(2),v(1),b"\x00",v(0),b"\x00")),
 ("string-id-out-of-range",raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x06",v(0))),
 ("row-kind-out-of-range",raw(o.MAGIC,v(1),v(0),v(0),v(1),v(1),v(0),v(0),v(1),b"p",b"\x00",v(0),v(0),v(0))),
 ("row-fact-out-of-range",raw(o.MAGIC,v(1),v(1),v(0),v(1),v(1),v(0),v(0),v(1),b"p",v(0),b"\x00",v(0),v(0),v(1))),
 ("class-owner-out-of-range",raw(o.MAGIC,v(0),v(0),v(0),v(0),v(0),v(1),v(0),v(1),v(0))),
 ("class-row-out-of-range",raw(o.MAGIC,v(0),v(0),v(0),v(0),v(0),v(1),v(1),v(0),v(0))),
 ("class-negative-id",raw(o.MAGIC,v(1),v(0),v(1),v(0),v(0),v(1),v(0),v(1),b"o",v(0),v(0),v(1),v(1))),
 ("nesting-65",raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x07\x01"*65,b"\x00")),
 ("truncated-float",raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x05\x00\x00")),
]
nan=raw(o.MAGIC,v(0),v(0),v(0),v(1),v(0),v(0),b"\x05",b"\x00\x00\x00\x00\x00\x00\xf8\x7f")
refusals.append(("float-nan",nan))
out={"version":1,"indexes":indexes,"refusals":[{"id":i,"hex":b.hex()} for i,b in refusals],"sizes":{"owners":300,"rowsPerOwner":200,"maximumExpandedBytes":5500000,"maximumRatio":0.27}}
sys.stdout.write(json.dumps(out,ensure_ascii=False,indent=1))
