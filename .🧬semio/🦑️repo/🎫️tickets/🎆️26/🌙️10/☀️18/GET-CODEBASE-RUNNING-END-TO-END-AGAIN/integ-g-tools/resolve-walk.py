"""Graph-free import resolution guard: streams every .ts/.tsx/.mjs/.js file under the given roots and reports relative import specifiers (`from "./x"`, `import "./x"`, `import("./x")`) whose target does not exist. Aborts above 500 MB RSS. usage: resolve-walk.py <root>..."""
import os, re, sys, resource
pattern = re.compile(r'(?:from\s+|import\s*\(\s*|import\s+)["\'](\.{1,2}/[^"\']+)["\']')
skip = {"node_modules", ".git", "target", "dist", ".cache", "🗑️generated", "build", ".venv", "🧪️tests", "🧫️fixtures"}
files = missing = 0
for root in sys.argv[1:]:
    for directory, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d not in skip and not d.startswith(".")]
        for name in names:
            if not name.endswith((".ts", ".tsx", ".mjs", ".js")) or name.endswith(".d.ts"): continue
            path = os.path.join(directory, name)
            try:
                if os.path.getsize(path) > 600000: continue
                files += 1
                for number, line in enumerate(open(path, encoding="utf8", errors="replace"), 1):
                    if "import" not in line and "from" not in line: continue
                    if line.lstrip().startswith(("*", "//", "/*")): continue
                    for match in pattern.finditer(line):
                        spec = match.group(1).split("?")[0]
                        if "\\u" in spec or "<" in spec or "${" in spec: continue
                        target = os.path.normpath(os.path.join(directory, spec))
                        if not any(os.path.exists(target + suffix) for suffix in ("", ".ts", ".tsx", ".mjs", ".js", ".json", "/index.ts", "/index.js")) and not (target.endswith(".js") and os.path.exists(target[:-3] + ".ts")):
                            missing += 1; print(f"MISSING {path}:{number} -> {match.group(1)}")
            except OSError: pass
        if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1024 * 1024: print("RSS limit"); sys.exit(2)
print("files", files, "missing", missing, "maxrss_mb", resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // 1048576)
sys.exit(1 if missing else 0)
