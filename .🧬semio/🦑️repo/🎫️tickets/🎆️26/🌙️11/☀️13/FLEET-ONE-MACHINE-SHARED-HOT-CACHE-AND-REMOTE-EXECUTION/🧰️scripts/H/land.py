import os,shutil,sys
V="️"
def e(cp,rest=""): return chr(cp)+V+rest
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/H/src"
LIB="/Users/Shared/semio/"+"/".join([e(0x1F9F0,"framework"),e(0x1F6CD,"products"),e(0x1F991,"repo"),e(0x1F528,"modules"),e(0x1F4DA,"library")])
BOOT=LIB+"/"+e(0x26A1,"caching")+"/"+e(0x1F680,"bootstrap")
FF=BOOT+"/"+e(0x1F6F0,"fleet-fetch")
SCHEMA=e(0x1F9EC,"schema"); TESTS=e(0x1F9EA,"tests"); TS=e(0x1F7E6,".ts"); JS=e(0x1F523,".json")
RC=LIB+"/"+e(0x1F3D7,"build")+"/"+e(0x1F310,"remote-cache")
MAP={
 "fleet-fetch.ts":FF+"/"+TS,
 "fleet-fetch.schema.json":FF+"/"+SCHEMA+"/"+JS,
 "fleet-fetch.test.ts":FF+"/"+TESTS+"/"+TS,
 "install.ts":FF+"/"+e(0x1F4E6,"install")+"/"+TS,
 "install.test.ts":FF+"/"+e(0x1F4E6,"install")+"/"+TESTS+"/"+TS,
 "crates.ts":FF+"/"+e(0x1F980,"crates")+"/"+TS,
 "crates.test.ts":FF+"/"+e(0x1F980,"crates")+"/"+TESTS+"/"+TS,
 "mirror-client.ts":FF+"/"+e(0x1FA9E,"mirror")+"/"+TS,
 "mirror-client.test.ts":FF+"/"+e(0x1FA9E,"mirror")+"/"+TESTS+"/"+TS,
 "mirror.ts":RC+"/"+e(0x1FA9E,"mirror")+"/"+TS,
 "mirror.schema.json":RC+"/"+e(0x1FA9E,"mirror")+"/"+SCHEMA+"/"+JS,
 "mirror.test.ts":RC+"/"+e(0x1FA9E,"mirror")+"/"+TESTS+"/"+TS,
}
only=sys.argv[1:] or list(MAP)
for k in only:
    dst=MAP[k]; os.makedirs(os.path.dirname(dst),exist_ok=True)
    shutil.copyfile(os.path.join(S,k),dst); print("landed",k)
