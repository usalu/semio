from pathlib import Path
import re
import zlib

root = Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts')
for family in ('din4108', 'en1991'):
    family_root = next(p for p in root.iterdir() if p.name.endswith(family))
    for pack in sorted(family_root.rglob('*.pack.semio')):
        dsl = next(pack.parent.glob('*.dsl.semio'), None) or next(pack.parent.glob('*/*.dsl.semio'), None)
        if not dsl:
            continue
        data = pack.read_bytes()
        base = 12 + int.from_bytes(data[8:12], 'little')
        for offset in range(base + 30, base + 50):
            try:
                payload = zlib.decompress(data[offset:], -15)
                if len(payload) > 50:
                    break
            except zlib.error:
                pass
        else:
            raise ValueError(f'no compressed payload in {pack}')
        pos = 0
        def varint():
            global pos
            value = shift = 0
            while True:
                b = payload[pos]
                pos += 1
                value |= (b & 127) << shift
                if b < 128:
                    return value
                shift += 7
        count = varint()
        symbols = []
        for _ in range(count):
            size = varint()
            symbols.append(payload[pos:pos + size].decode())
            pos += size
        text = dsl.read_text()
        absent = [s for s in symbols if s and s not in text]
        print(f'{family}/{pack.parent.parent.name}/{pack.parent.name}: {count} symbols; absent from DSL: {absent}')
