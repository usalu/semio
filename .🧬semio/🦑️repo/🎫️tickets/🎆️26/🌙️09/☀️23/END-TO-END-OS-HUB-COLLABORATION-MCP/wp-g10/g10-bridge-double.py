"""🎭️ G10 bridge live proof: an old-gateway stand-in that publishes a real rendezvous offer and then
behaves on the wire like one of two old gateways.

usage: python3 g10-bridge-double.py <bridge-dir> <silent|v2-refusal> <status-file>

- silent:     listens and never accepts — exactly what pid 88810 does once its 64 slots leaked
              (the kernel completes the TCP handshake, the gateway never reads a byte).
- v2-refusal: completes the websocket upgrade, reads the shell's Hello, answers the typed
              `Refused { reason: version, gatewayVersion: 2 }` frame (tag 12) and closes.

One JSON line per event goes to <status-file>. SIGTERM withdraws the offer and exits.
"""
import base64, hashlib, json, os, secrets, signal, socket, sys, threading, time

bridge_dir, mode, status_path = sys.argv[1], sys.argv[2], sys.argv[3]
status = open(status_path, "a", buffering=1)


def emit(**row):
    row["at"] = time.strftime("%H:%M:%S")
    status.write(json.dumps(row) + "\n")


listener = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listener.bind(("127.0.0.1", 0))
listener.listen(128)
port = listener.getsockname()[1]
offers = os.path.join(bridge_dir, "offers")
os.makedirs(offers, exist_ok=True)
offer_path = os.path.join(offers, f"{os.getpid()}.json")
partial = offer_path + ".partial"
with open(partial, "w") as handle:
    json.dump({"schemaVersion": 1, "url": f"ws://127.0.0.1:{port}/bridge", "admissionProof": secrets.token_hex(32), "principal": f"agent:g10-double-{mode}", "pid": os.getpid(), "publishedAtMs": int(time.time() * 1000)}, handle)
os.chmod(partial, 0o600)
os.replace(partial, offer_path)
emit(event="offered", mode=mode, pid=os.getpid(), url=f"ws://127.0.0.1:{port}/bridge")


def withdraw(*_):
    try:
        os.remove(offer_path)
    except FileNotFoundError:
        pass
    emit(event="withdrawn")
    os._exit(0)


signal.signal(signal.SIGTERM, withdraw)


def read_exact(conn, count):
    data = b""
    while len(data) < count:
        chunk = conn.recv(count - len(data))
        if not chunk:
            raise ConnectionError("peer closed")
        data += chunk
    return data


def serve_refusal(conn, ordinal):
    try:
        conn.settimeout(10)
        request = b""
        while b"\r\n\r\n" not in request:
            chunk = conn.recv(4096)
            if not chunk:
                raise ConnectionError("peer closed before upgrade")
            request += chunk
        headers = {line.split(b":", 1)[0].strip().lower(): line.split(b":", 1)[1].strip() for line in request.split(b"\r\n")[1:] if b":" in line}
        accept = base64.b64encode(hashlib.sha1(headers[b"sec-websocket-key"] + b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11").digest())
        conn.sendall(b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: " + accept + b"\r\nSec-WebSocket-Protocol: semio.mcp.bridge.v1\r\n\r\n")
        head = read_exact(conn, 2)
        length = head[1] & 0x7F
        if length == 126:
            length = int.from_bytes(read_exact(conn, 2), "big")
        elif length == 127:
            length = int.from_bytes(read_exact(conn, 8), "big")
        mask = read_exact(conn, 4)
        hello = bytes(byte ^ mask[index % 4] for index, byte in enumerate(read_exact(conn, length)))
        refused = bytes([0x0C, 0x00]) + (2).to_bytes(2, "little")
        conn.sendall(bytes([0x82, len(refused)]) + refused + bytes([0x88, 0x00]))
        emit(event="refused", connection=ordinal, helloTag=hello[0] if hello else None, helloBridgeVersion=int.from_bytes(hello[1:3], "little") if len(hello) >= 3 else None, sent=refused.hex())
        time.sleep(0.5)
    except Exception as error:
        emit(event="connection-fault", connection=ordinal, error=str(error))
    finally:
        conn.close()


if mode == "silent":
    while True:
        time.sleep(3600)
ordinal = 0
while True:
    conn, _ = listener.accept()
    ordinal += 1
    emit(event="accepted", connection=ordinal)
    threading.Thread(target=serve_refusal, args=(conn, ordinal), daemon=True).start()
