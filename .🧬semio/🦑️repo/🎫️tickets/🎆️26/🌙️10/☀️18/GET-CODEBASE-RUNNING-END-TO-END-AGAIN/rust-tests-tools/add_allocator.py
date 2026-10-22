import os, re, sys
ROOT = "/Users/ueli/Documents/semio/"
TARGET = ROOT + "🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust"
STATIC = '\n#[cfg(test)]\n#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;\n'
def run(cargo_rel, lock_rels=("🧰️framework/Cargo.lock","Cargo.lock"), lib_file=None):
    cargo = ROOT + cargo_rel
    text = open(cargo).read()
    name = re.search(r'^name = "([^"]+)"', text, re.M).group(1)
    base = os.path.dirname(cargo)
    if lib_file is None:
        m = re.search(r'^\[lib\][^\[]*?path = "([^"]+)"', text, re.M | re.S)
        lib_file = os.path.normpath(os.path.join(base, m.group(1))) if m else os.path.join(base, "src/lib.rs")
    src = open(lib_file).read()
    if "RequestedAllocator" not in src:
        open(lib_file, "w").write(src.rstrip("\n") + "\n" + STATIC)
    if "semio-framework-trace-memory-testing" not in text:
        rel = os.path.relpath(TARGET, base)
        line = 'semio-framework-trace-memory-testing = { path = "%s" }\n' % rel
        if "[dev-dependencies]\n" in text:
            text = text.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + line, 1)
        else:
            text = text.rstrip("\n") + "\n\n[dev-dependencies]\n" + line
        open(cargo, "w").write(text)
        for lock_rel in lock_rels:
            lock = open(ROOT + lock_rel).read()
            m = re.search(r'\[\[package\]\]\nname = "%s"\nversion = "[^"]*"\n(dependencies = \[\n(.*?)\n\]\n)?' % re.escape(name), lock, re.S)
            if not m: continue
            if m.group(1):
                deps = [x.strip().strip(",").strip('"') for x in m.group(2).split("\n")]
                if "semio-framework-trace-memory-testing" not in deps: deps.append("semio-framework-trace-memory-testing")
                deps.sort()
                lock = lock[:m.start(2)] + "\n".join(' "%s",' % d for d in deps) + lock[m.end(2):]
            else:
                lock = lock[:m.end()] + 'dependencies = [\n "semio-framework-trace-memory-testing",\n]\n' + lock[m.end():]
            open(ROOT + lock_rel, "w").write(lock)
    print("ok", name, lib_file.replace(ROOT, ""))
if __name__ == "__main__":
    run(sys.argv[1], lib_file=(ROOT + sys.argv[2]) if len(sys.argv) > 2 else None)
