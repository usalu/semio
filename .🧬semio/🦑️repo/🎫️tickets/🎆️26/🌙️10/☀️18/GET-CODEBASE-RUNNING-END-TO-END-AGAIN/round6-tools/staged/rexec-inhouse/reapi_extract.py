#!/usr/bin/env python3
# usage: reapi_extract.py <A-server.jar> <out.json> <bazel version>
# Extracts the serialized FileDescriptorProtos that protoc embedded in Bazel's own generated classes (RemoteExecutionProto, ByteStreamProto, OperationsProto, ...)
# and writes {"source": {jar sha256}, "files": {name: base64}} - the authoritative REAPI definition of the pinned Bazel, used to generate the wire schema and as the test oracle.
import base64, hashlib, json, struct, sys, zipfile

CLASSES = ["build/bazel/semver/SemverProto", "build/bazel/remote/execution/v2/RemoteExecutionProto", "com/google/bytestream/ByteStreamProto", "com/google/longrunning/OperationsProto", "com/google/rpc/StatusProto", "com/google/rpc/ErrorDetailsProto", "com/google/protobuf/AnyProto", "com/google/protobuf/DurationProto", "com/google/protobuf/TimestampProto", "com/google/protobuf/EmptyProto", "com/google/protobuf/WrappersProto"]


def pool(data):
    count = struct.unpack(">H", data[8:10])[0]
    index, entries, offset = 1, [], 10
    while index < count:
        tag = data[offset]
        if tag == 1:
            length = struct.unpack(">H", data[offset + 1:offset + 3])[0]
            entries.append(data[offset + 3:offset + 3 + length])
            offset += 3 + length
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            entries.append(None); offset += 5
        elif tag in (5, 6):
            entries.append(None); entries.append(None); offset += 9; index += 1
        elif tag in (7, 8, 16, 19, 20):
            entries.append(None); offset += 3
        elif tag == 15:
            entries.append(None); offset += 4
        else:
            raise SystemExit(f"constant tag {tag}")
        index += 1
    return entries


def latin1(modified):
    out, i = bytearray(), 0
    while i < len(modified):
        b = modified[i]
        if b < 0x80:
            out.append(b); i += 1
        elif b & 0xE0 == 0xC0:
            out.append(((b & 0x1F) << 6) | (modified[i + 1] & 0x3F)); i += 2
        else:
            return None
    return bytes(out)


def varint(data, i):
    shift = result = 0
    while True:
        b = data[i]; i += 1
        result |= (b & 0x7F) << shift
        if not b & 0x80:
            return result, i
        shift += 7


def parses(data):
    i = 0
    try:
        while i < len(data):
            key, i = varint(data, i)
            wire = key & 7
            if wire == 0: _, i = varint(data, i)
            elif wire == 2:
                n, i = varint(data, i); i += n
            elif wire == 1: i += 8
            elif wire == 5: i += 4
            else: return False
            if i > len(data): return False
    except IndexError:
        return False
    return i == len(data) and len(data) > 0


def descriptor(data):
    strings = [s for s in (latin1(e) for e in pool(data) if e is not None) if s is not None]
    for start, first in enumerate(strings):
        if not first.startswith(b"\n") or len(first) < 8:
            continue
        name_length = first[1]
        if name_length >= 0x80 or not first[2:2 + name_length].endswith(b".proto"):
            continue
        joined = b""
        for part in strings[start:start + 400]:
            joined += part
            if parses(joined):
                return joined
    raise SystemExit("descriptor not found")


def main():
    jar, out = sys.argv[1], sys.argv[2]
    archive = zipfile.ZipFile(jar)
    files = {}
    for name in CLASSES:
        blob = descriptor(archive.read(name + ".class"))
        files[name.rsplit("/", 1)[1]] = base64.b64encode(blob).decode()
    json.dump({"source": {"jar": "A-server.jar", "bazel": sys.argv[3], "sha256": hashlib.sha256(open(jar, "rb").read()).hexdigest()}, "files": files}, open(out, "w"), indent=1)
    print("extracted", {k: len(base64.b64decode(v)) for k, v in files.items()})


main()
