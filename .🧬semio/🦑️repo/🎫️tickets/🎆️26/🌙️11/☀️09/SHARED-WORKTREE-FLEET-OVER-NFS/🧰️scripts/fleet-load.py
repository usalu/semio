#!/usr/bin/env python3
"""Agent-like mixed load for the fleet rehearsal (module, driven by fleet-run.py). One thread per machine: bursts of 1-50 file operations (edit of real files, own and shared work areas, hot files, create, delete, file and directory rename), a few 10-50 MB files, and one latency probe file every ~8 s. Counts only operations that succeeded."""
import hashlib, json, os, random, threading, time
import fleet

HOT = 20
BIG_BUDGET = 8

class Load:
    def __init__(self, rate, pool, probes_path, big_budget=BIG_BUDGET):
        self.rate, self.pool, self.probes_path = rate, pool, probes_path
        self.stop = threading.Event(); self.threads = {}; self.counts = {}; self.lock = threading.Lock()
        self.big_left = big_budget; self.n = 0; self.errors = 0
        self.probe_fd = os.open(probes_path, os.O_WRONLY | os.O_APPEND | os.O_CREAT, 0o644)

    def count(self, o, kind):
        self.counts.setdefault(o, {}); c = self.counts[o]; c[kind] = c.get(kind, 0) + 1

    def total(self): return sum(sum(c.values()) for c in list(self.counts.values()))

    def add(self, o, population):
        t = threading.Thread(target=self.agent, args=(o, population), daemon=True); self.threads[o] = t; t.start()

    def join(self): [t.join() for t in list(self.threads.values())]

    def probe(self, o, seq, big=None):
        root = fleet.tree(o); rel = f"work/probe/{o}/p{seq % 8}"
        os.makedirs(os.path.dirname(f"{root}/{rel}"), exist_ok=True)
        data = f"{o} {seq} {os.urandom(6).hex()}\n".encode()
        with open(f"{root}/{rel}", "wb") as f: f.write(data)
        os.write(self.probe_fd, (json.dumps({"w": o, "p": rel, "n": len(data), "sha": hashlib.sha256(data).hexdigest(), "t": time.time()}) + "\n").encode())

    def agent(self, o, population):
        rnd = random.Random(o); root = fleet.tree(o); mine = f"work/{o}"
        files = {}; seq = 0; made = 0; bigs = []; next_probe = time.time() + rnd.uniform(1, 8); per = self.rate / max(1, population)
        def text(n): return (os.urandom(n // 2 + 1).hex()[:n - 1] + "\n").encode()
        def write(rel, data, mode="wb"):
            path = f"{root}/{rel}"; os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, mode) as f: f.write(data)
        def size(): return rnd.choice((200, 600, 1500, 4000, 4000, 12000, 40000))
        def op():
            nonlocal made, seq
            r = rnd.random(); made += 1
            if r < 0.20:
                rel = rnd.choice(self.pool); write(rel, f"\n// {o} {made} {os.urandom(4).hex()}\n".encode(), "ab"); return "edit-real"
            if r < 0.34 and files:
                d = rnd.choice(list(files)); 
                if not files[d]: return None
                n = rnd.choice(files[d]); write(f"{mine}/{d}/{n}", text(size())); return "edit-own"
            if r < 0.48:
                d = f"{root}/work/shared/s{rnd.randrange(12)}"
                try: names = os.listdir(d)
                except OSError: return None
                if not names: return None
                write(f"work/shared/{os.path.basename(d)}/{rnd.choice(names)}", text(size())); return "edit-shared"
            if r < 0.63:
                d = f"d{rnd.randrange(6)}"; n = f"f{made}.txt"; write(f"{mine}/{d}/{n}", text(size())); files.setdefault(d, []).append(n); return "create-own"
            if r < 0.73:
                write(f"work/shared/s{rnd.randrange(12)}/{o}-{made}.txt", text(size())); return "create-shared"
            if r < 0.79 and files:
                d = rnd.choice(list(files))
                if not files[d]: return None
                n = files[d].pop(rnd.randrange(len(files[d]))); os.unlink(f"{root}/{mine}/{d}/{n}"); return "delete-own"
            if r < 0.83:
                d = f"{root}/work/shared/s{rnd.randrange(12)}"
                try: names = os.listdir(d)
                except OSError: return None
                if not names: return None
                os.unlink(f"{d}/{rnd.choice(names)}"); return "delete-shared"
            if r < 0.87 and files:
                d = rnd.choice(list(files))
                if not files[d]: return None
                i = rnd.randrange(len(files[d])); n = files[d][i]; m = f"r{made}.txt"; os.rename(f"{root}/{mine}/{d}/{n}", f"{root}/{mine}/{d}/{m}"); files[d][i] = m; return "rename-file"
            if r < 0.88 and files:
                d = rnd.choice(list(files)); e = f"d{rnd.randrange(6)}x{made}"
                if os.path.exists(f"{root}/{mine}/{d}"): os.rename(f"{root}/{mine}/{d}", f"{root}/{mine}/{e}"); files[e] = files.pop(d); return "rename-dir"
                return None
            if r < 0.97:
                write(f"work/hot/h{rnd.randrange(HOT)}", f"{o} {made} {time.time()}\n".encode() + text(rnd.choice((100, 2000)))); return "hot"
            return None
        def big():
            with self.lock:
                if self.big_left <= 0: return None
                self.big_left -= 1
            mb = rnd.randint(10, 50); rel = f"{mine}/big/b{made}.bin"; path = f"{root}/{rel}"; os.makedirs(os.path.dirname(path), exist_ok=True)
            h = hashlib.sha256()
            with open(path, "wb") as f:
                for i in range(mb):
                    c = os.urandom(1 << 20); h.update(c); f.write(c)
            os.write(self.probe_fd, (json.dumps({"w": o, "p": rel, "n": mb << 20, "sha": h.hexdigest(), "t": time.time(), "big": True}) + "\n").encode()); return "big"
        while not self.stop.is_set():
            k = rnd.choices((rnd.randint(1, 3), rnd.randint(4, 15), rnd.randint(16, 50)), (60, 30, 10))[0]
            for _ in range(k):
                if self.stop.is_set(): break
                try:
                    kind = op() if rnd.random() > 0.0015 else big()
                    if kind: self.count(o, kind)
                except OSError:
                    self.errors += 1
                time.sleep(rnd.uniform(0.002, 0.03))
            if time.time() > next_probe:
                seq += 1; next_probe = time.time() + rnd.uniform(6, 10)
                try: self.probe(o, seq); self.count(o, "probe")
                except OSError: self.errors += 1
            self.stop.wait(rnd.expovariate(per / 7.4))
