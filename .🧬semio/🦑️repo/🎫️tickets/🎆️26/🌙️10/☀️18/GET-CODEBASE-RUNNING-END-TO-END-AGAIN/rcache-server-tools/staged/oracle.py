"""🐍️ Independent oracle for the remote cache protocol tests: a second implementation, in Python's standard library only, of the strict ActionResult and Tree decoder, of the collection plan and of a protobuf encoder that builds the test vectors. The TypeScript tests compare their answers with this file's; nothing here is shared with the server code.

usage: python3 -I oracle.py vectors            print the named vectors with the verdicts the encoder knows
       python3 -I oracle.py decode             stdin: JSON list of {"kind": "result"|"tree", "hex": ...}; stdout: one verdict per entry
       python3 -I oracle.py plan               stdin: JSON {"rows": [...], "bounds": {...}, "dangling": [...]}; stdout: the removals in order
"""
import hashlib
import json
import sys

HASH_LENGTH = 64
EMPTY = hashlib.sha256(b"").hexdigest()


class Bad(Exception):
    pass


def varint(data, offset):
    result, shift = 0, 0
    for index in range(10):
        if offset + index >= len(data):
            raise Bad("truncated varint")
        byte = data[offset + index]
        if index == 9 and byte & 0x7E:
            raise Bad("varint exceeds 64 bits")
        result |= (byte & 0x7F) << shift
        if not byte & 0x80:
            return result, offset + index + 1
        shift += 7
    raise Bad("varint longer than ten bytes")


def fields(data):
    offset = 0
    while offset < len(data):
        tag, offset = varint(data, offset)
        number, wire = tag >> 3, tag & 7
        if number < 1 or number > 536870911:
            raise Bad("invalid field number")
        if wire == 0:
            value, offset = varint(data, offset)
            yield number, wire, value, b""
        elif wire == 1 or wire == 5:
            length = 8 if wire == 1 else 4
            if offset + length > len(data):
                raise Bad("truncated fixed field")
            yield number, wire, 0, data[offset:offset + length]
            offset += length
        elif wire == 2:
            length, offset = varint(data, offset)
            if length > len(data) - offset:
                raise Bad("length-delimited field exceeds the message")
            yield number, wire, length, data[offset:offset + length]
            offset += length
        else:
            raise Bad("wire type %d is not allowed" % wire)


def need(wire, expected, what):
    if wire != expected:
        raise Bad("%s has wire type %d, expected %d" % (what, wire, expected))


def string(wire, data, what):
    need(wire, 2, what)
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        raise Bad("%s is not UTF-8" % what)


def digest(wire, data, what):
    need(wire, 2, what)
    hash_value, size = None, 0
    for number, w, value, body in fields(data):
        if number == 1:
            hash_value = string(w, body, what + ".hash")
        elif number == 2:
            need(w, 0, what + ".size_bytes")
            if value > 2**53 - 1:
                raise Bad(what + ".size_bytes is out of range")
            size = value
    if hash_value is None or len(hash_value) != HASH_LENGTH or any(c not in "0123456789abcdef" for c in hash_value):
        raise Bad(what + " needs a lowercase sha256 hash")
    if (size == 0) != (hash_value == EMPTY):
        raise Bad(what + " is not consistent with the empty blob")
    return hash_value, size


def result(data):
    references, trees = [], []
    for number, wire, value, body in fields(data):
        if number == 2:
            need(wire, 2, "output_files")
            path, entry, contents = "", None, None
            for n, w, v, b in fields(body):
                if n == 1:
                    path = string(w, b, "output_files.path")
                    if path == "":
                        raise Bad("output_files.path must not be empty")
                elif n == 2:
                    entry = digest(w, b, "output_files.digest")
                elif n == 4:
                    need(w, 0, "output_files.is_executable")
                elif n == 5:
                    need(w, 2, "output_files.contents")
                    contents = b
            if path == "":
                raise Bad("output_files.path is required")
            if entry is None:
                raise Bad("output_files.digest is required")
            if contents is not None:
                if len(contents) != entry[1] or hashlib.sha256(contents).hexdigest() != entry[0]:
                    raise Bad("output_files.contents does not match its digest")
            elif entry[1] > 0:
                references.append(entry)
        elif number == 3:
            need(wire, 2, "output_directories")
            path, tree = "", None
            for n, w, v, b in fields(body):
                if n == 1:
                    path = string(w, b, "output_directories.path")
                    if path == "":
                        raise Bad("output_directories.path must not be empty")
                elif n == 3:
                    tree = digest(w, b, "output_directories.tree_digest")
                elif n == 5:
                    digest(w, b, "output_directories.root_directory_digest")
            if path == "":
                raise Bad("output_directories.path is required")
            if tree is None:
                raise Bad("output_directories.tree_digest is required")
            if tree[1] > 0:
                trees.append(tree)
        elif number == 4:
            need(wire, 0, "exit_code")
        elif number in (5, 7):
            need(wire, 2, "stdout_raw" if number == 5 else "stderr_raw")
        elif number in (6, 8):
            entry = digest(wire, body, "stdout_digest" if number == 6 else "stderr_digest")
            if entry[1] > 0:
                references.append(entry)
        elif number == 9:
            need(wire, 2, "execution_metadata")
            for _ in fields(body):
                pass
        elif number in (10, 11, 12):
            need(wire, 2, "output_symlinks")
            path = ""
            for n, w, v, b in fields(body):
                if n == 1:
                    path = string(w, b, "output_symlinks.path")
                    if path == "":
                        raise Bad("output_symlinks.path must not be empty")
                elif n == 2:
                    string(w, b, "output_symlinks.target")
            if path == "":
                raise Bad("output_symlinks.path is required")
    return references, trees


