"""⚙️ Shared engine of the staged patches: copies staged test sets, overwrites the author's own earlier test files, applies anchored production edits and registers tests in the lanes that run them. All checks run before anything is written; a drifted anchor or a differing destination refuses the whole patch. Dry run unless `apply` is true."""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
FE = "️"
LIB = os.environ.get("LAWCOV_LIBRARY") or os.path.join(ROOT, "🧰" + FE + "framework", "🛍" + FE + "products", "🦑" + FE + "repo", "🔨" + FE + "modules", "📚" + FE + "library")


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def exact(rel):
    cur = LIB
    for part in rel.split("/"):
        found = None
        if os.path.isdir(cur):
            for entry in os.listdir(cur):
                if entry.replace(FE, "") == part.replace(FE, ""):
                    found = entry
                    break
        cur = os.path.join(cur, found if found else part)
    return os.path.relpath(cur, LIB).replace(os.sep, "/")


class Patch:
    def __init__(self, name, apply):
        self.name, self.apply, self.writes, self.problems, self.log = name, apply, [], [], []

    def say(self, status, what):
        self.log.append("%-8s %s" % (status, what))

    def stage_write(self, path, text, label, status="write"):
        self.writes.append((path, text))
        self.say(status if self.apply else "would", label)

    def copy_sets(self, group, overwrite, stage="base"):
        manifest = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
        for slug, row in manifest.items():
            if row.get("group", "main") != group or bool(row.get("overwrite")) != overwrite or row.get("stage", "base") != stage:
                continue
            for name, dest_rel in row["files"].items():
                text = open(os.path.join(HERE, slug, name), encoding="utf8").read()
                dest = os.path.join(LIB, dest_rel)
                if os.path.exists(dest):
                    current = open(dest, encoding="utf8").read()
                    if current == text:
                        self.say("same", dest_rel)
                    elif overwrite and sha(current) == row["expect"].get(name):
                        self.stage_write(dest, text, dest_rel, "replace")
                    else:
                        self.problems.append("destination differs: " + dest_rel)
                        self.say("REFUSED", dest_rel + " (exists and differs)")
                else:
                    self.stage_write(dest, text, dest_rel)

    def edits(self, group):
        pending = {}
        for row in json.load(open(os.path.join(HERE, "fixes", group + ".json"), encoding="utf8")):
            path = os.path.join(LIB, row["file"])
            text = pending.get(path) if path in pending else open(path, encoding="utf8").read()
            label = os.path.relpath(path, LIB)
            if row["marker"] in text:
                self.say("same", label + " (already edited)")
            elif text.count(row["old"]) != 1:
                self.problems.append("anchor drift: " + label)
                self.say("REFUSED", label + " (anchor found %d times)" % text.count(row["old"]))
            else:
                pending[path] = text.replace(row["old"], row["new"])
                self.say("edit" if self.apply else "would", label)
        for path, text in pending.items():
            self.writes.append((path, text))

    def require_edited(self, group):
        for row in json.load(open(os.path.join(HERE, "fixes", group + ".json"), encoding="utf8")):
            text = open(os.path.join(LIB, row["file"]), encoding="utf8").read()
            if row["marker"] not in text:
                self.problems.append("production edit missing: " + row["file"])
                self.say("REFUSED", "%s lacks %s (apply cache-gc apply3 and apply4 first)" % (os.path.relpath(os.path.join(LIB, row["file"]), LIB), row["marker"][:40]))

    def register(self, group):
        reg = json.load(open(os.path.join(HERE, "registry.json"), encoding="utf8"))
        build = [t["path"] for t in reg["tests"] if t["group"] == group and t["path"].replace(FE, "").startswith("🏗build/")]
        caching = [t["path"] for t in reg["tests"] if t["group"] == group and t["path"].replace(FE, "").startswith("⚡caching/")]
        if build:
            script = os.path.join(LIB, "🏗" + FE + "build", "📜" + FE + "script.ts")
            text = open(script, encoding="utf8").read()
            tail = ',"../⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts")'
            names = [n for n in ("dir", "dirname") if text.count("resolve(import.meta." + n + tail) == 1]
            name = names[0] if len(names) == 1 else "dir"
            anchor = "resolve(import.meta." + name + tail
            todo = [t for t in build if 'resolve(import.meta.%s,"%s")' % (name, t.split("/", 1)[1]) not in text]
            if not todo:
                self.say("same", "🏗build/📜script.ts (already registered)")
            elif text.count(anchor) != 1:
                self.problems.append("anchor drift: 🏗build/📜script.ts")
                self.say("REFUSED", "🏗build/📜script.ts")
            else:
                extra = "".join(',resolve(import.meta.%s,"%s")' % (name, t.split("/", 1)[1]) for t in todo)
                self.writes.append((script, text.replace(anchor, anchor + extra)))
                self.say("edit" if self.apply else "would", "🏗build/📜script.ts (%d tests)" % len(todo))
        if caching:
            self.register_caching(group, caching)

    def register_caching(self, group, caching):
        lane = "law-coverage" if group == "main" else "law-coverage-" + group
        commands = os.path.join(LIB, "⚡" + FE + "caching", "🧪" + FE + "tests", "🔬" + FE + "script-commands", "🟦" + FE + ".ts")
        anchor = "    await testCacheContracts(this.invocation,phaseControl);\n  }\n}"
        text = open(commands, encoding="utf8").read()
        files = ",".join('"%s"' % t.split("/", 1)[1] for t in caching)
        block = ('    if (args[0] === "%s") {\n      if (args.length !== 1) throw new Error("Expected test %s");\n'
                 '      await this.runBunTests(["test", ...[%s].map(path => join(SCRIPT_ROOT, path))],orchestratorBudgetOpts(environment).budgetMs!,environment,phaseControl);\n      return;\n    }\n' % (lane, lane, files))
        if 'args[0] === "%s"' % lane in text:
            self.say("same", "⚡caching script-commands (lane exists)")
        elif text.count(anchor) != 1:
            self.problems.append("anchor drift: ⚡caching script-commands")
            self.say("REFUSED", "⚡caching script-commands")
        else:
            self.writes.append((commands, text.replace(anchor, block + anchor)))
            self.say("edit" if self.apply else "would", "⚡caching script-commands (lane %s)" % lane)

        def unit_of(rel):
            parts = rel.split("/", 1)[1].split("/")
            return "/".join(parts[:[p.replace(FE, "") for p in parts].index("🧪tests")])

        project = os.path.join(LIB, "⚡" + FE + "caching", "📋" + FE + "project.json")
        ptext = open(project, encoding="utf8").read()
        panchor = '    "test-cache-command-source": {'
        name = "test-" + lane
        inputs = sorted({"{projectRoot}/" + unit_of(t) + "/**/*" for t in caching})
        target = ('    "%s": {\n      "executor": "repo:script",\n      "cache": true,\n      "inputs": [\n        "{projectRoot}/📜' % name + FE + 'script.ts",\n' + ",\n".join('        "%s"' % i for i in inputs) +
                  '\n      ],\n      "outputs": [],\n      "options": {\n        "command": "bun ./📜' + FE + 'script.ts test %s"\n      }\n    },\n' % lane)
        if '"%s"' % name in ptext:
            self.say("same", "⚡caching/📋project.json (target exists)")
        elif ptext.count(panchor) != 1:
            self.problems.append("anchor drift: ⚡caching/📋project.json")
            self.say("REFUSED", "⚡caching/📋project.json")
        else:
            self.writes.append((project, ptext.replace(panchor, target + panchor)))
            self.say("edit" if self.apply else "would", "⚡caching/📋project.json (target %s)" % name)

    def finish(self):
        print("\n".join(self.log))
        if self.problems:
            print("REFUSED:", "; ".join(self.problems))
            sys.exit(2)
        if self.apply:
            for path, text in self.writes:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w", encoding="utf8").write(text)
            print(self.name + ": applied %d files" % len(self.writes))
        else:
            print(self.name + ": dry run, nothing written (pass --apply)")
