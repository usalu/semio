import json, struct, sys
def hx(b): return b.hex()
data = bytes.fromhex("80ff0180ff7fff7f0100000000000080")
def ru(fmt, off): return struct.unpack_from("<"+fmt, data, off)[0]
reads = [
 ["i8",0,ru("b",0)],["u8",0,ru("B",0)],["i16",0,ru("h",0)],["u16",0,ru("H",0)],
 ["i16",2,ru("h",2)],["u16",2,ru("H",2)],["i32",0,ru("i",0)],["u32",0,ru("I",0)],["i32",4,ru("i",4)],["u32",4,ru("I",4)],
 ["i64",8,{"bigint":str(ru("q",8))}],["u64",8,{"bigint":str(ru("Q",8))}],["ptr",8,float(ru("Q",8))],
]
def n(v): return v
def case(id, symbol, args, returns, expect, **kw):
    d = {"id": id, "symbol": symbol, "args": args, "returns": returns, "expect": expect}; d.update(kw); return d
B = lambda h, view="Buffer", **kw: dict({"bytes": h, "view": view}, **kw)
calls = [
 case("abs-negative","abs",[["i32",-7]],"i32",7),
 case("abs-bool-true","abs",[["i32",True]],"i32",1),
 case("abs-float-number-truncates","abs",[["i32",-7.9]],"i32",7),
 case("abs-null-is-zero","abs",[["i32",None]],"i32",0),
 case("abs-returns-i8-truncates","abs",[["i32",70000]],"i8",112),
 case("abs-returns-u8-truncates","abs",[["i32",70000]],"u8",112),
 case("abs-returns-i16-sign-extends","abs",[["i32",40000]],"i16",-25536),
 case("abs-returns-u16","abs",[["i32",40000]],"u16",40000),
 case("abs-returns-u32","abs",[["i32",-1]],"u32",1),
 case("abs-returns-bool","abs",[["i32",0]],"bool",False),
 case("abs-returns-bool-true","abs",[["i32",3]],"bool",True),
 case("labs-i64-number","labs",[["i64",-5]],"i64",{"bigint":"5"}),
 case("labs-i64-bigint-large","labs",[["i64",{"bigint":"-9223372036854775807"}]],"i64",{"bigint":"9223372036854775807"}),
 case("labs-u64-negative-wraps","labs",[["u64",{"bigint":"-5"}]],"u64",{"bigint":"5"}),
 case("llabs-returns-u64","llabs",[["i64",{"bigint":"-9223372036854775807"}]],"u64",{"bigint":"9223372036854775807"}),
 case("toupper-u8","toupper",[["i32",97]],"u8",65),
 case("toupper-i8","toupper",[["u8",122]],"i8",90),
 case("strlen-cstring-string","strlen",[["cstring",{"string":"héllo"}]],"u64",{"bigint":"6"}),
 case("strlen-cstring-buffer","strlen",[["cstring",B("616263 00".replace(" ",""))]],"u64",{"bigint":"3"}),
 case("strlen-ptr-buffer","strlen",[["ptr",B("68656c6c6f00")]],"u64",{"bigint":"5"}),
 case("strlen-ptr-uint8array-offset","strlen",[["ptr",B("7a7a61626300","Uint8Array",offset=2)]],"u64",{"bigint":"3"}),
 case("strlen-ptr-dataview-offset","strlen",[["ptr",B("7a7a7a6162636400","DataView",offset=3)]],"u64",{"bigint":"4"}),
 case("strlen-ptr-arraybuffer","strlen",[["ptr",B("61626364656600","ArrayBuffer")]],"u64",{"bigint":"6"}),
 case("strlen-ptr-pointer-number","strlen",[["ptr",{"pointer":"b","offset":1}]],"u64",{"bigint":"3"},buffers=[{"id":"b","bytes":"7a61626300"}]),
 case("strtoull-maximum","strtoull",[["cstring",{"string":"18446744073709551615"}],["ptr",None],["i32",10]],"u64",{"bigint":"18446744073709551615"}),
 case("strtoll-negative","strtoll",[["cstring",{"string":"-9223372036854775807"}],["ptr",None],["i32",10]],"i64",{"bigint":"-9223372036854775807"}),
 case("strtoll-hex","strtoll",[["cstring",{"string":"ff"}],["ptr",None],["i32",16]],"i64",{"bigint":"255"}),
 case("memset-fills-and-returns-destination","memset",[["ptr",{"ref":"b"}],["i32",65],["u64",3]],"ptr",{"pointer":"b","offset":0},buffers=[{"id":"b","bytes":"0000000000"}],after={"b":"4141410000"}),
 case("memcpy-copies-bytes","memcpy",[["ptr",{"ref":"b"}],["ptr",B("cafebabe")],["u64",4]],"ptr",{"pointer":"b","offset":0},buffers=[{"id":"b","bytes":"0000000000"}],after={"b":"cafebabe00"}),
 case("memchr-finds","memchr",[["ptr",{"ref":"b"}],["i32",3],["u64",5]],"ptr",{"pointer":"b","offset":2},buffers=[{"id":"b","bytes":"0102030405"}]),
 case("memchr-misses-null","memchr",[["ptr",{"ref":"b"}],["i32",9],["u64",5]],"ptr",None,buffers=[{"id":"b","bytes":"0102030405"}]),
 case("memcmp-equal","memcmp",[["ptr",B("0102")],["ptr",B("0102")],["u64",2]],"i32",0),
 case("getenv-cstring","getenv",[["cstring",{"string":"SEMIO_NODEFFI_CORPUS"}]],"cstring",{"string":"wörld"}),
 case("getenv-unset-cstring-null","getenv",[["cstring",{"string":"SEMIO_NODEFFI_CORPUS_UNSET"}]],"cstring",None),
 case("getenv-unset-ptr-null","getenv",[["cstring",{"string":"SEMIO_NODEFFI_CORPUS_UNSET"}]],"ptr",None),
 case("getpid-i32","getpid",[],"i32",{"context":"pid"}),
 case("getpid-u32","getpid",[],"u32",{"context":"pid"}),
 case("getpid-i64","getpid",[],"i64",{"context":"pidBigint"}),
 case("getpid-u64","getpid",[],"u64",{"context":"pidBigint"}),
 case("getpid-ptr","getpid",[],"ptr",{"context":"pid"}),
 case("getppid-i32","getppid",[],"i32",{"context":"ppid"}),
 case("access-missing-sets-errno","access",[["cstring",{"string":"/nonexistent/semio-nodeffi"}],["i32",0]],"i32",-1,errno=2),
 case("access-root-clears-nothing","access",[["cstring",{"string":"/"}],["i32",0]],"i32",0),
 case("void-return","srand",[["u32",1]],"void",{"void":True}),
]
fixture = {
 "version": 1,
 "environment": {"SEMIO_NODEFFI_CORPUS": "wörld"},
 "calls": calls,
 "memory": [{"id":"read-widths","bytes":hx(data),"reads":reads}],
 "view": {"id":"toarraybuffer-is-live","bytes":"0102030405060708","offset":2,"length":4,"fill":66,"fillLength":4,"expectBefore":"03040506","expectAfter":"42424242"},
 "refusals": [
  {"id":"missing-symbol","library":"libc","symbol":"semio_nodeffi_missing","args":[],"returns":"i32"},
  {"id":"missing-library","library":"/nonexistent/semio-nodeffi.dylib","symbol":"x","args":[],"returns":"i32"},
 ],
 "host": ["rename-swap","statfs","case-sensitivity","process-tree"],
}
print(json.dumps(fixture, ensure_ascii=False, indent=1))
