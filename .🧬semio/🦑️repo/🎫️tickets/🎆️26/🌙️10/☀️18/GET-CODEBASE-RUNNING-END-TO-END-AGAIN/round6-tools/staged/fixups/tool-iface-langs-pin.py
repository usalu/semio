#!/usr/bin/env python3
"""Re-bases the tool-iface copy on langs-pin's live `repo tools verify [tool...]` (pins of the tool-setup table, `tools setup [nx|bazel|toolchains|all]`): the registry keeps `tools list` and `tools exec`, its own `verify` (and the three messages only it used) is dropped, so one `verify` exists. The catalogue block of `tools` is taken from the checkout, not from the staged text.
usage: tool-iface-langs-pin.py <tool-iface copy> <checkout root>"""
import os, re, sys

T, ROOT = sys.argv[1], sys.argv[2]
R = "\U0001F9F0️framework"
L = R + "/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library"
B = L + "/\U0001F3D7️build"
CAT = os.path.join(ROOT, B, "⚙️operations", "\U0001F523️catalog.json")
E = os.path.join(T, "build", "edits.py")
text = open(E, encoding="utf8").read()
cat = open(CAT, encoding="utf8").read()
start = cat.index('      "name": "tools",')
end = cat.index("\n    },", start) + len("\n    },")
cur = cat[start:end]

new_old = re.search(r"TOOLS_NEW = '''(.*?)'''", text, re.S).group(1)
list_block = new_old[new_old.index('        {\n          "name": "list",'):new_old.index('        {\n          "name": "verify",')]
exec_block = new_old[new_old.index('        {\n          "name": "exec",'):new_old.index("\n        }\n      ],", new_old.index('"name": "exec"')) + len("\n        }")]
example_lines = [l for l in new_old.split("\n") if l.lstrip().startswith('{ "command": "repo tools list"') or l.lstrip().startswith('{ "command": "repo tools exec cargo')]
notes = new_old[new_old.index('      "notes": ['):new_old.rindex("\n    },")]
assert list_block and exec_block and len(example_lines) == 2 and notes

mod = cur
mod = mod.replace('"synopsis": "tools setup [nx|bazel|toolchains|all]|verify [tool...]",', '"synopsis": "tools setup [nx|bazel|toolchains|all]|verify [tool...]|list [--json]|exec <tool> -- <arguments>",', 1)
mod = mod.replace("Provision the pinned tools for this host and report where it differs from the pins.", "Provision the pinned tools for this host, report where it differs from the pins and run every other external tool through the tool registry.", 1)
mod = mod.replace("Richtet die gepinnten Werkzeuge für diesen Rechner ein und meldet, wo er von den Pins abweicht.", "Richtet die gepinnten Werkzeuge für diesen Rechner ein, meldet, wo er von den Pins abweicht, und führt jedes andere externe Werkzeug über die Werkzeugregistratur aus.", 1)
tail = "\n        }\n      ],\n      \"flags\": [],"
assert mod.count(tail) == 1, "tools block: subcommand tail not found"
mod = mod.replace(tail, "\n        },\n" + list_block + exec_block + "\n      ],\n      \"flags\": [\"--json\", \"--cwd\", \"--light\"],", 1)
ex_tail = "\n      ]\n    },"
assert mod.endswith(ex_tail), "tools block: examples tail"
mod = mod[: -len(ex_tail)] + ",\n" + ",\n".join(l.rstrip(",") for l in example_lines) + "\n      ],\n" + notes + "\n    },"

text = re.sub(r"TOOLS_OLD = '''.*?'''", lambda m: "TOOLS_OLD = '''" + cur + "'''", text, count=1, flags=re.S)
text = re.sub(r"TOOLS_NEW = '''.*?'''", lambda m: "TOOLS_NEW = '''" + mod + "'''", text, count=1, flags=re.S)
only_block = re.search(r"\n    \{\n      \"name\": \"--only\",.*?\n    \},", text, re.S)
assert only_block
text = text.replace(only_block.group(0), "", 1)
assert text.count('["exec","list","verify"]') == 1
text = text.replace('["exec","list","verify"]', '["exec","list"]')
open(E, "w", encoding="utf8").write(text)

CMD = os.path.join(T, "src", "files", B, "\U0001F9F0️tool-registry", "⌨️command", "\U0001F7E6️.ts")
code = open(CMD, encoding="utf8").read()
a = code.index("/** \U0001FA7A️ Resolves every registered tool")
b = code.index("/** ▶️ Runs a registered tool")
code = code[:a] + code[b:]
code = code.replace('  if (args[0] === "verify") return verify(head, context, root);\n', "")
code = code.replace("`repo tools list [--json]`, `repo tools verify [--only=<ids>]` and `repo tools exec", "`repo tools list [--json]` and `repo tools exec")
code = code.replace("probeToolV1, ", "", 1)
open(CMD, "w", encoding="utf8").write(code)

REG = os.path.join(T, "build", "registry.py")
reg = open(REG, encoding="utf8").read()
reg = "\n".join(l for l in reg.split("\n") if not l.lstrip().startswith(('"versionMismatch"', '"verifyOk"', '"verifyProblem"')))
open(REG, "w", encoding="utf8").write(reg)
DOCKER_OLD = 'B + "/\U0001F3C3\ufe0fexecution/\U0001F433\ufe0fservice/\U0001F7E6\ufe0f.ts", [B + "/\U0001F3C3\ufe0fexecution/\U0001F433\ufe0fservice/**", L + "/\u26A1\ufe0fcaching/\U0001F4E6\ufe0fartifacts/\U0001F433\ufe0fcontainers/**"]'
DOCKER_NEW = 'L + "/\u26A1\ufe0fcaching/\U0001F4E6\ufe0fartifacts/\U0001F433\ufe0fcontainers/\U0001F501\ufe0flifecycle/\U0001F7E6\ufe0f.ts", [L + "/\u26A1\ufe0fcaching/\U0001F4E6\ufe0fartifacts/\U0001F433\ufe0fcontainers/**"]'
reg = open(REG, encoding="utf8").read()
assert reg.count(DOCKER_OLD) == 1, "docker adapter row"
open(REG, "w", encoding="utf8").write(reg.replace(DOCKER_OLD, DOCKER_NEW))
P = "\U0001F9F0️framework/\U0001F528️modules/\U0001F3C3️process/\U0001F9F0️tools"
SCH = os.path.join(T, "src", "files", P, "\U0001F9EC️schema", "\U0001F523️.json")
sch = open(SCH, encoding="utf8").read()
sch = sch.replace('"versionMismatch", ', "").replace('"verifyOk", "verifyProblem", ', "")
sch = "\n".join(l for l in sch.split("\n") if not l.lstrip().startswith(('"versionMismatch": {', '"verifyOk": {', '"verifyProblem": {')))
open(SCH, "w", encoding="utf8").write(sch)
TYP = os.path.join(T, "src", "files", P, "\U0001F4D6️registry", "\U0001F7E6️.ts")
typ = open(TYP, encoding="utf8").read()
typ = typ.replace(' | "versionMismatch"', "").replace(' | "verifyOk" | "verifyProblem"', "")
open(TYP, "w", encoding="utf8").write(typ)
print("tool-iface-langs-pin: tools block from the checkout, verify dropped")
