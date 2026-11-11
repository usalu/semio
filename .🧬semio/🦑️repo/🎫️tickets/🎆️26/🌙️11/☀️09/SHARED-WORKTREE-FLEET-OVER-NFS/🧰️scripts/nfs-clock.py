#!/usr/bin/env python3
"""usage: nfs-clock.py serve PORT | measure HOST PORT [ROUNDS] -- NTP-style clock offset between two machines over TCP; measure prints offset_ms (server minus client, median of the 20 smallest-RTT rounds) and rtt_ms."""
import json, socket, statistics, sys, time

if sys.argv[1] == "serve":
    s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1); s.bind(("0.0.0.0", int(sys.argv[2]))); s.listen(1)
    while True:
        c, _ = s.accept(); c.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        try:
            while c.recv(1):
                c.sendall(b"%.9f\n" % time.time())
        except OSError:
            pass
        c.close()
else:
    host, port, rounds = sys.argv[2], int(sys.argv[3]), int(sys.argv[4]) if len(sys.argv) > 4 else 400
    c = socket.create_connection((host, port)); c.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    rows = []
    for _ in range(rounds):
        t0 = time.time(); c.sendall(b"x"); ts = float(c.recv(64)); t1 = time.time()
        rows.append((t1 - t0, ts - (t0 + t1) / 2))
    rows.sort()
    best = rows[:20]
    print(json.dumps({"offset_ms": round(1000 * statistics.median(x[1] for x in best), 3), "rtt_ms": round(1000 * best[0][0], 3), "rtt_median_ms": round(1000 * statistics.median(x[0] for x in rows), 3)}))
