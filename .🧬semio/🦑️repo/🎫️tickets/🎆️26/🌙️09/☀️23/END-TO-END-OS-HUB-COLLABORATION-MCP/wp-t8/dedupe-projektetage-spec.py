import re, pathlib
p = pathlib.Path("/Users/ueli/Documents/semio/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts")
s = p.read_text()
start = s.index("//#region 🔖️spec\n")
end = s.index("//#endregion 🔖️spec\n") + len("//#endregion 🔖️spec\n")
region = s[start:end]
spec = pathlib.Path(p.parent, "🔖️spec.ts").read_text()
exported = set(re.findall(r"^export (?:const|function|interface|type) (\w+)", spec, re.M))
rest = s[:start] + s[end:]
used = sorted(n for n in exported if re.search(r"\b%s\b" % n, rest))
block = "//#region 🔖️spec\nimport { " + ", ".join(used) + ' } from "./🔖️spec.ts";\nexport * from "./🔖️spec.ts";\n//#endregion 🔖️spec\n'
p.write_text(s[:start] + block + s[end:])
print(len(region.splitlines()), "lines replaced; imported:", used)
