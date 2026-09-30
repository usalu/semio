"""Independent oracle for the exact ClientFrame::Commands vector (Rust `encode_client_frame`, TS `encodeClientCommandsFrameExact`)."""

def varint(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
        else:
            out.append(byte)
            return bytes(out)

def text(value):
    raw = value.encode()
    return varint(len(raw)) + raw

def envelope():
    out = text("transition-1") + text("document-1") + text("actor-1")
    out += varint(1) + text("op-1")
    out += varint(0)
    out += varint(1) + text("title")
    out += text("semio.history-transition.v1") + varint(3) + bytes([1, 2, 3])
    out += text("semio.history-transition.v1") + varint(0)
    out += varint(0xFEDCBA9876543210) + varint((1 << 53) + 1) + varint(1 << 60)
    out += varint(0)
    return out

frame = bytes([0, 1]) + varint(7) + varint(1) + envelope()
print(frame.hex())
