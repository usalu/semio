#!/usr/bin/env python3
"""Edits the world witness in a work root: `edit_witness.py <work-root>`. Idempotent; refuses when an anchor is missing."""
import json, os, sys

R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness"
root = os.path.abspath(sys.argv[1])

def edit(rel, pairs):
    path = os.path.join(root, R, rel)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if new in text:
            continue
        if text.count(old) != 1:
            sys.exit(f"anchor not unique/missing in {rel}: {old[:90]!r} ({text.count(old)})")
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)

AMBIENT = {"prefixes": ["CLAUDE", "ANTHROPIC", "__CF", "XPC_"], "names": ["TERM_SESSION_ID", "SECURITYSESSIONID", "LaunchInstanceID"]}

schema_path = os.path.join(root, R, "🧬️schema/🔣️.json")
text = open(schema_path, encoding="utf8").read()
tail = '"SEMIO_BUILD_GRAPH_YIELD_ROUND"]'
if '"SEMIO_AGENT_SESSION"' not in text:
    assert text.count(tail) == 1
    text = text.replace(tail, '"SEMIO_BUILD_GRAPH_YIELD_ROUND", "SEMIO_AGENT_SESSION", "AI_AGENT"]')
if '"ambientEnvironment"' not in text:
    anchor = '        "ignoredEnvironment": ['
    assert text.count(anchor) == 1
    line = '        "ambientEnvironment": ' + json.dumps(AMBIENT, ensure_ascii=False) + ',\n'
    text = text.replace(anchor, line + anchor)
    old = '"authorityEnvironment", "ignoredEnvironment"],'
    assert text.count(old) == 1
    text = text.replace(old, '"authorityEnvironment", "ignoredEnvironment", "ambientEnvironment"],')
    old = '        "ignoredEnvironment": { "type": "array", "items": { "type": "string" } }\n'
    assert text.count(old) == 1
    text = text.replace(old, '        "ignoredEnvironment": { "type": "array", "items": { "type": "string" } },\n        "ambientEnvironment": { "type": "object", "additionalProperties": false, "required": ["prefixes", "names"], "properties": { "prefixes": { "type": "array", "items": { "type": "string", "minLength": 1 } }, "names": { "type": "array", "items": { "type": "string", "minLength": 1 } } } }\n')
    open(schema_path, "w", encoding="utf8").write(text)

edit("🟨️.mjs", [
    ('const policy = schema.definitions.Policy.const;\n',
     'const policy = schema.definitions.Policy.const;\n\n'
     '/** 🌫️ Tells whether a name is an ambient host, agent or session variable: the terminal, the desktop session and the agent that started the build are not inputs of any plan or action, whatever their values are, so they leave no observation and never break a witness. */\n'
     'export function isAmbientEnvironmentNameV1(name) { return policy.ambientEnvironment.names.includes(name) || policy.ambientEnvironment.prefixes.some(prefix => name.startsWith(prefix)); }\n'),
    ('const environmentWrites = new Set(), authority = new Set(config.authorityEnvironment), ignoredForDigest = new Set([...config.authorityEnvironment, ...config.ignoredEnvironment]);\n',
     'const environmentWrites = new Set(), authority = new Set(config.authorityEnvironment), ignoredNames = new Set([...config.authorityEnvironment, ...config.ignoredEnvironment]), ambientNames = new Set(config.ambientEnvironment.names);\n'
     ' const ignoredForDigest = { has: name => ignoredNames.has(name) || ambientNames.has(name) || config.ambientEnvironment.prefixes.some(prefix => name.startsWith(prefix)) };\n'),
    ('authorityEnvironment: policy.authorityEnvironment, ignoredEnvironment: policy.ignoredEnvironment};',
     'authorityEnvironment: policy.authorityEnvironment, ignoredEnvironment: policy.ignoredEnvironment, ambientEnvironment: policy.ambientEnvironment};'),
    ('  if (kind === "e") { const value = environment[subject]; return (value === undefined ? "-" : digestOf(value)) === expected; }\n',
     '  if (kind === "e") { if (isAmbientEnvironmentNameV1(subject)) return true; const value = environment[subject]; return (value === undefined ? "-" : digestOf(value)) === expected; }\n'),
    ('now = Object.keys(environment).filter(name => !ignored.has(name)).sort()',
     'now = Object.keys(environment).filter(name => !ignored.has(name) && !isAmbientEnvironmentNameV1(name)).sort()'),
])

edit("🟨️.d.mts", [
    ('export function worldWitnessV1(): WorldWitnessRecorderV1 | undefined;\n',
     'export function worldWitnessV1(): WorldWitnessRecorderV1 | undefined;\n/** 🌫️ Tells whether a name is an ambient host, agent or session variable that never is an input of a plan. */\nexport function isAmbientEnvironmentNameV1(name: string): boolean;\n'),
])
