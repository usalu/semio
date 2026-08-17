#!/usr/bin/env python3
"""Mechanical W5 facet absorb helper. Usage: python3 w5_migrate_artifact.py note|cad"""
from __future__ import annotations
import json, shutil, sys
from pathlib import Path

ROOT = Path('/Users/ueli/Documents/semio')
TICKET = next((ROOT/'.🦑️repo/🎫️tickets').rglob('STDIO-ARTIFACTS-AND-IO'))
TOK = json.loads((TICKET/'🧪tokens.json').read_text())
REF = ROOT/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json'

TARGETS = {
  'note': ROOT/'✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note',
  'cad': ROOT/'✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad',
}

# dirs that must exist after absorb (relative)
REQUIRED = [
  '🧬️schema/📸️snapshot/�<PRIVATE_USE_AREA>text',
]
# fix: use tokens
TEXT = TOK['text']
BIN = TOK['binary']
BUILDER = TOK['builder']
DECOMPOSER = TOK['decomposer']
DESER = TOK['deserializers']
SER = TOK['serializers']

def ensure_tree(dst: Path):
    # copy empty-ish structure from REF for missing dirs/files (do not overwrite existing rs content blindly)
    mapping = [
        f'🧬️schema/📸️snapshot/{TEXT}',
        f'🧬️schema/📸️snapshot/{BIN}',
        f'🧬️schema/🔺️diff/{TEXT}',
        f'🧬️schema/🔺️diff/{BIN}',
        f'🧬️schema/🧬️mutations/{TEXT}',
        f'🧬️schema/🧬️mutations/{BIN}',
        BUILDER,
        DECOMPOSER,
        f'🚪️io/📥️import/{DESER}/🗿️artifacts',
        f'🚪️io/📤️export/{SER}/🗿️artifacts',
    ]
    for rel in mapping:
        (dst/rel).mkdir(parents=True, exist_ok=True)
        ref = REF/rel
        if ref.exists():
            for f in ref.rglob('*'):
                if f.is_file():
                    out = dst/rel/f.relative_to(ref)
                    out.parent.mkdir(parents=True, exist_ok=True)
                    if not out.exists():
                        shutil.copy2(f, out)

def move_if(src: Path, dst: Path):
    if not src.exists():
        return False
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.exists():
        # merge files
        if src.is_dir():
            for f in src.rglob('*'):
                if f.is_file():
                    t = dst/f.relative_to(src)
                    t.parent.mkdir(parents=True, exist_ok=True)
                    if not t.exists():
                        shutil.copy2(f, t)
            shutil.rmtree(src)
        return True
    shutil.move(str(src), str(dst))
    return True

