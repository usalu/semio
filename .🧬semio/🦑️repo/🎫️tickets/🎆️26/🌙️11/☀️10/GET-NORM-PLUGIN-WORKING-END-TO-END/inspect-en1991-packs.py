from pathlib import Path
import struct
import zlib
import re
import shlex

root = Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets')

def varint(data, pos):
    value = shift = 0
    for _ in range(10):
        byte = data[pos]
        pos += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, pos
        shift += 7
    raise ValueError('overlong varint')

def segments(path):
    data = path.read_bytes()
    assert len(data) < 100000 and data[:8] == b'\x89SEM\r\n\x1a\n'
    pos = 12 + int.from_bytes(data[8:12], 'little')
    assert data[pos:pos+8] == b'\x89SPK\r\n\x1a\n'
    pos += 32
    result = []
    while pos < len(data)-84:
        kind, flags = data[pos:pos+2]
        pos += 2
        length, pos = varint(data, pos)
        rawlen = length
        if flags & 1:
            rawlen, pos = varint(data, pos)
        raw = data[pos:pos+length]
        pos += length + 4
        if flags & 1:
            assert flags >> 1 == 1
            raw = zlib.decompress(raw, -15)
        assert len(raw) == rawlen
        result.append((kind, raw))
    return result

class Reader:
    def __init__(self, data, symbols):
        self.data, self.pos, self.symbols = data, 0, symbols
    def byte(self):
        b = self.data[self.pos]
        self.pos += 1
        return b
    def number(self):
        n, self.pos = varint(self.data, self.pos)
        return n
    def float(self):
        v = struct.unpack_from('<d', self.data, self.pos)[0]
        self.pos += 8
        return v
    def record(self):
        return {self.number(): self.value() for _ in range(self.number())}
    def value(self):
        tag = self.byte()
        if tag == 0: return None
        if tag in (1, 2): return tag == 2
        if tag == 3:
            n = self.number()
            return (n >> 1) ^ -(n & 1)
        if tag in (4, 10): return self.number()
        if tag == 5: return self.float()
        if tag == 6: return self.symbols[self.number()]
        if tag == 7:
            n = self.number()
            value = self.data[self.pos:self.pos+n].decode()
            self.pos += n
            return value
        if tag in (11, 12): return [self.value() for _ in range(self.number())]
        if tag == 13: return self.record()
        if tag == 20:
            rows, columns = self.number(), self.number()
            result = [{} for _ in range(rows)]
            for _ in range(columns):
                field, presence = self.number(), self.byte()
                bitmap = None
                if presence:
                    n = (rows+7)//8
                    bitmap = self.data[self.pos:self.pos+n]
                    self.pos += n
                elem = self.byte()
                if elem == 1:
                    n = (rows+7)//8
                    bits = self.data[self.pos:self.pos+n]
                    self.pos += n
                for i in range(rows):
                    if bitmap is not None and not bitmap[i//8] & (1 << i%8): continue
                    if elem == 0: value = self.value()
                    elif elem == 1: value = bool(bits[i//8] & (1 << i%8))
                    elif elem == 2:
                        n = self.number()
                        value = (n >> 1) ^ -(n & 1)
                    elif elem in (3, 6): value = self.number()
                    elif elem == 4: value = self.float()
                    elif elem == 5: value = self.symbols[self.number()]
                    else: raise ValueError(('elem', elem))
                    result[i][field] = value
            return result
        raise ValueError(('tag', tag, self.pos))

def decode(path):
    blocks = dict(segments(path))
    reader = Reader(blocks[3], [])
    count = reader.number()
    symbols = []
    for _ in range(count):
        n = reader.number()
        symbols.append(reader.data[reader.pos:reader.pos+n].decode())
        reader.pos += n
    assert reader.pos == len(reader.data)
    reader = Reader(blocks[4], symbols)
    result = reader.record()
    assert reader.pos == len(reader.data), (reader.pos, len(reader.data))
    return result

for name in ('🏢de-office-compliant', '⚠️multi-fail-noncompliant'):
    records = []
    for directory in (root/name, root/name/name):
        pack = directory/'🎒️.pack.semio'
        print(name, 'nested' if directory.name == directory.parent.name else 'outer', [(kind, len(data), data[:12].hex()) for kind, data in segments(pack)])
        records.append(decode(pack))
    for key in sorted(set(records[0]) | set(records[1])):
        if records[0].get(key) != records[1].get(key):
            print('  field', key, 'outer=', repr(records[0].get(key)), 'nested=', repr(records[1].get(key)))
    dsl=(root/name/name/'🗣️.dsl.semio').read_text()
    tokens=dict(token.split('=',1) for token in shlex.split(dsl.splitlines()[1]))
    snapshot=Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs').read_text()
    body=snapshot.split('pub struct En1991Snapshot {',1)[1].split('\n}',1)[0]
    fields=re.findall(r'^\s+pub (\w+): ([^,]+),', body, re.M)
    for i,(field,typ) in enumerate(fields):
        if typ.startswith('Vec<'): break
        token=tokens[field.replace('_','-')]
        value=records[0][i]
        if typ=='bool': expected=token=='true'
        elif typ=='String': expected=token
        elif typ in ('f64','u8'):
            match=re.fullmatch(r'(-?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?)(.*)',token)
            assert match
            expected=float(match[1])
        else:
            enums={'annex':{'de':1},'fire_mode':{'none':0},'fire_curve':{'standard':0},'fire_occupancy':{'office':1},'structure_kind':{'building':0},'fire_load_basis':{'occupancy-table':0}}
            expected=enums[field][token]
        if value != expected: print('  scalar mismatch',field,token,repr(value))
    tables=re.findall(r'(?m)^([a-z0-9-]+) \[([^\n]+)\] \{\n(.*?)^\}',dsl,re.S | re.M)
    table_fields=[(i,n.replace('_','-')) for i,(n,t) in enumerate(fields) if t.startswith('Vec<')]
    assert [n for n,_,_ in tables] == [n for _,n in table_fields]
    for (index,name),(_,header,body) in zip(table_fields,tables):
        rows=[line.strip() for line in body.splitlines() if line.strip()]
        packed=records[0][index]
        assert len(rows)==len(packed),(name,rows,packed)
        if name=='cases':
            assert rows == ['case-e11-04 "Imposed-load reduction of the office floors" "Nutzlastabminderung der Bürogeschosse" E11.04 [ {id=area value=30} {id=storeys value=4} ] [ {id=category value=A} ]']
            assert packed == [{0:'case-e11-04',1:'Imposed-load reduction of the office floors',2:'Nutzlastabminderung der Bürogeschosse',3:'E11.04',4:[{0:'area',1:30.0},{0:'storeys',1:4.0}],5:[{0:'category',1:'A'}]}]
        elif name=='accidental-cases' and rows:
            assert rows == ['impact-1 [ { vehicle-mass=15000kg vehicle-speed=13.9m/s assumed-force=50000N } ] [ ]']
            print('  accidental packed',packed)
        else:
            columns=[col.split(':') for col in header.split()]
            for line,record in zip(rows,packed):
                values=shlex.split(line)
                assert len(values)==len(columns)
                for j,((column,kind),token) in enumerate(zip(columns,values)):
                    if kind=='TEXT': expected=token
                    elif kind=='BOOL': expected=token=='true'
                    else:
                        match=re.fullmatch(r'(-?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?)(.*)',token)
                        assert match,(name,column,token)
                        expected=float(match[1])
                    assert record[j]==expected,(name,column,record[j],expected)
    print('  verified scalar and table values against DSL except reported accidental nested row')
