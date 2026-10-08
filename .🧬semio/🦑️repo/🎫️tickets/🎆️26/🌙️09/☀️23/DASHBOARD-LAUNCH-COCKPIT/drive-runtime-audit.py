#!/usr/bin/env python3
"""Runtime audit driver for the installed native semio dashboard.

Forks the installed executable in a pseudo-terminal of a fixed size, feeds keys and SGR mouse bytes, interprets the
output with a small VT emulator and writes every screen as plain text (+ a style map showing highlighted cells).

usage: drive-runtime-audit.py <scenario> [args]      scenarios are the functions named scn_*
All captured output goes to OUT; the real workspace is only ever viewed (no Enter in the launcher, no Ctrl+B Q/k/c/r).
"""
import codecs, collections, fcntl, json, os, re, select, shutil, signal, struct, subprocess, sys, termios, time, unicodedata

REPO = "/Users/ueli/Documents/semio"
TICKET = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT"
OUT = TICKET + "/🗑️generated/runtime-audit"
FIX = OUT + "/fixture-root"
CTRL_B, ESC, ENTER, TAB, BTAB = b"\x02", b"\x1b", b"\r", b"\t", b"\x1b[Z"
UP, DOWN, RIGHT, LEFT = b"\x1b[A", b"\x1b[B", b"\x1b[C", b"\x1b[D"
HOME, END, PGUP, PGDN = b"\x1b[H", b"\x1b[F", b"\x1b[5~", b"\x1b[6~"


def binary():
    record = json.load(open(REPO + "/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"))
    return os.path.join(REPO, record["path"])


# ---------------------------------------------------------------------------------------------------------------- VT
def char_width(ch):
    if ch == "" or unicodedata.combining(ch) or unicodedata.category(ch) in ("Mn", "Me", "Cf"):
        return 0
    if unicodedata.east_asian_width(ch) in ("W", "F") or 0x1F300 <= ord(ch) <= 0x1FAFF:
        return 2
    return 1


