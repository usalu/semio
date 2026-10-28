#!/usr/bin/env python3
"""🧮️ Independent oracle of the native Windows process listing (python3 -I 🐍️.py < request.json prints one JSON object). It shares no code with the TypeScript module: the byte layouts come from `ctypes` structures declared after the Windows headers (PROCESSENTRY32W of tlhelp32.h, UNICODE_STRING of winternl.h), the records and command-line headers are encoded by those structures, and the FILETIME spelling comes from `datetime` arithmetic. Request keys (all optional): `layout`, `records` [{pid, ppid, name}], `headers` [text], `ticks` [iso text], `iso` [ticks]."""
import ctypes, datetime, json, re, sys


class ProcessEntry32W(ctypes.Structure):
    _fields_ = [("dwSize", ctypes.c_uint32), ("cntUsage", ctypes.c_uint32), ("th32ProcessID", ctypes.c_uint32), ("th32DefaultHeapID", ctypes.c_size_t),
                ("th32ModuleID", ctypes.c_uint32), ("cntThreads", ctypes.c_uint32), ("th32ParentProcessID", ctypes.c_uint32), ("pcPriClassBase", ctypes.c_int32),
                ("dwFlags", ctypes.c_uint32), ("szExeFile", ctypes.c_uint16 * 260)]


class UnicodeString(ctypes.Structure):
    _fields_ = [("Length", ctypes.c_uint16), ("MaximumLength", ctypes.c_uint16), ("Buffer", ctypes.c_void_p)]


ORIGIN = datetime.datetime(1601, 1, 1)


def units(text):
    raw = text.encode("utf-16-le")
    return [int.from_bytes(raw[i:i + 2], "little") for i in range(0, len(raw), 2)]


def record(row):
    entry = ProcessEntry32W()
    entry.dwSize = ctypes.sizeof(ProcessEntry32W)
    entry.th32ProcessID = row["pid"]
    entry.th32ParentProcessID = row["ppid"]
    for index, value in enumerate(units(row["name"])[:260]):
        entry.szExeFile[index] = value
    return bytes(entry).hex()


def header(text):
    data = text.encode("utf-16-le")
    string = UnicodeString()
    string.Length = len(data)
    string.MaximumLength = len(data) + 2
    string.Buffer = ctypes.sizeof(UnicodeString)
    return (bytes(string) + data + b"\0\0").hex()


def ticks(text):
    match = re.fullmatch(r"(\d{4})-(\d\d)-(\d\d)T(\d\d):(\d\d):(\d\d)\.(\d{7})Z", text)
    moment = datetime.datetime(*[int(match.group(i)) for i in range(1, 7)])
    delta = moment - ORIGIN
    return str((delta.days * 86400 + delta.seconds) * 10_000_000 + int(match.group(7)))


def iso(value):
    moment = ORIGIN + datetime.timedelta(microseconds=int(value) // 10)
    return "%04d-%02d-%02dT%02d:%02d:%02d.%06d0Z" % (moment.year, moment.month, moment.day, moment.hour, moment.minute, moment.second, moment.microsecond)


request = json.load(sys.stdin)
answer = {}
if request.get("layout"):
    answer["layout"] = {
        "pointerBytes": ctypes.sizeof(ctypes.c_void_p),
        "entry": {"bytes": ctypes.sizeof(ProcessEntry32W), "processId": ProcessEntry32W.th32ProcessID.offset, "parentId": ProcessEntry32W.th32ParentProcessID.offset,
                  "name": ProcessEntry32W.szExeFile.offset, "nameUnits": len(ProcessEntry32W().szExeFile)},
        "commandLine": {"headerBytes": ctypes.sizeof(UnicodeString), "textOffset": UnicodeString.Buffer.offset + ctypes.sizeof(ctypes.c_void_p)},
    }
if "records" in request:
    answer["records"] = [record(row) for row in request["records"]]
if "headers" in request:
    answer["headers"] = [header(text) for text in request["headers"]]
if "ticks" in request:
    answer["ticks"] = [ticks(text) for text in request["ticks"]]
if "iso" in request:
    answer["iso"] = [iso(value) for value in request["iso"]]
print(json.dumps(answer))
