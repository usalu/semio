"""Userspace NFSv3 probe on a fresh TCP connection: MOUNT3 MNT, then NFS3 GETATTR and FSINFO."""
import socket, struct, sys, time

HOST, EXPORT = sys.argv[1], sys.argv[2]


def pad(b):
    return b + b"\0" * ((4 - len(b) % 4) % 4)


def auth_sys():
    body = struct.pack(">I", 0) + struct.pack(">I", 5) + pad(b"probe") + struct.pack(">III", 0, 0, 0)
    return struct.pack(">II", 1, len(body)) + body + struct.pack(">II", 0, 0)


def connect(port):
    for local in range(1000, 600, -1):
        s = socket.socket()
        s.settimeout(10)
        try:
            s.bind(("", local))
        except OSError:
            s.close()
            continue
        s.connect((HOST, port))
        return s
    raise SystemExit("no reserved port")


def call(s, prog, vers, proc, args, xid):
    msg = struct.pack(">IIIIII", xid, 0, 2, prog, vers, proc) + auth_sys() + args
    s.sendall(struct.pack(">I", 0x80000000 | len(msg)) + msg)
    data = b""
    while True:
        head = recvn(s, 4)
        n = struct.unpack(">I", head)[0]
        data += recvn(s, n & 0x7FFFFFFF)
        if n & 0x80000000:
            break
    rxid, mtype, stat = struct.unpack(">III", data[:12])
    assert rxid == xid and mtype == 1 and stat == 0, data[:24]
    vlen = struct.unpack(">I", data[16:20])[0]
    off = 20 + vlen
    accept = struct.unpack(">I", data[off:off + 4])[0]
    assert accept == 0, f"accept_stat {accept}"
    return data[off + 4:]


def recvn(s, n):
    out = b""
    while len(out) < n:
        chunk = s.recv(n - len(out))
        if not chunk:
            raise SystemExit("connection closed")
        out += chunk
    return out


def portmap(prog, vers):
    s = connect(111)
    r = call(s, 100000, 2, 3, struct.pack(">IIII", prog, vers, 6, 0), 1)
    s.close()
    return struct.unpack(">I", r[:4])[0]


t = time.time()
mport = portmap(100005, 3)
s = connect(mport)
r = call(s, 100005, 3, 1, struct.pack(">I", len(EXPORT)) + pad(EXPORT.encode()), 2)
s.close()
status = struct.unpack(">I", r[:4])[0]
print(f"MNT status {status} via mountd tcp {mport} in {time.time() - t:.2f}s")
if status:
    raise SystemExit(1)
flen = struct.unpack(">I", r[4:8])[0]
fh = r[8:8 + flen]
n = connect(2049)
for name, proc in (("GETATTR", 1), ("FSINFO", 19)):
    t = time.time()
    try:
        r = call(n, 100003, 3, proc, struct.pack(">I", flen) + pad(fh), 10 + proc)
        print(f"{name} status {struct.unpack('>I', r[:4])[0]} in {time.time() - t:.2f}s")
    except socket.timeout:
        print(f"{name} TIMEOUT after {time.time() - t:.2f}s")
