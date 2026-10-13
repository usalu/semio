"""⏱️ Reads one cargo `--timings` HTML report: wall, unit-time sum, family sums and the critical path (unblock edges)."""
import json, re, sys

FAMILIES = {
    "wasmtime/cranelift": re.compile(r"^(wasmtime|cranelift|wasmparser|wasm-encoder|wit-|pulley|regalloc|gimli|object|cap-|wasi|wat|wast|winch|target-lexicon|wasm-wave|wasmprinter|ittapi|io-extras|fs-set-times|system-interface|rustix)"),
    "semio-framework-ui*": re.compile(r"^semio-framework-ui"),
    "tokio": re.compile(r"^tokio"),
}

def main(path):
    units = json.loads(re.search(r"const UNIT_DATA = (\[.*?\]);\n", open(path).read(), re.S).group(1))
    by = {u["i"]: u for u in units}
    end = {u["i"]: u["start"] + u["duration"] for u in units}
    parent = {}
    for u in units:
        for child in u["unblocked_units"] + u["unblocked_rmeta_units"]:
            if child in by and (child not in parent or end[u["i"]] > end[parent[child]]):
                parent[child] = u["i"]
    wall = max(end.values())
    print(f"units={len(units)} wall={wall:.1f}s unit_sum={sum(u['duration'] for u in units):.0f}s")
    for name, pattern in FAMILIES.items():
        members = [u for u in units if pattern.match(u["name"])]
        print(f"family {name}: units={len(members)} sum={sum(u['duration'] for u in members):.0f}s")
    top = sorted(units, key=lambda u: -u["duration"])[:15]
    print("top units:", ", ".join(f"{u['name']}{u['target'].strip() and '('+u['target'].strip()+')'} {u['duration']:.0f}s" for u in top))
    node = max(end, key=end.get)
    chain = []
    while node is not None:
        chain.append(by[node])
        node = parent.get(node)
    print("critical path (start s, duration s):")
    for u in reversed(chain):
        print(f"  {u['start']:7.1f} {u['duration']:7.1f}  {u['name']} {u['target'].strip()}")

if __name__ == "__main__":
    main(sys.argv[1])
