import re, sys, hashlib, time

PATH = r"C:\git\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧪️tests\🔬️unit\🦀️.rs"
START = "async fn store_close_releases_a_returned_read_before_its_displaced_root"
END = "async fn space_checkpoint_commits_dirty_members_and_pins_their_checkpoints"
RECV = re.compile(r"([A-Za-z_][A-Za-z_0-9]*)\.dispatch\(")


def match_close(text, open_index):
    depth = 0
    i = open_index
    while i < len(text):
        c = text[i]
        if c == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced")


def rewrite(segment):
    out = []
    pos = 0
    count = 0
    while True:
        m = RECV.search(segment, pos)
        if not m:
            out.append(segment[pos:])
            break
        open_index = m.end() - 1
        close = match_close(segment, open_index)
        args = segment[open_index + 1 : close]
        out.append(segment[pos : m.start()])
        out.append(f"test_support::dispatch_test_command(&mut {m.group(1)}, {args})")
        pos = close + 1
        count += 1
    return "".join(out), count


for attempt in range(5):
    with open(PATH, encoding="utf-8", newline="") as f:
        text = f.read()
    digest = hashlib.sha256(text.encode()).hexdigest()
    s = text.index(START)
    e = text.index(END)
    segment, count = rewrite(text[s:e])
    new = text[:s] + segment + text[e:]
    with open(PATH, encoding="utf-8", newline="") as f:
        again = f.read()
    if hashlib.sha256(again.encode()).hexdigest() != digest:
        time.sleep(0.2)
        continue
    with open(PATH, "w", encoding="utf-8", newline="") as f:
        f.write(new)
    print("rewrote", count)
    sys.exit(0)
print("file kept changing")
sys.exit(1)
