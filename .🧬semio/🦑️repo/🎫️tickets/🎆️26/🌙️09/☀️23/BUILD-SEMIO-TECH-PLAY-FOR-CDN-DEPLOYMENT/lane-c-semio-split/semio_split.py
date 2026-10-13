"""🧰️ Shared helpers for the Lane C stdio-semio split codemods (paths, subset table, file walks, guarded edits)."""
import os, re, subprocess

REPO = "/Users/ueli/Documents/semio"
SEMIO = REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
SUBSETS_REL = "🏅️standards/🔖️v1/🪆️subsets"
SUBSETS = SEMIO + "/" + SUBSETS_REL
ROOT_RS = SEMIO + "/🦀️.rs"
SKIP_DIRS = ("🏭️generator", "🔬️probes", "🏭️bridge", "🔮️oracles", "/target/")
NAMES = ["animation", "audio", "brep", "cad", "document", "drawing", "flow", "graph", "image", "kit", "mesh", "model", "object", "presentation", "table", "text", "value", "video"]
EMOJI = {re.sub(r"^[^a-z]+", "", d): d for d in os.listdir(SUBSETS) if os.path.isdir(os.path.join(SUBSETS, d))}
CONVERSION_SUBSETS = ["animation", "audio", "brep", "cad", "document", "drawing", "flow", "image", "mesh", "model", "presentation", "value", "video"]


def semio_rust_files(include_nested=False):
    """📂️ Every `.rs` under the semio artifact, without generator/probe/bridge/oracle crates."""
    for root, dirs, files in os.walk(SEMIO):
        if not include_nested and any(s in root + "/" for s in SKIP_DIRS):
            continue
        for f in files:
            if f.endswith(".rs"):
                yield os.path.join(root, f)


def consumer_rust_files():
    """📂️ Every `.rs` outside the semio artifact that names the semio crate."""
    out = subprocess.run(["rg", "-l", "--no-messages", "semio_s_artifact_stdio_semio", "-g", "*.rs", "-g", "!.🧬semio/**", "-g", "!**/target/**", REPO], capture_output=True, text=True).stdout.split()
    return [p for p in out if not p.startswith(SEMIO + "/")]


def read(p):
    with open(p) as f:
        return f.read()


def write(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w") as f:
        f.write(s)


def subst_files(files, pattern, repl, flags=0):
    """🔁️ Regex substitution over files; returns {path: count} of changed files."""
    rx = re.compile(pattern, flags)
    changed = {}
    for p in files:
        s = read(p)
        n_s, n = rx.subn(repl, s)
        if n and n_s != s:
            write(p, n_s)
            changed[p] = n
    return changed


def block_end(lines, start):
    """🧱️ Index of the line closing the brace block opened on or after `start`."""
    depth = 0
    opened = False
    for i in range(start, len(lines)):
        code = re.sub(r'"(?:\\.|[^"\\])*"', '""', lines[i].split("//")[0])
        for c in code:
            if c == "{":
                depth += 1
                opened = True
            elif c == "}":
                depth -= 1
        if opened and depth == 0:
            return i
    raise ValueError("unbalanced block from line %d" % start)


def fn_span(src, name, start=0):
    """✂️ (start, end) char span of `fn name` with its leading doc/attribute/comment lines."""
    m = re.compile(r"(?m)^[ \t]*(?:pub(?:\([a-z]+\))? )?(?:const )?(?:async )?fn " + re.escape(name) + r"\b").search(src, start)
    if not m:
        return None
    s = m.start()
    lines_before = src[:s].split("\n")
    k = len(lines_before) - 1
    while k > 0 and re.match(r"^\s*(///|//|#\[)", lines_before[k - 1]):
        k -= 1
    s = sum(len(l) + 1 for l in lines_before[:k])
    i = src.index("{", m.end())
    depth = 0
    for j in range(i, len(src)):
        if src[j] == "{":
            depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0:
                e = j + 1
                if src[e:e + 1] == "\n":
                    e += 1
                return s, e
    raise ValueError(name)


def extract_functions(src, names):
    """✂️ Removes `fn <name>` items (with leading comments/attributes) from `src`; returns (src, [texts], first_offset)."""
    spans = []
    for n in names:
        sp = fn_span(src, n)
        if sp is None:
            raise ValueError("missing fn " + n)
        spans.append((sp, n))
    spans.sort()
    texts = [src[a:b] for (a, b), _ in spans]
    first = spans[0][0][0]
    for (a, b), _ in reversed(spans):
        src = src[:a] + src[b:]
    return src, texts, first


def used_names(src, names):
    """🔎️ The subset of `names` referenced in `src` outside their own definitions."""
    from use_rewrite import _mask
    masked = _mask(src)
    out = []
    for n in names:
        refs = [m for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(n) + r"(?![A-Za-z0-9_])", masked)]
        defs = [m for m in re.finditer(r"\bfn\s+" + re.escape(n) + r"\b", masked)]
        if len(refs) > len(defs):
            out.append(n)
    return out


def module_path_of_import(seg_prefix):
    return tuple(seg_prefix)


def head_text(p):
    """📜️ The committed (HEAD) content of a repository file, read-only."""
    return subprocess.run(["git", "-C", REPO, "show", "HEAD:" + os.path.relpath(p, REPO)], capture_output=True, text=True, check=True).stdout


def squeeze_blank_runs(paths):
    """🧹️ Collapses runs of three or more blank lines a removal left behind into one blank line, comparing against HEAD so
    pre-existing runs stay untouched."""
    for p in paths:
        s = read(p)
        try:
            h = head_text(p)
        except Exception:
            h = ""
        if h.count("\n\n\n\n") >= s.count("\n\n\n\n"):
            continue
        write(p, re.sub(r"\n{4,}", "\n\n", s))


def toml_loads(text):
    """🧾️ Parses TOML through Bun's parser (the system Python predates `tomllib`)."""
    import json as _json
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.stdin.text())))"], input=text, capture_output=True, text=True, check=True).stdout
    return _json.loads(out)