def tree(data):
    files = []
    for number, wire, value, body in fields(data):
        if number not in (1, 2):
            continue
        need(wire, 2, "tree.directory")
        for n, w, v, b in fields(body):
            if n == 1:
                need(w, 2, "directory.files")
                name, entry = "", None
                for k, kw, kv, kb in fields(b):
                    if k == 1:
                        name = string(kw, kb, "file.name")
                        if name == "":
                            raise Bad("file.name must not be empty")
                    elif k == 2:
                        entry = digest(kw, kb, "file.digest")
                if name == "" or entry is None:
                    raise Bad("file nodes need a name and a digest")
                if entry[1] > 0:
                    files.append(entry)
            elif n == 2:
                need(w, 2, "directory.directories")
                name, entry = "", None
                for k, kw, kv, kb in fields(b):
                    if k == 1:
                        name = string(kw, kb, "directory.name")
                        if name == "":
                            raise Bad("directory.name must not be empty")
                    elif k == 2:
                        entry = digest(kw, kb, "directory.digest")
                if name == "" or entry is None:
                    raise Bad("directory nodes need a name and a digest")
            elif n == 3:
                need(w, 2, "directory.symlinks")
                name = ""
                for k, kw, kv, kb in fields(b):
                    if k == 1:
                        name = string(kw, kb, "symlink.name")
                        if name == "":
                            raise Bad("symlink.name must not be empty")
                    elif k == 2:
                        string(kw, kb, "symlink.target")
                if name == "":
                    raise Bad("symlink nodes need a name")
    return files


def verdict(kind, hex_value):
    data = bytes.fromhex(hex_value)
    try:
        if kind == "tree":
            return {"valid": True, "references": [list(d) for d in tree(data)], "trees": []}
        references, trees = result(data)
        return {"valid": True, "references": [list(d) for d in references], "trees": [list(d) for d in trees]}
    except Bad as error:
        return {"valid": False, "reason": str(error)}


