import os,sys
U="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"
HERE=os.path.dirname(os.path.abspath(__file__))
edits=[]
def sub(path,old,new):
    edits.append((U+path,old,new))
K="🔏️key/🟦️.ts"
sub(K,'''  if(argument.startsWith("--diagnostic-width="))continue;
''','''  if(argument.startsWith("--diagnostic-width="))continue;
  if(argument==="-Z"&&(args[index+1]??"").startsWith("threads=")){index++;continue;}
  if(argument.startsWith("-Zthreads="))continue;
''')
sub(K,'''export function neutralRustflagsV1(encoded:string):string{
 const parts=encoded.split("\\u001f");
 return parts.map(''','''export function neutralRustflagsV1(encoded:string):string{
 const parts=withoutFrontEndThreadsV1(encoded.split("\\u001f"));
 return parts.map(''')
sub(K,'''/** 🚩️ The Cargo-encoded compiler flags (separated by U+001F)''','''/** 🧵️ The flags without the front-end thread count (`-Z threads=N` in both spellings): how many threads the front end uses changes the speed and the memory of a compilation, never its output (a unit that is shared through the fleet always compiles with one thread, see {@link deterministicFrontEndArgumentsV1}), and it differs between the launch paths of Cargo, the hosts and the moments (the repository's `-Z threads=0`, doubled or missing, the memory policy of `🧵️threads`), so it never enters a key. */
export function withoutFrontEndThreadsV1(flags:readonly string[]):string[]{
 const kept:string[]=[];
 for(let index=0;index<flags.length;index++){
  const flag=flags[index]!;
  if(flag==="-Z"&&(flags[index+1]??"").startsWith("threads=")){index++;continue;}
  if(flag.startsWith("-Zthreads="))continue;
  kept.push(flag);
 }
 return kept;
}

/** 🚩️ The Cargo-encoded compiler flags (separated by U+001F)''')
sub("🧬️schema/🔣️.json",'''            "CARGO_BIN_NAME",
            "CARGO_MANIFEST_DIR",
            "CARGO_ENCODED_RUSTFLAGS",
            "RUSTC_BOOTSTRAP",''','''            "CARGO_BIN_NAME",
            "CARGO_MANIFEST_DIR",
            "RUSTC_BOOTSTRAP",''')
sub("🔏️key/🧪️tests/🐍️oracle/🐍️.py",'''        if argument == "--diagnostic-width":''','''        if argument == "-Z" and following.startswith("threads="):
            index += 2
            continue
        if argument.startswith("-Zthreads="):
            index += 1
            continue
        if argument == "--diagnostic-width":''')
P="🧪️tests/🧳️portable/🟦️.ts"
sub(P,'''["joined remapping in the encoded flags","-Zthreads=0\\u001f--remap-path-prefix=/home/deck/.cargo/registry/src=","-Zthreads=0\\u001f--remap-path-prefix="+REGISTRY+"="],''','''["joined remapping in the encoded flags","-Cdebuginfo=1\\u001f--remap-path-prefix=/home/deck/.cargo/registry/src=","-Cdebuginfo=1\\u001f--remap-path-prefix="+REGISTRY+"="],''')
sub(P,'''["flags without a path stay as they are","-Z\\u001fthreads=0","-Z\\u001fthreads=0"],''','''["flags without a path stay as they are","-C\\u001fdebuginfo=1","-C\\u001fdebuginfo=1"],
 ["the front-end thread count is no input (two spellings, any count)","-Z\\u001fthreads=0\\u001f-Zthreads=4\\u001f-C\\u001fdebuginfo=1\\u001f-Z\\u001fthreads=0","-C\\u001fdebuginfo=1"],''')
C="🧪️tests/🔁️churn/🟦️.ts"
sub(C,'''CARGO_ENCODED_RUSTFLAGS:"-Zthreads=4"}),"real rustflags are still inputs").not.toBe(plain);''','''CARGO_ENCODED_RUSTFLAGS:"-Zthreads=4"}),"the front-end thread count is no input").toBe(plain);
 expect(keyOf(cargoArguments(["-Z","threads=0","-Zthreads=3"]),{CARGO_PKG_NAME:"dep"}),"neither in the arguments").toBe(plain);
 expect(keyOf(cargoArguments(["-C","target-cpu=native"]),{CARGO_PKG_NAME:"dep"}),"real rustflags are still inputs").not.toBe(plain);''')
sub(C,'expect(core.differs).toEqual(["args.features"]);','expect(core.differs,"Cargo\'s own unit identity (`-C extra-filename`) follows the features").toEqual(["args.features","cargo"]);')
sub(C,'cause.components.join()==="args.features"&&','cause.components.join()==="args.features,cargo"&&')
# verify all, then write all (atomic per file)
state={}
for path,old,new in edits:
    s=state.get(path) or open(path,encoding='utf-8').read()
    assert s.count(old)==1,(path,old[:50],s.count(old))
    state[path]=s.replace(old,new)
if "--check" in sys.argv: print("ok",len(edits));sys.exit(0)
for path,s in state.items():
    tmp=path+".h81.tmp";open(tmp,'w',encoding='utf-8').write(s);os.replace(tmp,path)
d=U+"🧪️tests/🚩️flags/";os.makedirs(d,exist_ok=True)
tmp=d+"🟦️.ts.tmp";open(tmp,'w',encoding='utf-8').write(open(HERE+"/flags-test.ts",encoding='utf-8').read());os.replace(tmp,d+"🟦️.ts")
print("landed",len(state),"files + test")
