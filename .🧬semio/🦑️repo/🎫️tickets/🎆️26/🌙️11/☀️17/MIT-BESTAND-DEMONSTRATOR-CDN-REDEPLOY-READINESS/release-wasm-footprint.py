"""🔢️ Inventory defined Wasm function counts in the last staged main-plugin cores."""
import os
from pathlib import Path
import sys

ROOTS=("cad","demonstrator","energy","fem","flow","gis","playbook",
       "procedural","process","puzzle","sourcing")

def uint(stream):
    value=0
    for shift in range(0,35,7):
        item=stream.read(1)
        if not item:
            raise ValueError("incomplete Wasm unsigned integer")
        digit=item[0]
        value|=(digit&127)<<shift
        if not digit&128 and value<=0xffffffff:
            return value
    raise ValueError("oversized Wasm unsigned integer")

CORE=b"\0asm\x01\0\0\0"
COMPONENT=b"\0asm\x0d\0\x01\0"

def sections(stream, end):
    while stream.tell()<end:
        kind=stream.read(1)
        size=uint(stream)
        section_end=stream.tell()+size
        if section_end>end:
            raise ValueError("section crosses its original Wasm boundary")
        yield kind,size,section_end
        stream.seek(section_end,os.SEEK_SET)

def core_functions(stream,end):
    counts={}
    for kind,_,section_end in sections(stream,end):
        if kind in (b"\x03",b"\x0a"):
            if kind in counts:
                raise ValueError("duplicate Wasm function or code section")
            counts[kind]=uint(stream)
            if stream.tell()>section_end:
                raise ValueError("incomplete Wasm function count")
    declared=counts.get(b"\x03",0)
    if counts.get(b"\x0a",0)!=declared:
        raise ValueError("different Wasm function and code counts")
    return declared

def linked_functions(stream,end):
    header=stream.read(8)
    if header==CORE:
        yield end-stream.tell()+8,core_functions(stream,end)
    elif header==COMPONENT:
        for kind,_,section_end in sections(stream,end):
            if kind in (b"\x01",b"\x04"):
                yield from linked_functions(stream,section_end)
    else:
        raise ValueError("unrecognized original Wasm binary header")

def count(path):
    with path.open("rb") as stream:
        if stream.read(8)!=CORE:
            raise ValueError(f"not an original core {path}")
        return core_functions(stream,path.stat().st_size)

def main():
    if sys.argv[1]=="--linked":
        path=Path(sys.argv[2])
        with path.open("rb") as stream:
            for index,(size,functions) in enumerate(linked_functions(stream,path.stat().st_size),1):
                print(f"core {index}: {functions:,} defined functions, {size:,} bytes")
        return
    raw,site=map(Path,sys.argv[1:3])
    print("| Main plug-in | Staged bytes (cores) | Compact bytes (cores) | Max defined functions in one staged core |")
    print("| --- | ---: | ---: | ---: |")
    for name in ROOTS:
        mods=list(raw.glob(f"*️{name}"))
        if len(mods)!=1:
            raise ValueError(f"expected one staged {name} module")
        wasm=sorted(mods[0].glob("*.wasm"))
        if not wasm:
            raise ValueError(f"no core files for {name}")
        served=[site/"🔌️plugin-modules"/mods[0].name/f"{core.name}.gz.bin" for core in wasm]
        if not all(item.is_file() for item in served):
            raise ValueError(f"incomplete compact output for {name}")
        sizes=", ".join(f"{core.stat().st_size:,}" for core in wasm)
        compact=", ".join(f"{core.stat().st_size:,}" for core in served)
        print(f"| {name} | {sizes} | {compact} | {max(map(count,wasm)):,} |")
if __name__=="__main__":
    main()
