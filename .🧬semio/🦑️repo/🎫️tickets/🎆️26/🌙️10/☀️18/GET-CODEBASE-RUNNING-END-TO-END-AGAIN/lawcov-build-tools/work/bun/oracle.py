"""🥟️ Independent Python oracle of the pinned Bun release rules (standard library only): it builds ZIP archives with `zipfile`, applies byte-level mutations at offsets it finds itself, reads members back with `zipfile`, and derives release archive names from platform, architecture and libc. Requests arrive as JSON on stdin."""
import base64, hashlib, io, json, struct, sys, zipfile

FIELDS = {"signature": (0, "<I"), "flags": (8, "<H"), "method": (10, "<H"), "compressedSize": (20, "<I"), "expandedSize": (24, "<I")}


def content(spec):
    return (spec["repeat"] * spec["times"]).encode() if "repeat" in spec else spec["text"].encode()


def build(members):
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w") as archive:
        for member in members:
            info = zipfile.ZipInfo(member["name"], date_time=(2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED if member["method"] == "deflated" else zipfile.ZIP_STORED
            archive.writestr(info, content(member["content"]))
    return bytearray(buffer.getvalue())


def directory(data):
    end = data.rfind(b"PK\x05\x06")
    count, _, offset = struct.unpack_from("<HII", data, end + 10)
    rows, cursor = {}, offset
    for _ in range(count):
        name_length, extra, comment = struct.unpack_from("<HHH", data, cursor + 28)
        name = bytes(data[cursor + 46:cursor + 46 + name_length]).decode()
        rows[name] = {"central": cursor, "local": struct.unpack_from("<I", data, cursor + 42)[0]}
        cursor += 46 + name_length + extra + comment
    return rows


def mutate(data, mutations):
    for row in mutations:
        if row["op"] in ("central", "local"):
            where = directory(data)[row["entry"]]
            base = where["central"] if row["op"] == "central" else where["local"]
            offset, fmt = FIELDS[row["field"]]
            struct.pack_into(fmt, data, base + offset, row["value"])
        elif row["op"] == "truncate":
            del data[len(data) - row["bytes"]:]
        else:
            data.extend(b"\0" * row["bytes"])
    return data


def release_name(platform, arch, musl):
    system = {"win32": "windows", "darwin": "darwin", "linux": "linux"}.get(platform)
    cpu = {"x64": "x64", "arm64": "aarch64"}.get(arch)
    if system is None or cpu is None or (musl and platform != "linux"):
        return None
    return "bun-" + system + "-" + cpu + ("-musl" if musl else "") + ("-baseline" if cpu == "x64" else "") + ".zip"


def main():
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "archive":
        rows = []
        for row in request["rows"]:
            data = mutate(build(row["members"]), row["mutations"])
            reference = None
            try:
                reference = hashlib.sha256(zipfile.ZipFile(io.BytesIO(bytes(data))).read(row["select"])).hexdigest()
            except Exception:
                reference = None
            rows.append({"archive": base64.b64encode(bytes(data)).decode(), "reference": reference})
        print(json.dumps(rows))
    elif op == "names":
        print(json.dumps([release_name(r["platform"], r["arch"], r["musl"]) for r in request["rows"]]))
    elif op == "sha256":
        print(json.dumps([hashlib.sha256(base64.b64decode(text)).hexdigest() for text in request["texts"]]))
    else:
        raise SystemExit("unknown op " + op)


main()