def enc_varint(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
        else:
            out.append(byte)
            return bytes(out)


def enc_field(number, wire, payload):
    tag = enc_varint((number << 3) | wire)
    if wire == 0:
        return tag + enc_varint(payload)
    if wire == 2:
        return tag + enc_varint(len(payload)) + payload
    return tag + payload


def enc_digest(content):
    h = hashlib.sha256(content).hexdigest()
    return enc_field(1, 2, h.encode()) + (enc_field(2, 0, len(content)) if content else b"")


def enc_file(path, content, executable=False, inline=False):
    body = enc_field(1, 2, path.encode()) + enc_field(2, 2, enc_digest(content))
    if executable:
        body += enc_field(4, 0, 1)
    if inline:
        body += enc_field(5, 2, content)
    return enc_field(2, 2, body)


def enc_file_node(name, content):
    return enc_field(1, 2, enc_field(1, 2, name.encode()) + enc_field(2, 2, enc_digest(content)))


def enc_tree(files):
    root = b"".join(enc_file_node(name, content) for name, content in files)
    return enc_field(1, 2, root)


def vectors():
    a, b, c = b"alpha\n", b"bravo bytes", b"charlie"
    out_tree = enc_tree([("x.txt", a), ("y/z.txt", b)])
    base = enc_file("out/a.txt", a) + enc_file("out/b.bin", b, executable=True)
    streams = enc_field(6, 2, enc_digest(c)) + enc_field(8, 2, enc_digest(a)) + enc_field(4, 0, 0)
    metadata = enc_field(9, 2, enc_field(1, 2, b"worker-1") + enc_field(2, 2, b"\x08\x01"))
    directory = enc_field(3, 2, enc_field(1, 2, b"gen") + enc_field(3, 2, enc_digest(out_tree)))
    bad_hash = enc_field(1, 2, b"ABC") + enc_field(2, 0, 3)
    short_hash = enc_field(1, 2, b"0" * 63) + enc_field(2, 0, 3)
    empty_ok = enc_field(2, 2, enc_field(1, 2, b"empty") + enc_field(2, 2, enc_digest(b"")))
    rows = []

    def add(name, kind, data, valid, references=(), trees=(), blobs=None):
        row = {"name": name, "kind": kind, "hex": data.hex(), "valid": valid}
        if valid:
            row["references"] = [[hashlib.sha256(x).hexdigest(), len(x)] for x in references]
            row["trees"] = [[hashlib.sha256(x).hexdigest(), len(x)] for x in trees]
            row["blobs"] = [x.hex() for x in (blobs if blobs is not None else [*references, *trees])]
        rows.append(row)

    add("empty message is a valid result", "result", b"", True)
    add("files, streams, exit code and metadata", "result", base + streams + metadata, True, [a, b, c, a])
    add("output directory points at a tree", "result", base + directory, True, [a, b], [out_tree], [a, b, out_tree])
    add("empty output file needs no blob", "result", empty_ok, True)
    add("inline contents replace the blob", "result", enc_file("small.txt", a, inline=True), True)
    add("unknown fields are skipped", "result", base + enc_field(99, 2, b"future") + enc_field(100, 0, 7), True, [a, b])
    add("reserved field one is skipped", "result", enc_field(1, 0, 1) + base, True, [a, b])
    add("output symlinks are accepted", "result", enc_field(10, 2, enc_field(1, 2, b"l") + enc_field(2, 2, b"t")) + enc_field(12, 2, enc_field(1, 2, b"m") + enc_field(2, 2, b"u")), True)
    add("negative exit code is a long varint", "result", enc_field(4, 0, (1 << 64) - 1) + base, True, [a, b])
    add("a tree with nested directories", "tree", out_tree + enc_field(2, 2, enc_file_node("w.txt", c)), True, [a, b, c], [], [a, b, c])
    add("an empty tree", "tree", b"", True)
    add("truncated length-delimited field", "result", base[:-3], False)
    add("truncated varint", "result", b"\x20\x80", False)
    add("group wire type", "result", enc_varint((2 << 3) | 3), False)
    add("field number zero", "result", b"\x00\x01", False)
    add("output file without a path", "result", enc_field(2, 2, enc_field(2, 2, enc_digest(a))), False)
    add("output file without a digest", "result", enc_field(2, 2, enc_field(1, 2, b"p")), False)
    add("digest with an uppercase hash", "result", enc_field(2, 2, enc_field(1, 2, b"p") + enc_field(2, 2, bad_hash)), False)
    add("digest with a short hash", "result", enc_field(2, 2, enc_field(1, 2, b"p") + enc_field(2, 2, short_hash)), False)
    add("size zero with a non-empty hash", "result", enc_field(2, 2, enc_field(1, 2, b"p") + enc_field(2, 2, enc_field(1, 2, hashlib.sha256(a).hexdigest().encode()))), False)
    add("inline contents that do not match", "result", enc_field(2, 2, enc_field(1, 2, b"p") + enc_field(2, 2, enc_digest(a)) + enc_field(5, 2, b"other")), False)
    add("exit code with a length-delimited wire type", "result", enc_field(4, 2, b"x"), False)
    add("output file as a varint", "result", enc_field(2, 0, 5), False)
    add("path that is not UTF-8", "result", enc_field(2, 2, enc_field(1, 2, b"\xff\xfe") + enc_field(2, 2, enc_digest(a))), False)
    add("output directory without a tree digest", "result", enc_field(3, 2, enc_field(1, 2, b"d")), False)
    add("size beyond 2^53", "result", enc_field(2, 2, enc_field(1, 2, b"p") + enc_field(2, 2, enc_field(1, 2, hashlib.sha256(a).hexdigest().encode()) + enc_field(2, 0, 1 << 60))), False)
    add("tree file node without a name", "tree", enc_field(1, 2, enc_field(1, 2, enc_field(2, 2, enc_digest(a)))), False)
    add("varint wider than ten bytes", "result", b"\x20" + b"\xff" * 10 + b"\x01", False)
    return rows


def plan(request):
    rows, bounds, dangling = request["rows"], request["bounds"], set(request.get("dangling", []))
    need_bytes, reserved = bounds.get("need", 0), bounds.get("reserved", 0)
    drops, kept, total = [], [], 0
    for row in rows:
        if row["key"] in dangling:
            drops.append({"key": row["key"], "size": row["size"], "reason": "dangling"})
        elif bounds["maximumAgeMilliseconds"] > 0 and bounds["now"] - row["accessed"] > bounds["maximumAgeMilliseconds"]:
            drops.append({"key": row["key"], "size": row["size"], "reason": "age"})
        else:
            kept.append(row)
            total += row["size"]
    if total + reserved > bounds["maximumBytes"] - need_bytes:
        target = bounds["maximumBytes"] * 9 // 10 - need_bytes
        for row in sorted(kept, key=lambda r: (r["accessed"], r["key"])):
            if total + reserved <= target:
                break
            drops.append({"key": row["key"], "size": row["size"], "reason": "budget"})
            total -= row["size"]
    return drops


def main():
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "vectors":
        print(json.dumps(vectors()))
    elif command == "decode":
        print(json.dumps([verdict(row["kind"], row["hex"]) for row in json.loads(sys.stdin.read())]))
    elif command == "plan":
        request = json.loads(sys.stdin.read())
        print(json.dumps([plan(row) for row in request] if isinstance(request, list) else plan(request)))
    else:
        sys.exit("usage: oracle.py vectors|decode|plan")


main()
