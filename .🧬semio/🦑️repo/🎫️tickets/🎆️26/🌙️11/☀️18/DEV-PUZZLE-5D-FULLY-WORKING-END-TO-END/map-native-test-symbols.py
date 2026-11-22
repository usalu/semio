import collections
import mmap
import re
import struct
import subprocess
import sys


def executable_segment(data):
    header = struct.unpack_from("<16sHHIQQQIHHHHHH", data)
    program_offset, header_size, header_count = header[5], header[9], header[10]
    for index in range(header_count):
        kind, flags, offset, address, _, size, *_ = struct.unpack_from(
            "<IIQQQQQQ", data, program_offset + index * header_size
        )
        if kind == 1 and flags & 1:
            return offset, address, size
    raise ValueError("executable ELF segment is absent")


def symbols(path, pattern):
    process = subprocess.Popen(["nm", "-S", "-C", path], stdout=subprocess.PIPE, text=True)
    for line in process.stdout:
        columns = line.rstrip().split(maxsplit=3)
        if len(columns) == 4 and columns[2] in ("T", "t") and pattern.search(columns[3]):
            yield int(columns[0], 16), int(columns[1], 16), columns[3]
    if process.wait():
        raise RuntimeError("native reference symbols could not be read")


def stable_bytes(path, address, size):
    body = subprocess.check_output(
        ["objdump", "-d", "--insn-width=16", f"--start-address={address}",
         f"--stop-address={address + size}", path], text=True
    )
    contiguous = bytearray()
    opening = 0
    for line in body.splitlines():
        instruction = re.match(r"\s*([0-9a-f]+):\s+((?:[0-9a-f]{2} )+)\s*(.*)", line)
        if not instruction:
            continue
        offset = int(instruction[1], 16) - address
        shown = instruction[3]
        if "%rip" in shown or re.match(r"\s*(?:call|j[a-z]+)\s+\b[0-9a-f]", shown):
            if len(contiguous) >= 16:
                yield opening, bytes(contiguous)
            contiguous.clear()
            continue
        if not contiguous:
            opening = offset
        contiguous.extend(bytes.fromhex(instruction[2]))
    if len(contiguous) >= 16:
        yield opening, bytes(contiguous)


def matching_addresses(data, segment, anchor, relative):
    offset, address, size = segment
    end = offset + size
    position = data.find(anchor, offset, end)
    while position >= 0:
        yield address + position - offset - relative
        position = data.find(anchor, position + 1, end)


def match_code(data, segment, path, address, size):
    matches = collections.Counter()
    anchors = sorted(stable_bytes(path, address, size), key=lambda item: len(item[1]), reverse=True)
    for relative, anchor in anchors[:8]:
        matches.update(matching_addresses(data, segment, anchor[:40], relative))
    return matches.most_common(3)


reference, target, expression, *extra = sys.argv[1:]
if expression == "--frames":
    with open(extra[0]) as trace:
        trace = trace.read()
    base = int(re.search(r"executable_base (0x[0-9a-f]+)", trace)[1], 16)
    with open(target, "rb") as binary, mmap.mmap(binary.fileno(), 0, access=mmap.ACCESS_READ) as data:
        _, start, size = executable_segment(data)
    frames = [(number, int(address, 16) - base) for number, address in re.findall(r"^#(\d+)\s+(0x[0-9a-f]+).* in \?\?", trace, re.M)]
    with open(reference, "rb") as binary, mmap.mmap(binary.fileno(), 0, access=mmap.ACCESS_READ) as data:
        segment = executable_segment(data)
        for number, address in frames[:48]:
            if not start <= address < start + size:
                continue
            matches = match_code(data, segment, target, address, 140)
            if matches and (len(matches) == 1 or matches[0][1] > matches[1][1]):
                candidate = hex(matches[0][0])
                symbol = subprocess.check_output(["addr2line", "-f", "-C", "-e", reference, candidate], text=True).splitlines()[0]
                print(number, hex(address), candidate, matches[0][1], symbol, flush=True)
else:
    with open(target, "rb") as binary, mmap.mmap(binary.fileno(), 0, access=mmap.ACCESS_READ) as data:
        segment = executable_segment(data)
        for address, size, symbol in symbols(reference, re.compile(expression)):
            best = [(hex(candidate), count) for candidate, count in match_code(data, segment, reference, address, size)]
            print(hex(address), best, symbol, flush=True)