def absorb(art: Path):
    ensure_tree(art)
    # path map
    move_if(art/'🗣️dsl', art/f'🧬️schema/📸️snapshot/{TEXT}')
    move_if(art/'📸️snapshot'/'🎒️pack', art/f'🧬️schema/📸️snapshot/{BIN}')
    # snapshot schema leaves -> schema/snapshot
    snap_schema = art/'📸️snapshot'/'🧬️schema'
    if snap_schema.exists():
        for f in snap_schema.iterdir():
            if f.is_file():
                t = art/'🧬️schema'/'📸️snapshot'/f.name
                t.parent.mkdir(parents=True, exist_ok=True)
                if not t.exists():
                    shutil.copy2(f, t)
        shutil.rmtree(snap_schema)
    # remaining snapshot component -> schema/snapshot
    snap = art/'📸️snapshot'
    if snap.exists():
        for f in snap.iterdir():
            if f.is_file() or f.name == '🦀️component.rs':
                t = art/'🧬️schema'/'📸️snapshot'/f.name
                if f.is_file() and not t.exists():
                    shutil.copy2(f, t)
            elif f.is_dir() and f.name not in {'🎒️pack','🧬️schema'}:
                t = art/'🧬️schema'/'📸️snapshot'/f.name
                if not t.exists():
                    shutil.copytree(f, t)
        # remove old snapshot after copy
        shutil.rmtree(snap, ignore_errors=True)
    # diff
    diff = art/'🔺️diff'
    if diff.exists():
        # schema subdir
        ds = diff/'🧬️schema'
        if ds.exists():
            for f in ds.iterdir():
                if f.is_file():
                    t = art/'🧬️schema'/'🔺️diff'/f.name
                    t.parent.mkdir(parents=True, exist_ok=True)
                    if not t.exists(): shutil.copy2(f, t)
            shutil.rmtree(ds)
        # grammar-like + rs/ts into text
        for f in list(diff.iterdir()):
            if f.is_file():
                t = art/'🧬️schema'/'🔺️diff'/TEXT/f.name
                t.parent.mkdir(parents=True, exist_ok=True)
                if not t.exists(): shutil.copy2(f, t)
        shutil.rmtree(diff, ignore_errors=True)
    # op -> mutations/text
    move_if(art/'🔧️op', art/f'🧬️schema/🧬️mutations/{TEXT}')
    # spr -> mutations/binary
    move_if(art/'📡️spr', art/f'🧬️schema/🧬️mutations/{BIN}')
    # mutations dir
    mut = art/'🧬️mutations'
    if mut.exists() and mut.is_dir():
        dest = art/'🧬️schema'/'🧬️mutations'
        dest.mkdir(parents=True, exist_ok=True)
        for child in list(mut.iterdir()):
            t = dest/child.name
            if not t.exists():
                shutil.move(str(child), str(t))
            else:
                if child.is_dir():
                    shutil.rmtree(child)
                else:
                    child.unlink()
        shutil.rmtree(mut, ignore_errors=True)
    # IO rewrite: old format/import|export -> new
    io = art/'🚪️io'
    if io.exists():
        for fmt in list(io.iterdir()):
            if not fmt.is_dir():
                continue
            if fmt.name in {'📥️import','📤️export'}:
                continue
            # format emoji dir
            for direction, bucket in [('📥️import', DESER), ('📤️export', SER)]:
                old = fmt/direction
                if not old.exists():
                    # also accept import/export without emoji
                    continue
                new = io/direction/bucket/'🗿️artifacts'/fmt.name
                new.mkdir(parents=True, exist_ok=True)
                for f in old.rglob('*'):
                    if f.is_file():
                        t = new/f.relative_to(old)
                        t.parent.mkdir(parents=True, exist_ok=True)
                        if not t.exists():
                            shutil.copy2(f, t)
            shutil.rmtree(fmt)

def verify(art: Path) -> list[str]:
    errs=[]
    for rel in [BUILDER, DECOMPOSER, f'🧬️schema/📸️snapshot/{TEXT}', f'🧬️schema/📸️snapshot/{BIN}',
                f'🚪️io/📥️import/{DESER}/🗿️artifacts', f'🚪️io/📤️export/{SER}/🗿️artifacts']:
        if not (art/rel).exists():
            errs.append(f'missing {rel}')
    for old in ['🗣️dsl','📸️snapshot','🔺️diff','🔧️op','📡️spr']:
        if (art/old).exists():
            errs.append(f'old facet remains {old}')
    # root mutations should be gone
    if (art/'🧬️mutations').exists() and not str(art/'🧬️mutations').endswith('schema/🧬️mutations'):
        # only root
        if (art/'🧬️mutations').parent == art:
            errs.append('old root 🧬️mutations remains')
    return errs

def main():
    if len(sys.argv)<2 or sys.argv[1] not in TARGETS:
        print('usage: w5_migrate_artifact.py note|cad'); sys.exit(2)
    key=sys.argv[1]
    art=TARGETS[key]
    absorb(art)
    errs=verify(art)
    out=TICKET/f'🧪w5-{key}-mechanical-verify.json'
    out.write_text(json.dumps({'art':str(art),'errors':errs,'top':sorted(p.name for p in art.iterdir())},indent=2))
    print(json.dumps({'errors':errs,'top':sorted(p.name for p in art.iterdir())},indent=2))
    sys.exit(1 if errs else 0)

if __name__=='__main__':
    main()
