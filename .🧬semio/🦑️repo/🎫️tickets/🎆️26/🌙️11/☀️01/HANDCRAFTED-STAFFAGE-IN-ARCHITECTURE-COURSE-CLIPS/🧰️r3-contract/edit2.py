import sys
def edit(p, pairs):
    s = open(p, encoding="utf-8", newline="").read()
    crlf = "\r\n" in s
    s = s.replace("\r\n", "\n")
    for a, b in pairs:
        assert s.count(a) == 1, (p, a, s.count(a))
        s = s.replace(a, b)
    if crlf: s = s.replace("\n", "\r\n")
    open(p, "w", encoding="utf-8", newline="").write(s)
d = sys.argv[1]
edit(d + "/🧬️schema/🔣️.schema.json", [(
'''        { "if": { "properties": { "transportAgeS": {} }, "required": ["transportAgeS"] }, "then": { "properties": { "clip": {} }, "required": ["clip"] } },''',
'''        { "if": { "properties": { "transportAgeS": {} }, "required": ["transportAgeS"] }, "then": { "properties": { "clip": {} }, "required": ["clip"] } },
        { "if": { "properties": { "transport": {} }, "required": ["transport"] }, "then": { "properties": { "view": { "const": "thermal" }, "transportAgeS": {} }, "required": ["transportAgeS"] } },''')])
edit(d + "/🧬️schema/🟦️.ts", [(
'''  if ("transportAgeS" in cast && !("clip" in cast)) issues.push(`${path}: transportAgeS needs a clip`);''',
'''  if ("transportAgeS" in cast && !("clip" in cast)) issues.push(`${path}: transportAgeS needs a clip`);
  if ("transport" in cast && !("transportAgeS" in cast)) issues.push(`${path}: transport layers need transportAgeS, the explicit start of the field`);
  if ("transport" in cast && cast.view !== "thermal") issues.push(`${path}: transport layers exist for the thermal view only`);''')])
edit(d + "/🟦️.ts", [(
"options.environmentTimeline ?? options.environment ?? state.asset.thermal.environment,",
"options.environmentTimeline ?? options.environment!,")])
