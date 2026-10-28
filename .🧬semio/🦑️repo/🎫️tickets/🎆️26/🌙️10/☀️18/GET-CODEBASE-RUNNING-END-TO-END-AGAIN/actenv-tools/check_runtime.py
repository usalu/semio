"""🔎 Prints the NAMES (never values) of the newest published runtime channel `runtime.json` below the products cache and exits 1 when a credential-like or desktop/agent/session name is present. usage: check_runtime.py [ROOT]"""
import glob, json, os, re, sys
here = os.path.dirname(os.path.abspath(__file__)); root = sys.argv[1] if len(sys.argv) > 1 else here
while not os.path.isdir(os.path.join(root, "🧰️framework")) and root != "/": root = os.path.dirname(root)
files = sorted(glob.glob(os.path.join(root, ".cache/b/p/m/bazel/workspaces/*/*/runtime.json")), key=os.path.getmtime)
if not files: sys.exit("no runtime.json")
names = sorted(json.load(open(files[-1], encoding="utf8"))["environment"])
bad = re.compile(r"(^|_)(TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIALS?|AUTH|COOKIES?)(_|$)|_KEYS?$|^(CLAUDE|ANTHROPIC|XPC_|__CF|NEO4J_|MCP_|TERM_SESSION_ID|SECURITYSESSIONID|LAUNCHINSTANCEID|SSH_|DISPLAY|WAYLAND_DISPLAY|DBUS_|AI_AGENT)", re.I)
leaks = [n for n in names if bad.search(n)]
print(os.path.relpath(files[-1], root), len(names), "names")
print(" ".join(names))
print("LEAKS:", " ".join(leaks) if leaks else "none")
sys.exit(1 if leaks else 0)