class Vt:
    """Minimal VT100/xterm interpreter: cursor addressing, erase, scroll, SGR (recorded), private modes (recorded)."""

    def __init__(self, cols, rows):
        self.styles = [("", "", "")]
        self.style_index = {("", "", ""): 0}
        self.decoder = codecs.getincrementaldecoder("utf-8")("replace")
        self.modes = collections.OrderedDict()
        self.title = None
        self.bell = 0
        self.state = "g"
        self.buf = ""
        self.resize(cols, rows)

    def resize(self, cols, rows):
        self.cols, self.rows = cols, rows
        self.grid = [[(" ", 0)] * cols for _ in range(rows)]
        self.x = self.y = 0
        self.pending = False
        self.fg = self.bg = ""
        self.attrs = set()
        self.cursor_visible = True
        self.top, self.bottom = 0, rows - 1
        self.saved = (0, 0)

    def style(self):
        key = (self.fg, self.bg, ",".join(sorted(self.attrs)))
        if key not in self.style_index:
            self.style_index[key] = len(self.styles)
            self.styles.append(key)
        return self.style_index[key]

    def feed(self, data):
        for ch in self.decoder.decode(data):
            self.char(ch)

    def put(self, ch):
        width = char_width(ch)
        if width == 0:
            return
        if self.pending or self.x + width > self.cols:
            self.x, self.pending = 0, False
            self.linefeed()
        row = self.grid[self.y]
        style = self.style()
        row[self.x] = (ch, style)
        if width == 2 and self.x + 1 < self.cols:
            row[self.x + 1] = ("", style)
        self.x += width
        if self.x >= self.cols:
            self.x, self.pending = self.cols - 1, True

    def linefeed(self):
        if self.y == self.bottom:
            del self.grid[self.top]
            self.grid.insert(self.bottom, [(" ", 0)] * self.cols)
        elif self.y < self.rows - 1:
            self.y += 1

    def char(self, ch):
        s = self.state
        if s == "g":
            if ch == "\x1b": self.state = "e"
            elif ch == "\r": self.x, self.pending = 0, False
            elif ch == "\n": self.linefeed(); self.pending = False
            elif ch == "\b": self.x, self.pending = max(0, self.x - 1), False
            elif ch == "\t": self.x = min(self.cols - 1, (self.x // 8 + 1) * 8)
            elif ch == "\a": self.bell += 1
            elif ch < " " or ch == "\x7f": pass
            else: self.put(ch)
        elif s == "e":
            if ch == "[": self.state, self.buf = "c", ""
            elif ch == "]": self.state, self.buf = "o", ""
            elif ch in "P_^X": self.state, self.buf = "s", ""
            elif ch in "()*+#%": self.state = "x"
            elif ch == "7": self.saved = (self.x, self.y); self.state = "g"
            elif ch == "8": self.x, self.y = self.saved; self.state = "g"
            elif ch == "M":
                if self.y == self.top:
                    del self.grid[self.bottom]; self.grid.insert(self.top, [(" ", 0)] * self.cols)
                else: self.y = max(0, self.y - 1)
                self.state = "g"
            elif ch == "c": self.resize(self.cols, self.rows); self.state = "g"
            else: self.state = "g"
        elif s == "x":
            self.state = "g"
        elif s == "c":
            if "@" <= ch <= "~":
                self.csi(self.buf, ch); self.state = "g"
            else: self.buf += ch
        elif s == "o":
            if ch == "\a" or (ch == "\\" and self.buf.endswith("\x1b")):
                body = self.buf.rstrip("\x1b")
                if body.startswith(("0;", "2;")): self.title = body[2:]
                self.state = "g"
            else: self.buf += ch
        elif s == "s":
            if ch == "\a" or (ch == "\\" and self.buf.endswith("\x1b")): self.state = "g"
            else: self.buf += ch

    def erase(self, y, x0, x1):
        row = self.grid[y]
        for x in range(max(0, x0), min(self.cols, x1)):
            row[x] = (" ", self.style() if self.bg else 0)

    def csi(self, params, final):
        private = params.startswith("?")
        raw = params.lstrip("?<>=!")
        nums = [int(p) if p.isdigit() else 0 for p in raw.split(";")] if raw else []
        n = lambda i, d=1: (nums[i] if len(nums) > i and nums[i] else d)
        self.pending = False if final in "ABCDHfGd" else self.pending
        if final in "Hf": self.y, self.x = min(self.rows - 1, n(0) - 1), min(self.cols - 1, n(1) - 1)
        elif final == "A": self.y = max(self.top, self.y - n(0))
        elif final == "B": self.y = min(self.bottom, self.y + n(0))
        elif final == "C": self.x = min(self.cols - 1, self.x + n(0))
        elif final == "D": self.x = max(0, self.x - n(0))
        elif final == "G": self.x = min(self.cols - 1, n(0) - 1)
        elif final == "d": self.y = min(self.rows - 1, n(0) - 1)
        elif final == "J":
            mode = nums[0] if nums else 0
            if mode == 0: self.erase(self.y, self.x, self.cols); [self.erase(y, 0, self.cols) for y in range(self.y + 1, self.rows)]
            elif mode == 1: self.erase(self.y, 0, self.x + 1); [self.erase(y, 0, self.cols) for y in range(0, self.y)]
            else: [self.erase(y, 0, self.cols) for y in range(self.rows)]
        elif final == "K":
            mode = nums[0] if nums else 0
            if mode == 0: self.erase(self.y, self.x, self.cols)
            elif mode == 1: self.erase(self.y, 0, self.x + 1)
            else: self.erase(self.y, 0, self.cols)
        elif final == "X": self.erase(self.y, self.x, self.x + n(0))
        elif final == "P":
            row = self.grid[self.y]; count = n(0); del row[self.x:self.x + count]; row.extend([(" ", 0)] * (self.cols - len(row)))
        elif final == "@":
            row = self.grid[self.y]; count = n(0); row[self.x:self.x] = [(" ", 0)] * count; del row[self.cols:]
        elif final == "L":
            for _ in range(n(0)):
                del self.grid[self.bottom]; self.grid.insert(self.y, [(" ", 0)] * self.cols)
        elif final == "M":
            for _ in range(n(0)):
                del self.grid[self.y]; self.grid.insert(self.bottom, [(" ", 0)] * self.cols)
        elif final == "S":
            for _ in range(n(0)): del self.grid[self.top]; self.grid.insert(self.bottom, [(" ", 0)] * self.cols)
        elif final == "T":
            for _ in range(n(0)): del self.grid[self.bottom]; self.grid.insert(self.top, [(" ", 0)] * self.cols)
        elif final == "r": self.top, self.bottom = (n(0) - 1, n(1, self.rows) - 1) if nums else (0, self.rows - 1); self.x = self.y = 0
        elif final == "s": self.saved = (self.x, self.y)
        elif final == "u": self.x, self.y = self.saved
        elif final == "m": self.sgr(nums or [0])
        elif final in "hl" and private:
            for mode in nums:
                self.modes[mode] = (final == "h")
                if mode == 25: self.cursor_visible = final == "h"
                if mode in (47, 1047, 1049) and final == "h": [self.erase(y, 0, self.cols) for y in range(self.rows)]

    def sgr(self, nums):
        i = 0
        while i < len(nums):
            c = nums[i]
            if c == 0: self.fg = self.bg = ""; self.attrs = set()
            elif c in (1, 2, 3, 4, 5, 7, 9): self.attrs.add({1: "b", 2: "d", 3: "i", 4: "u", 5: "k", 7: "r", 9: "s"}[c])
            elif c in (22, 23, 24, 25, 27, 29):
                self.attrs.discard({22: "b", 23: "i", 24: "u", 25: "k", 27: "r", 29: "s"}[c])
                if c == 22: self.attrs.discard("d")
            elif 30 <= c <= 37: self.fg = str(c - 30)
            elif 90 <= c <= 97: self.fg = str(c - 90 + 8)
            elif 40 <= c <= 47: self.bg = str(c - 40)
            elif 100 <= c <= 107: self.bg = str(c - 100 + 8)
            elif c == 39: self.fg = ""
            elif c == 49: self.bg = ""
            elif c in (38, 48):
                target = "fg" if c == 38 else "bg"
                if i + 1 < len(nums) and nums[i + 1] == 5 and i + 2 < len(nums): value = str(nums[i + 2]); i += 2
                elif i + 1 < len(nums) and nums[i + 1] == 2 and i + 4 < len(nums): value = "#%02x%02x%02x" % tuple(nums[i + 2:i + 5]); i += 4
                else: value = ""
                setattr(self, target, value)
            i += 1

    def text(self):
        return "\n".join("".join(ch for ch, _ in row).rstrip() for row in self.grid).rstrip("\n")

    def lines(self):
        """Rows with one char per column (the second cell of a wide char is NUL), so str.find() returns the column."""
        return ["".join(ch if ch != "" else "\x00" for ch, _ in row) for row in self.grid]

    def find(self, needle, nth=0):
        """1-based (x, y) of the nth occurrence of `needle`, or None."""
        seen = 0
        for y, line in enumerate(self.lines()):
            start = 0
            while True:
                x = line.find(needle, start)
                if x < 0: break
                if seen == nth: return (x + 1, y + 1)
                seen += 1; start = x + 1
        return None

    def styles_map(self):
        """One char per cell: '.' background equal to the most common background, otherwise a letter per distinct (bg,reverse)."""
        counter = collections.Counter()
        for row in self.grid:
            for _, st in row: counter[self.key(st)] += 1
        base = counter.most_common(1)[0][0] if counter else ("", "")
        legend, letters = {}, iter("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
        lines = []
        for row in self.grid:
            out = []
            for _, st in row:
                k = self.key(st)
                if k == base: out.append(".")
                else:
                    if k not in legend: legend[k] = next(letters, "?")
                    out.append(legend[k])
            lines.append("".join(out).rstrip("."))
        return "\n".join(lines), {v: k for k, v in legend.items()}, base

    def key(self, style_id):
        fg, bg, attrs = self.styles[style_id]
        return (bg, "r" if "r" in attrs.split(",") else "")


# ------------------------------------------------------------------------------------------------------------ driver
class Tty:
    def __init__(self, name, argv, cols=160, rows=48, cwd=REPO, env=None):
        self.name, self.cols, self.rows = name, cols, rows
        os.makedirs(OUT, exist_ok=True)
        master, slave = os.openpty()
        fcntl.ioctl(master, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))
        environment = dict(os.environ, TERM="xterm-256color", LANG="en_US.UTF-8", LC_ALL="en_US.UTF-8")
        environment.update(env or {})
        self.pid = os.fork()
        if self.pid == 0:
            try:
                os.setsid(); fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
                for fd in (0, 1, 2): os.dup2(slave, fd)
                os.close(master)
                if slave > 2: os.close(slave)
                os.chdir(cwd); os.execvpe(argv[0], argv, environment)
            finally: os._exit(127)
        os.close(slave)
        self.master, self.argv, self.exit_status = master, argv, None
        self.vt = Vt(cols, rows)
        self.raw = bytearray()
        self.timeline = []
        self.t0 = time.monotonic()
        self.marks = []
        self.snapshots = []

    # ---- io
    def now(self): return time.monotonic() - self.t0

    def pump(self, seconds=0.2, until=None, quiet=None):
        """Reads output for `seconds`; stops early when `until(self)` holds or after `quiet` seconds without data."""
        end, last, got = time.monotonic() + seconds, time.monotonic(), 0
        while time.monotonic() < end:
            ready, _, _ = select.select([self.master], [], [], 0.01)
            if ready:
                try: data = os.read(self.master, 1 << 16)
                except OSError: data = b""
                if not data: self.reap(); return got
                self.raw += data; got += len(data); last = time.monotonic()
                self.timeline.append((self.now(), len(data)))
                self.vt.feed(data)
                if until and until(self): return got
            elif quiet is not None and got and time.monotonic() - last >= quiet: return got
            elif until and until(self): return got
        return got

    def reap(self):
        if self.exit_status is None:
            try:
                pid, status = os.waitpid(self.pid, os.WNOHANG)
                if pid: self.exit_status = status
            except ChildProcessError: self.exit_status = -1
        return self.exit_status

    def alive(self):
        return self.reap() is None

    def send(self, data, settle=0.0):
        if isinstance(data, str): data = data.encode()
        os.write(self.master, data)
        if settle: self.pump(settle)

    def type(self, text, delay=0.03):
        for ch in text:
            os.write(self.master, ch.encode()); self.pump(delay)

    def wait_for(self, needle, timeout=10.0):
        self.pump(timeout, until=lambda s: needle in s.vt.text())
        return needle in self.vt.text()

    def resize(self, cols, rows, signal_child=True):
        fcntl.ioctl(self.master, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))
        if signal_child:
            try: os.killpg(self.pid, signal.SIGWINCH)
            except ProcessLookupError: pass
        self.cols, self.rows = cols, rows
        self.vt.resize(cols, rows)

    def mark(self, label):
        self.marks.append((self.now(), len(self.raw), label))

    # ---- mouse (1-based cell coordinates as the terminal reports them)
    def mouse(self, code, x, y, release=False):
        self.send("\x1b[<%d;%d;%d%s" % (code, x, y, "m" if release else "M"))

    def click(self, x, y, settle=0.4):
        self.mouse(0, x, y); self.pump(0.05); self.mouse(0, x, y, release=True); self.pump(settle)

    def motion(self, x, y, settle=0.2): self.mouse(35, x, y); self.pump(settle)
    def drag(self, x, y, settle=0.1): self.mouse(32, x, y); self.pump(settle)
    def wheel(self, x, y, up=True, settle=0.2): self.mouse(64 if up else 65, x, y); self.pump(settle)

    # ---- capture
    def snapshot(self, label, note=""):
        text = self.vt.text()
        styles, legend, base = self.vt.styles_map()
        header = "# %s/%s  t=%.2fs  size=%dx%d  cursor=(%d,%d) visible=%s  raw_bytes=%d  alive=%s\n# %s\n" % (
            self.name, label, self.now(), self.cols, self.rows, self.vt.x + 1, self.vt.y + 1, self.vt.cursor_visible, len(self.raw), self.alive(), note)
        with open("%s/%s--%s.txt" % (OUT, self.name, label), "w") as f:
            f.write(header + text + "\n")
        with open("%s/%s--%s.styles.txt" % (OUT, self.name, label), "w") as f:
            f.write("# style map: '.' = common background %s; letters = other (bg,reverse) pairs: %s\n%s\n" % (base, legend, styles))
        self.snapshots.append((self.now(), label))
        return text

    def finish(self, detach=True):
        """Detach with Ctrl+B d (never Q), wait for exit, then force-kill the view process only."""
        if detach and self.alive():
            try:
                self.send(ESC); self.pump(0.1); self.send(CTRL_B); self.pump(0.1); self.send("d"); self.pump(2.0, until=lambda s: not s.alive())
            except OSError: pass
        if self.alive():
            try: os.kill(self.pid, signal.SIGKILL)
            except ProcessLookupError: pass
            time.sleep(0.2); self.reap()
        with open("%s/%s.raw" % (OUT, self.name), "wb") as f: f.write(self.raw)
        with open("%s/%s.timeline.tsv" % (OUT, self.name), "w") as f:
            f.write("t_seconds\tbytes\n" + "".join("%.4f\t%d\n" % row for row in self.timeline))
        with open("%s/%s.modes.txt" % (OUT, self.name), "w") as f:
            f.write("exit_status=%r\nprivate modes seen (in order of first appearance, last value): %s\n" % (self.exit_status, dict(self.vt.modes)))
            f.write("marks: %r\n" % self.marks)
        try: os.close(self.master)
        except OSError: pass


def modes_in(raw):
    """Every DECSET/DECRST sequence in the raw stream, in order."""
    return [(m.group(1).decode(), m.group(2).decode()) for m in re.finditer(rb"\x1b\[\?([0-9;]+)([hl])", bytes(raw))]


def real(name, cols=160, rows=48, extra=None, env=None):
    return Tty(name, [binary()] + (extra or []), cols, rows, REPO, env)


# ---------------------------------------------------------------------------------------------------------- fixture
LAUNCH = r'''{
  // fixture launch configurations for the runtime audit
  "version": "0.2.0",
  "configurations": [
    { "name": "echo-hello", "type": "node-terminal", "request": "launch", "command": "echo hello world", "cwd": "${workspaceFolder}" },
    { "name": "echo-fail", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo failing now; exit 3'", "cwd": "${workspaceFolder}" },
    { "name": "dev-ticker", "type": "node-terminal", "request": "launch", "command": "sh -c 'i=0; while true; do i=$((i+1)); echo tick $i; sleep 0.2; done'", "cwd": "${workspaceFolder}" },
    { "name": "test-cat", "type": "node-terminal", "request": "launch", "command": "cat", "cwd": "${workspaceFolder}" },
    { "name": "build-colors", "type": "node-terminal", "request": "launch", "command": "sh -c 'printf \"\\033[31mred\\033[0m \\033[1;32mbold-green\\033[0m \\033[38;5;208morange256\\033[0m \\033[48;2;10;80;160m truecolor-bg \\033[0m\\n\"; CLICOLOR_FORCE=1 ls -G /'", "cwd": "${workspaceFolder}" },
    { "name": "build-wide", "type": "node-terminal", "request": "launch", "command": "sh -c 'printf \"日本語テキスト 🚀 👩‍💻 ñ é ü ✓ ⚡️ ─│┌┐\\n\"; sleep 30'", "cwd": "${workspaceFolder}" },
    { "name": "dev-title", "type": "node-terminal", "request": "launch", "command": "sh -c 'printf \"\\033]0;custom-title-from-task\\007titled\\n\"; sleep 30'", "cwd": "${workspaceFolder}" },
    { "name": "test-heavy", "type": "node-terminal", "request": "launch", "command": "sh -c 'yes line-of-heavy-output | head -200000; echo HEAVY-DONE'", "cwd": "${workspaceFolder}" },
    { "name": "check-env", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo TERM=$TERM COLORTERM=$COLORTERM LANG=$LANG AUDIT_MARK=$AUDIT_MARK; tty; stty size; echo done-env'", "cwd": "${workspaceFolder}" },
    { "name": "watch-winch", "type": "node-terminal", "request": "launch", "command": "sh -c 'trap \"echo WINCH $(stty size)\" WINCH; stty size; while true; do sleep 0.2; done'", "cwd": "${workspaceFolder}" },
    { "name": "run-less", "type": "node-terminal", "request": "launch", "command": "less /etc/services", "cwd": "${workspaceFolder}" },
    { "name": "run-top", "type": "node-terminal", "request": "launch", "command": "top", "cwd": "${workspaceFolder}" },
    { "name": "run-vim", "type": "node-terminal", "request": "launch", "command": "vim -u NONE -N", "cwd": "${workspaceFolder}" },
    { "name": "stop-stubborn", "type": "node-terminal", "request": "launch", "command": "sh -c 'trap \"\" INT; echo ignoring-sigint; while true; do sleep 1; done'", "cwd": "${workspaceFolder}" },
    { "name": "stop-tree", "type": "node-terminal", "request": "launch", "command": "sh -c 'sleep 301 & sleep 302 & echo spawned-children; wait'", "cwd": "${workspaceFolder}" },
    { "name": "stop-orphan", "type": "node-terminal", "request": "launch", "command": "python3 -c \"import os,time\nif os.fork()==0:\n  os.setsid()\n  if os.fork()==0:\n    time.sleep(303)\n  os._exit(0)\nprint('double-forked orphan')\ntime.sleep(60)\"", "cwd": "${workspaceFolder}" },
    { "name": "build-longline", "type": "node-terminal", "request": "launch", "command": "sh -c 'python3 -c \"print(\\\"x\\\"*400)\"; echo END-OF-LONG-LINE; sleep 30'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep1", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo one; sleep 120'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep2", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo two; sleep 121'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep3", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo three; sleep 122'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep4", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo four; sleep 123'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep5", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo five; sleep 124'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep6", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo six; sleep 125'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep7", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo seven; sleep 126'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep8", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo eight; sleep 127'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep9", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo nine; sleep 128'", "cwd": "${workspaceFolder}" },
    { "name": "build-sleep10", "type": "node-terminal", "request": "launch", "command": "sh -c 'echo ten; sleep 129'", "cwd": "${workspaceFolder}" },
    { "name": "task-short", "type": "node-terminal", "request": "launch", "command": "sh -c 'sleep 4; echo bye'", "cwd": "${workspaceFolder}" },
    { "name": "build-multiline", "type": "node-terminal", "request": "launch", "command": "sh -c 'for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60; do echo \"line $i: some build output to scroll back through\"; done; sleep 60'", "cwd": "${workspaceFolder}" },
  ],
  "compounds": [
    { "name": "dev-both", "configurations": ["echo-hello", "dev-ticker"], "stopAll": true }
  ]
}
'''


def fixture_prepare():
    fixture_stop()
    shutil.rmtree(FIX, ignore_errors=True)
    os.makedirs(FIX + "/.vscode", exist_ok=True)
    open(FIX + "/package.json", "w").write('{"name":"fixture","scripts":{"build:docs":"nx run docs:build"}}\n')
    open(FIX + "/.vscode/launch.json", "w").write(LAUNCH)
    os.makedirs(OUT + "/fixture-config", exist_ok=True)


def fixture_view(name, cols=160, rows=48, env=None, extra=None):
    return Tty(name, [binary(), "--root", FIX, "--config", OUT + "/fixture-config/preferences.jsonl"] + (extra or []), cols, rows, FIX, env)


def fixture_daemon_pid():
    try: return int(open(FIX + "/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/daemon.pid").read().strip())
    except Exception: return None


def fixture_stop():
    """Stops only the fixture workspace daemon (and with it every fixture task)."""
    pid = fixture_daemon_pid()
    if not os.path.isdir(FIX): return
    subprocess.run([binary(), "daemon", "stop", "--root", FIX], cwd=FIX, capture_output=True, timeout=15)
    time.sleep(0.5)
    if pid:
        try:
            os.kill(pid, 0)
            os.kill(pid, signal.SIGKILL)
        except ProcessLookupError: pass


def launcher_run(tty, query, wait=0.4):
    """Types a search into the launcher and presses Enter (fixture workspace only)."""
    tty.type(query, 0.02); tty.pump(wait); tty.send(ENTER); tty.pump(wait)


def open_launcher(tty):
    tty.send(ENTER); tty.pump(0.5)            # Overview row 0 = New task


def leaks():
    """Fixture processes that outlived their task: the unique sleep durations of stop-tree / stop-orphan."""
    out = subprocess.run(["pgrep", "-fl", r"sleep 30[12]$|sleep\(303\)"], capture_output=True, text=True).stdout.strip()
    return [line[:90] for line in out.splitlines() if line]


def kill_leaks():
    subprocess.run(["pkill", "-f", r"sleep 30[12]$|sleep\(303\)"], capture_output=True)


QUERY = {"echo-hello": "echo-hello world", "echo-fail": "echo-fail failing", "dev-ticker": "dev-ticker while", "dev-both": "dev-both", "build-sleep1": "build-sleep1"}


def start_task(t, name, first=False, settle=0.8):
    """Opens a launcher (first window: New task row, otherwise Ctrl+B n), filters to the entry named `name`, presses Enter. Fixture only."""
    if first: open_launcher(t)
    else: leader(t, "n", 0.4)
    t.type(QUERY.get(name, name), 0.015); t.pump(0.3)
    t.send(ENTER); t.pump(settle)


def log(*args):
    print(time.strftime("%H:%M:%S"), *args, flush=True)


# ------------------------------------------------------------------------------------------------------- scenarios
def scn_a():
    """Startup frames at 160x48 and 80x24 on the real workspace; terminal modes."""
    for cols, rows in ((160, 48), (80, 24)):
        t = real("a-startup-%dx%d" % (cols, rows), cols, rows)
        t.pump(0.15); t.snapshot("t0150ms")
        t.pump(0.35); t.snapshot("t0500ms")
        t.pump(1.5); t.snapshot("t2s")
        t.pump(3.0); t.snapshot("t5s")
        t.send(CTRL_B); t.pump(0.2); t.snapshot("leader-armed")
        t.send(ESC); t.pump(0.2)
        t.send(CTRL_B); t.send("e"); t.pump(0.3)      # cancel discovery: never let a real-workspace view rewrite the cache
        t.snapshot("discovery-cancelled")
        log("modes", modes_in(t.raw)[:12])
        first = [(round(a, 3), b) for a, b in t.timeline[:6]]
        log("first output chunks (t,bytes)", first, "total", sum(b for _, b in t.timeline))
        t.finish()
        log("exit", t.exit_status)


BIG = OUT + "/fixture-big"
VERBS = ["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"]


def big_fixture(count=44000):
    """A workspace whose launch.json yields `count` launcher entries (seed path, no Nx needed): same Wizard load as the real 44k catalog."""
    os.makedirs(BIG + "/.vscode", exist_ok=True)
    if os.path.exists(BIG + "/.vscode/launch.json") and os.path.getsize(BIG + "/.vscode/launch.json") > 1_000_000:
        return
    plugins = ["mit-bestand", "cad", "fem", "raster", "puzzle3d", "wfc", "forms", "layout", "note", "trinity", "lowpoly", "energy", "generation3d", "draw", "world3d"]
    renderers = ["react", "wgpu-wasm", "wgpu-native"]
    rows = []
    for i in range(count):
        verb, plugin, renderer = VERBS[i % len(VERBS)], plugins[(i // 7) % len(plugins)], renderers[(i // 3) % 3]
        name = "%s-%s-%s-%05d" % (verb, plugin, renderer, i)
        command = "bun nx run @semio-tech/framework-os-dev:%s-%s-%s-dev" % (verb, plugin, renderer)
        rows.append('    { "name": "%s", "type": "node-terminal", "request": "launch", "command": "%s", "cwd": "${workspaceFolder}", "env": {"S_OS_PORT": "%d"} }' % (name, command, 6000 + i % 900))
    open(BIG + "/.vscode/launch.json", "w").write('{\n  "version": "0.2.0",\n  "configurations": [\n' + ",\n".join(rows) + "\n  ]\n}\n")
    open(BIG + "/package.json", "w").write('{"name":"big","scripts":{}}\n')


def rss_kb(pid):
    out = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return int(out) if out else 0


def measure_key(tty, data, quiet=0.06, limit=6.0):
    """Sends `data`; returns (seconds to first output byte, seconds until output went quiet, bytes)."""
    start = tty.now()
    tty.send(data)
    tty.pump(limit, quiet=quiet)
    chunks = [(t, n) for t, n in tty.timeline if t >= start]
    if not chunks: return (None, None, 0)
    return (round(chunks[0][0] - start, 3), round(chunks[-1][0] - start, 3), sum(n for _, n in chunks))


def footer(tty):
    lines = tty.vt.text().split("\n")
    return lines[-1] if lines else ""


def scn_c():
    """Mouse on the fixture workspace: motion, clicks on rows/tabs/controls, wheel, drag. Every action is followed by a snapshot."""
    fixture_prepare()
    t = fixture_view("c-mouse", 160, 48)
    t.pump(1.0); t.snapshot("00-overview")
    log("modes", modes_in(t.raw))
    before = t.vt.styles_map()[0]
    for x, y in ((20, 8), (20, 9), (20, 10), (10, 5), (60, 5), (140, 5), (154, 5), (156, 5), (158, 5)):
        t.motion(x, y, 0.1)
    t.snapshot("01-after-motion-without-button")
    log("hover changed the style map:", t.vt.styles_map()[0] != before)
    for x, y in ((20, 9), (20, 10)):
        t.mouse(32, x, y); t.pump(0.1)                      # motion with the left button held (what ?1002 reports)
    t.snapshot("02-after-drag-motion")
    pos = t.vt.find("Settings"); log("Settings at", pos)
    t.click(pos[0] + 3, pos[1]); t.snapshot("03-click-settings-row")
    pos = t.vt.find("Back to tasks"); log("Back at", pos)
    t.click(pos[0] + 3, pos[1]); t.snapshot("04-click-back-to-tasks")
    pos = t.vt.find("New task"); t.click(pos[0] + 3, pos[1]); t.snapshot("05-click-new-task")
    t.type("echo-hello", 0.02); t.pump(0.3); t.snapshot("06-filter-echo-hello")
    rows = [y for y in range(1, 49) if t.vt.lines()[y - 1].find("launch / echo-hello") >= 0]
    log("hit rows", rows)
    t.click(8, rows[0]); t.pump(1.0); t.snapshot("07-single-click-on-launcher-row")
    log("single click on a launcher row started a task:", "hello world" in t.vt.text())
    # a second and third task through the keyboard so there are tabs to click
    t.send(CTRL_B); t.send("n"); t.pump(0.4)
    launcher_run(t, "dev-ticker"); t.pump(0.8); t.snapshot("08-ticker-started")
    t.send(CTRL_B); t.send("n"); t.pump(0.4)
    launcher_run(t, "test-cat"); t.pump(0.8); t.snapshot("09-cat-started")
    line = t.vt.lines()[4]
    log("tab strip:", line.replace("\x00", "")[:158])
    # click each visible tab label
    for needle in ("echo", "ticker", "cat", "Tasks"):
        pos = t.vt.find(needle)
        if pos and pos[1] <= 6:
            t.click(pos[0] + 1, pos[1]); t.snapshot("10-click-tab-" + needle)
    # window controls of the focused window
    for glyph, name in (("⤢", "maximize"), ("⤢", "restore"), ("⧉", "newtab"), ("✕", "close")):
        pos = t.vt.find(glyph)
        if pos:
            log("click", name, pos); t.click(pos[0], pos[1], 0.6); t.snapshot("11-click-" + name)
    # wheel over launcher and over scrollback
    t.send(CTRL_B); t.send("n"); t.pump(0.4)
    before = t.vt.text()
    for _ in range(5): t.wheel(30, 20, up=False, settle=0.1)
    t.snapshot("12-wheel-down-over-launcher"); log("wheel moved the launcher:", t.vt.text() != before)
    t.send(ESC); t.pump(0.2)
    t.finish(); fixture_stop(); log("done")


def leader(t, key, settle=0.4):
    t.send(CTRL_B); t.pump(0.08); t.send(key); t.pump(settle)


def tabline(t):
    return t.vt.lines()[4].replace("\x00", "")


def scn_d():
    """Windows: split, zoom, Tab, Ctrl+W, tab strip and titles at each step; then resize the pty."""
    fixture_prepare()
    t = fixture_view("d-windows", 160, 48)
    t.pump(1.0); t.snapshot("00-start")
    open_launcher(t); launcher_run(t, "echo-hello"); t.pump(0.6); t.snapshot("01-first-task")
    steps = []
    leader(t, "-"); t.snapshot("02-ctrl-b-minus-split"); 
    leader(t, "|"); t.snapshot("03-ctrl-b-pipe-split")
    leader(t, "z"); t.snapshot("04-ctrl-b-z-zoom")
    leader(t, "z"); t.snapshot("05-ctrl-b-z-unzoom")
    for i in range(4):
        t.send(TAB); t.pump(0.3); t.snapshot("06-tab-%d" % i)
    for i in range(2):
        t.send(BTAB); t.pump(0.3); t.snapshot("07-backtab-%d" % i)
    # click into another pane and type: does the keyboard follow the mouse?
    t.send(b"\x17"); t.pump(0.4); t.snapshot("08-ctrl-w-closes-focused")
    leader(t, "n"); t.snapshot("09-ctrl-b-n-new-tab")
    leader(t, "n"); leader(t, "n"); t.snapshot("10-three-new-tabs")
    t.send(TAB); t.pump(0.3); t.send(TAB); t.pump(0.3); t.snapshot("11-tab-tab")
    t.send(b"\x17"); t.pump(0.4); t.snapshot("12-ctrl-w-after-tabbing")
    leader(t, "x"); t.snapshot("13-ctrl-b-x")
    leader(t, "h"); t.snapshot("14-ctrl-b-h")
    leader(t, "p"); t.snapshot("15-ctrl-b-p-settings")
    leader(t, "s"); t.snapshot("16-ctrl-b-s-show-all")
    leader(t, "t"); t.snapshot("17-ctrl-b-t")
    # resize
    t.send(ESC); t.pump(0.2)
    for cols, rows in ((100, 30), (60, 20), (40, 12), (160, 48)):
        t.resize(cols, rows); t.pump(1.0)
        t.snapshot("20-resized-%dx%d-no-input" % (cols, rows), "after TIOCSWINSZ+SIGWINCH, before any key: bytes since resize=%d" % len(t.raw))
        n0 = len(t.raw); t.send(DOWN); t.pump(0.6)
        t.snapshot("21-resized-%dx%d-after-key" % (cols, rows), "bytes written after one key: %d" % (len(t.raw) - n0))
    t.finish(); fixture_stop(); log("done")


def scn_b():
    """Launcher with a 44k-entry catalog: open, search, navigate, per-key latency. Synthetic fixture, so nothing real can run."""
    big_fixture()
    os.makedirs(OUT + "/fixture-config", exist_ok=True)
    t = Tty("b-launcher-44k", [binary(), "--root", BIG, "--config", OUT + "/fixture-config/big-preferences.jsonl"], 160, 48, BIG)
    results = {"loadavg": os.getloadavg()}
    t.pump(0.3); t.snapshot("first-frame")
    started = t.now()
    t.pump(90, until=lambda s: re.search(r"\b4[0-9]{4} commands", footer(s)) is not None)
    results["seconds_until_44k_commands_in_footer"] = round(t.now() - started, 2)
    t.pump(1.0); t.snapshot("catalog-ready")
    results["view_rss_mb_after_catalog"] = rss_kb(t.pid) // 1024
    log("catalog", results)
    results["open_launcher"] = measure_key(t, ENTER, quiet=0.15, limit=10)
    t.snapshot("launcher-open")
    results["rss_mb_launcher"] = rss_kb(t.pid) // 1024
    queries = ["build mit bestand zwischenbericht", "dev cad react", "test dashboard"]
    for query in queries:
        per_key = []
        for ch in query:
            per_key.append(measure_key(t, ch, quiet=0.05, limit=8))
        t.snapshot("search-" + query.replace(" ", "_"))
        firsts = [k[0] for k in per_key if k[0] is not None]; lasts = [k[1] for k in per_key if k[1] is not None]
        results["search:" + query] = {"keys": len(per_key), "first_byte_ms_p50": round(sorted(firsts)[len(firsts) // 2] * 1000), "first_byte_ms_max": round(max(firsts) * 1000), "settle_ms_p50": round(sorted(lasts)[len(lasts) // 2] * 1000), "settle_ms_max": round(max(lasts) * 1000), "bytes_per_key": per_key[-1][2]}
        log(query, results["search:" + query])
        for _ in query: t.send(b"\x7f"); t.pump(0.06)       # clear again (Backspace); the last one is the cost of re-filtering 44k
        t.pump(0.5)
    t.snapshot("after-clear")
    # navigation keys
    before = t.vt.text()
    nav = {}
    for label, key in (("Down", DOWN), ("PgDn", PGDN), ("End", END), ("Home", HOME), ("PgUp", PGUP), ("Ctrl+D", b"\x04"), ("Alt+Down", b"\x1b[1;3B")):
        nav[label] = measure_key(t, key, quiet=0.06)
        t.snapshot("nav-" + label.replace("+", "_"))
    results["nav"] = nav
    # type then arrows, Esc behaviour
    t.type("test", 0.03); t.pump(0.3)
    t.snapshot("typed-test")
    t.send(ESC); t.pump(0.4); t.snapshot("esc-1")
    t.send(ESC); t.pump(0.6); t.snapshot("esc-2")
    results["rss_mb_end"] = rss_kb(t.pid) // 1024
    json.dump(results, open(OUT + "/b-launcher-44k.results.json", "w"), indent=1)
    t.finish()
    fixture_stop_root(BIG)
    log("done", json.dumps(results)[:1500])


def scn_b_real():
    """Real workspace launcher: time until the first search hit (seed), the three requested searches, never Enter."""
    t = real("b-real-launcher", 160, 48)
    results = {"loadavg": os.getloadavg()}
    t.pump(0.6); t.send(ENTER); t.pump(0.4); t.snapshot("launcher-empty-at-start")
    t.type("build mit bestand zwischenbericht", 0.02)
    start = t.now()
    t.pump(60, until=lambda s: any(" / " in l and "zwischenbericht" in l.lower() and not l.startswith("│/") for l in s.vt.text().split("\n")))
    results["seconds_until_first_hit"] = round(t.now() - start + 0.7, 2)
    t.send(CTRL_B); t.pump(0.1); t.send("e"); t.pump(0.3)            # cancel discovery before its walk can finish and rewrite the cache
    t.pump(0.5); t.snapshot("hits-build_mit_bestand_zwischenbericht")
    results["rss_mb"] = rss_kb(t.pid) // 1024
    for query in ("dev cad react", "test dashboard", "launch json", "bun nx run"):
        t.send(ESC); t.pump(0.2)                                      # Esc clears a non-empty filter (Backspace on an empty one leaves the launcher)
        t.type(query, 0.02); t.pump(0.4); t.snapshot("hits-" + query.replace(" ", "_"))
    t.send(ESC); t.pump(0.3); t.send(ESC); t.pump(0.4); t.snapshot("after-esc-esc")
    results["footer"] = footer(t)
    json.dump(results, open(OUT + "/b-real-launcher.results.json", "w"), indent=1)
    t.finish(); log(results)


def fixture_stop_root(root):
    path = root + "/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/daemon.pid"
    try: pid = int(open(path).read().strip())
    except Exception: pid = None
    subprocess.run([binary(), "daemon", "stop", "--root", root], cwd=root, capture_output=True, timeout=15)
    time.sleep(0.4)
    if pid:
        try: os.kill(pid, 0); os.kill(pid, signal.SIGKILL)
        except ProcessLookupError: pass




REAL_SOCKET_HASH = "semio-dashboard-d4bd4b41432b4211"


def _frames(buf):
    out = []
    while len(buf) >= 4:
        n = struct.unpack("<I", buf[:4])[0]
        if len(buf) < 4 + n: break
        out.append((buf[4], bytes(buf[5:4 + n]))); del buf[:4 + n]
    return out


def fixture_socket():
    """The Unix socket of the fixture daemon: the connectable socket whose greeting names the pid in the fixture pid file."""
    import glob, socket
    want = fixture_daemon_pid()
    for path in sorted(glob.glob(os.environ.get("TMPDIR", "/tmp") + "/semio-dashboard-*/daemon.sock"), key=os.path.getmtime, reverse=True):
        if REAL_SOCKET_HASH in path: continue
        try:
            s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.settimeout(0.5); s.connect(path)
            buf = bytearray(s.recv(4096)); frames = _frames(buf); s.close()
            if frames and json.loads(frames[0][1]).get("daemon_pid") == want: return path
        except Exception: continue
    return None


def daemon_sessions():
    """Oracle: the session table the fixture daemon holds (read-only `list`)."""
    import socket
    path = fixture_socket()
    if not path: return None
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.settimeout(0.3); s.connect(path)
    payload = json.dumps({"type": "list"}).encode(); s.sendall(struct.pack("<IB", len(payload) + 1, 1) + payload)
    buf, end = bytearray(), time.time() + 1.5
    while time.time() < end:
        try: buf += s.recv(1 << 20)
        except socket.timeout: pass
        for kind, body in _frames(buf):
            if kind == 1:
                m = json.loads(body)
                if m["type"] == "sessions":
                    s.close()
                    return [(x["session_id"].split("-")[-1][-6:], x["status"], x["code"], x["command"]["args"][-1][:34] if x["command"]["args"] else x["command"]["cmd"], x["command"]["cols"], x["command"]["rows"]) for x in m["sessions"]]
    s.close(); return None


def sess(label):
    log(label, "daemon sessions:", daemon_sessions())


def titles(t):
    """The tab strip rows (3..5) with NUL fillers removed."""
    return [l.replace("\x00", "")[:158] for l in t.vt.lines()[2:5]]


def scn_f1():
    """Task lifecycle on the fixture: echo, failure, long-running, stop, restart, kill, stubborn task, process trees."""
    fixture_prepare(); kill_leaks()
    t = fixture_view("f1-lifecycle", 160, 48)
    t.pump(1.0)
    start_task(t, "echo-hello", first=True); t.snapshot("01-echo"); sess("after echo-hello")
    start_task(t, "echo-fail"); t.snapshot("02-echo-fail"); sess("after echo-fail")
    start_task(t, "dev-ticker", settle=1.2); t.snapshot("03-ticker-running")
    leader(t, "c", 0.3); t.snapshot("04-ticker-after-ctrl-b-c-0.3s"); sess("0.3s after Ctrl+B c")
    t.pump(0.9); t.snapshot("05-ticker-after-1.2s"); sess("1.2s after Ctrl+B c")
    t.pump(2.0); t.snapshot("06-ticker-after-3.2s"); sess("3.2s after Ctrl+B c")
    ticks_before = t.vt.text().count("tick ")
    leader(t, "r", 1.5); t.snapshot("07-ticker-restarted"); sess("after Ctrl+B r"); log("tick lines on screen", ticks_before, "->", t.vt.text().count("tick "))
    leader(t, "k", 1.0); t.snapshot("08-ticker-killed"); sess("after Ctrl+B k")
    start_task(t, "stop-stubborn", settle=1.0); t.snapshot("09-stubborn-running")
    leader(t, "c", 0.3); t.snapshot("10-stubborn-stop-0.3s"); sess("stubborn +0.3s")
    t.pump(1.0); t.snapshot("11-stubborn-stop-1.3s"); sess("stubborn +1.3s")
    t.pump(1.5); t.snapshot("12-stubborn-stop-2.8s"); sess("stubborn +2.8s")
    start_task(t, "stop-tree", settle=1.5); log("tree running, processes:", leaks()); sess("tree")
    leader(t, "k", 1.5); log("after Ctrl+B k, surviving processes:", leaks()); t.snapshot("13-tree-killed"); sess("tree killed")
    kill_leaks()
    start_task(t, "stop-orphan", settle=2.0); log("orphan running, processes:", [l[:60] for l in leaks() if l[:1].isdigit()]); sess("orphan")
    leader(t, "k", 1.5); log("after Ctrl+B k, surviving processes:", [l[:60] for l in leaks() if l[:1].isdigit()]); t.snapshot("14-orphan-killed"); sess("orphan killed")
    kill_leaks()
    leader(t, "h", 0.5); t.snapshot("15-overview-with-sessions")
    t.finish(); fixture_stop(); kill_leaks(); log("done")


def scn_f2():
    """Interactive input into a PTY task: cat, '/', Tab, bracketed paste, burst input, Ctrl+D; then less, vim and top."""
    fixture_prepare()
    t = fixture_view("f2-interactive", 160, 48)
    t.pump(1.0)
    start_task(t, "test-cat", first=True, settle=1.0); t.snapshot("01-cat-running"); log("cat titles", titles(t)[1])
    t.type("hello cat", 0.02); t.send(ENTER); t.pump(0.5); t.snapshot("02-typed-hello")
    log("cat echoed twice:", t.vt.text().count("hello cat") >= 2)
    t.type("a/b", 0.05); t.pump(0.4); t.snapshot("03-typed-slash")
    log("after typing a/b, last row:", t.vt.lines()[-3:])
    t.type("zzz", 0.05); t.send(ENTER); t.pump(0.5); t.snapshot("04-after-slash-then-zzz-enter")
    t.send(ESC); t.pump(0.3); t.snapshot("05-esc"); t.send(CTRL_B); t.send("t"); t.pump(0.3)
    t.type("after-t", 0.03); t.send(ENTER); t.pump(0.5); t.snapshot("06-after-ctrl-b-t-typing")
    n = len(t.vt.text())
    t.send(TAB); t.pump(0.4); t.snapshot("07-tab-key"); log("Tab: footer", footer(t)[:90])
    t.send(b"\x1b[200~pasted-text-line\x1b[201~"); t.pump(0.5); t.send(ENTER); t.pump(0.4); t.snapshot("08-bracketed-paste")
    log("bracketed paste reached cat:", "pasted-text-line" in t.vt.text())
    t.send(b"\x03"); t.pump(0.4); t.snapshot("09-ctrl-c-after")
    t.finish(detach=True); fixture_stop()


def scn_f2b():
    """Burst input (paste-like), Ctrl+D, less, top, vim."""
    fixture_prepare()
    t = fixture_view("f2b-burst", 160, 48)
    t.pump(1.0)
    start_task(t, "test-cat", first=True, settle=1.0)
    tokens = " ".join("t%03d" % i for i in range(300))
    t.send(tokens.encode() + b"\r"); t.pump(3.0); t.snapshot("01-burst-300-tokens")
    seen = sum(1 for i in range(300) if ("t%03d" % i) in t.vt.text())
    log("burst of 300 tokens in one write: distinct tokens visible", seen, "footer", footer(t)[-70:])
    t.send(b"\x04"); t.pump(0.8); t.snapshot("02-ctrl-d"); log("Ctrl+D titles", titles(t)[1])
    start_task(t, "run-less", settle=1.0); t.snapshot("03-less")
    t.type("/", 0.1); t.type("services", 0.04); t.send(ENTER); t.pump(0.5); t.snapshot("04-less-search-slash")
    t.send(b" "); t.pump(0.3); t.snapshot("05-less-space"); t.send(PGDN); t.pump(0.3); t.snapshot("06-less-pgdn")
    t.send(b"q"); t.pump(0.5); t.snapshot("07-less-q")
    start_task(t, "run-top", settle=2.0); t.snapshot("08-top")
    t.send(b"q"); t.pump(0.5); t.snapshot("09-top-q")
    start_task(t, "run-vim", settle=1.5); t.snapshot("10-vim")
    t.type("ihello vim", 0.05); t.pump(0.3); t.snapshot("11-vim-insert"); t.send(ESC); t.pump(0.4); t.snapshot("12-vim-esc")
    log("after Esc: footer", footer(t)[:80]); t.type(":q!", 0.1); t.pump(1.0); t.snapshot("13-vim-colon-q")
    log("view still alive after typing ':q!' to vim:", t.alive())
    t.finish(); fixture_stop()


def scn_f3():
    """Heavy output, 12 tasks (tab strip), detach, reattach: restore, replay size, hidden-tab terminal geometry, settle time."""
    fixture_prepare()
    t = fixture_view("f3-heavy-and-tabs", 160, 48)
    t.pump(1.0)
    start = t.now(); start_task(t, "test-heavy", first=True, settle=0.1)
    t.pump(60, until=lambda s: "HEAVY-DONE" in s.vt.text())
    log("heavy output until HEAVY-DONE: %.1fs" % (t.now() - start), "bytes to the view", len(t.raw), "footer", footer(t)[-80:], "rss", rss_kb(t.pid) // 1024, "MB")
    t.snapshot("01-heavy-done")
    for i in range(1, 11): start_task(t, "build-sleep%d" % i, settle=0.35)
    t.snapshot("02-ten-more-tasks"); log("tab strip with 11 tasks:", titles(t))
    start_task(t, "build-multiline", settle=1.0); t.snapshot("03-multiline")
    t.snapshot("04-before-detach")
    t.send(ESC); t.pump(0.1); leader(t, "d", 1.0); log("view exit status", t.exit_status)
    t.finish(detach=False)
    t2 = fixture_view("f3-reattach", 160, 48)
    stamp = time.monotonic()
    for label, wait in (("a-0.3s", 0.3), ("b-1s", 0.7), ("c-3s", 2.0), ("d-8s", 5.0)):
        t2.pump(wait); t2.snapshot("10-reattach-" + label)
    log("reattach: bytes", len(t2.raw), "settle at", [round(a, 2) for a, _ in t2.timeline[-3:]], "rss", rss_kb(t2.pid) // 1024, "MB", "footer", footer(t2)[-70:])
    log("reattach titles", titles(t2))
    # walk the windows and snapshot each: geometry of background-tab terminals
    for i in range(14):
        t2.send(TAB); t2.pump(0.35); t2.snapshot("11-tab-%02d" % i, "after %d Tab presses" % (i + 1))
    t2.finish(detach=False); fixture_stop()


def scn_f4():
    """Colors, wide/emoji, window title escape, long line, env/TERM, compound, stale daemon environment."""
    fixture_prepare()
    t = fixture_view("f4-render", 160, 48, env={"AUDIT_MARK": "first-view"})
    t.pump(1.0)
    start_task(t, "build-colors", first=True, settle=1.0); t.snapshot("01-colors")
    rows = t.vt.lines()
    red = [(x, y) for y, row in enumerate(t.vt.grid) for x, (ch, st) in enumerate(row) if ch == "r" and t.vt.styles[st][0] == "1"]
    log("red text cells found:", len(red), "styles used:", sorted({t.vt.styles[st] for row in t.vt.grid for _, st in row if t.vt.styles[st][0] or "b" in t.vt.styles[st][2]})[:12])
    start_task(t, "build-wide", settle=1.0); t.snapshot("02-wide-emoji")
    start_task(t, "dev-title", settle=1.0); t.snapshot("03-osc-title"); log("title row", titles(t)[1])
    start_task(t, "build-longline", settle=1.0); t.snapshot("04-long-line")
    start_task(t, "check-env", settle=1.0); t.snapshot("05-env"); log("env screen:", [l for l in t.vt.text().split("\n") if "TERM=" in l or "tty" in l or "/dev/" in l or "AUDIT" in l or l.strip().replace(" ", "").isdigit()][:5])
    start_task(t, "dev-both", settle=1.5); t.snapshot("06-compound"); log("compound titles", titles(t))
    leader(t, "h", 0.5); t.snapshot("07-overview"); 
    t.send(ESC); t.pump(0.1); leader(t, "d", 0.8); t.finish(detach=False)
    t2 = fixture_view("f4-second-view", 160, 48, env={"AUDIT_MARK": "second-view"})
    t2.pump(2.0)
    start_task(t2, "check-env", first=False, settle=1.5)
    t2.snapshot("08-env-from-second-view"); log("second view env line:", [l for l in t2.vt.text().split("\n") if "AUDIT_MARK" in l])
    t2.finish(); fixture_stop()


def scn_f5():
    """Does a task see the view's resize? watch-winch prints `stty size` on SIGWINCH."""
    fixture_prepare()
    t = fixture_view("f5-winch", 160, 48)
    t.pump(1.0)
    start_task(t, "watch-winch", first=True, settle=1.0); t.snapshot("01-initial")
    log("initial", [l.strip() for l in t.vt.text().split("\n") if l.strip().replace(" ", "").isdigit() or "WINCH" in l])
    t.resize(120, 36); t.pump(1.5); t.snapshot("02-after-resize-120x36")
    t.send(DOWN); t.pump(0.6)
    log("daemon-side size / WINCH lines after resize:", [l.strip() for l in t.vt.text().split("\n") if "WINCH" in l or l.strip().replace(" ", "").isdigit()])
    t.finish(); fixture_stop()



def cpu(pid):
    out = subprocess.run(["ps", "-o", "%cpu=,rss=,state=", "-p", str(pid)], capture_output=True, text=True).stdout.split()
    return (float(out[0]), int(out[1]) // 1024, out[2]) if len(out) >= 3 else None


def scn_f3b():
    """Heavy output throughput: CPU of the view and the daemon while `yes | head -200000` (4.4 MB) streams; time until the last line shows."""
    fixture_prepare()
    t = fixture_view("f3b-heavy-cpu", 160, 48)
    t.pump(1.0)
    start = t.now(); start_task(t, "test-heavy", first=True, settle=0.05)
    daemon = fixture_daemon_pid(); samples = []
    done = None
    while t.now() - start < 120 and done is None:
        t.pump(1.0)
        samples.append((round(t.now() - start, 1), cpu(t.pid), cpu(daemon), os.getloadavg()[0]))
        if "HEAVY-DONE" in t.vt.text(): done = t.now() - start
    log("HEAVY-DONE visible after", done)
    for row in samples[:12] + samples[-3:]: log("sample (t, view cpu%/rss/state, daemon cpu%/rss/state, load)", row)
    sess("heavy"); t.snapshot("01-final")
    t.finish(); fixture_stop()



def scn_g(seconds="20", binary_override=None):
    """Footer status over time on the real workspace: connection state transitions (previous note: flaps once per second)."""
    seconds = float(seconds)
    t = real("g-footer-real", 160, 48)
    series = []; last = None
    t.pump(1.0)
    t.send(CTRL_B); t.pump(0.05); t.send("e")                      # cancel discovery so a real-workspace view cannot rewrite the cache
    start = t.now()
    while t.now() - start < seconds:
        t.pump(0.25)
        f = footer(t).strip()
        status = f.split("  ")[-1].strip() if "  " in f else f
        if status != last: series.append((round(t.now(), 2), status)); last = status
    for row in series[:60]: log("footer", row)
    transitions = sum(1 for _, st in series if "reconnecting" in st or "disconnected" in st or "daemon" in st)
    log("distinct footer states", len(series), "connection-related states", transitions)
    t.snapshot("final")
    t.finish()


def scn_g_old_binary(seconds="20"):
    """Same with the previous installed executable (the one the running daemon was started from): version-skew control experiment."""
    old = REPO + "/.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/153facd59ca0d86ee619624020f6e9ace89c088ccac42251ade3c9c68fab95eb/semio"
    t = Tty("g-footer-real-old-binary", [old], 160, 48, REPO)
    series = []; last = None
    t.pump(1.0); t.send(CTRL_B); t.pump(0.05); t.send("e")
    start = t.now()
    while t.now() - start < float(seconds):
        t.pump(0.25)
        f = footer(t).strip(); status = f.split("  ")[-1].strip() if "  " in f else f
        if status != last: series.append((round(t.now(), 2), status)); last = status
    for row in series[:60]: log("footer", row)
    t.snapshot("final"); t.finish()



def scn_h():
    """Real workspace attach: walk every restored session window with Tab and snapshot it (all 7 real sessions are exited; no key but Tab is sent)."""
    t = real("h-real-restore", 160, 48)
    t.pump(1.5); t.send(CTRL_B); t.pump(0.05); t.send("e"); t.pump(0.4)
    t.snapshot("00-after-attach")
    for i in range(9):
        t.send(TAB); t.pump(0.5)
        t.snapshot("01-tab-%d" % (i + 1), "after %d Tab presses; footer: %s" % (i + 1, footer(t).strip()[:90]))
    t.finish()



def status_series(t, seconds, step=0.2):
    series, last = [], None
    end = t.now() + seconds
    while t.now() < end:
        t.pump(step)
        f = footer(t).strip(); status = f.split("  ")[-1].strip() if "  " in f else f
        if status != last: series.append((round(t.now(), 2), status)); last = status
    return series


def scn_i():
    """Second view attaching to a daemon that holds retained output and a running, chattering task: reproduce the 'disconnected/connected' flap."""
    fixture_prepare()
    a = fixture_view("i-view-a", 160, 48)
    a.pump(1.0)
    start_task(a, "test-heavy", first=True, settle=0.1)
    a.pump(75, until=lambda s: "HEAVY-DONE" in s.vt.text())
    start_task(a, "dev-ticker", settle=1.0)
    sess("before second view")
    b = fixture_view("i-view-b", 160, 48)
    b.pump(0.5)
    series = status_series(b, 15)
    for row in series[:40]: log("view B footer", row)
    log("view B: connection-related states:", sum(1 for _, st in series if "disconnected" in st or "reconnecting" in st or "unavailable" in st or "daemon" in st), "of", len(series))
    series_a = status_series(a, 3)
    log("view A footer while B flaps:", series_a[:5])
    b.snapshot("final"); a.snapshot("final")
    b.finish(); a.finish(); fixture_stop()



def scn_j():
    """After a reattach with retained output: does the reattached view still receive live output and status of a task it starts itself?"""
    fixture_prepare()
    a = fixture_view("j-first-view", 160, 48)
    a.pump(1.0)
    start_task(a, "test-heavy", first=True, settle=0.1)
    a.pump(75, until=lambda s: "HEAVY-DONE" in s.vt.text())
    a.send(ESC); a.pump(0.1); leader(a, "d", 1.0); a.finish(detach=False)
    sess("after first view detached (heavy output retained, nothing running)")
    b = fixture_view("j-reattached-view", 160, 48)
    b.pump(1.5); b.snapshot("00-reattached")
    start_task(b, "echo-hello", first=True, settle=2.5); b.snapshot("01-after-starting-echo-hello")
    log("echo-hello output visible in the reattached view:", "hello world" in b.vt.text(), "| footer:", footer(b).strip()[-90:])
    series = status_series(b, 6)
    log("footer states in the following 6 s:", series[:12])
    b.snapshot("02-final"); sess("j end")
    b.finish(); fixture_stop()



def scn_k():
    """Settings, language/appearance switches (journal on the fixture), the q-quits-while-typing trap, Ctrl+C, close-window focus, Ctrl+B Q."""
    fixture_prepare()
    journal = OUT + "/fixture-config/preferences.jsonl"
    if os.path.exists(journal): os.remove(journal)
    t = fixture_view("k-settings", 160, 48)
    t.pump(1.0); t.snapshot("00-start")
    leader(t, "l", 0.8); t.snapshot("01-ctrl-b-l-german"); log("journal after Ctrl+B l:", open(journal).read().strip() if os.path.exists(journal) else None)
    leader(t, "a", 0.8); t.snapshot("02-ctrl-b-a-light"); log("journal after Ctrl+B a:", open(journal).read().strip().splitlines()[-1] if os.path.exists(journal) else None)
    leader(t, "p", 0.8); t.snapshot("03-settings-german-light")
    t.send(DOWN); t.send(DOWN); t.send(DOWN); t.send(ENTER); t.pump(1.0); t.snapshot("04-renderer-toggled")
    leader(t, "l", 0.8); leader(t, "a", 0.8); t.snapshot("05-back-to-english-dark")
    log("journal events:", len(open(journal).read().strip().splitlines()))
    t.send(ESC); t.pump(0.3); t.send(ESC); t.pump(0.3)
    t.send(b"\x03"); t.pump(0.4); t.snapshot("06-ctrl-c-in-overview"); log("alive after Ctrl+C in Overview:", t.alive())
    try: t.type("queue", 0.1); t.pump(0.6)
    except OSError: pass
    t.reap(); log("alive after typing 'queue' into the Overview filter:", t.alive(), "exit", t.exit_status)
    t.finish(detach=False)
    # close-window focus
    t = fixture_view("k-close-focus", 160, 48)
    t.pump(1.0)
    start_task(t, "build-sleep1", first=True); start_task(t, "build-sleep2"); start_task(t, "build-sleep3"); start_task(t, "build-sleep4")
    t.snapshot("10-four-tabs-last-active"); log("active body before close:", [l.strip("│ ") for l in t.vt.text().split("\n")[5:7]])
    t.send(ESC); t.pump(0.1)
    t.send(TAB); t.pump(0.3); t.send(TAB); t.pump(0.3); t.snapshot("11-after-two-tabs"); log("after 2 Tab presses, body:", [l.strip("│ ") for l in t.vt.text().split("\n")[5:7]], footer(t)[:60])
    t.send(b"\x17"); t.pump(0.5); t.snapshot("12-after-ctrl-w"); log("after Ctrl+W, body:", [l.strip("│ ") for l in t.vt.text().split("\n")[5:7]], "| footer:", footer(t)[:70])
    t.send(b"hello"); t.pump(0.4); t.snapshot("13-typing-after-ctrl-w"); log("typed 'hello' after Ctrl+W, screen shows hello:", "hello" in t.vt.text())
    leader(t, "Q", 2.0)
    log("view alive after Ctrl+B Q:", t.alive(), "| daemon pid file:", fixture_daemon_pid(), "| leftover sleeps:", subprocess.run(["pgrep", "-fc", "sleep 12[0-9]$"], capture_output=True, text=True).stdout.strip())
    t.finish(detach=False); fixture_stop()



def scn_m(runs="3"):
    """Entry cost: `bun run dashboard` (package script -> bootstrap script -> installed native) until the first dashboard frame, versus the native binary directly."""
    bun = shutil.which("bun")
    rows = []
    for label, argv in (("bun run dashboard", [bun, "run", "dashboard"]), ("native binary directly", [binary()])):
        for i in range(int(runs)):
            t = Tty("m-entry-%s-%d" % (label.split()[0], i), argv, 160, 48, REPO)
            ok = t.wait_for("New task", 60)
            first = t.now()
            t.send(CTRL_B); t.pump(0.05); t.send("e"); t.pump(0.3)
            if i == 0: t.snapshot("first-frame", label)
            rows.append((label, round(first, 2), round(os.getloadavg()[0], 1), ok))
            log(label, "run", i + 1, "-> first dashboard frame after %.2fs (load %.1f)" % (first, os.getloadavg()[0]))
            t.finish()
    json.dump(rows, open(OUT + "/m-entry.results.json", "w"), indent=1)



def scn_n():
    """Terminal hygiene: what the view leaves behind on SIGTERM / SIGHUP / SIGINT, and what Ctrl+Z / Ctrl+L do."""
    fixture_prepare()
    out = {}
    for name, sig in (("SIGTERM", signal.SIGTERM), ("SIGHUP", signal.SIGHUP), ("SIGINT", signal.SIGINT)):
        t = fixture_view("n-" + name.lower(), 160, 48)
        t.pump(1.0); n0 = len(t.raw)
        os.kill(t.pid, sig); t.pump(1.5)
        t.reap()
        tail = bytes(t.raw[n0:])
        restored = b"\x1b[?1049l" in tail
        out[name] = {"exited": not t.alive(), "exit_status": t.exit_status, "teardown_sequence_written": restored}
        log(name, out[name])
        t.finish(detach=False)
    t = fixture_view("n-keys", 160, 48)
    t.pump(1.0)
    t.send(b"\x1a"); t.pump(0.5); log("Ctrl+Z alive:", t.alive())
    t.send(b"\x0c"); t.pump(0.5); log("Ctrl+L alive:", t.alive())
    t.send(b"\x1b[200~paste in overview filter\x1b[201~"); t.pump(0.5); t.snapshot("paste-overview"); log("paste into the Overview filter shows:", [l for l in t.vt.text().split("\n") if l.startswith("│/")][:1])
    open_launcher(t); t.send(b"\x1b[200~dev cad react\x1b[201~"); t.pump(0.6); t.snapshot("paste-launcher"); log("paste into the launcher filter shows:", [l.strip()[:40] for l in t.vt.text().split("\n") if l.startswith("│/")][:1])
    t.type("abc", 0.05); t.send(LEFT); t.send(b"X"); t.pump(0.3); t.snapshot("cursor-left-edit"); log("filter after 'abc', Left, 'X':", [l.strip()[:40] for l in t.vt.text().split("\n") if l.startswith("│/")][:1])
    t.send(b"\x15"); t.pump(0.3); log("filter after Ctrl+U:", [l.strip()[:40] for l in t.vt.text().split("\n") if l.startswith("│/")][:1])
    t.finish(); fixture_stop()
    json.dump(out, open(OUT + "/n-signals.results.json", "w"), indent=1)



def selected_row(t):
    """0-based screen row whose cells carry the selection-highlight background (#ff344f), with its text."""
    for y, row in enumerate(t.vt.grid):
        if any(t.vt.styles[st][1] == "#ff344f" and ch.strip() for ch, st in row[:40]):
            return y, "".join(c for c, _ in row).strip("│ ")[:60]
    return None


def scn_o():
    """Overview selection when the selected task's status label changes (selection restored by label text)."""
    fixture_prepare()
    t = fixture_view("o-selection", 160, 48)
    t.pump(1.0)
    start_task(t, "task-short", first=True, settle=0.5)
    leader(t, "h", 0.5); leader(t, "s", 0.8); t.snapshot("00a-overview-after-ctrl-b-s")
    for _ in range(6): t.send(DOWN); t.pump(0.1)
    t.pump(0.3); log("selected before exit:", selected_row(t)); t.snapshot("00-selected-session-row")
    t.pump(5.0); log("selected after the task exited:", selected_row(t)); t.snapshot("01-after-exit")
    t.finish(); fixture_stop()



def scn_p():
    """Live output into a window restored by a second view (hidden tab): is its terminal sized to the pane?"""
    fixture_prepare()
    a = fixture_view("p-first-view", 160, 48)
    a.pump(1.0)
    start_task(a, "dev-ticker", first=True, settle=1.0)
    a.send(ESC); a.pump(0.1); leader(a, "d", 1.0); a.finish(detach=False)
    b = fixture_view("p-second-view", 160, 48)
    b.pump(2.0); b.snapshot("00-after-attach")
    b.send(TAB); b.pump(1.5); b.snapshot("01-tab-to-restored-ticker")
    body = [l.strip("│ ") for l in b.vt.text().split("\n")[5:44] if l.strip("│ ")]
    log("restored ticker window: non-empty body rows:", len(body), "first:", body[:3], "last:", body[-2:])
    sess("daemon-side PTY size of the ticker (cols,rows)")
    b.finish(); fixture_stop()



def scn_q():
    """Esc then Ctrl+B in a terminal pane: does `Esc` leave pane input, and does Ctrl+B d detach? (timing of the lone-Esc flush)"""
    fixture_prepare()
    for gap in (0.02, 0.1, 0.3, 0.8):
        t = fixture_view("q-esc-gap-%d" % int(gap * 1000), 160, 48)
        t.pump(1.0)
        start_task(t, "test-cat", first=True, settle=1.0)
        t.send(ESC); t.pump(gap)
        hint_after_esc = footer(t).strip()[:40]
        t.send(CTRL_B); t.pump(0.1); t.send("d"); t.pump(1.0)
        t.reap()
        t.type  # (no-op)
        log("Esc, wait %.2fs, Ctrl+B d: footer after Esc: %r | view exited: %s" % (gap, hint_after_esc, not t.alive()))
        t.finish(detach=False)
        fixture_stop(); time.sleep(0.3)
        fixture_prepare()



def scn_r():
    """Does a mouse click on another pane move keyboard focus? Task pane (left) + launcher pane (right) after Ctrl+B |, click the task pane, type."""
    fixture_prepare()
    t = fixture_view("r-mouse-focus", 160, 48)
    t.pump(1.0)
    start_task(t, "test-cat", first=True, settle=1.0)
    leader(t, "|", 0.6); t.snapshot("00-split")
    log("footer after split (focus is the new launcher):", footer(t).strip()[:60])
    t.click(10, 20, 0.5); t.snapshot("01-clicked-task-pane")
    log("footer after clicking the task pane:", footer(t).strip()[:60])
    t.type("zz", 0.1); t.pump(0.4); t.snapshot("02-typed-zz")
    left = [l[:80].strip("│ ") for l in t.vt.text().split("\n")[5:12]]
    log("left (task) pane rows:", left)
    right_filter = [l for l in t.vt.text().split("\n") if "/ zz" in l or l.strip("│ ").startswith("/zz")]
    log("a launcher filter shows 'zz':", bool(right_filter))
    t.finish(); fixture_stop()


SCENARIOS = {name[4:]: fn for name, fn in globals().items() if name.startswith("scn_")}

if __name__ == "__main__":
    which = sys.argv[1]
    SCENARIOS[which](*sys.argv[2:])
