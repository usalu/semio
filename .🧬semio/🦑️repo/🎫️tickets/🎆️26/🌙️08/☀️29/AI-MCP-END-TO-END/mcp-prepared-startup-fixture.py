from pathlib import Path
import json
import re
import sys


def unsigned(value):
    result = []
    while value >= 128:
        result.append((value & 127) | 128)
        value >>= 7
    return bytes(result + [value])


def signed(value):
    result = []
    while True:
        byte = value & 127
        value >>= 7
        if (value == 0 and byte < 64) or (value == -1 and byte >= 64):
            return bytes(result + [byte])
        result.append(byte | 128)


def vector(items):
    return unsigned(len(items)) + b"".join(items)


def section(tag, content):
    return bytes([tag]) + unsigned(len(content)) + content


def name(value):
    return unsigned(len(value)) + value.encode()


source = Path(sys.argv[1]).read_text()
exports = dict(re.findall(r'pub const (SEMIO_OWNED_\w+_EXPORT)\s*:\s*&str\s*=\s*"([^"]+)"', source))
names = re.findall(r"Self::(\w+)\s*=>\s*(SEMIO_OWNED_\w+_EXPORT),", source)
assert len(names) == 36
i32, i64 = b"\x7f", b"\x7e"
signatures = []
functions = []
bodies = []
function_exports = []


def function(params, result, body, export=None):
    signature = b"\x60" + unsigned(len(params)) + params + unsigned(len(result)) + result
    if signature not in signatures:
        signatures.append(signature)
    if export:
        function_exports.append(name(export) + b"\0" + unsigned(len(functions)))
    functions.append(unsigned(signatures.index(signature)))
    full = b"\0" + body + b"\x0b"
    bodies.append(unsigned(len(full)) + full)
    return len(functions) - 1


def store_marker(value):
    return b"\x41" + signed(64) + b"\x41" + signed(value) + b"\x3a\0\0"


for variant, constant in names:
    params, result, body = i32 * 2, i64, b"\0"
    if variant == "Allocate":
        params, result = i32, i32
    elif variant == "Deallocate":
        result = b""
    elif variant in ("StartJob", "StepJob", "ReadJobOutput", "AcknowledgeJobOutput", "CancelJob"):
        params = i64 * 20
    elif variant in ("PublishActorFailure", "ReadActorFailure", "AcknowledgeActorFailure"):
        params = i64 * 21
    elif variant in ("CloseJobs", "Poll", "AdmitActorContext", "CloseActorContext", "CloseActorOutput"):
        params = i64 * 19
    elif variant == "BorrowJobOutput":
        params = i64 * 5
    elif variant == "BorrowActorFailure":
        params = i64 * 6
    elif variant == "AcknowledgeJobOutputChunk":
        params = i64 * 22
    elif variant == "AcknowledgeActorFailureChunk":
        params = i64 * 23
    elif variant == "AdmitActorInput":
        params = i32 * 2 + i64 * 19
    elif variant in ("Checkpoint", "Describe", "ChannelVersion"):
        params = b""
        if variant == "ChannelVersion":
            body = b"\x42" + signed((2 << 32) | 1024)
    function(params, result, body, exports[constant])

start = function(b"", b"", store_marker(17))
function(b"", i32, store_marker(93) + b"\x41\x01", "semio_owned_prepare_actor_v1")
core = b"\0asm\x01\0\0\0" + b"".join([
    section(1, vector(signatures)),
    section(3, vector(functions)),
    section(5, b"\x01\0\x01"),
    section(7, vector([name("memory") + b"\x02\0"] + function_exports)),
    section(8, unsigned(start)),
    section(10, vector(bodies)),
    section(11, b"\x01\0\x41" + signed(1024) + b"\x0b\x02" + b"25"),
])
Path(sys.argv[2]).write_text(json.dumps({
    "coreHex": core.hex(),
    "channelVersion": 25,
    "channelInstructions": 2,
    "marker": {"offset": 64, "afterStart": 17, "prepared": 93, "mutated": 121},
}, indent=2) + "\n")
