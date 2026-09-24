"""Shared helpers: render a vocabulary's 📡️.protocol.semio from its real op frame, and name tag consts."""
import os, re

def const_name(kind):
    return "TAG_" + kind.upper().replace("-", "_")

def protocol_include(rs_path, proto_path):
    rel = os.path.relpath(proto_path, os.path.dirname(rs_path))
    return f'include_str!("{rel}")'

def existing_fields(text):
    """Per-tag and per-kind field lines already declared (repeat arms, records), so rewriting keeps them."""
    by_tag, by_kind = {}, {}
    for m in re.finditer(r"arm\s+(\d+)\s*\{([^}]*)\}", text):
        toks = m.group(2).split()
        by_tag[int(m.group(1))] = [f"field {toks[i]} {toks[i+1]}" for i in range(0, len(toks) - 1, 2)]
    current = None
    for line in text.splitlines():
        r = re.match(r"\s*record\s+([a-z][a-z0-9-]*)\s+tag=\d+", line)
        if r:
            current = r.group(1); by_kind[current] = []; continue
        if current and re.match(r"\s*field\s+\S+\s+\S+", line):
            by_kind[current].append(line.strip()); continue
        if line.strip(): current = None
    return by_tag, by_kind

def render(proto_path, header, records, frame_note):
    text = open(proto_path, encoding="utf-8").read()
    def directive(name, default):
        m = re.search(r"(?m)^" + name + r"\s+(\S+)", text)
        return m.group(1) if m else default
    pid = directive("protocol", None)
    by_tag, by_kind = existing_fields(text)
    out = ["dialect protocol", f"protocol {pid}", f"version {directive('version', '1')}", f"schema {directive('schema', pid)}", f"start {directive('start', 'op')}", "framing record", ""]
    out += ["# " + line for line in frame_note]
    out += [f"header fixed {len(header) if all(t == 'u8' for _, t in header) else len(header)}"]
    out += [f"field {n} {t}" for n, t in header]
    for kind, tag in records:
        out.append(f"record {kind} tag={tag}")
        out += by_kind.get(kind) or by_tag.get(tag) or ["field payload bytes"]
    new = "\n".join(out) + "\n"
    open(proto_path, "w", encoding="utf-8").write(new)
    return new
