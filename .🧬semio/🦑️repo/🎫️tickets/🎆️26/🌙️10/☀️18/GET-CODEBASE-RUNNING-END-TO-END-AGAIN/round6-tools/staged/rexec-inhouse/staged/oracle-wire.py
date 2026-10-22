#!/usr/bin/env python3
# Independent oracle of the wire codec: reads the same generated schema and, for every line of stdin `{"message": name, "hex": ...}`, prints `{"ok": true, "value": ...}` (bytes as hex, 64-bit integers as numbers, field names as in the schema,
# every scalar defaulted, repeated fields as lists) or `{"ok": false}` when the bytes are malformed. Standard library only; written from the protobuf encoding specification, not from the TypeScript codec.
import json, struct, sys

schema = json.load(open(sys.argv[1], encoding="utf8"))
MESSAGES = {name: {row["number"]: row for row in rows} for name, rows in schema["messages"].items()}
VARINT = {"bool", "enum", "int32", "int64", "uint32", "uint64"}


class Bad(Exception):
    pass


def varint(data, i, end):
    result = shift = 0
    for count in range(10):
        if i >= end:
            raise Bad("truncated")
        b = data[i]; i += 1
        result |= (b & 0x7F) << shift
        shift += 7
        if not b & 0x80:
            return result, i
    raise Bad("long")


def signed(value, bits):
    value &= (1 << 64) - 1
    return value - (1 << 64) if value >= 1 << 63 else value


def scalar(row, value):
    kind = row["type"]
    if kind == "bool": return value != 0
    if kind in ("int32", "int64"):
        value = signed(value, 64)
    if kind in ("uint32", "uint64", "enum", "int32", "int64") and abs(value) > 2 ** 53 - 1:
        raise Bad("range")
    return value


def default(row):
    if row["repeated"]: return []
    return {"string": "", "bytes": "", "bool": False, "message": None}.get(row["type"], 0)


def decode(name, data, begin, end):
    rows = MESSAGES[name]
    out = {row["name"]: default(row) for row in rows.values() if row["repeated"] or row["type"] != "message"}
    i = begin
    while i < end:
        key, i = varint(data, i, end)
        wire, number = key & 7, key >> 3
        if number == 0: raise Bad("number")
        row = rows.get(number)
        expected = 1 if row and row["type"] == "double" else 5 if row and row["type"] == "float" else 0 if row and row["type"] in VARINT else 2
        packed = row is not None and wire == 2 and row["repeated"] and (row["type"] in VARINT or row["type"] in ("double", "float"))
        if row is None or (wire != expected and not packed):
            if wire == 0: _, i = varint(data, i, end)
            elif wire == 1: i += 8
            elif wire == 5: i += 4
            elif wire == 2:
                n, i = varint(data, i, end); i += n
            else: raise Bad("wire")
            if i > end: raise Bad("truncated")
            continue
        kind = row["type"]
        if wire == 2 and row["repeated"] and (kind in VARINT or kind in ("double", "float")):
            n, i = varint(data, i, end)
            stop = i + n
            if stop > end: raise Bad("truncated")
            while i < stop:
                if kind in VARINT:
                    v, i = varint(data, i, stop); out[row["name"]].append(scalar(row, v))
                else:
                    width = 8 if kind == "double" else 4
                    if i + width > stop: raise Bad("truncated")
                    out[row["name"]].append(struct.unpack("<d" if kind == "double" else "<f", data[i:i + width])[0]); i += width
            continue
        if kind in VARINT:
            v, i = varint(data, i, end); value = scalar(row, v)
        elif kind == "double":
            if i + 8 > end: raise Bad("truncated")
            value = struct.unpack("<d", data[i:i + 8])[0]; i += 8
        elif kind == "float":
            if i + 4 > end: raise Bad("truncated")
            value = struct.unpack("<f", data[i:i + 4])[0]; i += 4
        else:
            n, i = varint(data, i, end)
            if i + n > end: raise Bad("truncated")
            chunk = data[i:i + n]
            if kind == "string":
                try: value = chunk.decode("utf8")
                except UnicodeDecodeError: raise Bad("utf8")
            elif kind == "bytes": value = chunk.hex()
            else: value = decode(row["message"], data, i, i + n)
            i += n
        if row["repeated"]: out[row["name"]].append(value)
        else: out[row["name"]] = value
    return out


for line in sys.stdin:
    request = json.loads(line)
    data = bytes.fromhex(request["hex"])
    try:
        print(json.dumps({"ok": True, "value": decode(request["message"], data, 0, len(data))}, ensure_ascii=False, allow_nan=True))
    except (Bad, IndexError, RecursionError):
        print(json.dumps({"ok": False}))
    sys.stdout.flush()
