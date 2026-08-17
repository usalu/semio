from collections import Counter
from pathlib import Path
import hashlib
import importlib.util
import re
import struct


TICKET = Path(__file__).parent
FIXTURE = Path("/Users/ueli/Documents/semio/temp/architectural_example.dwg")
SPEC = importlib.util.spec_from_file_location("d2", TICKET / "🧪️dwg-d2-policy-probe.py")
D2 = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(D2)


def decoded_page(address, compressed_size, semantic_size):
    source = FIXTURE.read_bytes()
    return D2.decompress(source[address + 32:address + 32 + compressed_size])[0][:semantic_size]


def crc16(data, seed=0xC0C1):
    crc = seed
    for byte in data:
        crc ^= byte
        for _ in range(8):
            crc = (crc >> 1) ^ 0xA001 if crc & 1 else crc >> 1
    return crc


fixture = FIXTURE.read_bytes()

header = decoded_page(0x23B80, 946, 896)
end_sentinel = bytes.fromhex("3084e0dc0221c756a0839747b192cca0")
end_offset = header.index(end_sentinel)
print("HEADER", len(header), int.from_bytes(header[16:20], "little"), int.from_bytes(header[20:24], "little"), end_offset, header[end_offset - 2:end_offset].hex(), f"{crc16(header[16:end_offset - 2]):04x}")

aux = decoded_page(0x23A80, 205, 123)
aux_format = "<3BHHIiHHI4H6H5I4IQHH8I"
aux_values = struct.unpack(aux_format, aux)
print("AUX", len(aux), aux_values)

revision = decoded_page(0x161A0, 135, 16)
print("REVISION", len(revision), struct.unpack("<4I", revision))

free_space = decoded_page(0x21EE0, 169, 89)
free_values = struct.unpack("<QQIIB8Q", free_space)
print("FREE_SPACE", len(free_space), free_values)

preview = fixture[0x1C0:0x1C0 + 86191]
overall_size, count = struct.unpack_from("<IB", preview, 16)
position = 21
records = []
for _ in range(count):
    code = preview[position]
    start, size = struct.unpack_from("<II", preview, position + 1)
    position += 9
    records.append((code, start, size))
header_record = fixture[records[0][1]:records[0][1] + records[0][2]]
dib = fixture[records[1][1]:records[1][1] + records[1][2]]
dib_fields = struct.unpack_from("<IiiHHIIiiII", dib)
width, height = dib_fields[1:3]
palette = dib[40:1064]
pixels = dib[1064:]
stride = (width + 3) // 4 * 4
logical_pixels = b"".join(pixels[row * stride:row * stride + width] for row in range(height))
padding = Counter(pixels[row * stride + width:(row + 1) * stride] for row in range(height))
print("PREVIEW", len(preview), overall_size, records, set(header_record), dib_fields, stride, padding, hashlib.sha256(logical_pixels).hexdigest(), sum(palette[index + 3] != 0 for index in range(0, len(palette), 4)))

history = fixture[0x15900:0x15900 + 1390]
position = 32
class_version = struct.unpack_from("<I", history, position)[0]
position += 4


def text16(data, offset):
    count = struct.unpack_from("<H", data, offset)[0]
    start = offset + 2
    return data[start:start + count * 2 - 2].decode("utf-16le"), start + count * 2


name, position = text16(history, position)
count = struct.unpack_from("<I", history, position)[0]
position += 4
entries = []
for _ in range(count):
    digest = history[position:position + 16].hex()
    position += 16
    value, position = text16(history, position)
    entries.append((digest, value))
properties = re.findall(r'<prop id="(\d+)"><(string|datetime)>(.*?)</\2></prop>', entries[2][1])
product = dict(re.findall(r'(name|build_version|registry_version|install_id_string|registry_localeID)\s*=\\"(.*?)\\"', entries[3][1]))
print("APP_HISTORY", len(history), history[:16].hex(), history[16:32].hex(), class_version, name, count, position)
for index, (digest, value) in enumerate(entries):
    print("APP_HISTORY_ENTRY", index, digest, len(value), value if index == 0 else value[:96])
print("APP_HISTORY_PROPERTIES", properties)
print("APP_HISTORY_PRODUCT", product)
