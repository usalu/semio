#!/usr/bin/env python3
"""Overwrite presence wire fixture bins with newly encoded bytes from cargo test panic left-side."""
from pathlib import Path

# left bytes from wire_fixtures panic for client-presence
client_left = bytes([
    1, 4, 197, 2, 7, 97, 99, 116, 111, 114, 45, 49, 237, 7, 128, 208, 149, 255, 188, 49, 3, 65, 100, 97, 6, 117, 115, 101, 114, 45, 57, 5, 111, 119, 110, 101, 114, 5, 115, 112, 97, 99, 101, 3, 7, 111, 117, 116, 108, 105, 110, 101, 4, 116, 97, 115, 107, 2, 2, 116, 49, 2, 116, 50, 0, 5, 98, 111, 97, 114, 100, 4, 99, 97, 114, 100, 0, 1, 2, 99, 49, 6, 99, 97, 110, 118, 97, 115, 4, 110, 111, 100, 101, 1, 2, 110, 57, 2, 2, 110, 57, 3, 110, 49, 48, 5, 23, 115, 46, 115, 112, 97, 99, 101, 46, 104, 111, 109, 101, 64, 49, 47, 42, 35, 101, 100, 105, 116, 111, 114, 2, 2, 119, 49, 5, 119, 111, 114, 108, 100, 1, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 8, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 70, 64, 0, 0, 0, 0, 0, 0, 144, 64, 0, 0, 0, 0, 0, 0, 136, 64, 1, 0, 0, 0, 0, 0, 0, 224, 63, 0, 0, 0, 0, 0, 0, 224, 63, 0, 0, 0, 0, 0, 0, 224, 63, 0, 0, 2, 119, 50, 6, 99, 97, 110, 118, 97, 115, 0, 0, 0, 0, 0, 0, 0, 41, 64, 0, 0, 0, 0, 0, 0, 16, 192, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 137, 64, 0, 0, 0, 0, 0, 192, 130, 64, 0, 0, 1, 9, 114, 111, 119, 91, 50, 93, 35, 116, 49, 0, 0
])

fixtures = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-c3b/links/replication-fixtures").resolve()
wire = next(fixtures.glob("*wire*"))
# client-presence folder
client_dirs = [d for d in wire.iterdir() if d.is_dir() and "client-presence" in d.name]
server_dirs = [d for d in wire.iterdir() if d.is_dir() and "server-presence" in d.name]
print("client", client_dirs, "server", server_dirs)
client_bin = next(client_dirs[0].glob("*.bin"))
print("writing", client_bin, "old", client_bin.stat().st_size, "new", len(client_left))
client_bin.write_bytes(client_left)
print("client written")
